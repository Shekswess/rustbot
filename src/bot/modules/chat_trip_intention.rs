//! The travel-plan subset of the source chat intention reader.

use crate::bot::{Memory, Reading, compose, memory::ack, understanding::kinds::Kind};

const SPEAKER: &[&str] = &["i'm ", "im ", "i am ", "we're ", "we are "];
const INTENTION: &[&str] = &[
    "thinking of ",
    "thinking about ",
    "considering ",
    "toying with ",
    "planning to ",
    "hoping to ",
    "looking to ",
    "keen to ",
    "meaning to ",
];
const TRAVEL: &[&str] = &[
    "go to ",
    "going to ",
    "travel to ",
    "travelling to ",
    "traveling to ",
    "visit ",
    "visiting ",
];
const CARE_OR_LOSS: &[&str] = &[
    "ambulance",
    "bereavement",
    "burial",
    "chemotherapy",
    "clinic",
    "crematorium",
    "death",
    "dentist",
    "died",
    "doctor",
    "doctors",
    "funeral",
    "gp",
    "hospital",
    "hospice",
    "inhaler",
    "insulin",
    "medication",
    "medicine",
    "memorial",
    "pharmacy",
    "pills",
    "radiotherapy",
    "surgery",
    "tablets",
    "treatment",
];
const WEEKDAYS: &[&str] = &[
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
];
const MONTHS: &[&str] = &[
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];

fn strip_any<'a>(text: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    prefixes.iter().find_map(|prefix| text.strip_prefix(prefix))
}

fn remove_schedule(tail: &str) -> Option<bool> {
    let Some(schedule) = tail.strip_prefix(' ') else {
        return tail.is_empty().then_some(false);
    };
    let words = schedule.split_whitespace().collect::<Vec<_>>();
    match words.as_slice() {
        ["tonight" | "today" | "tomorrow" | "later" | "soon" | "now"]
        | ["later", "on"]
        | ["at", "the", "weekend"]
        | ["at", "some", "point"]
        | [
            "this",
            "evening" | "afternoon" | "morning" | "weekend" | "week" | "year",
        ]
        | ["next", "week" | "weekend" | "month" | "year"] => Some(true),
        [day] | ["on" | "next" | "this", day] if WEEKDAYS.contains(day) => Some(false),
        [day, "morning" | "afternoon" | "evening"]
        | ["on", day, "morning" | "afternoon" | "evening"]
            if WEEKDAYS.contains(day) =>
        {
            Some(false)
        }
        ["in", month] if MONTHS.contains(month) => Some(false),
        _ => None,
    }
}

fn named_plan<'a>(reading: &'a Reading<'_>) -> Option<&'a str> {
    let said = reading.clean.trim_matches([' ', '.', ',', '!', ';', ':']);
    let thing = strip_any(strip_any(said, SPEAKER)?, INTENTION)?;
    let destination = strip_any(thing, TRAVEL)?;
    if thing.split_whitespace().count() > 8 {
        return None;
    }
    reading
        .entities
        .iter()
        .filter(|entity| entity.kind == "place")
        .find_map(|place| {
            let tail = destination.strip_prefix(&place.text.to_lowercase())?;
            let trim = remove_schedule(tail)?;
            Some(if trim {
                thing[..thing.len() - tail.len()].trim_end()
            } else {
                thing
            })
        })
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory, words: &[&str]) -> Option<String> {
    if reading.kind != Kind::Statement
        || !reading.features.asks.is_empty()
        || reading.line.signals.question_mark
        || !(3..=16).contains(&words.len())
    {
        return None;
    }
    let thing = named_plan(reading)?;
    if reading.body.is_some()
        || reading.own_topic.is_some_and(|topic| topic.0 == "health")
        || words.iter().any(|word| {
            CARE_OR_LOSS.contains(word)
                || matches!(
                    *word,
                    "if" | "unless" | "but" | "without" | "or" | "though" | "although"
                )
        })
        || compose::loss_reader::reads_as_loss(reading.raw)
        || memory
            .turns
            .iter()
            .rev()
            .take(6)
            .any(|turn| compose::loss_reader::reads_as_loss(&turn.user))
        || super::super::answers::offer(reading, memory).is_some()
    {
        return None;
    }
    let said = ack::you(thing);
    let mut chars = said.chars();
    let label: String = chars.next()?.to_uppercase().chain(chars).collect();
    let named = format!("{label}? Good thing to want. What's the bit you're least sure about?");
    let pool = [
        named.as_str(),
        "Worth planning rather than rushing into: what would have to go right for you?",
        "I'd say do it if the practical side works out. Want to work through what it would take?",
    ];
    Some(compose::pick(&pool, memory, "").to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;
    use crate::bot::understanding::normalize::tokens;

    fn reply(raw: &str, memory: &Memory) -> Option<String> {
        let words = tokens(raw);
        let words = words.iter().map(String::as_str).collect::<Vec<_>>();
        respond(&Reading::new(raw), memory, &words)
    }

    fn record(memory: &mut Memory, user: &str) {
        memory.turns.push(Exchange {
            user: user.to_owned(),
            bot: String::new(),
            module: "chat",
        });
    }

    #[test]
    fn picks_the_source_intention_reply_after_forty_turns() {
        let mut memory = Memory::default();
        for _ in 0..40 {
            record(&mut memory, "That makes sense.");
        }
        assert_eq!(
            reply("I'm planning to go to York on Saturday.", &memory).as_deref(),
            Some("Worth planning rather than rushing into: what would have to go right for you?")
        );
    }

    #[test]
    fn reads_named_travel_plans_and_source_schedule_tails() {
        for (raw, expected) in [
            (
                "We're thinking of visiting Edinburgh next weekend.",
                "visiting edinburgh",
            ),
            (
                "I'm hoping to travel to New York later on.",
                "travel to new york",
            ),
            (
                "I'm planning to go to Mexico City in May.",
                "go to mexico city in may",
            ),
        ] {
            assert_eq!(named_plan(&Reading::new(raw)), Some(expected), "{raw}");
        }
    }

    #[test]
    fn yields_for_questions_care_conditional_plans_and_recent_loss() {
        let mut memory = Memory::default();
        for raw in [
            "I'm planning to go to Rome, what should I see?",
            "I'm planning to go to York Hospital tomorrow.",
            "I'm planning to go to York for a funeral.",
            "I'd go to York, but my dad is ill.",
            "I'm planning to go to York tomorrow if it stays dry.",
            "I'm planning to go to York on Saturday without taking my medication.",
            "I'm planning to go to York or Bath.",
        ] {
            assert!(reply(raw, &memory).is_none(), "{raw}");
        }
        record(&mut memory, "My dad died in March.");
        assert!(reply("I'm planning to go to York tomorrow.", &memory).is_none());
    }
}
