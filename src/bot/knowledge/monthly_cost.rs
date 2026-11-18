//! Annualised costs for amounts and rates the user gives.

use super::monthly_cost_data as data;
use crate::bot::{Reading, knowledge::money_savings::comma};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $value:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

pattern!(SUM, data::SUM);
pattern!(TOO_MUCH, data::TOO_MUCH);
pattern!(WORTH_IT, data::WORTH_IT);
pattern!(ABOUT_RIGHT, data::ABOUT_RIGHT);
pattern!(FIXED_FEE, data::FIXED_FEE);
pattern!(NOT_POUNDS, data::NOT_POUNDS);
pattern!(SHORT_PERIOD, data::SHORT_PERIOD);
pattern!(ONE_OCCASION, data::ONE_OCCASION);
pattern!(NOT_A_BILL, data::NOT_A_BILL);
pattern!(USES_IT, data::USES_IT);
pattern!(NOT_GOING, data::NOT_GOING);

static PERIODS: LazyLock<Vec<(Regex, f64)>> = LazyLock::new(|| {
    data::PERIODS
        .iter()
        .map(|(pattern, year)| (Regex::new(pattern).expect("valid billing period"), *year))
        .collect()
});

pub(super) struct Price {
    pub(super) key: &'static str,
    pub(super) names: &'static [&'static str],
    pub(super) thing: &'static str,
    pub(super) span: (f64, f64),
    pub(super) per_visit: bool,
    pub(super) advice: &'static str,
}

enum Visits {
    Unspecified,
    NoPeriod,
    Count(f64),
}

fn per_month(text: &str) -> Option<f64> {
    SUM.captures_iter(text).find_map(|capture| {
        let mut amount: f64 = capture[1].replace(',', "").parse().ok()?;
        if capture.get(2).is_some() {
            amount *= 1_000.0;
        }
        if !(1.0..1e12).contains(&amount) {
            return None;
        }
        let rest = &text[capture.get(0)?.end()..];
        let suffix_end = rest
            .char_indices()
            .nth(22)
            .map_or(rest.len(), |(index, _)| index);
        PERIODS.iter().find_map(|(pattern, year)| {
            pattern
                .is_match(&rest[..suffix_end])
                .then_some(amount * year / 12.0)
        })
    })
}

fn visits(text: &str) -> Visits {
    let Some(capture) = USES_IT.captures(text) else {
        return Visits::Unspecified;
    };
    let said = &capture[1];
    let count = data::VISITS
        .iter()
        .find_map(|(name, count)| (*name == said).then_some(*count as f64))
        .or_else(|| said.split_whitespace().next()?.parse().ok())
        .unwrap_or(0.0);
    if !(0.0..1e9).contains(&count) || count == 0.0 {
        return Visits::Unspecified;
    }
    match capture.get(2).map(|period| period.as_str()) {
        Some("month") => Visits::Count(count),
        Some("day") => Visits::Count(count * 30.0),
        Some("fortnight") => Visits::Count(count * 2.0),
        Some(_) => Visits::Count(count * 52.0 / 12.0),
        None => Visits::NoPeriod,
    }
}

fn pounds(value: f64) -> String {
    format!("£{}", comma(value.round_ties_even() as u64))
}

fn rough_verdict(text: &str, monthly: f64, price: &Price) -> &'static str {
    let (low, high) = price.span;
    if WORTH_IT.is_match(text) {
        return if monthly < (low + high) / 2.0 {
            "That falls towards the lower end of the rough comparison range."
        } else if monthly <= high {
            "That falls within the rough comparison range."
        } else {
            "That is above the rough comparison range."
        };
    }
    if ABOUT_RIGHT.is_match(text) {
        return if monthly <= high {
            "That falls within or below the rough comparison range."
        } else {
            "That is above the rough comparison range."
        };
    }
    if monthly < low {
        "That is below the rough comparison range."
    } else if monthly <= high {
        "That falls within the rough comparison range."
    } else if monthly <= high * 1.6 {
        "That is a little above the rough comparison range."
    } else {
        "That is well above the rough comparison range."
    }
}

fn best_match(where_: &str) -> Option<&'static Price> {
    let where_ = format!(
        " {} ",
        where_.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    data::SUBSCRIPTIONS
        .iter()
        .filter_map(|price| {
            let best = price
                .names
                .iter()
                .filter(|name| where_.contains(&format!(" {name} ")))
                .map(|name| (!data::PAYMENT_ONLY.contains(name), name.len()))
                .max()?;
            Some((best, price))
        })
        .reduce(|best, candidate| {
            if candidate.0 > best.0 {
                candidate
            } else {
                best
            }
        })
        .map(|(_, price)| price)
}

pub(super) fn reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.raw.to_lowercase();
    if !(TOO_MUCH.is_match(&text) || WORTH_IT.is_match(&text) || ABOUT_RIGHT.is_match(&text))
        || FIXED_FEE.is_match(&text)
        || NOT_A_BILL.is_match(&text)
        || NOT_POUNDS.is_match(&text)
    {
        return None;
    }
    let monthly = per_month(&text)?;
    let monthly_price = pounds(monthly);
    let annual_price = pounds(monthly * 12.0);
    let at_rate = format!("{monthly_price} a month is {annual_price} a year at the same rate.");
    let named = reading.line.words.join(" ");
    let named = if named.is_empty() { &text } else { &named };
    let price = best_match(named)?;
    if NOT_GOING.is_match(&text) {
        if price.per_visit {
            return Some(Cow::Owned(format!(
                "{at_rate} If you rarely use {} you may get better value paying per visit. Check the minimum term, notice period and exit fees before changing the membership.",
                price.thing,
            )));
        }
        return None;
    }
    match visits(&text) {
        Visits::Count(count) if count >= 1.0 && price.per_visit => {
            let per_visit = pounds(monthly / count);
            return Some(Cow::Owned(format!(
                "{at_rate} At about {:.0} visits a month, that is {per_visit} per visit. Compare that with the current pay-as-you-go price, including joining or booking fees. {}",
                count.round_ties_even(),
                price.advice,
            )));
        }
        Visits::Unspecified => (),
        _ => return None,
    }
    if price.per_visit && SHORT_PERIOD.is_match(&text) && ONE_OCCASION.is_match(&text) {
        return None;
    }
    if matches!(price.key, "gas" | "electricity" | "energy") {
        return Some(Cow::Owned(format!("{at_rate} {}", price.advice)));
    }
    let (low, high) = price.span;
    Some(Cow::Owned(format!(
        "{} {at_rate} A rough comparison for {} is {} to {} per month. Check current local prices and what is included. {}",
        rough_verdict(&text, monthly, price),
        price.thing,
        pounds(low),
        pounds(high),
        price.advice,
    )))
}

pub(super) fn can_compare(reading: &Reading<'_>) -> bool {
    reply(reading).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annualises_and_uses_the_specific_subscription() {
        let reply = reply(&Reading::new(
            "Is £20 a month for a magazine subscription too much?",
        ))
        .unwrap();
        assert!(reply.contains("£240 a year") && reply.contains("a magazine"));
        let reply = super::reply(&Reading::new("Is £10 a week for a gym too much?")).unwrap();
        assert!(reply.contains("£520 a year"));
    }

    #[test]
    fn paid_use_and_non_pounds_yield_when_frequency_or_currency_is_unknown() {
        let answer = reply(&Reading::new(
            "Is £40 a month for a gym too much if I go twice a month?",
        ))
        .unwrap();
        assert!(answer.contains("£20 per visit"));
        assert!(reply(&Reading::new("Is $40 a month for a gym too much?")).is_none());
        assert!(
            reply(&Reading::new(
                "Is £40 a month for a gym too much if I go twice?"
            ))
            .is_none()
        );
    }
}
