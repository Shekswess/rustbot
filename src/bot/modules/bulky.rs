//! Bulky waste disposal reader and three-turn object lookup.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;
static FLOOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:carpets?|carpet\s+tiles?|underlay|lino(?:leum)?|vinyl\s+flooring|vinyl\s+tiles?|flooring|doormats?)\b"#).unwrap()
});
static FURNITURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:sofas?|settees?|armchairs?|mattress(?:es)?|beds?|wardrobes?|tables?|chairs?|desks?|bookcases?|bookshelves?|shelving|cabinets?|sideboards?|chests?\s+of\s+drawers|furniture)\b"#).unwrap()
});
static ELECTRICAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:fridges?|freezers?|washing\s+machines?|dishwashers?|tumble\s+dryers?|ovens?|cookers?|microwaves?|tvs?|televisions?|monitors?|computers?|laptops?|printers?|kettles?|hoovers?|vacuum\s+cleaners?|boilers?|radiators?)\b"#).unwrap()
});
static GARDEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:trampolines?|sheds?|fences?|patios?|slabs?|soil|bushes|bush|trees?|garden\s+waste|green\s+waste)\b"#).unwrap()
});
static DISPOSAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:what\s+(?:do|should|can)\s+(?:i|we)\s+do\s+with|how\s+(?:do|can|should)\s+(?:i|we)\s+(?:get\s+rid\s+of|dispose\s+of|throw\s+(?:away|out))|where\s+(?:can|do|should)\s+(?:i|we)\s+(?:take|put|get\s+rid\s+of)|get\s+rid\s+of|dispose\s+of|throw\s+(?:away|out)|take\s+(?:it\s+)?to\s+the\s+tip|goes?\s+in\s+the\s+bin|recycle)\b"#).unwrap()
});
static REPLACING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:replacing|pulling\s+up|taking\s+up|ripping\s+up|(?:having|getting)\s+(?:the\s+|a\s+)?[a-z ]{0,12}(?:replaced|lifted))\b"#).unwrap()
});
static FINISHED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:old|broken|worn|worn\s+out|tired|replaced|replacing|ripped|stained|damp|smelly|had\s+it|no\s+good|unwanted|spare|left\s?over|pulled\s+up)\b"#).unwrap()
});
static POINTING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)\b(?:it|that|this|one|them|those|these)\b"#).unwrap());
static NOT_OURS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:asbestos|needles?|syringe|chemicals?|acid|gas\s+(?:bottle|cylinder)|oil|paint|solvent|pesticide|weedkiller|medical|clinical)\b"#).unwrap()
});
static KEEPS_ITS_ANSWER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:clean|cleaning|wash|washing\s+it|stain|smell|moth|moths|remove|mend|repair|fit|lay|choose|choosing|buy|buying)\b"#).unwrap()
});
const FLOOR_REPLY: &str = "Roll the {thing} up and cut it into lengths you can lift, then take it to the recycling centre - most tips take {thing} in the general waste skip. If it will not fit in a car, book a bulky waste collection with your council; most charge twenty to forty pounds for a doorstep pickup. Check your council's site for the fee and the booking, because I can't see your council's charges from here.";
const FURNITURE_REPLY: &str = "Take the {thing} apart if it will come apart, then book a bulky waste collection with your council - most charge twenty to forty pounds - or take it to the recycling centre. If it is still usable, a charity or a local reuse group may collect it; the British Heart Foundation collects furniture in many areas. Check your council's site for the fee and the collection, because I can't see your council's charges from here.";
const ELECTRICAL_REPLY: &str = "Anything with a plug or a battery is electrical waste, so it cannot go in the bin: a shop that sells the same kind of thing has to take it back, and the recycling centre has an electricals point. A {thing} is also a bulky collection if you cannot move it - ask the shop delivering a new one to take the old one. Check your council's site for the fee, because I can't see your council's charges from here.";
const GARDEN_REPLY: &str = "Dismantle the {thing} as far as it will come apart and take it to the recycling centre, or book a bulky waste collection with your council; most charge twenty to forty pounds. The metal goes in the scrap metal skip and the rest in general waste, and green garden waste goes in the council's green bin. Check your council's site for the fee and any permit, because I can't see your council's charges from here.";
fn group(line: &str) -> Option<(&'static str, String)> {
    [
        (&*FLOOR, FLOOR_REPLY),
        (&*FURNITURE, FURNITURE_REPLY),
        (&*ELECTRICAL, ELECTRICAL_REPLY),
        (&*GARDEN, GARDEN_REPLY),
    ]
    .into_iter()
    .find_map(|(pattern, reply)| {
        pattern
            .find(line)
            .map(|m| (reply, m.as_str().to_lowercase()))
    })
}
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.folded();
    let disposal = DISPOSAL.is_match(line);
    let replacing = REPLACING.is_match(line);
    if (!disposal && !replacing) || NOT_OURS.is_match(line) || KEEPS_ITS_ANSWER.is_match(line) {
        return None;
    }
    let pointing = POINTING.is_match(line);
    let (reply, item) = group(line).or_else(|| {
        if !pointing {
            return None;
        }
        memory
            .turns
            .iter()
            .rev()
            .take(3)
            .find_map(|turn| group(&turn.user))
    })?;
    if replacing && !disposal {
        return (reply == FLOOR_REPLY).then(|| {
            "Good. What are you doing with the old one - tip or a council collection?".into()
        });
    }
    (pointing || FINISHED.is_match(line)).then(|| reply.replace("{thing}", &item))
}
