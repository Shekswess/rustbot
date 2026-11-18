//! Meals to carry, counter orders, and desserts from the original dining tables.

use super::{diet_plan, guards::Context};
use crate::bot::{
    Reading, compose::seeded_choice_at, knowledge::dishes::same_word,
    understanding::normalize::tokens,
};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "meal_scenes_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $re:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($re).expect(stringify!($name)));
    };
}
pattern!(COUNTER_DEFINITION, data::COUNTER_DEFINITION);
pattern!(COUNTER_VERDICT, r"^(?:is|are|was|were)\b");
pattern!(ATE_ALREADY, data::ATE_ALREADY);
pattern!(REWARDED, data::REWARDED);
pattern!(PUDDING_ASK, data::PUDDING_ASK);
pattern!(
    OTHER_LIMIT,
    r"\b(?:allerg\w*|coeliac|celiac|gluten[- ]free|wheat[- ]free|diabetic|diabetes|intoleran\w*)\b"
);
pattern!(
    DAIRY_FREE,
    r"\b(?:dairy[- ]free|egg[- ]free|vegan|plant[- ]based|no dairy|no eggs)\b"
);
pattern!(CARRIED_VEGAN, r"\bvegan\b");

fn mentions(text: &str, cues: &[&str]) -> bool {
    cues.iter().any(|cue| text.contains(cue))
}

pub(super) fn literal(text: &str, term: &str) -> bool {
    text.match_indices(term).any(|(start, _)| {
        let end = start + term.len();
        let word = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_';
        (start == 0 || !word(text.as_bytes()[start - 1]))
            && (end == text.len() || !word(text.as_bytes()[end]))
    })
}

fn counter_matches(text: &str) -> impl Iterator<Item = (&'static str, usize)> {
    data::COUNTERS.iter().filter_map(move |(names, reply)| {
        names
            .iter()
            .filter(|name| literal(text, name))
            .map(|name| name.len())
            .max()
            .map(|size| (*reply, size))
    })
}

pub(super) fn one_counter(reading: &Reading<'_>) -> bool {
    let line = &reading.line.clean;
    !COUNTER_VERDICT.is_match(line)
        && COUNTER_DEFINITION
            .captures(line)
            .is_none_or(|hit| data::COUNTER_ASK.contains(&hit[1].as_ref()))
        && counter_matches(line).take(2).count() <= 1
}

fn carry(context: &Context<'_>) -> Option<String> {
    let line = &context.reading.line.clean;
    let travel = [
        "flight",
        "flights",
        "plane",
        "airport",
        "airline",
        "cancelled",
        "canceled",
        "delayed",
        "delay",
        "refund",
        "owed",
        "compensation",
        "lighter",
        "luggage",
        "baggage",
        "security",
        "passport",
        "check in",
        "check-in",
    ];
    let transport = [
        "flight", "flights", "plane", "airport", "airline", "security",
    ];
    let permission = [
        "can i take",
        "can i bring",
        "can we take",
        "can we bring",
        "could i take",
        "could i bring",
        "am i allowed",
        "are we allowed",
        "allowed to take",
        "allowed to bring",
        "allowed on",
        "100ml",
        "liquids",
    ];
    let food = [
        "food",
        "eat",
        "eating",
        "lunch",
        "snack",
        "snacks",
        "picnic",
        "sandwich",
        "meal",
        "meals",
        "drink",
        "drinks",
        "breakfast",
        "dinner",
        "packed",
    ];
    let asks = [
        "make",
        "take",
        "bring",
        "pack",
        "packing",
        "food",
        "eat",
        "eating",
        "lunch",
        "idea",
        "ideas",
        "need",
        "snack",
        "snacks",
        "something",
        "anything",
        "for a picnic",
        "on a picnic",
        "day out",
        "veggie",
        "vegetarian",
        "vegan",
        "no meat",
        "gluten",
        "kid",
        "kids",
        "children",
        "child",
        "toddler",
        "people",
        "family",
        "friends",
    ];
    let vegan = CARRIED_VEGAN.is_match(line);
    let vegetarian = vegan || mentions(line, &["veggie", "vegetarian", "no meat"]);
    if !mentions(line, &asks)
        || mentions(
            line,
            &["went to", "was at", "we had", "yesterday", "last week"],
        )
        || mentions(
            line,
            &[
                "packed lunch",
                "lunchbox",
                "lunch box",
                "school lunch",
                "bento",
            ],
        )
        || mentions(line, &travel) && !mentions(line, &food)
        || mentions(line, &transport) && mentions(line, &permission)
        || OTHER_LIMIT.is_match(line)
        || !vegan && DAIRY_FREE.is_match(line)
        || context.memory.is_some_and(|memory| {
            if vegetarian {
                memory.pantry.blocks_plant_offer(memory, vegan)
            } else {
                memory.pantry.blocks_typed_offer(memory)
            }
        })
    {
        return None;
    }
    let plain = seeded_choice_at(data::CARRY, context.earlier.len(), line);
    let mut reply = if vegan && plain == data::CARRY[0] {
        "Things that travel well: vegan oatcakes, cherry tomatoes, and fruit that survives a bag. Pack anything wet in its own box and take a bag for the rubbish.".to_owned()
    } else if vegan {
        "Cold and easy to carry: vegan oatcakes or bread, tomatoes and a bag of grapes. Freeze a bottle of water the night before - it keeps the food cold and then it is your drink.".to_owned()
    } else if vegetarian {
        plain
            .replace("cold sausages or sausage rolls, ", "")
            .replace("sausage rolls, ", "")
    } else {
        plain.to_owned()
    };
    if mentions(
        line,
        &["kids", "children", "child", "toddler", "little ones"],
    ) {
        if vegan {
            reply.push_str(
                " For small people, add something they will actually eat: vegan breadsticks.",
            );
        } else {
            reply.push_str(
                " For small people, add something they will actually eat: breadsticks, cheese.",
            );
        }
    }
    Some(reply)
}

/// A picnic meal explicitly requested as vegan should keep only the plant options.
pub(super) fn vegan_carry(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    CARRIED_VEGAN.is_match(text)
        && mentions(
            text,
            &[
                "picnic",
                "beach",
                "day out",
                "day trip",
                "road trip",
                "train journey",
                "long journey",
                "flight",
                "cool box",
                "coolbox",
                "cooler box",
                "flask",
            ],
        )
        && mentions(
            text,
            &["food", "lunch", "snack", "idea", "meal", "eat", "dinner"],
        )
}

pub(super) fn disagrees_with_vegan_carry(reply: &str) -> bool {
    let text = reply.to_lowercase();
    mentions(
        &text,
        &[
            "cheese", "sausage", "frittata", "quiche", "butter", "chicken", "ham", "tuna",
        ],
    )
}

fn pudding(context: &Context<'_>) -> Option<&'static str> {
    let line = &context.reading.line.norm_text;
    let words = tokens(line);
    if !words
        .iter()
        .any(|word| data::DESSERT_WORDS.contains(&word.as_str()))
        || REWARDED.is_match(line)
        || ATE_ALREADY.is_match(line) && !PUDDING_ASK.is_match(line)
        || diet_plan::swaps(context).is_some()
        || DAIRY_FREE.is_match(line)
        || OTHER_LIMIT.is_match(line)
        || context
            .memory
            .is_some_and(|memory| memory.pantry.blocks_plant_offer(memory, false))
    {
        return None;
    }
    if let Some((_, reply)) = data::PUDDINGS.iter().find(|(phrases, _)| {
        phrases
            .iter()
            .any(|phrase| literal(line, phrase) || words.iter().any(|word| same_word(word, phrase)))
    }) {
        return Some(reply);
    }
    let crowd = |text: &str| {
        tokens(text).iter().any(|word| {
            data::CROWD_WORDS.contains(&word.as_str())
                || word.parse::<u32>().is_ok_and(|n| (6..=200).contains(&n))
        })
    };
    if crowd(line)
        || context
            .earlier
            .iter()
            .rev()
            .take(4)
            .any(|turn| crowd(&turn.user))
    {
        return Some(data::CROWD_PUDDING);
    }
    Some(data::FRUIT_PUDDING)
}

pub(super) fn registered(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    match id {
        "meal-carry" => carry(context).map(Cow::Owned),
        "pick-at-the-counter" => {
            if context
                .memory
                .is_some_and(|memory| memory.pantry.blocks_typed_offer(memory))
            {
                return None;
            }
            counter_matches(&context.reading.line.clean)
                .max_by_key(|(_, size)| *size)
                .map(|(reply, _)| Cow::Borrowed(reply))
        }
        "plan-pudding" => pudding(context).map(Cow::Borrowed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vegan_picnic_keeps_source_plant_foods_and_omits_animal_foods() {
        let reading = Reading::new("vegan picnic ideas for children");
        let reply = carry(&Context::new(&reading, &[])).unwrap();
        assert!(
            reply.contains("vegan oatcakes") && reply.contains("vegan breadsticks"),
            "{reply}"
        );
        assert!(
            !mentions(&reply, &["frittata", "quiche", "cheese", "sausage"]),
            "{reply}"
        );
    }

    #[test]
    fn counter_definitions_and_comparisons_keep_other_answer_routes() {
        assert!(one_counter(&Reading::new("what's nice from the bakery")));
        for question in [
            "what is a patisserie",
            "is my bakery nice",
            "what is best pizza or bakery",
        ] {
            assert!(!one_counter(&Reading::new(question)), "{question}");
        }
    }
}
