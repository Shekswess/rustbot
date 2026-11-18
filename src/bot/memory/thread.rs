//! Read the subject, most recent question, and most recent telling from the chat.

use std::sync::LazyLock;

use regex::{Captures, Regex};

use crate::bot::{
    Memory, Reading,
    understanding::{
        entities::Entity,
        kinds::{Intent, Kind},
    },
};

static CLOSING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:right |ok |okay |well )?(?:good ?night|night|bye|see (?:you|ya|you later)|i(?:'m|m| am) off (?:to bed|now))\b").unwrap()
});
static DETAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:its|it's|it is|thats|that's|that is|theyre|they're|they are|hes|he's|she's|shes)\b",
    )
    .unwrap()
});
static OWN_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:i'm|im|i've|ive|i'll|i'd|i am|i was|we're|we've|we'll|we'd|we are|we were|i|we|my|our|me|us|mine|ours|myself|ourselves)\b").unwrap()
});
static SHORT_AUXILIARY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:(?P<wh>what|why|when|where|which(?: \S+)?|who|how(?: (?:long|much|many|far|old|often))?) )?(?P<aux>can|could|should|would|will|shall|do|does|did|am|is|are|was|were|have|has|had) (?P<subject>i|we|you|he|she|it|they|this|that)(?: (?P<tail>.+))?$").unwrap()
});
static QUESTION_WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:what|why|when|where|which|who|how)\b").unwrap());

pub(in crate::bot) struct Recap {
    pub(in crate::bot) subject: Option<String>,
    pub(in crate::bot) asked: Option<String>,
    pub(in crate::bot) said: Option<String>,
}

fn clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    reading
        .line
        .clean
        .trim_matches([' ', '.', '!', '?', ',', ';', ':'])
}

fn strip_lead(mut text: &str) -> &str {
    while let Some((first, rest)) = text.split_once(' ') {
        if !matches!(
            first.to_ascii_lowercase().as_str(),
            "and" | "so" | "well" | "okay" | "ok" | "right"
        ) {
            break;
        }
        text = rest.trim_start();
    }
    text
}

fn own_words(text: &str) -> String {
    OWN_WORDS
        .replace_all(text, |m: &Captures<'_>| {
            match m[0].to_ascii_lowercase().as_str() {
                "i'm" | "im" | "we're" => "you're",
                "i've" | "ive" | "we've" => "you've",
                "i'll" | "we'll" => "you'll",
                "i'd" | "we'd" => "you'd",
                "i am" | "we are" => "you are",
                "i was" | "we were" => "you were",
                "my" | "our" => "your",
                "mine" | "ours" => "yours",
                "myself" | "ourselves" => "yourself",
                _ => "you",
            }
        })
        .into_owned()
}

fn embedded_ask(text: &str) -> Option<String> {
    let text = strip_lead(text);
    if let Some(parts) = SHORT_AUXILIARY.captures(text) {
        let question = parts.name("wh").map_or("whether", |part| part.as_str());
        let subject = &parts["subject"];
        let person = matches!(subject.to_ascii_lowercase().as_str(), "i" | "we");
        let auxiliary = match &parts["aux"].to_ascii_lowercase()[..] {
            "am" if person => "are",
            "was" if person => "were",
            "do" => "",
            _ => &parts["aux"],
        };
        let mut words = Vec::with_capacity(4);
        words.extend([question, subject]);
        if !auxiliary.is_empty() {
            words.push(auxiliary);
        }
        if let Some(tail) = parts.name("tail") {
            words.push(tail.as_str());
        }
        return Some(own_words(&words.join(" ")));
    }
    QUESTION_WORD.is_match(text).then(|| own_words(text))
}

fn entity_subject(reading: &Reading<'_>) -> Option<String> {
    let allowed = |entity: &&Entity| {
        if !matches!(entity.kind, "thing" | "person") {
            return true;
        }
        entity
            .at
            .checked_sub(1)
            .and_then(|index| reading.line.norm.get(index))
            .is_none_or(|word| {
                !matches!(
                    word.as_str(),
                    "for" | "with" | "about" | "from" | "at" | "by"
                )
            })
    };
    let entity = ["place", "person", "name", "thing"]
        .iter()
        .find_map(|kind| {
            reading
                .entities
                .iter()
                .rev()
                .filter(allowed)
                .find(|entity| entity.kind == *kind)
        })?;
    let name = &entity.text;
    if name.is_empty() {
        return None;
    }
    let lower = name.to_ascii_lowercase();
    if matches!(entity.kind, "place" | "name") {
        let raw = reading.raw;
        let lower_raw = raw.to_ascii_lowercase();
        return Some(
            lower_raw
                .find(&lower)
                .map_or_else(|| name.clone(), |at| raw[at..at + name.len()].to_owned()),
        );
    }
    if let Some(name) = lower.strip_prefix("my ") {
        return Some(format!("your {name}"));
    }
    if ["a ", "an ", "the ", "your "]
        .iter()
        .any(|prefix| lower.starts_with(prefix))
    {
        return Some(name.clone());
    }
    let qualifier = entity
        .at
        .checked_sub(1)
        .and_then(|at| reading.line.norm.get(at))
        .filter(|word| {
            matches!(
                word.as_str(),
                "birthday" | "wedding" | "job" | "bathroom" | "kitchen" | "spare"
            )
        });
    Some(match qualifier {
        Some(word) => format!("the {word} {name}"),
        None => format!("the {name}"),
    })
}

fn explicit_subject(reading: &Reading<'_>) -> Option<String> {
    const DETERMINERS: &[&str] = &["a", "an", "the", "my", "our", "this", "that", "your"];
    const WEAK: &[&str] = &[
        "day", "time", "thing", "things", "bit", "lot", "way", "one", "question", "chat", "plan",
        "stuff", "good", "bad", "certain", "same", "last", "week",
    ];
    reading.line.words.windows(2).rev().find_map(|pair| {
        let (head, noun) = (pair[0].as_str(), pair[1].as_str());
        (DETERMINERS.contains(&head)
            && !WEAK.contains(&noun)
            && noun
                .bytes()
                .all(|byte| byte.is_ascii_alphabetic() || byte == b'-')
            && !noun.ends_with("ing"))
        .then(|| {
            format!(
                "{} {noun}",
                if matches!(head, "my" | "our" | "your") {
                    "your"
                } else {
                    "the"
                }
            )
        })
    })
}

pub(in crate::bot) fn recap(memory: &Memory, skip: impl Fn(&str) -> bool) -> Recap {
    let mut said = None;
    let mut asked = None;
    let mut named = None;
    let mut scoped = None;
    let mut passed_closing = false;
    for (index, turn) in memory.turns.iter().enumerate().rev() {
        let reading = Reading::new(&turn.user);
        let line = clean(&reading);
        if super::again::contentless(line)
            || super::again::about_the_chat(line)
            || skip(line)
            || reading.features.about_bot
        {
            continue;
        }
        if reading.features.intent == Intent::Farewell || CLOSING.is_match(line) {
            passed_closing = true;
            continue;
        }
        let question = reading.kind == Kind::Question || reading.line.signals.question_mark;
        if question && asked.is_none() {
            asked = embedded_ask(line).map(|text| (index, text));
        }
        if !question && said.is_none() && !DETAIL.is_match(line) {
            said = Some(own_words(strip_lead(line)));
        }
        if named.is_none() {
            named = entity_subject(&reading).map(|subject| (index, subject));
        }
        if scoped.is_none()
            && !question
            && reading.features.intent != Intent::Feeling
            && !DETAIL.is_match(line)
        {
            scoped = explicit_subject(&reading).map(|subject| (index, subject));
        }
        if named.is_some() && asked.is_some() && said.is_some() {
            break;
        }
    }
    let topic = named.or(scoped);
    Recap {
        asked: asked
            .filter(|(index, _)| topic.as_ref().is_none_or(|(since, _)| index >= since))
            .map(|(_, ask)| ask),
        said: if passed_closing && topic.is_some() {
            None
        } else {
            said
        },
        subject: topic.map(|(_, subject)| subject),
    }
}

pub(in crate::bot) fn choose_recap(options: &[&str], memory: &Memory) -> String {
    super::again::vary(options, memory)
}
