//! Denials of a stored name and replies to an explicit name question.
use super::{bare_name_words, filler};
use regex::Regex;
use std::sync::LazyLock;

static DENIAL: LazyLock<Regex> = LazyLock::new(|| {
    let name = r"[a-z][a-z'\-]{1,19}(?:\s+[a-z][a-z'\-]{1,19})?";
    Regex::new(&format!(
        r"(?i)^\W*(?:(?:no|nope|sorry|ah|oh|actually|wait|oi|hey)[,.!]?\s+){{0,2}}(?:i(?:'?m|\s+am)\s+not\s+({name})|my\s+name(?:'?s|\s+is)?\s*(?:is|was)?\s*not\s+({name})|my\s+name(?:'?s)?\s*(?:isn'?t|ain'?t|aint)\s+({name})|(?:that|this)(?:'?s|\s+is|\s+was)\s+not\s+my\s+name|my\s+name(?:'?s)?\s+(?:is\s+)?wrong|you(?:'?ve|\s+have)?\s+got\s+my\s+name\s+wrong|(?:please\s+)?(?:don'?t|do\s+not|stop)\s+(?:call|calling)\s+me\s+({name}))\W*(?:[.!]|\?)?$"
    )).unwrap()
});
static BARE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\W_]*([a-zA-Z][a-zA-Z'-]{1,19})[\W_]*$").unwrap());

pub(crate) fn takes_back(text: &str, known: &str) -> bool {
    let clean = filler::without_filler(text);
    let Some(m) = DENIAL.captures(&clean) else {
        return false;
    };
    let Some(said) = m.iter().skip(1).flatten().next() else {
        return !known.trim().is_empty();
    };
    let said = said.as_str().to_lowercase();
    let known = known
        .replace('.', " ")
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    !known.is_empty()
        && (said == known
            || said.split_whitespace().any(|w| w == known)
            || known.split_whitespace().any(|w| w == said))
}

pub(super) fn bare(text: &str) -> Option<&str> {
    let word = BARE.captures(text.trim())?.get(1)?.as_str();
    let low = word.to_ascii_lowercase();
    (low.contains(['a', 'e', 'i', 'o', 'u', 'y'])
        && bare_name_words::BLOCKED
            .binary_search(&low.as_str())
            .is_err())
    .then_some(word)
}

static HEDGES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:for what its worth|to be truthful|if that helps|to be honest|to be fair|if im honest|by the way|i suppose|i expect|i think|i guess|honestly|anyway|actually|though|thank you|thanks|cheers|tbh|btw|ta)\b").unwrap()
});
static ANSWER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|\W)(?:it's|its|it is|this is|that's|thats|my name'?s|my name is|the name'?s|the name is|call me|im|i'm|i am)\s+([a-z][a-z'-]{1,19})\s*[.!]?\s*$").unwrap()
});

pub(super) fn answer(text: &str) -> Option<String> {
    let raw = text.to_lowercase();
    let stripped = HEDGES.replace_all(&raw, " ");
    let joined = stripped.split_whitespace().collect::<Vec<_>>().join(" ");
    let clean = joined.trim_matches([' ', ',', '.', ':', ';', '!', '?']);
    let word = ANSWER
        .captures(clean)
        .and_then(|m| m.get(1))
        .map_or(clean, |m| m.as_str());
    bare(word).map(str::to_owned)
}
