//! Explicit facts, correction and recall for the migrated personal-name rules.
pub(crate) mod a_year_of_birth;
pub(crate) mod aboutthem;
pub(crate) mod ack;
mod again;
pub(crate) use again::starting_out_names_something;
mod age_words;
pub(crate) mod already;
mod answer_figures;
pub(crate) mod asleep;
pub(crate) mod attribute_said_again;
mod bare_name_words;
pub(crate) mod bare_subject;
mod budget;
pub(crate) mod callback;
mod carried_on;
pub(super) mod frequency;
pub(in crate::bot) use carried_on::{later_part, ordered_steps, sentences};
pub(crate) use carried_on::{next_step_answer, repeated_step_answer};
pub(crate) mod decided;
pub(crate) mod done_topic;
pub(crate) mod earlier;
mod figures;
mod filler;
mod fix_asked;
mod food;
pub(crate) mod groupages;
pub(crate) mod history;
pub(crate) mod household;
mod joined_followup;
pub(crate) mod kidcount;
pub(crate) mod lineback;
mod lineback_kinds;
pub(crate) mod meant;
pub(crate) mod measure;
pub(crate) mod measurement;
pub(crate) mod measurement_subject;
mod name_vocabulary;
pub(crate) mod names;
pub(crate) mod occupations;
mod personal;
mod places;
pub(crate) mod plan;
pub(crate) mod possession_parts;
pub(crate) mod possessions;
pub(crate) mod quote_back;
pub(super) mod repair;
pub(super) mod return_advice;
pub(crate) mod shorter;
pub(super) mod stand_figure;
pub(crate) mod stated_plans;
mod summary;
mod surname;
pub(super) mod taking_again;
pub(super) mod thread;
pub(crate) mod travel_trip;
mod when;
pub(crate) mod when_length;
pub(crate) mod will_it_work;
pub(crate) mod workplace;
mod workplace_data;
pub(crate) mod years_in;
use super::{Memory, Reading, compose};
pub(crate) use food::{FoodAck, food_ack};
pub(crate) use personal::Personal;
use regex::Regex;
use std::sync::LazyLock;

static INTRO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?i)^(?:(?:hi|hey|hiya|hello)[, !]+)?(?:(?:actually|sorry|no)[, ]+)?(?:my name is|my name's|my names|call me|i'm called|i am called|i'm named|i am named|i'm|im|i am)\s+(?:(?:actually|honestly|truthfully|officially|properly|tbh)\s+)?([\p{L}][\p{L}'’-]{0,39})(?:[\s]*[.!]*|,?\s+and\b.*)$"
).unwrap()
});
static ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"^(?:(?:and|so|okay|ok)\s+)?(?:what(?:'s|s| is| was) my name|what did i (?:say|tell you) my name was|do you (?:know|remember) my name|who am i|(?:remind me of )?my name again|what (?:do|did) you call me)(?:\s+again)?[?.!]*$"
).unwrap()
});
static FORGET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:please\s+)?(?:forget|drop|remove|delete) my name(?:[ ,]+please)?[.!]*$")
        .unwrap()
});

pub(in crate::bot) fn asks_for_name(reading: &Reading<'_>) -> bool {
    ASK.is_match(&reading.clean)
}

pub(in crate::bot) fn asks_to_forget_name(line: &str) -> bool {
    FORGET.is_match(line)
}

fn valid_name(name: &str, explicit: bool) -> bool {
    let low = name.to_lowercase();
    let excluded = name_vocabulary::NOT_NAMES
        .binary_search(&low.as_str())
        .is_ok()
        || name_vocabulary::NOT_NAME_ANSWERS
            .binary_search(&low.as_str())
            .is_ok();
    !excluded
        && (explicit
            || name.chars().next().is_some_and(char::is_uppercase)
            || name_vocabulary::COMMON.binary_search(&low.as_str()).is_ok())
}

static GO_BY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|[,;.!]\s*|\b(?:but|and|though|although)\s+)(?:i|we)\s+go(?:es)?\s+by\s+([a-z][a-z'-]{1,19})[.!?]*$").unwrap()
});

pub(super) fn observe(reading: &Reading<'_>, memory: &mut Memory) {
    memory.group_ages.observe(reading.raw);
    memory.household.observe(reading.raw, &memory.turns);
    memory
        .family_counts
        .observe(reading, memory.turns.last().map(|turn| turn.user.as_str()));
    memory.personal.observe(reading, memory.turns.len());
    memory.possessions.observe(reading.raw, &reading.line.plain);
    memory.decided.observe(reading.raw, &memory.turns);
    workplace::observe(&mut memory.occupations, reading);
    memory.stated_plans.observe(reading.raw);
    if let Some(plan) = travel_trip::booking_to_file(reading.raw) {
        memory.stated_plans.file_booked_trip(plan, reading.raw);
    }
    memory.introduced_now = false;
    memory.previous_name = None;
    if asks_to_forget_name(&reading.clean) {
        return;
    }
    let named = GO_BY
        .captures(reading.raw.trim())
        .or_else(|| INTRO.captures(reading.raw.trim()))
        .and_then(|m| m.get(1))
        .map(|m| m.as_str());
    let answering = memory.turns.last().is_some_and(|turn| {
        turn.module == "name_taken_back"
            || (memory.name.is_none() && surname::awaits_given_name(turn))
    });
    let reply_name = answering.then(|| names::answer(reading.raw)).flatten();
    let Some(name) = reply_name.as_deref().or(named) else {
        return;
    };
    let explicit = ["my name", "call me", "called", "named", "go by"]
        .iter()
        .any(|cue| reading.clean.contains(cue));
    if !valid_name(name, explicit || answering) {
        return;
    }
    let mut chars = name.chars();
    let name: String = chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect();
    memory.introduced_now = memory.name.as_deref() != Some(&name);
    if memory.introduced_now {
        memory.denied_name = None;
        memory.previous_name = memory.name.replace(name);
    }
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(reply) = &memory.trip_date_reply {
        return Some(reply.clone());
    }
    if let Some(old) = &memory.previous_name
        && let Some(name) = &memory.name
    {
        return Some(format!("{name}, not {old} — got it."));
    }
    if asks_to_forget_name(&reading.clean) {
        return name_reply(memory);
    }
    if let Some(age) = a_year_of_birth::recalled_age(reading, memory) {
        return Some(age);
    }
    if !asks_for_name(reading) {
        return kidcount::store_reply(reading, memory)
            .or_else(|| household::count_reply(reading, memory))
            .or_else(|| groupages::store_reply(reading, memory))
            .or_else(|| memory.personal.respond(reading, memory))
            .or_else(|| figures::respond(reading, memory))
            .or_else(|| possessions::respond(reading, memory))
            .or_else(|| decided::respond(reading, memory))
            .or_else(|| answer_figures::respond(reading, memory))
            .or_else(|| possession_parts::respond(reading, memory));
    }
    name_reply(memory)
}

pub(super) fn favourite(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    memory.personal.respond_favourite(reading, memory)
}

pub(in crate::bot) fn asks_for_repeat(text: &str) -> bool {
    again::asks_for_repeat(text)
}

pub(crate) fn unframed_reply(reply: &str) -> Option<&str> {
    again::unframed(reply)
}

pub(crate) fn frame_reply(reply: &str, text: &str, memory: &Memory) -> Option<String> {
    again::instead(reply, text, memory)
}

pub(crate) fn frame_answer_card(reply: &str, text: &str, memory: &Memory) -> Option<String> {
    again::frame_answer_card(reply, text, memory)
}

pub(crate) fn carry_chat_reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    carried_on::respond(reading, memory)
}

pub(crate) fn chat_answer_step(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    carried_on::respond_step(reading, memory)
}

pub(crate) fn chat_joined_followup(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    joined_followup::reply(reading, memory)
}

pub(crate) fn first_step(reply: &str, text: &str, memory: &Memory) -> Option<String> {
    carried_on::first_step(reply, text, memory)
}

pub(crate) fn record_carried_chat_reply(
    reading: &Reading<'_>,
    chosen_reply: &str,
    memory: &mut Memory,
) {
    carried_on::record(reading, chosen_reply, memory);
}

fn name_reply(memory: &Memory) -> Option<String> {
    Some(match &memory.name {
        None => compose::pick(
            &[
                "You haven't told me your name yet.",
                "I don't know it yet — you haven't said.",
            ],
            memory,
            "",
        )
        .into(),
        Some(name) => {
            let replies = [
                format!("You're {name}."),
                format!("{name} — you told me."),
                format!("Your name's {name}."),
            ];
            compose::pick(&replies.each_ref().map(String::as_str), memory, "").into()
        }
    })
}

pub(crate) fn recognized_name(name: &str) -> bool {
    name_vocabulary::COMMON
        .binary_search(&name.to_lowercase().as_str())
        .is_ok()
}
