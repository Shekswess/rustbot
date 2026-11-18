//! Ask and comparison gates shared by opinions and everyday choices.

use crate::bot::{Reading, understanding::kinds::Kind};
use regex::Regex;
use std::sync::LazyLock;

use super::{
    OPINION_ASK_MARKERS, OPINION_CHOICE, OPINION_CHOICE_ASK_WORDS, OPINION_CHOICE_ASKS,
    OPINION_CHOICE_WORDS_PATTERN, OPINION_EXPLAINS_PATTERN, OPINION_FIRST_PERSON,
    OPINION_GADGET_TIPS, OPINION_GADGET_VERDICT, OPINION_INTENT, OPINION_TREE_WORDS,
    OPINION_VERDICT_ASK, OWN_OPINION_PATTERN,
};

static EXPLAINS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(OPINION_EXPLAINS_PATTERN).expect("valid opinion explanation pattern")
});
static OWN_OPINION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(OWN_OPINION_PATTERN).expect("valid own-opinion pattern"));
static CHOICE_WORDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(OPINION_CHOICE_WORDS_PATTERN).expect("valid choice words pattern"));

fn meets(words: &[String], markers: &[&str]) -> bool {
    words.iter().any(|word| markers.contains(&word.as_str()))
}

pub(super) fn asked(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    let said = reading.raw.to_lowercase();
    if EXPLAINS.is_match(&said) && !meets(words, &["you", "your", "yourself"]) {
        return false;
    }
    if reading.kind == Kind::Question {
        return true;
    }
    let line = words.join(" ");
    if OPINION_INTENT
        .iter()
        .any(|marker| line.contains(marker) || said.contains(marker))
    {
        return true;
    }
    if OWN_OPINION.is_match(&said) && !meets(words, &["you", "your", "yourself", "yours"]) {
        return false;
    }
    meets(words, OPINION_ASK_MARKERS) || words.len() <= 6 && !meets(words, OPINION_FIRST_PERSON)
}

pub(super) fn either(reading: &Reading<'_>, subjects: &[&[&str]]) -> bool {
    let [left, right] = subjects else {
        return false;
    };
    let words = &reading.line.words;
    let said = reading.raw.to_lowercase();
    let joined = format!(" {} ", words.join(" "));
    if !meets(words, OPINION_CHOICE)
        && !OPINION_CHOICE_ASKS.iter().any(|word| joined.contains(word))
    {
        return false;
    }
    let line = if words.is_empty() { &said } else { &joined };
    if ![left, right]
        .into_iter()
        .all(|side| side.iter().any(|option| line.contains(option)))
    {
        return false;
    }
    asked(reading)
        || (CHOICE_WORDS.is_match(&said)
            && (!meets(words, OPINION_FIRST_PERSON) || meets(words, OPINION_CHOICE_ASK_WORDS))
            && (!OWN_OPINION.is_match(&said)
                || meets(words, &["you", "your", "yourself", "yours"])))
}

pub(super) fn gadget_view(reading: &Reading<'_>) -> &'static str {
    let words = reading.line.words.join(" ");
    let said = if words.is_empty() {
        reading.raw.to_lowercase()
    } else {
        words.to_lowercase()
    };
    if OPINION_VERDICT_ASK.iter().any(|ask| said.contains(ask)) {
        return OPINION_GADGET_VERDICT;
    }
    OPINION_GADGET_TIPS
        .iter()
        .find_map(|(names, reply)| {
            names
                .iter()
                .any(|name| said.contains(name))
                .then_some(*reply)
        })
        .unwrap_or(OPINION_GADGET_VERDICT)
}

pub(super) fn about_a_tree(reading: &Reading<'_>) -> bool {
    let text = format!(" {} ", reading.line.words.join(" ").to_lowercase());
    OPINION_TREE_WORDS
        .iter()
        .any(|name| text.contains(&format!(" {name}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::knowledge::lookup::{guards::Context, rows};

    #[test]
    fn choice_wrappers_preserve_the_authored_ask_and_rendering() {
        for (id, text, starts) in [
            (
                "tea-coffee",
                "Which is healthier, tea or coffee?",
                Some("Neither is bad"),
            ),
            (
                "air-fryer-or-oven",
                "Air fryer or oven?",
                Some("Oven for a big tray"),
            ),
            (
                "cats-dogs",
                "Which is easier to train, a cat or a dog?",
                Some("A dog, on that one"),
            ),
            (
                "best-season",
                "What do you think is the best season?",
                Some("Late spring for me"),
            ),
            (
                "opinion-on-my-gadget",
                "Any tips for my new blender?",
                Some("Liquid in first"),
            ),
            (
                "opinion-on-my-gadget",
                "What do you think of my new blender?",
                Some("The honest test"),
            ),
            (
                "real-or-fake-tree",
                "Should I get a real or fake Christmas tree?",
                Some("Fake if you will use it"),
            ),
            ("real-or-fake-tree", "Real or fake leather?", None),
            (
                "tea-coffee",
                "I usually have tea or coffee in the morning.",
                None,
            ),
        ] {
            let row = rows().find(|row| row.id == id).expect("opinion card");
            let reading = Reading::new(text);
            let answer = row.answer(&Context::new(&reading, &[]));
            if let Some(starts) = starts {
                assert!(
                    answer.is_some_and(|answer| answer.starts_with(starts)),
                    "{text}"
                );
            } else {
                assert!(answer.is_none(), "{text}");
            }
        }
    }
}
