//! Identify preferences before reading the tone of a statement.

use super::{has, matches, split};
use crate::bot::memory::{FoodAck, food_ack};

#[path = "trouble_taste_data.rs"]
mod data;
use data::*;

fn named_thing(text: &str) -> Option<String> {
    let mut kept = Vec::new();
    for word in split(text) {
        let word = word.trim_matches(['.', ',', ';', ':', '!', '?']);
        if word.is_empty() {
            continue;
        }
        if !kept.is_empty()
            && (has(THING_STOP, word)
                || matches!(word, "my" | "our" | "his" | "her" | "their" | "your"))
        {
            break;
        }
        if kept.is_empty() && has(TAIL, word) {
            continue;
        }
        kept.push(word);
    }
    while kept.last().is_some_and(|word| has(TAIL, word)) {
        kept.pop();
    }
    let first = kept.first_mut()?;
    if has(NOT_A_THING, first) {
        return None;
    }
    if matches!(*first, "my" | "our") {
        *first = "your";
    }
    Some(kept.join(" "))
}

fn tastes_in(clause: &str) -> bool {
    if clause.is_empty() || clause.ends_with('?') {
        return false;
    }
    for pattern in [&UNSURE, &NO, &NOT_KEEN, &MY_THING, &YES] {
        let Some(thing) = pattern
            .captures(clause)
            .ok()
            .flatten()
            .and_then(|parts| parts.get(1))
        else {
            continue;
        };
        let raw = thing.as_str();
        if split(raw).next() == Some("to") {
            return false;
        }
        if let Some(thing_name) = named_thing(raw)
            && !matches(&QUANTITY, &thing_name)
            && !matches(&QUANTITY, raw.trim())
            && !(matches(&SHORTFALL_VERB, &clause[..thing.start()])
                && matches(&SHORTFALL_TAIL, raw))
        {
            return true;
        }
    }
    HATE.captures(clause)
        .ok()
        .flatten()
        .and_then(|parts| parts.get(1))
        .is_some_and(|thing| {
            !split(thing.as_str())
                .next()
                .is_some_and(|word| matches!(word, "my" | "our"))
                && named_thing(thing.as_str()).is_some()
        })
}

pub(super) fn only_dislike(line: &str) -> bool {
    let body = DISLIKE_IN.replace(line, " ");
    !matches(&TROUBLE_IN_TELLING, &body)
        && CLAUSE_SPLIT.split(line).flatten().any(|phrase| {
            let phrase = phrase.trim();
            if matches(&TASTE_VERB, phrase)
                && !split(phrase)
                    .next()
                    .is_some_and(|word| matches!(word, "i" | "we" | "he" | "she" | "they"))
            {
                return tastes_in(&format!("i {phrase}"));
            }
            tastes_in(phrase)
        })
        && !matches!(food_ack(line), Some(FoodAck::Diet(_)))
}
