//! Vocabulary taken from the pinned reading orchestrator.
use fancy_regex::Regex;
use std::sync::LazyLock;
pub(super) static ASKS_WHY_IT_IS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^\s*(?:why|how\s+come|what\s+makes|whats\s+making|what'?s\s+making)\b"##)
        .expect("ASKS_WHY_IT_IS")
});
pub(super) static DOES_ALL_THEY_CAN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\beverything\s+(?:i|we)\s+(?:can|could)\b"##).expect("DOES_ALL_THEY_CAN")
});
pub(super) static WORK_SCOPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:at|from|in|to)\s+work\b|\bwork(?:ing)?\s+(?:all|every)\s+(?:day|night)\b"##)
        .expect("WORK_SCOPE")
});
pub(super) static HOME_SCOPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:at|in)\s+(?:home|the\s+house|our\s+house|this\s+house)\b|\bhome\b"##)
        .expect("HOME_SCOPE")
});
pub(super) static DOES_IT_RIGHT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:it|everything|things)\s+(?:all\s+)?(?:right|perfectly|correctly)\b"##)
        .expect("DOES_IT_RIGHT")
});
pub(super) static RUNS_MYSELF_RAGGED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bi\s+(?:run|runs|running|work|working|push|pushing|drive|driving)\s+myself\s+(?:ragged|into\s+the\s+ground|to\s+the\s+bone)\b"##).expect("RUNS_MYSELF_RAGGED")
});
pub(super) static RUNS_THE_HOUSE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bi\s+(?:run|runs|running|manage|managing)\s+(?:the|this|our)\s+(?:house|home|family)\b|\bi'?m\s+(?:the\s+)?only\s+one\s+(?:who\s+)?(?:holds?|holding|keeps?|keeping|runs|running)\b[^.]*\btogether\b|\b(?:i'?m|i\s+am)\s+(?:always\s+|usually\s+)?(?:the\s+)?one\s+who\s+(?:runs?|keeps?|holds?)\s+(?:the|this|our)\s+(?:house|home|family)\b"##).expect("RUNS_THE_HOUSE")
});
pub(super) static HELPS_ME_NOT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r##"\b(?:no ?one|nobody)\s+(?:else\s+)?(?:helps?|helped)\s+me\s+(?:with|at|around)\b"##,
    )
    .expect("HELPS_ME_NOT")
});
pub(super) static LEANS_ON_THEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:everyone|everybody|they all|all of them)\s+(?:leans?|rel(?:y|ies)|depends?|counts?|comes?|come)\s+(?:on|to)\s+(?:me|us)\b"##).expect("LEANS_ON_THEM")
});
pub(super) static DOES_IT_ALL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bi\s+(?:do|handle|manage|carry|hold|sort|run|organise|organize|deal\s+with)\s+(?:it\s+all|everything|the\s+lot|all\s+of\s+it|the\s+whole\s+lot)\b|\bi\s+(?:carry|hold)\s+(?:the\s+)?(?:whole\s+)?(?:house|family)\b|\bi\s+do\s+everything\s+for\s+(?:everyone|everybody|them|the\s+kids)\b"##).expect("DOES_IT_ALL")
});
pub(super) static NO_TIME_OFF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bi\s+(?:never|hardly\s+ever|rarely)\s+get\s+(?:a\s+break|five\s+minutes)\b|\bi\s+(?:never|hardly\s+ever|rarely)\s+get\s+a\s+(?:minute|moment|second)\s+(?:to\s+myself|for\s+myself|off)\b|\bi\s+(?:never|dont|do\s+not)\s+get\s+any\s+time\s+(?:to\s+myself|off|for\s+myself)\b"##).expect("NO_TIME_OFF")
});
pub(super) static ALL_ON_THEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:it'?s|its|it\s+is|everything\s+is|everything'?s)\s+(?:all\s+)?on\s+me\b|\bit'?s\s+all\s+(?:down\s+)?to\s+me\b|\ball\s+on\s+my\s+(?:own\s+)?(?:plate|shoulders|back)\b"##).expect("ALL_ON_THEM")
});
pub(super) static NO_ONE_HELPS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:no ?one|nobody)\s+(?:else\s+)?(?:helps?|helped|helps\s+out|does\s+anything|lifts\s+a\s+finger)\b"##).expect("NO_ONE_HELPS")
});
pub(super) static FIRST_PERSON_ANYWHERE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:i|me|my|mine|myself|we|us|our)\b"##).expect("FIRST_PERSON_ANYWHERE")
});
pub(super) static THIRD_PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:she|he|they|her|his|their|them|him|mum|mom|mother|dad|father|parents|sister|brother|son|daughter|partner|husband|wife|friend|colleague|boss)\b"##).expect("THIRD_PERSON")
});
pub(super) static THE_ONE_WHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\bi'?m\s+(?:always\s+|usually\s+|often\s+)?(?:the\s+|the\s+only\s+)?one\s+(?:who|that)\b|\bi\s+am\s+(?:always\s+)?(?:the\s+|the\s+only\s+)?one\s+(?:who|that)\b"##).expect("THE_ONE_WHO")
});
pub(super) static EVERYTHING_AT_ONCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"\b(?:everything|everyone|everybody|all\s+of\s+it|it\s+all|nothing|nobody|no ?one|the\s+lot|the\s+whole)\b"##).expect("EVERYTHING_AT_ONCE")
});
pub(super) static A_STAND_IN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(the|these|those|my|our|your|his|her|their|both)\s+(?:([a-z][a-z'-]*)\s+)?(ones|one)\b"##).expect("A_STAND_IN")
});
pub(super) const STOPWORDS: &[&str] = &[
    "a", "about", "above", "after", "again", "against", "all", "also", "am", "an", "and", "any",
    "are", "as", "at", "be", "been", "being", "both", "but", "by", "can", "could", "did", "do",
    "does", "doing", "done", "each", "few", "find", "for", "from", "get", "give", "got", "guess",
    "had", "has", "have", "having", "he", "hello", "help", "her", "here", "hers", "hey", "hi",
    "him", "his", "how", "i", "if", "in", "into", "is", "it", "its", "just", "know", "like", "may",
    "me", "might", "more", "most", "must", "my", "myself", "need", "no", "nor", "not", "of", "off",
    "oh", "ok", "okay", "on", "only", "onto", "or", "other", "our", "out", "over", "own", "please",
    "quite", "really", "same", "shall", "she", "should", "show", "so", "some", "sorry", "such",
    "sure", "tell", "than", "thank", "thanks", "that", "the", "their", "theirs", "them", "then",
    "there", "these", "they", "think", "this", "those", "to", "too", "uh", "um", "under", "up",
    "us", "very", "want", "was", "we", "were", "what", "when", "where", "which", "who", "whom",
    "whose", "why", "will", "with", "within", "without", "would", "yeah", "yes", "you", "your",
    "yours",
];
pub(super) const STAND_IN_WORDS: &[&str] = &[
    "back",
    "big",
    "black",
    "blue",
    "bottom",
    "broken",
    "cheap",
    "chipped",
    "clean",
    "cold",
    "cool",
    "cracked",
    "dark",
    "dead",
    "dear",
    "dirty",
    "dodgy",
    "downstairs",
    "dry",
    "empty",
    "expensive",
    "extra",
    "fast",
    "faulty",
    "freezing",
    "front",
    "full",
    "green",
    "hard",
    "heavy",
    "hot",
    "huge",
    "inner",
    "large",
    "leaky",
    "left",
    "light",
    "little",
    "long",
    "loose",
    "loud",
    "middle",
    "missing",
    "narrow",
    "new",
    "noisy",
    "old",
    "outer",
    "pale",
    "quiet",
    "red",
    "right",
    "rusty",
    "short",
    "slow",
    "small",
    "soft",
    "spare",
    "stained",
    "tall",
    "thick",
    "thin",
    "tight",
    "tiny",
    "top",
    "upstairs",
    "warm",
    "wet",
    "white",
    "wide",
    "worn",
    "wrong",
];
pub(super) const STAND_IN_SELECTORS: &[&str] = &[
    "first", "former", "last", "latter", "next", "other", "same", "second", "third",
];
pub(super) const PERSON_HEADS: &[&str] = &[
    "he", "hes", "i", "id", "ill", "im", "ive", "she", "shes", "they", "theyre", "us", "we",
    "were", "weve", "you", "youre",
];
pub(super) const POSSESSIVE: &[&str] = &["her", "his", "my", "our", "their", "your"];
pub(super) const AT_IT_STILL: &[&str] = &["again", "now", "still"];
pub(super) const DID_IT: &[&str] = &[
    "added", "adjusted", "asked", "bought", "came", "changed", "cleaned", "did", "drained",
    "filled", "fixed", "got", "left", "lowered", "made", "moved", "put", "raised", "ran", "read",
    "set", "swapped", "switched", "threw", "told", "took", "tried", "turned", "used", "watched",
    "went",
];
pub(super) const PRAISED: &[&str] = &[
    "amazing",
    "brilliant",
    "fixed",
    "gorgeous",
    "great",
    "lovely",
    "marvellous",
    "marvelous",
    "perfect",
    "perfectly",
    "solved",
    "sorted",
    "spot",
    "working",
    "works",
];
pub(super) const THANKED_FOR: &[&str] = &["about", "again", "for", "of", "so"];
pub(super) const PERSON_NOUNS: &[&str] = &[
    "anger",
    "ankle",
    "ankles",
    "anxiety",
    "appetite",
    "arm",
    "arms",
    "aunt",
    "babies",
    "baby",
    "back",
    "body",
    "boss",
    "boy",
    "boyfriend",
    "boys",
    "brain",
    "brother",
    "budgie",
    "cat",
    "cats",
    "chest",
    "child",
    "children",
    "class",
    "classes",
    "colleague",
    "college",
    "concentration",
    "confidence",
    "cousin",
    "dad",
    "daughter",
    "day",
    "days",
    "deadline",
    "depression",
    "dog",
    "doggy",
    "dogs",
    "doubt",
    "ear",
    "ears",
    "energy",
    "evening",
    "exam",
    "exams",
    "eye",
    "eyes",
    "family",
    "father",
    "fear",
    "feet",
    "focus",
    "foot",
    "friend",
    "friends",
    "girl",
    "girlfriend",
    "girls",
    "gran",
    "grandad",
    "grandma",
    "grandpa",
    "grief",
    "guinea",
    "hair",
    "hamster",
    "hand",
    "hands",
    "he",
    "head",
    "health",
    "her",
    "hers",
    "him",
    "hip",
    "hips",
    "his",
    "husband",
    "infant",
    "interview",
    "job",
    "kid",
    "kids",
    "kitten",
    "knee",
    "knees",
    "lad",
    "leg",
    "legs",
    "life",
    "loneliness",
    "mate",
    "mates",
    "memory",
    "mind",
    "mom",
    "month",
    "months",
    "mood",
    "morning",
    "mornings",
    "mother",
    "motivation",
    "mum",
    "nan",
    "nana",
    "neck",
    "neighbor",
    "neighbour",
    "nerves",
    "newborn",
    "night",
    "nights",
    "panic",
    "parrot",
    "partner",
    "pet",
    "pets",
    "presentation",
    "pup",
    "puppy",
    "rabbit",
    "rabbits",
    "sadness",
    "school",
    "she",
    "shift",
    "shifts",
    "shoulder",
    "shoulders",
    "sister",
    "skin",
    "sleep",
    "son",
    "stomach",
    "stress",
    "teen",
    "teenager",
    "teeth",
    "term",
    "test",
    "their",
    "them",
    "they",
    "throat",
    "time",
    "times",
    "toddler",
    "toddlers",
    "tooth",
    "tummy",
    "uncle",
    "uni",
    "voice",
    "week",
    "weekend",
    "weeks",
    "wife",
    "work",
    "worries",
    "worry",
    "year",
    "years",
];
pub(super) const ADDRESS_WORDS: &[&str] = &[
    "chuck",
    "darling",
    "dear",
    "duck",
    "folks",
    "friend",
    "friends",
    "guys",
    "hon",
    "hun",
    "hunny",
    "love",
    "mate",
    "mates",
    "pet",
    "son",
    "sunshine",
    "sweetheart",
    "sweetie",
];
