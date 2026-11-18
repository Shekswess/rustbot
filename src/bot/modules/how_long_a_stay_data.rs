//! Authored guest-stay and duration patterns from PyBot 7d63c7f.

pub(super) const STAY: &str = r#"(?i)\b(?:coming|comes|came|coming over|coming round|getting here|arriving|arrives|visiting)\b[^.!?]{0,40}?\b(?:stay|staying|stays|visit|to us|over|round|for (?:the )?weekend|for \w+ (?:days|nights|weeks))\b|\b(?:is|are|was|were)\s+(?:coming\s+)?(?:to\s+)?staying\b|\bstaying\s+(?:with\s+us|here|over|for)\b|\bstopping\s+(?:with\s+us|over|here)\b|\b(?:a|the)\s+visit\b"#;

pub(super) const A_LENGTH: &str = r#"(?i)\bfor\s+(?:about\s+|just\s+|only\s+)?(\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|a\s+few|a\s+couple\s+of)\s*(day|days|night|nights|week|weeks)\b|\b(?:for\s+)?(?:the|this|that|next|at\s+the)\s+weekend\b|\b(?:a|one)\s+week\b|\b(?:a|one)\s+fortnight\b"#;

pub(super) const A_LENGTH_SAID: &str = r#"(?i)\b(?:\d+|two|three|four|five|six|seven|eight|nine|ten|twenty)\s*(?:full|whole|more|extra)?\s*(?:days?|nights?|weeks?|hours?|minutes?)\b|\ba\s+(?:week|fortnight|month)\b|\bweekend\b|\bfortnight\b"#;

pub(super) const ASKS_A_LENGTH: &str =
    r#"(?i)\bhow\s+long\b|\bhow\s+many\s+(?:days|nights|weeks)\b|\bhow\s+much\s+time\b"#;

pub(super) const ASKS_ADVICE: &str = r#"(?i)\bhow\s+long\s+(?:should|do\s+you\s+think|would\s+you|is\s+(?:it\s+)?(?:right|normal))\b|\bhow\s+many\s+(?:days|nights)\s+should\b|\bhow\s+long\s+(?:is\s+)?(?:too\s+long|long\s+enough)\b"#;

pub(super) const GENERIC: &[&str] = &[
    "about", "actually", "again", "all", "also", "an", "and", "any", "are", "around", "as", "at",
    "be", "been", "before", "bit", "bits", "both", "but", "by", "can", "could", "days", "do",
    "does", "doing", "else", "even", "ever", "exactly", "for", "from", "get", "getting", "go",
    "going", "got", "had", "has", "have", "he", "her", "here", "hers", "him", "his", "how", "i",
    "if", "in", "into", "is", "it", "its", "just", "keep", "know", "like", "long", "may", "me",
    "might", "more", "much", "my", "nearly", "nights", "no", "not", "now", "of", "off", "on",
    "once", "one", "only", "or", "our", "ours", "out", "over", "own", "please", "really", "right",
    "said", "say", "she", "should", "so", "some", "sorted", "stay", "staying", "stays", "still",
    "stuff", "take", "takes", "taking", "than", "that", "the", "their", "theirs", "them", "then",
    "there", "these", "they", "thing", "this", "those", "to", "too", "up", "us", "very", "visit",
    "visiting", "want", "was", "way", "we", "weekend", "well", "were", "what", "when", "where",
    "which", "while", "who", "why", "will", "with", "would", "you", "your", "yours",
];

pub(super) const A_LENGTH_REPLY: &[&str] = &[
    "{length} is a comfortable stay: one shared outing a day is enough for that long, and keep one evening back for yourself.",
    "For {length}, plan one outing a day rather than a schedule - and leave one evening free.",
    "{length} gives you room for an outing a day. The rest of the card is what matters more: the bed and the meal times.",
];

pub(super) const ADVICE: &[&str] = &[
    "Three or four days is the sweet spot for a first stay: long enough to settle in, short enough that nobody runs out of things to say.",
    "A weekend to a week. Past a week, plan one evening apart or the visit starts to feel like a duty for both of you.",
    "Up to a week is the usual answer - long enough to settle in, and the length where an outing a day and one evening kept back still feel like a visit.",
];

pub(super) const NO_LENGTH: &[&str] = &[
    "You have not said how long they are staying, and that is what decides it: a weekend is one outing, a week is three or four. How long is it?",
    "How long is the stay? A weekend wants one thing planned and a week wants two or three, so the answer changes with it.",
    "How long are they with you for? Tell me and I will say how much to plan - the bed and the meal times are the same either way.",
];
