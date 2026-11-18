use super::guards::Context;
use crate::bot::modules::safety::{current_medical_urgency, has_breathlessness};
use regex::RegexSet;
use std::sync::LazyLock;

const LOW_MORNING: &[&str] = &[
    "cant get out of bed",
    "can't get out of bed",
    "cannot get out of bed",
    "cant get up in the morning",
    "can't get up in the morning",
    "dont want to get up",
    "don't want to get up",
    "cant face the day",
    "can't face the day",
    "no point getting up",
    "no point in getting up",
    "no point at all",
    "hopeless",
    "worthless",
    "hate mornings",
    "cant get up",
    "can't get up",
    "struggling in the mornings",
    "struggling to get up",
    "not sleeping and not",
    "no energy for anything",
];

const NIGHT_SYMPTOM: &[&str] = &[
    "pain",
    "painful",
    "ache",
    "aches",
    "aching",
    "achy",
    "hurt",
    "hurts",
    "hurting",
    "cramp",
    "cramps",
    "cough",
    "coughing",
    "tooth",
    "toothache",
    "headache",
    "headaches",
    "migraine",
    "heartburn",
    "reflux",
    "nausea",
    "nauseous",
    "fever",
    "temperature",
    "itch",
    "itching",
    "sore",
    "swollen",
    "stiff",
    "restless legs",
    "sick",
    "throwing up",
    "period pain",
];

static EARLY_HOUR: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"\b(?:at|by|around|about)\s*(?:5|five|6|six)\b",
        r"\b(?:5|five|6|six)\s*(?:am|a\.m\.|o'?clock)\b",
    ])
    .expect("valid early-hour patterns")
});

static STILL_ASLEEP: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        r"\bback\s+(?:off\s+)?to\s+sleep\b",
        r"\bstay(?:ing)?\s+asleep\b",
        r"\bsleep(?:ing)?\s+through\b",
        r"\bwake\s+(?:up\s+)?(?:several|a\s+few|couple|twice|three)\s+times\b",
    ])
    .expect("valid staying-asleep patterns")
});

pub(super) fn allows(context: &Context<'_>) -> bool {
    let reading = context.reading;
    if reading
        .topics
        .iter()
        .any(|(topic, _)| matches!(*topic, "health" | "feelings"))
    {
        return false;
    }
    let line = &reading.line;
    if has_breathlessness(&line.clean) || current_medical_urgency(&line.clean).is_some() {
        return false;
    }
    // The source reads clean and plain text plus the norm list, whose tokens
    // are separated by the list's punctuation.
    let mut views = [&line.clean, &line.plain].into_iter().chain(&line.norm);
    if views
        .clone()
        .any(|text| NIGHT_SYMPTOM.iter().any(|phrase| text.contains(phrase)))
        || views.clone().any(|text| EARLY_HOUR.is_match(text))
            && !views.any(|text| STILL_ASLEEP.is_match(text))
    {
        return false;
    }
    let earlier = context
        .earlier
        .iter()
        .filter(|turn| !turn.user.trim().is_empty())
        .map(|turn| turn.user.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    !LOW_MORNING.iter().any(|phrase| earlier.contains(phrase))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::knowledge::lookup::{by_id, lookup};
    use crate::bot::{Bot, Exchange, Reading};

    #[test]
    fn waking_card_preserves_the_frequency_answer_anchor() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let opener = bot.turn("i keep waking up at 3am");
        assert_eq!(opener.module, Some("answers"));
        assert_eq!(
            opener.reply,
            by_id("science", "wake-at-night").unwrap().reply
        );

        let every_night = bot.turn("its every night");
        assert_eq!(every_night.module, Some("chat"));
        assert_eq!(
            every_night.reply,
            "Every night is a lot. How long has it been like that?"
        );

        let followup = bot.turn("what do i do now?");
        assert_eq!(followup.module, Some("answers"));
        assert_eq!(
            followup.reply,
            "Same as before: Waking in the night is normal - sleep runs in cycles of about ninety minutes - and the common patch is the small hours, around three."
        );
    }

    #[test]
    fn distinguish_early_waking_symptoms_and_low_morning_history() {
        let early = Reading::new("i keep waking up at 5");
        let staying_asleep = Reading::new("i wake at 5 and cant get back to sleep");
        assert!(!allows(&Context::new(&early, &[])));
        assert!(allows(&Context::new(&staying_asleep, &[])));

        for raw in [
            "i wake up every night with a headache",
            "i wake up at 3am because i feel hopeless",
            "my baby keeps waking up at 3am",
            "i keep waking up at 5am and cant get back to sleep",
        ] {
            assert!(
                lookup(raw).is_none_or(|card| card.id != "wake-at-night"),
                "{raw}"
            );
        }

        let overlap = lookup("i keep waking up in the night").unwrap();
        assert_eq!(overlap.id, "wake-at-night");

        let waking = Reading::new("i keep waking up at 3am");
        let earlier = [Exchange {
            user: "I CAN'T get out of bed most mornings".into(),
            bot: String::new(),
            module: "greeting",
        }];
        assert!(!allows(&Context::new(&waking, &earlier)));
    }

    #[test]
    fn waking_with_current_breathing_difficulty_uses_the_safety_route() {
        let now = "i keep waking up at 3am gasping for breath";
        assert!(lookup(now).is_none_or(|card| card.id != "wake-at-night"));
        assert!(
            lookup("i keep waking up at 3am out of breath")
                .is_none_or(|card| card.id != "wake-at-night")
        );
        let denied = Reading::new("i wake up at 3am and am not gasping for breath");
        assert!(allows(&Context::new(&denied, &[])));
        let mut bot = Bot::new(Some("0")).unwrap();
        for line in [now, "its every night"] {
            let turn = bot.turn(line);
            assert_eq!(turn.module, Some("safety"));
            assert!(turn.reply.starts_with("Call 999 now"));
            assert!(
                !turn
                    .candidates
                    .iter()
                    .any(|candidate| candidate.module == "answers")
            );
        }
    }
}
