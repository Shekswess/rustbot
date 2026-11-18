// Patterns, vocabulary, and perspective from bot/memory/decided.py.
rule!(
    LEAD_IN,
    r#"(?i)^(?:ok(?:ay)?|so|right|well|then|hmm|oh|and|but)[, ]+"#
);
rule!(
    OPTIONS,
    r#"(?i)\bor\b|\bbetween\b|\btorn\b|\bthinking of\b|\bthinking about\b|\bweighing up\b|\bshould i\b|\beither\b|\bwhich one\b|\bdecid(?:e|ed|ing)\b|cannot decide|can'?t decide|\bchoose\b|\bchoices?\b|\boptions\b|\bhelp me pick\b|\bvs\b"#
);
rule!(
    CLAUSE_CHOICE,
    r#"(?i)^(?:actually|instead|in the end|turns out|it turns out)[,]?\s+(?:i|we)(?:'ve|ve|'d|d)?\s+(?:have\s+|had\s+|just\s+)?(?:booked|bought|got|ordered|chosen|chose|picked|reserved|taken|took|signed up for|settled on|decided on|gone with|went with|landed on)\s+(?P<what>.+)$"#
);
rule!(
    CORRECTING,
    r#"(?i)^(?:no|nah|nope|i mean|i meant|wait|sorry)[, ]+"#
);
rule!(
    TAIL,
    r#"(?i)\s+(?:instead|after all|i think|i reckon|i suppose|then|actually|last night|last week|yesterday|today|this morning)$"#
);
rule!(
    ASK,
    r#"\bwhat (?:did|do|have|had) (?:i|we) (?:decide|decided|settle(?:d)? on|choose|chose|pick|picked|go for|went for|end up with)\b|\bwhat (?:am|are) (?:i|we) (?:getting|going with|going for|ending up with|having)\b|\bwhich (?:one|of (?:them|those))? (?:did (?:i|we) (?:pick|choose|go for|decide|settle on|go with|end up with)|(?:am|are) (?:i|we) (?:getting|going with))\b|\bwhich (?:one|of (?:them|those)) (?:i|we) (?:picked|chose|went with|went for|settled on|decided on)\b|\bwhere did (?:i|we) (?:decide|settle) (?:to go|on)\b|\bso what (?:am|are) (?:i|we) (?:getting|going with|doing)\b|\bwhat (?:i|we) (?:decided|chose|picked|settled on|went with|went for)\b|\b(?:have|has|had|did) (?:i|we) (?:decide|decided|picked|chosen|chose|settled)\b|\bare (?:we|you) decided\b|\bwhat was (?:decided|chosen|picked|settled on)\b|\bwhat (?:did|do) (?:i|we) (?:land|opt) (?:on|for)\b|\bwhich did (?:i|we) (?:go for|pick|choose|decide on|settle on)\b|\bwhat did (?:i|we) say (?:i|we)(?:'d| would) (?:get|choose|pick|go for|have)\b|\bremind me (?:of|about) (?:the|our) choice\b|\bdid (?:i|we) (?:go with|go for|settle on|pick|choose|decide on|end up with)\b"#
);
rule!(
    GUESSING_ASK,
    r#"(?i)\bdid (?:i|we) (?:go with|go for|settle on|pick|choose|decide on|end up with)\s+(?P<what>[^?.!,]+)"#
);
rule!(
    INNER_DECIDE,
    r#"(?i)^(?:i|we)(?:'ve|'d|ve|d| have| had)?\s+(?:decided|settled|chosen|chose|picked|gone|went|going|go)\b\s*(?:on|with|for)?\s*"#
);
rule!(
    REACHES_FOR_A_CHOICE,
    r#"(?i)\b(?:go|goes|going|went)\s+(?:with|for)\b|\b(?:settled|decided|landed|ended up)\s+(?:on|for|with)\b|\b(?:(?:going|gonna)\s+to\s+)?(?:take|taking|took|book|booking|pick|picked|choose|chose|choosing)\s+(?:the|a|an|my|that|this)\b|\b(?:picked|chose|choosing)\b"#
);
rule!(
    OPENING_QUESTION,
    r#"(?i)^(?:is|are|was|were|should|shall|do|does|did|can|could|would|will|what|which|how|where|when|why|any|if)\b"#
);
rule!(
    DECIDING_WORD,
    r#"(?i)\b(?:decid|settl|pick|cho(?:o|s)|decided|deciding)\w*"#
);
rule!(
    COMPANION,
    r#"\b(?:drink|drinking|drinks|has|have|having|order|ordering|orders|pair|pairing|pairs|pour|pouring|pours|serve|serves|serving)\s+(?:with|alongside)\b"#
);
rule!(
    SETTLING_0,
    r#"^(?:(?:actually|instead|in the end|in that case|on second thoughts?|on second thought)[,]?\s+)(?P<what>.+)$"#
);
rule!(
    SETTLING_1,
    r#"\b(?:i|we|i've|ive|we've|weve)\s+(?:have\s+|had\s+)?(?:decided|settled)\s+(?:on|for)?\s*(?P<what>.+)$"#
);
rule!(
    SETTLING_2,
    r#"\b(?:i|we)\s+(?:picked|chose|landed on|ended up with)\s+(?P<what>.+)$"#
);
rule!(
    SETTLING_3,
    r#"\b(?:i|we)\s+(?:go|went|goes)\s+with\s+(?P<what>.+)$"#
);
const SETTLING: &[&LazyLock<Regex>] = &[&SETTLING_0, &SETTLING_1, &SETTLING_2, &SETTLING_3];
rule!(
    GOING_0,
    r#"\b(?:i'?m|im|i am|we'?re|were)\s+(?:going|gonna)\s+(?:with|for)\s+(?P<what>.+)$"#
);
rule!(
    GOING_1,
    r#"\b(?:i'?m|im|i am|we'?re|were)\s+going to\s+go\s+(?:with|for)\s+(?P<what>.+)$"#
);
const GOING: &[&LazyLock<Regex>] = &[&GOING_0, &GOING_1];
rule!(
    CORRECTING_LINE_0,
    r#"^(?:no|nah|nope)[,:]\s*(?:i mean(?:t)?[, ]+)?(?P<what>.+)$"#
);
rule!(
    CORRECTING_LINE_1,
    r#"^(?:no|nah|nope)\s+i mean(?:t)?[, ]+(?P<what>.+)$"#
);
const CORRECTING_LINE: &[&LazyLock<Regex>] = &[&CORRECTING_LINE_0, &CORRECTING_LINE_1];
rule!(
    CHOOSING_0,
    r#"\b(?:lets|let's|i'?ll|ill|we'?ll|well|i'?m going to|im going to|i'?m gonna|im gonna|we'?re going to|were going to)\s+(?:go|going|get|getting|have|having|take|taking|do|doing|pick|picking|choose|choosing|book|booking|buy|buying)\s+(?:with|for)?\s*(?P<what>.+)$"#
);
rule!(CHOOSING_1, r#"\b(?:i|we)\s+went for\s+(?P<what>.+)$"#);
rule!(
    CHOOSING_2,
    r#"^(?P<what>.{2,40}?),?\s+i (?:think|reckon|suppose)$"#
);
rule!(
    CHOOSING_3,
    r#"^(?P<what>(?:a|an|the|some)\s+[\w'’-]+(?:\s+[\w'’-]+)?)$"#
);
rule!(CHOOSING_4, r#"\bmake it\s+(?P<what>.+)$"#);
rule!(
    CHOOSING_5,
    r#"\b(?:lets|let's|i'?ll|ill|we'?ll|well|i will|we will)\s+(?:take|do|have)\s+(?P<what>.+)$"#
);
rule!(CHOOSING_6, r#"\b(?:i|we)\s+said\s+(?P<what>.+)$"#);
const CHOOSING: &[&LazyLock<Regex>] = &[
    &CHOOSING_0,
    &CHOOSING_1,
    &CHOOSING_2,
    &CHOOSING_3,
    &CHOOSING_4,
    &CHOOSING_5,
    &CHOOSING_6,
];
const ACTIVITY: &[&str] = &[
    "bath",
    "break",
    "breakfast",
    "coffee",
    "curry",
    "cycle",
    "dinner",
    "drink",
    "drive",
    "fag",
    "haircut",
    "hike",
    "jog",
    "kip",
    "leak",
    "look",
    "lunch",
    "massage",
    "nap",
    "pee",
    "pint",
    "rest",
    "ride",
    "run",
    "shop",
    "shopping",
    "shower",
    "sit",
    "sleep",
    "smoke",
    "stroll",
    "supper",
    "swim",
    "walk",
    "wander",
    "wee",
];
const NOT_A_THING: &[&str] = &[
    "am", "are", "did", "didn't", "do", "does", "doesnt", "don't", "dont", "he", "her", "here",
    "his", "i", "i'm", "im", "is", "it", "it's", "its", "me", "my", "our", "she", "that", "their",
    "there", "they", "this", "was", "we", "we're", "were", "what", "which", "who", "you", "you're",
    "your",
];
const IDIOM: &[&str] = &[
    "better",
    "bigger",
    "brief",
    "chance",
    "cheaper",
    "clear",
    "count",
    "doubt",
    "easy",
    "fast",
    "fun",
    "happen",
    "idea",
    "it",
    "louder",
    "money",
    "nice",
    "now",
    "official",
    "problem",
    "quick",
    "quieter",
    "right",
    "sense",
    "short",
    "smaller",
    "snappy",
    "soon",
    "thank you",
    "thanks",
    "that",
    "the crowd",
    "the flow",
    "the grain",
    "the rest",
    "them",
    "this",
    "up",
    "way",
    "work",
    "worries",
    "worse",
    "worth",
];
const OPTION_FILLER: &[&str] = &[
    "about", "also", "am", "and", "are", "been", "being", "best", "better", "between", "can't",
    "cannot", "cant", "choose", "choosing", "decide", "decided", "deciding", "did", "do", "does",
    "get", "gets", "getting", "go", "goes", "going", "gonna", "got", "help", "i", "if", "im", "is",
    "it", "just", "like", "maybe", "me", "need", "needs", "of", "pick", "picking", "prefer",
    "rather", "shall", "should", "so", "still", "then", "they", "thinking", "thought", "to",
    "torn", "unsure", "up", "want", "wanted", "wants", "was", "we", "weighing", "well", "were",
    "whether", "worse", "worth", "you",
];
const ASK_FRAME: &[&str] = &[
    "a", "about", "again", "am", "an", "and", "are", "choice", "choose", "chose", "chosen",
    "decide", "decided", "deciding", "did", "do", "does", "end", "ended", "for", "get", "gets",
    "getting", "go", "going", "got", "had", "has", "have", "having", "how", "i", "it", "kind",
    "land", "landed", "long", "many", "me", "much", "my", "of", "on", "one", "ones", "opt",
    "opted", "our", "pick", "picked", "picking", "place", "remember", "remind", "said", "say",
    "settle", "settled", "settling", "should", "so", "sort", "take", "that", "the", "them", "then",
    "there", "this", "those", "time", "to", "took", "up", "us", "was", "we", "went", "were",
    "what", "when", "where", "which", "who", "whose", "why", "will", "with", "would", "you",
];
const NOT_A_SUBJECT: &[&str] = &[
    "again", "also", "best", "better", "ever", "just", "now", "one", "one's", "ones", "please",
    "really", "right", "still", "then", "thing", "things", "way", "worse",
];
const DOING: &[&str] = &[
    "build", "building", "builds", "buy", "buying", "buys", "cook", "cooked", "cooking", "cooks",
    "drink", "drinking", "drinks", "eat", "eating", "eats", "grow", "growing", "grows", "learn",
    "learning", "learns", "made", "make", "makes", "making", "plant", "planting", "plants",
    "serve", "serves", "serving", "studies", "study", "studying", "watch", "watches", "watching",
    "wear", "wearing", "wears",
];
const TO_YOU: &[(&str, &str)] = &[
    ("i", "you"),
    ("i'm", "you're"),
    ("i've", "you've"),
    ("im", "you're"),
    ("ive", "you've"),
    ("me", "you"),
    ("mine", "yours"),
    ("my", "your"),
    ("myself", "yourself"),
    ("our", "your"),
    ("ours", "yours"),
    ("we", "you"),
];
