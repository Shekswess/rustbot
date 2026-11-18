//! An action in the user's report points to the next clause in their previous how-to answer.
#[path = "step_after_data.rs"]
mod data;
#[cfg(test)]
#[path = "step_after_tests.rs"]
mod tests;

use super::{cap, seen};
use crate::bot::Memory;
use fancy_regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    WORD,
    HOW_TO,
    A_QUESTION,
    FORBADE,
    CLAUSE,
    THEY_DID,
    FINISHED_IT,
    NOT_A_STEP,
    TENSE,
    QUALIFIER
);
const EDGE: &[char] = &[' ', ',', ';', ':'];
static JOIN_LEAD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:and|then|but|so|also|plus|next)\s+").unwrap());

fn words(text: &str) -> Vec<String> {
    let low = text.to_lowercase().replace('’', "'");
    WORD.find_iter(&low)
        .filter_map(Result::ok)
        .map(|word| word.as_str().to_owned())
        .collect()
}

fn stem(word: &str) -> String {
    let alternate = (word.len() > 4)
        .then(|| word.strip_suffix("ied"))
        .flatten()
        .map(|root| format!("{root}y"));
    let root = TENSE.replace(alternate.as_deref().unwrap_or(word), "");
    let bytes = root.as_bytes();
    if bytes.len() > 3
        && bytes[bytes.len() - 1] == bytes[bytes.len() - 2]
        && !b"lsz".contains(&bytes[bytes.len() - 1])
    {
        return root[..root.len() - 1].into();
    }
    root.into_owned()
}

fn doing(word: &str) -> String {
    if word.len() < 3 {
        return String::new();
    }
    if word.ends_with("ing") {
        return word.into();
    }
    let root = if let Some(part) = word.strip_suffix("ied").filter(|_| word.len() > 4) {
        format!("{part}y")
    } else if let Some(part) = word
        .strip_suffix("ed")
        .or_else(|| word.strip_suffix('s'))
        .filter(|_| word.len() > 3)
    {
        part.into()
    } else {
        word.into()
    };
    let bytes = root.as_bytes();
    if let [.., before, last] = bytes
        && bytes.len() > 3
        && before == last
        && !b"lsz".contains(last)
    {
        return format!("{root}ing");
    }
    if let Some(part) = root.strip_suffix('e').filter(|_| root.len() > 3) {
        return format!("{part}ing");
    }
    if let [.., first, before, last] = bytes
        && !b"aeiouwxy".contains(last)
        && b"aeiou".contains(before)
        && !b"aeiou".contains(first)
    {
        return format!("{root}{}ing", char::from(*last));
    }
    format!("{root}ing")
}

fn steps(answer: &str) -> Vec<String> {
    CLAUSE
        .split(answer)
        .filter_map(Result::ok)
        .filter_map(|part| {
            let mut words: Vec<_> = part
                .trim_matches([' ', '.', ';', ',', '-'])
                .split_whitespace()
                .collect();
            while words
                .last()
                .is_some_and(|word| data::JOINING.contains(&word.to_lowercase().trim_matches(EDGE)))
            {
                words.pop();
            }
            let part = words.join(" ");
            let part = part.trim_matches(EDGE);
            (!part.is_empty()).then(|| part.to_owned())
        })
        .collect()
}

fn whole_step(said: &str) -> String {
    let mut words: Vec<_> = said.split_whitespace().collect();
    if words.len() <= data::STEP_WORDS {
        return said.trim_matches(EDGE).into();
    }
    words.truncate(data::STEP_WORDS);
    if let Some(at) = (3..words.len())
        .rev()
        .find(|at| words[at - 1].ends_with([':', ';', ',']))
    {
        words.truncate(at);
    } else {
        while words.len() > 2
            && words.last().is_some_and(|word| {
                word.ends_with([':', ';', ','])
                    || data::DANGLING.contains(&word.to_lowercase().trim_matches(EDGE))
            })
        {
            words.pop();
        }
    }
    words.join(" ").trim_matches(EDGE).into()
}

fn their_action(words: &[String]) -> Option<String> {
    words
        .iter()
        .rev()
        .filter(|word| {
            word.len() >= 4 && !data::FRAME.contains(&word.as_str()) && seen(&TENSE, word)
        })
        .map(|word| stem(word))
        .find(|word| word.len() > 2 && !data::FRAME.contains(&word.as_str()))
}

pub(super) fn reply(text: &str, memory: &Memory) -> Option<String> {
    let said_words = words(text);
    let last = memory.turns.last()?;
    if said_words.is_empty()
        || said_words.len() > data::SHORT
        || seen(&A_QUESTION, text)
        || last.module != "answers"
        || !seen(&HOW_TO, &last.user)
        || !seen(&THEY_DID, text)
        || seen(&NOT_A_STEP, text)
    {
        return None;
    }
    let steps = steps(&last.bot);
    if steps.len() < 2 {
        return None;
    }
    let action = their_action(&said_words);
    let finished = seen(&FINISHED_IT, text);
    let things: HashSet<_> = said_words
        .iter()
        .filter(|word| finished && word.len() > 3 && !data::FRAME.contains(&word.as_str()))
        .map(|word| stem(word))
        .collect();
    if action.is_none() && things.is_empty() {
        return None;
    }
    let mut possible = steps
        .iter()
        .enumerate()
        .filter(|(_, step)| !seen(&FORBADE, step));
    let done = possible
        .clone()
        .find(|(_, step)| {
            action
                .as_ref()
                .is_some_and(|action| words(step).iter().any(|word| stem(word) == *action))
        })
        .or_else(|| {
            possible.find(|(_, step)| words(step).iter().any(|word| things.contains(&stem(word))))
        })?;
    let next = steps[done.0 + 1..]
        .iter()
        .find(|step| !seen(&QUALIFIER, step))?;
    let next = JOIN_LEAD.replace(next, "");
    if next.split_whitespace().count() < 2
        || words(&next)
            .iter()
            .all(|word| data::POINTS_BACK.contains(&word.as_str()))
    {
        return None;
    }
    let next = whole_step(&next);
    let first = next.split_whitespace().next()?;
    let upper = first.chars().any(char::is_uppercase) && !first.chars().any(char::is_lowercase);
    let next = if upper && (first.chars().count() > 1 || first == "I") {
        next
    } else {
        cap(&next, false)
    };
    let named = action
        .filter(|act| words(done.1).first().is_some_and(|word| stem(word) == *act))
        .map(|act| doing(&act))
        .filter(|word| !matches!(word.as_str(), "" | "do" | "doing" | "go" | "going" | "did"));
    let choice = next.chars().map(|c| c as u32).sum::<u32>() % 3;
    let lead = match (named, choice) {
        (Some(named), 0) => format!("Good - {named} first is right. Now {next}."),
        (Some(named), 1) => format!("Right - {named} is the first bit done. Now {next}."),
        _ => format!("That was the right first move. Next, {next}."),
    };
    Some(lead.replace("Now then ", "Now "))
}
