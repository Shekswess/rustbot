//! Answer a question about splitting or filling a cake.

use crate::bot::{
    Memory, Reading,
    knowledge::lookup,
    understanding::kinds::{Intent, Kind},
};
use fancy_regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
include!("cake_layer_data.rs");

pub(crate) fn respond(reading: &Reading<'_>, _memory: &Memory) -> Option<String> {
    if !matches!(reading.kind, Kind::Question) && reading.features.intent != Intent::Request {
        return None;
    }
    let mut text = format!("{} {}", reading.line.clean, reading.raw);
    for word in &reading.line.words {
        text.push(' ');
        text.push_str(word);
    }
    if !A_CAKE.is_match(&text).unwrap_or(false)
        || !BETWEEN.is_match(&text).unwrap_or(false)
        || NOT_ASKING.is_match(&text).unwrap_or(false)
    {
        return None;
    }
    if let Some(id) = lookup::matched_id(reading)
        && !NOT_THE_CAKE_ROW.contains(&id)
    {
        return None;
    }
    Some(THE_FILLING.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod reference {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/cake_layers.rs"
        ));
    }

    #[test]
    fn fills_the_cakes_in_the_pinned_cases() {
        let memory = Memory::default();
        let mismatches: Vec<_> = reference::CASES
            .iter()
            .filter_map(|&(text, expected)| {
                let got = respond(&Reading::new(text), &memory);
                (got.as_deref() != expected.then_some(THE_FILLING)).then(|| {
                    (
                        text,
                        expected,
                        got.is_some(),
                        lookup::matched_id(&Reading::new(text)),
                    )
                })
            })
            .collect();
        assert!(mismatches.is_empty(), "{mismatches:?}");
    }
}
