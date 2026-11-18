//! Dog training and everyday animal behaviour from bot/knowledge/pets.py.
#[path = "pets_training_data.rs"]
mod data;
#[cfg(test)]
#[path = "pets_training_tests.rs"]
mod tests;

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    Recall,
    Chewing,
    Barking,
    Wary,
    Noise,
    Night,
    Bored,
    Name,
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(NAME_ASK, CALLED_NAME, AGE_PATTERN, NIGHT_AGE);
static CAT_SEASON: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PET_SEASON_WORD).expect("cat in season"));
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static CAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(cat|cats|kitten|kittens)\b").unwrap());
static RABBIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(rabbit|rabbits|bunny|bunnies)\b").unwrap());
static BORED_AT_WORK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(work|working|left|alone|out)\b").unwrap());

fn recall(line: &str) -> Option<&'static str> {
    if NAME_ASK.is_match(line)
        || CALLED_NAME.captures(line).is_some_and(|m| {
            m.get(1).is_none_or(|word| {
                data::CALLED_NOT_A_NAME
                    .binary_search(&word.as_str())
                    .is_err()
            })
        })
    {
        return None;
    }
    Some(data::DOG_RECALL_REPLY)
}

pub(super) fn not_a_cat_name(reading: &Reading<'_>) -> bool {
    const AT_HOME: &[&str] = &[
        "about",
        "saying",
        "yesterday",
        "today",
        "this",
        "last",
        "just",
        "shortly",
        "earlier",
        "round",
        "since",
        "later",
        "once",
    ];
    let clean = reading.line.clean.to_lowercase();
    if CAT_SEASON.is_match(&clean) {
        return true;
    }
    if CALLED_NAME.captures_iter(&clean).any(|found| {
        found.get(1).is_none_or(|word| {
            !data::CALLED_NOT_A_NAME.contains(&word.as_str()) && !AT_HOME.contains(&word.as_str())
        })
    }) {
        return false;
    }
    let said = if clean.is_empty() {
        Cow::Owned(reading.line.text.to_lowercase())
    } else {
        Cow::Borrowed(clean.as_str())
    };
    CAT_SEASON.is_match(&said)
        || (!NAME_ASK.is_match(&said)
            && CALLED_NAME.captures(&said).is_none_or(|found| {
                found
                    .get(1)
                    .is_some_and(|word| data::CALLED_NOT_A_NAME.contains(&word.as_str()))
            }))
}

fn age_in_years(reading: &Reading<'_>) -> Option<f32> {
    let early = if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    };
    let line = early.to_lowercase();
    let age = AGE_PATTERN.captures(&line)?;
    let given = &age[1];
    let number = data::AGE_WORDS
        .iter()
        .find_map(|(word, age)| (*word == given).then_some(*age))
        .or_else(|| given.parse::<f32>().ok())?;
    let unit = &age[2];
    if unit.starts_with("month") || unit.starts_with("mth") {
        Some(number / 12.0)
    } else if unit.starts_with("week") || unit.starts_with("wk") {
        Some(number / 52.0)
    } else {
        Some(number)
    }
}

pub(super) fn dog_not_young(reading: &Reading<'_>) -> bool {
    age_in_years(reading).is_none_or(|age| age >= 7.0)
        || WORD.find_iter(&reading.line.clean).any(|word| {
            matches!(
                word.as_str(),
                "stiff"
                    | "stiffness"
                    | "stiffening"
                    | "limp"
                    | "limping"
                    | "limps"
                    | "lame"
                    | "lameness"
                    | "arthritis"
                    | "arthritic"
                    | "hips"
                    | "hip"
                    | "dysplasia"
                    | "slowing"
                    | "slow"
                    | "slower"
                    | "sore"
                    | "aching"
                    | "aches"
                    | "painful"
            )
        })
}

pub(super) fn warning_sound_said(reading: &Reading<'_>) -> bool {
    let text = if reading.line.clean.is_empty() {
        reading.raw.to_lowercase()
    } else {
        reading.line.clean.to_lowercase()
    };
    let mut little = false;
    let mut play = false;
    let mut warning = false;
    let mut teeth = false;
    let mut show = false;
    for word in WORD.find_iter(&text).map(|word| word.as_str()) {
        little |= matches!(
            word,
            "puppy" | "puppy's" | "puppies" | "pup" | "pups" | "doggy"
        );
        play |= data::HANDS_OR_PLAY.contains(&word);
        warning |= matches!(
            word,
            "growl"
                | "growls"
                | "growled"
                | "growling"
                | "snap"
                | "snaps"
                | "snapped"
                | "snapping"
                | "snarl"
                | "snarls"
                | "snarled"
                | "snarling"
                | "bares"
                | "baring"
                | "lunge"
                | "lunges"
                | "lunged"
        );
        teeth |= word == "teeth";
        show |= matches!(
            word,
            "bares" | "bared" | "baring" | "shows" | "showed" | "showing"
        );
    }
    !(little && play) && (warning || teeth && show)
}

fn chewing(reading: &Reading<'_>) -> &'static str {
    if age_in_years(reading).map_or_else(
        || {
            let line = format!(" {} ", reading.line.words.join(" ").to_lowercase());
            [
                " puppy ",
                " puppies ",
                " pup ",
                " pups ",
                " teething ",
                " teeth ",
                " months old ",
                " month old ",
                " weeks old ",
                " week old ",
                " just got him ",
                " just got her ",
                " just got it ",
                " new puppy ",
                " young dog ",
            ]
            .iter()
            .any(|word| line.contains(word))
        },
        |years| years < 1.0,
    ) {
        data::YOUNG_CHEW
    } else {
        data::ADULT_CHEW
    }
}

fn reason(reading: &Reading<'_>) -> bool {
    reading.wants == Some("reason")
}

fn wary(reading: &Reading<'_>) -> Cow<'static, str> {
    const TRUST: &str =
        "Being boring and calm is what builds trust, and it usually takes weeks rather than days.";
    let line = reading.line.clean.to_lowercase();
    let words: std::collections::HashSet<_> = WORD.find_iter(&line).map(|w| w.as_str()).collect();
    let found = |k: &str| words.contains(k);
    let single = data::WARY_ONE.iter().find(|(word, _)| found(word));
    let plural = data::WARY_MANY.iter().find(|(word, _)| found(word));
    let plan: Cow<'static, str> = match (single, plural) {
        (Some((_, who)), _) => format!(
            "Let him come to {who} on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while {who} sits nearby and ignores the dog. {TRUST}"
        ).into(),
        (None, Some((_, who))) => format!(
            "Let him come to {who} on his own terms: no eye contact, no reaching over him, and treats tossed on the floor while they sit nearby and ignore the dog. {TRUST}"
        ).into(),
        (None, None) => Cow::Borrowed(data::WARY_DEFAULT),
    };
    if reason(reading) {
        return Cow::Owned(format!(
            "Wary dogs are usually more frightened than bold, so let{}",
            plan.strip_prefix("Let").unwrap_or(&plan)
        ));
    }
    plan
}

fn night(reading: &Reading<'_>, earlier: &[Exchange]) -> &'static str {
    let has = |given: &[&str]| {
        reading
            .line
            .words
            .iter()
            .any(|word| given.contains(&word.as_str()))
    };
    let line = &reading.line.clean;
    let older = has(&[
        "elderly", "senior", "old", "older", "oldest", "aging", "ageing",
    ]) || NIGHT_AGE.is_match(line);
    let previous_puppy = !older
        && earlier.iter().rev().take(2).any(|turn| {
            let said = format!(" {} ", turn.user.to_lowercase());
            said.contains("puppy") || said.contains(" pup ")
        });
    if has(&["puppy", "puppies"])
        || previous_puppy
        || line.contains("weeks old")
        || line.contains("months old")
    {
        data::PUPPY_NIGHT
    } else if older {
        data::SENIOR_NIGHT
    } else {
        data::DOG_NIGHT
    }
}

#[derive(Clone, Copy)]
enum Animal {
    Dog,
    Cat,
    Rabbit,
}

impl Animal {
    fn read(reading: &Reading<'_>) -> Self {
        let text = if let Some(animal) = reading.animal_in_hand.as_deref() {
            if animal.starts_with("cat") || animal.starts_with("kitten") {
                return Self::Cat;
            }
            if animal.starts_with("rabbit") || animal.starts_with("bunny") {
                return Self::Rabbit;
            }
            if animal.starts_with("dog") || animal.starts_with("pupp") {
                return Self::Dog;
            }
            animal
        } else {
            &reading.line.clean
        };
        if CAT.is_match(text) {
            Self::Cat
        } else if RABBIT.is_match(text) {
            Self::Rabbit
        } else {
            Self::Dog
        }
    }
}

fn other_pet(reading: &Reading<'_>) -> bool {
    let text = format!(" {} ", reading.line.clean.to_lowercase());
    data::OTHER_PETS
        .iter()
        .any(|pet| text.contains(&format!(" {pet} ")))
}

pub(super) fn reply(
    kind: Reply,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let answer = match kind {
        Reply::Recall => recall(&reading.line.clean)?,
        Reply::Chewing => chewing(reading),
        Reply::Barking if reason(reading) => data::BARKING_CAUSE,
        Reply::Barking => data::BARKING,
        Reply::Wary => return Some(wary(reading)),
        Reply::Noise => match Animal::read(reading) {
            Animal::Dog => data::DOG_NOISE,
            Animal::Cat => data::CAT_NOISE,
            Animal::Rabbit => data::RABBIT_NOISE,
        },
        Reply::Night => night(reading, earlier),
        Reply::Name | Reply::Bored if other_pet(reading) => return None,
        Reply::Name => match Animal::read(reading) {
            Animal::Dog => data::DOG_NAME,
            Animal::Cat => data::CAT_NAME,
            Animal::Rabbit => data::RABBIT_NAME,
        },
        Reply::Bored => match Animal::read(reading) {
            Animal::Cat => data::CAT_BORED,
            Animal::Rabbit => data::RABBIT_BORED,
            Animal::Dog if BORED_AT_WORK.is_match(&reading.line.clean) => data::DOG_BORED_WORK,
            Animal::Dog => data::DOG_BORED,
        },
    };
    Some(Cow::Borrowed(answer))
}
