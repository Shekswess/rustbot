//! Recall the latest age list for the family group the user asks about.

use crate::bot::{Memory, Reading, memory};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    memory::groupages::reply(reading, memory)
}

pub(crate) fn claims(reading: &Reading<'_>, memory: &Memory) -> bool {
    respond(reading, memory).is_some()
}
