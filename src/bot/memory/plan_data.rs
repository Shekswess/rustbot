// Sequence wording, cue patterns, and exclusions from the pinned plan reader.

pattern!(
    CARRY_ON,
    r#"(?i)^(?:(?:ok|okay|right|so|yeah|yes|alright|and|then|well|ah)\s+)*(?:and\s+then|then\s+what|what\s+next|what\s+now|next|and\s+after\s+that|after\s+that|and\s+next|go\s+on|carry\s+on|keep\s+going)\s*[.?!]*$"#
);
pub(super) const FIRST_LEADS: &[&str] = &[
    "Start with this: {step}.",
    "Begin here: {step}.",
    "The thing to do first: {step}.",
    "Do this bit first: {step}.",
];
pub(super) const AMOUNTS_LEADS: &[&str] = &[
    "First up: {step}.",
    "Begin with the mix: {step}.",
    "Start with what goes in: {step}.",
];
pattern!(
    OPENS_ON_THE_INSTRUCTION,
    r#"(?i)^(?:start(?:\s+(?:with|by|off))?|begin(?:\s+(?:with|by))?|first(?:ly)?|to\s+start|the\s+first\s+thing)\b(?:[,:]|\s)"#
);
pub(super) const LEADS: &[&str] = &[
    "Next: {step}.",
    "Then: {step}.",
    "The next bit: {step}.",
    "Once that is done: {step}.",
    "The step after that: {step}.",
];
pub(super) const DONE: &[&str] = &[
    "There is nothing after that bit that I can pick out. The last of it is: {step}. Where have you got to?",
    "That is the last step I can pick out of it: {step}. Say where you are and I'll help with the rest.",
    "I have nothing more to hand you after: {step}. Tell me which bit you are on.",
];
pub(super) const FINISHED: &[&str] = &[
    "That is every step I can pick out of it - nothing left of what I can hand you. Tell me how it went.",
    "There is nothing after that I can pick out. Say what happened and we will go from there.",
    "Nothing more I can pull out of that answer. How did it go?",
];
pub(super) const NOT_A_JOB: &[&str] = &["loss", "grief", "safety"];
pub(super) const NOT_THE_THING: &[&str] = &[
    "about", "after", "again", "also", "away", "back", "been", "before", "being", "done", "each",
    "every", "from", "have", "here", "into", "just", "like", "make", "once", "over", "some",
    "take", "than", "that", "them", "then", "there", "they", "this", "under", "what", "when",
    "while", "will", "with", "your",
];
pattern!(
    HAD_A_GO,
    r#"(?i)\b(?:tried|trying|already|gave it|had a go|no luck|used)\b"#
);
pattern!(
    AMOUNTS,
    r#"(?i)\b\d+(?:\.\d+)?\s*(?:g|kg|ml|l|oz|lb|tsp|tbsp|teaspoons?|tablespoons?)\b"#
);
pub(super) const WHILE_IT_LASTS: usize = 3;
pub(super) const COUNTS: &[(usize, &str)] = &[
    (2, "I make it two steps."),
    (3, "I make it three steps."),
    (4, "I make it four steps."),
    (5, "I make it five steps."),
];
