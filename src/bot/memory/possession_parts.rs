//! Ask back for an omitted property of a possession the person put on record.

use super::possessions;
use crate::{
    bot::{Memory, Reading},
    text::is_space,
};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! rule {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}
include!("possession_parts_data.rs");
rule!(FIRST_PERSON, r"\b(?:i|we|my|our|im|ive)\b");
rule!(I_AM, r"\b(?:i|im)\b");
rule!(DIGIT, r"\d");
rule!(WHAT_IS_THE, r"\bwhat is the\b");
rule!(IT_IS, r"\bit is\b");
rule!(YOU_ARE, r"\byou are\b");
rule!(WHERE_IS_THE, r"\bwhere is the\b");

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.line.norm_text.replace(" | ", " ").to_lowercase();
    let line = line
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let folded = CHAT_TAIL.replace_all(&line, "");
    let low = folded.trim_matches([' ', '.', ',', ';', ':', '!', '?']);
    if low.split_whitespace().count() > 8
        || PART_ADVICE.is_match(low)
        || PART_HELP.is_match(low)
        || DIGIT.is_match(low)
        || !PART_ASK.is_match(low)
        || !RECORD_PART.is_match(low)
        || !memory.possessions.has_any()
    {
        return None;
    }
    if let Some(possessive) = PART_MINE.captures(low) {
        let noun = possessive.get(1)?.as_str();
        let at = PART_HELD
            .binary_search_by_key(&noun, |&(key, _)| key)
            .ok()?;
        if PART_HELD[at].1 != "owns" {
            return None;
        }
    } else if !PART_POINTS.is_match(low) && !(I_AM.is_match(low) && PART_OWNS.is_match(low)) {
        return None;
    }
    if !memory.turns.iter().rev().take(5).any(|turn| {
        let user = turn.user.trim_matches(is_space);
        !user.ends_with('?')
            && FIRST_PERSON.is_match(&user.to_lowercase())
            && possessions::has_fact(user)
    }) {
        return None;
    }
    let mut text = low
        .split_whitespace()
        .map(|word| {
            TURNED
                .binary_search_by_key(&word, |&(key, _)| key)
                .map_or(word, |index| TURNED[index].1)
        })
        .collect::<Vec<_>>()
        .join(" ");
    for (pattern, replacement) in [
        (&WHAT_IS_THE, "what's the"),
        (&IT_IS, "it's"),
        (&YOU_ARE, "you're"),
        (&WHERE_IS_THE, "where's the"),
    ] {
        if let std::borrow::Cow::Owned(turned) = pattern.replace_all(&text, replacement) {
            text = turned;
        }
    }
    (!text.is_empty()).then(|| format!("You haven't said - {text}?"))
}
