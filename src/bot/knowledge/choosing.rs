//! Comparisons whose source row names both the options and the kind of choice.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

#[path = "choosing_data.rs"]
mod data;

struct Choice {
    left: &'static [&'static str],
    right: &'static [&'static str],
    asks: &'static [&'static str],
    needs: &'static [&'static str],
    response: &'static str,
}

static ORDER_OF_STEPS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\bwhat\s+(?:should|shall|do|would|must|can)\s+(?:i|we)\s+(?:do|sort|pick|choose|get|buy|wash|clean|tackle|start|book|pack|fix|handle|plan|change|replace|check|ask|say|look\s+at|deal\s+with|set\s+up|make|go\s+for|put\s+up)\b.*\b(?:first|next|before|afterwards|after\s+that)\b")
        .expect("asking for the order of steps")
});

pub(super) fn names(text: &str, options: &[&str]) -> bool {
    let bytes = text.as_bytes();
    options.iter().any(|option| {
        text.match_indices(option).any(|(start, _)| {
            (start == 0 || !bytes[start - 1].is_ascii_alphabetic())
                && bytes
                    .get(start + option.len())
                    .is_none_or(|c| !c.is_ascii_alphabetic())
        })
    })
}

fn named_response(text: &str) -> Option<&'static str> {
    data::CHOICES
        .iter()
        .find(|choice| {
            names(text, choice.left)
                && (choice.right.is_empty() || names(text, choice.right))
                && names(text, choice.asks)
                && (choice.needs.is_empty() || names(text, choice.needs))
        })
        .map(|choice| choice.response)
}

pub(crate) fn pair_and_request(reading: &Reading<'_>) -> bool {
    !ORDER_OF_STEPS.is_match(reading.raw) && named_response(&reading.clean).is_some()
}

pub(crate) fn reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    named_response(&reading.clean).map(Cow::Borrowed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choices_require_the_two_named_sides_and_yield_to_step_order() {
        assert!(pair_and_request(&Reading::new(
            "is it better to overpay my mortgage or save"
        )));
        assert!(!pair_and_request(&Reading::new("what does a boiler cost")));
        assert!(!pair_and_request(&Reading::new(
            "moving to a bigger flat, what should i do first"
        )));
    }
}
