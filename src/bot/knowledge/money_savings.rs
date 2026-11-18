//! Savings goals, contribution rates, and starting a buffer.

use std::{borrow::Cow, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use jiff::{Zoned, civil::Date};
use regex::{Captures, Regex};

use crate::bot::{Exchange, Reading, understanding::kinds::Kind};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

const AMOUNT: &str = r"£?\s*([0-9][0-9,]*(?:\.[0-9]+)?)(?:\s*(k|grand|thousand)\b)?";
const BETWEEN: &str = r"\s*(?:(?!\d|(?:days?|hours?|hrs?|minutes?|mins?|weeks?|months?|years?|nights?|times|sessions?|miles|km|kilos?|kg|lbs?|stone|stones|litres?|liters?|pints?|cups?|eggs?|people|shifts?|pages?|chapters?|lessons?|classes?|meetings?)\b)[\w']+\s+){0,2}";
pattern!(SUM, &format!("(?i){AMOUNT}"));
pattern!(
    DIRECT_RATE,
    &format!(r"(?i){AMOUNT}\s*(?:a|per|each|every)\s*(month|week|fortnight|year)\b")
);
pattern!(
    AGE_BEFORE,
    r"(?i)\b(?:i(?:'m|m|'ve|ve)?|he(?:'s|s)?|she(?:'s|s)?|they(?:'re|re)?|we(?:'re|re)?|mum|mom|mother|dad|father|son|daughter|wife|husband|partner|boyfriend|girlfriend|nan|gran|granny|grandma|grandad|grandpa|brother|sister|niece|nephew|cousin|friend|kid|child|boy|girl|baby|lad|lass|hubby)\s*(?:is|are|was|were|am|'s|’s)?\s*$|\baged\s*$"
);
pattern!(
    AGE_AFTER,
    r"(?i)^\s*(?:(?:years?|yrs?|y/o)(?:\s*old)?|st|nd|rd|th)\b"
);
pattern!(
    MONEY_AFTER,
    r"(?i)^\s*(?:£|(?:pounds?|quid|grand|k|thousand|a|per|each|every)\b)"
);
pattern!(
    DURATION,
    r"(?i)\b(?:for|in|over|after|within|across)\s+(a|an|one|two|three|four|five|six|seven|eight|nine|ten|\d+(?:\.\d+)?)\s*(years?|months?|weeks?|fortnights?)\b"
);
pattern!(
    LENGTH_ASK,
    r"(?i)\b(?:how long|how soon|how many (?:weeks|months|years)|when will i have)\b"
);
pattern!(
    AIM_FOR,
    r"(?i)\b(?:aim for|aim to save|target|goal|save|saving|save up|put aside|set aside|need|needs|deposit of|deposit is|at least)\b[^.£]{0,24}£([\d,]+)"
);
pattern!(
    MONTH_NAME,
    r"(?i)\b(january|february|march|april|may|june|july|august|september|october|november|december|christmas|xmas)\b"
);
pattern!(WEEKLY, r"(?i)\b(?:(?:a|per|each|every)\s*week|weekly)\b");
pattern!(
    INTRODUCTION,
    r"(?i)^\s*(?:(?:hi|hey|hiya|hello)[, !]+)?(?:my name is|my name's|call me|i'm called|i am called|i'm|im|i am)\s+([\p{L}][\p{L}'’-]{0,39})\s*(?:[,!]|and\b)"
);
pattern!(
    MONEY_SUBJECT,
    r"(?i)[\d£]|\b(?:pounds?|quid|money|cash|sav(?:e[sd]?|ing|ings)|afford(?:able)?|budget(?:ing)?|income|salary|wages?|pay|paid|payday|earn(?:s|ings|ed)?|spend(?:s|ing)?|spent|costs?|put by|put aside|set aside|put away|holiday|christmas|wedding|deposit|house|flat|mortgage|car|trip|emergency|funds?)\b"
);
pattern!(
    SAVING_RATE,
    r"(?i)\b(?:save|saving|saved|put by|put away|put aside|set aside)\b[^.!?]{0,30}?(\d[\d,]*(?:\.\d+)?)\s*(?:(?:k|grand|thousand)\b)?\s*(?:pounds?|quid)?\s*(?:a|per|each|/)\s*(?:month|week)\b"
);
pattern!(
    INCOME,
    r"(?i)\b(?:on|from|out of|of|with|earn|earns|earned|earning|make|makes|made|paid|salary|wage|income)\b(?:\s+(?:my|our|your|his|her|their|the|a|an|some))?(?:\s+[a-z]+){0,2}\s*$"
);
static RATE: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(&format!(
        r"(?i){AMOUNT}{BETWEEN}\s*(?:a|per|each|every)\s*(month|week|fortnight|year)\b"
    ))
    .expect("monthly savings rate")
});
static RATE_AFTER: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(&format!(
        r"(?i)^{BETWEEN}\s*(?:a|per|each|every|/)\s*(?:month|week|fortnight|year|day)\b"
    ))
    .expect("words following a savings rate")
});

const GOING_OUT: &[&str] = &[
    "rent",
    "mortgage",
    "bill",
    "spend",
    "spent",
    "cost",
    "pay for",
    "paying for",
    "childcare",
    "nursery",
    "council tax",
    "insurance",
    "fuel",
    "petrol",
    "diesel",
    "commuting",
    "season ticket",
    "subscription",
    "fee",
    "fare",
    "on coffee",
    "on food",
    "on shopping",
    "on drink",
    "takeaway",
    "smoking",
    "cigarettes",
    "gym",
];
const GOING_IN: &[&str] = &[
    "save",
    "saving",
    "put away",
    "put by",
    "put aside",
    "putting away",
    "putting by",
    "set aside",
    "setting aside",
    "spare",
    "add to",
    "adding to",
    "into savings",
    "savings account",
    "pay in",
    "paying in",
    "pay off",
    "paying off",
    "overpay",
    "clear the card",
    "clearing",
    "towards",
    "toward",
    "goes in",
    "going in",
    "i can manage",
    "i could manage",
    "i can do",
    "i could do",
    "i can put",
    "i could put",
    "i manage",
];
const RATES: &[u64] = &[100, 250, 500, 750, 1_000, 1_500];
const MONTHS: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const SAVINGS_FALLBACK: &str = "Work backwards from the date you need it. Divide the total by the weeks left to get the weekly figure, then set a standing order for an amount your budget can cover after payday into a separate account. If you can spare a bonus, refund or saving on a bill, add it to the pot.";

fn amount(captures: &Captures<'_>) -> Option<f64> {
    let base = captures
        .get(1)?
        .as_str()
        .replace(',', "")
        .parse::<f64>()
        .ok()?;
    let total = base
        * if captures.get(2).is_some() {
            1_000.0
        } else {
            1.0
        };
    (total.is_finite() && total < 1e15).then_some(total)
}

pub(crate) fn sums(text: &str) -> impl Iterator<Item = (f64, regex::Match<'_>)> {
    SUM.captures_iter(text).filter_map(move |captures| {
        let raw = captures.get(0)?;
        let after = &text[raw.end()..];
        if !(raw.as_str().contains('£') || captures.get(2).is_some() || MONEY_AFTER.is_match(after))
            && (AGE_BEFORE.is_match(&text[..raw.start()]) || AGE_AFTER.is_match(after))
        {
            return None;
        }
        Some((amount(&captures)?, raw))
    })
}

pub(crate) fn comma(amount: u64) -> String {
    let digits = amount.to_string();
    let mut grouped = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

fn contribution(text: &str, start: usize, end: usize) -> bool {
    let preceding = &text[..start];
    let left = [" and ", " but ", " so ", " then "]
        .into_iter()
        .filter_map(|delimiter| {
            preceding
                .rfind(delimiter)
                .map(|index| index + delimiter.len())
        })
        .chain(preceding.rfind(['.', ';', ',', '?']).map(|index| index + 1))
        .max()
        .unwrap_or(0);
    let rest = &text[end..];
    let right = [" and ", " but ", " so ", " then "]
        .into_iter()
        .filter_map(|delimiter| rest.find(delimiter))
        .chain(rest.find(['.', ';', ',', '?']))
        .min()
        .unwrap_or(rest.len());
    let clause = &text[left..end + right];
    !GOING_OUT.iter().any(|word| clause.contains(word))
        || GOING_IN.iter().any(|word| clause.contains(word))
}

fn per_year(unit: &str) -> f64 {
    match unit {
        "year" => 1.0,
        "month" => 12.0,
        "week" => 52.0,
        "fortnight" => 26.0,
        _ => unreachable!("period readers limit units"),
    }
}

fn monthly(text: &str) -> Option<(u64, f64, &str)> {
    let rate = RATE.captures(text).ok()??;
    let span = rate.get(0)?;
    if !contribution(text, span.start(), span.end()) {
        return None;
    }
    let amount = rate.get(1)?.as_str().replace(',', "").parse::<f64>().ok()?
        * if rate.get(2).is_some() { 1_000.0 } else { 1.0 };
    let unit = rate.get(3)?.as_str();
    let monthly = (amount * per_year(unit) / 12.0).round_ties_even();
    (monthly > 0.0 && monthly < 1e15).then_some((monthly as u64, amount, unit))
}

fn goal_in(text: &str, monthly: u64) -> u64 {
    sums(text)
        .filter(|(value, span)| {
            *value as u64 != monthly && !RATE_AFTER.is_match(&text[span.end()..]).unwrap_or(false)
        })
        .map(|(value, _)| value as u64)
        .max()
        .unwrap_or(0)
}

fn stated_goal(earlier: &[Exchange], monthly: u64) -> u64 {
    earlier
        .iter()
        .rev()
        .filter(|turn| !turn.user.is_empty())
        .take(4)
        .map(|turn| goal_in(&turn.user, monthly))
        .find(|goal| *goal > monthly)
        .unwrap_or(0)
}

fn suggested_goal(earlier: &[Exchange]) -> u64 {
    earlier
        .iter()
        .rev()
        .take(4)
        .filter_map(|turn| AIM_FOR.captures(&turn.bot))
        .filter_map(|captures| captures[1].replace(',', "").parse().ok())
        .find(|goal| *goal > 0)
        .unwrap_or(0)
}

pub(super) fn count(word: &str) -> Option<u64> {
    match word {
        "a" | "an" | "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        _ => word.parse().ok(),
    }
}

fn over_time(text: &str) -> Option<String> {
    let rate = DIRECT_RATE.captures(text)?;
    let span = rate.get(0)?;
    if !contribution(text, span.start(), span.end()) {
        return None;
    }
    let amount = amount(&rate)?;
    let rate_unit = &rate[3];
    let duration = DURATION.captures(text)?;
    let count = count(&duration[1]).or_else(|| Some(duration[1].parse::<f64>().ok()? as u64))?;
    if !(1..=60).contains(&count) || amount <= 0.0 {
        return None;
    }
    let unit = duration[2].trim_end_matches('s');
    let payments = per_year(rate_unit) * count as f64 / per_year(unit);
    if payments < 1.0 {
        return None;
    }
    let total = (amount * payments).round_ties_even() as u64;
    let length = if count == 1 {
        format!("a {unit}")
    } else {
        format!("{count} {unit}s")
    };
    let inferred = total as f64 / amount;
    let working = if (inferred - inferred.round()).abs() < 0.01 && inferred.round() as u64 != count
    {
        format!(
            " That is {} {rate_unit}s at £{} each.",
            inferred.round(),
            comma(amount as u64)
        )
    } else {
        String::new()
    };
    Some(format!(
        "£{} a {rate_unit} for {length} is £{}.{working} Set the standing order for the day after payday if the amount fits your budget.",
        comma(amount as u64),
        comma(total)
    ))
}

fn length(months: u64) -> String {
    let (years, rest) = (months / 12, months % 12);
    match (months, rest) {
        (0..=1, _) => "a single month at that rate".to_owned(),
        (2..=23, _) => format!("{months} months"),
        (_, 0) => format!("{months} months, {years} years"),
        (_, 1) => format!("{months} months, about {years} years and a month"),
        _ => format!("{months} months, about {years} years and {rest} months"),
    }
}

pub(crate) fn sample_rates(goal: u64, earlier: &[Exchange]) -> Option<String> {
    let rates = RATES
        .iter()
        .filter(|rate| **rate < goal)
        .filter(|rate| {
            let amount = format!("£{} a month", comma(**rate));
            !earlier.iter().any(|turn| turn.bot.contains(&amount))
        })
        .take(3)
        .map(|rate| {
            format!(
                "£{} a month is {}",
                comma(*rate),
                length(goal.div_ceil(*rate))
            )
        })
        .collect::<Vec<_>>();
    let options = if !rates.is_empty() {
        rates.join(", ")
    } else if (21..=100).contains(&goal) && earlier.is_empty() {
        ["one month", "two months", "three months", "four months"]
            .iter()
            .enumerate()
            .filter_map(|(index, label)| {
                let months = index as u64 + 1;
                let rate = goal / months;
                (goal.is_multiple_of(months)
                    && rate >= 20
                    && (months == 1 || rate.is_multiple_of(5)))
                .then(|| format!("{label} at £{} a month", comma(rate)))
            })
            .take(2)
            .collect::<Vec<_>>()
            .join(" or ")
    } else {
        return None;
    };
    Some(format!(
        "For £{}: {options}. Pick a rate your budget can carry and set the standing order for the day after payday.",
        comma(goal)
    ))
}

pub(crate) fn time_to_save(text: &str, earlier: &[Exchange]) -> Option<String> {
    if let Some(answer) = over_time(text) {
        return Some(answer);
    }
    let Some((monthly, contribution, period)) = monthly(text) else {
        let goal = goal_in(text, 0);
        return if goal > 0 {
            sample_rates(goal, &[])
        } else if LENGTH_ASK.is_match(text) {
            sample_rates(stated_goal(earlier, 0), earlier)
        } else {
            None
        };
    };
    let goal = [
        goal_in(text, monthly),
        stated_goal(earlier, monthly),
        suggested_goal(earlier),
    ]
    .into_iter()
    .find(|goal| *goal > 0)
    .unwrap_or(0);
    let mut rate = format!("£{} a month", comma(monthly));
    if period != "month" {
        rate.push_str(&format!(" (£{} a {period})", comma(contribution as u64)));
    }
    if goal == 0 {
        return Some(format!(
            "{rate} is £{} in a year and £{} in five. Say what you are saving for and the total, and I will work out how long it takes.",
            comma(monthly * 12),
            comma(monthly * 60)
        ));
    }
    Some(format!(
        "£{} at {rate} is {}. A standing order after payday can help if the amount fits your budget.",
        comma(goal),
        length(goal.div_ceil(monthly))
    ))
}

fn deadline(text: &str) -> Option<(u8, &'static str)> {
    let captures = MONTH_NAME.captures(text)?;
    let word = &captures[1];
    let index = MONTHS
        .iter()
        .position(|month| month.eq_ignore_ascii_case(word))
        .unwrap_or(11);
    Some((index as u8 + 1, MONTHS[index]))
}

fn weeks_until(today: Date, month: u8) -> Option<u64> {
    let month = i8::try_from(month).ok()?;
    let year = today.year() + i16::from(month <= today.month());
    let deadline = Date::new(year, month, 1).ok()?;
    let days = u64::try_from(today.until(deadline).ok()?.get_days()).ok()?;
    Some(days.div_ceil(7))
}

// MoneyHelper's savings habit and budget guidance use affordable regular contributions.
fn put_by(text: &str, earlier: &[Exchange], today: Date) -> Option<String> {
    if let Some(answer) = over_time(text) {
        return Some(answer);
    }
    let goal = [
        goal_in(text, 0),
        stated_goal(earlier, 0),
        suggested_goal(earlier),
    ]
    .into_iter()
    .find(|goal| *goal > 0)
    .unwrap_or(0);
    if goal > 0 {
        let month = deadline(text).or_else(|| {
            earlier
                .iter()
                .rev()
                .take(4)
                .find_map(|turn| deadline(&turn.user))
        });
        if let Some((month, label)) = month
            && let Some(weeks) = weeks_until(today, month)
            && weeks >= 2
        {
            return Some(format!(
                "£{} by {label} is about £{} a week over the {} weeks left. Set a standing order on payday if that amount fits your budget.",
                comma(goal),
                comma(goal.div_ceil(weeks)),
                comma(weeks)
            ));
        }
        return time_to_save(text, earlier);
    }
    if !MONEY_SUBJECT.is_match(text) {
        return None;
    }
    let example = if WEEKLY.is_match(text) {
        "On £2,000 a month, 10% to 20% is roughly £46 to £92 a week."
    } else {
        "On £2,000 a month, 10% to 20% would be £200 to £400 a month."
    };
    Some(format!(
        "{example} Treat that as an example. Work out what's left after essential bills and debt repayments, then pick a regular amount you can afford."
    ))
}

pub(crate) fn rate_prompt(reading: &Reading<'_>) -> bool {
    let text = reading.clean.as_str();
    if ![
        "cant save",
        "can't save",
        "cannot save",
        "unable to save",
        "no savings",
        "nothing put by",
    ]
    .iter()
    .any(|phrase| text.contains(phrase))
    {
        return true;
    }
    SAVING_RATE
        .captures_iter(text)
        .filter_map(|captures| captures.get(1))
        .any(|sum| !INCOME.is_match(&text[..sum.start()]))
}

pub(crate) fn reply(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let text = reading.raw.to_lowercase();
    match id {
        "save-goal-time" => time_to_save(&text, earlier).map(Cow::Owned),
        "save-for-a-thing" | "save-goal" => {
            if reading.kind != Kind::Question
                && INTRODUCTION
                    .captures(reading.raw)
                    .is_some_and(|name| crate::bot::memory::recognized_name(&name[1]))
            {
                return None;
            }
            Some(time_to_save(&text, earlier).map_or(Cow::Borrowed(SAVINGS_FALLBACK), Cow::Owned))
        }
        "save-goal-week" => put_by(&text, earlier, Zoned::now().date()).map(Cow::Owned),
        "savings-gap" => {
            let age = reading
                .clean
                .split_whitespace()
                .filter_map(|word| word.parse::<u16>().ok())
                .find(|age| (16..=80).contains(age))
                .map_or_else(String::new, |age| format!(" at {age}"));
            Some(Cow::Owned(format!(
                "If you've got nothing put by{age}, start with an amount you can afford after bills. If £10 or £25 on payday fits, put it in an easy-access savings account. £500 and £1,000 can be useful milestones. Then work towards a month of rent, bills and food. What is left each month after the bills go out?"
            )))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn savings_calculates_goals_and_ignores_rent_rates() {
        assert!(
            time_to_save("how long to save £5,000 at £250 a month", &[])
                .unwrap()
                .contains("20 months")
        );
        assert!(
            time_to_save("if i save £50 a week for two years", &[])
                .unwrap()
                .contains("£5,200")
        );
        assert!(
            time_to_save("i pay 800 a month rent and want to save 5000", &[])
                .unwrap()
                .contains("£100 a month is")
        );
        let today = Date::new(2026, 11, 18).unwrap();
        assert!(
            put_by(
                "how much to save a week to have £600 by december",
                &[],
                today
            )
            .unwrap()
            .contains("£300 a week")
        );
    }
}
