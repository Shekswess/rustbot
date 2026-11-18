//! Country recommendations and city climate before a named trip.

use super::{
    city_guides::named_place, travel_duration::is_object, travel_logistics::named,
    travel_stays::in_plan,
};
use crate::bot::{Exchange, Reading, memory::stated_plans::Facts};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "travel_places_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(TRIP_WORDS, data::TRIP_WORDS);
pattern!(A_FACT_OF_THE_COUNTRY, data::A_FACT_OF_THE_COUNTRY);
pattern!(US_AS_A_PLACE, data::US_AS_A_PLACE);

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
    plans: Option<&Facts>,
) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    match id {
        "country-trip" | "country-where-to-go" => {
            let (place, advice) = country(clean)?;
            if place == "us" && !US_AS_A_PLACE.is_match(clean) {
                return None;
            }
            if id == "country-where-to-go"
                && (is_object(clean, place)
                    || (!TRIP_WORDS.is_match(clean) && followed_country_fact(earlier, place)))
            {
                return None;
            }
            Some(Cow::Borrowed(advice))
        }
        "city-weather" => weather(clean, earlier, plans).map(Cow::Owned),
        _ => None,
    }
}

fn country(line: &str) -> Option<(&'static str, &'static str)> {
    named(line, data::COUNTRIES).or_else(|| named(line, data::ALIASES))
}

fn followed_country_fact(earlier: &[Exchange], current: &str) -> bool {
    earlier.last().is_some_and(|turn| {
        let previous = &turn.user;
        !TRIP_WORDS.is_match(previous)
            && A_FACT_OF_THE_COUNTRY.is_match(previous)
            && country(previous).is_some_and(|(name, _)| name != current)
    })
}

fn month(text: &str) -> Option<usize> {
    data::MONTHS
        .iter()
        .find_map(|&(name, index)| text.contains(name).then_some(index))
}

fn weather(clean: &str, earlier: &[Exchange], plans: Option<&Facts>) -> Option<String> {
    let city_named = |line: &str| data::CITIES.iter().find(|city| named_place(line, city.0));
    let city = city_named(clean)
        .or_else(|| {
            earlier
                .iter()
                .rev()
                .take(2)
                .find_map(|turn| city_named(&turn.user.to_lowercase()))
        })
        .or_else(|| in_plan(plans, city_named))?;
    let month = month(clean).or_else(|| {
        earlier
            .iter()
            .rev()
            .take(4)
            .find_map(|turn| month(&turn.user.to_lowercase()))
    })?;
    let (low, high) = data::CLIMATES[city.2][month];
    Some(format!(
        "{} in {} is usually about {low} to {high}C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        data::LONG_MONTHS[month],
        city.1
    ))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/travel_places.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countries_and_city_climate_match_pinned_pybot() {
        let mut failures = Vec::new();
        for fixtures::Case(id, prompt, history, reply) in fixtures::CASES {
            let reading = Reading::new(prompt);
            let mut facts = Facts::default();
            let earlier: Vec<_> = history
                .iter()
                .map(|text| {
                    facts.observe(text);
                    Exchange {
                        user: (*text).to_owned(),
                        bot: String::new(),
                        module: "answers",
                    }
                })
                .collect();
            let actual = registered(id, &reading, &earlier, Some(&facts));
            if actual.as_deref() != *reply {
                failures.push(format!(
                    "{id}: {prompt} after {history:?}: {actual:?} vs {reply:?}"
                ));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}
