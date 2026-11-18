//! Topic fit when a missing possession detail and a knowledge card both propose.

use super::fit::Fit;
use crate::bot::{Candidate, Memory, Reading, memory};
use regex::Regex;
use std::sync::LazyLock;

include!("arbiter_possession_data.rs");
static TERMS: LazyLock<Regex> = LazyLock::new(|| Regex::new(WORD).expect("proposal terms"));
static LETTERS: LazyLock<Regex> = LazyLock::new(|| Regex::new("[a-z]+").expect("reply letters"));

fn stem(word: &str) -> Option<String> {
    if word.len() < 3 || STOP.binary_search(&word).is_ok() {
        return None;
    }
    for ending in ["ies", "es", "s"] {
        if let Some(root) = word.strip_suffix(ending)
            && root.len() > 2
        {
            return Some(if ending == "ies" {
                format!("{root}y")
            } else {
                root.to_owned()
            });
        }
    }
    Some(word.to_owned())
}

fn prefix(one: &str, other: &str) -> bool {
    one.starts_with(other) || other.starts_with(one)
}

fn about(reply: &str, words: &[&str]) -> bool {
    let reply = reply.to_lowercase();
    let said: Vec<_> = TERMS
        .find_iter(&reply)
        .filter_map(|word| stem(word.as_str()))
        .collect();
    !said.is_empty()
        && words.iter().any(|word| {
            stem(word).map_or_else(
                || {
                    word.len() >= 4
                        && TERMS
                            .find_iter(&reply)
                            .any(|item| prefix(word, item.as_str()))
                },
                |word| {
                    said.iter().any(|one| {
                        *one == word || (word.len() >= 4 && one.len() >= 4 && prefix(one, &word))
                    })
                },
            )
        })
}

fn only_says_line(reply: &str, words: &[&str]) -> bool {
    let lower = reply.to_lowercase();
    let said: Vec<_> = LETTERS
        .find_iter(&lower)
        .map(|word| word.as_str())
        .filter(|word| word.len() >= 4)
        .collect();
    if said.is_empty() || said.len() > 8 || words.iter().all(|word| word.len() < 4) {
        return false;
    }
    let new = said
        .iter()
        .filter(|word| !words.iter().any(|one| one.len() >= 4 && prefix(one, word)))
        .count();
    new * 2 <= said.len()
}

pub(super) fn candidate<'a>(
    reading: &Reading<'_>,
    memory: &Memory,
    proposals: &'a [Candidate],
    first: &Candidate,
) -> Option<&'a Candidate> {
    if !matches!(first.module, "memory_store" | "said_fact_back") {
        return None;
    }
    let part = memory::possession_parts::respond(reading, memory)?;
    if !proposals
        .iter()
        .any(|candidate| candidate.module == "memory_store" && candidate.reply == part)
    {
        return None;
    }
    let answer = proposals
        .iter()
        .find(|proposal| proposal.module == "answers")?;
    let topic = Fit::new(reading);
    let carried = Fit::carried(memory.ack_topics.stored_topic(&memory.turns));
    if topic.score(answer.module, &carried) <= topic.score(first.module, &carried) {
        return None;
    }
    let clean = reading.clean.to_lowercase();
    let words: Vec<_> = TERMS.find_iter(&clean).map(|word| word.as_str()).collect();
    if about(&first.reply, &words)
        && !only_says_line(&first.reply, &words)
        && !about(&answer.reply, &words)
    {
        return None;
    }
    Some(answer)
}
