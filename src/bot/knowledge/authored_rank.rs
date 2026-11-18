//! Rank selected cards by PyBot's authored alternatives and domain rarity.

use std::cmp::Reverse;

use super::{Entry, Form};

const ASK_FRAME: &[&str] = &[
    "how", "hows", "what", "whats", "which", "who", "whom", "whose", "when", "where", "why", "is",
    "are", "was", "were", "am", "be", "been", "can", "could", "should", "would", "will", "shall",
    "may", "might", "must", "do", "does", "did", "doing", "i", "im", "get", "gets", "got",
    "getting",
];

const TAIL_ASK: &[&str] = &[
    "what", "whats", "which", "how", "where", "when", "who", "why", "should", "shall", "could",
    "would", "can",
];

// Pinned plan-start vocabulary from dishes.py and lookup._rank. Rarity here
// includes the source rows that the Rust extractor has yet to register.
const PLAN_START_UNIQUE: &[&str] = &[
    "2",
    "4",
    "6",
    "8",
    "considering",
    "eight",
    "evening",
    "fancying",
    "feel",
    "feeling",
    "four",
    "friday",
    "later",
    "might",
    "planning",
    "saturday",
    "six",
    "tea",
    "thinking",
];
const PLAN_START_FRAME: &[&str] = &[
    "am", "are", "do", "doing", "evening", "friday", "i", "might", "saturday", "sunday", "this",
    "time", "today", "tomorrow", "tonight", "we", "weekend",
];

pub(super) fn rank(entry: &Entry<'_>, line: &[Vec<Form>], words: &[String]) -> i32 {
    let tail = words
        .iter()
        .enumerate()
        .skip(4)
        .rfind(|(_, word)| TAIL_ASK.contains(&word.as_str()))
        .map(|(index, _)| index);
    let mut said = 0;
    let mut kept = 0;
    let mut asking = 0;
    let plan_start = entry.row.id == "plan-start";
    let mut names_its_subject = false;
    for (group, &authored) in entry.key.iter().zip(entry.row.key) {
        // PyBot ranks the first authored hit among equally weighted alternatives.
        // It chooses again without the covered positions used to qualify the card.
        let best = group
            .iter()
            .zip(authored)
            .enumerate()
            .filter_map(|(index, (phrase, text))| {
                let (start, end) = phrase.spans(line).next()?;
                let weight = if end - start == 1 { 2 } else { 3 };
                let frame = text
                    .split_whitespace()
                    .all(|word| ASK_FRAME.contains(&word));
                let (rarity, names_its_subject) = if plan_start {
                    (
                        text.split_whitespace()
                            .filter(|word| PLAN_START_UNIQUE.contains(word))
                            .count(),
                        text.split_whitespace()
                            .any(|word| !PLAN_START_FRAME.contains(&word)),
                    )
                } else {
                    (phrase.unique_in_domain, false)
                };
                Some((
                    weight,
                    !frame,
                    Reverse(index),
                    start,
                    rarity,
                    names_its_subject,
                ))
            })
            .max();
        if let Some((weight, subject, _, start, rarity, names_subject)) = best {
            names_its_subject |= names_subject;
            let score = weight + i32::try_from(rarity).expect("short authored key");
            if !subject && tail.is_some_and(|tail| start >= tail) {
                asking += score;
            } else {
                said += weight;
                kept += score;
            }
        }
    }
    if plan_start && !names_its_subject {
        return 1;
    }
    entry.row.bias + kept + if said > 2 { 0 } else { asking }
}
