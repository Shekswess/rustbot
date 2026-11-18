//! Authored verdicts and their order within a knowledge reply.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::Reading;

use super::lead_data as data;

#[path = "leading_shape.rs"]
mod shape;

pub(super) fn move_answer_sentence(reading: &Reading<'_>, reply: &str) -> Option<String> {
    shape::apply(reading.wants, reading.raw, reply)
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Lead {
    None,
    Fixed(&'static str),
    ColdOrFlu,
    Compost,
    ChangeOfMind,
    Assessment,
    Passport,
}

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(SENTENCE_END, r"[.!?]\s+");
pattern!(
    ABBREVIATION,
    r"(?i)\b(?:e\.g|i\.e|approx|incl|etc|vs|cf|fig|dept|est|ref|tel|mr|mrs|ms|dr|prof)\.$"
);
pattern!(COLD_SAME, data::COLD_SAME);
pattern!(COLD_TWICE, data::COLD_TWICE);
pattern!(COMPOST_MATERIAL, data::COMPOST_MATERIAL);
pattern!(TOO_LATE, data::TOO_LATE);
pattern!(STILL_TIME, data::STILL_TIME);
pattern!(ASSESSMENT, data::ASSESSMENT);
pattern!(VALIDITY, data::VALIDITY);
pattern!(UNDER_SIX, data::UNDER_SIX);
pattern!(TIME_LEFT, data::TIME_LEFT);
pattern!(ABOUT_PASSPORT, data::ABOUT_PASSPORT);
pattern!(ABOUT_JOURNEY, data::ABOUT_JOURNEY);

impl Lead {
    fn resolve(self, reading: &Reading<'_>) -> Option<&'static str> {
        let raw = reading.folded();
        let clean = reading
            .line
            .clean
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        match self {
            Self::None => None,
            Self::Fixed(text) => Some(text),
            Self::ColdOrFlu => {
                if COLD_SAME.is_match(raw) {
                    Some("No")
                } else {
                    COLD_TWICE.is_match(raw).then_some("Yes")
                }
            }
            Self::Compost => COMPOST_MATERIAL
                .is_match(&clean)
                .then_some(data::NO_COOKED_FOOD),
            Self::ChangeOfMind => Some(if TOO_LATE.is_match(raw) {
                "No, not if it is within 14 days"
            } else if STILL_TIME.is_match(raw) {
                "Yes, if it is within 14 days"
            } else {
                "Yes"
            }),
            Self::Assessment => ASSESSMENT
                .is_match(&clean)
                .then_some(data::NOT_MEANS_TESTED),
            Self::Passport => {
                too_little_passport_time(raw).then_some("No - that is too little time left")
            }
        }
    }
}

fn too_little_passport_time(said: &str) -> bool {
    let said = said.split_whitespace().collect::<Vec<_>>().join(" ");
    if !VALIDITY.is_match(&said) {
        return false;
    }
    if UNDER_SIX.is_match(&said) {
        return true;
    }
    let marks = |pattern: &Regex| {
        said.split_whitespace()
            .enumerate()
            .filter_map(|(index, word)| pattern.is_match(word).then_some(index))
            .collect::<Vec<_>>()
    };
    let mut near = marks(&ABOUT_PASSPORT);
    if near.is_empty() {
        near = marks(&ABOUT_JOURNEY);
    }
    let best = TIME_LEFT
        .captures_iter(&said)
        .filter_map(|capture| {
            let matched = capture.get(0)?;
            let first = said[..matched.start()].split_whitespace().count();
            let last = first
                + matched
                    .as_str()
                    .split_whitespace()
                    .count()
                    .saturating_sub(1);
            let distance = near
                .iter()
                .flat_map(|mark| [mark.abs_diff(first), mark.abs_diff(last)])
                .min()?;
            (distance <= 6).then_some((distance, capture))
        })
        .min_by_key(|(distance, _)| *distance);
    let Some((_, best)) = best else {
        return false;
    };
    let count = best[1]
        .parse::<u16>()
        .ok()
        .or_else(|| {
            data::NUMBERS
                .iter()
                .find_map(|(name, value)| (*name == &best[1]).then_some(*value))
        })
        .unwrap_or(99);
    let unit = &best[2];
    unit.starts_with("day")
        || (unit.starts_with("week") && count < 26)
        || (unit.starts_with("month") && count < 6)
}

fn sentences(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    for ending in SENTENCE_END.find_iter(text) {
        let end = ending.start() + 1;
        let chunk = &text[start..end];
        if !ABBREVIATION.is_match(chunk) {
            parts.push(chunk);
            start = ending.end();
        }
    }
    if start < text.len() {
        parts.push(&text[start..]);
    }
    parts
}

fn tokens(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
}

fn holds(text: &str, needed: &[&str]) -> bool {
    let words = text.to_lowercase();
    needed
        .iter()
        .all(|need| tokens(&words).any(|word| &word == need))
}

pub(super) fn apply(
    lead: Lead,
    reading: &Reading<'_>,
    payload: &str,
    sent: &str,
) -> Option<String> {
    if reading.wants != Some("yesno") || matches!(lead, Lead::None) {
        return None;
    }
    let card = payload.split_whitespace().collect::<Vec<_>>().join(" ");
    let reply = sent.split_whitespace().collect::<Vec<_>>().join(" ");
    let opening: String = card.chars().take(50).collect();
    if opening.is_empty() || !reply.starts_with(&opening) {
        return None;
    }
    let lead = lead.resolve(reading)?;
    let lead = lead.split_whitespace().collect::<Vec<_>>().join(" ");
    let bare = lead.trim_end_matches([' ', '.', '!', '?']);
    if bare.is_empty() || reply.to_lowercase().starts_with(&bare.to_lowercase()) {
        return None;
    }
    let parts = sentences(&reply);
    let first = parts.first()?;
    let lower = bare.to_lowercase();
    let expected: Vec<_> = tokens(&lower).filter(|word| word.len() > 2).collect();
    let echoed = if expected.is_empty() {
        lower.trim_matches([' ', '.', ',', '!', '?'])
            == tokens(&first.to_lowercase()).collect::<Vec<_>>().join(" ")
    } else {
        holds(first, &expected)
    };
    if echoed {
        return None;
    }
    if !expected.is_empty()
        && let Some(index) = parts.iter().position(|part| holds(part, &expected))
        && index > 0
    {
        return Some(
            parts[index..]
                .iter()
                .chain(parts[..index].iter())
                .copied()
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
    Some(format!("{bare}. {}", sent.trim_start()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_yesno_question_moves_the_cards_own_verdict() {
        let card = "A suddenly louder exhaust usually means a joint has leaked. It is not an emergency, but fumes can get into the cabin. Book it in this week.";
        let yesno = Reading::new("is a loud exhaust an emergency?");
        let statement = Reading::new("my exhaust suddenly got louder");
        let lead = Lead::Fixed("It is not an emergency");
        let actual = apply(lead, &yesno, card, card).expect("authored verdict");
        assert_eq!(
            actual,
            "It is not an emergency, but fumes can get into the cabin. Book it in this week. A suddenly louder exhaust usually means a joint has leaked."
        );
        assert!(apply(lead, &statement, card, card).is_none());
        assert!(
            apply(
                lead,
                &yesno,
                card,
                "Same as before: A suddenly louder exhaust usually means a joint has leaked."
            )
            .is_none()
        );
    }

    #[test]
    fn passport_time_prefers_the_length_about_validity_over_the_trip() {
        assert!(too_little_passport_time(
            "can i fly if my trip is 10 days and my passport has two months left"
        ));
        assert!(!too_little_passport_time(
            "can i fly if my trip is 10 days and my passport has eight months left"
        ));
    }
}
