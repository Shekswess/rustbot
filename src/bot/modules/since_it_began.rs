//! Recall the time since a recorded activity began.

use crate::bot::{Memory, Reading, memory};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    memory::when_length::respond(reading, memory)
}
