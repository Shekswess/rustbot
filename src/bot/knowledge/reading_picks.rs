//! Subject, whole-book and comics recommendations from PyBot.

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use super::{BOOK_SIMILAR, books, entertainment, has_ascii_phrase};

#[path = "reading_picks_data.rs"]
mod data;
#[cfg(test)]
#[path = "reading_picks_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    BOOK_ANCHOR,
    BOOK_SUBJECT_BEFORE,
    BOOK_VERB_BEFORE,
    BOOK_VERB_AFTER,
    A_DETERMINER_JUST_BEFORE,
    A_BOOK_I_AM_NOT_ENJOYING,
    ASKS_FOR_A_PICK,
    SUITABILITY_ASK,
    COMIC_MEANING_ASK,
);

enum Subject {
    Covered(&'static str),
    Uncovered,
    Unnamed,
}

fn says(text: &str, phrase: &str) -> bool {
    text.match_indices(phrase).any(|(start, _)| {
        let bytes = text.as_bytes();
        (start == 0 || !bytes[start - 1].is_ascii_lowercase())
            && bytes
                .get(start + phrase.len())
                .is_none_or(|next| !next.is_ascii_lowercase())
    })
}

fn subject(text: &str) -> Subject {
    let Some(tail) = BOOK_ANCHOR
        .captures(text)
        .or_else(|| BOOK_SUBJECT_BEFORE.captures(text))
        .and_then(|matched| matched.get(1))
    else {
        return Subject::Unnamed;
    };
    let tail = tail.as_str();
    if let Some((_, key)) = data::SUBJECT_BOOK_NAMES
        .iter()
        .find(|(phrase, _)| says(tail, phrase))
    {
        data::SUBJECT_BOOKS
            .iter()
            .find_map(|(name, pick)| (name == key).then_some(Subject::Covered(pick)))
            .unwrap_or(Subject::Uncovered)
    } else if data::SUBJECT_UNCOVERED_ORDER
        .iter()
        .any(|phrase| says(tail, phrase))
    {
        Subject::Uncovered
    } else {
        Subject::Unnamed
    }
}

fn subject_text(reading: &Reading<'_>) -> String {
    format!("{} {}", reading.line.clean, reading.line.norm_text).to_lowercase()
}

fn reservation(text: &str) -> bool {
    data::RESERVING_PHRASES
        .iter()
        .any(|phrase| says(text, phrase))
        || BOOK_VERB_BEFORE.is_match(text)
        || BOOK_VERB_AFTER
            .find_iter(text)
            .any(|matched| !A_DETERMINER_JUST_BEFORE.is_match(&text[..matched.start()]))
}

pub(super) fn reserving(reading: &Reading<'_>) -> bool {
    [&reading.line.clean, &reading.line.norm_text]
        .into_iter()
        .any(|text| reservation(&text.to_lowercase()))
}

pub(super) fn subject_is_wanted(reading: &Reading<'_>) -> bool {
    let text = subject_text(reading);
    !reserving(reading)
        && (!data::JUDGE_WORDS.iter().any(|word| says(&text, word))
            || !data::JUDGED_WORKS
                .iter()
                .any(|title| has_ascii_phrase(&text, title)))
}

pub(super) fn about_subject(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    if books::genre_is_named(reading) {
        return None;
    }
    if let Subject::Covered(pick) = subject(&subject_text(reading)) {
        Some(Cow::Borrowed(pick))
    } else {
        None
    }
}

pub(super) fn recommendation_is_wanted(reading: &Reading<'_>) -> bool {
    !reserving(reading) && !entertainment::reads_mood(reading)
}

fn contains_one(text: &str, options: &[&str]) -> bool {
    options.iter().any(|word| text.contains(word))
}

pub(super) fn recommend(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let said = subject_text(reading);
    match subject(&said) {
        Subject::Covered(pick) => return Some(Cow::Borrowed(pick)),
        Subject::Uncovered => return None,
        Subject::Unnamed => {}
    }
    if A_BOOK_I_AM_NOT_ENJOYING.is_match(&said) && !ASKS_FOR_A_PICK.is_match(&said) {
        return Some(Cow::Borrowed(data::BOOK_COMPLAINT));
    }
    let mut context = format!(
        "{} {}",
        reading.line.clean,
        earlier[earlier.len().saturating_sub(3)..]
            .iter()
            .map(|turn| turn.user.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    )
    .to_lowercase();
    if let Some((title, _)) = books::longest_named(&context, BOOK_SIMILAR) {
        context = context.replace(title, " ");
        if !contains_one(&context, data::CHILD_BOOK_WORDS) {
            return books::similar(reading);
        }
    }
    let pick = if contains_one(&context, data::BABY_BOOK_WORDS) {
        data::BOOK_FOR_BABY
    } else if contains_one(&context, data::CLASS_BOOK_WORDS) {
        data::BOOK_FOR_CLASS
    } else if contains_one(&context, data::CHILD_BOOK_WORDS) {
        data::BOOK_FOR_CHILD
    } else {
        data::BOOK_GENERAL
    };
    Some(Cow::Borrowed(pick))
}

fn has_word(words: &[String], options: &[&str]) -> bool {
    words.iter().any(|word| options.contains(&word.as_str()))
}

pub(super) fn comic_pick_is_wanted(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    !(has_word(words, data::CHILD_OF_THEIRS)
        && has_word(words, &["my", "our"])
        && has_word(words, data::LIKING)
        && !has_word(words, data::ASKS_FOR_A_PICK_WORDS))
}

#[derive(Copy, Clone)]
enum YoungReader {
    Child,
    Teen,
}

fn named_reader<'a>(
    mut words: impl Iterator<Item = &'a str> + Clone,
    ages: impl Iterator<Item = f64>,
) -> Option<YoungReader> {
    if words.clone().any(|word| data::TEEN_WORDS.contains(&word)) {
        return Some(YoungReader::Teen);
    }
    for age in ages.filter(|age| age.is_finite()).map(f64::trunc) {
        if (13.0..=19.0).contains(&age) {
            return Some(YoungReader::Teen);
        }
        if age <= 12.0 {
            return Some(YoungReader::Child);
        }
    }
    words
        .any(|word| data::CHILD_OF_THEIRS.contains(&word))
        .then_some(YoungReader::Child)
}

fn comic_kind(said: &str, words: &[String]) -> Option<&'static str> {
    if COMIC_MEANING_ASK.is_match(said) {
        return Some(data::COMIC_MEANING);
    }
    if !has_word(words, data::KIND_ASKS) {
        return None;
    }
    for &(name, card) in data::KIND_WORDS {
        if words.iter().any(|word| word == name) {
            return Some(card);
        }
    }
    (has_word(words, data::BEGINNER_WORDS)
        || said.contains("where do i start")
        || said.contains("where to start"))
    .then_some(data::COMIC_BEGINNER)
}

pub(super) fn comics(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let said = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
    .to_lowercase();
    if SUITABILITY_ASK.is_match(&said) {
        return Some(Cow::Borrowed(data::COMIC_SUITABILITY));
    }
    let words = &reading.line.words;
    let ages = reading
        .numbers
        .iter()
        .filter(|number| number.unit == Some("years"))
        .map(|number| number.value);
    let mut reader = named_reader(words.iter().map(String::as_str), ages);
    if reader.is_none() {
        if let Some(pick) = comic_kind(&said, words) {
            return Some(Cow::Borrowed(pick));
        }
        reader = earlier.iter().rev().find_map(|turn| {
            let lower = turn.user.to_lowercase();
            named_reader(
                lower.split(|ch: char| !ch.is_ascii_lowercase() && ch != '\''),
                std::iter::empty(),
            )
        });
    }
    let pick = match reader {
        Some(YoungReader::Child) => data::COMIC_FOR_A_CHILD,
        Some(YoungReader::Teen) => data::COMIC_FOR_A_TEEN,
        None => data::COMIC_FOR_A_GROWN_UP,
    };
    Some(Cow::Borrowed(pick))
}
