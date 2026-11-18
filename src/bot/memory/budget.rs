//! The explicit budget fact from PyBot's facts.BUDGET and store_replies._fact_reply.
use super::again;
use crate::bot::{Memory, Reading, understanding::kinds::Kind};
use regex::Regex;
use std::sync::LazyLock;

static BUDGET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(?:my|our) budget\b[^.\d]{0,40}?",
        r"((?:under |over |less than |up to )?[£$€]?\s?\d[\d,.]*",
        r"(?:\s?(?:pounds|quid|euros?|dollars?|grand|bucks|k)\b)?)"
    ))
    .unwrap()
});

static ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?:(?:and|so|but|right|ok|okay)[ ,]+)*",
        r"(?:(?:what(?:'s|s| is| was)|(?:do|can) you (?:know|remember)|remind me of) (?:my|our) budget",
        r"|how much can (?:i|we) spend)(?: (?:again|now|then|please))*[?.!]*$"
    )).unwrap()
});

pub(super) fn given(reading: &Reading<'_>) -> Option<String> {
    if reading.kind != Kind::Statement {
        return None;
    }
    let found = BUDGET.captures(reading.raw)?;
    Some(
        found
            .get(1)?
            .as_str()
            .trim_matches([' ', '.', ',', ';', ':'])
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    )
}

pub(super) fn respond(
    reading: &Reading<'_>,
    memory: &Memory,
    budget: Option<&str>,
) -> Option<String> {
    if !ASK.is_match(&reading.clean) {
        return None;
    }
    let amount = budget?;
    let plain = format!("Your budget is {amount}.");
    if !memory.turns.iter().any(|turn| turn.bot == plain) {
        return Some(plain);
    }
    let alternatives = [
        format!("It's {amount} — that's what you told me."),
        format!("You said {amount}."),
        format!("Still {amount}."),
    ];
    Some(again::vary(
        &alternatives.each_ref().map(String::as_str),
        memory,
    ))
}
