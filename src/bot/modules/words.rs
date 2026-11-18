//! Hand back the drafted words already quoted in the previous answer.

use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::sync::LazyLock;

#[path = "words_data.rs"]
mod data;
#[cfg(test)]
#[path = "words_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(ASKS_WORDS, EXAMPLED, ASK_OBJECT, ASK_FILLER);
static QUOTED: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(data::QUOTED).expect("drafted quote pattern"));

fn quoted(reply: &str) -> Option<&str> {
    for found in QUOTED.captures_iter(reply).filter_map(Result::ok) {
        let matched = found.get(0)?;
        let words = (1..=4).find_map(|index| found.get(index))?.as_str().trim();
        if words.split_whitespace().take(3).count() < 3 {
            continue;
        }
        let start = reply[..matched.start()]
            .char_indices()
            .rev()
            .nth(17)
            .map_or(0, |(start, _)| start);
        if EXAMPLED.is_match(&reply[start..matched.start()])
            && !words.ends_with(['?', '!', '.'])
            && !words.contains(',')
        {
            continue;
        }
        return Some(matched.as_str().trim());
    }
    None
}

fn names_its_own(mut rest: &str) -> bool {
    rest = rest.trim();
    for _ in 0..2 {
        rest = rest.trim_matches([' ', ',', '.', '-']);
        if let Some(object) = ASK_OBJECT.find(rest) {
            rest = rest[object.end()..].trim();
        }
        for _ in 0..3 {
            let Some(filler) = ASK_FILLER.find(rest) else {
                break;
            };
            rest = rest[filler.end()..].trim();
        }
    }
    rest.bytes().any(|ch| ch.is_ascii_alphanumeric())
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.raw.trim();
    if line.split_whitespace().take(15).count() > 14 {
        return None;
    }
    let ask = ASKS_WORDS.find(line)?;
    if names_its_own(&line[ask.end()..]) {
        return None;
    }
    let quote = quoted(&memory.turns.last()?.bot)?;
    Some(compose::pick(data::LINES, memory, line).replace("{words}", quote))
}
