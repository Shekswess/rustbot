//! Three parts of the original dish planner, using the shared plan and pairing readers.

use super::{cooking_pairings, food_fold};
use crate::bot::{
    Exchange, Memory, Reading,
    knowledge::dishes,
    memory::{FoodAck, food_ack},
    understanding::normalize::tokens,
};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $expression:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($expression).expect(stringify!($name)));
    };
}
pattern!(DELIBERATING, super::DISH_PLAN_DELIBERATING_START);
pattern!(COLOUR, super::DISH_PLAN_COLOUR_CHOICE);
pattern!(CHOICE, super::DISH_PLAN_CHOICE_FRAME);
pattern!(
    COOKING_NOW,
    r"\b(?:make|makes|making|made|cook|cooks|cooking|cooked|(?:i'?m|i am|we'?re|we are) doing)\b"
);
// These are the calendar phrases from plans/dish-for-tonight's second key.
const RECIPE_WHEN: &[&str] = &[
    "tonight",
    "this evening",
    "this afternoon",
    "for tea",
    "for dinner",
    "for supper",
    "for lunch",
    "later",
    "this weekend",
    "tomorrow night",
];

struct Side {
    names: Vec<Vec<String>>,
    reply: &'static str,
}

static SIDES: LazyLock<Vec<Side>> = LazyLock::new(|| {
    super::DISH_SIDES
        .iter()
        .map(|&(names, reply)| Side {
            names: phrases(names),
            reply,
        })
        .collect()
});
static SIDE_ASKS: LazyLock<Vec<Vec<String>>> =
    LazyLock::new(|| phrases(super::DISH_PLAN_ASK_PHRASES));
static USE_UP: LazyLock<Vec<Vec<String>>> =
    LazyLock::new(|| phrases(super::DISH_PLAN_USE_UP_PHRASES));

struct RecipeSubject {
    names: Vec<Vec<String>>,
    avoid: Vec<Vec<String>>,
    reply: &'static str,
}

static RECIPE_SUBJECTS: LazyLock<Vec<RecipeSubject>> = LazyLock::new(|| {
    super::DISH_PLAN_RECIPE_SUBJECTS
        .iter()
        .map(|&(names, avoid, reply)| RecipeSubject {
            names: phrases(names),
            avoid: phrases(avoid),
            reply,
        })
        .collect()
});

fn phrases(values: &[&str]) -> Vec<Vec<String>> {
    values.iter().map(|value| tokens(value)).collect()
}

pub(super) fn tentative_recipe(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    if !DELIBERATING.is_match(text)
        || COOKING_NOW.is_match(text)
        || !RECIPE_WHEN.iter().any(|when| text.contains(when))
    {
        return false;
    }
    let words = tokens(text);
    RECIPE_SUBJECTS
        .iter()
        .any(|recipe| dishes::names(&words, &recipe.names) && !dishes::names(&words, &recipe.avoid))
}

pub(super) fn dinner_recipe(reading: &Reading<'_>) -> Option<&'static str> {
    let words = tokens(&reading.line.clean);
    let mut best = None;
    for recipe in &*RECIPE_SUBJECTS {
        if dishes::names(&words, &recipe.avoid) {
            continue;
        }
        for alias in &recipe.names {
            if alias.is_empty() || !words.windows(alias.len()).any(|chunk| chunk == alias) {
                continue;
            }
            let length = alias.iter().map(String::len).sum::<usize>() + alias.len() - 1;
            if best.is_none_or(|(previous, _)| length > previous) {
                best = Some((length, recipe.reply));
            }
        }
    }
    best.map(|(_, reply)| reply)
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
    memory: Option<&Memory>,
) -> Option<Cow<'static, str>> {
    let line = &reading.line;
    let text = if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    };
    match id {
        "plan-temp" => dishes::oven_setting(text).map(Cow::Borrowed),
        "plan-followup" | "plan-keep" | "plan-done" => {
            dishes::followup::reply(reading, earlier, memory)
        }
        "plan-sides" => side(text, earlier).map(Cow::Borrowed),
        "plan-start" => {
            let deliberation = DELIBERATING.is_match(text)
                && super::DISH_PLAN_WHEN_WORDS
                    .iter()
                    .any(|when| text.contains(when));
            let reply = dishes::practical_plan(text, earlier, deliberation)?;
            plan_fits_diet(reading, earlier, &reply).then_some(Cow::Owned(reply))
        }
        _ => None,
    }
}

fn side(text: &str, earlier: &[Exchange]) -> Option<&'static str> {
    let words = tokens(text);
    let opener = words.first()?;
    let asks = super::DISH_PLAN_ASK_OPENERS.contains(&opener.as_str())
        || dishes::names(&words, &SIDE_ASKS);
    if !asks
        || (COLOUR.is_match(text) && CHOICE.is_match(text))
        || dishes::names(&words, &USE_UP)
        || super::DISH_PLAN_SIDE_METHOD
            .iter()
            .any(|phrase| text.contains(phrase))
    {
        return None;
    }
    let plan = dishes::plan_label_in_chat(text, earlier);
    if let Some(which) = SIDES.iter().find(|side| dishes::names(&words, &side.names)) {
        return (plan.is_some() || !cooking_pairings::asks_to_pair(text)).then_some(which.reply);
    }
    if let Some(plan) = plan {
        return super::DISH_PLAN_SIDES
            .iter()
            .find_map(|&(label, reply)| (label == plan).then_some(reply));
    }
    cooking_pairings::generic_pairing(text)
}

fn plan_fits_diet(reading: &Reading<'_>, earlier: &[Exchange], reply: &str) -> bool {
    if !cooking_pairings::fits_plant_dish(reading, reply) {
        return false;
    }
    let dish = reply.split_once(": ").map_or(reply, |(dish, _)| dish);
    let words = tokens(dish);
    !std::iter::once(reading.raw)
        .chain(earlier.iter().rev().take(4).map(|turn| turn.user.as_str()))
        .filter_map(food_ack)
        .any(|fact| match fact {
            FoodAck::Diet(name) => {
                (name.contains("vegetarian") || name.contains("vegan"))
                    && cooking_pairings::names_meat_or_fish(dish)
            }
            FoodAck::Allergy(foods) => foods.iter().any(|food| {
                let food = food_fold::fold(food);
                words.iter().any(|word| food_fold::fold(word) == food)
            }),
            _ => false,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/dish_plan.rs"
    ));

    #[test]
    fn dish_plan_callbacks_match_pinned_source() {
        let mut differences = Vec::new();
        for &(id, line, prior, expected) in CASES {
            let row = super::super::rows()
                .find(|row| row.domain == "dishes" && row.id == id)
                .unwrap();
            let earlier = prior
                .iter()
                .map(|user| Exchange {
                    user: (*user).to_owned(),
                    bot: String::new(),
                    module: "answers",
                })
                .collect::<Vec<_>>();
            let reading = Reading::new(line);
            let context = super::super::guards::Context::new(&reading, &earlier);
            let actual = row.answer(&context);
            let expected = expected.map(|index| REPLIES[index]);
            if actual.as_deref() != expected {
                differences.push((id, line, actual, expected));
            }
        }
        assert!(differences.is_empty(), "{differences:#?}");
    }

    #[test]
    fn dish_plan_respects_candidate_rank_and_dinner_probes() {
        for (prompt, module, reply) in [
            (
                "I'm doing a steak tonight.",
                "greeting",
                "Get stuck in. What's the first bit?",
            ),
            (
                "what can we make tonight",
                "answers",
                "Here's my pick: a tuna salad in 10 minutes, or tomato pasta in 20, or a lentil dhal in 30. Tell me what is in the fridge and I will narrow it down.",
            ),
        ] {
            let turn = crate::bot::Bot::new(Some("0")).unwrap().turn(prompt);
            assert_eq!(turn.module, Some(module));
            assert_eq!(turn.reply, reply);
        }
        assert!(
            super::super::lookup("I was thinking of a spaghetti bolognese tonight.")
                .is_none_or(|card| card.id != "plan-start")
        );
    }
}
