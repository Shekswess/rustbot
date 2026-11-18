//! Distinguish an availability check from a question with its own subject.
#[path = "presence_data.rs"]
mod data;
use super::kinds::Intent;
use data::*;
fn has(words: &[&str], s: &str) -> bool {
    words.binary_search(&s).is_ok()
}
fn any(words: &[&str], set: &[&str]) -> bool {
    words.iter().any(|w| has(set, w))
}
fn manner(norm: &[&str], at: usize) -> bool {
    let w = norm[at];
    if w.len() <= 4 || !w.ends_with("ly") || has(NOT_MANNER, w) {
        return false;
    }
    if has(MANNER_TALK, w) {
        return true;
    }
    let Some(mut i) = at.checked_sub(1) else {
        return false;
    };
    if has(MORE_WORDS, norm[i]) {
        let Some(before) = i.checked_sub(1) else {
            return false;
        };
        i = before;
    }
    has(TALK_VERBS, norm[i])
}
fn only(norm: &[&str], sets: &[&[&str]]) -> bool {
    norm.iter().enumerate().all(|(i, w)| {
        manner(norm, i)
            || has(MORE_WORDS, w) && i + 1 < norm.len() && manner(norm, i + 1)
            || sets.iter().any(|s| has(s, w))
    })
}
fn moment(mut norm: &[&str]) -> bool {
    if norm.is_empty() || norm.len() > 9 {
        return false;
    }
    let bare = norm.join(" ");
    for phrase in ASK_PERMISSION {
        if !bare.contains(phrase) {
            continue;
        }
        let joined = bare.replace(phrase, " ");
        let mut rest = Vec::new();
        if let Some(verb) = phrase
            .split_whitespace()
            .last()
            .filter(|v| has(TALK_VERBS, v))
        {
            rest.push(verb);
        }
        rest.extend(joined.split_whitespace());
        if only(
            &rest,
            &[MOMENT_FILLER, MOMENT_WORDS, MOMENT_VERBS, PERMISSION_FILLER],
        ) {
            return true;
        }
    }
    while norm.first().is_some_and(|w| has(PRESENCE_GREETING, w)) {
        norm = &norm[1..]
    }
    if norm.len() > 2 && has(GREETING_PHRASE, &norm[..2].join(" ")) {
        norm = &norm[2..]
    }
    if norm.is_empty() {
        return false;
    }
    if any(norm, ASK_FREE_WORDS) && (any(norm, PRESENCE_SUBJECT) || norm.len() <= 2) {
        return only(norm, &[ASK_FREE_FILLER, ASK_FREE_WORDS]);
    }
    if norm.join(" ").contains("the time")
        && !norm.iter().any(|w| *w != "time" && has(MOMENT_WORDS, w))
        && !any(norm, TALK_VERBS)
    {
        return false;
    }
    if any(norm, MOMENT_WORDS) && any(norm, MOMENT_VERBS) {
        return only(
            norm,
            &[
                MOMENT_FILLER,
                MOMENT_WORDS,
                MOMENT_VERBS,
                &["u", "ya", "you"],
            ],
        );
    }
    if norm.iter().any(|w| matches!(*w, "time" | "moment"))
        && norm
            .iter()
            .all(|w| has(TALK_VERBS, w) || has(GOOD_TIME_WORDS, w))
    {
        return true;
    }
    norm.len() <= 6 && any(norm, TALK_VERBS) && only(norm, &[TALK_FILLER, TALK_VERBS])
}
pub(super) fn reads(norm: &[String], intent: Intent) -> bool {
    let words: Vec<_> = norm
        .iter()
        .map(String::as_str)
        .filter(|w| !w.is_empty() && *w != "|")
        .collect();
    let rest = if words.first().is_some_and(|w| has(PRESENCE_GREETING, w)) {
        &words[1..]
    } else {
        &words
    };
    let addressed = any(rest, PRESENCE_SUBJECT) || any(rest, PRESENCE_ASKER);
    let there = rest.len() <= 5
        && any(rest, PRESENCE_WORD)
        && (addressed || rest.len() <= 2)
        && rest.iter().all(|w| {
            [
                PRESENCE_WORD,
                PRESENCE_SUBJECT,
                PRESENCE_ASKER,
                PRESENCE_FILLER,
            ]
            .iter()
            .any(|s| has(s, w))
        });
    moment(&words)
        || there
            && (addressed
                || !matches!(
                    intent,
                    Intent::Greeting | Intent::Thanks | Intent::Farewell | Intent::Empty
                ))
}
