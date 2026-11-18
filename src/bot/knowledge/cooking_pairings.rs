//! Authored meal planning and pairing callbacks shared by the cooking rows.

use super::food_fold;
use crate::bot::{Exchange, Reading, knowledge::dishes, understanding::normalize::tokens};
use regex::{Regex, RegexSet};
use std::{cmp::Reverse, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $expression:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($expression).expect(stringify!($name)));
    };
}

pattern!(SEASONING_ASKS, super::PAIRING_SEASONING_ASKS);
pattern!(TIME_OR_HEAT, super::PAIRING_A_TIME_OR_HEAT_ASK);
pattern!(MONTHS, super::PAIRING_MONTHS_SAID);
pattern!(GOING, super::PAIRING_GOING_SAID);
pattern!(GOES_WITH, super::PAIRING_GOES_WITH_SHAPE);
pattern!(
    A_RUB,
    r"\b(?:dry rub|spice rub|make a rub|what goes in a rub|rub recipe)\b"
);
pattern!(
    NON_FOOD,
    r"\bpain\b|\bstitch\b|\bside effects?\b|\bleft side\b|\bright side\b"
);
pattern!(SHORT_BUDGET, r"\b([0-9]{1,3})\s*(?:minutes?|mins?)\b");
pattern!(FREEZING_OR_BATCH, r"\bfreez|\bbatch");
pattern!(WORD, r"[a-z']+");
pattern!(
    PLANT_DISH,
    r"\b(?:vegetarian|veggie|vego|vegan|plant[- ]based|plantbased|meat[- ]free) (?:mince|burgers?|sausages?|bangers?|steaks?|chicken|duck|pork|fish|lamb|beef)\b"
);
pattern!(
    MEAT_OR_FISH,
    r"(?i)\b(?:meat|sausages?|bacon|ham|steaks?|chicken|turkey|duck|pork|fish|lamb|beef|anchovy|anchovies)\b"
);

type PairingRows = &'static [(&'static [&'static str], &'static str)];

struct PhraseTable {
    names: RegexSet,
    replies: Vec<&'static str>,
}

impl PhraseTable {
    fn new(rows: PairingRows) -> Self {
        let mut aliases: Vec<_> = rows
            .iter()
            .flat_map(|(names, reply)| names.iter().map(move |name| (*name, *reply)))
            .collect();
        // Source takes the longest matching name and keeps the earliest row on a tie.
        aliases.sort_by_key(|(name, _)| Reverse(name.chars().count()));
        let names = RegexSet::new(
            aliases
                .iter()
                .map(|(name, _)| format!(r"\b{}\b", regex::escape(name))),
        )
        .expect("literal cooking pairing names");
        Self {
            names,
            replies: aliases.iter().map(|(_, reply)| *reply).collect(),
        }
    }

    fn find(&self, text: &str) -> Option<&'static str> {
        self.names
            .matches(text)
            .iter()
            .next()
            .map(|index| self.replies[index])
    }
}

static SIDES: LazyLock<PhraseTable> = LazyLock::new(|| PhraseTable::new(super::SIDE_REPLIES));
static SEASONINGS: LazyLock<PhraseTable> =
    LazyLock::new(|| PhraseTable::new(super::SEASONING_REPLIES));

static PAIRABLE: LazyLock<Vec<Vec<String>>> =
    LazyLock::new(|| phrases(super::PAIRING_PAIRING_FOODS));
static RICH: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| phrases(super::PAIRING_RICH_FOODS));
static PLAIN: LazyLock<Vec<Vec<String>>> = LazyLock::new(|| phrases(super::PAIRING_PLAIN_FOODS));

fn phrases(names: &[&str]) -> Vec<Vec<String>> {
    names.iter().map(|name| tokens(name)).collect()
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<&'static str> {
    let line = &reading.line;
    let text = if line.clean.is_empty() {
        line.words.join(" ").to_lowercase().into()
    } else {
        std::borrow::Cow::Borrowed(line.clean.as_str())
    };
    match id {
        "meal-sides" => sides(&text, earlier),
        "a-side-dish" => sides(&text, earlier).or_else(|| {
            // The generic side card checks the current plan and up to three recent user lines.
            plan_side(&text, earlier)
                .or_else(|| {
                    std::iter::once(text.as_ref())
                        .chain(earlier.iter().rev().take(3).map(|turn| turn.user.as_str()))
                        .find_map(|line| SIDES.find(&line.to_lowercase()))
                })
                .or(Some(super::PAIRING_A_SIDE_WITH_NO_DISH))
        }),
        "meal-plan" => meal_plan(&text, &line.words, earlier),
        "time-budget" => time_budget(&line.words),
        _ => None,
    }
}

pub(super) fn fits_plant_dish(reading: &Reading<'_>, reply: &str) -> bool {
    !PLANT_DISH.is_match(&reading.line.clean)
        || !names_meat_or_fish(reply.split_once(". ").map_or(reply, |(first, _)| first))
}

pub(super) fn names_meat_or_fish(text: &str) -> bool {
    MEAT_OR_FISH.is_match(text)
}

fn sides(text: &str, earlier: &[Exchange]) -> Option<&'static str> {
    if NON_FOOD.is_match(text) || TIME_OR_HEAT.is_match(text) {
        return None;
    }
    if SEASONING_ASKS.is_match(text)
        && let Some(reply) = SEASONINGS
            .find(text)
            .or_else(|| A_RUB.is_match(text).then_some(super::PAIRING_A_RUB))
    {
        return Some(reply);
    }
    if let Some(reply) = plan_side(text, earlier).or_else(|| SIDES.find(text)) {
        return Some(reply);
    }
    generic_pairing(text)
}

pub(super) fn asks_to_pair(text: &str) -> bool {
    GOES_WITH.is_match(text)
}

pub(super) fn generic_pairing(text: &str) -> Option<&'static str> {
    if !asks_to_pair(text) {
        return None;
    }
    let words = tokens(text);
    dishes::names(&words, &PAIRABLE).then(|| {
        if dishes::names(&words, &RICH) {
            super::PAIRING_PAIRING_RICH
        } else if dishes::names(&words, &PLAIN) {
            super::PAIRING_PAIRING_PLAIN
        } else {
            super::PAIRING_PAIRING_LEAN
        }
    })
}

fn plan_side(text: &str, earlier: &[Exchange]) -> Option<&'static str> {
    let plan = std::iter::once(text)
        .chain(earlier.iter().rev().take(2).map(|turn| turn.user.as_str()))
        .find_map(|line| dishes::plan_label(&line.to_lowercase()))?;
    super::DISH_PLAN_SIDES
        .iter()
        .find_map(|&(label, reply)| (label == plan).then_some(reply))
}

fn meal_plan(text: &str, words: &[String], earlier: &[Exchange]) -> Option<&'static str> {
    let text = std::iter::once(text)
        .chain(
            earlier
                .iter()
                .rev()
                .take(4)
                .rev()
                .map(|turn| turn.user.as_str()),
        )
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let folded: Vec<_> = words
        .iter()
        .map(String::as_str)
        .chain(WORD.find_iter(&text).map(|found| found.as_str()))
        .map(food_fold::fold)
        .collect();
    let has = |terms: &[&str]| {
        folded
            .iter()
            .any(|word| terms.binary_search(&word.as_str()).is_ok())
    };
    if (MONTHS.is_match(&text) || GOING.is_match(&text)) && !has(super::PAIRING_EATING_WORDS) {
        return None;
    }
    Some(
        if has(super::PAIRING_PLAN_WORDS) || FREEZING_OR_BATCH.is_match(&text) {
            super::PAIRING_PLAN_BATCH
        } else if has(super::PAIRING_CHEAP_PLAN_WORDS) {
            super::PAIRING_PLAN_CHEAP
        } else {
            super::PAIRING_PLAN_WEEK
        },
    )
}

static TIME_WORDS: LazyLock<Vec<(Regex, u16)>> = LazyLock::new(|| {
    super::PAIRING_TIME_WORDS
        .iter()
        .map(|&(word, minutes)| {
            (
                Regex::new(&format!(r"\b{}\b", regex::escape(word))).expect("literal minute word"),
                minutes,
            )
        })
        .collect()
});

fn time_budget(words: &[String]) -> Option<&'static str> {
    let said = words
        .iter()
        .map(|word| food_fold::fold(word))
        .collect::<Vec<_>>()
        .join(" ");
    let minutes = SHORT_BUDGET
        .captures(&said)
        .and_then(|hit| hit[1].parse().ok())
        .or_else(|| {
            super::PAIRING_TIME_PHRASES
                .iter()
                .find_map(|&(phrase, minutes)| said.contains(phrase).then_some(minutes))
        })
        .or_else(|| {
            TIME_WORDS
                .iter()
                .find_map(|(word, minutes)| word.is_match(&said).then_some(*minutes))
        })?;
    match minutes {
        0..=15 => Some(super::PAIRING_ANY_MEAL),
        16..=22 => Some(super::PAIRING_TIME_TWENTY),
        23..=28 => Some(super::PAIRING_TIME_SOON),
        29..=90 => Some(super::PAIRING_TIME_MIDDLING),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/cooking_pairings.rs"
    ));

    #[test]
    fn authored_pairings_and_plans_match_source_callbacks() {
        let mut differences = Vec::new();
        for &(id, line, prior, expected) in CASES {
            let row = super::super::rows()
                .find(|row| row.domain == "cooking" && row.id == id)
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
    fn meal_plans_and_plant_dishes_route_through_the_conversation() {
        let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
        let turn = bot.turn("I am making a meal plan for next week.");
        assert_eq!(turn.module, Some("answers"));
        assert_eq!(turn.reply, super::super::PAIRING_PLAN_WEEK);

        bot.reset().unwrap();
        let turn = bot.turn("What seasoning goes with vegan mince?");
        assert_eq!(turn.module, Some("the_shape_of_the_answer"));
        assert!(
            turn.reply
                .starts_with("Nothing in front of me to suggest from.")
        );
    }
}
