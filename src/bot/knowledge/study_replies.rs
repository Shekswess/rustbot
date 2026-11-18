//! Phrasebook replies and study explanations selected from the line itself.

use crate::bot::Reading;
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

#[path = "study_replies_data.rs"]
mod data;

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static LANGUAGE: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::LANGUAGES.iter().map(|&(_, pattern, _)| pattern))
        .expect("phrasebook languages")
});
static PHRASE_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::PHRASE_ASK).expect("phrase request"));
static MEANING_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::A_MEANING_IS_ASKED).expect("translation request"));
static MEANING_FRAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::MEANING_FRAME).expect("translation frame"));

pub(crate) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    let said = typed_or_clean(reading, true).to_lowercase();
    match id {
        "say-language" | "say-welsh-irish" | "say-language-phrase" => {
            let said = collapsed(&said);
            PHRASE_ASK.is_match(&said) || concept(&said).is_some() || language_alone(&said)
        }
        "what-does-that-mean" => {
            if !MEANING_FRAME.is_match(&said) {
                return false;
            }
            let mut named = native_in(&said).peekable();
            named.peek().is_some()
                && (named.any(|(word, _)| !data::ENGLISH_TOO.contains(&word))
                    || LANGUAGE.is_match(&said)
                    || said.contains(['"', '“']))
        }
        "essay-conclusion" => {
            let said = collapsed(&said);
            data::A_CONCLUSION_SAID
                .iter()
                .chain(data::SOMETHING_WRITTEN)
                .any(|word| named(&said, word, u8::is_ascii_alphanumeric))
        }
        _ => false,
    }
}

pub(crate) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = match id {
        "say-language" | "say-welsh-irish" | "say-language-phrase" => {
            phrasebook(&reading.line.clean.to_lowercase())?
        }
        "what-does-that-mean" => {
            let said = typed_or_clean(reading, true).to_lowercase();
            if !translation_asked(reading, &said) {
                return None;
            }
            native_in(&reading.line.clean.to_lowercase()).next()?.1
        }
        "past-papers" => past_papers(reading),
        "fractions-sums" => fractions(reading),
        "memorise-faster" => memorise(reading)?,
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}

fn typed_or_clean<'a>(reading: &'a Reading<'_>, clean_first: bool) -> &'a str {
    let line = &reading.line;
    let preferred = if clean_first { &line.clean } else { &line.text };
    if preferred.is_empty() {
        if clean_first { &line.text } else { &line.clean }
    } else {
        preferred
    }
}

fn collapsed(text: &str) -> String {
    text.split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn named(line: &str, target: &str, inside: fn(&u8) -> bool) -> bool {
    let bytes = line.as_bytes();
    line.match_indices(target).any(|(start, found)| {
        !start
            .checked_sub(1)
            .and_then(|offset| bytes.get(offset))
            .is_some_and(inside)
            && !bytes.get(start + found.len()).is_some_and(inside)
    })
}

fn concept(line: &str) -> Option<&'static str> {
    data::CONCEPTS.iter().find_map(|&(alternatives, concept)| {
        alternatives
            .iter()
            .any(|word| named(line, word, u8::is_ascii_lowercase))
            .then_some(concept)
    })
}

fn native_in(line: &str) -> impl Iterator<Item = (&'static str, &'static str)> {
    data::NATIVE
        .iter()
        .copied()
        .filter(|&(word, _)| named(line, word, u8::is_ascii_lowercase))
}

fn language_alone(line: &str) -> bool {
    let mut words = WORD
        .find_iter(line)
        .map(|item| item.as_str())
        .filter(|word| !data::CORRECTION_FURNITURE.contains(word))
        .peekable();
    words.peek().is_some() && words.all(|word| data::LANGUAGE_NAMES.contains(&word))
}

fn translation_asked(reading: &Reading<'_>, clean: &str) -> bool {
    if MEANING_ASK.is_match(clean) {
        return true;
    }
    static SAY: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\bsay\b|\bhow do you\b|\bhow do i\b").unwrap());
    let mut content = reading
        .line
        .words
        .iter()
        .map(|word| word.to_lowercase())
        .filter(|word| !data::ENGLISH_FURNITURE.contains(&word.as_str()))
        .peekable();
    let is_native = |word: &str| data::NATIVE.iter().any(|&(entry, _)| entry == word);
    let foreign = data::LANGUAGE_NAMES
        .iter()
        .any(|word| named(clean, word, u8::is_ascii_lowercase));
    (foreign && (SAY.is_match(clean) || content.clone().any(|word| is_native(&word))))
        || (content.peek().is_some() && content.all(|word| is_native(&word)))
}

fn phrasebook(clean: &str) -> Option<&'static str> {
    const PLATE: &[&str] = &[
        "food",
        "foods",
        "cuisine",
        "dish",
        "dishes",
        "eat",
        "eating",
        "meal",
        "meals",
        "restaurant",
        "restaurants",
    ];
    const SAY: &[&str] = &[
        "say",
        "says",
        "saying",
        "pronounce",
        "pronounced",
        "translate",
        "translated",
        "translation",
        "mean",
        "means",
        "how do you say",
    ];
    let mentions = |options: &[&str]| {
        options
            .iter()
            .any(|word| named(clean, word, u8::is_ascii_lowercase))
    };
    if mentions(PLATE) && !mentions(SAY) {
        return None;
    }
    let index = LANGUAGE.matches(clean).into_iter().next()?;
    let (language, _, default) = data::LANGUAGES[index];
    Some(
        concept(clean)
            .and_then(|concept| {
                data::PHRASES
                    .binary_search_by(|&(one, other, _)| (one, other).cmp(&(concept, language)))
                    .ok()
                    .map(|index| data::PHRASES[index].2)
            })
            .unwrap_or(default),
    )
}

fn words_for_study(reading: &Reading<'_>) -> String {
    if reading.line.text.is_empty() {
        reading.line.norm.join(" ").to_lowercase()
    } else {
        reading.line.text.to_lowercase()
    }
}

fn past_papers(reading: &Reading<'_>) -> &'static str {
    let line = words_for_study(reading);
    let words: Vec<_> = WORD.find_iter(&line).map(|word| word.as_str()).collect();
    let wants_timing = words
        .iter()
        .any(|word| data::PAST_PAPERS_ASK_WHEN.contains(word))
        && (words
            .iter()
            .any(|word| ["when", "should", "start", "begin", "soon"].contains(word))
            || line.contains("how long")
            || line.contains("how many"));
    if wants_timing {
        data::PAST_PAPERS_WHEN
    } else {
        data::PAST_PAPERS
    }
}

fn fractions(reading: &Reading<'_>) -> &'static str {
    let line = words_for_study(reading);
    let words: Vec<_> = WORD.find_iter(&line).map(|word| word.as_str()).collect();
    data::FRACTIONS_FIRST
        .iter()
        .find_map(|&(asking, reply)| {
            words
                .iter()
                .any(|word| asking.contains(word))
                .then_some(reply)
        })
        .unwrap_or(data::FRACTIONS_MULTIPLY)
}

fn memorise(reading: &Reading<'_>) -> Option<&'static str> {
    let line = &reading.line;
    let words = if line.norm.is_empty() {
        line.plain
            .chars()
            .map(|word| word.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        line.norm.join(" ")
    };
    let said = format!(" {} ", words.to_lowercase());
    data::REMEMBERS
        .iter()
        .any(|word| said.contains(word))
        .then_some(data::MEMORISE)
}

#[cfg(test)]
#[path = "../../../tests/fixtures/study_replies.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_study_and_phrasebook_replies_match_pybot() {
        let mut differences = Vec::new();
        for &fixtures::Case(id, raw, guard, reply) in fixtures::CASES {
            let state = Reading::new(raw);
            let actual = registered(id, &state);
            if guard.is_some_and(|expected| passes(id, &state) != expected)
                || actual.as_deref() != reply
            {
                differences.push((id, raw, guard, passes(id, &state), reply, actual));
            }
        }
        assert!(differences.is_empty(), "{differences:#?}");
    }
}
