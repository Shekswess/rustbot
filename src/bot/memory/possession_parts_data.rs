// Part recall patterns and perspective from bot/memory/store_replies.py and store_lines.py.
rule!(
    PART_ASK,
    r#"(?i)\b(?:what|which|who|where|when)\b[^?.!]{0,40}?\b(?:do|does|did|is|are|was|were|can)\b|\bhow (?:much|many|old|long|big|far|often|tall|heavy)\b"#
);
rule!(PART_MINE, r#"(?i)\b(?:my|our)\s+([a-z']+)"#);
rule!(
    PART_POINTS,
    r#"(?i)\b(?:it|its|he|she|they|them|their|there|his|her)\b"#
);
rule!(
    RECORD_PART,
    r#"(?i)\b(?:earn|earns|earning|paid|pay|wage|wages|salary|hours|shifts?|boss|manager|colleagues?|coworkers?|uniform|(?:do|doing|did) there|work with|working with|works with|work for|school|nursery|childminder|called|names?|named|model|make|year|colour|color|reg|brand|mileage|breed)\b"#
);
rule!(
    PART_ADVICE,
    r#"(?i)\b(?:should|shall|recommend|suggest|best|better|worth|instead|advice|ideas?|how do i|how can i|can i|could i|would you)\b"#
);
rule!(
    PART_HELP,
    r#"(?i)\b(?:what|how) (?:do|can|should|could|would) i do (?:about|with|for|to|next|now|then)\b"#
);
rule!(PART_OWNS, r#"\b(?:car|bike|laptop|phone|van)\b"#);
const TURNED: &[(&str, &str)] = &[
    ("i", "you"),
    ("im", "you are"),
    ("ive", "you've"),
    ("me", "you"),
    ("my", "your"),
    ("our", "your"),
    ("us", "you"),
    ("we", "you"),
];
const PART_HELD: &[(&str, &str)] = &[
    ("age", "age"),
    ("area", "city"),
    ("bike", "owns"),
    ("boss", "job"),
    ("brother", "people"),
    ("budget", "budget"),
    ("car", "owns"),
    ("cat", "pets"),
    ("child", "kids"),
    ("children", "kids"),
    ("colleagues", "job"),
    ("dad", "people"),
    ("daughter", "people"),
    ("diet", "diet"),
    ("dog", "pets"),
    ("family", "people"),
    ("father", "people"),
    ("flat", "city"),
    ("garden", "city"),
    ("home", "city"),
    ("hours", "job"),
    ("house", "city"),
    ("husband", "people"),
    ("job", "job"),
    ("jobs", "job"),
    ("kid", "kids"),
    ("kids", "kids"),
    ("kitten", "pets"),
    ("laptop", "owns"),
    ("mother", "people"),
    ("mum", "people"),
    ("name", "name"),
    ("parents", "people"),
    ("partner", "people"),
    ("pay", "job"),
    ("pets", "pets"),
    ("phone", "owns"),
    ("puppy", "pets"),
    ("role", "job"),
    ("salary", "job"),
    ("shift", "job"),
    ("shifts", "job"),
    ("sister", "people"),
    ("son", "people"),
    ("town", "city"),
    ("wife", "people"),
    ("work", "job"),
];
rule!(
    CHAT_TAIL,
    r#"(?:[\s,]*\b(?:of|in|from|for|to)\s+(?:this|our|the)\s+(?:chat|conversation|talk|session|thread|conversation so far)\b(?:\s+(?:so far|today|here|with you|we have had))?)+"#
);
