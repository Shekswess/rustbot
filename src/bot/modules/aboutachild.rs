//! Answer a measured question using the recorded age of the child it points at.

use crate::bot::{Memory, Reading, memory::aboutthem};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let question = aboutthem::with_the_age(reading.raw, memory)?;
    let mut written = Reading::understand(
        &question,
        memory.turns.iter().map(|turn| turn.user.as_str()),
        None,
        &memory.prior_readings,
    );
    written.wants = None;
    let answer = super::answers::offer(&written, memory)?.reply;
    (!answer.trim().is_empty()).then(|| answer.trim().to_owned())
}

pub(crate) fn claims(reading: &Reading<'_>, memory: &Memory) -> bool {
    aboutthem::with_the_age(reading.raw, memory).is_some()
}
