//! An attempt named in the user's own words, after advice about a problem.
#[cfg(test)]
#[path = "remedy_tests.rs"]
mod tests;

use crate::bot::{Memory, compose};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::sync::LazyLock;

static ATTEMPT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^\W*(?:(?:and|but|so|oh|ok|okay|well|right|alright)\s+)?",
        r"(?:i|i've|ive|i'd|we|we've|weve)\s*",
        r"(?:have\s+|had\s+|just\s+|already\s+|also\s+|finally\s+)?",
        r"(?P<verb>tried|took|used|gave|been using|been trying)\s+(?P<thing>.+)$",
    ))
    .unwrap()
});
static SHIFT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:but|so|then|though|because|again|twice|afterwards)\b").unwrap()
});
static AND: LazyLock<Fancy> = LazyLock::new(|| {
    Fancy::new(concat!(
        r"\band\b(?=\s+(?:it|its|it's|they|i|we|now|still|nothing|didnt|doesnt|",
        r"no\b|the\s+\w+\s+(?:did|helped|worked)))",
    ))
    .unwrap()
});
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9']+").unwrap());
static PROBLEM_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static VERDICT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:did\s?n'?t|does\s?n'?t|not|no|never|helped|helps|worked|works|better|worse|",
        r"relief|eased|eases|nothing|useless|pointless|yet|again|still)\b",
    ))
    .unwrap()
});
static ADVICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:try|trying|keep|rest|ice|see (?:a|your) (?:gp|doctor))\b").unwrap()
});

const PROBLEMS: &str = "hurt hurts hurting hurtful pain painful ache aches aching achy achey sore
ill sick illness unwell cough coughing cold flu headache migraine fever temperature back knee
shoulder neck ankle wrist hip throat stomach belly tooth toothache ear earache rash itch itchy burn
burnt sprain sprained stiff stiffness swelling swollen dizzy faint sleep
sleeping slept insomnia wake waking wakes sneeze sneezing blocked stuffy running congestion
bleeding bleed cut bruise bruised lump nausea vomit diarrhoea constipated constipation heartburn
reflux bloat bloated cramp cramps period contractions
dripping drips leaking leaks leaked leak broken broke cracked squeaks squeaking squeak clogged
banging rattling stuck jammed seized mould mildew damp stains stained smell smells
wont won'ts starts start starting
freezing freezes frozen slow slowly crashing crashes crashed glitching lagging laggy dropping
drops disconnects restart reboots overheating";

const NO_OBJECT: &[&str] = &[
    "that",
    "this",
    "it",
    "them",
    "those",
    "these",
    "everything",
    "all",
    "anything",
    "something",
    "nothing",
    "to",
    "for",
    "at",
    "in",
    "on",
    "with",
    "about",
    "of",
];
const TAIL_ARTICLES: &[&str] = &["a", "an", "the", "some", "my", "our", "their"];
const HEAD_ARTICLES: &[&str] = &[
    "a", "an", "the", "some", "my", "our", "their", "his", "her", "of", "that", "this",
];

fn the_thing(rest: &str, verb: &str) -> Option<String> {
    let clause = SHIFT.find(rest).map_or(rest, |mark| &rest[..mark.start()]);
    let clause = AND
        .find(clause)
        .ok()
        .flatten()
        .map_or(clause, |mark| &clause[..mark.start()]);
    let mut words: Vec<_> = WORD.find_iter(clause).map(|word| word.as_str()).collect();
    if NO_OBJECT.contains(words.first()?) {
        return None;
    }
    if verb == "gave" {
        let (stem, ending) = words.split_at(words.len().saturating_sub(2));
        if ending != ["a", "go"] && ending != ["a", "try"] {
            return None;
        }
        words.truncate(stem.len());
    }
    words.truncate(5);
    while words
        .last()
        .is_some_and(|word| TAIL_ARTICLES.contains(word))
    {
        words.pop();
    }
    let begin = words
        .iter()
        .take_while(|word| HEAD_ARTICLES.contains(word))
        .count();
    (begin < words.len()).then(|| words[begin..].join(" "))
}

pub(super) fn reply(clean: &str, memory: &Memory) -> Option<String> {
    let found = ATTEMPT.captures(clean)?;
    let rest = found.name("thing")?.as_str();
    let thing = the_thing(rest, found.name("verb")?.as_str())?;
    let suffix = rest
        .char_indices()
        .nth(thing.len())
        .map_or("", |(at, _)| &rest[at..]);
    if VERDICT.is_match(suffix) {
        return None;
    }
    if !memory.turns.iter().rev().take(3).any(|turn| {
        PROBLEM_WORD
            .find_iter(&turn.user.to_lowercase())
            .any(|word| {
                PROBLEMS
                    .split_whitespace()
                    .any(|problem| problem == word.as_str())
            })
    }) || !memory.turns.iter().rev().take(4).any(|turn| {
        matches!(turn.module, "answers" | "safety")
            || ADVICE.is_match(&turn.bot.to_lowercase())
            || turn.bot.trim().ends_with('?')
    }) {
        return None;
    }
    let cap = super::cap(&thing, true);
    let options = [
        format!("Did the {thing} take the edge off, or nothing?"),
        format!("{cap} - did that do anything?"),
        format!("And did the {thing} make a difference?"),
        format!("{cap} - any help at all?"),
        format!("How did the {thing} go - anything, or no difference?"),
    ];
    let said = memory
        .turns
        .iter()
        .map(|turn| turn.bot.as_str())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let mut unseen: Vec<_> = options
        .iter()
        .filter(|option| !said.contains(&option.to_lowercase()))
        .map(String::as_str)
        .collect();
    if unseen.is_empty() {
        unseen.extend(options.iter().map(String::as_str));
    }
    Some(compose::seeded_choice(&unseen, memory, "").into())
}
