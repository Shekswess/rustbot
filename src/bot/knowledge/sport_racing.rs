//! The pinned sport domain's race readers, recovery advice and start-time replies.
#[path = "sport_racing_data.rs"]
mod data;
#[cfg(test)]
#[path = "sport_racing_tests.rs"]
mod tests;

use crate::bot::{Exchange, Reading};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    Recovery,
    Start,
}

static SHORTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::A_SHORTER_RACE).expect("source shorter-race pattern"));
static FULL: LazyLock<Fancy> =
    LazyLock::new(|| Fancy::new(data::THE_FULL_RACE).expect("source full-race pattern"));

fn contains_any(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

pub(super) fn half_race(reading: &Reading<'_>) -> bool {
    let line = format!(
        "{} {} {}",
        reading.line.clean, reading.line.norm_text, reading.line.text
    )
    .to_lowercase()
    .replace('-', " ");
    let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
    if line.contains("half marathon") {
        return true;
    }
    let padded = format!(" {line} ");
    !data::CLOCK_HALF
        .iter()
        .any(|hour| padded.contains(&format!(" {hour} ")))
}

fn race_words(reading: &Reading<'_>) -> String {
    reading.line.words.join(" ").replace('-', " ")
}

fn has_half(reading: &Reading<'_>) -> bool {
    reading
        .line
        .words
        .iter()
        .any(|word| matches!(word.as_str(), "half" | "13.1" | "21.1"))
}

pub(super) fn full_distance(reading: &Reading<'_>) -> bool {
    if !has_half(reading) {
        return true;
    }
    let said = race_words(reading);
    if SHORTER.is_match(&said) && !FULL.is_match(&said).unwrap_or(false) {
        return false;
    }
    contains_any(&said, data::COMPARED)
}

pub(super) fn compare_full(reading: &Reading<'_>) -> bool {
    has_half(reading) && !SHORTER.is_match(&race_words(reading))
}

pub(super) fn compare_shorter(reading: &Reading<'_>) -> bool {
    let said = race_words(reading);
    SHORTER.is_match(&said) && !FULL.is_match(&said).unwrap_or(false)
}

fn distance_line(reading: &Reading<'_>) -> String {
    let text = if reading.line.plain.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.plain
    };
    text.to_lowercase()
}

fn recovery_for(line: &str) -> Option<&'static str> {
    data::RECOVERY_BY_DISTANCE
        .iter()
        .find_map(|(phrases, reply)| contains_any(line, phrases).then_some(*reply))
}

pub(super) fn named_distance(reading: &Reading<'_>) -> bool {
    recovery_for(&distance_line(reading)).is_some()
}

fn recovery(reading: &Reading<'_>, earlier: &[Exchange]) -> &'static str {
    recovery_for(&distance_line(reading))
        .or_else(|| {
            earlier
                .iter()
                .rev()
                .take(data::RECOVERY_LOOKBACK)
                .find_map(|turn| recovery_for(&turn.user.to_lowercase()))
        })
        .unwrap_or(data::RECOVERY_GENERAL)
}

fn start(reading: &Reading<'_>) -> Option<&'static str> {
    let text = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    let text = text.to_lowercase();
    let mut replies = data::SCHEDULES
        .iter()
        .filter_map(|(phrases, reply)| contains_any(&text, phrases).then_some(*reply));
    let first = replies.next()?;
    Some(if replies.next().is_some() {
        data::EVENT_START_GENERIC
    } else {
        first
    })
}

pub(super) fn reply(
    form: Reply,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let reply = match form {
        Reply::Recovery => recovery(reading, earlier),
        Reply::Start => start(reading)?,
    };
    Some(Cow::Borrowed(reply))
}
