//! Ask who a first-turn pronoun refers to when the user asks for that person's facts.
#[path = "unnamed_person_data.rs"]
mod data;
use crate::bot::{Memory, Reading, compose};
use data::*;

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if memory.turns.iter().any(|t| !t.user.trim().is_empty()) {
        return None;
    }
    let s = reading
        .raw
        .to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let hit = |r: &fancy_regex::Regex| r.is_match(&s).unwrap_or(false);
    if !(s.contains('?') || hit(&AN_ASK))
        || !hit(&A_THIRD_PARTY)
        || !hit(&A_FACT_ABOUT_THEM)
        || hit(&SOMEBODY_IN_THE_LINE)
        || hit(&NAMES_THEM)
    {
        return None;
    }
    Some(compose::pick(WHO_IS_IT, memory, &s).into())
}

#[cfg(test)]
#[path = "../../../tests/fixtures/unnamed_person.rs"]
mod reference;
#[test]
fn matches_reference_questions() {
    let mut memory = Memory::default();
    for &(i, q, expected) in reference::CASES {
        memory.turns = reference::HISTORY[i]
            .iter()
            .map(|s| crate::bot::Exchange {
                user: s.to_string(),
                bot: "Understood.".into(),
                module: "greeting",
            })
            .collect();
        assert_eq!(
            respond(&crate::bot::Reading::new(q), &memory).as_deref(),
            expected,
            "{i} / {q}"
        );
    }
}
