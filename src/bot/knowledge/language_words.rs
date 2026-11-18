//! Word meanings, synonym sets, and regular and irregular English word forms.

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "language_words_data.rs"]
mod data;

static PAST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::PAST_OF).expect("past tense question"));
static MEANING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::A_MEANING_IS_ASKED).expect("word definition question"));

pub(super) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    match id {
        "regular-past" | "irregular-past" => {
            let said = clean(reading).to_lowercase();
            PAST.captures(&said).is_some_and(|ask| {
                pair_value(data::IRREGULAR_VERBS, &ask[1]).is_some() == (id == "irregular-past")
            })
        }
        "plural-word" => {
            let words = words_of(reading);
            words.iter().any(|word| {
                let word = word.as_str();
                pair_value(data::PLURALS, word).is_some()
                    || value(data::PLURAL_SINGULARS, word).is_some()
                    || data::UNCOUNTABLE.contains(&word)
                    || data::PLURAL_GENERAL.contains(&word)
            }) || asked_plurals(&words).next().is_some()
        }
        "everyday-word-meaning" => meaning_asked(reading),
        _ => false,
    }
}

pub(super) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "regular-past" => regular_past(reading),
        "irregular-past" => {
            let said = clean(reading).to_lowercase();
            irregular_reply(&PAST.captures(&said)?[1]).map(Cow::Owned)
        }
        "plural-word" => plural(reading),
        "word-meaning" => Some(Cow::Borrowed(
            definition(
                &reading.line.words,
                &[data::WORD_GLOSSES, data::EVERYDAY_WORDS],
            )
            .unwrap_or(data::ASK_FOR_THE_WORD),
        )),
        "everyday-word-meaning" => {
            definition(&reading.line.words, &[data::EVERYDAY_WORDS]).map(Cow::Borrowed)
        }
        "word-synonyms" => synonyms(&reading.line.words).map(Cow::Borrowed),
        _ => None,
    }
}

fn clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn value(table: &'static [(&str, &str)], key: &str) -> Option<&'static str> {
    let at = table
        .binary_search_by(|&(candidate, _)| candidate.cmp(key))
        .ok()?;
    Some(table[at].1)
}

fn pair_value(
    table: &'static [(&str, &str, &str)],
    key: &str,
) -> Option<(&'static str, &'static str)> {
    let at = table
        .binary_search_by(|&(candidate, _, _)| candidate.cmp(key))
        .ok()?;
    Some((table[at].1, table[at].2))
}

fn capitalized(word: &str) -> String {
    let mut chars = word.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars.flat_map(char::to_lowercase))
        .collect()
}

fn words_of(reading: &Reading<'_>) -> Vec<String> {
    reading
        .line
        .words
        .iter()
        .map(|word| {
            word.trim_matches(|ch: char| ch.is_ascii_punctuation())
                .to_lowercase()
        })
        .collect()
}

fn regular_past(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = clean(reading).to_lowercase();
    let found = PAST.captures(&said)?;
    let verb = &found[1];
    if let Some(reply) = irregular_reply(verb) {
        return Some(Cow::Owned(reply));
    }
    if let Some(reply) = value(data::TWO_VERBS, verb) {
        return Some(Cow::Borrowed(reply));
    }
    if verb.len() > 12 {
        return Some(Cow::Owned(format!(
            "I don't know a verb '{verb}'. If it is a name or a made-up word the -ed rule still gives {verb}ed, and if it is a typo the nearest real verb is the one to take."
        )));
    }
    Some(Cow::Owned(format!(
        "{} -> {}. Regular verbs add -ed; after an e it is just -d (hope -> hoped), a consonant and a y becomes -ied (carry -> carried), and a one-syllable verb doubles its last letter (stop -> stopped). If none of that fits, it is irregular: swim -> swam, buy -> bought.",
        capitalized(verb),
        regular_form(verb),
    )))
}

fn irregular_reply(verb: &str) -> Option<String> {
    let (past, participle) = pair_value(data::IRREGULAR_VERBS, verb)?;
    const TAIL: &str = "Most verbs just add -ed (walk, walked), so the irregular ones are the ones worth learning.";
    Some(if participle == past {
        format!(
            "The past tense of {verb} is {past}, and the past participle is the same: 'I {past} it yesterday, I have {past} it before.' {TAIL}"
        )
    } else {
        format!(
            "The past tense of {verb} is {past} - 'I {past} yesterday' - and the past participle is {participle}, the one that goes with have: 'I have {participle}'. {TAIL}"
        )
    })
}

fn regular_form(verb: &str) -> String {
    if verb.ends_with('e') {
        return format!("{verb}d");
    }
    let bytes = verb.as_bytes();
    let vowel = |letter: u8| data::VOWELS.as_bytes().contains(&letter);
    if let [.., before, b'y'] = bytes
        && !vowel(*before)
    {
        return format!("{}ied", &verb[..verb.len() - 1]);
    }
    if let [.., first, middle, end] = bytes {
        let doubles = !vowel(*first)
            && vowel(*middle)
            && !vowel(*end)
            && !b"wxy".contains(end)
            && bytes
                .split(|letter| !vowel(*letter))
                .filter(|span| !span.is_empty())
                .count()
                == 1;
        if data::DOUBLED_L.contains(&verb) || doubles {
            return format!("{verb}{}ed", char::from(*end));
        }
    }
    format!("{verb}ed")
}

fn asked_plurals(words: &[String]) -> impl Iterator<Item = &str> {
    let asked = words.iter().position(|word| {
        data::PLURAL_ASK.contains(&word.as_str()) || data::PLURAL_ASK_FRAME.contains(&word.as_str())
    });
    let after = asked.map_or(words.len(), |at| at + 1);
    words[after..].iter().map(String::as_str).filter(|&word| {
        word.chars().count() > 2
            && word.chars().all(char::is_alphabetic)
            && !data::PLURAL_NOT_THE_NOUN.contains(&word)
            && !data::PLURAL_ASK.contains(&word)
            && pair_value(data::PLURALS, word).is_none()
            && value(data::PLURAL_SINGULARS, word).is_none()
            && !data::UNCOUNTABLE.contains(&word)
            && !data::PLUS_FORMS.contains(&word)
    })
}

fn plural(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words = words_of(reading);
    for word in &words {
        let word = word.as_str();
        if data::PLURAL_GENERAL.contains(&word) {
            continue;
        }
        if let Some((more, pattern)) = pair_value(data::PLURALS, word) {
            return Some(Cow::Owned(if more == word {
                format!("{} {pattern}", own_plural(word))
            } else {
                format!("{} is the plural of {word}. {pattern}", capitalized(more))
            }));
        }
        if let Some(one) = value(data::PLURAL_SINGULARS, word) {
            let (_, pattern) =
                pair_value(data::PLURALS, one).expect("source singular has a plural");
            return Some(Cow::Owned(format!(
                "{} is already plural - one {one}, two {word}. {pattern}",
                capitalized(word)
            )));
        }
        if data::UNCOUNTABLE.contains(&word) {
            return Some(Cow::Owned(format!(
                "{} has no plural in the everyday sense: you would say some {word}, or a bit of {word}, not {word}s.",
                capitalized(word)
            )));
        }
    }
    if words
        .iter()
        .any(|word| data::PLURAL_GENERAL.contains(&word.as_str()))
    {
        return Some(Cow::Borrowed(data::PLURAL_RULE));
    }
    let mut asked = asked_plurals(&words);
    let word = asked.next()?;
    Some(if asked.all(|other| other == word) {
        ending(word)
    } else {
        Cow::Borrowed(data::PLURAL_RULE)
    })
}

fn own_plural(word: &str) -> String {
    format!(
        "{} is its own plural - the same word for one and for many: one {word}, two {word}.",
        capitalized(word)
    )
}

fn ending(word: &str) -> Cow<'static, str> {
    if data::PLURAL_SAME_WORD.contains(&word) {
        return Cow::Owned(own_plural(word));
    }
    if data::PLURAL_NO_COUNT
        .iter()
        .any(|suffix| word.ends_with(suffix))
    {
        return Cow::Owned(format!(
            "{} has no plural: it is an amount, not a count, so you would say some {word}, or a bit of {word}.",
            capitalized(word)
        ));
    }
    if data::PLURAL_NOT_A_NOUN_AT_ALL.contains(&word) {
        return Cow::Borrowed(data::PLURAL_RULE);
    }
    if let Some(root) = word.strip_suffix("us") {
        let name = capitalized(&format!("{root}uses"));
        return Cow::Owned(if let Some(other) = value(data::PLURAL_LATIN_TWIN, word) {
            format!(
                "{name} is the plural of {word}, and {other} is the Latin form plenty of people still use. {}",
                data::PLURAL_US_SENTENCE
            )
        } else {
            format!(
                "{name} is the plural of {word}. {}",
                data::PLURAL_US_SENTENCE
            )
        });
    }
    let (more, key) = if ["s", "x", "z", "ch", "sh"]
        .iter()
        .any(|suffix| word.ends_with(suffix))
    {
        let append = if word.ends_with('z') && !word.ends_with("zz") {
            "zes"
        } else {
            "es"
        };
        (format!("{word}{append}"), "es")
    } else if let Some(root) = word.strip_suffix('y')
        && root.chars().count() >= 2
        && !root.ends_with(|ch| data::VOWELS.contains(ch))
    {
        (format!("{root}ies"), "ies")
    } else if word.ends_with('o') {
        if data::PLURAL_OES.contains(&word) {
            (format!("{word}es"), "oes")
        } else {
            (format!("{word}s"), "os")
        }
    } else {
        (format!("{word}s"), "plain")
    };
    let rule = value(data::PLURAL_RULE_SENTENCE, key).expect("known source suffix");
    Cow::Owned(format!(
        "{} is the plural of {word}. {rule}",
        capitalized(&more)
    ))
}

fn meaning_asked(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    if MEANING.is_match(&line.clean) {
        return true;
    }
    if line.words.len() > 6 {
        return false;
    }
    let words: Vec<_> = line
        .words
        .iter()
        .map(|word| word.to_lowercase())
        .skip_while(|word| {
            data::WHAT_IS.contains(&word.as_str()) || data::FILLER.contains(&word.as_str())
        })
        .collect();
    if words.len() == 1 {
        return true;
    }
    let flat = words.join(" ");
    data::EVERYDAY_WORDS
        .iter()
        .any(|&(phrase, _)| phrase == flat)
}

fn definition(words: &[String], tables: &[&'static [(&str, &str)]]) -> Option<&'static str> {
    let words: Vec<_> = words.iter().map(|word| word.to_lowercase()).collect();
    let flat = words.join(" ");
    let exact = || {
        tables
            .iter()
            .flat_map(|table| table.iter().copied())
            .find_map(|(key, reply)| {
                (words.iter().any(|word| word == key) || (key.contains(' ') && flat.contains(key)))
                    .then_some(reply)
            })
    };
    exact().or_else(|| {
        tables
            .iter()
            .flat_map(|table| table.iter().copied())
            .find_map(|(key, reply)| {
                (!key.contains(' ') && words.iter().any(|word| word_form(key, word)))
                    .then_some(reply)
            })
    })
}

fn word_form(key: &str, word: &str) -> bool {
    if key.chars().count() < 4 || word.chars().count() < 4 {
        return false;
    }
    const ENDINGS: &[&str] = &["s", "d", "ed", "es", "ing", "ings"];
    for (root, other) in [(key, word), (word, key)] {
        if other
            .strip_prefix(root)
            .is_some_and(|suffix| ENDINGS.contains(&suffix))
            || (root
                .strip_suffix('y')
                .zip(other.strip_suffix("ies"))
                .is_some_and(|(left, right)| left == right))
        {
            return true;
        }
    }
    false
}

fn synonyms(words: &[String]) -> Option<&'static str> {
    let words: Vec<_> = words.iter().map(|word| word.to_lowercase()).collect();
    let exact = data::ASK_SPANS
        .iter()
        .filter_map(|parts| {
            words
                .windows(parts.len())
                .rposition(|window| window == *parts)
                .map(|at| at + parts.len())
        })
        .max();
    let begin = exact
        .or_else(|| {
            words
                .iter()
                .rposition(|word| data::CONNECTORS.contains(&word.as_str()))
                .map(|index| index + 1)
        })
        .unwrap_or(0);
    let span = &words[begin..];
    let padding = |words: &[String]| {
        words
            .iter()
            .all(|word| data::PADDING.contains(&word.as_str()))
    };
    for (at, word) in span.iter().enumerate() {
        for key in data::PHRASE_KEYS {
            let mut parts = key.split(' ');
            let length = parts.clone().count();
            if let Some(slot) = span.get(at..at + length)
                && slot.iter().all(|word| parts.next() == Some(word))
                && padding(&span[at + length..])
            {
                return value(data::WORD_SETS, key);
            }
        }
        let one = value(data::WORD_SETS, word).or_else(|| {
            word.strip_suffix('s')
                .and_then(|single| value(data::WORD_SETS, single))
        });
        if one.is_some() && !word.contains(' ') && padding(&span[at + 1..]) {
            return one;
        }
    }
    None
}
