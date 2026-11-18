//! Authored tables from PyBot 7d63c7f, bot/knowledge/entertainment.py.

pub(super) const WET_WORDS: &[&str] = &[
    "rain",
    "rainy",
    "raining",
    "rains",
    "showers",
    "drizzle",
    "drizzling",
    "pouring",
    "downpour",
    "storm",
    "storms",
    "sleeting",
    "sleet",
    "snowing",
];

pub(super) const WET_PHRASES: &[&str] = &[
    "wet day",
    "wet days",
    "wet weekend",
    "wet weekends",
    "wet afternoon",
    "wet weather",
    "wet out",
    "wet one",
];

pub(super) const EVENING_WORDS: &[&str] = &[
    "night in",
    "tonight",
    "evening",
    "at home",
    "indoors",
    "cosy",
    "cozy",
    "this afternoon in",
    "day in",
    "we are staying in",
    "were staying in",
    "im staying in",
    "i am staying in",
    "staying in tonight",
    "stay in tonight",
    "staying in this evening",
    "stay in this evening",
    "staying in all",
];

pub(super) const AN_HOUR: &str = "For an hour, pick one thing you can finish inside it rather than a plan: a walk round the block with a podcast, one drawer or one shelf, a chapter, or the phone call you keep putting off. It ends when the hour does, which is the point. Are you on your own, or is someone with you?";

pub(super) const A_DAY: &str = "One thing out of the house and one thing at home usually fills it. Out: a walk somewhere you have not been, a Saturday market, or a museum or gallery - most are free and good for an hour or two. Home: something to cook properly, a film, or a jigsaw with the radio on. Rain only moves the walk to a pool or a cinema. Is it just you, or is someone coming with you?";

pub(super) const A_SHORT_TIME: &str =
    r#"\b(?:an?|one|two|three|couple of|few) hours?\b|\bhalf an hour\b"#;
