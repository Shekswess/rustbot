//! Planning cards that choose a reply from the current wording.

use crate::bot::{Reading, compose::said_again, memory::groupages::decimal};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

use super::{dish_plan, fun, guards::Context, leisure};

#[path = "planning_home.rs"]
mod home;

#[path = "planning_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    OWN_COUNT,
    A_DATE_ASK,
    A_DO,
    HOME_IN_THE_LINE,
    A_MOVE_TO_PLAN,
    A_MOVE_ELSEWHERE,
    FOR_A_RAINY_DAY,
    MONEY_WITH_IT,
    BARE_RID,
    TOO_MUCH,
    A_MILESTONE_PERSON,
    A_MILESTONE_AGE,
    A_CHILD_AT_A_MILESTONE,
    A_MILESTONE_SOON,
    A_PRESENT_ASKED,
    AGE_AT_PARTY
);
static CHILD_AT_PARTY: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        data::CHILD_AT_PARTY
            .iter()
            .map(|phrase| format!(r"\b{phrase}\b")),
    )
    .expect("words for children at a party")
});
static DECLUTTER_SUBJECT: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        data::JOB_NAMED
            .iter()
            .chain(data::DECLUTTER_ROOM)
            .chain(data::DECLUTTER_STUFF)
            .map(|phrase| format!(" {} ", regex::escape(phrase))),
    )
    .expect("words for a clear-out")
});

pub(super) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    match id {
        "move-house-plan" => {
            let said = said_words(reading);
            HOME_IN_THE_LINE.is_match(&said)
                || (A_MOVE_TO_PLAN.is_match(&said) && !A_MOVE_ELSEWHERE.is_match(&said))
        }
        "declutter-plan" => {
            let said = if line.clean.is_empty() {
                &line.text
            } else {
                &line.clean
            };
            let padded = format!(
                " {} ",
                said.split(crate::text::is_space)
                    .filter(|word| !word.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            BARE_RID.is_match(&padded)
                || TOO_MUCH.is_match(&padded)
                || DECLUTTER_SUBJECT.is_match(&padded)
        }
        "saving-plan" => {
            let said = said_words(reading);
            !FOR_A_RAINY_DAY.is_match(&said) || MONEY_WITH_IT.is_match(&said)
        }
        "party-kids" => !OWN_COUNT.is_match(&typed_text(reading)),
        "weekend-ideas" => weekend(reading).is_some(),
        "baby-gear" => home::asks_for_gear(reading),
        "party-milestone" => {
            let said = format!("{} {}", line.clean, line.words.join(" ")).to_lowercase();
            A_MILESTONE_PERSON.is_match(&said)
                && A_MILESTONE_AGE
                    .captures(&said)
                    .and_then(|captures| decimal(&captures[1]))
                    .is_some_and(|years| years >= 18 || A_CHILD_AT_A_MILESTONE.is_match(&said))
                && !A_PRESENT_ASKED.is_match(&said)
                && A_MILESTONE_SOON.is_match(&said)
        }
        "party-grownup" => {
            let said = format!(
                " {} {} ",
                line.clean.to_lowercase(),
                line.words.join(" ").to_lowercase()
            );
            let typed = typed_text(reading);
            !data::GIFT_ASK.iter().any(|phrase| said.contains(phrase))
                && (!A_DATE_ASK.is_match(&typed) || A_DO.is_match(&typed))
                && super::family_repair::not_their_deed(reading)
                && !super::family_repair::asks_amends(reading)
        }
        _ => false,
    }
}

pub(super) fn party(reading: &Reading<'_>) -> &'static str {
    let line = &reading.line;
    let said = format!("{} {}", line.clean.to_lowercase(), line.words.join(" "));
    let age = AGE_AT_PARTY
        .captures_iter(&said)
        .filter_map(|captures| decimal(&captures[1]))
        .filter(|age| (1..=99).contains(age))
        .min();
    if age.map_or_else(|| CHILD_AT_PARTY.is_match(&said), |age| age < 13) {
        data::KIDS_PARTY
    } else {
        data::ADULT_PARTY
    }
}

fn weekend(reading: &Reading<'_>) -> Option<&'static str> {
    let clean = &reading.line.clean;
    if let Some(action) = clean.strip_prefix("can i take ")
        && ["anything", "something"]
            .iter()
            .any(|word| action.starts_with(word))
    {
        return Some(
            "What are you hoping to treat? The right advice depends on the symptom and any other medicines you take. A pharmacist can help you check what is suitable.",
        );
    }
    if !leisure::cheap_day(reading)
        || [
            "what time is",
            "what time does",
            "when does",
            "what's happening in the world",
            "world news",
            "in the news",
            "news today",
            "latest news",
            "headlines",
            "what can we make",
            "what can i make",
            "what should we make",
            "what should i make",
        ]
        .iter()
        .any(|phrase| clean.contains(phrase))
    {
        return None;
    }
    if ["weekend", "saturday", "sunday", "bank holiday"]
        .iter()
        .any(|phrase| clean.contains(phrase))
    {
        Some(super::PLANS_FREE_WEEKEND)
    } else if ["tonight", "this evening"]
        .iter()
        .any(|phrase| clean.contains(phrase))
    {
        Some(super::PLANS_EVENING_PLAN)
    } else if [
        "today",
        "this morning",
        "this afternoon",
        "tomorrow",
        "day off",
    ]
    .iter()
    .any(|phrase| clean.contains(phrase))
    {
        Some(super::PLANS_DAY_OFF_PLAN)
    } else {
        Some(super::PLANS_FREE_WEEKEND)
    }
}

fn try_once(context: &Context<'_>, again: bool) -> Option<&'static str> {
    let reading = context.reading;
    let pool = super::PLANS_TRY_ONCE;
    if reading.context_fields.subject.is_some() {
        return None;
    }
    if again {
        let last = said_again(&context.earlier.last()?.bot);
        if !pool.iter().any(|idea| said_again(idea) == last) {
            return None;
        }
    } else if reading
        .own_topic
        .is_some_and(|(topic, _)| !matches!(topic, "small-talk" | "opinions" | "creative"))
    {
        return None;
    }
    Some(fun::pick(context, pool, &reading.line.words.join(" ")))
}

pub(super) fn registered(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    let reading = context.reading;
    match id {
        "party-grownup" | "party-milestone" => Some(Cow::Borrowed(party(reading))),
        "weekend-ideas" => weekend(reading).map(Cow::Borrowed),
        "baby-gear" => home::baby_gear(reading, context.earlier),
        "room-plan" => home::room(reading).map(Cow::Borrowed),
        "dish-for-tonight" => dish_plan::dinner_recipe(reading).map(Cow::Borrowed),
        "try-once" | "try-once-more" => try_once(context, id == "try-once-more").map(Cow::Borrowed),
        _ => None,
    }
}

fn said_words(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    [&line.clean, &line.text]
        .into_iter()
        .chain(&line.words)
        .filter(|part| !part.is_empty())
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn typed_text(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    if line.text.is_empty() {
        line.words.join(" ").to_lowercase()
    } else {
        line.text.to_lowercase()
    }
}

#[cfg(test)]
mod remaining_tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn planning_callbacks_distinguish_fixtures_gear_recipes_and_followups() {
        let room = Reading::new("How should I lay out a utility room?");
        assert!(home::room(&room).unwrap().contains("Gas Safe"));
        assert!(home::room(&Reading::new("I'm planning to redo my CV")).is_none());
        let gear = Reading::new("The brake on my pushchair is sticking");
        assert!(
            home::baby_gear(&gear, &[])
                .unwrap()
                .starts_with("Stop using")
        );
        let recipe = Reading::new("I fancy making bolognese tonight");
        assert!(dish_plan::dinner_recipe(&recipe).is_some());
        let more = Reading::new("Give me another one");
        assert!(try_once(&Context::new(&more, &[]), true).is_none());
        let earlier = [Exchange {
            user: String::new(),
            bot: super::super::PLANS_TRY_ONCE[0].to_owned(),
            module: "answers",
        }];
        assert!(try_once(&Context::new(&more, &earlier), true).is_some());
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("How should I plan the layout of my utility room?");
        assert!(turn.reply.contains("Gas Safe"), "{}", turn.reply);
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/planning.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planning_guards_and_party_replies_match_the_reference() {
        for fixtures::Case(id, prompt, guard, reply) in fixtures::CASES {
            let reading = Reading::new(prompt);
            assert_eq!(passes(id, &reading), *guard, "{id}: {prompt}");
            if let Some(reply) = reply {
                assert_eq!(party(&reading), *reply, "{id}: {prompt}");
            }
        }
    }
}
