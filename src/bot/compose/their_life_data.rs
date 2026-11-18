//! Source patterns and wording from the pinned PyBot reader.
use fancy_regex::Regex;
use std::sync::LazyLock;
pub(super) static TASTE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)^(?:so |and |but |ok |okay )?(?:do|did|would|will|should)\s+i\s+(?:actually\s+|even\s+|really\s+|still\s+|probably\s+|think\s+i\s+)?(?:like|love|enjoy|hate|want|fancy|prefer|get\s+on\s+with)\b"##).expect("TASTE")
});
pub(super) static AN_AGE_NAME_OR_PLACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow\s+old\b|\bname\b|\bwhere\b"##).expect("AN_AGE_NAME_OR_PLACE")
});
pub(super) static WORK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bwhat\s+(?:does|do|did)\s+(?:(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)|she|he|they|him|her|them)\s+(?:do|work\s+as)\b|\bwhats?\s+(?:(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)|she|he|they|him|her|them)\s+like\b|\bwhat\s+is\s+(?:(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)|she|he|they|him|her|them)\s+like\b"##).expect("WORK")
});
pub(super) static ABOUT_THEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:who|what|where|when|which|how(?:\s+\w+)?)\s+(?P<about>(?:\w+\s+){0,2})(?:is|was|are|were|does|did|do|has|have)\s+(?:(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)|she|he|they|him|her|them)\b"##).expect("ABOUT_THEM")
});
pub(super) static ALREADY_SAID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"\b[A-Za-z]{4,}\b"##).expect("ALREADY_SAID"));
pub(super) const STOP_ABOUT: &[&str] = &[
    "about", "being", "did", "does", "doing", "going", "has", "have", "their", "there", "these",
    "those", "what", "whats", "when", "where", "which", "who", "whom", "whose",
];
pub(super) static A_NEED_OF_THEIRS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)|she|he|they|him|her|them)\s+(?:need|needs|want|wants|take|takes|build|builds|make|makes|cook|cooks|wear|wears|bring|brings|give|gives|try|tries|use|uses|buy|buys|eat|eats|say|says|tell|tells|get|gets|say\s+to|do\s+about)\b"##).expect("A_NEED_OF_THEIRS")
});
pub(super) static STAY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow\s+long\s+(?:is|are|will|would|might|could|does|do)\s+(?:(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)|she|he|they|him|her|them)\s+(?:(?:likely|going|expected|probably|meant)\s+to\s+)?(?:be\s+)?(?:in|staying|stay|away|off|be\s+off|be\s+in|have\s+to\s+stay|need\s+to\s+stay)\b|\bhow\s+long\s+(?:is|are)\s+(?:the\s+)?(?:course|treatment|recovery|stay|operation)\s+(?:going\s+to\s+)?(?:take|last)\b"##).expect("STAY")
});
pub(super) static MISSING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:missing|lost|gone|disappeared|found|ran\s+off|run\s+off|not\s+come\s+home|havent\s+come\s+home|hasnt\s+come\s+home)\b"##).expect("MISSING")
});
pub(super) static WHERE_THEY_ARE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bwhere(?:'s|\s+is|\s+are|\s+was|\s+were)\s+(?:(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)|she|he|they|him|her|them)\b"##).expect("WHERE_THEY_ARE")
});
pub(super) static A_PRONOUN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"(?i)\b(?:she|he|they|him|her|them)\b"##).expect("A_PRONOUN"));
pub(super) static SAID_WHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)\b|\b(?:mum|dad|son|daughter|sister|brother|friend|mate|partner|husband|wife|nan|gran|boss|colleague|neighbour)\b"##).expect("SAID_WHO")
});
pub(super) static NOT_THIS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:excess|booking|reference|policy|premium|prescription|appointment|balance|account|order|invoice|bill|refund|renewal|renew|due\s+date|expiry|expires|warranty|guarantee|membership|subscription|statement|receipt|tracking|delivery|parcel|form|deadline|opening|closing|open|close|closed|hours|times|timetable|train|bus|flight|what\s+time|cost|price|how\s+much)\b"##).expect("NOT_THIS")
});
pub(super) static RECALL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:was\s+it|which\s+(?:\w+\s+)?was\s+it|what\s+did\s+i\s+say|again\b.*\bwhat|remind\s+me\s+what)\b"##).expect("RECALL")
});
pub(super) static A_HOW_TO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\bhow\s+(?:do|does|did|can|could|should)\s+(?:i|you|we)\b"##)
        .expect("A_HOW_TO")
});
pub(super) static THEIR_RELATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:my|our|his|her|their)\s+(?:mum|mom|mam|dad|father|mother|son|daughter|child|kid|boy|girl|baby|sister|brother|sibling|friend|mate|boss|manager|partner|husband|wife|boyfriend|girlfriend|fianc\w*|nan|nana|gran|granny|grandma|grandad|grandpa|aunt|auntie|uncle|cousin|niece|nephew|neighbour|neighbor|landlord|landlady|carer|colleague|coworker|in-?laws?|stepson|stepdaughter|stepsister|stepbrother|ex|flatmate|housemate|roommate|tutor|teacher|dog|cat|puppy|kitten|vet)"##).unwrap()
});
pub(super) const TASTE_LINES: &[&str] = &[
    "That one's yours, not mine: I can't know what you like from here. What does your gut say?",
    "No way for me to know that, sorry - only you can say. What is your instinct telling you?",
    "I'd only be guessing, and you'd know in a second. What do you reckon?",
];
pub(super) const WORK_LINES: &[&str] = &[
    "I've not met them, so I can't say what they do. What do they do?",
    "No idea, sorry - nobody has told me. What do they do for a living?",
    "Not a thing I could know. Tell me about them and I'll keep it straight from here: what do they do for a living?",
];
pub(super) const THEIRS_LINES: &[&str] = &[
    "I've never met them, so I've nothing on that one - you'd know better than any page. Tell me and I'll keep it for the rest of our chat.",
    "That's yours to know and mine to be told. What's the answer?",
    "No idea, sorry - nobody has introduced us. Tell me and it stays here with me.",
];
pub(super) const STAY_LINES: &[&str] = &[
    "Only the people looking after them can say that - ask the nurse on the ward and they will give you a rough day. How are they in themselves?",
    "Nobody here can say how long they'll keep them in. The staff on the ward can, and the desk will ring through for you. How are they doing?",
    "That's the ward's call, not mine - the nurse looking after them knows, and it is one question at the desk. How is she in herself?",
];
