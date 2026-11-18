//! Follow-up duration statements about a job from the same conversation.

use super::{
    groupages::decimal,
    workplace::{capitalize, with_article},
    workplace_data as data,
};
use crate::bot::{Memory, Reading, compose};
use regex::{Captures, Regex};
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $expression:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($expression).expect(stringify!($name)));
    };
}
pattern!(BEEN_AT_IT, data::BEEN_AT_IT);
pattern!(BEEN_AT_IT_BARE, data::BEEN_AT_IT_BARE);
pattern!(NOT_A_STINT, data::NOT_A_STINT);
pattern!(THEIR_LENGTH, data::THEIR_LENGTH);
pattern!(ARTICLE, r"(?i)^(?:a|an|the)\s+");
pattern!(
    POINTS_BACK,
    r"(?i)\A(?:doing\s+)?(?:it|that|this|the\s+job)\z"
);

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = reading.raw.trim_matches(crate::text::is_space);
    if said.is_empty() || said.contains('?') {
        return None;
    }
    if let Some(job) = memory.occupations.own_job()
        && !NOT_A_STINT.is_match(said)
        && let Some(stint) = BEEN_AT_IT
            .captures(said)
            .or_else(|| BEEN_AT_IT_BARE.captures(said))
        && refers_to(&stint, job)
    {
        let count = count(&stint["count"]);
        if count > 0 {
            return Some(own_stint(
                job,
                count,
                unit_is_months(&stint),
                reading.raw,
                memory,
            ));
        }
    }
    let stint = THEIR_LENGTH.captures(said)?;
    let (relation, job) = memory.occupations.their_job(&stint["who"].to_lowercase())?;
    let number = &stint["count"];
    let count = count(number);
    if !refers_to(&stint, job) || (count == 0 && !number.eq_ignore_ascii_case("years")) {
        return None;
    }
    Some(their_stint(
        relation,
        job,
        count,
        unit_is_months(&stint),
        said,
        memory,
    ))
}

fn refers_to(stint: &Captures<'_>, job: &str) -> bool {
    let doing = stint
        .name("doing")
        .map_or("", |part| part.as_str().trim_matches(crate::text::is_space));
    let doing = ARTICLE.replace(doing, "");
    if POINTS_BACK.is_match(&doing) {
        return true;
    }
    let only_letters = |line: &str| {
        line.to_lowercase()
            .chars()
            .filter(char::is_ascii_lowercase)
            .collect::<String>()
    };
    let doing = only_letters(&doing);
    let job = only_letters(job);
    !doing.is_empty()
        && !job.is_empty()
        && (doing.contains(&job)
            || job.contains(&doing)
            || doing.bytes().take(4).eq(job.bytes().take(4)))
}

fn unit_is_months(stint: &Captures<'_>) -> bool {
    stint
        .name("unit")
        .is_some_and(|part| part.as_str().to_ascii_lowercase().starts_with("month"))
}

fn count(word: &str) -> u16 {
    decimal(word).unwrap_or_else(|| match word.to_ascii_lowercase().as_str() {
        "a" | "an" | "one" => 1,
        "a couple of" => 2,
        "a few" => 3,
        "several" => 4,
        "many" => 8,
        _ => 0,
    })
}

fn figure(count: u16) -> String {
    let count = usize::from(count);
    if let Some(word) = data::NUMBERS.get(count) {
        return capitalize(word);
    }
    let tens = data::TENS[count / 10];
    let units = count % 10;
    capitalize(&if units > 0 {
        format!("{tens}-{}", data::NUMBERS[units])
    } else {
        tens.to_owned()
    })
}

fn own_stint(job: &str, count: u16, months: bool, said: &str, memory: &Memory) -> String {
    let (pool, count) = if months && count < 24 {
        (
            if count < 12 {
                data::OWN_MONTHS_EARLY
            } else {
                data::OWN_MONTHS_LATER
            },
            count,
        )
    } else {
        let years = if months { count / 12 } else { count };
        (
            if years < 2 {
                data::OWN_ONE
            } else {
                data::OWN_YEARS
            },
            years,
        )
    };
    let figure = figure(count);
    let at_it = format!("as {}", with_article(job));
    let options: Vec<_> = pool
        .iter()
        .map(|text| text.replace("{figure}", &figure).replace("{at_it}", &at_it))
        .collect();
    pick(&options, said, job, memory)
}

fn their_stint(
    relation: &str,
    job: &str,
    count: u16,
    months: bool,
    said: &str,
    memory: &Memory,
) -> String {
    let theirs = format!("Your {relation}");
    let (pool, count, span) = if count == 0 {
        (data::THEIR_VAGUE, count, "years")
    } else if months && count < 24 {
        (
            if count < 12 {
                data::THEIR_MONTHS_EARLY
            } else {
                data::THEIR_TIMED
            },
            count,
            "months",
        )
    } else {
        (
            data::THEIR_TIMED,
            if months { count / 12 } else { count },
            "years",
        )
    };
    let figure = figure(count);
    let low = figure.to_lowercase();
    let theirs_lower = theirs.to_lowercase();
    let a_job = with_article(job);
    let options: Vec<_> = pool
        .iter()
        .map(|text| {
            text.replace("{theirs}", &theirs)
                .replace("{theirs_lower}", &theirs_lower)
                .replace("{figure}", &figure)
                .replace("{low}", &low)
                .replace("{a_job}", &a_job)
                .replace("{span}", span)
        })
        .collect();
    pick(&options, said, job, memory)
}

fn pick(options: &[String], said: &str, job: &str, memory: &Memory) -> String {
    let options: Vec<_> = options.iter().map(String::as_str).collect();
    compose::pick(&options, memory, &format!("{said} {job}")).to_owned()
}
