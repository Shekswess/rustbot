//! Ordinary recipe methods and food-holding replies from the pinned cooking registry.

use super::food_fold::fold;
use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "cooking_methods_data.rs"]
mod data;
#[path = "cooking_fault_extras.rs"]
mod fault_extras;
#[path = "cooking_ingredients.rs"]
pub(super) mod ingredients;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(SMOKER_WORD, r"[a-z']+");
pattern!(GLUTEN_WORDS, r"[^a-z0-9 ]+");
pattern!(GLUTEN_FACT, r"^(?:is|are|does|do|can|should)\b");
pattern!(PEAS_SHORTAGE, data::PEAS_SHORTAGE);

fn contains(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn mentions(padded: &str, terms: &[&str]) -> bool {
    terms
        .iter()
        .any(|term| padded.contains(&format!(" {term} ")))
}

fn any_word(words: &[String], terms: &[&str]) -> bool {
    words.iter().any(|word| terms.contains(&word.as_str()))
}

fn tea_milk(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let line = format!(" {} ", reading.line.clean);
    if !mentions(&line, data::TEA_MILK_BREW)
        && earlier.iter().rev().take(2).any(|turn| {
            mentions(
                &format!(" {} ", turn.user.to_lowercase()),
                data::TEA_MILK_PAN,
            )
        })
    {
        return None;
    }
    let plant = mentions(&line, data::TEA_MILK_PLANT_SAID);
    if contains(&line, data::TEA_MILK_AMOUNT_ASKS) {
        let next = if plant {
            data::TEA_MILK_PLANT
        } else {
            data::TEA_MILK_KIND
        };
        Some(Cow::Owned(format!("{} {next}", data::TEA_MILK_AMOUNT)))
    } else if contains(&line, data::TEA_MILK_KIND_ASKS) {
        Some(if plant {
            Cow::Owned(format!("{} {}", data::TEA_MILK_KIND, data::TEA_MILK_PLANT))
        } else {
            Cow::Borrowed(data::TEA_MILK_KIND)
        })
    } else if plant {
        Some(Cow::Borrowed(data::TEA_MILK_PLANT))
    } else {
        Some(Cow::Borrowed(data::TEA_MILK_ORDER))
    }
}

pub(super) fn has_peas(reading: &Reading<'_>) -> bool {
    !PEAS_SHORTAGE.is_match(&reading.line.text.to_lowercase())
}

pub(super) fn asks_about_smoker(reading: &Reading<'_>) -> bool {
    let said = reading.line.text.to_lowercase();
    let named = || SMOKER_WORD.find_iter(&said).map(|word| word.as_str());
    !named().any(|word| word == "smoked")
        || named().any(|word| word != "smoked" && data::SMOKING_WORDS.contains(&word))
        || contains(&said, data::SMOKING_PHRASES)
}

fn smoke(words: &[String]) -> Option<&'static str> {
    let name = data::SMOKE_THINGS
        .iter()
        .copied()
        .find(|word| words.iter().any(|said| said == *word));
    let recipe = match name {
        Some("rib" | "ribs") => "ribs",
        Some("pulled pork" | "pork" | "shoulder") if words.iter().any(|word| word == "rib") => {
            "ribs"
        }
        Some("chicken" | "turkey" | "wings") => "chicken",
        Some("salmon" | "trout" | "mackerel" | "haddock" | "fish") => "salmon",
        Some("brisket") => "brisket",
        Some("cheese") => "cheese",
        _ => "default",
    };
    data::SMOKE_REPLIES
        .iter()
        .find_map(|&(key, value)| (key == recipe).then_some(value))
}

fn smoked_fish(reading: &Reading<'_>) -> Cow<'static, str> {
    let said = reading.line.text.to_lowercase();
    if contains(&said, &["smoked salmon", "lox"]) {
        return Cow::Borrowed(data::SMOKED_SALMON_ALREADY);
    }
    let fish = data::SMOKED_FISH
        .iter()
        .copied()
        .find(|name| said.contains(name))
        .map_or("haddock", |name| match name {
            "kipper" | "kippers" | "smoked herring" => "kipper",
            _ => name.strip_prefix("smoked ").unwrap_or(name),
        });
    Cow::Owned(data::SMOKED_FISH_REPLY.replace("{fish}", fish))
}

fn vegan_baking(words: &[String]) -> &'static str {
    let folded: Vec<_> = words.iter().map(|word| fold(word)).collect();
    let has = |terms: &[&str]| terms.iter().any(|term| folded.contains(&fold(term)));
    if has(&[
        "pancake",
        "pancakes",
        "crepe",
        "crepes",
        "waffle",
        "waffles",
        "drop scone",
    ]) {
        data::VEGAN_PANCAKE
    } else if has(&["scone", "scones", "drop scone", "drop scones"]) {
        data::VEGAN_SCONE
    } else if has(&[
        "pastry",
        "shortcrust",
        "shortcrust pastry",
        "pie",
        "pie crust",
        "tart",
        "tart case",
        "flan",
        "quiche",
        "quiche case",
    ]) {
        data::VEGAN_PASTRY
    } else if has(&["crumble", "fruit crumble", "cobbler"]) {
        data::VEGAN_CRUMBLE
    } else if has(&["biscuit", "cookie", "shortbread"]) {
        data::VEGAN_BISCUIT
    } else if has(&["flapjack", "oat bar", "granola bar"]) {
        data::VEGAN_FLAPJACK
    } else if has(&[
        "bread",
        "loaf",
        "loaves",
        "roll",
        "baguette",
        "focaccia",
        "pizza",
        "pizza dough",
        "flatbread",
        "naan",
    ]) && !has(&[
        "banana",
        "carrot",
        "courgette",
        "zucchini",
        "apple",
        "date",
        "pumpkin",
        "lemon",
        "fruit",
        "soda",
        "cake",
        "brioche",
        "challah",
    ]) {
        data::VEGAN_BREAD
    } else if has(&["brownie", "brownies", "blondie", "blondies"]) {
        data::VEGAN_BROWNIE
    } else {
        data::VEGAN_CAKE
    }
}

fn gluten_free(clean: &str) -> Cow<'static, str> {
    let padded = format!(" {} ", GLUTEN_WORDS.replace_all(clean, " "));
    let has = |wanted| mentions(&padded, wanted);
    let dairy_free = has(&["dairy free", "without dairy", "no dairy", "vegan"]);
    if GLUTEN_FACT.is_match(clean) && !has(data::GLUTEN_MAKING) {
        Cow::Borrowed(data::GLUTEN_FACT)
    } else if has(&[
        "bread", "loaf", "loaves", "roll", "rolls", "baguette", "pizza", "naan", "focaccia",
    ]) {
        Cow::Borrowed(data::GLUTEN_BREAD)
    } else if has(&[
        "pasta",
        "spaghetti",
        "penne",
        "macaroni",
        "lasagne",
        "noodles",
    ]) {
        Cow::Borrowed(data::GLUTEN_PASTA)
    } else if has(&[
        "pastry",
        "shortcrust",
        "tart",
        "pie",
        "quiche",
        "flan",
        "crust",
    ]) {
        if dairy_free {
            let mut recipe =
                data::GLUTEN_PASTRY.replace("cold butter", "cold dairy-free block margarine");
            if has(&["vegan"]) {
                recipe = recipe.replace(
                    "a beaten egg or a few spoons of iced water",
                    "a few spoons of iced water",
                );
            }
            Cow::Owned(recipe)
        } else {
            Cow::Borrowed(data::GLUTEN_PASTRY)
        }
    } else if has(&[
        "dairy", "milk", "butter", "cheese", "yoghurt", "yogurt", "vegan", "plant",
    ]) {
        Cow::Borrowed(data::GLUTEN_BAKE_DAIRY)
    } else {
        Cow::Borrowed(data::GLUTEN_BAKE)
    }
}

fn cauliflower(words: &[String]) -> &'static str {
    if words.iter().map(|word| fold(word)).any(|word| {
        ["carb", "carbs", "keto", "diet", "diabetes", "sugar", "low"].contains(&word.as_str())
    }) {
        data::CAULIFLOWER_CARBS
    } else {
        data::CAULIFLOWER_BASIC
    }
}

fn hold_food(words: &[String]) -> Option<&'static str> {
    let has = |terms| any_word(words, terms);
    if !has(data::KEEP_QUALITIES) || !has(data::KEEP_FOOD) {
        return None;
    }
    if has(data::KEEP_STICK_WORDS) {
        return (!has(&["rice"]) && has(data::KEEP_BOILED)).then_some(data::KEEP_STICK);
    }
    if has(data::KEEP_HOT_WORDS) {
        return Some(if has(&["rice"]) {
            data::KEEP_RICE
        } else if has(data::KEEP_GRAIN_WORDS) {
            data::KEEP_GRAIN
        } else if has(data::KEEP_POUR_WORDS) {
            data::KEEP_POUR
        } else if has(data::KEEP_COVERED_WORDS) {
            data::KEEP_COVERED
        } else if has(data::KEEP_FLATBREAD_WORDS) || (has(&["garlic"]) && has(&["bread"])) {
            data::KEEP_FLATBREAD
        } else if has(data::KEEP_STACK_WORDS) {
            data::KEEP_STACK
        } else if has(data::KEEP_CRISP_HELD_WORDS) {
            data::KEEP_CHIPS
        } else {
            data::KEEP_HOT
        });
    }
    if has(data::KEEP_SOFT_WORDS) {
        return has(data::KEEP_MEAT).then_some(data::KEEP_SOFT);
    }
    Some(data::KEEP_CRISP)
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let line = &reading.line;
    match id {
        "tea-milk" => tea_milk(reading, earlier),
        "smoke-meat" => smoke(&line.words).map(Cow::Borrowed),
        "smoked-fish" => Some(smoked_fish(reading)),
        "cauliflower-rice" => Some(Cow::Borrowed(cauliflower(&line.words))),
        "vegan-baking" => Some(Cow::Borrowed(vegan_baking(&line.words))),
        "gluten-free-baking" => Some(gluten_free(&line.clean)),
        "keep-how" => hold_food(&line.words).map(Cow::Borrowed),
        "peas-frozen" => Some(Cow::Borrowed(data::PEAS_FROZEN_REPLY)),
        _ => fault_extras::registered(id, reading)
            .or_else(|| ingredients::registered(id, reading))
            .or_else(|| super::cooking_nutrition::registered(id, reading)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooking_methods_use_the_named_dish_and_avoid_recipe_context() {
        let earlier = [Exchange {
            user: "I am making scrambled eggs".to_owned(),
            bot: String::new(),
            module: "answers",
        }];
        assert!(tea_milk(&Reading::new("Does the milk go in first?"), &earlier).is_none());
        let amount = tea_milk(
            &Reading::new("How much milk in tea if I pour oat milk first?"),
            &earlier,
        )
        .unwrap();
        assert!(amount.starts_with(data::TEA_MILK_AMOUNT));
        assert!(amount.ends_with(data::TEA_MILK_PLANT));
        assert!(!has_peas(&Reading::new("I have no frozen peas.")));
        assert!(has_peas(&Reading::new("Do I use peas out of the bag?")));
        assert!(!asks_about_smoker(&Reading::new(
            "How do I cook smoked haddock?"
        )));
        assert!(asks_about_smoker(&Reading::new(
            "Can I cook smoked ribs on a barbecue?"
        )));
        let fish = smoked_fish(&Reading::new("How do I cook smoked trout?"));
        assert!(fish.contains("smoked trout"));
        assert!(fish.contains("steaming hot throughout"));
    }

    #[test]
    fn ingredient_choices_and_holding_differ_by_recipe() {
        let wheat_free = gluten_free("how to make gluten free vegan pastry");
        assert!(wheat_free.contains("dairy-free block margarine"));
        assert!(!wheat_free.contains("a beaten egg"));
        let vegan = Reading::new("How do I make vegan brownies?");
        assert_eq!(vegan_baking(&vegan.line.words), data::VEGAN_BROWNIE);
        let rice = Reading::new("How do I keep rice warm?");
        assert_eq!(hold_food(&rice.line.words), Some(data::KEEP_RICE));
        let sticking = Reading::new("How do I stop rice from sticking?");
        assert!(hold_food(&sticking.line.words).is_none());
        let cauliflower_rice = Reading::new("How do I make cauliflower rice on a low carb diet?");
        assert_eq!(
            cauliflower(&cauliflower_rice.line.words),
            data::CAULIFLOWER_CARBS
        );
    }
}
