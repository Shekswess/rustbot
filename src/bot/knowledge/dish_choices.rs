//! Suggest a dish, replace an ingredient, or diagnose a cooking fault.

use super::{dietary_menus, guards::Context, meal_scenes::literal};
use crate::bot::{
    Reading,
    knowledge::{dishes, kitchen},
    understanding::normalize::tokens,
};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "dish_choices_data.rs"]
mod data;

pub(super) struct Fault {
    id: &'static str,
    aliases: &'static [&'static str],
    general: &'static str,
    dishes: &'static [(&'static str, &'static str)],
}

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(SAFETY, data::SAFETY_WORDS);
pattern!(NO_APPLIANCE, data::NO_APPLIANCE);
pattern!(WHOLE_FOOD, data::WHOLE_FOOD);
pattern!(CLEANING, data::_CLEANING_IT);
pattern!(PICK_ONE, data::_PICK_ONE);
pattern!(RUIN, data::_PLAN_THAT_MIGHT_BE_RUINED);
pattern!(FAULT_ASK, data::_ASKED_ABOUT_THE_FAULT);
pattern!(STATES, data::_A_STATE_VERB);
pattern!(OUT_OF, data::OUT_OF_CUE);
pattern!(
    NAMED_CHILD,
    r"\b(?:my\s+(?:son|daughter)|toddler|children|kids?|child)\b"
);
pattern!(OVEN, r"\boven\b");
pattern!(BACKREF, r"\b(?:this|it|that|one|its)\b");
pattern!(
    BARE_FREE_TIME,
    r"^(?:i|we) (?:only )?(?:have|have got|got) (?:half an hour|an hour|(?:a couple of|two) hours|\d+ (?:minutes?|hours?))[, ]+what (?:can|should) (?:i|we) do$"
);

fn names(words: &[String], forms: &[&str]) -> bool {
    words
        .iter()
        .any(|word| forms.iter().any(|form| dishes::same_word(word, form)))
}

fn phrases(text: &str, forms: &[&str]) -> bool {
    forms.iter().any(|phrase| literal(text, phrase.trim()))
}

fn has_food_context(text: &str, context: &Context<'_>) -> bool {
    dietary_menus::about_food(text)
        || context
            .earlier
            .iter()
            .rev()
            .take(3)
            .any(|turn| dietary_menus::about_food(&turn.user))
}

fn line<'a>(context: &'a Context<'_>) -> &'a str {
    let line = &context.reading.line;
    if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    }
}

fn recently_said(context: &Context<'_>, reply: &str) -> bool {
    context.earlier.last().is_some_and(|turn| {
        turn.bot
            .to_lowercase()
            .split_whitespace()
            .take(4)
            .eq(reply.to_lowercase().split_whitespace().take(4))
    })
}

pub(super) fn fault_asked(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    !RUIN.is_match(text) || FAULT_ASK.is_match(text)
}

fn named_fault(text: &str) -> Option<&'static Fault> {
    data::PROBLEMS.iter().find(|fault| {
        fault.aliases.iter().any(|word| {
            literal(text, word)
                && (!data::_SINK_VERBS.contains(word) || phrases(text, data::_SINK_AS_A_BAKE))
        })
    })
}

fn state_is_of_food(text: &str, fault: &Fault) -> bool {
    for form in fault.aliases {
        let Some(at) = text.find(form).filter(|_| literal(text, form)) else {
            continue;
        };
        let clause = text[..at]
            .rsplit([',', '.', ';', ':', '!', '?'])
            .next()
            .unwrap_or("");
        let clause = clause.rsplit(" and ").next().unwrap_or("");
        let Some(verb) = STATES.find_iter(clause).last() else {
            continue;
        };
        let subject = tokens(&clause[..verb.start()]);
        let subject = &subject[subject.len().saturating_sub(2)..];
        if subject.is_empty() {
            continue;
        }
        return names(subject, data::_STANDS_FOR_IT) || !names(subject, data::NOT_THE_FOOD);
    }
    true
}

fn by_names(words: &[String], rows: &[(&[&str], &'static str)]) -> Option<&'static str> {
    rows.iter()
        .find_map(|(forms, reply)| names(words, forms).then_some(*reply))
}

fn veg_fix(text: &str) -> Option<&'static str> {
    if !phrases(
        text,
        &["hard", "tough", "woody", "stringy", "chewy", "undercooked"],
    ) || phrases(
        text,
        &["dried", "soak", "soaked", "packet", "lentils", "canned"],
    ) {
        return None;
    }
    by_names(&tokens(text), data::VEG_FAMILIES)
}

pub(super) fn problem(context: &Context<'_>) -> Option<&'static str> {
    let text = line(context);
    if !fault_asked(context.reading) {
        return None;
    }
    let fault = if let Some(fault) = named_fault(text) {
        state_is_of_food(text, fault).then_some(fault)?
    } else if let Some(fault) = context.earlier.iter().rev().take(2).find_map(|turn| {
        let text = turn.user.to_lowercase();
        let fault = named_fault(&text)?;
        state_is_of_food(&text, fault).then_some(fault)
    }) {
        fault
    } else {
        return veg_fix(text);
    };
    let dish = dishes::plan_label_in_chat(text, context.earlier);
    if let Some(reply) = fault
        .dishes
        .iter()
        .find_map(|&(name, reply)| (Some(name) == dish).then_some(reply))
    {
        return Some(reply);
    }
    let words = tokens(text);
    let here = names(&words, data::FOOD_WORDS);
    if dish.is_none()
        && !here
        && !context
            .earlier
            .iter()
            .rev()
            .take(2)
            .any(|turn| names(&tokens(&turn.user), data::FOOD_WORDS))
    {
        return None;
    }
    if let Some(reply) = veg_fix(text) {
        return Some(reply);
    }
    match fault.id {
        "dense" => (!names(&words, data::_POT))
            .then(|| by_names(&words, data::HEAVY_FAMILIES).unwrap_or(data::HEAVY_DEFAULT)),
        "sticky" if names(&words, data::_MUSHY_WORDS) => {
            Some(by_names(&words, data::MUSHY_FAMILIES).unwrap_or(data::MUSHY_DEFAULT))
        }
        "sticky" => Some(by_names(&words, data::GLUE_GROUPS).unwrap_or(data::GLUE_DEFAULT)),
        "grainy" => {
            by_names(&words, data::GRAINY_FAMILIES).or(here.then_some(data::GRAINY_DEFAULT))
        }
        "lumpy" => by_names(&words, data::LUMPY_FAMILIES).or(here.then_some(data::LUMPY_DEFAULT)),
        _ => Some(fault.general),
    }
}

fn disliked(words: &[String]) -> Option<&str> {
    words.iter().enumerate().find_map(|(at, word)| {
        if !data::DISLIKED
            .iter()
            .any(|form| dishes::same_word(word, form))
        {
            return None;
        }
        let before = &words[at.saturating_sub(4)..at];
        let cue = before.join(" ");
        if !phrases(&cue, data::DISLIKE_SHAPES)
            || !phrases(&cue, data::EATING_REFUSALS) && names(before, data::WASTE_WORDS)
            || dishes::same_word(word, "peppers")
                && names(
                    before,
                    &[
                        "black",
                        "white",
                        "ground",
                        "cracked",
                        "peppercorn",
                        "peppercorns",
                    ],
                )
            || words.get(at + 1).is_some_and(|word| {
                !data::FOOD_TAIL_OK
                    .iter()
                    .any(|tail| dishes::same_word(word, tail))
            })
        {
            return None;
        }
        Some(word.as_str())
    })
}

fn swap(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let text = line(context);
    let words = tokens(text);
    if names(&words, data::LUNCH_MEALS)
        || !has_food_context(text, context)
        || NAMED_CHILD.is_match(text)
            && phrases(text, data::EATING_REFUSALS)
            && !phrases(text, data::DINNER_ASK)
            && !phrases(text, data::SHORTAGE_MARKERS)
    {
        return None;
    }
    if let Some(reply) = kitchen::without(&context.reading.line) {
        return Some(Cow::Owned(reply));
    }
    let food = disliked(&words)?;
    if data::DIET_FAMILIES
        .iter()
        .any(|name| dishes::same_word(food, name))
    {
        return None;
    }
    let in_flight = dishes::plan_label_in_chat(text, context.earlier).is_some()
        || context
            .earlier
            .iter()
            .rev()
            .take(3)
            .any(|turn| dietary_menus::about_food(&turn.user));
    let another_person = phrases(text, data::OTHER_PEOPLE);
    if !in_flight
        && !another_person
        && !(phrases(text, data::DINNER_ASK) && phrases(text, data::SAID_DISLIKE))
    {
        return None;
    }
    let across = |forms| {
        context
            .earlier
            .iter()
            .rev()
            .take(4)
            .any(|turn| names(&tokens(&turn.user), forms))
    };
    if matches!(food, "egg" | "eggs") && across(data::BAKE_WORDS) {
        return Some(Cow::Borrowed(data::EGG_FREE_BAKE));
    }
    if matches!(food, "egg" | "eggs") && across(data::EGG_DISH_WORDS) {
        return Some(Cow::Borrowed(data::EGG_IS_THE_DISH));
    }
    if data::DEFER_TO_ALLERGY
        .iter()
        .any(|nut| dishes::same_word(food, nut))
        && across(data::BAKE_WORDS)
    {
        return Some(Cow::Borrowed(data::NUT_FREE_BAKE));
    }
    if OUT_OF.is_match(text)
        && data::KITCHEN_SWAP
            .iter()
            .any(|name| dishes::same_word(food, name))
    {
        return None;
    }
    let other = if data::_BIRDS
        .iter()
        .any(|bird| dishes::same_word(food, bird))
    {
        "beef or pork"
    } else if data::_RED_MEAT
        .iter()
        .any(|meat| dishes::same_word(food, meat))
    {
        "chicken thighs"
    } else {
        "chicken thighs or beef"
    };
    let mut table = by_names(&[food.to_owned()], data::SUBSTITUTES).unwrap_or(data::GENERIC_SWAP);
    let mut reply = table.replace("{food}", food).replace("{other}", other);
    if reply.contains("chicken thighs") {
        if phrases(
            text,
            &[
                "no chicken",
                "no meat",
                "without chicken",
                "without meat",
                "or chicken",
                "or meat",
                "vegetarian",
                "vegan",
            ],
        ) {
            table = data::GENERIC_SWAP;
            reply = table.replace("{food}", food);
        } else {
            reply = reply.replace("go into the same sauce in the same time",
                "go into the same sauce. Cook chicken until the thickest part stays at or above 70°C for 2 minutes")
                .replace("same dish, same time, and for a roast the same oven, potatoes and gravy",
                    "keep the same dish, potatoes and gravy. If using chicken, cook it until the thickest part stays at or above 70°C for 2 minutes");
        }
    }
    if data::ALLERGENS
        .iter()
        .any(|name| dishes::same_word(food, name))
    {
        reply.push_str(data::_ALLERGEN_NOTE);
    }
    Some(Cow::Owned(reply))
}

fn appliance(text: &str) -> Option<&'static str> {
    if CLEANING.is_match(text) {
        return None;
    }
    let mut named = data::APPLIANCE_IDEAS
        .iter()
        .filter(|(forms, _)| phrases(text, forms));
    let first = named.next();
    if (named.next().is_some() || first.is_some() && OVEN.is_match(text)) && PICK_ONE.is_match(text)
    {
        return None;
    }
    first
        .map(|(_, reply)| *reply)
        .or_else(|| OVEN.is_match(text).then_some(data::TRAY_IDEA))
}

pub(super) fn free_time(context: &Context<'_>) -> bool {
    let said = context
        .reading
        .line
        .clean
        .trim_end_matches(['?', '.', '!', ' ']);
    (BARE_FREE_TIME.is_match(said)
        || ["what can i do", "what can we do", "what should i do"]
            .iter()
            .any(|ask| said.ends_with(ask))
            && super::leisure::cheap_day(context.reading))
        && !context
            .earlier
            .iter()
            .rev()
            .take(3)
            .any(|turn| dietary_menus::about_food(&turn.user))
}

fn ideas(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let text = line(context);
    let words = tokens(text);
    let earlier = context.earlier.iter().rev();
    if !has_food_context(text, context)
        || SAFETY.is_match(text)
        || names(&words, data::CUT_WORDS) && appliance(text).is_some()
        || dishes::named_recipe_phrase(text)
        || earlier
            .clone()
            .take(2)
            .any(|turn| WHOLE_FOOD.is_match(&turn.user))
    {
        return None;
    }
    if names(
        &words,
        &[
            "kids", "kid", "children", "child", "toddler", "toddlers", "family",
        ],
    ) {
        return Some(Cow::Borrowed(data::KIDS_IDEA));
    }
    let reading = context.reading;
    if let Some(memory) = context.memory
        && let Some(reply) = super::pantry_combination(reading, memory)
    {
        return Some(Cow::Borrowed(reply));
    }
    if let Some(reply) = kitchen::without(&reading.line) {
        return Some(Cow::Owned(reply));
    }
    if let Some(reply) = super::single_ingredient_meal(reading) {
        return Some(Cow::Borrowed(reply));
    }
    let named = data::IDEAS
        .iter()
        .position(|(forms, _)| names(&words, forms));
    if let Some(index) = named
        && earlier.clone().take(3).any(|turn| {
            let asked: Vec<_> = words
                .iter()
                .filter(|word| {
                    data::IDEAS[index]
                        .0
                        .iter()
                        .any(|form| dishes::same_word(word, form))
                })
                .collect();
            let kept = tokens(&turn.bot);
            asked.iter().any(|name| kept.contains(name))
        })
    {
        return None;
    }
    if words.iter().any(|word| word == "or")
        && let Some((_, reply)) = data::COMPARE_IDEAS
            .iter()
            .find(|(phrases, _)| phrases.iter().all(|term| literal(text, term)))
    {
        return Some(Cow::Borrowed(reply));
    }
    let device = appliance(text).or_else(|| {
        BACKREF.is_match(text).then_some(())?;
        earlier
            .clone()
            .take(3)
            .filter(|turn| !NO_APPLIANCE.is_match(&turn.user))
            .find_map(|turn| appliance(&turn.user))
    });
    if let Some(reply) = device {
        return Some(Cow::Borrowed(reply));
    }
    if names(&words, &["cheap", "cheaper", "cheapest", "budget", "broke"]) {
        return Some(Cow::Borrowed(data::CHEAP_IDEA));
    }
    if names(&words, &["tomorrow", "tomorrows"]) {
        return Some(Cow::Borrowed(data::AHEAD_IDEA));
    }
    if named.is_none() && (names(&words, data::GUEST_WORDS) || names(&words, data::NICE_WORDS)) {
        return Some(Cow::Borrowed(data::NICE_IDEA));
    }
    if named.is_none() && !names(&words, data::PICK_WORDS) {
        return None;
    }
    let initial = named.unwrap_or(0);
    let reply = data::IDEAS[initial].1;
    let reply = if recently_said(context, reply) {
        data::IDEAS
            .iter()
            .enumerate()
            .find(|(index, (_, answer))| *index != initial && !recently_said(context, answer))
            .map_or(reply, |(_, (_, answer))| *answer)
    } else {
        reply
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn registered(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    if context
        .memory
        .is_some_and(|memory| memory.pantry.blocks_typed_offer(memory))
        && id != "plan-problem"
    {
        return None;
    }
    match id {
        "plan-problem" => problem(context).map(Cow::Borrowed),
        "plan-dislike" => swap(context),
        "plan-ideas" => ideas(context),
        _ => None,
    }
    .filter(|reply| {
        context
            .memory
            .is_none_or(|memory| !memory.pantry.reply_uses_blocked_food(reply, memory))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_faults_stay_with_the_named_dish() {
        let reading = Reading::new("why are my muffins always dry?");
        let reply = problem(&Context::new(&reading, &[])).unwrap();
        assert!(reply.contains("oven"), "{reply}");
        let reading = Reading::new("why is the soil always dry?");
        assert!(problem(&Context::new(&reading, &[])).is_none());
    }

    #[test]
    fn chicken_substitution_uses_the_approved_doneness_check() {
        let reading = Reading::new("my partner hates beef");
        let reply = swap(&Context::new(&reading, &[])).unwrap();
        assert!(reply.contains("chicken thighs"), "{reply}");
        assert!(reply.contains("70°C for 2 minutes"), "{reply}");
        assert!(!reply.contains("same time"), "{reply}");
    }
}
