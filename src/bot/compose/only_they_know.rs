//! Questions about another person's private facts, from the pinned composer.

#[path = "only_they_know_data.rs"]
mod data;

use crate::bot::{Memory, Reading};
use fancy_regex::Regex;
use std::sync::LazyLock;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    NOT_A_FACT_ASK,
    A_BODY_OR_ILLNESS,
    A_FACT_ABOUT_THEIR_PERSON,
    A_WH_ABOUT_THEIR_PERSON,
    A_STATE_ABOUT_THEIR_PERSON,
    A_PLACE_ABOUT_THEIR_PERSON,
    A_FIGURE_ABOUT_THEIR_PERSON,
    ABOUT_ME_ASKED,
    THEIR_PERSON_WORD,
    THEIR_PERSON_THROUGH_ANOTHER,
);

fn personal_fact(line: &str, seed: &str, memory: &Memory) -> Option<String> {
    let fact = A_FACT_ABOUT_THEIR_PERSON.captures(line).ok().flatten();
    let state = A_STATE_ABOUT_THEIR_PERSON.captures(line).ok().flatten();
    let place = A_PLACE_ABOUT_THEIR_PERSON.captures(line).ok().flatten();
    let figure = A_FIGURE_ABOUT_THEIR_PERSON.captures(line).ok().flatten();
    let asked = place
        .as_ref()
        .or(fact.as_ref())
        .or(state.as_ref())
        .or(figure.as_ref());
    let wh;
    let asked = if let Some(asked) = asked {
        asked
    } else {
        wh = A_WH_ABOUT_THEIR_PERSON.captures(line).ok().flatten()?;
        &wh
    };
    if place.is_none()
        && (NOT_A_FACT_ASK.is_match(line).unwrap_or(true)
            || A_BODY_OR_ILLNESS.is_match(line).unwrap_or(true))
    {
        return None;
    }
    let said = asked.name("who")?.as_str();
    let named = THEIR_PERSON_WORD.captures(line).ok().flatten();
    let through = THEIR_PERSON_THROUGH_ANOTHER.find(line).ok().flatten();
    let relation;
    let (subject, object) = if let Some(named) = named {
        relation = format!("your {}", named.get(1)?.as_str());
        (relation.as_str(), relation.as_str())
    } else if let Some(through) = through {
        (through.as_str(), through.as_str())
    } else {
        match said {
            "he" | "him" => ("he", "him"),
            "she" | "her" => ("she", "her"),
            "they" | "them" => ("they", "them"),
            _ => return None,
        }
    };
    let pool = if ABOUT_ME_ASKED.is_match(line).unwrap_or(false) {
        data::THEIR_FEELINGS_LINES
    } else if figure.is_some() {
        data::THEIR_PERSON_FIGURE_LINES
    } else if fact.is_none() && state.is_none() {
        data::THEIR_PERSON_WH_LINES
    } else {
        data::THEIR_PERSON_LINES
    };
    let written: Vec<_> = pool
        .iter()
        .map(|line| {
            line.replace("{she}", subject)
                .replace("{who}", object)
                .replace("{subject}", subject)
                .replace("{object}", object)
        })
        .collect();
    let refs: Vec<_> = written.iter().map(String::as_str).collect();
    Some(super::choose_recent(&refs, memory, seed).to_owned())
}

pub(in crate::bot) fn facts(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = &reading.line.clean;
    personal_fact(clean, clean, memory).or_else(|| {
        let repaired = &reading.line.plain;
        (repaired != clean)
            .then(|| personal_fact(repaired, clean, memory))
            .flatten()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_facts_stay_with_the_person_and_medical_questions_are_left_to_their_readers() {
        let memory = Memory::default();
        let work = facts(&Reading::new("Where does she work?"), &memory).unwrap();
        assert!(work.contains("she") || work.contains("her"));
        assert!(
            facts(&Reading::new("Does my mum like golf?"), &memory)
                .unwrap()
                .contains("your mum")
        );
        for question in [
            "Does my dad take insulin?",
            "Where does the council work from?",
        ] {
            assert!(
                facts(&Reading::new(question), &memory).is_none(),
                "{question}"
            );
        }
    }
}
