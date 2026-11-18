//! Clarification of a preceding answer, after PyBot's bot/memory/meant.py.
use super::measurement::sentences;
use crate::bot::{
    Memory, Reading, compose,
    understanding::normalize::{stem, tokens},
};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($source).unwrap());
    };
}

pattern!(
    MEANING,
    concat!(
        r"^(?:(?:and|so|but|ok|okay|well|right|sorry|wait|basically|hmm|um|oh|then)[ ,]+)*(?:",
        r"(?:what|wot|wat|wut|how) (?:(?:do|did|d) )?(?:you|u|ya) mean(?: by (?:that|it|this))?|",
        r"(?:whatcha|watcha|whaddya|whatdya) mean(?: by (?:that|it|this))?|",
        r"what(?:'s|s)? (?:(?:does|do|is) )?(?:that|it|this) (?:(?:actually|really|exactly|precisely|even|then) )?(?:supposed to )?mean|",
        r"what(?:'s|s| is)? (?:the )?meaning of (?:that|it|this)|what is meant by (?:that|it|this)|",
        r"(?:that|it|this) means what|meaning what|explain (?:what )?(?:you|u) mean|wym|wdym)",
        r"(?: for (?:me|us|him|her|them|my case|my situation|(?:my|our) [\p{L}'-]+(?: [\p{L}'-]+)?))?",
        r"(?: about (?:my|our|the|this|that) [\p{L}'-]+(?: [\p{L}'-]+){0,2})?",
        r"(?: in (?:practice|reality|real terms))?(?:[ ,]+(?:then|though|exactly|precisely|really|actually))?[.?!]*$"
    )
);

pattern!(
    SIMPLER,
    concat!(
        r"^(?:(?:and|so|but|ok|okay|well|right|sorry|er|um)[ ,]+)*(?:",
        r"(?:can|could|would) you (?:please )?(?:(?:say|repeat|explain|rephrase|clarify)(?: (?:that|it))?(?: again)?|",
        r"put (?:that|it) (?:another way|in (?:simple terms|plain english)))|",
        r"(?:say|repeat) (?:that|it) again|explain(?: that| again)?|meaning|",
        r"(?:i )?(?:(?:still|really|just|honestly|actually) )?(?:don't|dont|do not|didn't|didnt|did not) ",
        r"(?:(?:really|still|quite|honestly|fully) )?(?:understand|follow|get it|catch that|(?:get|see|follow) what you mean)",
        r"(?: (?:it|that|this|at all|one bit|any of it|what you mean|the last bit))?|",
        r"(?:i'm|im|i am) (?:not (?:following|with you)|(?:(?:still|a bit|totally|completely) )?(?:confused|lost))|",
        r"(?:you )?(?:have )?lost me(?: (?:there|completely|entirely))?|(?:that is|that's|this is) confusing|",
        r"huh|eh|come again|say what|(?:sorry[ ,]+)?(?:what|pardon)\?|",
        r"(?:(?:say|explain|put) it )?in (?:plain english|english|simple terms)|english please|break (?:that|it) down)",
        r"(?: please)?[.?!]*$"
    )
);
pattern!(
    PERSONAL,
    r"\bfor (?:me|us|him|her|them|my [\p{L}'-]+|our [\p{L}'-]+)\b|\bin (?:practice|reality|real terms)\b"
);
pattern!(
    SUBJECT,
    r"\b(?:for|about) (?:my|our|the|this|that) (?P<subject>[\p{L}'-]+(?: [\p{L}'-]+)?)\b"
);
pattern!(
    DEFINES,
    r"(?i)\b(?:means?|is when|is what|is where|in other words|refers to|stands for|which means|that is what)\b"
);
pattern!(CLAUSE, r"(?i):\s+|\s+-\s+|;\s+|,\s+(?:so|and|but|which)\s+");
pattern!(
    RECEIPT,
    r"(?i)^\s*(?:(?:sure|again)\s*[-—]\s*)?(?:noted\b|got it\b|so far i know\b|thanks for telling me\b|i hear you\b|i'?m listening\b|say more\b|tell me more\b|that sounds\b|i'?ll keep that\b|good to know\b|i'?m here if\b|nothing of mine to go back over|i haven'?t put anything)|[-—]\s*(?:noted|good to know|got it|i'?ll remember that)[.!]*$"
);
pattern!(
    HEDGE,
    r"(?i)\b(?:i (?:don'?t know enough|don'?t have enough|can'?t answer that)|that'?s beyond what i know|i'?d rather not guess|a quick search online|i haven'?t got enough)\b"
);
pattern!(
    CONDOLENCE,
    r"(?i)^\s*(?:i'?m so sorry|i'?m sorry|sorry to hear|ugh|oh no|that sounds (?:rough|tough|hard|awful)|that'?s (?:rough|hard|awful))\b"
);

const MEANING_LEADS: &[&str] = &["I mean this:", "Just this:", "What I meant:"];
const REPEAT_LEADS: &[&str] = &["I said:", "Same again:", "Let me put it plainer:"];
const PERSONAL_LEADS: &[&str] = &["For you, that means:", "In your case:"];
const SAID_TWICE: &[&str] = &[
    "I've put that twice now, so the words in it are probably the problem - tell me which bit and I'll unpack that one.",
    "That is the second time I've said it that way. Which word in it is the trouble?",
];
const NOTHING_WRITTEN: &[&str] = &[
    "I haven't put anything about it into words yet - give me a bit more and I'll start from there.",
    "Nothing of mine to go back over yet. Tell me a little more and I'll take it from the top.",
    "I hadn't answered anything yet, so there's nothing of mine to put straighter - tell me what you were after and I'll take it on.",
    "Nothing of mine to explain yet: I said how that landed and stopped there. Say what you were asking and I'll have a proper go at it.",
    "That was me saying how it lands, not an answer. Ask me again and I'll take it on.",
];
const NO_ANSWER: &[&str] = &[
    "I didn't have an answer there, so there's nothing of mine to explain - it just didn't know. Say a bit more about it and I'll have a proper go.",
    "Nothing of mine to go over: I didn't know that one. Tell me more and I'll try again.",
];

fn body(said: &str) -> Option<&str> {
    MEANING_LEADS
        .iter()
        .chain(REPEAT_LEADS)
        .chain(PERSONAL_LEADS)
        .chain([&"Same as before:", &"As I said:", &"Still:"])
        .find_map(|lead| said.strip_prefix(lead).map(str::trim))
}

fn first_clause(sentence: &str) -> String {
    let mut text = sentence.trim().to_owned();
    if let Some(cut) = CLAUSE
        .find_iter(&text)
        .find(|cut| text[..cut.start()].chars().count() >= 30)
    {
        let before = &text[..cut.start()];
        let after = text[cut.end()..].trim();
        if cut.as_str().starts_with(':') && after.chars().count() >= 30 && DEFINES.is_match(after) {
            let mut chars = after.chars();
            text = chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars)
                .collect();
        } else {
            text = before.to_owned();
        }
    }
    if let Some((at, _)) = text.char_indices().nth(150) {
        text.truncate(at);
        if let Some((at, _)) = text.rsplit_once(' ') {
            text.truncate(at.len());
        }
    }
    let text = text.trim().trim_end_matches([',', ';', ':', '-', ' ']);
    if text.ends_with(['.', '!', '?']) {
        text.to_owned()
    } else {
        format!("{text}.")
    }
}

fn substantial(said: &str, question: &str) -> bool {
    if said.chars().any(|letter| letter.is_ascii_digit()) {
        return true;
    }
    const STOP: &str = "the and this that are was were with you your they're their have had but because from what which when how into can could would has does just like then this such";
    let meaningful = |line: &str| {
        tokens(line)
            .into_iter()
            .filter(|word| {
                word.len() > 2 && !STOP.split_ascii_whitespace().any(|stop| stop == word)
            })
            .collect::<Vec<_>>()
    };
    let said = meaningful(said);
    said.len() >= 5 || meaningful(question).iter().any(|word| said.contains(word))
}

fn cite(answer: &str, question: &str, meaning: bool) -> Option<String> {
    if HEDGE.is_match(answer) || RECEIPT.is_match(answer) || !substantial(answer, question) {
        return None;
    }
    let parts = sentences(answer);
    let mut parts: Vec<_> = parts
        .into_iter()
        .filter(|part| !CONDOLENCE.is_match(part) && !HEDGE.is_match(part))
        .collect();
    let first = *parts.first()?;
    let chosen = if answer.ends_with('?') && (meaning || parts.len() == 1) {
        parts.pop().unwrap_or(first)
    } else {
        parts
            .iter()
            .copied()
            .find(|part| DEFINES.is_match(part))
            .unwrap_or(first)
    };
    (chosen.ends_with('?') || substantial(chosen, question)).then(|| first_clause(chosen))
}

fn applies_to_known_subject(line: &str, memory: &Memory) -> bool {
    let Some(subject) = SUBJECT.captures(line) else {
        return true;
    };
    let subject = &subject["subject"];
    if matches!(subject, "case" | "situation" | "own case" | "own situation") {
        return true;
    }
    let nouns = tokens(subject);
    memory.turns.iter().rev().take(3).any(|turn| {
        let words = tokens(&format!("{} {}", turn.user, turn.bot));
        nouns
            .iter()
            .all(|noun| words.iter().any(|word| stem(word) == stem(noun)))
    })
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = &reading.clean;
    if super::shorter::reads_it(line) {
        return None;
    }
    let meaning = MEANING.is_match(line);
    if !meaning && !SIMPLER.is_match(line) || !applies_to_known_subject(line, memory) {
        return None;
    }
    let personal = meaning && PERSONAL.is_match(line);
    let recent = memory
        .turns
        .iter()
        .rev()
        .take(7)
        .take_while(|turn| !matches!(turn.module, "safety" | "urgent" | "loss" | "grief"))
        .filter(|turn| !turn.bot.trim().is_empty());
    let last = recent.clone().next()?;
    let mut handed_back = Vec::new();
    for turn in recent {
        let answer = turn.bot.trim();
        if let Some(previous) = body(answer) {
            if personal
                && !PERSONAL_LEADS.iter().any(|lead| answer.starts_with(lead))
                && handed_back.is_empty()
            {
                let plain = cite(previous, &turn.user, true).unwrap_or_else(|| previous.to_owned());
                let lead = compose::pick(PERSONAL_LEADS, memory, line);
                return Some(format!("{lead} {plain}"));
            }
            handed_back.push(previous);
            continue;
        }
        if let Some(plain) = cite(answer, &turn.user, meaning)
            && !handed_back
                .iter()
                .any(|seen| seen.starts_with(&plain) || plain.starts_with(seen))
        {
            let leads = if meaning { MEANING_LEADS } else { REPEAT_LEADS };
            let lead = compose::pick(leads, memory, line);
            return Some(format!("{lead} {plain}"));
        }
    }
    let fallback = if body(&last.bot).is_some() {
        SAID_TWICE
    } else if HEDGE.is_match(&last.bot) {
        NO_ANSWER
    } else {
        NOTHING_WRITTEN
    };
    Some(compose::pick(fallback, memory, line).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn clarification_cites_a_sentence_and_does_not_carry_into_a_new_subject() {
        let mut memory = Memory::default();
        memory.turns.push(Exchange {
            user: "Why do onions make you cry?".into(),
            bot: "Cutting an onion breaks its cells and releases a gas that irritates your eyes, so they make tears to wash it away. A sharp knife releases less of it.".into(),
            module: "answers",
        });
        let response = respond(&Reading::new("I don't follow."), &memory).unwrap();
        assert_eq!(
            response,
            "I said: Cutting an onion breaks its cells and releases a gas that irritates your eyes."
        );
        assert!(respond(&Reading::new("What does that mean for my rent?"), &memory).is_none());
    }

    #[test]
    fn clarification_does_not_cross_an_emergency_turn() {
        let mut memory = Memory::default();
        memory.turns.push(Exchange {
            user: "An earlier conversation turn".into(),
            bot: "An emergency reply.".into(),
            module: "safety",
        });
        assert!(respond(&Reading::new("Pardon?"), &memory).is_none());
    }
}
