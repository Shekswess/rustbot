//! Immediate help and nearby follow-ups for suspected medicine poisoning.

use super::a_count_of_packets::{ANIMAL, MEDICINE};
use crate::bot::{Memory, Reading, compose, memory};
use regex::Regex;
use std::sync::LazyLock;

// https://www.nhs.uk/conditions/poisoning/
// https://poisonhelp.hrsa.gov/faq/first-steps-poisoning-emergency
const REPLIES: &[&str] = &[
    "Call 999 now if you're in the UK. In the US, call Poison Help at 1-800-222-1222 right away, or 911 if the person collapses, has a seizure, has trouble breathing, or cannot be woken. Elsewhere, call your local emergency number. Keep the packet or bottle and tell the responder what was taken and when. Do not make anyone vomit or wait for symptoms.",
    "Get help now, even if there are no symptoms. Call 999 in the UK. In the US, call Poison Help at 1-800-222-1222, or 911 for collapse, seizure, trouble breathing, or if the person cannot be woken. Elsewhere, use your local emergency number. Keep the medicine packaging and do not make the person vomit.",
];

const WHILE_WAITING: &str = "Follow the call handler's instructions. Keep the medicine packaging and tell the crew what was taken and when. Stay with the person and keep checking their breathing. Do not make them vomit or give them anything to eat or drink.";

static EXPLICIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
    r"\b(?:i|i've|ive|he|she|they|we|my \w+|our \w+)\s+(?:just\s+|has\s+|have\s+)?(?:took|taken|swallowed|ate|drank)\b[^.?!]{{0,12}}\b(?:(?:whole|full|entire)\s+(?:packet|pack|box|bottle|blister)\s+of\s+{MEDICINE}|(?:too many|too much|\d{{2,3}})\s+{MEDICINE})\b|\b(?:i|i've|ive|he|she|they|we)\s+(?:took|have taken|had|has had)\s+(?:an?|the)\s+overdose\b"
)).expect("valid statements of medicine overdose")
});
static CHILD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:baby|toddler|kid|child|son|daughter|boy|girl)\b")
        .expect("valid child subject")
});
static PET_DOSE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"\b{ANIMAL}\b[^.?!]{{0,22}}\b(?:swallowed|took|ate|drank)\b"
    ))
    .expect("valid animal medicine exposure")
});
static FOLLOWUP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"^\s*(?:(?:yes|no|okay|ok|and|but)\s*[, ]\s*)*(?:(?:is|will) (?:he|she|my child|they|i) (?:going to be )?(?:be )?(?:ok|okay|fine|all right|alright|safe)|what (?:do i do (?:now|next)|now|next)|can (?:i|they|he|she) (?:drink|eat|sleep)(?: now)?|(?:i|we) (?:have |just )?(?:called|rang|phoned)(?: them| 999| 911| the ambulance)?|they(?:'re| are) on (?:their|the) way)\W*$"
).expect("valid medicine exposure follow-up")
});
static CALLED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:called|rang|phoned|on (?:their|the) way)\b")
        .expect("valid emergency contact confirmation")
});

pub(super) fn explicit_dose(reading: &Reading<'_>) -> bool {
    let line = reading.raw.to_lowercase();
    EXPLICIT.is_match(&line) && !PET_DOSE.is_match(&line)
}

pub(super) fn is_child_dose(reading: &Reading<'_>) -> bool {
    CHILD.is_match(&reading.clean) && super::a_count_of_packets::is_dose(reading.raw)
}

pub(super) fn emergency(_: &Reading<'_>, memory: &Memory) -> String {
    compose::pick(REPLIES, memory, "").to_owned()
}

pub(super) fn initial(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    (explicit_dose(reading) || is_child_dose(reading)).then(|| emergency(reading, memory))
}

pub(super) fn followup(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.raw.to_lowercase();
    let turn = memory.turns.last()?;
    let previous = memory::unframed_reply(&turn.bot).unwrap_or(&turn.bot);
    if !matches!(turn.module, "a_count_of_packets" | "safety")
        || !(REPLIES.contains(&previous) || previous == WHILE_WAITING)
        || !FOLLOWUP.is_match(&line)
    {
        return None;
    }
    if CALLED.is_match(&line) {
        return Some(WHILE_WAITING.to_owned());
    }
    Some(emergency(reading, memory))
}
