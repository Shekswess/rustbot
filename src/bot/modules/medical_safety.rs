//! Route current chest pain, severe breathing difficulty, and signs of stroke to urgent help.

use crate::bot::knowledge::lookup::PET_BREATHING_EMERGENCY;
use crate::bot::{Memory, Reading, memory};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $value:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::bot) enum Urgency {
    Emergency,
    ChestAssessment,
    NightBreathing,
    PetBreathing,
}

const EMERGENCY: &str = "Call 999 now in the UK, or your local emergency number elsewhere. Tell the call handler about the symptoms and follow their instructions. Do not drive yourself.";
const CHEST_ASSESSMENT: &str = "Call NHS 111 now for advice about the chest pain. Call 999 immediately if the pain is sudden and does not go away, spreads, or comes with shortness of breath, sweating, nausea or light-headedness.";
const NIGHT_BREATHING: &str = "Call NHS 111 today and tell them about the breathing symptoms that woke you. If you are struggling to breathe now, call 999 immediately.";

pattern!(
    CHEST,
    r"\b(?:chest (?:pain|discomfort|pressure|tightness|feels? (?:tight|heavy)|hurts?)|(?:pain|pressure|tightness|squeezing|crushing) (?:in|across) (?:my|his|her|their|the) chest)\b"
);
pattern!(
    DANGEROUS_CHEST,
    r"\b(?:chest (?:pressure|tightness|feels? (?:tight|heavy))|(?:crushing|squeezing|tight|heavy|severe) chest (?:pain|discomfort|pressure)|(?:crushing|squeezing|tight|heavy) (?:pain|pressure) (?:in|across) (?:my|his|her|their|the) chest|(?:wont|won't|will not|doesnt|doesn't|does not|not) (?:go away|ease|settle)|(?:spreads?|radiat\w*|moves?|travels?|goes) (?:down|into|to|up to) (?:my|his|her|their|the) (?:arm|arms|back|neck|jaw|stomach))\b"
);
pattern!(
    WITH_CHEST,
    r"\b(?:short of breath|shortness of breath|breathless(?:ness)?|sweaty|sweating|light[- ]headed|faint|dizzy|feel(?:s|ing)? sick|nause(?:a|ous))\b"
);
pattern!(
    BREATHING,
    r"\b(?:(?:cant|can't|cannot|unable to) (?:breathe|catch (?:my|his|her|their) breath)|struggl(?:e|es|ing) to breathe|severe (?:difficulty breathing|shortness of breath)|gasping for (?:air|breath)|not breathing|stopped breathing)\b"
);
pattern!(
    THROAT_ILLNESS,
    r"\b(?:tonsillitis|tonsils?|quinsy|sore throat|throat infection)\b"
);
pattern!(
    THROAT_EMERGENCY,
    concat!(
        r"\b(?:(?:have|has|having|developed|experiencing) (?:difficulty|trouble) breathing\b|",
        r"(?:it is|it'?s|find it|finds it) hard to breathe\b|(?:can'?t|cannot|unable to) breathe\b|",
        r"drooling\b|(?:can'?t|cannot|unable to) (?:even )?swallow",
        r"(?: (?:at all|anything|water|(?:my |his |her |their |your )?(?:own )?(?:spit|saliva)|",
        r"and|or|but|right now|any ?more)\b|\s*(?:[,.;!?]|$)))"
    )
);
pattern!(
    THROAT_CAUSAL_QUESTION,
    r"^\W*(?:(?:so|well|ok|okay)\s+)*(?:can|could|does|would|will) (?:having )?(?:a |an |my |his |her |their )?(?:tonsillitis|quinsy|sore throat|throat infection) (?:cause|lead to|make)\b"
);
static ANIMAL_THROAT: LazyLock<Regex> = LazyLock::new(|| {
    let animal = super::super::a_count_of_packets::ANIMAL;
    Regex::new(&format!(
        r"\b{animal} (?:has|have|got|with) (?:a )?(?:tonsillitis|quinsy|sore throat|throat infection)\b"
    ))
    .expect("animal with a throat illness")
});
static PET_BREATHING_SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
    let animal = super::super::a_count_of_packets::ANIMAL;
    Regex::new(&format!(
        r"^(?:my|our|the) (?:(?:old|little|young|rescue|elderly|adult) ){{0,2}}{animal}(?:'s| (?:is|are)(?: dying and)?)?$"
    ))
    .expect("pet with current breathing difficulty")
});
pattern!(
    WAKING_BREATHLESS,
    r"\b(?:keep|keeps|kept|always|still|been|often|repeatedly)\s+waking(?: up)?\s+(?:gasping for (?:air|breath)|short of breath|breathless)\b"
);
pattern!(
    WOKEN_SHORT_OF_BREATH,
    r"\b(?:wake|wakes|waking|woke)(?: up)?(?: at (?:[0-9]{1,2}(?::[0-9]{2})?(?: ?[ap]\.?m\.?)?|night)| in the (?:night|morning)| every night)? (?:out of breath|short of breath|breathless)\b"
);
pattern!(
    BREATHLESSNESS,
    r"\b(?:out of breath|short of breath|shortness of breath|breathless(?:ness)?|gasp(?:s|ing)? for (?:air|breath))\b"
);
pattern!(
    STROKE,
    r"\b(?:(?:one|1) side of (?:my|his|her|their|the) face (?:is )?(?:weak|drooping|numb)|(?:weakness|numbness|drooping) (?:on|in) (?:one|1|the left|the right) side of (?:my|his|her|their|the) face|(?:my|his|her|their) (?:face|mouth) (?:is )?(?:droop(?:ing|s|ed)?|gone (?:numb|droopy))|(?:sudden(?:ly)? (?:weakness|numbness)|(?:can'?t|cannot|unable to) (?:lift|raise)) (?:in )?(?:my|his|her|their|one) (?:(?:left|right) )?arm|(?:my|his|her|their) speech (?:is |started )?slur(?:red|ring|s)?)\b"
);
pattern!(
    PERSON,
    r"\b(?:i|im|i'm|ive|i've|we|he|she|they|hes|shes|theyre|he's|she's|they're|my|our|their|his|her|someone)\b"
);
pattern!(
    HYPOTHETICAL,
    r"(?i)^\s*(?:(?:and|but|so|well|hey|hi)\s+)*(?:what if|if|imagine|suppose|what (?:does|do|is|are|causes?)|why (?:does|do|is|are)|how (?:does|do) (?:you|people)|(?:can|could) (?:you|one) (?:get|have))\b"
);
pattern!(
    HISTORY,
    r"\b(?:last (?:week|month|year|winter|summer)|(?:weeks?|months?|years?) ago|used to|when i was (?:a kid|younger)|no longer|has gone away|went away|have been cleared|(?:doctor|clinician|gp|a&e|hospital) (?:says?|said|told me|confirmed) (?:i(?: am|'m|m)?|we are|it is|its|it's) (?:ok|okay|fine|nothing serious)|(?:doctor|clinician|gp) cleared (?:me|us|him|her|them))\b"
);
pattern!(
    ACTIVE_AGAIN,
    r"\b(?:is|it's|its|has|have) (?:back|started again)(?: now)?\b|\b(?:back now|still have|still having|still can(?:'t|t) breathe|happening now|again today)\b|\b(?:now|today) (?:i|we|he|she|they)(?: am| are| is| have| has| can't| cant| cannot)\b"
);
pattern!(
    DENY,
    r"\b(?:no|not|without|dont have|don't have|do not have|doesnt have|doesn't have|does not have|never had|denies)\b"
);
pattern!(
    BREAK,
    r"[,.;!?]|\b(?:but|although|though|however|instead|and now)\b"
);
pattern!(
    LEAD,
    r"^\W*(?:(?:and|but|so|oh|well|yeah|yep|ok|okay|right|alright|no)\W+)*"
);
pattern!(
    FREQUENCY,
    r"^(?:(?:it'?s|it is|that'?s|there'?s)\s+)?(?:(?:every|most)(?:\s+single)?\s+(?:nights?|mornings?|days?)|all\s+(?:the\s+)?time|nightly|daily|constantly|non ?stop)(?:\s+(?:now|though|at the moment|lately|these days|still))*\W*$"
);
pattern!(
    BARE_NEXT,
    r"^(?:what (?:do i do (?:now|next)|now|next)|(?:can|could|should) i (?:take (?:anything|something) for it|call (?:anyone|someone|999|111|the doctor)|wait)|how long (?:should i wait|do i wait))\W*$"
);

fn stated(pattern: &Regex, line: &str) -> bool {
    let bare = LEAD.replace(line, "");
    pattern.find_iter(line).any(|symptom| {
        let before = &line[..symptom.start()];
        let clause = BREAK.split(before).last().unwrap_or(before);
        !DENY.is_match(clause)
            && (PERSON.is_match(clause)
                || symptom.as_str().len()
                    == bare
                        .trim_end_matches(|ch: char| !ch.is_alphanumeric())
                        .len()
                || clause.trim().is_empty() && !line.ends_with('?'))
    })
}

fn affirmed(pattern: &Regex, line: &str) -> bool {
    pattern.find_iter(line).any(|symptom| {
        let before = &line[..symptom.start()];
        !DENY.is_match(BREAK.split(before).last().unwrap_or(before))
    })
}

pub(in crate::bot) fn has_breathlessness(line: &str) -> bool {
    (!HISTORY.is_match(line) || ACTIVE_AGAIN.is_match(line)) && affirmed(&BREATHLESSNESS, line)
}

// Inability to swallow or difficulty breathing during a throat illness needs
// emergency assessment. Pain on swallowing alone is a different symptom.
// https://www.nhs.uk/conditions/tonsillitis/
fn throat_emergency(line: &str) -> bool {
    if !(line.contains("swallow") || line.contains("breath") || line.contains("drool"))
        || !THROAT_ILLNESS.is_match(line)
        || ANIMAL_THROAT.is_match(line)
        || HISTORY.is_match(line) && !ACTIVE_AGAIN.is_match(line)
    {
        return false;
    }
    if HYPOTHETICAL.is_match(line) || THROAT_CAUSAL_QUESTION.is_match(line) {
        // An informational question can be followed by a separate current report.
        return BREAK.split(line).skip(1).any(|clause| {
            !HYPOTHETICAL.is_match(clause)
                && PERSON.is_match(clause)
                && stated(&THROAT_EMERGENCY, clause)
        });
    }
    stated(&THROAT_EMERGENCY, line) || PERSON.is_match(line) && affirmed(&THROAT_EMERGENCY, line)
}

fn pet_breathing(line: &str) -> bool {
    let bare = LEAD.replace(line, "");
    let Some(sign) = BREATHING.find(&bare) else {
        return false;
    };
    // Require a single, explicitly animal subject before carrying veterinary urgency.
    let subject = bare[..sign.start()].trim();
    let tail = bare[sign.end()..].trim_matches([' ', ',', '.', '!', '?']);
    PET_BREATHING_SUBJECT.is_match(subject) && matches!(tail, "" | "now" | "right now" | "again")
}

pub(in crate::bot) fn current(line: &str) -> Option<Urgency> {
    if HYPOTHETICAL.is_match(line) || HISTORY.is_match(line) && !ACTIVE_AGAIN.is_match(line) {
        return None;
    }
    if pet_breathing(line) {
        return Some(Urgency::PetBreathing);
    }
    let chest = stated(&CHEST, line);
    let breathing = stated(&BREATHING, line);
    let waking = stated(&WAKING_BREATHLESS, line) || stated(&WOKEN_SHORT_OF_BREATH, line);
    if stated(&STROKE, line)
        || breathing && (!waking || chest || ACTIVE_AGAIN.is_match(line))
        || chest && (affirmed(&DANGEROUS_CHEST, line) || affirmed(&WITH_CHEST, line))
    {
        Some(Urgency::Emergency)
    } else if waking && !DENY.is_match(line) {
        Some(Urgency::NightBreathing)
    } else if chest {
        Some(Urgency::ChestAssessment)
    } else {
        None
    }
}

pub(in crate::bot) fn applies(reading: &Reading<'_>, memory: &Memory) -> bool {
    route(reading, memory).is_some()
}

fn route(reading: &Reading<'_>, memory: &Memory) -> Option<Urgency> {
    let line = &reading.line.clean;
    let urgency = if reading.animal_in_hand.is_none() && throat_emergency(line) {
        Some(Urgency::Emergency)
    } else {
        current(line)
    };
    // The approved child-cough card already sends a child with breathing
    // difficulty to emergency services. Keep that existing answer and route.
    if urgency == Some(Urgency::Emergency)
        && reading
            .line
            .words
            .iter()
            .any(|word| matches!(word.as_str(), "cough" | "coughs" | "coughing"))
        && !stated(&CHEST, line)
        && !stated(&STROKE, line)
        && crate::bot::knowledge::lookup::child_cough_emergency(reading, memory)
    {
        return None;
    }
    // A new symptom has its own urgency. Only a short continuation of the last
    // assessment inherits it without naming a symptom again.
    urgency.or_else(|| {
        let bare = LEAD.replace(line, "");
        (FREQUENCY.is_match(&bare) || BARE_NEXT.is_match(&bare))
            .then_some(memory.urgent_medical)
            .flatten()
    })
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let urgency = route(reading, memory)?;
    Some(
        match urgency {
            Urgency::Emergency => EMERGENCY,
            Urgency::ChestAssessment => CHEST_ASSESSMENT,
            Urgency::NightBreathing => NIGHT_BREATHING,
            Urgency::PetBreathing => PET_BREATHING_EMERGENCY,
        }
        .into(),
    )
}

pub(in crate::bot) fn selected(reply: &str) -> Option<Urgency> {
    match memory::unframed_reply(reply).unwrap_or(reply) {
        EMERGENCY => Some(Urgency::Emergency),
        CHEST_ASSESSMENT => Some(Urgency::ChestAssessment),
        NIGHT_BREATHING => Some(Urgency::NightBreathing),
        PET_BREATHING_EMERGENCY => Some(Urgency::PetBreathing),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn explicitly_owned_pet_breathing_distress_uses_veterinary_urgency() {
        for line in [
            "my dog is dying and struggling to breathe",
            "our old cat can't breathe right now",
            "my guinea pig is not breathing",
        ] {
            assert!(current(line) == Some(Urgency::PetBreathing), "{line}");
        }
        for line in [
            "i'm struggling to breathe",
            "my dog is struggling to breathe and so am i",
        ] {
            assert!(current(line) == Some(Urgency::Emergency), "{line}");
        }
        for line in [
            "what if my dog is struggling to breathe?",
            "my cat stopped breathing last year",
        ] {
            assert!(current(line).is_none(), "{line}");
        }
    }

    #[test]
    fn current_throat_red_flags_take_the_safety_route() {
        for line in [
            "I have tonsillitis and cannot swallow my own spit",
            "My tonsils are swollen and I have difficulty breathing",
            "I have a sore throat, can't swallow, what do I do?",
            "What are the signs of tonsillitis? I cannot swallow saliva.",
            "I have chest pain and a sore throat and cannot swallow water",
        ] {
            let turn = Bot::new(Some("0")).unwrap().turn(line);
            assert_eq!(turn.module, Some("safety"), "{line}: {}", turn.reply);
            assert!(turn.reply.contains("Call 999"), "{line}: {}", turn.reply);
        }
        for line in [
            "Can my tonsillitis cause difficulty breathing?",
            "Can my sore throat cause difficulty breathing?",
            "I read that tonsillitis can cause difficulty breathing",
            "I had tonsillitis last year and couldn't swallow",
            "I have tonsillitis but no difficulty breathing",
            "My tonsils are swollen and it hurts to swallow",
            "I have a sore throat and can't swallow these tablets",
            "My dog has tonsillitis and can't swallow",
        ] {
            let turn = Bot::new(Some("0")).unwrap().turn(line);
            assert_ne!(turn.module, Some("safety"), "{line}: {}", turn.reply);
        }
    }

    #[test]
    fn acute_red_flags_keep_safety_across_frequency() {
        for (opening, recurrence) in [
            (
                "i have severe chest pain and cant breathe",
                "its every night",
            ),
            (
                "i have a headache and weakness on one side of my face",
                "its every morning",
            ),
            (
                "my chest feels tight and the pain spreads into my left arm",
                "most days",
            ),
            (
                "i had the flu last week but now i have severe chest pain and cant breathe",
                "its every night",
            ),
        ] {
            let mut bot = Bot::new(Some("0")).unwrap();
            for line in [opening, recurrence, "what do i do now?"] {
                let turn = bot.turn(line);
                assert_eq!(turn.module, Some("safety"), "{line}");
                assert_eq!(turn.reply, EMERGENCY, "{line}");
                assert!(
                    !turn
                        .candidates
                        .iter()
                        .any(|c| matches!(c.module, "answers" | "chat")),
                    "{line}"
                );
            }
        }
    }

    #[test]
    fn keeps_assessment_wording_for_unqualified_pain_or_waking_breathless() {
        for (opening, wanted) in [
            ("i have chest pain", CHEST_ASSESSMENT),
            (
                "i have chest pain but no shortness of breath",
                CHEST_ASSESSMENT,
            ),
            ("i keep waking up gasping for breath", NIGHT_BREATHING),
            ("i keep waking up at 3am out of breath", NIGHT_BREATHING),
            ("i wake up out of breath", NIGHT_BREATHING),
        ] {
            let mut bot = Bot::new(Some("0")).unwrap();
            assert_eq!(bot.turn(opening).reply, wanted, "{opening}");
            assert_eq!(bot.turn("its every night").reply, wanted, "{opening}");
        }
    }

    #[test]
    fn leaves_denials_hypotheticals_and_cleared_history_to_other_readers() {
        for line in [
            "i dont have chest pain or trouble breathing",
            "what if i have severe chest pain and cannot breathe?",
            "can you have chest pain from stress?",
            "i had chest pain last year but my doctor cleared me",
        ] {
            assert!(!applies(&Reading::new(line), &Memory::default()), "{line}");
        }
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(bot.turn("i cant breathe").module, Some("safety"));
        assert_ne!(
            bot.turn("how often should i water my basil?").module,
            Some("safety")
        );
        assert_ne!(bot.turn("its every night").module, Some("safety"));

        assert_eq!(bot.turn("i have chest pain").module, Some("safety"));
        assert_ne!(bot.turn("my doctor said im fine").module, Some("safety"));
        assert_ne!(bot.turn("what do i do now?").module, Some("safety"));
    }
}
