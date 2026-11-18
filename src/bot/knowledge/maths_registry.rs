//! Descriptive statistics and scope guards for school maths cards.

use crate::bot::Reading;
use crate::text::decimal_digit;
use fancy_regex::Regex as Lookaround;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "maths_registry_data.rs"]
mod data;
#[cfg(test)]
#[path = "maths_registry_tests.rs"]
mod tests;

static DIGIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").expect("decimal digit"));
static FIGURES: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(r"(?<![\w.])\d+(?:\.\d+)?").expect("statistical values"));
static WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-z']+").expect("statistical vocabulary"));
static NAMED_ROUNDING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::ROUNDING_NAMED).expect("maths rounding vocabulary"));
static ROUND_IS_AROUND: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::ROUND_IS_AROUND).expect("round used as a preposition"));
static BRACKET_MATHS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::BRACKET_MATHS).expect("algebraic brackets"));

fn said<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

pub(super) fn rounding_asked(reading: &Reading<'_>) -> bool {
    let all = format!(
        "{} {} {}",
        reading.line.clean, reading.line.norm_text, reading.line.text
    )
    .to_lowercase();
    if ROUND_IS_AROUND.is_match(&all).unwrap_or(false) {
        return false;
    }
    let line = said(reading).to_lowercase();
    DIGIT.is_match(&line) || NAMED_ROUNDING.is_match(&line)
}

pub(super) fn algebraic_bracket(reading: &Reading<'_>) -> bool {
    [&reading.line.plain, &reading.line.clean, &reading.line.text]
        .into_iter()
        .find(|line| !line.is_empty())
        .is_some_and(|line| BRACKET_MATHS.is_match(&line.to_lowercase()))
}

fn ratio(value: &str) -> Option<BigRational> {
    let decimal_places = value
        .split_once('.')
        .map_or(0, |(_, tail)| tail.chars().count());
    let number = value
        .chars()
        .filter(|&character| character != '.')
        .map(|one| decimal_digit(one).map(|digit| digit + b'0'))
        .collect::<Option<Vec<_>>>()?;
    let integer = BigInt::parse_bytes(&number, 10)?;
    let denominator = BigInt::from(10).pow(decimal_places.try_into().ok()?);
    Some(BigRational::new(integer, denominator))
}

fn short(value: &BigRational) -> Option<String> {
    if value.is_integer() {
        return Some(value.numer().to_string());
    }
    let value = value.to_f64().filter(|value| value.is_finite())?;
    Some(
        format!("{value:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned(),
    )
}

fn plain_words(text: &str) -> impl DoubleEndedIterator<Item = &str> {
    text.split_whitespace()
        .map(|word| word.trim_matches(['?', '.', '!', ',']))
}

fn names_a_statistic(words: &[&str]) -> bool {
    let names_other = words
        .iter()
        .any(|word| data::OTHER_STATISTICS.contains(word));
    words.iter().enumerate().any(|(index, word)| {
        data::STATISTIC_WORDS.contains(word)
            && (!data::MEAN_VERB.contains(word)
                || !names_other
                    && !index
                        .checked_sub(1)
                        .is_some_and(|before| data::POINTS_AT_A_THING.contains(&words[before])))
    })
}

fn explanation(asked: &str) -> Option<&'static str> {
    let mut words: Vec<_> = plain_words(asked).collect();
    if !WORD
        .find_iter(asked)
        .all(|word| data::AVERAGE_FILLER.contains(&word.as_str()))
        || !names_a_statistic(&words)
    {
        return None;
    }
    if let Some(at) = words
        .iter()
        .rposition(|word| !data::AVERAGE_FILLER.contains(word) || data::MEAN_VERB.contains(word))
        && at > 0
        && data::MEAN_VERB.contains(&words[at])
    {
        words.remove(at);
    }
    Some(
        if asked.contains("difference")
            || asked.contains("median") && words.join(" ").contains("mean")
        {
            data::AVERAGE_EXPLAIN
        } else if asked.contains("median") {
            data::MEDIAN_EXPLAIN
        } else if asked.contains("mode") {
            data::MODE_EXPLAIN
        } else {
            data::AVERAGE_EXPLAIN
        },
    )
}

fn mode(ordered: &[BigRational]) -> Option<&BigRational> {
    ordered
        .chunk_by(|left, right| left == right)
        .filter(|part| part.len() > 1)
        .max_by_key(|part| (part.len(), std::cmp::Reverse(&part[0])))
        .map(|part| &part[0])
}

pub(super) fn averages(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = said(reading);
    let mut values = FIGURES
        .find_iter(text)
        .map(|matched| ratio(matched.ok()?.as_str()))
        .collect::<Option<Vec<_>>>()?;
    let count = values.len();
    let asked = text.to_lowercase();
    if count < 2 {
        return explanation(&asked).map(Cow::Borrowed);
    }
    values.sort_unstable();
    let list = values
        .iter()
        .map(short)
        .collect::<Option<Vec<_>>>()?
        .join(", ");
    let middle = count / 2;
    let median = || {
        if count % 2 == 1 {
            short(&values[middle])
        } else {
            short(&((&values[middle - 1] + &values[middle]) / BigInt::from(2)))
        }
    };
    if asked.contains("median") {
        return Some(Cow::Owned(format!(
            "The median of {list} is {}: in order, that is the middle one.",
            median()?
        )));
    }
    let total: BigRational = values.iter().sum();
    let mean = short(&(&total / BigInt::from(count)))?;
    if asked.contains("mode") {
        return Some(Cow::Owned(if let Some(mode) = mode(&values) {
            format!(
                "The mode of {list} is {}, the number that appears most often.",
                short(mode)?
            )
        } else {
            format!(
                "No number in {list} repeats, so there is no mode. The mean is {mean} and the median is {}.",
                median()?
            )
        }));
    }
    let total = short(&total)?;
    Some(Cow::Owned(format!(
        "The mean of {list} is {mean}: they add to {total} and {total} ÷ {count} = {mean}."
    )))
}
