//! Presence and opening wording and state guards from PyBot.
use regex::Regex;
use std::sync::LazyLock;
pub(super) const PRESENCE: &[&str] = &[
    "Still here. What's up?",
    "Yep, right here — go on.",
    "I'm here. What's on your mind?",
];
pub(super) const OPENING: &[&str] = &[
    "Of course — go ahead, what's on your mind?",
    "Ask away, I'm listening. What's going on?",
    "Sure. Tell me what's happening and we'll work it out together.",
];
pub(super) const POINTS_AT_IT: &[&str] = &[
    "at it",
    "doing it",
    "like that",
    "like this",
    "no better",
    "not working",
    "the same",
    "the same thing",
    "there",
];
pub(super) static STILL_ONLY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^(?:it|its|it's|it is|they|theyre|they're|they are|that|thats|that's|that is|this|this is|there|theres|there's|he|hes|he's|he is|she|shes|she's|she is)?\s*still\s+(?P<tail>[a-z][a-z' -]*)$"##).unwrap()
});
pub(super) static BUYING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:which one|which (?:\w+ )?(?:is|to|should)|recommend\w*|best|any good|worth (?:buying|getting)|should i (?:buy|get)|need a new|needs a new|thinking of (?:buying|getting)|going to (?:buy|get)|want to (?:buy|get)|shopping for|looking (?:for|at) (?:a|some)|what should i (?:buy|get)|how much (?:do|does|are|is) (?:they|it|these))\b"##).unwrap()
});
