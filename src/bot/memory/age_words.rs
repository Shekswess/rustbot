//! Age expressions from bot/memory/facts.py.
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};
static AGE_IN_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r###"(?i)\b(?P<words>(?:twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)(?:[-\s](?:one|two|three|four|five|six|seven|eight|nine))?|(?:ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen)|(?:one|two|three|four|five|six|seven|eight|nine))\b"###).unwrap()
});
static BEFORE_A_FIGURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r###"(?i)(?:\b(?:is|are|was|were|aged)\s+|\bi\s+am\s+|\bim\s+|\bi'm\s+|\bam\s+|\b(?:he|she|they|it)(?:'s|s|'re|re)\s+)$"###).unwrap()
});
static AFTER_A_FIGURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r###"(?i)\s*(?:years?\b(?:\s+old\b)?|yrs?\b(?:\s+old\b)?|yo\b)?\s*(?:[,.!?;:]|$|\band\b|\bbut\b|\bso\b)"###).unwrap()
});
static NOT_YEARS_AFTER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r###"(?i)\s*(?:weeks?|months?|days?|hours?|o.clock)\b"###).unwrap()
});
const NUMBERS: &[(&str, u8)] = &[
    ("eight", 8),
    ("eighteen", 18),
    ("eighty", 80),
    ("eleven", 11),
    ("fifteen", 15),
    ("fifty", 50),
    ("five", 5),
    ("forty", 40),
    ("four", 4),
    ("fourteen", 14),
    ("nine", 9),
    ("nineteen", 19),
    ("ninety", 90),
    ("one", 1),
    ("seven", 7),
    ("seventeen", 17),
    ("seventy", 70),
    ("six", 6),
    ("sixteen", 16),
    ("sixty", 60),
    ("ten", 10),
    ("thirteen", 13),
    ("thirty", 30),
    ("three", 3),
    ("twelve", 12),
    ("twenty", 20),
    ("two", 2),
];

pub(super) fn figures(text: &str) -> Cow<'_, str> {
    let Some(m) = AGE_IN_WORDS.find(text) else {
        return Cow::Borrowed(text);
    };
    if !BEFORE_A_FIGURE.is_match(&text[..m.start()])
        || NOT_YEARS_AFTER.is_match(&text[m.end()..])
        || !AFTER_A_FIGURE.is_match(&text[m.end()..])
    {
        return Cow::Borrowed(text);
    }
    let total: u16 = m
        .as_str()
        .split([' ', '-'])
        .filter_map(|w| {
            NUMBERS
                .binary_search_by_key(&w.to_ascii_lowercase().as_str(), |(name, _)| *name)
                .ok()
                .map(|i| u16::from(NUMBERS[i].1))
        })
        .sum();
    if total == 0 {
        return Cow::Borrowed(text);
    }
    let mut replaced = text.to_owned();
    replaced.replace_range(m.range(), &total.to_string());
    Cow::Owned(replaced)
}
