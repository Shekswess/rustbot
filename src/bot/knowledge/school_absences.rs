//! Preserve the authored school-absence gate. Triage and return periods follow NHS and UKHSA.

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use crate::bot::modules::safety::fever_red_flags;
#[path = "school_absence_data.rs"]
mod data;

fn names(words: &[&str]) -> Regex {
    let alternatives = words
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"\b(?:{alternatives})\b")).expect("school absence terms")
}

macro_rules! groups {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| names(data::$name));)+
    };
}

groups!(SICKNESS, POX, ANTIBIOTIC, FEVER, MILD, EXCLUSION_ASK);

static PERTUSSIS: LazyLock<Regex> =
    LazyLock::new(|| names(&["whooping", "whooping cough", "pertussis"]));
static SCARLET_OR_STREP: LazyLock<Regex> =
    LazyLock::new(|| names(&["scarlet fever", "scarlet", "strep"]));
static TONSILLITIS: LazyLock<Regex> = LazyLock::new(|| names(&["tonsillitis"]));
static MILD_CONDITIONS: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    data::MILD_CONDITIONS
        .iter()
        .map(|(words, reply)| (names(words), *reply))
        .collect()
});

const EMERGENCY: &str = "If your child has the signs you've described now, call 999 immediately in the UK. If you're outside the UK, call your local emergency number. Their symptoms need emergency care before any decision about going back to school.";
const SICKNESS_REPLY: &str = "In England, keep them off school or nursery until 48 hours after the last episode of vomiting or diarrhoea. Tell the school why they're absent. If you're worried they're dehydrated or getting worse, contact NHS 111.";
const POX_REPLY: &str = "In England, keep a child with chickenpox off school for at least five days after the rash starts. Wait until the blisters have crusted over before they return. Tell the school when the rash started.";
const PERTUSSIS_REPLY: &str = "In England, with whooping cough, keep them off school for 48 hours after antibiotics start, or for 21 days after symptoms start if they aren't treated with antibiotics. Ask their clinician and let the school know.";
const SCARLET_REPLY: &str = "In England, keep them off school for at least 24 hours after antibiotics start for scarlet fever or a confirmed group A strep infection. Let the school know. If they haven't been assessed and you think they have scarlet fever, contact your GP or NHS 111.";
const FEVER_REPLY: &str = "Keep your child home while they have a high temperature. For a mild illness they can usually go back once the fever is gone and they're well enough to take part. In England, some infections have separate exclusion periods. Ask your school about its policy and contact NHS 111 if you're worried or they're getting worse.";
const GENERAL_REPLY: &str = "In England, it depends on the illness. Keep them off school until 48 hours after vomiting or diarrhoea stops. For a mild cold or cough they can go in if they feel well and have no high temperature. Chickenpox, scarlet fever and whooping cough have separate exclusion periods. Let the school know what they have and check the UKHSA guidance for that infection.";
const ANTIBIOTICS_REPLY: &str = "In England, the return date depends on the infection being treated. Antibiotics alone don't set one waiting period. If the clinician didn't say when your child can return, ask them or check the school's policy for that infection. Keep them home while they're feverish or too unwell to take part.";

fn render(clean: &str) -> Option<Cow<'static, str>> {
    let sickness = SICKNESS.is_match(clean);
    let pox = POX.is_match(clean);
    let antibiotic = ANTIBIOTIC.is_match(clean);
    let fever = FEVER.is_match(clean);
    let mild = MILD.is_match(clean);
    if !(sickness || pox || antibiotic || fever || mild || EXCLUSION_ASK.is_match(clean)) {
        return None;
    }
    let reply = if fever_red_flags(clean) {
        EMERGENCY
    } else if sickness {
        SICKNESS_REPLY
    } else if pox {
        POX_REPLY
    } else if antibiotic {
        match (PERTUSSIS.is_match(clean), SCARLET_OR_STREP.is_match(clean)) {
            (true, true) => {
                "In England, the exclusion periods differ. For scarlet fever or a confirmed group A strep infection, wait at least 24 hours after antibiotics start. For whooping cough, wait 48 hours after antibiotics start, or 21 days from the start of symptoms if they aren't treated with antibiotics. If the diagnosis is unclear, ask the clinician when your child can go back."
            }
            (true, false) => PERTUSSIS_REPLY,
            (false, true) => SCARLET_REPLY,
            (false, false) if TONSILLITIS.is_match(clean) => {
                "In England, tonsillitis alone doesn't require a fixed time off school. Keep them home if they have a high temperature or are too unwell to take part. If they've been diagnosed with group A strep or scarlet fever, follow the clinician's instructions about when to return."
            }
            (false, false) => ANTIBIOTICS_REPLY,
        }
    } else if fever {
        FEVER_REPLY
    } else if mild {
        MILD_CONDITIONS
            .iter()
            .find_map(|(condition, reply)| condition.is_match(clean).then_some(*reply))
            .unwrap_or(data::MILD_COLD)
    } else {
        GENERAL_REPLY
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn answer(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    render(&reading.line.clean)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fever_emergencies_precede_the_school_calendar() {
        for line in [
            "how long off school with a fever and a stiff neck",
            "my son has fever and it is hard to wake him, should i keep him off school",
            "can she go back to school with a fever and a rash that does not fade when pressed",
        ] {
            assert!(
                answer(&Reading::new(line))
                    .unwrap()
                    .contains("call 999 immediately"),
                "{line}"
            );
        }
        let ordinary =
            Reading::new("when can my child go back to school with a fever but no stiff neck");
        assert_eq!(answer(&ordinary), Some(Cow::Borrowed(FEVER_REPLY)));
    }

    #[test]
    fn england_reentry_rules_distinguish_infection_and_admission() {
        for (line, key) in [
            (
                "when can my child go back to school after scarlet fever",
                "24 hours after antibiotics start",
            ),
            (
                "when can my child go back to school after whooping cough",
                "21 days after symptoms start",
            ),
            (
                "when can my child go back to school after vomiting",
                "48 hours after the last episode",
            ),
            (
                "when can my child go back to school after chickenpox",
                "at least five days after the rash starts",
            ),
            (
                "can my child go to school with nits",
                "head lice don't require time off school",
            ),
        ] {
            let reply = answer(&Reading::new(line)).unwrap();
            assert!(
                reply.starts_with("In England,") && reply.contains(key),
                "{line}: {reply}"
            );
        }
        assert_eq!(
            answer(&Reading::new(
                "my children are off school tomorrow and it is raining"
            )),
            None
        );
    }
}
