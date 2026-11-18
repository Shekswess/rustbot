//! A setback named without the details that would explain it.

use fancy_regex::Regex;
use std::sync::LazyLock;

#[path = "trouble_setback_data.rs"]
mod data;
use data::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tense {
    Past,
    Now,
}

fn owner_phrase(word: &str, after: &str, owner: &str) -> Option<String> {
    if NOT_A_THING.binary_search(&word).is_ok() {
        return None;
    }
    let after = if JOINERS.binary_search(&after).is_ok() {
        ""
    } else {
        after
    };
    Some(if after.is_empty() {
        format!("{owner} {word}")
    } else {
        format!("{owner} {word} {after}")
    })
}

fn thing(text: &str) -> Option<String> {
    let last = OWNER.captures_iter(text).flatten().last()?;
    let qualifier = last.get(1)?.as_str();
    let word = last.get(2)?.as_str();
    let after = last.get(3).map_or("", |m| m.as_str());
    let owner = if YOURS.contains(&qualifier) {
        "your"
    } else {
        "the"
    };
    if matches!(word, "whole" | "entire") && !after.is_empty() {
        owner_phrase(after, "", owner)
    } else if MODIFIER.binary_search(&word).is_ok() && !after.is_empty() {
        owner_phrase(&format!("{word} {after}"), "", owner)
    } else {
        owner_phrase(word, after, owner)
    }
}

fn named_after(text: &str) -> Option<String> {
    static NAME: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(?:the|my|our|this|that)\s+(?:whole\s+)?([a-z][a-z'-]{1,18})(?:\s+([a-z][a-z'-]{1,18}))?").unwrap()
    });
    let found = NAME.captures(text).ok().flatten()?;
    let word = found.get(1)?.as_str();
    let after = found.get(2).map_or("", |m| m.as_str());
    if MODIFIER.binary_search(&word).is_ok() && !after.is_empty() {
        owner_phrase(&format!("{word} {after}"), "", "your")
    } else {
        owner_phrase(word, after, "your")
    }
}

fn bare_subject(head: &str) -> Option<String> {
    let word = super::split(head)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .find(|word| {
            !matches!(
                *word,
                "is" | "are" | "was" | "were" | "been" | "has" | "have" | "had" | "really" | "just"
            )
        })?;
    (NOT_A_THING.binary_search(&word).is_err()
        && !matches!(
            word,
            "i" | "im" | "it" | "that" | "this" | "there" | "he" | "she" | "they" | "we" | "you"
        ))
    .then(|| word.to_owned())
}

fn read(text: &str) -> Option<(String, Tense)> {
    let clean = super::flat(&text.replace('’', "'"));
    if clean.is_empty() || clean.contains('?') {
        return None;
    }
    for (pattern, tense) in [
        (&*VAGUE, Tense::Past),
        (&*NOW, Tense::Now),
        (&*BLAME, Tense::Past),
    ] {
        let Some(found) = pattern.find(&clean).ok().flatten() else {
            continue;
        };
        let tail = HELPLESS.replace_all(&clean[found.end()..], "");
        let rest: Vec<_> = super::split(&tail).collect();
        if rest.len() > MORE_THAN
            || rest.iter().any(|word| {
                GOES_ON
                    .binary_search(&word.trim_matches(['.', ',', '!']))
                    .is_ok()
            })
        {
            return None;
        }
        let name = thing(&clean[..found.start()])
            .or_else(|| {
                if std::ptr::eq(pattern, &*BLAME) {
                    named_after(&clean[found.end()..])
                } else if tense == Tense::Now {
                    bare_subject(&clean[..found.start()])
                } else {
                    None
                }
            })
            .unwrap_or_default();
        if super::split(&name)
            .last()
            .is_some_and(|word| PEOPLE.binary_search(&word).is_ok())
        {
            return None;
        }
        return Some((name, tense));
    }
    None
}

pub(super) fn recognizes(text: &str) -> bool {
    read(text).is_some()
}

pub(super) fn pool(text: &str) -> Vec<String> {
    let Some((name, tense)) = read(text) else {
        return Vec::new();
    };
    let word = super::split(&name).last().unwrap_or("");
    let pool = match (tense, word) {
        (Tense::Now, "") => WHOLE_NOW,
        (Tense::Now, _) => ONGOING,
        (Tense::Past, "") => WHOLE,
        (Tense::Past, word) if NOT_A_DOER.binary_search(&word).is_ok() => ABOUT_OCCASION,
        _ => ABOUT_THING,
    };
    let plural = word.ends_with('s')
        && !["ss", "us", "is", "ws"]
            .iter()
            .any(|ending| word.ends_with(ending));
    pool.iter()
        .map(|line| {
            let reply = line.replace("{thing}", &name);
            if tense == Tense::Now && plural {
                reply
                    .replace(" sounds ", " sound ")
                    .replace(" it sounds ", " they sound ")
            } else {
                reply
            }
        })
        .collect()
}
