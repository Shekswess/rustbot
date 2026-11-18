//! Claim the person to see when the user asks a bare referral question.

use crate::bot::{Memory, Reading, compose};

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    compose::who::reply(reading, memory)
}
