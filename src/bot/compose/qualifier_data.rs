//! Word and phrase lists from PyBot’s qualifier reader.
use regex::Regex;
use std::sync::LazyLock;
pub(super) static CONDITION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(if|unless|until|once|before|after|whenever|as long as)\b"##)
        .expect("_CONDITION")
});
pub(super) static HANDOFF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(gp|doctor|nurse|pharmacist|chemist|dentist|vet|council|landlord|seller|retailer|shop|airline|insurer|bank|999|111|nhs|advice|adviser|advisor|helpline|ombudsman|employer|school)\b"##).expect("_HANDOFF")
});
pub(super) static LIMIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(\d+|one|two|three|four|five|six|seven|eight|nine|ten|twelve|fourteen|twenty)\s+(day|days|week|weeks|month|months|year|years)\b"##).expect("_LIMIT")
});
pub(super) static NEGATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(not|no|none|never|dont|doesnt|didnt|isnt|arent|wasnt|cant|cannot|without|nobody|nothing)\b"##).expect("_NEGATION")
});
pub(super) static AMOUNT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(\d+|one|two|three|four|five|six|seven|eight|nine|ten)\b"##).expect("_AMOUNT")
});
pub(super) const QUESTION_START: &[&str] = &[
    "am", "any", "anything", "are", "arent", "can", "cant", "could", "did", "do", "does", "doesnt",
    "dont", "had", "has", "have", "how", "hows", "is", "isnt", "may", "might", "must", "shall",
    "should", "was", "were", "what", "whats", "when", "where", "wheres", "which", "who", "whom",
    "whose", "why", "whys", "will", "wont", "would",
];
pub(super) static CORRECTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(i meant|i mean|i said|i wasnt saying|im talking about|i was talking about|the other one|not my|not the)\b"##).expect("_CORRECTION")
});
pub(super) const LEAD: &[&str] = &["Right.", "Got it.", "Fair enough.", "Okay.", "Right then."];
pub(super) const POOL: &[&str] = &[
    "{lead} The test does not turn on that: {clause}.",
    "{lead} Same test either way: {clause}.",
    "{lead} That leaves the test where it was: {clause}.",
    "{lead} The bit that decides it is unchanged: {clause}.",
];
pub(super) const STOP: &[&str] = &[
    "a", "about", "also", "an", "and", "any", "are", "as", "at", "be", "been", "being", "bit",
    "but", "can", "could", "did", "do", "does", "done", "for", "from", "had", "has", "have", "he",
    "her", "his", "i", "i'm", "if", "im", "in", "into", "is", "it", "it's", "its", "just", "kind",
    "lot", "many", "me", "more", "most", "much", "my", "no", "not", "of", "on", "or", "our",
    "over", "quite", "really", "she", "should", "so", "some", "sort", "than", "that", "the",
    "their", "them", "then", "these", "they", "thing", "things", "this", "those", "to", "too",
    "under", "us", "very", "was", "we", "were", "will", "with", "would", "you", "your",
];
