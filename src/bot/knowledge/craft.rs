//! Small decorating and costume questions from the pinned registry.

use std::{borrow::Cow, sync::LazyLock};

use crate::bot::Reading;
use regex::{Regex, RegexSet};

use super::{
    COSTUME_MAKING, COSTUME_METHOD, COSTUME_TIPS, PREP_MATERIAL, PREP_NOT_WOODWORK,
    PREP_ORDER_WORDS, PREP_STEPS,
};

static MAKING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(COSTUME_MAKING).expect("costume making pattern"));
static TIPS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(COSTUME_TIPS.iter().map(|&(words, _)| {
        let names = words
            .iter()
            .map(|word| regex::escape(word))
            .collect::<Vec<_>>();
        format!(r"\b(?:{})\b", names.join("|"))
    }))
    .expect("costume names are literals")
});

pub(super) fn orders_preparation(reading: &Reading<'_>) -> bool {
    let text = if reading.raw.is_empty() {
        &reading.line.clean
    } else {
        reading.raw
    }
    .to_lowercase();
    let has = |words: &[&str]| words.iter().any(|word| text.contains(word));
    !has(PREP_NOT_WOODWORK)
        && !has(PREP_MATERIAL)
        && has(PREP_ORDER_WORDS)
        && PREP_STEPS.iter().filter(|words| has(words)).take(2).count() == 2
}

fn costume_line<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    }
}

pub(super) fn asks_for_costume(reading: &Reading<'_>) -> bool {
    MAKING.is_match(costume_line(reading))
}

pub(super) fn costume(reading: &Reading<'_>) -> Cow<'static, str> {
    let text = costume_line(reading).to_lowercase();
    if let Some(index) = TIPS.matches(&text).into_iter().next() {
        return format!("{COSTUME_METHOD} {}", COSTUME_TIPS[index].1).into();
    }
    Cow::Borrowed(COSTUME_METHOD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preparation_guard_requires_two_kinds_of_steps() {
        assert!(orders_preparation(&Reading::new("varnish or sand first")));
        for line in [
            "sand first or smooth it later",
            "should i sand my floor before varnishing",
            "sand first, or what varnish should i use",
        ] {
            assert!(!orders_preparation(&Reading::new(line)), "{line}");
        }
    }

    #[test]
    fn costume_tip_uses_authored_character_order() {
        let line = Reading::new("make a dinosaur costume for World Book Day");
        assert!(asks_for_costume(&line));
        assert!(costume(&line).contains("For World Book Day pick a character"));
        assert!(!asks_for_costume(&Reading::new(
            "what can we do at halloween"
        )));
    }
}
