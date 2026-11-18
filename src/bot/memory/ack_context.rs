//! Context for acknowledging newly supplied details about an earlier subject.
use super::{CONTENT_WORD, STATE, TROUBLE, has, strip_lead};
use crate::bot::{Memory, Reading, compose, modules, understanding::kinds::Kind};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! rule {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Fancy> =
            LazyLock::new(|| Fancy::new($pattern).expect(stringify!($name)));
    };
}

include!("ack_context_data.rs");
static TOPIC: LazyLock<Regex> = LazyLock::new(|| Regex::new(TOPIC_WORD).unwrap());
fn matches(rule: &Fancy, text: &str) -> bool {
    rule.is_match(text).unwrap_or(false)
}
fn topic_words(line: &str) -> Vec<String> {
    TOPIC
        .find_iter(&line.to_lowercase().replace('’', "'"))
        .map(|m| m.as_str().trim_end_matches("'s").replace('\'', ""))
        .filter(|w| !w.is_empty())
        .collect()
}

// Keep the raw conversation separate from the subject as corrected.
struct Topic {
    turn: usize,
    corrected: Option<String>,
}

impl Topic {
    fn text<'a>(&'a self, turns: &'a [crate::bot::Exchange]) -> Option<&'a str> {
        self.corrected
            .as_deref()
            .or_else(|| turns.get(self.turn).map(|turn| turn.user.as_str()))
    }
}

#[derive(Default)]
pub(crate) struct Topics {
    previous: Option<Topic>,
    current: Option<Topic>,
    current_topic: Option<&'static str>,
}
impl Topics {
    pub(crate) fn has_earlier_topic(&self) -> bool {
        self.previous.is_some()
    }

    pub(crate) fn note(&mut self, read: &Reading<'_>, prior: &[crate::bot::Exchange]) {
        if matches!(read.kind, Kind::Empty | Kind::Greeting | Kind::Thanks)
            || read.topics.iter().any(|(s, _)| *s == "about-the-bot")
        {
            return;
        }
        let words = topic_words(read.raw);
        if words.is_empty()
            || has(ACKS, &words.join(" "))
            || (words.len() <= 4
                && words.iter().any(|w| has(ACKS, w))
                && words
                    .iter()
                    .all(|w| w.len() <= 3 || has(ACKS, w) || has(STOPWORDS, w)))
        {
            return;
        }
        if words.len() < 4 && read.topics.is_empty() {
            let mut keys = words.iter().filter(|w| {
                w.len() >= 3 && !has(STOPWORDS, w) && !w.bytes().all(|b| b.is_ascii_digit())
            });
            let first = keys.next();
            if !keys.any(|w| Some(w) != first) {
                return;
            }
        }
        if self
            .current
            .as_ref()
            .and_then(|topic| topic.text(prior))
            .is_some_and(|text| topic_words(text) == words)
        {
            return;
        }
        self.previous = self.current.take();
        self.current = Some(Topic {
            turn: prior.len(),
            corrected: None,
        });
        self.current_topic = read.own_topic.filter(|item| item.1 >= 2).map(|item| item.0);
    }

    pub(in crate::bot) fn correct(&mut self, text: String) {
        if let Some(topic) = &mut self.current {
            topic.corrected = Some(text);
        }
    }

    pub(crate) fn stored_topic<'a>(
        &'a self,
        turns: &'a [crate::bot::Exchange],
    ) -> impl Iterator<Item = &'a str> {
        self.active(turns)
            .map(|(_, text)| text)
            .into_iter()
            .chain(self.current_topic)
    }

    pub(in crate::bot::memory) fn active<'a>(
        &'a self,
        turns: &'a [crate::bot::Exchange],
    ) -> Option<(usize, &'a str)> {
        let topic = self.current.as_ref()?;
        Some((topic.turn, topic.text(turns)?))
    }

    fn prior<'a>(&'a self, clean: &str, turns: &'a [crate::bot::Exchange]) -> Option<&'a str> {
        let last = self.current.as_ref()?.text(turns)?;
        if topic_words(last) == topic_words(clean) {
            return self
                .previous
                .as_ref()
                .and_then(|topic| topic.text(turns))
                .or(Some(last));
        }
        Some(last)
    }
}

pub(super) fn heavy(memory: &Memory) -> bool {
    memory.turns.iter().rev().take(3).any(|t| {
        let line = t.user.split_whitespace().collect::<Vec<_>>().join(" ");
        compose::loss_reader::reads_as_loss(&line)
            || matches(&SEPARATION, &line.to_lowercase())
            || [&*BULLIED, &*A_NAME_WAS_VIOLENT, &*HURTS_A_PERSON]
                .iter()
                .any(|r| matches(r, &line))
    })
}

pub(super) fn detail(read: &Reading<'_>, clean: &str, memory: &Memory) -> bool {
    let Some(record) = memory.ack_topics.prior(clean, &memory.turns) else {
        return false;
    };
    let repaired = if read.line.clean == clean.to_lowercase() && !read.line.plain.is_empty() {
        read.line.plain.as_str()
    } else {
        clean
    };
    let body = strip_lead(clean);
    let letter = |s: &str| {
        s.chars()
            .any(|c| c.is_ascii_alphabetic() || ('\u{00c0}'..='\u{024f}').contains(&c))
    };
    if clean.contains('?')
        || (!matches(&STATE, body) && body.chars().count() > 30)
        || matches(&TROUBLE, body)
        || !letter(body)
        || !letter(&REPAIR_FRAME.replace(clean, ""))
        || (clean.split_whitespace().count() <= 2 && !clean.chars().any(char::is_numeric))
        || [
            &*WANTS_ANSWER,
            &*TROUBLE,
            &*NOT_A_DETAIL,
            &*ANSWER_OPENER,
            &*ASKED_FOR,
            &*SAID_YES_NO,
            &*ASKS_MIDWAY,
            &*FAULT,
        ]
        .iter()
        .any(|r| matches(r, repaired))
    {
        return false;
    }
    let words = repaired.split_whitespace().collect::<Vec<_>>();
    if compose::trouble::reads_as_trouble(repaired, &words, &read.line.plain).is_some() {
        return false;
    }
    let last = &memory.turns[memory.turns.len().saturating_sub(3)..];
    let carried = std::iter::once(record)
        .chain(last.iter().map(|t| t.user.as_str()))
        .collect::<Vec<_>>()
        .join(" ");
    if matches(&DEATH_WORDS, &carried)
        || matches(&NOT_A_DETAIL_SUBJECT, &carried)
        || last
            .iter()
            .rev()
            .take(2)
            .any(|t| modules::the_detail_the_thread_adds::heavy::bad_news(&t.user))
    {
        return false;
    }
    let lower = clean.to_lowercase();
    let mut meaningful = CONTENT_WORD
        .find_iter(&lower)
        .map(|m| m.as_str())
        .filter(|w| !has(EMPTY, w))
        .peekable();
    let Some(first) = meaningful.next() else {
        return false;
    };
    if meaningful.peek().is_none() {
        return false;
    }
    let known = record.to_lowercase();
    std::iter::once(first)
        .chain(meaningful)
        .any(|w| !CONTENT_WORD.find_iter(&known).any(|m| m.as_str() == w))
}

pub(super) fn resolved_worry(clean: &str, memory: &Memory) -> bool {
    let text = clean.to_lowercase();
    if text.contains('?')
        || matches(&ASK_OPEN, &text)
        || matches(&CONDITIONAL, &text)
        || !memory
            .turns
            .iter()
            .rev()
            .filter(|turn| !turn.user.trim().is_empty())
            .take(4)
            .any(|turn| {
                turn.user
                    .to_lowercase()
                    .split(|c: char| !c.is_ascii_lowercase() && c != '\'')
                    .any(|w| has(WORRY_WORDS, w))
            })
    {
        return false;
    }
    settled_patterns().into_iter().any(|p| {
        let Some(hit) = p.find(&text).ok().flatten() else {
            return false;
        };
        let begin = text[..hit.start()]
            .char_indices()
            .rev()
            .nth(25)
            .map_or(0, |(offset, _)| offset);
        if matches(&DENIED, &text[begin..hit.end()]) {
            return false;
        }
        if !matches(&BARE_GONE, hit.as_str()) {
            return true;
        }
        let tail = text[hit.end()..].replace([',', ';', ':', '!', '?'], " ");
        !text.contains("gone again")
            && !matches(&BODY_GAVE_WAY, &text)
            && (tail.trim().is_empty() || matches(&SETTLED_TAIL, &format!(" {}", tail.trim())))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/ack_context.rs"
    ));

    #[test]
    fn contextual_details_and_recent_loss_match_reference() {
        let mut differences = Vec::new();
        for &(prior, text, wanted_detail, wanted_heavy) in CASES {
            let mut memory = Memory::default();
            memory.ack_topics.note(&Reading::new(prior), &[]);
            memory.turns.push(crate::bot::Exchange {
                user: prior.to_owned(),
                bot: String::new(),
                module: "",
            });
            let read = Reading::new(text);
            let actual = (detail(&read, text, &memory), heavy(&memory));
            if actual != (wanted_detail, wanted_heavy) {
                differences.push((prior, text, actual, (wanted_detail, wanted_heavy)));
            }
        }
        assert!(differences.is_empty(), "{differences:#?}");
    }
}
