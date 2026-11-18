//! Point-to-point directions, attractions, and what to try in a country.

use crate::bot::{Exchange, Reading};
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

#[path = "travel_logistics_data.rs"]
mod data;

fn whole_word(name: &str) -> String {
    format!(r"\b{}\b", regex::escape(name))
}
static ROUTE_PLACES: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::ROUTE_PLACES.iter().map(|place| whole_word(place)))
        .expect("locations named by the route index")
});
static SITE_NAMES: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::SITES.iter().map(|(name, _)| whole_word(name)))
        .expect("site names in source tie order")
});
static FOOD_NOT_A_TRIP: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        data::FOOD_NOT_A_TRIP
            .iter()
            .map(|phrase| format!(r"(?:^|[^a-z]){}(?:$|[^a-z])", regex::escape(phrase))),
    )
    .expect("another subject besides a country's food")
});

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(STAYS_THERE, data::STAYS_THERE);
pattern!(ASKS_A_LENGTH, data::ASKS_A_LENGTH);
pattern!(LIVES_THERE, data::LIVES_THERE);
pattern!(TRIP_FRAME, data::TRIP_FRAME);
pattern!(
    ORIGIN,
    r"(?:^|[^a-z])(?:from|born in|raised in|grew up in)\s+"
);
pattern!(THE, r"^the\s+");

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<&'static str> {
    let said = &reading.line.clean;
    match id {
        "route-between" => {
            let named = ROUTE_PLACES.matches(said);
            data::ROUTES.iter().find_map(|&(from, to, reply)| {
                (named.matched(from) && named.matched(to)).then_some(reply)
            })
        }
        "site-worth-visiting" => SITE_NAMES
            .matches(said)
            .iter()
            .next()
            .map(|index| data::SITES[index].1),
        "food-in-the-country" => {
            let clean = if said.is_empty() {
                &reading.line.text
            } else {
                said
            };
            food(&clean.to_lowercase(), said, earlier)
        }
        _ => None,
    }
}

pub(super) fn named<'a>(said: &str, rows: &'a [(&'a str, &'a str)]) -> Option<(&'a str, &'a str)> {
    rows.iter()
        .copied()
        .find(|&(name, _)| super::city_guides::named_place(said, name))
}

pub(super) fn in_chat(
    clean: &str,
    earlier: &[Exchange],
    rows: &'static [(&str, &str)],
) -> Option<(&'static str, &'static str)> {
    named(clean, rows).or_else(|| {
        earlier
            .iter()
            .rev()
            .take(2)
            .find_map(|turn| named(&turn.user.to_lowercase(), rows))
    })
}

fn food(clean: &str, typed_clean: &str, earlier: &[Exchange]) -> Option<&'static str> {
    if FOOD_NOT_A_TRIP.is_match(clean)
        || data::CHEAP_EATS
            .iter()
            .any(|city| super::city_guides::named_place(clean, city))
    {
        return None;
    }
    let place = named(clean, data::FOOD)
        .map(|item| item.0)
        .or_else(|| named(clean, data::FOOD_ADJECTIVES).map(|item| item.1))
        .or_else(|| named(clean, data::CITY_COUNTRY).map(|item| item.1));
    let place = match place {
        Some(place) => place,
        None => {
            let place = in_chat(typed_clean, earlier, data::CITY_COUNTRY)
                .map(|item| item.1)
                .or_else(|| in_chat(typed_clean, earlier, data::FOOD).map(|item| item.0))?;
            if !trip_to(place, typed_clean, earlier) {
                return None;
            }
            place
        }
    };
    if ORIGIN.find_iter(clean).any(|origin| {
        let tail = &clean[origin.end()..];
        tail.starts_with(place)
            || THE
                .find(tail)
                .is_some_and(|article| tail[article.end()..].starts_with(place))
    }) {
        return None;
    }
    data::FOOD
        .iter()
        .find_map(|&(name, reply)| (name == place).then_some(reply))
}

pub(super) fn home(place: &str, said: &str) -> bool {
    LIVES_THERE.find_iter(said).any(|verb| {
        let tail = &said[verb.end()..];
        let end = tail
            .char_indices()
            .nth(40)
            .map_or(tail.len(), |(end, _)| end);
        tail[..end].contains(place)
    })
}

pub(super) fn trip_to(place: &str, clean: &str, earlier: &[Exchange]) -> bool {
    let lines: Vec<_> = std::iter::once(clean)
        .chain(earlier.iter().rev().take(3).map(|turn| turn.user.as_str()))
        .map(str::to_lowercase)
        .collect();
    let named = || lines.iter().take(3).filter(|said| said.contains(place));
    !named().any(|line| home(place, line))
        && (named().any(|line| STAYS_THERE.is_match(line) || TRIP_FRAME.is_match(line))
            || (ASKS_A_LENGTH.is_match(clean)
                && lines.iter().skip(1).any(|line| line.contains(place))
                && !lines.iter().skip(1).any(|line| home(place, line))))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/travel_logistics.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_sites_and_dishes_match_pinned_pybot() {
        for fixtures::Case(id, prompt, history, reply) in fixtures::CASES {
            let reading = Reading::new(prompt);
            let earlier: Vec<_> = history
                .iter()
                .map(|text| Exchange {
                    user: (*text).to_owned(),
                    bot: String::new(),
                    module: "answers",
                })
                .collect();
            assert_eq!(
                registered(id, &reading, &earlier),
                *reply,
                "{id}: {prompt} after {history:?}"
            );
        }
    }
}
