//! The vegetable timing, preparation, and texture callbacks from the pinned registry.

use std::sync::LazyLock;

use crate::bot::Reading;
use regex::Regex;

use super::{
    VEGETABLE_ASK_BETWEEN, VEGETABLE_ASKS, VEGETABLE_COOK, VEGETABLE_FILLER, VEGETABLE_HOW_ASKS,
    VEGETABLE_NAMED_METHOD, VEGETABLE_OTHER, VEGETABLE_ROAST_METHODS, VEGETABLE_ROAST_MINUTES,
    VEGETABLE_ROAST_OTHER, VEGETABLE_RULE_OUT, VEGETABLE_TIME_WORDS, VEGETABLE_WATER_FAMILIES,
    tokens,
};

#[cfg(test)]
#[path = "vegetables_tests.rs"]
mod tests;

pub(super) struct Reply {
    pub(super) name: &'static str,
    pub(super) method: Option<&'static str>,
    pub(super) frozen_method: Option<&'static str>,
    pub(super) timing: &'static str,
    pub(super) frozen_timing: &'static str,
}

fn singular_or_plural(one: &str, two: &str) -> bool {
    one == two || one.strip_suffix('s') == Some(two) || two.strip_suffix('s') == Some(one)
}

fn names_word(given: &str, wanted: &str) -> bool {
    singular_or_plural(given, wanted) || given.strip_suffix("es") == Some(wanted)
}

fn names(words: &[String], wants: &[&str]) -> bool {
    wants
        .iter()
        .any(|want| words.iter().any(|word| names_word(word, want)))
}

fn names_phrase<T: AsRef<str>>(words: &[T], phrase: &[T]) -> bool {
    let (Some((last_word, prefix)), Some((last_want, expected))) =
        (words.split_last(), phrase.split_last())
    else {
        return false;
    };
    prefix.len() == expected.len()
        && prefix
            .iter()
            .zip(expected)
            .all(|(left, right)| left.as_ref() == right.as_ref())
        && names_word(last_word.as_ref(), last_want.as_ref())
}

fn phrase_in(text: &str, phrase: &str) -> bool {
    text.match_indices(phrase).any(|(start, matched)| {
        !text[..start].ends_with(|c: char| c.is_ascii_lowercase())
            && !text[start + matched.len()..].starts_with(|c: char| c.is_ascii_lowercase())
    })
}

pub(super) fn defers_to_food_safety(reading: &Reading<'_>) -> bool {
    static TIME_OUT_OF_STORAGE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"\b(?:left|sat|been|was|were|stayed)\b[^.!?]{0,40}\b(?:out|on the counter|at room temperature)\b[^.!?]{0,28}\b(?:overnight|all night|since yesterday|\d+ hours?)\b",
        )
        .expect("food storage pattern")
    });
    let text = &reading.clean;
    super::guards::food_has_gone_off(text)
        || [
            "safe to eat",
            "ok to eat",
            "okay to eat",
            "still eat",
            "can i eat",
            "could i eat",
            "still edible",
            "there is mould",
            "there's mould",
            "there is mold",
            "there's mold",
            "smell sour",
            "smells sour",
        ]
        .iter()
        .any(|phrase| phrase_in(text, phrase))
        || TIME_OUT_OF_STORAGE.is_match(text)
}

fn ruled_out<'a>(words: &[&str], aliases: impl Iterator<Item = &'a str>) -> bool {
    for alias in aliases {
        let name = alias.replace('\'', " ");
        let wanted = name.split_whitespace().collect::<Vec<_>>();
        let Some((last, prefix)) = wanted.split_last() else {
            continue;
        };
        if words.windows(wanted.len()).enumerate().any(|(start, run)| {
            let (end, lead) = run.split_last().expect("one or more vegetable words");
            lead == prefix
                && words[start.saturating_sub(4)..start]
                    .iter()
                    .any(|word| VEGETABLE_RULE_OUT.contains(word))
                && names(&tokens(end), &[*last])
        }) {
            return true;
        }
    }
    false
}

fn another_question(text: &str, raw_words: &[&str], named: &str) -> bool {
    let near = |name: &str| {
        raw_words
            .iter()
            .position(|word| singular_or_plural(word, name))
    };
    let position = named.split_whitespace().next_back().and_then(near);
    for phrase in VEGETABLE_OTHER
        .iter()
        .copied()
        .filter(|phrase| phrase_in(text, phrase))
    {
        if phrase.split_whitespace().any(|word| word == named) {
            return true;
        }
        let (Some(position), Some(spot)) = (position, near(phrase)) else {
            continue;
        };
        if spot < position {
            if !raw_words[spot + 1..position]
                .iter()
                .any(|word| VEGETABLE_ASK_BETWEEN.contains(word))
            {
                return true;
            }
        } else {
            let takes_to = spot > position && {
                let between = &raw_words[position + 1..spot];
                between.iter().all(|word| VEGETABLE_FILLER.contains(word))
                    && !between.iter().any(|word| VEGETABLE_COOK.contains(word))
                    && between
                        .iter()
                        .any(|word| ["take", "takes", "took"].contains(word))
            };
            if takes_to || spot - position <= 2 {
                return true;
            }
        }
    }
    false
}

pub(super) fn answer(text: &str, aliases: &[Reply]) -> Option<&'static str> {
    let words = tokens(text);
    let raw = text.to_lowercase().replace('\'', " ");
    let raw_words = raw.split_whitespace().collect::<Vec<_>>();
    if !names(&words, VEGETABLE_ASKS)
        || ruled_out(&raw_words, aliases.iter().map(|alias| alias.name))
    {
        return None;
    }
    let timing = VEGETABLE_TIME_WORDS
        .iter()
        .any(|word| phrase_in(text, word));
    let how =
        !timing && names(&words, VEGETABLE_HOW_ASKS) && !names(&words, VEGETABLE_NAMED_METHOD);
    if !timing && !how {
        return None;
    }
    if names(&words, &["freezer", "freeze", "freezing"])
        && !names(
            &words,
            &[
                "boil", "boiling", "steam", "steaming", "cook", "cooking", "blanch",
            ],
        )
    {
        return None;
    }
    let matched = aliases.iter().find(|alias| {
        let expected = tokens(alias.name);
        !expected.is_empty()
            && words
                .windows(expected.len())
                .any(|run| names_phrase(run, &expected))
    })?;
    if [
        "in the oven",
        "in an oven",
        "in my oven",
        "in the air fryer",
        "in a hot oven",
    ]
    .into_iter()
    .any(|phrase| phrase_in(text, phrase))
        || another_question(text, &raw_words, matched.name)
    {
        return None;
    }
    let frozen = names(&words, &["frozen"]);
    match (how, frozen) {
        (true, false) => matched.method,
        (true, true) => matched.frozen_method,
        (false, false) => Some(matched.timing),
        (false, true) => Some(matched.frozen_timing),
    }
}

pub(super) fn roast(text: &str) -> Option<String> {
    if !VEGETABLE_ROAST_METHODS
        .iter()
        .any(|word| phrase_in(text, word))
    {
        return None;
    }
    let words = tokens(text);
    let raw = text.to_lowercase().replace('\'', " ");
    let raw_words = raw.split_whitespace().collect::<Vec<_>>();
    if names(&words, VEGETABLE_ROAST_OTHER)
        || ruled_out(
            &raw_words,
            VEGETABLE_ROAST_MINUTES.iter().map(|&(name, _, _)| name),
        )
    {
        return None;
    }
    let &(name, minutes, method) = VEGETABLE_ROAST_MINUTES.iter().find(|(name, _, _)| {
        let expected = tokens(name);
        words
            .windows(expected.len())
            .any(|run| names_phrase(run, &expected))
    })?;
    let mut name = name.to_owned();
    name[..1].make_ascii_uppercase();
    Some(format!(
        "{name}: {minutes} minutes in the oven at 200C (180C fan), {method}, in a single layer with a little oil so it browns rather than steams. It is done when the edges are browned and a knife goes into the thickest piece easily."
    ))
}

pub(super) fn watery(reading: &Reading<'_>) -> Option<&'static str> {
    let text = if reading.line.words.is_empty() {
        reading.line.clean.to_lowercase()
    } else {
        reading.line.words.join(" ").to_lowercase()
    };
    let mut best = None;
    for &(aliases, reply) in VEGETABLE_WATER_FAMILIES {
        for &name in aliases {
            if best.is_none_or(|(length, _)| name.len() > length) && phrase_in(&text, name) {
                best = Some((name.len(), reply));
            }
        }
    }
    best.map(|(_, reply)| reply)
}
