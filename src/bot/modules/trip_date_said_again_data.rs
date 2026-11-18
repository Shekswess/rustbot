//! Source phrases that move a trip date (Apache-2.0).

pattern!(
    A_DATE_SAID_NOW,
    r#"(?i)^\W*(?:(?:ok|okay|right|well|so|and|no|nope|sorry|actually|oops|wait|ah|oh|hmm)[,.]?\s+){0,4}(?:(?:it'?s|its|it\s+is|that'?s|thats)\s+)?((?:the\s+)?\d{1,2}(?:st|nd|rd|th)?(?:\s+of\s+(?:the\s+)?[a-z]{3,10})?|(?:the\s+)?\d{1,2}(?:st|nd|rd|th)?\s+[a-z]{3,10}|the\s+[a-z]{3,10}|[a-z]{3,10})\s+now\b"#
);

pattern!(
    A_MARKER_SAID,
    r#"(?i)\b(?:no|nope|sorry|actually|oops|wait|ah|oh|hmm)\b"#
);

pattern!(
    TRIP_DATE_MOVED,
    r#"(?i)^\W*(?:(?:ok|okay|right|well|so|and)[,.]?\s+)?(?:(?:no|nope|sorry|actually|oops|wait|ah|oh|hmm)[,.]?\s+){1,3}(?:(?:we|were|i|im|they|theyre|he|hes|she|shes)\s*(?:'re|'ve|'ll|'m|'s|\s+are|\s+have|\s+had|\s+will)?\s+)?(?:fly|flies|flying|go|goes|going|leave|leaves|leaving|head|heading|travel|travelling|traveling|drive|driving|sail|sailing|arrive|arriving)\b[^.!?]{0,40}?\b(?:in|on|for)\s+((?:the\s+)?\d{1,2}(?:st|nd|rd|th)?(?:\s+of\s+(?:the\s+)?[a-z]{3,10})?|(?:the\s+)?\d{1,2}(?:st|nd|rd|th)?\s+[a-z]{3,10}|the\s+[a-z]{3,10}|[a-z]{3,10})(?:\s+(?:now|instead))?\s*[.!]?\s*$"#
);

pub(super) const A_DATE_WORD: &[&str] = &[
    r#"apr"#,
    r#"april"#,
    r#"aug"#,
    r#"august"#,
    r#"dec"#,
    r#"december"#,
    r#"feb"#,
    r#"february"#,
    r#"friday"#,
    r#"jan"#,
    r#"january"#,
    r#"jul"#,
    r#"july"#,
    r#"jun"#,
    r#"june"#,
    r#"mar"#,
    r#"march"#,
    r#"may"#,
    r#"monday"#,
    r#"nov"#,
    r#"november"#,
    r#"oct"#,
    r#"october"#,
    r#"saturday"#,
    r#"sep"#,
    r#"sept"#,
    r#"september"#,
    r#"sunday"#,
    r#"thursday"#,
    r#"tuesday"#,
    r#"wednesday"#,
    r#"weekend"#,
];
