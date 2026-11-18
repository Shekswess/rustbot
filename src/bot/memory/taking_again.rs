//! Carry a bare medicine follow-up onto the speaker's recently named complaint.

use super::{done_topic, measurement};
use crate::bot::{Memory, Reading, knowledge::lookup};
use regex::Regex;
use std::{borrow::Cow, collections::HashSet, sync::LazyLock};

macro_rules! rule {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}

rule!(
    FILLER,
    r"^(?:(?:so|and|but|ok|okay|well|right|ah|oh|hmm|then|now|also|please)\b[\s,]+)*"
);
rule!(
    BARE,
    r"^(?:(?:what|which)\s+(?:(?:else|kind|sort)\s+)*(?:can|could|should|may|shall|would)\s+(?:i|we)\s+(?:take|use|have)|(?:is\s+there\s+)?(?:anything|something)\s+(?:else\s+)?(?:that\s+)?(?:i|we)\s+(?:can|could|should|may)\s+(?:take|use|have)|(?:what|which)\s+is\s+(?:safe|ok|okay|best)\s+to\s+(?:take|use))\b(?:\s+for\s+(?:it|this|that|them|those|the pain|the ache|the soreness|the hurt|any of it|any of that|any of those|the same))?[\s!.,?]*$"
);
rule!(
    TRIP,
    r"(?i)\b(?:flight|flights|flying|plane|airport|holiday|holidays|trip|travel|abroad|away|luggage|suitcase|bag|bags|hand luggage|carry on|packing|packed|passport|camping|campsite|hotel|ferry|cruise)\b"
);
rule!(
    OWN,
    r"\b(?:i|im|i'm|ive|i've|id|i'd|my|me|we|we're|weve|our|us)\b"
);
rule!(
    ANOTHER,
    r"\b(?:(?:my|our|the)\s+(?:child|baby|toddler|kid|son|daughter|mum|mom|mother|dad|father|wife|husband|partner|brother|sister|friend|dog|cat)|his|her|their)\b"
);
rule!(
    SYMPTOM,
    r"\b(?:heartburn|indigestion|reflux|nausea|migraine|headache|toothache|earache|backache|stomachache|cough\w*|cold|flu|covid|temperature|fever|cystitis|thrush|hay ?fever|rash|hives|eczema|psoriasis|dandruff|ulcer|blister|sunburn|burn\w*|sprain\w*|cramp\w*|constipation|diarrhoea|diarrhea|sick|vomit\w*|warts?|verruca|sciatica|gout|arthritis|ibs|period|swoll\w*|infect\w*|throat|tooth|teeth|gums?|ears?|stomach|tummy|belly|chest|queasy|nauseous|dizz\w*|faint\w*|bleed\w*|pregnan\w*|pain\w*|ache\w*|hurt\w*|sore|throbbing|killing me|tender)\b"
);
rule!(
    FEEDING,
    r"(?i)\b(?:breast[ -]?feed\w*|nursing|expressing|pumping)\b"
);
rule!(
    PREGNANT,
    r"(?i)\b(?:pregnan\w*|expecting|trimester|weeks gone|weeks along|midwife)\b"
);
rule!(
    PAIN,
    r"\b(?:pain\w*|ache\w*|hurt\w*|sore|throbbing|killing|agony|stinging|burning|tender)\b"
);
rule!(
    SEALED_NEGATION,
    r"\b(?:no longer|went away|no symptoms now|gone away|don't have|do not have)\b"
);

const SUBJECT_FILLER: &str = "ache aching bad badly better bit constantly really still sore tender terrible awful worse worst keep keeps keeping get gets getting got gotten feel feels feeling felt hurt hurts hurting kill kills killing make makes making made go goes going gone come comes coming give gives giving put wont sleep sleeping woke wake waking too very lot lots much many every always sometimes again anymore either though bother bothering last night morning evening day days week weeks month months hour hours while since thing things stuff pain";

fn flat(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn same_text(one: &str, other: &str) -> bool {
    flat(one) == flat(other)
}

fn subject_words(subject: &str) -> impl Iterator<Item = String> {
    let mut seen = HashSet::new();
    done_topic::keywords(subject)
        .filter_map(|word| {
            if SUBJECT_FILLER
                .split_whitespace()
                .any(|filler| word == filler)
                || !word.bytes().all(|byte| byte.is_ascii_alphabetic())
            {
                return None;
            }
            let word = match word.as_str() {
                "coughing" | "coughed" => "cough".to_owned(),
                "vomiting" | "vomited" => "vomit".to_owned(),
                "bleeding" => "bleeding".to_owned(),
                _ => word,
            };
            Some(word)
        })
        .filter(move |word| seen.insert(word.clone()))
        .take(2)
}

fn symptom_subjects(memory: &Memory) -> impl Iterator<Item = String> + '_ {
    let mut seen = HashSet::new();
    memory
        .doneness_topics
        .texts()
        .into_iter()
        .take(2)
        .chain(
            memory
                .turns
                .iter()
                .rev()
                .take(3)
                .map(|turn| turn.user.as_str()),
        )
        .map(|subject| flat(subject).to_lowercase())
        .filter(move |subject| seen.insert(subject.clone()))
        .filter(|subject| {
            !subject.contains('?')
                && OWN.is_match(subject)
                && !ANOTHER.is_match(subject)
                && !SEALED_NEGATION.is_match(subject)
                && SYMPTOM.is_match(subject)
        })
        .take(2)
}

fn health_card(query: &str) -> Option<Cow<'static, str>> {
    let card = lookup::lookup(query)?;
    let medical = matches!(card.domain, "health" | "body" | "complaints" | "pregnancy")
        || matches!(
            (card.domain, card.id),
            (
                "science",
                "heartburn"
                    | "painkillers-together-safe"
                    | "cough-lasting"
                    | "cough-lingering"
                    | "chesty-cough"
            )
        );
    medical.then_some(card.reply)
}

fn names(reply: &str, word: &str) -> bool {
    done_topic::keywords(reply).any(|term| term == word || term.starts_with(word))
}

fn on_screen(reply: &str, previous: &str) -> bool {
    let first = reply
        .split_whitespace()
        .take(10)
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    !first.is_empty() && flat(previous).to_lowercase().contains(&first)
}

fn safe_context(reply: &str, memory: &Memory) -> String {
    let expecting = memory
        .turns
        .iter()
        .rev()
        .take(5)
        .any(|turn| PREGNANT.is_match(&turn.user));
    let feeding = memory
        .turns
        .iter()
        .rev()
        .take(5)
        .any(|turn| FEEDING.is_match(&turn.user));
    if expecting && !feeding && FEEDING.is_match(reply) {
        let scoped: Vec<_> = measurement::sentences(reply)
            .into_iter()
            .filter(|sentence| !FEEDING.is_match(sentence))
            .collect();
        if !scoped.is_empty() {
            return scoped.join(" ");
        }
    }
    if expecting {
        // https://www.nhs.uk/pregnancy/keeping-well/medicines/
        return "Tell your pharmacist, midwife or GP that you are pregnant before taking any new medicine. They can check the exact product, your symptoms and any other medicines you take.".into();
    }
    reply.to_owned()
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = flat(reading.raw).to_lowercase();
    if !BARE.is_match(FILLER.replace(&line, "").trim())
        || TRIP.is_match(&line)
        || memory
            .turns
            .iter()
            .rev()
            .take(2)
            .any(|turn| TRIP.is_match(&turn.user))
    {
        return None;
    }
    let previous = memory.turns.last()?;
    let last_subject = previous.user.to_lowercase();
    if OWN.is_match(&last_subject)
        && SYMPTOM.is_match(&last_subject)
        && SEALED_NEGATION.is_match(&last_subject)
    {
        return None;
    }
    if matches!(previous.module, "safety" | "urgent" | "a_count_of_packets") {
        return None;
    }
    let current = (previous.bot.len() >= 120)
        .then(|| lookup::lookup_in_context(reading, memory))
        .flatten();
    let same = |reply: &str| {
        current
            .as_ref()
            .is_some_and(|entry| same_text(reply, &entry.reply))
    };
    let mut kept = None;
    let mut asked = 0;
    for subject in symptom_subjects(memory) {
        for word in subject_words(&subject) {
            for suffix in ["", " pain"] {
                if asked >= 3 {
                    return None;
                }
                asked += 1;
                let Some(reply) = health_card(&format!("what can i take for {word}{suffix}"))
                else {
                    continue;
                };
                if !names(&reply, &word) {
                    continue;
                }
                if on_screen(&reply, &previous.bot) {
                    if kept.is_none() {
                        kept = Some((reply, PAIN.is_match(&subject)));
                    }
                } else if !same(&reply) {
                    return Some(safe_context(&reply, memory));
                }
            }
        }
    }
    let (reply, pain) = kept?;
    if pain
        && let Some(general) = health_card("what painkiller can i take")
        && !on_screen(&general, &previous.bot)
        && !same(&general)
    {
        return Some(safe_context(&general, memory));
    }
    (!same(&reply)).then(|| safe_context(&reply, memory))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn yields_for_travel_and_a_complaint_about_someone_else() {
        for subject in [
            "I am packing for a flight",
            "My son has a cough",
            "My cough has gone away",
        ] {
            let memory = Memory {
                turns: vec![Exchange {
                    user: subject.into(),
                    bot: "Tell me more.".into(),
                    module: "chat",
                }]
                .into(),
                ..Memory::default()
            };
            assert!(
                respond(&Reading::new("What can I take?"), &memory).is_none(),
                "{subject}"
            );
        }
    }

    #[test]
    fn abstains_when_the_most_recent_complaint_has_resolved() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("I keep getting heartburn");
        bot.turn("My heartburn went away");
        assert!(respond(&Reading::new("What can I take?"), &bot.memory).is_none());
        assert!(respond(&Reading::new("Can I take ibuprofen?"), &bot.memory).is_none());
    }

    #[test]
    fn bare_question_can_reuse_the_selected_heartburn_knowledge() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("I keep getting heartburn");
        bot.turn("Is it normal?");
        let answer = bot.turn("What can I take?");
        assert!(
            answer.reply.to_lowercase().contains("heartburn"),
            "{:?}",
            answer.candidates
        );
        assert!(matches!(
            answer.module,
            Some("taking_asked_again" | "answers")
        ));
    }
}
