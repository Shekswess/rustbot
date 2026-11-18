//! Relevance rules for travel plans that share words with household jobs and training.

use crate::bot::Reading;
use regex::Regex;
use std::sync::LazyLock;

#[path = "travel_guard_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    THE_TRIP_IN_THE_LINE,
    TRAINS_FOR,
    A_WALK_IS_SAID,
    FLAT_YOU_LIVE_IN,
    A_BAKE_OR_A_PAN,
    CAME_OUT_FLAT,
    THE_GROUND_UNDERFOOT,
);
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static FLAT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bflat\b").unwrap());

pub(crate) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    match id {
        "travel/book-a-flight" => {
            let said = if line.words.is_empty() {
                line.clean.clone()
            } else {
                line.words.join(" ")
            };
            !said.to_lowercase().split(' ').any(|word| word == "booked")
        }
        "travel/package-or-diy" => {
            if line.clean.is_empty() {
                THE_TRIP_IN_THE_LINE.is_match(&line.words.join(" "))
            } else {
                THE_TRIP_IN_THE_LINE.is_match(&line.clean)
            }
        }
        "travel/camping-kit" | "travel/pack-warm-week" => !carrying_food(reading),
        "travel/train-journey-packing" => {
            !TRAINS_FOR.is_match(&line.clean)
                && !matches!(reading.sense.get("coach"), Some(&"teacher" | &"furniture"))
        }
        "travel/an-easy-walk" => flat_ground(reading),
        _ => false,
    }
}

fn carrying_food(reading: &Reading<'_>) -> bool {
    const ACTIONS: &[&str] = &[
        "bring", "brings", "bringing", "take", "takes", "taking", "carry", "food", "eat", "make",
        "makes", "cook", "bake",
    ];
    let line = &reading.line;
    let said = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    }
    .to_lowercase();
    let mut food = false;
    let mut action = false;
    for word in WORD.find_iter(&said).map(|word| word.as_str()) {
        food |= data::FOOD_TO_CARRY.contains(&word);
        action |= ACTIONS.contains(&word);
        if food && action {
            return true;
        }
    }
    false
}

fn flat_ground(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let words;
    let said = if !line.text.is_empty() {
        &line.text
    } else if !line.clean.is_empty() {
        &line.clean
    } else {
        words = line.words.join(" ");
        &words
    };
    if FLAT.is_match(said)
        && (A_BAKE_OR_A_PAN.is_match(said)
            || (CAME_OUT_FLAT.is_match(said) && !THE_GROUND_UNDERFOOT.is_match(said)))
    {
        return false;
    }
    A_WALK_IS_SAID.is_match(said) || !FLAT_YOU_LIVE_IN.is_match(said)
}

#[cfg(test)]
#[path = "../../../tests/fixtures/travel_guards.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn travel_guards_match_pinned_source() {
        let mismatches: Vec<_> = fixtures::CASES
            .iter()
            .filter_map(|(id, prompt, expected)| {
                let actual = passes(id, &Reading::new(prompt));
                (actual != *expected).then_some((*id, *prompt, *expected, actual))
            })
            .collect();
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }
}
