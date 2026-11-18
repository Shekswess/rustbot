//! Compose the fallback response to medical news and a continuing fall emergency.
use crate::bot::{Memory, Reading, modules::the_detail_the_thread_adds::heavy};
use fancy_regex::Regex;
use std::sync::LazyLock;
macro_rules! rule {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}
include!("medical_news_data.rs");

fn serious_illness(line: &str) -> bool {
    if NOT_THEIR_NEWS.is_match(line).unwrap_or(false)
        || !CLOSE_OWNER.is_match(line).unwrap_or(false)
    {
        return false;
    }
    let Some(serious) = SERIOUS_NEWS.find(line).ok().flatten() else {
        return false;
    };
    NOT_YET_NEWS
        .find(line)
        .ok()
        .flatten()
        .is_none_or(|p| p.start() >= serious.start())
}
pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = &reading.clean;
    let repaired = &reading.line.plain;
    if crate::bot::modules::safety::fall_continuation(clean, memory) {
        return Some(super::choose_recent(EMERGENCY_AFTER, memory, clean).to_owned());
    }
    if ![clean, repaired].iter().any(|line| {
        (heavy::medical_news(line) || ILL_RELATIVE.is_match(line).unwrap_or(false))
            && heavy::bad_news(line)
    }) {
        return None;
    }
    let pool = if reading.kind != crate::bot::understanding::kinds::Kind::Question
        && (serious_illness(clean) || serious_illness(repaired))
    {
        SOMEONE_SERIOUSLY_ILL
    } else if heavy::named_news(clean) || heavy::named_news(repaired) {
        NAMED_NEGATIVE
    } else {
        NEGATIVE
    };
    Some(super::choose_recent(pool, memory, clean).to_owned())
}
