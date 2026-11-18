//! Airport arrival planning and live wait referrals for PyBot's airport-arrival card.

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

pub(crate) const LIVE_QUEUE: &str = "I can't see the current queue. Check the airport's website or app for any live wait updates, or ask airport staff. Check your airline's app for flight and boarding updates.";

const DEADLINES: &str =
    "Check your airline's check-in, bag-drop and gate deadlines for your flight.";
const PLAN: &str = "Check your departure airport's arrival guidance and your airline's check-in, bag-drop and gate deadlines. Allow time for the journey to the airport, security and reaching the gate.";
// Heathrow's guidance for departures. https://www.heathrow.com/heathrow-blog/top-tips-for-timing-your-airport-journey
const HEATHROW: &str = "If you're departing from Heathrow, the airport recommends arriving two hours before a UK or European flight and three hours before a long-haul flight. Check your airline's check-in, bag-drop and gate deadlines for your flight.";
// Gatwick's typical check-in opening times. https://www.gatwickairport.com/passenger-guides/checking-in.html
const GATWICK: &str = "If you're departing from Gatwick, the airport recommends arriving at least two hours before departure. Check-in normally opens around four hours before long-haul flights and three hours before European flights. Gatwick lists two hours for UK and Ireland check-in. Confirm your airline's check-in, bag-drop and gate deadlines.";
const CONNECTION: &str = "Check your itinerary and ask the airline about the time needed to make your connection. Allow extra time if you have separate bookings and need to collect bags, change terminals, check in again or go through security.";
// Heathrow's connecting flight guidance. https://www.heathrow.com/connecting-flights
const HEATHROW_CONNECTION: &str = "For a Heathrow connection on one ticket, ask the airline for your itinerary's minimum connection time. With separate bookings, allow time for any immigration, bag collection, terminal transfer, check-in and security you need to do.";

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    OTHER_TRAVEL,
    concat!(
        r"\b(?:cancel|cancelled|canceled|cancelling|canceling|refunds?|compensation|",
        r"delay|delayed|voucher|cheap|cheaper|cheapest|prices?|fares?|deals?|cost|",
        r"cruise|ferr(?:y|ies)|boat|ship|trains?|rail|eurostar|coach|bus|hotel|hostel|",
        r"campsite|book|books|booking|booked|buy|buying)\b"
    )
);
pattern!(
    FLIGHT_LENGTH,
    concat!(
        r"\b(?:how long(?:'s| is) (?:the |a |my )?flight|how long does (?:the |a )?flight take|",
        r"how long to fly|how many hours (?:is|does|to) (?:a |the |my )?flight|",
        r"how many hours to fly|how long is the plane|is it a long flight|is it a long plane|",
        r"long flight to|flight time to|how long will (?:the |my )?flight take|",
        r"how long (?:does|do) it take to fly)\b"
    )
);
pattern!(PACKING, r"\b(?:take|bring|carry|pack|eat)\b");
pattern!(
    FOOD,
    r"\b(?:foods?|snacks?|eat(?:ing)?|meals?|sandwich(?:es)?|lunch|picnic|drinks?)\b"
);
pattern!(
    CONNECTION_ASK,
    r"\b(?:connecting|connections?|layover|stopover|transit|chang(?:e|ing) planes)\b"
);
pattern!(ARRIVAL_SCOPE, r"\b(?:airport|before|leave|early)\b");
pattern!(
    LANDING,
    r"\b(?:land|lands|landing|arrive at the other|get in at)\b"
);
pattern!(TRANSFER, r"\btransfers?\b");
pattern!(
    IN_FLIGHT,
    r"\b(?:on (?:the|my|a|our) (?:plane|flight)|(?:sleep|nap) before (?:i|you|we) fly)\b"
);
pattern!(
    MISSED_FLIGHT,
    r"\b(?:miss(?:ed|ing|es)? (?:my|our|the|a) (?:flight|plane))\b"
);
pattern!(
    MISSED_SCENARIO,
    r"\b(?:what happens|what do i do|missed|too late|no.?show|rebook(?:ed)?)\b"
);
pattern!(QUEUE, r"\b(?:queues?|wait(?:ing)?|busy|backed up)\b");
pattern!(
    CHECK_IN_WAIT,
    r"\bhow long\b[^?.]*\b(?:check[ -]?in|security|bag drop|passport control)\b[^?.]*\btake\b"
);
pattern!(
    ARRIVAL_QUESTION,
    concat!(
        r"\b(?:how early|how long before|(?:what time|when) (?:should|do|can) (?:i|we) ",
        r"(?:arrive|get (?:there|to)|leave|be (?:there|at)))\b"
    )
);
pattern!(
    ARRIVAL_TARGET,
    r"\b(?:airport|flight|boarding|check[ -]?in|gate|takeoff)\b"
);
pattern!(HEATHROW_NAME, r"\b(?:heathrow|lhr)\b");
pattern!(GATWICK_NAME, r"\b(?:gatwick|lgw)\b");
pattern!(
    LEAVE_HOME,
    r"\b(?:leave home|leave the house|leav(?:e|ing) for the airport|should (?:i|we) leave|set off|set out)\b"
);
pattern!(
    DEPARTURE,
    concat!(
        r"\b(?:(?:flight|plane) (?:is|leaves|departs|takes off) at|(?:fly|depart|take off) at|",
        r"departure (?:at|is at))\s+",
        r"(?<hour>[0-9]{1,2})(?:[:.](?<minute>[0-9]{2}))?\s*(?<period>[ap]\.?m)?\b"
    )
);
pattern!(
    ARRIVAL_BUFFER,
    concat!(
        r"\b(?:arrive|be (?:there|at the airport)|get (?:there|to the airport))\s+",
        r"(?:(?<hours>[0-9]{1,2}|an?|one|two|three|four|five|six)\s*(?:hours?|hrs?)(?: and)?\s*)?",
        r"(?:(?<minutes>[0-9]{1,3})\s*(?:minutes?|mins?)\s+)?",
        r"(?:before|ahead of)\b"
    )
);
pattern!(
    PROPOSED_BUFFER,
    concat!(
        r"\b(?:is (?:that|it) enough|(?:i|we) (?:can|could) arrive|",
        r"(?:do|would) (?:i|we) need to arrive|(?:don't|do not|never) (?:arrive|get there|be there))\b"
    )
);

pub(crate) fn is_live_queue(text: &str) -> bool {
    !ARRIVAL_QUESTION.is_match(text) && (QUEUE.is_match(text) || CHECK_IN_WAIT.is_match(text))
}

pub(crate) fn asks_when_to_arrive(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    ARRIVAL_TARGET.is_match(text) && ARRIVAL_QUESTION.is_match(text)
}

pub(crate) fn arrival_calculation(reading: &Reading<'_>) -> Option<(i32, i32)> {
    let text = &reading.line.clean;
    if !asks_when_to_arrive(reading) || PROPOSED_BUFFER.is_match(text) {
        return None;
    }
    departure_minutes(text).zip(arrival_buffer(text))
}

/// Render the approved guidance for a match to the original travel card.
pub(crate) fn reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    if OTHER_TRAVEL.is_match(text)
        || FLIGHT_LENGTH.is_match(text)
        || (!ARRIVAL_QUESTION.is_match(text) && IN_FLIGHT.is_match(text))
        || (!ARRIVAL_SCOPE.is_match(text) && LANDING.is_match(text))
        || (PACKING.is_match(text) && FOOD.is_match(text))
    {
        return None;
    }
    if CONNECTION_ASK.is_match(text) {
        return Some(
            if HEATHROW_NAME.is_match(text) {
                HEATHROW_CONNECTION
            } else {
                CONNECTION
            }
            .into(),
        );
    }
    if TRANSFER.is_match(text)
        || (MISSED_FLIGHT.is_match(text)
            && MISSED_SCENARIO.is_match(text)
            && !DEPARTURE.is_match(text))
    {
        return None;
    }
    if is_live_queue(text) {
        return Some(LIVE_QUEUE.into());
    }
    if let Some((departure, buffer)) = arrival_calculation(reading) {
        let arrival = departure - buffer;
        let clock = arrival.rem_euclid(24 * 60);
        let day = if arrival < 0 { " the previous day" } else { "" };
        let journey = if LEAVE_HOME.is_match(text) {
            " Work back from that arrival time using your journey time."
        } else {
            ""
        };
        return Some(
            format!(
                "Using that arrival buffer, aim to be at the airport by {:02}:{:02}{day}.{journey} {DEADLINES}",
                clock / 60,
                clock % 60
            )
            .into(),
        );
    }
    Some(
        match (HEATHROW_NAME.is_match(text), GATWICK_NAME.is_match(text)) {
            (true, false) => HEATHROW,
            (false, true) => GATWICK,
            _ => PLAN,
        }
        .into(),
    )
}

fn departure_minutes(text: &str) -> Option<i32> {
    let mut clocks = DEPARTURE.captures_iter(text);
    let clock = clocks.next()?;
    if clocks.next().is_some() {
        return None;
    }
    let hour: i32 = clock.name("hour")?.as_str().parse().ok()?;
    let minute: i32 = clock
        .name("minute")
        .map_or(Some(0), |value| value.as_str().parse().ok())?;
    let hour = match clock.name("period").map(|value| value.as_str()) {
        Some("am" | "a.m") if (1..=12).contains(&hour) => hour % 12,
        Some("pm" | "p.m") if (1..=12).contains(&hour) => hour % 12 + 12,
        None if hour < 24
            && clock.name("minute").is_some()
            && (hour >= 13 || clock["hour"].starts_with('0')) =>
        {
            hour
        }
        _ => return None,
    };
    (minute < 60).then_some(hour * 60 + minute)
}

fn arrival_buffer(text: &str) -> Option<i32> {
    let mut buffers = ARRIVAL_BUFFER.captures_iter(text);
    let buffer = buffers.next()?;
    if buffers.next().is_some() {
        return None;
    }
    let hours = match buffer.name("hours").map(|value| value.as_str()) {
        None => 0,
        Some("a" | "an" | "one") => 1,
        Some("two") => 2,
        Some("three") => 3,
        Some("four") => 4,
        Some("five") => 5,
        Some("six") => 6,
        Some(number) => number.parse().ok()?,
    };
    let minutes = buffer
        .name("minutes")
        .map_or(Some(0), |value| value.as_str().parse().ok())?;
    let total = hours * 60 + minutes;
    (1..24 * 60).contains(&total).then_some(total)
}
