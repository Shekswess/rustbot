//! Convert quoted amounts across pay periods and compare pay with a sample budget.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::{
    money_household::pounds,
    money_savings::{comma, sums},
    money_tax::{SCOTLAND, pay_breakdown},
};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    NAMED_PERIOD,
    r"(?i)^(?:\s*(?:pounds?|quid)\b)?\s*(?:(?:a|an|per|each|every|/)\s*(week|fortnight|month|year)\b|(weekly|fortnightly|monthly|yearly|annually|annual)\b)"
);
pattern!(
    OTHER_PERIOD,
    r"(?i)\b(?:(?:a|an|per|each|every|over|in|for|of|the)\s*(week|fortnight|month|year)|(weekly|fortnightly|monthly|yearly|annually|annual))\b"
);
pattern!(
    EARN,
    r"\b(?:need to earn|earn to live|earn enough|salary do i need|salary would i need|wage do i need)\b|\b(?:salary|wage)\b.{0,20}\b(?:need|needed|require[sd]?)\b"
);
pattern!(LONDON, r"\b(?:london|zone [12]|inner city)\b");
pattern!(
    GROSS,
    r"\b(?:earn\w*|salary|wages?|gross|before tax|my pay|our pay|(?:get|gets|got|am|is|are|was|were) paid)\b"
);
pattern!(
    NET,
    r"\b(?:net pay|net income|after tax|take[- ]home|in the bank)\b"
);

const LIVING: &[&str] = &[
    "enough to live on",
    "enough to live",
    "enough to get by",
    "enough to survive",
    "to live on my own",
    "to live alone",
    "to live in london",
    "to live on your own",
    "enough money to live",
    "money to live on",
    "cost of living",
    "living costs",
    "afford to live",
    "live on",
    "live off",
    "survive on",
    "get by on",
    "need to live",
    "left to live on",
];
const COPING: &[&str] = &[
    "enough",
    "get by",
    "manage",
    "survive",
    "cope",
    "cover",
    "afford",
    "live off",
    "stretch to",
    "left over",
];
const ASK_NEEDED: &[&str] = &[
    "how much does a single person need",
    "how much does someone need",
    "how much do i need",
    "how much money do i need",
    "how much money does it take",
    "what do i need to live",
    "how much does it cost",
];
const ASK_CONVERSION: &[&str] = &[
    "how much",
    "how many",
    "whats",
    "what's",
    "what is",
    "what does",
    "what would",
    "what will",
    "comes to",
    "come to",
    "convert",
    "worth",
    "equivalent",
    "in a year",
    "in a month",
    "in a week",
    "over a year",
    "over a month",
    "a year in total",
    "a year altogether",
];
const KEEP_RECORDS: &[&str] = &[
    "keep",
    "records",
    "receipts",
    "payslips",
    "statements",
    "tax return",
    "how long",
    "how many years",
];

#[derive(Clone, Copy, Eq, PartialEq)]
enum Period {
    Week,
    Fortnight,
    Month,
    Year,
}

impl Period {
    fn parsed(captures: &regex::Captures<'_>) -> Option<Self> {
        let label = captures.get(1).or_else(|| captures.get(2))?.as_str();
        match label.to_ascii_lowercase().as_str() {
            "week" | "weekly" => Some(Self::Week),
            "fortnight" | "fortnightly" => Some(Self::Fortnight),
            "month" | "monthly" => Some(Self::Month),
            "year" | "yearly" | "annual" | "annually" => Some(Self::Year),
            _ => None,
        }
    }

    fn in_year(self) -> f64 {
        match self {
            Self::Week => 52.0,
            Self::Fortnight => 26.0,
            Self::Month => 12.0,
            Self::Year => 1.0,
        }
    }

    fn phrase(self) -> &'static str {
        match self {
            Self::Week => "a week",
            Self::Fortnight => "a fortnight",
            Self::Month => "a month",
            Self::Year => "a year",
        }
    }
}

fn amount_and_period(text: &str) -> Option<(f64, Period, usize, usize)> {
    sums(text).find_map(|(amount, span)| {
        let found = NAMED_PERIOD.captures(&text[span.end()..])?;
        Some((
            amount,
            Period::parsed(&found)?,
            span.start(),
            span.end() + found.get(0)?.end(),
        ))
    })
}

fn period_conversion(text: &str) -> Option<(f64, Period, Period)> {
    let (sum, own, start, end) = amount_and_period(text)?;
    let wanted = OTHER_PERIOD.captures_iter(text).find_map(|captures| {
        let found = captures.get(0)?;
        if found.start() < end && found.end() > start {
            return None;
        }
        Period::parsed(&captures).filter(|&period| period != own)
    })?;
    Some((sum, own, wanted))
}

pub(crate) fn names_two_periods(reading: &Reading<'_>) -> bool {
    period_conversion(reading.raw).is_some()
}

pub(crate) fn about_living_costs(reading: &Reading<'_>) -> bool {
    let text = &reading.clean;
    let about_life = LIVING.iter().any(|word| text.contains(word));
    let coping = COPING.iter().any(|word| text.contains(word));
    if sums(reading.raw).next().is_some() {
        return about_life || (coping && reading.topic.is_none_or(|(topic, _)| topic == "money"));
    }
    (ASK_NEEDED.iter().any(|word| text.contains(word)) || EARN.is_match(text))
        && (about_life || coping)
}

fn net_pay(annual: f64, scotland: bool) -> f64 {
    let (tax, ni) = pay_breakdown(annual, scotland);
    annual - tax - ni
}

fn gross_for(target_monthly: u64, scotland: bool) -> u64 {
    let target = (target_monthly * 12) as f64;
    let mut low = 0;
    let mut high = (target_monthly * 12 * 4).div_ceil(100);
    while low < high {
        let middle = low + (high - low) / 2;
        if net_pay((middle * 100) as f64, scotland) < target {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    low * 100
}

fn convert(text: &str) -> Option<Cow<'static, str>> {
    if KEEP_RECORDS.iter().any(|word| text.contains(word))
        || !ASK_CONVERSION.iter().any(|word| text.contains(word))
    {
        return None;
    }
    let (amount, own, wanted) = period_conversion(text)?;
    let annual = amount * own.in_year();
    let value = annual / wanted.in_year();
    let mut answer = format!(
        "{} {} is about {} {}.",
        pounds(amount),
        own.phrase(),
        pounds(value),
        wanted.phrase()
    );
    if GROSS.is_match(text) && !NET.is_match(text) {
        let net = net_pay(annual, SCOTLAND.is_match(text));
        answer.push_str(&format!(" Using 2026–27 UK rates for a single job with a standard tax code, about {} a month or {} a year would be left after Income Tax and employee National Insurance. Other deductions can change the result.", pounds(net / 12.0), pounds(net)));
    }
    Some(Cow::Owned(answer))
}

fn living_costs(text: &str) -> Cow<'static, str> {
    let scotland = SCOTLAND.is_match(text);
    let london = LONDON.is_match(text);
    let (target, budget) = if london {
        (
            2_250,
            "For an example in London, if you find a place at £1,850 a month and allow £400 for bills, food and travel, you'd need £2,250 after tax. Use local listings and your own bills to replace both figures.",
        )
    } else {
        (
            1_500,
            "The ONS's October 2026 UK average for a one-bedroom private rental was £1,145 a month. In an example using that rent and £355 for bills, food and travel, the monthly total is £1,500. Rent and bills vary, so use figures for where you plan to live.",
        )
    };
    let finish = "Include Council Tax or rates, insurance and a buffer when you make your own budget. MoneyHelper's budget planner can add up the costs.";
    if EARN.is_match(text) {
        return Cow::Owned(format!(
            "{budget} That example needs roughly £{} gross a year under 2026–27 employment tax and National Insurance rates, before any pension or student loan deductions. {finish}",
            comma(gross_for(target, scotland))
        ));
    }
    let Some((amount, period, _, _)) = amount_and_period(text) else {
        return Cow::Owned(format!(
            "{budget} {finish} What would you get after tax each month?"
        ));
    };
    let annual = amount * period.in_year();
    let annual_is_gross = !NET.is_match(text) && (period == Period::Year || GROSS.is_match(text));
    let monthly = if annual_is_gross {
        net_pay(annual, scotland) / 12.0
    } else {
        annual / 12.0
    };
    let available = if annual_is_gross {
        format!(
            "A gross salary of {} a year works out to approximately {} a month after Income Tax and employee National Insurance in 2026–27, before other payroll deductions.",
            pounds(amount),
            pounds(monthly)
        )
    } else {
        format!(
            "That works out to about {} a month, assuming it's money available after tax.",
            pounds(monthly)
        )
    };
    let difference = monthly - target as f64;
    let comparison = if difference >= 0.0 {
        format!(
            "That would leave about {} a month on this example.",
            pounds(difference)
        )
    } else {
        format!(
            "That would be about {} a month short on this example.",
            pounds(-difference)
        )
    };
    Cow::Owned(format!("{available} {budget} {comparison} {finish}"))
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.raw.to_lowercase();
    match id {
        "enough-to-live-on" => Some(living_costs(&text)),
        "a-sum-in-another-period" => convert(&text),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn income_periods_are_annualized_and_reuse_the_pay_estimate() {
        assert!(
            convert("how much is £2,500 a month over a year")
                .unwrap()
                .contains("£30,000 a year")
        );
        assert!(
            convert("if i earn £30,000 a year how much is that a month")
                .unwrap()
                .contains("£2,093.30 a month")
        );
        assert!(
            convert("i pay £42 a month for the gym and go about twice a week, is that daft")
                .is_none()
        );
    }
}
