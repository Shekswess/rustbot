//! Interpret common figurative descriptions before the conversational reading.
#[path = "pre_read_data.rs"]
mod data;
use data::*;
use regex::{Captures, Regex};
use std::borrow::Cow;

fn replace<'a>(
    source: Cow<'a, str>,
    pattern: &Regex,
    sub: impl Fn(&Captures<'_>) -> String,
) -> Cow<'a, str> {
    match pattern.replace_all(&source, sub) {
        Cow::Borrowed(_) => source,
        Cow::Owned(updated) => Cow::Owned(updated),
    }
}
pub(super) fn prepare(raw: &str) -> Cow<'_, str> {
    let text = replace(Cow::Borrowed(raw), &FEELING_BEFORE_SICK, |m| {
        m[0].split_whitespace().next().unwrap_or("").to_owned()
    });
    let text = replace(text, &SICK_AS_FEELING, |m| {
        feeling(
            &m[1]
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        )
        .into()
    });
    let text = replace(text, &PART_THAT_IS_BAD, |m| {
        let part = m[2].to_lowercase();
        let plural = part.ends_with('s') || THE_ODD_PLURALS.contains(&part.as_str());
        let ongoing = m["link"].to_lowercase().split_whitespace().last() == Some("been");
        let verb = match (ongoing, plural) {
            (true, true) => "have been hurting",
            (true, false) => "has been hurting",
            (false, true) => "hurt",
            (false, false) => "hurts",
        };
        format!("{} {} {verb}", &m[1], &m[2])
    });
    replace(text, &SUGAR_FIRST_PERSON, |m| {
        let tail = m[2].to_lowercase();
        let words = tail
            .split_whitespace()
            .map(|w| w.trim_matches(['.', ',', '!', '?']))
            .collect::<Vec<_>>();
        let unchanged = || m[0].to_owned();
        let meter = || format!("{} blood sugar{}", &m[1], &m[2]);
        if words.iter().any(|w| SUGAR_PRICE_SHAPE.is_match(w)) {
            return unchanged();
        }
        if words
            .iter()
            .any(|w| w.as_bytes().first().is_some_and(u8::is_ascii_digit))
        {
            return meter();
        }
        let levels = words
            .iter()
            .take(3)
            .filter(|w| SUGAR_LEVEL.binary_search(w).is_ok())
            .copied()
            .collect::<Vec<_>>();
        if levels.is_empty() {
            return unchanged();
        }
        if levels == ["rose"]
            && words
                .iter()
                .position(|w| *w == "rose")
                .and_then(|i| words.get(i + 1))
                .is_some_and(|w| SUGAR_AFTER_ROSE.binary_search(w).is_err())
        {
            return unchanged();
        }
        if words.iter().enumerate().any(|(i, w)| {
            SUGAR_LEVEL.binary_search(w).is_ok()
                && words[i + 1..]
                    .iter()
                    .take(3)
                    .any(|w| SUGAR_PRICE.binary_search(w).is_ok())
        }) {
            return unchanged();
        }
        meter()
    })
}
#[cfg(test)]
#[path = "../../../tests/fixtures/pre_read.rs"]
mod reference;
#[test]
fn matches_original_rewrites() {
    for &(raw, expected) in reference::CASES {
        assert_eq!(prepare(raw), expected, "{raw}");
    }
}
