//! Acknowledge a hard event the person describes as past.

use crate::bot::{Memory, Reading, compose, understanding::kinds::Kind};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

#[path = "hardpast_data.rs"]
mod data;
#[cfg(test)]
#[path = "hardpast_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(A_HARD_PAST, A_THE_PAST, A_NEWS_ARRIVING, A_CLOSED, A_LOSS);
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static A_OWNED: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| data::A_OWNED.split_ascii_whitespace().collect());
static A_BETWEEN: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| data::A_BETWEEN.split_ascii_whitespace().collect());

fn personal(before: &str) -> bool {
    let words: Vec<_> = WORD.find_iter(before).map(|word| word.as_str()).collect();
    if words.iter().rev().take(4).any(|word| {
        A_OWNED.contains(word)
            || word
                .strip_suffix("'s")
                .is_some_and(|base| A_OWNED.contains(base))
    }) {
        return true;
    }
    let preceding: Vec<_> = words
        .into_iter()
        .rev()
        .take(6)
        .map(|word| word.replace('\'', ""))
        .collect();
    preceding
        .iter()
        .position(|word| matches!(word.as_str(), "i" | "we" | "ive" | "weve"))
        .is_some_and(|at| {
            preceding[..at]
                .iter()
                .all(|word| A_BETWEEN.contains(word.as_str()))
        })
}

fn qualifying_line(reading: &Reading<'_>) -> Option<String> {
    if reading.kind == Kind::Question || reading.raw.contains('?') {
        return None;
    }
    let clean = reading
        .raw
        .to_lowercase()
        .replace('’', "'")
        .split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if clean.is_empty()
        || A_NEWS_ARRIVING.is_match(&clean)
        || !A_THE_PAST.is_match(&clean)
        || A_LOSS.is_match(&clean)
        || A_CLOSED.is_match(&clean)
    {
        return None;
    }
    A_HARD_PAST
        .find_iter(&clean)
        .any(|found| personal(&clean[..found.start()]))
        .then_some(clean)
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = qualifying_line(reading)?;
    Some(compose::pick(data::A_HARD_PAST_SAID, memory, &clean).to_string())
}
