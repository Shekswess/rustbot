//! Ingredient-only stock and cooking juice substitutions for meat-free replies.

use super::{dietary_menus, guards::Context};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

static PLANT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:vegan|vegetarian|plant[- ]based|dairy[- ]free|egg free|margarine|flax egg|flaxseed|oat milk|soya milk|soy milk|almond milk|oat cream|soya cream|nutritional yeast|aquafaba)\b")
        .expect("valid plant ingredient cues")
});

static CLAUSE_END: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[.!?:,]\s+").expect("valid ingredient clause separator"));

static SWAP_CUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:in place of|instead of|rather than|or use|substitute for|swap for|no need for|works too)\b")
        .expect("valid existing swap cues")
});

static SWAPS: LazyLock<[(Regex, &str); 3]> = LazyLock::new(|| {
    [
        (r"(?i)\bmeat juices\b", "vegetable juices"),
        (
            r"(?i)\b(?:chicken|beef|ham|turkey|lamb|pork|fish)\s+stock\b",
            "vegetable stock",
        ),
        (
            r"(?i)\b(?:chicken|beef|ham|turkey|lamb|pork)\s+stocks?\s+cubes?\b",
            "vegetable stock cubes",
        ),
    ]
    .map(|(pattern, replacement)| {
        (
            Regex::new(pattern).expect("valid ingredient to replace"),
            replacement,
        )
    })
});

pub(super) fn fit(reply: Cow<'static, str>, context: &Context<'_>) -> Cow<'static, str> {
    if !(reply.contains("juices") || reply.contains("stock")) {
        return reply;
    }
    let needs_swap = context
        .memory
        .is_some_and(|memory| memory.personal.avoids_meat_or_fish())
        || dietary_menus::requested_meal(context.reading, context.memory).is_some();
    if needs_swap {
        let revised = swap(&reply, context.reading.raw);
        if let Cow::Owned(revised) = revised {
            return Cow::Owned(revised);
        }
    }
    reply
}

fn swap<'a>(reply: &'a str, input: &str) -> Cow<'a, str> {
    if PLANT.is_match(reply) {
        return Cow::Borrowed(reply);
    }
    let applicable: Vec<_> = SWAPS
        .iter()
        .filter(|(pattern, _)| pattern.is_match(reply) && !pattern.is_match(input))
        .collect();
    if applicable.is_empty() {
        return Cow::Borrowed(reply);
    }
    let mut start = 0;
    let mut revised = String::with_capacity(reply.len());
    for separator in CLAUSE_END.find_iter(reply) {
        replace_clause(&reply[start..separator.start()], &applicable, &mut revised);
        revised.push_str(separator.as_str());
        start = separator.end();
    }
    replace_clause(&reply[start..], &applicable, &mut revised);
    Cow::Owned(revised)
}

fn replace_clause(clause: &str, applicable: &[&(Regex, &str)], revised: &mut String) {
    if SWAP_CUE.is_match(clause) {
        revised.push_str(clause);
        return;
    }
    let mut part = Cow::Borrowed(clause);
    for (pattern, replacement) in applicable {
        if let Cow::Owned(changed) = pattern.replace_all(&part, *replacement) {
            part = Cow::Owned(changed);
        }
    }
    revised.push_str(&part);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn only_ingredient_use_changes_and_existing_swaps_stay_intact() {
        let reply = "Whisk flour into the meat juices, add chicken stock and use oil instead of chicken stock in the other dish.";
        assert_eq!(
            swap(reply, "My gravy is lumpy"),
            "Whisk flour into the vegetable juices, add chicken stock and use oil instead of chicken stock in the other dish."
        );
        assert_eq!(
            swap(
                "Use beef stock to thin it.",
                "How much beef stock goes in gravy?"
            ),
            "Use beef stock to thin it."
        );
        assert_eq!(
            swap(
                "Use a pinch of salt and chicken stock.",
                "How can I thin the gravy?"
            ),
            "Use a pinch of salt and vegetable stock."
        );
    }

    #[test]
    fn remembered_vegetarian_choice_reaches_the_registered_gravy_card() {
        let mut bot = Bot::new(Some("23")).unwrap();
        bot.turn("I'm vegetarian");
        let answer = bot.turn("How do I fix lumpy gravy?");
        assert!(
            answer.reply.contains("vegetable juices"),
            "{}",
            answer.reply
        );
        assert!(!answer.reply.contains("meat juices"));
    }
}
