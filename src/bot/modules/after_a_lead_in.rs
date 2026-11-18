//! Re-read a question whose opening words kept the fallback from recognizing it.

use crate::bot::{Memory, Reading, compose, transcript::Transcript};
use crate::text::is_space;
use regex::Regex;
use std::{collections::HashSet, mem, sync::LazyLock};

#[path = "after_a_lead_in_data.rs"]
mod data;
#[cfg(test)]
#[path = "after_a_lead_in_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(LEAD_IN, A_QUESTION, THE_FLOOR_OR_A_NOD, NAMED_HELP_NOW);
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9&']+").unwrap());
static SENTENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[.!?]\s+").unwrap());
static DASH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+(?:—|–|-)\s+").unwrap());

fn collapsed(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn question(line: &str) -> Option<&str> {
    let lead = LEAD_IN.find(line)?;
    let rest = line[lead.end()..].trim_matches(is_space);
    (A_QUESTION.is_match(rest) && rest.split_whitespace().nth(2).is_some()).then_some(rest)
}

fn is_receipt(reply: &str) -> bool {
    fn framed(line: &str) -> bool {
        data::RECEIPT_FRAMES.iter().any(|&(front, back)| {
            line.strip_prefix(front)
                .and_then(|middle| middle.strip_suffix(back))
                .is_some_and(|middle| {
                    !middle
                        .trim_matches([' ', '—', '-', '.', ':', ',', ';'])
                        .is_empty()
                })
        })
    }
    let said = collapsed(&reply.to_lowercase());
    let comma = DASH.replace_all(&said, ", ");
    [said.as_str(), comma.as_ref()].into_iter().any(|line| {
        framed(line)
            || SENTENCE
                .find(line)
                .is_some_and(|end| framed(&line[..end.start() + 1]))
    })
}

fn floor_or_nod(reply: &str) -> bool {
    let text = reply.trim_matches(is_space);
    text.is_empty()
        || compose::routes::hedge(text)
        || is_receipt(text)
        || THE_FLOOR_OR_A_NOD.is_match(text)
}

fn echo(reply: &str, line: &str) -> bool {
    let line = line.to_lowercase();
    let asked = WORD
        .find_iter(&line)
        .map(|word| word.as_str())
        .collect::<HashSet<_>>();
    WORD.find_iter(&reply.to_lowercase())
        .map(|word| word.as_str())
        .all(|word| data::NO_NEWS.contains(&word) || asked.contains(word))
}

fn reply_for(rest: &str, memory: &Memory) -> Option<String> {
    let read = Reading::understand(
        rest,
        memory.turns.iter().map(|turn| turn.user.as_str()),
        memory.intent_wants,
        &memory.prior_readings,
    );
    probe(&read, memory)
}

fn probe(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let reply = compose::fallback::respond(reading, memory);
    if reply
        .as_deref()
        .is_some_and(|said| NAMED_HELP_NOW.is_match(said))
    {
        return reply;
    }
    compose::lead_in_probe::specific(reading, memory).or(reply)
}

struct WithoutTurns<'a> {
    memory: &'a mut Memory,
    earlier: Transcript,
}

impl<'a> WithoutTurns<'a> {
    fn new(memory: &'a mut Memory) -> Self {
        let earlier = mem::take(&mut memory.turns);
        Self { memory, earlier }
    }
}

impl Drop for WithoutTurns<'_> {
    fn drop(&mut self) {
        self.memory.turns = mem::take(&mut self.earlier);
    }
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &mut Memory) -> Option<String> {
    let line = collapsed(reading.raw);
    let rest = question(&line)?;
    let whole = probe(reading, memory).unwrap_or_default();
    if !floor_or_nod(&whole) && !echo(&whole, &line) {
        return None;
    }
    let mut without = reply_for(rest, memory);
    if without
        .as_deref()
        .is_none_or(|reply| reply == whole || floor_or_nod(reply))
    {
        let alone = WithoutTurns::new(memory);
        if let Some(fresh) = reply_for(rest, alone.memory)
            && fresh != whole
            && !floor_or_nod(&fresh)
        {
            without = Some(fresh);
        }
    }
    without.filter(|said| {
        *said != whole
            && !NAMED_HELP_NOW.is_match(&whole)
            && !NAMED_HELP_NOW.is_match(said)
            && !floor_or_nod(said)
    })
}
