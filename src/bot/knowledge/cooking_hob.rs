//! Hob, oven, and batch preparation cards from the pinned cooking registry.

use super::{contains, phrase_pattern, prefix_word};
use crate::bot::{Reading, understanding::body::Part};
use fancy_regex::Regex as Lookaround;
use regex::Regex;
use std::sync::LazyLock;

#[path = "cooking_hob_data.rs"]
mod data;

macro_rules! whole_phrases {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| phrase_pattern(data::$name));)+
    };
}

whole_phrases! {
    BURNT_ON_A_PAN, SOMETHING_IS_WRONG_WITH_THE_PAN,
    OIL_POINT, OIL_WATER, OIL_ORDER, OIL_FAT_OR_PAN, OIL_TEMP, OIL_FAT_NAMED,
    OIL_DRAIN, OIL_DISPOSAL, OIL_REUSE, OIL_FOAM,
}

static SCORCH: LazyLock<Regex> = LazyLock::new(|| {
    phrase_pattern(&[
        "burnt",
        "burned",
        "black",
        "blackened",
        "scorched",
        "scorch",
        "burn mark",
        "scorch mark",
        "scorched on",
        "stain",
        "stained",
        "discoloured",
        "discolored",
        "scratch",
        "scratched",
    ])
});

static BODY_PART: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::BODY_PART).unwrap());
static HOT_SAID: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::HOT_SAID).unwrap());
static KEPT_PERIOD: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::A_PERIOD_OF_TIME).unwrap());
static AMENDS: LazyLock<Lookaround> = LazyLock::new(|| Lookaround::new(data::AMENDS).unwrap());
static PET_OR_PROFESSIONAL: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::PET_OR_PROFESSIONAL).unwrap());
static PHYSICAL_HARM: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::PHYSICAL_HARM).unwrap());
static SURFACES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    data::HOT_PAN_SURFACES
        .iter()
        .map(|(pattern, reply)| (Regex::new(pattern).unwrap(), *reply))
        .collect()
});

fn burnt_on_the_pan(text: &str) -> bool {
    BURNT_ON_A_PAN.is_match(text) && SCORCH.is_match(text)
}

fn oil_smoking(text: &str) -> Option<&'static str> {
    if (BODY_PART.is_match(text) && contains(text, data::ONTO_A_PERSON)) || burnt_on_the_pan(text) {
        return None;
    }
    if OIL_POINT.is_match(text) {
        return Some(data::OIL_POINT_REPLY);
    }
    if OIL_WATER.is_match(text) {
        return None;
    }
    if OIL_ORDER.is_match(text) {
        return Some(data::OIL_ORDER_REPLY);
    }
    if !OIL_FAT_OR_PAN.is_match(text) && super::super::dishes::named_dish_has_heat(text) {
        return None;
    }
    if OIL_TEMP.is_match(text) {
        return Some(data::OIL_TEMP_REPLY);
    }
    if OIL_FAT_NAMED.is_match(text) {
        if OIL_DRAIN.is_match(text) {
            return Some(data::OIL_DRAIN_REPLY);
        }
        if OIL_DISPOSAL.is_match(text) {
            return Some(data::OIL_DISPOSAL_REPLY);
        }
    }
    if OIL_REUSE.is_match(text) {
        Some(data::OIL_REUSE_REPLY)
    } else if OIL_FOAM.is_match(text) {
        Some(data::OIL_FOAM_REPLY)
    } else {
        SOMETHING_IS_WRONG_WITH_THE_PAN
            .is_match(text)
            .then_some(data::OIL_SMOKE_REPLY)
    }
}

fn sugar(words: &[String]) -> Option<&'static str> {
    let has = |terms| prefix_word(words, terms);
    let dissolve = [
        "dissolve",
        "dissolves",
        "dissolving",
        "dissolved",
        "melt",
        "melts",
        "melting",
        "melted",
    ];
    if has(&["bitter", "burnt", "burned", "burning", "smoky", "smells"]) {
        (!burnt_on_the_pan(&words.join(" "))).then_some(data::SUGAR_BURNT)
    } else if has(&[
        "hard",
        "hardened",
        "hardens",
        "block",
        "solid",
        "lumps",
        "lumpy",
        "stuck",
        "soften",
        "softens",
        "softening",
    ]) {
        Some(data::SUGAR_HARD)
    } else if has(data::SUGAR_TROUBLE) && !has(&dissolve) {
        Some(data::SUGAR_GRAINY)
    } else if has(&["syrup", "coffee", "cocktail", "cocktails", "tea"])
        || has(&dissolve)
        || !has(data::SUGAR_MAKING)
    {
        Some(data::SUGAR_DISSOLVE)
    } else if has(&["toffee", "toffees", "fudge", "praline", "brittle", "candy"]) {
        Some(data::SUGAR_TOFFEE)
    } else {
        Some(data::SUGAR_CARAMEL)
    }
}

fn popcorn(words: &[String]) -> &'static str {
    if prefix_word(
        words,
        &[
            "microwave",
            "microwaved",
            "microwaving",
            "microwaves",
            "bag",
            "bags",
        ],
    ) {
        if words.iter().any(|word| word == "hob" || word == "pan") {
            data::POPCORN_BOTH
        } else {
            data::POPCORN_MICROWAVE
        }
    } else if prefix_word(words, &["oil", "air", "popper", "diet", "healthy"]) {
        data::POPCORN_DRY
    } else {
        data::POPCORN_PAN
    }
}

pub(in crate::bot::knowledge) fn cooling_asked(reading: &Reading<'_>) -> bool {
    if reading.line.kept.is_some() {
        return false;
    }
    let words = reading.line.words.join(" ");
    !KEPT_PERIOD
        .is_match(if words.is_empty() {
            &reading.line.text
        } else {
            &words
        })
        .unwrap_or(false)
}

pub(in crate::bot::knowledge) fn hot_pan_asked(reading: &Reading<'_>) -> bool {
    HOT_SAID.is_match(&reading.line.text)
}

pub(in crate::bot::knowledge) fn oven_heat_asked(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    let mention = |word: &str| data::TEMPERATURE_WORDS.contains(&word);
    let probe = words
        .windows(2)
        .any(|pair| mention(&pair[0]) && data::THING_AFTER_THE_WORD.contains(&pair[1].as_str()));
    let person = !probe
        && (reading.body == Some(Part::Temperature)
            || (words.iter().any(|word| mention(word))
                && (words.iter().any(|word| {
                    data::OTHER_PEOPLE.contains(&word.as_str())
                        || data::WE_OUR_OURS.contains(&word.as_str())
                }) || words.windows(2).any(|pair| {
                    data::POSSESSIVE_OF_A_PERSON.contains(&pair[0].as_str()) && mention(&pair[1])
                }))));
    !person && super::super::dishes::plan_label(&reading.line.clean).is_none()
}

pub(in crate::bot::knowledge) fn made_too_much_asked(reading: &Reading<'_>) -> bool {
    let said = format!(" {} ", reading.line.words.join(" "));
    let has = |terms: &[&str]| terms.iter().any(|term| said.contains(&format!(" {term} ")));
    !has(data::DRINK_NAMES)
        || has(data::COOKING_WORDS)
        || !has(data::ASK_WORDS)
        || !has(data::DOSE_WORDS)
}

pub(in crate::bot::knowledge) fn kids_tea_asked(reading: &Reading<'_>) -> bool {
    reading.line.words.iter().all(|word| {
        data::KIDS_NAMED_INGREDIENTS
            .binary_search(&word.as_str())
            .is_err()
    })
}

pub(in crate::bot::knowledge) fn party_bake_asked(reading: &Reading<'_>) -> bool {
    let line = reading.line.clean.replace('’', "'");
    !AMENDS.is_match(&line).unwrap_or(false)
        || PET_OR_PROFESSIONAL.is_match(&line).unwrap_or(false)
        || PHYSICAL_HARM.is_match(&line).unwrap_or(false)
}

pub(super) fn registered(id: &str, reading: &Reading<'_>) -> Option<&'static str> {
    let line = &reading.line;
    let words = &line.words;
    match id {
        "oil-smoking" => oil_smoking(if line.clean.is_empty() {
            &line.text
        } else {
            &line.clean
        }),
        "hot-pan-surface" => Some(
            SURFACES
                .iter()
                .find_map(|(pattern, reply)| pattern.is_match(&line.clean).then_some(*reply))
                .unwrap_or(data::HOT_PAN_DEFAULT),
        ),
        "sugar-work" => sugar(words),
        "stuck-jar" => Some(
            if contains(&line.text.to_lowercase(), data::CONTAINER_FOODS) {
                data::JAR_LID_AND_KEEP
            } else {
                data::JAR_LID
            },
        ),
        "popcorn" => Some(popcorn(words)),
        "popcorn-keep" => Some(
            if prefix_word(words, &["chewy", "soggy", "squidgy", "tough", "rubbery"]) {
                data::POPCORN_CHEWY
            } else {
                data::POPCORN_KEEP
            },
        ),
        "fridge-hot-food" => Some(data::FRIDGE_HOT),
        "made-too-much" => Some(data::COOKED_TOO_MUCH),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oil_routes_smoke_disposal_temperature_and_abstentions() {
        for (prompt, expected) in [
            ("My oil is smoking", Some(data::OIL_SMOKE_REPLY)),
            (
                "Where do I put used cooking oil?",
                Some(data::OIL_DISPOSAL_REPLY),
            ),
            (
                "Can I pour oil down the drain?",
                Some(data::OIL_DRAIN_REPLY),
            ),
            (
                "What is the smoke point of sunflower oil?",
                Some(data::OIL_POINT_REPLY),
            ),
            (
                "How hot should the frying oil be?",
                Some(data::OIL_TEMP_REPLY),
            ),
            ("Can I reuse oil after frying?", Some(data::OIL_REUSE_REPLY)),
            ("My oil is foaming", Some(data::OIL_FOAM_REPLY)),
            ("Should I add oil to the pasta water?", None),
            ("Hot oil splashed on my arm", None),
            ("Get burnt milk out of a pan", None),
            ("What temperature for chips?", None),
        ] {
            assert_eq!(
                registered("oil-smoking", &Reading::new(prompt)),
                expected,
                "{prompt}"
            );
        }
    }

    #[test]
    fn preparation_guards_and_replies_keep_named_subject_boundaries() {
        assert!(cooling_asked(&Reading::new(
            "Should I cool the stew before it goes in the fridge?"
        )));
        assert!(!cooling_asked(&Reading::new(
            "The stew has been in the fridge for two days."
        )));
        assert!(hot_pan_asked(&Reading::new(
            "Can I put a hot pan on quartz?"
        )));
        assert!(!hot_pan_asked(&Reading::new(
            "Can I put the cold pan on quartz?"
        )));
        assert!(
            registered(
                "hot-pan-surface",
                &Reading::new("Can I put a hot pan on quartz?")
            )
            .unwrap()
            .starts_with("No - not on quartz")
        );
        assert!(oven_heat_asked(&Reading::new(
            "How do I calibrate my oven?"
        )));
        assert!(!oven_heat_asked(&Reading::new(
            "Should I check my temperature?"
        )));
        assert!(!oven_heat_asked(&Reading::new(
            "What oven temperature for chips?"
        )));
        assert!(!made_too_much_asked(&Reading::new(
            "How much tea is too much?"
        )));
        assert!(made_too_much_asked(&Reading::new("I made too much tea.")));
        assert!(!kids_tea_asked(&Reading::new(
            "My kids won't eat kidney beans."
        )));
        assert!(kids_tea_asked(&Reading::new(
            "What should I make the kids for tea?"
        )));
        assert!(!party_bake_asked(&Reading::new(
            "How do I make up for my mum's birthday and make it up to her?"
        )));
        assert!(party_bake_asked(&Reading::new(
            "What can I bake for my mum's birthday?"
        )));
        assert_eq!(
            registered("sugar-work", &Reading::new("My caramel crystallised")),
            Some(data::SUGAR_GRAINY)
        );
        assert_eq!(
            registered(
                "popcorn",
                &Reading::new("Can I microwave popcorn or use a pan?")
            ),
            Some(data::POPCORN_BOTH)
        );
    }
}
