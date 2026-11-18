//! Recall an earlier quotation attributed to someone else.

use crate::bot::{Memory, Reading, memory::quote_back};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    quote_back::third_party(reading.raw, memory)
}
