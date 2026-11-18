//! Situation, interests, work, visits, and routine cues from a single turn.

use super::{Intent, Kind, has, opens_question};
use crate::bot::understanding::{entities, line::Line, line::strip_leaders};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "kinds_extra_data.rs"]
mod data;
use data::*;

/// Situation described in the clauses preceding an ask.
pub(crate) fn setup(line: &Line) -> Option<String> {
    let clauses: Vec<_> = line
        .clauses
        .iter()
        .filter(|clause| !clause.is_empty())
        .collect();
    let ask_at = clauses
        .iter()
        .enumerate()
        .skip(1)
        .rev()
        .find(|(_, clause)| {
            let words = strip_leaders(clause);
            opens_question(&words.iter().map(String::as_str).collect::<Vec<_>>())
                || words.iter().any(|word| has(TAIL_ASKS, word))
                    && words
                        .iter()
                        .all(|word| has(TAIL_ASKS, word) || has(TAIL_ASK_FILLER, word))
        })?
        .0;
    let before = &clauses[..ask_at];
    if before
        .iter()
        .any(|clause| opens_question(&clause.iter().map(String::as_str).collect::<Vec<_>>()))
        || before.iter().all(|clause| {
            strip_leaders(clause)
                .iter()
                .all(|word| has(SETUP_NOTHING, word) || has(SETUP_CONNECTIVES, word))
        })
    {
        return None;
    }
    let setup = before
        .iter()
        .flat_map(|clause| strip_leaders(clause).iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" ");
    let stripped = A_FRAME_IN_FRONT.replace(&setup, "");
    let setup = stripped.trim_matches([' ', ',', '-']);
    if setup.is_empty()
        || setup.split_whitespace().count() > 45
        || FRAME_NOT_A_SETUP.is_match(&format!(" {setup}"))
        || setup
            .split_whitespace()
            .all(|word| has(SETUP_CONNECTIVES, word))
    {
        return None;
    }
    Some(setup.into())
}

/// Whether a complete turn asks for room to talk without naming a topic yet.
pub(crate) fn talk_frame(norm: &[String]) -> bool {
    let words: Vec<_> = norm
        .iter()
        .map(String::as_str)
        .filter(|word| !word.is_empty())
        .collect();
    let mut remaining = &words[..];
    loop {
        let old_len = remaining.len();
        if remaining.first().is_some_and(|word| has(TALK_HEAD, word)) {
            remaining = &remaining[1..];
        }
        if let Some(tail_len) = TALK_TAIL.iter().find_map(|phrase| {
            let tail: Vec<_> = phrase.split_whitespace().collect();
            (remaining.len() > tail.len() + 1 && remaining.ends_with(&tail)).then_some(tail.len())
        }) {
            remaining = &remaining[..remaining.len() - tail_len];
        }
        if remaining.len() == old_len {
            break;
        }
    }
    !remaining.is_empty() && TALK_OPENING.is_match(&remaining.join(" "))
}

/// Whether the complete turn offers a question, requests advice in general, or asks for room to talk.
pub(crate) fn opening_line(norm: &[String]) -> bool {
    talk_frame(norm)
        || (1..=9).contains(&norm.len())
            && !matches!(norm[0].as_str(), "have" | "do" | "does" | "did")
            && norm.iter().all(|word| has(OPENING_WORDS, word))
            && OPENING_FRAME.is_match(&norm.join(" "))
}

/// Whether a line asks for the bot's view without naming a subject.
pub(crate) fn asks_with_nothing_named(norm: &[String]) -> bool {
    const BOT: &[&str] = &["u", "ya", "you", "your", "yours", "yourself"];
    const VIEW: &[&str] = &[
        "advice",
        "advise",
        "idea",
        "ideas",
        "opinion",
        "reckon",
        "recommend",
        "recommendation",
        "say",
        "suggest",
        "suggestion",
        "take",
        "think",
        "thought",
        "thoughts",
        "view",
    ];
    const ASK: &[&str] = &[
        "about",
        "any",
        "anything",
        "are",
        "can",
        "could",
        "did",
        "do",
        "does",
        "else",
        "for",
        "how",
        "i",
        "in",
        "is",
        "it",
        "me",
        "more",
        "of",
        "on",
        "should",
        "something",
        "that",
        "this",
        "to",
        "we",
        "what",
        "when",
        "where",
        "which",
        "who",
        "why",
        "will",
        "with",
        "would",
    ];
    const FILLER: &[&str] = &[
        "actually", "and", "but", "first", "here", "next", "now", "ok", "okay", "please", "pls",
        "right", "so", "then", "well",
    ];
    let words: Vec<_> = norm
        .iter()
        .map(String::as_str)
        .filter(|word| !matches!(*word, "" | "|"))
        .collect();
    words.iter().any(|word| has(BOT, word))
        && words.iter().any(|word| has(VIEW, word))
        && words
            .iter()
            .all(|word| has(BOT, word) || has(VIEW, word) || has(ASK, word) || has(FILLER, word))
        && !matches!(words.as_slice(), ["can" | "could", subject, ..] if has(BOT, subject))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum InterestShape {
    Started,
    Doing,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub(crate) struct Interest {
    pub(crate) text: String,
    pub(crate) words: Vec<String>,
    pub(crate) shape: InterestShape,
    pub(crate) back: bool,
}

/// The activity the speaker says they enjoy or have taken up.
pub(crate) fn taking_up(line: &Line) -> Option<Interest> {
    static INVERTED_SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?i)\A\W*(?:am|is|are|was|were)\s+(?:i|we|you|they|he|she)\b").unwrap()
    });
    let source = line.text.replace(['\'', '’'], "");
    let source = source.trim();
    if source.is_empty()
        || format!(" {} ", source.to_lowercase()).contains(" not ")
        || INVERTED_SUBJECT.is_match(source)
    {
        return None;
    }
    let capture = INTEREST_HEAD.captures(source)?.name("thing")?.as_str();
    let captured = capture.trim_matches([' ', '.', ',', '!', '?']);
    let interest = INTEREST_TAIL.replace(captured, "");
    let interest = interest.trim();
    let words: Vec<_> = interest.split_whitespace().collect();
    if !(1..=4).contains(&words.len()) || words.iter().take(2).any(|word| has(INTEREST_NOT, word)) {
        return None;
    }
    Some(Interest {
        text: interest.to_lowercase(),
        words: words.into_iter().map(str::to_owned).collect(),
        shape: if STARTED_OUT.is_match(source) {
            InterestShape::Started
        } else {
            InterestShape::Doing
        },
        back: BACK_INTO.is_match(source),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Shift {
    Early,
    Late,
    Night,
}

/// The shift pattern the statement names.
pub(crate) fn shift_worked(text: &str) -> Option<Shift> {
    [
        (Shift::Night, &*SHIFT_NIGHT),
        (Shift::Late, &*SHIFT_LATE),
        (Shift::Early, &*SHIFT_EARLY),
    ]
    .into_iter()
    .find_map(|(shift, pattern)| pattern.is_match(text).then_some(shift))
}

/// Whether the speaker is describing a habit they still keep.
pub(crate) fn is_a_routine(line: &Line, kind: Kind, asks: &[&str]) -> bool {
    if kind != Kind::Statement || !asks.is_empty() {
        return false;
    }
    let text = [&line.plain, &line.clean, &line.text]
        .into_iter()
        .find(|text| !text.is_empty())
        .map_or("", |text| text.trim());
    !text.is_empty()
        && !text.ends_with('?')
        && ROUTINE_SUBJECT.is_match(text)
        && !ROUTINE_IS_A_FAULT.is_match(text)
        && !ROUTINE_IN_THE_PAST.is_match(text)
        && ROUTINE_SHAPE.is_match(text)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CareShape {
    Guilt,
    Visiting,
    Moved,
    In,
    Bring,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum CarePronoun {
    He,
    She,
    They,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub(crate) struct CareHome {
    pub(crate) text: String,
    pub(crate) who: String,
    pub(crate) relation: Option<String>,
    pub(crate) pronoun: CarePronoun,
    pub(crate) shape: CareShape,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) carried: Option<String>,
}

/// The person described as living in a care home, with the turn's concern there.
pub(crate) fn a_care_home(text: &str) -> Option<CareHome> {
    static NAMED: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\b(?:my|our|your)\s+([a-z][a-z-]*)").unwrap());
    static PRONOUN: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\b(?:shes|hes|she|he|they|theyre)\b").unwrap());
    let source = text.replace(['\'', '’'], "");
    let source = source.trim();
    let place = CARE_HOME_PLACE.find(source)?;
    if CARE_HOME_NOT.is_match(source) {
        return None;
    }
    let lowered = source.to_lowercase();
    let relation = NAMED.captures_iter(&lowered).find_map(|capture| {
        let relation = capture.get(1)?.as_str();
        (entities::is_relation(relation) || has(CARE_HOME_KIN, relation))
            .then(|| (capture.get(0).unwrap().as_str(), relation))
    });
    let (who, relation) = if let Some((who, relation)) = relation {
        (who, Some(relation))
    } else {
        let spoken: Vec<_> = PRONOUN.find_iter(&lowered).map(|m| m.as_str()).collect();
        (
            ["shes", "hes", "she", "he", "they", "theyre"]
                .into_iter()
                .find(|word| spoken.contains(word))?,
            None,
        )
    };
    let pronoun = match relation {
        Some(
            "husband" | "father" | "dad" | "son" | "brother" | "uncle" | "cousin" | "grandad"
            | "grandpa" | "grandfather" | "granddad" | "partner" | "boyfriend" | "friend"
            | "male friend" | "nephew" | "stepdad" | "stepfather",
        ) => CarePronoun::He,
        Some(
            "wife" | "mother" | "mum" | "mom" | "daughter" | "sister" | "aunt" | "gran" | "granny"
            | "grandma" | "grandmother" | "girlfriend" | "niece" | "stepmum" | "stepmother" | "nan"
            | "nana" | "mam" | "mammy" | "grannie" | "mother-in-law" | "sister-in-law"
            | "daughter-in-law",
        ) => CarePronoun::She,
        _ if matches!(who, "he" | "hes") => CarePronoun::He,
        _ if matches!(who, "she" | "shes") => CarePronoun::She,
        _ => CarePronoun::They,
    };
    let shape = if CARE_HOME_GUILT.is_match(&lowered) {
        CareShape::Guilt
    } else if CARE_HOME_VISIT.is_match(&lowered) {
        CareShape::Visiting
    } else if CARE_HOME_MOVED.is_match(&lowered) {
        CareShape::Moved
    } else {
        CareShape::In
    };
    Some(CareHome {
        text: place
            .as_str()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
        who: who.split_whitespace().collect::<Vec<_>>().join(" "),
        relation: relation.map(str::to_owned),
        pronoun,
        shape,
        carried: None,
    })
}

/// The recent care-home subject invoked by a short question about bringing something to a visit.
pub(crate) fn a_home_carried_forward<'a>(
    text: &str,
    newest_first: impl IntoIterator<Item = &'a str>,
) -> Option<CareHome> {
    let text = text.replace(['\'', '’'], "");
    let text = text.trim();
    if a_care_home(text).is_some()
        || !CARE_HOME_BRING_ASKS.is_match(text)
        || !CARE_HOME_BRING_WORD.is_match(text)
        || !CARE_HOME_BRING_WHO.is_match(text)
    {
        return None;
    }
    newest_first.into_iter().find_map(|earlier| {
        let mut home = a_care_home(earlier)?;
        home.shape = CareShape::Bring;
        home.carried = Some(earlier.into());
        Some(home)
    })
}

/// Whether the normalized opening is an exchange of greetings with a question about the bot.
pub(crate) fn asks_how_i_am(norm: &[String], intent: Intent) -> bool {
    matches!(intent, Intent::Greeting | Intent::Smalltalk)
        && super::feature::how_i_am(&norm.iter().map(String::as_str).collect::<Vec<_>>())
}
