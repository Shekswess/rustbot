//! Newborn news and birth-weight context.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

macro_rules! pattern {
    ($name:ident, $pattern:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}

pattern!(ELSE_BABY, super::NEWBORN_ELSE_BABY);
pattern!(DAYS, super::NEWBORN_DAYS);
pattern!(WEEKS, super::NEWBORN_WEEKS);
pattern!(WEIGHT, super::NEWBORN_WEIGHT);
pattern!(BIRTH, super::NEWBORN_CHAT_BIRTH);
pattern!(PRONOUN, r"\b(?:she|he|they)\s+(?:was|is|were|has been)\b");
pattern!(FAMILY, r"\b(?:my|our)\b");
pattern!(
    MINE,
    r"\b(?:my|our)\s+(?:baby|son|daughter|newborn)\b|\bi (?:had|gave)\b"
);
pattern!(
    OLDER,
    r"\b(?:[2-9]|1[0-9]|2[0-4])\s+months?\s+old\b|\byears?\s+old\b"
);

fn age(pattern: &Regex, text: &str) -> Option<u8> {
    pattern.captures(text)?[1].parse().ok()
}

pub(super) fn news(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    !text.is_empty()
        && (!PRONOUN.is_match(text) || FAMILY.is_match(text))
        && (!ELSE_BABY.is_match(text) || MINE.is_match(text))
        && !age(&DAYS, text).is_some_and(|days| days > 14)
        && !age(&WEEKS, text).is_some_and(|weeks| weeks > 2)
        && !OLDER.is_match(text)
        && !super::NEWBORN_NOT_FORTNIGHT
            .iter()
            .any(|word| text.contains(word))
}

pub(super) fn birth_weight(
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    if !WEIGHT.is_match(text) {
        return None;
    }
    if !text.contains("birth")
        && (age(&DAYS, text).is_some_and(|days| days > 21)
            || age(&WEEKS, text).is_some_and(|weeks| weeks > 6)
            || (!BIRTH.is_match(text)
                && !earlier
                    .iter()
                    .rev()
                    .take(2)
                    .any(|turn| BIRTH.is_match(&turn.user))))
    {
        return None;
    }
    Some(Cow::Borrowed(
        "Birth weight depends in part on how early the baby was born. Your midwife or health visitor can plot their weight in the red book and explain the trend. Get advice promptly if they are feeding poorly, have fewer wet nappies than usual or you are worried about weight loss.",
    ))
}
