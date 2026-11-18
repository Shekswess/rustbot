//! Recall the number of distinct children identified in the conversation.

use crate::bot::{Memory, Reading, memory};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    memory::kidcount::reply(reading, memory)
}
