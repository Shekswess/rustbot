//! Lens scratch care and the upstream distinction between spectacles and a pane.
#[cfg(test)]
#[path = "clothing_lenses_tests.rs"]
mod tests;

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

static PUNCTUATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9' ]+").unwrap());
static NOT_SPECTACLES: LazyLock<Regex> = LazyLock::new(|| Regex::new(PANES_AND_TUMBLERS).unwrap());

fn names_a_pane(line: &str) -> bool {
    NOT_SPECTACLES.is_match(&PUNCTUATION.replace_all(&line.to_lowercase(), " "))
}

pub(super) fn not_a_pane(reading: &Reading<'_>) -> bool {
    let line = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    !names_a_pane(line)
}

pub(super) fn scratch(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    (not_a_pane(reading)
        && !earlier
            .iter()
            .rev()
            .take(2)
            .any(|turn| names_a_pane(&turn.user)))
    .then_some(Cow::Borrowed(LENS_HELP))
}

const PANES_AND_TUMBLERS: &str = r#"\b(?:oven|hobs?|cooker|microwave|door|doors|window|windows|windscreen|windshield|mirror|table|tables|shower|bicarb|bicarbonate|soda|chimney|glasses of|wine glass|pint glass|car|mot|test)\b"#;

const LENS_HELP: &str = "A scratch in the coating can't be polished out, and paper towels and tissues are what make most of them: wash the lenses with water and washing-up liquid, then dry them with a microfibre cloth. An optician can fit new lenses into your own frame.";
