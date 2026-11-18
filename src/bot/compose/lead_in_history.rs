//! The fallback's evidence for an answer about an earlier subject.

use crate::bot::{Exchange, Memory};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

#[path = "lead_in_history_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    ACK_ONLY,
    CHAT_QUESTION,
    FIRST_SAID_BY_BOT,
    FIRST_THING,
    MORE_ON_THIS,
    PERSON_NAMED,
    REACTED_ONLY,
    RECALL_ABOUT,
    RECALL_SHAPE,
    REMIND_ME_RECAP,
    SAID_ABOUT_HEAD,
    STORE_SUMMARY,
    TALKED_ABOUT,
    WHAT_ABOUT,
    WHERE_WE_WERE,
    WORD,
);

static NOTHING_SAID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"^(?:{}|{})$",
        data::NOTHING_SAID,
        data::GREETING_WITH_AN_ADDRESS
    ))
    .unwrap()
});

fn flat(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches([' ', '?', '.', '!', ','])
        .to_owned()
}

fn names_prior_chat(text: &str) -> bool {
    let named = SAID_ABOUT_HEAD
        .captures(text)
        .and_then(|found| found.name("thing"))
        .map_or("", |thing| {
            thing.as_str().trim_matches([' ', '?', '.', '!', ','])
        });
    if !["", "it", "that", "this", "them", "those", "these"].contains(&named) {
        return true;
    }
    FIRST_SAID_BY_BOT.is_match(text) || FIRST_THING.is_match(text) || TALKED_ABOUT.is_match(text)
}

fn not_a_subject(text: &str) -> bool {
    let low = flat(text);
    if [
        &*NOTHING_SAID,
        &*RECALL_ABOUT,
        &*CHAT_QUESTION,
        &*RECALL_SHAPE,
    ]
    .iter()
    .any(|pattern| pattern.is_match(&low))
    {
        return true;
    }
    ![&*WHERE_WE_WERE, &*STORE_SUMMARY, &*REMIND_ME_RECAP]
        .iter()
        .any(|pattern| pattern.is_match(&low))
        && names_prior_chat(&low)
}

fn stem(word: &str) -> String {
    if word.len() > 4
        && let Some(root) = word.strip_suffix("ies")
    {
        format!("{root}y")
    } else if word.len() > 3 && !word.ends_with("ss") {
        word.strip_suffix("es")
            .filter(|core| core.ends_with(['s', 'x', 'z', 'h']))
            .or_else(|| word.strip_suffix('s'))
            .unwrap_or(word)
            .to_owned()
    } else {
        word.to_owned()
    }
}

fn keywords(text: &str) -> impl Iterator<Item = String> + '_ {
    WORD.find_iter(text).filter_map(|found| {
        let word = found.as_str();
        let word = word.strip_suffix("'s").unwrap_or(word).replace('\'', "");
        (word.len() >= 3
            && !word.bytes().all(|ch| ch.is_ascii_digit())
            && !data::STOPWORDS.contains(&word.as_str()))
        .then(|| stem(&word))
    })
}

fn asks_for_more(clean: &str, turns: &[Exchange]) -> bool {
    if !MORE_ON_THIS.is_match(clean) {
        return false;
    }
    let Some(named) = WHAT_ABOUT.captures(clean).and_then(|found| found.get(1)) else {
        return true;
    };
    let named = named.as_str();
    if PERSON_NAMED.is_match(named) {
        return false;
    }
    let wanted = keywords(named).collect::<HashSet<_>>();
    wanted.is_empty()
        || turns.iter().rev().take(3).any(|turn| {
            let text = format!("{} {}", turn.user, turn.bot)
                .to_lowercase()
                .replace('’', "'");
            keywords(&text).any(|word| wanted.contains(&word))
        })
}

fn only_acknowledges(reply: &str) -> bool {
    let text = reply.split_whitespace().collect::<Vec<_>>().join(" ");
    text.split_whitespace().take(9).count() <= 8 && ACK_ONLY.is_match(&text)
}

fn gave_an_answer(turns: &[Exchange]) -> bool {
    let Some(last) = turns
        .iter()
        .enumerate()
        .rev()
        .find(|(index, turn)| *index == 0 || !only_acknowledges(&turn.bot))
        .map(|(_, turn)| turn)
    else {
        return false;
    };
    let reply = last.bot.trim();
    !data::NO_MORE_OF.contains(&last.module)
        && !reply.is_empty()
        && !reply.ends_with('?')
        && !REACTED_ONLY.is_match(reply)
        && !not_a_subject(&last.user)
}

pub(super) fn more(clean: &str, memory: &Memory) -> Option<&'static [&'static str]> {
    if !asks_for_more(clean, &memory.turns) || !gave_an_answer(&memory.turns) {
        return None;
    }
    (!hedged_recently(memory)).then_some(data::HELP_ALREADY)
}

pub(in crate::bot::compose) fn method_after_answer(
    memory: &Memory,
) -> Option<&'static [&'static str]> {
    memory
        .turns
        .last()
        .is_some_and(|turn| turn.module == "answers")
        .then_some(data::HELP_ALREADY)
}

pub(in crate::bot::compose) fn hedged_recently(memory: &Memory) -> bool {
    memory.turns.iter().rev().take(3).any(|turn| {
        let said = turn.bot.trim();
        let lower = said.to_lowercase();
        data::PUNT.iter().any(|punt| lower.contains(punt))
            || super::super::fallback::unknown_replies()
                .iter()
                .any(|known| known.chars().take(40).eq(said.chars().take(40)))
    })
}
