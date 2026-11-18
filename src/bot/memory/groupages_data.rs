// Rule and wording data from the pinned source.

pub(super) const GROUP_WORDS: &[&str] = &[
    "kids",
    "children",
    "sons",
    "daughters",
    "boys",
    "girls",
    "parents",
    "grandparents",
    "grandchildren",
    "grandkids",
    "brothers",
    "sisters",
    "twins",
    "cousins",
    "nieces",
    "nephews",
    "aunts",
    "uncles",
    "stepchildren",
    "stepkids",
    "in-laws",
    "flatmates",
    "housemates",
    "neighbours",
];
pattern!(
    GIVES_A_LIST,
    r#"(?i)\b(?:my|our|the|his|her)\s+(?:\w+\s+){0,2}?(?P<word>grandchildren|grandparents|stepchildren|housemates|neighbours|daughters|grandkids|flatmates|children|brothers|stepkids|parents|sisters|cousins|nephews|in-laws|nieces|uncles|girls|twins|aunts|kids|sons|boys)\b(?:\s+(?:are|were|is|was|aged|both))?[,\s]+(?P<ages>\d{1,3}(?![\d.])(?:\s*(?:,|and|&)\s*\d{1,3}(?![\d.])){1,3})(?![\d,.])(?!\s*(?:times|days|weeks|months|hours|minutes|miles|metres|kilometres|km|kilos|kilograms|kg|pounds|quid|per\b|a\s+(?:week|month|year|day))|\s*years(?!\s+old))"#
);
pattern!(
    GIVES_A_LIST_BARE,
    r#"(?i)\b(?P<word>grandchildren|grandparents|stepchildren|housemates|neighbours|daughters|grandkids|flatmates|children|brothers|stepkids|parents|sisters|cousins|nephews|in-laws|nieces|uncles|girls|twins|aunts|kids|sons|boys)\s+(?:are|were|aged|both\s+aged)\s+(?P<ages>\d{1,3}(?![\d.])(?:\s*(?:,|and|&)\s*\d{1,3}(?![\d.])){1,3})(?![\d,.])(?!\s*(?:times|days|weeks|months|hours|minutes|miles|metres|kilometres|km|kilos|kilograms|kg|pounds|quid|per\b|a\s+(?:week|month|year|day))|\s*years(?!\s+old))"#
);
pattern!(
    THEYRE_A_LIST,
    r#"(?i)\b(?:my|our|the)?\s*(?:\w+\s+){0,2}?(?P<word>grandchildren|grandparents|stepchildren|housemates|neighbours|daughters|grandkids|flatmates|children|brothers|stepkids|parents|sisters|cousins|nephews|in-laws|nieces|uncles|girls|twins|aunts|kids|sons|boys)\b[^.!?]{0,20}?\b(?:they'?re|they\s+are|both\s+aged)\s+(?P<ages>\d{1,3}(?![\d.])(?:\s*(?:,|and|&)\s*\d{1,3}(?![\d.])){1,3})(?!\s*(?:times|days|weeks|months|hours|minutes|miles|metres|kilometres|km|kilos|kilograms|kg|pounds|quid|per\b|a\s+(?:week|month|year|day))|\s*years(?!\s+old))"#
);
pattern!(
    ASKS_THE_LIST,
    r#"(?i)\bhow\s+old\s+(?:are|were|is)\s+(?:they|their|(?:\w+\s+){0,2}?(?:grandchildren|grandparents|stepchildren|housemates|neighbours|daughters|grandkids|flatmates|children|brothers|stepkids|parents|sisters|cousins|nephews|in-laws|nieces|uncles|girls|twins|aunts|kids|sons|boys))\b|\b(?:what|whats|what\s+are|what\s+were)\s+(?:are\s+|were\s+)?(?:their|the|my|our)\s+(?:\w+\s+){0,2}?(?:grandchildren|grandparents|stepchildren|housemates|neighbours|daughters|grandkids|flatmates|children|brothers|stepkids|parents|sisters|cousins|nephews|in-laws|nieces|uncles|girls|twins|aunts|kids|sons|boys|ages)\b|\bhow\s+old\s+are\s+(?:my|our|the)\s+(?:\w+\s+){0,2}?(?:grandchildren|grandparents|stepchildren|housemates|neighbours|daughters|grandkids|flatmates|children|brothers|stepkids|parents|sisters|cousins|nephews|in-laws|nieces|uncles|girls|twins|aunts|kids|sons|boys)\b|\b(?:their|the\s+kids'?|my\s+kids'?)\s+ages\b|\bwhat\s+(?:were|are)\s+my\s+(?:\w+\s+){0,2}?(?:grandchildren|grandparents|stepchildren|housemates|neighbours|daughters|grandkids|flatmates|children|brothers|stepkids|parents|sisters|cousins|nephews|in-laws|nieces|uncles|girls|twins|aunts|kids|sons|boys)\s+ages\b"#
);
pattern!(
    ASKS_THE_END_ONE,
    r#"(?i)\bhow\s+old\s+(?:is|was)\s+(?:your\s+|the\s+|my\s+|our\s+)?(?P<which>youngest|oldest|elder|eldest|older|younger|little|big|little\s+one|big\s+one)\b"#
);
pub(super) const SINGULARS: &[(&str, &[&str])] = &[
    ("children", &["child"]),
    ("kids", &["kid", "child"]),
    ("parents", &["parent"]),
    ("people", &["person"]),
    ("men", &["man"]),
    ("women", &["woman"]),
    ("twins", &["twin"]),
    ("boys", &["boy"]),
    ("girls", &["girl"]),
    ("sisters", &["sister"]),
    ("brothers", &["brother"]),
    ("sons", &["son"]),
    ("daughters", &["daughter"]),
    ("grandchildren", &["grandchild"]),
    ("grandparents", &["grandparent"]),
    ("aunts", &["aunt"]),
    ("uncles", &["uncle"]),
    ("nieces", &["niece"]),
    ("nephews", &["nephew"]),
    ("cousins", &["cousin"]),
    ("flatmates", &["flatmate"]),
    ("roommates", &["roommate"]),
    ("friends", &["friend"]),
    ("neighbours", &["neighbour"]),
    ("grandkids", &["grandkid"]),
];
pub(super) const DECIMAL_ZEROES: &[u32] = &[
    0x30, 0x660, 0x6f0, 0x7c0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xbe6, 0xc66, 0xce6, 0xd66,
    0xde6, 0xe50, 0xed0, 0xf20, 0x1040, 0x1090, 0x17e0, 0x1810, 0x1946, 0x19d0, 0x1a80, 0x1a90,
    0x1b50, 0x1bb0, 0x1c40, 0x1c50, 0xa620, 0xa8d0, 0xa900, 0xa9d0, 0xa9f0, 0xaa50, 0xabf0, 0xff10,
    0x104a0, 0x10d30, 0x10d40, 0x11066, 0x110f0, 0x11136, 0x111d0, 0x112f0, 0x11450, 0x114d0,
    0x11650, 0x116c0, 0x116d0, 0x116da, 0x11730, 0x118e0, 0x11950, 0x11bf0, 0x11c50, 0x11d50,
    0x11da0, 0x11f50, 0x16130, 0x16a60, 0x16ac0, 0x16b50, 0x16d70, 0x1ccf0, 0x1d7ce, 0x1d7d8,
    0x1d7e2, 0x1d7ec, 0x1d7f6, 0x1e140, 0x1e2f0, 0x1e4f0, 0x1e5f1, 0x1e950, 0x1fbf0,
];
