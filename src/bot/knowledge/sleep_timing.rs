//! Bedtime clock parsing and the resulting time available for sleep.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Captures, Regex};

use crate::bot::Reading;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    CLOCK,
    r"\b(0?\d|1\d|2[0-3])(?:[:.](\d{2}))?\s*(a\.?m\.?|p\.?m\.?|o'?clock)\b"
);
pattern!(
    BARE_CLOCK,
    r"\b(?:at|till|until|by|around|about)\s+(0?\d|1\d|2[0-3])(?:[:.](\d{2}))?\b"
);
pattern!(
    HALF,
    r"\bhalf(?: past)?\s+(0?\d|1\d|2[0-3]|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b"
);
pattern!(
    WORD_CLOCK,
    r"\b(?:at|till|until|by|around|about)\s+(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b"
);
pattern!(NAMED_HOUR, r"\b(one|two|three|eleven|twelve)\b");
pattern!(LATE, r"\b(?:late|later|midnight)\b");

const VARIABLE: &[&str] = &[
    "all over the place",
    "varies",
    "different every night",
    "no set time",
    "whenever",
    "when i feel like it",
    "at different times",
];
const ROUTINE: &str = "Keep a regular wake time and allow enough time to wind down. If caffeine affects your sleep, leave it for earlier in the day.";

fn hour(word: &str) -> Option<u16> {
    let named = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ];
    named
        .iter()
        .position(|name| *name == word)
        .map(|index| index as u16 + 1)
        .or_else(|| word.parse::<u16>().ok())
        .filter(|hour| *hour <= 23)
}

fn assumed_hour(hour: u16) -> u16 {
    match hour {
        12 => 0,
        6..=11 => hour + 12,
        _ => hour,
    }
}

fn numeric_clock(found: &Captures<'_>) -> Option<u16> {
    let hour = hour(&found[1])?;
    let minute = found
        .get(2)
        .map_or(Some(0), |minute| minute.as_str().parse::<u16>().ok())?;
    if minute > 59 {
        return None;
    }
    let hour = match found.get(3).map(|suffix| suffix.as_str().as_bytes()[0]) {
        Some(b'a') if (1..=12).contains(&hour) => hour % 12,
        Some(b'p') if (1..=12).contains(&hour) => hour % 12 + 12,
        Some(b'a' | b'p') => return None,
        _ => assumed_hour(hour),
    };
    Some(hour * 60 + minute)
}

fn bedtime(text: &str) -> Option<u16> {
    CLOCK
        .captures(text)
        .and_then(|found| numeric_clock(&found))
        .or_else(|| {
            HALF.captures(text)
                .and_then(|found| hour(&found[1]))
                .map(|hour| assumed_hour(hour) * 60 + 30)
        })
        .or_else(|| text.contains("midnight").then_some(0))
        .or_else(|| {
            BARE_CLOCK
                .captures(text)
                .and_then(|found| numeric_clock(&found))
        })
        .or_else(|| {
            WORD_CLOCK
                .captures(text)
                .or_else(|| NAMED_HOUR.captures(text))
                .and_then(|found| hour(&found[1]))
                .map(|hour| assumed_hour(hour) * 60)
        })
}

pub(super) fn clock_label(minutes: u16) -> String {
    if minutes == 0 {
        return "midnight".to_owned();
    }
    let hour = minutes / 60;
    let minute = minutes % 60;
    let suffix = if hour < 12 { "am" } else { "pm" };
    let hour = (hour + 11) % 12 + 1;
    if minute == 0 {
        format!("{hour}{suffix}")
    } else {
        format!("{hour}:{minute:02}{suffix}")
    }
}

fn duration(minutes: u16) -> String {
    let hours = minutes / 60;
    let rest = minutes % 60;
    match (hours, rest) {
        (0, rest) => format!("{rest} minutes"),
        (1, 0) => "1 hour".to_owned(),
        (_, 0) => format!("{hours} hours"),
        (1, _) => format!("1 hour and {rest} minutes"),
        _ => format!("{hours} hours and {rest} minutes"),
    }
}

pub(crate) fn named_bedtime(reading: &Reading<'_>) -> bool {
    let text = reading.raw.to_lowercase();
    bedtime(&text).is_some()
        || LATE.is_match(&text)
        || VARIABLE.iter().any(|cue| text.contains(cue))
}

fn answer(text: &str) -> Cow<'static, str> {
    if let Some(bed) = bedtime(text) {
        let minutes = (24 * 60 + 7 * 60 - bed) % (24 * 60);
        let advice = if minutes < 7 * 60 {
            "Most adults need around 7 to 9 hours of sleep, so an earlier bedtime may give you more time if 7am is fixed."
        } else {
            "That gives you a window of at least seven hours. How much you sleep will also depend on how long it takes to drop off and how often you wake."
        };
        return Cow::Owned(format!(
            "From {} to a 7am start is {} in bed. {advice} {ROUTINE}",
            clock_label(bed),
            duration(minutes)
        ));
    }
    if VARIABLE.iter().any(|cue| text.contains(cue)) {
        return Cow::Owned(format!(
            "If bedtime changes a lot, start by picking a wake time you can keep. You can bring bedtime forward in small steps if that suits your schedule. {ROUTINE}"
        ));
    }
    Cow::Owned(format!(
        "If bedtime is regularly late, count back about 7 to 9 hours from when you need to get up to see what window you have. {ROUTINE}"
    ))
}

pub(crate) fn reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(answer(&reading.raw.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_bedtime_uses_minutes_and_says_the_wake_time_assumption() {
        assert!(answer("i go to bed at 11:30 pm").contains("7 hours and 30 minutes"));
        assert!(answer("i stay up till 1am").contains("7am start is 6 hours"));
        assert!(answer("my bedtime varies").contains("picking a wake time"));
    }
}
