//! Replies to statements about something the speaker cannot follow.
use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::sync::LazyLock;
const WITH_A_THING_LINES: &[&str] = &[
    "Which bit of {thing} stops making sense? Say the first line that does and we'll take it from there.",
    "Which part of {thing} loses you - the start, or somewhere in the middle?",
    "What is it about {thing} that isn't going in - the words, or the order of it?",
    "Let's take {thing} one line at a time. Which line is the first one that doesn't land?",
];
const WITHOUT_A_THING_LINES: &[&str] = &[
    "What is the thing itself - a letter, a form, a set of instructions? Even the name helps.",
    "Say the last bit that made sense and I'll take it from there.",
    "Which part is clear and which part is foggy? Start where it stops making sense.",
];
static NOTHING_SAID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:i'?m listening|say more|not sure about that one|don'?t know enough|beyond what i know|rather not guess|everything i have on that one|not much (?:i can add|to go on)|which bit do you|tell me more|i'?m here if you want|anything you'?d like a hand with|thanks for telling me)\b|\u2014\s*noted\.|\s-\s*noted\.|^noted[:.]|,\s*good to know\.?\s*$"#).unwrap()
});
static OUT_OF_THEIR_HANDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)out of your hands|when do you find out"#).unwrap());
static MISREAD_AS_GOOD_NEWS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:good to hear|glad to hear|best bit|nice one|sounds alright|sounds good|lovely|how lovely|brilliant)\b"#).unwrap()
});

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let confusion = reading.confusion.as_ref()?;
    let (state, index) = {
        let rng = memory.rng.borrow();
        (*rng.0.get_state(), rng.0.get_index())
    };
    let fallback = super::greeting::respond(reading, memory).unwrap_or_default();
    let mut rng = memory.rng.borrow_mut();
    rng.0.set_state(&state);
    rng.0.set_index(index);
    if !(fallback.is_empty()
        || NOTHING_SAID.is_match(&fallback)
        || OUT_OF_THEIR_HANDS.is_match(&fallback)
        || MISREAD_AS_GOOD_NEWS.is_match(&fallback))
    {
        return None;
    }
    let clean: String = reading.clean.chars().filter(|&c| c != '\'').collect();
    let Some(thing) = &confusion.thing else {
        return Some(compose::choose_recent(WITHOUT_A_THING_LINES, memory, &clean).into());
    };
    let thing = thing
        .split_whitespace()
        .map(|w| match w {
            "my" | "our" => "your",
            "me" | "us" => "you",
            "mine" | "ours" => "yours",
            word => word,
        })
        .collect::<Vec<_>>()
        .join(" ");
    let pool = WITH_A_THING_LINES
        .iter()
        .map(|line| line.replace("{thing}", &thing))
        .collect::<Vec<_>>();
    Some(
        compose::choose_recent(
            &pool.iter().map(String::as_str).collect::<Vec<_>>(),
            memory,
            &clean,
        )
        .into(),
    )
}
