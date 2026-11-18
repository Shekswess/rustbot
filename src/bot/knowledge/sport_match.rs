//! Match rules, ground dimensions, boots and wet-weather kit.
#[path = "sport_match_data.rs"]
mod data;
#[cfg(test)]
#[path = "sport_match_tests.rs"]
mod tests;

use crate::bot::Reading;
use fancy_regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    Ground,
    Competition,
    WetKit,
}

pub(super) fn phrase_pattern(phrases: &[&str]) -> Regex {
    let alternatives = phrases
        .iter()
        .map(|phrase| regex::escape(phrase))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"(?<!\w)(?:{alternatives})")).expect("source match phrases")
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| phrase_pattern(data::$name));)+
    };
}
patterns!(
    WET_MATCH_SPORT,
    WET_MATCH_WEATHER,
    WET_MATCH_WEAR,
    WET_MATCH_NOT,
    WET_MATCH_I_PLAY,
    WET_MATCH_WATCHING_WORDS,
    WET_MATCH_PLAYS,
);

fn says(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn ground(reading: &Reading<'_>) -> Option<&'static str> {
    let text = reading
        .line
        .clean
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if text.is_empty() {
        return Some(data::GROUNDS[0].1);
    }
    data::GROUNDS.iter().find_map(|(names, reply)| {
        names
            .iter()
            .any(|name| text.contains(name))
            .then_some(*reply)
    })
}

fn competition(reading: &Reading<'_>) -> Option<&'static str> {
    let text = format!("{} {}", reading.line.clean, reading.line.words.join(" ")).to_lowercase();
    let cups = data::CUPS.iter().any(|name| text.contains(name));
    let leagues = data::LEAGUES
        .iter()
        .filter(|name| text.contains(*name))
        .take(2)
        .count();
    (!cups && leagues <= 1).then_some(data::LEAGUE_AND_CUP)
}

pub(super) fn booking_card(reading: &Reading<'_>) -> bool {
    let text = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    let text = text.to_lowercase();
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let padded = format!(" {text} ");
    data::A_CARD_WORDS
        .iter()
        .any(|word| padded.contains(&format!(" {word} ")))
}

fn match_line(reading: &Reading<'_>) -> String {
    format!(" {} ", reading.line.clean.to_lowercase())
}

pub(super) fn wet_match(reading: &Reading<'_>) -> bool {
    let text = match_line(reading);
    says(&WET_MATCH_SPORT, &text)
        && says(&WET_MATCH_WEATHER, &text)
        && says(&WET_MATCH_WEAR, &text)
        && !says(&WET_MATCH_NOT, &text)
}

fn wet_kit(reading: &Reading<'_>) -> &'static str {
    let text = match_line(reading);
    if says(&WET_MATCH_I_PLAY, &text)
        || (!says(&WET_MATCH_WATCHING_WORDS, &text) && says(&WET_MATCH_PLAYS, &text))
    {
        data::WET_MATCH_PLAYING
    } else {
        data::WET_MATCH_WATCHING
    }
}

pub(super) fn reply(form: Reply, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let response = match form {
        Reply::Ground => ground(reading)?,
        Reply::Competition => competition(reading)?,
        Reply::WetKit => wet_kit(reading),
    };
    Some(Cow::Borrowed(response))
}
