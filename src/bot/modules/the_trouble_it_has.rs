//! A question about the fault of a thing when the fallback did not address it.
use crate::bot::{Memory, Reading, compose};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    compose::trouble::machine::reply(reading, memory)
}
