//! Match ingredient choices and answer the use the line names first.

use crate::bot::Reading;
use regex::Regex;
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use super::{PICK_NOT_A_CHOICE_PATTERN, PICK_POTATO_CHOICE_PATTERN, PICK_POTATO_QUALITY_PATTERN};

static PATTERNS: LazyLock<Mutex<HashMap<&'static str, Regex>>> = LazyLock::new(Mutex::default);

fn matches(pattern: &'static str, line: &str) -> bool {
    PATTERNS
        .lock()
        .expect("ingredient pattern cache")
        .entry(pattern)
        .or_insert_with(|| Regex::new(pattern).expect("valid generated ingredient pattern"))
        .is_match(line)
}

fn line(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    let words = if line.plain.is_empty() {
        &line.clean
    } else {
        &line.plain
    };
    format!(" {words} ")
}

pub(super) fn chooses(reading: &Reading<'_>, ingredients: &'static str) -> bool {
    let text = line(reading);
    !matches(PICK_NOT_A_CHOICE_PATTERN, &text) && matches(ingredients, &text)
}

pub(super) fn chooses_potato(reading: &Reading<'_>) -> bool {
    chooses(reading, PICK_POTATO_CHOICE_PATTERN)
        || (matches(PICK_POTATO_QUALITY_PATTERN, reading.folded())
            && matches(r"\bpotato", reading.folded()))
}

pub(super) fn answer(
    reading: &Reading<'_>,
    choices: &'static [(&'static str, &'static str)],
    default: &'static str,
) -> &'static str {
    let text = line(reading);
    choices
        .iter()
        .find_map(|&(pattern, reply)| matches(pattern, &text).then_some(reply))
        .unwrap_or(default)
}
