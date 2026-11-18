//! Glossary, scaling, and serving choices from the authored cooking cards.

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

static CROWD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::COOKING_CROWD).expect("authored pudding crowd size"));

pub(super) struct Term {
    pub word: &'static str,
    pub reply: &'static str,
    pub needs_definition: bool,
    pub subjects: &'static [&'static str],
}

pub(super) fn meaning(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = if reading.line.clean.is_empty() {
        reading.line.words.join(" ")
    } else {
        reading.line.clean.clone()
    };
    let said = line.to_lowercase();
    let definition = super::COOKING_DEFINITION_WORDS
        .iter()
        .any(|word| said.contains(word));
    let mut matches: Vec<_> = super::COOKING_GLOSSARY
        .iter()
        .filter_map(|term| {
            let stem = term.word.trim_end_matches('e');
            let position = said
                .find(term.word)
                .or_else(|| (stem.len() > 3).then(|| said.find(stem)).flatten())?;
            Some((position, term))
        })
        .collect();
    matches.sort_by_key(|(position, term)| (*position, term.word));
    let mut terms = matches.iter().filter_map(|&(position, term)| {
        let enclosed = matches.iter().any(|&(other_position, other)| {
            term.word != other.word
                && position >= other_position
                && position + term.word.len() <= other_position + other.word.len()
        });
        let named_subject = || {
            term.subjects.iter().any(|subject| said.contains(subject))
                || [" it", " them", " this", " these"]
                    .iter()
                    .any(|pronoun| said.contains(&format!("{}{pronoun}", term.word)))
        };
        (!enclosed && (definition || !term.needs_definition || named_subject())).then_some(term)
    });
    let first = terms.next()?.reply;
    Some(match terms.next() {
        Some(second) => Cow::Owned(format!("{first} {}", second.reply)),
        None => Cow::Borrowed(first),
    })
}

pub(super) fn scale(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = if reading.line.text.is_empty() {
        &reading.line.norm_text
    } else {
        &reading.line.text
    };
    let said = line.to_lowercase();
    let words: Vec<_> = said
        .split(|character: char| !character.is_ascii_lowercase() && character != '\'')
        .collect();
    let scaling_up = super::COOKING_SCALE_UP_WORDS
        .iter()
        .any(|word| words.contains(word) || (word.contains(' ') && said.contains(word)))
        || (words.contains(&"scale") && words.contains(&"up"));
    Some(Cow::Borrowed(if scaling_up {
        super::COOKING_SCALE_UP
    } else {
        super::COOKING_SCALE_DOWN
    }))
}

pub(super) fn pudding(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let result = super::COOKING_PUDDING_IDEAS;
    let said = reading.line.clean.to_lowercase();
    Some(match CROWD.find(&said) {
        Some(many) => Cow::Owned(format!(
            "{result} For {} people make two dishes of it and put the custard out in a jug.",
            many.as_str()
        )),
        None => Cow::Borrowed(result),
    })
}

pub(super) fn vegetable_ideas(
    reading: &Reading<'_>,
    earlier: &[Exchange],
    child_required: bool,
) -> Option<&'static str> {
    let about_a_child = |said: &str| {
        let said = format!(" {said} ");
        super::COOKING_CHILD_WORDS
            .iter()
            .any(|word| said.contains(word))
    };
    let child = about_a_child(&reading.line.clean)
        || earlier
            .iter()
            .rev()
            .take(3)
            .any(|turn| about_a_child(&turn.user));
    if !child {
        return (!child_required).then_some(super::COOKING_ADULT_VEG);
    }
    let answer = super::COOKING_KIDS_VEG;
    let opening: String = answer
        .chars()
        .take(40)
        .flat_map(char::to_lowercase)
        .collect();
    (!earlier
        .iter()
        .rev()
        .take(2)
        .any(|turn| turn.bot.to_lowercase().contains(&opening)))
    .then_some(answer)
}
