//! Weekend, evening and rainy-day questions from PyBot's entertainment domain.

use super::{Form, Key, city_guides, entertainment::plain_words};
use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "entertainment_time_data.rs"]
mod data;
#[cfg(test)]
#[path = "entertainment_time_tests.rs"]
mod tests;

static SHORT_TIME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::A_SHORT_TIME).expect("spare hour pattern"));

fn typed_line(reading: &Reading<'_>) -> String {
    let line = if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    };
    format!(" {} ", plain_words(line))
}

pub(super) fn names_weather(reading: &Reading<'_>) -> bool {
    let line = typed_line(reading);
    data::WET_WORDS
        .iter()
        .any(|word| line.contains(&format!(" {word} ")))
        || data::WET_PHRASES.iter().any(|phrase| line.contains(phrase))
}

pub(super) fn names_evening(reading: &Reading<'_>) -> bool {
    let line = typed_line(reading);
    data::EVENING_WORDS
        .iter()
        .any(|phrase| line.contains(&format!(" {phrase} ")))
}

pub(super) fn no_city(reading: &Reading<'_>) -> bool {
    let line = reading.line.clean.to_lowercase().replace('-', " ");
    !city_guides::guide_city_named_except(&line, "bath")
}

pub(super) fn weekend_rank_deduction(
    groups: Key<'_>,
    source: &[&[&str]],
    line: &[Vec<Form>],
    words: &[String],
) -> Option<i32> {
    // PyBot discards rank-one rows. Its rank uses the first highest-weight
    // authored phrase in each part, with a subject phrase ahead of an ask
    // frame at the same weight.
    const FRAME: &[&str] = &["should i do", "can i do", "shall i do", "what do i do"];
    const TIME_ONLY: &[&str] = &[
        "shall we do",
        "weekend",
        "weekends",
        "this weekend",
        "saturday",
        "sunday",
        "sundays",
        "this saturday",
        "this sunday",
    ];
    const QUESTION: &[&str] = &[
        "what", "whats", "which", "how", "where", "when", "who", "why", "should", "shall", "could",
        "would", "can",
    ];
    let tail = words
        .iter()
        .enumerate()
        .skip(4)
        .rfind(|(_, word)| QUESTION.contains(&word.as_str()))
        .map(|(index, _)| index);
    let best = groups.iter().zip(source).filter_map(|(group, &authored)| {
        group
            .iter()
            .zip(authored)
            .enumerate()
            .filter_map(|(index, (phrase, &wording))| {
                let (start, end) = phrase.spans(line).next()?;
                Some((index, wording, start, end))
            })
            .max_by_key(|(index, wording, start, end)| {
                (
                    end - start > 1,
                    !FRAME.contains(wording),
                    std::cmp::Reverse(*index),
                )
            })
    });
    let mut has_subject = false;
    let mut said = 0;
    let mut asking = 0;
    for (_, wording, start, end) in best {
        let frame = FRAME.contains(&wording);
        has_subject |= !frame && !TIME_ONLY.contains(&wording);
        let weight = if end - start > 1 { 3 } else { 2 };
        if frame && tail.is_some_and(|tail| start >= tail) {
            asking += weight;
        } else {
            said += weight;
        }
    }
    has_subject.then_some(if said > 2 { asking } else { 0 })
}

pub(super) fn spare_time(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    Some(Cow::Borrowed(
        if SHORT_TIME.is_match(&line.to_lowercase()) {
            data::AN_HOUR
        } else {
            data::A_DAY
        },
    ))
}
