//! How long a visitor normally spends in the stated destination.

use super::{
    city_guides::named_place,
    travel_logistics::{home, in_chat, named, trip_to},
};
use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::sync::LazyLock;

#[path = "travel_duration_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $expression:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($expression).expect(stringify!($name)));
    };
}
pattern!(A_JOURNEY_OR_A_BAG, data::A_JOURNEY_OR_A_BAG);
pattern!(ASKS_A_LENGTH, data::ASKS_A_LENGTH);
pattern!(COUNTS_AS_A_THING, data::COUNTS_AS_A_THING);
pattern!(COUNTS_AS_A_TRIP, data::COUNTS_AS_A_TRIP);
pattern!(
    DAYS_STATED,
    r"\b(?:\d+|one|two|three|four|five|six|seven|eight|nine|ten|a few|a couple of|several)\s+(?:full\s+)?(?:days|nights|day|night)\b"
);
pattern!(
    ASKS_WITH_FIGURE,
    r"\b(?:enough|too long|too short|too much|worth it|worth|do i need|should i|should we|do we need)\b"
);
pattern!(
    ASKS_FOR_FIGURE,
    r"\b(?:how long|how many days|how many nights|how much time)\b"
);
pattern!(
    PLACE_PREPOSITION,
    r"\b(?:in|at|to see|to visit|around|for)\s+"
);
pattern!(
    POINTS_BACK,
    r"(?i)\b(?:there|that city|this city|the city)\b"
);

static OBJECT_NAMES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::ALSO_A_THING
        .iter()
        .map(|object| {
            Regex::new(&format!(r"\b{}\b", regex::escape(object))).expect("ambiguous country name")
        })
        .collect()
});

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<&'static str> {
    let line = &reading.line;
    let clean = &line.clean;
    match id {
        "city-stay" => stay(clean, &line.text, earlier),
        "country-stay-length" => country_stay(clean, &line.text, earlier),
        "city-stay-followup" => {
            if named(clean, data::CITIES).is_some() {
                return None;
            }
            let (place, reply) = in_chat(clean, earlier, data::CITIES)?;
            trip_to(place, clean, earlier).then_some(reply)
        }
        _ => None,
    }
}

fn stay(clean: &str, typed: &str, earlier: &[Exchange]) -> Option<&'static str> {
    if A_JOURNEY_OR_A_BAG.is_match(clean)
        || (!ASKS_WITH_FIGURE.is_match(clean)
            && (DAYS_STATED.is_match(clean) || !ASKS_FOR_FIGURE.is_match(clean)))
    {
        return None;
    }
    let Some((city, reply)) = in_chat(clean, earlier, data::CITIES) else {
        return country_stay(clean, typed, earlier);
    };
    if trip_to(city, clean, earlier) {
        return Some(reply);
    }
    let has_reference = named(clean, data::CITIES).is_some()
        && PLACE_PREPOSITION
            .find_iter(clean)
            .any(|at| clean[at.end()..].starts_with(city));
    if !has_reference {
        return None;
    }
    let resident = std::iter::once(clean)
        .chain(earlier.iter().rev().take(2).map(|turn| turn.user.as_str()))
        .map(str::to_lowercase)
        .any(|line| line.contains(city) && home(city, &line));
    (!resident).then_some(reply)
}

fn country_stay(clean: &str, typed: &str, earlier: &[Exchange]) -> Option<&'static str> {
    if A_JOURNEY_OR_A_BAG.is_match(clean)
        || in_chat(clean, earlier, data::CITIES).is_some()
        || POINTS_BACK.is_match(clean)
    {
        return None;
    }
    let data::Country(country, reply) = country_in_chat(clean, earlier)?;
    if (!typed.is_empty() && !typed.to_lowercase().contains(country))
        || (!trip_to(country, clean, earlier)
            && !(ASKS_A_LENGTH.is_match(clean) && clean.to_lowercase().contains(country)))
        || home(country, clean)
    {
        return None;
    }
    *reply
}

fn country_in_chat(clean: &str, earlier: &[Exchange]) -> Option<&'static data::Country> {
    let in_line = |line: &str| {
        data::COUNTRIES
            .iter()
            .find(|item| named_place(line, item.0))
    };
    if let Some(country) = in_line(clean) {
        return (!is_object(clean, country.0)).then_some(country);
    }
    earlier
        .iter()
        .rev()
        .take(2)
        .find_map(|turn| in_line(&turn.user.to_lowercase()))
}

pub(super) fn is_object(clean: &str, country: &str) -> bool {
    let Some(at) = data::ALSO_A_THING.iter().position(|name| *name == country) else {
        return false;
    };
    let low = format!(
        " {} ",
        clean
            .to_lowercase()
            .split(crate::text::is_space)
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    );
    let mut matches = OBJECT_NAMES[at].find_iter(&low).peekable();
    matches.peek().is_some()
        && matches.all(|found| {
            COUNTS_AS_A_THING.is_match(&low[..found.start()])
                && !COUNTS_AS_A_TRIP
                    .find(&low[found.end()..])
                    .is_some_and(|verb| verb.start() == 0)
        })
}

#[cfg(test)]
#[path = "../../../tests/fixtures/travel_duration.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::super::guards::{Context, Guard};
    use super::*;

    #[test]
    fn city_country_and_followup_stay_lengths_match_the_source() {
        let mut failures = Vec::new();
        for fixtures::Case(id, prompt, prior, reply, guard) in fixtures::CASES {
            let reading = Reading::new(prompt);
            let turns: Vec<_> = prior
                .iter()
                .map(|text| Exchange {
                    user: (*text).to_owned(),
                    bot: String::new(),
                    module: "answers",
                })
                .collect();
            if let Some(expected) = guard {
                let actual = Guard::NotWeight.passes(&Context::new(&reading, &turns), id, &[]);
                if actual != *expected {
                    failures.push(format!("{id}: {prompt} guard {actual:?} vs {expected:?}"));
                }
            }
            let actual = registered(id, &reading, &turns);
            if actual != *reply {
                failures.push(format!(
                    "{id}: {prompt} after {prior:?}: {actual:?} vs {reply:?}"
                ));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}
