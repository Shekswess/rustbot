//! A filed trip and the source line it came from.

use fancy_regex::{Captures, Match, Regex};

use crate::{bot::Memory, text::is_space};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "travel_trip_data.rs"]
mod data;
use data::*;

pattern!(WORD, r"[a-z'’]+");
pattern!(PAST, r"\bwent\b|\bhave been\b|\bwas in\b|\blast year\b");
pattern!(
    CONFIRMS_ONESELF,
    r"(?i)^(?:(?:sorry|hey|hi|ok|okay|so|but|and|well|quick one)[, ]+)?(?:am|was|do|have) i\s+"
);
pattern!(
    QUESTION,
    r"\b(?:when|where|how|what|why|should|shall|can|could|would|will|worth)\b"
);
pattern!(STATE_SUBJECT, r"\b(?:we're|were|we are|i'm|im|i am)\b");
pattern!(ARTICLE, r"^(?:the|a|an)\s+");
pattern!(BOOK, r"(?i)\bbook");
pattern!(
    ASK_TAIL,
    r"(?i),\s*(?:and\s+)?(?:how|what|when|where|who|why|which|whose|should|shall|can|could|do|does|did|is|are|was|were|will|would|am)\b"
);
pattern!(
    NAME_AT_FRONT,
    r"(?i)^(?:i'?m|i am|my name is|call me)\s+([a-z][a-z'-]+)\s*[,;]"
);

pub(crate) struct Trip<'a> {
    pub(crate) plan: Option<&'a str>,
    pub(crate) origin: String,
    pub(crate) place: String,
    pub(crate) month: Option<String>,
}

fn matched<'a>(pattern: &Regex, word: &'a str) -> Option<Match<'a>> {
    pattern.find(word).ok().flatten()
}

fn finds<'a>(pattern: &Regex, word: &'a str) -> Option<Captures<'a>> {
    pattern.captures(word).ok().flatten()
}

fn test(pattern: &Regex, line: &str) -> bool {
    pattern.is_match(line).unwrap_or_default()
}

fn begins(pattern: &Regex, line: &str) -> bool {
    matched(pattern, line).is_some_and(|found| found.start() == 0)
}

fn trim(value: &str) -> &str {
    value.trim_matches([' ', ',', '.', ';', ':', '!', '?'])
}

fn word(value: &str) -> &str {
    value.trim_matches([' ', ',', '.', ';', ':', '!', '?', '\'', '’'])
}

fn flat(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn low(text: &str) -> String {
    flat(text).to_lowercase()
}

fn upper_head(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn not_a_destination(words: &[&str], place: &str) -> bool {
    let mut name = words
        .iter()
        .copied()
        .filter(|text| !word(text).is_empty())
        .map(word);
    let Some(first) = name.next() else {
        return false;
    };
    let first = if BELONGS_TO_THE_PLACE.contains(&first) {
        name.next().unwrap_or_default()
    } else {
        first
    };
    if DOING_AFTER_TO.contains(&first)
        || DOING.contains(&first)
        || name
            .next()
            .is_some_and(|second| DETERMINER.contains(&second) || PARTICLE.contains(&second))
    {
        return true;
    }
    let words = place
        .split_whitespace()
        .map(word)
        .filter(|word| !word.is_empty());
    words
        .skip_while(|word| BELONGS_TO_THE_PLACE.contains(word))
        .take(1)
        .any(|word| ORDINARY.contains(&word))
}

fn an_occasion(line: &str) -> bool {
    if !test(&ACK_DATE_AHEAD, line)
        || test(&ACK_OBLIGED, line)
        || WORD
            .find_iter(line)
            .flatten()
            .any(|found| ACK_PAST_PLAN.contains(&found.as_str()))
        || ["my ", "her ", "his ", "their ", "she ", "he "]
            .iter()
            .any(|head| line.starts_with(head))
        || !test(&ACK_TRIP_VERB, line)
    {
        return false;
    }
    let destination = finds(&ACK_DESTINATION, line).or_else(|| {
        test(&ACK_ARRIVE_VERB, line)
            .then(|| finds(&ACK_DESTINATION_ARRIVAL, line))
            .flatten()
    });
    let Some(destination) = destination else {
        return false;
    };
    if ACK_NOT_A_TRIP.contains(&&destination[1]) {
        return false;
    }
    let start = destination.get(0).unwrap().start();
    let name: Vec<_> = WORD
        .find_iter(&line[start..])
        .flatten()
        .skip(1)
        .take(4)
        .map(|word| word.as_str())
        .collect();
    !not_a_destination(&name, name.first().copied().unwrap_or_default())
        && !name.iter().any(|word| ACK_NOT_A_TRIP.contains(word))
        && (!name.contains(&"shower") || name.iter().any(|word| ACK_SHOWER.contains(word)))
        && name.iter().any(|word| ACK_OCCASIONS.contains(word))
}

fn booked_thing(text: &str, vocabulary: &[&str]) -> bool {
    text.split_whitespace()
        .find(|word| !matches!(*word, "a" | "an" | "the" | "my" | "our" | "his" | "her"))
        .is_some_and(|word| vocabulary.contains(&word))
}

fn first_capture<'a>(pattern: &Regex, line: &'a str) -> Option<&'a str> {
    finds(pattern, line)?.get(1).map(|found| found.as_str())
}

fn location(line: &str, visiting: bool) -> Option<String> {
    if let Some(booked) = matched(&BOOKED, line)
        && booked_thing(&low(&line[booked.end()..]), BOOKED_THING)
    {
        return None;
    }
    for pattern in [
        &*TO_PLACE,
        &*BOOKED_IN_PLACE,
        &*BOOKED_IN_WHEN,
        &*BOOKED_PLACE,
        &*IN_PLACE,
    ] {
        let Some(named) = first_capture(pattern, line) else {
            continue;
        };
        let words: Vec<_> = named
            .split(is_space)
            .filter(|word| !word.is_empty())
            .collect();
        let low_words: Vec<_> = words.iter().map(|word| word.to_lowercase()).collect();
        let mut kept = Vec::new();
        for (word, low) in words.iter().zip(&low_words) {
            let bare = trim(low);
            if kept.is_empty() && matches!(bare, "the" | "a" | "an") {
                kept.push(*word);
                continue;
            }
            let stops = MONTHS.contains(&bare)
                || DAYS.contains(&bare)
                || SEASON.contains(&bare)
                || STOP_AFTER.contains(&bare)
                || if kept.is_empty() {
                    NOT_A_PLACE.contains(&bare)
                } else {
                    matches!(bare, "with" | "and" | "for" | "next" | "this")
                };
            if stops {
                break;
            }
            kept.push(*word);
        }
        let name = kept.join(" ");
        let name = trim(&name);
        let low = name.to_lowercase();
        let head = low
            .split_whitespace()
            .find(|word| !matches!(*word, "the" | "a" | "an"));
        let full: Vec<_> = low_words.iter().map(String::as_str).collect();
        let bare = ARTICLE.replace(&low, "");
        if head.is_some_and(|head| TIME_WORD.contains(&head) || begins(&DURATION, &low))
            || begins(&NOT_A_TRIP, &low)
            || not_a_destination(&full, &low)
            || low.chars().count() < 3
            || matches!(low.as_str(), "the" | "a" | "an")
            || NOT_A_PLACE.contains(&low.as_str())
            || MONTHS.contains(&low.as_str())
            || DAYS.contains(&low.as_str())
            || low.split_whitespace().any(|part| SEASON.contains(&part))
            || begins(&A_CARING_PLACE_HEAD, &low)
            || NOT_A_PLACE.contains(&bare.as_ref())
            || MONTHS.contains(&bare.as_ref())
            || DAYS.contains(&bare.as_ref())
            || !visiting && relation(&low)
            || test(&BOOK, line)
                && (booked_thing(&low, BOOKED_THING) || booked_thing(&low, STAY_THING))
        {
            continue;
        }
        return Some(name.to_owned());
    }
    None
}

fn relation(place: &str) -> bool {
    if test(&THEIR_PERSON, place) {
        return true;
    }
    let Some(value) = finds(&A_PERSON_IS_NOT_A_PLACE, place) else {
        return false;
    };
    let rest = &place[value.get(0).unwrap().end()..];
    rest.split_whitespace()
        .next()
        .is_none_or(|next| NOT_A_THING_OF_THEIRS.contains(&next))
}

fn visiting(line: &str) -> Option<String> {
    if test(&IN_A_CARING_PLACE, line) {
        return None;
    }
    if let Some(person) = finds(&VISIT_TO, line) {
        let before = &line[..person.get(0)?.start()];
        if test(&GOING_BEFORE, before) || test(&FILLER_BEFORE, before) {
            let relation = low(&person["person"]);
            let relation = if relation.starts_with("in") {
                "in-laws"
            } else {
                &relation
            };
            let who = format!("{} {relation}", person["own"].to_lowercase());
            return Some(person.name("thing").map_or(who.clone(), |thing| {
                format!("{who} {}", thing.as_str().to_lowercase())
            }));
        }
    }
    location(line, true).filter(|place| relation(&place.to_lowercase()))
}

pub(in crate::bot) fn named_month(text: &str) -> Option<String> {
    let low = low(text);
    WORD.find_iter(&low)
        .flatten()
        .find(|word| MONTHS.contains(&word.as_str()))
        .map(|word| word.as_str().to_owned())
}

fn when_phrase(text: &str) -> Option<String> {
    matched(&A_WHEN_PHRASE, text)
        .map(|found| low(found.as_str()))
        .or_else(|| first_capture(&BARE_DAY, text).map(str::to_lowercase))
}

fn read<'a>(plan: Option<&'a str>, text: &str) -> Option<Trip<'a>> {
    let line = text.trim_matches(is_space);
    if line.is_empty() || line.ends_with('?') {
        return None;
    }
    let said = low(line);
    if an_occasion(&said)
        || test(&EVERYDAY_TRIP, &said)
        || test(&ON_A, &said)
        || test(&PAST, &said)
        || test(&QUESTION, &said) && !test(&STATE_SUBJECT, &said)
    {
        return None;
    }
    let visit = visiting(line);
    let place = match visit {
        Some(who) => who,
        None => {
            if !test(&GOING, &said) && !test(&GOING_BARE, &said) {
                return None;
            }
            if let Some(intro) = first_capture(&NAME_AT_FRONT, line)
                && super::valid_name(intro, false)
            {
                return None;
            }
            location(line, false)?
        }
    };
    Some(Trip {
        plan,
        origin: line.to_owned(),
        place,
        month: named_month(&said),
    })
}

pub(crate) fn in_hand(memory: &Memory) -> Option<Trip<'_>> {
    in_hand_before(memory, "")
}

pub(in crate::bot) fn in_hand_before<'a>(memory: &'a Memory, statement: &str) -> Option<Trip<'a>> {
    let omitted = flat(statement);
    for (plan, origin) in memory
        .stated_plans
        .trip_origins()
        .rev()
        .filter(|(_, origin)| omitted.is_empty() || flat(origin) != omitted)
        .take(3)
    {
        let mut source = flat(origin);
        if let Some(ask) = matched(&ASK_TAIL, &source) {
            source.truncate(ask.start());
        }
        let source = trim(&source);
        if let Some(trip) = read(Some(plan), source) {
            return Some(trip);
        }
    }
    memory
        .turns
        .iter()
        .rev()
        .take(4)
        .find_map(|turn| read(None, &turn.user))
}

pub(in crate::bot) fn display_place(place: &str) -> Option<String> {
    super::personal::place(place)
}

pub(in crate::bot) fn booking_to_file(text: &str) -> Option<String> {
    if test(&CONFIRMS_ONESELF, &flat(text)) {
        return None;
    }
    booked_trip(text)
}

fn booked_trip(text: &str) -> Option<String> {
    if !test(&BOOKED, text) {
        return None;
    }
    let place = upper_head(&location(text, false)?);
    Some(
        named_month(text)
            .or_else(|| when_phrase(text))
            .map_or_else(|| place.clone(), |when| format!("{place} in {when}")),
    )
}
