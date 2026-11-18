//! Instruction-opening vocabulary used by the upstream verdict and memory readers.
use regex::Regex;
use std::sync::LazyLock;
static FIRST_ITSELF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)\b(?:start|begin|first)\b"#).unwrap());
static COPULA: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)^(?:[\w,'-]+\s+){0,5}(?:is|are|was|were)\b"#).unwrap());
static CONDITIONAL_OPEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^(?:if|unless|once|when|whenever|should|in case)\b"#).unwrap()
});
static A_STATEMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^(?:[a-z']+\s+)(?:is|are|was|were|has|have|had|can|cannot|will|would|should|takes?|gives?|means?|costs?|comes?|goes?|does|did|gets?|keeps?)\b"#).unwrap()
});
static QUESTION_FRAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^(?:(?:so|and|but|or|ok|okay|right|well|then|now|look|listen|go on|go ahead|come on|tell me)\b[\s,\u2014\u2013.:;-]*)*(?:(?:do|did|does|would|will|can|could|shall|should|have|has|are|is|was|were)\s+(?:you|i|we|they|he|she|it|there|that|this|anyone|anybody|someone)\b|(?:what|why|how|where|who|which|whose)\b)"#).unwrap()
});
static RED_FLAG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^(?:get (?:it|that|one|them|him|her) (?:looked at|checked|seen)|have (?:it|that|one) (?:looked at|checked)|ring (?:111|999|911|nhs 24)|call (?:111|999|911)|go (?:to|into) (?:a&e|a and e|the hospital|hospital|the emergency department)|see (?:a|your|the) (?:gp|doctor|nurse|pharmacist|midwife)|speak to (?:a|your|the) (?:gp|doctor)|if (?:it|that|they) (?:is |are )?(?:still|gets? worse|does not settle|doesnt settle|has not settled|had not settled|keeps? (?:on|coming)))"#).unwrap()
});
const WHEN: &[&str] = &[
    "a",
    "about",
    "afternoon",
    "an",
    "around",
    "couple",
    "daily",
    "day",
    "each",
    "evening",
    "every",
    "five",
    "four",
    "hour",
    "minute",
    "month",
    "morning",
    "night",
    "nightly",
    "once",
    "per",
    "roughly",
    "second",
    "seven",
    "six",
    "three",
    "time",
    "times",
    "twice",
    "two",
    "week",
    "year",
];
const IMPERATIVES: &[&str] = &[
    "add",
    "agree",
    "aim",
    "air",
    "answer",
    "apply",
    "ask",
    "avoid",
    "bake",
    "balance",
    "bathe",
    "be",
    "begin",
    "bin",
    "bleed",
    "blend",
    "blot",
    "boil",
    "book",
    "bring",
    "brush",
    "budget",
    "build",
    "buy",
    "call",
    "cancel",
    "change",
    "check",
    "chill",
    "choose",
    "chop",
    "clean",
    "clear",
    "close",
    "compare",
    "cook",
    "cool",
    "cover",
    "cut",
    "dab",
    "dig",
    "do",
    "drain",
    "draw",
    "drink",
    "dry",
    "ease",
    "eat",
    "email",
    "empty",
    "feed",
    "fill",
    "find",
    "fit",
    "fix",
    "flush",
    "fold",
    "follow",
    "freeze",
    "get",
    "give",
    "go",
    "grate",
    "grill",
    "halve",
    "hang",
    "heat",
    "hold",
    "hoover",
    "ice",
    "install",
    "iron",
    "keep",
    "label",
    "lay",
    "learn",
    "leave",
    "let",
    "line",
    "log",
    "look",
    "loosen",
    "make",
    "mash",
    "measure",
    "melt",
    "mend",
    "mix",
    "move",
    "offer",
    "open",
    "order",
    "pack",
    "pat",
    "pay",
    "peel",
    "phone",
    "photograph",
    "pick",
    "place",
    "plant",
    "plug",
    "pour",
    "practice",
    "practise",
    "press",
    "prime",
    "print",
    "prop",
    "prune",
    "pull",
    "push",
    "put",
    "raise",
    "read",
    "recite",
    "rehearse",
    "reheat",
    "remove",
    "rent",
    "repair",
    "replace",
    "reply",
    "report",
    "repot",
    "reset",
    "rest",
    "restart",
    "ring",
    "rinse",
    "roast",
    "roll",
    "rub",
    "run",
    "save",
    "scan",
    "scour",
    "scrape",
    "screw",
    "scrub",
    "seal",
    "sell",
    "send",
    "separate",
    "serve",
    "set",
    "settle",
    "shop",
    "shred",
    "shut",
    "simmer",
    "sit",
    "skip",
    "slice",
    "soak",
    "sort",
    "spend",
    "split",
    "sponge",
    "spray",
    "stand",
    "start",
    "stay",
    "stir",
    "stop",
    "store",
    "stretch",
    "swaddle",
    "swap",
    "switch",
    "take",
    "tell",
    "test",
    "text",
    "throw",
    "tidy",
    "tighten",
    "top",
    "track",
    "treat",
    "trim",
    "try",
    "tuck",
    "turn",
    "unplug",
    "use",
    "vacuum",
    "walk",
    "wash",
    "watch",
    "water",
    "wear",
    "weigh",
    "wipe",
    "work",
    "wrap",
    "write",
];

fn imperative(sentence: &str) -> bool {
    static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Za-z'’-]+").unwrap());
    let mut words = WORD
        .find_iter(sentence)
        .take(5)
        .map(|w| w.as_str().trim_matches(['-', '–', '—']).to_lowercase())
        .peekable();
    if words
        .peek()
        .is_some_and(|w| matches!(w.as_str(), "the" | "a" | "an"))
    {
        return false;
    }
    words
        .find(|w| WHEN.binary_search(&w.as_str()).is_err())
        .is_some_and(|w| IMPERATIVES.binary_search(&w.as_str()).is_ok())
}

pub(crate) fn first_instruction(answer: &str) -> Option<&str> {
    instructions(answer).next()
}

pub(crate) fn instructions(answer: &str) -> impl Iterator<Item = &str> {
    static SENTENCES: LazyLock<fancy_regex::Regex> =
        LazyLock::new(|| fancy_regex::Regex::new(r"(?<=[.!?]) ").unwrap());
    SENTENCES.split(answer).flatten().filter_map(|sentence| {
        let text = sentence.trim();
        if text.is_empty()
            || CONDITIONAL_OPEN.is_match(text)
            || RED_FLAG.is_match(text)
            || A_STATEMENT.is_match(text)
        {
            return None;
        }
        if text.ends_with('?')
            && (QUESTION_FRAME.is_match(text)
                || text
                    .rsplit_once(", ")
                    .is_some_and(|(_, t)| QUESTION_FRAME.is_match(t)))
        {
            return None;
        }
        if imperative(text)
            || text
                .split_once(": ")
                .is_some_and(|(_, tail)| imperative(tail))
            || FIRST_ITSELF.is_match(text)
                && !COPULA.is_match(text)
                && text.split_whitespace().count() >= 5
        {
            return Some(text.trim_matches([' ', '.', '!', '?']));
        }
        None
    })
}
