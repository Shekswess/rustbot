//! Programme and documentary picks from PyBot's entertainment domain.

use super::entertainment::{plain_words, word_turned_down};
use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "entertainment_programmes_data.rs"]
mod data;
#[cfg(test)]
#[path = "entertainment_programmes_tests.rs"]
mod tests;

static NO_PROGRAMME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(data::PLAYING_NOT_WATCHING).expect("programme participation pattern")
});
static REFUSED_AFTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::KIND_REFUSED_AFTER).expect("documentary refusal pattern"));
static YES_OR_NO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::SHOW_YES_OR_NO).expect("programme question pattern"));
static SUBJECT_LEAD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::SHOW_SUBJECT_LEAD).expect("programme opening pattern"));

pub(super) fn programme_is_wanted(reading: &Reading<'_>) -> bool {
    let text = if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    };
    !NO_PROGRAMME.is_match(&text.to_lowercase())
}

pub(super) fn documentary_is_wanted(reading: &Reading<'_>) -> bool {
    let refused = |text: &str| {
        word_turned_down(
            text,
            |word| data::DOCUMENTARY_WORDS.contains(&word),
            &REFUSED_AFTER,
        )
    };
    if reading.line.text.trim().is_empty() {
        !refused(&reading.line.norm_text) && !refused(&reading.line.clean)
    } else {
        !refused(&reading.line.text)
    }
}

pub(super) fn subject_is_wanted(reading: &Reading<'_>) -> bool {
    super::entertainment::own_texts(reading).all(|text| {
        !word_turned_down(
            text,
            |word| {
                data::SHOW_SUBJECT_NAMES
                    .iter()
                    .any(|(name, _)| *name == word)
            },
            &REFUSED_AFTER,
        )
    })
}

fn named_subject(text: &str) -> Option<&'static str> {
    plain_words(text).split_whitespace().find_map(|word| {
        if data::SHOW_MEDIUM_WORDS.contains(&word) {
            return None;
        }
        data::SHOW_SUBJECT_NAMES
            .iter()
            .find_map(|&(name, subject)| (name == word).then_some(subject))
    })
}

pub(super) fn show_about(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = &reading.line;
    // The original scans the reading's dictionary in insertion order. With no
    // earlier subject value, the key "wants_a_quiz" itself supplies "quiz".
    let subject = [
        line.text.as_str(),
        line.clean.as_str(),
        &line.words.join(" "),
        line.plain.as_str(),
        reading.animal_in_hand.as_deref().unwrap_or_default(),
    ]
    .into_iter()
    .find_map(named_subject)
    .unwrap_or("quiz");
    let reply = data::SHOW_SUBJECTS
        .iter()
        .find_map(|&(name, reply)| (name == subject).then_some(reply))?;
    let said = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    if YES_OR_NO.is_match(said) {
        Some(Cow::Owned(format!(
            "Yes - {}",
            SUBJECT_LEAD.replace(reply, "")
        )))
    } else {
        Some(Cow::Borrowed(reply))
    }
}
