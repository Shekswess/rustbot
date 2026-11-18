//! Recall cues from the pinned a_fact_asked_again module (Apache-2.0).

pattern!(AGAIN, r#"(?i)\bagain\b"#);

pattern!(
    AN_ASK,
    r#"(?i)\b(?:what|whats|what's|when|which|where|who|how much|how many)\b"#
);

pattern!(
    NOT_A_RECALL,
    r#"(?i)\b(?:should|shall|ought|best|better|tips?|advice|recommend\w*|worth|cheapest|easiest|normal|usually|ok|okay|fine|bad|safe|possible|allowed|do i need to|can i|could i|is it)\b"#
);

pattern!(A_TIME, r#"(?i)\bwhen\b|\bwhat (?:day|date|time)\b"#);

pattern!(
    A_THING_TO_AVOID,
    r#"(?i)(?:keep\w* (?:away|off)|stay\w* (?:away|off)|avoid\w*|allerg\w*|cant have|can'?t have|not allowed|off limits|cant eat|can'?t eat)"#
);

pattern!(
    A_TARGET,
    r#"(?i)\b(?:target|goal|aim|trying to get to|trying to reach|hoping to get to|hoping for|working towards|saving for)\b"#
);

pub(super) const DATES: &[&str] = &[
    r#"(?i)\b(?:the\s+)?\d{1,2}(?:st|nd|rd|th)\s+(?:of\s+)?(?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[a-z]*\b"#,
    r#"(?i)\b(?:jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)[a-z]*\s+\d{1,2}(?:st|nd|rd|th)?\b"#,
    r#"(?i)\bnext\s+(?:week|month|year|weekend|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\b"#,
    r#"(?i)\b(?:the\s+)?\d{1,2}(?:st|nd|rd|th)\b"#,
    r#"(?i)\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)\b"#,
    r#"(?i)\b(?:january|february|march|april|may|june|july|august|september|october|november|december)\b"#,
];
pub(super) const PLAN_OPENS: &[&str] = &[
    r#"i want to "#,
    r#"i wanna "#,
    r#"i'm trying to "#,
    r#"im trying to "#,
    r#"i am trying to "#,
    r#"i'm hoping to "#,
    r#"im hoping to "#,
    r#"i am hoping to "#,
    r#"i hope to "#,
    r#"to "#,
];
