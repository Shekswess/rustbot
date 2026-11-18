// Exact age question patterns and family aliases from the pinned facts and aboutthem readers.

pattern!(
    AHEAD_ASK,
    r#"(?i)\b(?:how\s+old|what\s+age)\s+(?:will|would|is|does)\b"#
);
pattern!(
    AHEAD_TIME,
    r#"(?i)\bnext\s+(?P<next>year|summer|birthday)\b|\bthis\s+time\s+next\s+year\b|\bin\s+(?P<years>\d{1,2}|a|one|two|three|four|five|six|seven|eight|nine|ten)\s+years?\b|\bin\s+(?P<year>(?:19|20)\d\d)\b"#
);
pattern!(
    KIN,
    r#"(?i)\bmy\s+(?:little\s+|big\s+|eldest\s+|youngest\s+)?(?P<who>son|daughter|boy|girl|child|kid|lad|little one)\b"#
);
pattern!(
    PRONOUN,
    r#"(?i)\b(?P<own>his|her|their)\s+(?P<thing>[a-z][a-z' -]{0,20})|\b(?P<who>he|she|they)\b|\b(?P<obj>him|her|them)\b"#
);
pub(super) const HE: &[&str] = &[
    "boss",
    "boy",
    "boyfriend",
    "brother",
    "brother in law",
    "brother in laws",
    "colleague",
    "cousin",
    "dad",
    "father",
    "father in law",
    "father in laws",
    "fiance",
    "flatmate",
    "friend",
    "godson",
    "grandad",
    "granddad",
    "grandfather",
    "grandpa",
    "grandson",
    "housemate",
    "husband",
    "landlady",
    "landlord",
    "lodger",
    "manager",
    "mate",
    "neighbor",
    "neighbour",
    "nephew",
    "partner",
    "roommate",
    "son",
    "son in law",
    "stepbrother",
    "stepdad",
    "stepfather",
    "stepson",
    "uncle",
];
pub(super) const SHE: &[&str] = &[
    "aunt",
    "auntie",
    "aunty",
    "boss",
    "colleague",
    "cousin",
    "daughter",
    "daughter in law",
    "fiancee",
    "flatmate",
    "friend",
    "girl",
    "girlfriend",
    "goddaughter",
    "gran",
    "granddaughter",
    "grandma",
    "grandmom",
    "grandmother",
    "grandmum",
    "granny",
    "housemate",
    "landlady",
    "landlord",
    "lodger",
    "manager",
    "mate",
    "mom",
    "mother",
    "mother in law",
    "mother in laws",
    "mum",
    "nan",
    "nana",
    "nanna",
    "neighbor",
    "neighbour",
    "niece",
    "partner",
    "roommate",
    "sister",
    "sister in law",
    "sister in laws",
    "stepdaughter",
    "stepmom",
    "stepmother",
    "stepmum",
    "stepsister",
    "wife",
];
pub(super) const CHILD_AGE: u8 = 18;
pub(super) const SPOKEN: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
];
pub(super) const KIN_SAME: &[(&str, &[&str])] = &[
    ("son", &["son", "boy"]),
    ("boy", &["boy", "son"]),
    ("lad", &["lad", "boy", "son"]),
    ("daughter", &["daughter", "girl"]),
    ("girl", &["girl", "daughter"]),
    ("child", &["child", "kid", "son", "daughter", "boy", "girl"]),
    ("kid", &["kid", "child", "son", "daughter", "boy", "girl"]),
    (
        "little one",
        &[
            "little one",
            "child",
            "kid",
            "son",
            "daughter",
            "boy",
            "girl",
        ],
    ),
];

pattern!(
    THEIR_AGE,
    r#"(?i)\bhow\s+old\b|\bwhat\s+age\b|\bhow\s+big\b|\btheir\s+age\b|\bhis\s+age\b|\bher\s+age\b"#
);
pattern!(
    ASKS,
    r#"(?i)\bhow\s+(?:much|many|long|often|early|late)\b|\bwhat\s+time\b|\bwhen\s+(?:is|are|do|does|will|can)\b"#
);
pattern!(
    GIVEN,
    r#"(?i)\b\d{1,3}\b|\ba\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen)\b"#
);
pub(super) const RETURN_ASKS: &[&str] = &[
    "am", "are", "can", "could", "did", "do", "does", "had", "has", "have", "how", "hows", "if",
    "is", "shall", "should", "was", "were", "what", "whats", "when", "whens", "where", "wheres",
    "whether", "which", "whichs", "who", "whom", "whos", "whose", "why", "whys", "will", "would",
];
pub(super) const BABY_MONTHS: u8 = 24;
