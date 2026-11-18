//! Ask which of the common interpretations fits a thing named as the trouble.

use crate::bot::{Memory, Reading, compose, modules::greeting};
use fancy_regex::Regex;
use std::sync::LazyLock;

#[path = "trouble_named_data.rs"]
mod data;
use data::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Family {
    Vehicle,
    Connection,
    Bill,
    Supply,
    Heating,
    Water,
}

const FAMILIES: &[(Family, &[&str], &[&str])] = &[
    (Family::Vehicle, VEHICLE, VEHICLE_REPLIES),
    (Family::Connection, CONNECTION, CONNECTION_REPLIES),
    (Family::Bill, BILL, BILL_REPLIES),
    (Family::Supply, SUPPLY, SUPPLY_REPLIES),
    (Family::Heating, HEATING, HEATING_REPLIES),
    (Family::Water, WATER, WATER_REPLIES),
];

fn family(clean: &str) -> Option<Family> {
    if clean.is_empty() || clean.contains('?') {
        return None;
    }
    let capture = A_THE_TROUBLE.captures(clean).ok().flatten()?;
    let matched = capture.get(0)?;
    let thing = capture
        .name("thing")
        .or_else(|| capture.name("thing2"))?
        .as_str()
        .trim();
    let rest = format!("{} {}", &clean[..matched.start()], &clean[matched.end()..]);
    if thing.is_empty()
        || super::matches(&A_ASK, clean)
        || super::words(&rest).any(|word| FILLER.binary_search(&word).is_err())
        || super::matches(&A_BROKEN_NOW, clean)
    {
        return None;
    }
    let words: Vec<_> = super::words(thing)
        .filter(|word| FILLER.binary_search(word).is_err())
        .collect();
    FAMILIES
        .iter()
        .find(|(_, vocabulary, _)| words.iter().any(|word| vocabulary.contains(word)))
        .map(|(family, _, _)| *family)
}

fn starts(pattern: &Regex, text: &str) -> bool {
    pattern
        .find(text)
        .ok()
        .flatten()
        .is_some_and(|matched| matched.start() == 0)
}

fn floor(reply: &str) -> bool {
    static SEPARATE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"[.!?;,—–]|\s-\s").unwrap());
    let shaped = super::flat(&reply.replace(['—', '–'], "-"));
    let parts: Vec<_> = SEPARATE
        .split(&shaped)
        .map(|part| part.trim_matches([' ', ',', '.', ';', ':', '!', '?', '—', '–', '-', '\t']))
        .filter(|part| !part.is_empty())
        .collect();
    !parts.is_empty()
        && parts.len() <= 4
        && parts.into_iter().all(|part| {
            starts(&FLOOR_COURTESY, part)
                || starts(&FLOOR_CLAUSE, part)
                    && (super::split(part).count() <= 8 || super::matches(&FLOOR_WAITING, part))
        })
}

fn a_receipt(reply: &str) -> bool {
    let line = super::flat(reply);
    if line.is_empty() {
        return false;
    }
    let frame = |reply: &str| {
        SAID_BACK.iter().any(|(front, back)| {
            reply
                .strip_prefix(front)
                .and_then(|rest| rest.strip_suffix(back))
                .is_some_and(|inside| {
                    !inside
                        .trim_matches([' ', '—', '-', '.', ':', ',', ';'])
                        .is_empty()
                })
        })
    };
    let spaced = SPACED_DASH.replace_all(&line, ", ");
    [line.as_str(), &spaced].into_iter().any(|variant| {
        frame(variant)
            || SENTENCE
                .find(variant)
                .ok()
                .flatten()
                .is_some_and(|stop| frame(&variant[..stop.start()]))
    })
}

pub(in crate::bot) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let family = family(&super::flat(reading.raw))?;
    let already = greeting::respond(reading, memory).unwrap_or_default();
    if !already.is_empty()
        && !compose::fallback::is_neutral(&already)
        && !a_receipt(&already)
        && !floor(&already)
        && !compose::routes::hedge(&already)
    {
        return None;
    }
    let (_, _, options) = FAMILIES.iter().find(|(group, _, _)| *group == family)?;
    Some(options[memory.turns.len() % options.len()].to_owned())
}
