// Rule and wording data from the pinned source.

pattern!(
    KID_GROUP_AGES_ARE,
    r#"(?i)\b(?P<whose>my|our)\s+(?:(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+)?(?P<who>kids|children|child|kid|sons|daughters|boys|girls)\s+(?:are|is)\s+(?:both\s+)?(?:aged?\s+)?(?P<ages>\d{1,3}(?:\s*(?:,|and)\s*\d{1,3})*)"#
);
pattern!(
    KID_COUNT_AGES,
    r#"(?i)\b(?:i|we)(?:\s+have|'ve|ve)(?:\s+got)?\s+(?:\w+\s+)?(?P<who>kids|children)\b(?:[^.\d]{0,14})(?P<ages>\d{1,3}(?:\s*(?:,|and)\s*\d{1,3})*)"#
);
pattern!(
    KID_ADULTS_AGES,
    r#"(?i)\b(?:kids?|children|child)\b(?:[^.\d]{0,14})?(?:aged?|of|are)\s+(?P<ages>\d{1,3}(?:\s*(?:,|and)\s*\d{1,3})+)"#
);
pattern!(
    KID_AGES_NOT,
    r#"(?i)\b(?:under|over|above|below|nearly|almost|about|round|around)\s*$"#
);
pattern!(
    AGE_TAIL,
    r#"(?i)^\s*(?:$|[.,;:!?)]|\b(?:and|but|so|now|today|tomorrow|yesterday|too)\b|\bgoing on \d{1,3}\b|\bas well\b|\bthis year\b|\byears? old\b|\bold\b|\bnext (?:week|month|year|spring|summer|autumn|winter|monday|tuesday|wednesday|thursday|friday|saturday|sunday)\b|\b(?:in|on|by) [a-z]{3,9}\b)"#
);
pattern!(
    A_LIST_OF_AGES,
    r#"(?i)(?P<ages>\d{1,3}(?:\s*(?:,|and|, and)\s*\d{1,3})+)\b"#
);
pattern!(
    QUESTION_OPEN,
    r#"(?i)^(?:(?:and|but|so|then|also|oh|ok|okay|well|now|hey|sorry|please|quick|hi)\s+)*(?:is|isnt|are|arent|was|wasnt|were|werent|am|do|dont|does|doesnt|did|didnt|has|hasnt|have|havent|had|can|cant|could|couldnt|would|wouldnt|will|wont|shall|should|shouldnt)\b"#
);
pattern!(
    CLAUSE_JOIN,
    r#"(?i)\b(?:if|because|since|though|although|when|whenever|and|but|so)\b|[,;]"#
);
pattern!(
    AGE_ASK,
    r#"\bhow old (?:is|are|was|were)\b|\bwhat age(?:s)? (?:is|are|was|were)\b|\bwhat ages?\b|\bhow old\b"#
);
pattern!(
    AGE_WORD,
    r#"(?i)\b(?:my|our|the|their|his|her|your)\s+(?:\w+\s+)?ages?\b|\bages?\s+of\s+(?:my|our|the|their)\b"#
);
pattern!(
    AGE_AS_A_CAUSE,
    r#"(?i)\b(?:just|only|part of|down to|because of|normal for|usual for|expected at)\s+(?:\w+\s+){0,2}(?:age|old age|getting old)\b|\bold age\b|\bat (?:her|his|their|my) age\b"#
);
pattern!(
    AGE_RULE_WORD,
    r#"(?i)^(?:do|does|did|should|shall|can|could|will|would|must|may|might|need)\b"#
);
pattern!(
    AGE_ADVERB,
    r#"(?i)^(?:exactly|really|roughly|approximately|about|now|actually|again|already|even)\b"#
);
pattern!(AGE_COPULA, r#"(?i)^(?:is|are|was|were|be|am)\b"#);
pattern!(
    AGE_YOU_SAY,
    r#"(?i)^(?:you|u|i)\s+(?:say|said|think|guess|remember|know|reckon)\b"#
);
pattern!(
    AGE_DETERMINER,
    r#"(?i)^(?:my|our|your|their|his|her|the|a|an)\b"#
);
pattern!(
    AGE_PERSON_WORD,
    r#"(?i)^(?:kids?|children|child|family|i|you|u|we|they|them|he|she|everyone|both|little|one)\b"#
);
pattern!(
    THE_KIDS,
    r#"(?i)\b(?:kids?|children|child|family|everyone|both|they|them|their|boys?|girls?)\b"#
);
pattern!(
    CHILD_WORD,
    r#"(?i)\b(?:kids?|children|child|family|boys?|girls?)\b"#
);
pattern!(
    GROUP_ASK,
    r#"(?i)\b(?:my|the|his|her|their|our|your)\s+(grandchildren|grandparents|siblings|brothers|children|parents|sisters|cousins|uncles|twins|girls|aunts|kids|boys)\b"#
);
pattern!(
    A_PLACE_ASK,
    r#"(?i)\b(?:the|my|our|your)\s+(?P<who>little girl|younger one|little boy|little one|little lad|older one|small one|youngest|littlest|wee one|toddler|big one|biggest|newborn|oldest|eldest|elder|baby)\b"#
);
pattern!(
    RELATION_AGE,
    r#"(?i)\bmy\s+(?:(?P<many>(?:(?:granddaughter|stepdaughter|grandmother|grandfather|stepbrother|grandmother|grandfather|girlfriend|stepmother|stepfather|stepsister|grandchild|neighbour|boyfriend|stepchild|daughter|neighbor|grandmum|grandmom|granddad|grandson|granddad|partner|husband|brother|parents|grandma|grandad|grandpa|fiancee|stepmum|stepmom|stepdad|stepson|mother|father|sister|granny|auntie|nephew|cousin|friend|fiance|missus|twins|nanna|aunty|uncle|niece|hubby|wife|gran|nana|aunt|gran|son|mum|mom|dad|nan|gfs|bfs|gf|bf)(?:\s+in\s+laws?)?|grandparents|grandchildren)s?)\s+are|(?P<one>(?:granddaughter|stepdaughter|grandmother|grandfather|stepbrother|grandmother|grandfather|girlfriend|stepmother|stepfather|stepsister|grandchild|neighbour|boyfriend|stepchild|daughter|neighbor|grandmum|grandmom|granddad|grandson|granddad|partner|husband|brother|parents|grandma|grandad|grandpa|fiancee|stepmum|stepmom|stepdad|stepson|mother|father|sister|granny|auntie|nephew|cousin|friend|fiance|missus|twins|nanna|aunty|uncle|niece|hubby|wife|gran|nana|aunt|gran|son|mum|mom|dad|nan|gfs|bfs|gf|bf)(?:\s+in\s+laws?)?)\s*(?:'s|is|was))\s+(?P<age>\d{1,3})\b"#
);
pattern!(
    DIRECT_RELATION,
    r#"(?i)\b(?:my|our|your|their|his|her|the)\s+(?:(?:little|big|older|younger|eldest|youngest|oldest|other)\s+)?(?P<who>grandchildren|granddaughter|grandparents|stepchildren|stepdaughter|childminder|goddaughter|grandfather|grandmother|hairdresser|stepbrother|girlfriend|grandchild|housemates|instructor|little one|neighbours|stepfather|stepmother|stepsister|boyfriend|colleague|daughters|flatmates|grandkids|housemate|neighbour|stepchild|brothers|children|daughter|flatmate|granddad|grandmom|grandmum|grandson|landlady|landlord|neighbor|stepkids|youngest|brother|cousins|fiancee|grandad|grandma|grandpa|husband|in-laws|manager|nephews|newborn|parents|partner|sisters|stepdad|stepmom|stepmum|stepson|toddler|wee one|auntie|barber|cousin|eldest|father|fiance|friend|godson|granny|lodger|missus|mother|nephew|nieces|oldest|sister|uncles|aunts|aunty|carer|child|girls|hubby|nanna|niece|tutor|twins|uncle|aunt|baby|boss|boys|girl|gran|kids|mate|nana|sons|wife|bfs|boy|dad|gfs|kid|mom|mum|nan|son|bf|gf)s?\b"#
);
pub(super) const FAMILY_GROUP: &[(&str, &[&str])] = &[
    (
        "parents",
        &[
            "parent",
            "mother",
            "mum",
            "mom",
            "mam",
            "mummy",
            "mommy",
            "father",
            "dad",
            "daddy",
            "papa",
            "stepmother",
            "stepfather",
        ],
    ),
    (
        "grandparents",
        &[
            "grandparent",
            "grandmother",
            "grandma",
            "gran",
            "nan",
            "nana",
            "granny",
            "grandfather",
            "grandad",
            "granddad",
            "grandpa",
        ],
    ),
    (
        "siblings",
        &[
            "sibling",
            "brother",
            "sister",
            "half brother",
            "half sister",
        ],
    ),
    ("brothers", &["brother", "half brother"]),
    ("sisters", &["sister", "half sister"]),
    (
        "kids",
        &[
            "son", "daughter", "twins", "baby", "child", "kid", "boy", "girl", "youngest",
            "eldest", "oldest",
        ],
    ),
    (
        "children",
        &[
            "son", "daughter", "twins", "baby", "child", "kid", "boy", "girl", "youngest",
            "eldest", "oldest",
        ],
    ),
    ("twins", &["twin", "son", "daughter", "child", "kid"]),
    ("boys", &["son", "boy", "child", "kid"]),
    ("girls", &["daughter", "girl", "child", "kid"]),
    (
        "grandchildren",
        &["grandson", "granddaughter", "grandchild"],
    ),
    ("uncles", &["uncle"]),
    ("aunts", &["aunt"]),
    ("cousins", &["cousin"]),
];
pub(super) const KNOWN_RELATIONS: &[&str] = &[
    "daughter",
    "son",
    "twins",
    "partner",
    "husband",
    "wife",
    "mum",
    "mom",
    "mother",
    "dad",
    "father",
    "sister",
    "brother",
    "parents",
    "gran",
    "grandma",
    "grandad",
    "granny",
    "nana",
    "nan",
    "nanna",
    "grandmother",
    "grandfather",
    "grandpa",
    "aunt",
    "auntie",
    "aunty",
    "uncle",
    "niece",
    "nephew",
    "cousin",
    "friend",
    "neighbour",
    "neighbor",
    "boyfriend",
    "girlfriend",
    "fiance",
    "fiancee",
    "stepmum",
    "stepmom",
    "stepmother",
    "stepdad",
    "stepfather",
    "stepson",
    "stepdaughter",
    "stepbrother",
    "stepsister",
    "grandmum",
    "grandmom",
    "grandmother",
    "granddad",
    "grandfather",
    "grandson",
    "granddaughter",
    "grandchild",
    "gran",
    "granddad",
    "stepchild",
    "missus",
    "hubby",
    "gf",
    "gfs",
    "bf",
    "bfs",
];
pub(super) const CHILD: &[&str] = &[
    "son", "daughter", "twins", "baby", "child", "kid", "boy", "girl", "youngest", "eldest",
    "oldest",
];
pub(super) const KID_GROUP: &[&str] = &[
    "kids",
    "children",
    "child",
    "kid",
    "sons",
    "daughters",
    "boys",
    "girls",
];
pub(super) const YOUNGER_END: &[&str] = &[
    "baby",
    "little boy",
    "little girl",
    "little lad",
    "little one",
    "littlest",
    "newborn",
    "small one",
    "toddler",
    "wee one",
    "younger one",
    "youngest",
];
pub(super) const PLACE_STANDS_ALONE: &[&str] = &[
    "actually", "again", "already", "and", "anyway", "are", "at", "can", "could", "got", "had",
    "has", "have", "here", "in", "is", "just", "might", "must", "now", "nowadays", "of", "on",
    "only", "or", "please", "really", "should", "still", "then", "there", "though", "today", "too",
    "turned", "turns", "was", "were", "will", "would", "yet",
];
pub(super) const CHILD_OF: &[(&str, &str)] = &[
    ("children", "child"),
    ("kids", "kid"),
    ("childs", "child"),
    ("babies", "baby"),
    ("boys", "boy"),
    ("girls", "girl"),
    ("sons", "son"),
    ("daughters", "daughter"),
];
pub(super) const MANY_OF_THEM: &[(&str, &str)] = &[
    ("child", "children"),
    ("grandchild", "grandchildren"),
    ("baby", "babies"),
    ("person", "people"),
    ("woman", "women"),
    ("man", "men"),
];
