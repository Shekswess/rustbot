//! Seasonal advice and transport for the city named in the request or filed trip.

use super::{
    TRAVEL_CITY_AREAS, TRAVEL_FAMILY_VERDICTS, TRAVEL_NOT_WHERE_ASKS, TRAVEL_TOWN_STAY,
    city_guides::{named_place, names_time, seasonal_reply},
    travel_logistics::{in_chat, named},
};
use crate::bot::{Exchange, Reading, memory::stated_plans::Facts};
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

#[path = "travel_stays_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(WORTH_ASK, data::WORTH_ASK);
pattern!(BOOK_ASK, data::BOOK_ASK);
pattern!(BOOK_NOT_THE_TRIP, data::BOOK_NOT_THE_TRIP);
pattern!(FLIGHT_WORD, super::TRAVEL_LODGING_FLIGHT);
pattern!(CAR_NIGHT, super::TRAVEL_LODGING_CAR);
pattern!(SHIFT_WORK, super::TRAVEL_LODGING_SHIFT);
pattern!(STAY_PLACE_ASK, super::TRAVEL_LODGING_PLACE_ASK);
pattern!(GUIDE_ASK, super::TRAVEL_FAMILY_GUIDE_ASK);
pattern!(
    AIRPORT_STAY,
    r"\b(?:gatwick|heathrow|stansted|prestwick|(?:luton|liverpool) airport|airport hotel|early flight|dawn flight|red eye)\b"
);
pattern!(
    DAY_TRIP_WAY,
    r"\bhow (?:do|can|would|should) (?:i|we|you) get\b|\bhow to get\b|\bhow far\b|\bhow long does it take\b"
);
static BOOK_MONTHS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        data::BOOK_BY
            .iter()
            .map(|(name, _)| format!(r"\b{}\b", regex::escape(name))),
    )
    .expect("ordered booking months")
});

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
    plans: Option<&Facts>,
) -> Option<&'static str> {
    let clean = &reading.line.clean;
    match id {
        "city-best-time" => best_time(clean, earlier, plans),
        "city-transit" => transit(clean, earlier, plans),
        "day-trip-place" | "day-trip-way" => day_trip(clean, earlier, plans),
        "where-to-stay" | "where-to-stay-there" => {
            if !asks_where(reading) {
                return None;
            }
            in_chat(clean, earlier, TRAVEL_CITY_AREAS).map(|(_, reply)| reply)
        }
        "where-to-stay-anywhere" => in_chat(clean, earlier, TRAVEL_CITY_AREAS)
            .is_none()
            .then_some(TRAVEL_TOWN_STAY),
        "a-city-good-for-families" | "is-it-good-for-families" => {
            in_chat(clean, earlier, TRAVEL_FAMILY_VERDICTS).map(|(_, reply)| reply)
        }
        _ => None,
    }
}

fn asks_where(reading: &Reading<'_>) -> bool {
    !reading
        .features
        .asks
        .iter()
        .any(|ask| TRAVEL_NOT_WHERE_ASKS.contains(ask))
}

pub(super) fn airport_stay_without_destination(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    AIRPORT_STAY.is_match(clean) && named(clean, TRAVEL_CITY_AREAS).is_none()
}

pub(super) fn asks_for_town_stay(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    asks_where(reading)
        && !FLIGHT_WORD.is_match(clean)
        && !CAR_NIGHT.is_match(clean)
        && !SHIFT_WORK.is_match(clean)
        && STAY_PLACE_ASK.is_match(clean)
        && named(clean, TRAVEL_CITY_AREAS).is_none()
}

pub(super) fn asks_for_family_verdict(reading: &Reading<'_>) -> bool {
    let text = reading
        .line
        .text
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    !GUIDE_ASK.is_match(&text)
}

fn latest_plan(plans: &Facts) -> Option<&str> {
    plans
        .latest()
        .filter(|plan| !plan.trim().is_empty())
        .or_else(|| plans.plans().rev().find(|plan| !plan.trim().is_empty()))
}

pub(super) fn in_plan<T: Copy>(
    plans: Option<&Facts>,
    name: impl Fn(&str) -> Option<T>,
) -> Option<T> {
    let plans = plans?;
    if let Some(current) = latest_plan(plans).map(str::to_lowercase) {
        if let Some(value) = name(&current) {
            return Some(value);
        }
        if city_name(&current).is_some() {
            return None;
        }
    }
    plans
        .plans()
        .rev()
        .find_map(|line| name(&line.to_lowercase()))
}

fn city_name(line: &str) -> Option<&'static str> {
    data::CITIES
        .iter()
        .copied()
        .find(|city| named_place(line, city))
}

fn best_time(clean: &str, earlier: &[Exchange], plans: Option<&Facts>) -> Option<&'static str> {
    if WORTH_ASK.is_match(clean) && !names_time(clean) {
        return None;
    }
    if BOOK_ASK.is_match(clean) && !BOOK_NOT_THE_TRIP.is_match(clean) {
        let window = plans
            .and_then(Facts::latest)
            .and_then(|plan| BOOK_MONTHS.matches(&plan.to_lowercase()).iter().next())
            .map_or(data::BOOK_GENERIC, |month| data::BOOK_BY[month].1);
        return Some(window);
    }
    in_chat(clean, earlier, data::WHEN_TO_GO)
        .or_else(|| in_chat(clean, earlier, data::BEST_TIME))
        .map(|item| item.1)
}

fn transit(clean: &str, earlier: &[Exchange], plans: Option<&Facts>) -> Option<&'static str> {
    if data::TRANSIT_NOT.iter().any(|word| clean.contains(word)) {
        return None;
    }
    let card = named(clean, data::TRANSIT_CARDS);
    if card.is_none() && !data::TRANSIT_WORDS.iter().any(|word| clean.contains(word)) {
        return None;
    }
    let direct = in_chat(clean, earlier, data::CITY_TRANSIT)
        .or_else(|| in_plan(plans, |line| named(line, data::CITY_TRANSIT)));
    if let Some((_, reply)) = direct {
        return Some(reply);
    }
    let city = card
        .or_else(|| named(clean, data::CITY_TELLS))
        .map(|item| item.1);
    city.and_then(|city| {
        data::CITY_TRANSIT
            .iter()
            .find(|&&(name, _)| name == city)
            .map(|item| item.1)
    })
    .or_else(|| {
        data::CARD_WORDS
            .iter()
            .any(|word| named_place(clean, word))
            .then_some(data::GENERAL_CARD)
    })
}

fn day_trip(clean: &str, earlier: &[Exchange], plans: Option<&Facts>) -> Option<&'static str> {
    let by_name = |line: &str| data::TRIPS.iter().find(|trip| named_place(line, trip.0));
    let asking_way = DAY_TRIP_WAY.is_match(clean);
    let trip = by_name(clean).or_else(|| {
        if asking_way {
            earlier
                .iter()
                .rev()
                .take(2)
                .find_map(|turn| by_name(&turn.user.to_lowercase()))
        } else {
            None
        }
    })?;
    let city = city_name(clean)
        .or_else(|| {
            earlier
                .iter()
                .rev()
                .take(2)
                .find_map(|turn| city_name(&turn.user.to_lowercase()))
        })
        .or_else(|| in_plan(plans, city_name));
    if city.is_some_and(|city| city != trip.1) {
        return None;
    }
    let reply = if asking_way { trip.3 } else { trip.2 };
    Some(seasonal_reply(
        reply,
        clean,
        plans.and_then(latest_plan),
        earlier,
    ))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/travel_stays.rs"]
mod fixtures;

#[cfg(test)]
#[path = "../../../tests/fixtures/travel_lodging.rs"]
mod lodging_fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transit_day_trips_and_booking_match_in_context() {
        let mut failures = Vec::new();
        for fixtures::Case(id, prompt, history, expected) in fixtures::CASES {
            let mut plans = Facts::default();
            let earlier: Vec<_> = history
                .iter()
                .map(|&user| {
                    plans.observe(user);
                    Exchange {
                        user: user.to_owned(),
                        bot: String::new(),
                        module: "answers",
                    }
                })
                .collect();
            let reading = Reading::new(prompt);
            let actual = registered(id, &reading, &earlier, Some(&plans));
            if actual != *expected {
                failures.push(format!("{id}: {prompt} after {history:?}; held {:?}; actual {actual:?}; expected {expected:?}", plans.latest()));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn lodging_and_family_cards_preserve_authored_guards_and_replies() {
        use super::super::{guards, rows};
        let mut failures = Vec::new();
        for &lodging_fixtures::Case(id, prompt, history, guard, expected) in lodging_fixtures::CASES
        {
            let earlier: Vec<_> = history
                .iter()
                .map(|&user| Exchange {
                    user: user.to_owned(),
                    bot: String::new(),
                    module: "answers",
                })
                .collect();
            let reading = Reading::new(prompt);
            let context = guards::Context::new(&reading, &earlier);
            let row = rows()
                .find(|row| row.domain == "travel" && row.id == id)
                .unwrap_or_else(|| panic!("missing travel/{id}"));
            let expected = expected.map(|index| lodging_fixtures::REPLIES[index]);
            let active = row.guard.passes(&context, id, &[]);
            let actual = row.answer(&context);
            if (active, actual.as_deref()) != (guard, expected) {
                failures.push(format!("{id}: {prompt} after {history:?}; asks {:?}; actual {active} {actual:?}; expected {guard} {expected:?}", reading.features.asks));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn travel_questions_keep_the_current_destination_and_the_authored_module() {
        for (prompt, opening) in [
            ("Where should we stay?", "Stay in Monti"),
            ("Is it good for families?", "Rome works with children"),
            (
                "Where should I stay near Gatwick?",
                "For a night before an early flight",
            ),
            ("Where should I stay in Paris?", "Stay in Le Marais"),
        ] {
            let mut bot = crate::Bot::new(Some("0")).unwrap();
            bot.turn("We are going to Rome in June.");
            let turn = bot.turn(prompt);
            assert_eq!(turn.module, Some("answers"), "{prompt}: {turn:?}");
            assert!(turn.reply.starts_with(opening), "{prompt}: {turn:?}");
        }
    }
}
