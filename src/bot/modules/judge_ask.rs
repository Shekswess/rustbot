//! Fallback replies when asked whether the bot judges other people.

use super::greeting;
use crate::bot::{Memory, Reading, compose};
use fancy_regex::Regex as Lookaround;
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

static PEOPLE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:will|would|do|did|are|r|can|could)\s+(?:you|u)\s+",
        r"(?:just\s+|really\s+|actually\s+|even\s+|ever\s+|not\s+)?judg\w*\s+",
        r"(?:(?:a|the|other)\s+)?",
        r"(?:people|person|persons|someone|somebody|anyone|anybody|folk|folks|others|them|him|her)\b",
        r"|\b(?:dont|do not|never)\s+judg\w*\s+",
        r"(?:(?:a|the|other)\s+)?(?:people|anyone|anybody|someone|somebody|person|folk|folks)\b"
    ))
    .expect("judgment directed at the bot")
});
static NOT_PEOPLE: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(concat!(
        r"\bjudg\w*\s+(?:the|this|that|it|my|his|its)\b",
        r"|\bjudg\w*\s+(?:a|an)\s+(?!person\b|people\b|human\b|child\b|kid\b|man\b|woman\b",
        r"|guy\b|girl\b|boy\b|teen\w*\b)\w+"
    ))
    .expect("judgment directed at an object")
});
static OTHER_JUDGMENT: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"\bjudg(?:ing|ed)\s+(?:by|on|from)\b",
        concat!(
            r"\b(?:everyone|everybody|people|they|them|someone|somebody|nobody|no one|parents|friends|",
            r"family|colleagues|strangers|relatives|neighbours)\b(?:\s+\w+){0,3}\s+judg\w*",
            r"|\b(?:my|his|her|their|our|your)\s+(?:\w+\s+){0,2}judg\w*",
            r"|\b(?:she|he|they|it|shell|hell|theyll|theyd|shed|shes|hes)\b(?:\s+\w+){0,3}\s+judg\w*",
            r"|\b(?:the|a|my|his|her|our|their|your)\s+(?:\w+\s+){0,2}to\s+judg\w*\s+(?:me|us)\b"
        ),
    ])
    .expect("judgment by a standard or another person")
});

const REPLIES: [&str; 3] = [
    "I try not to. Nobody gets a verdict off me - what people tell me stays here. What's on your mind?",
    "No. I've no interest in scoring people - say whatever you came to say.",
    "I don't, no. Whatever you tell me I take as it comes, so where are we starting?",
];

const EMPTY_HEADS: &[&str] = &[
    "got it",
    "noted:",
    "thanks for telling me",
    "i'm listening",
    "i am listening",
    "that's your call",
    "that sounds like a lot",
    "sorry, i don't know",
    "i don't know enough",
];

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !reading.line.clean.contains("judg") {
        return None;
    }
    let clean = reading
        .raw
        .to_lowercase()
        .split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if !PEOPLE_ASK.is_match(&clean)
        || NOT_PEOPLE.is_match(&clean).unwrap_or(true)
        || OTHER_JUDGMENT.is_match(&clean)
    {
        return None;
    }
    let composing = [
        greeting::respond(reading, memory),
        compose::fallback::respond(reading, memory),
    ];
    if composing.into_iter().flatten().any(|reply| {
        let lower = reply.to_lowercase();
        !reply.is_empty()
            && !compose::routes::hedge(&reply)
            && !EMPTY_HEADS.iter().any(|head| lower.starts_with(head))
    }) {
        return None;
    }
    let start = clean
        .chars()
        .fold(0, |sum, ch| (sum + ch as usize) % REPLIES.len());
    let last = memory.turns.last().map_or("", |turn| turn.bot.as_str());
    let reply = REPLIES
        .iter()
        .cycle()
        .skip(start)
        .take(REPLIES.len())
        .find(|reply| !last.contains(&reply[..24]))
        .unwrap_or(&REPLIES[start]);
    Some((*reply).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn follows_source_pool_order_and_avoids_the_last_reply() {
        let mut bot = Bot::new(Some("0")).unwrap();
        for index in [1, 2, 1] {
            let turn = bot.turn("Do you judge people?");
            assert_eq!(turn.module, Some("judge_ask"));
            assert_eq!(turn.reply, REPLIES[index]);
        }
        for (prompt, index) in [
            ("Do you judge a person by their past?", 0),
            ("Do you\tjudge\u{a0}people? 🦊", 1),
            ("are you judging them?", 2),
            ("can u ever judge people?", 0),
        ] {
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            let reply = turn
                .candidates
                .iter()
                .find(|part| part.module == "judge_ask");
            assert_eq!(
                reply.map(|part| (part.priority, part.reply.as_str())),
                Some((-500.0, REPLIES[index])),
                "{prompt}"
            );
        }
    }

    #[test]
    fn defers_to_the_composer_and_to_other_judgment_subjects() {
        for prompt in [
            "Would you judge someone for that?",
            "Don't judge me.",
            "Do you judge a cake by looks and do you judge people?",
            "Are people judging me?",
            "Do you judge people when their parents judge them?",
        ] {
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            assert!(
                turn.candidates
                    .iter()
                    .all(|part| part.module != "judge_ask"),
                "{prompt}"
            );
        }
    }
}
