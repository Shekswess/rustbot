//! Floor-level answers to questions implied by a factual follow-up.

use crate::bot::{Memory, Reading, compose};

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    compose::their_detail::reply(reading, memory)
}
