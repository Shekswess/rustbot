//! The source presentation callback opens with the symptom the user actually named.

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use super::{
    PRESENTATION_PLURAL_SIGNS, PRESENTATION_SIGNS, PRESENTATION_SIGNS_OF_NERVES, has_ascii_phrase,
};

static NON_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9' ]+").unwrap());

fn read_line(reading: &Reading<'_>) -> String {
    if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    }
    .to_lowercase()
}

fn physical_sign(text: &str) -> Option<(&'static str, &'static str)> {
    let clean = NON_WORD.replace_all(text, " ");
    if !PRESENTATION_SIGNS_OF_NERVES
        .iter()
        .any(|phrase| has_ascii_phrase(&clean, phrase))
    {
        return None;
    }
    let subject = PRESENTATION_SIGNS
        .iter()
        .find_map(|(phrases, label)| {
            phrases
                .iter()
                .any(|phrase| text.contains(phrase))
                .then_some(*label)
        })
        .unwrap_or("That feeling");
    let verb = if PRESENTATION_PLURAL_SIGNS.contains(&subject) {
        "are"
    } else {
        "is"
    };
    Some((subject, verb))
}

pub(super) fn reply(reading: &Reading<'_>, main: &'static str) -> Cow<'static, str> {
    let text = read_line(reading);
    let Some((subject, verb)) = physical_sign(&text) else {
        return Cow::Borrowed(main);
    };
    let mut chars = main.chars();
    let initial = chars
        .next()
        .into_iter()
        .flat_map(char::to_lowercase)
        .collect::<String>();
    Cow::Owned(format!(
        "{subject} before a talk {verb} adrenaline, and it drops once you are a minute in. Eat something plain first, then {initial}{}",
        chars.as_str(),
    ))
}

pub(super) fn interview_reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    const MAIN: &str = "Nerves are expected and mostly read as caring. Before you go in: a minute of slow breaths out, and read your two best stories once. If your mind blanks, say \"let me think about that for a second\" and take it - it reads as considered, not slow.";
    let text = read_line(reading);
    let Some((subject, _)) = physical_sign(&text) else {
        return Some(Cow::Borrowed(MAIN));
    };
    let lead = if subject == "That tight chest" {
        "If you are struggling to breathe now or your chest feels tight or heavy, call 999 in the UK. If the difficulty breathing has eased but is unusual for you, contact NHS 111. "
    } else {
        ""
    };
    Some(Cow::Owned(format!(
        "{lead}{subject} before an interview can be a reaction to stress. If the feeling is familiar and settles, try sipping water and breathing out slowly before you go in. {MAIN}"
    )))
}
