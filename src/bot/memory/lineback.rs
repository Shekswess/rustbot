//! Replies about earlier details from PyBot's memory/lineback.py.
use super::lineback_kinds;
use crate::bot::{Memory, Reading};
use fancy_regex::Regex as ContextRegex;
use regex::Regex;
use std::{collections::BTreeSet, sync::LazyLock};

macro_rules! rule {
    ($name:ident, $pattern:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
    };
}

const ASK_WORDS: &str = "what which who whom whose when where why how remind me again say said saying says tell told telling mention mentioned i you my our we us it its that this these those was were is are be been being did do does done about of to the a an and or for with in on at by as if so but not any some please just really thing things one ones kind sort name called exact exactly already";
const VAGUE: &str = "go going went get got getting do doing done make making take taking put thing things way bit lot time now then well good bad sure really quite maybe first last next other another same back again more most much many little big small old new long short like want wanted need";
const KIND_WORDS: &str =
    "size|time|date|day|birthday|price|cost|number|weight|paid|pay|spend|spent|height|dose";

rule!(WORD, r"[a-z']+");
rule!(
    ASKS_BACK,
    concat!(
        r"\b(?:what|which|who|when|where|how\s+\w+)\b[^?!]{0,40}?\bdid\s+i\s+(?:say|said|tell\s+you|told\s+you|mention|mentioned)\b",
        r"|\b(?:what|which)\s+(?:[a-z']+\s+){1,3}?(?:was|is|were)\s+it\b",
        r"|\bwhat(?:'s|\s+is|\s+was)\s+my\s+(?:[a-z']+\s+){1,3}?again\b",
        r"|\bremind\s+me\s+(?:what|which|when|where|who|how\s+many|how\s+much)\b",
        r"|\bwhat\s+did\s+i\s+say\b|\bwhat\s+(?:was|were|am|is)\s+(?:i|we)\s+going\s+to\s+\w+",
        r"|\bwhat\s+(?:was|is)\s+it\s+(?:he|she|they|we|i|you)\b",
        r"|\bhow\s+(?:much|many)\b[^?!]{0,25}?\bdid\s+i\s+(?:say|said|need|want|ask)\b"
    )
);
rule!(
    ASKS_KIND,
    &format!(
        concat!(
            r"\b(?:what|which)\s+(?:{kinds})\b[^?!]{{0,22}}?\bagain\b",
            r"|\bthe\s+(?:{kinds})\s+(?:(?:that\s+)?(?:i|we)\s+(?:said|gave|told\s+you|mentioned)|again)\b",
            r"|\b(?:did|have)\s+(?:i|we)\b[^?!]{{0,30}}?\b(?:{kinds})\b|\bhow\s+(?:much|many)\b[^?!]{{0,25}}?\bagain\b"
        ),
        kinds = KIND_WORDS
    )
);
rule!(
    TELLING,
    r"^\s*(?:was|is|were|are|should\s+be|came\s+in|goes|needs|will\s+be)\b"
);
rule!(PAST_ACT, r"\b(?:did|have)\s+(?:i|we)\b");
rule!(
    QUESTION,
    r"(?i)^\s*(?:what|which|who|when|where|why|how|should|shall|can|could|would|will|do|does|did|is|are|am|have|has|if|any)\b|\?\s*$"
);
rule!(
    INTENTION,
    r"\b(?:was|were|am|is|going)\s+(?:i\s+)?(?:going\s+)?to\s+(?:ask|say|do|tell|mention)\b|\bwhat\s+(?:i\s+)?was\s+going\s+to\b"
);
rule!(
    ATTRIBUTE,
    r"^(?:age|ages|old|size|name|price|cost|weight|height|colour|color|birthday|date|time|number|address|surname|title)$"
);
rule!(
    NOT_DETAIL,
    r"\bhow\s+(?:old|many|much|long|often|far|heavy|tall)\b|\bwhat\s+time\b|\bwhat\s+(?:day|date)\b|\bwhen\b"
);
rule!(
    MY_VALUE,
    r"(?i)\bmy\s+(?:[a-z']+\s+){0,3}?(?:is|was)\s+([^.;,?]+?)\s*$"
);
rule!(
    VALUE_START,
    r"(?i)^(?:[0-9]|a\b|an\b|the\b|on\b|in\b|at\b|for\b|about\b)"
);
rule!(
    VALUE_VERB,
    r"(?i)^(?:quoted|paid|charged|cost|costs|size|in\s+(?:a|an)|at|from)\s+"
);
rule!(
    CALENDAR,
    r"^\W*(?:what|which)\s+(?:day|date|time)\s+is\s+it\s*\??$"
);
rule!(
    AGAIN,
    r"\b(?:again|before|earlier|did\s+(?:i|we)|have\s+i|you\s+know|tell\s+me|remind)"
);
rule!(
    FIGURE,
    r"(?i)[£$€]\s?\d[\d.,]*|\b\d[\d.,]*\s?(?:%|g|kg|grams?|kilos?|ml|cl|litres?|liters?|pints?|oz|lbs?|tbsp|tsp|cups?|mugs?|slices?|sheets?|hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?|miles?|km|metres?|meters?|pounds?|quid|dollars?|euros?|degrees?)\b"
);
rule!(AMOUNT, r"\bhow\s+(?:much|many)\b");
rule!(MUCH, r"\bhow\s+much\b");

static SENTENCE_END: LazyLock<ContextRegex> =
    LazyLock::new(|| ContextRegex::new(r"(?<=[.!?;])\s+|,\s+(?=and\b|but\b)").unwrap());

#[derive(Clone, Copy)]
enum Kind {
    Size,
    Time,
    Day,
    Date,
    Price,
    Weight,
    Height,
    Dose,
}

impl Kind {
    fn word(word: &str) -> Option<Self> {
        Some(match word {
            "size" => Self::Size,
            "time" => Self::Time,
            "day" => Self::Day,
            "date" | "birthday" => Self::Date,
            "price" | "cost" | "number" | "paid" | "pay" | "spend" | "spent" => Self::Price,
            "weight" => Self::Weight,
            "height" => Self::Height,
            "dose" => Self::Dose,
            _ => return None,
        })
    }
    fn pattern(self) -> &'static Regex {
        rule!(
            SIZE,
            r"(?i)\b(?:in\s+(?:a|an)|size)\s+(xs|s|m|l|xl|xxl|small|medium|large|extra large)\b|\b(?:was|is|its|it's|make\s+it|actually|instead)\s+(?:a\s+)?(xs|xl|xxl|small|medium|large|extra\s+large)\b"
        );
        rule!(
            TIME,
            r"(?i)\b(?:at|from)\s+(\d{1,2}(?:[:.]\d{2})?\s?(?:am|pm)?|half\s+\w+|midday|noon|midnight)\b|\b(\d{1,2}(?:[:.]\d{2})?\s?(?:am|pm))\b|\b(half\s+(?:past\s+)?(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve))\b|\b(midday|noon|midnight)\b"
        );
        rule!(
            DAY,
            r"(?i)\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|tonight)\b"
        );
        rule!(
            DATE,
            r"(?i)\b(?:the\s+)?((?:0?[1-9]|[12][0-9]|3[01])(?:st|nd|rd|th))\b|\b(?:in|on)\s+(january|february|march|april|may|june|july|august|september|october|november|december)\b"
        );
        rule!(
            PRICE,
            r"(?i)(?:£|\$|€)\s?\d[\d,]*(?:\.\d+)?|\b\d[\d,]*(?:\.\d+)?\s?(?:quid|pounds|euros|dollars|grand)\b|\b(?:quoted|paid|charged|cost)\s+(?:me\s+)?(?:£|\$|€)?\s?\d[\d,]*"
        );
        rule!(
            WEIGHT,
            r"(?i)\b\d+(?:\.\d+)?\s?(?:kg|kilos?|stone|st|lbs?|pounds?)\b"
        );
        rule!(
            HEIGHT,
            r"(?i)\b\d+(?:\.\d+)?\s?(?:cm|ft|feet|foot|inches|in)\b"
        );
        rule!(DOSE, r"(?i)\b\d+(?:\.\d+)?\s?(?:mg|ml|mcg|micrograms?|g)\b");
        match self {
            Self::Size => &SIZE,
            Self::Time => &TIME,
            Self::Day => &DAY,
            Self::Date => &DATE,
            Self::Price => &PRICE,
            Self::Weight => &WEIGHT,
            Self::Height => &HEIGHT,
            Self::Dose => &DOSE,
        }
    }
}

fn flat(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn words(low: &str) -> impl Iterator<Item = &str> {
    WORD.find_iter(low)
        .map(|word| word.as_str().trim_matches('\''))
}

fn stem(word: &str) -> &str {
    ["ing", "ies", "es", "ed", "s"]
        .into_iter()
        .find_map(|suffix| word.strip_suffix(suffix).filter(|root| root.len() > 3))
        .unwrap_or(word)
}

fn about(line: &str, loose: bool) -> BTreeSet<String> {
    let low = line.to_ascii_lowercase();
    words(&low)
        .filter(|word| {
            !ASK_WORDS.split_whitespace().any(|stop| stop == *word)
                && if loose {
                    word.len() >= 3
                } else {
                    !VAGUE.split_whitespace().any(|stop| stop == *word)
                }
        })
        .map(|word| stem(word).to_owned())
        .collect()
}

fn recall_ask(low: &str) -> bool {
    ASKS_BACK.is_match(low)
        || ASKS_KIND
            .find(low)
            .is_some_and(|found| !TELLING.is_match(&low[found.end()..]))
        || (PAST_ACT.is_match(low)
            && (words(low).any(|word| Kind::word(word).is_some()) || MUCH.is_match(low)))
}

fn kind_asked(low: &str, about: &BTreeSet<String>) -> Option<Kind> {
    if CALENDAR.is_match(low) && !AGAIN.is_match(low) {
        return None;
    }
    let words: BTreeSet<_> = words(low)
        .filter(|word| Kind::word(word).is_some())
        .collect();
    if words.is_empty() {
        return (MUCH.is_match(low) && about.is_empty()).then_some(Kind::Price);
    }
    let keys: BTreeSet<_> = words.iter().map(|word| stem(word).to_owned()).collect();
    if about.is_subset(&keys) {
        Kind::word(words.first()?)
    } else {
        None
    }
}

fn value_of_kind(kind: Kind, lines: &[&str]) -> Option<String> {
    lines.iter().rev().find_map(|line| {
        let captures = kind.pattern().captures(line)?;
        let value = captures
            .iter()
            .skip(1)
            .flatten()
            .next()
            .or_else(|| captures.get(0))?
            .as_str();
        let value = VALUE_VERB.replace(value.trim_matches([' ', '.', ',', ';']), "");
        let value = value.trim_matches([' ', '.', ',', ';']);
        (alphanumeric(value) && value.split_whitespace().count() <= 4)
            .then(|| format!("You said {value}."))
    })
}

fn alphanumeric(line: &str) -> bool {
    line.bytes()
        .any(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
}

fn best_line<'a>(wanted: &BTreeSet<String>, lines: &[&'a str], questions: bool) -> Option<&'a str> {
    lines
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, line)| {
            !recall_ask(&line.to_ascii_lowercase()) && (questions || !QUESTION.is_match(line))
        })
        .filter_map(|(place, line)| {
            let actual = about(line, true);
            let mut shared = wanted.intersection(&actual);
            let longest = shared.clone().map(String::len).max()?;
            (longest >= 3).then(|| ((shared.by_ref().count(), longest, place), line))
        })
        .max_by_key(|(score, _)| *score)
        .map(|(_, line)| line)
}

fn quote_of(line: &str, wanted: &BTreeSet<String>) -> String {
    if line.chars().count() > 120 {
        let mut start = 0;
        for split in SENTENCE_END.find_iter(line).filter_map(Result::ok) {
            let part = &line[start..split.start()];
            if !wanted.is_disjoint(&about(part, true)) {
                return part.trim_matches([' ', ',', '.', ';']).to_owned();
            }
            start = split.end();
        }
        let part = &line[start..];
        if !wanted.is_disjoint(&about(part, true)) {
            return part.trim_matches([' ', ',', '.', ';']).to_owned();
        }
    }
    line.to_owned()
}

fn reply_of_line(
    low: &str,
    wanted: &BTreeSet<String>,
    lines: &[&str],
    memory: &Memory,
) -> Option<String> {
    if wanted.is_empty() || NOT_DETAIL.is_match(low) {
        return None;
    }
    let line = best_line(wanted, lines, INTENTION.is_match(low))?;
    let named: BTreeSet<_> = words(low)
        .filter(|word| ATTRIBUTE.is_match(word))
        .map(|word| stem(word).to_owned())
        .collect();
    let earlier = about(line, true);
    if !named.is_empty() && named.is_disjoint(&earlier) {
        return None;
    }
    let last = memory
        .turns
        .iter()
        .rev()
        .find(|turn| !turn.bot.trim().is_empty());
    if last.is_some_and(|turn| {
        let answered = about(&turn.bot, true);
        wanted
            .intersection(&earlier)
            .any(|word| answered.contains(word))
    }) {
        return None;
    }
    if let Some(value) = MY_VALUE
        .captures(line)
        .and_then(|found| found.get(1))
        .map(|value| value.as_str().trim())
        && alphanumeric(value)
        && value.split_whitespace().count() <= 6
        && VALUE_START.is_match(value)
    {
        return Some(format!("You said {}.", value.trim_end_matches([' ', '.'])));
    }
    Some(format!("You said: \"{}\"", quote_of(line, wanted)))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = flat(reading.raw);
    if said.is_empty() {
        return None;
    }
    let low = said.to_ascii_lowercase();
    let all: Vec<_> = memory
        .turns
        .iter()
        .map(|turn| flat(&turn.user))
        .filter(|line| !line.is_empty())
        .collect();
    let mut lines: Vec<_> = all.iter().map(String::as_str).collect();
    if lines.len() > 1 && lines.last() == Some(&said.as_str()) {
        lines.pop();
    }
    if recall_ask(&low) {
        let named = about(&low, false);
        if let Some(value) = kind_asked(&low, &named).and_then(|kind| value_of_kind(kind, &lines)) {
            return Some(value);
        }
        if AMOUNT.is_match(&low) && !named.is_empty() {
            for line in lines
                .iter()
                .rev()
                .filter(|line| !named.is_disjoint(&about(line, true)))
            {
                if let Some(figure) = FIGURE.find(line) {
                    return Some(format!(
                        "You said {}.",
                        figure.as_str().trim_matches([' ', '.', ','])
                    ));
                }
            }
        }
        if let Some(reply) = reply_of_line(&low, &named, &lines, memory) {
            return Some(reply);
        }
    }
    if CALENDAR.is_match(&low) && !AGAIN.is_match(&low) {
        return None;
    }
    let found = lineback_kinds::find(&said, &lines)?;
    Some(format!("You said: \"{found}\""))
}
