//! Help with a decision stated as an object, role, or event.

use crate::{
    bot::{Memory, Reading, modules::greeting},
    text::is_space,
};
use regex::Regex;
use std::sync::LazyLock;

#[derive(Clone, Copy)]
enum Kind {
    Bought,
    Job,
    Event,
}

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

include!("a_thing_being_weighed_data.rs");

pattern!(THAN, r"\bthan\b");
const EDGES: &[char] = &[' ', '.', ',', '!', '?', '-'];

fn plain(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn bare(phrase: &str) -> &str {
    let phrase = phrase.trim_matches(EDGES);
    for opener in OPENERS {
        if let Some(rest) = phrase
            .strip_prefix(opener)
            .and_then(|rest| rest.strip_prefix(' '))
        {
            return rest.trim_matches([' ', '.', ',', '!', '-']);
        }
    }
    phrase.trim_matches([' ', '.', ',', '!', '-'])
}

fn cut_at<'a>(phrase: &'a str, cuts: &[&str]) -> &'a str {
    let surrounded = format!(" {phrase} ");
    cuts.iter()
        .filter_map(|cut| surrounded.find(&format!(" {cut} ")))
        .min()
        .map_or(phrase, |cut| phrase[..cut].trim())
}

fn head_and_name(thing: &str) -> Option<(&str, &str, &str)> {
    let raw = thing.trim_matches(EDGES);
    let head_part = cut_at(bare(raw), CUTS_HEAD);
    let head = head_part.split(' ').next_back()?.trim_matches(EDGES);
    let name = cut_at(raw, CUTS_NAME);
    (!head.is_empty() && !name.is_empty()).then_some((head, name, head_part))
}

fn numeric(word: &str) -> bool {
    !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_digit())
}

fn plural(word: &str) -> bool {
    word.ends_with('s') && !word.ends_with("ss")
}

fn as_said(name: &str, head: &str) -> Option<String> {
    let first = name.split(' ').next()?;
    if matches!(first, "a" | "an") || POSSESSIVE.contains(&first) || numeric(first) {
        return Some(name.to_owned());
    }
    let name = if matches!(first, "the" | "some" | "any") {
        name.split_once(' ')?.1
    } else {
        name
    };
    let first = name.split(' ').next()?;
    if POSSESSIVE.contains(&first) || numeric(first) || plural(head) {
        return Some(name.to_owned());
    }
    let said = bare(name);
    let first = said.split(' ').next()?;
    let starts_digit = said.as_bytes().first().is_some_and(u8::is_ascii_digit);
    if said.is_empty() || starts_digit || said.split(' ').next_back().is_some_and(plural) {
        return Some(said.to_owned());
    }
    let vowel = said.starts_with(['a', 'e', 'i', 'o', 'u']);
    Some(if vowel && !A_NOT_AN.contains(&first) {
        format!("an {said}")
    } else {
        format!("a {said}")
    })
}

fn read(said: &str) -> Option<(Kind, &str)> {
    if said.is_empty() || ASKS.is_match(said) || NOT_A_DECISION.is_match(said) {
        return None;
    }
    let captured = WEIGH
        .captures(said)
        .or_else(|| DECIDING.captures(said))
        .or_else(|| LOOKING_FOR.captures(said))
        .or_else(|| ITS_FOR.captures(said))?;
    let thing = captured
        .name("thing")
        .or_else(|| captured.name("bare"))?
        .as_str();
    if THAN.is_match(thing) {
        return None;
    }
    let (head, name, head_part) = head_and_name(thing)?;
    if name.split(' ').count() > 6
        || head.ends_with("ing")
        || name
            .split(' ')
            .next()
            .is_some_and(|word| word.ends_with("ing"))
    {
        return None;
    }
    let index = KINDS.binary_search_by_key(&head, |&(word, _)| word).ok()?;
    let kind = KINDS[index].1;
    if matches!(kind, Kind::Job | Kind::Bought) && ALREADY_HAVE.is_match(head_part) {
        return None;
    }
    Some((kind, name))
}

fn pool(raw: &str) -> Option<Vec<String>> {
    let line = plain(raw);
    let (kind, thing) = read(&line)?;
    let (head, name, _) = head_and_name(thing)?;
    let said = as_said(name, head)?;
    let itself = if plural(head) { "them" } else { "it" };
    let options = match kind {
        Kind::Bought => BOUGHT,
        Kind::Job => JOB,
        Kind::Event => EVENT,
    };
    Some(
        options
            .iter()
            .map(|reply| reply.replace("{thing}", &said).replace("{it}", itself))
            .collect(),
    )
}

fn pick(options: &[String], memory: &Memory, raw: &str) -> String {
    let choices: Vec<_> = options.iter().map(String::as_str).collect();
    super::pick(&choices, memory, raw).to_owned()
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let options = pool(reading.raw)?;
    let reply = greeting::respond(reading, memory)?;
    super::fallback::is_neutral(&reply).then(|| pick(&options, memory, reading.raw))
}

#[cfg(test)]
#[path = "a_thing_being_weighed_tests.rs"]
mod tests;
