//! Recall a child's recorded age, advanced by the interval in the current question.

use crate::bot::{Memory, Reading, memory};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    memory::aboutthem::an_age_ahead(reading, memory)
}
