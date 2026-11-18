//! Interpret a nounless question about the bot and a state whose wording depends on its owner.
#[path = "ownership_data.rs"]
mod data;
use super::{
    entities::Entity,
    kinds::{Intent, Kind},
    line::Line,
};
use data::*;
fn has(set: &[&str], s: &str) -> bool {
    set.binary_search(&s).is_ok()
}
pub(super) fn about_bot(line: &Line, intent: Intent, kind: Kind, entities: &[Entity]) -> bool {
    if intent == Intent::AboutBot {
        return true;
    }
    if kind != Kind::Question || !entities.is_empty() {
        return false;
    }
    let w = line
        .words
        .iter()
        .map(String::as_str)
        .filter(|s| *s != "|")
        .collect::<Vec<_>>();
    if w.len() < 2 {
        return false;
    }
    if w[0] == "how" && w.len() > 3 && has(SECOND_PERSON, w[2]) && has(DAY_VERBS, w[1]) {
        return w[3..].iter().any(|word| has(DAY_WORDS, word));
    }
    let meaningful = |s: &&str| !has(AFTER_BOT_FILLER, s);
    let bot_noun = w.iter().any(|s| has(BOT_NOUNS, s));
    if has(SECOND_PERSON, w[1]) {
        if has(COPULA_AUX, w[0]) {
            let first = w[2..].iter().copied().find(meaningful);
            return first.is_none_or(|s| has(BOT_STATE, s)) || bot_noun;
        }
        return has(MODAL_WORDS, w[0]) && (3..=4).contains(&w.len()) && has(BOT_STATE, w[2]);
    }
    let Some(at) = w.iter().take(4).position(|s| has(SECOND_PERSON, s)) else {
        return false;
    };
    if !has(ASKING_WORDS, w[0]) {
        return false;
    }
    if w[0] == "who" {
        return has(SECOND_PERSON, w[w.len() - 1]) || has(WHO_ENDS, w[w.len() - 1]);
    }
    if w[1..at].iter().any(|s| has(HYPOTHETICAL, s)) {
        return false;
    }
    let mut rest = w[at + 1..].iter().copied().filter(meaningful);
    let first = rest.next();
    first.is_some_and(|s| has(MEANING_WORDS, s)) && rest.all(|s| has(MEANING_TAIL, s))
        || w.len() <= 5 && (first.is_none_or(|s| has(BOT_STATE, s)) || bot_noun)
}
pub(super) fn plain(line: &str, entities: &[Entity]) -> Option<String> {
    if !BROKEN_OF_A_THING.iter().any(|word| line.contains(word)) {
        return None;
    }
    let words: Vec<_> = line.split_whitespace().collect();
    let bare = |word: &str| {
        word.trim_matches(|c| EDGE_PUNCTUATION.contains(c))
            .to_lowercase()
    };
    let norm: Vec<_> = words.iter().map(|w| bare(w)).collect();
    let mut out = Vec::new();
    for (at, w) in words.iter().enumerate() {
        if !has(BROKEN_OF_A_THING, &norm[at]) {
            out.push((*w).to_owned());
            continue;
        }
        let mut between = Vec::new();
        let mut owner = None;
        for i in (0..at).rev() {
            if words[i].contains([',', ';', ':', '?', '!', '.']) {
                break;
            }
            let matches = |kind: &str| {
                entities.iter().any(|e| {
                    e.kind == kind
                        && e.text
                            .split_whitespace()
                            .any(|word| word.to_lowercase() == norm[i])
                })
            };
            if matches("thing") {
                owner = Some("thing");
                break;
            }
            if matches("person") {
                owner = Some("person");
                break;
            }
            between.push(norm[i].as_str())
        }
        if between.iter().any(|s| has(OTHER_PRONOUNS, s)) {
            owner = None
        } else if owner.is_none() {
            let mut subjects = between.iter().copied().filter(|s| {
                !s.is_empty() && s.chars().all(char::is_alphanumeric) && !has(NOT_A_SUBJECT, s)
            });
            if subjects.next().is_some_and(|s| has(SPEAKER_SUBJECTS, s))
                && subjects.next().is_none()
            {
                owner = Some("person")
            }
        }
        if owner.is_none()
            || norm[at + 1..]
                .iter()
                .take(2)
                .any(|s| has(A_CONDITION_AFTER, s))
            || between.iter().any(|s| has(A_CLAUSE_VERB, s))
        {
            out.push((*w).to_owned());
            continue;
        }
        let replacement = if owner == Some("thing") {
            "broken"
        } else if norm[at] == "buggered" {
            "exhausted"
        } else {
            "knackered"
        };
        out.push(w.replace(
            w.trim_matches(|c| EDGE_PUNCTUATION.contains(c)),
            replacement,
        ));
    }
    Some(out.join(" "))
}
