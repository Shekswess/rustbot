//! Read the place an answer is kept from the user's preceding subject.

use crate::bot::{Memory, Reading, compose};
use regex::{Regex, RegexSet};
use std::{collections::HashSet, sync::LazyLock};

#[path = "the_kind_one_line_up_data.rs"]
mod data;
#[cfg(test)]
#[path = "the_kind_one_line_up_tests.rs"]
mod tests;

static LEANING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::LEANING).expect("follow-up openers"));
static A_METHOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::A_METHOD).expect("method questions"));
static BAGGAGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::BAGGAGE).expect("baggage vocabulary"));
static PLACE: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::PLACE_POOLS.iter().map(|(pattern, _)| pattern))
        .expect("where an answer is kept")
});
static ASKING: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| data::ASKING.split_ascii_whitespace().collect());

fn clean(text: &str) -> String {
    text.to_lowercase()
        .split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn without_plurals(line: &str) -> String {
    line.split_ascii_whitespace()
        .map(|word| {
            if word.chars().take(4).count() > 3
                && !["ss", "us", "is"]
                    .iter()
                    .any(|suffix| word.ends_with(suffix))
            {
                word.strip_suffix('s').unwrap_or(word)
            } else {
                word
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(in crate::bot) fn place_pool(line: &str) -> Option<&'static [&'static str]> {
    PLACE
        .matches(format!(" {line} ").as_str())
        .into_iter()
        .next()
        .map(|index| data::PLACE_POOLS[index].1)
}

fn earlier_pool(earlier: &str, line: &str) -> Option<&'static [&'static str]> {
    if earlier.is_empty() || earlier == line {
        return None;
    }
    let without_plurals = without_plurals(earlier);
    for said in [earlier, without_plurals.as_str()] {
        if let Some(pool) = place_pool(said) {
            if pool
                .iter()
                .all(|reply| !reply.to_lowercase().contains("council"))
            {
                return Some(pool);
            }
        } else if BAGGAGE.is_match(said) {
            return Some(data::BAGGAGE_POOL);
        }
    }
    None
}

fn nearby_pool(line: &str, memory: &Memory) -> Option<&'static [&'static str]> {
    if line.is_empty() || place_pool(line).is_some() {
        return None;
    }
    if BAGGAGE.is_match(line) {
        return Some(data::BAGGAGE_POOL);
    }
    if !LEANING.is_match(line) || A_METHOD.is_match(line) {
        return None;
    }
    let words = line
        .split(|ch: char| !ch.is_ascii_lowercase() && ch != '\'')
        .filter(|word| !word.is_empty());
    for (index, word) in words.enumerate() {
        if index >= 11 || !ASKING.contains(word) {
            return None;
        }
    }
    let mut past = memory.turns.iter().rev();
    let previous = clean(&past.next()?.user);
    if let Some(pool) = earlier_pool(&previous, line) {
        return Some(pool);
    }
    if LEANING.is_match(&previous) {
        return earlier_pool(&clean(&past.next()?.user), line);
    }
    None
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !matches!(memory.turns.last()?.module, "greeting" | "chat") {
        return None;
    }
    let pool = nearby_pool(&clean(reading.raw), memory)?;
    Some(compose::choose_recent(pool, memory, reading.raw).to_string())
}
