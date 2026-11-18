//! The shape of answer owed to the current turn.

use super::{kinds::Kind, line::Line};
use fancy_regex::Regex;
use std::{collections::BTreeSet, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "intent_patterns.rs"]
mod patterns;
use patterns::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Wants {
    Time,
    Amount,
    Place,
    Person,
    Meaning,
    Yesno,
    Pick,
    Compare,
    Steps,
    Options,
    Reason,
    Opinion,
    Advice,
    Comfort,
    Answer,
}

impl Wants {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Amount => "amount",
            Self::Place => "place",
            Self::Person => "person",
            Self::Meaning => "meaning",
            Self::Yesno => "yesno",
            Self::Pick => "pick",
            Self::Compare => "compare",
            Self::Steps => "steps",
            Self::Options => "options",
            Self::Reason => "reason",
            Self::Opinion => "opinion",
            Self::Advice => "advice",
            Self::Comfort => "comfort",
            Self::Answer => "answer",
        }
    }

    pub(crate) fn from_str(value: &str) -> Option<Self> {
        Some(match value {
            "time" => Self::Time,
            "amount" => Self::Amount,
            "place" => Self::Place,
            "person" => Self::Person,
            "meaning" => Self::Meaning,
            "yesno" => Self::Yesno,
            "pick" => Self::Pick,
            "compare" => Self::Compare,
            "steps" => Self::Steps,
            "options" => Self::Options,
            "reason" => Self::Reason,
            "opinion" => Self::Opinion,
            "advice" => Self::Advice,
            "comfort" => Self::Comfort,
            "answer" => Self::Answer,
            _ => return None,
        })
    }
}

use Wants::*;

pub(super) type Asks = BTreeSet<&'static str>;

/// Return whether the turn asks for a duration without another ask label.
pub(crate) fn asks_only_how_long(line: &Line, kind: Kind) -> bool {
    let (asks, _) = ask_labels(line, kind, false);
    asks.len() == 1 && asks.contains("how_long")
}

/// Return whether the line adds any ask label outside a bare follow-up.
pub(crate) fn asks_only_what_or_yesno(line: &Line, kind: Kind) -> bool {
    let (asks, _) = ask_labels(line, kind, false);
    !has_other(&asks, &["what", "yesno"])
}

fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn starts(pattern: &Regex, text: &str) -> bool {
    pattern
        .find(text)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == 0)
}

fn may_contain(text: &str, required_fragments: &[&str]) -> bool {
    // Use literal gates only on lowercase ASCII input. Other input must go
    // through the regex's case-insensitive Unicode matching.
    required_fragments
        .iter()
        .any(|fragment| text.contains(fragment))
        || text
            .bytes()
            .any(|byte| !byte.is_ascii() || byte.is_ascii_uppercase())
}

fn fold(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn said(line: &Line) -> String {
    let words = line.norm.join(" ");
    fold(&[words.as_str(), &line.norm_text, &line.clean, &line.text].join(" "))
}

fn first_text<'a>(versions: &[&'a str]) -> &'a str {
    versions
        .iter()
        .copied()
        .find(|text| !text.trim().is_empty())
        .unwrap_or("")
}

/// Read the turn's answer shape, with a previous shape available for bare follow-ups.
pub(crate) fn wants(line: &Line, kind: Kind, previous: Option<Wants>) -> Option<Wants> {
    let said = said(line);
    let whole = fold(first_text(&[&line.plain, &line.text]));
    let once = fold(first_text(&[&line.plain, &line.text, &line.clean]));
    let (asks, asking) = ask_labels(line, kind, true);
    let verdict = verdict_ask(&line.text);
    let about_bot = about_the_bot(&once);
    let own = if matches(&A_RECALL_ASK, &said) {
        None
    } else {
        let own = if let Some(remembered) = recall(first_text(&[&whole, &said])) {
            Some(remembered)
        } else if about_bot {
            None
        } else if verdict {
            Some(Yesno)
        } else {
            wants_alone(line, kind, &said, &whole, &mut asks.clone(), asking)
        };
        shape_named_outside(&said, own)
            .or_else(|| bare_definition(&whole, about_bot).then_some(Meaning))
            .or_else(|| previous.filter(|_| follow_up(kind, &said, &whole, &asks, verdict)))
    };
    let own = lift_clause(line, &asks, own);
    correct_shape(&once, own)
}

const ASKED_FOR: [(Wants, &[&str]); 12] = [
    (Pick, &["choice", "compare", "which"]),
    (Meaning, &["definition"]),
    (Place, &["where"]),
    (Person, &["who"]),
    (Reason, &["why"]),
    (Time, &["how_long"]),
    (Amount, &["how_much", "how_often"]),
    (Amount, &["maths"]),
    (Options, &["list"]),
    (Steps, &["steps", "how"]),
    (Pick, &["recommend"]),
    (Yesno, &["yesno"]),
];

pattern!(
    BOT_QUESTION,
    r#"^(?:(?:hi|hello|hey|and|so|well)\s+)*(?:(?:what|who) (?:are|r) (?:you|u)(?: exactly)?[.?!]*$|are (?:you|u) (?:an? (?:ai|bot|human|robot|language model)|real|human|alive|sentient|smart|conscious|safe|biased)\b|how old (?:are|r) (?:you|u)\b|(?:where|how) (?:are|r) (?:you|u)\b|(?:what|which) (?:model|version) (?:are|r) (?:you|u)\b|do (?:you|u) have (?:a name|feelings|a body)\b|(?:can|could) (?:you|u) (?:remember|see|hear|feel) (?:me|what i said)\b|what (?:can|could) (?:you|u) do\b)"#
);
pattern!(TRIM_WORDS, r#"[^a-z0-9 ]+"#);

fn strip(pattern: &Regex, text: &str) -> String {
    pattern
        .try_replacen(text, 0, "")
        .map_or_else(|_| text.to_owned(), |text| text.into_owned())
}

fn intersects(asks: &Asks, labels: &[&str]) -> bool {
    labels.iter().any(|label| asks.contains(label))
}

fn has_other(asks: &Asks, allowed: &[&str]) -> bool {
    asks.iter().any(|word| !allowed.contains(word))
}

fn any_phrase(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| text.contains(phrase))
}

fn about_the_bot(once: &str) -> bool {
    starts(&BOT_QUESTION, once)
}

pub(super) fn verdict_ask(text: &str) -> bool {
    let lower = text.to_lowercase();
    let flat = TRIM_WORDS.replace_all(&lower, " ");
    let words: Vec<_> = flat.split_whitespace().collect();
    if words.is_empty() || words.len() > 30 {
        return false;
    }
    let full_match = |text: &str| {
        LABEL_VERDICT_ASK
            .find(text)
            .ok()
            .flatten()
            .is_some_and(|part| part.start() == 0 && part.end() == text.len())
    };
    if full_match(&words.join(" ")) {
        return true;
    }
    for begin in words.len().saturating_sub(7).max(1)..words.len().saturating_sub(1) {
        if matches(&LABEL_QUESTION_WORD, &words[..begin].join(" ")) {
            return false;
        }
        if full_match(&words[begin..].join(" ")) {
            return true;
        }
    }
    false
}

fn label_help(text: &str) -> bool {
    if matches(&LABEL_HELP_ASK_OWN_SUBJECT, text) {
        return false;
    }
    LABEL_HELP_ASK.find_iter(text).flatten().any(|found| {
        let tail = &text[found.end()..];
        !matches(&LABEL_HELP_ASK_NOT, &text[..found.start()])
            && !matches(&LABEL_HELP_ASK_SUBJECT, tail)
            && !matches(&LABEL_HELP_ASK_TAIL_CLAUSE, tail)
    })
}

fn tips(text: &str, words: &BTreeSet<&str>) -> bool {
    LABEL_TIPS.find_iter(text).flatten().any(|part| {
        let before = &text[..part.start()];
        let after = &text[part.end()..];
        if matches(&LABEL_TIPS_AMOUNT, before) {
            return false;
        }
        if matches(&LABEL_TIPS_WANTED, before) {
            return words.iter().any(|word| !LABEL_TIPS_FRAME.contains(word));
        }
        matches!(part.as_str(), "tips" | "hints" | "pointers")
            && !matches(&LABEL_TIPS_SUBJECT, before)
            && (starts(&LABEL_TIPS_ABOUT, after)
                || after.trim_matches([' ', '?', '.', '!', ',']).is_empty())
    })
}

fn recommends(norm: &[String], text: &str, words: &BTreeSet<&str>) -> bool {
    if any_phrase(text, LABEL_RECOMMEND_PHRASES)
        || LABEL_RECOMMEND
            .iter()
            .any(|word| !LABEL_COMPARATIVE.contains(word) && words.contains(word))
        || LABEL_COMPARATIVE.iter().any(|word| words.contains(word))
            && matches(&LABEL_COMPARISON, text)
        || tips(text, words)
    {
        return true;
    }
    norm.iter().enumerate().any(|(index, word)| {
        let before = index
            .checked_sub(1)
            .map_or("", |prior| norm[prior].as_str());
        let after = norm.get(index + 1).map_or("", String::as_str);
        LABEL_COMPARATIVE.contains(&word.as_str())
            && (!LABEL_A_PERSON_AFTER.contains(&after)
                || matches!(after, "man" | "woman")
                    && norm.get(index + 2).is_some_and(|word| word == "for"))
            && !LABEL_DESCRIBES.contains(&before)
            && !LABEL_AFTER_COMPARATIVE.contains(&after)
    })
}

fn opposite_scale(text: &str) -> bool {
    if !(starts(&LABEL_SCALE_ASK, text) || text.contains("sooner or later")) {
        return false;
    }
    matches(&LABEL_OPPOSITE_PAIRS, text)
}

pub(super) fn ask_labels(line: &Line, kind: Kind, prune_news: bool) -> (Asks, bool) {
    read_ask_labels(line, Some(kind), prune_news)
}

pub(super) fn raw_ask_labels(line: &Line) -> Asks {
    read_ask_labels(line, None, false).0
}

fn read_ask_labels(line: &Line, kind: Option<Kind>, prune_news: bool) -> (Asks, bool) {
    let mut asks = BTreeSet::new();
    let words: Vec<_> = line.norm.iter().map(String::as_str).collect();
    let filtered = super::kinds::gratitude_span(&words).map(|range| {
        line.norm[..range.start]
            .iter()
            .chain(&line.norm[range.end..])
            .cloned()
            .collect::<Vec<_>>()
    });
    let norm = filtered.as_deref().unwrap_or(&line.norm);
    let labels: BTreeSet<_> = norm.iter().map(String::as_str).collect();
    let has = |word: &str| labels.contains(word);
    let phrases = if filtered.is_some() {
        format!(" {} ", norm.join(" "))
    } else {
        format!(" {} ", line.norm_text)
    };
    let text = phrases.as_str();
    if kind == Some(Kind::Empty)
        || prune_news && matches!(kind, Some(Kind::Greeting | Kind::Thanks))
    {
        return (asks, false);
    }
    if has("how") {
        asks.insert(if has("long") {
            "how_long"
        } else if has("much") || has("many") {
            "how_much"
        } else if has("often") || has("frequently") {
            "how_often"
        } else {
            "how"
        });
    }
    for label in ["what", "why", "when", "where", "who", "which"] {
        if has(label) {
            asks.insert(label);
        }
    }
    let ask_the_room = matches!(
        norm.first().map(String::as_str),
        Some("anyone" | "anybody" | "someone" | "somebody")
    ) && line
        .norm
        .get(1)
        .is_some_and(|word| LABEL_ASK_FOR_A_NAME.contains(&word.as_str()));
    if ask_the_room || recommends(norm, text, &labels) {
        asks.insert("recommend");
    }
    if any_phrase(text, LABEL_STEPS) || matches(&LABEL_STEPS_WORD, text) || label_help(text) {
        asks.insert("steps");
    }
    if any_phrase(text, LABEL_OPINION) {
        asks.insert("opinion");
    }
    if any_phrase(text, LABEL_COMPARE) || has("vs") || has("versus") {
        asks.insert("compare");
    }
    let definition = may_contain(text, &["what"])
        && LABEL_BARE_DEFINITION
            .captures(text)
            .ok()
            .flatten()
            .and_then(|parts| parts.get(1))
            .is_some_and(|part| {
                !part.as_str().ends_with("'s")
                    && !part.as_str().ends_with("’s")
                    && !LABEL_NOT_A_DEFINITION_HEAD.contains(&part.as_str())
            });
    if !matches(&LABEL_ASKS_ABOUT_A_PERSON, text)
        && (any_phrase(text, LABEL_DEFINITION) || definition)
    {
        asks.insert("definition");
    }
    let number = strip(&LABEL_A_DATE_FIGURE, text)
        .bytes()
        .any(|byte| byte.is_ascii_digit());
    if LABEL_MATH_WORDS.iter().any(|word| has(word))
        || text.contains('%')
        || number && ["of", "each", "per", "split"].iter().any(|word| has(word))
    {
        asks.insert("maths");
    }
    if let Some((before, _)) = text.split_once(" or ")
        && !opposite_scale(text)
        && !matches(&LABEL_REFUSED_PAIR, before)
    {
        asks.insert("choice");
    }
    if ["list", "ideas", "options", "examples", "suggest"]
        .iter()
        .any(|word| has(word))
    {
        asks.insert("list");
    }
    if ["write", "word", "rewrite", "draft"]
        .iter()
        .any(|word| has(word))
    {
        asks.insert("write");
    }
    let Some(kind) = kind else {
        return (asks, false);
    };
    if kind == Kind::Question && super::kinds::yesno_clause(line) {
        asks.insert("yesno");
    }
    if kind != Kind::Question {
        super::kinds::remove_background_labels(&words, &mut asks);
    }
    let request = asks.contains("recommend")
        || any_phrase(text, LABEL_STEPS)
        || ["help", "need", "want"].iter().any(|word| has(word))
        || norm.first().is_some_and(|word| {
            matches!(
                word.as_str(),
                "tell" | "explain" | "show" | "give" | "write" | "make"
            )
        });
    let news = kind == Kind::Statement
        && !request
        && ["i", "me", "my", "we", "our"].iter().any(|word| has(word));
    if news && prune_news {
        asks.remove("choice");
        if !(matches(&SUM_OUT_LOUD, text) || matches(&SUM_ASKS, text)) {
            asks.remove("maths");
        }
    }
    (asks, kind == Kind::Question || request)
}

fn bare_definition(whole: &str, about_bot: bool) -> bool {
    if about_bot {
        return false;
    }
    let whole = strip(&LEADING_JOIN, whole);
    let whole = BARE_WHAT_AS_IS
        .try_replacen(&whole, 0, "what is")
        .unwrap_or_else(|_| whole.as_str().into());
    let Some(phrase) = BARE_WHAT_IS_PLAIN
        .captures(&whole)
        .ok()
        .flatten()
        .and_then(|parts| parts.name("thing"))
        .map(|part| part.as_str().trim())
    else {
        return false;
    };
    !phrase.is_empty()
        && !BARE_NOTHING_ASKED.contains(&phrase)
        && !starts(&BARE_NOT_A_THING_AT_THE_START, phrase)
        && (!starts(&BARE_VALUE_NOUN, phrase)
            || ["mean", "means", "work", "works"]
                .iter()
                .any(|word| phrase.split_whitespace().any(|found| &found == word)))
        && !matches(&BARE_NOT_A_THING_ANYWHERE, &whole)
        && !matches(&BARE_A_SECOND_QUESTION, phrase)
        && !matches(&BARE_A_RECALL, phrase)
        && !phrase.ends_with(" for")
}

fn follow_up(kind: Kind, said: &str, whole: &str, asks: &Asks, verdict: bool) -> bool {
    !verdict
        && matches(&FOLLOWUP, said)
        && !has_other(asks, &["what", "yesno"])
        && !matches(&NEW_ASK, said)
        && (kind == Kind::Question
            || said.ends_with('?')
            || kind == Kind::Statement
                && whole.split_whitespace().count() <= 6
                && matches(&FOLLOWUP, whole))
}

fn lift_clause(line: &Line, asks: &Asks, own: Option<Wants>) -> Option<Wants> {
    if matches!(
        own,
        Some(Pick | Options | Steps | Time | Amount | Place | Person | Meaning | Reason)
    ) {
        return own;
    }
    let said = fold(first_text(&[&line.clean, &line.text]));
    if matches(&NOTHING_TO_A_THING, &said) && !asks.contains("list") {
        return Some(Pick);
    }
    if matches(&WHAT_TO_LOOK, &said) {
        return Some(Steps);
    }
    let focus = fold(&line.focus.join(" "));
    if focus.is_empty()
        || focus == said
        || focus.chars().count() * 100 > 85 * said.chars().count().max(1)
        || !OPENS_A_QUESTION
            .iter()
            .any(|start| focus.starts_with(start))
    {
        return own;
    }
    let focus = format!(" {focus} ");
    if matches(&WHAT_TO_PICK, &focus) {
        Some(Pick)
    } else if matches(&WHAT_I_NEED, &focus) || matches(&WHAT_YOU_TAKE, &focus) {
        Some(Options)
    } else if matches(&WHAT_TO_STEPS, &focus)
        || matches(&WHAT_TO_DO, &focus)
        || matches(&WHAT_TO_LOOK, &focus)
    {
        Some(Steps)
    } else {
        own
    }
}

fn correct_shape(line: &str, own: Option<Wants>) -> Option<Wants> {
    if line.is_empty() {
        return own;
    }
    if matches(&SPEND_IT_ON, line) {
        return Some(Pick);
    }
    if let Some(found) = MONEY_VERB_ASK.find(line).ok().flatten() {
        let start = line[..found.start()]
            .rfind([',', ';', ':', '?', '.', '!'])
            .map_or(0, |at| at + 1);
        let tail = &line[found.end()..];
        let end = tail
            .find([',', ';', ':', '?', '.', '!'])
            .map_or(line.len(), |at| at + found.end());
        if !matches(&ALTERNATIVES, &line[start..end]) {
            return Some(Amount);
        }
    }
    if own.is_none() && matches(&PRESENT_LOOKING, line) && !matches(&NOT_A_PRESENT_ASK, line) {
        return Some(Options);
    }
    own
}

fn shape_named_outside(said: &str, own: Option<Wants>) -> Option<Wants> {
    if said.is_empty() || starts(&IF_IT_HAPPENS, said) {
        return own;
    }
    if (own == Some(Reason) && matches(&CONFIRM_CAUSE, said))
        || matches!(own, None | Some(Reason | Meaning)) && matches(&ASK_TO_ASK, said)
    {
        return Some(Yesno);
    }
    if own.is_none() && matches(&A_FIGURE_NAMED, said) {
        return Some(if matches(&WHAT_TIME_NAMED, said) {
            Time
        } else {
            Amount
        });
    }
    own
}

fn a_help_ask_that_names_nothing(text: &str) -> bool {
    let found = HELP_ASK_ANY
        .find(text)
        .ok()
        .flatten()
        .or_else(|| HELP_WORD.find(text).ok().flatten());
    matches(
        &HELP_ASK_NO_TASK,
        found.map_or(text, |found| &text[found.end()..]),
    )
}

fn a_fragment_asks_for_a_time(said: &str) -> bool {
    !matches(&TALKING_ABOUT_A_LENGTH, said)
        && (matches(&TRAILING_MEASURE, said) || matches(&MEASURE_THEN_VERB, said))
}

fn the_ask_clause(said: &str) -> &str {
    let parts: Vec<_> = ASK_CLAUSE_SPLIT
        .split(said)
        .flatten()
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();
    if parts.len() >= 2 {
        let last = parts[parts.len() - 1];
        if starts(&ASK_CLAUSE_OPENS, last) {
            return last;
        }
    }
    ""
}

fn names_a_pair(said: &str) -> bool {
    [
        &*SIDE_PAIR_0,
        &*SIDE_PAIR_1,
        &*SIDE_PAIR_2,
        &*SIDE_PAIR_3,
        &*SIDE_PAIR_4,
        &*SIDE_PAIR_5,
        &*SIDE_PAIR_6,
        &*SIDE_PAIR_7,
    ]
    .iter()
    .any(|pattern| matches(pattern, said))
}

fn relative_who(line: &Line) -> bool {
    super::entities::detect(line).into_iter().any(|entity| {
        let next = entity.at + entity.text.split_whitespace().count();
        entity.kind == "person"
            && next > 0
            && line.words.get(next).is_some_and(|word| {
                let word = word.trim_matches(['\'', '’']);
                word.starts_with("who") || word == "that"
            })
    })
}

fn recall(typed: &str) -> Option<Wants> {
    if matches(&RECALL_NOT, typed) {
        return None;
    }
    for verb in RECALL_SAID.find_iter(typed).flatten() {
        let after = strip(&RECALL_PRONOUN, typed[verb.end()..].trim());
        for (pattern, shape) in [
            (&*RECALL_TIME, Time),
            (&*RECALL_AMOUNT, Amount),
            (&*RECALL_PLACE, Place),
            (&*RECALL_PERSON, Person),
        ] {
            if starts(pattern, &after) {
                return Some(shape);
            }
        }
        if let Some(noun) = RECALL_KIND_NOUN_ASK
            .captures(&after)
            .ok()
            .flatten()
            .and_then(|found| found.get(1))
        {
            let word = noun.as_str();
            if let Some((_, shape)) = RECALL_KIND_NOUN.iter().find(|(name, _)| *name == word) {
                return Wants::from_str(shape);
            }
        }
        if let Some(name) = RECALL_SHAPE_BEFORE
            .find(&typed[..verb.start()])
            .ok()
            .flatten()
        {
            let name = name.as_str().trim();
            return Some(if name == "where" {
                Place
            } else if matches!(name, "who" | "what name") {
                Person
            } else if name.starts_with("when")
                || name.starts_with("what time")
                || name.starts_with("what date")
                || name.starts_with("what day")
                || name.starts_with("how long")
                || name.starts_with("how soon")
                || name.starts_with("how often")
                || name.starts_with("how many ")
            {
                Time
            } else {
                Amount
            });
        }
    }
    None
}

fn wants_alone(
    line: &Line,
    kind: Kind,
    said: &str,
    whole: &str,
    asks: &mut Asks,
    asking: bool,
) -> Option<Wants> {
    let typed = if whole.is_empty() { said } else { whole };
    if matches(&OPEN_WHO_HELP, said) {
        return Some(Person);
    }
    if label_help(&line.norm_text) {
        return (!a_help_ask_that_names_nothing(typed)).then_some(Steps);
    }
    let asking = asking
        || matches(&HOW_ASK_FORM, said)
        || matches(&QUESTION_IN_THE_LINE, said) && !matches(&REPORTED, said);
    classify_rules(line, kind, said, whole, asks, asking)
}
pattern!(
    LEADING_VERDICT_AUXILIARY,
    r#"\s*(?:is|are|can|could|will|would|do|does)\b"#
);
pattern!(PRICE_ASK_WORD, r#"\b(?:what|how much)\b"#);
pattern!(DIGIT, r#"[0-9]"#);
pattern!(FIRST_PERSON_METHOD, r#"\bhow (?:do|can|should) i\b"#);
pattern!(
    EXPLICIT_MEANING,
    r#"\b(?:called|mean|means|meaning|stand for|stands for)\b"#
);
pattern!(WHAT_DOES, r#"\bwhat does\b"#);
pattern!(HOW_DO_I, r#"(?i)\bhow do i\b"#);
pattern!(HOW_MUCH_OR_MANY, r#"\bhow (?:much|many)\b"#);
pattern!(
    EXPLICIT_MEANING_BASE,
    r#"\b(?:called|mean|means|meaning|stand for)\b"#
);
pattern!(TIME_WORD, r#"\b(?:time|timeframe)\b"#);
fn classify_rules(
    line: &Line,
    kind: Kind,
    said: &str,
    whole: &str,
    asks: &mut Asks,
    asking: bool,
) -> Option<Wants> {
    let typed = if whole.is_empty() { said } else { whole };
    if may_contain(said, &["how"]) && (starts(&HOW_ARE_YOU, said) || matches(&HOW_IT_WENT, said)) {
        return None;
    }
    if matches(&HOW_IT_HAPPENED, said) {
        return Some(Reason);
    }
    if matches(&HELP_ME, said) {
        return Some(Steps);
    }
    if matches(&HELP_ASK_ANY, typed) {
        return if a_help_ask_that_names_nothing(typed) {
            None
        } else {
            Some(Steps)
        };
    }
    if asking
        && !(matches(&REPORTED, said))
        && matches(&DOING_WRONG, said)
        && !(matches(&WRONG_ASKED_ABOUT, said))
        && !(matches(&WRONG_RECALL, said))
    {
        return Some(Reason);
    }
    if matches(&FIX_ASK, said) && !(starts(&LEADING_VERDICT_AUXILIARY, said)) {
        return Some(Steps);
    }
    if matches(&WHERE_START, said) || matches(&BEST_WAY, said) {
        return Some(Steps);
    }
    if may_contain(said, &[" to "])
        && matches(&NEED_TO, said)
        && !(matches(&NEED_TO_ASKED, said))
        && !(has_other(asks, &["maths", "choice", "write"]))
    {
        return Some(Steps);
    }
    if matches(&WHERE_POINT, said) {
        return None;
    }
    if may_contain(said, &["where"]) && matches(&WHERE_RECALL, said)
        || may_contain(whole, &["where"]) && matches(&WHERE_ALONE, whole)
    {
        return None;
    }
    if matches(&HOW_TO_ANSWER, said) {
        return Some(Steps);
    }
    if matches(&PRICE_IDIOM, said) && asking && matches(&PRICE_ASK_WORD, said) {
        return Some(Amount);
    }
    if matches(&HOW_COME, said) {
        return Some(Reason);
    }
    if asks.contains("steps")
        && matches(&HOW_I, said)
        && !(matches(&COMPARE_MYSELF, said))
        && !((asks.contains("maths") || asks.contains("how_much")) && matches(&DIGIT, said))
    {
        return Some(Steps);
    }
    if matches(&STEPS_ASK, said)
        && !(matches(&COUNTED_STEPS, said) && !(matches(&FIRST_PERSON_METHOD, said)))
    {
        return Some(Steps);
    }
    if may_contain(said, &["what "]) && matches(&MEMBERS_ASK, said) {
        return Some(Options);
    }
    if matches(&KINDS_OF, said) && !(matches(&NOT_KINDS_OF, said)) {
        return Some(Options);
    }
    if (matches(&COMPARISON, said) || matches(&COMPARED, said))
        && !(matches(&NOT_A_COMPARISON, said))
        && names_a_pair(said)
        && (asking || matches(&EXPLICIT_COMPARE, said))
    {
        return Some(Compare);
    }
    if (matches(&ORIGIN_ASK, said) && !(matches(&ORIGIN_PERSON, said))) || matches(&MADE_OF, said) {
        return Some(Meaning);
    }
    if matches(&BEST_OF_MANY, said)
        && !(matches(&COST_ASK, said) && !has_other(asks, COST_STANDS_IN))
    {
        return Some(Options);
    }
    if asks.contains("definition")
        && matches(&WHAT_ARE_MINE, said)
        && !(matches(&EXPLICIT_MEANING, said))
    {
        return Some(Steps);
    }
    if matches(&MY_THING_CALLED, said) {
        return None;
    }
    if asks.contains("definition")
        && matches(&WHAT_DOES, said)
        && !(matches(&ASKS_FOR_MEANING, said))
    {
        asks.remove("definition");
    }
    if asks.contains("definition") && !(asking) && !(matches(&ASKS_FOR_MEANING, said)) {
        asks.remove("definition");
    }
    if matches(&GOOD_MEASURE, said) {
        return Some(Amount);
    }
    if matches(&GOOD_THING, said)
        && asks.contains("definition")
        && !(matches(&GOOD_FIGURE_WORD, said))
    {
        return if matches(&GOOD_WAY, said) {
            Some(Steps)
        } else {
            Some(Options)
        };
    }
    if matches(&HOW_IT_WORKS, said) && !(matches(&HOW_DO_I, said)) {
        return Some(Meaning);
    }
    if asks.contains("who") && !(starts(&WHO_OPENS, said)) && relative_who(line) {
        asks.remove("who");
    }
    if asks.contains("who")
        && (matches(&WHO_AND_MEASURE, said)
            || matches(&WHO_OF_A_PRONOUN, said)
            || (matches(&WHO_OF_A_NOUN, said) && !(starts(&WHO_OPENS, said))))
    {
        asks.remove("who");
    }
    if asks.contains("where") && matches(&WHERE_MONEY_GOES, said) {
        return Some(Steps);
    }
    if asks.contains("where")
        && intersects(asks, &["yesno", "recommend"])
        && matches(&WHERE_RELATIVE, said)
        && !(matches(&ASKS_TO_BE_TOLD_WHERE, said))
    {
        asks.remove("where");
    }
    if matches(&LONG_TIME, said) {
        asks.remove("how_long");
        asks.remove("how_often");
    }
    if may_contain(said, &["how long "]) && matches(&CONVERSION, said)
        || may_contain(said, &["what", "convert"]) && matches(&CONVERTED_FIGURE, said)
    {
        return Some(Amount);
    }
    if matches(&NOT_A_FIGURE, said) {
        return None;
    }
    if intersects(asks, MEASURE_LABELS) && asking {
        if asks.contains("how_long") && matches(&WRITTEN_THING, said) {
            return Some(Amount);
        }
        if asks.contains("how_long") && matches(&THING_THAT_EXTENDS, said) {
            return Some(Amount);
        }
        if asks.contains("how_long")
            && asks.contains("steps")
            && starts(&METHOD_OPENS, &strip(&LEADING_JOIN, said))
        {
            return Some(Steps);
        }
        if asks.contains("how_long")
            && matches(&A_SIZE_LONG, said)
            && !(matches(&A_DURATION_WORD, said))
        {
            if asks.contains("how_much")
                || asks.contains("how_many")
                || matches(&HOW_MUCH_OR_MANY, said)
            {
                asks.remove("how_long");
                return Some(Amount);
            }
            return None;
        }
        return if asks.contains("how_long") || matches(&HOW_MANY_TIME, said) {
            Some(Time)
        } else {
            Some(Amount)
        };
    }
    if matches(&WHAT_DOES_MEASURE, said)
        && !(matches(&ASKS_FOR_MEANING, said))
        && !(matches(&COSTS_NOTHING, said))
    {
        return Some(Amount);
    }
    if matches(&WHAT_IS_PER, said) {
        return Some(Amount);
    }
    if asks.contains("how") {
        if matches(&HOW_SOON, said) {
            return Some(Time);
        }
        if matches(&HOW_MEASURE, said) {
            return Some(Amount);
        }
        if starts(&HOW_MEASURE_HEAD, &strip(&LEADING_JOIN, said)) {
            return Some(Amount);
        }
        if matches(&HOW_TO_DO, said) {
            asks.remove("definition");
        }
    }
    if a_fragment_asks_for_a_time(said) {
        return Some(Time);
    }
    let asked = strip(&LEADING_JOIN, said);
    let focus_said = if line.asking.is_empty() {
        &line.focus
    } else {
        &line.asking
    }
    .join(" ");
    if (starts(&WHEN_ASKS, &asked) || starts(&WHEN_ASKS, &strip(&LEADING_JOIN, focus_said.trim())))
        && !(matches(&WHICH_DAY_IS_BEST, said))
    {
        return Some(Time);
    }
    let ask_clause = if asking { the_ask_clause(said) } else { "" };
    if !ask_clause.is_empty()
        && starts(&WHEN_ASKS, &strip(&LEADING_JOIN, ask_clause))
        && !matches(&WHICH_DAY_IS_BEST, said)
    {
        return Some(Time);
    }
    if may_contain(said, &["what"])
        && starts(&WHAT_ARE_TIMES, said)
        && !(matches(&REPORTED, said))
        && !(matches(&EXPLICIT_MEANING_BASE, said))
    {
        return Some(Time);
    }
    if matches(&A_GOOD_TIME, said) {
        return Some(Yesno);
    }
    if matches(&GOOD_FIGURE, said) && !(starts(&VERDICT_FRAME, said)) {
        return if matches(&TIME_WORD, said) {
            Some(Time)
        } else {
            Some(Amount)
        };
    }
    if matches(&JUDGED_MEASURE, said)
        && asking
        && !(matches(&ON_THE_CLOCK, said))
        && !(matches(&REPORTED, said))
        && !(starts(&ASKS_YES_OR_NO, &strip(&LEADING_JOIN, said)))
    {
        return Some(Amount);
    }
    if matches(&GOOD_ONE, said) {
        return Some(Options);
    }
    if asking
        && !matches(&GIVE_ME_NOT_A_THING, said)
        && !matches(&GIVE_ME_MADE_UP, said)
        && !matches(&GIVE_ME_AN_IDIOM, said)
        && !asks.contains("list")
    {
        for (pattern, want) in [(&*GIVE_ME_A_FEW, Options), (&*GIVE_ME_ONE, Pick)] {
            if let Some(found) = pattern.find(said).ok().flatten()
                && said[..found.start()]
                    .split_whitespace()
                    .next_back()
                    .is_none_or(|word| GIVE_ME_LEAD_IN.contains(&word))
            {
                return Some(want);
            }
        }
    }
    if matches(&A_THING_TO_GIVE, said)
        && matches(&GIVING_FRAME, said)
        && !asks.contains("yesno")
        && !(matches(&NOT_A_GIVING_THING, said))
    {
        return Some(Options);
    }
    if matches(&WHAT_FIGURE, said) || matches(&WHAT_AGE, said) {
        return Some(Amount);
    }
    if matches(&NEED_TO_DO, said) {
        return Some(Steps);
    }
    if matches(&WHAT_HELPS, said) || matches(&WHAT_YOU_TAKE, said) || matches(&WHAT_I_NEED, said) {
        return Some(Options);
    }
    if matches(&WHAT_YOU_DO, said) {
        return Some(Steps);
    }
    if !(matches(&REPORTED, said)) && !(starts(&OTHER_QUESTION, said)) {
        if matches(&WHAT_I_WRITE, said) {
            return Some(Steps);
        }
        if matches(&WHAT_I_GROW, said) {
            return Some(Options);
        }
    }
    if matches(&WHAT_TO_BAKE, said) {
        return Some(Options);
    }
    if matches(&WHAT_THING_FRAGMENT, said) && !(matches(&SUBSTITUTE, said)) {
        return Some(Pick);
    }
    if matches(&DO_WITH_A_PERSON, said) {
        return None;
    }
    if asking && (matches(&WHAT_CAN_I_DO_WITH, said) || matches(&WHAT_SHOULD_I_DO_WITH, said)) {
        if intersects(asks, PICK_SIDES) {
            return Some(Pick);
        }
        if matches(&WHAT_SHOULD_I_DO_WITH, said) {
            return Some(Steps);
        }
        return Some(Options);
    }
    if matches(&WHAT_ELSE_WITH, said) {
        return Some(Options);
    }
    if matches(&WHAT_ELSE_STEPS, said) {
        return Some(Steps);
    }
    if may_contain(said, &["what "])
        && (matches(&PICK_ONE, said) || matches(&PICK_ONE_OF_A_KIND, said))
        || (matches(&WHAT_TO_OPENS, said) && matches(&WHAT_TO_PICK, said))
    {
        return if starts(&HAVE_RECALL, said) {
            None
        } else {
            Some(Pick)
        };
    }
    if matches(&SUBSTITUTE, said) {
        return Some(Options);
    }
    if may_contain(said, &["what "]) && matches(&PAIRING_ASK, said) && !(matches(&REPORTED, said)) {
        return Some(Options);
    }
    if (matches(&WHAT_TO_DO, said)
        || (matches(&WHAT_TO_OPENS, said) && matches(&WHAT_TO_STEPS, said)))
        && !(matches(&REPORTED, said))
        && !(matches(&NOT_STEPS, said))
        && !(starts(&OTHER_QUESTION, said))
    {
        return Some(Steps);
    }
    if asking && !(matches(&REPORTED, said)) && matches(&REASON_ASK, said) {
        return Some(Reason);
    }
    if !matches(&ASKS_ITS_OWN_YES_NO, said)
        && let Some(found) = POINT_ASK.captures(said).ok().flatten()
        && let Some(first) = found
            .get(1)
            .or_else(|| found.get(2))
            .and_then(|group| group.as_str().split_whitespace().next())
        && !POINT_NOT_A_THING.contains(&first)
    {
        return Some(Reason);
    }
    if !has_other(asks, &[&["recommend"][..], MEASURE_LABELS].concat())
        && !(said.trim_end().ends_with("?"))
        && !(starts(&ASKS_OUT_LOUD, said))
        && !(matches(&TAIL_VERDICT_ASK, whole))
    {
        return None;
    }
    if matches(&DIFFERENCE, said) {
        return Some(Meaning);
    }
    if intersects(asks, PICK_SIDES) {
        return Some(Pick);
    }
    if matches(&STEPS_ARE_COUNTED, said) || matches(&STEPS_AS_A_DISTANCE, said) {
        asks.remove("steps");
        if matches(&STEPS_AS_A_DISTANCE, said) {
            return Some(Amount);
        }
    }
    if matches(&PRICE_ADJECTIVE, said)
        && !(matches(&PRICE_ADJECTIVE_KEEPS, said))
        && !(matches(&PRICE_ADJECTIVE_THE_ASK_IS_LATER, &fold(&line.text)))
        && !(intersects(asks, PRICE_ADJECTIVE_WHY))
    {
        return Some(Amount);
    }
    if matches(&PRICE_ADJECTIVE, said) && intersects(asks, PRICE_ADJECTIVE_WHY) {
        return Some(Reason);
    }
    if matches(&A_PRICE_ASK, said) && !(matches(&NOT_A_PRICE_ASK, said)) {
        return Some(Amount);
    }
    if matches(&RECALL_OF_MINE, said) {
        asks.remove("yesno");
    }
    if asks.contains("yesno") && !(intersects(asks, SPECIFIC_ASKS)) {
        return Some(Yesno);
    }
    if matches(&COST_ASK, said) && !has_other(asks, COST_STANDS_IN) {
        return Some(Amount);
    }
    if matches(&A_FIGURE_OF_A_MONEY_VERB, said) && !(matches(&MONEY_VERB_NOT_A_PRICE, said)) {
        return Some(Amount);
    }
    if matches(&WHAT_CAN_I_DO, said) && !(intersects(asks, PICK_SIDES)) {
        return Some(Steps);
    }
    for (shape, labels) in ASKED_FOR {
        if labels
            .iter()
            .any(|label| asks.contains(label) && (asking || !MEASURE_LABELS.contains(label)))
        {
            if shape == Pick && labels.contains(&"recommend") && matches(&A_FEW_THINGS, said) {
                return Some(Options);
            }
            if shape == Amount
                && asks.contains("maths")
                && !asks.contains("how_much")
                && kind != Kind::Question
                && !matches(&SUM_OUT_LOUD, said)
                && !matches(&SUM_ASKS, said)
            {
                continue;
            }
            return Some(shape);
        }
    }
    if matches(&TAIL_VERDICT_ASK, whole) {
        return Some(Yesno);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{Kind, Wants, asks_only_how_long, may_contain, wants};
    use crate::bot::understanding::line::normalise;

    #[test]
    fn literal_gates_preserve_potential_unicode_and_uppercase_matches() {
        assert!(!may_contain("hi there", &["what ", "where"]));
        assert!(may_contain("remind me where i was", &["where"]));
        assert!(may_contain("WHAT should i buy", &["what "]));
        assert!(may_contain("whatſ next", &["whats"]));
    }

    #[test]
    fn inherits_a_shape_only_when_the_follow_up_has_no_shape_of_its_own() {
        let before = wants(&normalise("how long does rice take?"), Kind::Question, None);
        assert_eq!(before, Some(Wants::Time));
        assert_eq!(
            wants(&normalise("and for brown rice?"), Kind::Question, before),
            Some(Wants::Time)
        );
        assert_eq!(
            wants(&normalise("and can i freeze it?"), Kind::Question, before),
            Some(Wants::Yesno)
        );
        assert_eq!(
            wants(&normalise("thanks that helps"), Kind::Thanks, before),
            None
        );
    }

    #[test]
    fn the_ask_after_the_context_supplies_the_answer_shape() {
        assert_eq!(
            wants(
                &normalise("camping, might rain, what to pack"),
                Kind::Question,
                None
            ),
            Some(Wants::Pick)
        );
        assert_eq!(
            wants(
                &normalise("what do i look for when buying a mattress"),
                Kind::Question,
                None
            ),
            Some(Wants::Steps)
        );
    }

    #[test]
    fn a_money_verb_distinguishes_an_amount_from_a_way_to_spend_it() {
        assert_eq!(
            wants(
                &normalise("what should i tip my hairdresser"),
                Kind::Question,
                None
            ),
            Some(Wants::Amount)
        );
        assert_eq!(
            wants(
                &normalise("what should i spend it on"),
                Kind::Question,
                None
            ),
            Some(Wants::Pick)
        );
    }

    #[test]
    fn recipe_duration_only_uses_the_label_set() {
        assert!(asks_only_how_long(
            &normalise("how long to cook rice"),
            Kind::Question
        ));
        assert!(!asks_only_how_long(
            &normalise("how long can i keep it?"),
            Kind::Question
        ));
    }
}
