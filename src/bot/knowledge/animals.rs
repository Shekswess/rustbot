//! Animal facts, species names, diet cards and records.

use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

use crate::bot::Reading;

use super::{animal_data as data, guards::Context};

pub(super) struct Row {
    pub(super) names: &'static [&'static str],
    pub(super) replies: [Option<&'static str>; 4],
}

#[derive(Clone, Copy)]
pub(super) enum Fact {
    Life,
    Gestation,
    Size,
    Speed,
}

#[derive(Clone, Copy)]
pub(super) enum Word {
    Young,
    Group,
    Female,
    Male,
}

struct Species {
    names: Vec<(&'static Row, &'static str)>,
    patterns: RegexSet,
}

impl Species {
    fn new(rows: &'static [Row]) -> Self {
        let names: Vec<_> = rows
            .iter()
            .flat_map(|row| row.names.iter().map(move |&name| (row, name)))
            .collect();
        let patterns = RegexSet::new(
            names
                .iter()
                .map(|(_, name)| format!(r"\b{}\b", regex::escape(name))),
        )
        .expect("valid animal name literals");
        Self { names, patterns }
    }

    fn named(&self, text: &str) -> Option<&'static Row> {
        self.patterns
            .matches(text)
            .into_iter()
            .max_by_key(|&index| {
                (
                    self.names[index].1.chars().count(),
                    std::cmp::Reverse(index),
                )
            })
            .map(|index| self.names[index].0)
    }
}

static FACTS: LazyLock<Species> = LazyLock::new(|| Species::new(data::ANIMALS));
static WORDS: LazyLock<Species> = LazyLock::new(|| Species::new(data::WORDS));

fn line<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.plain.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.plain
    }
}

impl Fact {
    pub(super) fn answer(self, context: &Context<'_>) -> Option<&'static str> {
        FACTS.named(line(context.reading))?.replies[self as usize]
    }
}

impl Word {
    pub(super) fn answer(self, context: &Context<'_>) -> Option<Cow<'static, str>> {
        let row = WORDS.named(line(context.reading))?;
        let word = row.replies[self as usize]?;
        let article = if word.starts_with(['a', 'e', 'i', 'o', 'u']) {
            "an"
        } else {
            "a"
        };
        let name = row.names[0];
        let result = match self {
            Self::Young => format!("A young {name} is {article} {word}."),
            Self::Group => {
                let many = row
                    .names
                    .get(1)
                    .map_or_else(|| format!("{name}s").into(), |name| Cow::Borrowed(*name));
                format!("A group of {many} is {article} {word}.")
            }
            Self::Female => format!("A female {name} is {article} {word}."),
            Self::Male => format!("A male {name} is {article} {word}."),
        };
        Some(Cow::Owned(result))
    }
}

pub(super) fn eats(context: &Context<'_>) -> Option<&'static str> {
    let typed = line(context.reading);
    let named = FACTS.named(typed)?;
    let text = format!(" {typed} ");
    for name in named.names {
        if data::DIET_AVOID
            .iter()
            .any(|(animal, words)| animal == name && words.iter().any(|word| text.contains(word)))
        {
            return None;
        }
        if let Some((_, reply)) = data::DIET.iter().find(|(animal, _)| animal == name) {
            return Some(*reply);
        }
    }
    None
}

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(BIRD, r"\bbird|birds\b");
pattern!(SEA, r"\b(fish|sea|ocean|water)\b");
pattern!(LAND, r"\b(land|on land)\b");
pattern!(
    LONGEVITY,
    r"\b(longest lived|longest living|longest-lived|oldest|longest|lives the longest|live the longest)\b"
);
pattern!(SPEED, r"\b(fastest|quickest|speed)\b");
pattern!(TALL, r"\btall\b|\btallest\b");
pattern!(STRENGTH, r"\b(strongest|weakest)\b");
pattern!(DANGER, r"\b(dangerous|deadliest|kill)\b");

pub(super) fn record(context: &Context<'_>) -> Option<&'static str> {
    let text = line(context.reading);
    let place = if BIRD.is_match(text) {
        "bird"
    } else if SEA.is_match(text) {
        "sea"
    } else if LAND.is_match(text) {
        "land"
    } else {
        "any"
    };
    let category = if LONGEVITY.is_match(text) {
        "life"
    } else if SPEED.is_match(text) {
        "speed"
    } else if TALL.is_match(text) {
        "tall"
    } else if STRENGTH.is_match(text) {
        if text.contains("strong") {
            "strong"
        } else {
            "weak"
        }
    } else if DANGER.is_match(text) {
        "danger"
    } else {
        "big"
    };
    let answer = |place| {
        data::RECORDS.iter().find_map(|&(what, where_, reply)| {
            (what == category && place == where_).then_some(reply)
        })
    };
    answer(place).or_else(|| answer("any"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animal_readers_use_the_longest_name_and_the_requested_fact() {
        let read = Reading::new("how large is a blue whale");
        let fact = Fact::Size
            .answer(&Context::new(&read, &[]))
            .expect("blue whale size");
        assert!(fact.to_lowercase().contains("blue whale"));
        let read = Reading::new("what is a group of ravens called");
        let word = Word::Group.answer(&Context::new(&read, &[]));
        assert_eq!(word.as_deref(), Some("A group of ravens is an unkindness."));
        let read = Reading::new("which is the fastest bird");
        assert!(
            record(&Context::new(&read, &[]))
                .is_some_and(|reply| reply.starts_with("The peregrine"))
        );
    }
}
