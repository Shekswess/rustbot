// Tokens and subject words from the pinned proposal fit rules.
const WORD: &str = r#"[a-z0-9]+(?:'[a-z]+)?"#;
const STOP: &[&str] = &[
    "about", "again", "all", "am", "and", "any", "are", "be", "been", "but", "came", "can",
    "cannot", "cant", "come", "could", "did", "does", "done", "down", "even", "ever", "for", "get",
    "gets", "going", "gone", "got", "had", "has", "have", "her", "here", "hers", "him", "his",
    "how", "into", "is", "it", "its", "just", "keep", "keeps", "kept", "know", "let", "lets",
    "like", "made", "make", "many", "may", "might", "more", "most", "much", "must", "need",
    "never", "nor", "not", "now", "off", "one", "only", "onto", "our", "ours", "out", "over",
    "put", "really", "said", "say", "says", "she", "should", "some", "still", "take", "than",
    "that", "the", "their", "them", "then", "there", "these", "they", "think", "this", "those",
    "too", "took", "two", "under", "us", "very", "want", "was", "went", "were", "what", "when",
    "where", "which", "who", "why", "will", "with", "would", "you", "your", "yours",
];
