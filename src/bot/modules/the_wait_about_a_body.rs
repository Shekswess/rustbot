//! Replaces a generic acknowledgment of a medical wait or significant health news.

use std::sync::LazyLock;

use fancy_regex::Regex as FancyRegex;
use regex::Regex;

use crate::bot::{Memory, Reading, compose};

#[path = "the_wait_about_a_body_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(data::$name).expect(stringify!($name))
        });)+
    };
}

patterns!(
    OWN_SERIOUS_CONDITION,
    A_FINDING,
    ITS_THE_SPEAKERS,
    NOT_ABOUT_THEIR_BODY,
    A_SCARE,
    A_GOOD_TURN,
    NOT_LANDED,
    A_WORRY,
    A_MEDICAL_TEST,
    NOT_A_MEDICAL_TEST,
    AN_ANIMAL,
    A_QUESTION_ABOUT_IT,
    RULED_OUT,
    NEGATED_CONDITION,
);

static RELATIVE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"\b(?:my|our)\s+(?:{})(?:'?s)?\b", data::KIN))
        .expect("medical report relative")
});
static A_BENIGN_FINDING: LazyLock<FancyRegex> =
    LazyLock::new(|| FancyRegex::new(data::A_BENIGN_FINDING).expect("benign finding"));
static QUESTION_START: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:is|are|was|were|did|does|do|has|have|had|can|could|would|will|should|any)\b")
        .expect("question rather than news")
});
static WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-z0-9']+").expect("acknowledgment words"));
static RULED_OUT_TAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("^{}", data::RULED_OUT_TAIL)).expect("end of the health report")
});

const GENERIC: &[&str] = &[
    "that sounds rough",
    "ugh, that's a lot to carry",
    "i'm sorry — that's frustrating",
    "i'm sorry - that's frustrating",
    "i'm sorry, that's frustrating",
    "sorry to hear that",
    "i'm sorry to hear that",
    "i'm listening",
    "that's a lot to carry",
    "noted: ",
];
const ECHOES: &[&str] = &["got it — ", "got it - ", "got it, ", "got it: ", "noted: "];

fn normalized(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    WORD.find_iter(text).map(|word| {
        let word = word.as_str();
        if word == "my" { "your" } else { word }
    })
}

fn placeholder(reply: &str, clean: &str) -> bool {
    let reply = normalized(reply);
    let family = reply.strip_prefix("same as before: ").unwrap_or(&reply);
    GENERIC.iter().any(|prefix| family.starts_with(prefix))
        || ECHOES.iter().any(|prefix| {
            reply
                .strip_prefix(prefix)
                .is_some_and(|said| words(said).next().is_some() && words(said).eq(words(clean)))
        })
}

fn reports_own_condition(clean: &str, condition: Option<regex::Match<'_>>) -> bool {
    let Some(condition) = condition.or_else(|| {
        A_FINDING
            .find(clean)
            .filter(|_| !A_BENIGN_FINDING.is_match(clean).unwrap_or(false))
    }) else {
        return false;
    };
    ITS_THE_SPEAKERS.find(clean).is_some_and(|speaker| {
        NOT_LANDED
            .find(clean)
            .is_none_or(|not_landed| not_landed.start() >= condition.start())
            && RELATIVE
                .find(clean)
                .is_none_or(|relative| relative.start() >= speaker.start())
    })
}

fn pool(clean: &str) -> Option<&'static [&'static str]> {
    let question = clean.contains('?')
        || QUESTION_START.is_match(clean)
        || A_QUESTION_ABOUT_IT.is_match(clean);
    if !question
        && [&*RULED_OUT, &*NEGATED_CONDITION].iter().any(|pattern| {
            pattern
                .find_iter(clean)
                .any(|found| RULED_OUT_TAIL.is_match(&clean[found.end()..]))
        })
    {
        return Some(data::THE_THING_RULED_OUT);
    }
    if clean.is_empty() || NOT_ABOUT_THEIR_BODY.is_match(clean) {
        return None;
    }
    let condition = OWN_SERIOUS_CONDITION.find(clean);
    let improvement = A_GOOD_TURN.find(clean);
    if !question
        && condition.is_some()
        && let Some(good) = improvement
        && !NOT_LANDED.is_match(&clean[..good.start()])
        && RULED_OUT_TAIL.is_match(&clean[good.end()..])
    {
        return Some(data::THE_ILLNESS_GOING);
    }
    if AN_ANIMAL.is_match(clean)
        || A_QUESTION_ABOUT_IT.is_match(clean)
        || A_SCARE.is_match(clean)
        || improvement.is_some()
    {
        return None;
    }
    let own = reports_own_condition(clean, condition);
    if !A_WORRY.is_match(clean) {
        return own.then_some(data::THE_NEWS);
    }
    let names_condition = condition.is_some() || A_FINDING.is_match(clean);
    if !names_condition && (NOT_A_MEDICAL_TEST.is_match(clean) || !A_MEDICAL_TEST.is_match(clean)) {
        return None;
    }
    if RELATIVE.is_match(clean) {
        return Some(data::THEIR_WAIT);
    }
    if own {
        return None;
    }
    Some(if names_condition {
        data::THE_WAIT
    } else {
        &data::THE_WAIT[..2]
    })
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = normalized(reading.raw);
    let options = pool(&clean)?;
    let reply = compose::fallback::respond(reading, memory)?;
    placeholder(&reply, &clean).then(|| compose::pick(options, memory, reading.raw).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn the_wait_about_a_body_distinguishes_a_wait_from_news_and_other_tests() {
        assert_eq!(
            pool("they are testing me for cancer").unwrap(),
            data::THE_WAIT
        );
        assert_eq!(pool("my mum might have cancer").unwrap(), data::THEIR_WAIT);
        assert_eq!(pool("i have cancer").unwrap(), data::THE_NEWS);
        assert_eq!(pool("it's not cancer").unwrap(), data::THE_THING_RULED_OUT);
        assert_eq!(pool("the cancer is gone").unwrap(), data::THE_ILLNESS_GOING);
        assert!(pool("the cancer is gone but they want another scan").is_none());
        assert!(pool("do you think it's cancer?").is_none());
        assert!(pool("i am worried about my driving test results").is_none());
        assert!(pool("my cat might have cancer").is_none());
        assert!(placeholder(
            "Got it — your sister might have MS.",
            "my sister might have ms"
        ));
        assert!(!placeholder(
            "Got it — your sister might have MS. When do you hear?",
            "my sister might have ms"
        ));
        let turn = Bot::new(Some("0")).unwrap().turn("They think it's cancer");
        assert_eq!(turn.module, Some("the_wait_about_a_body"), "{turn:?}");
    }
}
