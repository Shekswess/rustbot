//! Follow a short status update about somebody in a place of care.

use super::{flat, mentions};
use crate::bot::{Memory, compose};

#[path = "chat_someone_status_data.rs"]
mod data;
use data::*;

pattern!(WITH_NAME, r"\bwith\s+(?:my\s+|our\s+|the\s+)?([a-z']+)");
pattern!(SELF_IN, r"\bi(?:'?m| am)\b.*\bin\b|^im\b");

enum Patient<'a> {
    User,
    Unnamed,
    Woman(&'a str),
    Man(&'a str),
}

fn contains(table: &[&str], word: &str) -> bool {
    table.binary_search(&word).is_ok()
}

fn named(word: &str) -> Option<Patient<'_>> {
    let word = word.strip_suffix("'s").unwrap_or(word);
    let bare = word.strip_suffix('s').unwrap_or(word);
    let known = |word| contains(SHE, word) || contains(HE, word);
    let word = if known(bare) { bare } else { word };
    if contains(SHE, word) {
        return Some(Patient::Woman(word));
    }
    contains(HE, word).then_some(Patient::Man(word))
}

fn care_patient<'a>(line: &str, words: &'a [&'a str]) -> Patient<'a> {
    if let Ok(Some(matched)) = WITH_NAME.captures(line) {
        let with = matched.get(1).unwrap().as_str();
        if let Some(word) = words
            .iter()
            .find(|word| **word == with)
            .and_then(|word| named(word))
        {
            return word;
        }
        let with = with.trim_end_matches(['\'', 's']);
        if contains(A_PERSON, with)
            || contains(A_PERSON, with.trim_end_matches('s'))
            || contains(PEOPLE_AGED, with)
        {
            return Patient::Unnamed;
        }
    }
    if mentions(&SELF_IN, line) {
        return Patient::User;
    }
    words
        .iter()
        .find_map(|word| named(word))
        .unwrap_or(Patient::Unnamed)
}

fn choose(patient: Patient<'_>, memory: &Memory) -> String {
    let (word, subj, obj) = match patient {
        Patient::User => return compose::pick(IN_CARE_SELF, memory, "").to_owned(),
        Patient::Unnamed => return compose::pick(IN_CARE_ASK_UNNAMED, memory, "").to_owned(),
        Patient::Woman(word) => (word, "she", "her"),
        Patient::Man(word) => (word, "he", "him"),
    };
    let options: Vec<_> = IN_CARE_ASK
        .iter()
        .map(|line| {
            line.replace("{subj}", subj)
                .replace("{obj}", obj)
                .replace("{word}", word)
        })
        .filter(|line| !line.contains(['{', '}']))
        .collect();
    let options: Vec<_> = options.iter().map(String::as_str).collect();
    let pool = if options.is_empty() {
        IN_CARE_ASK_UNNAMED
    } else {
        &options
    };
    compose::pick(pool, memory, "").to_owned()
}

pub(super) fn reply(raw: &str, memory: &Memory) -> Option<String> {
    let clean = flat(raw);
    let words: Vec<_> = WORD
        .find_iter(&clean)
        .flatten()
        .map(|word| word.as_str())
        .take(7)
        .collect();
    let at = words.iter().position(|word| *word == "still")?;
    let after = &words[at + 1..];
    if words.len() > 6
        || after.is_empty()
        || !after
            .iter()
            .all(|word| contains(STILL_ABOUT_A_PERSON, word))
        || !words[..at].iter().all(|word| contains(POINTING, word))
    {
        return None;
    }
    for turn in memory
        .turns
        .iter()
        .rev()
        .take(6)
        .filter(|turn| !turn.user.is_empty())
    {
        let clean = flat(&turn.user);
        let words: Vec<_> = WORD
            .find_iter(&clean)
            .flatten()
            .map(|word| word.as_str())
            .collect();
        let from = words.join(" ");
        if TRADES.iter().any(|term| {
            words.contains(term)
                || if term.contains(' ') {
                    from.contains(term)
                } else {
                    term.chars().count() >= 5 && words.iter().any(|word| word.starts_with(term))
                }
        }) {
            return None;
        }
        if mentions(&IN_CARE, &from) {
            return Some(choose(care_patient(&from, &words), memory));
        }
    }
    None
}
