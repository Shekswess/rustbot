//! A response to a rough patch concerning someone named by the speaker.
use super::{fallback, routes};
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};
macro_rules! pattern {
    ($name:ident,$source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}
#[path = "theirperson_data.rs"]
mod data;
use data::*;

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new("[a-z0-9'?!]+").unwrap());
static SUBJECT: LazyLock<Regex> = LazyLock::new(|| Regex::new("[a-z']+").unwrap());
static GROUP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:are|were|have|have\s+been|were\s+being)\b").unwrap());
static MAY_HAVE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:could|might|may|must)\s+have\b").unwrap());
static ALONGSIDE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:and|&)\s+(?:i|me)\b").unwrap());

pub(crate) fn reply_for(text: &str, last: Option<&str>) -> Option<String> {
    let low = text.to_lowercase();
    let said = WORD
        .find_iter(&low)
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let padded = format!(" {said} ");
    if said.is_empty() || STANDS_DOWN.is_match(&padded).unwrap_or(false) {
        return None;
    }
    let person = A_PERSON_OF_THEIRS.find(&padded).ok()??;
    let tail = &padded[person.end()..];
    let condition = A_CONDITION.is_match(tail).unwrap_or(false);
    if condition && MAY_HAVE.is_match(tail) || ALONGSIDE.is_match(tail) {
        return None;
    }
    let who = person.as_str().split_once(' ')?.1;
    let mut templates = Vec::new();
    if condition {
        templates.extend_from_slice(if A_WONDERING.is_match(tail).unwrap_or(false) {
            A_CONDITION_WONDERED
        } else {
            A_CONDITION_STATED
        });
    } else {
        if ![&*A_ROUGH_PATCH, &*A_NOT_COPING, &*A_LOW]
            .iter()
            .any(|p| p.is_match(tail).unwrap_or(false))
        {
            return None;
        }
        templates.extend_from_slice(POOL);
        if !GROUP.is_match(tail) {
            templates.extend_from_slice(NAMED)
        }
    }
    let mut worded: Vec<_> = templates
        .into_iter()
        .map(|s| s.replace("{who}", who))
        .collect();
    if worded.len() > 1
        && let Some(previous) = last
    {
        worded.retain(|line| line != previous);
    }
    let index = super::index(said.chars().map(|c| c as u64).sum(), worded.len());
    Some(worded.swap_remove(index))
}

fn words(text: &str) -> HashSet<&str> {
    SUBJECT
        .find_iter(text)
        .map(|m| m.as_str())
        .filter(|w| w.len() > 2 && FILLER.binary_search(w).is_err())
        .collect()
}

pub(crate) fn may_replace(reply: &str, text: &str) -> bool {
    if reply.trim().is_empty() {
        return false;
    }
    if routes::hedge(reply) || fallback::is_neutral(reply) {
        return true;
    }
    let text = text.to_lowercase();
    let reply = reply.to_lowercase();
    let mine = words(&text);
    let theirs = words(&reply);
    let shared = mine.intersection(&theirs).count();
    shared >= 2 && shared * 5 >= mine.len() * 3
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let reply = reply_for(reading.raw, memory.turns.last().map(|t| t.bot.as_str()))?;
    let fallback = fallback::respond(reading, memory)?;
    may_replace(&fallback, reading.raw).then_some(reply)
}

#[cfg(test)]
mod tests {
    use super::*;
    mod cases {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/theirperson.rs"
        ));
    }
    #[test]
    fn reads_the_relative_and_their_condition() {
        for &(prompt, previous, expected) in cases::CASES {
            assert_eq!(
                reply_for(prompt, Some(previous)).as_deref(),
                (expected != 0).then(|| cases::POOL[expected]),
                "{prompt:?} after {previous:?}"
            );
        }
    }
    #[test]
    fn preserves_replies_that_address_the_line() {
        for &(prompt, reply, expected) in cases::REPLACE {
            assert_eq!(
                may_replace(reply, prompt),
                expected,
                "{prompt:?} -> {reply:?}"
            );
        }
    }
}
