//! Personal residence, relations, and preferences from the conversation.
use super::{
    again, budget,
    carried_on::{Attempt, HAND_BACK_LIMIT},
    earlier, food, places, summary, surname, when,
};
use crate::bot::{Memory, Reading, compose};
use regex::{Captures, Regex};
use std::{collections::VecDeque, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($source).unwrap());
    };
}

pattern!(
    LEAD,
    r"(?i)^(?:(?:actually|sorry|no wait|nope|no|so|and|but|well|oh|also|btw|right|okay|ok)[ ,.!;-]+)+"
);
pattern!(
    CORRECTION_LEAD,
    r"(?i)^(?:actually|sorry|no wait|nope|no|i meant)[ ,.!;-]+"
);
pattern!(CONJUNCTION, r"(?i)\s+(?:and|but)\s+");
pattern!(
    CLAUSE_SUBJECT,
    r"(?i)^(?:i\b|i['’](?:ve|m)\b|we\b|we['’](?:ve|re)\b|my\b|our\b|he\b|she\b|they\b|what\b|which\b|where\b|how\b|who\b|can\b|could\b|should\b|the (?:youngest|eldest|oldest|baby|toddler|newborn|little one)\b)"
);
pattern!(
    OWN_HOME,
    r"(?i)^(?:i|we)(?:'ve|'m|'re| (?:am|are|have))?\s+(?:(?:just|recently|finally|already|also|now)\s+)?(?:live|living|moved|settled|relocated|based)\s+(?:(?:up|back|over|out|down|just|right)\s+)?(?P<preposition>in|to|into|near|outside|round|from)\s+(?P<value>.+)$"
);
pattern!(
    MOVE_FROM,
    r"(?i)^(?P<origin>.+?)\s+(?:to|into)\s+(?P<home>.+)$"
);
pattern!(MOVE_TO, r"(?i)^(?P<home>.+?)\s+from\s+(?P<origin>.+)$");
pattern!(
    ORIGIN,
    r"(?i)^(?:i|we)(?:'m|'re| am| are| was| were)?\s+(?:(?:originally |come )?from|born in|grew up in)\s+(?P<value>.+)$"
);
pattern!(
    HOME_ASK,
    r"^where (?:(?:do (?:i|we) live)|(?:am i (?:living|based))|(?:are we (?:living|based)))(?: (?:now|again|at|then))?$"
);
pattern!(
    ORIGIN_ASK,
    r"^(?:where (?:(?:am i|are we|do (?:i|we) come) from)|what(?:'s| is| was) my hometown)(?: (?:originally|again|then))?$"
);
pattern!(GREW_UP_ASK, r"^where did (?:i|we) grow up(?: again)?$");
pattern!(
    HOME_CONFIRM,
    r"^(?:(?:do i|do we) live|am i (?:living|based)) (?:in|near|outside) (?P<value>.+)$"
);
pattern!(
    PLACE_WORDS,
    r"^[\p{L}][\p{L}\p{M}'-]*(?:\s+[\p{L}][\p{L}\p{M}'-]*){0,4}$"
);
pattern!(
    PLACE_TAIL,
    r"(?i)\s+(?:originally|now|again|instead|these days|at the moment|for .+|because .+)$"
);
pattern!(
    NAME_WORDS,
    r"^[\p{L}][\p{L}\p{M}'-]*(?:\s+[\p{L}][\p{L}\p{M}'-]*){0,2}$"
);
pattern!(
    KIN,
    r"(?i)^(?:my|our) (?P<role>(?:little|wee) one|[\p{L}-]+?(?:[ -]in[ -]law)?)(?P<possessive>'s)?(?: (?P<rest>.+))?$"
);
pattern!(
    PRONOUN,
    r"(?i)^(?P<pronoun>he|she|they)(?P<contraction>'?s|'?re|'?ve)? (?P<rest>.+)$"
);
pattern!(
    POSSESSIVE_NAME,
    r"(?i)^(?:his|her|their) name (?:is|was) (?P<value>.+)$"
);
pattern!(
    PERSON_NAME,
    r"(?i)^(?:(?:is|was|are) )?(?:called|named) (?P<value>.+)$"
);
pattern!(
    PERSON_HOME,
    r"(?i)^(?:(?:has|have) )?(?:(?:just|recently|now) )?(?:lives?|living|moved|settled|relocated|based) (?P<preposition>in|near|outside|to|into) (?P<value>.+)$"
);
pattern!(
    PERSON_ORIGIN,
    r"(?i)^(?:(?:is|was|are) )?(?:from|comes? from|grew up in|born in) (?P<value>.+)$"
);
pattern!(
    PERSON_AGE,
    r"(?i)^(?:(?:is(?: turning)?|are|was|turns?|turned|will (?:turn|be)) )?(?P<value>[0-9]{1,3})(?: (?P<unit>years?|months?)(?: old\b.*)?| going on [0-9]{1,3})?(?: (?:and|now|today|tomorrow|next (?:week|month|year|spring|summer|autumn|winter|[a-z]+day)|(?:in|on|by) [a-z]{3,9})\b.*)?$"
);
pattern!(
    AGE_AHEAD,
    r"(?i)^(?:turns?|will be|is|are) [0-9]{1,3} (?:next (?:week|month|year|spring|summer|autumn|winter)|(?:in|on) [a-z]{3,9})\b"
);
pattern!(
    THE_PLACE_AGE,
    r"(?i)^the (?P<who>youngest|eldest|oldest|baby|toddler|newborn|little one)(?:'s| is) (?P<rest>.+)$"
);
pattern!(THEIR_AGE, r"(?i)^ages? (?:is|was) (?P<rest>.+)$");
pattern!(
    WHOS_AGE,
    r"(?i)\b(?:my|our)\s+(?P<who>[a-z-]+)\s*,?\s+who(?:'s|s)?\s+(?:is\s+)?(?P<rest>[^,.;!?]+)"
);
pattern!(
    NAMED_FACT,
    r"(?i)^(?P<name>[\p{L}][\p{L}\p{M}'-]*(?: [\p{L}][\p{L}\p{M}'-]*)?) (?P<rest>(?:(?:has|have|just|recently|now) )?(?:lives?|living|moved|settled|relocated|based|comes?|grew|from|(?:is|was|are)\s+|turned|likes?|loves?|enjoys?|hates?|dislikes?|doesn't|does not|don't|do not)\b.*)$"
);
pattern!(
    TASTE,
    r"(?i)^(?P<verb>(?:(?:really|absolutely|still) )?(?:likes?|loves?|enjoys?|hates?|dislikes?|(?:do(?:es)?n't|do(?:es)? not) (?:like|love|enjoy)|(?:is|am|are) (?:(?:not|really) )?keen on)) (?P<value>.+)$"
);
pattern!(
    PERSON_HOME_ASK,
    r"^where (?:do|does|did) (?P<subject>.+?) live(?: (?:now|again|then))?$"
);
pattern!(
    PERSON_ORIGIN_ASK,
    r"^where (?:is|are|was|were) (?P<subject>.+?) from(?: (?:originally|again|now))?$"
);
pattern!(
    PERSON_AGE_ASK,
    r"^(?:how old|what age) (?:is|are|was|were) (?P<subject>.+?)(?: (?:now|again))?$"
);
pattern!(
    PERSON_NAME_ASK,
    r"^what(?:'s|s| is| was) (?:(?P<subject>.+?) (?:called|named)|(?P<owner>.+?) name)(?: again)?$"
);
pattern!(
    PERSON_TASTES_ASK,
    r"^what (?:do|does|did) (?P<subject>.+?) (?P<verb>like|love|enjoy|dislike|hate|not like|not enjoy)(?: (?:now|again))?$"
);
pattern!(
    TASTE_CONFIRM,
    r"^(?:do|does|did) (?P<subject>.+?) (?:like|love|enjoy|likes|liked) (?P<value>.+)$"
);
pattern!(
    FAVOURITE,
    r"(?i)^(?:my|our) (?:favou?rite|fav|best) (?P<category>[\p{L}'-]+(?: [\p{L}'-]+){0,2}?)(?: (?:is|are|was|will be|has to be|would be)|'s) (?P<value>[^?.!;]{1,60})$"
);
pattern!(
    REVERSE_FAVOURITE,
    r"(?i)^(?P<value>[^?.!;]{1,60}?) (?:is|are|has always been) my (?:favou?rite|fav|best) (?P<category>[\p{L}'-]+(?: [\p{L}'-]+){0,2})$"
);
pattern!(
    FAVOURITE_ASK,
    r"^(?:(?:what|who)(?:'s|s| is| was| are| were)?|which(?: is| one)?|do you (?:know|remember)|did i (?:say|tell you)|what did i (?:say|tell you)|can you (?:remember|tell me))\b[^?.!]{0,40}?\bmy (?:favou?rite|fav|best) (?P<category>[\p{L}' -]{1,48})$"
);
pattern!(
    REVERSE_FAVOURITE_ASK,
    r"^which (?P<category>[\p{L}' -]{1,48}) (?:is|was) my (?:favou?rite|fav|best)$"
);
pattern!(
    ALL_FAVOURITES,
    r"^(?:what|which)(?: are| were| do you know about| have i told you about)? my (?:favou?rites|favs)$"
);
pattern!(
    FAVOURITE_CORRECTION,
    r"(?i)^(?:no wait|nope|no|actually|sorry|wait|i mean|rather|scratch that|make that|not that|that's wrong)[ ,.!-]+(?:its|it's|it is|i mean|i meant|make it|just) (?P<value>[^?.!;]{1,60})$"
);
pattern!(
    PREFERS_NOW,
    r"(?i)^(?:i|we) (?:prefer|like) (?P<value>[^?.!,;]{1,40}?) (?:better now|more now|better these days|more these days|nowadays|these days|now|better|more)$"
);
pattern!(
    VALUE_TAIL,
    r"(?i)[ ,]+(?:obviously|tbh|tho|though|really|lol|haha|i think|probably|at the moment|right now|for sure|of course)$"
);
pattern!(
    FAVOURITE_RESPONSE,
    r"^[\p{L}\p{N}][\p{L}\p{M}\p{N}' ,-]{0,40}$"
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Gender {
    Female,
    Male,
    Unspecified,
}

#[derive(Clone, Copy, Debug)]
struct Relation {
    key: &'static str,
    display: &'static str,
    gender: Gender,
    source: &'static str,
}

fn person_modifier(word: &str) -> bool {
    matches!(
        word.to_lowercase().as_str(),
        "little"
            | "big"
            | "small"
            | "tiny"
            | "eldest"
            | "oldest"
            | "youngest"
            | "baby"
            | "middle"
            | "older"
            | "younger"
            | "teenage"
            | "grown"
            | "first"
            | "second"
            | "third"
            | "fourth"
            | "other"
            | "poor"
            | "darling"
            | "littlest"
            | "new"
    )
}

fn relation(word: &str) -> Option<Relation> {
    use Gender::{Female, Male, Unspecified};
    let low = word.to_lowercase();
    let word = match low.split_once(' ') {
        Some((modifier, rest)) if person_modifier(modifier) && low != "little one" => rest,
        _ => low.as_str(),
    };
    let (key, display, gender) = match word {
        "mum" | "mam" => ("mother", "mum", Female),
        "mom" => ("mother", "mom", Female),
        "mother" => ("mother", "mother", Female),
        "dad" => ("father", "dad", Male),
        "father" => ("father", "father", Male),
        "sister" | "sis" => ("sister", "sister", Female),
        "brother" | "bro" => ("brother", "brother", Male),
        "daughter" | "girl" => ("daughter", "daughter", Female),
        "son" | "boy" | "lad" => ("son", "son", Male),
        "wife" => ("wife", "wife", Female),
        "husband" => ("husband", "husband", Male),
        "boyfriend" => ("boyfriend", "boyfriend", Male),
        "girlfriend" => ("girlfriend", "girlfriend", Female),
        "partner" => ("partner", "partner", Unspecified),
        "aunt" | "auntie" => ("aunt", "aunt", Female),
        "uncle" => ("uncle", "uncle", Male),
        "niece" => ("niece", "niece", Female),
        "nephew" => ("nephew", "nephew", Male),
        "cousin" => ("cousin", "cousin", Unspecified),
        "nan" | "nana" | "gran" | "granny" => ("grandmother", "nan", Female),
        "grandmother" | "grandma" => ("grandmother", "grandmother", Female),
        "grandfather" | "grandpa" | "grandad" | "granddad" => ("grandfather", "grandfather", Male),
        "grandson" => ("grandson", "grandson", Male),
        "granddaughter" => ("granddaughter", "granddaughter", Female),
        "stepmum" | "stepmom" | "stepmother" => ("stepmother", "stepmum", Female),
        "stepdad" | "stepfather" => ("stepfather", "stepdad", Male),
        "stepson" => ("stepson", "stepson", Male),
        "stepdaughter" => ("stepdaughter", "stepdaughter", Female),
        "mother-in-law" | "mother in law" | "mum-in-law" | "mum in law" => {
            ("mother-in-law", "mother-in-law", Female)
        }
        "father-in-law" | "father in law" | "dad-in-law" | "dad in law" => {
            ("father-in-law", "father-in-law", Male)
        }
        "sister-in-law" | "sister in law" => ("sister-in-law", "sister-in-law", Female),
        "brother-in-law" | "brother in law" => ("brother-in-law", "brother-in-law", Male),
        "friend" | "mate" => ("friend", "friend", Unspecified),
        "child" | "kid" => ("child", "child", Unspecified),
        "baby" => ("baby", "baby", Unspecified),
        "toddler" => ("toddler", "toddler", Unspecified),
        "newborn" => ("newborn", "newborn", Unspecified),
        "little one" | "wee one" => ("little one", "little one", Unspecified),
        "youngest" => ("youngest", "youngest", Unspecified),
        "eldest" => ("eldest", "eldest", Unspecified),
        "oldest" => ("oldest", "oldest", Unspecified),
        "twins" => ("twins", "twins", Unspecified),
        "boss" => ("boss", "boss", Unspecified),
        "colleague" | "coworker" => ("colleague", "colleague", Unspecified),
        "neighbour" | "neighbor" => ("neighbour", "neighbour", Unspecified),
        _ => return None,
    };
    Some(Relation {
        key,
        display,
        gender,
        source: match word {
            "boy" => "boy",
            "girl" => "girl",
            "kid" => "kid",
            "wee one" => "wee one",
            "lad" => "lad",
            "mother in law" | "mother-in-law" | "mum in law" | "mum-in-law" => "mother in law",
            "father in law" | "father-in-law" | "dad in law" | "dad-in-law" => "father in law",
            "sister in law" | "sister-in-law" => "sister in law",
            "brother in law" | "brother-in-law" => "brother in law",
            _ => display,
        },
    })
}

#[derive(Clone, Debug)]
struct Residence {
    place: String,
    preposition: &'static str,
}

impl Residence {
    fn new(place: String, preposition: &str) -> Self {
        let preposition = match preposition.to_lowercase().as_str() {
            "near" | "round" => "near",
            "outside" => "outside",
            _ => "in",
        };
        Self { place, preposition }
    }
}

#[derive(Debug)]
pub(super) enum Age {
    Years(u8),
    Months(u8),
    FutureYears(u8),
}

impl std::fmt::Display for Age {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Years(years) | Self::FutureYears(years) => write!(output, "{years}"),
            Self::Months(months) => write!(output, "{months} months old"),
        }
    }
}

#[derive(Debug)]
struct Taste {
    subject: String,
    verb: &'static str,
    likes: bool,
}

#[derive(Debug)]
enum Fact {
    Name(String),
    Age(Age),
    Home,
    Origin(String),
    Taste(Taste),
    Food(food::Change),
}

#[derive(Debug)]
struct Person {
    role: Relation,
    filed: bool,
    name: Option<String>,
    origin: Option<String>,
    age: Option<Age>,
    age_years: Option<u8>,
    age_months: Option<u8>,
    tastes: Vec<Taste>,
    food: food::Food,
}

impl Person {
    fn new(role: Relation) -> Self {
        Self {
            role,
            filed: false,
            name: None,
            origin: None,
            age: None,
            age_years: None,
            age_months: None,
            tastes: Vec::new(),
            food: food::Food::default(),
        }
    }
}

#[derive(Debug)]
enum FavouriteUpdate {
    Given(String),
    Answered(String),
    Preferred { category: String, previous: String },
}

#[derive(Default)]
pub(crate) struct Personal {
    schedule: when::Schedule,
    surname: Option<String>,
    budget: Option<String>,
    carried_parts: VecDeque<String>,
    attempt: Option<Attempt>,
    food: food::Food,
    food_update: Option<String>,
    own_limits_last_turn: Option<food::Limits>,
    other_limits_last_turn: Option<food::Limits>,
    shared_food: Option<(Option<&'static str>, food::Limits)>,
    home: Option<Residence>,
    origin: Option<String>,
    people: Vec<Person>,
    focus: Option<usize>,
    unnamed_she_age: Option<Age>,
    unnamed_he_age: Option<Age>,
    tastes: Vec<Taste>,
    favourites: Vec<(String, String)>,
    pending_favourite: Option<String>,
    recent_favourite: Option<(String, usize)>,
    favourite_update: Option<FavouriteUpdate>,
    name_correction: Option<(String, String)>,
    home_mentioned: Option<String>,
}

fn tidy(text: &str) -> &str {
    text.trim_matches(|c: char| c.is_whitespace() || matches!(c, '.' | '!' | '?' | ',' | ';'))
}

fn upper_first(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn first(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or_default()
}

fn capture<'a>(groups: &Captures<'a>, field: &str) -> Option<&'a str> {
    groups.name(field).map(|part| part.as_str())
}

fn person_and_fact<'a>(groups: &Captures<'a>) -> Option<(Relation, &'a str)> {
    let who = capture(groups, "role")?;
    let rest = capture(groups, "rest")?;
    if person_modifier(who)
        && let Some((noun, fact)) = rest.split_once(' ')
        && let Some(role) = relation(noun)
    {
        if role.key == "child" {
            let position = relation(who)?;
            return matches!(position.key, "youngest" | "eldest" | "oldest")
                .then_some((position, fact));
        }
        return Some((role, fact));
    }
    Some((relation(who)?, rest))
}

fn name(text: &str) -> Option<String> {
    let text = tidy(text);
    let low = text.to_lowercase();
    if !NAME_WORDS.is_match(text)
        || text.chars().count() > 42
        || matches!(
            first(&low),
            "a" | "an"
                | "the"
                | "my"
                | "your"
                | "too"
                | "not"
                | "fine"
                | "well"
                | "happy"
                | "sad"
                | "tired"
                | "away"
                | "here"
                | "there"
                | "coming"
                | "going"
                | "moving"
                | "working"
                | "busy"
                | "gone"
                | "home"
                | "in"
                | "from"
                | "called"
                | "named"
                | "is"
                | "was"
                | "are"
                | "has"
                | "have"
        )
    {
        return None;
    }
    Some(
        text.split_whitespace()
            .map(upper_first)
            .collect::<Vec<_>>()
            .join(" "),
    )
}

fn same_value(left: &str, right: &str) -> bool {
    left == right || left.to_lowercase() == right.to_lowercase()
}

pub(super) fn place(text: &str) -> Option<String> {
    let text = PLACE_TAIL.replace(tidy(text), "");
    let text = text.as_ref();
    let low = text.to_lowercase();
    if !PLACE_WORDS.is_match(text)
        || text.chars().count() > 64
        || matches!(
            first(&low),
            "a" | "an"
                | "the"
                | "with"
                | "my"
                | "our"
                | "hope"
                | "debt"
                | "fear"
                | "poverty"
                | "pain"
                | "tears"
                | "there"
                | "here"
                | "home"
                | "bed"
                | "hospital"
                | "school"
                | "work"
                | "nowhere"
                | "somewhere"
                | "monday"
                | "tuesday"
                | "wednesday"
                | "thursday"
                | "friday"
                | "saturday"
                | "sunday"
                | "january"
                | "february"
                | "march"
                | "april"
                | "may"
                | "june"
                | "july"
                | "august"
                | "september"
                | "october"
                | "november"
                | "december"
        )
        || low.split_whitespace().any(|word| {
            matches!(
                word,
                "not"
                    | "but"
                    | "moved"
                    | "live"
                    | "love"
                    | "like"
                    | "from"
                    | "to"
                    | "today"
                    | "tomorrow"
                    | "yesterday"
            )
        })
    {
        return None;
    }
    Some(
        text.split_whitespace()
            .map(|word| {
                if matches!(word, "of" | "and" | "de" | "du") {
                    word.to_owned()
                } else {
                    upper_first(word)
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
    )
}

fn clauses(text: &str) -> Vec<&str> {
    let mut clauses = Vec::new();
    for phrase in text.split_inclusive([',', ';', '.', '?', '!']) {
        let mut start = 0;
        for split in CONJUNCTION.find_iter(phrase) {
            if CLAUSE_SUBJECT.is_match(&phrase[split.end()..]) {
                clauses.push(&phrase[start..split.start()]);
                start = split.end();
            }
        }
        clauses.push(&phrase[start..]);
    }
    clauses
}

fn swap_possessives(text: &str) -> String {
    text.split_whitespace()
        .map(|word| match word.to_lowercase().as_str() {
            "my" | "our" => "your",
            "me" | "us" | "i" => "you",
            "myself" => "yourself",
            _ => word,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn taste(text: &str) -> Option<Taste> {
    let parsed = TASTE.captures(text)?;
    let verb = capture(&parsed, "verb")?.to_lowercase();
    let value = tidy(capture(&parsed, "value")?);
    if value.is_empty() || value.split_whitespace().count() > 8 || value.contains('?') {
        return None;
    }
    let value = ["the ", "an ", "a "]
        .into_iter()
        .find_map(|article| value.strip_prefix(article))
        .unwrap_or(value);
    let low = value.to_lowercase();
    if matches!(
        first(&low),
        "it" | "them"
            | "that"
            | "this"
            | "when"
            | "how"
            | "anything"
            | "everything"
            | "nothing"
            | "if"
            | "what"
    ) {
        return None;
    }
    let likes = !(verb.contains("n't")
        || verb.contains("not")
        || verb.contains("hate")
        || verb.contains("dislike"));
    let reply_verb = if verb.contains("hate") {
        "hates"
    } else if !likes {
        "doesn't like"
    } else if verb.contains("love") {
        "loves"
    } else if verb.contains("enjoy") {
        "enjoys"
    } else {
        "likes"
    };
    Some(Taste {
        subject: swap_possessives(value),
        verb: reply_verb,
        likes,
    })
}

fn save_taste(tastes: &mut Vec<Taste>, taste: Taste) {
    if let Some(previous) = tastes
        .iter_mut()
        .find(|previous| same_value(&previous.subject, &taste.subject))
    {
        *previous = taste;
    } else {
        tastes.push(taste);
    }
}

fn person_fact(text: &str, bare_name: bool) -> Option<Fact> {
    let digits = super::age_words::figures(text);
    let digits = match digits {
        std::borrow::Cow::Borrowed(text) => super::kidcount::figures_as_digits(text),
        digits => digits,
    };
    if let Some(groups) = PERSON_AGE.captures(&digits) {
        let number = capture(&groups, "value")?
            .parse::<u8>()
            .ok()
            .filter(|age| *age <= 125)?;
        let unit = capture(&groups, "unit").unwrap_or_default().to_lowercase();
        return Some(Fact::Age(if unit.starts_with("month") {
            Age::Months(number)
        } else if AGE_AHEAD.is_match(&digits) {
            Age::FutureYears(number)
        } else {
            Age::Years(number)
        }));
    }
    if let Some(groups) = PERSON_HOME.captures(text) {
        place(capture(&groups, "value")?)?;
        return Some(Fact::Home);
    }
    if let Some(groups) = PERSON_ORIGIN.captures(text) {
        return place(capture(&groups, "value")?).map(Fact::Origin);
    }
    if let Some(groups) = PERSON_NAME.captures(text) {
        return name(capture(&groups, "value")?).map(Fact::Name);
    }
    if let Some(change) = food::Change::from(text, false) {
        return Some(Fact::Food(change));
    }
    if let Some(taste) = taste(text) {
        return Some(Fact::Taste(taste));
    }
    let value = text
        .strip_prefix("is ")
        .or_else(|| text.strip_prefix("Is "))
        .unwrap_or(text);
    if bare_name && first(value).chars().next().is_some_and(char::is_uppercase) {
        return name(value).map(Fact::Name);
    }
    None
}

fn category(text: &str) -> Option<String> {
    let low = text.to_lowercase();
    let words: Vec<_> = low
        .split_whitespace()
        .filter(|word| {
            !matches!(
                *word,
                "again"
                    | "is"
                    | "was"
                    | "are"
                    | "were"
                    | "then"
                    | "though"
                    | "tho"
                    | "now"
                    | "please"
                    | "pls"
                    | "exactly"
                    | "already"
                    | "anyway"
            )
        })
        .collect();
    if words.is_empty()
        || words.len() > 3
        || matches!(
            words[0],
            "thing"
                | "things"
                | "one"
                | "which"
                | "what"
                | "who"
                | "friend"
                | "friends"
                | "mate"
                | "man"
                | "buddy"
        )
    {
        return None;
    }
    let category = words.join(" ");
    Some(if category == "color" {
        "colour".into()
    } else {
        category
    })
}

fn favourite_value(text: &str) -> Option<String> {
    let value = VALUE_TAIL.replace(tidy(text), "");
    let low = value.to_lowercase();
    let plain = low.replace('\'', "");
    if value.is_empty()
        || value.split_whitespace().count() > 6
        || !value.chars().any(char::is_alphanumeric)
        || matches!(
            first(&plain),
            "to" | "when" | "if" | "that" | "because" | "so" | "how" | "why" | "what" | "whether"
        )
        || matches!(
            plain.as_str(),
            "nothing"
                | "none"
                | "whatever"
                | "anything"
                | "something"
                | "everything"
                | "same"
                | "closed"
                | "shut"
                | "over"
                | "broken"
                | "rubbish"
                | "injured"
                | "cancelled"
                | "finished"
                | "sold"
                | "delayed"
                | "busy"
                | "open"
                | "missing"
                | "ill"
                | "gone"
                | "dunno"
                | "unsure"
                | "not sure"
                | "no idea"
                | "dont know"
                | "hard to say"
                | "cant decide"
        )
        || [
            " now",
            " today",
            " tomorrow",
            " tonight",
            " again",
            " these days",
        ]
        .iter()
        .any(|tail| plain.ends_with(tail))
    {
        return None;
    }
    Some(tidy(&value).to_owned())
}

fn asks_favourite(line: &str) -> Option<String> {
    let found = FAVOURITE_ASK
        .captures(line)
        .or_else(|| REVERSE_FAVOURITE_ASK.captures(line))?;
    category(capture(&found, "category")?)
}

fn ask(line: &str) -> bool {
    line.ends_with('?')
        || matches!(
            first(&line.to_lowercase()),
            "who"
                | "what"
                | "when"
                | "where"
                | "why"
                | "how"
                | "which"
                | "can"
                | "could"
                | "should"
                | "do"
                | "does"
                | "did"
                | "is"
                | "are"
                | "will"
                | "would"
        )
}

pub(super) fn relative_food(line: &str) -> Option<(&'static str, food::Change)> {
    if ask(line) {
        return None;
    }
    let line = line.replace(['’', '‘', 'ʼ'], "'");
    let line = LEAD.replace(tidy(&line), "");
    let found = KIN.captures(&line)?;
    if found.name("possessive").is_some() {
        return None;
    }
    let role = relation(capture(&found, "role")?)?;
    let rest = capture(&found, "rest")?;
    Some((role.display, food::Change::from(rest, false)?))
}

impl Personal {
    pub(crate) fn gift_people(&self) -> impl Iterator<Item = (&'static str, Option<u8>)> {
        self.people
            .iter()
            .filter(|person| person.filed)
            .map(|person| {
                (
                    person.role.source,
                    person.age_years.or(match person.age {
                        Some(Age::Years(years) | Age::FutureYears(years)) => Some(years),
                        _ => None,
                    }),
                )
            })
    }

    pub(crate) fn gift_food_allergy(&self, relation: &str) -> bool {
        self.people.iter().any(|person| {
            (person.role.source == relation || person.role.display == relation)
                && (person.food.has_allergy() || person.food.has_coeliac_disease())
        })
    }

    pub(super) fn recorded_ages(&self) -> impl Iterator<Item = (&'static str, Age)> {
        self.people.iter().flat_map(|person| {
            [
                person.age_months.map(Age::Months),
                person.age_years.map(Age::Years),
            ]
            .into_iter()
            .flatten()
            .map(|age| (person.role.source, age))
        })
    }

    pub(super) fn family_people(
        &self,
    ) -> impl DoubleEndedIterator<Item = (&'static str, Option<&str>, Option<u8>)> {
        self.people
            .iter()
            .filter(|person| person.filed || person.food.has_limits())
            .map(|person| {
                let years = match person.age {
                    Some(Age::Years(years)) => Some(years),
                    _ => None,
                };
                (person.role.source, person.name.as_deref(), years)
            })
    }

    pub(crate) fn sole_young_child_age(&self) -> Option<u8> {
        let mut recorded = self
            .people
            .iter()
            .filter(|person| person.filed || person.food.has_limits());
        let person = recorded.next()?;
        if recorded.next().is_some() {
            return None;
        }
        match person.age {
            Some(Age::Years(years @ 1..=8)) => Some(years),
            _ => None,
        }
    }

    pub(super) fn carried_parts(&self) -> impl ExactSizeIterator<Item = &str> {
        self.carried_parts.iter().map(String::as_str)
    }

    pub(super) fn remember_carried(&mut self, piece: String) {
        if self.carried_parts.len() >= HAND_BACK_LIMIT {
            self.carried_parts.pop_front();
        }
        self.carried_parts.push_back(piece);
    }

    pub(super) fn last_attempt(&self) -> Option<&Attempt> {
        self.attempt.as_ref()
    }

    pub(super) fn remember_attempt(&mut self, attempt: Attempt) {
        self.attempt = Some(attempt);
    }

    pub(crate) fn move_trip_month(&mut self, place: &str, month: &str, turns: usize) {
        self.schedule.move_trip_month(place, month, turns);
    }

    pub(crate) fn recall_when(&self, reading: &Reading<'_>, memory: &Memory) -> Option<String> {
        self.schedule.respond(reading, memory)
    }

    pub(crate) fn last_other_allergy(&self) -> Option<(&str, &str)> {
        self.people.iter().rev().find_map(|person| {
            person
                .food
                .latest_allergen()
                .map(|what| (person.role.display, what))
        })
    }

    pub(super) fn own_has_allergy(&self) -> bool {
        self.food.has_allergy()
    }

    pub(crate) fn reported_allergens(&self) -> impl Iterator<Item = &str> {
        self.food.reported_allergens().chain(
            self.people
                .iter()
                .flat_map(|person| person.food.reported_allergens()),
        )
    }

    pub(crate) fn avoids_meat_or_fish(&self) -> bool {
        self.food.avoids_meat_or_fish()
    }

    pub(crate) fn allows_plant_menu(&self, vegan: bool) -> bool {
        self.food.allows_plant_menu(vegan)
            && self
                .people
                .iter()
                .all(|person| person.food.allows_plant_menu(vegan))
    }

    pub(crate) fn has_food_limits(&self) -> bool {
        self.food.has_limits() || self.people.iter().any(|person| person.food.has_limits())
    }

    pub(crate) fn observe(&mut self, reading: &Reading<'_>, turns: usize) {
        self.schedule.observe(reading, turns);
        if let Some(name) = surname::given(reading.raw) {
            self.surname = Some(name);
        }
        if let Some(amount) = budget::given(reading) {
            self.budget = Some(amount);
        }
        self.observe_food(reading);
        self.name_correction = None;
        self.home_mentioned = None;
        let raw = reading.raw.replace(['’', '‘', 'ʼ'], "'");
        let flat = raw.split_whitespace().collect::<Vec<_>>().join(" ");
        let main = LEAD.replace(tidy(&reading.clean), "");
        let corrected = CORRECTION_LEAD.is_match(&flat);
        let parts = clauses(&flat);
        self.observe_favourite(&flat, &main, &parts, turns);
        if !ask(&flat) {
            for found in WHOS_AGE.captures_iter(&flat) {
                if let Some((role, Fact::Age(age))) = capture(&found, "who")
                    .and_then(relation)
                    .zip(capture(&found, "rest").and_then(|rest| person_fact(rest.trim(), false)))
                {
                    let index = self.ensure_person(role, None);
                    self.update_person(index, Fact::Age(age), corrected);
                }
            }
        }
        for part in parts {
            if ask(part.trim()) {
                continue;
            }
            let body = LEAD.replace(tidy(part), "");
            if !body.is_empty() {
                self.observe_clause(&body, corrected || CORRECTION_LEAD.is_match(tidy(part)));
            }
        }
    }

    fn observe_food(&mut self, reading: &Reading<'_>) {
        let previous_own = self.own_limits_last_turn.take();
        let previous_other = self.other_limits_last_turn.take();
        (self.food_update, self.own_limits_last_turn) = self.food.observe(reading);
        self.shared_food = None;
        match food::sharing(&reading.clean) {
            Some(food::ShareTarget::Own) => {
                if let Some(limits) = previous_other {
                    self.food.share(&limits);
                    self.shared_food = Some((None, limits));
                }
            }
            Some(food::ShareTarget::Person(who)) => {
                if let Some((role, limits)) = relation(who).zip(previous_own) {
                    let person = self.ensure_person(role, None);
                    self.people[person].food.share(&limits);
                    self.focus = Some(person);
                    self.shared_food = Some((Some(role.display), limits));
                }
            }
            Some(food::ShareTarget::Last) => {
                if let Some((person, limits)) = self.focus.zip(previous_own) {
                    self.people[person].food.share(&limits);
                    self.shared_food = Some((Some(self.people[person].role.display), limits));
                }
            }
            None => {
                if self.food_update.is_none()
                    && let Some(person) = self
                        .people
                        .iter_mut()
                        .rev()
                        .find(|person| person.food.has_limits())
                    && let Some(taken) = person.food.take_back_shared(&reading.clean)
                {
                    let article = match person.role.gender {
                        Gender::Female => "her",
                        Gender::Male => "him",
                        Gender::Unspecified => "them",
                    };
                    let verb = if person.role.display.ends_with('s') {
                        "aren't"
                    } else {
                        "isn't"
                    };
                    self.food_update = Some(format!(
                        "Your {} {verb} {taken}, then - I've taken that back off {article}.",
                        person.role.display
                    ));
                }
            }
        }
    }

    fn observe_clause(&mut self, line: &str, corrected: bool) {
        if let Some(groups) = OWN_HOME.captures(line) {
            self.store_home(&groups);
        } else if let Some(value) = ORIGIN
            .captures(line)
            .and_then(|groups| place(capture(&groups, "value")?))
        {
            self.home_mentioned = Some(value.clone());
            self.origin = Some(value);
        } else if let Some(rest) = line
            .strip_prefix("I ")
            .or_else(|| line.strip_prefix("i "))
            .or_else(|| line.strip_prefix("We "))
            .or_else(|| line.strip_prefix("we "))
            && let Some(value) = taste(rest)
        {
            save_taste(&mut self.tastes, value);
        } else {
            self.observe_person(line, corrected);
        }
    }

    fn store_home(&mut self, groups: &Captures<'_>) {
        let value = capture(groups, "value").unwrap_or_default();
        let preposition = capture(groups, "preposition")
            .unwrap_or_default()
            .to_lowercase();
        let home = if preposition == "from" {
            let Some(moved) = MOVE_FROM.captures(value) else {
                return;
            };
            let Some(home) = capture(&moved, "home").and_then(place) else {
                return;
            };
            if self.origin.is_none() {
                self.origin = capture(&moved, "origin").and_then(place);
            }
            Some(home)
        } else if let Some(moved) = MOVE_TO.captures(value) {
            let Some(home) = capture(&moved, "home").and_then(place) else {
                return;
            };
            if self.origin.is_none() {
                self.origin = capture(&moved, "origin").and_then(place);
            }
            Some(home)
        } else {
            place(value)
        };
        if let Some(place) = home {
            self.home_mentioned = Some(place.clone());
            self.home = Some(Residence::new(place, &preposition));
        }
    }

    fn observe_person(&mut self, line: &str, corrected: bool) {
        if let Some(groups) = THE_PLACE_AGE.captures(line)
            && let Some((role, Fact::Age(age))) = capture(&groups, "who")
                .and_then(relation)
                .zip(capture(&groups, "rest").and_then(|rest| person_fact(rest, false)))
        {
            let index = self.ensure_person(role, None);
            self.update_person(index, Fact::Age(age), corrected);
            return;
        }
        if let Some(groups) = KIN.captures(line)
            && let Some(age) = capture(&groups, "rest").and_then(|rest| THEIR_AGE.captures(rest))
            && let Some((role, Fact::Age(age))) = capture(&groups, "role")
                .and_then(|role| {
                    relation(role).or_else(|| role.strip_suffix('s').and_then(relation))
                })
                .zip(capture(&age, "rest").and_then(|rest| person_fact(rest, false)))
        {
            let index = self.ensure_person(role, None);
            self.update_person(index, Fact::Age(age), corrected);
            return;
        }
        if let Some(groups) = KIN.captures(line)
            && let Some((role, rest)) = person_and_fact(&groups)
        {
            let direct_name = if groups.name("possessive").is_some() {
                rest.strip_prefix("name is ")
                    .or_else(|| rest.strip_prefix("name was "))
                    .and_then(name)
            } else {
                None
            };
            if let Some(new_name) = direct_name {
                let index = self.ensure_person(role, None);
                self.update_person(index, Fact::Name(new_name), corrected);
            } else if groups.name("possessive").is_none() {
                if let Some(named) = NAMED_FACT.captures(rest)
                    && let Some((person_name, fact)) = capture(&named, "name")
                        .and_then(name)
                        .zip(capture(&named, "rest").and_then(|rest| person_fact(rest, false)))
                {
                    let index = self.ensure_person(role, Some(person_name));
                    self.update_person(index, fact, corrected);
                } else if let Some(fact) = person_fact(rest, true) {
                    let index = self.ensure_person(role, None);
                    self.update_person(index, fact, corrected);
                } else {
                    self.focus = Some(self.ensure_person(role, None));
                }
            }
            return;
        }
        if let Some(groups) = POSSESSIVE_NAME.captures(line)
            && let Some(index) = self.resolve(first(line))
            && let Some(value) = capture(&groups, "value").and_then(name)
        {
            self.update_person(index, Fact::Name(value), corrected);
            return;
        }
        if let Some(groups) = PRONOUN.captures(line)
            && let Some(pronoun) = capture(&groups, "pronoun")
            && let Some(rest) = capture(&groups, "rest")
            && let Some(fact) = person_fact(rest, groups.name("contraction").is_some())
        {
            if let Some(index) = self.resolve(pronoun) {
                self.update_person(index, fact, corrected);
            } else if let Fact::Age(age) = fact {
                if pronoun.eq_ignore_ascii_case("she") {
                    self.unnamed_she_age = Some(age);
                } else if pronoun.eq_ignore_ascii_case("he") {
                    self.unnamed_he_age = Some(age);
                }
            }
            return;
        }
        if let Some(groups) = NAMED_FACT.captures(line)
            && let Some(index) = capture(&groups, "name").and_then(|who| self.resolve(who))
            && let Some(fact) = capture(&groups, "rest").and_then(|rest| person_fact(rest, false))
        {
            self.update_person(index, fact, corrected);
        }
    }

    fn ensure_person(&mut self, role: Relation, person_name: Option<String>) -> usize {
        if let Some(index) = self.people.iter().rposition(|person| {
            person.role.key == role.key
                && (!matches!(role.key, "son" | "daughter" | "child")
                    || person.role.source == role.source)
                && person_name.as_ref().is_none_or(|given| {
                    person
                        .name
                        .as_ref()
                        .is_none_or(|kept| same_value(given, kept))
                })
        }) {
            if person_name.is_some() && self.people[index].name.is_none() {
                self.people[index].name = person_name;
            }
            return index;
        }
        let mut person = Person::new(role);
        person.name = person_name;
        self.people.push(person);
        self.people.len() - 1
    }

    fn update_person(&mut self, index: usize, fact: Fact, corrected: bool) {
        let person = &mut self.people[index];
        person.filed = true;
        match fact {
            Fact::Name(value) => {
                if let Some(old) = person.name.replace(value.clone())
                    && corrected
                    && !same_value(&old, &value)
                {
                    self.name_correction = Some((old, value));
                }
            }
            Fact::Home => (),
            Fact::Origin(value) => person.origin = Some(value),
            Fact::Age(value) => {
                person.age = Some(match value {
                    Age::Months(number) => {
                        person.age_months = Some(number);
                        Age::Months(number)
                    }
                    Age::Years(number) => {
                        person.age_years = Some(number);
                        Age::Years(number)
                    }
                    Age::FutureYears(number) => {
                        person.age_years.get_or_insert(number);
                        Age::Years(number)
                    }
                });
            }
            Fact::Taste(value) => save_taste(&mut person.tastes, value),
            Fact::Food(value) => {
                self.other_limits_last_turn = value.limits();
                person.food.apply(value, false);
            }
        }
        self.focus = Some(index);
    }

    fn resolve(&self, subject: &str) -> Option<usize> {
        let low = subject.to_lowercase();
        let cleaned = low.trim_end_matches("'s");
        if let Some(role) = cleaned
            .strip_prefix("my ")
            .or_else(|| cleaned.strip_prefix("our "))
            .and_then(|role| relation(role).or_else(|| relation(role.trim_end_matches('s'))))
        {
            return self
                .people
                .iter()
                .rposition(|person| person.role.key == role.key);
        }
        let gender = match cleaned {
            "she" | "her" => Some(Gender::Female),
            "he" | "him" | "his" => Some(Gender::Male),
            "they" | "them" | "their" => Some(Gender::Unspecified),
            _ => None,
        };
        if let Some(gender) = gender {
            let compatible =
                |person: &Person| gender == Gender::Unspecified || person.role.gender == gender;
            if let Some(index) = self.focus.filter(|&index| compatible(&self.people[index])) {
                return Some(index);
            }
            let mut candidates = self
                .people
                .iter()
                .enumerate()
                .filter(|(_, person)| compatible(person));
            let (index, _) = candidates.next().or_else(|| {
                (self.people.len() == 1 && self.people[0].role.gender == Gender::Unspecified)
                    .then(|| (0, &self.people[0]))
            })?;
            return candidates.next().is_none().then_some(index);
        }
        self.people.iter().rposition(|person| {
            person
                .name
                .as_ref()
                .is_some_and(|name| same_value(name, cleaned))
        })
    }

    pub(crate) fn respond_home(&self, memory: &Memory) -> Option<String> {
        let place = self.home_mentioned.as_deref()?;
        Some(pick(
            &[
                format!("{place} — nice. How long have you been there?"),
                format!("{place} — what's it like there?"),
                format!("{place}? Good to know. Do you like it there?"),
            ],
            memory,
            "",
        ))
    }

    pub(crate) fn respond(&self, reading: &Reading<'_>, memory: &Memory) -> Option<String> {
        let plain = LEAD.replace(tidy(&reading.clean), "");
        let plain = plain.as_ref();
        if let Some((old, new)) = &self.name_correction {
            return Some(format!("{new}, not {old} — got it."));
        }
        if self.food_update.is_some() {
            return self.food_update.clone();
        }
        if let Some((role, food)) = &self.shared_food
            && let Some(reply) = food.respond(*role, memory)
        {
            return Some(reply);
        }
        if let Some(reply) = surname::respond(reading.raw, self.surname.as_deref(), memory) {
            return Some(reply);
        }
        let other = self
            .people
            .iter()
            .rev()
            .find(|person| person.food.has_allergy())
            .map(|person| (person.role.display, &person.food));
        if let Some(reply) = self.food.respond(reading, memory, other) {
            return Some(recall_reply(reply, memory));
        }
        if let Some(reply) = self.schedule.respond(reading, memory) {
            return Some(reply);
        }
        if let Some(reply) = budget::respond(reading, memory, self.budget.as_deref()) {
            return Some(reply);
        }
        if let Some(reply) = places::changed(reading.raw, memory) {
            return Some(reply);
        }
        if HOME_ASK.is_match(plain) {
            return Some(recall_reply(self.home_reply(memory), memory));
        }
        if ORIGIN_ASK.is_match(plain) {
            return Some(recall_reply(self.origin_reply(memory), memory));
        }
        if GREW_UP_ASK.is_match(plain) {
            return Some(recall_reply(self.grew_up_reply(memory), memory));
        }
        if let Some(found) = HOME_CONFIRM.captures(plain) {
            let home = self.home.as_ref()?;
            let same = same_value(&home.place, capture(&found, "value")?);
            let yes_no = if same { "Yes" } else { "No" };
            return Some(format!(
                "{yes_no} — you live {} {}.",
                home.preposition, home.place
            ));
        }
        if matches!(
            plain,
            "what do i like"
                | "what did i say i like"
                | "what have i told you i like"
                | "what do i love"
                | "what do i dislike"
                | "what don't i like"
        ) {
            let negatives_only = plain.contains("dislike") || plain.contains("don't like");
            return self.tastes_reply(negatives_only);
        }
        if asks_favourite(plain).is_some_and(|category| self.kept_favourite(&category).is_none())
            && !self.tastes.is_empty()
        {
            return self.tastes_reply(false);
        }
        if let Some(found) = TASTE_CONFIRM.captures(plain) {
            let subject = capture(&found, "subject")?;
            let value = capture(&found, "value")?;
            let (person, tastes) = if subject == "i" {
                (None, &self.tastes)
            } else {
                let person = &self.people[self.resolve(subject)?];
                (Some(person), &person.tastes)
            };
            let value = value.trim_end_matches(" again").trim_end_matches(" now");
            let value = value
                .strip_prefix("my ")
                .map_or_else(|| value.to_owned(), |name| format!("your {name}"));
            let found = tastes
                .iter()
                .find(|taste| same_value(&taste.subject, &value))?;
            return Some(match person {
                None if found.likes => format!("You do, you told me you like {}.", found.subject),
                None => format!(
                    "You don't, you told me you're not keen on {}.",
                    found.subject
                ),
                Some(who) => format!(
                    "Your {} {} {}.",
                    who.role.display, found.verb, found.subject
                ),
            });
        }
        self.person_reply(plain, memory)
            .map(|reply| recall_reply(reply, memory))
            .or_else(|| summary::respond(reading, memory))
            .or_else(|| earlier::recall(reading, memory))
            .or_else(|| again::respond(reading, memory))
    }

    fn home_reply(&self, memory: &Memory) -> String {
        if let Some(home) = &self.home {
            return format!("You live {} {}.", home.preposition, home.place);
        }
        if let Some(origin) = &self.origin {
            return pick(
                &[
                    format!("You haven't told me where you live — I know you're from {origin}."),
                    format!("I don't know where you live. You're from {origin}, that I do know."),
                ],
                memory,
                "",
            );
        }
        "You haven't mentioned a place yet.".to_owned()
    }

    fn origin_reply(&self, memory: &Memory) -> String {
        let home = self.home.as_ref().map(|home| home.place.as_str());
        match (self.origin.as_deref(), home) {
            (Some(origin), Some(home)) if !same_value(origin, home) => pick(
                &[
                    format!("You're from {origin} originally, and you live in {home} now."),
                    format!("You said you're from {origin}, and you're in {home} now."),
                ],
                memory,
                "",
            ),
            (Some(origin), _) => pick(
                &[
                    format!("You're from {origin}."),
                    format!("{origin} — you told me you're from there."),
                ],
                memory,
                "",
            ),
            (None, Some(home)) => pick(
                &[
                    format!("You haven't said where you're from — I know you live in {home}."),
                    format!("Not sure where you're from. You mentioned {home} though."),
                ],
                memory,
                "",
            ),
            (None, None) => compose::pick(
                &[
                    "You haven't said where you're from yet.",
                    "I don't know that one — you haven't said.",
                ],
                memory,
                "",
            )
            .into(),
        }
    }

    fn grew_up_reply(&self, memory: &Memory) -> String {
        if let Some(origin) = &self.origin {
            return pick(
                &[
                    format!("You grew up in {origin}."),
                    format!("{origin} — that's where you grew up."),
                ],
                memory,
                "",
            );
        }
        if let Some(home) = &self.home {
            return pick(
                &[
                    format!(
                        "You haven't said where you grew up — I know you live in {}.",
                        home.place
                    ),
                    format!(
                        "Not sure where you grew up. You mentioned {} though.",
                        home.place
                    ),
                ],
                memory,
                "",
            );
        }
        compose::pick(
            &[
                "You haven't said where you grew up yet.",
                "I don't know that one — you haven't told me where you grew up.",
            ],
            memory,
            "",
        )
        .into()
    }

    fn person_reply(&self, line: &str, memory: &Memory) -> Option<String> {
        if let Some((subject, allergy)) = food::asked_relation(line)
            && let Some(index) = self.resolve(subject)
        {
            let person = &self.people[index];
            return person.food.relation_reply(person.role.display, allergy);
        }
        if let Some(found) = PERSON_NAME_ASK.captures(line) {
            let subject = capture(&found, "subject").or_else(|| capture(&found, "owner"))?;
            let person = &self.people[self.resolve(subject)?];
            let relation = person.role.display;
            let Some(name) = &person.name else {
                return Some(format!("You haven't told me your {relation}'s name."));
            };
            return Some(pick(
                &[
                    format!("Your {relation} is called {name}."),
                    format!("{name} — your {relation}, you told me."),
                    format!("You said your {relation} is called {name}."),
                ],
                memory,
                "",
            ));
        }
        if let Some(found) = PERSON_HOME_ASK.captures(line) {
            let subject = capture(&found, "subject")?;
            let role = self
                .resolve(subject)
                .map(|index| self.people[index].role.display);
            return places::respond(role, subject, memory);
        }
        if let Some((person, _)) = self.asked_person(&PERSON_ORIGIN_ASK, line) {
            return Some(format!(
                "Your {} is from {}.",
                person.role.display,
                person.origin.as_deref()?
            ));
        }
        if let Some(found) = PERSON_AGE_ASK.captures(line) {
            let subject = capture(&found, "subject")?;
            if let Some(person) = self.resolve(subject).map(|index| &self.people[index]) {
                return Some(format!(
                    "Your {} is {}.",
                    person.role.display,
                    person.age.as_ref()?
                ));
            }
            return match subject {
                "she" => Some(format!("She is {}.", self.unnamed_she_age.as_ref()?)),
                "he" => Some(format!("He is {}.", self.unnamed_he_age.as_ref()?)),
                _ => None,
            };
        }
        let (person, found) = self.asked_person(&PERSON_TASTES_ASK, line)?;
        let verb = capture(&found, "verb")?;
        let likes = !matches!(verb, "dislike" | "hate" | "not like" | "not enjoy");
        let values: Vec<_> = person
            .tastes
            .iter()
            .filter(|taste| taste.likes == likes)
            .collect();
        let first = values.first()?;
        let subjects = join_list(values.iter().map(|taste| taste.subject.as_str()));
        Some(format!(
            "Your {} {} {subjects}.",
            person.role.display, first.verb
        ))
    }

    fn asked_person<'a>(&self, pattern: &Regex, line: &'a str) -> Option<(&Person, Captures<'a>)> {
        let found = pattern.captures(line)?;
        let person = &self.people[self.resolve(capture(&found, "subject")?)?];
        Some((person, found))
    }

    fn tastes_reply(&self, negatives_only: bool) -> Option<String> {
        let mut sentences = Vec::new();
        let likes = join_list(
            self.tastes
                .iter()
                .filter(|taste| taste.likes)
                .map(|taste| taste.subject.as_str()),
        );
        if !negatives_only && !likes.is_empty() {
            sentences.push(format!("You like {likes}."));
        }
        sentences.extend(
            self.tastes
                .iter()
                .filter(|taste| !taste.likes)
                .map(|taste| format!("You're not keen on {}.", taste.subject)),
        );
        (!sentences.is_empty()).then(|| sentences.join(" "))
    }

    pub(super) fn summary_facts(&self) -> Vec<String> {
        let mut parts = Vec::new();
        if let Some(home) = &self.home {
            parts.push(format!("you live {} {}", home.preposition, home.place));
        }
        if let Some(origin) = &self.origin
            && self
                .home
                .as_ref()
                .is_none_or(|home| !same_value(origin, &home.place))
        {
            parts.push(format!("you're from {origin}"));
        }
        parts.extend(self.tastes.iter().map(|taste| {
            let words = if taste.likes {
                "you like"
            } else {
                "you're not keen on"
            };
            format!("{words} {}", taste.subject)
        }));
        parts.extend(self.food.allergy_fact(None));
        for person in &self.people {
            let role = person.role.display;
            parts.extend(person.food.allergy_fact(Some(role)));
            let age = person.age.as_ref().map(ToString::to_string);
            let child = matches!(
                person.role.key,
                "son" | "daughter" | "child" | "baby" | "toddler"
            );
            match (person.name.as_deref(), age) {
                (Some(name), Some(age)) if child => parts.push(format!("{name} is {age}")),
                (Some(name), Some(age)) => {
                    parts.push(format!("your {role} is called {name} and is {age}"))
                }
                (Some(name), None) => parts.push(format!("your {role} is called {name}")),
                (None, Some(age)) => parts.push(format!("your {role} is {age}")),
                (None, None) => {}
            }
            parts.extend(person.food.diet_fact(Some(role)));
        }
        parts.extend(self.food.diet_fact(None));
        parts.extend(self.food.condition_fact());
        parts.extend(
            self.budget
                .as_ref()
                .map(|amount| format!("your budget is {amount}")),
        );
        parts.extend(self.food.denied_diet_summary().map(str::to_owned));
        parts
    }

    pub(super) fn covers_statement(&self, text: &str) -> bool {
        let text = text.replace(['’', '‘', 'ʼ'], "'");
        let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if ask(&flat) {
            return false;
        }
        if food::Change::from(&flat, true).is_some() {
            return true;
        }
        let body = LEAD.replace(tidy(&flat), "");
        let body = body.as_ref();
        if OWN_HOME.is_match(body) || ORIGIN.is_match(body) {
            return true;
        }
        if let Some(rest) = body
            .strip_prefix("I ")
            .or_else(|| body.strip_prefix("i "))
            .or_else(|| body.strip_prefix("We "))
            .or_else(|| body.strip_prefix("we "))
            && taste(rest).is_some()
        {
            return true;
        }
        if let Some(groups) = KIN.captures(body)
            && capture(&groups, "role").and_then(relation).is_some()
            && let Some(rest) = capture(&groups, "rest")
        {
            return if groups.name("possessive").is_some() {
                rest.starts_with("name is ") || rest.starts_with("name was ")
            } else {
                person_fact(rest, true).is_some()
                    || NAMED_FACT
                        .captures(rest)
                        .and_then(|named| person_fact(capture(&named, "rest")?, false))
                        .is_some()
            };
        }
        NAMED_FACT.captures(body).is_some_and(|groups| {
            capture(&groups, "name")
                .and_then(|name| self.resolve(name))
                .is_some()
                && capture(&groups, "rest")
                    .and_then(|rest| person_fact(rest, false))
                    .is_some()
        })
    }

    fn kept_favourite(&self, name: &str) -> Option<&str> {
        let singular = name
            .strip_suffix('s')
            .filter(|_| name.len() > 3 && !name.ends_with("ss"));
        self.favourites
            .iter()
            .find(|(category, _)| category == name || Some(category.as_str()) == singular)
            .map(|(_, value)| value.as_str())
    }

    pub(in crate::bot) fn favourite_value_for(&self, line: &str) -> Option<&str> {
        let line = LEAD.replace(tidy(line), "");
        self.kept_favourite(&asks_favourite(&line)?)
    }

    fn observe_favourite(&mut self, raw: &str, main: &str, clauses: &[&str], turns: usize) {
        self.favourite_update = None;
        let pending = self.pending_favourite.take();
        if let Some(category) = asks_favourite(main) {
            if self.kept_favourite(&category).is_none() && self.tastes.is_empty() {
                self.pending_favourite = Some(category);
            }
            return;
        }
        if let Some((category, value)) = clauses.iter().find_map(|clause| {
            let clause = clause.trim();
            // The pinned favourite reader's statements do not consume a
            // terminal sentence mark. The original line stays in the transcript.
            if ask(clause) || clause.ends_with(['.', '!']) {
                return None;
            }
            let plain = LEAD.replace(tidy(clause), "");
            let found = FAVOURITE
                .captures(&plain)
                .or_else(|| REVERSE_FAVOURITE.captures(&plain))?;
            Some((
                category(capture(&found, "category")?)?,
                favourite_value(capture(&found, "value")?)?,
            ))
        }) {
            let request = clauses
                .iter()
                .filter(|clause| !clause.trim().is_empty())
                .count()
                > 1
                && clauses.iter().any(|clause| ask(clause.trim()));
            self.save_favourite(category.clone(), value, turns);
            if !request {
                self.favourite_update = Some(FavouriteUpdate::Given(category));
            }
            return;
        }
        if ask(raw) {
            return;
        }
        let raw = tidy(raw);
        if let Some((category, learned_on)) = self.recent_favourite.clone() {
            if learned_on + 1 == turns
                && let Some(value) = FAVOURITE_CORRECTION
                    .captures(raw)
                    .and_then(|groups| favourite_value(capture(&groups, "value")?))
            {
                self.save_favourite(category.clone(), value, turns);
                self.favourite_update = Some(FavouriteUpdate::Answered(category));
                return;
            }
            if turns.saturating_sub(learned_on) <= 2
                && let Some(value) = PREFERS_NOW
                    .captures(raw)
                    .and_then(|groups| favourite_value(capture(&groups, "value")?))
                && let Some(previous) = self
                    .kept_favourite(&category)
                    .filter(|previous| !same_value(previous, &value))
                    .map(str::to_owned)
            {
                self.save_favourite(category.clone(), value, turns);
                self.favourite_update = Some(FavouriteUpdate::Preferred { category, previous });
                return;
            }
        }
        if let Some(category) = pending
            && FAVOURITE_RESPONSE.is_match(raw)
            && !raw.to_lowercase().split_whitespace().all(|word| {
                matches!(
                    word,
                    "thanks"
                        | "thank"
                        | "you"
                        | "cheers"
                        | "ta"
                        | "ok"
                        | "okay"
                        | "cool"
                        | "nice"
                        | "sure"
                        | "yes"
                        | "yeah"
                        | "no"
                        | "nope"
                        | "nothing"
                )
            })
            && !raw.split_whitespace().any(|word| {
                matches!(
                    word.to_lowercase().as_str(),
                    "am" | "is"
                        | "are"
                        | "have"
                        | "would"
                        | "like"
                        | "love"
                        | "go"
                        | "going"
                        | "live"
                        | "want"
                        | "need"
                )
            })
            && let Some(value) = favourite_value(raw)
        {
            self.save_favourite(category.clone(), value, turns);
            self.favourite_update = Some(FavouriteUpdate::Answered(category));
        }
    }

    fn save_favourite(&mut self, category: String, value: String, turns: usize) {
        self.recent_favourite = Some((category.clone(), turns));
        if let Some((_, old)) = self
            .favourites
            .iter_mut()
            .find(|(name, _)| *name == category)
        {
            *old = value;
        } else {
            self.favourites.push((category, value));
        }
    }

    pub(crate) fn respond_favourite(
        &self,
        reading: &Reading<'_>,
        memory: &Memory,
    ) -> Option<String> {
        let line = LEAD.replace(tidy(&reading.clean), "");
        let raw = reading.raw;
        if let Some(update) = &self.favourite_update {
            match update {
                FavouriteUpdate::Answered(category) => {
                    let value = swap_possessives(self.kept_favourite(category)?);
                    return Some(format!("Noted: your favourite {category} is {value}."));
                }
                FavouriteUpdate::Preferred { category, previous } => {
                    let value = swap_possessives(self.kept_favourite(category)?);
                    let previous = swap_possessives(previous);
                    let shown = upper_first(&value);
                    return Some(pick(
                        &[
                            format!(
                                "{shown} now, then — I have your favourite {category} as {value}."
                            ),
                            format!(
                                "Changed: your favourite {category} is {value} now, not {previous}."
                            ),
                        ],
                        memory,
                        raw,
                    ));
                }
                FavouriteUpdate::Given(category) => {
                    // PyBot stores a new favourite even when an indexed answer
                    // handles the line. The receipt stands down in that case.
                    if crate::bot::knowledge::lookup::lookup_in_context(reading, memory).is_some() {
                        return None;
                    }
                    let value = swap_possessives(self.kept_favourite(category)?);
                    let mut options = vec![
                        format!("Noted: your favourite {category} is {value}."),
                        format!("Favourite {category}: {value}. Got it."),
                    ];
                    if value.split_whitespace().count() <= 2
                        && !value.starts_with("your ")
                        && !value.starts_with("the ")
                    {
                        options.push(format!(
                            "{} for your favourite {category}, then — noted.",
                            upper_first(&value)
                        ));
                    }
                    return Some(pick(&options, memory, raw));
                }
            }
        }
        if ALL_FAVOURITES.is_match(&line) && !self.favourites.is_empty() {
            let parts: Vec<_> = self
                .favourites
                .iter()
                .skip(self.favourites.len().saturating_sub(4))
                .map(|(category, value)| {
                    format!("your favourite {category} is {}", swap_possessives(value))
                })
                .collect();
            let joined = join_list(parts.iter().map(String::as_str));
            let amount = if parts.len() == 1 { "one: " } else { "" };
            return Some(pick(
                &[
                    format!("You have told me {amount}{joined}."),
                    format!("Here is what I have: {amount}{joined}."),
                ],
                memory,
                raw,
            ));
        }
        let asked = asks_favourite(&line)?;
        if let Some(value) = self.kept_favourite(&asked) {
            let value = swap_possessives(value);
            return Some(pick(
                &[
                    format!("Your favourite {asked} is {value}."),
                    format!("You said {value} — that is your favourite {asked}."),
                    format!(
                        "{}, you said. That is your favourite {asked}.",
                        upper_first(&value)
                    ),
                ],
                memory,
                raw,
            ));
        }
        if !self.tastes.is_empty() {
            return None;
        }
        Some(pick(
            &[
                format!("You haven't told me your favourite {asked} — what is it?"),
                format!("I don't have a favourite {asked} for you yet. What is it?"),
                format!("Your favourite {asked} hasn't come up yet. What is it?"),
            ],
            memory,
            raw,
        ))
    }
}

pub(super) fn pick(options: &[String], memory: &Memory, seed: &str) -> String {
    compose::pick(
        &options.iter().map(String::as_str).collect::<Vec<_>>(),
        memory,
        seed,
    )
    .into()
}

fn recall_reply(reply: String, memory: &Memory) -> String {
    again::frame_short(&reply, memory).unwrap_or(reply)
}

pub(super) fn join_list<'a>(values: impl Iterator<Item = &'a str>) -> String {
    let values: Vec<_> = values.collect();
    match values.split_last() {
        Some((last, [])) => (*last).to_owned(),
        Some((last, before)) => format!("{} and {last}", before.join(", ")),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observe(personal: &mut Personal, lines: &[&str]) {
        for (turn, line) in lines.iter().enumerate() {
            personal.observe(&Reading::new(line), turn);
        }
    }

    fn recall(personal: &Personal, line: &str) -> Option<String> {
        personal.respond(&Reading::new(line), &Memory::default())
    }

    #[test]
    fn residence_and_origin_are_distinct_and_questions_do_not_change_them() {
        let mut personal = Personal::default();
        observe(
            &mut personal,
            &[
                "I live in Leeds",
                "I'm from Belfast originally",
                "I moved to York",
                "Do I live in Bristol?",
            ],
        );
        assert_eq!(
            recall(&personal, "Where do I live now?").as_deref(),
            Some("You live in York.")
        );
        assert_eq!(
            recall(&personal, "Do I live in Bristol?").as_deref(),
            Some("No — you live in York.")
        );
        assert!(
            recall(&personal, "Where am I from?")
                .unwrap()
                .contains("Belfast")
        );
    }

    #[test]
    fn named_relations_keep_distinct_locations_and_resolve_pronoun_updates() {
        let mut personal = Personal::default();
        let lines = [
            "My sister Maya lives in Bristol.",
            "I live in Leeds.",
            "My brother Theo is 26.",
            "Actually, she moved to Bath.",
            "Actually, she's called Maggie.",
        ];
        observe(&mut personal, &lines);
        assert_eq!(
            recall(&personal, "Actually, she's called Maggie").as_deref(),
            Some("Maggie, not Maya — got it.")
        );
        personal.observe(&Reading::new("Where does Maggie live?"), 5);
        let history = Memory {
            turns: lines
                .iter()
                .map(|line| crate::bot::Exchange {
                    user: (*line).into(),
                    bot: "Got it.".into(),
                    module: "greeting",
                })
                .collect(),
            ..Memory::default()
        };
        assert_eq!(
            personal
                .respond(&Reading::new("Where does Maggie live?"), &history)
                .as_deref(),
            Some("Your sister lives in Bath.")
        );
        assert_eq!(
            recall(&personal, "How old is my brother?").as_deref(),
            Some("Your brother is 26.")
        );
        assert_eq!(
            recall(&personal, "Where do I live?").as_deref(),
            Some("You live in Leeds.")
        );
    }

    #[test]
    fn favourite_categories_take_corrections_and_short_answers_without_stealing_other_facts() {
        let mut personal = Personal::default();
        observe(
            &mut personal,
            &[
                "My favourite colour is blue",
                "Actually, it's violet",
                "My mum's favourite colour is green",
                "What is my favourite film?",
                "Blade Runner.",
                "Where do I live?",
            ],
        );
        assert_eq!(personal.kept_favourite("colour"), Some("violet"));
        assert_eq!(personal.kept_favourite("film"), Some("Blade Runner"));
        assert!(
            personal
                .respond_favourite(
                    &Reading::new("What's my favorite color now?"),
                    &Memory::default()
                )
                .unwrap()
                .contains("violet")
        );
        assert_eq!(
            recall(&personal, "Where do I live?").as_deref(),
            Some("You haven't mentioned a place yet.")
        );
    }

    #[test]
    fn favourite_declaration_leaves_the_source_punctuation_in_place() {
        let mut personal = Personal::default();
        observe(&mut personal, &["My favourite fruit is a mango."]);
        assert_eq!(personal.kept_favourite("fruit"), None);
        observe(&mut personal, &["My favourite fruit is a mango"]);
        assert_eq!(personal.kept_favourite("fruit"), Some("a mango"));
    }

    #[test]
    fn named_tastes_can_change_without_confusing_whose_taste_it_is() {
        let mut personal = Personal::default();
        observe(
            &mut personal,
            &[
                "My sister Maya loves cooking",
                "I love a good cup of tea",
                "She doesn't like cooking",
            ],
        );
        assert_eq!(
            recall(&personal, "What do I like?").as_deref(),
            Some("You like good cup of tea.")
        );
        assert_eq!(
            recall(&personal, "What does my sister dislike?").as_deref(),
            Some("Your sister doesn't like cooking.")
        );
        assert!(recall(&personal, "What does my sister like?").is_none());
    }

    #[test]
    fn age_range_recalls_current_age_without_inventing_a_named_person() {
        let mut personal = Personal::default();
        observe(&mut personal, &["She's 7 going on 8."]);
        assert_eq!(
            recall(&personal, "How old is she?").as_deref(),
            Some("She is 7.")
        );
        assert!(recall(&personal, "How old is my sister?").is_none());
    }
}
