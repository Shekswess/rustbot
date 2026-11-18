//! Native guards and renderers for the pinned body care cards.

use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use crate::bot::{Reading, understanding::topics::background_words};

#[path = "body_care_data.rs"]
mod data;

use data::*;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(BREAK, WORK_BREAK);
pattern!(HEAL, HEALING_WORD);
pattern!(MEAT, MEAT_JOINT);
pattern!(THING, COMPRESS_THING);
pattern!(PROTECT_WORD, r"[a-z']+");

const SLOW_BONES: &[&str] = &[
    "leg", "legs", "shin", "tibia", "fibula", "femur", "ankle", "ankles", "knee", "hip", "foot",
    "feet",
];

pub(super) fn face_shaving(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words = reading
        .line
        .words
        .iter()
        .map(String::as_str)
        .chain(background_words(&reading.line.norm));
    let has = |options: &[&str]| words.clone().any(|word| options.contains(&word));
    let reply = if has(BLADE_WORDS) {
        RAZOR_BLADE_REPLY
    } else if has(SHAVE_BURN_WORDS) {
        RAZOR_BURN_REPLY
    } else if has(BEARD_TRIM_WORDS) {
        BEARD_TRIM_REPLY
    } else {
        STRAIGHT_SHAVE_REPLY
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn not_work_break(reading: &Reading<'_>) -> bool {
    let said = reading.raw.to_lowercase();
    !MEAT.is_match(&said)
        && (reading
            .line
            .words
            .iter()
            .any(|word| BONE_PARTS.contains(&word.as_str()))
            || HEAL.is_match(&said)
            || !BREAK.is_match(&said))
}

pub(super) fn bone_healing(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = if reading
        .line
        .words
        .iter()
        .any(|word| SLOW_BONES.contains(&word.as_str()))
    {
        BONE_SLOW_REPLY
    } else {
        BONE_ORDINARY_REPLY
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn is_body_compress(reading: &Reading<'_>) -> bool {
    let said = if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    };
    let said = said.to_lowercase();
    let line = format!(
        " {} ",
        said.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    let mentions = |words: &[&str]| words.iter().any(|word| line.contains(&format!(" {word} ")));
    !mentions(NOT_ON_A_BODY)
        && !THING.captures_iter(&line).any(|matched| {
            let noun = &matched[1];
            NON_BODY_THINGS.contains(&noun) && !ACHE_WORDS.contains(&noun)
        })
        && (mentions(ON_A_BODY) || mentions(ACHE_WORDS))
}

pub(super) fn only_asks_to_protect_part(reading: &Reading<'_>) -> bool {
    let said = if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    };
    let said = said.to_lowercase().replace('’', "'");
    let allowed = |word| PROTECT_ALLOWED.binary_search(&word).is_ok();
    PROTECT_WORD.find_iter(&said).all(|matched| {
        let word = matched.as_str();
        allowed(word)
            || ["ing", "es", "s", "ed", "d"].into_iter().any(|ending| {
                word.strip_suffix(ending)
                    .is_some_and(|stem| stem.len() >= 3 && allowed(stem))
            })
    })
}

pub(super) fn protect_part(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let part = super::body::part_named_with_background(reading).unwrap_or("joint");
    let article = if part.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "An"
    } else {
        "A"
    };
    let drill = PROTECT_DRILLS
        .iter()
        .find_map(|(name, drill)| (*name == part).then_some(*drill))
        .unwrap_or(
            "two short sessions a week of the movement you want to keep, stopping short of pain",
        );
    let movement = if WEIGHT_BEARING.contains(&part) {
        "Swimming, cycling and a flat walk are the kind ones on the other days. Warm up before a big job, keep the load close to you when you lift, and change position every half hour rather than holding one."
    } else {
        "Keep it moving every day rather than resting it, and keep it in the middle of where it moves - not locked straight, not at the end of its range."
    };
    Some(Cow::Owned(format!(
        "{article} {part} is protected by the muscle round it and by how you use it more than by anything you wear or avoid: {drill}. {movement}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::understanding::context::History;

    #[test]
    fn shaving_uses_the_current_line_and_prefers_blade_then_burn_then_trim() {
        let contextual = Reading::new("it started when i was at Blade, how do i shave my face");
        assert!(background_words(&contextual.line.norm).any(|word| word == "blade"));
        for (input, expected) in [
            (
                "razor blade pulls on my beard and leaves a burn",
                RAZOR_BLADE_REPLY,
            ),
            ("how do i stop my beard going red", RAZOR_BURN_REPLY),
            ("how do i trim my neckline", BEARD_TRIM_REPLY),
            ("how do i shave my face", STRAIGHT_SHAVE_REPLY),
        ] {
            assert_eq!(
                face_shaving(&Reading::new(input)).as_deref(),
                Some(expected)
            );
        }
        assert_eq!(
            face_shaving(&contextual).as_deref(),
            Some(RAZOR_BLADE_REPLY)
        );

        let prior = "i need to replace the lawn mower blade";
        let mut history = History::default();
        history.retain(&mut Reading::new(prior));
        let unrelated = Reading::in_context(
            "how do i shave my face",
            [prior].into_iter(),
            None,
            &history,
        );
        assert_eq!(
            face_shaving(&unrelated).as_deref(),
            Some(STRAIGHT_SHAVE_REPLY)
        );
    }

    #[test]
    fn work_breaks_and_food_decline_before_bone_healing() {
        for (input, guard, expected) in [
            ("how long is my tea break", false, BONE_ORDINARY_REPLY),
            (
                "how long before a break between shifts",
                false,
                BONE_ORDINARY_REPLY,
            ),
            (
                "how long should i rest a leg of lamb",
                false,
                BONE_SLOW_REPLY,
            ),
            (
                "how long will my break take to heal",
                true,
                BONE_ORDINARY_REPLY,
            ),
            (
                "how long will my wrist heal if i take my break at work",
                true,
                BONE_ORDINARY_REPLY,
            ),
            (
                "how long does a broken ankle take to heal",
                true,
                BONE_SLOW_REPLY,
            ),
        ] {
            let reading = Reading::new(input);
            assert_eq!(not_work_break(&reading), guard, "{input}");
            assert_eq!(bone_healing(&reading).as_deref(), Some(expected), "{input}");
        }
    }

    #[test]
    fn body_compresses_require_an_ache_or_compress_phrase_and_exclude_other_uses() {
        for (input, expected) in [
            ("is heat better than ice", true),
            ("should i ICE my wrist", true),
            ("how long should i hold an ice pack on", true),
            ("how long do frozen peas take to cook", false),
            ("how long to ice the cake", false),
            (
                "why is the boiler so slow to heat the house even when my back hurts",
                false,
            ),
            ("should i pull away in second gear on ice", false),
            ("how long does it take to get hot", false),
        ] {
            assert_eq!(is_body_compress(&Reading::new(input)), expected, "{input}");
        }
    }

    #[test]
    fn prevention_requires_only_the_part_and_request() {
        for (input, expected, part, tailored) in [
            (
                "how do i protect my wrists",
                true,
                "A wrist",
                "palm up and palm down",
            ),
            (
                "how do i protect my ankles",
                true,
                "An ankle",
                "heel raises holding a chair",
            ),
            (
                "how do i protect my wrists at a desk",
                false,
                "A wrist",
                "palm up and palm down",
            ),
            (
                "how do i protect my knees when running",
                false,
                "A knee",
                "sit-to-stand",
            ),
            (
                "how do i look after my wrists and shoulders",
                true,
                "A shoulder",
                "slow wall push-up",
            ),
            (
                "how can i protect my collarbone",
                true,
                "A collarbone",
                "stopping short of pain",
            ),
        ] {
            let reading = Reading::new(input);
            let reply = protect_part(&reading).unwrap();
            assert_eq!(only_asks_to_protect_part(&reading), expected, "{input}");
            assert!(
                reply.starts_with(part) && reply.contains(tailored),
                "{input}: {reply}"
            );
        }
    }
}
