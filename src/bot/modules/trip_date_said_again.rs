//! Observe a correction to a trip that was already filed.

use fancy_regex::Captures;

use crate::{
    bot::{
        Memory,
        memory::{self, travel_trip},
    },
    text::is_space,
};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "trip_date_said_again_data.rs"]
mod data;
use data::*;

pattern!(ORDINAL, r"(?i)^\d{1,2}(?:st|nd|rd|th)$");

fn flat(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_a_date(text: &str) -> bool {
    let mut named = false;
    let mut number = false;
    for word in text.to_lowercase().split_whitespace() {
        if matches!(word, "of" | "the") {
            continue;
        }
        if A_DATE_WORD.contains(&word) || ORDINAL.is_match(word).unwrap_or_default() {
            named = true;
        } else if !number && word.chars().count() <= 2 && word.chars().all(char::is_numeric) {
            number = true;
        } else {
            return false;
        }
    }
    named
}

fn tidy(date: &str) -> String {
    let date = date
        .trim_matches(is_space)
        .trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"']);
    let mut chars = date.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn chosen(groups: Captures<'_>) -> Option<String> {
    groups
        .iter()
        .skip(1)
        .flatten()
        .find(|part| is_a_date(part.as_str()))
        .map(|date| tidy(date.as_str()))
}

fn travelling(text: &str) -> Option<String> {
    if text.contains('?') {
        return None;
    }
    chosen(TRIP_DATE_MOVED.captures(text).ok().flatten()?)
}

fn said_now(text: &str) -> Option<String> {
    if text.contains('?') {
        return None;
    }
    let date = A_DATE_SAID_NOW.captures(text).ok().flatten()?;
    A_MARKER_SAID
        .is_match(&text[..date.get(1)?.start()])
        .ok()?
        .then_some(())?;
    chosen(date)
}

fn date_moved(text: &str) -> Option<String> {
    travelling(text)
        .or_else(|| {
            let head = text.split(',').next()?.trim_matches(is_space);
            (head
                .split(is_space)
                .filter(|word| !word.is_empty())
                .take(2)
                .count()
                == 2)
                .then(|| travelling(head))
                .flatten()
        })
        .or_else(|| said_now(text))
}

fn names_month(line: &str, month: &str) -> bool {
    line.split(|ch: char| !ch.is_alphabetic())
        .any(|word| word.eq_ignore_ascii_case(month))
}

fn already_said(month: &str, said: &str, memory: &Memory) -> bool {
    let line = flat(said).to_lowercase();
    memory.stated_plans.values().any(|plan| {
        let stem = flat(plan).to_lowercase();
        !(stem.contains(&line) || line.contains(&stem)) && names_month(&stem, month)
    })
}

pub(crate) fn observe(text: &str, memory: &mut Memory) {
    memory.trip_date_reply = None;
    let Some(target) = date_moved(text) else {
        return;
    };
    let Some(month) = travel_trip::named_month(&target) else {
        return;
    };
    if !month.eq_ignore_ascii_case(&target) || already_said(&month, text, memory) {
        return;
    }
    let eligible = |trip: &travel_trip::Trip<'_>| trip.plan.is_some() && trip.month.is_some();
    let Some(trip) = travel_trip::in_hand(memory).filter(eligible).or_else(|| {
        travelling(text).and_then(|_| travel_trip::in_hand_before(memory, text).filter(eligible))
    }) else {
        return;
    };
    let (Some(plan), Some(old_month)) = (trip.plan, trip.month.as_ref()) else {
        return;
    };
    if month == *old_month {
        return;
    }
    let plan = plan.to_owned();
    let said_before = trip.origin;
    let place = trip.place;
    let Some(updated) =
        memory
            .stated_plans
            .move_trip_month(&plan, &said_before, old_month, &target, text)
    else {
        return;
    };
    memory
        .personal
        .move_trip_month(&place, &target, memory.turns.len());

    let mut reply = memory::ack::you(&updated)
        .trim_end_matches([' ', '.', '!', '?'])
        .to_owned();
    if let Some(display) = travel_trip::display_place(&place)
        && let Ok(pattern) = regex::Regex::new(&format!(r"(?i)\b{}\b", regex::escape(&place)))
    {
        reply = pattern.replace(&reply, display.as_str()).into_owned();
    }
    memory.trip_date_reply = Some(format!("Got it — {reply}."));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_date_needs_the_source_correction_marker() {
        assert_eq!(
            date_moved("actually we fly in september").as_deref(),
            Some("September")
        );
        assert_eq!(
            date_moved("actually its june now, my sister moved her holiday").as_deref(),
            Some("June")
        );
        assert_eq!(date_moved("we fly in september"), None);
        assert_eq!(date_moved("is it september now?"), None);
        assert_eq!(date_moved("actually its broken now"), None);
    }

    use crate::Bot;

    #[test]
    fn a_reason_for_the_move_keeps_the_other_plan_and_replaces_the_original_date() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("im going to lisbon in may with my sister");
        bot.turn("we want to see the old town and eat well");
        let reply = bot.turn("actually its june now, my sister moved her holiday");
        assert_eq!(
            bot.memory.stated_plans.plans().collect::<Vec<_>>(),
            [
                "Lisbon in June with my sister",
                "See the old town and eat well"
            ]
        );
        assert_eq!(reply.module, Some("memory_store"));
        assert_eq!(
            reply.reply,
            "Got it — you're going to Lisbon in June with your sister."
        );
        let trip = travel_trip::in_hand(&bot.memory).unwrap();
        assert_eq!(trip.month.as_deref(), Some("june"));
        assert!(trip.origin.ends_with("with my sister"));
    }

    #[test]
    fn a_booked_cottage_keeps_its_place_and_replaces_the_month() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("we booked a cottage in cornwall for august");
        let reply = bot.turn("actually we drive in september");
        assert_eq!(
            bot.memory.stated_plans.latest(),
            Some("Cornwall in September")
        );
        assert_eq!(
            bot.memory.stated_plans.plans().collect::<Vec<_>>(),
            ["Cornwall in September"]
        );
        assert_eq!(reply.module, Some("memory_store"));
        assert_eq!(
            reply.reply,
            "Got it — you booked a cottage in Cornwall for September."
        );
    }

    #[test]
    fn a_new_destination_in_the_wording_still_moves_only_the_date() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("im going to spain in june");
        let reply = bot.turn("actually we fly to Porto in september");
        assert_eq!(bot.memory.stated_plans.latest(), Some("Spain in September"));
        assert_eq!(reply.reply, "Got it — you're going to Spain in September.");
        assert_eq!(bot.turn("when do we fly").reply, "In September.");
    }
}
