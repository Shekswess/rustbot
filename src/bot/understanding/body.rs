//! Read the part of the body the speaker says is unwell.

use fancy_regex::Regex as ContextRegex;
use regex::Regex;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::sync::LazyLock;

#[path = "body_data.rs"]
mod data;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Part {
    Head,
    Throat,
    Temperature,
    Stomach,
    Chest,
    Teeth,
    Ear,
}

macro_rules! pattern {
    ($name:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));
    };
}

pattern!(AN_ABSCESS);
pattern!(A_MOUTH_WORD);
pattern!(A_SPREADING_SIGN);
pattern!(ANOTHER_PART);
pattern!(ALREADY_DEALT_WITH);
pattern!(A_SAMPLE);
pattern!(ASKING);
pattern!(A_LOOSE_STOOL);
pattern!(A_THING_IS_TERRIBLE);
pattern!(A_CHAIR_WORD);
pattern!(A_GUT_ONLY);
pattern!(A_POO_COLOUR);
pattern!(SPEAKER_ILL);
pattern!(SPEAKER_TAIL);

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static LETTERS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b[a-z]+\b").unwrap());
static SPEAKER_SHORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:i'?m|i'?ve|we'?re|we'?ve)\b").unwrap());

struct Complaint {
    part: Part,
    pattern: ContextRegex,
    nouns: &'static [&'static str],
}

static COMPLAINTS: LazyLock<Vec<Complaint>> = LazyLock::new(|| {
    data::PARTS
        .iter()
        .map(|&(part, pattern, nouns)| Complaint {
            part,
            pattern: ContextRegex::new(pattern).expect("body pattern"),
            nouns,
        })
        .collect()
});

fn has(words: &[&str], word: &str) -> bool {
    words.binary_search(&word).is_ok()
}

/// Find the first matching complaint about the speaker in a normalized line.
pub(crate) fn read(text: &str) -> Option<Part> {
    if text.trim().is_empty() {
        return None;
    }
    let without_boundaries = if text.contains('|') {
        Cow::Owned(text.replace('|', " "))
    } else {
        Cow::Borrowed(text)
    };
    let text = spell_parts(&without_boundaries);
    let text = text.as_ref();
    let sample = A_SAMPLE.is_match(text);
    let other = someone_else(text);
    if A_LOOSE_STOOL.is_match(text)
        && !sample
        && !A_POO_COLOUR.is_match(text)
        && !A_CHAIR_WORD.is_match(text)
        && !A_THING_IS_TERRIBLE.is_match(text)
        && !other
        && gut_is_settled(text)
    {
        return Some(Part::Stomach);
    }
    let asking = !other && ASKING.is_match(text);
    for complaint in &*COMPLAINTS {
        if complaint.part == Part::Teeth && abscess_elsewhere(text)
            || complaint.part == Part::Stomach && sample
        {
            continue;
        }
        for found in complaint.pattern.find_iter(text).flatten() {
            if belongs_to_speaker(text, found.end(), found.start(), complaint.nouns)
                || asking
                || !other && just_the_complaint(text, found.start(), found.end())
            {
                return Some(complaint.part);
            }
        }
    }
    said_about_itself(text, other)
}

fn abscess_elsewhere(text: &str) -> bool {
    AN_ABSCESS.is_match(text)
        && (A_MOUTH_WORD.is_match(text)
            || A_SPREADING_SIGN.is_match(text)
            || ANOTHER_PART.is_match(text)
            || ALREADY_DEALT_WITH.is_match(text))
}

fn someone_else(text: &str) -> bool {
    WORD.find_iter(text).any(|word| {
        has(data::OTHER_PEOPLE, word.as_str())
            || has(data::OTHER_PEOPLE, word.as_str().trim_end_matches('s'))
    })
}

fn gut_is_settled(text: &str) -> bool {
    let lower = text.to_lowercase();
    let words: BTreeSet<_> = WORD.find_iter(&lower).map(|found| found.as_str()).collect();
    !words.contains("stool") && !words.contains("stools")
        || A_GUT_ONLY
            .find_iter(&lower)
            .any(|clue| words.contains(clue.as_str()))
}

fn one_letter_apart(first: &str, second: &str) -> bool {
    match first.len().abs_diff(second.len()) {
        0 => {
            first
                .bytes()
                .zip(second.bytes())
                .filter(|(a, b)| a != b)
                .count()
                == 1
        }
        1 => {
            let (short, long) = if first.len() < second.len() {
                (first.as_bytes(), second.as_bytes())
            } else {
                (second.as_bytes(), first.as_bytes())
            };
            let differing = short
                .iter()
                .zip(long)
                .position(|(a, b)| a != b)
                .unwrap_or(short.len());
            short[differing..] == long[differing + 1..]
        }
        _ => false,
    }
}

fn spelling(word: &str) -> Option<&'static str> {
    if word.len() < 4
        || super::line::lexical_word(word)
        || has(data::A_WORD_OF_ITS_OWN, word)
        || super::topics::is_condition(word)
    {
        return None;
    }
    let mut parts = data::PART_NOUNS
        .iter()
        .copied()
        .filter(|part| part.as_bytes()[0] == word.as_bytes()[0] && one_letter_apart(part, word));
    parts.next().filter(|_| parts.next().is_none())
}

fn spell_parts(text: &str) -> Cow<'_, str> {
    let mut written = String::new();
    let mut end = 0;
    for matched in LETTERS.find_iter(text) {
        if let Some(part) = spelling(matched.as_str()) {
            written.push_str(&text[end..matched.start()]);
            written.push_str(part);
            end = matched.end();
        }
    }
    if end == 0 {
        Cow::Borrowed(text)
    } else {
        written.push_str(&text[end..]);
        Cow::Owned(written)
    }
}

fn spell_speaker(text: &str, plural: bool) -> Cow<'_, str> {
    SPEAKER_SHORT.replace_all(text, |captures: &regex::Captures<'_>| match &captures[0] {
        "im" | "i'm" => "i am",
        "ive" | "i've" => "i have",
        "were" | "we're" if plural => "we are",
        "weve" | "we've" if plural => "we have",
        "were" => "were",
        "we're" => "we're",
        "weve" => "weve",
        _ => "we've",
    })
}

fn belongs_to_speaker(text: &str, at: usize, began: usize, nouns: &[&str]) -> bool {
    let words: Vec<_> = WORD
        .find_iter(&text[..at])
        .map(|found| found.as_str())
        .collect();
    if let Some((index, &owner)) = words
        .iter()
        .enumerate()
        .rev()
        .find(|(_, word)| **word == "my" || has(data::OTHER_PEOPLE, word))
    {
        return owner == "my" && words.get(index + 1).is_some_and(|part| has(nouns, part));
    }
    let mut start = words.len().saturating_sub(6);
    while start > 0 && has(data::A_VERB_NEEDING_ITS_SUBJECT, words[start]) {
        start -= 1;
    }
    let tail = words[start..].join(" ");
    if SPEAKER_ILL.is_match(&spell_speaker(&tail, true)) || speaker_before(&text[..at]) {
        return true;
    }
    let began = WORD.find_iter(&text[..began]).count();
    words.iter().enumerate().rev().any(|(at, &word)| {
        matches!(word, "i" | "we") && only_fillers(words.get(at + 1..began).unwrap_or(&[]))
    })
}

fn only_fillers(mut words: &[&str]) -> bool {
    if words.first().is_some_and(|word| has(data::A_MAYBE, word)) {
        if !words.get(1).is_some_and(|word| has(data::A_NO, word)) {
            return false;
        }
        words = &words[1..];
    }
    words
        .iter()
        .all(|word| has(data::BEFORE_THE_COMPLAINT, word))
}

pub(super) fn speaker_before(text: &str) -> bool {
    let words: Vec<_> = text
        .split_whitespace()
        .map(|word| {
            word.trim_matches(['.', ',', '!', '?', ';', ':', '-', '|', '"', '\''])
                .to_lowercase()
        })
        .collect();
    let end = words
        .iter()
        .rposition(|word| !has(data::FEELING_FILLER, word))
        .map_or(0, |at| at + 1);
    let tail = words[end.saturating_sub(5)..end].join(" ");
    if SPEAKER_TAIL.is_match(&spell_speaker(&tail, false)) {
        return true;
    }
    let recent = &words[end.saturating_sub(4)..end];
    recent
        .iter()
        .position(|word| word == "my")
        .is_some_and(|at| {
            recent[at + 1..]
                .iter()
                .any(|word| has(data::MOOD_NOUNS, word))
        })
}

fn just_the_complaint(text: &str, start: usize, end: usize) -> bool {
    WORD.find_iter(&text[..start])
        .chain(WORD.find_iter(&text[end..]))
        .all(|word| {
            let word = word.as_str();
            has(data::FILLER, word)
                || has(data::WHEN_IT_HAPPENED, word)
                || has(data::A_HELPER_BEFORE, word)
        })
}

fn part_noun(word: &str, nouns: &[&str]) -> bool {
    has(nouns, word)
        || word
            .strip_suffix("'s")
            .or_else(|| word.strip_suffix('s'))
            .is_some_and(|part| has(nouns, part))
}

fn unwell(words: &[&str], at: usize) -> bool {
    let word = words[at];
    has(data::PLAIN_PAIN, word)
        || has(data::PLAIN_UNWELL, word)
            && (word != "off"
                || words
                    .get(at + 1)
                    .is_none_or(|next| has(data::OFF_MAY_GO_ON, next)))
}

fn pain_after(words: &[&str], index: usize) -> bool {
    const PAIN_REACH: usize = 3;
    for at in index + 1..=index + PAIN_REACH {
        let Some(&word) = words.get(at) else {
            return false;
        };
        if unwell(words, at) {
            if word == "off" && !(index > 0 && has(data::POSSESSIVE, words[index - 1])) {
                break;
            }
            return true;
        }
        if !has(data::PAIN_BETWEEN, word) {
            break;
        }
    }
    data::PAIN_PHRASE.iter().any(|phrase| {
        let mut at = index + 1;
        phrase.iter().all(|component| {
            if words
                .get(at)
                .is_some_and(|word| has(data::PAIN_BETWEEN, word))
            {
                at += 1;
            }
            if words.get(at) != Some(component) {
                return false;
            }
            at += 1;
            true
        })
    })
}

fn said_about_itself(text: &str, other: bool) -> Option<Part> {
    let spans: Vec<_> = WORD.find_iter(text).collect();
    let words: Vec<_> = spans.iter().map(|span| span.as_str()).collect();
    for (index, &word) in words.iter().enumerate() {
        let before = index.checked_sub(1).map(|index| words[index]);
        for &(part, _, nouns) in data::PARTS {
            if matches!(part, Part::Chest | Part::Temperature)
                || word == "jaw"
                || !part_noun(word, nouns)
            {
                continue;
            }
            let adjective_first = before.is_some_and(|word| has(data::PAIN_ADJECTIVE, word));
            let follows = !adjective_first && pain_after(&words, index);
            if !adjective_first && !follows {
                continue;
            }
            if belongs_to_speaker(text, spans[index].end(), spans[index].start(), nouns)
                || matches!(before, Some("my" | "our" | "me"))
                || index == 0 && follows
            {
                return Some(part);
            }
            if !other
                && (adjective_first
                    || words[..index]
                        .iter()
                        .chain(&words[index + 1..])
                        .all(|word| {
                            [
                                data::FILLER,
                                data::PLAIN_PAIN,
                                data::PLAIN_UNWELL,
                                data::PAIN_BETWEEN,
                            ]
                            .iter()
                            .any(|set| has(set, word))
                        }))
            {
                return Some(part);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{Part, read};

    #[test]
    fn assigns_complaints_to_the_speaker_only() {
        assert_eq!(read("my throat is hurting"), Some(Part::Throat));
        assert_eq!(read("she gets migraines"), None);
        assert_eq!(read("my son has a fever"), None);
    }

    #[test]
    fn recognizes_only_cold_to_hot_alternation_in_one_subject() {
        assert_eq!(read("i am hot and | i am cold"), Some(Part::Temperature));
        assert_eq!(read("i am hot and the pool is cold"), None);
        assert_eq!(read("hay fever"), None);
    }

    #[test]
    fn respects_nonmedical_stools_and_literal_samples() {
        assert_eq!(read("loose stools since monday"), Some(Part::Stomach));
        assert_eq!(read("my bar stool is loose"), None);
        assert_eq!(read("my loose stool sample"), None);
    }

    #[test]
    fn retains_spelling_repairs_and_abscess_guards() {
        assert_eq!(read("my stamach is killing me"), Some(Part::Stomach));
        assert_eq!(read("i think i have an abscess"), Some(Part::Teeth));
        assert_eq!(read("an abscess on my leg"), None);
    }
}
