//! Typed meal requests and the vegetarian or vegan dinner card.

use super::{
    DIET_COOK_ASK_PATTERN, DIET_COOKING_FOODS, DIET_COOKING_PATTERN, DIET_DISH_WORDS,
    DIET_GUEST_PATTERN, DIET_MAKE_PATTERN, DIET_MEAL_WORDS, DIET_RECALL_PATTERN, DIET_VEGAN_MENU,
    DIET_VEGAN_PATTERN, DIET_VEGAN_SWAPS, DIET_VEGETARIAN_MENU, DIET_VEGETARIAN_SWAPS,
    DIET_YES_NO_PATTERN, guards::Context,
};
use crate::bot::{
    Memory, Reading,
    knowledge::meal_shortlist::{self, Diet, MealPlan},
    understanding::{normalize::tokens, topics},
};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(RECALL, DIET_RECALL_PATTERN);
pattern!(GUEST, DIET_GUEST_PATTERN);
pattern!(VEGAN, DIET_VEGAN_PATTERN);
pattern!(COOKING, DIET_COOKING_PATTERN);
pattern!(MAKE, DIET_MAKE_PATTERN);
pattern!(YES_NO, DIET_YES_NO_PATTERN);
pattern!(COOK_ASK, DIET_COOK_ASK_PATTERN);

static FOOD: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    const VERBS: &[&str] = &[
        "make", "made", "cook", "cooking", "fix", "fixing", "taste", "tastes", "tasted", "came",
        "out", "turned", "went", "wrong",
    ];
    DIET_DISH_WORDS
        .iter()
        .chain(DIET_COOKING_FOODS)
        .copied()
        .filter(|word| !VERBS.contains(word))
        .collect()
});

fn cooking_context(line: &str) -> bool {
    COOKING.is_match(line)
        || MAKE.is_match(line) && tokens(line).iter().any(|word| FOOD.contains(word.as_str()))
}

pub(super) fn about_food(line: &str) -> bool {
    tokens(line)
        .iter()
        .any(|word| FOOD.contains(word.as_str()) || DIET_MEAL_WORDS.contains(&word.as_str()))
}

pub(super) fn plan(context: &Context<'_>) -> Option<&'static str> {
    let reading = context.reading;
    let line = if reading.line.norm_text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.norm_text
    };
    let recent = context.earlier.iter().rev().map(|turn| turn.user.as_str());
    if RECALL.is_match(line)
        || YES_NO.is_match(line) && !COOK_ASK.is_match(line)
        || !(GUEST.is_match(line) || about_food(line) || recent.clone().take(3).any(about_food))
    {
        return None;
    }
    let menu =
        context.earlier.is_empty() || cooking_context(line) || recent.take(4).any(cooking_context);
    Some(match (VEGAN.is_match(line), menu) {
        (true, true) => DIET_VEGAN_MENU,
        (false, true) => DIET_VEGETARIAN_MENU,
        (true, false) => DIET_VEGAN_SWAPS,
        (false, false) => DIET_VEGETARIAN_SWAPS,
    })
}

pattern!(
    SPECIFIC_RESTRICTION,
    r"(?i)\b(?:allerg\w*|intoleran\w*|without|does(?:n['’]?t| not) (?:eat|have)|don['’]?t (?:eat|have)|coeliac|celiac|gluten[- ]free|dairy[- ]free|no (?:dairy|egg|gluten|milk|cheese|nuts?|fish|meat))\b"
);

pub(crate) fn detail_answer(
    said: &str,
    reading: &Reading<'_>,
    selected: Option<(&str, &str)>,
    plan: Option<&MealPlan>,
    memory: &Memory,
) -> bool {
    if topics::gift(&reading.line.norm) || NEGATED_DIET.is_match(reading.raw) {
        return false;
    }
    if let Some(diet) = Diet::named(said) {
        return !SPECIFIC_RESTRICTION.is_match(reading.raw)
            && plan.and_then(|plan| plan.diet) == Some(diet)
            && !memory.pantry.blocks_plant_offer(memory, diet.vegan());
    }
    // These fresh lookups have authored answers for the actual restriction.
    // Unrelated dinner shortlists or generic meal cards are not diet details.
    matches!(
        (said, selected),
        ("dairy free" | "dairy-free", Some(("dishes", "plan-diet")))
            | ("pescatarian", Some(("cooking", "vegetarian-guest")))
            | (
                "coeliac" | "celiac" | "gluten free" | "gluten-free",
                Some(("cooking", "meal-for-the-situation"))
            )
    ) && !memory.pantry.blocks_typed_offer(memory)
}

pattern!(
    NEGATED_DIET,
    r"(?i)\b(?:not|no longer|never)\s+(?:an?\s+)?(?:vegan|vegetarian|pescatarian)\b"
);
pattern!(
    ALLERGY_OR_CONDITION,
    r"(?i)\b(?:allerg\w*|anaphyla\w*|intoleran\w*|coeliac|celiac|gluten[- ]free)\b"
);
pattern!(
    PLANT_LIMIT,
    r"(?i)\b(?:vegan|vegans|vegetarian|vegetarians|meat[- ]free|meatless|(?:does not|doesnt|doesn['’]t|do not|dont|don['’]t) eat meat|no meat)\b"
);
pattern!(
    ANIMAL_PRODUCTS,
    r"(?i)\b(?:dairy|eggs?|milk|butter|cheese|cheddar|feta|cream|ghee|yog[hu]?urt|yoghurt|parmesan|honey|gelat(?:in|ine))\b"
);
pattern!(
    OTHER_MEAT,
    r"(?i)\b(?:salmon|tuna|cod|haddock|prawns?|shrimp|mackerel|mince|chorizo|gelat(?:in|ine))\b"
);
pattern!(
    OPEN_PLANT_CONTINUATION,
    r"(?i)\A(?:and\s+)?(?:what about\s+)?(?:something\s+|anything\s+)?(?:vegan|vegetarian|meat[- ]free|meatless)(?:\s+instead)?(?:\s*,?\s+please)?\s*[,?.!]*\z"
);
pattern!(
    REVISION,
    r"(?i)\b(?:make it|make that|can i make with|can we make with|what else|another idea)\b"
);
pattern!(
    FOOD_EXCLUSION,
    r"(?i)\b(?:without|no|(?:does not|doesnt|doesn['’]t|do not|dont|don['’]t) (?:eat|have))\s+([^,.?!]+)"
);

pub(super) fn requested_meal(reading: &Reading<'_>, memory: Option<&Memory>) -> Option<Diet> {
    let line = reading.raw;
    let explicit = PLANT_LIMIT.is_match(line);
    let recent_plan = memory
        .and_then(|memory| memory.meal_plan.as_ref().map(|plan| (memory, plan)))
        .filter(|(memory, plan)| memory.turns.len().saturating_sub(plan.turn) <= 2)
        .map(|(_, plan)| plan);
    if !explicit && recent_plan.and_then(|plan| plan.diet).is_none()
        || NEGATED_DIET.is_match(line)
        || topics::gift(&reading.line.norm)
        || YES_NO.is_match(line) && !COOK_ASK.is_match(line)
    {
        return None;
    }
    let continuation = recent_plan.is_some() && OPEN_PLANT_CONTINUATION.is_match(line);
    if explicit
        && (COOK_ASK.is_match(line)
            || GUEST.is_match(line)
            || continuation
            || memory.is_some_and(|memory| {
                memory
                    .turns
                    .iter()
                    .rev()
                    .take(3)
                    .any(|turn| about_food(&turn.user))
            }))
    {
        return Some(
            if VEGAN.is_match(line)
                || continuation && recent_plan.and_then(|plan| plan.diet) == Some(Diet::Vegan)
            {
                Diet::Vegan
            } else {
                Diet::Vegetarian
            },
        );
    }
    let tracked = recent_plan?.diet?;
    (meal_shortlist::is_open_dinner(reading)
        || meal_shortlist::is_another_idea_ask(reading)
        || REVISION.is_match(line))
    .then_some(tracked)
}

pub(super) fn lets_recipe_through(
    reading: &Reading<'_>,
    reply: &str,
    memory: Option<&Memory>,
    diet: Diet,
) -> bool {
    if ALLERGY_OR_CONDITION.is_match(reading.raw)
        || memory.is_some_and(|memory| memory.pantry.blocks_plant_offer(memory, diet.vegan()))
    {
        return false;
    }
    let authored = reply
        .split_once(". Check the stock cubes")
        .or_else(|| reply.split_once(". One dish everyone"))
        .map_or(reply, |(menu, _)| menu);
    let ingredients = authored
        .replace("swap the meat for", "")
        .replace("in place of the cheese sauce", "")
        .replace("instead of butter", "");
    if super::pantry_intake::uses_excluded_food(reading, &ingredients, &FOOD_EXCLUSION) {
        return false;
    }
    if [DIET_VEGAN_MENU, DIET_VEGAN_SWAPS].contains(&reply)
        || !diet.vegan() && [DIET_VEGETARIAN_MENU, DIET_VEGETARIAN_SWAPS].contains(&reply)
    {
        return true;
    }
    let reply = reply
        .strip_prefix(
            "Pick a dish that is good on its own rather than meat with the meat left out: ",
        )
        .unwrap_or(reply);
    !super::cooking_pairings::names_meat_or_fish(reply)
        && !OTHER_MEAT.is_match(reply)
        && (!diet.vegan() || !ANIMAL_PRODUCTS.is_match(reply))
}

pub(super) fn indexed_offer(domain: &str, id: &str, reply: &str, context: &Context<'_>) -> bool {
    let authored = match (domain, id) {
        ("dishes", "plan-veggie") if [DIET_VEGAN_MENU, DIET_VEGAN_SWAPS].contains(&reply) => {
            Diet::Vegan
        }
        ("cooking", "vegan-main") => Diet::Vegan,
        ("dishes", "plan-veggie") | ("cooking", "vegetarian-main" | "vegetarian-guest") => {
            Diet::Vegetarian
        }
        _ => return true,
    };
    let diet = requested_meal(context.reading, context.memory).unwrap_or(authored);
    lets_recipe_through(context.reading, reply, context.memory, diet)
}

#[cfg(test)]
#[path = "dietary_menus_tests.rs"]
mod tests;
