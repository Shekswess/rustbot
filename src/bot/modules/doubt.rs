//! A question about the bot's own answer or a word with two possible subjects.

use crate::bot::{Memory, Reading, compose, memory};
use crate::text::is_space;
use regex::Regex;
use std::sync::LazyLock;

#[path = "doubt_data.rs"]
mod data;
#[cfg(test)]
#[path = "doubt_tests.rs"]
mod tests;

static DOUBTS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::DOUBTS_THE_ANSWER).expect("answer doubt pattern"));
static SAYS_WRONG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::SAYS_THE_ANSWER_IS_WRONG).expect("wrong answer pattern"));
static SENSE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bsense\b").unwrap());

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(word) = reading.context_fields.subject_unclear {
        let index = reading.raw.chars().fold(0, |index, ch| {
            (index + ch as usize) % data::SUBJECT_UNCLEAR.len()
        });
        return Some(data::SUBJECT_UNCLEAR[index].replace("{word}", word));
    }
    let last = memory.turns.last()?;
    if last.bot.trim_matches(is_space).chars().count() < 40 {
        return None;
    }
    let line = reading
        .raw
        .to_lowercase()
        .replace('’', "'")
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if line.split_whitespace().count() > 9 {
        return None;
    }
    if !DOUBTS.find_iter(&line).any(|hit| {
        line[hit.end()..]
            .trim_matches([' ', '.', ',', '!', '?', ';', ':', '-'])
            .split_whitespace()
            .all(|word| data::DOUBT_TAIL.contains(&word))
    }) {
        return None;
    }
    if SAYS_WRONG.is_match(&line) && !SENSE.is_match(&line) {
        return Some(compose::pick(data::DOUBTING, memory, &line).to_owned());
    }
    if memory::meant::respond(reading, memory).is_some_and(|before| {
        let before = before.to_lowercase();
        data::MEANT_LEADS
            .iter()
            .any(|lead| before.starts_with(lead))
    }) {
        return None;
    }
    Some(compose::pick(data::UNCLEAR_ANSWER, memory, &line).to_owned())
}
