//! Guards shared by family advice about an incident between people.

use crate::bot::Reading;
use fancy_regex::Regex as Lookaround;
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

#[path = "family_repair_data.rs"]
mod data;

static AMENDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::AMENDS).expect("wording asking to make amends"));
static PET_OR_PROFESSIONAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::PET_OR_PROFESSIONAL).expect("pet or professional exception"));
static NEVER_DEED: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(data::NEVER_DEED).expect("exceptions to a family deed"));
static NOT_A_DEED: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(data::NOT_A_DEED).expect("shapes that are not family deeds"));
static PHYSICAL_HARM: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::PHYSICAL_HARM).expect("family harm exception"));
static LAPSES: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(data::LAPSES).expect("family contact lapses"));
static DEEDS: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(data::DEEDS).expect("family deeds"));
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static TOLD_OFF: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::TOLD_OFF).unwrap());
static SMACKED: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::SMACKED).unwrap());

fn clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    [&reading.line.clean, &reading.line.text]
        .into_iter()
        .find(|part| !part.is_empty())
        .map_or("", String::as_str)
}

fn owns_deed(reading: &Reading<'_>) -> bool {
    let said = normalized(clean(reading));
    !(said.is_empty()
        || NEVER_DEED.is_match(&said)
        || PHYSICAL_HARM.is_match(&said).unwrap_or(false))
        && (LAPSES.is_match(&said) || !NOT_A_DEED.is_match(&said) && DEEDS.is_match(&said))
}

pub(super) fn not_their_deed(reading: &Reading<'_>) -> bool {
    !owns_deed(reading)
}

fn normalized(said: &str) -> String {
    let said = said.to_lowercase().replace('’', "'");
    said.split(crate::text::is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn asks_amends(reading: &Reading<'_>) -> bool {
    let said = clean(reading);
    let said = if said.is_empty() {
        normalized(&reading.line.words.join(" "))
    } else {
        normalized(said)
    };
    AMENDS.is_match(&said)
        && !PET_OR_PROFESSIONAL.is_match(&said)
        && !PHYSICAL_HARM.is_match(&said).unwrap_or(false)
}

fn named_person_let_them_down(words: &[&str]) -> bool {
    for (index, word) in words.iter().enumerate() {
        if !data::LET_DOWN.contains(word) {
            continue;
        }
        let after = &words[index + 1..words.len().min(index + 4)];
        if !matches!(after.first(), Some(&("me" | "us"))) || !after.contains(&"down") {
            continue;
        }
        if words[..index]
            .iter()
            .rev()
            .find(|word| !data::LET_DOWN_SKIPPED.contains(word))
            .is_some_and(|word| !data::LET_DOWN_PEOPLE.contains(word))
        {
            return false;
        }
    }
    true
}

pub(super) fn friend_cancelled(reading: &Reading<'_>) -> bool {
    if owns_deed(reading) {
        return false;
    }
    let words: Vec<_> = reading.line.norm.iter().map(String::as_str).collect();
    if words.is_empty() {
        let text = clean(reading).to_lowercase();
        let words: Vec<_> = WORD.find_iter(&text).map(|word| word.as_str()).collect();
        named_person_let_them_down(&words)
    } else {
        named_person_let_them_down(&words)
    }
}

pub(super) fn told_them_off(reading: &Reading<'_>) -> bool {
    TOLD_OFF.is_match(clean(reading))
}

pub(super) fn after_telling_off(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = if SMACKED.is_match(clean(reading)) {
        data::AFTER_HITTING
    } else {
        data::AFTER_SHOUTING
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_person_deeds_and_relational_lapses_own_their_turn() {
        assert!(!not_their_deed(&Reading::new("i shouted at my mum")));
        assert!(!not_their_deed(&Reading::new(
            "i have not called my mum in months"
        )));
        assert!(not_their_deed(&Reading::new("i shouted at my dog")));
        assert!(not_their_deed(&Reading::new(
            "i haven't spoken to my dad since he died"
        )));
        assert!(!friend_cancelled(&Reading::new(
            "the bus let me down again"
        )));
        assert!(friend_cancelled(&Reading::new(
            "my friend let me down again"
        )));
    }

    #[test]
    fn telling_off_preserves_the_original_incident_specific_repair() {
        let read = Reading::new("i told my son off and i feel terrible");
        assert!(told_them_off(&read));
        assert_eq!(
            after_telling_off(&read).as_deref(),
            Some(data::AFTER_SHOUTING)
        );
        let read = Reading::new("i smacked my child and i want to put this right");
        assert!(told_them_off(&read));
        assert_eq!(
            after_telling_off(&read).as_deref(),
            Some(data::AFTER_HITTING)
        );
        assert!(!told_them_off(&Reading::new(
            "my son is too hard on himself"
        )));
    }
}
