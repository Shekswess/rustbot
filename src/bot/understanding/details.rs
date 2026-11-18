//! Additional structured fields from PyBot's shared reading.
#[path = "detail_data.rs"]
mod data;
use super::{Reading, topics};
use data::*;
use fancy_regex::Regex;
use std::sync::LazyLock;
fn hit(r: &Regex, s: &str) -> bool {
    r.is_match(s).unwrap_or(false)
}
#[derive(serde::Serialize)]
pub(crate) struct StandIn {
    word: String,
    noun: Option<String>,
}
fn stand_in(s: &str) -> Option<StandIn> {
    which_one(s).map(|place| StandIn {
        word: s[place].to_ascii_lowercase(),
        noun: None,
    })
}
pub(super) fn which_one(s: &str) -> Option<std::ops::Range<usize>> {
    A_STAND_IN.captures_iter(s).flatten().find_map(|m| {
        if let Some(lead) = m.get(2) {
            let w = lead.as_str().to_lowercase();
            if STAND_IN_WORDS.binary_search(&w.as_str()).is_err()
                || STAND_IN_SELECTORS.binary_search(&w.as_str()).is_ok()
            {
                return None;
            }
        }
        let word = m.get(3)?;
        Some(word.start()..word.end())
    })
}
fn load_carried(s: &str) -> bool {
    static CLAUSE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"[.!?,;:]+|\bbut\b|\bthough\b").unwrap());
    let s = s
        .to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    CLAUSE
        .split(s.trim_matches(['\'', '"', ' ']))
        .map(str::trim)
        .any(|clause| {
            if clause.is_empty()
                || ASKS_WHY_IT_IS
                    .find(clause)
                    .ok()
                    .flatten()
                    .is_some_and(|m| m.start() == 0)
                || hit(&DOES_ALL_THEY_CAN, clause)
                || hit(&WORK_SCOPE, clause) && !hit(&HOME_SCOPE, clause)
                || hit(&DOES_IT_RIGHT, clause)
            {
                return false;
            }
            if [
                &*RUNS_MYSELF_RAGGED,
                &RUNS_THE_HOUSE,
                &HELPS_ME_NOT,
                &LEANS_ON_THEM,
                &DOES_IT_ALL,
                &NO_TIME_OFF,
                &ALL_ON_THEM,
            ]
            .iter()
            .any(|r| hit(r, clause))
            {
                return true;
            }
            NO_ONE_HELPS.find(clause).ok().flatten().is_some_and(|m| {
                let head = &clause[..m.start()];
                hit(&FIRST_PERSON_ANYWHERE, head) && !hit(&THIRD_PERSON, head)
            }) || hit(&THE_ONE_WHO, clause) && hit(&EVERYTHING_AT_ONCE, clause)
        })
}
#[derive(serde::Serialize)]
pub(crate) struct Details {
    warmth: Option<String>,
    own_output: Option<usize>,
    tight_on_money: bool,
    amount_left: bool,
    a_load_carried: bool,
    stand_in: Option<StandIn>,
    keywords: Vec<String>,
    background: Vec<String>,
}
pub(crate) fn inspect(reading: &Reading<'_>) -> Details {
    let line = &reading.line;
    let n = &line.norm;
    let (bg, _) = topics::background(n);
    Details {
        warmth: topics::warmth(n).map(str::to_owned),
        own_output: topics::output_position(n),
        tight_on_money: reading.kind != super::kinds::Kind::Question
            && n.len() <= 10
            && topics::tight_money(n, &line.text).is_some(),
        amount_left: topics::purse(n, &line.text).is_some(),
        a_load_carried: load_carried(&line.clean),
        stand_in: stand_in(reading.raw).map(|mut found| {
            found.noun.clone_from(&reading.stand_in_noun);
            found
        }),
        keywords: keywords(n).map(str::to_owned).collect(),
        background: n
            .iter()
            .zip(bg)
            .filter(|(_, b)| *b)
            .map(|(s, _)| s.clone())
            .collect(),
    }
}

/// Read the statement under a courtesy when it carries a subject and still needs an answer.
pub(super) fn after_courtesy(line: &super::line::Line) -> Option<super::line::Line> {
    use super::{entities, kinds, numbers, topics};
    let tail = kinds::courtesy_tail(&line.norm)?;
    if tail.len() < 2 {
        return None;
    }
    let head = tail[0].as_str();
    let some = |set: &[&str]| tail.iter().any(|w| set.binary_search(&w.as_str()).is_ok());
    if THANKED_FOR.contains(&head)
        || ADDRESS_WORDS.contains(&head)
        || some(PRAISED)
        || kinds::courtesy_signoff(tail)
    {
        return None;
    }
    let trouble = kinds::courtesy_trouble(tail);
    if head == "i" && DID_IT.contains(&tail[1].as_str()) && !trouble {
        return None;
    }
    if !(trouble
        || some(AT_IT_STILL)
        || PERSON_HEADS.contains(&head)
        || POSSESSIVE.contains(&head) && PERSON_NOUNS.contains(&tail[1].as_str()))
    {
        return None;
    }
    let rest = super::line::normalise(&tail.join(" "));
    let found = topics::rank_for_ask(
        &rest,
        &entities::detect(&rest),
        &numbers::detect(&rest),
        None,
        Some(false),
    );
    found
        .0
        .first()
        .is_some_and(|(_, score)| *score >= 3)
        .then_some(rest)
}

pub(crate) fn keywords(words: &[String]) -> impl Iterator<Item = &str> + Clone {
    words
        .iter()
        .map(String::as_str)
        .filter(|word| {
            STOPWORDS.binary_search(word).is_err()
                && (word.chars().count() > 1 || word.chars().all(char::is_numeric))
        })
        .take(14)
}
