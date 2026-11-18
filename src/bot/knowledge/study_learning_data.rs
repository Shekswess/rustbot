//! Pinned assessment, revision, and course wording and selectors.

pub(super) const TOP_GRADES: &str =
    r#"(?i)\b(?:[2-9]|1[0-2])\s+a[\s*-]?stars?\b|\b(?:straight|all)\s+(?:9s|8s)|\b[2-9]\s+a\*s\b"#;
pub(super) const SCORE: &str = r#"\b(\d{1,3})\b[^0-9,]{0,30}?\b(?:out of|/)\s*(\d{1,3})\b"#;
pub(super) const GRADE_NAME: &str =
    r#"\A(?:^(?:a[\s*-]?stars?|a\*s?|grades?|gcses?|a[\s-]?levels?|btecs?)\b)"#;
pub(super) const LETTER_GRADES: &str = r#"\A(?:^(?:as|bs|cs|ds|es|fs|us)(?:$|\s+(?:and|in)\b))"#;
pub(super) const TEST_AS_A_VERB: &str = r#"(?i)\btest(?:ed|ing|s)?\s+(?:if|whether|that|the|a|an|my|your|our|his|her|its|this|these|those|any|some|for|out|with|by)\b"#;
pub(super) const TIME_AFTER_TEST: &str = r#"(?i)\btest\s+(?:a|an|one|two|three|\d+)\s+(?:week|weeks|day|days|month|months|year|years|fortnight|hour|hours|minute|minutes)\b"#;
pub(super) const LATER_STUDY: &str = r#"\b(?:course|courses|college|university|uni|degree|gcse|gcses|a[ -]?levels?|night school|evening class|evening classes|access course|study|studying|lessons?|retrain|retraining|qualification|qualifications|apprenticeship|masters|phd|bookkeeping|nursing|tutor|lectures?|homework|exams?|revision|traineeship|nvq)\b"#;
pub(super) const CAREER_CHANGE: &str = r#"\b(?:too old|too late)\b[^.?!]*\b(?:become|becoming|train as|training as|retrain as|retraining as|qualify as|qualifying as|career change|change career|change jobs|switch careers?|switch to|move into|go into|get into)\b"#;
pub(super) const A_MARK_NOT_A_GRADE: &[&str] = &["out", "of", "than", "per", "over", "under"];
pub(super) const COUNTED_AFTER: &[&str] = &[
    "marks",
    "mark",
    "out",
    "%",
    "percent",
    "subjects",
    "hours",
    "years",
    "days",
    "weeks",
    "months",
    "kids",
    "children",
    "grandchildren",
    "questions",
    "pages",
    "miles",
    "minutes",
    "times",
    "weeks",
    "gcse",
    "gcses",
];
pub(super) const LEARNING_OBJECTS: &[&str] = &[
    "a-level",
    "alevel",
    "biology",
    "chemistry",
    "chinese",
    "comprehension",
    "english",
    "exam",
    "exams",
    "french",
    "gcse",
    "geography",
    "german",
    "grammar",
    "history",
    "italian",
    "knew",
    "know",
    "knowledge",
    "knows",
    "learned",
    "learning",
    "learnt",
    "math",
    "maths",
    "memory",
    "mock",
    "mocks",
    "paper",
    "physics",
    "reading",
    "remember",
    "remembered",
    "remembers",
    "revise",
    "revised",
    "revising",
    "revision",
    "science",
    "spanish",
    "spelling",
    "spellings",
    "subject",
    "subjects",
    "syllabus",
    "tenses",
    "topic",
    "topics",
    "vocab",
    "vocabulary",
];
pub(super) const NERVES_BODY: &[&str] = &[
    "feel sick",
    "feeling sick",
    "felt sick",
    "am sick with nerves",
    "sick with nerves",
    "ill with nerves",
    "feel ill",
    "queasy",
    "butterflies",
    "stomach turns",
    "stomach turning",
    "stomach churns",
    "stomach churning",
    "churning stomach",
    "tummy turns",
    "gut churning",
    "cant eat",
    "can't eat",
    "cannot eat",
    "dry mouth",
    "shaky hands",
    "hands shake",
    "shaking hands",
    "shake all over",
    "shaking all over",
    "legs like jelly",
    "heart racing",
    "heart races",
    "heart pounding",
    "pounding heart",
    "sweaty palms",
    "clammy",
    "cant breathe",
    "can't breathe",
    "need the loo",
    "running to the loo",
    "bright red",
    "go red",
    "blush",
];
pub(super) const EXAM_NERVES: &str = "Going blank is your body, not your memory: breathe out longer than you breathe in for a minute and it settles. Then read the whole paper and start with the question you know best, and if a question stalls, jot the first line you can and move on. Do not spend the first five minutes trying to feel calm — just start writing.";
pub(super) const COURSE_HOURS: &str = "Budget two to three hours of reading and homework for every hour in class: a three-hour evening class is usually 9 to 12 hours a week including the class, and a part-time degree is 16 to 20 hours a week because it is half of a full-time course. The college has to publish those guided-learning hours on the course page, so ask for them before you enrol and pick the two evenings you would protect. Reading is what gets dropped first, so put it in the diary as an appointment rather than a hope.";
pub(super) const REVISION: &str = "Start with something tiny today: open the past paper or the topic list and write down what you already know about one topic — ten minutes, that is the whole job. Then work backwards from the exam date and give each topic a day, in 25-minute blocks with a 5-minute break. The last week should be past papers under time, not reading notes.";
pub(super) const SIGN_PLURAL: &[&str] = &[
    "Butterflies",
    "Clammy hands",
    "Legs like jelly",
    "Shaky hands",
];
pub(super) const SIGNS: &[(&[&str], &str)] = &[
    (
        &[
            "feel sick",
            "feeling sick",
            "felt sick",
            "am sick",
            "sick with nerves",
            "ill with nerves",
            "feel ill",
            "feeling ill",
            "nauseous",
            "nausea",
        ],
        "Feeling sick",
    ),
    (
        &[
            "queasy",
            "stomach turns",
            "stomach turning",
            "stomach churns",
            "stomach churning",
            "churning stomach",
            "tummy turns",
            "gut churning",
        ],
        "That churning stomach",
    ),
    (&["butterflies"], "Butterflies"),
    (
        &["cant eat", "can't eat", "cannot eat"],
        "Not being able to eat",
    ),
    (&["dry mouth"], "A dry mouth"),
    (
        &["shaky hands", "hands shake", "shaking hands"],
        "Shaky hands",
    ),
    (
        &["shake all over", "shaking all over", "shaking", "shaky"],
        "That shaking",
    ),
    (
        &["legs like jelly", "jelly legs", "weak at the knees"],
        "Legs like jelly",
    ),
    (
        &[
            "heart racing",
            "heart races",
            "heart pounding",
            "pounding heart",
            "heart thumping",
        ],
        "A racing heart",
    ),
    (&["sweaty palms", "sweaty hands", "clammy"], "Clammy hands"),
    (
        &["cant breathe", "can't breathe", "hard to breathe"],
        "That tight chest",
    ),
    (&["need the loo", "running to the loo"], "Needing the loo"),
    (
        &["bright red", "go red", "going red", "blush", "blushing"],
        "Going red",
    ),
];
pub(super) const GRADES: [&str; 9] = [
    "A 1 is below a pass at GCSE. Most sixth forms and colleges take you on other subjects and have you retake English or maths alongside, and that retake is free until you are 18, so it does not close anything on its own.",
    "A 2 is below a pass at GCSE. Most sixth forms and colleges take you on other subjects and have you retake English or maths alongside, and that retake is free until you are 18, so it does not close anything on its own.",
    "A 3 is below a pass at GCSE. Most sixth forms and colleges take you on other subjects and have you retake English or maths alongside, and that retake is free until you are 18, so it does not close anything on its own.",
    "A 4 is a standard pass, the old C: you are over the line most colleges ask for, and a 5 is the strong pass above it.",
    "A 5 is a strong pass, the old high C, and over the line most colleges ask for.",
    "A 6 is a good pass, about the old B.",
    "A 7 is a top grade, the old A and above.",
    "An 8 is a top grade, the old A and above.",
    "A 9 is the top grade, above the old A*.",
];
pub(super) const DECIMAL_ZEROES: &[u32] = &[
    0x660, 0x6f0, 0x7c0, 0x966, 0x9e6, 0xa66, 0xae6, 0xb66, 0xbe6, 0xc66, 0xce6, 0xd66, 0xde6,
    0xe50, 0xed0, 0xf20, 0x1040, 0x1090, 0x17e0, 0x1810, 0x1946, 0x19d0, 0x1a80, 0x1a90, 0x1b50,
    0x1bb0, 0x1c40, 0x1c50, 0xa620, 0xa8d0, 0xa900, 0xa9d0, 0xa9f0, 0xaa50, 0xabf0, 0xff10,
    0x104a0, 0x10d30, 0x10d40, 0x11066, 0x110f0, 0x11136, 0x111d0, 0x112f0, 0x11450, 0x114d0,
    0x11650, 0x116c0, 0x116d0, 0x116da, 0x11730, 0x118e0, 0x11950, 0x11bf0, 0x11c50, 0x11d50,
    0x11da0, 0x11f50, 0x16130, 0x16a60, 0x16ac0, 0x16b50, 0x16d70, 0x1ccf0, 0x1d7ce, 0x1d7d8,
    0x1d7e2, 0x1d7ec, 0x1d7f6, 0x1e140, 0x1e2f0, 0x1e4f0, 0x1e5f1, 0x1e950, 0x1fbf0,
];

pub(super) const NOT_AGE_STUDY: &str = r#"\b(?:baby|babies|pregnan\w*|ivf|fertility|children|kids|puppy|puppies|dog|dogs|cat|cats|driving|drink\w*|tattoo|piercing|marathon|gym|weight|surgery|operation|piano|guitar|violin|viola|cello|drums|instrument|instruments|keyboard|ukulele|flute|business|businesses|startup|startups|company|shop|restaurant|cafe|dad|father|mum|mom|mother|parent|parents|grandma|grandmother|grandad|grandfather|footballer?|athlete|olympic|rugby|boxer|boxing|singer|dancer|actor|actress|model|influencer|youtuber|streamer)\b"#;
pub(super) const AGE_ASK: &str = r#"\b(?:too old|too late|at my age|my age|the age|an age|age matter|age matters|age stop|age a problem|age hold|can i still|could i still|is it still|still worth|worth it at|at \d{2}|in my (?:thirties|forties|fifties|sixties|seventies))\b"#;
pub(super) const AGE_NUMBER: &str = r#"\b(\d{2})\b"#;
pub(super) const STUDY_AGE: &str = "evening classes, access courses, apprenticeships and part-time degrees are full of people in their thirties, forties and fifties, and the ones who finish are the ones with a protected evening rather than the fastest readers. What decides it is the routine — one fixed evening in class and one more for reading, kept for the length of the course — and a subject you will still open on a bad week. Ask the college what share of last year's class finished, because these courses are lost to work and life far more than to age.";
pub(super) const STUDY_AGE_OLDER: &str = "and that is normal on these courses: what decides it is not the age but the routine - one fixed evening in class and one more for reading, kept for the length of the course - and a subject you will still open on a bad week. Ask the college what share of last year's class finished, and whether any of it can be done in the daytime or online, because that is what makes it stick.";
