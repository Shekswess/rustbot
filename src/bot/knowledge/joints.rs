//! Joint guards and reply branches from `bot/knowledge/joints.py`.

use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use crate::bot::Reading;

use super::{
    JOINT_LOCK_PARTS, JOINT_LOCK_REPLY, JOINT_PART_NAMES, JOINT_PARTS, JOINT_REPLACING_PATTERN,
    JOINT_REPLACING_WORDS, JOINT_ROAD_WORDS, JOINT_SHOULDER_WORDS, JOINT_TRIGGER_FINGER_REPLY,
};

static REPLACING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(JOINT_REPLACING_PATTERN).expect("valid replacing pattern"));

pub(super) fn replacement(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    if words
        .iter()
        .any(|word| JOINT_ROAD_WORDS.contains(&word.as_str()))
        && !words.iter().any(|word| {
            JOINT_PARTS.contains(&word.as_str())
                || [
                    "hip",
                    "knee",
                    "shoulder",
                    "surgeon",
                    "operation",
                    "physio",
                    "hospital",
                    "nurse",
                    "ward",
                    "recovery",
                    "physiotherapy",
                ]
                .contains(&word.as_str())
        })
    {
        return false;
    }
    let mut replacements = REPLACING.captures_iter(&reading.line.clean).peekable();
    replacements.peek().is_none()
        || replacements.any(|part| JOINT_REPLACING_WORDS.contains(&&part["thing"]))
}

pub(super) fn joint_reply(
    reading: &Reading<'_>,
    other: &'static str,
    shoulder: &'static str,
) -> &'static str {
    if reading
        .line
        .words
        .iter()
        .any(|word| JOINT_SHOULDER_WORDS.contains(&word.as_str()))
    {
        shoulder
    } else {
        other
    }
}

fn named_part(reading: &Reading<'_>, candidates: &'static [&'static str]) -> &'static str {
    let words = &reading.line.words;
    let part = candidates
        .iter()
        .copied()
        .find(|candidate| words.iter().any(|word| word == candidate))
        .unwrap_or("joint");
    JOINT_PART_NAMES
        .iter()
        .find_map(|&(plural, single)| (plural == part).then_some(single))
        .unwrap_or(part)
}

pub(super) fn noisy(reading: &Reading<'_>) -> String {
    let part = named_part(reading, JOINT_PARTS);
    let article = if part.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "An"
    } else {
        "A"
    };
    format!(
        "{article} {part} that clicks or pops and does not hurt is almost always harmless: a tendon flicking over a bump, or gas coming out of the joint fluid, and it needs nothing doing. Keep the muscles around it working, because clicking that turns into pain is usually a strength problem rather than the noise. If it starts to hurt, swell or lock, have a physio or your GP look at it."
    )
}

pub(super) fn locked(reading: &Reading<'_>) -> Cow<'static, str> {
    let part = named_part(reading, JOINT_LOCK_PARTS);
    if matches!(part, "finger" | "fingers" | "thumb" | "thumbs") {
        Cow::Borrowed(JOINT_TRIGGER_FINGER_REPLY)
    } else {
        Cow::Owned(JOINT_LOCK_REPLY.replace("{part}", part))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_guard_and_joint_specific_reply() {
        assert!(!replacement(&Reading::new(
            "theyre resurfacing our road next week"
        )));
        assert!(!replacement(&Reading::new(
            "is it worth replacing a bath with a shower"
        )));
        assert!(replacement(&Reading::new(
            "i replaced the dressing on my knee"
        )));
        let reading = Reading::new("how long does a shoulder replacement last");
        assert_eq!(joint_reply(&reading, "hip or knee", "shoulder"), "shoulder");
    }

    #[test]
    fn noisy_and_locked_joint_replies_use_the_named_part() {
        assert!(noisy(&Reading::new("my ankles keep clicking")).starts_with("An ankle"));
        assert!(locked(&Reading::new("my knees keep locking")).starts_with("A knee"));
        assert_eq!(
            locked(&Reading::new("my fingers keep locking")),
            JOINT_TRIGGER_FINGER_REPLY
        );
    }
}
