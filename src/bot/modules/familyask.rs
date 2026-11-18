//! Questions about relatives the bot would have if it were a person.

use crate::bot::{Memory, Reading, compose};
use fancy_regex::Regex as Lookaround;
use regex::Regex;
use std::sync::LazyLock;

#[path = "familyask_data.rs"]
mod data;
use data::*;

static RELATIONS: LazyLock<String> = LazyLock::new(|| {
    let mut words: Vec<_> = FAMILIES
        .iter()
        .flat_map(|family| family.relations.iter().copied())
        .collect();
    words.sort_by_key(|word| std::cmp::Reverse(word.len()));
    words.join("|")
});

static THEIR_RELATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?i)\b(?:your|ur|yours|you're|youre)\s+(?:\w+\s+){{0,2}}?(?:{})\b",
        *RELATIONS
    ))
    .expect("family possessive")
});

static HAS_RELATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        concat!(
            r"(?i)^{LEAD_IN}(?:(?:have|has|do|does|did)\s+(?:you|u|ya)\s+(?:have\s+|got\s+|ever\s+)*",
            r"|got\s+|any\s+)(?:any\s+|a\s+|an\s+|some\s+)?(?:{relations})\b"
        ),
        LEAD_IN = LEAD_IN,
        relations = *RELATIONS
    ))
    .expect("family possession question")
});

static HOW_MANY_RELATIONS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?i)^{LEAD_IN}how many\s+(?:\w+\s+){{0,2}}?(?:{})\b[\w\s,']*\b(?:you|u|ya)\b",
        *RELATIONS
    ))
    .expect("family count question")
});

static SOMEONE_ELSES_RELATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?i)\b(?:my|our|his|her|their)\s+(?:{})\b",
        *RELATIONS
    ))
    .expect("other family's relation")
});

static CHOICE: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(&format!(
        r"(?i)\bor\b\s+(?!(?:not|no|is|are|was|were|else|anything|something|otherwise)\b|(?:{})\b)\w+",
        *RELATIONS
    ))
    .expect("choice between something other than relations")
});

static QUESTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        concat!(
            r"(?i)^(?:{lead}(?:(?:can|could|would|will)\s+you\s+)?(?:tell|told|say|talk|chat)\s+(?:me\s+)?",
            r"(?:something\s+|a\s+bit\s+|more\s+|anything\s+)?about\s+|",
            r"(?:is|are|was|were|do|does|did|have|has|had|will|would|can|could|should|shall|",
            r"who|whos|whose|what|whats|what's|where|wheres|when|whens|why|whys|how|hows|which|",
            r"any|anyone|ever|doesnt|didnt|isnt|arent|wasnt|werent)\b)"
        ),
        lead = LEAD_IN
    ))
    .expect("family question opener")
});

static PAST_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(?:was|were|did|said|had|went|wore|told|thought|felt|been|there|at it|come)\b",
    )
    .expect("past family question")
});

static NO_HAND_BACK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:think|feel|felt|mind|care|miss|lonely|sorry|upset)\b")
        .expect("family feelings question")
});

fn collapsed(raw: &str) -> String {
    raw.split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn family_question(said: &str) -> Option<&'static Family> {
    let own_relation = THEIR_RELATION.find(said);
    let possession = HAS_RELATION.find(said);
    let count = HOW_MANY_RELATIONS.find(said);
    let named = own_relation.or(possession).or(count)?;
    if SOMEONE_ELSES_RELATION.is_match(said)
        || CHOICE.is_match(said).unwrap_or(false)
        || !(possession.is_some()
            || count.is_some()
            || said.ends_with('?')
            || QUESTION.is_match(said))
    {
        return None;
    }
    let family = named
        .as_str()
        .split_whitespace()
        .map(|word| word.trim_matches([',', '.', '\'']).to_lowercase())
        .find_map(|word| {
            FAMILIES
                .iter()
                .find(|family| family.relations.contains(&word.as_str()))
        });
    Some(family.unwrap_or(&GENERAL_FAMILY))
}

fn reply_for(raw: &str, memory: &Memory) -> Option<String> {
    let said = collapsed(raw);
    let family = family_question(&said)?;
    let seed = said.to_lowercase();
    let honest = compose::pick(family.replies, memory, &seed);
    if NO_HAND_BACK.is_match(&said) {
        return Some(honest.to_owned());
    }
    let pool = if PAST_ASK.is_match(&said) {
        HOW_IT_WENT
    } else {
        THE_PEOPLE
    };
    Some(format!("{honest} {}", compose::pick(pool, memory, &seed)))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    reply_for(reading.raw, memory)
}

pub(crate) fn claims(reading: &Reading<'_>, _: &Memory) -> bool {
    family_question(&collapsed(reading.raw)).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asks_about_the_bot_allow_questions_in_several_forms() {
        for (line, family) in [
            ("were your mum and dad there", &FAMILIES[1]),
            ("OK, do you have any sisters", &FAMILIES[2]),
            ("how many daughters have you got", &FAMILIES[3]),
            ("tell me about your other half", &GENERAL_FAMILY),
            ("do any of your grandparents live nearby?", &FAMILIES[0]),
            ("have you got brothers or sisters", &FAMILIES[2]),
        ] {
            assert_eq!(
                family_question(line).map(|given| given.relations),
                Some(family.relations),
                "{line}"
            );
        }
        for line in [
            "give your mum a ring",
            "i rang your mum yesterday",
            "was your mum with my friend or her brother?",
            "does your gran want a curry or a chinese",
            "do you have a cat?",
        ] {
            assert!(family_question(line).is_none(), "{line}");
        }
    }

    #[test]
    fn combines_the_honest_answer_and_the_appropriate_hand_back() {
        let memory = Memory::default();
        for (line, pool) in [
            ("what did your dad say", HOW_IT_WENT),
            ("are your aunt and uncle coming", THE_PEOPLE),
        ] {
            let reply = reply_for(line, &memory).unwrap();
            assert!(pool.iter().any(|ending| reply.ends_with(ending)), "{reply}");
        }
        let reply = reply_for("do you miss your family?", &memory).unwrap();
        assert!(GENERAL_FAMILY.replies.contains(&reply.as_str()));
    }
}
