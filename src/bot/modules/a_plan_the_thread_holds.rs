//! Put the person's stated day-out plans in the order the source supplies.

use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

#[path = "a_plan_the_thread_holds_data.rs"]
mod data;
#[cfg(test)]
#[path = "a_plan_the_thread_holds_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    PLAN_PLACE,
    PLAN_MEAL,
    PLAN_BED,
    PLAN_BOOKED,
    PLAN_TOGETHER,
    PLAN_DOORS,
    PLAN_QUESTION,
);
static PLAN_ASK: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(data::PLAN_ASK).expect("plan questions"));
static PLAN_EMPTY: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| data::PLAN_EMPTY.split_ascii_whitespace().collect());

fn clean(text: &str) -> String {
    text.to_lowercase()
        .split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn statement(line: &str) -> Option<&str> {
    let line = line.trim_matches([' ', '.', '!', ',']);
    if line.is_empty() || line.contains('?') || PLAN_QUESTION.is_match(line) {
        return None;
    }
    let words: Vec<_> = line.split_ascii_whitespace().take(15).collect();
    ((2..=14).contains(&words.len()) && words.iter().any(|word| !PLAN_EMPTY.contains(word)))
        .then_some(line)
}

fn pool(ask: &str, memory: &Memory) -> Option<[String; 3]> {
    if memory.turns.len() < 3 || !PLAN_ASK.is_match(ask).unwrap_or(false) {
        return None;
    }
    let user_lines: Vec<_> = memory.turns.iter().map(|turn| clean(&turn.user)).collect();
    let mut said = user_lines
        .iter()
        .filter(|line| line.as_str() != ask)
        .filter_map(|line| statement(line));
    let thing = said.next()?;
    let other_statements: Vec<_> = said.collect();
    if other_statements.is_empty() {
        return None;
    }
    let thread = std::iter::once(thing)
        .chain(other_statements.iter().copied())
        .collect::<Vec<_>>()
        .join(" ");
    let place = PLAN_PLACE.is_match(&thread);
    let meal = PLAN_MEAL.is_match(&thread);
    let bed = PLAN_BED.is_match(&thread);
    let together = PLAN_TOGETHER.is_match(&thread);
    if !(place || bed)
        || [place, meal, bed, together]
            .into_iter()
            .filter(|part| *part)
            .count()
            < 2
    {
        return None;
    }
    let mut settled = Vec::with_capacity(2);
    for line in other_statements.into_iter().rev() {
        if !settled.contains(&line) {
            settled.push(line);
        }
        if settled.len() == 2 {
            break;
        }
    }
    let named = match settled.as_slice() {
        [last, previous] => format!("\"{thing}\", \"{previous}\" and \"{last}\""),
        [last] => format!("\"{thing}\" and \"{last}\""),
        _ => unreachable!("a plan has something settled"),
    };
    let (order, book) = match (meal, bed) {
        (true, true) => (
            "the hour that cannot move, then the meal, then the bed",
            "the table and the room",
        ),
        (true, false) => ("the hour that cannot move, then the meal", "the table"),
        (false, true) => ("the hour that cannot move, then the bed", "the room"),
        (false, false) => ("the hour that cannot move", "the two bookings"),
    };
    let about = format!("{} {thing}", settled.join(" "));
    let left = if PLAN_DOORS.is_match(&about) {
        "the doors time - the venue's own page has that"
    } else {
        "the hour you have to be there - the booking page has that"
    };
    let (tail, close) = if PLAN_BOOKED.is_match(&thread) {
        (
            "The one thing that runs out is done already, so the hours are all that is left."
                .to_string(),
            "Fix the hours round it and the day runs itself.".to_string(),
        )
    } else {
        (
            format!("Book {book} tonight: the rest is only times."),
            format!("Get {book} settled and the day runs itself."),
        )
    };
    Some([
        format!("You have told me {named}, so the order is {order}. {tail}"),
        format!(
            "I have it all: {named}. The order is {order}, and the one thing left to find is {left}."
        ),
        format!(
            "Nothing here needs looking up - you have told me {named}, and that is a day out with the order already in it: {order}. {close}"
        ),
    ])
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean_line = if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    };
    let ask = clean(&clean_line.replace(['—', '–'], "-")).replace(['’', '\''], "");
    let pool = pool(&ask, memory)?;
    let said = compose::fallback::respond(reading, memory)?;
    if !compose::routes::hedge(&said) {
        return None;
    }
    let seed = clean(reading.raw).replace(['’', '\''], "");
    let options = pool.each_ref().map(String::as_str);
    Some(compose::choose_recent(&options, memory, &seed).to_string())
}
