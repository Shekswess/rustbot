//! Original replies and card vocabulary from PyBot family and family_more.

pub(super) const YEAR_PATTERN: &str = r###"\byear\s*(1[01]|[1-9])\b"###;
pub(super) const HOSPITAL_PATTERN: &str = r###"(?i)\b(?:hospital|ward|icu|intensive care|visiting hours|visiting times|in hospital|in the hospital|care home|nursing home|residential home|hospice)\b"###;
pub(super) const BIRTH_PATTERN: &str = r###"(?i)\b(?:born|gave birth|given birth|had a baby|had her baby|had the baby|new baby|newborn|new arrival|just had)\b"###;
pub(super) const MONEY_ASKERS: &[&str] = &[
    "mum",
    "mother",
    "mam",
    "mom",
    "dad",
    "father",
    "son",
    "daughter",
    "brother",
    "sister",
    "cousin",
    "uncle",
    "aunt",
    "nan",
    "grandma",
    "grandad",
    "stepson",
    "stepdaughter",
    "stepbrother",
    "stepsister",
    "parents",
    "in law",
];
pub(super) const HOW_OFTEN: &[&str] = &[
    "how often",
    "how frequently",
    "how regularly",
    "how many times",
    "often",
    "frequently",
    "regularly",
    "how long between",
    "every how long",
    "frequency",
];
pub(super) const NAPPY_AGES: &[(&[&str], &str)] = &[
    (
        &[
            "3 month",
            "3 months",
            "three month",
            "three months",
            "4 month",
            "4 months",
            "four month",
            "four months",
            "6 week",
            "6 weeks",
            "six week",
            "six weeks",
            "2 month",
            "2 months",
            "two month",
            "two months",
        ],
        "By three months it is about six to eight a day - a pack of 30 every four or five days - and most babies have moved up two sizes by then.",
    ),
    (
        &[
            "6 month",
            "6 months",
            "six month",
            "six months",
            "7 month",
            "7 months",
            "seven month",
            "seven months",
            "8 month",
            "8 months",
            "eight months",
            "9 month",
            "9 months",
            "nine months",
        ],
        "By six months it is about five to seven a day, so a pack of 30 lasts around five days, and most babies are in size 3 or 4 by then.",
    ),
    (
        &[
            "1 year",
            "1 years",
            "one year",
            "12 month",
            "12 months",
            "twelve months",
            "18 month",
            "18 months",
            "eighteen months",
            "toddler",
            "2 year",
            "2 years",
            "two year",
            "two years",
        ],
        "A one year old gets through about four to six a day - two packs of 30 a month - and a toddler nearer four, falling off as potty training starts.",
    ),
];
pub(super) const HURT_FEELINGS: &str = "Say it as your own feeling about one thing, not a list: 'When you said that in front of everyone, it stung and I have been thinking about it.' Then say what you would like instead - 'I would rather you told me on my own' - and leave it there; a question at the end, like 'did you mean it that way?', keeps the door open. Soon and in private beats saving it up, and face to face beats a text if you can manage it.";
pub(super) const FAMILY_ASKING_MONEY: &str = "Decide your line before the next ask rather than in the moment: one clear no with no story - 'I can't lend money any more, even small amounts' - is kinder than a maybe. Anything already lent, ask for once and plainly - 'when can you pay me back?' - separately from that no, because being family does not make it a bill. If it is for something real, pay that bill or buy that thing yourself rather than handing over cash, and say it is a one-off.";
pub(super) const FRIEND_ASKING_MONEY: &str = "Decide your line before the next ask rather than in the moment: one clear no with no story - 'I can't lend money any more, even small amounts' - is kinder than a maybe, and you do not have to explain why. Anything already lent, ask for once and plainly - 'when can you pay me back?' - separately from that no, so the two are not argued together. If you do want to help, give only what you could afford to lose, once, and say that it is a one-off.";
pub(super) const YEAR_GENERIC: &str = "Going up a year is mostly about the run-up: walk the route once before term, check nothing has been outgrown, and move the sleep time back a week early. On the first morning tell them when you will see them again and be there on time - the wobble usually comes at the gate, not in the room.";
pub(super) const YEAR_TAIL: &str = "Before term starts, walk the route once, check nothing has been outgrown, and move the sleep time back a week early. On the first morning tell them when you will see them again and be there on time: the wobble usually comes at the gate, not in the room. If they are nervous, ask what they think will be hard rather than telling them it will be fine.";
pub(super) const NAPPY_OFTEN: &str = "At every feed and straight away after a poo, which for a newborn works out at 10 to 12 a day, and in the night only if there is poo or the nappy is heavy, so nobody gets woken for a wet one. Six or more wet nappies a day is normal and a sign the feeding is going in; fewer than that, or a nappy that is dry when it should not be, is worth a word with the health visitor. Everything within reach first - a fresh nappy, wipes or a damp flannel, a nappy sack and a clean vest - and one hand on the baby the whole time if you are on a raised surface. Unfasten the tabs and fold the front down out of the way, wipe front to back with the old nappy still underneath you, lift both legs by the ankles to clean underneath, then slide the fresh nappy under with the tabs at the back and do it up level with the waist, frills out. Red patches get a barrier cream rather than talc.";
pub(super) const NAPPY_METHOD: &str = "Everything within reach first - a fresh nappy, wipes or a damp flannel, a nappy sack and a clean vest - and one hand on the baby the whole time if you are on a raised surface. Unfasten the tabs and fold the front down out of the way, wipe front to back with the old nappy still underneath you, lift both legs by the ankles to clean underneath, then slide the fresh nappy under with the tabs at the back and do it up level with the waist, frills out. Change at every feed and straight away after a poo - six or more wet nappies a day is normal in a newborn - and in the night only if there is poo or the nappy is heavy, so nobody gets woken for a wet one. A newborn gets through 10 to 12 a day, so a pack of 30 lasts about three days - buy by the month, in two sizes rather than one. Red patches get a barrier cream rather than talc.";
pub(super) const NAPPY_NEWBORN: &str = "A newborn gets through eight to ten a day - about seventy a week, so two big packs a month or a subscription. It drops to around six to seven a day by six months and four or five by a year. Buy one pack of newborn size and one of the next size up: they grow out of newborn in a fortnight, and a size that leaks is usually too small.";
pub(super) const NEWBORN_VISIT: &str = "Ask them when they are ready for visitors rather than turning up: the first fortnight is for feeding and sleep, and a short visit at a time they choose is the one they can say yes to. If the baby is still in hospital, ring the ward for its visiting hours.";
pub(super) const WARD_VISIT: &str = "Ring the ward before you set off: visiting hours are set ward by ward and change, many open late morning or early afternoon until about 8pm. Ward rounds, meals and handovers are usually closed, and two at the bedside at a time is the common limit. Don't visit with a cough, cold or stomach bug, use the hand gel both ways, and leave flowers at home: most wards ban them.";
pub(super) const YEAR_READING: &str =
    "Ten minutes of reading aloud a day is still the thing that pays off most.";
pub(super) const YEAR_WORK: &str = "Ten minutes of reading a day, and a set time for homework from the first week - the work gets longer this year, and the habit is what carries it.";
pub(super) const YEAR_BUILDING: &str = "Most of the first term is the new building and the timetable: pack the night before and know the bus times, and let them go in knowing nobody else does either.";
pub(super) const YEAR_EXAMS: &str =
    "These are the exam years, so a set time and place for work from September saves the spring.";

// Direct feeding and hitting branches from family.py.
pub(super) const NOT_A_BITE_PATTERN: &str = r###"\b(?:a|an|the|quite|fair|little|wee|tiny)\s+bit\b|\bbit\s+(?:of|more|less|much|tired|better|worse|late|long|early|shy|rough|quiet|loud|sad|scared|worried|unwell|poorly|grumpy|hyper|older|younger|bigger|smaller|behind|ahead)\b"###;
pub(super) const NO_CAFFEINE_PATTERN: &str = r###"(?i)\bdecaf\b|\bdecaffeinated\b|\bcaffeine[- ]free\b|\bno caffeine\b|\bherbal\b|\bpeppermint\b|\bcamomile\b|\bchamomile\b|\brooibos\b|\bfennel\b|\bnettle\b|\bfruit tea\b|\blemon and ginger\b"###;
pub(super) const CAFFEINE_PATTERN: &str = r###"(?i)\b(?:coffee|coffees|espresso|latte|latte?s|cappuccino|flat white|americano|macchiato|mocha|filter|instant|beans|ground|nespresso|pods?|tea|teas|chai|matcha|earl grey|english breakfast|breakfast tea|black tea|green tea|energy drink|energy drinks|cola|coke|pepsi|irn bru|red bull|monster|caffeine)\b"###;
pub(super) const OTHER_DRINKS: &[&str] = &[
    "milk",
    "squash",
    "juice",
    "orange juice",
    "fizzy",
    "lemonade",
    "cordial",
    "smoothie",
    "coconut water",
    "milkshake",
];
pub(super) const WATER_WORDS: &[&str] = &[
    "water",
    "tap water",
    "a glass of water",
    "glasses of water",
    "fluids",
    "fluid",
    "hydration",
    "hydrated",
    "thirsty",
    "thirst",
    "drinking enough",
];
pub(super) const HURTING_WORDS: &[(&[&str], &str, &str)] = &[
    (
        &["bite", "bites", "bit", "bitten", "biting"],
        "biting",
        "'No - biting hurts'",
    ),
    (
        &["kick", "kicks", "kicked", "kicking"],
        "kicking",
        "'No - kicking hurts'",
    ),
    (
        &[
            "push", "pushes", "pushed", "pushing", "shove", "shoves", "shoved", "shoving",
        ],
        "pushing",
        "'No - no pushing'",
    ),
    (
        &[
            "snatch",
            "snatches",
            "snatched",
            "snatching",
            "grabs",
            "grabbing",
            "grabbed",
        ],
        "snatching",
        "'No - we do not snatch'",
    ),
    (
        &[
            "hit", "hits", "hitting", "smacked", "smacking", "smacks", "slapped", "slapping",
            "slaps", "thumped", "thumping",
        ],
        "hitting",
        "'No - hitting hurts'",
    ),
];
pub(super) const WATER_REPLY: &str = "Tap water is the right drink and there is no limit to count on it: about two litres a day is plenty, and forcing extra fluid down does not make more milk. Keep a glass beside the chair you feed in, because the let-down makes you thirsty, and drink whenever you are dry.";
pub(super) const DRINKS_REPLY: &str = "Drink to thirst: about two litres a day suits most people, and keep a glass beside the chair you feed in, because the let-down makes you thirsty. Water, milk, squash, juice and fizzy drinks are all fine, and forcing extra fluid down does not make more milk. The one to count is caffeine: about 200mg a day, which is two mugs of instant.";

// The current question and the last three lines decide the asking out branch.
pub(super) const SAY_TO_SOMEONE_PATTERN: &str =
    r###"(?i)\bsay to (?:(?:a|the|this|that) )?(?:girl|boy|woman|man|someone|her|him|them)\b"###;
pub(super) const NAMES_A_LIKING_PATTERN: &str = r###"(?i)\b(?:i|we)\s+(?:really\s+|quite\s+|kind of\s+|sort of\s+|properly\s+)?(?:like|likes|liked|fancy|fancies|fancied|love|loves|loved)\b|\b(?:have|has|got) a crush\b|\bcrush on\b|\binto (?:her|him|them)\b|\b(?:on|from) (?:a |an |the )?(?:dating app|dating site|tinder|hinge|bumble|app)\b|\bmatched with\b|\b(?:on|had) a date\b|\ba date with\b|\bgot a date\b|\bask(?:ing|ed)? (?:her|him|them) out\b|\bswapped (?:numbers|details)\b|\bgave me (?:her|his) number\b|\bgot (?:her|his) number\b|\bflirt(?:ed|ing)?\b|\bkissed\b"###;
pub(super) const NOT_A_LIKING_PATTERN: &str =
    r###"(?i)\b(?:don'?t|doesn'?t|didn'?t|not|never|hardly)\s+(?:like|fancy)"###;
pub(super) const ASKING_OUT: &str = "Name the thing and the day, then stop talking: \"I've liked talking to you this week - do you want to get a coffee on Saturday?\" A specific plan is far easier to say yes to than \"sometime\", and a plain message you thought about for a minute beats a clever one. If it is a no, take it as an answer and be normal the next day: that is what keeps the friendship and your dignity.";
pub(super) const NOT_WANTING: &str = "Then say that plainly and kindly, once, without a reason they have to argue with: \"I like you as a friend - I didn't want to leave you guessing.\" Keep it short, and be normal with them the next day, which is what makes it land as respect rather than a brush-off.";

pub(super) const YEAR_OR_MORE: &[&str] = &[
    "a year old",
    "one year old",
    "year and a half",
    "18 month",
    "eighteen month",
    "two year old",
    "three year old",
    "toddler",
];

pub(super) const AT_LEAST_SIX: &[&str] = &["half a year", "half a yr"];

pub(super) const MONTH_WORDS: &[(&str, u8)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("ten", 10),
    ("eleven", 11),
    ("twelve", 12),
    ("thirteen", 13),
    ("fourteen", 14),
    ("fifteen", 15),
    ("sixteen", 16),
    ("seventeen", 17),
    ("eighteen", 18),
    ("nineteen", 19),
    ("twenty", 20),
    ("twenty one", 21),
];
