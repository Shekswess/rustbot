//! Derive a speaker's stated age from a year of birth.

use crate::bot::{Memory, Reading, compose};
use fancy_regex::Regex as FancyRegex;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    BORN,
    r"(?i)\b(?:i\s+was\s+born|i\s+am\s+born|im\s+born|born)\b[^0-9]{0,14}\b(1[89]\d\d|20\d\d)\b"
);
pattern!(
    OTHER_PERSON,
    r"(?i)\b(?:my|his|her|their|our|your|its)\s+(?:\w+\s+){0,2}?(?:was|were|is|are)\s+born\b"
);
pattern!(
    QUESTION,
    r"(?i)\?|\b(?:how|what|when|where|why|who|which|should|can|could|would|do|does|did|is|are|will|any)\b"
);
pattern!(
    AGE_ASK,
    r"(?i)\b(?:what did i say my age|how old am i|what(?:'s| is| was)? my age|my age again|how old did i say|how old i (?:am|was))\b"
);
pattern!(
    AGE_STATEMENT,
    r"(?i)^(?:(?:so|oh|actually|sorry|no|well)[ ,]+)*(?:i'm|im|i am)\s+(\d{1,3})(?:\s+(?:years? old|yrs? old|yo|now))?[.,! ]*$"
);

static EXPLICIT_AGE: LazyLock<FancyRegex> = LazyLock::new(|| {
    FancyRegex::new(r"(?i)\b(?:i\s?m|i\s+am|im|i\s+turned|i\s+will\s+be)\s+(\d{1,3})\b(?!\s*(?:years?\s+(?:ago|from)|kg|pounds?|miles?|minutes?|hours?|days?|weeks?|months?))")
        .expect("stated age alongside a birth year")
});

#[derive(Default)]
pub(crate) struct Facts {
    year: Option<i16>,
    age: Option<i16>,
}

impl Facts {
    pub(crate) fn observe(&mut self, reading: &Reading<'_>) {
        let current = || jiff::Zoned::now().year();
        self.observe_at(reading.raw, current);
    }

    fn observe_at(&mut self, said: &str, current: impl FnOnce() -> i16) {
        if let Some(age) = AGE_STATEMENT
            .captures(said.trim())
            .and_then(|statement| statement[1].parse().ok())
            .filter(|age| (1..=120).contains(age))
        {
            self.age = Some(age);
        }
        let Some(year) = BORN
            .captures(said)
            .and_then(|born| born[1].parse::<i16>().ok())
        else {
            return;
        };
        if OTHER_PERSON.is_match(said) {
            return;
        }
        let this_year = current();
        if !(1900..=this_year).contains(&year) {
            return;
        }
        self.year = Some(year);
        if let Ok(Some(age)) = EXPLICIT_AGE.captures(said) {
            let current_age = age
                .get(0)
                .is_some_and(|matched| !matched.as_str().to_ascii_lowercase().contains("will"));
            if current_age
                && let Some(age) = age
                    .get(1)
                    .and_then(|value| value.as_str().parse().ok())
                    .filter(|age| (1..=120).contains(age))
            {
                self.age = Some(age);
            }
            return;
        }
        let turned = this_year - year;
        if self.age.is_none() && (1..130).contains(&turned) {
            self.age = Some(turned);
        }
    }
}

pub(crate) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = reading.raw;
    let year = memory.birth_facts.year?;
    let born = BORN.find(said)?;
    if OTHER_PERSON.is_match(said)
        || said
            .split(crate::text::is_space)
            .filter(|part| !part.is_empty())
            .take(13)
            .count()
            > 12
        || QUESTION.is_match(&said[..born.start()])
        || QUESTION.is_match(&said[born.end()..])
    {
        return None;
    }
    let turned = memory
        .birth_facts
        .age
        .unwrap_or_else(|| jiff::Zoned::now().year() - year);
    if !(1..130).contains(&turned) {
        return None;
    }
    let pool = [
        format!("Born in {year}, so you turn {turned} this year."),
        format!("{year}. That makes you {turned} this year."),
        format!("Noted - {turned} this year, going by {year}."),
    ];
    Some(compose::pick(&pool.each_ref().map(String::as_str), memory, said).to_owned())
}

pub(super) fn recalled_age(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !AGE_ASK.is_match(&reading.line.clean) {
        return None;
    }
    let Some(age) = memory.birth_facts.age.filter(|age| (1..=120).contains(age)) else {
        return Some(
            compose::pick(
                &[
                    "You haven't told me your age yet.",
                    "I don't know it — you haven't said.",
                ],
                memory,
                "",
            )
            .to_owned(),
        );
    };
    let pool = [
        format!("You told me you're {age}."),
        format!("{age} — you said."),
    ];
    Some(compose::pick(&pool.each_ref().map(String::as_str), memory, "").to_owned())
}

#[cfg(test)]
#[test]
fn a_birth_year_fills_an_empty_age_and_keeps_an_explicit_age() {
    let mut facts = Facts::default();
    facts.observe_at("My son was born in 2015", || 2026);
    assert_eq!((facts.year, facts.age), (None, None));
    facts.observe_at("I was born in 1990", || 2026);
    assert_eq!((facts.year, facts.age), (Some(1990), Some(36)));
    facts.observe_at("I was born in 1990 and im 35", || 2026);
    assert_eq!((facts.year, facts.age), (Some(1990), Some(35)));
    let mut future = Facts::default();
    future.observe_at("I was born in 1990 and I will be 37", || 2026);
    assert_eq!((future.year, future.age), (Some(1990), None));
}
