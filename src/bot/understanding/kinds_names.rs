//! The single-word name check used for introductions following a hello.

use super::{feature::sound, has};
use crate::bot::understanding::line;

#[path = "kinds_names_data.rs"]
mod data;
use data::*;

fn state_like(word: &str) -> bool {
    if has(STATE_LIKE_NAMES, word) {
        return false;
    }
    if STATE_ENDINGS
        .iter()
        .any(|(ending, minimum)| word.len() >= *minimum && word.ends_with(ending))
    {
        return true;
    }
    if word.len() >= 5 && doubled_end(word) && has(STATE_FOR_DEGREE, &word[..word.len() - 1]) {
        return true;
    }
    for (ending, repair) in [
        ("iest", "y"),
        ("ier", "y"),
        ("iest", ""),
        ("ier", ""),
        ("est", ""),
        ("er", ""),
    ] {
        if let Some(stem) = word.strip_suffix(ending)
            && stem.len() > 2
        {
            let restored = format!("{stem}{repair}");
            if has(STATE_FOR_DEGREE, &restored)
                || doubled_end(&restored) && has(STATE_FOR_DEGREE, &restored[..restored.len() - 1])
            {
                return true;
            }
        }
    }
    false
}

fn doubled_end(word: &str) -> bool {
    matches!(word.as_bytes(), [.., a, b] if a == b)
}

fn describes(word: &str) -> bool {
    has(DESCRIPTIONS, word)
        || word.ends_with("ed")
            && word.len() > 3
            && !matches!(word, "fred" | "ed" | "ned" | "ted" | "jed")
        || DESCRIPTION_ENDINGS
            .iter()
            .any(|ending| word.len() > ending.len() + 2 && word.ends_with(ending))
}

fn judgement(word: &str) -> bool {
    has(JUDGEMENT, word)
        || ["ish", "y"].iter().any(|ending| {
            word.strip_suffix(ending).is_some_and(|stem| {
                stem.len() > 2
                    && (has(JUDGEMENT, stem)
                        || doubled_end(stem) && has(JUDGEMENT, &stem[..stem.len() - 1]))
            })
        })
}

fn is_state(word: &str) -> bool {
    word.len() >= 3 && has(STATE_FOR_COMPARISON, word)
}

fn with_a_letter_removed(word: &str) -> impl Iterator<Item = String> + '_ {
    word.as_bytes()
        .windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] == pair[1])
        .map(|(at, _)| format!("{}{}", &word[..at], &word[at + 1..]))
}

fn compared_state(word: &str) -> bool {
    ["er", "est"].into_iter().any(|ending| {
        word.strip_suffix(ending).is_some_and(|stem| {
            stem.len() >= 3
                && (is_state(stem)
                    || stem
                        .strip_suffix('i')
                        .is_some_and(|bare| is_state(&format!("{bare}y")))
                    || is_state(&format!("{stem}y"))
                    || is_state(&format!("{stem}e"))
                    || with_a_letter_removed(stem).any(|candidate| is_state(&candidate)))
        })
    })
}

/// Read a possible name from already-normalized words, e.g. Sam or Zaphod.
pub(super) fn reads_as_a_name(word: &str) -> bool {
    let word = word.trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"']);
    if !(2..=20).contains(&word.len())
        || !word.starts_with(|ch: char| ch.is_ascii_lowercase())
        || !word.ends_with(|ch: char| ch.is_ascii_lowercase())
        || !word
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || matches!(byte, b'\'' | b'-'))
        || !word.contains(['a', 'e', 'i', 'o', 'u', 'y'])
        || has(BLOCKED, word)
        || line::is_laugh(word)
        || judgement(word)
    {
        return false;
    }
    let state_like_name = has(STATE_LIKE_NAMES, word);
    if !state_like_name
        && (describes(word)
            || state_like(word)
            || line::curated_spelling(word).is_some_and(|fixed| {
                fixed
                    .split_whitespace()
                    .any(|one| one != word && has(CORE, one))
            }))
    {
        return false;
    }
    let repair = line::repaired_word(word);
    if !state_like_name
        && repair.iter().any(|fixed| {
            fixed != word && fixed.len() >= word.len() && (has(CORE, fixed) || state_like(fixed))
        })
    {
        return false;
    }
    if word.bytes().all(|letter| letter.is_ascii_lowercase())
        && (line::spells_a_state(word)
            || has(VOCAL_FEELING, word)
            || repair.iter().any(|fixed| has(SPELLED_FEELING, fixed))
            || sound(word).is_some())
    {
        return false;
    }
    !compared_state(word)
        && !with_a_letter_removed(word).any(|cut| is_state(&cut) || compared_state(&cut))
}
