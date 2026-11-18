//! Stand aside when the preceding turn belongs to a care or loss reader.

use crate::bot::{compose::loss_reader, understanding::line};
use fancy_regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "thread_detail_heavy_data.rs"]
mod data;
use data::*;

fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn any(patterns: &[Regex], text: &str) -> bool {
    patterns.iter().any(|pattern| matches(pattern, text))
}

fn a_clear_result(text: &str) -> bool {
    let text = super::collapse(text);
    !text.is_empty()
        && !text.contains('?')
        && ![
            &*ASK_OPEN,
            &*NOT_CLEAR,
            &*NEGATED,
            &*CONDITIONAL,
            &*NOT_NEWS,
            &*NOT_QUITE,
        ]
        .iter()
        .any(|pattern| matches(pattern, &text))
        && CLEAR_RESULT.iter().any(|pattern| {
            pattern.find_iter(&text).flatten().any(|found| {
                CLEAR_TAIL
                    .find(&text[found.end()..])
                    .ok()
                    .flatten()
                    .is_some_and(|ending| ending.start() == 0)
            })
        })
}

fn nearly(word: &str) -> bool {
    if word.len() < 6 || NOT_A_TYPO.binary_search(&word).is_ok() || line::lexical_word(word) {
        return false;
    }
    EMERGENCY_TYPOS.iter().any(|target| {
        if word.len() == target.len() {
            let mut differences = word.bytes().zip(target.bytes()).filter(|(a, b)| a != b);
            differences
                .next()
                .is_some_and(|(a, b)| b"aeiou".contains(&a) && b"aeiou".contains(&b))
                && differences.next().is_none()
        } else if word.len().abs_diff(target.len()) == 1 {
            let (short, long) = if word.len() < target.len() {
                (word, *target)
            } else {
                (*target, word)
            };
            let mismatch = short
                .bytes()
                .zip(long.bytes())
                .position(|(a, b)| a != b)
                .unwrap_or(short.len());
            short.as_bytes()[mismatch..] == long.as_bytes()[mismatch + 1..]
        } else {
            false
        }
    })
}

pub(in crate::bot) fn bad_news(text: &str) -> bool {
    if a_clear_result(text) {
        return false;
    }
    let not_joked = JOKED.replace_all(text, " ");
    any(&MEDICAL, &not_joked)
        || any(&NAMED_NEWS, &not_joked)
        || text
            .split(|letter: char| !letter.is_ascii_lowercase() && letter != '\'')
            .any(nearly)
}

pub(in crate::bot) fn only_a_dream(text: &str) -> bool {
    (matches(&DREAM_NAMED, text) || matches(&DREAM_NAMED_LOOSE, text))
        && !matches(&A_WISH_TO_BE_GONE, text)
        && !matches(&A_SAID_WISH, text)
        && (!matches(&A_DREAM_AHEAD, text) || matches(&DREAM_VERB, text))
        && !matches(&DREAM_IS_HAPPENING_NOW, text)
        && !matches(&A_DEATH_BEFORE_THE_DREAM, text)
}

fn subject_before(text: &str, index: usize, said: &str) -> bool {
    if matches(&NAMES_ITS_SUBJECT, said) {
        return true;
    }
    let clause = text[..index]
        .rsplit(['.', '!', '?'])
        .next()
        .unwrap_or_default();
    let words: Vec<_> = WORD
        .find_iter(clause)
        .flatten()
        .map(|word| word.as_str())
        .collect();
    words
        .into_iter()
        .rev()
        .find(|word| CARRIES.binary_search(word).is_err() && PLACE.binary_search(word).is_err())
        .is_some_and(|who| PERSON.binary_search(&who).is_ok())
}

pub(in crate::bot) fn sudden_loss(text: &str) -> bool {
    if only_a_dream(text) {
        return false;
    }
    any(&SUDDEN_LOSS, text) || matches(&SUICIDE, text) || death_report(text)
}

fn death_report(text: &str) -> bool {
    matches!(text, "died" | "dies" | "she died" | "he died")
        || DEATH_SAID.find_iter(text).flatten().any(|found| {
            !matches(&NEARLY, &text[..found.end()])
                && !AFTER_IT
                    .find(&text[found.end()..])
                    .ok()
                    .flatten()
                    .is_some_and(|tail| tail.start() == 0)
                && subject_before(text, found.start(), found.as_str())
        })
}

pub(in crate::bot) fn someone_died(text: &str) -> bool {
    death_report(&super::collapse(&text.to_lowercase()))
}

pub(in crate::bot) fn medical_news(text: &str) -> bool {
    any(&MEDICAL, text)
}

pub(in crate::bot) fn named_news(text: &str) -> bool {
    any(&NAMED_NEWS, text)
}

pub(in crate::bot) fn thread_is_heavy(text: &str) -> bool {
    let text = text.to_lowercase();
    bad_news(&text) || sudden_loss(&super::collapse(&text)) || loss_reader::reads_as_loss(&text)
}

pub(super) fn heavy_news(line: &str) -> bool {
    medical_news(&super::collapse(&line.to_lowercase().replace('’', "'"))) || thread_is_heavy(line)
}
