//! Recurrence-step rules from PyBot memory/returned.py at 7d63c7f.
pub(super) const NEXT_STEP: &str = r####"(?i)\b(?:if|when|once)\s+(?:it|that|this|they|things)\s+(?:keeps?|still|comes? back|come back|returns?|happens? again|does it again|doesn'?t|does not|hasn'?t|has not|has been|have been|lasts?|persists?|keeps up|won'?t|will not|is no better|isn'?t better|isn'?t clearing|is not clearing|gets worse)"####;
pub(super) const LEADING_CONDITION: &str =
    r####"(?i)^(?:if|when|once)\s[^,.;:]{0,80},\s*(?=[A-Za-z])"####;
pub(super) const CONDITIONAL_ASK: &str = r####"(?i)^(?:and|but|so|then)?\s*(?:what\s+)?if\s+(?:it|that|this|they|theyre|it's|things|the\s+\w+)\s+(?:does\s?n[o']?t|does not|did\s?n[o']?t|did not|is\s?n[o']?t|is not|has\s?n[o']?t|has not|have\s?n[o']?t|gets?|stays?|keeps?|lasts?|still|stops?|eases?|no better|not better|not clearing)\b"####;
pub(super) const NOT_AN_ANSWER: &[&str] = &[
    r####"safety"####,
    r####"urgent"####,
    r####"grief"####,
    r####"loss"####,
];
pub(super) const CONTINUES: &[&str] = &[
    r####"or"####,
    r####"and"####,
    r####"keeps"####,
    r####"keep"####,
    r####"kept"####,
    r####"comes"####,
    r####"come"####,
    r####"still"####,
    r####"lasts"####,
    r####"persists"####,
    r####"gets"####,
    r####"goes"####,
    r####"happens"####,
    r####"does"####,
    r####"hasn't"####,
    r####"has not"####,
    r####"isn't"####,
    r####"is not"####,
    r####"won't"####,
    r####"will not"####,
    r####"stops"####,
    r####"swells"####,
    r####"spreads"####,
    r####"bleeds"####,
    r####"hurts"####,
    r####"wakes"####,
];
