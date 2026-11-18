//! Listening contexts, instrument facts and the music glossary.
#[path = "music_data.rs"]
mod data;
#[cfg(test)]
#[path = "music_tests.rs"]
mod tests;

use crate::bot::Reading;
use std::borrow::Cow;

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    ListenWhen,
    GenreStart,
    InTune,
    Term,
    Count,
    Practice,
}

struct ListenRule {
    keys: &'static [&'static str],
    other: &'static [&'static str],
    exclude: Option<&'static str>,
    reply: &'static str,
}

const GENRE_ORDER: &[&str] = &[
    "hip hop",
    "k-pop",
    "kpop",
    "r&b",
    "house music",
    "dance music",
    "electronic",
    "techno",
    "classical",
    "country",
    "metal",
    "motown",
    "soul",
    "blues",
    "folk",
    "reggae",
    "opera",
    "punk",
    "indie",
    "gospel",
    "disco",
    "jazz",
    "rap",
    "house",
];
const COMPARING: &[&str] = &[
    "difference",
    "different",
    "differ",
    " vs ",
    "versus",
    "compared",
    "compare",
];
const INSTRUMENTS: &[(&str, &str)] = &[
    ("piano", "piano"),
    ("keyboard", "piano"),
    ("guitar", "guitar"),
    ("bass", "bass"),
    ("ukulele", "ukulele"),
    ("violin", "violin"),
    ("fiddle", "violin"),
    ("cello", "violin"),
    ("banjo", "guitar"),
    ("mandolin", "guitar"),
];
const BRIDGE_WORDS: &[&str] = &[
    "song",
    "songs",
    "chorus",
    "verse",
    "music",
    "musical",
    "track",
    "tune",
    "album",
    "band",
    "bridge of the song",
    "in the song",
];

fn one_of(line: &str, words: &[&str]) -> bool {
    words.iter().any(|word| line.contains(word))
}

fn either(line: &str, terms: &[&str]) -> bool {
    terms.is_empty() || one_of(line, terms)
}

fn get<T: Copy>(table: &[(&str, T)], name: &str) -> Option<T> {
    table
        .iter()
        .find_map(|(key, value)| (*key == name).then_some(*value))
}

fn listen(clean: &str) -> Option<&'static str> {
    if clean.contains("podcast") {
        let flat = format!(
            " {} ",
            clean.split_whitespace().collect::<Vec<_>>().join(" ")
        );
        if one_of(&flat, &[" about ", " on "]) {
            return None;
        }
    }
    data::LISTEN
        .iter()
        .find(|rule| {
            either(clean, rule.keys)
                && either(clean, rule.other)
                && rule.exclude.is_none_or(|word| !clean.contains(word))
        })
        .map(|rule| rule.reply)
}

fn word_char(letter: char) -> bool {
    letter.is_alphanumeric() || letter == '_'
}

fn everyday(genre: &str, typed: &str) -> bool {
    data::ORDINARY_GENRES.contains(&genre)
        && typed.match_indices(genre).any(|(at, name)| {
            let before = &typed[..at];
            let after = &typed[at + name.len()..];
            if before.chars().next_back().is_some_and(word_char)
                || after.chars().next().is_some_and(word_char)
            {
                return false;
            }
            let punctuation = [',', '.', '!', '?', ';', ':'];
            let next = after
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches(punctuation);
            if data::GENRE_WORDS_AFTER.contains(&next) {
                return false;
            }
            if before.split_whitespace().next_back().is_some_and(|word| {
                data::GENRE_NAMES_BEFORE.contains(&word.trim_matches(punctuation))
            }) {
                return true;
            }
            !next.is_empty()
                && !["and", "or", "vs", "versus", "is"].contains(&next)
                && !data::GENRE_NOT_DESCRIBED_BY.contains(&next)
        })
}

fn genre(clean: &str, norm: &str) -> Option<&'static str> {
    let said = format!("{clean} {norm}");
    if one_of(&said, &["countryside", "village", "walk in the country"]) {
        return None;
    }
    if one_of(&said, COMPARING)
        && let Some((_, difference)) = data::MUSIC_PAIRS
            .iter()
            .find(|(pair, _)| pair.iter().all(|name| said.contains(name)))
    {
        return Some(*difference);
    }
    let (name, answer) = GENRE_ORDER.iter().find_map(|name| {
        said.contains(name)
            .then(|| get(data::GENRE_START, name).map(|reply| (*name, reply)))
            .flatten()
    })?;
    (!everyday(name, if clean.is_empty() { norm } else { clean })).then_some(answer)
}

fn instrument_tune(clean: &str, norm: &str) -> Option<&'static str> {
    let said = format!("{clean} {norm}");
    let (_, name) = INSTRUMENTS.iter().find(|(key, _)| said.contains(key))?;
    get(data::IN_TUNE, name)
}

fn possessed(words: &[String], at: usize) -> bool {
    let Some(before) = at.checked_sub(1).map(|index| words[index].as_str()) else {
        return false;
    };
    let two = at
        .checked_sub(2)
        .map_or_else(String::new, |first| words[first..at].join(" "));
    if !data::POSSESSIVE.contains(&before) && !data::POSSESSIVE.contains(&two.as_str()) {
        return false;
    }
    if before == "your" || two.ends_with("your") {
        return words
            .iter()
            .any(|word| data::LOOKING_FOR_THE_THING.contains(&word.as_str()));
    }
    true
}

pub(super) fn term_context(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    let flat = words.join(" ");
    for &(term, _) in data::MUSIC_TERMS {
        let plural = format!("{term}s");
        let mut matches = words
            .iter()
            .enumerate()
            .filter(|(_, word)| *word == term || *word == &plural)
            .peekable();
        if matches.peek().is_none() && !(term.contains(' ') && flat.contains(term)) {
            continue;
        }
        if data::A_THING_OF_THEIRS.contains(&term)
            && matches.any(|(at, _)| possessed(words, at))
            && !data::A_MUSIC_WORD
                .iter()
                .any(|word| *word != term && *word != plural && flat.contains(word))
        {
            return false;
        }
        if data::NEEDS_A_MUSIC_WORD.contains(&term) && !one_of(&flat, data::A_MUSIC_WORD) {
            return false;
        }
        return get(data::MUSIC_ELSEWHERE, term).is_none_or(|cues| !one_of(&flat, cues));
    }
    true
}

pub(super) fn bridge_context(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    let low = format!(" {clean} ");
    BRIDGE_WORDS
        .iter()
        .any(|word| low.contains(&format!(" {word} ")))
}

fn term(words: &[String], flat: &str) -> &'static str {
    data::MUSIC_TERMS
        .iter()
        .find_map(|&(word, reply)| {
            let named = (word.contains(' ') && flat.contains(word))
                || words.iter().any(|said| said == word);
            (named && get(data::MUSIC_WORD_BESIDE, word).is_none_or(|cues| one_of(flat, cues)))
                .then_some(reply)
        })
        .unwrap_or(data::DEFAULT_TERM)
}

fn count(flat: &str) -> &'static str {
    data::INSTRUMENT_COUNTS
        .iter()
        .find_map(|(word, reply)| flat.contains(word).then_some(*reply))
        .unwrap_or(data::DEFAULT_COUNT)
}

pub(super) fn reply(kind: Reply, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    let norm = &reading.line.norm_text;
    let words = &reading.line.words;
    let said = match kind {
        Reply::ListenWhen => listen(clean)?,
        Reply::GenreStart => genre(clean, norm)?,
        Reply::InTune => instrument_tune(clean, norm)?,
        Reply::Term => term(words, &words.join(" ")),
        Reply::Count => count(&words.join(" ")),
        Reply::Practice => {
            if one_of(&words.join(" "), data::READ_MUSIC_WORDS) {
                return Some(Cow::Owned(format!(
                    "{} {}",
                    data::PRACTICE_READ_MUSIC,
                    data::PRACTICE_CORE
                )));
            }
            data::PRACTICE_CORE
        }
    };
    Some(Cow::Borrowed(said))
}
