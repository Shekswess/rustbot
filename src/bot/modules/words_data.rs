//! Authored quotation and wording patterns from PyBot 7d63c7f.

pub(super) const ASKS_WORDS: &str = r#"(?i)\b(?:what|how)\s+(?:should|do|would|shall|can|could|might)\s+i\s+(?:say|text|email|message|write|reply|answer|word|phrase|tell|open|start)\b|\b(?:what|how)\s+(?:should|do|would|shall|can|could|might)\s+i\s+put\s+(?:it|this|that|these|those|them)\b|\b(?:what|how)\s+(?:should|do|would|shall|can|could|might)\s+i\s+put\s+(?:[^?.]{0,30}?\b(?:in|into|on)\s+)?(?:the|my|his|her|their|your|our|a|an)\s+(?:e-?mail|letter|card|note|text|message|msg|reply|answer|application|invitation|invite|whatsapp|bio|post|comment|review|essay|report)\b|\bhow\s+(?:do|should|would|can|could)\s+i\s+(?:word|phrase|put)\s+(?:it|this|that)\b|\bhow\s+(?:do|should|would|can|could)\s+i\s+ask\s+(?:for\s+)?(?:it|that|this|them|him|her|someone|somebody)\b|\b(?:can|could|should)\s+i\s+ask\s+(?:for\s+)?(?:it|that|this|them|him|her)\b(?:\s+(?:back|again))?[\s.?!]*$|\b(?:can|could|would)\s+you\s+(?:word|phrase)\s+(?:it|this|that)\b|\b(?:give|gimme)\s+me\s+the\s+words\b|\bwhat\s+are\s+the\s+words\b|\bwhat(?:'s| is)\s+the\s+wording\b|\b(?:don'?t|do\s+not|dont|no\s+idea|not\s+sure|never\s+know\s+how|can'?t\s+work\s+out)\s+(?:know\s+)?how\s+to\s+(?:tell|say|word|phrase|put|ask|bring|raise|mention|open|start|write|text|email|message)\b|\b(?:don'?t|do\s+not|dont|no\s+idea|not\s+sure)\s+what\s+to\s+(?:say|text|write|reply|message|email|call|tell)\b"#;

pub(super) const EXAMPLED: &str = r#"(?i)(?:\b(?:a|an|the|one|like|such\s+as|e\.g\.))\s*$"#;

pub(super) const ASK_OBJECT: &str = r#"(?i)^(?:(?:it|this|that|them|those)\b|(?:to|for|at|with|in|on)\s+(?:him|her|them|it|this|that|me|us|everyone|somebody|anybody)\b|(?:to|for|at|with|in|on)\s+(?:my|his|her|their|our|your|the|a|an)\s+[a-z']+|(?:him|her|them|everyone|somebody)\b|(?:my|his|her|their|our|your|the)\s+[a-z']+)"#;

pub(super) const ASK_FILLER: &str =
    r#"(?i)^(?:please|then|instead|just|really|up|out|loud|back)\b[,.]?"#;

pub(super) const QUOTED: &str = r#"(?<![A-Za-z0-9])'((?:[^']|'(?=[A-Za-z])){12,300})'(?![A-Za-z0-9])|(?<![A-Za-z0-9])\"((?:[^\"]|\"(?=[A-Za-z])){12,300})\"(?![A-Za-z0-9])|\u201c([^\u201d]{12,300})\u201d|\u2018([^\u2019]{12,300})\u2019"#;

pub(super) const LINES: &[&str] = &[
    "{words} Say that, and leave it there - it gives them an easy way to answer.",
    "{words} That is the whole of it, so don't add a second explanation after it.",
    "{words} Say it once, early, and let the silence do the rest.",
    "{words} Keep to those words and let them reply in their own time.",
    "{words} The short version lands better than a speech, so stop there.",
];
