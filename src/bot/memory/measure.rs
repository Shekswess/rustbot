//! Answer a bare follow-up with the measure from the last selected card.

use super::will_it_work::SENTENCE;
use crate::bot::{Memory, Reading};
use crate::text::is_space;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "measure_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(LEAD, TAIL, ASKED, THING_AFTER, A_CONTAINER);
static CLAUSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[,;:]\s+").expect("measure clause boundary"));
static MEASURES: LazyLock<Vec<(&'static str, Regex)>> = LazyLock::new(|| {
    data::MEASURES
        .iter()
        .map(|&(name, pattern)| (name, Regex::new(pattern).expect(name)))
        .collect()
});

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let measure = which_measure(reading.raw)?;
    let last = memory.turns.last()?;
    let card = memory.entries.on_screen(&last.bot, last.module)?;
    let pattern = &MEASURES.iter().find(|(name, _)| *name == measure)?.1;
    let sentence = SENTENCE
        .split(card)
        .filter_map(Result::ok)
        .map(|sentence| sentence.trim_matches(is_space))
        .find(|sentence| pattern.is_match(sentence))?;
    let said = clause(sentence);
    let rendered = if let Some(thing) = named_thing(memory)
        && !said.to_lowercase().contains(&thing.to_lowercase())
    {
        if let Some(container) = A_CONTAINER.find(said) {
            let (head, tail) = said.split_at(container.end());
            Cow::Owned(format!("{head} of {thing}{tail}"))
        } else {
            Cow::Owned(headline(&thing, said)?)
        }
    } else {
        Cow::Borrowed(said)
    };
    if rendered.ends_with(['.', '!', '?']) {
        Some(rendered.into_owned())
    } else {
        Some(format!("{rendered}."))
    }
}

fn which_measure(text: &str) -> Option<String> {
    let punctuation = [' ', '?', '.', '!', ',', ':', ';', '-'];
    let text = LEAD.replace(text.trim_matches(is_space), "");
    let text = TAIL.replace_all(&text, "");
    let text = LEAD.replace(text.trim_matches(punctuation), "");
    let matches = ASKED.captures(text.trim_matches(punctuation))?;
    Some(matches[1].to_lowercase())
}

fn clause(sentence: &str) -> &str {
    for boundary in CLAUSE.find_iter(sentence) {
        let first = sentence[boundary.end()..]
            .split(is_space)
            .find(|word| !word.is_empty())
            .unwrap_or_default()
            .trim_matches([',', '.', ';', ':'])
            .to_lowercase();
        if data::A_NEW_CLAUSE.binary_search(&first.as_str()).is_ok() {
            let said = sentence[..boundary.start()].trim_matches(is_space);
            if said
                .split(is_space)
                .filter(|word| !word.is_empty())
                .take(4)
                .count()
                == 4
            {
                return said;
            }
        }
    }
    sentence
}

fn named_thing(memory: &Memory) -> Option<String> {
    let previous = memory
        .turns
        .iter()
        .rev()
        .find(|turn| !turn.user.trim_matches(is_space).is_empty())?;
    let found = THING_AFTER.captures(&previous.user)?;
    let thing = found.name("thing")?.as_str();
    let kept = thing
        .split(is_space)
        .filter(|word| !word.is_empty())
        .map(|word| word.trim_matches([',', '.', ';', ':']))
        .take_while(|word| {
            data::STOPS_THE_THING
                .binary_search(&word.to_lowercase().as_str())
                .is_err()
        })
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");
    let kept = kept.trim_matches([' ', ',', '.', ';', ':', '-']);
    (!kept.is_empty()
        && data::THING_NOT_WORTH_SAYING
            .binary_search(&kept.to_lowercase().as_str())
            .is_err())
    .then(|| kept.to_owned())
}

fn headline(thing: &str, said: &str) -> Option<String> {
    let first = thing.chars().next()?;
    let head = format!("{}{}", first.to_uppercase(), &thing[first.len_utf8()..]);
    let mut chars = said.chars();
    let first = chars.next()?;
    if first.is_uppercase() && chars.next().is_some_and(char::is_lowercase) {
        return Some(format!(
            "{head}: {}{}",
            first.to_lowercase(),
            &said[first.len_utf8()..]
        ));
    }
    Some(format!("{head}: {said}"))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/measure.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn bare_measures_match_pinned_card_and_context_rules() {
        for &fixtures::Case(previous, bot, module, entries, ask, expected) in fixtures::CASES {
            let mut memory = Memory::default();
            memory.turns.push(Exchange {
                user: previous.into(),
                bot: bot.into(),
                module,
            });
            for &card in entries {
                memory.entries.remember(Cow::Borrowed(card));
            }
            assert_eq!(
                respond(&Reading::new(ask), &memory).as_deref(),
                expected,
                "{ask} after {previous}"
            );
        }
    }
}
