//! Reported attempts and requests to try another, from bot/understanding/attempts.py.
const ENDS_THE_ATTEMPT: &[&str] = &[
    "although", "because", "but", "however", "since", "still", "though", "whereas", "while", "yet",
];
const STARTS_A_CLAUSE: &[&str] = &[
    "even", "he", "her", "hes", "him", "i", "im", "it", "its", "me", "my", "our", "she", "shes",
    "that", "thatll", "them", "there", "they", "this", "though", "us", "we", "you",
];
const NOT_A_CLAUSE_AFTER_AND: &[&str] = &[
    "anymore",
    "anything",
    "everything",
    "he",
    "i",
    "it",
    "its",
    "neither",
    "no",
    "none",
    "nor",
    "nothing",
    "now",
    "she",
    "so",
    "still",
    "that",
    "then",
    "there",
    "they",
    "this",
    "we",
    "you",
];
const NOT_A_THING: &[&str] = &[
    "a",
    "about",
    "after",
    "afternoon",
    "again",
    "all",
    "already",
    "also",
    "an",
    "and",
    "any",
    "anyone",
    "anything",
    "around",
    "at",
    "away",
    "back",
    "before",
    "bit",
    "bits",
    "both",
    "but",
    "day",
    "days",
    "down",
    "earlier",
    "either",
    "even",
    "evening",
    "everyone",
    "everything",
    "first",
    "for",
    "her",
    "his",
    "hour",
    "hours",
    "in",
    "it",
    "just",
    "kind",
    "last",
    "later",
    "lot",
    "lots",
    "month",
    "months",
    "more",
    "morning",
    "much",
    "my",
    "night",
    "no",
    "not",
    "nothing",
    "now",
    "of",
    "off",
    "ok",
    "okay",
    "on",
    "once",
    "one",
    "ones",
    "or",
    "our",
    "out",
    "over",
    "quite",
    "rather",
    "really",
    "since",
    "so",
    "some",
    "someone",
    "something",
    "sooner",
    "sort",
    "stuff",
    "that",
    "the",
    "their",
    "them",
    "then",
    "these",
    "thing",
    "this",
    "those",
    "through",
    "time",
    "times",
    "to",
    "today",
    "tonight",
    "too",
    "twice",
    "up",
    "very",
    "week",
    "weeks",
    "well",
    "with",
    "yeah",
    "year",
    "years",
    "yesterday",
    "your",
];
const REPORT_SUBJECTS: &[&str] = &["i", "i've", "id", "ive", "we", "we've", "wed", "weve"];
const ONE_MORE: &[&str] = &["again", "another", "instead", "more", "one"];
const HANDS_OVER: &[&str] = &[
    "come", "do", "give", "got", "have", "make", "need", "rap", "read", "say", "sing", "tell",
    "think", "want", "write",
];
const STARTS_A_QUESTION: &[&str] = &[
    "any", "anything", "can", "could", "how", "shall", "should", "what", "whats", "when", "where",
    "whether", "which", "who", "why", "would",
];

fn has(set: &[&str], word: &str) -> bool {
    set.binary_search(&word).is_ok()
}
fn reported(words: &[String], index: usize) -> bool {
    index == 0
        || words[index.saturating_sub(4)..index]
            .iter()
            .any(|w| has(REPORT_SUBJECTS, w))
}
fn end(words: &[String], at: usize) -> usize {
    for i in at + 1..words.len() {
        let word = &words[i];
        if has(ENDS_THE_ATTEMPT, word)
            || has(STARTS_A_CLAUSE, word)
            || word == "and"
                && words
                    .get(i + 1)
                    .is_none_or(|w| has(NOT_A_CLAUSE_AFTER_AND, w))
        {
            return i;
        }
        let another = words.iter().skip(i + 1).take(4).any(|w| has(ONE_MORE, w));
        if has(STARTS_A_QUESTION, word)
            && (i == at + 1
                || matches!(
                    words[i - 1].as_str(),
                    "and"
                        | "but"
                        | "so"
                        | "then"
                        | "now"
                        | "everything"
                        | "anything"
                        | "something"
                        | "that"
                        | "this"
                        | "it"
                )
                || another)
            || has(HANDS_OVER, word) && another
        {
            return i;
        }
    }
    words.len()
}

#[derive(Debug, Default, serde::Serialize)]
pub(crate) struct Attempts {
    pub(crate) tried: Vec<String>,
    pub(crate) asks_for_one_more: bool,
}

pub(crate) fn read(words: &[String]) -> Attempts {
    let mut result = Attempts::default();
    let mut first = true;
    for (i, w) in words.iter().enumerate() {
        if w != "tried" || !reported(words, i) {
            continue;
        }
        let end = end(words, i);
        if words.len() <= 24 {
            for word in &words[i + 1..end] {
                if !has(NOT_A_THING, word)
                    && word.chars().count() >= 3
                    && word.chars().all(char::is_alphabetic)
                    && !result.tried.contains(word)
                {
                    result.tried.push(word.clone());
                }
            }
        }
        if first {
            let rest = &words[end..];
            result.asks_for_one_more = rest.iter().any(|w| has(ONE_MORE, w))
                && (rest
                    .iter()
                    .any(|w| has(HANDS_OVER, w) || has(STARTS_A_QUESTION, w))
                    || rest.windows(2).any(|p| {
                        matches!(p[0].as_str(), "can" | "could" | "would" | "will")
                            && matches!(p[1].as_str(), "you" | "u")
                    }));
            first = false;
        }
    }
    result
}
