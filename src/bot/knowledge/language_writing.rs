//! Select the authored example for the kind of message the person is writing.

use crate::bot::Reading;
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

#[path = "language_writing_data.rs"]
mod data;

static EMAIL: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::EMAIL_SUBJECTS.iter().map(|&(pattern, _)| pattern))
        .expect("email subject domains")
});
static PERSON: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:about|for|to)\s+(.+)$").unwrap());
static MATTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\babout\s+(.+)$").unwrap());

pub(super) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    if id != "a-kind-word" {
        return false;
    }
    let said = reading.line.clean.to_lowercase();
    MATTER.captures(&said).is_none_or(|after| {
        let words = after[1]
            .split(crate::text::is_space)
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        !padded_phrase(&format!(" {words} "), data::MATTERS)
    })
}

pub(super) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "thank-you-note" | "thanks-for-hospitality" => Some(Cow::Borrowed(thanks(reading))),
        "a-kind-word" => Some(Cow::Borrowed(kind(reading))),
        "write-an-email" => {
            let said = if reading.line.clean.is_empty() {
                &reading.line.text
            } else {
                &reading.line.clean
            };
            let example = EMAIL.matches(said).into_iter().next().map_or_else(
                || data::EMAIL_SUBJECTS.last().unwrap().1,
                |at| data::EMAIL_SUBJECTS[at].1,
            );
            Some(Cow::Owned(format!(
                "{}{example}{}",
                data::EMAIL_BEFORE,
                data::EMAIL_AFTER
            )))
        }
        _ => None,
    }
}

fn thanks(reading: &Reading<'_>) -> &'static str {
    let said = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
    .to_lowercase();
    data::THANKS
        .iter()
        .find_map(|&(terms, reply)| {
            terms
                .iter()
                .any(|&term| {
                    if term.contains(' ') {
                        said.contains(term)
                    } else {
                        reading
                            .line
                            .words
                            .iter()
                            .any(|word| word.eq_ignore_ascii_case(term))
                    }
                })
                .then_some(reply)
        })
        .unwrap_or(data::THANKS_DEFAULT)
}

fn kind(reading: &Reading<'_>) -> &'static str {
    let said = reading.line.clean.to_lowercase();
    let whole = format!(" {said} ");
    let after = PERSON
        .captures(&said)
        .and_then(|found| found.get(1))
        .map(|found| format!(" {} ", found.as_str()));
    let person = after.as_deref().unwrap_or(&whole);
    data::KIND
        .iter()
        .find_map(|&(terms, reply)| {
            (padded_phrase(person, terms) || padded_phrase(&whole, terms)).then_some(reply)
        })
        .unwrap_or(data::KIND_DEFAULT)
}

fn padded_phrase(said: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| {
        said.match_indices(term).any(|(start, found)| {
            said[..start].ends_with(' ') && said[start + found.len()..].starts_with(' ')
        })
    })
}
