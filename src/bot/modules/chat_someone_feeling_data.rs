// Feeling shapes and wording about other people in the pinned chat module.

pub(super) const SHE_WORDS: &[&str] = &[
    "aunt",
    "daughter",
    "girlfriend",
    "gran",
    "grandma",
    "granny",
    "mom",
    "mother",
    "mum",
    "nan",
    "she",
    "sister",
    "wife",
];
pub(super) const HE_WORDS: &[&str] = &[
    "boyfriend",
    "brother",
    "dad",
    "father",
    "grandad",
    "grandpa",
    "he",
    "husband",
    "son",
    "uncle",
];
pub(super) const PLURAL_WORDS: &[&str] = &[
    "boys",
    "children",
    "family",
    "girls",
    "grandparents",
    "kids",
    "little ones",
    "neighbors",
    "neighbours",
    "parents",
    "they",
    "twins",
];
pattern!(
    OTHER_BAD_NIGHT,
    r#"\b(?:slept (?:badly|terribly|awfully|rotten|horribly|awful|terrible|rubbish|lousy|poorly)|(?:hardly|barely|didn'?t|didnt|couldn'?t|no) (?:any )?(?:slept|sleep)|up all night|awake all night|(?:bad|terrible|awful|rubbish|rotten|rough|poor|lousy|restless|broken|sleepless) nights?|nights sleep)\b"#
);
pattern!(
    A_HELP_ASK_AT_END,
    r#"\b(?:what|how)\s+(?:can|could|should|do)\s+(?:i|we)\s+(?:do|help)\b(?:\s+(?:about|for|with)\s+(?:it|this|that|her|him|them|us|me)|\s+to\s+help(?:\s+(?:her|him|them|me|us))?(?:\s+out)?|\s+(?:her|him|them|me|us)(?:\s+out)?)?\s*[.!?]*$"#
);
pattern!(
    OTHER_STOP,
    r#"\b(?:floppy|limp|unresponsive|breath\w*|blue|fever|temperature|rash|sick|vomit\w*|diarr?h\w*|pain|hurt\w*|bleed\w*|collapse\w*|seizure|fit|stroke|hospital|doctor|gp|surgery|ambulance|infection|swell\w*|forget\w*|memory|dementia|confus\w*|dizzy|faint\w*|chest|blood|pregnan\w*|die[ds]?|death|passed away|cancer|chemo\w*|radiotherap\w*|dialysis|diagnos\w*|operation|recover\w*|hospice|terminal|palliative|parkinson\w*|tumou?r|transplant|consultant)\b"#
);
pattern!(
    OTHER_QUESTION,
    r#"\b(?:is|are|was|were|should|shall|can|could|do|does|did|will|would)\s+(?:i|we|you|they|he|she|it|that|this|my|the)\b"#
);
pattern!(CRY_WOLF, r#"(?i)\bcries?\s+wolf\b|\bcrying\s+wolf\b"#);
pattern!(
    OTHER_PERSON,
    r#"\b(?:my|our|the)\s+(wife|mum|mom|mother|dad|father|son|daughter|husband|partner|boyfriend|girlfriend|brother|sister|friend|mate|flatmate|boss|colleague|nan|granny|gran|grandma|grandad|grandpa|aunt|uncle|kid|kids|child|children|baby|parents|family|neighbours?|neighbors?)\b"#
);
pattern!(OTHER_PRONOUN, r#"^(?:he|she|they)(?:'s|s|'re|re)?\b"#);
pattern!(
    OTHER_ADVERB,
    r#"\b(?:really|very|so|quite|pretty|totally|completely|always|a bit|a lot|just|still|properly|proper|too|rather|extremely|incredibly|absolutely|constantly|getting|seems|looking|feeling)\b"#
);
pub(super) const OTHER_FEELING_VERB: &[&str] = &[
    "blub", "blubbed", "blubs", "cried", "cries", "cry", "crying", "sob", "sobbed", "sobbing",
    "sobs", "weep", "weeping", "weeps", "wept",
];
pattern!(
    OTHER_VERB,
    r#"\b(?:is|are|was|were|seems?|feels?|looks?|sounds?|been|gets?|got|being|keeps?|has|have)\b"#
);
pub(super) const WEARS_OTHER_PEOPLE_OUT: &[&str] =
    &["draining", "exhausting", "knackering", "tiring"];
pattern!(
    OTHER_NEG,
    r#"\b(?:not|never|no|hardly|barely)\b|\b\w+n't\b|\b(?:isnt|arent|wasnt|werent|dont|doesnt|didnt|cant|couldnt|wouldnt|shouldnt|hasnt|havent|aint)\b"#
);
pattern!(
    ROUGH_DAY,
    r#"\b(bad|rough|long|terrible|awful|rubbish|crap|hard|tough|mental|hectic|horrible|shitty|draining|mad|full|wild|crazy)\s+(day|week|morning|night|shift|one)\b|\b(?:today|the (?:day|morning|night|shift)|my (?:day|morning|night|shift)|this (?:day|morning|night|shift))(?:\s+(?:was|is|has been|had been)|['\u2019]?s been)\s+(?:so |really |just |pretty |very |a bit |quite |such a )?(?:long|slow|endless|never[- ]ending)\b(?:\s+(?:today|tonight|again|as well))?|\b(?:work|the day at work)(?:\s+(?:was|is|has been|had been)|['\u2019]?s been)\s+(?:so |really |just |pretty |very |a bit |quite |such a )?(?:long|endless|never[- ]ending)\b(?:\s+(?:today|tonight|again|as well))?|\b(?:i|we)\s+(?:was|were)\s+(?:at|in)\s+work\s+(?:all (?:day|night|evening|morning)|till|til|until|late)\b|\bone of those (?:days|mornings|nights)\b"#
);
pub(super) const HOW_MUCH_OF_IT: &[&str] = &[
    "awful", "bit", "complete", "little", "lot", "mite", "mood", "proper", "right", "tad",
    "terrible", "total", "touch", "wee",
];
pattern!(
    JOINED_PERSON,
    r#"\band\s+(?:(?:his|her|their|my|our)\s+)?(?:wife|husband|partner|brother|sister|son|daughter|mum|mom|mother|dad|father|kids?|children|parents?|family|friend|mate|boyfriend|girlfriend)\b"#
);
pattern!(
    THING_THAT_STAYS,
    r#"^(?:my|his|her|their|our|the|this|that|these|those)\b"#
);
pub(super) const FEDUP_GERUND_SAFE: &[&str] =
    &["What is it about {thing} that has worn {obj} out?"];
pub(super) const FEDUP_REPLIES: &[&str] = &[
    "{lead} {thing} is its own kind of tired. What's the worst part of it for {obj}?",
    "{lead} {thing} wears {obj} down more than a late night does. What would have to change?",
    "What is it about {thing} that has worn {obj} out?",
];
pub(super) const GENERIC_OTHER_REPLIES: &[&str] = &[
    "That's a lot for {obj} to be carrying. What's been happening?",
    "Sounds like {subj} has something heavy on {poss} mind. Is {subj} talking about it?",
];
pub(super) const OTHER_REPLIES_SMALL_CHILD: &[&str] = &[
    "A little one that upset is hard on everyone. What happens just before it starts?",
    "That is a lot for a small child. Is it worse at a particular time of day?",
    "Young children feel things hard and have no other way to say it yet. Does anything set it off?",
];
pub(super) const OTHER_REPLIES: &[(&str, &[&str])] = &[
    (
        "annoyed",
        &[
            "That would wind anyone up. Is there anything you can do about it?",
            "Fair enough for {obj} to be cross. What happened?",
        ],
    ),
    (
        "anxious",
        &[
            "That sounds rough for {obj}. Do you know what's behind it?",
            "Worry that runs on is tiring in itself. A walk together usually helps more than advice.",
        ],
    ),
    (
        "bored",
        &[
            "Boredom is easier to fix than most. What used to keep {obj} busy?",
            "Worth trying something new together — a walk, a class, anything with a fixed time.",
        ],
    ),
    (
        "happy",
        &[
            "Lovely to hear. What's made {obj} so happy?",
            "Good — that's worth celebrating.",
        ],
    ),
    (
        "lonely",
        &[
            "Loneliness is hard to watch. Is there someone nearby who could keep {obj} company?",
            "That's rough on {obj}. A visit or a phone call this week would probably mean a lot.",
        ],
    ),
    (
        "relieved",
        &[
            "That's a weight off {poss} mind. Did {subj} say what it was about?",
            "Relief like that usually follows something heavy. {Subj_has} {subj} had a rough time of it?",
            "Good - that sounds like it had been sitting on {obj} a while. What changed?",
        ],
    ),
    (
        "sad",
        &[
            "Low patches are hard to watch. Do you know what's behind it?",
            "That's a lot for {obj} to carry. Small things done together often help more than words.",
        ],
    ),
    (
        "stressed",
        &[
            "That sounds like a lot on {poss} {plate}. Is there anything you can lift for {obj}?",
            "Stress that runs on needs something to change, not just a rest. What's driving it?",
        ],
    ),
    (
        "tired",
        &[
            "Tired like that is hard to watch. Is there any chance for {obj} to rest?",
            "That's a lot of tired {target}. Worth taking something off {poss} {plate} this week.",
            "That deep tiredness usually has a cause. Worth getting it looked at if it does not lift.",
        ],
    ),
    (
        "unappreciated",
        &[
            "That is a slow one to watch. Is there one job {subj} could hand over this week?",
            "Being taken for granted wears anybody down. What would {subj} want noticed first?",
        ],
    ),
];
pub(super) const ABOUT_REPLIES: &[(&str, &[&str])] = &[
    (
        "annoyed",
        &[
            "That would wind anyone up. What's the worst part of it for {obj}?",
            "Fair enough for {obj} to be cross about it. {Subj_has} {subj} said anything to them?",
        ],
    ),
    (
        "anxious",
        &[
            "Nerves with a real cause in front of them are easier to talk about. What {subj_does} {subj} say the worst of it is?",
            "That's a big thing for {obj} to have in {poss} head all day. {Subj_is} {subj} talking about it?",
        ],
    ),
    (
        "guilty",
        &[
            "Feeling bad about something real is not the same as being to blame. What {subj_does} {subj} say about it?",
            "That's a heavy thing for {obj} to be carrying. {Subj_is} {subj} talking about it?",
        ],
    ),
    (
        "happy",
        &[
            "Lovely that {subj} {subj_is} pleased about it. What happened?",
            "Good news with a reason behind it — worth hearing the whole thing.",
        ],
    ),
    (
        "relieved",
        &[
            "Something real came off {poss} shoulders there. What {subj_does} {subj} say about it?",
            "That's a real weight to lose. {Subj_is} {subj} talking about what {subj} went through?",
        ],
    ),
    (
        "sad",
        &[
            "Sad about something that really happened needs room rather than fixing. {Subj_is} {subj} talking about it?",
            "That's a real thing for {obj} to be sad about. What {subj_does} {subj} say about it?",
        ],
    ),
    (
        "stressed",
        &[
            "That's a real weight to have on {poss} {plate}. What part of it is the worst?",
            "Stress with a cause in front of it is worth naming. What {subj_does} {subj} say about it?",
        ],
    ),
];
pub(super) const FEDUP_LEAD: &[(&str, &str)] = &[
    ("browned off", "Being browned off with"),
    ("browned off of", "Being browned off of"),
    ("browned off with", "Being browned off with"),
    ("done with", "Being done with"),
    ("fed up", "Being fed up with"),
    ("fed up of", "Being fed up of"),
    ("fed up with", "Being fed up with"),
    ("had enough of", "Having had enough of"),
    ("had it with", "Having had it with"),
    ("peeved", "Being peeved about"),
    ("peeved about", "Being peeved about"),
    ("peeved at", "Being peeved at"),
    ("peeved with", "Being peeved with"),
    ("sick and tired of", "Being sick and tired of"),
    ("sick of", "Being sick of"),
    ("tired of", "Being tired of"),
];
pub(super) const FEELINGS: &[(&str, &[&str])] = &[
    (
        "tired",
        &[
            "tired",
            "exhausted",
            "knackered",
            "shattered",
            "wiped",
            "sleepy",
            "bushed",
            "drained",
            "shattered",
            "done in",
            "no energy",
            "worn out",
            "exhausting",
            "draining",
            "tiring",
            "knackering",
            "wearing out",
            "beat",
            "spent",
            "zonked",
            "pooped",
            "dead on my feet",
            "fagged",
            "wrecked",
            "worn down",
            "used up",
            "like a zombie",
            "a zombie",
            "zombie",
        ],
    ),
    (
        "rough_day",
        &[
            "hungover",
            "hangover",
            "hang over",
            "hung over",
            "jet lagged",
            "jetlagged",
            "jet lag",
            "cream crackered",
            "crackered",
        ],
    ),
    (
        "stressed",
        &[
            "stressed",
            "stress",
            "overwhelmed",
            "swamped",
            "under pressure",
            "too much on",
            "pulled in every direction",
            "snowed under",
            "stressful",
            "overwhelming",
        ],
    ),
    (
        "anxious",
        &[
            "anxious",
            "anxiety",
            "nervous",
            "worried",
            "worry",
            "wound up",
            "on edge",
            "panicking",
            "panic",
            "scared",
            "frightened",
            "restless",
            "terrified",
            "petrified",
        ],
    ),
    (
        "sad",
        &[
            "sad",
            "down",
            "miserable",
            "low",
            "upset",
            "heartbroken",
            "gutted",
            "crying",
            "unhappy",
            "depressed",
            "not ok",
            "not okay",
            "not doing great",
            "not great",
            "not fine",
            "not alright",
            "not so good",
            "not doing well",
            "not feeling great",
            "weepy",
            "tearful",
            "homesick",
            "rotten",
            "lousy",
            "meh",
            "rough",
            "crap",
            "rubbish",
            "numb",
            "sadder",
            "saddest",
            "more miserable",
            "worse than before",
            "cry",
            "cried",
            "cries",
            "weep",
            "weeping",
            "wept",
            "sobs",
            "weeps",
            "sob",
            "sobbing",
            "bawl",
            "bawling",
            "blub",
            "blubbing",
            "tears",
            "in tears",
            "welling up",
            "burst into tears",
            "bursting into tears",
            "tearing up",
            "tear up",
            "choked up",
            "empty",
            "flat",
            "hollow",
            "blank",
            "dead inside",
            "nothing to give",
            "feel nothing",
            "like a failure",
            "like a mess",
            "like rubbish",
            "like crap",
            "like a burden",
            "like a fraud",
            "like nothing matters",
            "a mess",
            "in a bad mood",
            "in a low mood",
            "in a foul mood",
            "in a mood",
            "in a state",
            "in a right state",
            "out of sorts",
            "not myself",
            "not feeling myself",
            "off colour",
            "off form",
        ],
    ),
    (
        "guilty",
        &[
            "guilty",
            "ashamed",
            "embarrassed",
            "my fault",
            "feel bad about",
            "feel awful about",
            "feel terrible about",
            "feel dreadful about",
            "feel horrible about",
            "let them down",
            "let her down",
            "let him down",
            "let my family down",
            "should have been there",
            "wasnt there enough",
        ],
    ),
    (
        "lonely",
        &[
            "lonely",
            "alone",
            "isolated",
            "left out",
            "no one to talk to",
            "on my own",
        ],
    ),
    (
        "unappreciated",
        &["taken for granted", "not appreciated", "unappreciated"],
    ),
    (
        "bored",
        &["bored", "boring", "nothing to do", "nothing to do"],
    ),
    (
        "annoyed",
        &[
            "annoyed",
            "angry",
            "furious",
            "pissed",
            "irritated",
            "frustrated",
            "fed up",
            "cross",
            "livid",
            "raging",
            "mad",
            "grumpy",
            "cheesed off",
            "fuming",
            "miffed",
            "mardy",
            "browned off",
            "peeved",
            "stroppy",
            "stroppy with",
            "in a huff",
        ],
    ),
    (
        "relieved",
        &[
            "relieved",
            "relief",
            "such a relief",
            "what a relief",
            "weight off my shoulders",
            "weight off my mind",
        ],
    ),
    (
        "happy",
        &[
            "happy",
            "great",
            "glad",
            "excited",
            "thrilled",
            "chuffed",
            "delighted",
            "buzzing",
            "made up",
            "over the moon",
            "brilliant news",
            "good mood",
            "feeling good",
            "feel good",
            "in good form",
            "on good form",
            "cheerful",
            "cheery",
            "in a great mood",
            "great mood",
            "happier",
            "happiest",
            "gladder",
            "gladdest",
        ],
    ),
];
