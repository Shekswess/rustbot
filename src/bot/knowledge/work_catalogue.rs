//! Definitions, typical duties and pay for the named occupation cards.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::said_text;

#[path = "work_catalogue_data.rs"]
mod data;

static ABOUT_SOMETHING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(my|our)\b| about | with a | with my ").unwrap());
static MY_WORK_DAY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:what|how) (?:do|does|should|can|shall|could) (?:i|we)\b").unwrap()
});
static MY_PAYMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:how|what) (?:much )?(?:do|does|did|can|should|shall|could|would) (?:i|we)\b|\b(?:do|should|would|will) (?:i|we) pay\b").unwrap()
});

struct Name {
    length: usize,
    pattern: Regex,
    role: &'static str,
}

static DAY_NAMES: LazyLock<Vec<Name>> = LazyLock::new(|| {
    data::DAY_ALIASES
        .iter()
        .filter(|(_, role)| !role.is_empty())
        .map(|&(alias, role)| (alias.to_owned(), role))
        .chain(
            data::DAY_TABLE
                .iter()
                .flat_map(|&(role, _)| [(role.to_owned(), role), (format!("{role}s"), role)]),
        )
        .map(|(alias, role)| Name {
            length: alias.len(),
            pattern: Regex::new(&format!(r"\b{}\b", regex::escape(&alias))).unwrap(),
            role,
        })
        .collect()
});

fn day_name(line: &str) -> Option<(&'static str, usize)> {
    let mut best = None;
    let mut length = 0;
    for name in DAY_NAMES.iter() {
        if name.length > length
            && let Some(found) = name.pattern.find(line)
        {
            best = Some((name.role, found.start()));
            length = name.length;
        }
    }
    best
}

pub(super) fn definition(reading: &Reading<'_>) -> Option<&'static str> {
    let said = &reading.line.clean;
    let words = &reading.line.words;
    let mentions =
        |phrase: &&str| said.contains(phrase) || words.iter().any(|word| word == *phrase);
    data::WORK_WORDS.iter().find_map(|&(keys, reply, avoid)| {
        (keys.iter().any(mentions) && !avoid.iter().any(mentions)).then_some(reply)
    })
}

pub(super) fn day(reading: &Reading<'_>) -> Option<&'static str> {
    let line = format!(" {}", said_text(reading).to_lowercase());
    if ABOUT_SOMETHING.is_match(&line) || MY_WORK_DAY.is_match(&line) {
        return None;
    }
    let (role, offset) = day_name(&line)?;
    if !data::DAY_WORDS
        .iter()
        .any(|word| line.find(word).is_some_and(|at| at <= offset))
    {
        return None;
    }
    data::DAY_TABLE
        .iter()
        .find_map(|&(name, reply)| (name == role).then_some(reply))
}

pub(super) fn pay(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = format!(" {} ", said_text(reading).to_lowercase());
    if MY_PAYMENT.is_match(&line) {
        return None;
    }
    let mut best = None;
    let mut length = 0;
    for &(alias, name) in data::PAY_ALIASES {
        if alias.len() > length && line.contains(&format!(" {alias} ")) {
            best = Some(name);
            length = alias.len();
        }
    }
    let name = best.or_else(|| day_name(&line).map(|(name, _)| name))?;
    let reply = data::PAY_TABLE
        .iter()
        .find_map(|&(role, reply)| (name == role).then_some(reply))?;
    if reply.contains('£') && name != "nurse" {
        let reply = format!(
            "{reply} Those are rough UK figures. Check current local job adverts for comparable hours and experience."
        );
        return Some(Cow::Owned(reply));
    }
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jobs_are_named_in_the_reply_and_first_person_requests_abstain() {
        let read = Reading::new("What does a software developer do?");
        assert!(day(&read).is_some_and(|reply| reply.starts_with("A developer's day")));
        assert!(day(&Reading::new("What does a vet do about my limping dog?")).is_none());
        assert!(pay(&Reading::new("How much do I pay a plumber?")).is_none());
        assert!(
            pay(&Reading::new("How much do nurses get paid?"))
                .is_some_and(|reply| reply.contains("Band 5"))
        );
        assert!(
            definition(&Reading::new("What is a personal statement?"))
                .is_some_and(|reply| reply.contains("three questions"))
        );
    }
}
