//! Repeated conduct or dismissal by a named person in a role.

use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

#[path = "receiving_end_data.rs"]
mod data;
#[cfg(test)]
#[path = "receiving_end_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    A_PERSON_WHO_KEEPS,
    KEEPS_DOING_IT,
    A_COMPLAINT_WORD,
    A_WARM_MEMORY,
    A_BRUSH_OFF,
    A_BODY_LINE,
);
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());

fn flatten(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn role_for(text: &str) -> String {
    let role = flatten(text).to_lowercase();
    if let Some((_, rendered)) = data::ROLE_SAID_BACK.iter().find(|(said, _)| *said == role) {
        return (*rendered).to_owned();
    }
    let (first, rest) = role.split_once(' ').unwrap_or((&role, ""));
    if !rest.is_empty()
        && let Some((_, theirs)) = data::THEIR.iter().find(|(person, _)| *person == first)
    {
        return format!("{theirs} {rest}");
    }
    if matches!(first, "your" | "their" | "the" | "a") {
        role
    } else {
        format!("the {role}")
    }
}

fn heard(clean: &str, repaired: &str) -> Option<(String, &'static [&'static str])> {
    if clean.trim().is_empty() {
        return None;
    }
    for line in [clean, repaired].map(flatten) {
        let Some(person) = A_PERSON_WHO_KEEPS.find(&line) else {
            continue;
        };
        let tail = &line[person.end()..];
        let tail = tail
            .char_indices()
            .nth(70)
            .map_or(tail, |(end, _)| &tail[..end]);
        let brush_off = A_BRUSH_OFF.is_match(tail);
        if !A_WARM_MEMORY.is_match(&line)
            && (brush_off || KEEPS_DOING_IT.is_match(tail) && A_COMPLAINT_WORD.is_match(&line))
        {
            let pool = if brush_off {
                data::THE_BRUSH_OFF_REPLIES
            } else {
                data::PERSON_WHO_KEEPS_REPLIES
            };
            return Some((role_for(person.as_str()), pool));
        }
    }
    None
}

fn safety_thread(memory: &Memory) -> bool {
    let recent = &memory.turns[memory.turns.len().saturating_sub(data::THREAD_TURNS)..];
    let Some(index) = recent.iter().rposition(|turn| turn.module == "safety") else {
        return false;
    };
    let answer = recent[index].bot.to_lowercase();
    if answer.is_empty() {
        return false;
    }
    let own = WORD
        .find_iter(&answer)
        .map(|word| word.as_str())
        .collect::<HashSet<_>>();
    recent[index + 1..].iter().all(|turn| {
        if turn.module == "greeting" {
            return true;
        }
        let reply = turn.bot.to_lowercase();
        WORD.find_iter(&reply)
            .map(|word| word.as_str())
            .filter(|word| word.len() >= 5 && own.contains(word))
            .collect::<HashSet<_>>()
            .len()
            >= data::THE_THREAD_KEPT
    })
}

fn grief_thread(memory: &Memory) -> bool {
    memory
        .turns
        .iter()
        .rev()
        .take(data::THREAD_TURNS)
        .find(|turn| !turn.bot.is_empty())
        .is_some_and(|turn| {
            data::LOSS_OPENINGS
                .iter()
                .any(|head| turn.bot.starts_with(head))
        })
}

fn is_protected_thread(memory: &Memory) -> bool {
    safety_thread(memory)
        || grief_thread(memory)
        || memory
            .turns
            .iter()
            .rev()
            .take(4)
            .any(|turn| compose::loss_reader::reads_as_loss(&turn.user))
        || memory
            .turns
            .iter()
            .rev()
            .take(2)
            .any(|turn| A_BODY_LINE.is_match(&flatten(&turn.user)))
}

fn plain(line: &str) -> String {
    line.split_whitespace()
        .filter(|word| !data::FILLERS.contains(word))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = if reading.line.clean.is_empty() {
        reading.folded()
    } else {
        &reading.line.clean
    };
    let (role, pool) = heard(clean, &reading.line.plain)?;
    if is_protected_thread(memory) {
        return None;
    }
    // The pinned reader compares the previous user line with the unfilled pools.
    if memory.turns.last().is_some_and(|turn| {
        let said = plain(&turn.user);
        data::PERSON_WHO_KEEPS_REPLIES
            .iter()
            .chain(data::THE_BRUSH_OFF_REPLIES)
            .any(|line| said.starts_with(&plain(line)))
    }) {
        return None;
    }
    let mut letters = role.chars();
    let capitalized = letters
        .next()?
        .to_uppercase()
        .chain(letters)
        .collect::<String>();
    let replies = pool
        .iter()
        .map(|line| {
            line.replace("{role}", &role)
                .replace("{Role}", &capitalized)
        })
        .collect::<Vec<_>>();
    let options = replies.iter().map(String::as_str).collect::<Vec<_>>();
    Some(compose::choose_recent(&options, memory, clean).to_owned())
}
