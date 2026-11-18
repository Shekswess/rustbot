//! Resolve named sleep aids before reading the duration of a reported night.
use crate::bot::{Memory, Reading, knowledge::lookup, memory::asleep};
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    lookup::sleep_aid(reading.raw)
        .map(str::to_owned)
        .or_else(|| asleep::answer(reading.raw, memory))
}
