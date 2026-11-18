//! An in-context duration or recommendation for an announced house guest.

use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::sync::LazyLock;

#[path = "how_long_a_stay_data.rs"]
mod data;
#[cfg(test)]
#[path = "how_long_a_stay_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(STAY, A_LENGTH, A_LENGTH_SAID, ASKS_A_LENGTH, ASKS_ADVICE);
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9']+").unwrap());
static SENTENCE_END: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[.!?](?:\s|$)").unwrap());

fn names_another_subject(text: &str) -> bool {
    WORD.find_iter(&text.to_lowercase()).any(|word| {
        let word = word.as_str();
        word.len() > 2
            && !data::GENERIC.contains(&word)
            && !word.bytes().all(|ch| ch.is_ascii_digit())
    })
}

fn length(stay: &str) -> Option<String> {
    let said = A_LENGTH.captures(stay)?;
    Some(match said.get(1) {
        Some(count) => format!("{} {}", count.as_str(), &said[2]).to_lowercase(),
        None => said[0].trim().to_lowercase(),
    })
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let raw = reading.raw;
    if !ASKS_A_LENGTH.is_match(raw) || names_another_subject(raw) {
        return None;
    }
    let stay = memory
        .turns
        .iter()
        .rev()
        .find(|turn| STAY.is_match(&turn.user))?;
    let last = memory
        .turns
        .iter()
        .rev()
        .find(|turn| !turn.bot.is_empty())?;
    let answer = &last.bot;
    let opening = SENTENCE_END
        .find(answer)
        .map_or(answer.as_str(), |end| &answer[..end.end()]);
    if last.module != "answers" || A_LENGTH_SAID.is_match(opening) {
        return None;
    }
    if let Some(length) = length(&stay.user) {
        let answer =
            compose::pick(data::A_LENGTH_REPLY, memory, &length).replace("{length}", &length);
        let mut chars = answer.chars();
        return Some(chars.next()?.to_uppercase().chain(chars).collect());
    }
    let pool = if ASKS_ADVICE.is_match(raw) {
        data::ADVICE
    } else {
        data::NO_LENGTH
    };
    Some(compose::pick(pool, memory, &stay.user).to_owned())
}
