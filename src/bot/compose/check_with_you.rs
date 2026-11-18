//! Native port of bot/compose/check_with_you.py.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;
static A_CHECK_ASKED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:are|r|is|was|do|does|would|will|could)?\s*(?:you|u|ya)\s+(?:really\s+|still\s+|definitely\s+)?(?:be\s+)?(?:ok|okay|alright|all\s+right|fine|happy|cool|good|easy|content)\s+(?:with|wiv|wid|about|by)\s+(?:that|this|it|them|those|these|the\s+\w+|my\s+\w+|me(?:\s+\w+ing)?|me\s+\w+|if\s+i\b|it\s+if)\b"#).unwrap()
});
static A_THING_FOR_YOU: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:(?:is|was|are|were)\s+)?(?:that|this|it|those|these)\s+(?:really\s+)?(?:ok|okay|alright|all\s+right|fine)\s+(?:with|wiv|wid|by|for)\s+(?:you|u|ya)\b"#).unwrap()
});
static A_VERB_FOR_YOU: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:does|do|would|will|did)?\s*(?:that|this|it)\s+(?:work|suit|square|be\s+alright|be\s+ok|be\s+fine)\s+(?:for|with|wiv|wid)\s+(?:you|u|ya)\b"#).unwrap()
});
static A_VERB_YOU: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"\b(?:does|do|would|will|did|is)\s+(?:that|this|it)\s+(?:work|suit|do)\s+(?:you|u|ya)\b"#,
    )
    .unwrap()
});
static BARE_FOR_YOU: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*(?:so\s+)?(?:(?:ok|okay|alright|all\s+right|fine|happy|cool|easy|content)|work|works|suit|suits|square)\s+(?:for|with|wiv|wid|by)\s+(?:you|u|ya)\s*\??\s*$"#).unwrap()
});
static NOT_A_CHECK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:drive|driving|tow|towing|insure|sign|signing|sue|arrest|publish|fly|flying|sail|carry|carrying|keep\s+it|eat|drink)\b"#).unwrap()
});
const AGREED: &[&str] = &[
    "Fine by me - and it's your call, not mine.",
    "That suits me. Whether it suits you is the part I can't answer.",
    "No objection here. You're the one it has to work for, though.",
    "Sounds alright to me. I've got nothing at stake in it either way.",
    "Happy with that. It's your call at the end of it.",
    "Works for me - just don't count me as the one who decided it.",
    "That's fine as far as I'm concerned. What matters is whether it's fine by you.",
];

pub(crate) fn reads(text: &str) -> bool {
    let said: String = text
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '\'' | '?' | ' ') {
                c
            } else {
                ' '
            }
        })
        .collect();
    !NOT_A_CHECK.is_match(&said)
        && [
            &A_CHECK_ASKED,
            &A_THING_FOR_YOU,
            &A_VERB_FOR_YOU,
            &A_VERB_YOU,
            &BARE_FOR_YOU,
        ]
        .iter()
        .any(|pattern| pattern.is_match(&said))
}
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    reads(reading.raw).then(|| {
        let index = super::draw(&mut memory.rng.borrow_mut().0, AGREED.len());
        AGREED[index].to_owned()
    })
}
