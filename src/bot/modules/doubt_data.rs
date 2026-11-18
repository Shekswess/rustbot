//! Authored patterns and reply data from PyBot 7d63c7f.

pub(super) const DOUBTS_THE_ANSWER: &str = r#"(?i)\b(?:that|this|it|you|youre|you're)\s+(?:makes?|made|making|doesnt|doesn't|isnt|isn't|are|arent|aren't)\s+(?:no|not|any)\s+(?:sense|sensible)\b|\b(?:not|never) making (?:any )?sense\b|\b(?:doesnt|doesn't|didnt|didn't|wouldnt|wouldn't|wont|won't)\s+(?:make|makes|made)\s+(?:any\s+)?sense\b|\bare you (?:sure|certain)\b|\byou sure\b|\b(?:that|this|it) (?:cant|can't|cannot|doesnt|doesn't|wont|won't) be right\b|\b(?:thats|that's|this is) (?:nonsense|rubbish|wrong|not right|a load of)\b|\b(?:youre|you're|you are|u r) (?:confusing|confused) me\b|\bi (?:dont|don't) follow\b|\bi (?:dont|don't) get (?:that|this|it)\b"#;

pub(super) const SAYS_THE_ANSWER_IS_WRONG: &str = r#"(?i)\b(?:cant|can't|cannot|doesnt|doesn't|wont|won't) be right\b|\b(?:nonsense|rubbish|not right|wrong|a load of)\b|\bare you (?:sure|certain)\b|\byou sure\b"#;

pub(super) const DOUBT_TAIL: &[&str] = &[
    "all", "at", "god", "haha", "innit", "like", "lol", "love", "mate", "no", "ok", "okay",
    "really", "sorry", "tbh", "then", "tho", "though", "way",
];

pub(super) const UNCLEAR_ANSWER: &[&str] = &[
    "That is on me. Which bit was the muddled one?",
    "Fair enough. Say which part and I will put it another way.",
    "Let me try that again, plainer. Which bit lost you?",
];

pub(super) const DOUBTING: &[&str] = &[
    "Fair enough to check it. Which bit looks wrong, and I'll say where I got it.",
    "Then I may have it wrong. Say what you were expecting and we'll compare the two.",
    "It could be out of date, that is a real risk with these things. Point at the part and I will go through it.",
];

pub(super) const MEANT_LEADS: &[&str] = &[
    "i mean this:",
    "i said:",
    "just this:",
    "let me put it plainer:",
    "same again:",
    "what i meant:",
];

pub(super) const SUBJECT_UNCLEAR: &[&str] = &[
    "I'm not sure which {word} you mean - a word or two about it and I'll answer the right one.",
    "Which {word} do you mean? Tell me a bit more and I'll take it from there.",
    "There's more than one thing '{word}' could mean. What's the one you're asking about?",
];
