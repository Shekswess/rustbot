//! Destination and trip context for travel health cards.

use std::{borrow::Cow, sync::LazyLock};

use fancy_regex::Regex;

use crate::bot::Reading;

macro_rules! source_pattern {
    ($name:ident, $pattern:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(super::$pattern).expect(stringify!($pattern)));
    };
}

source_pattern!(ROUTINE_PLACE, TRIP_ROUTINE_PLACE);
source_pattern!(PLACE, TRIP_PLACE);
source_pattern!(AWAY, TRIP_AWAY);
source_pattern!(ANIMAL, TRIP_ANIMAL);
source_pattern!(BARE_NEED, TRIP_BARE_NEED);
source_pattern!(NOT_A_TRIP, TRIP_NOT_A_TRIP);
source_pattern!(MALARIA, TRIP_MALARIA_ASKED);
source_pattern!(YELLOW_FEVER, TRIP_YELLOW_ASKED);

fn has(regex: &Regex, text: &str) -> bool {
    regex.is_match(text).unwrap_or(false)
}

pub(super) fn a_trip_is_named(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    !has(&ANIMAL, text)
        && (has(&PLACE, text)
            || has(&AWAY, text)
            || (has(&BARE_NEED, text) && !has(&NOT_A_TRIP, text)))
}

const BOOK: &str = "Contact a travel vaccination service 4 to 6 weeks before departure if possible. Ask even if you leave sooner, since recommendations depend on your itinerary, health and vaccination history.";
const MEDICATION: &str = "Check your airline's requirements and the destination and transit rules with each country's embassy. Keep essential medicines in their original labelled packaging and ask your pharmacist how to carry any that need cooling. Carry proof of prescription where required, including for controlled medicines, and check import limits before packing. Check GOV.UK rules for carrying medicines through UK airports.";

pub(super) fn medication_abroad(_: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(MEDICATION))
}

pub(super) fn the_jabs_reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    let place = [&*ROUTINE_PLACE, &*PLACE]
        .into_iter()
        .find_map(|pattern| pattern.captures(text).ok().flatten()?.get(1))
        .map(|place| place.as_str());
    let lead = if has(&MALARIA, text) {
        "Check malaria risk for the areas you will visit and ask a travel clinician about bite prevention and whether you need tablets."
    } else if has(&YELLOW_FEVER, text) {
        "Check yellow fever vaccination and certificate requirements for the specific areas and countries on your route, including transit stops."
    } else {
        "Check the NHS travel vaccination advice and the destination health advice on GOV.UK. Make sure your routine vaccines are up to date."
    };
    Some(Cow::Owned(match place {
        Some(place) => {
            format!("{lead} Check the advice for {place} and the rest of your itinerary. {BOOK}")
        }
        None => format!("{lead} {BOOK}"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn travel_health_cards_distinguish_trip_from_pet_and_child() {
        let trip = Reading::new("Do I need malaria tablets for Kenya?");
        assert!(a_trip_is_named(&trip));
        let reply = the_jabs_reply(&trip).unwrap();
        assert!(reply.starts_with("Check malaria risk"));
        assert!(reply.contains("kenya"));
        assert!(!a_trip_is_named(&Reading::new(
            "Does my baby need her jabs?"
        )));
        assert!(!a_trip_is_named(&Reading::new(
            "Vaccination for my dog to travel abroad"
        )));
    }
}
