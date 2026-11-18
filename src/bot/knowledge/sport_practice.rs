//! Cricket and basketball practice suggestions and sports-specific reading guards.

use crate::{bot::Reading, text::is_space};
use std::{borrow::Cow, sync::LazyLock};

#[path = "sport_practice_data.rs"]
mod data;
#[cfg(test)]
#[path = "sport_practice_tests.rs"]
mod tests;

static GYM_KIT: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| super::sport_match::phrase_pattern(data::GYM_KIT_WORDS));
static RUN_WORD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?i)\b(?:run(?:s|ning)?|ran)\b").expect("run verb"));
static TOURNAMENT_YEAR: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"\b(?:19[0-9]{2}|20[0-2][0-9])\b").expect("valid World Cup year")
});

pub(super) fn world_cup(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    let years = TOURNAMENT_YEAR
        .find_iter(text)
        .filter_map(|year| year.as_str().parse::<u16>().ok());
    let mut first_year = None;
    for year in years {
        first_year.get_or_insert(year);
        if let Ok(index) = super::WORLD_CUP_WINNERS.binary_search_by_key(&year, |row| row.0) {
            let (_, winner, note) = super::WORLD_CUP_WINNERS[index];
            return Some(Cow::Owned(format!("In {year} it was {winner}, {note}.")));
        }
    }
    if let Some(year) = first_year {
        return Some(Cow::Owned(if year > 2022 && year % 4 == 2 {
            format!(
                "I have Men's World Cup winners through 2022. Check FIFA for the {year} tournament."
            )
        } else {
            format!(
                "The men's World Cup has been held every four years since 1930, apart from 1942 and 1946 for the war, so {year} is not a tournament year."
            )
        }));
    }
    let mut country = "";
    for &(_, name, _) in super::WORLD_CUP_WINNERS {
        let name = name.strip_prefix("West ").unwrap_or(name);
        if name.len() > country.len() && super::has_ascii_phrase(text, &name.to_ascii_lowercase()) {
            country = name;
        }
    }
    if country.is_empty() {
        return Some(Cow::Borrowed(
            "Brazil have won it five times (1958, 1962, 1970, 1994, 2002) and Germany and Italy four each; Argentina won in 2022 and France in 2018, England in 1966.",
        ));
    }
    let won: Vec<String> = super::WORLD_CUP_WINNERS
        .iter()
        .filter(|&&(_, name, _)| name.strip_prefix("West ").unwrap_or(name) == country)
        .map(|row| row.0.to_string())
        .collect();
    let (last, earlier) = won.split_last()?;
    let seasons = if earlier.is_empty() {
        last.clone()
    } else {
        format!("{} and {last}", earlier.join(", "))
    };
    let count = match won.len() {
        1 => "once",
        2 => "twice",
        3 => "three times",
        4 => "four times",
        _ => "five times",
    };
    Some(Cow::Owned(format!(
        "{country} have won the men's World Cup {count}: {seasons}."
    )))
}

pub(super) fn no_wound(reading: &Reading<'_>) -> bool {
    let mentions = |word| super::STITCH_WOUNDS.contains(&word);
    if reading.line.words.is_empty() {
        return !reading.line.clean.split(is_space).any(mentions);
    }
    !reading.line.words.iter().map(String::as_str).any(mentions)
}

pub(super) fn swimming_goggles(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let text = format!("{} {} {}", line.clean, line.plain, line.text).to_lowercase();
    super::SWIMMING_GOGGLES_WORDS
        .iter()
        .any(|word| text.contains(word) || line.words.iter().any(|candidate| candidate == word))
        || reading
            .features
            .asks
            .iter()
            .any(|ask| matches!(*ask, "how" | "what" | "why"))
}

pub(super) fn goggles_fog(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    const CARE: &str = "Rinse swimming goggles with clean water and leave them to air-dry. Avoid rubbing the inside if it has an anti-fog coating. If fogging persists, use a product the manufacturer recommends for your goggles.";
    if reading.raw.to_lowercase().contains("why") || reading.features.asks.contains(&"why") {
        return Some(Cow::Owned(format!(
            "The lenses fog when warm, moist air cools and forms droplets on them. {CARE}"
        )));
    }
    Some(Cow::Borrowed(CARE))
}

pub(super) fn stretch(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words = &reading.line.words;
    let any = |names: &[&str]| words.iter().any(|word| names.contains(&word.as_str()));
    let reply = if any(&["hamstring", "hamstrings"]) {
        super::SPORT_HAMSTRING_STRETCH
    } else if any(&["quad", "quads", "thigh", "thighs"]) {
        super::SPORT_QUAD_STRETCH
    } else if any(&["leg", "legs"]) {
        super::SPORT_LEG_STRETCH
    } else {
        super::SPORT_CALF_STRETCH
    };
    if any(&["before"]) && any(&["after"]) {
        return Some(Cow::Owned(format!(
            "Warm up with easy movement before exercise, and save these sustained stretches for the cool-down. Stretch gently and stop if it hurts. {reply}"
        )));
    }
    Some(Cow::Borrowed(reply))
}

fn tokens_say(reading: &Reading<'_>, low: &str, named: &[&str]) -> bool {
    reading
        .line
        .words
        .iter()
        .any(|word| named.contains(&word.to_lowercase().as_str()))
        || low.split(is_space).any(|word| named.contains(&word))
}

pub(super) fn is_not_coaching(reading: &Reading<'_>) -> bool {
    reading.sense.get("coach").copied() != Some("teacher")
}

pub(super) fn is_not_a_timetable(reading: &Reading<'_>) -> bool {
    let text = if reading.raw.is_empty() {
        &reading.line.clean
    } else {
        reading.raw
    };
    reading.topic.map(|(topic, _)| topic) != Some("geography-and-travel")
        || !RUN_WORD.is_match(text)
}

pub(super) fn gym_induction(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(
        if GYM_KIT.is_match(&reading.line.clean).unwrap_or(false) {
            data::GYM_INDUCTION_KIT
        } else {
            data::GYM_INDUCTION
        },
    ))
}

pub(super) fn names_cricket(reading: &Reading<'_>) -> bool {
    let clean = if reading.line.clean.is_empty() {
        Cow::Owned(reading.line.words.join(" "))
    } else {
        Cow::Borrowed(reading.line.clean.as_str())
    };
    let low = clean.to_lowercase();
    let flat = low.split_whitespace().collect::<Vec<_>>().join(" ");
    let tokens = flat.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '\'');
    if tokens.clone().any(|word| data::BOWL_WORDS.contains(&word))
        && !data::CRICKET_WORDS.iter().any(|phrase| {
            tokens.clone().any(|word| word == *phrase)
                || phrase.contains(' ') && flat.contains(phrase)
        })
    {
        return false;
    }
    match reading.sense.get("bat").copied() {
        Some("animal") => false,
        Some("sport") => true,
        _ => {
            !tokens_say(reading, &low, data::A_BAT_WORD)
                || tokens_say(reading, &low, data::ABOUT_THE_GAME)
        }
    }
}

pub(super) fn cricket(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let low = reading.line.clean.to_lowercase();
    let reply = if tokens_say(reading, &low, data::BATTING_WORDS) {
        data::CRICKET_BATTING
    } else if tokens_say(reading, &low, data::BOWLING_WORDS) {
        data::CRICKET_BOWLING
    } else {
        data::CRICKET_GENERAL
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn basketball(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let low = reading.line.clean.to_lowercase();
    let flat = low.split_whitespace().collect::<Vec<_>>().join(" ");
    let dribble = tokens_say(reading, &low, data::BASKETBALL_DRIBBLE_WORDS)
        || data::BASKETBALL_DRIBBLE_WORDS
            .iter()
            .any(|phrase| phrase.contains(' ') && flat.contains(phrase));
    Some(Cow::Borrowed(if dribble {
        data::BASKETBALL_DRIBBLE
    } else {
        data::BASKETBALL_SHOT
    }))
}
