//! The conversation walk for another person's town from PyBot's memory/places.py.
use super::personal;
use crate::bot::Memory;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! rule {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($source).unwrap());
    };
}

const PLACE: &str = r"[A-Za-z][A-Za-z'-]{1,}(?:[ -][A-Za-z][A-Za-z'-]{1,})?";
rule!(
    NAMED_PLACE,
    &format!(
        r"(?i)\bmy (\w+)s?\b[^.!?]{{0,60}}?\b(?:live|lives|living|moved|moves|moving|settled|based|staying|relocated)\s+(?:in|to|near|at)\s+({PLACE})"
    )
);
rule!(
    PRONOUN_PLACE,
    &format!(
        r"(?i)\b(?:she|he|they)\s+(?:has\s+|had\s+|just\s+|now\s+)?(?:live[sd]?|lives|living|moved|moves|moving|settled|relocated|is\s+based|are\s+based)\s+(?:in|to|near|at)\s+({PLACE})"
    )
);
rule!(
    PERSON_NAMED,
    r"(?i)\bmy (\w+)s?\b(?:[^.!?]{0,40}?\b(?:is|are|was|were)\s+(?:called|named|known as)\b|[^.!?]{0,12}$)"
);
rule!(
    QUESTION,
    r"(?i)^\W*(?:and\s+|but\s+|so\s+)?(?:do|does|did|is|are|was|were|has|have|had|can|could|will|would|should|shall|may|might|am)\b"
);
rule!(
    CORRECTION,
    r"(?i)^(?:\s*(?:sorry|no|nope|actually|wait|i meant|my mistake|make that)\b[\s,.-]*|not\s+[a-z'-]+[, ]+)"
);
rule!(PLACE_LEAD, r"^(?:in|to|at|from|near)\s+");

const PLACE_ENDS: &str = "last next this that year years month months week weeks ago previously before then soon now already after and but because";
const NOT_VALUE: &str = "i me my we us our you your he him his she her it its they them their one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen eighteen nineteen twenty thirty forty fifty sixty seventy eighty ninety hundred thousand million first second third last next again too either neither";

fn place(phrase: &str) -> Option<String> {
    let words: Vec<_> = phrase
        .split([' ', '-'])
        .take(2)
        .take_while(|word| {
            !PLACE_ENDS
                .split_whitespace()
                .any(|end| word.eq_ignore_ascii_case(end))
        })
        .collect();
    personal::place(&words.join(" "))
}

fn place_alone(line: &str) -> Option<String> {
    let low = line
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    if low.ends_with('?') {
        return None;
    }
    let rest = &low[CORRECTION.find(&low)?.end()..];
    let rest = PLACE_LEAD.replace(rest.trim_matches([' ', '.', '!', ',', '-']), "");
    let mut words = rest.split_whitespace();
    if !words
        .clone()
        .all(|word| !NOT_VALUE.split_whitespace().any(|stop| stop == word))
        || !(1..=2).contains(&words.by_ref().count())
    {
        return None;
    }
    personal::place(&rest)
}

fn said_places(memory: &Memory) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let mut last = None;
    let mut fresh = false;
    for turn in &memory.turns {
        let line = turn.user.trim();
        if line.is_empty() || line.ends_with('?') || QUESTION.is_match(line) {
            fresh = false;
            continue;
        }
        let mut gave = false;
        let mut named = None;
        for found in NAMED_PLACE.captures_iter(line) {
            let who = found[1].to_lowercase();
            if let Some(town) = place(&found[2]) {
                result.push((who.clone(), town));
                gave = true;
            }
            named = Some(who);
        }
        if named.is_none() {
            named = PERSON_NAMED
                .captures(line)
                .map(|found| found[1].to_lowercase());
        }
        if named.is_some() {
            last = named;
        } else if let Some(who) = &last
            && fresh
            && !PRONOUN_PLACE.is_match(line)
            && let Some(town) = place_alone(line)
        {
            result.push((who.clone(), town));
            gave = true;
        }
        if let Some(who) = &last
            && !gave
        {
            for found in PRONOUN_PLACE.captures_iter(line) {
                if let Some(town) = place(&found[1]) {
                    result.push((who.clone(), town));
                    gave = true;
                }
            }
        }
        fresh = gave;
    }
    result
}

pub(super) fn respond(known_role: Option<&str>, subject: &str, memory: &Memory) -> Option<String> {
    let places = said_places(memory);
    let role = known_role
        .or_else(|| subject.strip_prefix("my "))
        .or_else(|| {
            matches!(subject, "he" | "she" | "they")
                .then(|| places.last().map(|(who, _)| who.as_str()))
                .flatten()
        })?;
    if matches!(role, "i" | "we") {
        return None;
    }
    if let Some((who, town)) = places
        .iter()
        .rev()
        .find(|(who, _)| who.trim_end_matches('s') == role.trim_end_matches('s'))
    {
        let verb = if who.ends_with('s') { "live" } else { "lives" };
        return Some(format!("Your {who} {verb} in {town}."));
    }
    let verb = if role.ends_with('s') { "live" } else { "lives" };
    Some(format!("You haven't said where your {role} {verb}."))
}

pub(super) fn changed(line: &str, memory: &Memory) -> Option<String> {
    let town = place_alone(line).or_else(|| {
        CORRECTION
            .is_match(line)
            .then(|| PRONOUN_PLACE.captures(line))
            .flatten()
            .as_ref()
            .and_then(|found| place(&found[1]))
    })?;
    let before = &memory.turns.last()?.user;
    if !NAMED_PLACE.is_match(before) && !PRONOUN_PLACE.is_match(before) {
        return None;
    }
    let (who, previous) = said_places(memory).pop()?;
    if previous.eq_ignore_ascii_case(&town) {
        return None;
    }
    let verb = if who.ends_with('s') { "live" } else { "lives" };
    Some(format!("Got it — your {who} {verb} in {town}."))
}
