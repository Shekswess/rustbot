//! A bare referral question reuses the person already named in the conversation.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Memory, Reading, compose};

struct Expert {
    lead: &'static str,
    words: Regex,
    ways: [&'static str; 2],
}

static EXPERTS: LazyLock<[Expert; 9]> = LazyLock::new(|| {
    let expert = |lead, pattern, ways| Expert {
        lead,
        words: Regex::new(pattern).expect("expert pattern"),
        ways,
    };
    [
        expert(
            "Your health visitor",
            r"(?i)\bhealth visitors?\b",
            [
                "ring them and they will talk it through with you today",
                "ring them and describe what you have seen: they would rather hear from you than not",
            ],
        ),
        expert(
            "A vet",
            r"(?i)\bvets?\b|\bvet's\b|\bveterinary\b",
            [
                "ring the practice and say how long it has been going and what you have already tried",
                "book an appointment and take a note of what you have seen and any treatment they are on",
            ],
        ),
        expert(
            "A dentist",
            r"(?i)\bdentists?\b|\bhygienist\b|\bdental (?:practice|surgery)\b",
            [
                "ring the practice and ask for the earliest appointment",
                "ring the practice and ask for an urgent slot",
            ],
        ),
        expert(
            "Your GP",
            r"(?i)\bgp\b|\bgp's\b|\bdoctors?\b|\bfamily doctor\b",
            [
                "ring the surgery and ask for the earliest appointment",
                "book in and take a note of when it started and what makes it worse",
            ],
        ),
        expert(
            "A physio",
            r"(?i)\bphysios?\b|\bphysiotherapist\b",
            [
                "you can usually book with one directly, without going through a GP",
                "book an appointment and take a note of when the pain comes on",
            ],
        ),
        expert(
            "An optician",
            r"(?i)\bopticians?\b|\boptometrist\b",
            [
                "book an eye test, and say if anything has changed suddenly",
                "ring them and say what has changed, and they will say how soon to come in",
            ],
        ),
        expert(
            "A pharmacist",
            r"(?i)\bpharmacists?\b|\bchemist\b",
            [
                "they can advise today without an appointment",
                "ask at the counter, and take the packet of anything you already take",
            ],
        ),
        expert(
            "Your midwife",
            r"(?i)\bmidwives?\b|\bmaternity unit\b",
            [
                "ring your midwife or the maternity unit and they will say how soon to be seen",
                "ring the maternity unit rather than waiting for the next appointment",
            ],
        ),
        expert(
            "The practice nurse",
            r"(?i)\bpractice nurse\b|\bnurse\b",
            [
                "ring the surgery and ask whether the practice nurse can see you",
                "ask at the desk for the practice nurse",
            ],
        ),
    ]
});

static ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)^(?:(?:and|so|ok|okay|but|right|then|well)\s+)*",
        r"(?:who\s+(?:should|shall|do|can|could|would|must|might|am\s+i)\s+(?:i|we)\s+",
        r"(?:see|call|ring|contact|ask|book|speak\s+to|talk\s+to|go\s+and\s+see|go\s+to|",
        r"get\s+in\s+touch\s+with|be\s+seeing|need\s+to\s+see)",
        r"|who\s+to\s+(?:see|call|ring|ask|contact|speak\s+to|talk\s+to)",
        r"|who\s+do\s+i\s+need\s+to\s+(?:see|call)",
        r"|which\s+(?:doctor|dentist|vet|specialist|professional|person|one)\s+",
        r"(?:should|do|would)\s+i\s+(?:see|call|ring|go\s+to)",
        r"|where\s+do\s+i\s+go\s+for\s+(?:it|that|this))",
        r"(?:\s+(?:about|for|with)\s+(?:it|that|this|them|someone))?",
        r"(?:\s+(?:then|now|instead|first|please|again|next))?[\s,.!?]*$"
    ))
    .expect("bare referral ask")
});
static PREAMBLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:,\s*(?:and|but|so)\s+|;\s*|\.\s+)").unwrap());
static THREAD: LazyLock<[(usize, Regex); 3]> = LazyLock::new(|| {
    [
        (
            2,
            Regex::new(r"(?i)\b(?:tooth|teeth|gums?|dental|dentist|molar|abscess|braces?)\b")
                .unwrap(),
        ),
        (
            5,
            Regex::new(
                r"(?i)\b(?:eyes?|eyesight|vision|optician|blurry|blurred|contact lenses?)\b",
            )
            .unwrap(),
        ),
        (
            7,
            Regex::new(r"(?i)\bpregnan|\b(?:midwife|maternity|unborn)\b").unwrap(),
        ),
    ]
});
const QUIET: &[&str] = &["safety", "urgent", "grief", "loss", "a_count_of_packets"];

fn flat(text: &str) -> String {
    text.to_lowercase()
        .replace(['‘', '’'], "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn asks_who(text: &str) -> bool {
    ASK.is_match(flat(text).trim_matches([' ', '.', '!', '?']))
}

fn sentence_at(text: &str, position: usize) -> (&str, usize) {
    let mut begin = 0;
    for (i, pair) in text.as_bytes().windows(2).enumerate() {
        if pair[1] == b' ' && matches!(pair[0], b'.' | b'!' | b'?') {
            if i + 1 >= position {
                return (&text[begin..i + 1], begin);
            }
            begin = i + 2;
        }
    }
    (&text[begin..], begin)
}

fn person_in(text: &str) -> Option<(&Expert, &str, usize)> {
    let (expert, at) = EXPERTS
        .iter()
        .filter_map(|expert| expert.words.find(text).map(|m| (expert, m.start())))
        .min_by_key(|(_, at)| *at)?;
    let (sentence, begin) = sentence_at(text, at);
    Some((expert, sentence, at - begin))
}

fn says_again(way: &str, already: &str) -> bool {
    let words = way.split_whitespace().collect::<Vec<_>>();
    words
        .windows(3)
        .any(|words| already.contains(&words.join(" ")))
}

fn initial_uppercase(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn shape(
    expert: &Expert,
    sentence: &str,
    at: usize,
    repeat: bool,
    reading: &Reading<'_>,
    memory: &Memory,
) -> String {
    let offset = PREAMBLE
        .find_iter(sentence)
        .map(|m| m.end())
        .take_while(|end| *end <= at)
        .filter(|end| at - *end <= 80)
        .last()
        .unwrap_or(0);
    let clause = sentence[offset..]
        .trim()
        .trim_matches('"')
        .trim_end_matches([' ', ',', ';', ':']);
    let already = flat(clause);
    let fresh = expert
        .ways
        .iter()
        .copied()
        .filter(|way| !says_again(way, &already))
        .collect::<Vec<_>>();
    let ways = if fresh.is_empty() {
        &expert.ways[..]
    } else {
        &fresh
    };
    let way = compose::pick(ways, memory, reading.raw);
    if repeat && at - offset <= 45 && !clause.is_empty() {
        let mut body = initial_uppercase(clause);
        if !body.ends_with(['.', '!', '?']) {
            body.push('.');
        }
        if !says_again(way, &already) {
            body.push(' ');
            body.push_str(&initial_uppercase(way));
            body.push('.');
        }
        return body;
    }
    format!("{}: {way}.", expert.lead)
}

fn subject_has_content(text: &str) -> bool {
    if text.trim().is_empty() || asks_who(text) {
        return false;
    }
    let read = Reading::new(text);
    if matches!(
        read.features.intent,
        crate::bot::understanding::kinds::Intent::Greeting
            | crate::bot::understanding::kinds::Intent::Thanks
            | crate::bot::understanding::kinds::Intent::Ack
            | crate::bot::understanding::kinds::Intent::Farewell
    ) {
        return false;
    }
    !read.entities.is_empty()
        || read.line.focus.iter().any(|word| {
            word.len() > 2
                && ![
                    "what", "how", "why", "when", "then", "can", "could", "should", "would",
                    "thanks", "next", "please", "help", "about", "there",
                ]
                .contains(&word.as_str())
        })
}

pub(in crate::bot) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !asks_who(reading.raw) {
        return None;
    }
    let last = memory.turns.last()?;
    if last.bot.trim().is_empty() || QUIET.contains(&last.module) {
        return None;
    }
    if let Some((expert, sentence, at)) = person_in(&last.bot) {
        return Some(shape(expert, sentence, at, true, reading, memory));
    }
    let subject = memory
        .turns
        .iter()
        .rev()
        .find(|turn| subject_has_content(&turn.user));
    if let Some(subject) = subject
        && !std::ptr::eq(subject, last)
        && !QUIET.contains(&subject.module)
        && let Some((expert, sentence, at)) = person_in(&subject.bot)
    {
        return Some(shape(expert, sentence, at, false, reading, memory));
    }
    let read;
    let topics = if let Some(subject) = subject {
        read = Reading::new(&subject.user);
        if read.topics.is_empty() {
            &reading.topics
        } else {
            &read.topics
        }
    } else {
        &reading.topics
    };
    let has = |topic| topics.iter().any(|(name, _)| *name == topic);
    let care = has("health") || has("feelings") || has("family-and-parenting");
    let chosen = if care {
        THREAD.iter().find_map(|(expert, pattern)| {
            memory
                .turns
                .iter()
                .rev()
                .take(3)
                .any(|turn| pattern.is_match(&turn.user))
                .then_some(*expert)
        })
    } else {
        None
    };
    let index = chosen.or_else(|| {
        if has("pets") {
            Some(1)
        } else if has("health") || has("feelings") {
            Some(3)
        } else {
            None
        }
    })?;
    Some(shape(&EXPERTS[index], "", 0, false, reading, memory))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn a_bare_referral_reuses_the_person_named_in_the_last_answer() {
        let mut memory = Memory::default();
        memory.turns.push(Exchange {
            user: "My tooth hurts".into(),
            bot: "Until you can get help, avoid chewing on that side. See a dentist if the pain keeps you awake.".into(),
            module: "answers",
        });
        let result = reply(&Reading::new("Who should I see then?"), &memory).unwrap();
        assert!(result.starts_with("See a dentist if the pain keeps you awake."));
        assert!(
            reply(
                &Reading::new("Who should I call about the boiler?"),
                &memory
            )
            .is_none()
        );
        memory.turns[0].module = "safety";
        assert!(reply(&Reading::new("Who should I see then?"), &memory).is_none());
    }
}
