//! Resolve a near-me question to a place the user asked for in a previous turn.
use crate::bot::{Memory, Reading};
use fancy_regex::Regex;
use std::sync::LazyLock;
static THE_PRONOUN_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:is|are)\s+there\s+(?:one|any)\s+(?:near(?:\s+(?:me|here|us))?|nearby|close\s+by|close\s+to\s+(?:me|us|here)|round\s+here|around\s+here|in\s+(?:my|the|our)\s+(?:area|town|village|end|city))\b|(?<![a-z])(?:is|are)\s+there\s+(?:one|any)\s+(?:close\s+by|nearby)\b|(?<![a-z])where(?:'s|\s+is)\s+the\s+(?:nearest|closest)\s+one\b|(?<![a-z])(?:where|how)\s+(?:can|do|would|should)\s+i\s+(?:find|get|locate)\s+(?:one|any)\b|(?<![a-z])(?:can|could|will)\s+you\s+(?:find|point\s+me\s+to|tell\s+me\s+where\s+to\s+find)\s+(?:one|any)\b"#).unwrap()
});
static THE_FRAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:i\s*(?:'m|m)?\s*(?:looking\s+for|after|in\s+need\s+of|trying\s+to\s+find)|i\s+(?:really\s+)?(?:need|want|would\s+like|am\s+after)|i\s+could\s+(?:really\s+)?(?:do\s+with|use|find)|(?:im|i\s+am)\s+(?:desperate|dying)\s+for|looking\s+for|(?:where|how)\s+(?:can|do|would)\s+i\s+(?:get|find|buy|locate)|(?:is|are)\s+there\s+(?:a|an|any)|(?:can|could)\s+you\s+(?:find|recommend|point\s+me\s+to)|ive\s+been\s+(?:trying\s+to\s+find|looking\s+for|desperate\s+for)|(?:do\s+you\s+know\s+(?:where|of)\s+(?:a|an|any)?)|(?:any\s+idea|any\s+suggestions?)\s+(?:where|of|for)\s+(?:i\s+(?:can|could|might)\s+(?:find|get|buy))?)"#).unwrap()
});
const NEARBY_ONE_POOL: &[&str] = &[
    "Nothing here knows what is around you, sorry - no map, no local list. A map app search for {thing} near you is the quickest way to one, and the reviews on the listing beat anything I could recall about a place I have never been.",
    "I can't see your area, sorry. Put \"{thing}\" and your town into a map app and it shows what is close, with the opening hours and the distance; the place's own page or a phone call settles anything else about it.",
];
const THE_END: &[&str] = &[
    "about",
    "after",
    "afternoon",
    "all",
    "among",
    "and",
    "any",
    "around",
    "as",
    "asap",
    "at",
    "because",
    "before",
    "between",
    "but",
    "by",
    "day",
    "during",
    "evening",
    "for",
    "from",
    "her",
    "here",
    "him",
    "his",
    "hour",
    "if",
    "in",
    "into",
    "it",
    "its",
    "later",
    "me",
    "minute",
    "month",
    "morning",
    "my",
    "near",
    "nearby",
    "nearest",
    "night",
    "now",
    "on",
    "onto",
    "or",
    "our",
    "over",
    "per",
    "please",
    "round",
    "since",
    "so",
    "some",
    "soon",
    "than",
    "thank",
    "thanks",
    "that",
    "their",
    "them",
    "then",
    "there",
    "these",
    "this",
    "those",
    "through",
    "time",
    "to",
    "today",
    "tomorrow",
    "tonight",
    "under",
    "until",
    "us",
    "via",
    "week",
    "weekend",
    "when",
    "which",
    "while",
    "who",
    "whom",
    "with",
    "without",
    "year",
    "your",
];
const THE_ARTICLE: &[&str] = &[
    "a", "an", "another", "any", "couple", "few", "one", "other", "some", "the", "three", "two",
];
const THE_ADJECTIVE: &[&str] = &[
    "actual",
    "best",
    "better",
    "big",
    "cheap",
    "closest",
    "decent",
    "favorite",
    "favourite",
    "good",
    "great",
    "half-decent",
    "large",
    "little",
    "local",
    "nearby",
    "nearest",
    "new",
    "nice",
    "old",
    "open",
    "proper",
    "quick",
    "real",
    "reliable",
    "small",
    "special",
];
const A_VERB: &[&str] = &[
    "be",
    "boil",
    "book",
    "buy",
    "call",
    "change",
    "check",
    "cover",
    "cry",
    "drink",
    "eat",
    "find",
    "fix",
    "get",
    "give",
    "go",
    "keep",
    "learn",
    "leave",
    "make",
    "move",
    "open",
    "order",
    "register",
    "reinstall",
    "remember",
    "save",
    "service",
    "shower",
    "sleep",
    "sort",
    "spend",
    "start",
    "stop",
    "switch",
    "try",
    "use",
    "wake",
    "water",
];
const NOT_A_THING: &[&str] = &[
    "account",
    "alive",
    "allowed",
    "angry",
    "anxious",
    "any",
    "anyone",
    "anyplace",
    "anything",
    "anywhere",
    "asleep",
    "awake",
    "bad",
    "batteries",
    "battery",
    "best",
    "better",
    "bored",
    "broken",
    "busy",
    "card",
    "certain",
    "cheap",
    "clothes",
    "cold",
    "course",
    "dangerous",
    "dead",
    "degree",
    "difficult",
    "done",
    "early",
    "easy",
    "everything",
    "everywhere",
    "exhausted",
    "expensive",
    "fair",
    "false",
    "fine",
    "finished",
    "fixed",
    "free",
    "good",
    "great",
    "happy",
    "hard",
    "hot",
    "hungry",
    "ill",
    "illegal",
    "induction",
    "it",
    "job",
    "late",
    "legal",
    "likely",
    "lonely",
    "lost",
    "midwifery",
    "nice",
    "normal",
    "nothing",
    "nowhere",
    "ok",
    "okay",
    "one",
    "possible",
    "ready",
    "real",
    "right",
    "sad",
    "safe",
    "series",
    "session",
    "sick",
    "someone",
    "someplace",
    "something",
    "somewhere",
    "sorted",
    "strange",
    "stressed",
    "stuck",
    "sure",
    "thing",
    "thirsty",
    "tired",
    "true",
    "unfair",
    "unwell",
    "upset",
    "warm",
    "weird",
    "well",
    "working",
    "worried",
    "worse",
    "worst",
    "worth",
    "wrong",
];
const NOT_A_PLACE: &[&str] = &[
    "answer",
    "appointment",
    "bus",
    "chance",
    "coach",
    "coat",
    "color",
    "colour",
    "cost",
    "cure",
    "delivery",
    "difference",
    "ferry",
    "flight",
    "history",
    "idea",
    "jeans",
    "jewellery",
    "jewelry",
    "law",
    "lift",
    "list",
    "need",
    "news",
    "opinion",
    "pants",
    "parcel",
    "pay",
    "point",
    "price",
    "problem",
    "proof",
    "question",
    "quote",
    "reason",
    "refund",
    "risk",
    "rule",
    "salary",
    "shirt",
    "shoes",
    "taxi",
    "train",
    "tram",
    "treatment",
    "trousers",
    "tube",
    "vaccine",
    "view",
    "wage",
    "way",
];
const PLACE_HEADS: &[&str] = &[
    "agency",
    "bank",
    "banks",
    "barber",
    "cafe",
    "center",
    "centre",
    "chemist",
    "clinic",
    "company",
    "depot",
    "firm",
    "garage",
    "garages",
    "gym",
    "hairdresser",
    "hall",
    "hospital",
    "hotel",
    "launderette",
    "launderettes",
    "laundrette",
    "library",
    "market",
    "office",
    "offices",
    "optician",
    "park",
    "pharmacy",
    "pool",
    "pub",
    "salon",
    "station",
    "stations",
    "stop",
    "stops",
    "store",
    "stores",
    "studio",
    "supermarket",
    "surgery",
    "takeaway",
    "tip",
    "yard",
];
const A_CHORE: &[&str] = &["market", "markets", "shop", "shops", "store", "stores"];
const A_TRADE: &[&str] = &[
    "accountant",
    "acupuncturist",
    "architect",
    "barber",
    "barrister",
    "bricklayer",
    "builder",
    "carpenter",
    "caterer",
    "chef",
    "chiropodist",
    "cleaner",
    "counsellor",
    "cutter",
    "decorator",
    "dentist",
    "director",
    "doctor",
    "electrician",
    "gardener",
    "glazier",
    "groomer",
    "hairdresser",
    "instructor",
    "joiner",
    "locksmith",
    "mechanic",
    "midwife",
    "notary",
    "nurse",
    "optician",
    "osteopath",
    "painter",
    "photographer",
    "physio",
    "physiotherapist",
    "plasterer",
    "plumber",
    "podiatrist",
    "psychotherapist",
    "roofer",
    "solicitor",
    "surveyor",
    "tailor",
    "therapist",
    "tiler",
    "tuner",
    "tutor",
    "vet",
];
const PLACE_TAILS: &[&str] = &["ary", "ery", "ette", "ettes", "eum", "eums"];
const PLACE_STEMS: &[&str] = &[
    "agency",
    "bank",
    "bistro",
    "cafe",
    "caff",
    "center",
    "centre",
    "chemist",
    "chippie",
    "chippy",
    "clinic",
    "deli",
    "diner",
    "firm",
    "garage",
    "grill",
    "gym",
    "hall",
    "hospital",
    "hotel",
    "kiosk",
    "launderette",
    "laundrette",
    "market",
    "park",
    "pool",
    "pub",
    "salon",
    "shop",
    "stall",
    "station",
    "store",
    "studio",
    "surgery",
];
const CASH_POINT: &[&str] = &[
    "atm",
    "atms",
    "cash machine",
    "cash machines",
    "cash point",
    "cash points",
    "cashpoint",
    "cashpoints",
];
const NEAR_ME_PAIR: &[&str] = &[
    "bottle bank",
    "bus stop",
    "litter bin",
    "lost property office",
    "petrol station",
    "phone box",
    "pillar box",
    "post box",
    "postbox",
    "public toilet",
    "public toilets",
    "recycling point",
    "taxi rank",
];

fn has(list: &[&str], word: &str) -> bool {
    list.binary_search(&word).is_ok()
}
fn named_place(line: &str) -> Option<String> {
    const PUNCT: &[char] = &['.', ',', '!', '?', ';', ':', '\'', '"', '(', ')'];
    let after = &line[THE_FRAME.find(line).ok()??.end()..];
    let mut words = after
        .split_whitespace()
        .map(|w| w.trim_matches(PUNCT))
        .peekable();
    if words.peek() == Some(&"to") {
        return None;
    }
    let mut kept = Vec::with_capacity(3);
    for word in words {
        if word.is_empty() {
            continue;
        }
        if has(THE_END, word) || has(THE_ARTICLE, word) {
            if kept.is_empty() {
                continue;
            }
            break;
        }
        if kept.is_empty() && has(THE_ADJECTIVE, word) {
            continue;
        }
        if !word.as_bytes()[0].is_ascii_lowercase()
            || !word
                .bytes()
                .all(|c| c.is_ascii_lowercase() || matches!(c, b'\'' | b'&' | b'-'))
        {
            break;
        }
        kept.push(word);
        if kept.len() == 3 {
            break;
        }
    }
    let first = kept.first()?;
    let last = kept.last()?;
    let phrase = kept.join(" ");
    if has(NOT_A_THING, first) || has(NOT_A_THING, &phrase) || has(A_VERB, first) {
        return None;
    }
    if has(CASH_POINT, &phrase) || has(NEAR_ME_PAIR, &phrase) {
        return Some(phrase);
    }
    if kept
        .iter()
        .any(|w| has(NOT_A_PLACE, w) || has(NOT_A_THING, w))
        || kept.len() == 1 && has(A_CHORE, last)
    {
        return None;
    }
    (has(PLACE_HEADS, last)
        || has(A_TRADE, last)
        || PLACE_STEMS
            .iter()
            .chain(PLACE_TAILS)
            .any(|suffix| last.ends_with(suffix)))
    .then_some(phrase)
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.folded();
    if !THE_PRONOUN_ASK.is_match(line).unwrap_or(false) {
        return None;
    }
    let previous = memory
        .turns
        .iter()
        .rev()
        .map(|t| t.user.trim().to_lowercase())
        .filter(|s| !s.is_empty() && s != line && !THE_PRONOUN_ASK.is_match(s).unwrap_or(false))
        .take(4)
        .find(|s| THE_FRAME.is_match(s).unwrap_or(false))?;
    let thing = named_place(&previous)?;
    let index = line
        .chars()
        .fold(0, |index, c| (index + c as usize) % NEARBY_ONE_POOL.len());
    Some(NEARBY_ONE_POOL[index].replace("{thing}", &thing))
}
