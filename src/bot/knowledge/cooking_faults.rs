//! Authored fixes for serving, baking, and vegetable preparation faults.

use super::dishes::{names, same_word};
use crate::bot::{Reading, understanding::normalize::tokens};
use regex::Regex;
use std::sync::LazyLock;

#[path = "cooking_faults_data.rs"]
mod data;

static BREAD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "(?:^|[^a-z])(?:{})(?:$|[^a-z])",
        data::BREAD_WORDS.join("|")
    ))
    .expect("bread names")
});

struct Vegetables {
    faults: Vec<Vec<String>>,
    jacket: Vec<Vec<String>>,
    potatoes: Vec<Vec<String>>,
    roasting: Vec<Vec<String>>,
    mushrooms: Vec<Vec<String>>,
    pulses: Vec<Vec<String>>,
    families: Vec<(Vec<Vec<String>>, &'static str)>,
}

static VEGETABLES: LazyLock<Vegetables> = LazyLock::new(|| {
    let terms = |phrases: &[&str]| phrases.iter().map(|phrase| tokens(phrase)).collect();
    Vegetables {
        faults: terms(data::VEG_FAULTS),
        jacket: terms(&[
            "jacket potato",
            "baked potato",
            "jacket potatoes",
            "baked potatoes",
        ]),
        potatoes: terms(&["potato", "potatoes", "new potatoes"]),
        roasting: terms(&["roast", "roasted", "roasting", "oven", "crispy", "tray"]),
        mushrooms: terms(&["mushroom", "mushrooms"]),
        pulses: terms(&[
            "dried",
            "haricot",
            "haricots",
            "cannellini",
            "borlotti",
            "butter bean",
            "butter beans",
            "chickpea",
            "chickpeas",
            "split peas",
            "lentil",
            "lentils",
            "puy",
        ]),
        families: data::VEG_FAMILIES
            .iter()
            .map(|&(family, reply)| (terms(family), reply))
            .collect(),
    }
});

pub(crate) fn registered(id: &str, reading: &Reading<'_>) -> Option<&'static str> {
    let line = &reading.line;
    match id {
        "bake-gummy" => Some(if BREAD.is_match(&line.clean) {
            data::GUMMY_BREAD
        } else {
            data::GUMMY_CAKE
        }),
        "soggy-lunch" => Some(soggy_lunch(&line.words)),
        "bake-fault-more" => bake(&line.words),
        "veg-tough" => {
            let said = if line.norm_text.is_empty() {
                &line.clean
            } else {
                &line.norm_text
            };
            vegetable(&tokens(said))
        }
        _ => super::cooking_repairs::registered(id, reading),
    }
}

fn soggy_lunch(words: &[String]) -> &'static str {
    let said = words.join(" ");
    let refers_to = |family: &[&str]| {
        family.iter().any(|name| {
            words.iter().any(|word| word.starts_with(name))
                || (name.contains(' ') && said.contains(name))
        })
    };
    if refers_to(data::SOGGY_BREAD) {
        data::SOGGY_SANDWICH
    } else if refers_to(data::SOGGY_LEAF) {
        data::SOGGY_SALAD
    } else {
        data::SOGGY_BOX
    }
}

fn bake(words: &[String]) -> Option<&'static str> {
    let has = |alternatives: &[&str]| {
        words
            .iter()
            .any(|word| alternatives.iter().any(|option| same_word(word, option)))
    };
    if has(&["waffle", "waffles"]) {
        Some(data::WAFFLE)
    } else if has(&["flapjack", "flapjacks", "oat bar", "oat bars"]) {
        if has(&[
            "set", "firm", "firmed", "runny", "syrup", "sloppy", "sticky", "soggy", "soft",
        ]) {
            Some(data::FLAPJACK_SET)
        } else {
            Some(data::FLAPJACK_CRUMBLE)
        }
    } else if has(&["brownie", "brownies"]) {
        has(&[
            "dry",
            "cakey",
            "cake-like",
            "crumbly",
            "overbaked",
            "too long",
            "hard",
            "stale",
        ])
        .then_some(data::DRY_BROWNIE)
    } else {
        has(&["scone", "scones"]).then_some(data::DENSE_SCONE)
    }
}

fn vegetable(words: &[String]) -> Option<&'static str> {
    let recipe = &*VEGETABLES;
    if !names(words, &recipe.faults) {
        return None;
    }
    if names(words, &recipe.jacket) {
        return Some(data::JACKET);
    }
    if names(words, &recipe.potatoes) {
        return Some(if names(words, &recipe.roasting) {
            data::POTATO_ROAST
        } else {
            data::POTATO_BOILED
        });
    }
    if names(words, &recipe.mushrooms) {
        return Some(data::MUSHROOM);
    }
    if names(words, &recipe.pulses) {
        return None;
    }
    recipe
        .families
        .iter()
        .find_map(|(family, reply)| names(words, family).then_some(*reply))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/cooking_faults.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooking_faults_follow_pinned_precedence_and_abstention() {
        for &(id, raw, expected) in fixtures::CASES {
            assert_eq!(registered(id, &Reading::new(raw)), expected, "{id}: {raw}");
        }
    }
}
