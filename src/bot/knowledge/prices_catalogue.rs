//! The original UK everyday price examples, dated in each reply.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::Reading;

#[path = "prices_catalogue_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    A_FIGURE,
    A_JUDGEMENT,
    MONEY_WORD,
    JUDGING,
    PRICE,
    NOT_A_PRICE,
    NOT_AN_ITEM
);

static ITEM_NAMES: LazyLock<Vec<RegexSet>> = LazyLock::new(|| {
    data::ITEMS
        .iter()
        .map(|(names, _)| {
            RegexSet::new(
                names
                    .iter()
                    .map(|name| format!(r"\b{}\b", regex::escape(name))),
            )
            .expect("historical price items")
        })
        .collect()
});

pub(super) fn price_is_judged(text: &str) -> bool {
    JUDGING.is_match(text)
}

pub(crate) fn asks_price(reading: &Reading<'_>) -> bool {
    let text = &reading.clean;
    if text.is_empty() || NOT_AN_ITEM.is_match(text) || NOT_A_PRICE.is_match(text) {
        return false;
    }
    let judged = A_JUDGEMENT.is_match(text);
    let figure = A_FIGURE.is_match(text);
    if judged && !figure && !MONEY_WORD.is_match(text) {
        return false;
    }
    PRICE.is_match(text) || (judged && figure)
}

fn meaningful_words(name: &str) -> impl Iterator<Item = &str> {
    name.split_whitespace()
        .filter(|word| !matches!(*word, "a" | "an" | "the"))
}

pub(crate) fn reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let matches = ITEM_NAMES
        .iter()
        .zip(data::ITEMS)
        .filter_map(|(patterns, (names, reply))| {
            let longest = patterns
                .matches(&reading.clean)
                .into_iter()
                .map(|index| names[index])
                .max_by_key(|name| name.len())?;
            Some((longest, *reply))
        })
        .collect::<Vec<_>>();
    let answers = matches
        .iter()
        .filter(|(name, _)| {
            !matches.iter().any(|(other, _)| {
                meaningful_words(name).count() < meaningful_words(other).count()
                    && meaningful_words(name)
                        .all(|word| meaningful_words(other).any(|other| other == word))
            })
        })
        .take(3)
        .map(|(_, reply)| *reply)
        .collect::<Vec<_>>();
    if answers.is_empty() {
        return None;
    }
    Some(Cow::Owned(format!(
        "As a rough UK guide from 2025, {} Check local prices for what you'd pay today.",
        answers.join(" ")
    )))
}
