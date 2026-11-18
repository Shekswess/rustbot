//! Bare follow-up markers and subject vocabulary from the pinned understanding reader.
pub(super) const FOLLOW_OPENER: &[&str] = &[
    "and what about",
    "what about",
    "how about",
    "and",
    "or",
    "also",
    "what",
    "how",
];
pub(super) const NOT_A_THING: &[&str] = &[
    "although", "am", "and", "are", "be", "because", "been", "being", "but", "can", "could", "did",
    "do", "does", "had", "has", "have", "he", "here", "i", "if", "im", "is", "it", "its", "ive",
    "may", "me", "might", "no", "not", "or", "shall", "she", "should", "since", "than", "that",
    "then", "there", "they", "theyre", "this", "though", "u", "until", "us", "was", "we", "were",
    "when", "while", "will", "would", "you", "your", "youre",
];
pub(super) const CORRECTION_OPENER: &[&str] =
    &["actually", "nah", "no", "nope", "oops", "sorry", "wait"];
pub(super) const DETERMINER_TAIL: &[&str] = &[
    "a", "an", "each", "every", "her", "his", "my", "our", "the", "their", "your",
];
pub(super) const STOPWORDS: &[&str] = &[
    "a", "about", "above", "after", "again", "against", "all", "also", "am", "an", "and", "any",
    "are", "as", "at", "be", "been", "being", "both", "but", "by", "can", "could", "did", "do",
    "does", "doing", "done", "each", "few", "find", "for", "from", "get", "give", "got", "guess",
    "had", "has", "have", "having", "he", "hello", "help", "her", "here", "hers", "hey", "hi",
    "him", "his", "how", "i", "if", "in", "into", "is", "it", "its", "just", "know", "like", "may",
    "me", "might", "more", "most", "must", "my", "myself", "need", "no", "nor", "not", "of", "off",
    "oh", "ok", "okay", "on", "only", "onto", "or", "other", "our", "out", "over", "own", "please",
    "quite", "really", "same", "shall", "she", "should", "show", "so", "some", "sorry", "such",
    "sure", "tell", "than", "thank", "thanks", "that", "the", "their", "theirs", "them", "then",
    "there", "these", "they", "think", "this", "those", "to", "too", "uh", "um", "under", "up",
    "us", "very", "want", "was", "we", "were", "what", "when", "where", "which", "who", "whom",
    "whose", "why", "will", "with", "within", "without", "would", "yeah", "yes", "you", "your",
    "yours",
];
