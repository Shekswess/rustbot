//! Source branch selection for seasoning, texture and emulsions.

use std::sync::LazyLock;

use fancy_regex::Regex;

use crate::bot::Reading;

use super::{cooking_basics, food_fold::fold};

#[derive(Clone, Copy)]
pub(super) enum Rule {
    Soup,
    Salty,
    Watery,
    Lumpy,
    Split,
    Chocolate,
}

static WHITE_CHOCOLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bwhite\s+(?=chocolate\b)").expect("source chocolate distinction")
});

const WATERY_DOUGH: usize = 10;
const WATERY_YOGHURT: usize = 21;
const WATERY_EMPTY: usize = 25;
const WATERY_DEFAULT: usize = 26;

fn names<'a>(words: impl IntoIterator<Item = &'a str>, terms: &[&str]) -> bool {
    words.into_iter().any(|word| {
        !word.is_empty()
            && terms
                .iter()
                .any(|name| !name.is_empty() && word.starts_with(name))
    })
}

fn any_phrase(said: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| said.contains(term))
}

fn any_multiword(said: &str, terms: &[&str]) -> bool {
    terms
        .iter()
        .any(|term| term.contains(' ') && said.contains(term))
}

fn soup(words: &[String]) -> usize {
    let said = format!(
        " {} ",
        words
            .iter()
            .map(|word| fold(word))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let thickening = any_phrase(&said, &["thicken", "thicker", "thickening"]);
    usize::from(thickening || !any_phrase(&said, super::TEXTURE_TOO_THICK_SAID))
}

fn lumpy(words: &[String]) -> usize {
    if names(words.iter().map(String::as_str), &["mash"])
        || words
            .iter()
            .any(|word| matches!(word.as_str(), "potato" | "potatoes"))
    {
        0
    } else if names(words.iter().map(String::as_str), &["custard"]) {
        1
    } else if names(words.iter().map(String::as_str), &["porridge", "oats"]) {
        2
    } else {
        3
    }
}

fn split(words: &[String]) -> usize {
    let said = words.join(" ");
    if names(
        words.iter().map(String::as_str),
        super::TEXTURE_CUSTARD_NAMES,
    ) || any_multiword(&said, super::TEXTURE_CUSTARD_NAMES)
    {
        0
    } else if words.iter().any(|word| {
        matches!(
            word.as_str(),
            "mayonnaise" | "mayo" | "aioli" | "garlic mayo" | "garlic mayonnaise" | "remoulade"
        )
    }) {
        1
    } else if words.iter().any(|word| {
        matches!(
            word.as_str(),
            "hollandaise" | "béarnaise" | "bearnaise" | "butter sauce"
        )
    }) {
        2
    } else {
        3
    }
}

fn chocolate(words: &[String]) -> usize {
    let said = words.join(" ");
    let surface = WHITE_CHOCOLATE.replace_all(&said, "");
    if names(
        surface.split_whitespace(),
        &["bloom", "white", "streak", "streaks", "grey", "gray"],
    ) {
        0
    } else if any_phrase(
        &said,
        &[
            "healthy",
            "good for you",
            "bad for you",
            "better for you",
            "fattening",
            "sugar",
            "calorie",
            "calories",
            "diet",
        ],
    ) {
        1
    } else if any_phrase(
        &said,
        &[
            "real chocolate",
            "is it chocolate",
            "actually chocolate",
            "made of",
            "made from",
            "contain",
            "contains",
            "what is chocolate",
            "is white chocolate",
            "whats in chocolate",
            "what's in chocolate",
        ],
    ) {
        2 + usize::from(!words.iter().any(|word| word == "white"))
    } else if names(words.iter().map(String::as_str), &["temper"]) {
        4
    } else if names(
        words.iter().map(String::as_str),
        &["dip", "dipped", "dipping", "strawberry", "strawberries"],
    ) && !names(
        words.iter().map(String::as_str),
        &["melt", "melting", "melted"],
    ) {
        5
    } else if names(
        words.iter().map(String::as_str),
        &["seize", "seized", "seizing", "grainy", "lumpy", "stiff"],
    ) {
        6
    } else {
        7
    }
}

fn watery(words: &[String]) -> usize {
    let said = words.join(" ");
    if any_phrase(&said, super::TEXTURE_PLANT_MILK)
        && !any_phrase(&said, super::TEXTURE_PLANT_MILK_DISH)
    {
        return 0;
    }
    let words: Vec<_> = words
        .iter()
        .map(String::as_str)
        .filter(|word| !said.contains(&format!("{word} milk")))
        .collect();
    let Some(index) = super::TEXTURE_WATERY_NAMES
        .iter()
        .position(|terms| names(words.iter().copied(), terms) || any_multiword(&said, terms))
        .map(|index| index + 1)
    else {
        let pan_food = [
            "sauce",
            "sauces",
            "stew",
            "curry",
            "chilli",
            "chili",
            "gravy",
            "ragu",
            "bolognese",
            "bolognaise",
            "daal",
            "dhal",
            "dal",
            "tagine",
            "goulash",
            "casserole",
            "cassoulet",
            "broth",
            "stock",
            "soup",
            "soups",
            "jus",
        ];
        let thin_words = [
            "watery", "runny", "thin", "sloppy", "liquid", "thickens", "thicken", "reduce",
            "reduces",
        ];
        let thin = words
            .iter()
            .any(|word| pan_food.contains(word) || thin_words.contains(word))
            || any_phrase(
                &said,
                &[
                    "never thickens",
                    "wont thicken",
                    "won't thicken",
                    "not thickening",
                    "doesnt thicken",
                    "doesn't thicken",
                ],
            );
        return if !thin
            && any_phrase(
                &said,
                &["too wet", "a bit wet", "is wet", "went wet", "bit wet"],
            ) {
            WATERY_EMPTY
        } else {
            WATERY_DEFAULT
        };
    };
    match index {
        WATERY_YOGHURT => index + usize::from(!any_phrase(&said, super::TEXTURE_WET_DIP_WORDS)),
        22..=23 => index + 1, // The yoghurt branch contributes two authored replies.
        _ => index,
    }
}

pub(super) fn reply(
    rule: Rule,
    reading: &Reading<'_>,
    replies: &'static [&'static str],
) -> Option<&'static str> {
    let words = &reading.line.words;
    let index = match rule {
        Rule::Soup => soup(words),
        Rule::Salty => usize::from(!names(
            words.iter().map(String::as_str),
            super::TEXTURE_BAKE,
        )),
        Rule::Lumpy => lumpy(words),
        Rule::Split => split(words),
        Rule::Chocolate => chocolate(words),
        Rule::Watery => {
            let index = watery(words);
            if index == WATERY_DOUGH {
                return cooking_basics::reply(
                    reading,
                    cooking_basics::Rule::Dough,
                    super::TEXTURE_DOUGH,
                );
            }
            index
        }
    };
    replies.get(index).copied()
}
