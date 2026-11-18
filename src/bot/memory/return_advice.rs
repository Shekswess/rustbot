//! Bring back advice the user actually saw when they explicitly return to its topic.

use super::{again, done_topic, measurement};
use crate::bot::{Exchange, Memory, Reading};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

const LEADS: &[&str] = &[
    "Back to {thing}: {advice}",
    "Back on {thing}: {advice}",
    "On {thing}, then: {advice}",
];

static RECEIPT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)^(?:got it|noted|ok|okay|sure|right|you said|you've told me|you told me|that's right|",
        r"that is right|makes sense|fair enough|understood|",
        r"i'm not sure|i am not sure|that's beyond|that is beyond|no number for that|",
        r"i can't|i cannot|i'd rather not|i would rather not|searching for it|a quick search)\b"
    )).expect("valid receipt openings")
});

fn opening(text: &str, word_count: usize) -> String {
    text.split_whitespace()
        .take(word_count)
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn advice(reply: &str) -> Option<String> {
    let joined = reply.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut sentences = measurement::sentences(&joined).into_iter();
    let first = sentences.next()?.trim();
    let selected = if first.split_whitespace().count() < 5 {
        sentences.next().map_or_else(
            || first.to_owned(),
            |second| format!("{first} {}", second.trim()),
        )
    } else {
        first.to_owned()
    };
    (selected.split_whitespace().count() >= 6
        && !selected.ends_with('?')
        && !RECEIPT.is_match(&selected))
    .then_some(selected)
}

fn matching_keys(text: &str, keys: &HashSet<String>) -> usize {
    done_topic::keywords(text)
        .filter(|word| keys.contains(word))
        .count()
}

fn strongest<'a>(
    turns: impl Iterator<Item = &'a Exchange>,
    keys: &HashSet<String>,
) -> Vec<&'a str> {
    let mut best = 0;
    let mut answers = Vec::new();
    for turn in turns {
        if turn.bot.trim().is_empty()
            || matches!(turn.module, "safety" | "urgent" | "a_count_of_packets")
        {
            continue;
        }
        let score = matching_keys(&turn.user, keys);
        if score > best {
            best = score;
            answers.clear();
        }
        if score > 0 && score == best {
            answers.push(turn.bot.as_str());
        }
    }
    answers
}

fn topic<'a>(wanted: &HashSet<String>, memory: &'a Memory) -> Option<&'a str> {
    memory
        .doneness_topics
        .texts()
        .into_iter()
        .chain(memory.turns.iter().rev().map(|turn| turn.user.as_str()))
        .find(|text| matching_keys(text, wanted) == wanted.len())
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let (asked, asks) = again::return_target(reading.raw)?;
    if asks {
        return None;
    }
    let wanted: HashSet<_> = done_topic::keywords(&asked).collect();
    if wanted.is_empty() {
        return None;
    }
    let subject = topic(&wanted, memory)?;
    let keys: HashSet<_> = done_topic::keywords(subject).collect();

    let shown: Vec<_> = memory
        .entries
        .oldest_first()
        .filter_map(|reply| {
            let lead = opening(reply, 8);
            if lead.is_empty() {
                return None;
            }
            memory
                .turns
                .iter()
                .rev()
                .take(4)
                .find(|turn| opening(&turn.bot, 8).starts_with(&lead))
        })
        .collect();
    let candidates = strongest(shown.into_iter(), &keys)
        .into_iter()
        .chain(strongest(memory.turns.iter(), &keys));
    let selected = candidates.filter_map(advice).find(|candidate| {
        let lead = opening(candidate, 6);
        !memory
            .turns
            .iter()
            .rev()
            .take(2)
            .any(|turn| opening(&turn.bot, 6) == lead)
    })?;
    Some(
        LEADS[memory.turns.len() % LEADS.len()]
            .replace("{thing}", &asked)
            .replace("{advice}", &selected),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    fn turn(user: &str, bot: &str, module: &'static str) -> Exchange {
        Exchange {
            user: user.to_owned(),
            bot: bot.to_owned(),
            module,
        }
    }

    #[test]
    fn uses_an_older_seen_answer_and_leaves_a_new_question_alone() {
        let memory = Memory {
            turns: vec![
                turn(
                    "My boiler makes a banging noise",
                    "Banging when the heating starts can mean air or a circulation problem. Check the pressure gauge.",
                    "answers",
                ),
                turn(
                    "Can you grow a fig in a pot?",
                    "Figs can fruit in a pot in a sunny spot.",
                    "answers",
                ),
                turn(
                    "Which books did you suggest?",
                    "You said you liked mysteries.",
                    "memory_store",
                ),
            ].into(),
            ..Memory::default()
        };
        let reply = respond(&Reading::new("Back to the noise"), &memory).unwrap();
        assert_eq!(
            reply,
            "Back to the noise: Banging when the heating starts can mean air or a circulation problem."
        );
        assert!(
            respond(
                &Reading::new("Back to the noise, how do I fix it?"),
                &memory
            )
            .is_none()
        );
        assert!(respond(&Reading::new("Back to the laptop"), &memory).is_none());
    }

    #[test]
    fn does_not_quote_receipts_recent_repeats_or_emergency_guidance() {
        let mut memory = Memory {
            turns: vec![
                turn(
                    "The blanket is a wool one",
                    "Got it, you have a wool blanket.",
                    "memory_store",
                ),
                turn(
                    "I took tablets",
                    "Call emergency services right now and keep the packet.",
                    "safety",
                ),
                turn(
                    "The oven is heating unevenly",
                    "Rotate the tray halfway through baking so both sides brown.",
                    "answers",
                ),
                turn(
                    "Another thought",
                    "Tell me what you would like to work on next.",
                    "chat",
                ),
            ]
            .into(),
            ..Memory::default()
        };
        assert!(respond(&Reading::new("Back to the blanket"), &memory).is_none());
        assert!(respond(&Reading::new("Back to the tablets"), &memory).is_none());
        assert!(respond(&Reading::new("Back to the oven"), &memory).is_none());
        memory.turns.push(turn(
            "What colour is cobalt?",
            "Cobalt blue is a deep blue pigment.",
            "answers",
        ));
        assert!(respond(&Reading::new("Back to the oven"), &memory).is_some());
    }

    #[test]
    fn wins_a_real_bare_return_in_the_api() {
        let mut bot = Bot::new(Some("3")).unwrap();
        let original = bot.turn("my boiler is making a noise");
        bot.turn("how much is a new one");
        bot.turn("i rent so who pays");
        let answer = bot.turn("back to the noise");
        assert_eq!(
            answer.module,
            Some("a_return_brings_back_the_advice"),
            "{:?}",
            answer.candidates
        );
        assert_eq!(
            answer.reply,
            format!("Back to the noise: {}", advice(&original.reply).unwrap())
        );
    }
}
