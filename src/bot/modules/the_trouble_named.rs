//! A follow-up on a thing the speaker named as their trouble.
use crate::bot::{Memory, Reading, compose};

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    compose::trouble::named::reply(reading, memory)
}

pub(crate) fn takes_as_it_comes(reading: &Reading<'_>, memory: &Memory) -> bool {
    respond(reading, memory).is_some()
}
