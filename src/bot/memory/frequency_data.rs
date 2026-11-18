//! Frequency markers and replies from the pinned chat module.

pattern!(
    FREQUENCY_ONLY,
    r####"(?i)^\W*(?:(?:and|but|so|oh|well|yeah|yep|ok|okay|right|alright|no)\W+)*(?:(?:it'?s|its|it is|that'?s|thats|there'?s|theres)\s+)?((?:(?:every|most)(?:\s+single)?\s+(?:night|nights|morning|mornings|day|days)|all\s+(?:the\s+)?time|nightly|daily|constantly|non ?stop))\b(?:\s+(?:now|though|at the moment|lately|these days|still))*\W*$"####
);
pattern!(
    WORRY,
    r####"\b(?:worried|worry|worrying|worries|stress(?:ed|ful|ing)?|anxious|anxiety|overthink\w*|overwhelm\w*|on my mind|can'?t switch off|racing|panic(?:king)?)\b|\b(?:we|i|he|she|they) had a (?:row|argument|fight|falling out)\b|\bargued (?:with|about)\b|\bfell out with\b|\brow with\b"####
);
pattern!(
    MY_NIGHT,
    r####"\b(?:keep|keeps|kept) waking\b|\bwak(?:e|es|ing) up (?:at|in the|every|most|half)\b|\bwoke up (?:at|in the)\b|\b(?:awake|lying awake|lay awake|up) (?:all|half|most of) the night\b|\bno (?:night's )?sleep\b|\bbroken night\b"####
);
pattern!(
    NOT_MY_NIGHT,
    r####"\b(?:my|our|the|his|her|their)\s+(?:\w+\s+){0,2}(?:baby|son|daughter|kid|kids|child|children|wife|husband|partner|mum|mom|mother|dad|father|dog|cat|puppy|kitten|neighbou?r)s?\b|\b(?:he|she|they)\s+(?:keeps?|kept|was|were|is|are)\b"####
);
pattern!(
    SYMPTOM,
    r####"\b(?:pain|painful|ache|aches|aching|achy|hurt|hurts|hurting|cramp|cramps|cough|coughing|tooth|toothache|toothaches|headache|headaches|migraine|migraines|earache|earaches|backache|stomachache|tummyache|heartburn|reflux|nausea|nauseous|sick|fever|itch|itching|sore|swollen|stiff)\b"####
);
pattern!(
    LOSS,
    r####"\b(?:died|dying|death|dead|passed away|funeral|burial|cremation|bereave\w*|lost my (?:mum|mom|dad|father|mother|wife|husband|son|daughter|brother|sister|partner|baby|child)|(?:mum|mom|dad|father|mother|wife|husband|son|daughter|brother|sister|partner|gran|nan|grandad)\s+(?:has|have|had)\s+(?:just )?(?:died|passed))\b"####
);
pattern!(
    _A_CHILD_OR_THEM,
    r####"(?i)\b(?:child|children|kid|kids|baby|babies|toddler|they|them|their|he|she|him|her)\b"####
);
pattern!(
    END_OF_ROPE,
    r####"\b(?:can'?t|cannot) (?:do this|go on|take it|cope|keep going)\b|\bend(?:ing)? it\b|\bkill myself\b|\bhurt myself\b|\bwant to die\b|\bno point\b|\bhearing things\b|\bseeing things\b|\bnot worth it\b"####
);
pattern!(
    OTHER_AWAKE,
    r####"\b(?:my|our|the|his|her) (?:\d+ )?(?:baby|son|daughter|kid|kids|child|children|toddler|year[- ]olds?|little one|little boy|little girl|littleun|littlun|newborn|grandson|granddaughter|grandchild|twins|teenager|teen|stepson|stepdaughter|stepchild|husband|wife|partner|mum|dad|dog|cat|puppy|kitten|flatmate|neighbours?|neighbor)\b[^.?!]*?\b(?:up since|up all night|awake|up from|won'?t|wont|can'?t|cant)\b|\b(?:my|our|the|his|her) (?:boy|girl|lad|lass)\b[^.?!]*?\b(?:up since|up from|up every morning)\b|\b(?:little one|little boy|little girl|littleun|littlun|toddler|newborn)\b[^.?!]*?\b(?:up since|up all night|awake|up from)\b|\b(?:been |was )?up (?:since|all night|all hours|at \d)\b[^.?!]*?\bwith (?:a |the |my |our |his |her )?(?:sick|poorly|ill|unwell|teething|crying|colicky|newborn)\b"####
);

pub(super) const NIGHT_FREQUENCY: &[&str] = &[
    "{often} is a lot. How long has it been like that?",
    "{often}, then. That wears you down - has it been going on long?",
    "{often}, then. Is that recent, or has it been like this for weeks?",
];

pub(super) const NIGHT_FREQUENCY_KEPT: &[&str] = &[
    "{often}, then. Tonight, keep the room cool and dark, and if you are still awake after twenty minutes get up and read something dull until you feel heavy.",
    "{often}, then - keep the same getting-up time every day, even after a bad one; that is the bit that steadies it.",
    "{often}, then. Leave the phone out of reach and get ten minutes of daylight early; if it is still like this in a few weeks, go back to the GP.",
];

pub(super) const BODY_FREQUENCY: &[&str] = &[
    "{often} is a lot. How long has it been going on?",
    "{often}, then - is that recent, or has it been like this a while?",
    "{often} is worth a proper look. Has it been going on for weeks?",
];

pub(super) const BODY_FREQUENCY_KEPT: &[&str] = &[
    "{often}, then. That is enough to get it looked at if it keeps on, and a note of when it is worst helps.",
    "{often} is a lot. Keep a note of what sets it off and what you have tried, and take that to the GP.",
    "{often} is your body asking for a proper look. Book the GP and take a week of notes with you.",
];

pub(super) const A_CALL_ASK: &[&str] = &[
    "111", "999", "call", "contact", "dentist", "doctor", "gp", "hospital", "nurse", "phone",
    "ring", "see", "seeing", "surgery", "vet",
];
