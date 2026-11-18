//! The opening and availability replies consume the shared conversation reading.
#[path = "chat_presence_data.rs"]
mod data;
use crate::bot::{Memory, Reading, compose};
use data::*;

pub(super) fn opening(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if reading.context_fields.presence {
        let safety = memory
            .turns
            .iter()
            .rev()
            .take(3)
            .find(|t| !matches!(t.module, "carers_turn" | "safety_cards" | "greeting" | ""))
            .is_some_and(|t| t.module == "safety");
        if safety {
            return None;
        }
        let clean = reading.line.clean.replace(['\'', '’'], "");
        if let Some(last) = memory.turns.last()
            && last.module == "answers"
            && !BUYING.is_match(&last.user.to_lowercase())
            && STILL_ONLY
                .captures(clean.trim_end_matches(['.', '?', '!']))
                .and_then(|c| c.name("tail"))
                .is_some_and(|m| POINTS_AT_IT.contains(&m.as_str()))
        {
            return None;
        }
        return Some(compose::pick(PRESENCE, memory, "").into());
    }
    reading
        .context_fields
        .opening
        .then(|| compose::pick(OPENING, memory, "").into())
}
