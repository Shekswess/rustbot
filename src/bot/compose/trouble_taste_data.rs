// Taste shapes used to distinguish a preference from a bad day.

pattern!(
    UNSURE,
    r#"(?i)\b(?:not sure|not certain|unsure|don'?t know|do not know|dunno|no idea)\s+(?:if|whether|that)\s+(?:i|we)(?:'d|\s+would)?\s+(?:actually\s+|really\s+|even\s+)?(?:like|love|enjoy|be into|get on with)\s+([A-Za-z][A-Za-z'\u2019-]*(?:\s+(?:[A-Za-z][A-Za-z'\u2019-]*|and|or|of|to|the|a|my|our|your|their|his|her)){0,4})"#
);
pattern!(
    NO,
    r#"(?i)\b(?:i|we)\s+(?:really\s+|just\s+|actually\s+)?(?:don'?t|do not|dont|never)\s+(?:think\s+(?:i|we)\s+|really\s+)?(?:like|love|eat|drink|want)\s+([A-Za-z][A-Za-z'\u2019-]*(?:\s+(?:[A-Za-z][A-Za-z'\u2019-]*|and|or|of|to|the|a|my|our|your|their|his|her)){0,4})"#
);
pattern!(
    NOT_KEEN,
    r#"(?i)(?:\b(?:i'?m|i am|we'?re|we are)|^)\s*not\s+(?:(?:that|really|very|so|all that|quite)\s+)?(?:much\s+of\s+)?(?:a\s+|the\s+)?(?:(?:big|huge|massive|great)\s+)?(?:keen on|fan of|into|mad about|fussed on|bothered about)\s+([A-Za-z][A-Za-z'\u2019-]*(?:\s+(?:[A-Za-z][A-Za-z'\u2019-]*|and|or|of|to|the|a|my|our|your|their|his|her)){0,4})"#
);
pattern!(
    MY_THING,
    r#"(?i)\b([A-Za-z][A-Za-z'\u2019-]*(?:\s+(?:[A-Za-z][A-Za-z'\u2019-]*|and|or|of|to|the|a|my|our|your|their|his|her)){0,4})\s+(?:isn'?t|is not|aren'?t|are not)\s+(?:(?:really|that|exactly|quite)\s+)?(?:my|our)\s+(?:(?:kind|sort)\s+of\s+|cup\s+of\s+tea|thing)"#
);
pattern!(
    YES,
    r#"(?i)\b(?:i|we)\s+(?:do\s+|really\s+|absolutely\s+|just\s+|actually\s+)?(?:like|love|adore|prefer)\s+([A-Za-z][A-Za-z'\u2019-]*(?:\s+(?:[A-Za-z][A-Za-z'\u2019-]*|and|or|of|to|the|a|my|our|your|their|his|her)){0,4})"#
);
pattern!(
    HATE,
    r#"(?i)\b(?:i|we)\s+(?:really\s+|just\s+|actually\s+|absolutely\s+|properly\s+)?(?:hate|hates|dislike|dislikes|loathe|loathes|detest|detests|can'?t stand|cannot stand|really can'?t stand)\s+([A-Za-z][A-Za-z'\u2019-]*(?:\s+(?:[A-Za-z][A-Za-z'\u2019-]*|and|or|of|to|the|a|my|our|your|their|his|her)){0,4})"#
);
pattern!(
    CLAUSE_SPLIT,
    r#"(?i)\s*;\s*|\s+(?:but|and|though|whereas)\s+(?=(?:(?:i|we|he|she|they)\s+)?(?:do\s+|really\s+|absolutely\s+|just\s+|actually\s+|properly\s+)?(?:like|love|adore|prefer|hate|hates|dislike|dislikes|loathe|detest|enjoy|eat|drink|want|can'?t stand|cannot stand)\b)"#
);
pattern!(
    QUANTITY,
    r#"(?i)^(?:enough|too much|too little|much|any|more|less|loads of|lots of|plenty of|a lot of|as much)\b"#
);
pattern!(SHORTFALL_VERB, r#"(?i)\b(?:eat|drink)\s+$"#);
pattern!(SHORTFALL_TAIL, r#"(?i)\b(?:at\s+all|any\s?more)\s*$"#);
pub(super) const THING_STOP: &[&str] = &[
    "adore", "adores", "although", "am", "are", "aren't", "be", "because", "been", "being", "can",
    "could", "detest", "detests", "did", "dislike", "dislikes", "do", "does", "doesn't", "dont",
    "enjoy", "enjoys", "got", "had", "has", "hate", "hated", "hates", "have", "he", "how", "i",
    "if", "is", "isn't", "it", "like", "loathe", "loathes", "love", "loved", "loves", "may", "me",
    "might", "prefer", "prefers", "shall", "she", "should", "since", "so", "that", "them", "there",
    "these", "they", "this", "those", "though", "until", "us", "was", "we", "were", "what",
    "whatever", "when", "where", "which", "while", "who", "whose", "why", "will", "would", "you",
];
pub(super) const TAIL: &[&str] = &[
    "a",
    "about",
    "actually",
    "afternoon",
    "all",
    "also",
    "an",
    "and",
    "any",
    "anyhow",
    "anymore",
    "anyway",
    "as",
    "at",
    "birthday",
    "but",
    "by",
    "christmas",
    "either",
    "evening",
    "for",
    "friday",
    "from",
    "haha",
    "hehe",
    "idk",
    "imo",
    "in",
    "innit",
    "lmao",
    "loads",
    "lol",
    "lot",
    "lots",
    "monday",
    "more",
    "morning",
    "much",
    "ngl",
    "night",
    "now",
    "of",
    "on",
    "or",
    "pretty",
    "quite",
    "really",
    "right",
    "saturday",
    "so",
    "still",
    "sunday",
    "tbf",
    "tbh",
    "than",
    "the",
    "then",
    "tho",
    "though",
    "thursday",
    "to",
    "today",
    "tomorrow",
    "tonight",
    "too",
    "tuesday",
    "very",
    "wednesday",
    "week",
    "weekday",
    "weekend",
    "well",
    "with",
    "yet",
];
pub(super) const NOT_A_THING: &[&str] = &[
    "a",
    "an",
    "being",
    "doing",
    "having",
    "her",
    "here",
    "herself",
    "him",
    "himself",
    "how",
    "if",
    "it",
    "itself",
    "me",
    "myself",
    "one",
    "ones",
    "oneself",
    "ourselves",
    "so",
    "such",
    "that",
    "the",
    "them",
    "themselves",
    "there",
    "these",
    "this",
    "those",
    "to",
    "us",
    "way",
    "what",
    "when",
    "which",
    "who",
    "why",
    "you",
    "yourself",
    "yourselves",
];
pattern!(
    DISLIKE_IN,
    r#"(?i)\b(?:(?:really|just|actually|absolutely|properly|simply)\s+)*(?:hate|hates|hated|dislike|dislikes|disliked|loathe|loathes|loathed|detest|detests|detested|can'?t stand|cannot stand)\b.*$"#
);
pattern!(
    TASTE_VERB,
    r#"(?i)^(?:(?:i|we|he|she|they)\s+)?(?:do\s+|really\s+|absolutely\s+|just\s+|actually\s+|properly\s+)?(?:like|love|adore|prefer|hate|hates|dislike|dislikes|loathe|detest|enjoy|eat|drink|want|can'?t stand|cannot stand)\b"#
);
pattern!(
    TROUBLE_IN_TELLING,
    r#"(?i)\b(?:being|bullied|bullies|bully|teas\w+|never|always|keeps|kept|refuses|refused|stopped|stops|quit|hates?|hated|hit|hits|shout\w*|yell\w*|scream\w*|argu\w*|row|won'?t|doesn'?t|didn'?t|isn'?t|can'?t|cannot|failed|failing|behind|ignores?|ignored|blames?|blamed|lost|losing|dead against|just want|just need|driving me|drives me|doing my head in|fed up|sick of|had enough|tether|drives? me mad)\b"#
);
