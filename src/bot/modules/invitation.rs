//! Answer an invitation addressed to the bot with the source's occasion-specific pool.

use crate::bot::{Memory, Reading, compose};
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

#[path = "invitation_data.rs"]
mod data;
#[cfg(test)]
#[path = "invitation_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    INVITED,
    INVITED_AT_THE_START,
    NOT_THEIRS,
    NOT_WANTED,
    NOT_AN_INVITE,
    PRESENCE,
    NOT_PRESENCE,
    ASKING_IF_IM_HERE,
);

static OCCASIONS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::OCCASIONS.iter().map(|(pattern, _)| pattern)).expect("invitation occasions")
});

fn pool(line: &str) -> Option<&'static [&'static str]> {
    if line.is_empty() || ASKING_IF_IM_HERE.is_match(line) {
        return None;
    }
    let asking_to_come = PRESENCE.is_match(line) && !NOT_PRESENCE.is_match(line);
    if !(asking_to_come || INVITED.is_match(line))
        || (NOT_THEIRS.is_match(line) && !INVITED_AT_THE_START.is_match(line))
        || NOT_AN_INVITE.is_match(line)
        || (!asking_to_come && NOT_WANTED.is_match(line))
    {
        return None;
    }

    OCCASIONS
        .matches(line)
        .into_iter()
        .next()
        .map(|index| data::OCCASIONS[index].1)
        .or_else(|| asking_to_come.then_some(data::PRESENCE_REPLIES))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = reading
        .raw
        .to_lowercase()
        .split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    Some(compose::pick(pool(&clean)?, memory, &clean).to_string())
}
