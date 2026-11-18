//! Reply to a short personal qualifier using the condition in the preceding knowledge answer.
#[path = "qualifier_data.rs"]
mod data;
use crate::bot::{Memory, Reading};
use data::*;
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

fn clean(s: &str) -> String {
    s.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn qualifier(s: &str) -> bool {
    let mut words = s.split_whitespace();
    let Some(head) = words.next() else {
        return false;
    };
    !s.contains('?')
        && words.count() < 10
        && QUESTION_START
            .binary_search(&head.trim_matches([',', '.']))
            .is_err()
        && !CORRECTION.is_match(s)
        && (NEGATION.is_match(s)
            || AMOUNT.is_match(s)
            || s.contains("kind")
            || s.contains("sort of"))
}
fn content(s: &str) -> HashSet<&str> {
    static WORDS: LazyLock<Regex> = LazyLock::new(|| Regex::new("[a-z']+").unwrap());
    WORDS
        .find_iter(s)
        .map(|m| m.as_str().trim_matches('\''))
        .filter(|w| w.len() >= 4 && STOP.binary_search(w).is_err())
        .map(|w| {
            if w.len() > 4 {
                w.strip_suffix('s').unwrap_or(w)
            } else {
                w
            }
        })
        .collect()
}
fn condition(answer: &str) -> Option<String> {
    static BREAK: LazyLock<fancy_regex::Regex> =
        LazyLock::new(|| fancy_regex::Regex::new(r"(?<=[.!?])\s+").unwrap());
    let sentence = BREAK
        .split(answer.trim())
        .flatten()
        .filter(|sentence| {
            let low = sentence.to_lowercase();
            sentence.split_whitespace().count() >= 5
                && CONDITION.is_match(&low)
                && (HANDOFF.is_match(&low) || LIMIT.is_match(&low))
        })
        .last()?;
    let mut words = sentence
        .trim_end_matches([' ', '.', ';', ':', ','])
        .split_whitespace();
    let mut clause = words.by_ref().take(34).collect::<Vec<_>>().join(" ");
    if words.next().is_some()
        && let Some(comma) = clause.rfind(',')
        && clause[..comma].chars().count() > 60
    {
        clause.truncate(comma)
    }
    let clause = clause.trim_end_matches([' ', ',', ';', ':']);
    let mut head = clause.split_whitespace().next()?.chars();
    let first = head.next()?;
    let rest = head.as_str();
    if first.is_uppercase()
        && rest.chars().any(char::is_lowercase)
        && !rest.chars().any(char::is_uppercase)
    {
        Some(format!(
            "{}{}",
            first.to_lowercase(),
            &clause[first.len_utf8()..]
        ))
    } else {
        Some(clause.to_owned())
    }
}
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let last = memory.turns.last().filter(|t| t.module == "answers")?;
    let s = clean(reading.raw);
    if !qualifier(&s) {
        return None;
    }
    let clause = condition(&last.bot)?;
    if !content(&s).is_disjoint(&content(&clean(&clause))) {
        return None;
    }
    let sum = |v: &str| v.chars().map(|c| c as usize).sum::<usize>();
    let lead = LEAD[sum(&s) % LEAD.len()];
    Some(
        POOL[(sum(&s) + sum(&clause)) % POOL.len()]
            .replace("{lead}", lead)
            .replace("{clause}", &clause),
    )
}

#[cfg(test)]
#[path = "../../../tests/fixtures/what_decides_it.rs"]
mod reference;
#[test]
fn follows_reference_conditions() {
    let mut memory = Memory::default();
    for &(index, prompt, expected) in reference::CASES {
        memory.turns.clear();
        memory.turns.push(crate::bot::Exchange {
            module: "answers",
            bot: reference::PRIOR[index].into(),
            user: "earlier".into(),
        });
        assert_eq!(
            respond(&Reading::new(prompt), &memory).as_deref(),
            expected,
            "{index} / {prompt}"
        );
    }
}
