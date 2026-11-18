//! A reported recurrence or a conditional follow-up takes the branch already given.
#[path = "returned_data.rs"]
mod data;
#[cfg(test)]
#[path = "returned_tests.rs"]
mod tests;
use super::{Carried, MemoryNote, cap, seen, sentences, starts};
use crate::bot::{Memory, memory::again};
use fancy_regex::Regex;
use std::sync::LazyLock;

static NEXT_STEP: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::NEXT_STEP).unwrap());
static LEADING_CONDITION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::LEADING_CONDITION).unwrap());
static CONDITIONAL_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::CONDITIONAL_ASK).unwrap());

fn flatten(text: &str) -> String {
    text.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn next_step(answer: &str) -> Option<String> {
    for sentence in sentences(answer) {
        if !seen(&NEXT_STEP, sentence) {
            continue;
        }
        if let Some(lead) = LEADING_CONDITION.find(sentence).ok().flatten() {
            let bare = &sentence[lead.end()..];
            let mut words = bare.split_whitespace();
            if let Some(first) = words.next()
                && words.count() >= 3
                && !data::CONTINUES
                    .contains(&first.to_lowercase().trim_matches([',', '.', ';', ':']))
            {
                return Some(cap(bare, true));
            }
        }
        let lower = sentence.to_lowercase();
        if !["if ", "when ", "once "]
            .iter()
            .any(|lead| lower.starts_with(lead))
        {
            return Some(sentence.to_owned());
        }
    }
    None
}

fn prior_step(memory: &Memory) -> Option<(&str, String)> {
    for turn in memory.turns.iter().rev().take(4) {
        if data::NOT_AN_ANSWER.contains(&turn.module) {
            return None;
        }
        if let Some(step) = next_step(&turn.bot) {
            return Some((&turn.bot, step));
        }
    }
    None
}

pub(super) fn reply(text: &str, memory: &Memory) -> Option<Carried> {
    let conditional = starts(&CONDITIONAL_ASK, &flatten(text));
    let recurs = again::reads_as_returning(text);
    if !conditional && !recurs {
        return None;
    }
    let (said, step) = prior_step(memory)?;
    if flatten(said) == flatten(&step) {
        return None;
    }
    if conditional && said.replace(&step, " ").split_whitespace().count() <= 8 {
        return None;
    }
    let remembered = if recurs { said } else { &step };
    let remembered = Carried::hand_back(remembered.to_owned(), memory).reply;
    Some(Carried {
        earlier_handback: None,
        reply: step,
        note: MemoryNote::Whole(remembered),
    })
}
