//! Keep a fresh answer ahead of a repeated fragment from the previous reply.

use crate::bot::{Candidate, Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;

#[path = "arbiter_echo_data.rs"]
mod data;

static WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-z0-9]+(?:'[a-z]+)?").expect("arbiter reply words"));

fn content(reply: &str) -> String {
    WORD.find_iter(&reply.to_lowercase())
        .map(|word| word.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn echoes(reply: &str, previous: &[&str]) -> bool {
    let words: Vec<_> = reply.split(' ').filter(|word| !word.is_empty()).collect();
    if previous.len() < 12 || words.len() < 12 {
        return false;
    }
    let run_of = |run: &[&str]| {
        run.len() <= previous.len() && previous.windows(run.len()).any(|window| window == run)
    };
    if (0..=6.min(words.len() - 12)).any(|lead| run_of(&words[lead..])) {
        return true;
    }
    data::LEADS.iter().any(|head| {
        reply.strip_prefix(head).is_some_and(|quoted| {
            let run: Vec<_> = quoted.split_whitespace().collect();
            run.len() >= 8 && run_of(&run)
        })
    })
}

pub(super) fn should_step_back(
    reading: &Reading<'_>,
    top: &Candidate,
    other: &[Candidate],
    memory: &Memory,
) -> bool {
    if top.module != "a_measure_asked_again"
        && (top.module != "chat" || crate::bot::memory::asks_for_repeat(reading.raw))
    {
        return false;
    }
    let Some(last) = memory.turns.last() else {
        return false;
    };
    let last = content(&last.bot);
    let previous: Vec<_> = last.split_whitespace().collect();
    let reply = content(&top.reply);
    let words: Vec<_> = reply.split_whitespace().collect();
    let identical = words
        .iter()
        .zip(&previous)
        .take_while(|(a, b)| a == b)
        .count();
    let is_tail = previous.len() >= data::AGAIN_FIRST
        && identical >= data::AGAIN_FIRST
        && words.len() - identical <= data::AGAIN_TAIL;
    (is_tail || echoes(&reply, &previous))
        && other
            .iter()
            .take_while(|candidate| candidate.priority >= 0.0)
            .map(|candidate| content(&candidate.reply))
            .any(|reply| !reply.is_empty() && reply != last && !echoes(&reply, &previous))
}
