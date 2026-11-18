//! The source drink table's descriptions, serving matches, and comparisons.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::{DRINK_DIFFERENCES, DRINK_NOT_A_TASTE, DRINK_OWN_CUP, DRINK_STRENGTH, DRINKS};

#[derive(Clone, Copy)]
pub(super) enum Ask {
    Taste,
    Strength,
    Definition,
    Pairing,
    Difference,
}

pub(super) struct Drink {
    pub(super) names: &'static [&'static str],
    pub(super) definition: &'static str,
    pub(super) taste: &'static str,
    pub(super) pairing: &'static str,
}

static WORD_BREAK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9' ]+").unwrap());

fn padded_position(text: &str, phrase: &str) -> Option<usize> {
    text.match_indices(phrase).find_map(|(start, _)| {
        let left = text[..start].ends_with(' ');
        let right = text[start + phrase.len()..].starts_with(' ');
        (left && right).then_some(start)
    })
}

fn drink_in(line: &str) -> Option<&'static Drink> {
    if DRINK_NOT_A_TASTE
        .iter()
        .chain(DRINK_OWN_CUP)
        .any(|phrase| padded_position(line, phrase).is_some())
    {
        return None;
    }
    DRINKS
        .iter()
        .enumerate()
        .flat_map(|(index, drink)| drink.names.iter().map(move |&name| (name, index, drink)))
        .filter(|(name, _, _)| padded_position(line, name).is_some())
        .max_by_key(|(name, index, _)| (name.len(), std::cmp::Reverse(*index)))
        .map(|(_, _, drink)| drink)
}

fn difference(line: &str) -> Option<Cow<'static, str>> {
    let mut found: Vec<_> = DRINKS
        .iter()
        .filter_map(|drink| {
            let position = drink
                .names
                .iter()
                .find_map(|name| padded_position(line, name))?;
            Some((position, drink))
        })
        .collect();
    found.sort_by_key(|(position, drink)| (*position, drink.names[0]));
    let [(_, first), (_, second), ..] = found.as_slice() else {
        return None;
    };
    if let Some((_, reply)) = DRINK_DIFFERENCES
        .iter()
        .find(|(names, _)| names.contains(&first.names[0]) && names.contains(&second.names[0]))
    {
        return Some(Cow::Borrowed(reply));
    }
    Some(format!("{} {}", first.definition, second.definition).into())
}

pub(super) fn answer(ask: Ask, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
    .to_lowercase();
    let padded = format!(" {said} ");
    let words = WORD_BREAK.replace_all(&padded, " ");
    if matches!(ask, Ask::Difference) {
        return difference(&words);
    }
    let drink = drink_in(&words)?;
    let reply = match ask {
        Ask::Taste => drink.taste,
        Ask::Definition if padded.contains(" like ") => drink.taste,
        Ask::Definition => drink.definition,
        Ask::Pairing => drink.pairing,
        Ask::Strength => {
            let named = drink.names.iter().find(|name| said.contains(*name))?;
            let strength = |name| {
                DRINK_STRENGTH
                    .iter()
                    .find_map(|&(key, reply)| (key == name).then_some(reply))
                    .flatten()
            };
            strength(*named).or_else(|| strength(drink.names[0]))?
        }
        Ask::Difference => unreachable!(),
    };
    Some(Cow::Borrowed(reply))
}
