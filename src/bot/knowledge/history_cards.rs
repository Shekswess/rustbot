//! Names and question routing for the authored history and invention tables.

use crate::bot::{Exchange, Reading};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

pub(super) struct Record {
    pub names: &'static [&'static str],
    pub off: &'static [&'static str],
    pub tell: &'static str,
    pub long: Option<&'static str>,
    pub dead: Option<&'static str>,
    pub won: Option<&'static str>,
    pub built: Option<&'static str>,
}

struct MatchedRecord {
    data: &'static Record,
    names: RegexSet,
    exclusions: RegexSet,
}

pub(super) fn whole_words(words: &[&str]) -> RegexSet {
    RegexSet::new(
        words
            .iter()
            .map(|word| format!(r"\b{}\b", regex::escape(word))),
    )
    .expect("authored history names and exclusions")
}

fn table(rows: &'static [Record]) -> Vec<MatchedRecord> {
    rows.iter()
        .map(|row| MatchedRecord {
            data: row,
            names: whole_words(row.names),
            exclusions: whole_words(row.off),
        })
        .collect()
}

static PAST: LazyLock<Vec<MatchedRecord>> = LazyLock::new(|| table(super::HISTORY_PAST));
static MADE: LazyLock<Vec<MatchedRecord>> = LazyLock::new(|| table(super::HISTORY_MADE));
static PET_NAMES: LazyLock<RegexSet> = LazyLock::new(|| whole_words(super::HISTORY_NOT_THE_THING));
static COLD_WAR_WINNER: LazyLock<RegexSet> =
    LazyLock::new(|| whole_words(super::HISTORY_ASKS_WHO_WON));

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(FOLLOW_ON, super::HISTORY_FOLLOW_ON_ASK);
pattern!(LONG, super::HISTORY_PAST_LONG);
pattern!(DEAD, super::HISTORY_PAST_DEAD);
pattern!(WON, super::HISTORY_PAST_WON);
pattern!(BUILT, super::HISTORY_PAST_BUILT);
pattern!(NON_ALPHA, r"[^a-z]+");

fn named(table: &[MatchedRecord], said: &str) -> Option<&'static Record> {
    if PET_NAMES.is_match(said) {
        return None;
    }
    let mut best = None;
    let mut best_length = 0;
    for row in table.iter().filter(|row| !row.exclusions.is_match(said)) {
        for index in row.names.matches(said) {
            let length = row.data.names[index].chars().count();
            if length > best_length {
                best_length = length;
                best = Some(row.data);
            }
        }
    }
    best
}

pub(super) fn said(reading: &Reading<'_>) -> String {
    [&reading.line.plain, &reading.line.clean, &reading.line.text]
        .into_iter()
        .filter(|part| !part.is_empty())
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

pub(super) fn past_named(reading: &Reading<'_>) -> bool {
    let said = said(reading);
    named(&PAST, &said).is_some() || FOLLOW_ON.is_match(&said)
}

pub(super) fn past_follow_on(reading: &Reading<'_>) -> bool {
    let line = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    let line = line.to_lowercase();
    line.split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .take(7)
        .count()
        <= 6
        && named(&PAST, &said(reading)).is_none()
        && FOLLOW_ON.is_match(&line)
}

pub(super) fn invention_named(reading: &Reading<'_>) -> bool {
    named(&MADE, &said(reading)).is_some()
}

pub(super) fn past(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let said = said(reading);
    let row = named(&PAST, &said).or_else(|| {
        if !FOLLOW_ON.is_match(&said) {
            return None;
        }
        let last = earlier.last()?;
        let before = format!("{} {}", last.user, last.bot).to_lowercase();
        named(&PAST, &before)
    })?;
    for (reply, ask) in [
        (row.dead, &*DEAD),
        (row.won, &*WON),
        (row.built, &*BUILT),
        (row.long, &*LONG),
    ] {
        if let Some(reply) = reply.filter(|reply| !reply.is_empty())
            && ask.is_match(&said)
        {
            return Some(reply);
        }
    }
    Some(row.tell)
}

pub(super) fn inventor(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = if reading.line.plain.is_empty() {
        &reading.line.text
    } else {
        &reading.line.plain
    };
    named(&MADE, &line.to_lowercase()).map(|row| Cow::Borrowed(row.tell))
}

pub(super) fn cold_war(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = reading.line.text.to_lowercase();
    let words = NON_ALPHA.replace_all(&said, " ");
    let reply = if COLD_WAR_WINNER.is_match(&words) {
        super::HISTORY_COLD_WAR_FULL
    } else {
        super::HISTORY_COLD_WAR
    };
    Some(Cow::Borrowed(reply))
}
