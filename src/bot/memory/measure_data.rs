//! The exact measures and wording cues from pinned PyBot's memory/measure.py.

pub(super) const LEAD: &str =
    r#"(?i)^(?:(?:and|so|ok|okay|but|then|right|yeah|yes|well|oh|er|um|hmm)\b[\s,]*)+"#;
pub(super) const TAIL: &str = r#"(?i)[\s,]*(?:(?:again|then|exactly|please|thanks|though|for\s+that|for\s+one|for\s+each|per\s+(?:day|week|person|one)|roughly|about|or\s+so|is\s+that|was\s+that|would\s+that\s+be|does\s+that|do\s+i|did\s+i|would\s+i|should\s+i|will\s+i|of\s+that|there)\b[\s,]*)*$"#;
pub(super) const ASKED: &str = r#"(?i)\A(?:\bhow\s+(much|many|often|long|far|hot|deep|big)\b)\z"#;
pub(super) const THING_AFTER: &str = r#"(?i)\bhow\s+(?:much|many|often|long|far|hot|deep|big)\s+(?:of\s+)?(?P<thing>[a-z][a-z\\s'&/-]{0,28})\b"#;
pub(super) const A_CONTAINER: &str = r#"(?i)\b(?:glasses|glass|cups|cup|pints|pint|slices|slice|portions|portion|pieces|piece|sachets|sachet|scoops|scoop|cloves|clove|tins|tin|cans|can|teaspoons|teaspoon|tablespoons|tablespoon|sprays|spray|doses|dose|wipes|wipe|sheets|sheet)\b"#;
pub(super) const A_NEW_CLAUSE: &[&str] = &[
    "and", "are", "as", "because", "but", "if", "is", "it", "or", "plus", "so", "that", "then",
    "there", "these", "they", "this", "was", "were", "when", "which", "while", "with",
];
pub(super) const STOPS_THE_THING: &[&str] = &[
    "a", "after", "an", "and", "are", "at", "before", "but", "by", "can", "could", "did", "do",
    "does", "for", "her", "his", "i", "if", "in", "is", "it", "my", "on", "or", "our", "shall",
    "should", "that", "the", "their", "them", "they", "this", "to", "was", "we", "were", "when",
    "will", "with", "would", "you", "your",
];
pub(super) const THING_NOT_WORTH_SAYING: &[&str] = &["it", "one", "that", "them", "there", "this"];
pub(super) const MEASURES: &[(&str, &str)] = &[
    ("much", r#"(?i)\b\d"#),
    (
        "many",
        r#"(?i)\b\d|\b(?:one|two|three|four|five|six|seven|eight|nine|ten|dozen)\b"#,
    ),
    (
        "often",
        r#"(?i)\b(?:once|twice|thrice|every|each\s+(?:day|week|month|morning|night)|daily|weekly|monthly|hourly|times\s+a|a\s+day|a\s+week|a\s+month|\d\s*(?:-|to)?\s*\d*\s*times)\b"#,
    ),
    (
        "long",
        r#"(?i)\b\d+\s*(?:second|minute|hour|day|week|month|year)s?\b|\b\d+\s*(?:-|to)\s*\d+\s*(?:second|minute|hour|day|week|month|year)s?\b|\bovernight\b|\ball\s+(?:day|night)\b"#,
    ),
    (
        "far",
        r#"(?i)\b\d+\s*(?:mile|km|kilometre|kilometer|metre|meter|yard|foot|feet)s?\b"#,
    ),
    (
        "hot",
        r#"(?i)\b\d{2,3}\s*(?:\u00b0|c\b|f\b|degrees?)|\bgas\s*mark\s*\d"#,
    ),
    (
        "deep",
        r#"(?i)\b\d+\s*(?:cm|centimetre|centimeter|inch|in|mm|metre|meter)s?\b"#,
    ),
    (
        "big",
        r#"(?i)\b\d+\s*(?:cm|centimetre|centimeter|inch|in|mm|litre|liter|gram|kilo)s?\b"#,
    ),
];
