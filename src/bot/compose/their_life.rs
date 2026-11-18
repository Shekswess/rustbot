//! Personal preferences and questions about people mentioned earlier in the conversation.
#[path = "their_life_data.rs"]
mod data;
use crate::bot::Memory;
use data::*;

pub(crate) fn reply(raw: &str, memory: &Memory) -> Option<String> {
    let s = raw
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches([' ', '.', '!', ','])
        .to_lowercase();
    let hit = |r: &fancy_regex::Regex| r.is_match(&s).unwrap_or(false);
    if s.is_empty()
        || s.split_whitespace().count() > 12
        || [
            &*NOT_THIS,
            &RECALL,
            &A_HOW_TO,
            &AN_AGE_NAME_OR_PLACE,
            &MISSING,
            &WHERE_THEY_ARE,
            &A_NEED_OF_THEIRS,
        ]
        .iter()
        .any(|r| hit(r))
    {
        return None;
    }
    let lines = if hit(&TASTE) {
        TASTE_LINES
    } else {
        let (lines, about) = if hit(&WORK) {
            (WORK_LINES, None)
        } else if hit(&STAY) {
            (STAY_LINES, None)
        } else {
            (
                THEIRS_LINES,
                ABOUT_THEM
                    .captures(&s)
                    .ok()??
                    .name("about")
                    .map(|m| m.as_str()),
            )
        };
        if let Some(subject) = about {
            let previous = &memory.turns[..memory.turns.len().saturating_sub(1)];
            if ALREADY_SAID
                .find_iter(subject)
                .flatten()
                .map(|m| m.as_str())
                .filter(|w| STOP_ABOUT.binary_search(w).is_err())
                .any(|w| previous.iter().any(|t| t.user.to_lowercase().contains(w)))
            {
                return None;
            }
        }
        if hit(&A_PRONOUN)
            && !hit(&THEIR_RELATION)
            && !memory
                .turns
                .iter()
                .rev()
                .take(6)
                .any(|t| SAID_WHO.is_match(&t.user).unwrap_or(false))
        {
            return None;
        }
        lines
    };
    Some(lines[memory.turns.len() % lines.len()].into())
}

#[cfg(test)]
#[path = "../../../tests/fixtures/their_life.rs"]
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
        assert_eq!(reply(q, &memory).as_deref(), expected, "{i} / {q}");
    }
}
