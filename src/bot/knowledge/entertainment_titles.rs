//! Authored credits, verdicts and follow-up picks for individual works.

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use super::books;

#[path = "entertainment_titles_data.rs"]
mod data;
#[cfg(test)]
#[path = "entertainment_titles_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    AGE_ASK,
    ASKING_FIRST,
    SEEN_ALREADY,
    THE_OFFICE_IS,
    A_PERSON_TO_CONSULT,
    NAME_BEFORE_ASK,
    NAME_AFTER_VERB,
    CUT_TITLE,
);

fn title_pattern(title: &str) -> Regex {
    Regex::new(&format!(r"\b{}\b", regex::escape(title))).expect("work title pattern")
}

static FOLLOW_UP: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::FILM_MATCHES
        .iter()
        .map(|(title, _)| title_pattern(title))
        .collect()
});
static CREDITS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::WORKS
        .iter()
        .map(|(title, ..)| title_pattern(title))
        .collect()
});

fn says_one(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| text.contains(phrase))
}

fn verdict_line(reading: &Reading<'_>) -> String {
    if reading.line.clean.is_empty() {
        &reading.line.norm_text
    } else {
        &reading.line.clean
    }
    .to_lowercase()
}

fn is_about_a_person(text: &str) -> bool {
    AGE_ASK.is_match(text) || (!ASKING_FIRST.is_match(text) && SEEN_ALREADY.is_match(text))
}

fn is_about_a_place(text: &str) -> bool {
    THE_OFFICE_IS.is_match(text) || says_one(text, data::A_PLACE_NOT_A_WORK)
}

pub(super) fn worth_it(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = verdict_line(reading);
    if is_about_a_person(&text) || is_about_a_place(&text) {
        return None;
    }
    books::longest_named(&text, data::VIEWS).map(|(_, reply)| Cow::Borrowed(reply))
}

fn named_work(text: &str) -> Option<String> {
    for pattern in [&*NAME_BEFORE_ASK, &*NAME_AFTER_VERB] {
        let Some(name) = pattern.captures(text).and_then(|found| found.get(1)) else {
            continue;
        };
        let name = CUT_TITLE
            .find(name.as_str())
            .map_or(name.as_str(), |cut| &name.as_str()[..cut.start()]);
        let name = name.trim_matches([' ', '.', '?', '!', ',', '-', '\t']);
        if !data::UNNAMED.contains(&name) {
            return Some(books::display(name));
        }
    }
    None
}

pub(super) fn worth_it_unnamed(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = verdict_line(reading);
    if is_about_a_person(&text)
        || books::longest_named(&text, data::VIEWS).is_some()
        || is_about_a_place(&text)
        || says_one(&text, data::NOT_A_VERDICT)
        || A_PERSON_TO_CONSULT.is_match(&text)
        || !says_one(&text, data::JUDGE_ASKS)
    {
        return None;
    }
    Some(named_work(&text).map_or(Cow::Borrowed(data::NO_NAMED_VERDICT), |name| {
        Cow::Owned(format!(
            "I don't know *{name}* well enough to call it honestly. Tell me what you liked about the last thing you enjoyed and I'll say whether it's that kind."
        ))
    }))
}

pub(super) fn after_named(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = format!("{} {}", reading.line.clean, reading.line.norm_text).to_lowercase();
    following_title(&text).map(Cow::Borrowed)
}

pub(super) fn names_a_work(text: &str) -> bool {
    books::longest_named(text, data::VIEWS).is_some()
}

pub(super) fn following_title(text: &str) -> Option<&'static str> {
    let about_work = says_one(text, data::WORK_EVIDENCE) || says_one(text, data::NEXT_EVIDENCE);
    data::FILM_MATCHES
        .iter()
        .zip(FOLLOW_UP.iter())
        .find_map(|(&(title, reply), pattern)| {
            if !pattern.is_match(text) {
                return None;
            }
            if let Some((_, plain_sense)) =
                data::PLAIN_SENSE.iter().find(|(name, _)| *name == title)
                && (!about_work || says_one(text, plain_sense))
            {
                return None;
            }
            Some(reply)
        })
}

fn capitalized_work(title: &str) -> String {
    let words = title.split_whitespace();
    if words
        .clone()
        .next()
        .is_some_and(|word| word.chars().all(|digit| digit.is_ascii_digit()))
    {
        return title.to_owned();
    }
    words
        .enumerate()
        .map(|(index, word)| {
            if index > 0 && data::SMALL.contains(&word) {
                return word.to_owned();
            }
            let mut chars = word.chars();
            let first = chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .collect::<String>();
            format!("{first}{}", chars.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn who_made(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
    .to_lowercase()
    .replace(['\u{2019}', '\''], "");
    let (_, (&(title, maker, detail, kind), _)) = data::WORKS
        .iter()
        .zip(CREDITS.iter())
        .enumerate()
        .filter(|(_, (_, pattern))| pattern.is_match(&text))
        .max_by_key(|(index, ((title, ..), _))| {
            (title.chars().count(), std::cmp::Reverse(*index))
        })?;
    let name = capitalized_work(title);
    let article = if kind.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    };
    Some(Cow::Owned(format!(
        "{name} is {article} {kind} by {maker}, {detail}."
    )))
}
