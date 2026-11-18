//! Reply to a plan with the question that fits a trip, visit, or appointment.

use crate::bot::Memory;
use crate::text::is_space;
use std::borrow::Cow;

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($value).expect(stringify!($name)));
    };
}

#[path = "ack_plan_data.rs"]
mod data;
use data::*;

fn words(line: &str) -> impl Iterator<Item = &str> + '_ {
    pattern!(WORD, r"[a-z']+");
    WORD.find_iter(line).map(|found| found.as_str())
}

fn has(terms: &[&str], word: &str) -> bool {
    terms.binary_search(&word).is_ok()
}

fn destination_is_a_doing(words: &[&str]) -> bool {
    let after_article = if words
        .first()
        .is_some_and(|word| has(BELONGS_TO_THE_PLACE, word))
    {
        &words[1..]
    } else {
        words
    };
    let Some((first, rest)) = after_article.split_first() else {
        return false;
    };
    has(DOING_AFTER_TO, first)
        || has(DOING, first)
        || rest
            .first()
            .is_some_and(|word| has(DETERMINER, word) || has(PARTICLE, word))
}

fn trip_pool(said: &str) -> Option<&'static [&'static str]> {
    if OBLIGED.is_match(said)
        || ["my ", "her ", "his ", "their ", "she ", "he "]
            .iter()
            .any(|head| said.starts_with(head))
        || !TRIP_VERB.is_match(said)
    {
        return None;
    }
    let destination = DESTINATION.captures(said).or_else(|| {
        ARRIVE_VERB
            .is_match(said)
            .then(|| DESTINATION_ARRIVAL.captures(said))
            .flatten()
    })?;
    if has(NOT_A_TRIP, &destination[1]) {
        return None;
    }
    pattern!(DESTINATION_WORD, r"[a-z'’]+");
    let start = destination.get(0)?.start();
    let after: Vec<_> = DESTINATION_WORD
        .find_iter(&said[start..])
        .skip(1)
        .take(4)
        .map(|found| found.as_str())
        .collect();
    let first = *after.first()?;
    let tidy: Vec<_> = after
        .iter()
        .map(|word| word.trim_matches([' ', ',', '.', ';', ':', '!', '?', '\'', '’']))
        .filter(|word| !word.is_empty())
        .collect();
    if destination_is_a_doing(&tidy)
        || has(ORDINARY, first.trim_matches(['\'', '’']))
        || after.iter().any(|word| has(NOT_A_TRIP, word))
    {
        return None;
    }
    if after.iter().any(|word| has(OCCASIONS, word)) {
        if after.contains(&"shower") && !after.iter().any(|word| has(SHOWER, word)) {
            return None;
        }
        return Some(if DREADED_OCCASION.is_match(said) {
            OCCASION_FLAT_TAIL
        } else {
            OCCASION_TAIL
        });
    }
    if has(THING_BEFORE, first) || ["'s", "’s", "s'"].iter().any(|end| first.ends_with(end)) {
        let names = &after[1..];
        if first != "the"
            || !(names.first().is_some_and(|word| has(PLACES_WITH_THE, word))
                || names.len() >= 2 && has(PLACES_WITH_THE, &names[..2].join(" ")))
        {
            return None;
        }
    }
    Some(TRIP_TAIL)
}

fn more_than_one_visitor(said: &str, at: usize, word: &str) -> bool {
    pattern!(PLURAL_WORD, r"\b(?:are|were|were both|are both)\b");
    pattern!(TOGETHER, r"\band (?:i|me|we|us)\b");
    pattern!(JOINED, r"\band\b|&");
    if has(PLURAL_KIN, word) {
        return true;
    }
    if word
        .strip_suffix('s')
        .is_some_and(|root| has(RELATIONS, root) && !has(RELATIONS, word))
    {
        return PLURAL_WORD.is_match(said);
    }
    let rest = &said[at..];
    let names = A_COMING_WORD
        .find(rest)
        .map_or(rest, |found| &rest[..found.start()]);
    !TOGETHER.is_match(names) && JOINED.is_match(names)
}

fn visitor(said: &str) -> Option<&'static str> {
    pattern!(RELATION, r"\b(?:my|our)\s+([a-z]+)");
    pattern!(
        AT_HOME,
        r"\b(?:coming|heading|getting|moving|flying) (?:back )?home\b"
    );
    let owned = RELATION.captures(said)?;
    let word = &owned[1];
    let person = [
        word,
        word.strip_suffix("'s").unwrap_or(word),
        &word[..word.len() - 1],
    ]
    .into_iter()
    .find(|word| has(RELATIONS, word))?;
    if !COMING_TO_STAY.is_match(said) || AT_HOME.is_match(said) {
        return None;
    }
    if more_than_one_visitor(said, owned.get(0)?.end(), word) {
        return Some("they");
    }
    match (has(SHE, person), has(HE, person)) {
        (true, false) => Some("she"),
        (false, true) => Some("he"),
        _ => Some("they"),
    }
}

fn diary_pool(said: &str) -> Option<(&'static [&'static str], Option<&'static str>)> {
    pattern!(MEETING, r"\b(?:meet|meeting|seeing)\b");
    if let Some(who) = visitor(said) {
        return Some(if who == "they" {
            (VISIT_TAIL_MANY, None)
        } else {
            (VISIT_TAIL_ONE, Some(who))
        });
    }
    let timed = DIARY_TIME.is_match(said);
    if DIARY_THING.is_match(said) {
        Some((if timed { DIARY_TIME_TAIL } else { DIARY_TAIL }, None))
    } else if MEETING.is_match(said) {
        Some((
            if timed {
                MEETING_TAIL
            } else {
                MEETING_TIME_TAIL
            },
            None,
        ))
    } else if words(said).any(|word| has(OCCASIONS, word)) {
        Some((OCCASION_TAIL, None))
    } else {
        None
    }
}

pub(super) fn pool(
    clean: &str,
    filed_plan: bool,
    memory: &Memory,
) -> Option<Vec<Cow<'static, str>>> {
    let said = clean
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    if !DATE_AHEAD.is_match(&said) || words(&said).any(|word| has(PAST_PLAN, word)) {
        return None;
    }
    let (mut pool, visitor) = if let Some(trip) = trip_pool(&said) {
        (trip, None)
    } else if filed_plan {
        diary_pool(&said)?
    } else {
        return None;
    };
    let mut length_named = Vec::new();
    if HOW_LONG_ALREADY.is_match(clean) {
        length_named.extend(
            pool.iter()
                .copied()
                .filter(|line| !line.to_lowercase().contains("how long")),
        );
        if !length_named.is_empty() {
            pool = &length_named;
        }
    }
    let mut without_question = Vec::new();
    if memory
        .turns
        .last()
        .is_some_and(|turn| turn.bot.trim_end().ends_with('?'))
    {
        without_question.extend(pool.iter().copied().filter(|line| !line.ends_with('?')));
        if !without_question.is_empty() {
            pool = &without_question;
        }
    }
    Some(
        pool.iter()
            .map(|&line| {
                visitor.map_or(Cow::Borrowed(line), |who| {
                    Cow::Owned(line.replace("{who}", who))
                })
            })
            .collect(),
    )
}
