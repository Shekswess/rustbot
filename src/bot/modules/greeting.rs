//! Compose the fallback candidate with the information-source readers.
use crate::bot::{Memory, Reading, compose};
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(alternative) = super::their_taste::advice_obstacle(reading, memory) {
        return Some(alternative);
    }
    let reply = compose::fallback::respond(reading, memory)?;
    if let Some(phone) = crate::bot::knowledge::phone_water::greeting(reading, memory, &reply) {
        return Some(phone);
    }
    if let Some(place) = compose::findout::replace(&reading.line.clean, &reply, memory) {
        return Some(place);
    }
    let hedge = compose::routes::hedge(&reply);
    Some(
        hedge
            .then(|| {
                compose::ward::duration_followup(reading, memory)
                    .or_else(|| compose::only_they_know::facts(reading, memory))
                    .or_else(|| compose::carrier::reply(reading.raw, memory))
                    .or_else(|| super::talkto::specialist_route(reading, memory))
            })
            .flatten()
            .or_else(|| compose::gap::replace(&reading.line.clean, &reply, memory))
            .or_else(|| {
                hedge
                    .then(|| compose::their_life::reply(reading.raw, memory))
                    .flatten()
            })
            .or_else(|| compose::not_knowing::replace(&reading.line.clean, &reply, memory))
            .unwrap_or(reply),
    )
}
