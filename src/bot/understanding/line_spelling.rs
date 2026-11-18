//! Spelling repairs for words outside the conversational lexicon.

use super::super::normalize::one_edit;
use super::{get, has, present, tables::*};
use std::collections::BTreeSet;

#[path = "line_spelling_words.rs"]
mod words;
use words::BOT_WRITES;

const VOWELS: &str = "aeiou";
const UNRANKED: usize = 1_000_000;

fn rank(word: &str) -> usize {
    COMMON_RANK
        .binary_search_by_key(&word, |&(known, _)| known)
        .ok()
        .map_or(UNRANKED, |index| COMMON_RANK[index].1)
}

fn vowel(byte: u8) -> bool {
    b"aeiou".contains(&byte)
}

fn written(word: &str) -> bool {
    has(BASE, word) || has(BOT_WRITES, word)
}

fn target(word: &str) -> bool {
    !has(NO_SWAP_TO, word) && written(word)
}

fn bones(word: &str) -> impl Iterator<Item = u8> + '_ {
    word.bytes().filter(|&letter| !vowel(letter))
}

fn removing(word: &str, index: usize) -> String {
    format!("{}{}", &word[..index], &word[index + 1..])
}

fn adding(word: &str, index: usize, letter: u8) -> String {
    format!("{}{}{}", &word[..index], char::from(letter), &word[index..])
}

fn unique<'a>(mut choices: impl Iterator<Item = &'a str>) -> Option<&'a str> {
    let first = choices.next()?;
    choices.next().is_none().then_some(first)
}

pub(super) fn core_verb(
    word: &str,
    before: Option<&str>,
    after: Option<&str>,
) -> Option<&'static str> {
    if !(3..=7).contains(&word.len())
        || !word.bytes().all(|letter| letter.is_ascii_lowercase())
        || !matches!(before, Some("i" | "we" | "they" | "you" | "he" | "she"))
        || has(LEXICON, word)
        || has(COMMON_NAMES, word)
        || written(word)
    {
        return None;
    }
    let mut matches = CORE_VERBS.iter().copied().filter(|verb| {
        *verb != word && word.strip_suffix('d') != Some(verb) && one_edit(word, verb)
    });
    if let Some(found) = matches.next() {
        return matches.next().is_none().then_some(found);
    }
    if !present(AFTER_A_VERB, after) {
        return None;
    }
    unique(CORE_VERBS.iter().copied().filter(|verb| {
        *verb != word
            && word.as_bytes()[0] == verb.as_bytes()[0]
            && word.len().abs_diff(verb.len()) <= 1
            && bones(word).eq(bones(verb))
    }))
}

fn double_variants(word: &str) -> impl Iterator<Item = String> + '_ {
    let bytes = word.as_bytes();
    word.bytes()
        .enumerate()
        .filter(|&(_, letter)| !vowel(letter) && letter != b'y')
        .map(move |(index, letter)| {
            if bytes.get(index + 1) == Some(&letter) {
                removing(word, index)
            } else {
                adding(word, index, letter)
            }
        })
}

fn not_a_doubled_form(word: &str, variant: &str) -> bool {
    if variant.len() >= word.len() {
        return true;
    }
    !["ing", "est", "ed", "er"].iter().any(|ending| {
        variant.strip_suffix(ending).is_some_and(|stem| {
            stem.len() >= 2 && adding(variant, stem.len(), stem.as_bytes()[stem.len() - 1]) == word
        })
    })
}

fn doubled_words(word: &str) -> BTreeSet<String> {
    word.bytes()
        .enumerate()
        .map(|(index, letter)| adding(word, index, letter))
        .filter(|variant| {
            (has(BASE, variant) || has(LEXICON, variant) && !has(BASE, &format!("{variant}s")))
                && !variant
                    .as_bytes()
                    .windows(3)
                    .any(|group| group[0] == group[1] && group[1] == group[2])
        })
        .collect()
}

fn known_to_reading(word: &str) -> bool {
    let known = |word: &str| has(LEXICON, word) || has(THING_WORDS, word);
    if known(word) {
        return true;
    }
    for (ending, threshold, alt) in [("ing", 5, "e"), ("ed", 4, "e"), ("es", 5, ""), ("s", 4, "")] {
        if word.len() > threshold
            && let Some(stem) = word.strip_suffix(ending)
            && (known(stem) || !alt.is_empty() && known(&format!("{stem}{alt}")))
        {
            return true;
        }
    }
    false
}

fn missing_from_thing(word: &str) -> Option<&'static str> {
    if word.len() < 4 || known_to_reading(word) {
        return None;
    }
    unique(THING_WORDS.iter().copied().filter(|candidate| {
        candidate.len() == word.len() + 1
            && candidate.as_bytes()[0] == word.as_bytes()[0]
            && candidate.as_bytes().last() == word.as_bytes().last()
            && one_edit(word, candidate)
    }))
}

fn missing_vowels(word: &str) -> impl Iterator<Item = String> + '_ {
    let bytes = word.as_bytes();
    (1..=word.len())
        .filter(move |&index| {
            if word.len() < 5 {
                return false;
            }
            let start = bytes[..index]
                .iter()
                .rposition(|&letter| vowel(letter))
                .map_or(0, |i| i + 1);
            let end = bytes[index..]
                .iter()
                .position(|&letter| vowel(letter))
                .map_or(word.len(), |i| i + index);
            end - start >= 3
                && (bytes[start] != b's' || !b"tpskmlnwhqc".contains(&bytes[start + 1]))
        })
        .flat_map(move |index| {
            VOWELS
                .bytes()
                .map(move |letter| adding(word, index, letter))
        })
        .filter(|candidate| has(BASE, candidate) && target(candidate))
}

fn dropped_g(word: &str) -> Option<String> {
    let stem = word.strip_suffix("in")?;
    if word.len() < 4 || has(REAL_IN, word) {
        return None;
    }
    let with_g = format!("{word}g");
    if has(LEXICON, &with_g) {
        return Some(with_g);
    }
    if let Some(stem) = stem.strip_suffix('e')
        && stem.len() > 1
        && let candidate = format!("{stem}ing")
        && has(LEXICON, &candidate)
    {
        return Some(candidate);
    }
    let bytes = stem.as_bytes();
    let stem = if bytes.len() > 3 && bytes[bytes.len() - 2] == bytes[bytes.len() - 1] {
        &stem[..stem.len() - 1]
    } else {
        stem
    };
    let last = stem.chars().next_back()?;
    if !has(LEXICON, stem) {
        return None;
    }
    let candidate = format!("{stem}{last}ing");
    has(LEXICON, &candidate).then_some(candidate)
}

fn unstressed_ending(word: &str) -> impl Iterator<Item = String> + '_ {
    let stem = word
        .strip_suffix("as")
        .filter(|_| word.len() >= 5)
        .map(|stem| (stem, "s"))
        .or_else(|| {
            word.strip_suffix('a')
                .filter(|_| word.len() >= 4)
                .map(|stem| (stem, ""))
        });
    let stem = stem.filter(|(stem, _)| {
        !has(A_WORDS, word) && stem.as_bytes().last().is_some_and(|&letter| !vowel(letter))
    });
    stem.into_iter().flat_map(|(stem, plural)| {
        ["er", "or", "ar", "ow"]
            .into_iter()
            .map(move |ending| format!("{stem}{ending}{plural}"))
            .filter(move |candidate| {
                has(BASE, candidate) || !plural.is_empty() && has(LEXICON, candidate)
            })
    })
}

fn silent_e(word: &str) -> Option<String> {
    if word.len() < 4 || !word.ends_with('d') {
        return None;
    }
    let bytes = word.as_bytes();
    let preceding = bytes[word.len() - 2];
    let candidate = if preceding == b'y' {
        if vowel(bytes[word.len() - 3]) || bytes[word.len() - 3] == b'y' {
            return None;
        }
        format!("{}ied", &word[..word.len() - 2])
    } else {
        if !b"ktvpsmfgxzcjq".contains(&preceding) {
            return None;
        }
        format!("{}ed", &word[..word.len() - 1])
    };
    has(LEXICON, &candidate).then_some(candidate)
}

fn urgent_spelling(word: &str) -> Option<&'static str> {
    if word.len() < 6
        || !word
            .as_bytes()
            .windows(3)
            .any(|group| group.iter().all(|&letter| !vowel(letter) && letter != b'y'))
    {
        return None;
    }
    let urgent_bones = |word: &str| {
        word.bytes()
            .filter(|&letter| !vowel(letter) && letter != b'y')
            .collect::<Vec<_>>()
    };
    let brief = urgent_bones(word);
    unique(
        URGENT_WORDS
            .iter()
            .copied()
            .filter(|candidate| candidate.len() > word.len() && brief == urgent_bones(candidate)),
    )
}

fn dropped_ing(word: &str) -> Option<String> {
    let stem = word.strip_suffix('n')?;
    if word.len() < 5
        || word.as_bytes()[word.len() - 3..word.len() - 1]
            .iter()
            .copied()
            .any(vowel)
    {
        return None;
    }
    let candidate = format!("{stem}ing");
    has(LEXICON, &candidate).then_some(candidate)
}

fn ending_sound(word: &str) -> Option<String> {
    for ending in ["fings", "fing", "finks", "fink", "fins", "fin"] {
        if let Some(stem) = word.strip_suffix(ending) {
            if let Some(target) = get(STOPPED_TH_STEMS, stem) {
                let plural = if ending.ends_with('s') { "s" } else { "" };
                return Some(format!("{target}{plural}"));
            }
            break;
        }
    }
    if word.ends_with("shuns") && word.len() > 7 {
        return ending_sound(&word[..word.len() - 1]).map(|singular| format!("{singular}s"));
    }
    let stem = word
        .strip_suffix("kshun")
        .or_else(|| word.strip_suffix("shun"))?;
    if stem.len() < 2 {
        return None;
    }
    [
        "shion", "ssion", "sion", "tion", "stion", "sssion", "ssion", "cian",
    ]
    .into_iter()
    .map(|ending| format!("{stem}{ending}"))
    .find(|candidate| has(LEXICON, candidate) || has(SHION_WORDS, candidate))
    .or_else(|| Some(format!("{stem}tion")))
}

pub(super) fn repair_typo(word: &str) -> Option<String> {
    if written(word)
        || has(TYPED_WORDS, word)
        || has(NOT_A_FEELING, word)
        || has(COMMON_NAMES, word)
    {
        return None;
    }
    if let Some(spelled) = ending_sound(word) {
        return Some(spelled);
    }
    let bytes = word.as_bytes();
    if word.len() >= 3 {
        let short = (bytes[word.len() - 1] == bytes[word.len() - 2])
            .then(|| word[..word.len() - 1].to_owned());
        if let Some(spelled) = double_variants(word)
            .chain(short)
            .filter(|candidate| has(LEXICON, candidate) && has(FREQUENT, candidate))
            .min_by_key(|candidate| rank(candidate))
        {
            return Some(spelled);
        }
    }
    let doubled = doubled_words(word);
    if word.len() == 3 {
        return doubled
            .into_iter()
            .filter(|candidate| rank(candidate) < 2000)
            .min_by_key(|candidate| rank(candidate));
    }
    if word.len() < 4 {
        return None;
    }
    if let Some(thing) = missing_from_thing(word) {
        return Some(thing.into());
    }

    let mut candidates = doubled;
    candidates.extend(double_variants(word).filter(|variant| {
        has(BASE, variant) || (target(variant) && not_a_doubled_form(word, variant))
    }));
    for (index, pair) in bytes.windows(2).enumerate() {
        if pair[0] != pair[1] && (word.len() >= 5 || matches!(pair, b"ei" | b"ie")) {
            let swapped = format!(
                "{}{}{}{}",
                &word[..index],
                char::from(pair[1]),
                char::from(pair[0]),
                &word[index + 2..]
            );
            if target(&swapped)
                || word.len() >= 5
                    && !vowel(pair[0])
                    && !vowel(pair[1])
                    && has(URGENT_WORDS, &swapped)
            {
                candidates.insert(swapped);
            }
        }
    }
    candidates.extend(dropped_g(word));
    candidates.extend(dropped_ing(word));
    let with_w = format!("{word}w");
    if !word.ends_with('w') && target(&with_w) {
        candidates.insert(with_w);
    }
    let with_h = format!("h{word}");
    if vowel(bytes[0])
        && !(word.ends_with('s') && has(BASE, &word[..word.len() - 1]))
        && target(&with_h)
    {
        candidates.insert(with_h);
    }
    if word.len() >= 8 {
        for (index, &letter) in bytes.iter().enumerate().take(word.len() - 1) {
            if b"ei".contains(&letter)
                && (index == 0 || vowel(bytes[index - 1]) || vowel(bytes[index + 1]))
            {
                let candidate = removing(word, index);
                if target(&candidate) {
                    candidates.insert(candidate);
                }
            }
        }
    }
    candidates.extend(missing_vowels(word));
    candidates.extend(unstressed_ending(word));
    candidates.extend(silent_e(word));
    candidates.extend(urgent_spelling(word).map(str::to_owned));
    if word.len() >= 6
        && let Some(stem) = word.strip_suffix("eing")
        && !has(E_KEPT_WORDS, word)
    {
        let candidate = format!("{stem}ing");
        if has(LEXICON, &candidate) {
            candidates.insert(candidate);
        }
    }
    for ending in ["en", "le"] {
        if let Some(stem) = word.strip_suffix(ending)
            && stem.len() > 1
            && !vowel(stem.as_bytes()[stem.len() - 1])
        {
            let candidate = format!("{stem}t{ending}");
            if has(LEXICON, &candidate) {
                candidates.insert(candidate);
            }
        }
    }
    if candidates.len() == 1 {
        return candidates.pop_first();
    }
    let mut candidates: Vec<_> = candidates.into_iter().collect();
    candidates.sort_by_key(|candidate| rank(candidate));
    if let [first, second, ..] = candidates.as_slice()
        && rank(first) < 120
        && rank(second) > 900
    {
        return Some(first.clone());
    }
    None
}

pub(super) fn terminal_state(word: &str, context: &[String]) -> Option<&'static str> {
    if !state_frame(context, None) {
        return None;
    }
    terminal_state_word(word)
}

pub(super) fn terminal_state_word(word: &str) -> Option<&'static str> {
    if word.len() < 5 || has(LEXICON, word) || has(COMMON_NAMES, word) {
        return None;
    }
    unique(STATES_OF_A_PERSON.iter().copied().filter(|state| {
        if word.as_bytes()[0] != state.as_bytes()[0] || word.len().abs_diff(state.len()) > 1 {
            return false;
        }
        if word.len() != state.len() {
            let (short, long) = if word.len() < state.len() {
                (word, *state)
            } else {
                (*state, word)
            };
            return (1..long.len()).any(|index| {
                short.as_bytes()[..index] == long.as_bytes()[..index]
                    && short.as_bytes()[index..] == long.as_bytes()[index + 1..]
            });
        }
        let mismatches: Vec<_> = word
            .bytes()
            .zip(state.bytes())
            .filter(|(a, b)| a != b)
            .collect();
        match mismatches.as_slice() {
            [(first, second)] => b"aeiouy".contains(first) && b"aeiouy".contains(second),
            [_, _] => one_edit(word, state),
            _ => false,
        }
    }))
}

pub(super) fn state_without_vowel(word: &str) -> Option<&'static str> {
    if !(3..5).contains(&word.len()) || has(LEXICON, word) || has(COMMON_NAMES, word) {
        return None;
    }
    unique(ENDING_STATES.iter().copied().filter(|state| {
        state.len() == word.len() + 1
            && (1..state.len() - 1).any(|at| {
                vowel(state.as_bytes()[at])
                    && word.as_bytes()[..at] == state.as_bytes()[..at]
                    && word.as_bytes()[at..] == state.as_bytes()[at + 1..]
            })
    }))
}

pub(super) fn two_letters_gone(word: &str) -> Option<&'static str> {
    if word.len() < 6 || has(COMMON_NAMES, word) {
        return None;
    }
    unique(FEELINGS.iter().copied().filter(|state| {
        let difference = state.len().checked_sub(word.len());
        if !matches!(difference, Some(1 | 2))
            || state.as_bytes()[0] != word.as_bytes()[0]
            || feeling_edit(word, state)
        {
            return false;
        }
        let mut letters = state.bytes();
        word.bytes()
            .all(|word_letter| letters.any(|state_letter| word_letter == state_letter))
    }))
}

pub(super) fn said_with_f(word: &str) -> Option<String> {
    if let Some(common) = get(F_TYPED_FOR_TH, word) {
        return Some(common.to_owned());
    }
    if word.len() >= 4
        && let Some(stem) = word.strip_prefix('f')
    {
        let spelled = format!("th{stem}");
        if has(LEXICON, &spelled)
            || get(CHAT, &spelled).is_some()
            || get(EXPAND, &spelled).is_some()
            || ["s", "es", "d", "ed", "ing"].iter().any(|suffix| {
                spelled
                    .strip_suffix(suffix)
                    .is_some_and(|stem| stem.len() >= 5 && has(LEXICON, stem))
            })
        {
            return Some(spelled);
        }
        if spelled.ends_with("in") {
            let with_g = format!("{spelled}g");
            if has(LEXICON, &with_g) {
                return Some(with_g);
            }
        }
    }
    if word.len() < 3 {
        return None;
    }
    if let Some(stem) = word.strip_suffix("ff") {
        let replacement = format!("{stem}th");
        return (!word.starts_with('w') && has(LEXICON, &replacement)).then_some(replacement);
    }
    word.match_indices('f')
        .map(|(index, _)| format!("{}th{}", &word[..index], &word[index + 1..]))
        .find(|spelled| has(LEXICON, spelled))
}

pub(super) fn feeling_edit(word: &str, feeling: &str) -> bool {
    if word.len().abs_diff(feeling.len()) > 1 {
        return false;
    }
    if word.len() == feeling.len() {
        let mut different = word.bytes().zip(feeling.bytes()).filter(|(a, b)| a != b);
        let Some((a, b)) = different.next() else {
            return false;
        };
        return if different.next().is_none() {
            b"aeiouy".contains(&a) && b"aeiouy".contains(&b)
        } else {
            one_edit(word, feeling)
        };
    }
    let (short, long) = if word.len() < feeling.len() {
        (word, feeling)
    } else {
        (feeling, word)
    };
    let index = short.bytes().zip(long.bytes()).position(|(a, b)| a != b);
    index.is_some_and(|index| {
        short.as_bytes()[index..] == long.as_bytes()[index + 1..]
            && (long == feeling || vowel(long.as_bytes()[index]))
    })
}

fn measures(word: &str, wide: bool) -> bool {
    if wide && has(PHRASE_MEASURES, word) {
        return true;
    }
    has(AMOUNT_BEFORE_A_STATE, word)
        || has(BARE_TO, word)
        || word.len() >= 5 && word.ends_with("ly")
        || word.len() >= 6
            && word.bytes().all(|letter| letter.is_ascii_lowercase())
            && (["aly", "lye", "yl"]
                .iter()
                .any(|ending| word.ends_with(ending))
                || word.len() >= 9 && word.ends_with("ley")
                || AMOUNT_BEFORE_A_STATE
                    .iter()
                    .any(|measure| feeling_edit(word, measure)))
}

fn end_of_state(after: Option<&str>, opening: bool) -> bool {
    matches!(after, None | Some("|"))
        || present(AFTER_A_STATE, after)
        || !opening
            && (present(TIMES_A_STATE, after)
                || present(AFTER_A_TIME, after)
                || present(A_STATE_THEN_A_TIME, after))
}

fn state_frame(context: &[String], after: Option<&str>) -> bool {
    if let Some(last) = context.last()
        && has(DETERMINER_IN_FRONT, last)
        && !(has(POSSESSIVE_IN_FRONT, last) && context.iter().any(|word| has(CAUSES_A_STATE, word)))
    {
        return false;
    }
    let opens = || end_of_state(after, true) || present(NOT_A_THING, after);
    let Some(index) = context.iter().rposition(|word| !measures(word, false)) else {
        return opens();
    };
    let place = &context[index];
    let prev = index.checked_sub(1).map(|index| context[index].as_str());
    if has(BE_BEFORE_A_STATE, place) {
        return true;
    }
    if has(THIRD_PERSON_BE, place) {
        return end_of_state(
            after,
            !present(PERSON_WORDS, prev) && !present(OBJECT_PRONOUN, prev),
        );
    }
    if has(CAUSES_A_STATE, place) {
        return index + 1 < context.len() || end_of_state(after, false);
    }
    if present(CAUSES_A_STATE, prev) {
        if has(OBJECT_PRONOUN, place) {
            return true;
        }
        if place == "my" {
            return end_of_state(after, false);
        }
    }
    match context[..=index]
        .iter()
        .rposition(|word| !measures(word, true))
    {
        None => opens(),
        Some(wide) => wide < index && has(BE_BEFORE_A_STATE, &context[wide]),
    }
}

pub(super) fn state_ending(word: &str) -> Option<&'static str> {
    if word.len() < 3
        || !word.bytes().all(|letter| letter.is_ascii_lowercase())
        || has(ENDING_STATES, word)
    {
        return None;
    }
    for ending in ["ed", "d", "y", "s", "us", "e", "g", "ing", "l", "ty"] {
        let typed = format!("{word}{ending}");
        if let Ok(index) = ENDING_STATES.binary_search(&typed.as_str()) {
            return Some(ENDING_STATES[index]);
        }
    }
    None
}

fn introduction(context: &[String], next: &[String]) -> bool {
    let after = next.first().map(String::as_str);
    context.iter().any(|word| has(INTRODUCING_A_PERSON, word))
        || present(INTRODUCING_AFTER, after)
        || context.ends_with(&["this".into(), "is".into()])
        || context.ends_with(&["that".into(), "is".into()])
        || after == Some("is") && context.last().is_none_or(|word| word == "|")
        || (matches!(after, Some("and" | "|")) || after.is_some_and(super::is_digits))
            && next.iter().take(3).any(|word| super::is_digits(word))
}

pub(super) fn near_state(word: &str, context: &[String], next: &[String]) -> Option<String> {
    let after = next.first().map(String::as_str);
    if word.len() < 4
        || !word.bytes().all(|letter| letter.is_ascii_lowercase())
        || has(NOT_A_FEELING, word)
        || !state_frame(context, after)
    {
        return None;
    }
    let terminal = matches!(after, None | Some("|"));
    if !has(COMMON_NAMES, word) && !introduction(context, next) {
        for (index, pair) in word.as_bytes().windows(2).enumerate() {
            if pair[0] == pair[1] {
                let shortened = removing(word, index);
                if has(FEELINGS, &shortened) {
                    return Some(shortened);
                }
            }
        }
    }
    if word.len() == 4 && (!terminal || has(COMMON_NAMES, word)) {
        return None;
    }
    for ending in ["cious", "ctious"] {
        if let Some(prefix) = word.strip_suffix(ending) {
            let state = format!("{prefix}xious");
            if has(FEELINGS, &state) {
                return Some(state);
            }
        }
    }
    if let Some(state) = unique(FEELINGS.iter().copied().filter(|state| {
        feeling_edit(word, state)
            && (word.len() >= 5 || state.len() == word.len() + 1)
            && !(word.ends_with("ing")
                && state.len() == word.len() + 1
                && word
                    .bytes()
                    .zip(state.bytes())
                    .find(|(a, b)| a != b)
                    .is_some_and(|(_, missing)| !vowel(missing)))
    })) {
        return Some(state.to_owned());
    }
    if terminal && context.iter().rposition(|word| !measures(word, false)) == Some(0) {
        return two_letters_gone(word).map(str::to_owned);
    }
    None
}
