//! Fill in a doneness question from the subject the conversation has been on.
use crate::bot::{
    Memory, Reading,
    knowledge::lookup::{self, KnowledgeMatch},
    memory::done_topic::{candidate_words, topic_words},
};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

static ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(?:how (?:will|would|do|can|should) (?:i|you) (?:know|tell)|when (?:will|do) (?:i|you) know|",
        r"when (?:is|will) it(?: be)?|is it|are they|will it be|how long (?:do|should) (?:i|you) ",
        r"(?:cook|leave|bake|roast|boil|fry|grill|simmer))"
    )).expect("doneness ask")
});
static STATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:done|dones|cooked|uncooked|overdone|undercooked|overcooked|ready)\b")
        .expect("doneness state")
});

const STATES: &[&[&str]] = &[
    &["done"],
    &["cooked"],
    &["cooked", "through"],
    &["ready"],
    &["how", "can", "i", "tell"],
    &["how", "do", "i", "tell"],
    &["how", "do", "you", "tell"],
    &["when", "is", "it", "done"],
    &["when", "its", "done"],
    &["temperature"],
    &["temp"],
    &["thermometer"],
];

fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn entry(text: &str) -> Option<KnowledgeMatch> {
    lookup::matched_card(&Reading::new(&flat(text).to_lowercase()))
}

fn filed_entry(text: &str) -> Option<(&'static str, &'static str)> {
    let line = flat(text).to_lowercase();
    if crate::bot::knowledge::travel::capital(&line).is_some() {
        return Some(("travel", "capital-city"));
    }
    if let Some(card) = entry(&line) {
        return Some((card.domain, card.id));
    }
    crate::bot::knowledge::cooking::roast_chicken(&line)?;
    Some(("cooking", "roast-chicken"))
}

fn is_form(word: &str, base: &str) -> bool {
    ["ing", "ed"].into_iter().any(|ending| {
        word.strip_suffix(ending)
            .filter(|root| root.len() > 2)
            .is_some_and(|root| {
                (root.len() >= 4 && root == base)
                    || base.strip_suffix('e') == Some(root)
                    || (root.len() > 3
                        && root.as_bytes()[root.len() - 1] == root.as_bytes()[root.len() - 2]
                        && !root.ends_with(['a', 'e', 'i', 'o', 'u'])
                        && &root[..root.len() - 1] == base)
            })
    })
}

fn about(first: &[&str], subject: &str) -> bool {
    first
        .iter()
        .flat_map(|phrase| topic_words(phrase))
        .any(|word| word == subject || is_form(&word, subject) || is_form(subject, &word))
}

fn says_state(groups: &[&[&str]]) -> bool {
    groups.iter().any(|group| {
        let words: HashSet<String> = group.iter().flat_map(|part| topic_words(part)).collect();
        STATES
            .iter()
            .any(|state| state.iter().all(|word| words.contains(*word)))
    })
}

fn moved_on(card: &KnowledgeMatch, said: &str, topics: &[&str]) -> bool {
    topics
        .iter()
        .map(|topic| flat(topic))
        .filter(|topic| topic != said)
        .find_map(|topic| filed_entry(&topic))
        .is_some_and(|(domain, _)| domain != card.domain)
}

fn fill(line: &str, previous_answer: bool, topics: &[&str]) -> Option<String> {
    let said = flat(line).to_lowercase();
    if !previous_answer || !ASK.is_match(&said) || !STATE.is_match(&said) {
        return None;
    }
    let words = topic_words(&said);
    for word in candidate_words(topics.iter().copied(), &words) {
        for filled in [
            format!("{word} {said}"),
            format!("{} {word}", said.trim_end_matches(['?', '.', '!', ' '])),
        ] {
            let Some(card) = entry(&filled) else {
                continue;
            };
            let key = card.keys();
            if key.first().is_some_and(|group| about(group, &word)) && says_state(key) {
                return (!moved_on(&card, &said, topics)).then_some(filled);
            }
        }
    }
    None
}

pub(crate) fn observe(reading: &mut Reading<'_>, memory: &mut Memory) {
    memory
        .doneness_topics
        .note(reading, &memory.turns, filed_entry);
    if crate::bot::memory::history::in_a_safety_thread(memory) {
        return;
    }
    let prior = memory.turns.iter().any(|turn| !turn.bot.is_empty());
    if !prior || !ASK.is_match(&reading.clean) || !STATE.is_match(&reading.clean) {
        return;
    }
    let topics = memory.doneness_topics.texts();
    if let Some(text) = fill(&reading.clean, prior, &topics) {
        let words = topic_words(&text);
        reading.observe_words(text, words);
    }
}

#[cfg(test)]
#[path = "done_ask_tests.rs"]
mod pinned;
