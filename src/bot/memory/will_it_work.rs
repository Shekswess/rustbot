//! Recall the result clause of advice the bot actually put on screen.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::{borrow::Cow, collections::VecDeque, sync::LazyLock};

macro_rules! pattern {
    ($name:ident,$source:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($source).unwrap());
    };
}
#[path = "will_it_work_data.rs"]
mod data;
use data::*;

#[derive(Default)]
pub(crate) struct EntryAnswers(VecDeque<Cow<'static, str>>);

impl EntryAnswers {
    pub(super) fn oldest_first(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(Cow::as_ref)
    }

    pub(crate) fn latest(&self) -> Option<&str> {
        self.0.back().map(Cow::as_ref)
    }

    pub(crate) fn remember(&mut self, text: Cow<'static, str>) {
        if self.0.len() == 4 {
            self.0.pop_front();
        }
        self.0.push_back(text);
    }

    pub(crate) fn on_screen<'a>(&'a self, last: &str, module: &str) -> Option<&'a str> {
        if matches!(
            module,
            "loss" | "grief" | "safety" | "safety_cards" | "safety_phrases"
        ) {
            return None;
        }
        let said = last
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        if said.is_empty() {
            return None;
        }
        self.0.iter().rev().find_map(|s| {
            let opening = s
                .split_whitespace()
                .take(8)
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase();
            let first_five = opening
                .split_whitespace()
                .take(5)
                .collect::<Vec<_>>()
                .join(" ");
            (said.starts_with(&opening) || said.contains(&first_five)).then_some(s.as_ref())
        })
    }
}

static CLAUSE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[,;:]\s+|\s+-\s+").unwrap());
pub(super) static SENTENCE: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(r"(?<=[.!?])\s+").unwrap());

fn achieves(card: &str) -> Option<&str> {
    let sentences: Vec<_> = SENTENCE
        .split(card)
        .filter_map(Result::ok)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    for sentence in sentences
        .into_iter()
        .rev()
        .filter(|s| ACHIEVES.is_match(s) && !NOT_ACHIEVING.is_match(s))
    {
        for clause in CLAUSE.split(sentence) {
            let mut said = clause.trim_matches([' ', '.']);
            let Some(first) = said.split_whitespace().next() else {
                continue;
            };
            if !ACHIEVES.is_match(said)
                || NOT_ACHIEVING.is_match(said)
                || JOINS_ON
                    .binary_search(
                        &first
                            .trim_matches([',', '.', ';', ':'])
                            .to_ascii_lowercase()
                            .as_str(),
                    )
                    .is_ok()
            {
                continue;
            }
            for filler in TRAILING_FILLER {
                if let Some(tail) = said.get(said.len().saturating_sub(filler.len() + 1)..)
                    && tail.starts_with(' ')
                    && tail[1..].eq_ignore_ascii_case(filler)
                {
                    said = said[..said.len() - tail.len()].trim_end_matches([' ', ',', '.']);
                }
            }
            if (4..=11).contains(&said.split_whitespace().count()) {
                return Some(said);
            }
        }
    }
    None
}

fn from_card(raw: &str, card: &str) -> Option<String> {
    let clause = achieves(card)?;
    let low = clause.to_lowercase();
    if low.starts_with("yes") || low.starts_with("no") {
        return None;
    }
    let mut letters = clause.chars();
    let first = letters.next()?;
    let second = letters.next();
    let lower = first.is_uppercase()
        && (second.is_some_and(char::is_lowercase)
            || second == Some(' ') && letters.next().is_some_and(char::is_lowercase));
    let body = if lower {
        format!("{}{}", first.to_lowercase(), &clause[first.len_utf8()..])
    } else {
        clause.to_owned()
    };
    let at = raw
        .chars()
        .map(|c| c as usize % YES_IN_FRONT.len())
        .sum::<usize>()
        % YES_IN_FRONT.len();
    Some(format!(
        "{} — {}.",
        YES_IN_FRONT[at],
        body.trim_end_matches([' ', '.'])
    ))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let checked = reading.raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if !WORK_CHECK.is_match(checked.trim_matches([' ', '?', '.', '!', ','])) {
        return None;
    }
    let last = memory.turns.last()?;
    let card = memory.entries.on_screen(&last.bot, last.module)?;
    from_card(reading.raw, card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;
    mod cases {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/will_it_work.rs"
        ));
    }
    #[test]
    fn checks_the_card_for_a_result_clause() {
        let mut memory = Memory::default();
        for &(card, ask, wanted) in cases::RESULTS {
            let text = cases::CARDS[card];
            memory.entries.remember(Cow::Borrowed(text));
            memory.turns.push(Exchange {
                module: "answers",
                user: "How should I do that?".into(),
                bot: text.into(),
            });
            assert_eq!(
                respond(&Reading::new(cases::ASKS[ask]), &memory).as_deref(),
                wanted,
                "{} {text}",
                cases::ASKS[ask]
            );
        }
    }
    #[test]
    fn recalls_only_visible_registered_entries() {
        let mut entries = EntryAnswers::default();
        for &text in &cases::CARDS[..5] {
            entries.remember(Cow::Borrowed(text));
        }
        assert_eq!(entries.on_screen(cases::CARDS[0], "answers"), None);
        assert_eq!(entries.on_screen(cases::CARDS[1], "safety"), None);
        let words = cases::CARDS[1]
            .split_whitespace()
            .take(5)
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(
            entries.on_screen(&format!("Again: {words}"), "greeting"),
            Some(cases::CARDS[1])
        );
    }
}
