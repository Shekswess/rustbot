//! The first instruction of an answer when the person asks which part to do first.
#[path = "step_back_data.rs"]
mod data;
#[cfg(test)]
#[path = "step_back_tests.rs"]
mod tests;

use super::{
    ANSWER_WORD, ATTEMPT, Carried, RED_FLAG, already_repeated, cap, seen, sentences, starts,
};
use crate::bot::{Memory, compose, memory::again};
use fancy_regex::{Match, Regex};
use std::{collections::HashSet, sync::LazyLock};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).unwrap());)+
    };
}
patterns!(
    WORDING,
    QUALIFIER,
    LIST_LEAD,
    NOT_A_TAIL,
    NOT_A_DOING,
    A_QUESTION,
    DECLARATIVE,
    QUESTIONS_THAT_ARE_NOT_STEPS,
    PIECE_END,
    NOT_A_STEP,
    A_MENU,
    CONDITION_CLAUSE,
    CHANGE_ASK,
    STEP_SAYS_FIRST,
    LEAD_SAYS_FIRST,
);
static JOINER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:and|then|but|also|so|next|plus)\s+").unwrap());
static FINAL_FIRST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\s+(?:first|next)$").unwrap());
static FIGURE_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\bhow (?:long|much|many|often|far|hot|soon)\b").unwrap());

const PART_END: &[char] = &[' ', '.', ':', ';'];
const CLAUSE_END: &[char] = &[' ', ',', '.', ':', ';'];

fn instruction_verb(word: &str) -> bool {
    super::data::IMPERATIVES.contains(&word) || super::data::MOVE_VERBS.contains(&word)
}

fn starts_verb(text: &str) -> bool {
    text.trim_matches([' ', '.', ',', ':', ';'])
        .split_whitespace()
        .next()
        .is_some_and(|word| instruction_verb(&word.to_lowercase()))
}

fn after_short_lead(text: &str) -> bool {
    let tokens: Vec<_> = text
        .split_whitespace()
        .map(|word| word.trim_matches([' ', ',', '-']).to_lowercase())
        .collect();
    if tokens
        .first()
        .is_none_or(|word| data::AN_ARTICLE.contains(&word.as_str()))
    {
        return false;
    }
    (1..tokens.len().min(data::LEADING_PHRASE_MAX + 1)).any(|at| {
        instruction_verb(&tokens[at])
            && tokens[..at].iter().all(|item| {
                data::LEADING_PHRASE.contains(&item.as_str())
                    || data::AN_ARTICLE.contains(&item.as_str())
            })
    })
}

fn bracket_open(part: &str, index: usize) -> bool {
    let before = &part[..index];
    before.matches('(').count() > before.matches(')').count()
}

fn cuttable(part: &str) -> Vec<usize> {
    let mut spots: Vec<_> = QUALIFIER
        .find_iter(part)
        .filter_map(Result::ok)
        .filter_map(|found| {
            let first = part[found.end()..]
                .split_whitespace()
                .next()?
                .to_lowercase();
            (!data::OPENERS.contains(&first.as_str())).then_some(found.start())
        })
        .collect();
    spots.extend(
        part.match_indices('(')
            .filter(|(index, _)| part[*index..].contains(')'))
            .map(|(index, _)| index),
    );
    spots.sort_unstable();
    spots.dedup();
    spots
}

enum ColonTail {
    Split,
    AnnouncedList,
    Whole,
    FirstItem(String, usize),
}

fn colon_tail(part: &str, marks: &[Match<'_>], index: usize, start: usize) -> ColonTail {
    let colon = marks[index];
    let head = part
        .get(start..colon.start())
        .unwrap_or("")
        .trim_matches([' ', ',', ';']);
    if !starts_verb(head) {
        return ColonTail::Split;
    }
    let ending = marks.get(index + 1).map_or(part.len(), Match::start);
    let after = part[colon.end()..ending].trim_matches([' ', ',', ';', '.']);
    if seen(&NOT_A_TAIL, head) && !starts(&A_QUESTION, after) {
        return ColonTail::AnnouncedList;
    }
    let tail = &part[colon.end()..];
    if head.split_whitespace().count() + tail.split_whitespace().count()
        <= data::STEP_WITH_ITS_METHOD
    {
        return ColonTail::Whole;
    }
    for mark in &marks[index + 1..] {
        if !matches!(mark.as_str().trim(), "," | ";") {
            continue;
        }
        let first = part[colon.end()..mark.start()].trim_matches([' ', ',', ';']);
        if first.is_empty() {
            continue;
        }
        let clause = format!("{head}: {first}");
        if clause.split_whitespace().count() <= data::STEP_WITH_ITS_METHOD {
            return ColonTail::FirstItem(clause, mark.end());
        }
        return ColonTail::Split;
    }
    ColonTail::Split
}

fn pieces(part: &str) -> Vec<String> {
    let marks: Vec<_> = PIECE_END
        .find_iter(part)
        .filter_map(Result::ok)
        .filter(|found| !bracket_open(part, found.start()))
        .collect();
    let mut pieces = Vec::new();
    let mut start = 0;
    let mut listed = false;
    for (index, found) in marks.iter().enumerate() {
        let punctuation = found.as_str().trim();
        if punctuation == "," && !listed {
            let end = marks.get(index + 1).map_or(part.len(), Match::start);
            let after = JOINER.replace(part[found.end()..end].trim_matches(CLAUSE_END), "");
            if !starts_verb(&after) {
                continue;
            }
        }
        if punctuation == ":" && !listed {
            match colon_tail(part, &marks, index, start) {
                ColonTail::Split => (),
                ColonTail::AnnouncedList => {
                    start = part.len();
                    listed = true;
                    continue;
                }
                ColonTail::Whole => continue,
                ColonTail::FirstItem(first, at) => {
                    pieces.push(first);
                    start = at;
                    listed = true;
                    continue;
                }
            }
        }
        pieces.push(part.get(start..found.start()).unwrap_or("").into());
        start = found.end();
        listed |= punctuation == ":";
    }
    pieces.push(part[start..].into());
    pieces
}

fn list_item(part: &str, pieces: &[String]) -> Option<String> {
    let (lead, _) = part.split_once(':')?;
    if !seen(&LIST_LEAD, lead) {
        return None;
    }
    let mut items = pieces
        .iter()
        .skip(1)
        .map(|item| item.trim_matches(PART_END))
        .filter(|item| !item.is_empty());
    let first = items.next()?;
    items.next()?;
    if !(3..=8).contains(&first.split_whitespace().count())
        || seen(&NOT_A_DOING, first)
        || starts(&RED_FLAG, first)
        || starts(&NOT_A_STEP, first)
        || first.split_whitespace().any(|word| {
            instruction_verb(
                &word
                    .trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"'])
                    .to_lowercase(),
            )
        })
    {
        return None;
    }
    Some(cap(first, false))
}

fn first_instruction(part: &str) -> Option<&str> {
    let before = part.split(',').next()?.trim_matches(CLAUSE_END);
    (before.split_whitespace().count() >= data::SHORT_STEP
        && (starts_verb(before) || after_short_lead(before)))
    .then_some(before)
}

fn instruction(piece: &str) -> Option<String> {
    let joined = JOINER.replace(piece.trim_matches(PART_END), "");
    let mut head = joined.as_ref();
    if head.is_empty()
        || starts(&RED_FLAG, head)
        || starts(&NOT_A_STEP, head)
        || seen(&A_MENU, head)
        || starts(&DECLARATIVE, head)
        || head.ends_with('?')
        || starts(&QUESTIONS_THAT_ARE_NOT_STEPS, head)
    {
        return None;
    }
    let (limit, cut_at) = if head.contains(':') {
        (data::STEP_WITH_ITS_METHOD, data::STEP_WITH_ITS_METHOD)
    } else {
        (data::STEP_WORDS, data::CUT_WORDS)
    };
    let first = head.split_whitespace().next()?.to_lowercase();
    if head.split_whitespace().count() > cut_at
        && !data::NEEDS_COMPLEMENT.contains(&first.as_str())
        && let Some(short) = cuttable(head)
            .into_iter()
            .filter(|at| !bracket_open(head, *at))
            .find_map(|at| {
                let before = head[..at].trim_matches(CLAUSE_END);
                let words = before.split_whitespace();
                if words.clone().count() < data::SHORT_STEP
                    || words
                        .last()
                        .is_some_and(|word| data::DANGLING.contains(&word.to_lowercase().as_str()))
                {
                    return None;
                }
                Some(before)
            })
    {
        head = short;
    }
    if !(3..=limit).contains(&head.split_whitespace().count()) {
        head = first_instruction(head)?;
    }
    if !starts_verb(head) && !after_short_lead(head) {
        return None;
    }
    let no_first = FINAL_FIRST.replace(head, "");
    let part = if no_first
        .split_whitespace()
        .last()
        .is_some_and(|word| !data::DANGLING.contains(&word.to_lowercase().as_str()))
    {
        no_first.as_ref()
    } else {
        head
    };
    Some(cap(part, false))
}

pub(in crate::bot) fn steps(reply: &str) -> impl Iterator<Item = String> + '_ {
    sentences(reply)
        .filter(|part| !starts(&RED_FLAG, part) && !starts(&CONDITION_CLAUSE, part))
        .flat_map(|part| {
            let pieces = pieces(part);
            let mut found: Vec<_> = pieces
                .iter()
                .filter_map(|piece| instruction(piece))
                .collect();
            if found.is_empty() {
                found.extend(list_item(part, &pieces));
            }
            found
        })
}

fn step(reply: &str) -> Option<String> {
    steps(reply).next()
}

fn tried_it(step: &str, memory: &Memory) -> bool {
    let low = step.to_lowercase();
    let words: HashSet<_> = ANSWER_WORD
        .find_iter(&low)
        .map(|word| word.as_str())
        .filter(|word| word.len() > 2)
        .collect();
    if words.is_empty() {
        return false;
    }
    memory.turns.iter().rev().take(3).any(|turn| {
        let said = turn.user.to_lowercase();
        if !seen(&ATTEMPT, &said) {
            return false;
        }
        let same: HashSet<_> = ANSWER_WORD
            .find_iter(&said)
            .map(|word| word.as_str())
            .filter(|word| word.len() > 2 && words.contains(word))
            .collect();
        same.len() >= 2 || same.iter().any(|word| word.len() >= 5)
    })
}

pub(super) fn hand_back(text: &str, reply: &str, memory: &Memory) -> Option<String> {
    if reply.trim().is_empty()
        || !again::asks_for_move(text)
        || seen(&WORDING, text)
        || seen(&FIGURE_ASK, text)
    {
        return None;
    }
    let step = step(reply)?;
    if already_repeated(&step, memory, None) || tried_it(&step, memory) {
        return None;
    }
    let pool = if seen(&CHANGE_ASK, text) {
        data::CHANGE_LEADS
    } else {
        data::MOVE_LEADS
    };
    let clear_leads: Vec<_> = if starts(&STEP_SAYS_FIRST, step.trim()) {
        pool.iter()
            .copied()
            .filter(|line| !seen(&LEAD_SAYS_FIRST, line))
            .collect()
    } else {
        pool.to_vec()
    };
    let leads = if clear_leads.is_empty() {
        data::NOW_LEADS
    } else {
        &clear_leads
    };
    let lead = compose::pick(leads, memory, text);
    let said = lead.replace("{step}", &cap(step.trim_end(), true));
    let said = said.replace(".. ", ". ");
    let said = if said.ends_with("..") {
        format!("{}.", said.trim_end_matches('.'))
    } else {
        said
    };
    Some(Carried::hand_back(said, memory).reply)
}
