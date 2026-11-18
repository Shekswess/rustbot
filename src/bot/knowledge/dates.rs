//! Dates for the named events in the pinned historical date card.

use super::history_cards::{said, whole_words};
use crate::bot::Reading;
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

pub(super) struct Date {
    pub names: &'static [&'static str],
    pub off: &'static [&'static str],
    pub off_regex: Option<&'static str>,
    pub tell: &'static str,
    pub long: Option<&'static str>,
}

struct MatchedDate {
    date: &'static Date,
    names: RegexSet,
    off: RegexSet,
    off_regex: Option<Regex>,
}

static DATES: LazyLock<Vec<MatchedDate>> = LazyLock::new(|| {
    super::HISTORY_DATES
        .iter()
        .map(|date| MatchedDate {
            date,
            names: whole_words(date.names),
            off: whole_words(date.off),
            off_regex: date
                .off_regex
                .map(|pattern| Regex::new(pattern).expect("authored date exclusion")),
        })
        .collect()
});
static LONG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::HISTORY_DATE_LONG).expect("authored duration question"));

fn named(line: &str) -> Option<&'static Date> {
    let mut best = None;
    let mut longest = 0;
    for entry in DATES.iter().filter(|entry| {
        !entry.off.is_match(line)
            && entry
                .off_regex
                .as_ref()
                .is_none_or(|off| !off.is_match(line))
    }) {
        for index in entry.names.matches(line) {
            let length = entry.date.names[index].chars().count();
            if length > longest {
                longest = length;
                best = Some(entry.date);
            }
        }
    }
    best
}

pub(super) fn names_a_date(reading: &Reading<'_>) -> bool {
    named(&said(reading)).is_some()
}

pub(super) fn reply(reading: &Reading<'_>) -> Option<&'static str> {
    let line = said(reading);
    let date = named(&line)?;
    Some(
        date.long
            .filter(|_| LONG.is_match(&line))
            .unwrap_or(date.tell),
    )
}

#[cfg(test)]
#[test]
fn dated_events_distinguish_duration_and_unrelated_uses_of_the_name() {
    assert!(
        reply(&Reading::new("When was the Eiffel Tower built?"))
            .unwrap()
            .contains("March 1889")
    );
    assert!(
        reply(&Reading::new("How long did the last ice age last?"))
            .unwrap()
            .contains("100,000 years")
    );
    assert_eq!(reply(&Reading::new("When is Ice Age 6 out?")), None);
    assert_eq!(
        reply(&Reading::new("When is the NHS dentist appointment?")),
        None
    );
}
