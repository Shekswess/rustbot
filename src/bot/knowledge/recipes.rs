//! Recipe reply gates from the pinned cooking registry.

use crate::bot::{
    Reading,
    understanding::{intent, kinds::Kind},
};
use fancy_regex::Regex as FancyRegex;
use regex::Regex;
use std::sync::LazyLock;

use super::{
    COOKING_CLOCK_PATTERN, COOKING_KEPT_THING_PATTERN, COOKING_MAKE_IT_PATTERN, COOKING_OWN_FAULT,
    COOKING_SUPPLY_VERB, COOKING_WITHOUT_SUBJECT, RECIPE_ALONE_FILLER, RECIPE_ASK,
    RECIPE_COOKING_PATTERN, RECIPE_KEEPING_PATTERN, RECIPE_NO_YEAST, RECIPE_NOT_THIS_PATTERN,
    RECIPE_POT_PATTERN, RECIPE_WANT_ACTION_PATTERN, RECIPE_WANT_PATTERN,
    RECIPE_WANT_TO_COOK_PATTERN, RECIPE_WANT_TO_LEARN_PATTERN, guards::Context,
};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(WANT, RECIPE_WANT_PATTERN);
pattern!(WANT_ACTION, RECIPE_WANT_ACTION_PATTERN);
pattern!(WANT_TO_COOK, RECIPE_WANT_TO_COOK_PATTERN);
pattern!(WANT_TO_LEARN, RECIPE_WANT_TO_LEARN_PATTERN);
pattern!(KEEPING, RECIPE_KEEPING_PATTERN);
pattern!(COOKING, RECIPE_COOKING_PATTERN);
pattern!(SENTENCE_END, r"[.!?]\s+");
pattern!(COOKING_CLOCK, COOKING_CLOCK_PATTERN);
pattern!(KEPT_THING, COOKING_KEPT_THING_PATTERN);
pattern!(MAKE_IT_ASK, COOKING_MAKE_IT_PATTERN);

pub(super) fn bake(reading: &Reading<'_>) -> Option<&'static str> {
    use super::{
        BAKING_ALREADY_PHRASES, BAKING_ALREADY_WORDS, BAKING_RECIPE_ALIASES, BAKING_TIMING_ASKS,
        BAKING_VEGAN_ASK, PANTRY_INGREDIENT_NAMES, food_fold::fold, pantry_names,
    };

    let words: Vec<_> = reading.line.words.iter().map(|word| fold(word)).collect();
    let said = words.join(" ");
    if BAKING_TIMING_ASKS
        .iter()
        .chain(RECIPE_NO_YEAST)
        .chain(BAKING_VEGAN_ASK)
        .any(|phrase| said.contains(phrase))
        || (reading.features.asks.is_empty()
            && !words
                .iter()
                .any(|word| matches!(word.as_str(), "how" | "what" | "whats" | "recipe")))
    {
        return None;
    }

    let multiple_foods = pantry_names::ingredients(reading, &words).len() > 1;
    let (start, reply) = BAKING_RECIPE_ALIASES.iter().find_map(|&(parts, reply)| {
        if multiple_foods && parts.len() == 1 && PANTRY_INGREDIENT_NAMES.contains(&parts[0]) {
            return None;
        }
        let start = words
            .windows(parts.len())
            .position(|window| window.iter().map(String::as_str).eq(parts.iter().copied()))?;
        Some((start, reply))
    })?;
    let before = &words[start.saturating_sub(3)..start];
    if before
        .iter()
        .any(|word| BAKING_ALREADY_WORDS.contains(&word.as_str()))
    {
        return None;
    }
    let before = before.join(" ");
    (!BAKING_ALREADY_PHRASES
        .iter()
        .any(|phrase| before.contains(phrase)))
    .then_some(reply)
}

fn without_clauses<'words, 'line>(
    words: &'words [&'line str],
) -> impl Iterator<Item = &'words [&'line str]> {
    words
        .iter()
        .enumerate()
        .filter(|(_, word)| **word == "without")
        .map(|(index, _)| &words[index + 1..(index + 4).min(words.len())])
}

fn cooking_words(line: &str) -> Vec<&str> {
    line.split(|character: char| !(character.is_ascii_alphanumeric() || character == '\''))
        .filter(|word| !word.is_empty())
        .collect()
}

pub(super) fn swap_not_step(line: &str) -> bool {
    let words = cooking_words(line);
    let mut clauses = without_clauses(&words).peekable();
    if clauses.peek().is_none() {
        return true;
    }
    let outcome = |word: &&str| {
        word.ends_with("ing")
            && !COOKING_OWN_FAULT.contains(word)
            && !COOKING_SUPPLY_VERB.contains(word)
    };
    clauses.any(|after| {
        let Some((first, rest)) = after.split_first() else {
            return false;
        };
        if COOKING_WITHOUT_SUBJECT.contains(first) {
            !rest.iter().any(outcome)
        } else {
            !outcome(first)
        }
    })
}

pub(super) fn uses_required_ingredients(line: &str, ingredients: &[&str]) -> bool {
    !without_clauses(&cooking_words(line))
        .flatten()
        .any(|word| ingredients.contains(word))
}

pub(super) fn keeping_not_cooking(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    if MAKE_IT_ASK.is_match(&line.clean) {
        return false;
    }
    let words = line.words.join(" ");
    !COOKING_CLOCK.is_match(&words) || KEPT_THING.is_match(&words)
}

static POT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("(?i){RECIPE_POT_PATTERN}")).expect("valid prepared recipe pattern")
});
static NOT_THIS: LazyLock<FancyRegex> = LazyLock::new(|| {
    FancyRegex::new(RECIPE_NOT_THIS_PATTERN).expect("valid excluded recipe pattern")
});

pub(super) fn not_already_made(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let text = if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    };
    !POT.is_match(text)
}

pub(super) fn alone(line: &str, aliases: &[&str]) -> bool {
    let normalized = line.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut left = format!(" {normalized} ");
    let mut names = aliases.to_vec();
    names.sort_by_key(|name| std::cmp::Reverse(name.chars().count()));
    let mut named = false;
    for name in names {
        let needle = format!(" {} ", name.to_lowercase());
        if left.contains(&needle) {
            left = left.replace(&needle, " ");
            named = true;
        }
    }
    named
        && left
            .split_whitespace()
            .all(|word| RECIPE_ALONE_FILLER.contains(&word))
}

fn asks_for_method(text: &str) -> bool {
    RECIPE_ASK.iter().any(|phrase| {
        text.match_indices(phrase).any(|(start, hit)| {
            let end = start + hit.len();
            !text[..start].ends_with(|c: char| c.is_ascii_lowercase())
                && !text[end..].starts_with(|c: char| c.is_ascii_lowercase())
        })
    })
}

pub(super) fn answer(
    context: &Context<'_>,
    aliases: &[&str],
    reply: &'static str,
    no_yeast_ok: bool,
) -> Option<&'static str> {
    let reading = context.reading;
    let line = &reading.line;
    let current = if line.plain.trim().is_empty() {
        &line.clean
    } else {
        &line.plain
    };
    let history = &context.earlier[context.earlier.len().saturating_sub(4)..];
    let said = history
        .iter()
        .map(|turn| turn.user.as_str())
        .chain(std::iter::once(current.as_str()))
        .collect::<Vec<_>>()
        .join(" ");
    let only_words = if line.words.is_empty() {
        line.clean.clone()
    } else {
        line.words.join(" ")
    };
    let alone = alone(&only_words, aliases);
    let wants_to_cook = WANT_TO_COOK.is_match(&said) || WANT_TO_LEARN.is_match(&said);
    let wants_action = WANT_ACTION.is_match(&said);
    let wants_the_dish = wants_to_cook || WANT.is_match(&said) && !wants_action;
    let kind_allows = matches!(reading.kind, Kind::Question | Kind::Statement);
    if (!alone && !asks_for_method(&said))
        || (wants_action && !wants_to_cook)
        || (!kind_allows && !wants_the_dish && !alone)
        || (!no_yeast_ok && RECIPE_NO_YEAST.iter().any(|phrase| said.contains(phrase)))
        || NOT_THIS.is_match(&said).unwrap_or(true)
        || KEEPING.is_match(&line.clean) && !COOKING.is_match(&line.clean)
    {
        return None;
    }
    if intent::asks_only_how_long(line, reading.kind) {
        return Some(
            SENTENCE_END
                .find(reply)
                .map_or(reply, |found| &reply[..found.start() + 1]),
        );
    }
    Some(reply)
}
