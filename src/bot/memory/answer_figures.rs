//! A specific recorded measurement when the previous answer gave only one.

use crate::{
    bot::{Memory, Reading},
    text::is_space,
};
use fancy_regex::Regex as Lookaround;
use regex::Regex;
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

#[path = "answer_figures_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    FIGURE_ASK,
    ACTION_DURATION,
    ACTION_AMOUNT,
    LISTED_FIGURE,
    A_RATE_ASK,
    A_RATE_STATED
);
static MONEY: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::A_MONEY_FIGURE).expect("money figure"));
static PERCENT: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::A_PERCENT_FIGURE).expect("share figure"));
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").expect("qualifying words"));
static RATE_PREFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:a|an|per)\s+").expect("figure within a rate"));
static TIME_ARTICLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?i)^\W*(?:a|an|per|each)\s+(?:{})\b",
        time_units()
            .map(regex::escape)
            .collect::<Vec<_>>()
            .join("|")
    ))
    .expect("time as the denominator of a rate")
});

struct FigurePatterns {
    strict: Lookaround,
    loose: Lookaround,
}
static BY_UNIT: LazyLock<Mutex<HashMap<u16, Arc<FigurePatterns>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy)]
enum Units {
    Money,
    Denom(u16),
}

fn time_units() -> impl Iterator<Item = &'static str> {
    families()
        .filter(|(_, (family, _))| data::FIGURE_TIME.contains(family))
        .flat_map(|(_, (_, words))| words.iter().copied())
}

fn families() -> impl Iterator<Item = (u16, &'static (&'static str, &'static [&'static str]))> {
    data::FIGURE_UNITS
        .iter()
        .enumerate()
        .map(|(index, family)| (1u16 << index, family))
}

fn units_in(denominations: u16) -> impl Iterator<Item = &'static str> {
    families()
        .filter(move |(bit, _)| *bit & denominations != 0)
        .flat_map(|(_, (_, words))| words.iter().copied())
}

fn asked_units(text: &str) -> Option<Units> {
    let shape = FIGURE_ASK.captures(text)?;
    let rest = &shape["rest"];
    let mut denominations = 0;
    let mut money = false;
    for word in WORD.find_iter(rest).map(|word| word.as_str()) {
        if data::FIGURE_FILLER.contains(&word) {
            continue;
        }
        let (bit, (family, _)) = families().find(|(_, (_, names))| names.contains(&word))?;
        denominations |= bit;
        money |= *family == "money";
    }
    let time_mask = families()
        .filter(|(_, (family, _))| data::FIGURE_TIME.contains(family))
        .fold(0, |all, (bit, _)| all | bit);
    let ask = &shape["asked"];
    if money
        || (ask == "much"
            && (denominations == 0
                || denominations & !time_mask == 0 && TIME_ARTICLE.is_match(rest)))
    {
        return Some(Units::Money);
    }
    if denominations == 0 && ask == "many" {
        return None;
    }
    Some(Units::Denom(if denominations == 0 {
        time_mask
    } else {
        denominations
    }))
}

fn patterns_for(mask: u16) -> Arc<FigurePatterns> {
    let mut patterns = BY_UNIT.lock().unwrap_or_else(|err| err.into_inner());
    Arc::clone(patterns.entry(mask).or_insert_with(|| {
        let mut names: Vec<_> = units_in(mask).collect();
        names.sort_unstable_by_key(|name| (std::cmp::Reverse(name.len()), *name));
        names.dedup();
        let alternatives = names.into_iter().map(regex::escape).collect::<Vec<_>>().join("|");
        let unit = format!("(?-i:{alternatives})");
        let qualified = data::FIGURE_QUALIFIER;
        let number = data::FIGURE_NUMBER;
        let strict = Lookaround::new(&format!(
            r"(?i)(?<![\d,])\b(?P<qual>{qualified}\s+)?(?P<num>{number})(?:\s*(?:-|to|or)\s*(?P<more>{number}))?\s*(?P<unit>{unit})\b(?P<extra>\s+\d{{1,3}})?(?P<after>[^.;:!?,]*)"
        )).expect("known unit figure");
        let loose = Lookaround::new(&format!(
            r"(?i)(?<![\d,])(?:{number}|\d[\d,.]*)\s*(?:-|to|or)?\s*{unit}\b"
        )).expect("loosely held figure");
        Arc::new(FigurePatterns { strict, loose })
    }))
}

fn flatten(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn clause(said: &str, start: usize, end: usize) -> &str {
    let left = said[..start]
        .rfind(['.', ';', ':'])
        .map_or(0, |point| point + 1);
    let right = said[end..]
        .find(['.', ';', ':'])
        .map_or(said.len(), |point| end + point);
    &said[left..right]
}

fn money_in<'a>(said: &'a str, regex: &Lookaround) -> Vec<&'a str> {
    regex
        .find_iter(said)
        .filter_map(Result::ok)
        .filter(|found| !ACTION_AMOUNT.is_match(clause(said, found.start(), found.end())))
        .map(|found| {
            found
                .as_str()
                .trim_matches(is_space)
                .trim_end_matches([',', ';', ':'])
        })
        .collect()
}

fn amounts(said: &str, mask: u16, patterns: &FigurePatterns) -> Vec<String> {
    let mut out = Vec::new();
    for found in patterns.strict.captures_iter(said).filter_map(Result::ok) {
        let after = found["after"].trim_matches(is_space).to_lowercase();
        let next_word = WORD.find(&after).map_or("", |found| found.as_str());
        let unit = &found["unit"];
        let singular = !unit.ends_with('s') && !["km", "kg", "lb", "lbs"].contains(&unit);
        if singular
            && !next_word.is_empty()
            && !data::FIGURE_AFTER.contains(&next_word)
            && !units_in(mask).any(|word| word == next_word)
        {
            continue;
        }
        let full = found.get(0).expect("full figure");
        let number = &found["num"];
        if RATE_PREFIX.is_match(&after)
            || next_word == "per"
            || number.eq_ignore_ascii_case("a")
            || number.eq_ignore_ascii_case("an")
            || ACTION_DURATION.is_match(clause(said, full.start(), full.end()))
        {
            continue;
        }
        let mut value = String::new();
        if let Some(prefix) = found.name("qual") {
            value.push_str(prefix.as_str().trim_matches(is_space));
            value.push(' ');
        }
        value.push_str(number.trim_matches(is_space));
        if let Some(to) = found.name("more") {
            value.push_str(" to ");
            value.push_str(to.as_str().trim_matches(is_space));
        }
        value.push(' ');
        value.push_str(unit);
        if let Some(extra) = found.name("extra") {
            value.push(' ');
            value.push_str(extra.as_str().trim_matches(is_space));
        }
        out.push(value);
    }
    out
}

fn present(value: &str) -> Option<String> {
    let mut letters = value.chars();
    let first: String = letters.next()?.to_uppercase().collect();
    Some(format!("{first}{}.", letters.as_str()))
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let asked = flatten(&reading.raw.to_lowercase());
    let units = asked_units(&asked)?;
    let is_rate = A_RATE_ASK.is_match(&asked);
    for turn in memory.turns.iter().rev().take(data::LOOK_BACK_FIGURE) {
        if !data::ANSWER_MODULES.contains(&turn.module) {
            continue;
        }
        let said = flatten(&turn.bot);
        if said.chars().take(data::FIGURE_ANSWER_LEN).count() < data::FIGURE_ANSWER_LEN
            || said.ends_with('?')
            || (is_rate && A_RATE_STATED.is_match(&said))
            || LISTED_FIGURE.is_match(&said)
        {
            continue;
        }
        match units {
            Units::Money => {
                let mut found = money_in(&said, &MONEY);
                if found.is_empty() {
                    found = money_in(&said, &PERCENT);
                }
                if let Some(first) = found.first() {
                    let key = first.to_lowercase();
                    return found
                        .iter()
                        .all(|amount| amount.to_lowercase() == key)
                        .then(|| present(first))
                        .flatten();
                }
                if MONEY.is_match(&said).unwrap_or(false)
                    || PERCENT.is_match(&said).unwrap_or(false)
                {
                    return None;
                }
            }
            Units::Denom(mask) => {
                let patterns = patterns_for(mask);
                let found = amounts(&said, mask, &patterns);
                if let Some(first) = found.first() {
                    let key = first.to_lowercase();
                    return found
                        .iter()
                        .all(|amount| amount.to_lowercase() == key)
                        .then(|| present(first))
                        .flatten();
                }
                if patterns.loose.is_match(&said).unwrap_or(false) {
                    return None;
                }
            }
        }
    }
    None
}

#[cfg(test)]
#[path = "../../../tests/fixtures/answer_figures.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn answer_figure_read_back_matches_the_pinned_store() {
        let mut mismatches = Vec::new();
        for fixtures::Case(line, earlier, expected) in fixtures::CASES {
            let mut memory = Memory::default();
            memory
                .turns
                .extend(earlier.iter().map(|&(module, answer)| Exchange {
                    user: String::new(),
                    bot: answer.to_owned(),
                    module,
                }));
            let actual = respond(&Reading::new(line), &memory);
            if actual.as_deref() != *expected {
                mismatches.push(format!(
                    "{line} after {earlier:?}: {actual:?} vs {expected:?}"
                ));
            }
        }
        assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    }
}
