//! Priority replies for gas, falls and serious childhood fever symptoms.
use crate::bot::{Memory, Reading, compose};
use regex::RegexSet;
use std::sync::LazyLock;

#[path = "child_fever.rs"]
mod child_fever;
pub(in crate::bot) use child_fever::{breathing_or_colour_emergency, fever_red_flags};
#[path = "medical_safety.rs"]
mod medical_safety;
pub(in crate::bot) use medical_safety::{
    Urgency, applies as has_medical_urgency, current as current_medical_urgency,
    has_breathlessness, selected as selected_medical_urgency,
};
static GAS_SMELL: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
    r#"\b(?:smell|smells|smelling|smelled|smelt|whiff)\s+(?:of\s+|like\s+)?(?:a\s+)?(?:gas|gas\s+leak)\b"#,
    r#"\b(?:can|could)\s+(?:i|you|we|she|he|they)\s+smell\s+gas\b"#,
    r#"\bi\s+(?:can\s+)?smell\s+gas\b"#,
    r#"\b(?:strong|funny|odd|strange|faint)\s+(?:smell|whiff)\s+of\s+gas\b"#,
    r#"\b(?:gas|leak)\s+smell\b"#,
    r#"\bgas\s+(?:leak|leaking|leaked)\b"#,
    r#"\bleak(?:ing|s|ed)?\s+(?:out\s+)?(?:\w+\s+)?gas\b"#,
    r#"\bgas\s+(?:is\s+)?(?:leaking|escaping|coming\s+(?:out|from)|seeping)\b"#,
    r#"\b(?:smell\w*|whiff\w*)\b[^.?!]{0,30}\bgas\b"#,
    r#"\bgas\b[^.?!]{0,20}\b(?:smell|stink|reek)\w*\b"#,
    r#"\b(?:boiler|cooker|hob|heater|oven|fire|gas\s+fire)\b.{0,30}\bsmell\w*\b.{0,30}\b(?:gas|fumes?|rotten|sulphur|sulfur|eggs?|headach\w*|dizzy|nauseous)\b"#,
    r#"\bsmell\w*\s+(?:of\s+|like\s+)?(?:rotten\s+eggs?|sulphur|sulfur)\b.{0,30}\b(?:boiler|cooker|hob|heater|gas)\b"#,
]).unwrap()
});
static GAS_ON_YOU: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
    r#"\b(?:my|his|her|their|your|the)\s+(?:hands?|clothes|shirt|coat|jumper|fingers|gloves|cloths?)\b[^.?!]{0,25}\b(?:smell|smells|smelling|smelt|reek|reeks|stink|stinks)\b"#,
    r#"\b(?:smell|smells|smelling|smelt)\s+(?:of\s+|like\s+)?gas\b[^.?!]{0,20}\b(?:on|from)\s+(?:my|his|her|their|your)\s+(?:hands?|clothes|shirt|coat|jumper|fingers|gloves)\b"#,
]).unwrap()
});
static GAS_PAST: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
    r#"\b(?:smelt|smelled|smelling|smell)\b[^.?!]{0,40}\b(?:last\s+(?:year|week|month|winter|summer)|years?\s+ago|months?\s+ago|weeks?\s+ago|yesterday|ages\s+ago|once|when\s+i\s+was|as\s+a\s+kid|at\s+(?:my|her|his|our)\s+\w+'s)\b"#,
    r#"\b(?:last\s+(?:year|week|month)|years?\s+ago|when\s+i\s+was|ages\s+ago)\b[^.?!]{0,30}\b(?:smelt|smelled|smell)\b"#,
]).unwrap()
});
static GAS_IN_ROOM: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
    r#"\b(?:kitchen|boiler|oven|cooker|hob|flat|house|home|room|garage|meter|pipe|pipes|heater|fire|cooker|cylinder|hall|hallway|bathroom|cellar|basement|camper|caravan|shop|workshop)\b"#,
]).unwrap()
});
static GAS_NOW: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
    r#"\b(?:still|now|right\s+now|just\s+now|this\s+(?:morning|afternoon|evening)|tonight|today|again|earlier\s+today)\b"#,
    r#"\b(?:can|could)\s+(?:i|you|we|she|he|they)\s+smell\b"#,
]).unwrap()
});
const REPLIES: &[&str] = &[
    "Don't switch anything on or off - no lights, no flames - and open the doors and windows on your way out. Turn the gas off at the meter if it's easy to reach, then ring the gas emergency line from outside: 0800 111 999 in the UK, or 911.",
    "Get everyone away from it: put out any flames, don't touch light switches, open the doors and windows, and turn the gas off at the meter if you can reach it. Ring 0800 111 999 (UK) or 911 from outside, and 999 straight away if anyone feels ill.",
];

pub(crate) fn current_gas(line: &str) -> bool {
    GAS_SMELL.is_match(line)
        && (!GAS_ON_YOU.is_match(line) || GAS_IN_ROOM.is_match(line))
        && (!GAS_PAST.is_match(line) || GAS_NOW.is_match(line))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = &reading.clean;
    if super::the_detail_the_thread_adds::heavy::only_a_dream(line) {
        return None;
    }
    if !current_gas(line) {
        return super::fall::reply(reading, memory)
            .or_else(|| super::gas_followup::reply(reading, memory))
            .or_else(|| child_fever::respond(reading, memory))
            .or_else(|| super::poison::initial(reading, memory))
            .or_else(|| super::poison::followup(reading, memory))
            .or_else(|| medical_safety::reply(reading, memory));
    }
    Some(compose::pick(REPLIES, memory, "").to_owned())
}

pub(crate) fn fall_continuation(clean: &str, memory: &Memory) -> bool {
    super::fall::continuing_sign(clean, memory)
}

pub(super) fn is_initial_gas_reply(said: &str) -> bool {
    REPLIES.contains(&said)
}
