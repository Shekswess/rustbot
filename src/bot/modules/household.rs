//! State and recall for who lives in a household.

use crate::bot::{Memory, Reading, memory};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    memory::household::reply(reading, memory)
}

pub(crate) fn claims(reading: &Reading<'_>, memory: &Memory) -> bool {
    memory::household::claims(reading, memory)
}
