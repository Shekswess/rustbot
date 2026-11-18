//! Household replacement, running costs, and upkeep by the object named.

use crate::bot::{Exchange, Memory, Reading, compose};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, ops::Range, sync::LazyLock};

#[path = "home_upkeep_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(FIRST_PERSON, data::FIRST_PERSON);
pattern!(A_LIFE, data::A_LIFE);
pattern!(WHEN_TO_REPLACE, data::WHEN_TO_REPLACE);
pattern!(HOLDS, data::HOLDS);
pattern!(HOLDS_A_THING, data::HOLDS_A_THING);
pattern!(THE_PRICE_OF, data::THE_PRICE_OF);
pattern!(A_COMPARISON_SAID, data::A_COMPARISON_SAID);
pattern!(WHAT_IT_IS_FOR_SAID, data::WHAT_IT_IS_FOR_SAID);
pattern!(ENDS_IN_BE, r"\bbe\b\s*\??\s*$");
pattern!(
    NAMED_AS_A_PLACE,
    r"\b(?:in|into|inside|out of|from|kept in|stored in|keep in|store in)\s+(?:the|a|an|my|our|your|his|her|their|this|that)\s+([a-z]+(?: [a-z]+)?)\b"
);

static WEAR: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        data::WEAR
            .iter()
            .map(|(name, _)| format!(r"\b{}\b", regex::escape(name))),
    )
    .expect("objects replaced on a schedule")
});

fn bounded(line: &str, name: &str, both: bool) -> Option<Range<usize>> {
    let bytes = line.as_bytes();
    line.match_indices(name).find_map(|(start, _)| {
        let end = start + name.len();
        (start
            .checked_sub(1)
            .is_none_or(|prior| !bytes[prior].is_ascii_lowercase())
            && (!both || bytes.get(end).is_none_or(|next| !next.is_ascii_lowercase())))
        .then_some(start..end)
    })
}

pub(super) fn any_plural(line: &str, names: &[&str]) -> bool {
    names.iter().any(|name| {
        bounded(line, name, true).is_some()
            || bounded(line, &format!("{name}s"), true).is_some()
            || bounded(line, &format!("{name}es"), true).is_some()
    })
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
    memory: Option<&Memory>,
) -> Option<Cow<'static, str>> {
    let text = &reading.line.text;
    let clean = &reading.line.clean;
    match id {
        "appliance-life" | "bedding-finished" => {
            let raw = if text.is_empty() { clean } else { text };
            let context = if reading.reference.followup {
                earlier.last().map(|turn| turn.user.as_str())
            } else {
                None
            };
            picked_lifespan(raw, context, memory)
        }
        "running-cost" => running(clean),
        "worn-out-kit" => {
            let raw = if clean.is_empty() {
                &reading.line.plain
            } else {
                clean
            };
            WEAR.matches(&raw.to_lowercase())
                .iter()
                .next()
                .map(|row| Cow::Borrowed(data::WEAR[row].1))
        }
        "door-lock-stiff" => {
            let explicit_lock = reading
                .line
                .words
                .iter()
                .any(|word| data::NAMES_A_LOCK.contains(&word.as_str()));
            if !explicit_lock
                && earlier.iter().rev().take(4).any(|turn| {
                    turn.user
                        .to_lowercase()
                        .split(crate::text::is_space)
                        .any(|word| data::ELSEWHERE_KEY.contains(&word))
                })
            {
                return None;
            }
            Some(Cow::Borrowed(data::LOCK_STIFF))
        }
        _ => None,
    }
}

fn best_lifespan(text: &str) -> Option<&'static [&'static str]> {
    let mut chosen = None;
    let mut length = 0;
    let text = text.to_lowercase();
    for data::Life(names, pool) in data::LIFESPAN {
        for name in *names {
            if name.len() > length && any_plural(&text, &[name]) {
                chosen = Some(*pool);
                length = name.len();
            }
        }
    }
    chosen
}

fn picked_lifespan(
    text: &str,
    earlier: Option<&str>,
    memory: Option<&Memory>,
) -> Option<Cow<'static, str>> {
    let has_life = A_LIFE.is_match(text);
    let low = text.to_lowercase();
    let named_as_place = || {
        NAMED_AS_A_PLACE.captures_iter(&low).any(|found| {
            let after = &found[1];
            data::IN_A_MACHINE.iter().any(|name| {
                after == *name
                    || after
                        .strip_prefix(name)
                        .is_some_and(|rest| rest.starts_with(' '))
                    || name
                        .strip_prefix(after)
                        .is_some_and(|rest| rest.starts_with(' '))
            })
        })
    };
    if FIRST_PERSON.is_match(text)
        && !has_life
        && !WHEN_TO_REPLACE.is_match(text)
        && !any_plural(&low, &["white goods", "appliances", "appliance"])
        || named_as_place()
        || (!has_life
            && (ENDS_IN_BE.is_match(&low) || HOLDS.is_match(text) && HOLDS_A_THING.is_match(text)))
    {
        return None;
    }
    if let Some(pool) = best_lifespan(text).or_else(|| earlier.and_then(best_lifespan)) {
        let selected = match memory {
            Some(memory) => compose::pick(pool, memory, ""),
            None => compose::pick(pool, &Memory::default(), ""),
        };
        return Some(Cow::Borrowed(selected));
    }
    any_plural(
        &low,
        &["white goods", "appliances", "appliance", "kitchen gadgets"],
    )
    .then_some(Cow::Borrowed(data::GENERAL_LIFE))
}

fn running(clean: &str) -> Option<Cow<'static, str>> {
    let mut found = Vec::new();
    for row in data::RUNNING {
        let mut longest = None;
        for name in row.0 {
            if let Some(span) = bounded(clean, name, true)
                && longest
                    .as_ref()
                    .is_none_or(|old: &Range<usize>| old.len() < span.len())
            {
                longest = Some(span);
            }
        }
        if let Some(span) = longest {
            found.push((span, row));
        }
    }
    let mut kept: Vec<_> = found
        .iter()
        .filter(|(span, _)| {
            !found.iter().any(|(other, _)| {
                other != span && other.start <= span.start && span.end <= other.end
            })
        })
        .collect();
    kept.sort_by_key(|(span, _)| span.start);
    let &(_, first) = kept.first()?;
    if let Some(&(_, second)) = kept.get(1) {
        let (cheaper, dearer) = if first.1 <= second.1 {
            (first, second)
        } else {
            (second, first)
        };
        let tail = if data::COOKING.contains(&cheaper.2) && data::COOKING.contains(&dearer.2) {
            format!(" {}", data::RUNNING_TAIL)
        } else {
            String::new()
        };
        return Some(Cow::Owned(format!(
            "{} is the cheaper one, {}; {} is {}.{tail}",
            title(cheaper.2),
            cheaper.3,
            dearer.2,
            dearer.3
        )));
    }
    if let Some(band) = first.4
        && THE_PRICE_OF.is_match(clean)
        && !A_COMPARISON_SAID.is_match(clean)
        && !WHAT_IT_IS_FOR_SAID.is_match(clean)
        && first
            .2
            .split(' ')
            .filter(|word| !matches!(*word, "a" | "an" | "the"))
            .all(|name| bounded(clean, name, false).is_some())
    {
        return Some(Cow::Owned(format!("{band} To run it, it is {}.", first.3)));
    }
    Some(Cow::Owned(format!("{} is {}.", title(first.2), first.3)))
}

fn title(word: &str) -> String {
    let mut chars = word.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars.as_str().to_lowercase().chars())
        .collect()
}

#[cfg(test)]
#[path = "../../../tests/fixtures/home_upkeep.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upkeep_answers_match_the_pinned_reference() {
        let mut problems = Vec::new();
        for fixtures::Case(id, prompt, earlier, attached, expected) in fixtures::CASES {
            let reading = Reading::new(prompt);
            let mut memory = Memory::default();
            memory.turns.extend(earlier.iter().map(|line| Exchange {
                user: (*line).to_owned(),
                bot: String::new(),
                module: "answers",
            }));
            let actual = if *id == "appliance-life" || *id == "bedding-finished" {
                let line = &reading.line;
                picked_lifespan(
                    if line.text.is_empty() {
                        &line.clean
                    } else {
                        &line.text
                    },
                    *attached,
                    Some(&memory),
                )
            } else {
                registered(id, &reading, &memory.turns, Some(&memory))
            };
            if actual.as_deref() != *expected {
                problems.push(format!("{id}: {prompt}: {actual:?} vs {expected:?}"));
            }
        }
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }
}
