//! The topic records read by the doneness observer.
use super::again;
use crate::bot::{Exchange, Reading, compose::routes, understanding::kinds::Kind};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

macro_rules! rule {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}

include!("done_topic_data.rs");

pub(crate) fn topic_words(text: &str) -> Vec<String> {
    again::topic_words(text)
}

fn stem(word: String) -> String {
    if word.len() > 4
        && let Some(base) = word.strip_suffix("ies")
    {
        return format!("{base}y");
    }
    if word.len() > 3
        && let Some(base) = word.strip_suffix("es")
        && base.ends_with(['s', 'x', 'z', 'h'])
    {
        return base.to_owned();
    }
    if word.len() > 3
        && !word.ends_with("ss")
        && let Some(base) = word.strip_suffix('s')
    {
        return base.to_owned();
    }
    word
}

pub(super) fn keywords(text: &str) -> impl Iterator<Item = String> {
    let mut used = HashSet::new();
    topic_words(text)
        .into_iter()
        .filter(|word| {
            word.len() >= 3
                && !again::topic_stopword(word)
                && !word.bytes().all(|byte| byte.is_ascii_digit())
        })
        .map(stem)
        .filter(move |word| used.insert(word.clone()))
}

pub(crate) fn candidate_words<'a>(
    topics: impl IntoIterator<Item = &'a str>,
    words: &[String],
) -> Vec<String> {
    let mut seen = HashSet::new();
    topics
        .into_iter()
        .flat_map(keywords)
        .filter(|word| {
            !words.contains(word) && !has(MEASURE_WORDS, word) && seen.insert(word.clone())
        })
        .collect()
}

fn has(set: &[&str], word: &str) -> bool {
    set.binary_search(&word).is_ok()
}

fn lead_ack(word: &str) -> bool {
    has(ACKS, word) || has(GREET, word)
}

fn small_talk(clean: &str, words: &[String], kind: Kind) -> bool {
    matches!(kind, Kind::Empty | Kind::Greeting | Kind::Thanks)
        || words.is_empty()
        || lead_ack(clean)
        || (words.len() <= 4
            && words.iter().any(|word| lead_ack(word))
            && words
                .iter()
                .all(|word| word.len() <= 3 || lead_ack(word) || again::topic_stopword(word)))
}

fn switches(clean: &str) -> bool {
    let padded = format!(" {clean} ");
    SWITCHERS
        .iter()
        .any(|switcher| clean.starts_with(switcher) || padded.contains(&format!(" {switcher} ")))
}

fn short_followup(words: &[String], fresh: usize) -> bool {
    if words.len() > 16 {
        return false;
    }
    let clean = words
        .iter()
        .skip_while(|word| lead_ack(word))
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    if switches(&clean) {
        return false;
    }
    if words.len() <= 10 && KIND_ASK.is_match(&clean) {
        return true;
    }
    if has(CONTINUATION_START, &words[0])
        && ((words.len() <= 8 && fresh <= 2)
            || (words.len() <= 10 && fresh <= 3 && PROPERTY.is_match(&clean)))
    {
        return true;
    }
    (fresh <= 3 && words.iter().any(|word| has(REFERRING, word)))
        || (fresh <= 1 && words.len() <= 8)
}

type Identity = (&'static str, &'static str);

struct FiledLine<'a> {
    text: &'a str,
    words: Vec<String>,
    clean: String,
    keys: Vec<String>,
}

impl<'a> FiledLine<'a> {
    fn new(text: &'a str) -> Self {
        let words = topic_words(text);
        let clean = words.join(" ");
        let keys = keywords(text).collect();
        Self {
            text,
            words,
            clean,
            keys,
        }
    }
}

fn only_details(
    read: &Reading<'_>,
    line: &FiledLine<'_>,
    record: &Record,
    turn: usize,
    entry: &impl Fn(&str) -> Option<Identity>,
) -> Option<bool> {
    let said = &line.clean;
    let known = &record.words;
    let keys = &line.keys;
    if read.kind != Kind::Statement || turn - record.turn > 3 {
        return None;
    }
    let quantity = read
        .numbers
        .iter()
        .any(|number| number.unit.is_some() || SHORTHAND.is_match(number.text.trim()));
    let on_budget = ON_A_BUDGET.is_match(said);
    let constraint = if on_budget {
        said.bytes().any(|byte| byte.is_ascii_digit())
    } else {
        quantity && CONSTRAINT_IS.is_match(said)
    };
    if constraint {
        return Some(false);
    }
    if quantity
        && MY_THING_IS
            .captures(said)
            .is_none_or(|matched| known.contains(&stem(matched[1].to_owned())))
    {
        let numbers: HashSet<_> = read
            .numbers
            .iter()
            .flat_map(|number| topic_words(&number.text).into_iter().map(stem))
            .collect();
        if keys
            .iter()
            .filter(|word| !known.contains(word) && !numbers.contains(*word))
            .count()
            <= 2
        {
            return Some(true);
        }
    }
    (line.words.len() <= 8
        && keys.len() <= 4
        && WHEN_OR_AMOUNT.is_match(said)
        && !NAMED_THING.is_match(said)
        && entry(line.text).is_none())
    .then_some(true)
}

#[derive(Default)]
pub(crate) struct Topics {
    records: Vec<Record>,
    current: Option<usize>,
    last: Option<Change>,
}

struct Record {
    text: String,
    words: Vec<String>,
    turn: usize,
}

struct Change {
    previous: Option<usize>,
    added: usize,
}

impl Topics {
    pub(super) fn correct(&mut self, text: &str) {
        let Some(index) = self.current else {
            return;
        };
        let record = &mut self.records[index];
        record.text = text.to_owned();
        record.words = keywords(text).collect();
        self.last = None;
    }

    pub(crate) fn current_text(&self) -> Option<&str> {
        self.current.map(|index| self.records[index].text.as_str())
    }

    pub(crate) fn current_words(&self) -> &[String] {
        self.current.map_or(&[], |index| &self.records[index].words)
    }

    pub(crate) fn recent_texts(&self, count: usize) -> impl Iterator<Item = &str> {
        self.records[self.records.len().saturating_sub(count)..]
            .iter()
            .map(|record| record.text.as_str())
            .filter(|text| !text.is_empty())
    }

    pub(crate) fn recorded_on_turn(&self, turn: usize) -> bool {
        self.records.iter().any(|record| record.turn == turn)
    }

    pub(crate) fn texts(&self) -> Vec<&str> {
        let mut seen = HashSet::new();
        self.current
            .into_iter()
            .map(|index| &self.records[index])
            .chain(self.records.iter().rev())
            .map(|record| record.text.as_str())
            .filter(|text| seen.insert(*text))
            .collect()
    }

    pub(crate) fn note(
        &mut self,
        read: &Reading<'_>,
        turns: &[Exchange],
        entry: impl Fn(&str) -> Option<Identity>,
    ) {
        self.undo_step_back(turns.last());
        let text = if read.line.change_of_subject && !read.clean.trim().is_empty() {
            read.clean.trim()
        } else {
            read.raw.trim()
        };
        let line = FiledLine::new(text);
        let words = &line.words;
        let said = &line.clean;
        if small_talk(said, words, read.kind) || read.features.about_bot {
            return;
        }
        let keys = &line.keys;
        let in_emergency = turns.last().is_some_and(|last| last.module == "safety");
        if let Some(index) = self.current
            && !in_emergency
        {
            let record = &self.records[index];
            let fresh = keys
                .iter()
                .filter(|word| !record.words.contains(word))
                .count();
            let leans = read.reference.followup || short_followup(words, fresh);
            let used_switcher = switches(said);
            let full_switch = used_switcher
                && !keywords(
                    &words
                        .iter()
                        .filter(|word| {
                            !has(ACKS, word)
                                && !has(CORRECTION_NOISE, word)
                                && !SWITCHERS.iter().any(|switcher| {
                                    switcher.split_whitespace().any(|part| part == *word)
                                })
                        })
                        .map(String::as_str)
                        .collect::<Vec<_>>()
                        .join(" "),
                )
                .filter(|word| !record.words.contains(word))
                .all(|word| has(DETAIL_WORDS, &word));
            if leans && !full_switch {
                let asks = text.contains('?') || has(RETURN_ASKS, &words[0]);
                let last_answer = turns.last().map_or("", |turn| turn.bot.as_str());
                let points = words.iter().any(|word| has(REFERRING, word))
                    || PART_OF_THE_ONE.is_match(said)
                    || ABOUT_ANOTHER.is_match(said)
                    || keys.iter().all(|word| has(MEASURE_WORDS, word))
                    || keys.iter().any(|word| record.words.contains(word))
                    || (!last_answer.is_empty()
                        && keys
                            .iter()
                            .any(|word| topic_words(last_answer).contains(word)));
                let names_another = !(asks && points)
                    && entry(text)
                        .zip(entry(&record.text))
                        .is_some_and(|((_, mine), (_, old))| mine != old);
                let part_of_record = read.reference.followup
                    && turns
                        .last()
                        .is_some_and(|turn| topic_words(&turn.user) == topic_words(&record.text));
                if !names_another || part_of_record {
                    self.add(index, keys);
                    return;
                }
            }
            if let Some(add) = only_details(read, &line, record, turns.len(), &entry) {
                if add {
                    self.add(index, keys);
                }
                return;
            }
        }
        if (keys.len() < 2 && words.len() < 4 && read.topics.is_empty())
            || self
                .current
                .is_some_and(|index| self.records[index].text == text)
        {
            return;
        }
        if let Some(returned) = RETURN_LEAD.find(said) {
            let target = said[returned.end()..].trim();
            if let Some(index) = self.records.iter().rposition(|record| {
                let words: Vec<_> = keywords(target).collect();
                !words.is_empty() && words.iter().all(|word| record.words.contains(word))
            }) {
                self.last = Some(Change {
                    previous: self.current,
                    added: 0,
                });
                self.current = Some(index);
                return;
            }
            if !topic_words(target)
                .iter()
                .any(|word| has(RETURN_ASKS, word))
            {
                return;
            }
        }
        self.last = Some(Change {
            previous: self.current,
            added: 0,
        });
        self.current = Some(self.records.len());
        self.records.push(Record {
            text: text.to_owned(),
            words: line.keys,
            turn: turns.len(),
        });
    }

    fn add(&mut self, index: usize, keys: &[String]) {
        let record = &mut self.records[index];
        let extra: Vec<_> = keys
            .iter()
            .filter(|word| !record.words.contains(word))
            .take(4.min(12usize.saturating_sub(record.words.len())))
            .cloned()
            .collect();
        self.last = Some(Change {
            previous: self.current,
            added: extra.len(),
        });
        record.words.extend(extra);
    }

    fn undo_step_back(&mut self, turn: Option<&Exchange>) {
        if let Some(change) = self.last.take()
            && let Some(current) = self.current
            && turn.is_some_and(|turn| routes::hedge(&turn.bot))
        {
            let record = &mut self.records[current];
            record.words.truncate(record.words.len() - change.added);
            self.current = change.previous.or(self.current);
        }
    }
}
