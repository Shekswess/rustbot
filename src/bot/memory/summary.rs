//! Explicit recaps of the conversation, based on PyBot's store_replies._summary_reply.
use crate::bot::{Memory, Reading, compose};
use regex::{Captures, Regex};
use std::sync::LazyLock;

static ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?:(?:so|and|ok|okay|right|well)\s+)*(?:",
        r"what (?:do you (?:know|remember) about me|have i (?:told you|said)(?: about myself)?|did i (?:tell you|say)(?: about myself)?)|",
        r"(?:(?:can|could|would) you )?(?:(?:give me (?:a |the )?)?(?:quick )?(?:recap|summary|rundown)|",
        r"sum up|summari[sz]e)(?: (?:of )?(?:our (?:conversation|chat)|what (?:i've|i have) told you|",
        r"what we(?:'ve| have) (?:said|talked about)|the (?:chat|conversation)))?|",
        r"what (?:did|have) we talk(?:ed)? about)(?: (?:so far|then|again|please))*[?.!]*$"
    ))
    .unwrap()
});

static OWN_STATEMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:i|we)(?:(?:'m|'re|'ve|'d|'ll)\b|\s)|^(?:my|our)\s+\p{L}+(?:'s)?\s+\S+")
        .unwrap()
});

static PERSONAL_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(i'm|im|i've|i'll|i'd|i am|i was|we're|we've|we'll|we'd|we are|we were|i|we|my|our|me|us|mine|ours|myself|ourselves)\b").unwrap()
});

static UNSETTLED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)^(?:i|we)(?:'m|'re|'ve| am| are| have)?\s+(?:",
        r"think|guess|hope|wish|wonder|feel|felt|might|may|would|could|should|suppose|",
        r"want to know|need (?:to know|help)|(?:very |so |really )?(?:happy|sad|tired|upset|fine|better|scared|worried|feeling|the worst)|",
        r"(?:just )?(?:don't|do not) know)\b"
    ))
    .unwrap()
});

static UNFINISHED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\bbut (?:i|we) (?:haven't|havent|have not) done (?:it|that|so)(?: yet)?[.!]*$")
        .unwrap()
});

static FRIEND_MENTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(?:my|our)\s+friend\b|\ba\s+friend['’]s\b").unwrap());
static FRIEND_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bfriend\b").unwrap());
static WEEKDAY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)\b").unwrap()
});

fn filed_plan(memory: &Memory) -> Option<String> {
    let spoken = restate(memory.stated_plans.last_filed_origin()?)?;
    Some(
        WEEKDAY
            .replace_all(&spoken, |captured: &Captures<'_>| {
                let mut chars = captured[0].chars();
                chars
                    .next()
                    .into_iter()
                    .flat_map(char::to_uppercase)
                    .chain(chars)
                    .collect::<String>()
            })
            .into_owned(),
    )
}

fn restate(line: &str) -> Option<String> {
    let line = line.trim();
    let normalized = line.replace(['’', '‘', 'ʼ'], "'");
    if !OWN_STATEMENT.is_match(&normalized)
        || UNSETTLED.is_match(&normalized)
        || UNFINISHED.is_match(&normalized)
        || normalized.contains('?')
        || !(3..=14).contains(&line.split_whitespace().count())
    {
        return None;
    }
    let line = normalized.trim_end_matches(['.', '!', ',', ';']);
    let answer = PERSONAL_WORDS.replace_all(line, |capture: &Captures<'_>| {
        match capture[0].to_lowercase().as_str() {
            "i'm" | "im" | "we're" => "you're",
            "i've" | "we've" => "you've",
            "i'll" | "we'll" => "you'll",
            "i'd" | "we'd" => "you'd",
            "i am" | "we are" => "you are",
            "i was" | "we were" => "you were",
            "my" | "our" => "your",
            "mine" | "ours" => "yours",
            "myself" | "ourselves" => "yourself",
            _ => "you",
        }
    });
    Some(answer.into_owned())
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !ASK.is_match(&reading.clean) {
        return None;
    }
    // The active safety conversation has its own response rules.
    if memory
        .turns
        .iter()
        .rev()
        .take(3)
        .any(|turn| turn.module == "safety")
    {
        return None;
    }

    let mut parts: Vec<_> = memory
        .name
        .iter()
        .map(|name| format!("you're {name}"))
        .collect();
    parts.extend(memory.personal.summary_facts());
    let mentions_at = parts.len();
    parts.extend(filed_plan(memory));

    let mut seen = Vec::new();
    for turn in memory.turns.iter().rev().take(8) {
        if matches!(turn.module, "safety" | "urgent") {
            continue;
        }
        let line = turn.user.trim();
        let already_typed = memory.personal.covers_statement(line)
            || (memory.name.is_some() && super::INTRO.is_match(line))
            || memory.stated_plans.superseded_origin(line);
        if already_typed {
            continue;
        }
        if let Some(phrase) = restate(line) {
            let key = phrase.to_lowercase();
            if seen.contains(&key) || parts.iter().any(|part| part.to_lowercase() == key) {
                continue;
            }
            let prefix = if seen.is_empty() { "you said " } else { "" };
            parts.push(format!("{prefix}{phrase}"));
            seen.push(key);
            if seen.len() >= 2 {
                break;
            }
        }
    }
    if !parts.iter().any(|part| FRIEND_WORD.is_match(part))
        && memory.turns.iter().any(|turn| {
            !matches!(turn.module, "safety" | "urgent") && FRIEND_MENTION.is_match(&turn.user)
        })
    {
        parts.insert(mentions_at, "you mentioned your friend".to_owned());
    }
    if parts.is_empty() {
        return Some(
            "Not much yet — you haven't told me about yourself. What should I know?".to_owned(),
        );
    }
    let body = parts.join("; ");
    let replies = [
        format!("So far I know {body}."),
        format!("Here's what you've told me: {body}."),
        format!("You've told me {body}."),
    ];
    Some(compose::pick(&replies.each_ref().map(String::as_str), memory, "").into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeatable_recap_contains_current_facts_and_recent_tellings() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        for line in [
            "Hi, I'm Sam",
            "I live in Leeds",
            "I moved to York",
            "I finished painting yesterday",
            "Can I do it again?",
        ] {
            bot.turn(line);
        }
        let result = bot.turn("What have I told you so far?");
        assert_eq!(result.module, Some("memory_store"));
        assert!(
            result
                .reply
                .contains("you're Sam; you live in York; you said you finished painting yesterday"),
            "{result:?}"
        );
        assert!(!result.reply.contains("Leeds") && !result.reply.contains("do it again"));
    }

    #[test]
    fn recap_does_not_claim_external_summaries() {
        assert!(!ASK.is_match("summarise the news today"));
        assert!(ASK.is_match("can you give me a quick recap please?"));
    }

    #[test]
    fn recap_uses_filed_plans_and_explicit_mentions_without_inferring_a_child() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        for line in [
            "Hi, my name is Ana.",
            "How do I explain fractions to a ten-year-old?",
            "I'm planning to go to York on saturday.",
            "What could I bring to a friend's housewarming?",
            "I'm planning to go to Oslo next Saturday?",
        ] {
            bot.turn(line);
        }
        for _ in 0..10 {
            bot.turn("What is the capital of Chile?");
        }
        let result = bot.turn("What have I told you so far?");
        assert_eq!(result.module, Some("memory_store"));
        assert!(
            result.reply.contains(
                "you're Ana; you mentioned your friend; you're planning to go to York on Saturday"
            ),
            "{result:?}"
        );
        assert!(!result.reply.contains("year old"));
    }

    #[test]
    fn recap_does_not_restate_a_superseded_plan_origin() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("I'm planning to go to York.");
        bot.turn("I meant Leeds");
        assert!(bot.memory.stated_plans.latest().unwrap().contains("Leeds"));
        let result = bot.turn("What have I told you so far?");
        assert_eq!(result.module, Some("memory_store"));
        assert!(!result.reply.contains("York"), "{result:?}");
    }
}
