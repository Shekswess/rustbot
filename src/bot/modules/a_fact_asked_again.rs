//! Recall a filed fact according to the kind of information asked for again.

use std::sync::LazyLock;

use crate::{
    bot::{Memory, Reading},
    text::is_space,
};
use regex::Regex;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "a_fact_asked_again_data.rs"]
mod data;
use data::*;

pattern!(
    NAME,
    r"(?i)\b(?:vauxhall|ford|toyota|nissan|bmw|audi|honda|ikea|argos|january|february|march|april|may|june|july|august|september|october|november|december|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\b"
);
pattern!(GOAL, r"\bgoals?\b");
pattern!(AIM, r"\baims?\b");
pattern!(TARGET, r"\btargets?\b");

static PLAN_DATES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    DATES
        .iter()
        .map(|source| Regex::new(source).expect("ordered source plan date"))
        .collect()
});

fn flat(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn in_plan(plan: &str) -> Option<String> {
    PLAN_DATES
        .iter()
        .find_map(|pattern| pattern.find(plan))
        .map(|found| flat(found.as_str()))
}

fn time(memory: &Memory) -> Option<String> {
    let mut dates = memory.stated_plans.values().filter_map(in_plan);
    let date = dates.next()?;
    if dates.next().is_some() {
        return None;
    }
    Some(format!("You said {date}."))
}

fn allergy(memory: &Memory) -> Option<String> {
    let (who, what) = memory.personal.last_other_allergy()?;
    let mut chars = what.chars();
    let first = chars.next()?;
    let article: String = first.to_uppercase().collect();
    let rest = chars.as_str().to_lowercase();
    if who.is_empty() {
        Some(format!("{article}{rest}."))
    } else {
        Some(format!("{article}{rest} - your {who} is allergic to them."))
    }
}

fn target(text: &str, plan: &str) -> String {
    let low = plan.to_lowercase();
    let plan = PLAN_OPENS
        .iter()
        .find(|opener| low.starts_with(*opener))
        .map_or(plan, |opener| &plan[opener.len()..]);
    let noun = if GOAL.is_match(text) {
        "goal"
    } else if AIM.is_match(text) {
        "aim"
    } else if TARGET.is_match(text) {
        "target"
    } else {
        "plan"
    };
    let mut chars = plan.chars();
    let first = chars.next();
    let name = plan
        .split_whitespace()
        .next()
        .is_some_and(|word| NAME.is_match(word));
    let plan = if first.is_some_and(char::is_uppercase)
        && chars.next().is_some_and(char::is_lowercase)
        && !name
    {
        let first = first.unwrap();
        format!("{}{}", first.to_lowercase(), &plan[first.len_utf8()..])
    } else {
        plan.to_owned()
    };
    format!(
        "You said your {noun} is to {}.",
        plan.trim_end_matches(['.', '!', '?'])
    )
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let low = flat(reading.raw).to_lowercase();
    if !AGAIN.is_match(&low) || !AN_ASK.is_match(&low) || NOT_A_RECALL.is_match(&low) {
        return None;
    }
    if A_TIME.is_match(&low) {
        if memory.personal.recall_when(reading, memory).is_some() {
            return None;
        }
        return time(memory);
    }
    if A_THING_TO_AVOID.is_match(&low) {
        return allergy(memory);
    }
    if A_TARGET.is_match(&low) {
        return Some(target(&low, memory.stated_plans.values().next_back()?));
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{Bot, Turn};

    fn from_hook(turn: &Turn) -> Option<&str> {
        turn.candidates
            .iter()
            .find(|candidate| candidate.module == "a_fact_asked_again")
            .map(|candidate| candidate.reply.as_str())
    }

    #[test]
    fn recalls_the_filed_target_and_family_allergy() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("im 14 stone");
        bot.turn("i want to get to 12");
        let target = bot.turn("and whats my target again");
        assert_eq!(
            from_hook(&target),
            Some("You said your target is to get to 12.")
        );
        assert_eq!(target.module, Some("a_fact_asked_again"));
        assert_eq!(from_hook(&bot.turn("should i eat less again")), None);
        bot.turn("my daughter is allergic to nuts");
        let allergy = bot.turn("what was i keeping away from again");
        assert_eq!(
            from_hook(&allergy),
            Some("Nuts - your daughter is allergic to them.")
        );
    }

    #[test]
    fn the_date_fallback_yields_to_a_named_trip_and_declines_two_plans() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("im going to york in june");
        let trip = bot.turn("when is my trip again");
        assert_eq!(from_hook(&trip), None);
        assert_eq!(trip.module, Some("memory_store"));
        assert_eq!(trip.reply, "You're going to York in June.");
        let fallback = bot.turn("when is my return again");
        assert_eq!(from_hook(&fallback), Some("You said june."));
        bot.turn("my sister is coming in july");
        assert_eq!(from_hook(&bot.turn("when is my return again")), None);

        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("im going to buy a car in june");
        let action = bot.turn("when is my trip again");
        assert_eq!(from_hook(&action), None);
        assert_eq!(action.module, Some("memory_store"));
        assert_eq!(action.reply, "In June.");
    }
}
