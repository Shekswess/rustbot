//! Acknowledge a report of the previous night's drinking.

use crate::bot::{Memory, Reading};

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($value).expect(stringify!($name)));
    };
}

macro_rules! context_pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($value).expect(stringify!($name)));
    };
}

#[path = "morning_after_data.rs"]
mod data;
use data::*;

fn reads(line: &str) -> bool {
    if line.is_empty()
        || [&*AN_EMERGENCY, &*A_QUESTION_ABOUT_IT, &*A_CONSEQUENCE]
            .iter()
            .any(|pattern| pattern.is_match(line))
    {
        return false;
    }
    let gone = A_TIME_THAT_HAS_GONE.is_match(line);
    let start = A_STRONG_STATE
        .find(line)
        .ok()
        .flatten()
        .map(|found| found.start())
        .or_else(|| {
            gone.then(|| A_WEAK_STATE.find(line).map(|found| found.start()))
                .flatten()
        })
        .or_else(|| {
            A_MORNING
                .is_match(line)
                .then(|| A_HANGOVER.find(line).map(|found| found.start()))
                .flatten()
        });
    start.is_some_and(|at| !SOMEONE_ELSE.is_match(&line[..at]))
        && (gone || !A_STILL_TO_COME.is_match(line))
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = &reading.line.clean;
    reads(clean).then(|| super::choose_recent(A_MORNING_AFTER, memory, clean).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaves_other_people_consequences_and_future_drinking_alone() {
        for line in [
            "my husband was drunk again last night",
            "i was plastered and fell over",
            "im going on a bender on friday",
            "i blacked out after drinking last night",
        ] {
            assert_eq!(
                reply(&Reading::new(line), &Memory::default()),
                None,
                "{line}"
            );
        }
    }
}
