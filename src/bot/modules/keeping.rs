//! Questions about the length of a chat and whether the user can keep talking.

use crate::bot::{
    Memory, Reading,
    understanding::kinds::{AboutBotKind, a_stop_ask},
};
use regex::Regex;
use sha1::{Digest, Sha1};
use std::sync::LazyLock;

const WHILE: &[&str] = &[
    "As long as you like — I'm not watching a clock, and nothing else is waiting on me. Say when you've had enough and I'll leave it there.",
    "No limit I can see: there's nowhere else I need to be, so take the time you need. If you'd rather pick it up later, that's fine too.",
    "All the time you want — I'm not counting lines or minutes, and I keep the thread while we talk. Say the word when you want to stop.",
    "There's no clock on this at my end. Go at your own pace, and if you'd rather stop and come back to it, that's no trouble.",
];
const COUNT: &[&str] = &[
    "As many as you like - there's no cap on messages here, so send them as they come.",
    "No limit on messages: as many as you want, long or short.",
    "Send as many as you like; there's no count I'm keeping.",
];
const KEEPING: &[&str] = &[
    "Not at all — carry on, I'm following. Say the rest of it.",
    "No, and I'd say so if it got hard to follow. Keep going.",
    "You're not boring me. Take as long as it needs.",
    "Not a bit of it. Nothing else is happening at this end, so go on.",
    "You're fine. I'm reading all of it, and I'll ask if I lose the thread.",
];
const WHILE_TAILS: &[&str] = &["", "Where were we?", "What were you in the middle of?"];

static HOW_MANY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bhow\s+many\b").expect("count wording"));
static OWN_STOP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:should|shall|can|could|may)\s+(?:i|we)\s+(?:stop|quit|give\s+up|pack\s+it\s+in|leave\s+it|jack\s+it\s+in)(?:\s+(?:now|then|it|this|that|doing\s+(?:it|this|that)))?\s*[.!?]*$")
        .expect("bare stop question")
});

fn index(text: &str, tail: bool, size: usize) -> usize {
    // PyBot derives wording from the first six digest bytes of the raw input.
    let mut hash = Sha1::new();
    hash.update(text.as_bytes());
    if tail {
        hash.update(b" tail");
    }
    let digest = hash.finalize();
    let mut bytes = [0; 8];
    bytes[2..].copy_from_slice(&digest[..6]);
    (u64::from_be_bytes(bytes) % size as u64) as usize
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let text = reading.raw;
    let (pool, tails) = match reading.features.about_bot_kind? {
        AboutBotKind::While if HOW_MANY.is_match(text) => (COUNT, &[""][..]),
        AboutBotKind::While => (WHILE, WHILE_TAILS),
        AboutBotKind::Keeping => {
            let said = text
                .split(crate::text::is_space)
                .filter(|word| !word.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase();
            if OWN_STOP.is_match(&said)
                && a_stop_ask(&said)
                && memory
                    .ack_topics
                    .stored_topic(&memory.turns)
                    .next()
                    .is_some()
            {
                return None;
            }
            (KEEPING, &[""][..])
        }
        _ => return None,
    };
    let start = index(text, false, pool.len());
    let previous = memory.turns.last().map_or("", |turn| turn.bot.as_str());
    let reply = pool[start..]
        .iter()
        .chain(&pool[..start])
        .copied()
        .find(|line| !previous.contains(line))
        .unwrap_or(pool[start]);
    let tail = tails[index(text, true, tails.len())];
    Some(if tail.is_empty() {
        reply.to_owned()
    } else {
        format!("{reply} {tail}")
    })
}

#[cfg(test)]
#[test]
fn keeps_the_source_wording_for_duration_count_and_reassurance() {
    let memory = Memory::default();
    for (line, expected) in [
        (
            "How long can we talk for?",
            "No limit I can see: there's nowhere else I need to be, so take the time you need. If you'd rather pick it up later, that's fine too.",
        ),
        (
            "How many messages can I send you?",
            "No limit on messages: as many as you want, long or short.",
        ),
        (
            "Am I boring you?",
            "You're not boring me. Take as long as it needs.",
        ),
    ] {
        assert_eq!(
            respond(&Reading::new(line), &memory).as_deref(),
            Some(expected)
        );
    }
}
