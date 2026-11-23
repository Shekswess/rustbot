// Changed in the cppalliance fork of RustBot: the time comes from the turn. See "Changes in this fork" in README.md.

//! Current clocks, remembered flight times, and screening follow-ups.

use std::{borrow::Cow, sync::LazyLock};

use jiff::{Timestamp, Zoned, tz::TimeZone};
use regex::Regex;

use crate::bot::{
    Exchange, Reading, memory::stated_plans::Facts, understanding::normalize::tokens,
};

use super::city_guides::named_place;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(CLOCK_ASK, super::TRAVEL_CLOCK_ASK);
pattern!(SCHEDULE, super::TRAVEL_CLOCK_SCHEDULE);
pattern!(FREQUENCY, super::TRAVEL_CLOCK_FREQUENCY);
pattern!(AVAILABLE, super::TRAVEL_CLOCK_AVAILABLE);
pattern!(WEEKDAY, super::TRAVEL_FLIGHT_DAY);
pattern!(
    OTHER_TIME,
    r"\b(?:fly|flight|flying|flights|drive|driving|walk|walking|train|bus|queue|check in|checking|passport|shops|dinner|kick off|kickoff|lunch|sunrise|sunset|tablets|bed|check out|checkout)\b"
);
pattern!(
    GIVEN_HERE,
    r"\b(?<hour>\d{1,2})(?:[:.](?<minute>\d{2}))?\s*(?<period>am|pm)\b[^.?!]*\bhere\b"
);
pattern!(DIFFERENCE, r"hours|difference|ahead|behind|zone");
pattern!(LANDING, r"\b(?:land|lands|landing)\b");
pattern!(GATE, r"\b(?:gate|terminal)\b");
pattern!(
    DEPARTURE_SAID,
    r"\b(?:(?:flight|plane|departure)\s+(?:(?:is|leaves|departs|takes off)\s+)?(?:at|is|around)|(?:fly|flying|depart|take off)\s+(?:at|around))\s+(?<clock>\d{1,2}(?:[:.]\d{2})?(?:\s*[ap]\.?m\.?)?)\b"
);
pattern!(
    TIME_BEFORE_FLIGHT,
    r"\b(?<clock>\d{1,2}(?:[:.]\d{2})?\s*(?:am|pm)|\d{1,2}[:.]\d{2})\s+(?:flight|departure)\b"
);
pattern!(
    DURATION,
    r"^\s*(?:hours?|minutes?|mins?|days?|weeks?|nights?)\b"
);

static UK: LazyLock<Option<TimeZone>> = LazyLock::new(|| crate::bot::clock::zone("Europe/London"));

fn known_place<'a>(line: &str, places: &'a [&str]) -> Option<&'a str> {
    places
        .iter()
        .copied()
        .find(|place| named_place(line, place))
}

fn zone_place(clean: &str, earlier: &[Exchange]) -> Option<&'static str> {
    let named = |line: &str| {
        known_place(line, super::TRAVEL_CLOCK_PLACES)
            .or_else(|| known_place(line, super::TRAVEL_CLOCK_WIDE_PLACES))
    };
    named(clean).or_else(|| {
        earlier
            .iter()
            .rev()
            .take(2)
            .find_map(|turn| named(&turn.user.to_lowercase()))
    })
}

// https://www.iana.org/time-zones
fn zone_name(place: &str) -> Option<&'static str> {
    Some(match place {
        "london" | "edinburgh" | "uk" | "the uk" | "britain" | "england" | "scotland" | "wales" => {
            "Europe/London"
        }
        "dublin" | "ireland" => "Europe/Dublin",
        "lisbon" | "portugal" => "Europe/Lisbon",
        "reykjavik" | "iceland" => "Atlantic/Reykjavik",
        "marrakech" | "morocco" => "Africa/Casablanca",
        "paris" | "france" => "Europe/Paris",
        "berlin" | "germany" => "Europe/Berlin",
        "madrid" | "spain" => "Europe/Madrid",
        "rome" | "italy" => "Europe/Rome",
        "amsterdam" | "holland" | "netherlands" => "Europe/Amsterdam",
        "brussels" | "belgium" => "Europe/Brussels",
        "vienna" | "austria" => "Europe/Vienna",
        "prague" | "czech republic" | "czechia" => "Europe/Prague",
        "copenhagen" | "denmark" => "Europe/Copenhagen",
        "stockholm" | "sweden" => "Europe/Stockholm",
        "oslo" | "norway" => "Europe/Oslo",
        "zurich" | "switzerland" => "Europe/Zurich",
        "croatia" => "Europe/Zagreb",
        "poland" => "Europe/Warsaw",
        "hungary" => "Europe/Budapest",
        "tunis" => "Africa/Tunis",
        "lagos" | "nigeria" => "Africa/Lagos",
        "athens" | "greece" => "Europe/Athens",
        "helsinki" | "finland" => "Europe/Helsinki",
        "cairo" | "egypt" => "Africa/Cairo",
        "cape town" => "Africa/Johannesburg",
        "tel aviv" | "israel" => "Asia/Jerusalem",
        "amman" | "jordan" => "Asia/Amman",
        "istanbul" | "turkey" => "Europe/Istanbul",
        "moscow" => "Europe/Moscow",
        "nairobi" | "kenya" => "Africa/Nairobi",
        "qatar" => "Asia/Qatar",
        "dubai" | "abu dhabi" | "uae" => "Asia/Dubai",
        "oman" => "Asia/Muscat",
        "delhi" | "mumbai" | "india" => "Asia/Kolkata",
        "sri lanka" => "Asia/Colombo",
        "bangkok" | "thailand" => "Asia/Bangkok",
        "hanoi" | "vietnam" => "Asia/Ho_Chi_Minh",
        "singapore" => "Asia/Singapore",
        "hong kong" => "Asia/Hong_Kong",
        "beijing" | "shanghai" | "china" => "Asia/Shanghai",
        "bali" => "Asia/Makassar",
        "kuala lumpur" | "malaysia" => "Asia/Kuala_Lumpur",
        "perth" => "Australia/Perth",
        "tokyo" | "japan" => "Asia/Tokyo",
        "seoul" | "south korea" => "Asia/Seoul",
        "sydney" => "Australia/Sydney",
        "melbourne" => "Australia/Melbourne",
        "fiji" => "Pacific/Fiji",
        "auckland" | "new zealand" => "Pacific/Auckland",
        "new york" | "boston" | "miami" => "America/New_York",
        "toronto" => "America/Toronto",
        "havana" | "cuba" => "America/Havana",
        "bogota" | "colombia" => "America/Bogota",
        "lima" | "peru" => "America/Lima",
        "chicago" => "America/Chicago",
        "mexico city" => "America/Mexico_City",
        "los angeles" | "san francisco" | "seattle" | "california" => "America/Los_Angeles",
        "vancouver" => "America/Vancouver",
        "honolulu" => "Pacific/Honolulu",
        "rio" | "sao paulo" => "America/Sao_Paulo",
        "buenos aires" | "argentina" => "America/Argentina/Buenos_Aires",
        "santiago" => "America/Santiago",
        _ => return None,
    })
}

fn broad_place(place: &str) -> Option<&'static str> {
    Some(match place {
        "australia" => {
            "Australia has several time zones. Which city do you mean: Sydney, Melbourne, Perth or somewhere else?"
        }
        "america" | "usa" => {
            "The US has several time zones. Which city do you mean: New York, Chicago, Los Angeles or somewhere else?"
        }
        "canada" => {
            "Canada has several time zones. Which city do you mean: Toronto, Vancouver or somewhere else?"
        }
        "brazil" => {
            "Brazil has several time zones. Which city do you mean: Rio, São Paulo or somewhere else?"
        }
        "russia" => {
            "Russia has several time zones. Which city do you mean: Moscow or somewhere else?"
        }
        "indonesia" => "Indonesia has several time zones. Which city or island do you mean?",
        "mexico" => "Mexico has several time zones. Which city do you mean?",
        "chile" => "Chile has several time zones. Which city or region do you mean?",
        "florida" | "texas" => "That state spans two time zones. Which city do you mean?",
        _ => return None,
    })
}

fn title(text: &str) -> String {
    if text == "uae" {
        return "UAE".into();
    }
    text.split_whitespace()
        .map(|word| {
            let mut letters = word.chars();
            letters
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(letters)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn clock(hour: i32, minute: i32) -> String {
    let hour = hour.rem_euclid(24);
    let twelve = (hour + 11) % 12 + 1;
    let period = if hour < 12 { "am" } else { "pm" };
    if minute == 0 {
        format!("{twelve}{period}")
    } else {
        format!("{twelve}.{minute:02}{period}")
    }
}

fn clock_text(now: &Zoned) -> String {
    clock(i32::from(now.hour()), i32::from(now.minute()))
}

fn change_time(clean: &str, gap_minutes: i32, label: &str) -> Option<String> {
    let given = GIVEN_HERE.captures(clean)?;
    let hour: i32 = given["hour"].parse().ok()?;
    let minute: i32 = given
        .name("minute")
        .map_or(Some(0), |part| part.as_str().parse().ok())?;
    let period = &given["period"];
    if !(1..=12).contains(&hour) || minute >= 60 {
        return None;
    }
    let here = hour % 12 + if period == "pm" { 12 } else { 0 };
    let supplied = if given.name("minute").is_some() {
        format!("{hour}:{minute:02}{period}")
    } else {
        format!("{hour}{period}")
    };
    let total = here * 60 + minute + gap_minutes;
    let there = clock(total.div_euclid(60), total.rem_euclid(60));
    let day = match total.div_euclid(24 * 60) {
        n if n < 0 => " on the previous day",
        n if n > 0 => " on the next day",
        _ => "",
    };
    Some(format!(
        "{supplied} on the UK clock is {there}{day} in {label} using the current time difference."
    ))
}

fn clock_there(clean: &str, place: &str, now: Timestamp) -> Option<Cow<'static, str>> {
    if OTHER_TIME.is_match(clean) {
        return None;
    }
    if let Some(reply) = broad_place(place) {
        return Some(Cow::Borrowed(reply));
    }
    let here = now.to_zoned(UK.as_ref()?.clone());
    let there = now.to_zoned(crate::bot::clock::zone(zone_name(place)?)?);
    let label = match place {
        "portugal" => "mainland Portugal".to_owned(),
        "spain" => "mainland Spain".to_owned(),
        "new zealand" => "mainland New Zealand".to_owned(),
        "uk" | "the uk" => "the UK".to_owned(),
        _ => title(place),
    };
    let gap_minutes = (there.offset().seconds() - here.offset().seconds()) / 60;
    if let Some(reply) = change_time(clean, gap_minutes, &label) {
        return Some(Cow::Owned(reply));
    }
    let there = clock_text(&there);
    let here = clock_text(&here);
    if gap_minutes == 0 {
        return Some(Cow::Owned(format!(
            "It is {there} in {label}, the same time as the UK."
        )));
    }
    let gap = gap_minutes.abs();
    let hours = gap / 60;
    let minutes = gap % 60;
    let unit = if hours == 1 { "hour" } else { "hours" };
    let difference = match (hours, minutes) {
        (0, minutes) => format!("{minutes} minutes"),
        (hours, 0) => format!("{hours} {unit}"),
        (hours, minutes) => format!("{hours} {unit} and {minutes} minutes"),
    };
    let direction = if gap_minutes > 0 {
        "ahead of"
    } else {
        "behind"
    };
    let times = format!("It is {there} in {label} now, and {here} in the UK.");
    Some(Cow::Owned(if DIFFERENCE.is_match(clean) {
        format!("{label} is currently {difference} {direction} the UK. {times}")
    } else {
        times
    }))
}

fn clock_here(clean: &str, earlier: &[Exchange], now: Timestamp) -> Option<Cow<'static, str>> {
    if !CLOCK_ASK.is_match(clean)
        || FREQUENCY.is_match(clean)
        || AVAILABLE.is_match(clean)
        || SCHEDULE.is_match(clean)
        || zone_place(clean, earlier).is_some()
    {
        return None;
    }
    let here = now.to_zoned(UK.as_ref()?.clone());
    Some(Cow::Owned(format!(
        "It is {} on the UK clock.",
        clock_text(&here)
    )))
}

fn flight_time(line: &str) -> Option<String> {
    let captures = DEPARTURE_SAID
        .captures(line)
        .or_else(|| TIME_BEFORE_FLIGHT.captures(line))?;
    let clock = captures.name("clock")?;
    if DURATION.is_match(&line[clock.end()..]) {
        return None;
    }
    let mut digits = clock.as_str().split(|c: char| !c.is_ascii_digit());
    let hour: u32 = digits.next()?.parse().ok()?;
    let minute: u32 = digits
        .next()
        .filter(|part| !part.is_empty())
        .map_or(Some(0), |part| part.parse().ok())?;
    if hour > 23 || minute > 59 {
        return None;
    }
    let day = WEEKDAY
        .captures(line)
        .map(|day| match &day[1] {
            part @ ("tomorrow" | "tonight" | "this morning" | "this evening") => format!(" {part}"),
            weekday => format!(" on {}", title(weekday)),
        })
        .unwrap_or_default();
    Some(format!("{}{day}", clock.as_str().trim_end_matches('.')))
}

fn my_flight(clean: &str, earlier: &[Exchange], plans: Option<&Facts>) -> Cow<'static, str> {
    if LANDING.is_match(clean) {
        return Cow::Borrowed(
            "The scheduled landing time is on your ticket in the destination's local time. Check the airline's app for the live arrival time on the day.",
        );
    }
    if GATE.is_match(clean) {
        return Cow::Borrowed(
            "Check your boarding pass and the airline's app for your terminal and gate. They can change, so check the airport departure boards on the day.",
        );
    }
    let held = plans
        .and_then(|plans| {
            plans
                .latest()
                .and_then(|plan| flight_time(&plan.to_lowercase()))
                .or_else(|| {
                    plans
                        .plans()
                        .rev()
                        .find_map(|plan| flight_time(&plan.to_lowercase()))
                })
        })
        .or_else(|| {
            earlier
                .iter()
                .rev()
                .take(3)
                .find_map(|turn| flight_time(&turn.user.to_lowercase()))
        });
    if let Some(held) = held {
        Cow::Owned(format!(
            "You told me your flight is at {held}. Check your ticket or your airline's app for the scheduled departure, then work back using the airport's arrival guidance and your airline's check-in and gate deadlines."
        ))
    } else {
        Cow::Borrowed(
            "Check your ticket or your airline's app for your departure time. It is shown in the local time at the departure airport. If you share the time and any check-in or gate deadline, I can help work back to when you need to leave.",
        )
    }
}

fn flight_thread(earlier: &[Exchange]) -> bool {
    earlier.last().is_some_and(|last| {
        [&last.user, &last.bot].into_iter().any(|line| {
            tokens(line)
                .iter()
                .any(|word| super::TRAVEL_FLIGHT_THREAD.contains(&word.as_str()))
        })
    })
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
    plans: Option<&Facts>,
) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    match id {
        "time-here" => clock_here(clean, earlier, reading.now().timestamp()),
        "time-there" => clock_there(
            clean,
            zone_place(clean, earlier)?,
            reading.now().timestamp(),
        ),
        "my-flight-time" => Some(my_flight(clean, earlier, plans)),
        "flight-food-followup" if flight_thread(earlier) => {
            Some(Cow::Borrowed(super::TRAVEL_FLIGHT_FOOD))
        }
        "flight-liquids-followup" if flight_thread(earlier) => {
            Some(Cow::Borrowed(super::TRAVEL_FLIGHT_LIQUIDS))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_zones_cover_pinned_places_and_real_clock_changes() {
        for &place in super::super::TRAVEL_CLOCK_PLACES {
            assert!(
                broad_place(place).is_some()
                    || zone_name(place).is_some_and(|zone| crate::bot::clock::zone(zone).is_some()),
                "{place}"
            );
        }
        let now = "2026-03-20T15:00:00Z".parse().unwrap();
        let reply =
            clock_there("what is the time difference in new york", "new york", now).unwrap();
        assert!(reply.contains("4 hours behind"));
        assert!(reply.contains("11am in New York"));
        let now = "2026-11-18T15:00:00Z".parse().unwrap();
        assert!(
            clock_there("what time in delhi if it is 8.45pm here", "delhi", now)
                .unwrap()
                .contains("2.15am on the next day")
        );
        assert!(
            clock_here("what time is it", &[], now)
                .unwrap()
                .contains("3pm")
        );
        assert!(clock_here("the fan runs all the time now", &[], now).is_none());
    }

    #[test]
    fn flight_clock_and_screening_preserve_their_context() {
        assert_eq!(
            flight_time("my flight is at 7 on friday").as_deref(),
            Some("7 on Friday")
        );
        assert!(flight_time("i leave at 6am for the flight").is_none());
        let earlier = [Exchange {
            user: "My flight is at 8pm".into(),
            bot: String::new(),
            module: "answers",
        }];
        assert!(my_flight("what time is my flight", &earlier, None).contains("at 8pm"));
        let reading = Reading::new("Can I bring my own water?");
        assert!(registered("flight-liquids-followup", &reading, &[], None).is_none());
        assert!(
            registered("flight-liquids-followup", &reading, &earlier, None)
                .unwrap()
                .contains("varies")
        );
    }
}
