//! Patterns and phrase pools from the pinned will_it_work reader.
pattern!(
    WORK_CHECK,
    r##"(?i)^(?:(?:and|but|so|ok(?:ay)?|right|oh|hmm|well|then)[, ]+)*(?:will|would|does|do|might|should|can)\s+(?:that|it|this|they|those)\s+(?:work|help|do\s+(?:it|the\s+job)|fix\s+(?:it|that)|be\s+(?:enough|ok|okay|all\s+right|alright|fine)|make\s+a\s+difference)(?:\s+(?:for|on)\s+(?:me|us|him|her|them|this|that))?(?:\s+(?:then|though|now|too))?\s*$"##
);
pattern!(
    ACHIEVES,
    r##"(?i)\b(?:almost nothing|nothing needs|nothing to scrub|does the job|does the trick|catches|clears|lifts|comes away|comes off|gets rid of|no more|beats|kills|sinks|survives|stops?|fixes|sorts|dries|settles|holds|works|helps?|lets (?:her|him|them|you)|keeps (?:them|it) (?:off|away|out))\b"##
);
pattern!(
    NOT_ACHIEVING,
    r##"(?i)\b(?:does nothing|nothing except|does not|doesn't|dont|don't|never|won't|wont|isn't|is not|not\s+enough|makes it worse)\b"##
);
pub(super) const JOINS_ON: &[&str] = &[
    "also", "and", "as", "because", "but", "if", "so", "that", "then", "though", "when", "which",
    "while", "who",
];
pub(super) const YES_IN_FRONT: &[&str] = &["Yes", "It does", "That does it", "Yes, it does"];
pub(super) const TRAILING_FILLER: &[&str] = &["too", "as well", "also", "then", "though"];
