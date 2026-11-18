//! Reading recipe instructions and identifying ingredients, quantities and herbs.

use crate::{bot::Reading, text::is_space};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

fn word_set<'a>(words: impl Iterator<Item = &'a str>) -> RegexSet {
    RegexSet::new(words.map(|word| format!(r"\b{}\b", regex::escape(word))))
        .expect("authored cooking ingredient names")
}

static MEASURE_AWAY: LazyLock<RegexSet> =
    LazyLock::new(|| word_set(super::RECIPE_MEASURE_AWAY.iter().copied()));
static RECIPE_CUPS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(super::RECIPE_CUPS.iter().map(|(words, _)| {
        let terms = words
            .iter()
            .map(|word| regex::escape(word))
            .collect::<Vec<_>>();
        format!(r"\b(?:{})\b", terms.join("|"))
    }))
    .expect("authored recipe cup ingredients")
});
static RECIPE_TERMS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(super::RECIPE_TERMS.iter().map(|(pattern, _)| *pattern))
        .expect("authored recipe terms")
});
static HERB_OWNED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(super::RECIPE_HERB_OWNED).expect("authored herb possession phrase")
});

fn clean_or_text<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn lined(reading: &Reading<'_>) -> String {
    clean_or_text(reading)
        .to_lowercase()
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn measure(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = lined(reading);
    if MEASURE_AWAY.is_match(&line) {
        return None;
    }
    Some(Cow::Borrowed(
        RECIPE_CUPS
            .matches(&line)
            .iter()
            .next()
            .map_or(super::RECIPE_MEASURING_TAIL, |index| {
                super::RECIPE_CUPS[index].1
            }),
    ))
}

pub(super) fn read_recipe(
    reading: &Reading<'_>,
    fallback: &'static str,
) -> Option<Cow<'static, str>> {
    let line = lined(reading);
    Some(Cow::Borrowed(
        RECIPE_TERMS
            .matches(&line)
            .iter()
            .next()
            .map_or(fallback, |index| super::RECIPE_TERMS[index].1),
    ))
}

pub(super) fn recipe_amount(
    reading: &Reading<'_>,
    replies: &'static [&'static str; 3],
) -> Option<Cow<'static, str>> {
    let line = format!(" {} ", clean_or_text(reading).to_lowercase());
    let tin = has(&line, &[" tin ", " tins ", " sandwich tin", " cake tin"]);
    let different_size = has(
        &line,
        &[
            "bigger",
            "larger",
            "smaller",
            "different size",
            "different tin",
            "other tin",
            "wrong size",
            "too big",
            "too small",
            "deeper",
        ],
    );
    let reply = if tin && different_size {
        replies[0]
    } else if has(
        &line,
        &[
            "halve",
            "halving",
            "half the recipe",
            "half a recipe",
            "half this recipe",
            "half batch",
            "smaller batch",
            "only two of us",
            "just for one",
            "just me",
            "for two",
        ],
    ) {
        replies[1]
    } else {
        replies[2]
    };
    Some(Cow::Borrowed(reply))
}

fn has(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

fn names_word(reading: &Reading<'_>, padded: &str, names: &[&str]) -> bool {
    reading
        .line
        .words
        .iter()
        .any(|word| names.contains(&word.as_str()))
        || names_in(padded, names)
}

fn names_in(padded: &str, names: &[&str]) -> bool {
    names
        .iter()
        .any(|name| padded.contains(&format!(" {name} ")))
}

fn has_herb_use(reading: &Reading<'_>, clean: &str) -> bool {
    names_word(reading, clean, super::RECIPE_HERB_USE_ASK)
        || (names_in(clean, super::RECIPE_HERB_PLANT_WORDS) && HERB_OWNED.is_match(clean))
}

pub(super) fn herb_use_asked(reading: &Reading<'_>) -> bool {
    let clean = format!(" {} ", reading.line.clean.to_lowercase());
    has_herb_use(reading, &clean)
}

pub(super) fn use_herbs(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let clean = format!(" {} ", reading.line.clean.to_lowercase());
    let woody = names_word(reading, &clean, super::RECIPE_WOODY_HERB);
    let (using, picking) = if woody {
        (
            "Use them at the start of the cooking, because heat is what opens them up: rosemary or thyme with potatoes, lamb, chicken or a tray of vegetables, sage with pork or in a brown butter, and a bay leaf in rice or a stew.",
            "Take a few sprigs from the tips rather than cutting back to the old wood, and a third off the top is the most a bush wants at once - it grows back from there.",
        )
    } else {
        (
            "Use them at the end, off the heat, so the flavour stays - chopped into a green sauce with oil, lemon and garlic, torn over tomatoes or fish, or stirred into a soup just before it is served.",
            "Pinch the top few leaves and leave the stem, and the plant keeps going: a third off the top is plenty at once, and it grows back from the cuts.",
        )
    };
    Some(Cow::Owned(if has_herb_use(reading, &clean) {
        format!("{using} {picking}")
    } else {
        format!("{picking} {using}")
    }))
}
