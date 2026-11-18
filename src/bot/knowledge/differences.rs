//! Pinned side-by-side explanations for recognized pairs.

use crate::bot::Reading;

use super::{DIFFERENCE_FILLER, DIFFERENCE_STRONG_ASKS, DIFFERENCES};

fn mentions(line: &str, names: &[&str]) -> bool {
    names.iter().any(|name| {
        line.match_indices(name).any(|(start, found)| {
            let before = &line[..start];
            let after = &line[start + found.len()..];
            !before.ends_with(|c: char| c.is_ascii_lowercase())
                && !after.starts_with(|c: char| c.is_ascii_lowercase())
        })
    })
}

fn asks_about_pair(line: &str, raw: &str, left: &[&str], right: &[&str]) -> bool {
    if DIFFERENCE_STRONG_ASKS
        .iter()
        .any(|shape| line.contains(shape))
        || line.contains("which")
        || raw.contains('?')
    {
        return true;
    }
    let asks_or = raw.contains("or");
    if !asks_or && mentions(line, &["i", "im", "ive", "me", "my", "we", "our", "us"]) {
        return false;
    }
    let mut words = raw
        .split(|c: char| !c.is_ascii_lowercase() && c != '\'')
        .filter(|word| !word.is_empty() && !DIFFERENCE_FILLER.contains(word));
    if asks_or {
        return words.take(6).count() <= 5;
    }
    words.all(|word| {
        left.iter()
            .chain(right)
            .flat_map(|name| name.split_whitespace())
            .any(|name| name == word)
    })
}

pub(super) fn answer(reading: &Reading<'_>) -> Option<&'static str> {
    let raw = reading.raw.to_lowercase();
    let clean = &reading.line.clean;
    let line = if clean.is_empty() { &raw } else { clean };
    DIFFERENCES.iter().find_map(|&(left, right, reply)| {
        (mentions(line, left) && mentions(line, right) && asks_about_pair(line, &raw, left, right))
            .then_some(reply)
    })
}
