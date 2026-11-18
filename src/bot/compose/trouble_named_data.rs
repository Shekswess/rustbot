// Patterns and vocabularies from the two-way named trouble reader.

pattern!(
    A_THE_TROUBLE,
    r#"(?i)^(?:so\s+|well\s+|and\s+|but\s+|ok\s+|okay\s+|yeah\s+|yeh\s+|no\s+|also\s+|honestly\s+)*(?:(?:it'?s|its|it\s+is)\s+(?:the|my|our|his|her|their|this|that|me)?\s*(?P<thing>[a-z][a-z' -]{2,40}?)\s+(?:that'?s|thats|what'?s|which\s+is|is)\s+(?:the\s+|my\s+|our\s+)?(?:problem|trouble|issue|worry|wrong|bit\s+that'?s\s+wrong)|(?:the|my|our)\s+(?:problem|trouble|issue)\s+(?:is|was)\s+(?:the|my|our|his|her|their)?\s*(?P<thing2>[a-z][a-z' -]{2,40}))"#
);
pattern!(
    A_BROKEN_NOW,
    r#"(?i)\b(?:broken|broke|not\s+working|stopped\s+working|packed\s+in|packed\s+up|dead|leaking|flooding|blown|on\s+fire|smoking|sparking|no\s+power|no\s+hot\s+water|not\s+coming\s+on|not\s+starting|wont\s+start)\b"#
);
pattern!(
    A_ASK,
    r#"(?i)\?|\b(?:can|could|should|how|what|when|why|which|who|shall)\b"#
);
pub(super) const FILLER: &[&str] = &[
    "a", "an", "and", "at", "but", "for", "her", "his", "in", "it", "me", "my", "of", "on", "our",
    "so", "that", "the", "their", "this", "to", "with",
];
pub(super) const SUPPLY: &[&str] = &[
    "electric",
    "electricity",
    "electrics",
    "power",
    "meter",
    "fuse",
    "fuses",
    "gas",
];
pub(super) const HEATING: &[&str] = &[
    "boiler",
    "heating",
    "radiator",
    "radiators",
    "hot",
    "immersion",
    "thermostat",
];
pub(super) const WATER: &[&str] = &[
    "water", "leak", "leaking", "drain", "drains", "tap", "taps", "sink", "toilet", "plumbing",
    "pipe", "pipes", "shower", "bath",
];
pub(super) const VEHICLE: &[&str] = &[
    "car",
    "van",
    "bike",
    "motorbike",
    "scooter",
    "engine",
    "tyre",
    "tyres",
    "clutch",
    "mot",
    "gearbox",
    "battery",
    "brakes",
];
pub(super) const CONNECTION: &[&str] = &[
    "wifi",
    "internet",
    "broadband",
    "signal",
    "mobile",
    "phone",
    "router",
    "connection",
    "landline",
    "data",
];
pub(super) const BILL: &[&str] = &[
    "rent", "mortgage", "council", "tax", "bill", "bills", "energy", "debt", "loan", "money",
    "fees", "fee", "service", "charge", "charges",
];
pattern!(
    FLOOR_CLAUSE,
    r#"(?i)^(?:(?:go|carry)\s+on\b|tell\s+me\s+more\b|say\s+more\b|keep\s+going\b|go\s+ahead\b|ask\s+away\b|i'?m\s+listening\b|i\s+am\s+listening\b|i'?m\s+here\b|take\s+your\s+time\b|over\s+to\s+you\b|what'?s\s+on\s+your\s+mind\b|i'?ll\s+do\s+my\s+best\b|spill\s+it\b|i'?m\s+all\s+ears\b)"#
);
pattern!(
    FLOOR_COURTESY,
    r#"(?i)^(?:of\s+course|sure|ok|okay|right|yes|no|well|please|thanks|thank\s+you|no\s+need(?:\s+to\s+apologise)?|you'?re\s+not\s+dumping\s+anything\s+on\s+me|don'?t\s+apologise(?:\s+for\s+that)?|nothing\s+to\s+be\s+sorry\s+about|no\s+problem|you'?re\s+welcome|that'?s\s+what\s+i'?m\s+here\s+for|i'?d\s+rather\s+hear\s+it\s+than\s+not)$"#
);
pattern!(
    FLOOR_WAITING,
    r#"(?i)\b(?:if|when)\s+you(?:'re|\s+are)?\s+(?:want|need|ready|like)|for\s+as\s+long\s+as|\bwhenever\b|\bany\s+time\b"#
);
pattern!(SPACED_DASH, r#"\s+(?:\u2014|\u2013|-)\s+"#);
pattern!(SENTENCE, r#"(?<=[.!?])\s+"#);
pub(super) const SAID_BACK: &[(&str, &str)] = &[
    ("", "— noted."),
    ("got it —", "."),
    ("", ", good to know."),
    ("noted:", "."),
    ("", "— i'll remember that."),
    ("", ", right — i've got that."),
];
pub(super) const SUPPLY_REPLIES: &[&str] = &[
    "A supply goes wrong in one of two ways - a bill that has got away from you, or the supply itself - and the first step is different for each. Which is it for you?",
    "With the electric or the gas it is nearly always the bill or the supply, and one is a phone call to the supplier while the other is a payment plan. Which is it?",
];
pub(super) const HEATING_REPLIES: &[&str] = &[
    "Heating trouble is usually either a bill you can't cover or the boiler not doing its job, and the first step is different for each. Which is it for you?",
    "The heating has two ways of becoming the problem - what it costs, or it not coming on - and the fix is different for each. Which one is it?",
];
pub(super) const WATER_REPLIES: &[&str] = &[
    "Water in the wrong place is a today job and water that has stopped is the supplier's; the first move is different for each. Which is it for you?",
    "Two different problems wear the same word: a leak, which is a today job, and a supply that has gone, which is the water company's. Which one is yours?",
];
pub(super) const VEHICLE_REPLIES: &[&str] = &[
    "A vehicle goes wrong in two ways that matter - something is not right with it, or what putting it right costs - and the first step is different for each. Which is it?",
    "With a car it is usually the thing itself or the bill to fix it, and only one of those is the garage's. Which one is the trouble?",
];
pub(super) const CONNECTION_REPLIES: &[&str] = &[
    "Broadband goes wrong in two ways - the connection dropping, or the bill - and the first check is different for each. Which is it?",
    "A connection problem is either the line or the account, and the supplier's own status page answers the first before you ring anybody. Which of the two is it?",
];
pub(super) const BILL_REPLIES: &[&str] = &[
    "A bill becomes the problem in one of two ways - too big for the month, or wrong on the paper - and the first step is different for each. Which is it?",
    "It is usually either the amount or the paper, and a bill that is wrong is worth a call before a payment plan. Which one is it for you?",
];
