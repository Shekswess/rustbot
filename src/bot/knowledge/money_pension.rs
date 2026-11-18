//! State Pension age and amounts, plus the retirement income gap.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::money_savings::comma;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    AGE_QUESTION,
    r"\bwhen\b|how old|what age|\bdate\b|\bretir(?:e|ing|ement)\b"
);
pattern!(
    AMOUNT_QUESTION,
    r"\b(?:how much|amount|per week|a week|how many years|qualifying)\b|\b\d+\s*years?\b|\byears? of (?:national insurance|ni)\b"
);
pattern!(
    RECORD_QUESTION,
    r"\brecord\b|\bcheck\b.*\b(?:national insurance|ni)\b"
);
pattern!(STOPPING, r"\bretir(?:e|ing|ement)\b|\bstop(?:ping)?\s+work");
pattern!(RETIRE_AT, r"\b(?:at|aged|age|before)\s+(\d{2})\b");
pattern!(
    TIMING,
    r"\b(?:when|what age|how old|pension age|take my pension|draw|claim)\b"
);
pattern!(
    STATE_DETAILS,
    r"\b(?:how much|forecast|check|record|how many years|years do i need|qualifying|voluntary|buy extra|fill a gap)\b"
);

const FORECAST: &str = "The full new State Pension is £241.30 a week in 2026–27, about £12,548 over 52 weeks. You normally need at least 10 qualifying National Insurance years to get any new State Pension. If your record started after April 2016, you need 35 qualifying years for the full rate. Earlier records can work differently. Check your own forecast at gov.uk/check-state-pension.";
const STATE_AGE: &str = "State Pension age is rising from 66 to 67 between 2026 and 2028. The date depends on your birthday. Check gov.uk/state-pension-age and claim when eligible via GOV.UK.";
const PRIVATE_AGE: &str = "The normal minimum age for taking most private or workplace pensions is 55, rising to 57 on 6 April 2028. Your scheme may set a later age and some protections or exceptions apply.";
const RECORD: &str = "Check your National Insurance record and State Pension forecast on GOV.UK. Confirm whether voluntary contributions would increase your forecast before paying to fill a gap.";
const RETIREMENT_GAP: &str = "List your expected spending and subtract the State Pension forecast, any workplace income and other reliable income. That leaves the annual amount your savings need to cover. £10,000 a year from £250,000 would be a 4% starting withdrawal. Treat that as an illustration and test how the plan changes with returns, inflation, fees and a longer retirement. MoneyHelper has a retirement pension calculator.";

pub(crate) fn person_stopped_working(reading: &Reading<'_>) -> bool {
    let text = format!(" {} ", reading.line.norm.join(" ").replace('|', " "));
    let failure = [
        "stopped working",
        "stopped work",
        "stops working",
        "not working",
        "isnt working",
        "isn't working",
        "packed in",
        "wont work",
        "won't work",
        "wont start",
        "won't start",
        "wont come on",
        "won't come on",
        "dead",
        "died",
        "has gone",
    ];
    if !failure
        .iter()
        .any(|phrase| text.contains(&format!(" {phrase} ")))
    {
        return true;
    }
    !reading.entities.iter().any(|entity| {
        entity.kind == "thing"
            && reading
                .context_fields
                .subject
                .as_deref()
                .is_none_or(|subject| {
                    let subject = subject.to_lowercase();
                    subject.is_empty()
                        || subject.contains(&entity.text)
                        || entity.text.contains(&subject)
                })
    })
}

fn state(text: &str) -> String {
    let asks_age = AGE_QUESTION.is_match(text);
    let mut reply = if !asks_age && RECORD_QUESTION.is_match(text) {
        format!("{RECORD} {FORECAST} {STATE_AGE}")
    } else if !asks_age && AMOUNT_QUESTION.is_match(text) {
        format!("{FORECAST} {STATE_AGE}")
    } else {
        format!("{STATE_AGE} {FORECAST}")
    };
    if STOPPING.is_match(text) {
        reply.push_str(" If you stop work before State Pension age, plan how to cover costs until you can claim.");
    }
    reply
}

fn retirement(text: &str) -> String {
    if text.contains("state pension") && STATE_DETAILS.is_match(text) {
        return format!("{FORECAST} {RETIREMENT_GAP}");
    }
    if TIMING.is_match(text)
        && !["how much", "enough"]
            .iter()
            .any(|word| text.contains(word))
    {
        return format!("{PRIVATE_AGE} {STATE_AGE}");
    }
    let age = RETIRE_AT
        .captures(text)
        .and_then(|found| found[1].parse::<u64>().ok());
    if age.is_some_and(|age| age < 66)
        || ["early", "before i", "as soon as"]
            .iter()
            .any(|word| text.contains(word))
    {
        let gap = age.map_or_else(
            || {
                "If you stop work early, calculate the gap until each pension can be claimed."
                    .to_owned()
            },
            |age| {
                format!(
                    "If you stop work at {}, calculate the gap until each pension can be claimed.",
                    comma(age)
                )
            },
        );
        return format!(
            "{gap} {PRIVATE_AGE} {STATE_AGE} Budget for those years using money you can access or other income. {RETIREMENT_GAP}"
        );
    }
    format!("{RETIREMENT_GAP} {STATE_AGE} {PRIVATE_AGE}")
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "state-pension" => Some(Cow::Owned(state(&reading.clean))),
        "retire-how-much" => Some(Cow::Owned(retirement(&reading.clean))),
        _ => None,
    }
}
