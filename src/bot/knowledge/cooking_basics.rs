//! Small cooking decisions whose replies are authored in the pinned registry.

use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use crate::bot::{Reading, understanding::kinds::Kind};

use super::{cooking_data as data, food_fold, guards::Context};

#[derive(Clone, Copy)]
pub(super) enum Rule {
    Rice,
    Porridge,
    PorridgeFault,
    TomatoSauce,
    Thighs,
    Scramble,
    Mince,
    GrainyMince,
    Cookies,
    GrainySugar,
    Dough,
    Nonstick,
    Learn,
    Tea,
}

fn has(said: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| said.contains(phrase))
}

fn starts_with(words: &[String], cues: &[&str]) -> bool {
    words
        .iter()
        .any(|word| cues.iter().any(|cue| word.starts_with(cue)))
}

pub(super) fn reply(
    reading: &Reading<'_>,
    rule: Rule,
    texts: &'static [&'static str],
) -> Option<&'static str> {
    let words = &reading.line.words;
    let clean = &reading.line.clean;
    let index = match rule {
        Rule::Rice => {
            if words
                .iter()
                .any(|word| matches!(word.as_str(), "brown" | "wholegrain" | "whole"))
            {
                0
            } else if reading
                .features
                .asks
                .iter()
                .any(|ask| matches!(*ask, "how_long" | "when"))
            {
                1
            } else {
                2
            }
        }
        Rule::Porridge => usize::from(
            !(has(clean, &["microwave", "microwaving"])
                || clean.split_whitespace().any(|word| word == "micro")),
        ),
        Rule::PorridgeFault => {
            if has(clean, data::PORRIDGE_SALT) {
                0
            } else if has(clean, data::PORRIDGE_BLAND) {
                1
            } else if has(clean, &["overnight", "soak", "soaked", "in a jar"]) {
                2
            } else {
                3
            }
        }
        Rule::TomatoSauce => usize::from(!has(
            clean,
            &[
                "how long",
                "simmer",
                "minutes",
                "how many minutes",
                "time",
                "takes",
                "take",
            ],
        )),
        Rule::Thighs => usize::from(!has(&words.join(" "), &["lemon", "potato", "potatoes"])),
        Rule::Scramble => {
            let wet = &["watery", "wet", "sloppy"];
            let faults = &[
                "rubbery",
                "rubber",
                "tough",
                "dry",
                "hard",
                "overcooked",
                "chewy",
                "curdled",
                "mushy",
            ];
            if (reading.kind == Kind::Question || starts_with(words, &["how"]))
                && !starts_with(words, wet)
                && !starts_with(words, faults)
            {
                0
            } else if starts_with(words, wet) {
                1
            } else {
                2
            }
        }
        Rule::Mince => {
            if starts_with(words, data::KEEPING_SENSE) {
                return None;
            }
            if starts_with(words, &["tough", "rubbery", "chewy", "dry"]) {
                1
            } else {
                2
            }
        }
        Rule::GrainyMince => {
            if starts_with(words, data::KEEPING_SENSE)
                || MINCE_PIE.is_match(&words.join(" "))
                || (starts_with(words, data::BOLOGNESE_WORDS) && !starts_with(words, &["sauce"]))
            {
                return None;
            }
            3
        }
        Rule::Cookies => usize::from(!has(
            &words.join(" "),
            &[
                "didnt spread",
                "didn't spread",
                "did not spread",
                "not spread",
                "not spreading",
                "wont spread",
                "won't spread",
                "stayed in a ball",
                "stayed in balls",
                "too thick",
                "too small",
                "didnt flatten",
                "didn't flatten",
                "did not flatten",
                "stayed round",
            ],
        )),
        Rule::GrainySugar => [
            &[
                "icing",
                "frosting",
                "buttercream",
                "glaze",
                "fondant",
                "drizzle",
            ][..],
            &["fudge", "tablet", "praline", "brittle", "candy"],
            &["custard", "creme", "anglaise"],
            &["toffee", "caramel"],
        ]
        .iter()
        .position(|cues| starts_with(words, cues))
        .unwrap_or(4),
        Rule::Dough => {
            let more_flour = words
                .iter()
                .enumerate()
                .rev()
                .find_map(|(at, word)| {
                    let wet = data::DOUGH_WET.contains(&word.as_str());
                    let dry = data::DOUGH_DRY.contains(&word.as_str());
                    if !wet && !dry {
                        return None;
                    }
                    let negated = words[at.saturating_sub(2)..at].iter().any(|before| {
                        data::DOUGH_NOT.contains(&before.as_str()) || before.ends_with("n't")
                    });
                    Some(wet != negated)
                })
                .unwrap_or(false);
            usize::from(!more_flour)
        }
        Rule::Nonstick => {
            let said = format!("{} {}", reading.line.clean, reading.line.norm_text).to_lowercase();
            usize::from(has(&said, data::NONSTICK_CLEAN) || !has(&said, data::NONSTICK_KEEP))
        }
        Rule::Learn => usize::from(!words.iter().any(|word| {
            data::FOOD_WORDBANK
                .binary_search(&food_fold::fold(&word.to_lowercase()).as_str())
                .is_ok()
        })),
        Rule::Tea => {
            let said = format!("{} {}", reading.line.clean, reading.line.norm_text).to_lowercase();
            if TEA_FOR_MEAL.is_match(&said) && TEA_MEAL_ASK.is_match(&said) {
                return None;
            }
            let words: Vec<_> = TEA_WORD
                .find_iter(&said)
                .map(|word| word.as_str())
                .collect();
            let cues: &[&[&str]] = &[
                &["iced", "ice"],
                &["pot", "teapot", "teapots"],
                &["cloudy", "cloudiness", "scum", "murky", "film"],
                &["green", "greens", "sencha", "matcha", "jasmine"],
                &["twice", "again", "reuse", "re-use", "second"],
                &[
                    "bitter", "sour", "soured", "stewed", "weak", "strong", "stronger", "watery",
                    "tastes", "taste", "tasting", "stewing",
                ],
            ];
            cues.iter()
                .position(|cues| words.iter().any(|word| cues.contains(word)))
                .map_or(7, |index| index + 1)
        }
    };
    texts.get(index).copied().filter(|text| !text.is_empty())
}

pub(super) fn pancake_batch(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let people = reading.numbers.iter().find_map(|number| {
        ((2.0..=40.0).contains(&number.value)
            && matches!(
                number.unit,
                None | Some(
                    "" | "people"
                        | "person"
                        | "persons"
                        | "adults"
                        | "adult"
                        | "kids"
                        | "guests"
                        | "portions"
                        | "servings"
                )
            ))
        .then_some(number.value.round_ties_even() as u16)
    });
    let Some(people) = people else {
        return Some(Cow::Borrowed(concat!(
            "For four people allow 200g plain flour, 3 eggs and 600ml milk, which makes about ",
            "16 thin pancakes; a family with small children is happy on 150g. Cook them a ",
            "minute a side in a hot pan until the bubbles pop and the edges look dry."
        )));
    };
    let grams = 50 * people;
    let flour = if grams % 1000 == 0 {
        format!("{}kg", grams / 1000)
    } else if grams > 1000 && grams % 500 == 0 {
        format!("{} and a half kg", grams / 1000)
    } else {
        format!("{grams}g")
    };
    let eggs = 3 * people;
    let eggs = match eggs % 4 {
        2 => format!("{} or {} eggs", eggs / 4, eggs / 4 + 1),
        3 => format!("{} eggs", eggs / 4 + 1),
        _ => format!("{} eggs", eggs / 4),
    };
    let millilitres = 150 * people;
    let milk = if millilitres < 1000 {
        format!("{millilitres}ml of milk")
    } else if millilitres % 1000 == 0 {
        format!("{} litres of milk", millilitres / 1000)
    } else {
        format!("{:.1} litres of milk", f64::from(millilitres) / 1000.0)
            .replace(".0 litres", " litres")
    };
    let pancakes = 4 * people;
    Some(format!("For {people} people allow {flour} plain flour, {eggs} and {milk}, which makes about {pancakes} thin pancakes. Cook them a minute a side in a hot non-stick pan with a little butter, until the bubbles pop and the edges look dry; for thick American ones add a teaspoon of baking powder per 200g of flour.").into())
}

pub(super) fn kitchen_without(context: &Context<'_>) -> Option<Cow<'static, str>> {
    static DECIDING: LazyLock<Regex> = LazyLock::new(|| {
        let terms = data::KITCHEN_DECIDING
            .iter()
            .map(|word| regex::escape(word))
            .collect::<Vec<_>>();
        Regex::new(&format!(r"\b(?:{})\b", terms.join("|"))).expect("source kitchen decision terms")
    });
    let reading = context.reading;
    let said = format!("{} {}", reading.line.clean, reading.line.norm_text).to_lowercase();
    if DECIDING.is_match(&said)
        || context.earlier.last().is_some_and(|turn| {
            has(
                &turn.bot.to_lowercase(),
                &[
                    "you cannot bake without an oven",
                    "you can still eat well",
                    "you can still eat properly",
                    "a kettle still makes",
                    "with an oven and no hob",
                ],
            )
        })
    {
        return None;
    }
    crate::bot::knowledge::kitchen::without(&reading.line).map(Cow::Owned)
}

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(MINCE_PIE, data::MINCE_PIE_PATTERN);
pattern!(SOFT_FRUIT, data::SOFT_FRUIT_PATTERN);
pattern!(FRUIT_WANTS_HEAT, data::SOFT_FRUIT_HEAT_PATTERN);
pattern!(SOAK, r"\bsoak");
pattern!(SEASON_SHOW, data::SEASON_SHOW_PATTERN);
pattern!(COOKED_RICE, data::COOKED_RICE_PATTERN);
pattern!(TEA_MEAL_ASK, data::TEA_MEAL_ASK);
pattern!(TEA_FOR_MEAL, data::TEA_FOR_MEAL);
pattern!(TEA_WORD, r"[a-z']+");
pattern!(
    FRUIT_COOKED_IN_WATER,
    r"\b(?:stew|stews|stewed|stewing|boil|boils|boiled|boiling|simmer|simmered|simmering|poach|poached|poaching|puree|pureed)\b"
);
pattern!(MICROWAVE, r"\bmicrowave\b");

pub(super) fn soft_fruit_cooking(text: &str) -> bool {
    FRUIT_WANTS_HEAT.is_match(text) || !SOFT_FRUIT.is_match(text)
}

pub(super) fn fruit_cooking(text: &str) -> Option<&'static str> {
    static FRUITS: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
        [
            ("quince", "quince"),
            ("nectarine", "peach"),
            ("peach", "peach"),
            ("plum", "plum"),
            ("pear", "pear"),
        ]
        .into_iter()
        .map(|(fruit, label)| {
            (
                Regex::new(&format!(r"\b{fruit}(?:es|s)?\b")).expect("valid fruit name"),
                label,
            )
        })
        .collect()
    });
    let fruit = FRUITS
        .iter()
        .find_map(|(name, label)| name.is_match(text).then_some(*label))
        .unwrap_or("apple");
    let (recipes, fallback) = if FRUIT_COOKED_IN_WATER.is_match(text) {
        (data::STEWED_FRUIT, None)
    } else if MICROWAVE.is_match(text) {
        (data::MICROWAVED_FRUIT, None)
    } else {
        (data::BAKED_FRUIT, Some(data::BAKED_APPLES))
    };
    recipes
        .iter()
        .find_map(|(label, reply)| (*label == fruit).then_some(*reply))
        .or(fallback)
}

pub(super) fn is_out_of_the_fridge(text: &str) -> bool {
    !SOAK.is_match(text) || has(text, data::OUT_OF_THE_FRIDGE)
}

pub(super) fn not_a_tv_season(reading: &Reading<'_>) -> bool {
    !SEASON_SHOW.is_match(&format!(
        " {} {} {} ",
        reading.line.clean,
        reading.line.norm_text,
        reading.line.norm.join(" ")
    ))
}

pub(super) fn rice_cooked(text: &str) -> bool {
    COOKED_RICE.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooking_selects_the_requested_mode_and_does_not_treat_soaking_as_storage() {
        assert!(!is_out_of_the_fridge(
            "do i need to soak sticky rice overnight"
        ));
        assert!(is_out_of_the_fridge(
            "i soaked rice and left it out overnight"
        ));
        assert!(!soft_fruit_cooking("my peaches are going soft"));
        assert!(soft_fruit_cooking(
            "my peaches are going soft, can i bake them"
        ));
        let fruit = fruit_cooking("how long do i microwave quince").expect("quince branch");
        assert!(fruit.contains("will not soften"));
        let read = Reading::new("how long does brown rice take?");
        assert_eq!(
            reply(&read, Rule::Rice, &["brown", "time", "ratio"]),
            Some("brown")
        );
    }
}
