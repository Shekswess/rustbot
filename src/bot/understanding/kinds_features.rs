//! Structured dialogue signals read from the normalized line.

use super::{Kind, asks_help, has, starts};
use crate::bot::understanding::{intent, line::Line};
use fancy_regex::Regex as ContextRegex;
use std::sync::LazyLock;

macro_rules! context_pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: LazyLock<ContextRegex> =
            LazyLock::new(|| ContextRegex::new($source).expect(stringify!($name)));
    };
}

#[path = "kinds_features_data.rs"]
mod data;
use data::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Intent {
    Empty,
    Laugh,
    Farewell,
    AboutBot,
    Greeting,
    Thanks,
    Correction,
    Smalltalk,
    Ack,
    Deny,
    Affirm,
    Question,
    Request,
    Feeling,
    News,
    Opinion,
    Statement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub(crate) enum StateOwner {
    #[serde(rename = "self")]
    Speaker,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "thing")]
    Thing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum AboutBotKind {
    While,
    Keeping,
    Where,
    Identity,
    Warmth,
    Memory,
    Help,
    Model,
    Doings,
    Life,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub(crate) struct Suggestion {
    pub(crate) who: String,
    pub(crate) source: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum SummonsShape {
    Written,
    Rang,
    See,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub(crate) struct Summons {
    pub(crate) who: &'static str,
    pub(crate) shape: SummonsShape,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub(crate) struct KindFeatures {
    pub(crate) intent: Intent,
    pub(crate) asks: Vec<&'static str>,
    pub(crate) tag: bool,
    pub(crate) emotion: Option<&'static str>,
    pub(crate) state_owner: Option<StateOwner>,
    pub(crate) shortage: Option<String>,
    pub(crate) polarity: i32,
    pub(crate) short: bool,
    pub(crate) first_person: bool,
    pub(crate) asks_opinion: bool,
    pub(crate) asks_recommendation: bool,
    pub(crate) asks_steps: bool,
    pub(crate) help_ask: bool,
    pub(crate) asks_a_verdict: bool,
    pub(crate) about_bot: bool,
    pub(crate) about_bot_kind: Option<AboutBotKind>,
    pub(crate) counts_the_bots_own_day: bool,
    pub(crate) replies_to_bot: bool,
    pub(crate) agrees_to_try: bool,
    pub(crate) wants_a_quiz: bool,
    pub(crate) agreement: Option<&'static str>,
    pub(crate) summons: Option<Summons>,
    pub(crate) asks_back: bool,
    pub(crate) asks_what_was_meant: bool,
    pub(crate) excludes: Vec<String>,
    pub(crate) ruled_out: Vec<String>,
    pub(crate) constraint: bool,
    pub(crate) suggestion: Option<Suggestion>,
}

fn matches(pattern: &ContextRegex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn at_start(pattern: &ContextRegex, text: &str) -> bool {
    pattern
        .find(text)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == 0)
}

fn replace(pattern: &ContextRegex, text: &str, replacement: &str) -> String {
    pattern
        .try_replacen(text, 0, replacement)
        .map_or_else(|_| text.into(), |text| text.into_owned())
}

fn includes(words: &[&str], set: &[&str]) -> bool {
    words.iter().any(|word| has(set, word))
}

fn phrase(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| text.contains(phrase))
}

fn lead(words: &[&str], set: &[&str]) -> usize {
    words.iter().take_while(|word| has(set, word)).count()
}

fn finish(words: &[&str], set: &[&str]) -> usize {
    words
        .iter()
        .rposition(|word| !has(set, word))
        .map_or(0, |at| at + 1)
}

fn trimmed(word: &str) -> &str {
    word.trim_matches(['.', ',', '!', '?', ';', ':', '-', '|', '"', '\''])
}

fn strip_leaders<'a, 'b>(words: &'a [&'b str]) -> &'a [&'b str] {
    let index = lead(words, super::data::LEADERS);
    &words[index..]
}

fn agreement(text: &str) -> Option<&'static str> {
    let tidy = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let said = text.to_lowercase().replace(',', " ");
    let first = tidy(said.split('|').next().unwrap_or(""));
    let flat = tidy(&said.replace('|', " "));
    AGREEMENT_IDIOMS
        .iter()
        .copied()
        .find(|phrase| *phrase == first || *phrase == flat)
}

fn polarity(norm: &[&str]) -> i32 {
    let mut score = 0i32;
    let mut negated = false;
    for (index, &word) in norm.iter().enumerate() {
        if has(NEGATORS, word) {
            negated = true;
            continue;
        }
        if has(TERMS_OF_ADDRESS, word)
            && index + 1 == norm.len()
            && (2..=3).contains(&norm.len())
            && has(ADDRESSED_LINES, norm[0])
        {
            negated = false;
            continue;
        }
        let amount = if has(OK_WORDS, word) || has(SAYS_HOW_THEY_ARE, word) {
            state_of_speaker(norm, index).then_some(1)
        } else if has(POSITIVE, word) {
            Some(1)
        } else if has(NEGATIVE, word) {
            (!has(BODY_STATE_WORDS, word)
                || norm
                    .get(index + 1)
                    .is_none_or(|after| !has(STOMACH_WORDS, after)))
            .then_some(-1)
        } else {
            bad_verdict(norm, index).then_some(-1)
        };
        if let Some(amount) = amount {
            score += if negated { -amount } else { amount };
            negated = false;
        } else if !has(NEGATION_SKIP, word) {
            negated = false;
        }
    }
    score.signum()
}

fn bad_verdict(norm: &[&str], index: usize) -> bool {
    let word = norm[index];
    if !has(VERDICT_NOUNS, word) && !has(VERDICT_ADJECTIVES, word) {
        return false;
    }
    let mut before = norm[..index]
        .iter()
        .rev()
        .skip_while(|word| has(VERDICT_FILLER, word));
    if !before.next().is_some_and(|word| has(COPULAS, word)) {
        return false;
    }
    before.next().is_some_and(|subject| {
        [PRONOUN_SUBJECT, ASKING_SUBJECT, COPULAS, NOT_A_THING]
            .iter()
            .all(|set| !has(set, subject))
    })
}

fn state_of_speaker(norm: &[&str], index: usize) -> bool {
    let mut verb = false;
    for word in norm[..index].iter().rev().take(4) {
        if has(SPEAKER, word) {
            return verb;
        }
        if has(STATING_VERB, word) {
            verb = true;
        } else if !has(NEGATORS, word) && !has(NEGATION_SKIP, word) {
            return false;
        }
    }
    false
}

context_pattern!(RUN_OUT, r"\b(?:run|ran|running) out of ([a-z]+)");

fn shortage(text: &str) -> Option<String> {
    let words: Vec<_> = text.split_whitespace().map(trimmed).collect();
    for window in words.windows(3) {
        if has(SUPPLY_WORDS, window[0])
            && has(SUPPLY_PREPOSITIONS, window[1])
            && [
                DETERMINERS,
                OWN_RESERVES,
                OTHER_PEOPLE,
                PERSON_NOUNS,
                PERSON_SUBJECT,
            ]
            .iter()
            .all(|set| !has(set, window[2]))
        {
            return Some(window[2].to_owned());
        }
    }
    RUN_OUT
        .captures(text)
        .ok()
        .flatten()
        .and_then(|part| part.get(1))
        .filter(|word| !has(OWN_RESERVES, word.as_str()) && !has(DETERMINERS, word.as_str()))
        .map(|word| word.as_str().to_owned())
}

fn hedged_low(text: &str) -> bool {
    let lower = text.to_lowercase();
    let mut words: Vec<_> = lower
        .split_whitespace()
        .map(trimmed)
        .filter(|word| !word.is_empty())
        .collect();
    let mut doubted = false;
    for doubt in HEDGED_DOUBT {
        let tail: Vec<_> = doubt.split_whitespace().collect();
        if words.len() > tail.len() && words.ends_with(&tail) {
            words.truncate(words.len() - tail.len());
            doubted = true;
            break;
        }
    }
    let words = &words[lead(&words, HEDGED_WRAPPER)..];
    let answer = words[..finish(words, HEDGED_WRAPPER)].join(" ");
    has(HEDGED_LOW, &answer) || doubted && has(HEDGED_OK, &answer)
}

static LIMITS: LazyLock<Vec<ContextRegex>> = LazyLock::new(|| {
    LIMIT_SHAPES
        .iter()
        .map(|shape| ContextRegex::new(shape).expect("limit shape"))
        .collect()
});

fn limit(text: &str) -> Option<String> {
    if text.is_empty() {
        return None;
    }
    let mut ask = text
        .replace('|', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches([' ', '.', ',', '!', '?'])
        .to_lowercase();
    for _ in 0..3 {
        let before = replace(&LIMIT_SOFT_BEFORE, &ask, "");
        let after = replace(&LIMIT_SOFT_AFTER, &before, "");
        let after = after.trim_matches([' ', '.', ',', '!', '?']);
        if after == ask {
            break;
        }
        ask = after.into();
    }
    for shape in &*LIMITS {
        let Some(parts) = shape.captures(&ask).ok().flatten() else {
            continue;
        };
        let found = parts.get(0).unwrap();
        let words = parts.get(1).unwrap().as_str();
        let mut words: Vec<_> = words
            .trim_matches([' ', '.', ',', '!', '?'])
            .split_whitespace()
            .collect();
        if words.len() > 1 && matches!(words.last(), Some(&("one" | "ones"))) {
            words.pop();
        }
        if words.is_empty()
            || words.len() > LIMIT_ROOM
            || includes(&words, LIMIT_NOT_A_THING)
            || includes(&words, LIMIT_REFUSAL)
            || has(LIMIT_NOT_A_START, words[0])
            || at_start(&PAST_ENDS, words[words.len() - 1])
            || ask.starts_with("nothing") && claim(words[words.len() - 1])
            || includes(&words, LIMIT_A_FACT)
            || words.iter().any(|word| word.chars().all(char::is_numeric))
        {
            continue;
        }
        let before = ask[..found.start()].trim_matches([' ', '.', ',', '!', '?']);
        if !before.is_empty() && !replace(&LIMIT_SOFT_BEFORE, &format!("{before} "), "").is_empty()
        {
            continue;
        }
        return Some(words.join(" "));
    }
    None
}

fn ruled_out(text: &str) -> Vec<String> {
    let mut gone = Vec::new();
    for parts in RULED_OUT.captures_iter(text).flatten() {
        let before = &text[..parts.get(0).unwrap().start()];
        if !before.split_whitespace().rev().take(3).any(|word| {
            let word = word.trim_matches([' ', '.', ',', '!', '?', ';', ':', '\'', '’']);
            !word.is_empty() && !has(RULED_OUT_NO_THING, word)
        }) {
            continue;
        }
        let value = parts.get(1).unwrap().as_str();
        if value
            .split_whitespace()
            .any(|word| has(RULED_OUT_CATEGORY, word))
        {
            continue;
        }
        let same = matches!(value.split_whitespace().next_back(), Some("one" | "ones"));
        if !same && !matches(&RULED_OUT_CORRECTION, before) {
            continue;
        }
        for word in value.split_whitespace() {
            let word = word.trim_matches([' ', '.', ',', '!', '?', ';', ':', '\'', '’', '“', '”']);
            if word.is_empty() || has(RULED_OUT_ENDS, word) {
                break;
            }
            gone.push(word.into());
            if gone.len() >= 4 {
                break;
            }
        }
    }
    gone
}

fn suggestion(norm: &[&str]) -> Option<Suggestion> {
    let said = format!(" {} ", norm.join(" "));
    let reported = A_REPORTED_VERB.find(&said).ok().flatten()?;
    if !(matches(&A_SUGGESTED_CLAUSE, &said[reported.start()..]) || matches(&A_TOLD_TO, &said))
        || matches(&A_NOT_A_SUGGESTION, &said)
    {
        return None;
    }
    let before = &said[..reported.start()];
    let source = A_SAID_BY
        .find_iter(before)
        .flatten()
        .last()
        .map_or("", |words| words.as_str().trim());
    let who = match source.split_once(' ') {
        Some(("my" | "our", who)) => format!("your {who}"),
        _ => source.to_owned(),
    };
    let who = who.chars().next().map_or(who.clone(), |first| {
        format!("{}{}", first.to_uppercase(), &who[first.len_utf8()..])
    });
    Some(Suggestion {
        who,
        source: source.into(),
    })
}

fn asks_quiz(norm: &[&str]) -> bool {
    for (at, &word) in norm.iter().enumerate().take(norm.len().saturating_sub(1)) {
        if !has(HAND_OVER_VERBS, word) {
            continue;
        }
        let to = norm[at + 1];
        let handed = has(HAND_OVER_TO, to)
            || matches!(to, "my" | "our")
                && norm
                    .get(at + 2)
                    .is_some_and(|word| has(HAND_OVER_MINE, word));
        if !handed {
            continue;
        }
        let before = &norm[..at];
        if before.is_empty() {
            return true;
        }
        let modal = includes(before, HAND_OVER_MODALS);
        if before.iter().all(|word| {
            has(HAND_OVER_LEADS, word)
                || has(HAND_OVER_MODALS, word)
                || modal && has(HAND_OVER_SUBJECTS, word)
        }) && (modal || !includes(before, HAND_OVER_SUBJECTS))
        {
            return true;
        }
    }
    false
}

fn question_coming(norm: &[&str], text: &str, quiz: bool) -> bool {
    if norm.is_empty()
        || norm.len() > 12
        || text.contains('?')
        || includes(norm, ALREADY_ASKED)
        || quiz
        || matches!(norm[0], "do" | "does" | "did" | "any")
        || matches!(
            norm,
            [
                "have" | "has",
                "you" | "u" | "anyone" | "anybody" | "any" | "they",
                ..
            ]
        )
    {
        return false;
    }
    norm.iter().take(5).enumerate().any(|(at, &word)| {
        if word == "asked" && at == 1 && matches!(norm[0], "i" | "we")
            || includes(&norm[..at], PAST_ASK)
        {
            return false;
        }
        let after = norm.get(at + 1).copied().unwrap_or("");
        let target = has(ASK_TARGET, after);
        let asked_here = has(ASKING_VERBS, word)
            && (target || has(ASK_ABOUT, after))
            && (at == 0
                || target
                || includes(&norm[..at], ASK_MODALS)
                || includes(&norm[..at], ASK_AUX));
        (has(QUESTION_NOUNS, word) || asked_here) && (at == 0 || includes(&norm[..at], ASK_FRAMES))
    })
}

fn agrees_to_try(norm: &[&str]) -> bool {
    if norm.is_empty()
        || norm.len() > 9
        || includes(norm, super::data::QUESTION_WORDS)
        || includes(norm, COMMIT_STOP)
    {
        return false;
    }
    let bare = norm.join(" ");
    let mut hedged = norm;
    while let Some(first) = hedged.first() {
        if has(HEDGE_LEAD_ONE, first) {
            hedged = &hedged[1..];
        } else if hedged.len() >= 2 && has(HEDGE_LEAD_TWO, &hedged[..2].join(" ")) {
            hedged = &hedged[2..];
        } else {
            break;
        }
    }
    let combined = hedged.join(" ").replace("can not", "cant");
    let mut said: Vec<_> = combined.split_whitespace().collect();
    if said.first().is_some_and(|word| has(NO_HARM_SUBJECT, word)) {
        said.remove(0);
    }
    if (1..=7).contains(&said.len()) && COMMIT_NO_HARM.iter().any(|words| starts(&said, words))
        || has(COMMIT_ALONE, &bare)
        || norm.len() <= 5 && COMMIT_ALONE.iter().any(|words| starts(norm, words))
        || COMMIT_ALONE_TAIL.iter().any(|words| starts(norm, words))
    {
        return true;
    }
    let mut read = strip_leaders(norm);
    if read.first().is_some_and(|word| has(AFFIRM, word)) {
        read = &read[1..];
        if read.starts_with(&["one"]) {
            read = &read[1..];
        }
    }
    if read.first().is_some_and(|word| has(COMMIT_SUBJECT, word)) {
        read = &read[1..];
    }
    if read.first().is_some_and(|word| has(HEDGED_COMMIT, word)) {
        return HEDGED_ALONE.iter().any(|words| starts(&read[1..], words));
    }
    if read.first().is_some_and(|word| has(COMMIT_SUBJECT, word)) {
        read = &read[1..];
    }
    if !read.first().is_some_and(|word| has(COMMIT_OPEN, word)) {
        return false;
    }
    read = read[1..].strip_prefix(&["to"]).unwrap_or(&read[1..]);
    read.first().is_some_and(|word| has(COMMIT_TRY, word))
}

fn summons(norm: &[&str], kind: Kind) -> Option<Summons> {
    if kind == Kind::Question || norm.is_empty() {
        return None;
    }
    let said = norm.join(" ");
    let who = SUMMONS_WHO_PHRASES
        .iter()
        .copied()
        .find(|words| said.contains(words))
        .or_else(|| {
            norm.iter().find_map(|word| {
                SUMMONS_WHO
                    .binary_search(word)
                    .ok()
                    .map(|index| SUMMONS_WHO[index])
            })
        })?;
    let at = norm
        .iter()
        .position(|word| Some(*word) == who.split_whitespace().next())?;
    let verb = norm
        .iter()
        .position(|word| has(SUMMONS_VERB, word) && !matches!(*word, "has" | "have"))
        .or_else(|| norm.iter().position(|word| has(SUMMONS_VERB, word)))?;
    let joined = format!(" {said} ");
    let requests_letter = norm
        .iter()
        .position(|word| has(LETTER_RECEIVED, word))
        .is_some_and(|at| includes(&norm[..=at], LETTER_ASKED_FOR));
    let letter_to_other = norm[..verb].iter().enumerate().any(|(index, word)| {
        has(LETTER_SOMEONE_ELSES, word)
            && norm
                .get(index + 1)
                .is_some_and(|after| !has(LETTER_ITSELF, after))
    });
    let written = phrase(&joined, SUMMONS_WRITTEN);
    let letter_first = at > verb
        && written
        && includes(norm, LETTER_RECEIVED)
        && !requests_letter
        && !letter_to_other;
    if at > verb && !letter_first || includes(&norm[verb.saturating_sub(3)..verb], SUMMONS_NOT) {
        return None;
    }
    if letter_first || written {
        return Some(Summons {
            who,
            shape: SummonsShape::Written,
        });
    }
    if let Some(gap) = SUMMONS_RANG
        .iter()
        .find_map(|phrase| joined.split_once(phrase).map(|(_, gap)| gap))
        && gap
            .split_whitespace()
            .take(3)
            .all(|word| !has(SUMMONS_CALL_DONE, word) && !has(SUMMONS_SAID, word))
    {
        return Some(Summons {
            who,
            shape: SummonsShape::Rang,
        });
    }
    (includes(norm, SUMMONS_WANTS) && phrase(&joined, SUMMONS_SEE)).then_some(Summons {
        who,
        shape: SummonsShape::See,
    })
}

fn ask_words<'a>(norm: &[&'a str]) -> Vec<&'a str> {
    let words: Vec<_> = norm
        .iter()
        .map(|word| {
            BOUT.binary_search_by_key(word, |(key, _)| key)
                .ok()
                .map_or(*word, |at| BOUT[at].1)
        })
        .collect();
    let words = &words[lead(&words, FILLER_BEFORE)..];
    words[..finish(words, FILLER_AFTER)].to_vec()
}

static DAY_SHAPES: LazyLock<Vec<ContextRegex>> = LazyLock::new(|| {
    BOT_DAY_SHAPES
        .iter()
        .map(|shape| ContextRegex::new(shape).expect("bot day shape"))
        .collect()
});

fn asks_day(words: &[&str]) -> bool {
    !words.is_empty()
        && DAY_SHAPES
            .iter()
            .any(|shape| matches(shape, &words.join(" ")))
}

pub(super) fn how_i_am(mut words: &[&str]) -> bool {
    while words.len() > 2
        && words[words.len() - 2] == "with"
        && has(GREETING_ASK_SECOND_PERSON, words[words.len() - 1])
    {
        words = &words[..words.len() - 2];
    }
    if words.is_empty() || words.len() > 8 || !includes(words, GREETING_ASK_SUBJECT) {
        return false;
    }
    match words[0] {
        "what" => words.iter().all(|word| has(GREETING_WHAT_SAYING, word)),
        "how" => words.iter().all(|word| has(GREETING_ASK_WORDS, word)),
        _ => {
            words.len() <= 4
                && includes(words, GREETING_ASK_SECOND_PERSON)
                && words
                    .iter()
                    .position(|word| has(GREETING_ASK_STATE, word))
                    .is_some_and(|at| {
                        includes(&words[..at], GREETING_ASK_SECOND_PERSON)
                            && words
                                .get(at + 1)
                                .is_none_or(|word| !has(GREETING_ASK_ABOUT_SOMETHING, word))
                    })
        }
    }
}

fn is_small_talk(norm: &[&str]) -> bool {
    if norm.is_empty() || norm.len() > 10 {
        return false;
    }
    SMALL_TALK_WORDS.iter().any(|words| {
        norm.windows(words.len()).enumerate().any(|(start, span)| {
            span == *words
                && norm[..start]
                    .iter()
                    .chain(&norm[start + words.len()..])
                    .all(|word| has(SMALL_TALK_LEFTOVER, word))
        })
    })
}

fn bot_did(norm: &[&str]) -> bool {
    if !norm
        .iter()
        .any(|word| matches!(*word, "you" | "u" | "ya" | "your" | "yours"))
        || includes(norm, STILL_COMING)
        || !includes(norm, BEEN_SAID)
        || !norm
            .iter()
            .any(|word| matches!(*word, "did" | "done" | "been" | "were" | "was" | "had"))
            && !includes(norm, BEEN_WHEN)
    {
        return false;
    }
    if norm.contains(&"where")
        && includes(norm, SPENT_TIME)
        && norm
            .iter()
            .any(|word| matches!(*word, "have" | "has" | "been" | "were" | "was"))
    {
        return true;
    }
    norm.iter()
        .any(|word| matches!(*word, "do" | "does" | "did" | "done" | "doing"))
        && norm
            .iter()
            .all(|word| has(DOING_FILLER, word) || has(BEEN_WHEN, word) || has(BEEN_SAID, word))
}

fn bot_doing(norm: &[&str]) -> bool {
    let Some(last) = norm.last() else {
        return false;
    };
    if norm.len() > 10
        || !matches!(*last, "to" | "much" | "on")
            && !norm
                .iter()
                .any(|word| matches!(*word, "doing" | "do" | "up"))
        || norm.contains(&"going") && norm.contains(&"on") && includes(norm, GOING_ON_WHEN)
    {
        return false;
    }
    norm.iter().all(|word| has(DOING_FILLER, word)) || bot_did(norm)
}

fn manners(norm: &[&str]) -> bool {
    let norm = &norm[lead(norm, UP_TO_LEADS)..];
    let Some(first) = norm.first() else {
        return false;
    };
    let second_person = norm.iter().any(|word| matches!(*word, "you" | "u" | "ya"));
    if !has(SAYS_WHAT_ARE_YOU, first) && (!has(MANNERS_OPENERS, first) || !second_person)
        || norm.contains(&"been") && norm.iter().any(|word| matches!(*word, "do" | "doing"))
    {
        return false;
    }
    if norm.contains(&"busy") && norm.iter().any(|word| matches!(*word, "keeps" | "keeping")) {
        return norm.iter().all(|word| has(KEEPS_BUSY_SHAPE, word));
    }
    let enquiry = norm
        .iter()
        .any(|word| matches!(*word, "up" | "doing" | "doin" | "at" | "been"));
    (enquiry || norm.contains(&"do"))
        && (enquiry || !includes(norm, WHEN_IN_THE_ASK))
        && norm
            .iter()
            .all(|word| has(DOING_FILLER, word) || has(WHEN_IN_THE_ASK, word))
}

fn asks_back(words: &[&str], polarity: i32, emotion: Option<&str>) -> bool {
    let [before @ .., prior, last] = words else {
        return false;
    };
    if !has(BACK_AT, last) {
        return false;
    }
    if before.is_empty() && has(BACK_JOIN, prior)
        || before.len() == 1 && has(BACK_ASKING, before[0]) && has(BACK_JOIN, prior)
    {
        return true;
    }
    if before
        .last()
        .is_some_and(|word| has(BACK_NOT_A_SUBJECT, word))
        || words[..words.len() - 1]
            .iter()
            .any(|word| has(BACK_AT, word) || matches!(*word, "youre" | "your"))
        || *prior == "about" && !includes(before, BACK_ASKING)
    {
        return false;
    }
    if has(BACK_JOIN, prior) {
        polarity != 0 || emotion.is_some() || includes(before, BACK_AFTER)
    } else {
        has(BACK_AFTER, prior)
    }
}

fn counts_own_day(norm: &[&str]) -> bool {
    let tail = &norm[lead(norm, COUNT_OPENERS)..];
    let ["how", "many" | "much", rest @ ..] = tail else {
        return false;
    };
    if !includes(rest, BOT_COUNTED)
        || !rest
            .iter()
            .any(|word| matches!(*word, "you" | "u" | "ya" | "your" | "ur"))
    {
        return false;
    }
    rest.iter().enumerate().any(|(at, word)| {
        (has(BOT_DOINGS, word) || has(BOT_HAVE, word) && includes(rest, BOT_CHATS))
            && rest[..at]
                .iter()
                .rev()
                .find(|word| !has(COUNT_FILLERS, word) && !word.ends_with("ly"))
                .is_some_and(|word| has(BOT_DOERS, word))
    })
}

fn bot_identity(norm: &[&str]) -> bool {
    let words = &norm[..finish(norm, WHO_TAIL)];
    if words == ["who", "is", "this"] {
        return true;
    }
    match words {
        ["who", "have", "i", "been", doing, "to" | "with", rest @ ..] => {
            rest.len() <= 1 && has(WHO_TALKING, doing)
        }
        ["who", "did" | "was" | "am", "i", doing, "to" | "with"]
        | [
            "who",
            "did" | "was" | "am",
            "i",
            "just",
            doing,
            "to" | "with",
        ] => has(WHO_TALKING, doing),
        [
            "am",
            "i",
            "talking" | "speaking" | "chatting",
            "to" | "with",
            noun,
        ] => matches!(*noun, "a" | "an") || has(WHO_A_THING, noun),
        [
            "am",
            "i",
            "talking" | "speaking" | "chatting",
            "to" | "with",
            "a" | "an",
            noun,
        ] => has(WHO_A_THING, noun),
        ["is", "this" | "it", tail @ ..] if (1..=3).contains(&tail.len()) => {
            let tail = if matches!(tail.first(), Some(&("a" | "an"))) {
                &tail[1..]
            } else {
                tail
            };
            let tail = tail.strip_prefix(&["real"]).unwrap_or(tail);
            !tail.is_empty() && tail.iter().all(|word| has(WHO_A_THING, word))
        }
        _ => false,
    }
}

fn bot_where(norm: &[&str]) -> bool {
    let tail = match norm {
        ["where" | "whereabouts", "are", "you", rest @ ..]
        | ["where" | "whereabouts", "do", "you", "live", rest @ ..]
        | ["where" | "whereabouts", "you", "live", rest @ ..] => rest,
        _ => return false,
    };
    tail.iter().all(|word| has(WHERE_BOT_TAIL, word))
}

fn capability(norm: &[&str]) -> bool {
    let compact: Vec<_> = norm
        .iter()
        .copied()
        .filter(|word| !has(CAPABILITY_FILLER, word))
        .collect();
    for phrase in CAPABILITY {
        let words: Vec<_> = phrase.split_whitespace().collect();
        if let Some(at) = compact.windows(words.len()).position(|span| span == words) {
            let tail = &compact[at + words.len()..];
            return tail.iter().all(|word| has(CAPABILITY_CARRIER, word))
                && !tail
                    .windows(2)
                    .any(|pair| has(REFERRING, pair[0]) && has(POINTED_AT, pair[1]));
        }
    }
    false
}

fn sees_speaker(norm: &[&str]) -> bool {
    let mut words = norm;
    loop {
        let original = words.len();
        for phrase in MOOD_TAIL {
            let end: Vec<_> = phrase.split_whitespace().collect();
            if end.len() < words.len() && words.ends_with(&end) {
                words = &words[..words.len() - end.len()];
            }
        }
        words = &words[lead(words, ME_LEAD)..];
        words = &words[..finish(words, ME_TAIL)];
        if original == words.len() {
            break;
        }
    }
    let plain = words.join(" ");
    has(ME_ASKS, &plain)
        || words.last() == Some(&"my")
            && has(
                ME_ASKS,
                &format!("{} me", words[..words.len() - 1].join(" ")),
            )
}

fn bot_product(norm: &[&str]) -> bool {
    if !(2..=PRODUCT_ASK_WORDS).contains(&norm.len())
        || !matches!(norm[0], "are" | "is")
        || includes(norm, ROBOT_WORDS)
    {
        return false;
    }
    let end = finish(norm, PRODUCT_ASK_TAIL);
    end > 1 && has(ASSISTANT_NAMES, norm[end - 1])
}

fn bot_negative(norm: &[&str], index: usize) -> bool {
    norm.get(index + 1)
        .is_some_and(|word| has(NOT_BEFORE_THE_BOT, word))
        || index >= 1 && has(NOT_AFTER_THE_BOT, norm[index - 1])
        || index >= 2
            && has(NOT_BEFORE_THE_BOT, norm[index - 2])
            && has(NOT_AUX, norm[index - 1])
            && (index < 3 || !has(CLAUSE_OPENERS, norm[index - 3]))
}

fn self_ask(norm: &[&str]) -> bool {
    const SELF: &[&str] = &["you", "your", "yourself", "yours"];
    if norm.is_empty()
        || !norm.iter().any(|word| SELF.contains(word))
        || !includes(norm, SELF_ASK)
        || norm
            .iter()
            .enumerate()
            .any(|(at, word)| has(TASTE_VERBS, word) && includes(&norm[at + 1..], GENDER_WORDS))
        || includes(norm, LEARNED_ON)
            && !norm
                .iter()
                .any(|word| has(SELF_ASK, word) && !has(LEARNED_ON, word))
        || has(JOIN_OPENERS, norm[0]) && includes(norm, MAKER_WORDS)
        || norm.windows(2).any(|pair| {
            has(POSSESSIVE_WORDS, pair[0]) && has(A_PERSON_AFTER_A_POSSESSIVE, pair[1])
                || has(LEARNING_WORDS, pair[0]) && pair[1] == "from"
        })
        || includes(norm, A_HAVING) && includes(norm, CHILD_NOUNS)
        || norm
            .iter()
            .enumerate()
            .skip(1)
            .any(|(at, word)| *word == "not" && !bot_negative(norm, at))
    {
        return false;
    }
    norm.iter().all(|word| {
        matches!(*word, "not")
            || SELF.contains(word)
            || has(SELF_WORDS, word)
            || has(SELF_FILLER, word)
    })
}

fn starts_and_tail(pattern: &ContextRegex, said: &str, words: &[&str]) -> bool {
    pattern.find(said).ok().flatten().is_some_and(|prefix| {
        prefix.start() == 0
            && said[prefix.end()..]
                .split_whitespace()
                .all(|word| has(words, word))
    })
}

pub(crate) fn a_stop_ask(text: &str) -> bool {
    let stop_words: Vec<_> = text.split_whitespace().collect();
    let first = stop_words
        .iter()
        .take_while(|word| has(STOP_ASK_FILLER, trimmed(word)))
        .count();
    let stop = stop_words[first..].join(" ");
    let stop = stop.trim_matches([' ', '.', '!', '?']);
    STOP_ASK
        .find(stop)
        .ok()
        .flatten()
        .is_some_and(|part| part.start() == 0 && part.end() == stop.len())
}

fn bot_kind(text: &str, norm: &[&str], counted: bool) -> Option<AboutBotKind> {
    use AboutBotKind::*;
    let flat = text.replace('|', " ");
    let said = norm.join(" ");
    if matches(&WHILE_ASK, &flat) {
        return Some(While);
    }
    if matches(&KEEPING_ASK, &flat) || a_stop_ask(&flat) {
        return Some(Keeping);
    }
    if bot_where(norm) {
        return Some(Where);
    }
    if bot_identity(norm) {
        return Some(Identity);
    }
    let about_self = self_ask(norm);
    if matches(&ANSWERING_A_QUOTED_ASK, text) && !about_self {
        return None;
    }
    if sees_speaker(norm) {
        return Some(Warmth);
    }
    if norm.contains(&"remember") && (norm.contains(&"you") || norm.contains(&"your")) {
        return Some(Memory);
    }
    if capability(norm) {
        return Some(Help);
    }
    if bot_product(norm) {
        return Some(Model);
    }
    if at_start(&I_SAID_IT, &said) {
        return None;
    }
    let second_person = norm
        .iter()
        .any(|word| matches!(*word, "you" | "u" | "ya" | "your" | "ur"));
    let new = (second_person || norm.contains(&"yours"))
        && norm.len() <= 9
        && includes(norm, NEW_FRAME)
        && !includes(norm, NEW_ASK_AHEAD)
        && norm.iter().all(|word| has(NEW_ASK_WORDS, word));
    let around = second_person
        && norm.len() <= 6
        && !includes(norm, FIRST_PERSON)
        && !includes(norm, MANNERS_WORDS)
        && (includes(norm, ABOUT_AT_A_TIME) || includes(norm, FREE_OR_BUSY));
    let experience = EXPERIENCE_ASK
        .find(&said)
        .ok()
        .flatten()
        .is_some_and(|matched| {
            let rest = said[matched.end()..].trim();
            matched.start() == 0
                && !rest.is_empty()
                && rest.split_whitespace().all(|word| {
                    !has(THEIR_PROBLEM, word)
                        && (has(EXPERIENCE_WORDS, word)
                            || has(BOT_LIFE, word)
                            || has(BOT_LIFE_FILLER, word))
                })
        });
    if new
        || bot_did(norm)
        || around
        || second_person && norm.len() <= 12 && matches(&SPENDS_ITS_TIME, &said)
        || bot_doing(norm)
            && norm.iter().any(|word| matches!(*word, "you" | "u" | "ya"))
            && includes(norm, ASKS_IF_ANYTHING)
        || experience
    {
        return Some(Doings);
    }
    if about_self {
        return Some(
            if norm
                .iter()
                .any(|word| matches!(*word, "remember" | "recall" | "forget"))
            {
                Memory
            } else {
                Identity
            },
        );
    }
    if counted
        || starts_and_tail(&ITSELF_LONG, &said, ITSELF_LIFE)
        || starts_and_tail(&ITSELF_WHEN, &said, ITSELF_WHEN_TAIL)
    {
        return Some(Memory);
    }
    if matches(&A_YOUR_KIN, &said)
        || matches(&A_YOUR_OWN_THING, &said)
        || matches(&A_HOW_MANY_OF_THE_BOTS_OWN, &said) && includes(norm, BOT_LIFE)
    {
        return Some(Life);
    }
    if matches(&A_WHAT_ITS_LIKE, &said) {
        return Some(Identity);
    }
    if let Some(found) = BOT_LIFE_ASK.find(&said).ok().flatten()
        && found.start() == 0
        && (second_person || norm.contains(&"yours") || norm.contains(&"yourself"))
    {
        let rest: Vec<_> = said[found.end()..].split_whitespace().collect();
        if includes(&rest, BOT_LIFE)
            && rest
                .iter()
                .all(|word| has(BOT_LIFE, word) || has(BOT_LIFE_FILLER, word))
        {
            return Some(Life);
        }
    }
    (phrase(&format!(" {} ", text.trim()), ABOUT_BOT)
        || ABOUT_BOT_END.iter().any(|word| text.trim().ends_with(word))
        || (norm.contains(&"you") || norm.contains(&"your"))
            && includes(norm, ROBOT_WORDS)
            && norm.contains(&"are"))
    .then_some(Identity)
}

fn farewell(norm: &[&str]) -> bool {
    let Some(phrase) = FAREWELLS
        .iter()
        .copied()
        .filter(|phrase| starts(norm, phrase))
        .max_by_key(|phrase| phrase.len())
    else {
        return false;
    };
    let rest = &norm[phrase.split_whitespace().count()..];
    if includes(rest, NEEDS_ANSWER) {
        return false;
    }
    if includes(rest, WISH_WORDS)
        && rest
            .iter()
            .filter(|word| !has(WISH_WORDS, word))
            .all(|word| has(FAREWELL_TAIL, word) || has(POLITE, word))
    {
        return true;
    }
    rest.first().is_some_and(|word| has(THANKS_WORDS, word))
        || rest.iter().all(|word| {
            has(FAREWELL_TAIL, word)
                || has(POLITE, word)
                || has(SAID_ON_ITS_OWN, word)
                || has(TERMS_OF_ADDRESS, word)
        })
}

pub(in crate::bot) fn claim(word: &str) -> bool {
    if has(A_THING_NOT_A_CLAIM, word) {
        return true;
    }
    let fits = |form: &str| {
        let stem = if form.len() > 3 {
            form.strip_suffix('s').unwrap_or(form)
        } else {
            form
        };
        has(CLAIM_STEMS, stem)
    };
    let fits_double = |form: &str| {
        fits(form)
            || form.len() > 3
                && form.as_bytes()[form.len() - 2] == form.as_bytes()[form.len() - 1]
                && form.get(..form.len() - 1).is_some_and(fits)
    };
    fits_double(word)
        || [1, 2, 3].into_iter().any(|cut| {
            word.len() > cut + 2 && word.get(..word.len() - cut).is_some_and(fits_double)
        })
        || word
            .strip_suffix("ing")
            .is_some_and(|prefix| fits_double(&format!("{prefix}e")))
}

fn last_subject(words: &[&str]) -> Option<usize> {
    words.iter().rposition(|word| {
        !has(BEFORE_STATE, word)
            && !has(NUMBER_WORDS, word)
            && !(word.len() > 4 && (word.ends_with("ly") || word.ends_with("ing")))
            && !word
                .replace('.', "")
                .bytes()
                .all(|byte| byte.is_ascii_digit())
    })
}

fn is_person(word: &str) -> bool {
    matches!(word, "that" | "this" | "it")
        || [PERSON_SUBJECT, PERSON_NOUNS, MOOD_NOUNS, OTHER_PEOPLE]
            .iter()
            .any(|set| has(set, word))
}

fn takes_a_thing(words: &[&str], at: usize) -> bool {
    !has(EVENT_NOUNS, words[at]) && includes(&words[at.saturating_sub(3)..at], DETERMINERS)
}

fn has_copula(words: &[&str], at: usize) -> bool {
    words[..at]
        .iter()
        .rev()
        .find(|word| has(COPULA, word) || !has(BEFORE_STATE, word))
        .is_none_or(|word| has(COPULA, word))
}

fn whose_state(words: &[&str], pos: usize, head: Option<usize>) -> StateOwner {
    use StateOwner::*;
    let following = &words[pos + 1..];
    let mut index = last_subject(&words[..head.unwrap_or(pos)]);
    if let Some(at) = index
        && has(RELATIVE_SUBJECTS, words[at])
        && let Some(earlier) = last_subject(&words[..at])
    {
        index = Some(earlier);
    }
    let subject = index.map(|at| words[at]);
    if let Some(at) = index
        && has_copula(words, pos)
    {
        let named_other = words[at] == "old" && words[at.saturating_sub(3)..at].contains(&"year");
        if has(OTHER_PEOPLE, words[at]) || named_other {
            return Other;
        }
        return if is_person(words[at]) || !takes_a_thing(words, at) {
            Speaker
        } else {
            Thing
        };
    }
    if following
        .first()
        .is_some_and(|word| has(PERSON_PREPOSITIONS, word))
    {
        return match subject {
            Some(word) if has(OTHER_PEOPLE, word) => Other,
            Some(word) if !is_person(word) => Thing,
            _ => Speaker,
        };
    }
    for &word in following {
        if has(CONJUNCTIONS, word) {
            break;
        }
        if [BETWEEN_STATE_AND_NOUN, TIME_FILLERS, DETERMINERS]
            .iter()
            .any(|set| has(set, word))
            || word.len() > 4 && word.ends_with("ly")
        {
            continue;
        }
        return if word.len() > 4 && word.ends_with("ing")
            || has(PERSON_NOUNS, word)
            || has(MOOD_NOUNS, word)
        {
            Speaker
        } else if has(OTHER_PEOPLE, word) {
            Other
        } else {
            Thing
        };
    }
    match subject {
        Some(word) if has(OTHER_PEOPLE, word) => Other,
        Some(word) if !is_person(word) && index.is_some_and(|at| takes_a_thing(words, at)) => Thing,
        _ => Speaker,
    }
}

fn mood_tail<'a, 'b>(mut words: &'a [&'b str]) -> &'a [&'b str] {
    loop {
        if words.last().is_some_and(|word| has(MOOD_TAIL_WORDS, word)) {
            words = &words[..words.len() - 1];
            continue;
        }
        if let Some(length) = MOOD_TAIL.iter().find_map(|phrase| {
            let suffix: Vec<_> = phrase.split_whitespace().collect();
            (suffix.len() < words.len()
                && words.ends_with(&suffix)
                && !has(MOOD_TAIL_TAKEN, words[words.len() - suffix.len() - 1]))
            .then_some(suffix.len())
        }) {
            words = &words[..words.len() - length];
            continue;
        }
        return words;
    }
}

fn echoed(words: &[&str]) -> Option<StateOwner> {
    if words.is_empty() || words.len() > 6 {
        return None;
    }
    let carried = matches!(words, ["so", verb, ..] if has(BE_FORMS, verb))
        || matches!(words.last(), Some(&("too" | "also")))
        || words.ends_with(&["as", "well"])
        || words.starts_with(&["same", "here"])
        || matches!(words, [subject, verb] if has(PERSON_SUBJECT, subject) && has(BE_FORMS, verb));
    if !carried {
        return None;
    }
    if matches!(words, ["same", "here" | "same", ..] | ["me", "too", ..])
        || includes(words, PERSON_SUBJECT)
    {
        Some(StateOwner::Speaker)
    } else {
        includes(words, OTHER_PEOPLE).then_some(StateOwner::Other)
    }
}

fn thing_turned_down(words: &[&str], pos: usize) -> bool {
    pos >= 2 && has(DETERMINERS, words[pos - 2]) && (pos == 2 || has(MANIPULATE, words[pos - 3]))
}

fn state_owner(clauses: &[Vec<String>], text: &str) -> Option<StateOwner> {
    if works_on_speaker(&text.replace('|', " ")).is_some() {
        return Some(StateOwner::Speaker);
    }
    let mut owner = None;
    for clause in clauses {
        let words: Vec<_> = clause.iter().map(String::as_str).collect();
        let words = mood_tail(&words);
        if owner.is_some()
            && !includes(words, STATE_WORDS)
            && let Some(echoed) = echoed(words)
        {
            if echoed == StateOwner::Speaker {
                return Some(echoed);
            }
            owner = Some(echoed);
        }
        let mut state = None;
        for (pos, &word) in words.iter().enumerate() {
            if !has(STATE_WORDS, word) {
                continue;
            }
            state = Some(pos);
            if has(NOUN_STATES, word)
                && !pos.checked_sub(1).is_some_and(|at| has(DEGREE, words[at]))
                && includes(&words[pos.saturating_sub(2)..pos], DETERMINERS)
            {
                continue;
            }
            if has(PARTICLE_STATES, word) {
                if thing_turned_down(words, pos) {
                    owner = Some(StateOwner::Thing);
                    continue;
                }
                if !has_copula(words, pos) {
                    continue;
                }
            }
            if has(COPULA_STATES, word) && (pos == 0 || !has_copula(words, pos)) {
                continue;
            }
            if word == "fed" && words.get(pos + 1) != Some(&"up") {
                continue;
            }
            let (head, pos) = if word == "fed" {
                (Some(pos), pos + 1)
            } else {
                (None, pos)
            };
            let whose = whose_state(words, pos, head);
            if whose == StateOwner::Speaker {
                return Some(whose);
            }
            owner = Some(whose);
        }
        if owner.is_some() {
            let tail = &words[state.map_or(0, |at| at + 1)..];
            if let Some(echoed) = echoed(&tail[lead(tail, CONJUNCTIONS)..]) {
                if echoed == StateOwner::Speaker {
                    return Some(echoed);
                }
                owner = Some(echoed);
            }
        }
    }
    owner
}

fn works_on_speaker(text: &str) -> Option<&'static str> {
    let words: Vec<_> = text
        .split_whitespace()
        .map(|word| trimmed(word).trim_matches(['(', ')']))
        .collect();
    for (pos, &word) in words.iter().enumerate() {
        let Ok(index) = WORK_ON_ME.binary_search_by_key(&word, |(word, _)| word) else {
            continue;
        };
        let lead = words[pos.saturating_sub(3)..pos]
            .iter()
            .rev()
            .find(|word| !has(ACT_ON_FILLER, word) && !has(ACT_ON_LEAD, word));
        if lead.is_some_and(|word| {
            [
                "i", "i'm", "im", "me", "my", "myself", "we", "our", "not", "never", "no",
            ]
            .contains(word)
                || word.ends_with("nt")
        }) || includes(&words[pos.saturating_sub(4)..pos], PARTS_OF_ME)
        {
            continue;
        }
        if words[pos + 1..]
            .iter()
            .take(3)
            .find(|word| !has(ACT_ON_FILLER, word))
            .is_some_and(|word| matches!(*word, "me" | "us"))
        {
            return Some(WORK_ON_ME[index].1);
        }
    }
    None
}

fn occurrences<'a>(text: &'a str, cue: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
    let word = |ch: char| ch.is_alphanumeric() || ch == '_';
    text.match_indices(cue).filter_map(move |(start, _)| {
        let end = start + cue.len();
        (!text[..start].chars().next_back().is_some_and(word)
            && !text[end..].chars().next().is_some_and(word))
        .then_some((start, end))
    })
}

fn denied(text: &str, cue: &str) -> bool {
    occurrences(text, cue).any(|(start, _)| matches(&DENY_BEFORE, &text[..start]))
}

fn takes_object(rest: &str) -> bool {
    let mut words = rest.split_whitespace().map(trimmed).peekable();
    while let Some(word) = words.next() {
        if word.is_empty() || has(TAIL_STOPS, word) {
            return false;
        }
        if has(OBJECT_MARKERS, word) {
            return !(has(TIME_MARKERS, word)
                && words.peek().is_some_and(|word| has(TIME_FILLERS, word)));
        }
    }
    false
}

fn particle_is_place(words: &[&str], position: usize) -> bool {
    position
        .checked_sub(1)
        .is_some_and(|at| has(MOTION_WORDS, words[at]))
        || words
            .get(position + 1)
            .is_some_and(|word| has(DETERMINERS, word))
            && !includes(
                &words[position + 2..(position + 5).min(words.len())],
                TIME_AFTER_A_PARTICLE,
            )
}

fn state_of_person(text: &str, cue: &str) -> bool {
    for (start, end) in occurrences(text, cue) {
        let mut rest = text[end..].split_whitespace().map(trimmed);
        if has(SUPPLY_WORDS, cue)
            && rest
                .next()
                .is_some_and(|word| has(SUPPLY_PREPOSITIONS, word))
            && rest.next().is_some_and(|word| {
                !has(OTHER_PEOPLE, word) && !has(PERSON_NOUNS, word) && !has(PERSON_SUBJECT, word)
            })
        {
            continue;
        }
        if has(VERB_ONLY_STATE, cue)
            && !text[..start]
                .split_whitespace()
                .rev()
                .map(trimmed)
                .find(|word| !has(FEELING_FILLER, word) && !matches!(*word, "both" | "all"))
                .is_some_and(|word| has(COPULA_BEFORE, word))
        {
            continue;
        }
        if takes_object(&text[end..]) {
            continue;
        }
        let before: Vec<_> = text[..start].split_whitespace().map(trimmed).collect();
        let end = before
            .iter()
            .rposition(|word| !has(FEELING_FILLER, word) && !has(STATE_JOINERS, word))
            .map_or(0, |at| at + 1);
        let Some(subject) = before[..end].last() else {
            return true;
        };
        return has(PERSON_SUBJECT, subject)
            || has(OTHER_PEOPLE, subject)
            || has(MOOD_NOUNS, subject) && before[end.saturating_sub(4)..end - 1].contains(&"my")
            || super::super::body::speaker_before(&text[..start]);
    }
    false
}

fn verb_after(rest: &str) -> bool {
    rest.split_whitespace().take(5).map(trimmed).any(|word| {
        has(CLAUSE_HELPERS, word)
            || has(CLAUSE_VERBS, word)
            || word.len() > 3 && (word.ends_with("ed") || word.ends_with("ing"))
    })
}

fn clause_after(rest: &str) -> bool {
    rest.split_whitespace()
        .take(8)
        .map(trimmed)
        .filter(|word| !word.is_empty())
        .skip(1)
        .any(|word| has(CUE_FOLLOW, word) || has(CLAUSE_VERBS, word) || word.ends_with("ed"))
}

pub(crate) fn cue_about_person(text: &str, cue: &str) -> bool {
    if has(PARTICLE_STATES, cue)
        && occurrences(text, cue).any(|(start, _)| {
            let words: Vec<_> = text.split_whitespace().map(trimmed).collect();
            let at = text[..start].split_whitespace().count();
            at < words.len() && particle_is_place(&words, at)
        })
    {
        return false;
    }
    if cue
        .split_whitespace()
        .next_back()
        .is_some_and(|word| has(CUE_FOLLOW, word))
    {
        return occurrences(text, cue).next().is_some();
    }
    for (start, end) in occurrences(text, cue) {
        let before = text[..start]
            .split_whitespace()
            .next_back()
            .map(trimmed)
            .unwrap_or("");
        if before.ends_with("ing")
            && !has(FEELING_CUES_BEFORE, before)
            && !has(ING_THINGS, before)
            && !has(ING_TAKES_NO_OBJECT, before)
        {
            continue;
        }
        let rest = text[end..].trim_start();
        if rest.is_empty() || rest.starts_with(['.', ',', '!', '?', ';', ':', '-', '|']) {
            return true;
        }
        let following = rest.split_whitespace().next().map(trimmed).unwrap_or("");
        if following.is_empty() || has(CUE_FOLLOW, following) {
            return true;
        }
        if has(STATE_NOUNS, following) {
            let before: Vec<_> = text[..start]
                .split_whitespace()
                .rev()
                .take(4)
                .map(trimmed)
                .collect();
            if matches(
                &STATE_TAIL,
                &before.iter().rev().copied().collect::<Vec<_>>().join(" "),
            ) {
                return true;
            }
        }
        if has(PHRASE_OPENERS, following) && clause_after(rest)
            || super::super::body::speaker_before(&text[..start]) && verb_after(rest)
        {
            return true;
        }
    }
    false
}

fn episode(text: &str, cue: &str) -> bool {
    occurrences(text, cue).any(|(_, end)| {
        text[end..].starts_with(char::is_whitespace)
            && text[end..]
                .split_whitespace()
                .next()
                .map(trimmed)
                .is_some_and(|word| has(EPISODE_NOUNS, word))
    })
}

pub(crate) fn broken_heart(text: &str) -> bool {
    let text = text
        .to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    [
        &*HEART_IN_A_STATE,
        &*BROKEN_HEARTED,
        &*BROKE_A_HEART,
        &*A_BROKEN_HEART,
    ]
    .iter()
    .any(|pattern| matches(pattern, &text))
}

fn owns_broken_heart(text: &str) -> bool {
    if !broken_heart(text) || matches(&BROKE_A_HEART, text) && !matches(&HAVE_A_BROKEN_HEART, text)
    {
        return false;
    }
    if matches(&HAVE_A_BROKEN_HEART, text) {
        return true;
    }
    let ends = ["heart", "hearts", "broken"]
        .into_iter()
        .filter_map(|word| occurrences(text, word).next())
        .map(|(start, _)| start)
        .min()
        .unwrap_or(0);
    let window: Vec<_> = text[..ends].split_whitespace().rev().take(2).collect();
    let owner = window.first().copied().unwrap_or("");
    if has(NOT_THEIR_OWN, owner)
        || owner.ends_with("'s")
        || owner.ends_with('s') && !matches!(owner, "is" | "was" | "this" | "its" | "us")
    {
        return false;
    }
    if window.contains(&"my") || window.contains(&"our") {
        return true;
    }
    if matches!(owner, "a" | "an" | "the") {
        return false;
    }
    text.split_whitespace().next().is_some_and(|word| {
        matches!(
            word,
            "i" | "im" | "i'm" | "ive" | "i've" | "we" | "we're" | "weve" | "we've" | "our"
        )
    }) && !text
        .split_whitespace()
        .any(|word| matches!(trimmed(word), "her" | "his" | "their" | "your"))
}

fn another_heart(text: &str) -> bool {
    if !broken_heart(text) || owns_broken_heart(text) || matches(&A_POSSESSED_HEART, text) {
        return false;
    }
    if at_start(&A_PERSON_IS_THE_SUBJECT, text) {
        return true;
    }
    let words: Vec<_> = text.split_whitespace().take(3).map(trimmed).collect();
    matches!(words.as_slice(),["my"|"our"|"the",person,rest @ ..] if
        (has(A_PERSON_ELSE,person) || person.get(..person.len().saturating_sub(1)).is_some_and(|name| has(A_PERSON_ELSE,name)))
        && !matches!(rest.first(),Some(&("heart"|"hearts"))))
}

static MOOD_IDIOMS: LazyLock<Vec<(&str, ContextRegex)>> = LazyLock::new(|| {
    NOT_A_MOOD_THERE
        .iter()
        .map(|&(cue, shape)| (cue, ContextRegex::new(shape).expect("mood idiom")))
        .collect()
});
static ACTED: LazyLock<Vec<(ContextRegex, &str)>> = LazyLock::new(|| {
    ACTS_ON_THE_SPEAKER
        .iter()
        .map(|&(shape, name)| (ContextRegex::new(shape).expect("speaker emotion"), name))
        .collect()
});

fn acted_on_speaker(text: &str) -> Option<&'static str> {
    if matches(&LAUGH_AT_THE_END, text) {
        return None;
    }
    for (pattern, name) in &*ACTED {
        for hit in pattern.find_iter(text).flatten() {
            if !text[..hit.start()]
                .split_whitespace()
                .rev()
                .take(2)
                .map(trimmed)
                .any(|word| has(A_BODY_SUBJECT, word))
            {
                return Some(name);
            }
        }
    }
    None
}

fn period_owned(text: &str, cue: &str) -> bool {
    occurrences(text, cue).any(|(start, end)| {
        let noun = text[end..]
            .split_whitespace()
            .map(trimmed)
            .enumerate()
            .find(|(at, word)| *at >= 3 || !has(OWN_PERIOD_FILLER, word))
            .map(|(_, word)| word);
        noun.is_some_and(|noun| has(OWN_PERIOD_NOUNS, noun))
            && text[..start]
                .split_whitespace()
                .rev()
                .take(OWN_PERIOD_REACH)
                .map(trimmed)
                .take_while(|word| !has(OWN_PERIOD_STOPS, word))
                .any(|word| has(OWN_PERIOD_HEADS, word))
    })
}

fn skeleton(word: &str) -> String {
    word.chars()
        .filter(|ch| ch.is_alphabetic() && !"aeiouy".contains(*ch))
        .collect()
}

pub(super) fn sound(word: &str) -> Option<&'static str> {
    let word = trimmed(word).trim_matches(['(', ')']);
    if word.len() < 4
        || !word.chars().all(char::is_alphabetic)
        || crate::bot::memory::recognized_name(word)
        || EMOTION_WORDS.iter().any(|(_, cues)| cues.contains(&word))
    {
        return None;
    }
    let bones = skeleton(word);
    let mut names = EMOTION_WORDS.iter().filter_map(|(name, cues)| {
        cues.iter()
            .any(|cue| {
                !cue.contains(' ')
                    && cue.len() >= 4
                    && cue.as_bytes()[0] == word.as_bytes()[0]
                    && skeleton(cue) == bones
            })
            .then_some(*name)
    });
    names.next().filter(|_| names.next().is_none())
}

fn emotion(text: &str) -> Option<&'static str> {
    let text = replace(&BETTER_OFF, text, " ");
    let text = replace(&NO_BETTER, &text, "worse");
    let text = replace(&GETTING_WORSE, &text, "${1} worse");
    let text = replace(&IDIOM_STATES, &text, " ");
    let text = replace(&ABILITY_AT, &text, " ");
    for &(name, cues) in EMOTION_WORDS {
        if has(GOOD_EMOTION_NAMES, name)
            && matches(&GOOD_WORD_MISSING, &text)
            && !matches(&GOOD_WORD_COMPARED, &text)
            && !matches(&GOOD_WORD_BY_COMPARISON, &text)
        {
            return Some("sad");
        }
        for &cue in cues {
            if occurrences(&text, cue).next().is_none() {
                continue;
            }
            let state = if has(SELF_ONLY_STATE, cue) {
                Some(cue)
            } else {
                cue.split_whitespace()
                    .next()
                    .filter(|head| has(SELF_ONLY_STATE, head))
            };
            if state.is_some_and(|state| !state_of_person(&text, state)) {
                continue;
            }
            if has(BODY_STATE_CUES, cue)
                && occurrences(&text, cue).any(|(start, end)| {
                    text[..start]
                        .split_whitespace()
                        .rev()
                        .map(trimmed)
                        .find(|word| *word != "is" && !has(FEELING_FILLER, word))
                        .is_some_and(|word| has(BODY_SUBJECT, word))
                        || takes_object(&text[end..])
                })
            {
                continue;
            }
            if MOOD_IDIOMS
                .iter()
                .any(|(word, pattern)| *word == cue && matches(pattern, &text))
            {
                continue;
            }
            let speaker = occurrences(&text, cue)
                .any(|(start, _)| super::super::body::speaker_before(&text[..start]));
            if has(LABEL_ON_A_PERSON, cue) && speaker {
                continue;
            }
            if [
                SELF_ONLY_CUES,
                ALWAYS_SELF_ONLY,
                THING_OR_PERSON_CUES,
                CONFUSED_SELF_ONLY,
                A_COMPARISON,
            ]
            .iter()
            .any(|set| has(set, cue))
                && !speaker
            {
                continue;
            }
            if has(THING_OR_PERSON_CUES, cue)
                && occurrences(&text, cue)
                    .any(|(_, end)| at_start(&LAUGHED_OFF_AFTER, text[end..].trim()))
            {
                continue;
            }
            if has(PERIOD_CUES, cue)
                && matches(&A_PERIOD, &text)
                && !matches(&A_LIMIT_ABOUT_PEOPLE, &text)
                && (matches(&A_MONTH_PERIOD, &text) || matches(&A_MONEY_SUBJECT, &text))
            {
                continue;
            }
            let negation = denied(&text, cue);
            if has(SAID_AS_A_NEGATION, cue) {
                if !negation {
                    continue;
                }
            } else if !has(GOOD_EMOTION_NAMES, name) && negation {
                continue;
            }
            if cue_about_person(&text, cue) || episode(&text, cue) {
                return Some(name);
            }
        }
    }
    if let Some(emotion) = acted_on_speaker(&text) {
        return Some(emotion);
    }
    if broken_heart(&text) {
        return Some("sad");
    }
    for frame in FEELING_SOUND_FRAME.captures_iter(&text).flatten() {
        let Some(found) = frame.get(0) else {
            continue;
        };
        let rest = text[found.end()..].trim_start();
        if !takes_object(rest)
            && (rest.is_empty()
                || rest.starts_with(['.', ',', '!', '?', ';', ':'])
                || rest
                    .split_whitespace()
                    .next()
                    .map(trimmed)
                    .is_some_and(|word| has(SOUND_FOLLOWS, word)))
            && let Some(emotion) = frame.get(1).and_then(|word| sound(word.as_str()))
        {
            return Some(emotion);
        }
    }
    if let Some(emotion) = works_on_speaker(&text) {
        return Some(emotion);
    }
    let words: Vec<_> = text.split_whitespace().map(trimmed).collect();
    if words.iter().enumerate().any(|(pos, word)| {
        has(STATE_AS_FEELING, word)
            && !thing_turned_down(&words, pos)
            && words[..pos]
                .iter()
                .rev()
                .find(|word| !has(FEELING_FILLER, word))
                .is_some_and(|word| has(FEELING_COPULA, word))
            && !particle_is_place(&words, pos)
            && words
                .get(pos + 1)
                .is_none_or(|word| !has(FEELING_NOT_MOOD, word))
            && matches!(
                whose_state(&words, pos, None),
                StateOwner::Speaker | StateOwner::Other
            )
    }) {
        return Some("sad");
    }
    for &(name, cues) in EMOTION_WORDS {
        if !has(GOOD_EMOTION_NAMES, name)
            && cues.iter().any(|cue| {
                !has(NOT_THE_SPEAKERS_DAY, cue)
                    && !has(PERIOD_CUES, cue)
                    && !denied(&text, cue)
                    && period_owned(&text, cue)
            })
        {
            return Some(name);
        }
    }
    None
}

struct DialogueCues {
    laughter: bool,
    bot: Option<AboutBotKind>,
    emotion: Option<&'static str>,
    turned_back: bool,
    agreement: Option<&'static str>,
    trying: bool,
}

fn dialogue_intent(
    norm: &[&str],
    text: &str,
    kind: Kind,
    asks: &intent::Asks,
    cues: DialogueCues,
) -> Intent {
    use Intent::*;
    if kind == Kind::Empty {
        return Empty;
    }
    if cues.laughter && norm.len() <= 5 {
        return Laugh;
    }
    if farewell(strip_leaders(norm)) {
        return Farewell;
    }
    if cues.bot.is_some() {
        return AboutBot;
    }
    if kind == Kind::Greeting {
        return Greeting;
    }
    if kind == Kind::Thanks {
        return Thanks;
    }
    if phrase(&format!(" {text} "), CORRECTION)
        && (norm.contains(&"meant") || norm.contains(&"mean"))
    {
        return Correction;
    }
    if cues.turned_back || bot_doing(norm) || manners(norm) || how_i_am(norm) {
        return Smalltalk;
    }
    if cues.agreement.is_some()
        || norm.len() <= 6 && SHORT_REPLIES.iter().any(|phrase| starts(norm, phrase))
        || kind == Kind::Statement && cues.trying
    {
        return Ack;
    }
    if matches!(norm[0], "no" | "nah" | "nope" | "never")
        && norm.len() <= 5
        && norm.get(1) != Some(&"one")
    {
        return Deny;
    }
    if norm.len() <= 4 && has(ACK, norm[0]) {
        return Ack;
    }
    let bare: Vec<_> = norm
        .iter()
        .copied()
        .filter(|word| !has(POLITE, word))
        .collect();
    if bare.len() <= 3 {
        if bare.iter().all(|word| has(AFFIRM, word)) {
            return Affirm;
        }
        if bare.iter().all(|word| has(DENY, word)) {
            return Deny;
        }
        if bare.iter().all(|word| has(ACK, word)) || bare.iter().all(|word| has(LAUGH, word)) {
            return Ack;
        }
    }
    if is_small_talk(norm) {
        return Smalltalk;
    }
    if kind == Kind::Question {
        return Question;
    }
    if asks.contains("recommend")
        || phrase(text, STEPS)
        || norm
            .iter()
            .any(|word| matches!(*word, "help" | "need" | "want"))
        || matches!(
            norm[0],
            "tell" | "explain" | "show" | "give" | "write" | "make"
        )
    {
        return Request;
    }
    if norm.contains(&"i")
        && norm.iter().enumerate().any(|(index, &word)| {
            has(POSITIVE, word) && !(has(NEGATIVE, word) && denied(text, word))
                || has(NEGATIVE, word) && !denied(text, word)
                || has(SAYS_HOW_THEY_ARE, word) && state_of_speaker(norm, index)
        })
        || cues.emotion.is_some() && (norm.contains(&"i") || acted_on_speaker(text).is_some())
    {
        return Feeling;
    }
    if phrase(&format!(" {text} "), NEWS)
        || norm
            .iter()
            .any(|word| matches!(*word, "i" | "me" | "my" | "we" | "our"))
            && (norm.len() <= 12 || includes(norm, EVENT_WORDS))
    {
        return News;
    }
    if phrase(text, OPINION) {
        return Opinion;
    }
    Statement
}

pub(super) fn initial_asks(
    line: &Line,
    norm: &[&str],
    functional: &[&str],
) -> (intent::Asks, bool) {
    let mut asks = intent::raw_ask_labels(line);
    let quiz = asks_quiz(functional);
    if question_coming(norm, &line.norm_text, quiz) {
        asks.insert("question_coming");
    }
    (asks, quiz)
}

/// Derive the classification details consumed by dialogue and topic readers.
pub(super) fn features(
    line: &Line,
    norm: &[&str],
    functional: &[&str],
    kind: Kind,
    mut asks: intent::Asks,
    tag: bool,
    quiz: bool,
) -> KindFeatures {
    let text = line.norm_text.as_str();
    let ask_words = ask_words(norm);
    let turned_back = has(ASKS_IT_BACK, &ask_words.join(" ")) || asks_day(&ask_words);
    if turned_back {
        asks.clear();
    }
    if kind != Kind::Question {
        remove_background_labels(norm, &mut asks);
    } else if tag || !turned_back && super::yesno_clause(line) {
        asks.insert("yesno");
    }

    let counted = counts_own_day(norm);
    let bot = bot_kind(text, norm, counted);
    let polarity = polarity(norm);
    let agreement = agreement(text);
    let trying = agrees_to_try(functional);
    let heard_emotion = emotion(text);
    let emotion_named =
        norm.iter().any(|word| matches!(*word, "i" | "im" | "my")) || kind != Kind::Question;
    let mut state_owner = state_owner(&line.clauses, text);
    let mut emotion = emotion_named.then_some(heard_emotion).flatten();
    if matches!(state_owner, None | Some(StateOwner::Thing)) {
        if another_heart(text) {
            state_owner = Some(StateOwner::Other);
        } else if emotion.is_some() && owns_broken_heart(text) {
            state_owner = Some(StateOwner::Speaker);
        }
    }
    if emotion.is_none()
        && hedged_low(if line.clean.is_empty() {
            text
        } else {
            &line.clean
        })
    {
        emotion = Some("sad");
        if matches!(state_owner, None | Some(StateOwner::Thing)) {
            state_owner = Some(StateOwner::Speaker);
        }
    }
    let shortage = shortage(text);
    if shortage.is_some() && emotion.is_none() {
        state_owner = None;
    }
    let intent = dialogue_intent(
        norm,
        text,
        kind,
        &asks,
        DialogueCues {
            laughter: line.signals.laughter,
            bot,
            emotion: heard_emotion,
            turned_back,
            agreement,
            trying,
        },
    );
    let excludes: Vec<_> = limit(text).into_iter().collect();
    let asks_opinion = asks.contains("opinion");
    let asks_recommendation = asks.contains("recommend");
    let asks_steps = asks.contains("steps");
    KindFeatures {
        intent,
        asks: asks.into_iter().collect(),
        tag,
        emotion,
        state_owner,
        shortage,
        polarity,
        short: norm.len() <= 3,
        first_person: norm
            .iter()
            .any(|word| matches!(*word, "i" | "me" | "my" | "we" | "our" | "us" | "myself")),
        asks_opinion,
        asks_recommendation,
        asks_steps,
        help_ask: asks_help(text),
        asks_a_verdict: intent::verdict_ask(text),
        about_bot: intent == Intent::AboutBot,
        about_bot_kind: (intent == Intent::AboutBot).then_some(bot).flatten(),
        counts_the_bots_own_day: counted,
        replies_to_bot: matches!(
            intent,
            Intent::Affirm | Intent::Deny | Intent::Ack | Intent::Laugh
        ),
        agrees_to_try: trying,
        wants_a_quiz: quiz,
        agreement,
        summons: summons(norm, kind),
        asks_back: asks_back(functional, polarity, heard_emotion),
        asks_what_was_meant: at_start(&ASKS_MEANING, text.trim()),
        ruled_out: ruled_out(text),
        constraint: !excludes.is_empty(),
        excludes,
        suggestion: suggestion(norm),
    }
}

pub(in crate::bot::understanding) fn gratitude_span(
    norm: &[&str],
) -> Option<std::ops::Range<usize>> {
    if !THANKS.iter().any(|phrase| starts(norm, phrase)) {
        return None;
    }
    let is_gratitude = |word| {
        [GRATITUDE_OBJECT, RECOMMEND, TIP_WORDS]
            .iter()
            .any(|set| has(set, word))
    };
    for (at, &word) in norm.iter().take(5).enumerate() {
        if word != "for"
            || norm[..at]
                .iter()
                .any(|word| !has(COURTESY_WORD, word) && !has(GRATITUDE_OBJECT, word))
        {
            continue;
        }
        let mut end = at + 1;
        while end < norm.len() && is_gratitude(norm[end]) {
            if end > at + 1 && has(GRATITUDE_ASK_AGAIN, norm[end]) {
                break;
            }
            end += 1;
        }
        if end == at + 1 {
            continue;
        }
        while end + 1 < norm.len() && has(GRATITUDE_JOINERS, norm[end]) {
            let from = end + 1;
            let count = norm[from..]
                .iter()
                .take_while(|word| is_gratitude(word))
                .count();
            if count == 0 || norm.get(from + count) == Some(&"for") {
                break;
            }
            end = from + count;
        }
        return Some(at..end);
    }
    None
}

pub(in crate::bot::understanding) fn remove_background_labels(
    norm: &[&str],
    asks: &mut intent::Asks,
) {
    let clause_end = |start: usize| {
        norm[start + 1..]
            .iter()
            .position(|word| has(BACKGROUND_BREAKS, word))
            .map_or(norm.len(), |stop| start + 1 + stop)
    };
    let background = norm.iter().enumerate().skip(1).find_map(|(at, &word)| {
        (has(BACKGROUND_STARTS, word) && !has(BACKGROUND_REPORTING, norm[at - 1]))
            .then(|| at..clause_end(at))
    });
    if let Some(background) = &background {
        for word in &norm[background.clone()] {
            asks.remove(*word);
        }
    }
    let beginning =
        matches!(norm, ["it", verb, ..] if has(BACKGROUND_VERBS,verb)).then(|| 0..clause_end(1));
    for (span, anywhere) in beginning
        .into_iter()
        .map(|span| (span, true))
        .chain(background.into_iter().map(|span| (span, false)))
    {
        let mut at = span.start;
        let mut last_skipped = None;
        while at < span.end {
            if !has(BACKGROUND_PLACE_PREPS, norm[at])
                || !anywhere
                    && at > 0
                    && !has(BACKGROUND_PLACE_LEADS, norm[at - 1])
                    && last_skipped != Some(at - 1)
            {
                at += 1;
                continue;
            }
            at += 1;
            for _ in 0..3 {
                if at >= span.end
                    || [
                        BACKGROUND_BREAKS,
                        BACKGROUND_PLACE_STOP,
                        BACKGROUND_PLACE_PREPS,
                    ]
                    .iter()
                    .any(|set| has(set, norm[at]))
                {
                    break;
                }
                asks.remove(norm[at]);
                last_skipped = Some(at);
                at += 1;
            }
        }
    }
}
