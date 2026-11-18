//! Makes and models actually named for a person's possessions.

use crate::{
    bot::{Memory, Reading, compose},
    text::is_space,
};
use regex::Regex;
use std::sync::LazyLock;

#[path = "possessions_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    OWNED_VALUE,
    OPENS_AN_ASK,
    TELLS_ABOUT,
    I_DRIVE,
    CAR_ASKED,
    CAR_NAMED,
    CAR_IN_THE_PAST,
    OWNED_ASK,
    DRIVE_ASK
);
static PART: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Za-z0-9][A-Za-z0-9'+-]*").expect("model and make part"));

#[derive(Default)]
pub(crate) struct Facts {
    car: Option<String>,
    other: Vec<(&'static str, String)>,
}

impl Facts {
    pub(super) fn observe(&mut self, raw: &str, plain: &str) {
        if let Some(car) = car_in(raw).or_else(|| car_in(plain)) {
            self.car = Some(car);
        }
        let mut found = models_in(raw);
        if raw != plain {
            for (thing, model) in models_in(plain) {
                if !found.iter().any(|&(held, _)| thing == held) {
                    found.push((thing, model));
                }
            }
        }
        for (thing, model) in found {
            if let Some((_, held)) = self.other.iter_mut().find(|(name, _)| *name == thing) {
                *held = model;
            } else {
                self.other.push((thing, model));
            }
        }
    }

    pub(super) fn has_any(&self) -> bool {
        self.car.is_some() || !self.other.is_empty()
    }

    fn model(&self, thing: &str) -> Option<&str> {
        self.other
            .iter()
            .find(|&&(name, _)| name == thing)
            .map(|item| item.1.as_str())
    }
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let reply = answer(reading, memory)?;
    Some(super::again::instead(&reply, reading.raw, memory).unwrap_or(reply))
}

fn answer(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = flat(&reading.raw.to_lowercase());
    let facts = &memory.possessions;
    for found in OWNED_ASK.captures_iter(&said) {
        let thing = found
            .iter()
            .skip(1)
            .flatten()
            .next()?
            .as_str()
            .replace('-', " ");
        let name = facts
            .model(&thing)
            .or_else(|| facts.model(thing.trim_end_matches('s')));
        if thing == "car"
            && let Some(car) = name.or(facts.car.as_deref())
        {
            return Some(car_line(car));
        }
        if let Some(name) = name {
            let article = article_for_model(name);
            let object = format!("{article} {name}.");
            let (first, second) = match thing.as_str() {
                "bike" | "motorbike" => (
                    format!("You ride {object}"),
                    format!("Your {thing} is {object}"),
                ),
                "phone" => (
                    format!("Your phone is {object}"),
                    format!("You've got {object}"),
                ),
                _ => (
                    format!("Your {thing} is {object}"),
                    format!("You told me your {thing} is {object}"),
                ),
            };
            return Some(compose::pick(&[&first, &second], memory, "").to_owned());
        }
        if data::OWNED.contains(&thing.as_str()) {
            let first = format!("You haven't told me the make of your {thing}");
            let second = format!("You've not said what your {thing} is");
            return Some(format!(
                "{}.",
                compose::pick(&[&first, &second], memory, "")
            ));
        }
    }
    if DRIVE_ASK.is_match(&said) && reading.kind == crate::bot::understanding::kinds::Kind::Question
    {
        if let Some(car) = &facts.car {
            return Some(car_line(car));
        }
        return Some(
            compose::pick(
                &[
                    "You haven't told me what you drive.",
                    "I don't know your car — you haven't said.",
                ],
                memory,
                "",
            )
            .to_owned(),
        );
    }
    None
}

fn words(said: &str) -> impl Iterator<Item = &str> {
    said.split(is_space).filter(|word| !word.is_empty())
}

fn flat(said: &str) -> String {
    words(said).collect::<Vec<_>>().join(" ")
}

fn finish(said: &str) -> String {
    said.trim_matches(is_space)
        .trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"'])
        .to_lowercase()
}

fn car_in(text: &str) -> Option<String> {
    if CAR_ASKED.is_match(text) {
        return None;
    }
    for found in I_DRIVE.captures_iter(text) {
        let mut phrase = finish(&found["what"]);
        for tail in data::TONE_TAIL {
            if let Some(front) = phrase.strip_suffix(tail)
                && front.ends_with(' ')
            {
                phrase.truncate(front.trim_end_matches(is_space).len());
            }
        }
        let car: Vec<_> = words(&phrase)
            .skip_while(|word| word.chars().all(char::is_numeric))
            .collect();
        if let Some(last) = car.last()
            && !car.iter().any(|word| data::NOT_A_CAR.contains(word))
            && !data::MEASURED.contains(last)
            && (found.name("art").is_some() || !last.ends_with('s'))
        {
            return Some(car.join(" "));
        }
    }
    let past = CAR_IN_THE_PAST.is_match(text);
    for item in CAR_NAMED.captures_iter(text) {
        if past && item.name("mine").is_some() {
            continue;
        }
        let phrase = item.name("what").or_else(|| item.name("mine"))?;
        let line = flat(&phrase.as_str().replace('\'', "").to_lowercase());
        if let Some(name) = data::CAR_NAMES.iter().copied().find(|name| {
            line.strip_prefix(name)
                .is_some_and(|rest| rest.is_empty() || rest.starts_with(' '))
        }) {
            return Some(name.to_owned());
        }
    }
    None
}

pub(super) fn has_fact(text: &str) -> bool {
    car_in(text).is_some() || !models_in(text).is_empty()
}

fn models_in(text: &str) -> Vec<(&'static str, String)> {
    let said = text.trim_matches(is_space);
    if OPENS_AN_ASK.is_match(said) || (said.ends_with('?') && !TELLS_ABOUT.is_match(said)) {
        return Vec::new();
    }
    let mut models = Vec::new();
    for item in OWNED_VALUE.captures_iter(said) {
        let mut thing = item
            .name("thing")
            .or_else(|| item.name("thing2"))
            .map_or(String::new(), |name| name.as_str().to_lowercase());
        let value = ["value", "value2", "value3", "value4"]
            .into_iter()
            .find_map(|key| item.name(key))
            .map_or("", |value| value.as_str());
        if thing.is_empty()
            && item.name("value4").is_some()
            && let Some(word) = PART.find(value)
            && let Some((family, _)) = make(&word.as_str().to_lowercase())
            && ["car", "phone", "bike", "boiler"].contains(&family)
        {
            thing = family.to_owned();
        }
        if thing.is_empty() {
            thing = match item
                .name("verb")
                .map(|v| v.as_str().to_ascii_lowercase())
                .as_deref()
            {
                Some("drive") => "car",
                Some("ride") => "bike",
                _ => continue,
            }
            .to_owned();
        }
        let thing = thing.replace('-', " ");
        let Ok(index) = data::OWNED.binary_search(&thing.as_str()) else {
            continue;
        };
        let thing = data::OWNED[index];
        if models.iter().any(|&(held, _)| held == thing) {
            continue;
        }
        if let Some(model) = model(value, thing) {
            models.push((thing, model));
        }
    }
    models
}

fn make(word: &str) -> Option<(&'static str, &'static str)> {
    data::OWNED_MAKES
        .binary_search_by_key(&word, |row| row.0)
        .ok()
        .map(|i| (data::OWNED_MAKES[i].1, data::OWNED_MAKES[i].2))
}

fn known_model(word: &str) -> Option<(&'static str, &'static str)> {
    data::OWNED_MODELS
        .binary_search_by_key(&word, |row| row.0)
        .ok()
        .map(|i| (data::OWNED_MODELS[i].1, data::OWNED_MODELS[i].2))
}

fn first_upper(word: &str) -> String {
    let mut letters = word.chars();
    letters
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(letters)
        .collect()
}

fn model(phrase: &str, thing: &str) -> Option<String> {
    let mut parts: Vec<_> = PART
        .find_iter(phrase)
        .map(|part| part.as_str())
        .take_while(|part| !data::OWNED_STOP.contains(&part.to_lowercase().as_str()))
        .take(4)
        .collect();
    while let Some(span) = (1..=parts.len().min(3)).rev().find(|&span| {
        data::THE_MOMENT.contains(&finish(&parts[parts.len() - span..].join(" ")).as_str())
    }) {
        parts.truncate(parts.len() - span);
    }
    let first = parts.first()?.to_lowercase();
    if data::OWNED_NOT_A_MODEL.contains(&first.as_str()) {
        return None;
    }
    let kind = data::OWNED_KIND
        .iter()
        .find(|&&(name, _)| name == thing)
        .map_or("", |row| row.1);
    if let Some((family, _)) = make(&first) {
        if !kind.is_empty() && !family.is_empty() && family != kind {
            return None;
        }
    } else if kind.is_empty() || known_model(&first).is_none_or(|(family, _)| family != kind) {
        return None;
    }
    Some(
        parts
            .into_iter()
            .map(|word| {
                let low = word.to_lowercase();
                make(&low)
                    .or_else(|| known_model(&low))
                    .map_or_else(|| first_upper(word), |(_, name)| name.to_owned())
            })
            .collect::<Vec<_>>()
            .join(" "),
    )
}

fn car_line(car: &str) -> String {
    let said = flat(car);
    let low = said.to_lowercase();
    let descriptive = words(&low).any(|word| data::CAR_DESCRIBED.contains(&word))
        || data::NO_CAP.contains(&low.as_str());
    let shown = if descriptive {
        said
    } else {
        words(&said)
            .map(|word| {
                let low = word.to_lowercase();
                data::CAR_CAPS
                    .iter()
                    .find(|&&(model, _)| model == low)
                    .map_or_else(
                        || {
                            if word.chars().any(char::is_alphabetic) && word == low {
                                first_upper(word)
                            } else {
                                word.to_owned()
                            }
                        },
                        |row| row.1.to_owned(),
                    )
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    let art = if starts_with_vowel(&shown) { "an" } else { "a" };
    format!("You drive {art} {shown}.")
}

fn starts_with_vowel(word: &str) -> bool {
    word.chars()
        .next()
        .is_some_and(|ch| matches!(ch.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
}

fn article_for_model(model: &str) -> &'static str {
    let first = words(model).next().unwrap_or("");
    let all_caps = first.chars().any(char::is_uppercase) && !first.chars().any(char::is_lowercase);
    if !(first.chars().count() > 1 && all_caps) && starts_with_vowel(first) {
        "an"
    } else {
        "a"
    }
}
