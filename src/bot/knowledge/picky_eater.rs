//! A refusal at the table and an ingredient request can use the same words.

use crate::bot::Reading;
use regex::Regex;
use std::sync::LazyLock;

use super::{
    FOOD_FAMILIES, FOOD_INGREDIENT_WORDS, FOOD_REFUSAL_BARE, FOOD_REFUSAL_CUES,
    FOOD_REFUSAL_FILLER, FOOD_REFUSAL_JOINERS, FOOD_SHORTAGE_AFTER, PICKY_COOK_ASK, food_fold,
};

static TIRED_OF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:tired|sick|bored)\s+of\b|\bfed\s+up\s+(?:of|with)\b").unwrap()
});
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());

pub(super) fn old_enough(reading: &Reading<'_>) -> bool {
    !reading.numbers.iter().any(|number| {
        number.value.fract() == 0.0
            && number.unit.is_some_and(|unit| {
                unit.starts_with("week") || unit.starts_with("month") && number.value < 6.0
            })
    })
}

fn contains_refusal(text: &str) -> bool {
    let lower = food_fold::fold(text);
    let clean = TIRED_OF.replace_all(&lower, " no ");
    let words: Vec<_> = WORD.find_iter(&clean).map(|word| word.as_str()).collect();
    for (start, cue) in words.iter().enumerate() {
        if !FOOD_REFUSAL_CUES.contains(cue) {
            continue;
        }
        for (index, word) in words.iter().enumerate().skip(start + 1) {
            if FOOD_INGREDIENT_WORDS.binary_search(word).is_ok()
                || FOOD_INGREDIENT_WORDS
                    .binary_search(&food_fold::fold(word).as_str())
                    .is_ok()
            {
                if FOOD_REFUSAL_BARE.contains(cue)
                    && words[index + 1..]
                        .iter()
                        .take(2)
                        .any(|after| FOOD_SHORTAGE_AFTER.contains(after))
                {
                    break;
                }
                return true;
            }
            if !FOOD_FAMILIES.contains(word)
                && !FOOD_REFUSAL_FILLER.contains(word)
                && !FOOD_REFUSAL_JOINERS.contains(word)
            {
                break;
            }
        }
    }
    false
}

pub(super) fn answer(reading: &Reading<'_>, reply: &'static str) -> Option<&'static str> {
    let text = if reading.line.clean.is_empty() {
        &reading.line.norm_text
    } else {
        &reading.line.clean
    };
    let for_a_meal = reading
        .line
        .words
        .iter()
        .any(|word| PICKY_COOK_ASK.contains(&word.as_str()));
    (!(for_a_meal && contains_refusal(text))).then_some(reply)
}
