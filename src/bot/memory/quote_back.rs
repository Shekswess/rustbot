//! Recall a quotation from the user's earlier lines.

use std::{collections::HashSet, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use regex::Regex;

use crate::{bot::Memory, text::is_space};

#[path = "quote_back_data.rs"]
mod data;

use data::*;

macro_rules! rule {
    ($name:ident, $kind:ty, $source:expr) => {
        static $name: LazyLock<$kind> =
            LazyLock::new(|| <$kind>::new($source).expect(stringify!($name)));
    };
}

rule!(ASK, Lookaround, A_THIRD_PARTY_ASK);
rule!(SPOKE, Lookaround, SOMEBODY_SPOKE);
rule!(CLAUSE, Regex, REPORTING_VERB);
rule!(FIGURE, Regex, A_FIGURE);
rule!(MONEY, Regex, A_MONEY);
rule!(UNIT, Regex, A_UNIT_AFTER);
rule!(QUESTION_START, Regex, QUESTION);
rule!(WORDS, Regex, r"[a-z']+");
rule!(PAST_RECALL, Regex, ASKS_BACK);
rule!(NAMED_RECALL, Regex, ASKS_BACK_A_KIND);
rule!(TELLING, Regex, A_TELLING);
rule!(PAST_ACT, Regex, A_PAST_ACT);
rule!(HOW_MUCH, Regex, r"\bhow\s+much\b");
rule!(
    PREPOSITION_BEFORE_FIGURE,
    Lookaround,
    r"(?i)^(?:for|to|about|of|at)\s+(?=(?:£|\$|€)?\s?\d)"
);

fn flat(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn has_word(text: &str, choices: &[&str]) -> bool {
    WORDS
        .find_iter(&text.to_lowercase())
        .any(|word| choices.contains(&word.as_str().trim_matches('\'')))
}

fn is_recall_ask(text: &str) -> bool {
    PAST_RECALL.is_match(text)
        || NAMED_RECALL
            .find(text)
            .is_some_and(|found| !TELLING.is_match(&text[found.end()..]))
        || PAST_ACT.is_match(text) && (has_word(text, A_KIND_FOR_WORD) || HOW_MUCH.is_match(text))
}

fn details(text: &str) -> HashSet<String> {
    WORDS
        .find_iter(&text.to_lowercase())
        .map(|word| word.as_str().trim_matches('\''))
        .filter(|word| word.len() > 1 && !NOT_A_DETAIL.contains(word))
        .map(str::to_owned)
        .collect()
}

fn figure(line: &str) -> Option<&str> {
    let figures: Vec<_> = FIGURE
        .find_iter(line)
        .filter(|found| !UNIT.is_match(&line[found.end()..]))
        .collect();
    let mut money = figures
        .iter()
        .filter(|found| MONEY.is_match(found.as_str()));
    if let Some(first) = money.next() {
        return money.next().is_none().then(|| first.as_str().trim());
    }
    let [only] = figures.as_slice() else {
        return None;
    };
    Some(only.as_str().trim())
}

fn reported_clause(line: &str) -> Option<&str> {
    let mut clause = None;
    for found in CLAUSE.captures_iter(line) {
        let speaker = found.name("who")?.as_str().split(' ').next_back()?;
        if !THE_PERSON.contains(&speaker.trim_matches(['\'', ','])) {
            clause = found
                .get(2)
                .map(|after| after.as_str().trim_matches(is_space));
        }
    }
    let said = clause?.trim_end_matches([' ', '.', '!', '?', ',']);
    let said = PREPOSITION_BEFORE_FIGURE
        .find(said)
        .ok()
        .flatten()
        .map_or(said, |found| &said[found.end()..]);
    if said.split(' ').count() > 14
        || QUESTION_START.is_match(said)
        || !said.bytes().any(|byte| byte.is_ascii_alphanumeric())
    {
        return None;
    }
    Some(said)
}

pub(in crate::bot) fn third_party(text: &str, memory: &Memory) -> Option<String> {
    let ask = flat(text);
    if !ASK.is_match(&ask).unwrap_or(false) {
        return None;
    }
    let figure_asked = has_word(&ask, A_KIND);
    let wanted = details(&ask);
    let line = memory
        .turns
        .iter()
        .rev()
        .map(|turn| flat(&turn.user))
        .find(|line| {
            !QUESTION_START.is_match(line)
                && !is_recall_ask(line)
                && SPOKE.is_match(line).unwrap_or(false)
                && (figure_asked || wanted.is_empty() || !wanted.is_disjoint(&details(line)))
        })?;
    let recalled = figure_asked
        .then(|| figure(&line))
        .flatten()
        .or_else(|| reported_clause(&line))?;
    Some(format!("You said {recalled}."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    fn history(lines: &[&str]) -> Memory {
        Memory {
            turns: lines
                .iter()
                .map(|&text| Exchange {
                    user: text.into(),
                    bot: String::new(),
                    module: "greeting",
                })
                .collect(),
            ..Memory::default()
        }
    }

    #[test]
    fn the_latest_quotation_can_be_named_by_speaker_or_price() {
        let memory = history(&[
            "we got quoted 400 for the fence",
            "i told the garage about the noise and they said bring it in tuesday",
            "i need a new boiler, british gas said 3 grand",
        ]);
        assert_eq!(
            third_party("what did they say it would cost", &memory).as_deref(),
            Some("You said 3 grand.")
        );
        assert_eq!(
            third_party("what did garage say", &memory).as_deref(),
            Some("You said bring it in tuesday.")
        );
        assert_eq!(third_party("what did i say", &memory), None);
    }

    #[test]
    fn currency_wins_over_an_age_but_two_prices_do_not_pick_a_winner() {
        assert_eq!(
            figure("they said £2,000 for my 3 year old car"),
            Some("£2,000")
        );
        assert_eq!(figure("they said 400 for the fence"), Some("400"));
        assert_eq!(figure("they said £200 or £300"), None);
        assert_eq!(figure("my car is 3 years old"), None);
        assert_eq!(reported_clause("the vet asked for 60"), Some("60"));
        assert_eq!(reported_clause("i told my boss I was quitting"), None);
    }

    #[test]
    fn recall_asks_are_excluded_from_earlier_statements() {
        assert!(is_recall_ask("the size i gave"));
        assert!(!is_recall_ask("the size i gave was big"));
        assert!(is_recall_ask("i said, how much did i pay"));
        assert_eq!(third_party("what did they quote me", &history(&[])), None);
    }
}
