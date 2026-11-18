//! Life-report cues and response pools from the pinned PyBot source at 7d63c7f.

pub(super) const NOT_A_SHAPE: &str = concat!(
    r#"\b(?:pain|pains|painful|hurt|hurts|hurting|ill|sick|poorly|dying|died|dead|death|"#,
    r#"cancer|chemo|radiotherapy|tumour|tumor|dementia|alzheimers|alzheimer's|stroke|"#,
    r#"hospital|hospice|funeral|buried|passed away|lost|loss|bereave\w*|griev\w*|"#,
    r#"depress\w*|anxious|anxiety|panic|suicid\w*|self harm|crisis|emergency|"#,
    r#"broken|leaking|damp|mould|flood\w*|evicted|eviction|arrears|debt|bailiff\w*|"#,
    r#"divorce|separated|affair|abusive|violent|police|social services)\b"#,
);

pub(super) const STORY: &str = concat!(
    r#"\b(?:that|which|who|and|but|because|so|when|while|if|keeps?|kept|started|stopped|"#,
    r#"barking|bites|bit|chewed|escaped|ran|went|got|needs?|needed|wants?|made|makes|"#,
    r#"cost|costs|comes?|came|goes?|going|has|have|had|is|are|was|were|will|would|can|could)\b"#,
);

pub(super) const TONE: &str = concat!(
    r#"(?:\s+(?:tbh|tho|though|now|anyway|actually|really|literally|honestly|i\s+suppose|"#,
    r#"i\s+guess|still|also|too|by\s+the\s+way|btw|atm|at\s+the\s+moment|lol|ha|haha|lmao|"#,
    r#"so\s+yeah|yeah|innit|mate|dude|as\s+well))+$"#,
);

pub(super) const LIVE: &str = concat!(
    r#"^(?P<who>i|we)\s+(?:live|stay)\s+"#,
    r#"(?P<where>(?:in|near|on|by|above|below|behind|next\s+to|opposite|at)\s+"#,
    r#"(?:a|an|the|my|our|their|his|her)?\s*[a-z][a-z' -]{0,24})$"#,
);

pub(super) const PLACE_KINDS: &str = concat!(
    r#"^(?:in|near|on|by|above|below|behind|next\s+to|opposite|at)\s+"#,
    r#"(?:a|an|the|my|our)?\s*(?:flat|house|home|place|bungalow|cottage|farm|boat|caravan|"#,
    r#"car park|park|river|canal|sea|coast|beach|woods|wood|common|high\s+street|"#,
    r#"village|town|city|estate|council\s+estate|block|tower\s+block|camper|motorhome|"#,
    r#"shop|pub|office|station|hospital\s+car\s+park|school|church|allotment)$"#,
);

pub(super) const PET: &str = concat!(
    r#"^(?P<who>i|we)(?:'ve\s+got|\s+(?:have|got|own))\s+"#,
    r#"(?P<num>a|an|one|two|three|four|five|six|\d+|\d+\s+)?\s*"#,
    r#"(?P<pet>dogs?|cats?|puppies|puppy|kittens?|kitten|rabbits?|hamsters?|budgies?|parrots?|"#,
    r#"horses?|pony|guinea\s+pigs?|tortoises?|snakes?|lizards?|geckos?|fish|gerbils?|ferrets?|"#,
    r#"chickens?|hens?|ducks?|goats?|bees)$"#,
);

pub(super) const SIBLINGS: &str = concat!(
    r#"^(?P<who>i|we)(?:'ve\s+got|\s+(?:have|got))\s+(?P<what>"#,
    r#"(?:(?:a|an|one|two|three|four|five|six|seven|eight|\d+|older|younger|little|big|twin|"#,
    r#"step|half)[\s-]+)*(?:brother|sister|brothers|sisters)"#,
    r#"(?:\s*(?:,|and)\s*)?"#,
    r#"(?:(?:a|an|one|two|three|four|five|six|seven|eight|\d+|older|younger|little|big|twin|"#,
    r#"step|half)[\s-]+)*(?:brothers?|sisters?)?)$"#,
);

pub(super) const AGE: &str = concat!(
    r#"^(?:im|i'm|i\s+am)\s+(?P<age>\d{1,2})(?:\s*(?:years?\s+old|yrs?\s+old|y/?o|yo|now|"#,
    r#"today))?$"#,
);

pub(super) const NOT_AN_AGE: &str = concat!(
    r#"\b(?:weeks?|days?|months?|pregnan\w*|kilos?|kg|stone|pounds?|lbs?|miles?|minutes?|"#,
    r#"hours?|degrees?|percent|%|cm|feet|foot|ft)\b"#,
);

pub(super) const SOMEONE_WORK: &str =
    r#"^works?\s+(?:in|at|for)\s+(?:a|an|the)?\s*(?P<place>[a-z][a-z' ]{1,24})$"#;

pub(super) const SOMEONE_HOURS: &str = concat!(
    r#"^works?\s+(?P<hours>nights?|days?|night\s+shifts?|shifts|weekends?|evenings?|weekdays|"#,
    r#"part\s+time|full\s+time|(?:two|three|four|five|six)\s+days\s+a\s+week|"#,
    r#"long\s+hours|away\s+a\s+lot)$"#,
);

pub(super) const SOMEONE_AGE: &str =
    r#"^is\s+(?P<age>\d{1,2})(?:\s*(?:years?\s+old|yrs?\s+old|y/?o|yo|now|today))?$"#;

pub(super) const SOMEONE_KIDS: &str = concat!(
    r#"^has\s+(?P<num>\d+|one|two|three|four|five|six|seven|eight)\s+"#,
    r#"(?P<kids>kids|children|child|boys?|girls?|sons?|daughters?|twins?)$"#,
);

pub(super) const THE_LINE_BEFORE_IS_HEAVY: &str = concat!(
    r#"\b(?:ill|unwell|poorly|sick|dying|died|death|passed away|funeral|bereave\w*|griev\w*|"#,
    r#"cancer|chemo\w*|radiotherap\w*|tumou?r|dementia|alzheim\w*|stroke|sepsis|surgery|"#,
    r#"operation|transplant|hospice|palliative|terminal|hospital|icu|ambulance|diagnos\w*|"#,
    r#"bully\w*|bullied|depress\w*|suicid\w*|self harm|miscarr\w*|stillborn|addict\w*|"#,
    r#"relapse\w*|overdose\w*|sectioned|in pain|suffering)\b"#,
);

pub(super) const RELATION: &str = concat!(
    r#"mother\s+in\s+law|father\s+in\s+law|son\s+in\s+law|daughter\s+in\s+law|"#,
    r#"sister\s+in\s+law|brother\s+in\s+law|"#,
    r#"step\s?mum|stepmum|step\s?dad|stepdad|step\s?daughter|stepdaughter|step\s?son|"#,
    r#"stepson|"#,
    r#"mum|mom|mother|mam|dad|father|gran|granny|grandma|nan|nana|grandad|grandpa|grandfather|"#,
    r#"sister|brother|son|daughter|aunt|auntie|uncle|cousin|niece|nephew|wife|husband|partner|"#,
    r#"boyfriend|girlfriend|grandson|granddaughter|grandchild"#,
);

pub(super) const HER: &[&str] = &[
    "mum",
    "mom",
    "mother",
    "mam",
    "gran",
    "granny",
    "grandma",
    "nan",
    "nana",
    "sister",
    "daughter",
    "aunt",
    "auntie",
    "niece",
    "wife",
    "girlfriend",
    "stepmum",
    "step mum",
    "stepdaughter",
    "step daughter",
    "mother in law",
    "daughter in law",
    "sister in law",
    "granddaughter",
];

pub(super) const HIM: &[&str] = &[
    "dad",
    "father",
    "grandad",
    "grandpa",
    "grandfather",
    "brother",
    "son",
    "uncle",
    "nephew",
    "husband",
    "boyfriend",
    "stepdad",
    "step dad",
    "stepson",
    "step son",
    "father in law",
    "son in law",
    "brother in law",
    "grandson",
];

pub(super) const AGE_READ_ELSEWHERE: &[&str] = &[
    "son",
    "daughter",
    "kid",
    "kids",
    "child",
    "children",
    "boy",
    "girl",
    "eldest",
    "youngest",
    "grandson",
    "granddaughter",
    "grandchild",
    "grandchildren",
    "grandkids",
    "stepson",
    "stepdaughter",
    "step son",
    "step daughter",
    "son in law",
    "daughter in law",
    "niece",
    "nephew",
];

pub(super) const PLACES: &[&str] = &[
    "{Where} - what's it like there?",
    "How long have you been there?",
    "{Where}, then. Is it handy for what you need?",
];

pub(super) const ONE_PET: &[&str] = &[
    "{Pet} - what's their name?",
    "How long have they been with you?",
    "{Pet} - who spoils them most?",
];

pub(super) const PETS: &[&str] = &[
    "{Pets} - do they get on?",
    "{Pets} - what are their names?",
    "{Pets} - who spoils them most?",
];

pub(super) const SIBLING_LINES: &[&str] = &[
    "{What} - older or younger than you?",
    "{What} - do you see much of them?",
    "{What}, and do you get on?",
];

pub(super) const AGE_LINES: &[&str] = &[
    "{Age} - how's this year treating you?",
    "{Age}, then. Anything you'd like to be different by {Next}?",
    "{Age} - a good age to be, or not?",
];

pub(super) const SOMEONE_WORK_LINES: &[&str] = &[
    "A {Place}, then. What's that like for {them}?",
    "{Place} - is that new for {them}?",
    "A {Place}, then. Do you hear much about it from {them}?",
];

pub(super) const SOMEONE_HOURS_LINES: &[&str] = &[
    "{Hours} - does that leave much time for {them}?",
    "{Hours}, then. What's that like for {them}?",
    "{Hours} - does it suit {them}?",
];

pub(super) const SOMEONE_AGE_LINES: &[&str] = &[
    "{Age} - do you see {them} much?",
    "{Age}, then. Do you get to see {them} often?",
    "{Age} - how often do you get to see {them}?",
];

pub(super) const SOMEONE_KIDS_LINES: &[&str] = &[
    "{Num} kids - how old are they?",
    "{Num} kids, then. Do they live near you?",
    "{Num} kids - what are they into?",
];
