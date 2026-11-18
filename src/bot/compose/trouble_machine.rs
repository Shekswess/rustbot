//! One question about a named device whose fault has yet to be narrowed down.

use crate::bot::{
    Memory, Reading, compose,
    modules::{greeting, the_detail_the_thread_adds as detail},
};
use std::collections::BTreeSet;

#[path = "trouble_machine_data.rs"]
mod data;
use data::*;

fn contains(vocabulary: &[&str], word: &str) -> bool {
    vocabulary.binary_search(&word).is_ok()
}

fn the_thing(clean: &str) -> Option<String> {
    let words: Vec<_> = super::split(clean).collect();
    let trouble_words: BTreeSet<_> = A_TROUBLE
        .find_iter(clean)
        .flatten()
        .flat_map(|found| super::split(found.as_str()))
        .collect();
    let names = |word: &str| {
        word.chars().count() >= 3
            && word.chars().all(char::is_alphabetic)
            && !contains(FUNCTION, word)
            && !contains(NOT_THE_THING, word)
            && !trouble_words.contains(word)
    };
    let (at, first) = words
        .iter()
        .copied()
        .enumerate()
        .find(|(_, word)| names(word))?;
    let article = at
        .checked_sub(1)
        .map(|before| words[before])
        .filter(|word| contains(DETERMINERS, word))
        .unwrap_or("the");
    Some(match words.get(at + 1) {
        Some(next) if next.chars().count() >= 4 && names(next) && !contains(STOP, next) => {
            format!("{article} {first} {next}")
        }
        _ => format!("{article} {first}"),
    })
}

fn upper_first(reply: &str) -> String {
    let mut letters = reply.chars();
    letters
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(letters)
        .collect()
}

fn options(clean: &str, memory: &Memory) -> Option<Vec<String>> {
    if clean.contains('?')
        || super::split(clean).take(3).count() < 3
        || super::matches(&NOT_A_MACHINE, clean)
        || super::matches(&FIRE_WORD, clean)
        || super::matches(&ITS_THE_WEATHER, clean)
        || !super::matches(&A_TROUBLE, clean)
    {
        return None;
    }
    if memory.turns.last().is_some_and(|turn| {
        detail::heavy::medical_news(&super::split(&turn.user).collect::<Vec<_>>().join(" "))
            || detail::heavy::thread_is_heavy(&turn.user)
    }) || detail::heavy::medical_news(clean)
    {
        return None;
    }
    let thing = the_thing(clean)?;
    let options: Vec<_> = REPLIES
        .iter()
        .map(|reply| upper_first(&reply.replace("%s", &thing)))
        .collect();
    options
        .iter()
        .all(|reply| (3..22).contains(&super::split(reply).count()))
        .then_some(options)
}

fn says_nothing(reply: &str, text: &str) -> bool {
    let reply = reply.replace('’', "'");
    reply.trim_matches(crate::text::is_space).is_empty()
        || super::matches(&STALL, &reply)
        || super::split(&reply).count() <= STOCK_LENGTH
            && (!reply.trim_end_matches(crate::text::is_space).ends_with('?')
                || super::matches(&A_BARE_OFFER, &reply))
            && !super::matches(&ALREADY_NARROWING, &reply)
            && detail::says_nothing_about_the_line(&reply, text)
}

pub(in crate::bot) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let source = if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    };
    let clean = super::flat(&source.replace(['—', '–'], "-").replace(['\'', '’'], ""));
    let options = options(&clean, memory)?;
    let actual = greeting::respond(reading, memory).unwrap_or_default();
    let typed = super::flat(&reading.raw.replace(['\'', '’'], ""));
    if !says_nothing(&actual, &typed) {
        return None;
    }
    let options: Vec<_> = options.iter().map(String::as_str).collect();
    Some(compose::choose_recent(&options, memory, &typed).to_owned())
}
