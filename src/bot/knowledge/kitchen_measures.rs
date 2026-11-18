//! Source tables for amounts within food, drink, or packaging.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::{
    COUNT_ASKS, COUNT_NOT_COUNTING, FOOD_AMOUNTS, MEASURE_CAFFEINE, MEASURE_COFFEE_CROWD,
    MEASURE_COFFEE_METHOD, MEASURE_COFFEE_RATIO, MEASURE_COFFEE_TIME, MEASURE_COOKING_WORDS,
    MEASURE_NOT_RECIPE, MEASURE_ONLY_MADE, MEASURE_PACK_COUNTS, MEASURE_PORTIONS,
    MEASURE_RECIPE_ASKS,
};

#[derive(Clone, Copy)]
pub(super) enum Ask {
    Caffeine,
    CoffeeStrength,
    Count,
    Amount,
}

pub(super) struct Amount {
    pub(super) dishes: &'static [&'static str],
    pub(super) ingredients: &'static [&'static str],
    pub(super) cooking: &'static str,
    pub(super) as_eaten: Option<&'static str>,
}

pub(super) struct Pack {
    pub(super) things: &'static [&'static str],
    pub(super) packs: &'static [&'static str],
    pub(super) reply: &'static str,
}

static FOOD_BOUNDARY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9 ]+").unwrap());
static DRINK_BOUNDARY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9% ]+").unwrap());

fn padded_has(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(start, _)| {
        text[..start].ends_with(' ') && text[start + word.len()..].starts_with(' ')
    })
}

fn any_word(line: &str, words: &[&str]) -> bool {
    words.iter().any(|word| padded_has(line, word))
}

fn any_phrase(line: &str, words: &[&str]) -> bool {
    words.iter().any(|word| line.contains(word))
}

fn caffeine(line: &str) -> Cow<'static, str> {
    const REST: &str = "The usual daily limit for an adult is 400mg, which is about four instant coffees, and 200mg in pregnancy.";
    if let Some((_, lead)) = MEASURE_CAFFEINE
        .iter()
        .find(|(words, _)| any_word(line, words))
    {
        return format!("{lead}. {REST}").into();
    }
    format!(
        "It varies with the drink: a mug of instant coffee is 60 to 80mg, filter coffee 90 to 140mg, a single espresso about 65mg, black tea about 50mg, green tea 25 to 40mg and a 330ml cola about 35mg. {REST}"
    ).into()
}

fn coffee(line: &str) -> Cow<'static, str> {
    if any_word(line, MEASURE_COFFEE_TIME) {
        return format!(
            "In a cafetiere: pour water a minute off the boil, leave it three to four minutes, then press and pour it all off the grounds or it goes on brewing and turns bitter. That is {MEASURE_COFFEE_RATIO}."
        ).into();
    }
    if any_word(line, MEASURE_COFFEE_CROWD) {
        return format!(
            "240g of ground coffee makes 20 mugs, each mug holding about two level tablespoons, 12g, and 60g filling a litre jug. {MEASURE_COFFEE_METHOD}. Go stronger for espresso, weaker with milk."
        ).into();
    }
    format!("Measure it by the mug: {MEASURE_COFFEE_RATIO}. {MEASURE_COFFEE_METHOD}. Stronger for espresso, weaker with milk.").into()
}

fn count(line: &str) -> Option<&'static str> {
    if any_word(line, COUNT_NOT_COUNTING) || !any_phrase(line, COUNT_ASKS) {
        return None;
    }
    MEASURE_PACK_COUNTS.iter().find_map(|row| {
        let pack = row.packs.iter().any(|word| {
            line.match_indices(word)
                .any(|(start, _)| line[..start].ends_with(' '))
        });
        (pack && any_phrase(line, row.things)).then_some(row.reply)
    })
}

fn amount(line: &str) -> Option<&'static str> {
    if !any_phrase(line, MEASURE_RECIPE_ASKS) || any_phrase(line, MEASURE_PORTIONS) {
        return None;
    }
    let row = FOOD_AMOUNTS.iter().find(|row| {
        any_word(line, row.dishes)
            && row
                .ingredients
                .iter()
                .any(|word| padded_has(line, word) || padded_has(line, &format!("{word}s")))
    })?;
    let only_made = row
        .dishes
        .iter()
        .any(|dish| MEASURE_ONLY_MADE.contains(dish) && padded_has(line, dish));
    if any_phrase(line, MEASURE_NOT_RECIPE) && !only_made && !any_word(line, MEASURE_COOKING_WORDS)
    {
        return row.as_eaten;
    }
    Some(row.cooking)
}

pub(super) fn answer(ask: Ask, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let clean = reading.line.clean.to_lowercase();
    let boundary = match ask {
        Ask::Caffeine | Ask::CoffeeStrength => &*DRINK_BOUNDARY,
        Ask::Count | Ask::Amount => &*FOOD_BOUNDARY,
    };
    let line = format!(" {} ", boundary.replace_all(&clean, " "));
    match ask {
        Ask::Caffeine => Some(caffeine(&line)),
        Ask::CoffeeStrength => Some(coffee(&line)),
        Ask::Count => count(&line).map(Cow::Borrowed),
        Ask::Amount => amount(&line).map(Cow::Borrowed),
    }
}
