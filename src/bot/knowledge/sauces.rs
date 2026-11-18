//! Method and keeping replies for a named sauce.

use crate::bot::Reading;
use regex::Regex;
use std::sync::LazyLock;

use super::{
    SAUCE_COOKING_TIME_PATTERN, SAUCE_KEEPING_PATTERN, SAUCE_KEEPING_WORD_PATTERN,
    SAUCE_STEP_LENGTH_PATTERN, SAUCE_UNTIL_PATTERN,
};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(KEEPING, SAUCE_KEEPING_PATTERN);
pattern!(KEEPING_WORD, SAUCE_KEEPING_WORD_PATTERN);
pattern!(UNTIL, SAUCE_UNTIL_PATTERN);
pattern!(STEP, SAUCE_STEP_LENGTH_PATTERN);
pattern!(COOKING_TIME, SAUCE_COOKING_TIME_PATTERN);

pub(super) fn answer(
    reading: &Reading<'_>,
    method: &'static str,
    keeping: Option<&'static str>,
    yoghurt: Option<&'static str>,
) -> Option<&'static str> {
    let line = &reading.line;
    let text = if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    };
    if KEEPING_WORD.is_match(text) {
        return keeping;
    }
    if !UNTIL.is_match(text) && !STEP.is_match(text) {
        if COOKING_TIME.is_match(text) {
            return None;
        }
        if KEEPING.is_match(text) {
            return keeping;
        }
    }
    let yoghurt =
        yoghurt.filter(|_| line.clean.contains("yoghurt") || line.clean.contains("yogurt"));
    Some(yoghurt.unwrap_or(method))
}
