//! Recall a prior line about the object the person asks what they did with.

use crate::{
    bot::{Memory, Reading, memory},
    text::is_space,
};
use fancy_regex::{Captures, Regex};
use std::sync::LazyLock;

const HEAD: &str = r"^(?:and |so |but |also |oh |ok |okay )?";
static DID_WITH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"{HEAD}(?:what|which) (?P<thing>[a-z][a-z' ]{{1,24}}?)\s*(?:did i|have i)\s+(?!said?\b|tell\b|told\b|mention\b|mentioning\b)(?P<did>[a-z]+)\s*[?.!]*$"
    ))
    .expect("ask for a thing they did something with")
});
static WAS_IT_I: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"{HEAD}which (?P<thing>[a-z][a-z' ]{{1,24}}?)\s*(?:was it|is it|was that)\s+(?:that\s+)?i\s+(?!said?\b|tell\b|told\b|mention\b)(?P<did>[a-z]+)\s*[?.!]*$"
    ))
    .expect("ask which thing it was")
});
static WORD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-z']+").expect("words used in recalls"));
const ATTRIBUTE: &[&str] = &[
    "size", "colour", "color", "brand", "make", "model", "number", "price", "cost", "amount",
    "weight", "length", "width", "height", "age", "year", "month", "day", "date", "time",
];
const NOT_A_THING: &[&str] = &[
    "one", "thing", "things", "bit", "bits", "part", "parts", "point", "way", "kind", "sort",
    "time", "other", "else", "lot", "lots",
];

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let lower = flattened(reading.raw);
    let (thing, did) = subject(&lower)?;
    let words: Vec<_> = thing.split(' ').map(stem).collect();
    let line = earlier_line(&words, stem(did), memory)?;
    let unpadded = lower.trim_matches([' ', ',', '.', ';', ':', '!', '?']);
    let store_can_answer = if unpadded == reading.raw {
        memory::respond(reading, memory).is_some()
    } else {
        memory::respond(&Reading::new(unpadded), memory).is_some()
    };
    if store_can_answer {
        return None;
    }
    let said = line.to_lowercase();
    let times = memory.turns[memory.turns.len().saturating_sub(4)..]
        .iter()
        .filter(|turn| turn.bot.to_lowercase().contains(&said))
        .count();
    let opening = match times {
        0 => "You said:",
        even if even.is_multiple_of(2) => "Still the same one:",
        _ => "Same line again:",
    };
    Some(format!("{opening} \"{line}\""))
}

fn flattened(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn subject(line: &str) -> Option<(&str, &str)> {
    let read = |pattern: &Regex| -> Option<Captures<'_>> { pattern.captures(line).ok().flatten() };
    let matched = read(&WAS_IT_I).or_else(|| read(&DID_WITH))?;
    let mut thing = matched.name("thing")?.as_str().trim_matches(is_space);
    while let Some((field, rest)) = thing.split_once(' ')
        && ATTRIBUTE.contains(&field)
    {
        thing = rest.trim_start_matches(' ');
    }
    if thing.is_empty() || ATTRIBUTE.contains(&thing) || NOT_A_THING.contains(&thing) {
        return None;
    }
    Some((thing, matched.name("did")?.as_str()))
}

fn stem(word: &str) -> &str {
    let word = word.trim_matches('\'');
    for suffix in ["ing", "ed", "es", "s"] {
        if let Some(root) = word.strip_suffix(suffix)
            && root.len() > 3
        {
            return root;
        }
    }
    word
}

fn earlier_line<'a>(thing: &[&str], did: &str, memory: &'a Memory) -> Option<&'a str> {
    let mut fallback = None;
    for turn in memory.turns.iter().rev() {
        let typed = turn.user.trim_matches(is_space);
        if typed.is_empty() || typed.ends_with('?') {
            continue;
        }
        let lower = typed.to_lowercase();
        let tokens: Vec<_> = WORD
            .find_iter(&lower)
            .map(|token| stem(token.as_str()))
            .collect();
        if !thing.iter().all(|word| tokens.contains(word)) || subject(&flattened(typed)).is_some() {
            continue;
        }
        if tokens.contains(&did) {
            return Some(typed);
        }
        fallback.get_or_insert(typed);
    }
    fallback
}

#[cfg(test)]
#[path = "thing_i_did_with_tests.rs"]
mod tests;
