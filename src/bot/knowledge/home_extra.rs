//! Household clarifications and repairs whose reply depends on the named material or fault.

use crate::bot::Reading;
use regex::RegexSet;
use std::sync::LazyLock;

#[path = "home_extra_data.rs"]
mod data;

static CHIP_MATERIAL: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::CHIP_VARIANTS.iter().map(|&(pattern, _)| pattern))
        .expect("repair material patterns")
});

pub(super) fn reply(id: &str, reading: &Reading<'_>) -> Option<&'static str> {
    let choice = match id {
        "home/dryer-not-heating" => by_word(
            reading,
            data::DRYER_HEAT_TRIGGERS,
            data::DRYER_HEAT_TRIGGERED,
            data::DRYER_HEAT_OTHER,
        ),
        "home/hair-dryer-not-hot" => by_word(
            reading,
            data::HAIR_DRYER_TRIGGERS,
            data::HAIR_DRYER_TRIGGERED,
            data::HAIR_DRYER_OTHER,
        ),
        "home/a-nail-in-a-wall" => by_word(
            reading,
            data::A_NAIL_COMES_OUT,
            data::WALL_NAIL_1,
            data::WALL_NAIL_0,
        ),
        "home/wood-splits-when-nailed" => wood_nail(reading),
        "home/paper-going-yellow" => paper(reading),
        "home/chip-in-a-surface" => chip(reading),
        "home/property-word" => return property_word(reading),
        "home/home-word-2" => return home_word(reading),
        _ => return None,
    };
    Some(choice)
}

fn by_word(
    reading: &Reading<'_>,
    words: &[&str],
    matched: &'static str,
    default: &'static str,
) -> &'static str {
    if super::words_include(reading, words) {
        matched
    } else {
        default
    }
}

fn wood_nail(reading: &Reading<'_>) -> &'static str {
    let said = reading.line.words.join(" ").to_lowercase();
    if super::contains_any(&said, data::A_SIZE_ASK)
        && !super::words_include(reading, data::A_SPLIT_ASK)
    {
        data::WOOD_NAIL_1
    } else {
        data::WOOD_NAIL_0
    }
}

fn paper(reading: &Reading<'_>) -> &'static str {
    let clean = super::clean(reading).to_lowercase();
    let said = format!(" {clean} ");
    let names = |pool: &[&str]| {
        pool.iter().any(|word| {
            said.match_indices(word).any(|(offset, found)| {
                said[..offset].ends_with(' ') && said[offset + found.len()..].starts_with(' ')
            })
        })
    };
    if names(data::PAPER_SMELL) {
        data::PAPER_1
    } else if names(data::PAPER_PHOTOS) {
        data::PAPER_2
    } else {
        data::PAPER_0
    }
}

fn chip(reading: &Reading<'_>) -> &'static str {
    let said = if reading.line.text.is_empty() {
        super::clean(reading)
    } else {
        &reading.line.text
    };
    CHIP_MATERIAL
        .matches(&said.to_lowercase())
        .into_iter()
        .next()
        .map_or(data::CHIP_OTHER, |index| data::CHIP_VARIANTS[index].1)
}

fn property_word(reading: &Reading<'_>) -> Option<&'static str> {
    let said = reading.line.words.join(" ").to_lowercase();
    let named = |word: &str| {
        (word.contains(' ') && said.contains(word))
            || reading
                .line
                .words
                .iter()
                .any(|token| token.eq_ignore_ascii_case(word))
    };
    data::PROPERTY_WORDS.iter().find_map(|&(keys, reply)| {
        keys.iter()
            .any(|&word| {
                named(word)
                    && (!data::PROPERTY_SERVICE_WORDS.contains(&word)
                        || data::PROPERTY_SERVICE_CONTEXT
                            .iter()
                            .any(|word| named(word)))
            })
            .then_some(reply)
    })
}

fn home_word(reading: &Reading<'_>) -> Option<&'static str> {
    let names = |pool: &[&str]| {
        super::contains_any(&reading.line.clean, pool) || super::words_include(reading, pool)
    };
    data::HOME_WORDS
        .iter()
        .find_map(|&(keys, reply, avoid)| (names(keys) && !names(avoid)).then_some(reply))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/home_extra.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn household_materials_and_names_match_pybot() {
        let mut mismatches = Vec::new();
        for &(id, prompt, expected) in fixtures::CASES {
            let actual = reply(id, &Reading::new(prompt));
            if actual != expected {
                mismatches.push((id, prompt, expected, actual));
            }
        }
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }
}
