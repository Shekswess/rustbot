//! Whether an answer depends on the current state of the world.

use super::{Intent, Kind, KindFeatures};
use crate::bot::understanding::line::Line;
use fancy_regex::{Match, Regex};
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "kinds_time_data.rs"]
mod data;
use data::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TimeDependence {
    Unknown,
    Settled,
    Changing(Change),
}

impl TimeDependence {
    pub(crate) fn value(self) -> Option<bool> {
        match self {
            Self::Unknown => None,
            Self::Settled => Some(false),
            Self::Changing(_) => Some(true),
        }
    }

    pub(crate) fn what(self) -> Option<Change> {
        match self {
            Self::Changing(what) => Some(what),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Change {
    Price,
    Hours,
    Weather,
    News,
    Availability,
    Timetable,
    Rate,
    Event,
    Post,
}

enum Shape {
    Single(&'static LazyLock<Regex>),
    ServiceOnDay,
}

impl Shape {
    fn find<'a>(&self, text: &'a str) -> Option<Match<'a>> {
        match self {
            Self::Single(pattern) => pattern.find(text).ok().flatten(),
            Self::ServiceOnDay => {
                let service = A_SERVICE.find(text).ok().flatten()?;
                let time = A_DAY_OR_TIME.find(text).ok().flatten()?;
                Some(if service.start() <= time.start() {
                    service
                } else {
                    time
                })
            }
        }
    }
}

fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn any(patterns: &[&LazyLock<Regex>], text: &str) -> bool {
    patterns.iter().any(|pattern| matches(pattern, text))
}

fn is_asking(kind: Kind, features: &KindFeatures, said: &str) -> bool {
    if kind == Kind::Question || said.contains('?') {
        return true;
    }
    if matches(&A_REPORTS_THE_ASK, said) {
        return matches(&ASKS_WITH_A_QUESTION_WORD, said)
            || matches(&ASKS_WITH_AN_AUXILIARY, said.trim())
            || matches(&ASKS_AFTER_A_COMMA_WORD, &said.replace(" ,", ","));
    }
    !features.asks.is_empty()
        || features.intent == Intent::Question
        || matches(&ASKS_WITH_A_QUESTION_WORD, said)
        || matches(&ASKS_MID_LINE, said)
        || matches(&ASKS_AFTER_A_COMMA, &said.replace(" ,", ","))
        || matches(&ASKS_WITH_AN_AUXILIARY, said.trim())
}

fn padded(line: &str) -> String {
    format!(" {} ", line.trim().to_lowercase())
}

fn spoken(line: &Line) -> String {
    padded(if line.plain.is_empty() {
        &line.clean
    } else {
        &line.plain
    })
}

fn uses_maths(features: &KindFeatures) -> bool {
    features.asks.contains(&"maths")
}

fn early_read(line: &Line, maths: bool, text: &str, asking: bool) -> Option<TimeDependence> {
    use TimeDependence::{Settled, Unknown};
    if (asking && matches(&A_HOW_IT_WORKS, text) && !matches(&A_SAYS_NOW, text))
        || matches(&A_ARITHMETIC, text)
        || matches(
            &A_ARITHMETIC,
            &padded(if line.text.is_empty() {
                &line.clean
            } else {
                &line.text
            }),
        )
        || (maths && matches(&A_HAS_A_NUMBER, text) && !matches(&A_AN_AMOUNT_OF_A_THING, text))
        || any(
            &[
                &A_PAST_YEAR,
                &A_HOW_OLD_IS_THE_WORLD,
                &A_HISTORY_ITSELF,
                &A_A_DIFFERENCE,
            ],
            text,
        )
    {
        return Some(Settled);
    }

    if matches(&A_HOW_A_THING_MEASURES, text) && !matches(&A_WEATHER_HERE, text) {
        return Some(Settled);
    }
    if matches(&A_A_TRAFFIC_LIGHT, text)
        || matches(&A_A_PLAN_ABOUT_THE_WEATHER, text) && !matches(&A_WEATHER_HERE, text)
    {
        return Some(Unknown);
    }
    if any(
        &[
            &A_HOW_MUCH_IT_HOLDS,
            &A_HOW_MUCH_IS_IN,
            &A_HOW_MUCH_ENOUGH,
            &A_HOW_MUCH_A_FEELING,
        ],
        text,
    ) {
        return Some(Settled);
    }
    if matches(&A_AN_OCCASION_OF_THEIR_OWN, text) {
        return Some(Unknown);
    }
    if !matches(&MONEY, text) && any(&[&A_A_CONVERSION, &A_HOW_MUCH_OF_A_UNIT], text) {
        return Some(Settled);
    }
    if any(
        &[
            &A_HOW_MUCH_I_SPENT,
            &A_MY_OWN_FIGURE,
            &A_THE_CLOCK,
            &A_DID_YOU_SEE,
        ],
        text,
    ) {
        return Some(Unknown);
    }
    (!asking && matches(&A_BARE_RECORD, text) || matches(&A_WORD_MEANS, text)).then_some(Settled)
}

fn read_changing(text: &str, asking: bool, word_count: usize) -> Option<TimeDependence> {
    let fresh = A_FRESHENING.replace_all(text, " ");
    let reports = matches(&A_REPORTS, text);
    if word_count <= 9
        && !reports
        && let Some(what) = BARE_LIVE
            .iter()
            .find_map(|(what, pattern)| matches(pattern, &fresh).then_some(*what))
    {
        return Some(TimeDependence::Changing(what));
    }
    for (what, pattern) in CHANGING {
        let Some(found) = pattern.find(&fresh) else {
            continue;
        };
        if matches!(what, Change::Hours | Change::Timetable | Change::Event)
            && matches(&A_MY_OWN_PLAN, text)
        {
            continue;
        }
        let before = fresh[..found.start()].trim();
        let after = &fresh[found.end()..];
        if matches(&A_DEFINITION, text)
            && matches!(before, "what is" | "what are" | "whats" | "what's")
            && after.trim_matches([' ', '?', '!', '.', ',']).is_empty()
        {
            return Some(TimeDependence::Settled);
        }
        let bare = || {
            word_count <= 4
                && !reports
                && matches!(
                    before,
                    "" | "the" | "a" | "an" | "any" | "your" | "this" | "our" | "these"
                )
                && (after.trim().is_empty() || matches(&A_BARE_TAIL, after.trim()))
        };
        if asking || bare() {
            return Some(TimeDependence::Changing(*what));
        }
    }
    None
}

/// First reading of whether answering the turn would require current information.
pub(crate) fn over_time(line: &Line, kind: Kind, features: &KindFeatures) -> TimeDependence {
    let text = spoken(line);
    let asking = is_asking(kind, features, &text);
    let maths = uses_maths(features);
    early_read(line, maths, &text, asking)
        .or_else(|| read_changing(&text, asking, text.split_whitespace().count()))
        .unwrap_or_else(|| {
            if asking && (maths || any(SETTLED, &text)) {
                TimeDependence::Settled
            } else {
                TimeDependence::Unknown
            }
        })
}

/// Recheck a complete reading and carry a changing subject through short follow-ups.
pub(crate) fn over_time_followup(
    line: &Line,
    kind: Kind,
    features: &KindFeatures,
    raw: &str,
    followup: bool,
    previous: Option<(&str, TimeDependence)>,
) -> Option<TimeDependence> {
    let text = spoken(line);
    let count = text.split_whitespace().count();
    let asking = is_asking(kind, features, &text);
    if let Some(dependence) = early_read(line, uses_maths(features), &text, asking)
        .or_else(|| read_changing(&text, asking, count))
    {
        return Some(dependence);
    }
    let (earlier, dependence) = previous?;
    (matches!(dependence, TimeDependence::Changing(_))
        && followup
        && count <= 7
        && (earlier.is_empty() || earlier != raw))
        .then_some(dependence)
}
