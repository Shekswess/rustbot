//! Follow-up about someone the user previously mentioned.
use crate::bot::{Memory, Reading, memory};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    memory::callback::reply(reading.raw, memory)
}
