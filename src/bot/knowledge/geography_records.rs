//! Registry callbacks for world and regional records.

use super::geography::{borders, continent, countries, eu, region};
use crate::bot::{Reading, understanding::kinds::Kind};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "geography_records_data.rs"]
mod data;
use data::*;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    NAMED_PLACE,
    r"\b(?:in|of|make up|makes up|making up)\s+(?:the\s+)?([a-z']+)"
);
pattern!(
    ORDINAL,
    r"\b(?:second|third|fourth|fifth|2nd|3rd|4th|5th)\b"
);
pattern!(RECORD_LIST, LIST_PATTERN);
pattern!(TAIL_HAVE, r"\s+(?:have|has)\s*[?.!]*$");
pattern!(
    SUBSET,
    r"\b(?:have|has|with|without|allow|allows|legal|banned?|bans|speak|speaks|use|uses|need|needs)\b"
);
pattern!(
    RIVER_LENGTH_ASK,
    r"\b(how long|how far|long is|length|km|kilometres|kilometers|miles|distance)\b"
);

const WORLD_OR_UNIT: &[&str] = &[
    "world",
    "earth",
    "planet",
    "all",
    "them",
    "these",
    "those",
    "feet",
    "foot",
    "metres",
    "meters",
    "metre",
    "meter",
    "kilometres",
    "kilometers",
    "km",
    "miles",
    "mile",
    "centimetres",
    "centimeters",
    "cm",
    "inches",
    "inch",
    "total",
];

fn names(text: &str, phrase: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    text.match_indices(phrase).any(|(at, found)| {
        !text[..at].chars().next_back().is_some_and(word)
            && !text[at + found.len()..].chars().next().is_some_and(word)
    })
}

fn has(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| names(text, phrase))
}

fn world(text: &str, answer: &'static str) -> Option<&'static str> {
    NAMED_PLACE
        .captures(text)
        .is_none_or(|place| WORLD_OR_UNIT.contains(&&place[1]))
        .then_some(answer)
}

fn region_name(text: &str) -> Option<&'static str> {
    REGION_NAMES
        .iter()
        .find_map(|&(form, id)| names(text, form).then_some(id))
        .or_else(|| PLACES.iter().copied().find(|place| names(text, place)))
}

type RegionFacts = &'static [(&'static str, &'static str)];
type CountryRecord = (&'static str, RegionFacts, &'static [&'static str]);

fn regional_row<T: Copy>(
    text: &str,
    rows: impl Clone + Iterator<Item = (&'static str, T)>,
) -> Option<T> {
    if let Some(key) = region_name(text)
        && let Some((_, found)) = rows.clone().find(|&(name, _)| name == key)
    {
        return Some(found);
    }
    rows.filter(|&(key, _)| !PLACES.contains(&key) && names(text, key))
        .fold(None::<(&str, T)>, |best, row| {
            if best.is_none_or(|(name, _)| row.0.len() > name.len()) {
                Some(row)
            } else {
                best
            }
        })
        .map(|(_, row)| row)
}

fn field(facts: RegionFacts, key: &str) -> Option<&'static str> {
    facts
        .iter()
        .find_map(|&(kind, response)| (kind == key).then_some(response))
}

fn regional_record(text: &str, kind: &str, fallback: &'static str) -> Option<&'static str> {
    match regional_row(text, REGION_FACTS.iter().copied()) {
        Some(facts) => field(facts, kind),
        None => world(text, fallback),
    }
}

fn country_row(text: &str) -> Option<(RegionFacts, &'static [&'static str])> {
    regional_row(
        text,
        COUNTRY_FACTS
            .iter()
            .map(|&(name, facts, people)| (name, (facts, people))),
    )
}

fn ordinal(text: &str) -> Option<(usize, &'static str)> {
    Some(match ORDINAL.find(text)?.as_str() {
        "second" | "2nd" => (2, "second"),
        "third" | "3rd" => (3, "third"),
        "fourth" | "4th" => (4, "fourth"),
        "fifth" | "5th" => (5, "fifth"),
        _ => return None,
    })
}

fn country_record(text: &str, kind: &str, fallback: &'static str) -> Cow<'static, str> {
    let Some((facts, people)) = country_row(text) else {
        return Cow::Borrowed(fallback);
    };
    if let Some((rank, order)) = ordinal(text)
        && has(text, RECORD_POPULOUS)
        && let Some(&name) = people.get(rank - 1)
        && let [first, second, third, ..] = people
    {
        let region = region_name(text)
            .unwrap_or("")
            .split_whitespace()
            .map(|word| {
                let mut letters = word.chars();
                letters
                    .next()
                    .into_iter()
                    .flat_map(char::to_uppercase)
                    .chain(letters)
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ");
        return Cow::Owned(format!(
            "{name} has the {order} most people in {region}: {first} is first, then {second} and {third}."
        ));
    }
    let smallest = has(text, RECORD_SMALLEST) || kind == "small";
    let key = if ORDINAL.is_match(text) || RECORD_LIST.is_match(text) {
        if smallest { "bottom" } else { "top" }
    } else if smallest {
        "small"
    } else {
        kind
    };
    Cow::Borrowed(field(facts, key).unwrap_or(fallback))
}

fn population_record(text: &str) -> Option<&'static str> {
    let (facts, _) = country_row(text)?;
    let populous = has(text, RECORD_POPULOUS);
    if has(text, LEAST) || (has(text, RECORD_SMALLEST) && populous) {
        return field(facts, "least");
    }
    if populous {
        return field(facts, "populous");
    }
    None
}

fn from_table(text: &str, rows: &[(&str, &'static str)]) -> Option<&'static str> {
    rows.iter()
        .find_map(|&(name, reply)| names(text, name).then_some(reply))
}

pattern!(PART_OF, r"\bpart of\b");
pattern!(NATION, UK_NATION);

fn uk_part(text: &str, is_question: bool) -> Option<&'static str> {
    let says = |phrase| {
        text.match_indices(phrase).any(|(at, found)| {
            !text[..at]
                .bytes()
                .next_back()
                .is_some_and(|c| c.is_ascii_lowercase())
                && !text
                    .as_bytes()
                    .get(at + found.len())
                    .is_some_and(u8::is_ascii_lowercase)
        })
    };
    if ["british isles", "difference", "same", "versus", " vs "]
        .into_iter()
        .any(says)
    {
        return Some(UK_COMPARISON);
    }
    if !is_question {
        return None;
    }
    if let Some(position) = PART_OF.find(text)
        && let Some(first) = NATION.find_iter(&text[..position.start()]).last()
        && let Some(second) = NATION.find(&text[position.end()..])
        && let Some(reply) = UK_PAIRS.iter().find_map(|&(a, b, reply)| {
            (a == first.as_str() && b == second.as_str()).then_some(reply)
        })
    {
        return Some(reply);
    }
    let (_, choices) = UK_PARTS.iter().find(|(part, _)| says(part))?;
    let kind = if ["a country", "independent country", "sovereign"]
        .into_iter()
        .any(says)
    {
        1
    } else if ["independent", "separate country"].into_iter().any(says) {
        2
    } else {
        0
    };
    Some(choices[kind])
}

pub(crate) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    if let Some(&(_, response)) = WORLD.iter().find(|&&(entry, _)| entry == id) {
        return world(text, response).map(Cow::Borrowed);
    }
    if let Some(&(_, kind, response)) = REGION_RECORD.iter().find(|&&(entry, _, _)| entry == id) {
        return regional_record(text, kind, response).map(Cow::Borrowed);
    }
    if let Some(&(_, kind, response)) = COUNTRY_RECORD.iter().find(|&&(entry, _, _)| entry == id) {
        return Some(country_record(text, kind, response));
    }
    let response = match id {
        "continent-record" => population_record(text),
        "uk-parts" => uk_part(text, reading.kind == Kind::Question),
        "continent-count" => {
            let raw = reading.raw.to_lowercase();
            (!SUBSET.is_match(&TAIL_HAVE.replace(&raw, "")))
                .then(|| from_table(&raw, CONTINENT_COUNTS))
                .flatten()
        }
        "named-peak" => from_table(text, PEAKS),
        "highest-in-place" => from_table(text, HIGHEST),
        "river-length" => {
            let raw = reading.raw.to_lowercase();
            if !RIVER_LENGTH_ASK.is_match(&raw) {
                return None;
            }
            RIVERS.iter().find_map(|&(river, answer)| {
                (names(&raw, river)
                    && (!RIVER_NAME_NEEDS_RIVER.contains(&river) || names(&raw, "river")))
                .then_some(answer)
            })
        }
        "country-borders" | "country-continent" | "eu-membership" | "continent-countries"
            if reading.kind == Kind::Question =>
        {
            let reply = match id {
                "country-borders" => borders(text, &countries(text)),
                "country-continent" => continent(text, countries(text).first()?),
                "eu-membership" => Some(eu(text, countries(text).first())),
                "continent-countries" => {
                    let place = region(text)?;
                    super::geography::region_countries(place).map(str::to_owned)
                }
                _ => None,
            };
            return reply.map(Cow::Owned);
        }
        _ => None,
    };
    response.map(Cow::Borrowed)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod data {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/geography_records.rs"
        ));
    }

    #[test]
    fn the_records_match_source_callbacks() {
        let differences: Vec<_> = data::CASES
            .iter()
            .filter_map(|&(id, prompt, expected)| {
                let got = registered(id, &Reading::new(prompt));
                (got.as_deref() != expected).then_some((id, prompt, expected, got))
            })
            .collect();
        assert!(
            differences.is_empty(),
            "{} differences: {:?}",
            differences.len(),
            &differences[..differences.len().min(18)]
        );
    }
}
