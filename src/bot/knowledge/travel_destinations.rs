//! Destinations, day trips, and region-specific replies from the pinned cards.

use super::city_guides::named_place;
use crate::bot::{Exchange, Reading};
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

#[path = "travel_destinations_data.rs"]
mod data;

static MONTH: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::MONTHS.iter().map(|month| whole_word(month))).expect("month names")
});
static BEACH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bbeach(?:es)?\b").expect("beach question"));
static GET_TO: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::GET_TO
        .iter()
        .map(|(name, _)| Regex::new(&whole_word(name)).expect("destination name"))
        .collect()
});

pub(crate) fn registered(
    id: &str,
    reading: &Reading<'_>,
    turns: &[Exchange],
) -> Option<&'static str> {
    let said = &reading.line.clean;
    match id {
        "city-day-trip" => named_in_chat(said, turns, data::DAY_TRIPS),
        "city-airport" => named_in_chat(said, turns, data::AIRPORTS),
        "region-days" => region_days(said),
        "get-to-place" => GET_TO
            .iter()
            .zip(data::GET_TO)
            .find_map(|(name, &(_, reply))| name.is_match(said).then_some(reply)),
        _ => None,
    }
}

fn whole_word(name: &str) -> String {
    format!(
        r"(?:^|[^\p{{L}}\p{{N}}_]){}(?:$|[^\p{{L}}\p{{N}}_])",
        regex::escape(name)
    )
}

fn named_in_chat(
    said: &str,
    turns: &[Exchange],
    table: &'static [(&str, &str)],
) -> Option<&'static str> {
    let named = |said: &str| {
        table
            .iter()
            .find_map(|&(name, reply)| named_place(said, name).then_some(reply))
    };
    named(said).or_else(|| {
        turns
            .iter()
            .rev()
            .take(2)
            .find_map(|turn| named(&turn.user.to_lowercase()))
    })
}

fn region_days(said: &str) -> Option<&'static str> {
    if MONTH.is_match(said) {
        return None;
    }
    let &(_, region) = data::REGION_NAMES
        .iter()
        .find(|&&(name, _)| named_place(said, name))?;
    let (days, beach) = data::REGIONS[region];
    if BEACH.is_match(said) {
        beach
    } else {
        Some(days)
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/travel_destinations.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destination_replies_match_the_source_with_and_without_prior_places() {
        for &fixtures::Case(id, raw, prior, expected) in fixtures::CASES {
            let turns: Vec<_> = prior
                .iter()
                .map(|&user| Exchange {
                    user: user.into(),
                    bot: String::new(),
                    module: "answers",
                })
                .collect();
            assert_eq!(
                registered(id, &Reading::new(raw), &turns),
                expected,
                "{id}: {raw} after {prior:?}"
            );
        }
    }
}
