//! Source: bot/compose/still.py and bot/modules/nudge.py.

use crate::bot::{Memory, Reading, compose, memory};

const LEADING: &[&str] = &[
    "ok", "okay", "right", "well", "so", "yes", "yeah", "and", "but", "then", "alright", "cool",
    "great", "thanks", "ta", "ah", "oh",
];
const SYMPATHY_OPEN: &[&str] = &[
    "sorry to hear",
    "so sorry",
    "i'm sorry",
    "im sorry",
    "that sounds",
    "awful",
    "that's tough",
    "thats tough",
    "that's such",
    "thats such",
    "how awful",
];
const STILL_END: &[&str] = &[
    "That's the lot of it from me on that one. Say if you want to go back over any part of it.",
    "That's everything I have on it, and it is all above. Which bit do you want to unpick?",
    "That's the whole of it as far as I know. Is there a part of it you are unsure about?",
    "Nothing more from me on that one - it is all said. Anything in it worth going over?",
    "That's as far as I can take it. Say the word and we will go back over a piece of it.",
];
const STILL_AFTER_A_LOSS: &[&str] = &[
    "No rush on any of it. If you want to say more about them, I am here for that.",
    "Take your time - nothing here needs finishing. Say more about them whenever you like.",
    "That's all from me, and there is no hurry. I am still here if you want to talk about them.",
];
const STILL_AFTER_A_HARD_DAY: &[&str] = &[
    "No rush with it. I am still here if you want to say more about how things are.",
    "Nothing to finish here - take your time. Say more whenever you want to.",
    "That's all from me for now, and no hurry at all. I am here if you want to keep going.",
];

fn flat(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_a_nudge(clean: &str) -> bool {
    let parts: Vec<_> = clean
        .split_whitespace()
        .take(5)
        .map(|word| word.trim_matches(['.', ',', '!', '?', ';', ':', '-']))
        .collect();
    if parts.is_empty() || parts.len() > 4 {
        return false;
    }
    let mut parts = parts.as_slice();
    let matches = |parts: &[&str]| {
        let words: Vec<_> = parts
            .iter()
            .copied()
            .filter(|part| !part.is_empty())
            .collect();
        matches!(
            words.as_slice(),
            ["and" | "then"] | ["and", "then"] | ["the", "rest"] | ["and", "the", "rest"]
        )
    };
    if matches(parts) {
        return true;
    }
    while parts.len() > 1 && LEADING.contains(&parts[0]) {
        parts = &parts[1..];
    }
    matches(parts)
}

fn unreadable(module: &str) -> bool {
    matches!(
        module,
        "safety" | "urgent" | "loss" | "grief" | "a_count_of_packets"
    )
}

fn sympathy(reply: &str) -> bool {
    let reply = flat(reply);
    SYMPATHY_OPEN.iter().any(|line| reply.starts_with(line))
}

fn gave_an_answer(memory: &Memory) -> bool {
    for (index, turn) in memory.turns.iter().enumerate().rev() {
        if unreadable(turn.module) {
            return false;
        }
        let reply = turn.bot.trim();
        if reply.is_empty() || STILL_END.contains(&reply) {
            continue;
        }
        if reply.ends_with('?') || sympathy(reply) {
            return false;
        }
        if turn.module == "greeting" {
            if !memory.turns[..index].iter().any(|prior| {
                prior.module != "greeting"
                    && !unreadable(prior.module)
                    && prior.bot.split_whitespace().take(5).count() >= 5
            }) {
                return false;
            }
            continue;
        }
        let mut sentences = memory::sentences(reply);
        return (turn.module == "memory_store"
            && sentences
                .next()
                .is_some_and(|first| first.split_whitespace().take(6).count() >= 6))
            || memory::sentences(reply).nth(1).is_some()
            || memory::ordered_steps(reply).next().is_some();
    }
    false
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = flat(reading.raw);
    if !is_a_nudge(&clean) || super::lead_in_probe::hedged_recently(memory) {
        return None;
    }
    let last = memory.turns.last()?;
    let pool = match last.module {
        "safety" | "urgent" | "a_count_of_packets" => return None,
        "loss" | "grief" => STILL_AFTER_A_LOSS,
        _ if sympathy(&last.bot) => STILL_AFTER_A_HARD_DAY,
        _ if gave_an_answer(memory)
            && memory::chat_answer_step(reading, memory).is_none()
            && memory::carry_chat_reply(reading, memory).is_none() =>
        {
            STILL_END
        }
        _ => return None,
    };
    Some(compose::pick(pool, memory, &clean).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    fn previous(module: &'static str, bot: &str) -> Memory {
        Memory {
            turns: vec![Exchange {
                user: "Can you help?".into(),
                bot: bot.into(),
                module,
            }]
            .into(),
            ..Memory::default()
        }
    }

    #[test]
    fn short_nudges_follow_finished_answers_or_condolences() {
        let memory = previous(
            "answers",
            "There are several choices. A good example is a walk in the park.",
        );
        assert!(respond(&Reading::new("okay, and then?"), &memory).is_some());
        for line in [
            "what next",
            "and the radiator",
            "and the window in the back",
        ] {
            assert!(respond(&Reading::new(line), &memory).is_none());
        }
        let loss = previous("grief", "I'm very sorry to hear that.");
        assert!(
            STILL_AFTER_A_LOSS.contains(&respond(&Reading::new("and"), &loss).unwrap().as_str())
        );
        let urgent = previous(
            "urgent",
            "Call 999 now and follow the operator's instructions.",
        );
        assert!(respond(&Reading::new("and"), &urgent).is_none());
    }
}
