//! People, places, titles and everyday things, in their original mention order.
#[path = "entity_data.rs"]
mod data;
use super::line::Line;
use data::*;
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

#[derive(Clone, Debug, serde::Serialize)]
pub(crate) struct Entity {
    pub(crate) text: String,
    #[serde(rename = "type")]
    pub(crate) kind: &'static str,
    pub(crate) at: usize,
}
pub(super) fn is_relation(word: &str) -> bool {
    has(RELATIONS, word)
}
pub(super) fn pointable(e: &Entity) -> bool {
    if e.kind != "name" {
        return true;
    }
    let word = e.text.to_lowercase();
    !word.contains('\'') && !has(COMMON_WORDS, &word)
}
fn has(set: &[&str], w: &str) -> bool {
    set.binary_search(&w).is_ok()
}
fn any(norm: &[String], set: &[&str]) -> bool {
    norm.iter().any(|w| has(set, w))
}
fn is_name(w: &str) -> bool {
    let lower = w.to_lowercase();
    if lower != "i" && has(COMMON_WORDS, &lower) {
        return false;
    }
    if let Some((stem, _)) = lower.split_once('\'')
        && (has(CORE_WORDS, stem) || stem.strip_suffix('n').is_some_and(|s| has(CORE_WORDS, s)))
    {
        return false;
    }
    lower == "i" || !has(CHAT, &lower)
}
fn capitalised(text: &str) -> Vec<String> {
    let mut runs = Vec::<Vec<&str>>::new();
    let mut run = Vec::new();
    for token in text.split_whitespace() {
        let word =
            token.trim_matches(&['.', ',', '!', '?', ';', ':', '"', '\'', '(', ')', '[', ']'][..]);
        if word.starts_with(char::is_uppercase)
            && word
                .chars()
                .all(|c| c.is_alphabetic() || matches!(c, '-' | '\''))
        {
            run.push(word)
        } else if !run.is_empty() {
            runs.push(std::mem::take(&mut run));
        }
    }
    if !run.is_empty() {
        runs.push(run);
    }
    runs.into_iter()
        .filter_map(|run| {
            if let [word] = run.as_slice() {
                return is_name(word).then(|| (*word).to_owned());
            }
            let first = run[0].to_lowercase();
            let start = usize::from(has(ARTICLES, &first) || has(LEADING, &first));
            let start = start
                + run[start..]
                    .iter()
                    .position(|w| is_name(w) || has(PLACES, &w.to_lowercase()))?;
            let end = run[start..]
                .iter()
                .position(|w| has(NAME_BREAKS, &w.to_lowercase()))
                .map_or(run.len(), |i| i + start);
            (end > start).then(|| run[start..end].join(" "))
        })
        .collect()
}
pub(super) fn names_a_person(norm: &[String], index: usize) -> bool {
    index
        .checked_sub(1)
        .is_some_and(|i| has(PERSON_WORDS, &norm[i]) || has(NAME_FRAMES, &norm[i]))
        || matches!(norm.get(index+1..index+4),Some([v,p,r]) if v=="is"&&p=="my"&&has(PERSON_WORDS,r))
}
pub(crate) fn person_names(norm: &[String]) -> Vec<&str> {
    let mut names = Vec::new();
    for (i, s) in norm.iter().enumerate() {
        if has(PLACES, s) && names_a_person(norm, i) && !names.contains(&s.as_str()) {
            names.push(s.as_str())
        }
    }
    names
}
fn places_name(norm: &[String], at: usize) -> bool {
    let Some(i) = at.checked_sub(1) else {
        return false;
    };
    let before = &norm[i];
    if has(PLACE_WORDS_WITH_CUE, before) {
        return norm.iter().any(|w| {
            has(TRAVEL_CUES, w) || w.strip_suffix('s').is_some_and(|s| has(TRAVEL_CUES, s))
        });
    }
    has(PLACE_WORDS, before) && (before != "at" || !any(norm, WORK_WORDS))
}
fn said_as_place(run: &str, norm: &[String]) -> bool {
    let first = run.split_whitespace().next().unwrap_or("").to_lowercase();
    if has(NOT_PLACES, &first) {
        return false;
    }
    norm.iter().position(|w| *w == first).is_some_and(|i| {
        places_name(norm, i)
            || i >= 2
                && has(PLACE_THING, &norm[i - 1])
                && has(PLACE_WORDS, &norm[i - 2])
                && (norm[i - 2] != "at" || !any(norm, WORK_WORDS))
    })
}
fn country_query(norm: &[String], i: usize) -> bool {
    if i > 0 && has(ARTICLES, &norm[i - 1]) {
        return false;
    }
    if any(norm, COUNTRY_CUES) {
        return true;
    }
    let country = |w: &str| has(PLACES, w) && !has(PLACE_HOMOGRAPHS, w);
    if matches!(norm.get(i+1..i+3),Some([prep,w]) if prep=="in"&&country(w)) {
        return true;
    }
    (i.saturating_sub(3)..(i + 4).min(norm.len()))
        .any(|j| j != i && country(&norm[j]) && any(&norm[i.min(j)..i.max(j)], COMPARE_WORDS))
}
fn thing(word: &str) -> bool {
    has(THINGS, word)
        || word.chars().count() > 3
            && (has(THINGS, word.trim_end_matches('s'))
                || word.strip_suffix("es").is_some_and(|s| has(THINGS, s))
                || word.strip_suffix("ies").is_some_and(|s| has(THINGS, s)))
}
pub(super) fn verdict(norm: &[String], at: usize) -> bool {
    if !has(VERDICT_NOUNS, &norm[at]) && !has(VERDICT_ADJECTIVES, &norm[at]) {
        return false;
    }
    let mut before = norm[..at]
        .iter()
        .rev()
        .skip_while(|s| has(VERDICT_FILLER, s));
    before.next().is_some_and(|v| has(COPULAS, v))
        && before.next().is_some_and(|s| {
            ![PRONOUN_SUBJECT, ASKING_SUBJECT, COPULAS, NOT_A_THING]
                .iter()
                .any(|set| has(set, s))
        })
}
fn small_talk(norm: &[String]) -> bool {
    !norm.is_empty()
        && norm.len() <= 10
        && SMALL_TALK_PHRASES.iter().any(|p| {
            let len = p.split_ascii_whitespace().count();
            norm.windows(len).enumerate().any(|(i, part)| {
                part.iter()
                    .map(String::as_str)
                    .eq(p.split_ascii_whitespace())
                    && norm[..i]
                        .iter()
                        .chain(&norm[i + len..])
                        .all(|s| has(SMALL_TALK_LEFTOVER, s))
            })
        })
}
fn child_age(norm: &[String], i: usize) -> bool {
    let Some([owner, number, unit, old]) = norm.get(i..i + 4) else {
        return false;
    };
    if old != "old" || !has(THE_PERSONS_OWN, owner) || has(AGE_AFTER, number) {
        return false;
    }
    let Ok(pos) = AGE_IN_A_YEAR.binary_search_by_key(&unit.as_str(), |x| x.0) else {
        return false;
    };
    if super::numbers::age(number) / AGE_IN_A_YEAR[pos].1 > 19.0 {
        return false;
    }
    norm.get(i + 4)
        .is_none_or(|w| has(AGE_OF_A_CHILD_WORDS, w) || !has(PETS, w) && !thing(w))
}
pub(crate) fn detect(line: &Line) -> Vec<Entity> {
    static TITLES: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"\*([^*]{2,60})\*|"([^"]{2,60})""#).unwrap());
    static MINE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"\bmy ([a-z]+)(?:\s+[a-z]+)?").unwrap());
    static CAPITAL_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\w+\b").unwrap());
    let norm = &line.norm;
    if small_talk(norm) {
        return Vec::new();
    }
    let mut seen = HashSet::new();
    let mut entities = Vec::new();
    let mut add = |name: &str, kind, at: Option<usize>, seen: &mut HashSet<String>| {
        let key = name.to_lowercase();
        if key.chars().count() > 1 && seen.insert(key.clone()) {
            let first = key.split_whitespace().next().unwrap_or("");
            entities.push(Entity {
                text: name.trim().to_owned(),
                kind,
                at: at.unwrap_or_else(|| {
                    norm.iter()
                        .position(|w| w.starts_with(first))
                        .unwrap_or(norm.len())
                }),
            });
        }
    };
    for m in TITLES.captures_iter(&line.text) {
        add(
            m.get(1).or(m.get(2)).unwrap().as_str(),
            "title",
            None,
            &mut seen,
        )
    }
    for m in MINE.captures_iter(&line.clean) {
        let word = &m[1];
        if has(RELATIONS, word) {
            add(&format!("my {word}"), "person", None, &mut seen);
            seen.insert(word.to_owned());
        }
    }
    for i in 0..norm.len() {
        if child_age(norm, i) {
            add(&norm[i..i + 4].join(" "), "person", Some(i), &mut seen)
        }
    }
    for run in capitalised(&line.text) {
        let lower = run.to_lowercase();
        if matches!(lower.as_str(), "i" | "i'm" | "im") {
            continue;
        }
        let place = has(PLACES, &lower)
            || has(PLACES, lower.split_whitespace().next().unwrap_or(""))
            || said_as_place(&run, norm);
        add(&run, if place { "place" } else { "name" }, None, &mut seen);
    }
    for (i, w) in norm.iter().enumerate() {
        if verdict(norm, i) {
            continue;
        }
        if has(PLACES, w) {
            if names_a_person(norm, i) {
                continue;
            }
            let caps = || {
                let mut title = w.clone();
                if let Some(first) = title.get_mut(..1) {
                    first.make_ascii_uppercase()
                }
                CAPITAL_WORD
                    .find_iter(&line.text)
                    .any(|v| v.as_str() == title)
            };
            if !has(PLACE_HOMOGRAPHS, w)
                || has(COUNTRY_HOMOGRAPHS, w) && country_query(norm, i)
                || places_name(norm, i)
                || caps()
            {
                add(w, "place", None, &mut seen);
                continue;
            }
        }
        if has(RELATIONS, w) {
            if i > 0
                && has(OWNERS, &norm[i - 1])
                && !line.clean.split_whitespace().any(|word| word == w)
            {
                add(
                    &format!("{} {w}", norm[i - 1]),
                    "person",
                    Some(i - 1),
                    &mut seen,
                );
                seen.insert(w.clone());
            } else {
                add(w, "person", None, &mut seen)
            }
        } else if thing(w) {
            add(w, "thing", None, &mut seen)
        }
    }
    entities.sort_by_key(|e| e.at);
    entities
}

pub(super) fn is_person_place(norm: &[String], i: usize) -> bool {
    has(PLACES, &norm[i]) && names_a_person(norm, i)
}
pub(super) fn child_age_positions(norm: &[String]) -> Vec<usize> {
    norm.windows(4)
        .enumerate()
        .filter_map(|(i, p)| {
            let index = AGE_IN_A_YEAR
                .binary_search_by_key(&p[2].as_str(), |x| x.0)
                .ok()?;
            (p[3] == "old"
                && !has(AGE_AFTER, &p[1])
                && super::topics::determiner(&p[0])
                && super::numbers::age(&p[1]) / AGE_IN_A_YEAR[index].1 <= 19.)
                .then_some(i + 1)
        })
        .collect()
}
