//! Router, desktop, app and vacuum advice from PyBot's technology domain.
#[cfg(test)]
#[path = "technology_setup_tests.rs"]
mod tests;

use super::guards::Context;
use crate::bot::{
    Reading,
    understanding::{
        context,
        kinds::{Intent, Kind},
        line,
    },
};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

pub(super) fn router_place(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let has = |key: &str| {
        reading
            .line
            .words
            .iter()
            .any(|word| word.eq_ignore_ascii_case(key))
    };
    let response = if !has("where") && (has("move") || has("moving")) {
        ROUTER_MOVE
    } else {
        ROUTER_PLACE
    };
    Some(Cow::Borrowed(response))
}

pub(super) fn wifi_context(context: &Context<'_>) -> bool {
    let reading = context.reading;
    if WIFI_WORD.is_match(&reading.line.words.join(" ")) {
        return true;
    }
    if reading.line.change_of_subject
        || context::shifts(&reading.line)
        || (!reading.reference.followup && reading.own_topic.is_some())
        || matches!(
            reading.features.intent,
            Intent::Empty
                | Intent::Greeting
                | Intent::Farewell
                | Intent::Thanks
                | Intent::Ack
                | Intent::Smalltalk
                | Intent::Laugh
                | Intent::AboutBot
        )
    {
        return false;
    }
    context
        .memory
        .and_then(|memory| memory.ack_topics.stored_topic(context.earlier).next())
        .is_some_and(|text| WIFI_WORD.is_match(&line::normalise(text).norm.join(" ")))
}

pub(super) fn asks_to_remove(words: &[String]) -> bool {
    words.iter().any(|word| word == "off") || words.windows(2).any(|pair| pair == ["out", "of"])
}

pub(super) fn router_admin_password(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(if reading.kind == Kind::Question {
        WIFI_RESET
    } else {
        WIFI_LOCKED_SHORT
    }))
}

pub(super) fn vacuum_filter(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let reading = context.reading;
    if VACUUM_NAMES
        .iter()
        .any(|name| reading.line.clean.contains(name))
    {
        return Some(Cow::Borrowed(VACUUM_FILTER));
    }
    // PyBot checks retained topic records, which can outlive several exchanges.
    // Keep the conflicting machine while the native topic store is incomplete.
    (!another_filter_machine(&reading.line.clean)
        && !context
            .earlier
            .iter()
            .any(|turn| another_filter_machine(&turn.user)))
    .then_some(Cow::Borrowed(VACUUM_FILTER))
}

fn another_filter_machine(text: &str) -> bool {
    let said = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    FILTER_MACHINES.iter().any(|name| said.contains(name))
}

pub(super) fn app_not_paper(reading: &Reading<'_>) -> bool {
    let line = format!(
        "{} {} {} {}",
        reading.line.text, reading.line.norm_text, reading.line.clean, reading.raw
    );
    !PAPER_APPLICATION.is_match(&line) || ON_DEVICE.is_match(&line)
}

fn a_device_precedes(line: &str) -> bool {
    for word in line.split_whitespace().rev() {
        let word = word.trim_matches(|ch: char| ch.is_ascii_punctuation() || ch == '’');
        if A_DEVICE_THAT_SLEEPS.binary_search(&word).is_ok() {
            return true;
        }
        if A_PERSON_WHO_SLEEPS.binary_search(&word).is_ok() {
            return false;
        }
    }
    false
}

pub(super) fn not_a_sleepy_body(reading: &Reading<'_>) -> bool {
    let line = format!("{} {}", reading.line.text, reading.line.clean);
    let line = if line.trim().is_empty() {
        reading.line.words.join(" ").to_lowercase()
    } else {
        line.to_lowercase()
    };
    NOT_TIRED.is_match(&line)
        || SLEEP_SHORTAGE
            .find_iter(&line)
            .flatten()
            .all(|matched| a_device_precedes(&line[..matched.start()]))
}

static PAPER_APPLICATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(A_PAPER_APPLICATION).expect("source paper-application shape"));
static ON_DEVICE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(ON_A_DEVICE).expect("source device shape"));
static SLEEP_SHORTAGE: LazyLock<Fancy> =
    LazyLock::new(|| Fancy::new(SHORT_OF_SLEEP).expect("source sleep shapes"));
static NOT_TIRED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(NOT_TIRED_AFTER_ALL).expect("source eye-irritation shape"));
static WIFI_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(wifi|wi fi|wireless|router|broadband|internet|signal|reception|coverage|network|mesh|extender|repeater|hotspot|ethernet|dead spot|stream)\w*")
        .expect("source wifi vocabulary")
});

// Authored text, patterns and vocabulary from PyBot 7d63c7f.
const ROUTER_PLACE: &str = "Up high, out in the open and as close to the middle of the flat as the cable allows - on a shelf or the top of a bookcase, not on the floor, in a cupboard, behind the TV or beside the microwave, a cordless phone or the hot water tank. Radio waves spread out sideways and downwards from the box, so one in a corner mostly covers the corner. The line comes in where it comes in: if that is one end, keep the router there and add a mesh unit at the other end rather than running a long cable.";

const ROUTER_MOVE: &str = "Yes, if it is in a cupboard, behind the TV, on the floor or at the far end of the flat, and it is worth the five minutes. Up high, out in the open and as close to the middle of the flat as the cable allows - on a shelf or the top of a bookcase, not on the floor, in a cupboard, behind the TV or beside the microwave, a cordless phone or the hot water tank. Radio waves spread out sideways and downwards from the box, so one in a corner mostly covers the corner. The line comes in where it comes in: if that is one end, keep the router there and add a mesh unit at the other end rather than running a long cable.";

const WIFI_RESET: &str = "If the password on the sticker does not get you into the router, someone changed it: the way back is the reset hole - with the router on, push a paperclip into the hole marked Reset for 10 to 30 seconds until the lights blink, and it goes back to the sticker's password. That also wipes the wifi name and password you set, so every device needs joining again. Change the admin password on the way back in and write it on the sticker.";

const WIFI_LOCKED_SHORT: &str = "Then it is the reset hole: with the router on, push a paperclip into the hole marked Reset for 10 to 30 seconds and it comes back with the password on the sticker. That wipes the wifi name and password too, so every device needs joining again.";

const VACUUM_NAMES: &[&str] = &[
    "hoover", "vacuum", "henry", "dyson", "vax", "shark", "miele", "numatic",
];

const FILTER_MACHINES: &[&str] = &[
    "washing machine",
    "washer",
    "washing mashine",
    "dishwasher",
    "dish washer",
    "tumble dryer",
    "tumble drier",
    "dryer",
    "extractor",
    "cooker hood",
    "hood",
    "fridge",
    "freezer",
    "coffee machine",
    "water filter",
    "kettle",
    "shower",
    "tap",
    "pond",
    "aquarium",
    "fish tank",
    "car",
    "caravan",
];

const VACUUM_FILTER: &str = "Wash or shake out the filter every month or two, and change it once a year - a blocked filter is the usual reason a vacuum starts to smell and feels weak. Most are washable: rinse under the tap, then leave them to dry for a full 24 hours before they go back in, because a damp filter ruins the motor.";

const A_PAPER_APPLICATION: &str = r#"(?i)\b(?:job|course|college|uni|university|school|visa|grant|funding|rental|tenancy|membership|passport|licen[cs]e|travel)\s+applications?\b|\bapplications?\s+(?:form|forms|essay|letter|deadline|question)\b"#;

const ON_A_DEVICE: &str = r#"(?i)\b(?:phone|iphone|android|app|apps|tablet|ipad|laptop|computer|screen|browser|website|site|download|open|opens|opening)\b"#;

const SHORT_OF_SLEEP: &str = r#"(?i)\b(?:keep|keeps|keeping|kept)\s+(?:(?:nodding|dozing)\s*(?:off|asleep)?|(?:falling|dropping)\s+(?:off|asleep))\b|\bnodd(?:ing|ed|s)?\s+off\b|\bdoz(?:e|es|ed|ing)\s+off\b|\beyes?\s+(?:keep|keeps|are|were)?\s*(?:closing|dropping|shutting)\b|\b(?:keep|keeps|keeping|kept)\s+(?:my|your|his|her|their|our|both)?\s*eyes?\s+open\b(?!\s+for\b)|\bf(?:all|ell)(?:ing|s)?\s+asleep\s+(?:at|in|on|during|over)\b|\bcan\s*(?:'?t|cant|not)?\s*(?:barely|hardly)\s+(?:keep|stay)\s+(?:(?:my|your|his|her|their|our|both)\s+)?(?:eyes?\s+open|awake|alert)\b"#;

const NOT_TIRED_AFTER_ALL: &str = r#"(?i)\b(?:hay ?fever|allerg(?:y|ies|ic)|pollen|conjunctivitis|stye|sty|glued|swollen|puffy|watering|streaming)\b"#;

const A_DEVICE_THAT_SLEEPS: &[&str] = &[
    "android",
    "camera",
    "cameras",
    "computer",
    "computers",
    "console",
    "consoles",
    "fitbit",
    "ipad",
    "iphone",
    "kindle",
    "laptop",
    "laptops",
    "mac",
    "macbook",
    "macs",
    "monitor",
    "monitors",
    "pc",
    "pcs",
    "phone",
    "phones",
    "playstation",
    "printer",
    "printers",
    "screen",
    "screens",
    "tablet",
    "tablets",
    "television",
    "televisions",
    "tv",
    "watch",
    "xbox",
];

const A_PERSON_WHO_SLEEPS: &[&str] = &[
    "baby",
    "body",
    "boy",
    "boys",
    "child",
    "children",
    "dad",
    "daughter",
    "eye",
    "eyes",
    "girl",
    "girls",
    "he",
    "head",
    "her",
    "herself",
    "him",
    "himself",
    "i",
    "i'm",
    "im",
    "kid",
    "kids",
    "lad",
    "lass",
    "me",
    "mum",
    "myself",
    "nan",
    "ourselves",
    "she",
    "son",
    "teenager",
    "them",
    "themselves",
    "they",
    "toddler",
    "us",
    "we",
    "you",
];
