//! A close person moving away, before or after the move.
use crate::bot::{Memory, Reading, understanding::kinds::Kind};
use fancy_regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}
include!("leaving_data.rs");

pub(super) fn reply(read: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = &read.line.clean;
    let repaired = &read.line.plain;
    let seen = |pattern: &Regex| {
        pattern.is_match(clean).unwrap_or(false)
            || (clean != repaired && pattern.is_match(repaired).unwrap_or(false))
    };
    if !seen(&LEAVING_A_PERSON)
        || !seen(&LEFT_BEHIND)
        || seen(&NOT_LEAVING_ANYONE)
        || read.kind != Kind::Statement
        || super::fallback::asks_a_question(
            clean,
            read.raw,
            &read
                .line
                .words
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        || memory
            .turns
            .iter()
            .rev()
            .take(4)
            .any(|t| super::loss_reader::reads_as_loss(&t.user.to_lowercase()))
        || super::loss_reader::reads_as_loss(clean)
    {
        return None;
    }
    let pool = match (
        seen(&A_CHILD),
        seen(&THE_LEAVE_HAS_HAPPENED),
        A_FEELING_IN_THE_LINE.is_match(clean).unwrap_or(false),
    ) {
        (true, true, _) => A_CHILD_LEFT,
        (false, true, _) => SOMEONE_GONE,
        (true, false, true) => A_CHILD_LEAVING,
        (false, false, true) => SOMEONE_LEAVING,
        (true, false, false) => A_CHILD_LEAVING_PLAIN,
        (false, false, false) => SOMEONE_LEAVING_PLAIN,
    };
    Some(super::choose_recent(pool, memory, clean).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    mod fixture {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/leaving.rs"
        ));
    }
    #[test]
    fn close_person_moves_match_pinned_fallback() {
        for &(text, expected) in fixture::CASES {
            assert_eq!(
                reply(&Reading::new(text), &Memory::default()).as_deref(),
                expected,
                "{text}"
            );
        }
        for text in [
            "I moved the chairs to the garden",
            "Is my neighbour moving away?",
            "My daughter is leaving a review",
        ] {
            assert_eq!(
                reply(&Reading::new(text), &Memory::default()),
                None,
                "{text}"
            );
        }
    }
}
