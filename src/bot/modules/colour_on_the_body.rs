//! A colour on the speaker's own skin, or a brief blush.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Memory, Reading};

#[path = "colour_on_the_body_data.rs"]
mod data;

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").expect("colour words"));
static COLOUR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:red|reddish|pink|pinkish|blotchy|blotches|flushed)\b").expect("skin colours")
});
static BLUSH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:blush\w*|(?:face|cheeks?|neck|ears?)\b[^.!?]{0,24}?\b",
        r"(?:goes?|going|went|gone|turns?|turning|turned|gets?|getting|come up|came up)",
        r"\s+(?:(?:bright|very|all|so|quite|a bit|really|dead|properly)\s+)?red\b|",
        r"(?:red|redder|redden\w*)\b[^.!?]{0,30}?when(?:ever)? i (?:talk|speak|meet) (?:to )?people)\b"
    ))
    .expect("transient blushing")
});

// https://www.nhs.uk/symptoms/blushing/
const BLUSH_REPLIES: &[&str] = &[
    "Brief blushing can happen with nerves, heat or exercise. If this happens when you talk to people, take a moment and focus on the conversation. What usually brings it on?",
    "People can blush when they feel self-conscious or warm. If it happens a lot or affects daily life, a GP can help you look at the cause. Does it happen with people you know too?",
    "A blush can be brief and leave no lasting patch. If the redness stays or you notice other symptoms, ask a GP about it. How long does it usually last?",
];

// https://www.nhs.uk/conditions/cellulitis/
const PATCH_REPLIES: &[&str] = &[
    "Heat, friction and skin irritation can cause redness like that. Avoid scrubbing it or using new products on it. Ask a pharmacist or GP if it persists or changes. When did you first see it?",
    "A red patch can have several causes. Leave strong soaps or fragranced products off it and check whether it changes. Seek same-day advice if it becomes painful, hot or swollen. How long has it been there?",
    "Try to note where the redness started and whether it is spreading. A pharmacist can help if it persists or you are unsure what caused it. Heat, swelling or worsening pain are reasons to get advice promptly.",
    "Friction, heat or a new skin product might be involved. Keep an eye on any change. If you feel unwell with it or the redness becomes painful, hot or swollen, seek medical advice promptly.",
];

fn words(line: &str) -> impl Iterator<Item = &str> {
    WORD.find_iter(line).map(|word| word.as_str())
}

fn someone_else(word: &str) -> bool {
    let external = |who: &str| data::SOMEONE_ELSE.contains(&who) || data::PETS.contains(&who);
    external(word)
        || ["'s", "'", "s"]
            .iter()
            .any(|suffix| word.strip_suffix(suffix).is_some_and(external))
}

fn pick<'a>(pool: &'a [&str], line: &str, memory: &Memory) -> &'a str {
    let start = line.chars().map(|letter| letter as usize).sum::<usize>() % pool.len();
    let previous = memory
        .turns
        .iter()
        .rev()
        .find(|turn| !turn.bot.is_empty())
        .map(|turn| turn.bot.split_whitespace().collect::<Vec<_>>().join(" "));
    (0..pool.len())
        .map(|offset| pool[(start + offset) % pool.len()])
        .find(|reply| previous.as_ref().is_none_or(|last| !last.contains(reply)))
        .unwrap_or(pool[start])
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = reading
        .raw
        .replace('’', "'")
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if !COLOUR.is_match(&clean) && !clean.contains("blush") && !clean.contains("redden") {
        return None;
    }
    let words: Vec<_> = words(&clean).collect();
    let has = |phrases: &[&str]| words.iter().any(|word| phrases.contains(word));
    if words.iter().any(|word| someone_else(word))
        || has(data::SOMEWHERE_ELSE)
        || has(data::INSIDE)
        || has(data::ALSO_A_SYMPTOM)
        || has(data::A_BURN)
        || has(data::NOT_A_PATCH)
    {
        return None;
    }
    if BLUSH.is_match(&clean) {
        if clean.starts_with("why") && !has(&["i", "my", "me", "im", "i'm", "myself"]) {
            return None;
        }
        return Some(pick(BLUSH_REPLIES, &clean, memory).to_owned());
    }
    if !COLOUR.is_match(&clean) || !has(data::BODY_PARTS) {
        return None;
    }
    if memory
        .turns
        .iter()
        .rev()
        .filter(|turn| !turn.user.trim().is_empty())
        .take(4)
        .any(|turn| self::words(&turn.user.to_lowercase()).any(|word| data::PETS.contains(&word)))
    {
        return None;
    }
    Some(pick(PATCH_REPLIES, &clean, memory).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn colour_on_the_body_scopes_replies_and_avoids_repeats() {
        let mut memory = Memory::default();
        for prompt in [
            "My son's arm is red",
            "My back is red and itchy",
            "My arm has gone red after boiling water spilled on it",
            "My eyes are red",
            "My red socks stained the wash",
            "Why do we blush?",
        ] {
            assert!(
                respond(&Reading::new(prompt), &memory).is_none(),
                "{prompt}"
            );
        }
        let patch = Reading::new("My neck is blotchy");
        let first = respond(&patch, &memory).unwrap();
        memory.turns.push(Exchange {
            user: patch.raw.to_owned(),
            bot: first.clone(),
            module: "colour_on_the_body",
        });
        assert_ne!(respond(&patch, &memory).unwrap(), first);
        let blush = Reading::new("I go bright red when I talk to people");
        assert!(BLUSH_REPLIES.contains(&respond(&blush, &memory).unwrap().as_str()));
        memory.turns.push(Exchange {
            user: "My dog keeps scratching".into(),
            bot: String::new(),
            module: "answers",
        });
        assert!(respond(&patch, &memory).is_none());
        let turn = Bot::new(Some("0")).unwrap().turn(patch.raw);
        assert_eq!(turn.module, Some("colour_on_the_body"), "{turn:?}");
    }
}
