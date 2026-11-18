//! Culinary quantities for the first seasoning named in the source question.

use super::{SEASONING_AMOUNTS, SEASONING_LATTE, SEASONING_MULLED_WINE, SEASONING_NOT_A_DOSE};
use crate::bot::Reading;
use regex::Regex;
use std::{cmp::Reverse, sync::LazyLock};

static SEPARATORS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[^a-z0-9 ]+").expect("seasoning words retain letters and spaces")
});

pub(super) fn amount(reading: &Reading<'_>) -> Option<&'static str> {
    let plain = reading.line.clean.to_lowercase();
    let said = format!(" {} ", SEPARATORS.replace_all(&plain, " "));
    if SEASONING_NOT_A_DOSE
        .iter()
        .any(|phrase| phrase_start(&said, phrase).is_some())
    {
        return None;
    }
    if said.contains("mulled wine") {
        return Some(SEASONING_MULLED_WINE);
    }
    if said.contains("pumpkin spice")
        && ["latte", "coffee", "cappuccino", "mug", "cup"]
            .into_iter()
            .any(|drink| said.contains(drink))
    {
        return Some(SEASONING_LATTE);
    }
    SEASONING_AMOUNTS
        .iter()
        .flat_map(|&(names, reply)| names.iter().map(move |name| (*name, reply)))
        .filter_map(|(name, reply)| {
            Some(((phrase_start(&said, name)?, Reverse(name.len())), reply))
        })
        .min_by_key(|(rank, _)| *rank)
        .map(|(_, reply)| reply)
}

fn phrase_start(said: &str, phrase: &str) -> Option<usize> {
    let bytes = said.as_bytes();
    said.match_indices(phrase).find_map(|(start, _)| {
        (start > 0 && bytes[start - 1] == b' ' && bytes.get(start + phrase.len()) == Some(&b' '))
            .then_some(start)
    })
}

#[cfg(test)]
#[test]
fn prefers_the_earliest_then_longest_seasoning_and_leaves_dosage_to_other_cards() {
    for (prompt, cue) in [
        ("How much cumin in a chilli?", "A teaspoon of ground cumin"),
        ("How much chilli powder for four?", "Half to one teaspoon"),
        (
            "How much fresh coriander to garnish?",
            "A small handful of fresh coriander",
        ),
        (
            "How much cinnamon should I put in mulled wine?",
            "Four to six whole cloves",
        ),
        (
            "How much pumpkin spice in a latte?",
            "A quarter to half a teaspoon",
        ),
    ] {
        assert!(
            amount(&Reading::new(prompt)).is_some_and(|reply| reply.starts_with(cue)),
            "{prompt}"
        );
    }
    assert!(amount(&Reading::new("How much turmeric is too much a day?")).is_none());
}
