//! Household-object and service guidance for an otherwise unanswered detail ask.

use super::seeded_choice;
use crate::bot::Memory;
use crate::text::is_space;
use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

#[path = "what_i_do_know_data.rs"]
mod data;
#[path = "record_and_schedule.rs"]
mod record_and_schedule;
use data::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Animal,
    Appliance,
    Bought,
    Clinical,
    Device,
    Eyewear,
    Living,
    Paper,
    Part,
    Place,
    Plant,
    Service,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    Figure,
    Frequency,
    Method,
    Place,
    Reason,
    Time,
}

struct Rule {
    kind: Kind,
    part: Part,
    keeper: &'static str,
    missing: &'static str,
    known: &'static [&'static str],
    follow_up: &'static [&'static str],
}

static THING_AFTER_DETERMINER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:my|our|the|this|these|a|an)\s+([a-z][a-z'-]+(?:\s+[a-z][a-z'-]+){0,3})")
        .unwrap()
});
static POSSIBLE_THING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z][a-z'-]+").unwrap());
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());

/// Add the known part and its source when the fallback has reached a generic hedge.
pub(crate) fn reply_for(raw: &str, memory: &Memory) -> Option<String> {
    let said = raw.to_lowercase();
    if let Some(reply) = record_and_schedule::reply(&said, memory) {
        return Some(reply);
    }
    if said.trim_matches(is_space).is_empty()
        || A_PERSON.is_match(&said)
        || in_safety_thread(memory)
        || thread_names_a_body(memory)
        || COOKING.is_match(&said)
        || A_PURCHASE.is_match(&said)
        || NUTRITION.is_match(&said)
        || ANIMAL_AS_A_SOURCE.is_match(&said)
    {
        return None;
    }
    let before = previous_distinct_line(&said, memory);
    if A_BUYING_NOW.is_match(before) {
        return None;
    }
    let thing = named_thing(&said, before)?;
    if A_PERSON.is_match(&thing) || ANIMAL_AS_A_SOURCE.is_match(&thing) {
        return None;
    }
    let part = PART_READERS
        .iter()
        .find_map(|(part, pattern)| pattern.is_match(&said).then_some(*part))?;
    let kind = if thing
        .split(is_space)
        .all(|word| PART_OF_SOMETHING.contains(&word))
    {
        Kind::Part
    } else {
        recognized_kind(&said)
            .or_else(|| recognized_kind(&thing))
            .or_else(|| recognized_kind(before))
            .or_else(|| kind_from_ownership(&thing, [&said, before]))?
    };
    let rule = RULES
        .iter()
        .find(|rule| rule.kind == kind && rule.part == part)?;
    let determiner = if thing.ends_with('s') && !SINGULAR_BUT_ENDS_IN_S.contains(&thing.as_str()) {
        "those"
    } else {
        "that"
    };
    let variants: Vec<_> = rule.known.iter().flat_map(|known| {
        let thing = thing.as_str();
        rule.follow_up.iter().map(move |next| {
            format!(
                "I can't check {} for {determiner} {thing}, sorry. What I can say is that {known}. {} {next}",
                rule.missing, rule.keeper,
            )
        })
    }).collect();
    let options = variants.iter().map(String::as_str).collect::<Vec<_>>();
    // The upstream chooser receives the current reading, whose keys omit turns.
    Some(seeded_choice(&options, &Memory::default(), &said).to_owned())
}

fn previous_distinct_line<'a>(said: &str, memory: &'a Memory) -> &'a str {
    let said = said.trim_matches(is_space);
    memory
        .turns
        .iter()
        .rev()
        .find_map(|turn| {
            let user = turn.user.trim_matches(is_space);
            (!user.is_empty() && user.to_lowercase() != said).then_some(user)
        })
        .unwrap_or("")
}

fn recognized_kind(said: &str) -> Option<Kind> {
    if ANIMAL_AS_A_SOURCE.is_match(said) {
        return None;
    }
    KIND_READERS
        .iter()
        .find_map(|(kind, pattern)| pattern.is_match(said).then_some(*kind))
}

fn named_thing(said: &str, before: &str) -> Option<String> {
    let before_thing = named_after_correction(before, false);
    let points_back = REFERS_BACK
        .find(said)
        .is_some_and(|pronoun| !A_DETERMINER.is_match(&said[..pronoun.start()]));
    if points_back && before_thing.is_some() {
        return before_thing;
    }
    named_after_correction(said, true)
        .or_else(|| named_entity(said).map(str::to_owned))
        .or(before_thing)
}

fn named_entity(said: &str) -> Option<&str> {
    WORD.find_iter(said).map(|word| word.as_str()).find(|word| {
        if NOT_A_THING.contains(word) {
            return false;
        }
        let is_entity = |candidate| ENTITY_THINGS.binary_search(&candidate).is_ok();
        is_entity(word)
            || (word.len() > 3
                && [
                    Some(word.trim_end_matches('s')),
                    word.strip_suffix("es"),
                    word.strip_suffix("ies"),
                ]
                .into_iter()
                .flatten()
                .any(is_entity))
    })
}

fn named_after_correction(said: &str, determiner_only: bool) -> Option<String> {
    CORRECTION
        .find(said)
        .and_then(|mark| thing_from_text(&said[mark.end()..], true))
        .or_else(|| thing_from_text(said, determiner_only))
}

fn thing_from_text(said: &str, determiner_only: bool) -> Option<String> {
    for phrase in THING_AFTER_DETERMINER.captures_iter(said) {
        let mut words: Vec<_> = phrase
            .get(1)?
            .as_str()
            .split(is_space)
            .filter(|word| !word.is_empty())
            .skip_while(|word| MODIFIERS.contains(word))
            .take_while(|word| !MODIFIERS.contains(word) && !NOT_A_THING.contains(word))
            .take(2)
            .collect();
        remove_particles(&mut words);
        if !words.is_empty() {
            return Some(words.join(" "));
        }
    }
    if determiner_only {
        return None;
    }
    let words: Vec<_> = POSSIBLE_THING
        .find_iter(said)
        .map(|word| word.as_str())
        .collect();
    let mut last_run = words
        .rsplit(|word| NOT_A_THING.contains(word))
        .find(|run| !run.is_empty())?
        .iter()
        .rev()
        .take(2)
        .copied()
        .collect::<Vec<_>>();
    last_run.reverse();
    remove_particles(&mut last_run);
    let best = last_run.join(" ");
    (!best.is_empty()
        && best.len() >= 3
        && last_run
            .iter()
            .all(|word| word.bytes().all(|byte| byte.is_ascii_alphabetic())))
    .then_some(best)
}

fn remove_particles(words: &mut Vec<&str>) {
    while words.last().is_some_and(|word| PARTICLES.contains(word)) {
        words.pop();
    }
}

fn kind_from_ownership(thing: &str, sources: [&str; 2]) -> Option<Kind> {
    if thing.split(' ').all(|word| NOT_AN_OBJECT.contains(&word)) {
        return None;
    }
    let mine = format!("my {thing}");
    let ours = format!("our {thing}");
    let arrived = sources
        .iter()
        .map(|source| source.to_lowercase())
        .any(|source| {
            source.contains(thing)
                && (source.contains(&mine)
                    || source.contains(&ours)
                    || GOT_IT.is_match(&source)
                    || OWNED_ANYWAY.is_match(&source))
        });
    if !arrived {
        return None;
    }
    if LIVING.is_match(thing) {
        Some(Kind::Living)
    } else if CLINICAL.is_match(thing) {
        Some(Kind::Clinical)
    } else if ON_THE_BODY.is_match(thing) {
        None
    } else {
        Some(Kind::Bought)
    }
}

fn thread_names_a_body(memory: &Memory) -> bool {
    memory.turns.iter().rev().take(4).any(|turn| {
        let said = turn.user.to_lowercase();
        let without_forecast = FORECAST.replace(&said, " ");
        let without_analogy = BODY_WORD_FOR_A_THING.replace_all(&without_forecast, " ");
        let words: Vec<_> = WORD
            .find_iter(&without_analogy)
            .map(|word| word.as_str())
            .collect();
        let mut idiom = vec![false; words.len()];
        for phrase in NON_BODY_IDIOMS {
            for (start, _) in words
                .windows(phrase.len())
                .enumerate()
                .filter(|(_, window)| *window == *phrase)
            {
                idiom[start..start + phrase.len()].fill(true);
            }
        }
        words
            .iter()
            .zip(idiom)
            .any(|(word, idiom)| !idiom && BODY_THREAD_WORDS.binary_search(word).is_ok())
    })
}

fn in_safety_thread(memory: &Memory) -> bool {
    const PREVIOUS_TURNS: usize = 12;
    let window = &memory.turns[memory.turns.len().saturating_sub(PREVIOUS_TURNS)..];
    let Some(index) = window.iter().rposition(|turn| turn.module == "safety") else {
        return false;
    };
    let answer = window[index].bot.to_lowercase();
    if answer.is_empty() {
        return false;
    }
    let original_words: HashSet<_> = WORD.find_iter(&answer).map(|word| word.as_str()).collect();
    window[index + 1..].iter().all(|turn| {
        turn.module == "greeting" || {
            let reply = turn.bot.to_lowercase();
            let retained: HashSet<_> = WORD
                .find_iter(&reply)
                .map(|word| word.as_str())
                .filter(|word| word.len() >= 5 && original_words.contains(word))
                .collect();
            retained.len() >= 3
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_the_explicit_object_before_the_ordinary_entity_vocabulary() {
        let memory = Memory::default();
        assert_eq!(
            reply_for("how often should i check humidifiers?", &memory),
            None
        );
        let explicit = reply_for("how often should i check the humidifier?", &memory).unwrap();
        assert!(explicit.starts_with("I can't check the schedule for that humidifier, sorry."));
        let implicit = reply_for("what size is humidifiers?", &memory).unwrap();
        assert!(implicit.starts_with("I can't check the figure for that size, sorry."));
    }

    #[test]
    fn asks_about_the_previous_object_or_leaves_a_safety_thread_alone() {
        let mut memory = Memory::default();
        memory.turns.push(crate::bot::Exchange {
            user: "i got a dehumidifier".into(),
            bot: "Here are the usual checks for it.".into(),
            module: "answers",
        });
        let reply = reply_for("why does it make a noise?", &memory).unwrap();
        assert!(reply.starts_with("I can't check the reason for that dehumidifier, sorry."));
        memory.turns.push(crate::bot::Exchange {
            user: "call immediately".into(),
            bot: "Call emergency services now.".into(),
            module: "safety",
        });
        assert_eq!(
            reply_for("why does the air purifier keep buzzing?", &memory),
            None
        );
    }
}
