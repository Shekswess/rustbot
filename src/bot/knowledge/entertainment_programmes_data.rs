//! Authored tables from PyBot 7d63c7f, bot/knowledge/entertainment.py.

pub(super) const SHOW_SUBJECTS: &[(&str, &str)] = &[
    (
        "cooking",
        "For cooking: *MasterChef: The Professionals* is the one about the cooking itself, *The Great British Bake Off* for baking and *Saturday Kitchen* for a lazy morning, all of them long-running and easy to start anywhere.",
    ),
    (
        "baking",
        "*The Great British Bake Off* is the gentle one everybody watches, *Bake Off: The Professionals* if you want skill, and *Paul Hollywood* travel shows if it is the bread you care about.",
    ),
    (
        "gardening",
        "*Gardeners' World* is the one that tells you what to do this week, *Beechgrove* for a colder climate and *Love Your Garden* for the before-and-after. Start with Gardeners' World and its monthly jobs.",
    ),
    (
        "nature",
        "*Planet Earth II* is the best of them, *Blue Planet II* for the sea and *Springwatch* for British wildlife in real time. The first two look astonishing on any television.",
    ),
    (
        "history",
        "*The World at War* is the one people still cite, *Time Team* for digging and Lucy Worsley's series for the social side. All three are old and none of them feel dated.",
    ),
    (
        "science",
        "Brian Cox's *The Planets* is the easy way into space, *Forces of Nature* for how the world works, and *Chemistry: A Volatile History* if you would rather have the story of the ideas.",
    ),
    (
        "sport",
        "*The Last Dance* is the one that grips people who do not follow basketball, *Sunderland 'Til I Die* for football and *Welcome to Wrexham* if you want to enjoy one. All three are about the people, not the scores.",
    ),
    (
        "true crime",
        "*The Jinx* is the one with the ending nobody saw coming, *The Staircase* for the long haul, and *Des* for a British one with David Tennant. All three are about the case rather than the killings.",
    ),
    (
        "quiz",
        "*Only Connect* is the cleverest thing on television, *Would I Lie to You?* is the funniest panel show and *University Challenge* the most tense. Start with Would I Lie to You? for a group.",
    ),
    (
        "property",
        "*Grand Designs* for the build, *Location, Location, Location* for the search and *The Great Interior Design Challenge* if you want the rooms. Grand Designs is the one with the arguments.",
    ),
    (
        "travel",
        "*Travel Man* for a city in 48 hours, *Race Across the World* for the long way round and any of Michael Palin's journeys for the old-fashioned version. Travel Man is the short one.",
    ),
    (
        "repair",
        "*The Repair Shop* is the warm one about people mending things that matter to them, *Long Lost Family* for the other kind of mending, and *Salvage Hunters* if you like the hunt.",
    ),
    (
        "documentary",
        "*Free Solo* for one man on a cliff, *The Jinx* for a case that turns, and *Planet Earth II* for the pictures. None of the three needs any background to enjoy.",
    ),
];

pub(super) const SHOW_SUBJECT_NAMES: &[(&str, &str)] = &[
    ("cook", "cooking"),
    ("cooks", "cooking"),
    ("cooking", "cooking"),
    ("cookery", "cooking"),
    ("chef", "cooking"),
    ("chefs", "cooking"),
    ("kitchen", "cooking"),
    ("food", "cooking"),
    ("bake", "baking"),
    ("bakes", "baking"),
    ("baking", "baking"),
    ("bread", "baking"),
    ("cake", "baking"),
    ("cakes", "baking"),
    ("garden", "gardening"),
    ("gardens", "gardening"),
    ("gardening", "gardening"),
    ("gardeners", "gardening"),
    ("allotment", "gardening"),
    ("allotments", "gardening"),
    ("nature", "nature"),
    ("wildlife", "nature"),
    ("animals", "nature"),
    ("planet", "nature"),
    ("history", "history"),
    ("historical", "history"),
    ("war", "history"),
    ("wars", "history"),
    ("science", "science"),
    ("space", "science"),
    ("physics", "science"),
    ("universe", "science"),
    ("sport", "sport"),
    ("sports", "sport"),
    ("football", "sport"),
    ("rugby", "sport"),
    ("cricket", "sport"),
    ("tennis", "sport"),
    ("boxing", "sport"),
    ("racing", "sport"),
    ("crime", "true crime"),
    ("murder", "true crime"),
    ("murders", "true crime"),
    ("detective", "true crime"),
    ("police", "true crime"),
    ("quiz", "quiz"),
    ("quizzes", "quiz"),
    ("panel", "quiz"),
    ("game show", "quiz"),
    ("property", "property"),
    ("properties", "property"),
    ("house", "property"),
    ("houses", "property"),
    ("homes", "property"),
    ("renovation", "property"),
    ("travel", "travel"),
    ("travels", "travel"),
    ("holiday", "travel"),
    ("holidays", "travel"),
    ("repair", "repair"),
    ("repairs", "repair"),
    ("restoration", "repair"),
    ("antiques", "repair"),
];

pub(super) const SHOW_MEDIUM_WORDS: &[&str] = &[
    "show",
    "shows",
    "series",
    "programme",
    "programmes",
    "telly",
    "tv",
    "watch",
    "watching",
    "boxset",
    "box set",
    "netflix",
    "iplayer",
    "documentary",
    "documentaries",
    "docuseries",
    "docu series",
];

pub(super) const DOCUMENTARY_WORDS: &[&str] = &[
    "documentary",
    "documentaries",
    "docuseries",
    "docu series",
    "docu-series",
];

pub(super) const PLAYING_NOT_WATCHING: &str = r#"\b(?:i'?m|im|i am|we'?re|were|we are)\s+(?:playing|training|coaching|refereeing|running|swimming|cycling|rowing|competing|sailing|climbing|dancing)\b|\bnot watch(?:ing)?\b|\b(?:dont|don't) watch\b"#;

pub(super) const KIND_REFUSED_AFTER: &str = r#"(?i)\b(?:is|are|was|were)?\s*(?:not|never)\s+(?:really\s+)?(?:for\s+(?:me|us)|my\s+(?:thing|sort|cuppa|cup|kind)|what\s+i|something\s+i|anything\s+i|a\s+fan|keen|into\s+it)\b|\bis\s+out\b|\bare\s+out\b|\btoo\s+much\b|\bover\s+the\s+top\b"#;

pub(super) const SHOW_YES_OR_NO: &str = r#"(?i)\b(?:is|are) there (?:a|any|anything)\b|\bany good\b|\banything good\b|\bis (?:it|that|this) (?:a )?good\b"#;

pub(super) const SHOW_SUBJECT_LEAD: &str = r#"(?i)^(?:For|On|About) [a-z]+: "#;
