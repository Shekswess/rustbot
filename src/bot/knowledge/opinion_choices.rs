//! Buying judgements and the practical factors behind life choices.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::choosing::names;

#[path = "opinion_choices_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    A_NEW_ONE,
    WHAT_TO_BUY,
    TWO_OF_THEM,
    MADE_UP,
    ANOTHER_CATS_QUESTION
);

static UNDECIDED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:cant|can't|cannot|whether|which|torn|unsure)\b").expect("a choice still open")
});
static BETWEEN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bbetween\b").expect("between two options"));

const APPLIANCES: &[&[&str]] = &[
    &["robot vacuum", "robot vacuums", "robot hoover"],
    &["dishwasher", "dishwashers"],
    &["tumble dryer", "tumble dryers", "dryer"],
    &["washing machine", "washing machines"],
    &["slow cooker", "slow cookers"],
    &["bread maker", "bread makers"],
    &["blender", "blenders"],
    &["food processor", "food processors"],
    &["pressure cooker", "pressure cookers"],
    &["vacuum cleaner", "vacuum"],
    &["hoover", "hoovers"],
    &["freezer", "freezers"],
];

const DOG: &[&str] = &["dog", "dogs", "puppy", "puppies", "rescue dog"];
const CAT: &str = "Think about whether a cat fits your routine for the long term. Budget for food, litter, vet care and holiday cover, and allow time for play and company each day. If you want an adult cat, ask a rescue which cats fit your household and the time you spend at home.";
const DOG_TRADE: &str = "A dog needs daily exercise, companionship and training. Work out who will do those every day, and budget for food, equipment, vet care, insurance and any walker or daycare. Talk to a rescue about a dog that suits your household.";
const SMALL_PET: &str = "Check the needs of the exact species and budget for suitable housing, food and vet care before choosing. Hamsters need space for burrowing and usually live alone. Other small pets, such as gerbils, have different social and housing needs. An adult should be responsible for daily care.";
const CAR: &str = "List the journeys you actually make, then compare vehicles on total cost, including insurance, tax, servicing, charging or fuel and repairs. If you're considering an electric car, check where you'll charge and how longer trips would work.";

fn includes(text: &str, choices: &[&str]) -> bool {
    choices.iter().any(|choice| text.contains(choice))
}

fn appliance(text: &str) -> Option<Cow<'static, str>> {
    if includes(text, data::A_PART) && includes(text, data::A_JOB) && !includes(text, data::BUYING)
    {
        return None;
    }
    if text.contains("heat pump")
        && (text.contains("dryer") || text.contains("drier"))
        && (includes(text, data::BUYING)
            || includes(
                text,
                &["any good", "good", "worth", "should i get", "get one"],
            ))
    {
        return None;
    }
    let cleaned = [
        "hair dryer",
        "hair drier",
        "hairdryer",
        "nail dryer",
        "vacuum flask",
        "flask",
        "thermos",
    ]
    .iter()
    .fold(text.to_owned(), |text, word| text.replace(word, " "));
    let subject = APPLIANCES
        .iter()
        .find(|options| includes(&cleaned, options))
        .map(|options| options[0]);
    let lead = subject.map_or_else(
        || "It may be worth buying".to_owned(),
        |name| format!("A {name} may be worth buying"),
    );
    Some(Cow::Owned(format!(
        "{lead} if you use it regularly and it saves time or effort on a job you do often. Think about how much you would use it, where it would go, and what it costs to run. If you're unsure, try borrowing one before you buy."
    )))
}

fn cats_or_dogs(text: &str) -> &'static str {
    if includes(text, data::OUT_ALL_DAY) {
        "If you're out for long stretches, a dog will usually need someone to provide company and exercise while you're away. An adult cat may suit that schedule better if you can still meet its needs. Ask a rescue about the individual animal."
    } else if includes(text, data::TRAIN_WORDS) {
        "If training together is what you want, a dog may be a better fit. Dogs and cats can both learn with rewards. How much an individual animal enjoys training varies."
    } else if includes(text, data::CAT_WINS) {
        CAT
    } else if includes(text, data::DOG_WINS) {
        DOG_TRADE
    } else {
        "A dog may suit you if you want daily walks and training together. A cat may suit you if you prefer an animal that often does more on its own. Look at your schedule and the needs of the individual animal."
    }
}

fn decision(text: &str) -> Option<&'static str> {
    let cat = names(text, data::CAT_WORDS);
    if A_NEW_ONE.is_match(text)
        || (WHAT_TO_BUY.is_match(text) && cat)
        || (TWO_OF_THEM.is_match(text) && !BETWEEN.is_match(text))
        || (MADE_UP.is_match(text) && !UNDECIDED.is_match(text))
    {
        return None;
    }
    if cat && names(text, DOG) {
        return Some(cats_or_dogs(text));
    }
    if data::TWO_WAYS.iter().all(|side| names(text, side)) {
        return None;
    }
    if cat {
        return (!names(text, data::CAT_EQUIPMENT)).then_some(CAT);
    }
    if names(text, data::QUALIFIED) {
        return None;
    }
    data::DECISIONS
        .iter()
        .find_map(|(options, reply)| names(text, options).then_some(*reply))
}

pub(crate) fn no_specific_cat_question(reading: &Reading<'_>) -> bool {
    !ANOTHER_CATS_QUESTION.is_match(reading.raw)
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "worth-an-appliance" => appliance(&reading.raw.to_lowercase()),
        "decide-a-choice" | "cat-decide" => {
            decision(&reading.line.words.join(" ")).map(Cow::Borrowed)
        }
        _ => None,
    }
}
