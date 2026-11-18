//! Authored pace and duration patterns from PyBot 7d63c7f.

pub(super) const PACE_ASK: &str = r#"(?i)\bhow (?:slowly|fast|quickly|soon)\b"#;

pub(super) const KEEP_UP_ASK: &str = r#"(?i)\bhow long (?:should|shall|do|can|could|would) (?:i|we) (?:keep|carry on|go on|stick|stay|continue|persist)\b|\bhow long (?:should|shall|do|can|could|would) (?:i|we) keep \w+ing\b"#;

pub(super) const A_LENGTH: &str = r#"(?i)\b(?:\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|eighteen|twenty|thirty|forty|fifty|sixty|a few|a couple of|several|a)\s+(?:(?:or|to)\s+(?:\d+|two|three|four|five|six|seven|eight|ten|twelve|fifteen|twenty)\s+)?(?:weeks?|months?|days?|hours?|minutes?|fortnights?)\b"#;

pub(super) const A_RATE_LEAD: &str = r#"(?i)\b(?:\d+|one|two|three|four|five|six|seven|eight|nine|ten|twelve|a|an|once|twice|half)\s+(?:runs?|times|sessions?|days?|weeks?|months?|hours?|minutes?|miles?|km|kms|kilometres?|reps?|sets?|walks?|workouts?|feeds?|charges?|meals?|nights?)\s+$"#;

pub(super) const ALLOWED: &[&str] = &[
    "a", "add", "again", "am", "an", "are", "at", "be", "build", "can", "could", "did", "do",
    "does", "doing", "easy", "fast", "faster", "first", "for", "go", "going", "harder", "i", "in",
    "increase", "is", "it", "jog", "jogging", "me", "more", "much", "my", "next", "now", "of",
    "on", "our", "push", "run", "running", "runs", "shall", "should", "slow", "slower", "so",
    "start", "step", "that", "the", "then", "these", "this", "to", "too", "train", "training",
    "up", "walk", "walking", "walks", "was", "we", "will", "work", "would", "you",
];

pub(super) const KEEP_ALLOWED: &[&str] = &[
    "at",
    "doing",
    "for",
    "going",
    "it",
    "long",
    "longer",
    "looking",
    "now",
    "same",
    "searching",
    "that",
    "the",
    "this",
    "today",
    "tonight",
    "trying",
    "waiting",
    "watching",
    "way",
];

pub(super) const STEP: &str = r#"(?i)\b(?:add|increase|build(?:ing)? up|work(?:ing)? up|step(?:ping)? up|extend|progress)\b[^.;]{0,70}\b(?:a|an|each|every|per)\s+(?:week|day|session|run|month|fortnight)\b"#;

pub(super) const PACE: &str = r#"(?i)\b(?:add|increase|build(?:ing)? up|work(?:ing)? up|step(?:ping)? up|extend|progress|no more than|only)\b[^.;]{0,70}\b(?:a|an|each|every|per)\s+(?:week|day|session|run|month|fortnight)\b|\b(?:one|two|three|four|five|six|\d+)\s+times?\s+a\s+(?:week|day)\b|\bevery other day\b|\bweek by week\b|\ba (?:minute|day|session|week)\s+(?:at a time|more|longer)\b|\b(?:a|an|one)\s+\w+(?:\s+\w+)?\s+at a time\b|\ba little at a time\b|\bgradually\b"#;

pub(super) const TEST: &str = r#"(?i)[^.;]{0,60}\b(?:slow enough to talk|hold a conversation|talk test|conversational|can still talk|able to talk)\b[^.;]{0,60}"#;

pub(super) const LEADS: &[(&str, &[&str])] = &[
    (
        "slowly",
        &[
            "Slowly, a step at a time",
            "Slowly, and only as the plan steps up",
            "Slowly - less than you think",
        ],
    ),
    (
        "fast",
        &[
            "No faster than the plan says",
            "Only as fast as the answer above says",
            "Not fast - by the plan's own step",
        ],
    ),
    (
        "quickly",
        &[
            "Only as quickly as the plan steps up",
            "As quickly as the plan says, and no more",
        ],
    ),
    (
        "soon",
        &[
            "No sooner than the answer above says",
            "Only when the plan says you are ready",
        ],
    ),
];

pub(super) const REST: &[&str] = &[
    "That is the pace the plan sets: {rest}.",
    "The rest of the plan is the check on it: {rest}.",
    "The plan above already says the rest of it: {rest}.",
];

pub(super) const NEXT: &[&str] = &[
    "The rest of it is as the answer says: {next}.",
    "Then carry on as the answer says: {next}.",
    "Nothing else changes after that: {next}.",
];

pub(super) const TESTED: &[&str] = &[
    "The test is the plan's own - {test}.",
    "Judge it by the one the answer already gives - {test}.",
    "And keep the plan's own check - {test}.",
];

pub(super) const CONTINUES: &str =
    r#"(?i)^(?:for\s|\w+ing\b|until\b|then\s|and\b|so\b|which\b|that\b|it\b|this\b|about\b)"#;

pub(super) const NOT_A_TAIL: &str = r#"(?i)^(?:and|but|so|it|they|them|this|that|the|a|an|after|before|since|until|once|when|while)\b"#;

pub(super) const DANGLES: &str = r#"(?i)\b(?:until|till|because|so that|so|which|that|when|where|if|while|unless|than|and|or|but|to|of|in|on|at|for|with|by|from|as|into|about|after|before)[^,;:.!?]*$"#;

pub(super) const A_PORTION: &str =
    r#"(?i)\b(?:for each|for every|a time|per|twice|once a|\d+ times)\b"#;

pub(super) const STOPPED: &[&str] = &[
    "a", "an", "and", "are", "as", "at", "be", "been", "being", "but", "by", "can", "could", "did",
    "do", "does", "done", "for", "from", "get", "gets", "go", "going", "got", "he", "her", "here",
    "him", "how", "i", "if", "in", "is", "it", "its", "just", "keep", "keeping", "keeps", "made",
    "make", "makes", "me", "my", "no", "not", "of", "on", "one", "or", "our", "said", "say",
    "says", "she", "should", "so", "than", "that", "the", "them", "then", "there", "these", "they",
    "this", "those", "three", "to", "two", "us", "was", "we", "were", "what", "when", "where",
    "which", "who", "why", "will", "with", "would", "you", "your",
];

pub(super) const ASK_WORDS: &[&str] = &[
    "again", "can", "could", "did", "do", "does", "going", "how", "i", "it", "keep", "keeping",
    "keeps", "long", "longer", "many", "more", "much", "shall", "should", "that", "thing",
    "things", "this", "time", "times", "we", "would",
];

pub(super) const KEEP_LEADS: &[&str] = &[
    "Keep it up for {length}",
    "Stay with it for {length}",
    "Give it {length}",
];

pub(super) const KEEP_MARK: &[&str] = &[
    "the answer's own mark for it",
    "the answer's own length for it",
    "what the answer says to allow",
];
