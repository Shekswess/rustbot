//! Hand over the next instruction from a multi-step answer already given.

use crate::bot::{Memory, Reading, memory::plan};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    plan::reply(reading, memory)
}
