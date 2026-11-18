//! Counts and discovery lookups from the authored science cards.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Reading, memory::groupages::decimal};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(STEPS_COUNT, super::SCIENCE_STEPS_COUNT);
pattern!(HOURS, r"\b(\d+(?:\.\d+)?)\s*(?:hours?|hrs?)\b");
pattern!(AN_HOUR, r"\b(?:an|one|a single)\s+hour\b");
pattern!(HALF_HOUR, r"\bhalf an hour\b");
pattern!(MINUTES, r"\b(\d+)\s*(?:minutes|minute|mins|min)\b");
pattern!(BRISK, r"\b(brisk|fast|quick|power|uphill|hilly)\b");

struct Distance {
    unit: Regex,
    amount: Regex,
    steps: f64,
}

static DISTANCE: LazyLock<Vec<Distance>> = LazyLock::new(|| {
    [
        ("mile", 2000.0),
        ("miles", 2000.0),
        ("km", 1250.0),
        ("kms", 1250.0),
        ("kilometre", 1250.0),
        ("kilometres", 1250.0),
        ("kilometer", 1250.0),
        ("kilometers", 1250.0),
    ]
    .into_iter()
    .map(|(unit, steps)| Distance {
        unit: Regex::new(&format!(r"\b{unit}\b")).unwrap(),
        amount: Regex::new(&format!(r"(\d+(?:\.\d+)?)\s*{unit}\b")).unwrap(),
        steps,
    })
    .collect()
});

static DISCOVERERS: LazyLock<Vec<(Regex, usize, &'static str)>> = LazyLock::new(|| {
    super::SCIENCE_DISCOVERERS
        .iter()
        .map(|&(thing, reply)| {
            (
                Regex::new(&format!(r"\b{}\b", regex::escape(thing))).unwrap(),
                thing.chars().count(),
                reply,
            )
        })
        .collect()
});

pub(super) fn discoverer(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    let line = line.to_lowercase().replace('\'', "");
    DISCOVERERS
        .iter()
        .rev()
        .filter(|(pattern, _, _)| pattern.is_match(&line))
        .max_by_key(|(_, width, _)| *width)
        .map(|(_, _, reply)| Cow::Borrowed(*reply))
}

pub(super) fn building_steps(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = reading.line.clean.to_lowercase();
    let place = if line.contains("pisa") || line.contains("leaning tower") {
        0
    } else if line.contains("empire") {
        1
    } else if ["paul", "dome", "cathedral"]
        .iter()
        .any(|term| line.contains(term))
    {
        2
    } else if ["flight", "floor", "storey", "story", "riser", "stair"]
        .iter()
        .any(|term| line.contains(term))
    {
        3
    } else {
        0
    };
    Some(Cow::Borrowed(super::SCIENCE_BUILDING_STEPS[place]))
}

fn step_count(line: &str) -> Option<u32> {
    ascii_number(&STEPS_COUNT.captures(line)?[1])?.parse().ok()
}

fn ascii_number(text: &str) -> Option<Cow<'_, str>> {
    if text.is_ascii() && !text.contains(',') {
        return Some(Cow::Borrowed(text));
    }
    let mut number = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            ',' => (),
            '.' | '0'..='9' => number.push(character),
            other => {
                let digit = decimal(other.encode_utf8(&mut [0; 4]))?;
                number.push(char::from(b'0' + digit as u8));
            }
        }
    }
    Some(Cow::Owned(number))
}

fn grouped_number(count: f64) -> String {
    let digits = format!("{count:.0}");
    let mut formatted = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            formatted.push(',');
        }
        formatted.push(digit);
    }
    formatted
}

fn said_steps(value: f64) -> String {
    let step = [1000.0, 500.0, 250.0, 50.0]
        .into_iter()
        .find(|step| value >= step * 4.0)
        .unwrap_or(1.0);
    grouped_number((value / step).round_ties_even() * step)
}

pub(super) fn distance_steps(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = reading.line.clean.to_lowercase();
    if let Some(count) = step_count(&line).filter(|count| (1000..=100_000).contains(count))
        && super::SCIENCE_STEPS_REPORT_WORDS
            .iter()
            .any(|word| line.contains(word))
    {
        let miles = format!("{:.1}", f64::from(count) / 2000.0);
        let miles = miles.strip_suffix(".0").unwrap_or(&miles);
        let steps = grouped_number(f64::from(count));
        return Some(Cow::Owned(format!(
            "That's about {miles} miles - {steps} steps at roughly 2,000 to a mile. A 5k is about 6,500 steps and a marathon around 55,000, so that is a proper walk."
        )));
    }
    Some(Cow::Borrowed(super::SCIENCE_STEPS_A_MILE))
}

fn parsed_amount(pattern: &Regex, line: &str) -> Option<f64> {
    let number: f64 = ascii_number(&pattern.captures(line)?[1])?.parse().ok()?;
    number.is_finite().then_some(number)
}

fn minutes(line: &str, indefinite_hour: bool) -> Option<f64> {
    let time = if let Some(hours) = parsed_amount(&HOURS, line) {
        hours * 60.0
    } else if indefinite_hour && AN_HOUR.is_match(line) {
        60.0
    } else if HALF_HOUR.is_match(line) {
        30.0
    } else {
        parsed_amount(&MINUTES, line)?
    };
    (time != 0.0 && time.is_finite()).then_some(time)
}

fn duration_phrase(minutes: f64) -> String {
    if (minutes - 60.0).abs() < 0.5 {
        "an hour".to_owned()
    } else if (minutes - 30.0).abs() < 0.5 {
        "half an hour".to_owned()
    } else if (minutes % 60.0).abs() < 0.5 {
        let hours = (minutes / 60.0).floor();
        format!("{hours:.0} hour{}", if hours == 1.0 { "" } else { "s" })
    } else {
        format!("{:.0} minutes", minutes.round_ties_even())
    }
}

pub(super) fn time_steps(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = reading.line.clean.to_lowercase();
    let reply = if let Some(minutes) = minutes(&line, false) {
        let ordinary = 6250.0 * minutes / 60.0;
        let quick = 7500.0 * minutes / 60.0;
        if !quick.is_finite() {
            return None;
        }
        let ordinary = said_steps(ordinary);
        let quick = said_steps(quick);
        let duration = duration_phrase(minutes);
        format!(
            "About {ordinary} steps in {duration} of walking at an ordinary pace, or {quick} if you are walking briskly - that is 1,250 steps a kilometre at about 5km/h. Hills, a slow stroll and short legs all move it."
        )
    } else if BRISK.is_match(&line) {
        format!(
            "About {} steps in an hour of brisk walking - 6.5km/h or so, which is 1,250 steps a kilometre over six and a half of them. An ordinary-paced hour is about {}, and a slow wander round the shops is well under that.",
            said_steps(7500.0),
            said_steps(6250.0)
        )
    } else {
        format!(
            "About {} to {} steps in an hour of walking: an ordinary pace is about 5km/h, which is 1,250 steps a kilometre, so an hour gets you five of them. A brisk walk nearer 6.5km/h is the top of that range, and a slow wander round shops is well under it.",
            said_steps(6250.0),
            said_steps(7500.0)
        )
    };
    Some(Cow::Owned(reply))
}

pub(super) fn steps_calories(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = reading.line.clean.to_lowercase();
    let steps = if let Some(distance) = DISTANCE
        .iter()
        .find(|distance| distance.unit.is_match(&line))
    {
        parsed_amount(&distance.amount, &line).unwrap_or(1.0) * distance.steps
    } else if let Some(minutes) = minutes(&line, true) {
        6250.0 * minutes / 60.0
    } else {
        f64::from(
            step_count(&line)
                .filter(|count| (200..=100_000).contains(count))
                .unwrap_or(10_000),
        )
    };
    if !steps.is_finite() {
        return None;
    }
    let low = grouped_number((steps * 0.04 / 10.0).round_ties_even() * 10.0);
    let high = grouped_number((steps * 0.05 / 10.0).round_ties_even() * 10.0);
    let steps = said_steps(steps);
    Some(Cow::Owned(format!(
        "About {low} to {high} calories in {steps} steps - roughly 0.04 to 0.05 of a calorie a step, which is the figure for an average adult. It climbs with your weight and your pace, and it is a small number next to what a snack costs: a biscuit is often a whole mile's worth, so the walking is for the habit and the heart as much as the calories."
    )))
}
