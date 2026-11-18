//! Diet plan replies, their source fences, and the no-repeat rule.

use super::guards::Context;
use super::{
    DIET_DAIRY_FREE_VEGGIE, DIET_MEAL_ASK_PATTERN, DIET_NOT_THIS_PATTERN, DIET_PLANS,
    DIET_RECALL_PATTERN, DIET_TOO_MEDICAL_PATTERN, DIET_VEGGIE_KEY, MARK_LANDED_ON_PATTERN,
    MARK_SURFACES,
};
use crate::bot::{Reading, knowledge::dishes::names, understanding::normalize::tokens};
use fancy_regex::Regex;
use std::sync::LazyLock;

static OTHER_DIET_ASKS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        DIET_RECALL_PATTERN,
        DIET_NOT_THIS_PATTERN,
        DIET_TOO_MEDICAL_PATTERN,
        DIET_MEAL_ASK_PATTERN,
    ]
    .into_iter()
    .map(|pattern| Regex::new(pattern).expect("valid source diet condition"))
    .collect()
});
static LANDED_ON: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(MARK_LANDED_ON_PATTERN).expect("valid source mark pattern"));

struct Plan {
    phrases: Vec<Vec<String>>,
    reply: &'static str,
}

static PLANS: LazyLock<Vec<Plan>> = LazyLock::new(|| {
    DIET_PLANS
        .iter()
        .map(|(phrases, reply)| Plan {
            phrases: phrases.iter().map(|phrase| tokens(phrase)).collect(),
            reply,
        })
        .collect()
});
static VEGGIE: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| {
    DIET_VEGGIE_KEY
        .iter()
        .map(|phrase| tokens(phrase))
        .collect()
});

pub(super) fn is_not_a_mark(reading: &Reading<'_>) -> bool {
    let line = if reading.raw.is_empty() {
        &reading.line.clean
    } else {
        reading.raw
    };
    let text = format!(" {} ", line.to_lowercase());
    if LANDED_ON
        .captures_iter(&text)
        .any(|hit| MARK_SURFACES.contains(&hit[1].as_ref()))
    {
        return false;
    }
    let pronoun = [
        " on it ",
        " on them ",
        " on those ",
        " on that ",
        " on this ",
    ]
    .iter()
    .any(|phrase| text.contains(phrase));
    !pronoun || !MARK_SURFACES.iter().any(|word| text.contains(word))
}

pub(super) fn swaps(context: &Context<'_>) -> Option<&'static str> {
    let reading = context.reading;
    let line = if reading.line.norm_text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.norm_text
    };
    if line.trim().is_empty()
        || OTHER_DIET_ASKS
            .iter()
            .any(|pattern| pattern.is_match(line).unwrap_or(false))
    {
        return None;
    }
    let words = tokens(line);
    let plan = PLANS.iter().find(|plan| names(&words, &plan.phrases))?;
    if names(&words, &VEGGIE) {
        return Some(DIET_DAIRY_FREE_VEGGIE);
    }
    let repeated = context.earlier.last().is_some_and(|turn| {
        turn.bot.to_lowercase().split_whitespace().take(4).eq(plan
            .reply
            .to_lowercase()
            .split_whitespace()
            .take(4))
    });
    (!repeated).then_some(plan.reply)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn source_diet_card_answers_the_restriction_and_yields_on_a_repeat() {
        let reading = Reading::new("im dairy free");
        let card = super::super::lookup(reading.raw).expect("diet card");
        assert_eq!(card.id, "plan-diet");
        let earlier = [Exchange {
            user: reading.raw.to_owned(),
            bot: card.reply.to_string(),
            module: "answers",
        }];
        let joined = Reading::new("im dairy free what can i do with a courgette");
        assert_eq!(swaps(&Context::new(&joined, &earlier)), None);
        assert_eq!(
            super::super::single_ingredient_meal(&joined),
            Some("Slice the courgette thick, oil and salt it, and grill 4 minutes a side."),
        );
        for text in [
            "I can eat dairy",
            "I'm allergic to nuts",
            "I'm dairy free, what can I cook for dinner?",
        ] {
            let reading = Reading::new(text);
            assert_eq!(swaps(&Context::new(&reading, &[])), None, "{text}");
        }
        assert!(!is_not_a_mark(&Reading::new("There is egg on my shirt")));
    }
}
