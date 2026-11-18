//! Original unit and figure forms for amounts stated in a PyBot answer.

pub(super) const FIGURE_ASK: &str = r#"(?i)^\W*(?:(?:and|but|so|ok|okay|right|then|oh|well)\W+)*how\s+(?P<asked>many|much|long|far)\b(?P<rest>.*)$"#;
pub(super) const ACTION_DURATION: &str = r#"(?i)\bat a time\b|\bfor the first\b|\bfor the next\b|\bin a row\b|\beach time\b|\bthe (?:first|next|last|coming|following|remaining|final)\b|\b(?:once|twice|three times|a few times) a day\b|\ba day or two\b|\bper session\b"#;
pub(super) const ACTION_AMOUNT: &str =
    r#"(?i)\b(?:whenever|when|if|once|each time|any time|every time|in case|unless)\b"#;
pub(super) const A_MONEY_FIGURE: &str = r#"(?i)(?:£|\$|€)\s*\d[\d,]*(?:\.\d+)?(?:\s*(?:to|-|or)\s*(?:£|\$|€)?\s*\d[\d,]*(?:\.\d+)?)?|(?<![\d.,])\d[\d,]*(?:\.\d+)?(?:\s*(?:to|-|or)\s*\d[\d,]*(?:\.\d+)?)?\s*(?:pounds?|quid|dollars?|euros?|pence)\b"#;
pub(super) const A_PERCENT_FIGURE: &str = r#"(?i)(?<![\d.,])\d[\d,]*(?:\.\d+)?(?:\s*(?:to|-|or)\s*\d[\d,]*(?:\.\d+)?)?\s*(?:per\s?cent|percent|%)"#;
pub(super) const LISTED_FIGURE: &str = r#"(?i)(?:,|;)\s*\d+(?:\s*(?:to|-|or)\s*\d+)?\s+for\b"#;
pub(super) const A_RATE_ASK: &str =
    r#"(?i)how\s+much\b[^.?!]*\b(?:a|per|each|every)\s+(?:day|week|month|year)\b"#;
pub(super) const A_RATE_STATED: &str = r#"(?i)(?:£|\bgbp\s*)?\d[\d,.]*\s*(?:pounds?\s+)?(?:a|per|each|every)\s+(?:day|week|month|year)\b"#;
pub(super) const FIGURE_QUALIFIER: &str = r#"(?:about|around|roughly|nearly|almost|up to|at least|over|under|just|only|maybe|probably|another)"#;
pub(super) const FIGURE_NUMBER: &str = r#"(?:\d{1,3}(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?|a|an|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|ninety|hundred)"#;
pub(super) const FIGURE_FILLER: &[&str] = &[
    "a",
    "about",
    "after",
    "again",
    "all",
    "altogether",
    "an",
    "and",
    "approximately",
    "are",
    "around",
    "at",
    "be",
    "before",
    "but",
    "can",
    "could",
    "did",
    "do",
    "does",
    "each",
    "exactly",
    "for",
    "guess",
    "had",
    "has",
    "have",
    "he",
    "here",
    "i",
    "if",
    "in",
    "is",
    "it",
    "its",
    "less",
    "long",
    "many",
    "more",
    "much",
    "now",
    "of",
    "on",
    "overall",
    "per",
    "please",
    "reckon",
    "roughly",
    "say",
    "she",
    "should",
    "so",
    "suppose",
    "that",
    "the",
    "them",
    "then",
    "there",
    "they",
    "think",
    "this",
    "those",
    "time",
    "times",
    "to",
    "told",
    "total",
    "was",
    "we",
    "were",
    "when",
    "will",
    "would",
    "you",
];
pub(super) const FIGURE_AFTER: &[&str] = &[
    "a", "about", "after", "an", "and", "are", "around", "at", "before", "but", "by", "each",
    "every", "for", "from", "if", "in", "into", "is", "now", "of", "on", "or", "per", "roughly",
    "so", "than", "that", "the", "then", "till", "to", "until", "was", "were", "when", "where",
    "which", "while", "with", "without",
];
pub(super) const FIGURE_TIME: &[&str] =
    &["week", "day", "month", "year", "hour", "minute", "second"];
pub(super) const ANSWER_MODULES: &[&str] = &["answers", "grief", "loss", "safety", "urgent"];
pub(super) const FIGURE_ANSWER_LEN: usize = 120;
pub(super) const LOOK_BACK_FIGURE: usize = 3;
pub(super) const FIGURE_UNITS: &[(&str, &[&str])] = &[
    ("week", &["fortnight", "fortnights", "week", "weeks"]),
    ("day", &["day", "days"]),
    ("month", &["month", "months"]),
    ("year", &["year", "years"]),
    ("hour", &["hour", "hours"]),
    ("minute", &["minute", "minutes"]),
    ("second", &["second", "seconds"]),
    ("mile", &["mile", "miles"]),
    (
        "km",
        &["kilometer", "kilometers", "kilometre", "kilometres", "km"],
    ),
    ("kilo", &["kg", "kilo", "kilogram", "kilograms", "kilos"]),
    ("stone", &["stone", "stones"]),
    ("pound", &["lb", "lbs", "pound", "pounds"]),
];
