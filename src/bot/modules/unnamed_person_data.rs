//! Source patterns and wording from the pinned PyBot reader.
use fancy_regex::Regex;
use std::sync::LazyLock;
pub(super) const WHO_IS_IT: &[&str] = &[
    "Who do you mean?",
    "Who's that, then?",
    "Who are we talking about?",
];
pub(super) static A_THIRD_PARTY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:he|she|him|her|his|hers|they|them|their|theirs)\b"##)
        .expect("A_THIRD_PARTY")
});
pub(super) static SOMEBODY_IN_THE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:i|i'm|im|me|my|mine|myself|we|us|our|ours|you|your|yours|yourself|yourselves|someone|somebody|anyone|anybody|everyone|everybody|people|nobody|no one)\b"##).expect("SOMEBODY_IN_THE_LINE")
});
pub(super) static NAMES_THEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:my|our|the|this|that|these|those)\s+[a-z][a-z'-]{1,}"##)
        .expect("NAMES_THEM")
});
pub(super) static A_FACT_ABOUT_THEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow (?:old|tall|big|heavy|far|long)\b|\b(?:what|whats|what's)\b[^?]*\b(?:name|age|job|birthday|number|address)\b|\bwhere (?:does|do|did|is|was)\b[^?]*\b(?:live|work|come from|from|stay|based)\b|\b(?:is|are|was|were)\s+(?:he|she|they)\b|\bhow (?:is|are|was|were)\s+(?:he|she|they)\b"##).expect("A_FACT_ABOUT_THEM")
});
pub(super) static AN_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)^\s*(?:how|what|whats|what's|where|when|which|why|is|are|was|were|does|do|did|has|have|can|could|would|will)\b"##).expect("AN_ASK")
});
