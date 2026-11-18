//! Plant care and sowing gates from the pinned garden table.

use crate::bot::Reading;

pub(super) fn compost(reading: &Reading<'_>) -> std::borrow::Cow<'static, str> {
    use super::lead_data as data;
    use std::borrow::Cow;

    let text = reading.folded();
    if data::COMPOST_WHAT_IS
        .iter()
        .any(|phrase| text.contains(phrase))
    {
        return Cow::Owned(format!("{}{}", data::COMPOST_MEANS, data::COMPOST_JOB));
    }
    let tokens = text.split(|c: char| !(c.is_ascii_lowercase() || c == '\''));
    let says = |words: &[&str]| {
        words.iter().any(|phrase| {
            if phrase.contains(' ') {
                text.contains(phrase)
            } else {
                tokens.clone().any(|word| word == *phrase)
            }
        })
    };
    Cow::Borrowed(if !says(data::COMPOST_AILING) {
        data::COMPOST_JOB
    } else if says(data::COMPOST_FROZEN) {
        data::COMPOST_IN_THE_COLD
    } else if says(data::COMPOST_SMELLY) {
        data::COMPOST_TOO_WET
    } else {
        data::COMPOST_STALLED
    })
}
use regex::Regex;
use std::{ops::RangeInclusive, sync::LazyLock};

use super::{
    GARDEN_MAY_PHRASES, GARDEN_MONTHS, GARDEN_PERSON_CARE_PATTERN,
    GARDEN_PERSON_NEEDS_CARE_PATTERN, GARDEN_PET_CARE_PATTERN, GARDEN_PLANT_PATTERN,
};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(PLANT, GARDEN_PLANT_PATTERN);
pattern!(PERSON_CARE, GARDEN_PERSON_CARE_PATTERN);
pattern!(PERSON_NEEDS_CARE, GARDEN_PERSON_NEEDS_CARE_PATTERN);
pattern!(PET_CARE, GARDEN_PET_CARE_PATTERN);
pattern!(LATE, r"\btoo late\b|\btoo early\b|\blate for\b");
pattern!(NON_WORD, r"[^a-z0-9' ]+");

pub(super) fn plant_care(reading: &Reading<'_>) -> bool {
    let said = &reading.line.clean;
    PLANT.is_match(said)
        || !(PERSON_CARE.is_match(said)
            || PERSON_NEEDS_CARE.is_match(said)
            || PET_CARE.is_match(said))
}

pub(super) fn month(line: &str) -> Option<u8> {
    let words = |c: char| !c.is_ascii_alphanumeric();
    GARDEN_MONTHS
        .iter()
        .find_map(|&(label, month)| line.split(words).any(|word| word == label).then_some(month))
        .or_else(|| {
            GARDEN_MAY_PHRASES
                .iter()
                .any(|phrase| line.contains(phrase))
                .then_some(5)
        })
}

pub(super) fn sowing(
    reading: &Reading<'_>,
    allowed_months: RangeInclusive<u8>,
    reply: &'static str,
    late: &'static str,
    late_overrides_month: bool,
) -> Option<&'static str> {
    let line = NON_WORD.replace_all(&reading.line.clean, " ");
    let worried = LATE.is_match(&line);
    if worried && late_overrides_month {
        return Some(late);
    }
    if month(&line).is_some_and(|month| !allowed_months.contains(&month)) {
        return None;
    }
    Some(if worried { late } else { reply })
}
