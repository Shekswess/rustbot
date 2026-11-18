//! Move a time, amount or place to the front when it answers the current question.

use fancy_regex::Regex as ComplexRegex;
use regex::Regex;
use std::sync::LazyLock;

use super::sentences;
use crate::text::is_space;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

macro_rules! complex_pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<ComplexRegex> =
            LazyLock::new(|| ComplexRegex::new($source).expect(stringify!($name)));
    };
}

include!("leading_shape_data.rs");

pattern!(SUBJECT_WORD, r"[a-z]{4,}");
pattern!(WHEN, r"(?i)\bwhen\b");
pattern!(DIGIT, r"\d");

#[derive(Clone, Copy, Eq, PartialEq)]
enum Shape {
    Time,
    Amount,
    Place,
}

fn matches(pattern: &ComplexRegex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn time(text: &str) -> bool {
    if matches(&NAMED_CLOCK, text) || matches(&EDGED_PART_OF_DAY, text) {
        return true;
    }
    let without_rate = RATE_PHRASE.replace(text, "");
    let without_count = RATE_OF_A_COUNT.replace_all(&without_rate, " ");
    let said = without_count.as_ref();
    if DURATION.is_match(said)
        || matches(&CLOCK, said)
        || BARE_RANGE.is_match(said)
        || STAGE.is_match(said)
        || NEVER.is_match(said)
        || NO_FIXED_TIME.is_match(said)
        || MINUTES_AS_A_LENGTH.is_match(said)
    {
        return true;
    }
    let clause = said.split([',', ';', ':']).next().unwrap_or("");
    !INSTRUCTS.is_match(text)
        && (NAMED_TIME.is_match(clause) || DAY_NAME.is_match(clause) || matches(&DATE, clause))
}

fn age(text: &str) -> bool {
    AN_AGE_YEARS.is_match(text)
        || AN_AGE_MONTHS.is_match(text)
        || AN_AGE_BIRTHDAY.is_match(text)
        || AN_AGE_NAMED.is_match(text)
        || matches(&AN_AGE_BARE, text)
        || matches(&AN_AGE_SPELLED_OPENS, text)
}

fn amount(text: &str) -> bool {
    matches(&FIGURE, text) || MONEY.is_match(text) || RATE.is_match(text)
}

fn names_a_spot(text: &str) -> bool {
    SPOT_OF_THE_THING.is_match(text)
        || SPOT_ON_THE_THING.captures_iter(text).any(|matched| {
            let spot = matched[1].trim_matches(['\'', '-']).to_lowercase();
            SPOT_PARTS.binary_search(&spot.as_str()).is_ok()
        })
}

fn place(text: &str) -> bool {
    let said = CALENDAR_WORD.replace_all(text, " ");
    PLACE_WORD.is_match(&said)
        || matches(&PLACE_NAME, &said)
        || PLACE_NAME_ANY.find_iter(&said).flatten().take(2).count() == 2
        || SETTLED_ON.is_match(&said)
        || names_a_spot(&said)
        || PLACE_KIND.is_match(&said)
        || PLACE_AT_HOME.is_match(&said)
}

fn is_a_punt(text: &str) -> bool {
    PUNT.is_match(text) || matches(&HANDOFF, text) || REFERS_BACK.is_match(text)
}

fn opens_with_question(text: &str) -> bool {
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if matches!(ch, '"' | '“' | '”') {
            quoted = !quoted;
        } else if !quoted
            && matches!(ch, '.' | '!' | '?')
            && chars.peek().is_none_or(|after| is_space(*after))
        {
            return ch == '?';
        }
    }
    text.ends_with('?')
}

fn gives(text: &str, shape: Shape, line: &str) -> bool {
    if text.is_empty() || is_a_punt(text) || opens_with_question(text) {
        return false;
    }
    match shape {
        Shape::Time => {
            let without_rate = MONEY_RATE.replace_all(text, " ");
            if without_rate.as_ref() != text {
                return time(&without_rate);
            }
            ((STAGE_WHEN.is_match(text) || SEASON_WHEN.is_match(text)) && ASKS_WHEN.is_match(line))
                || time(text)
                || (EXTENT.is_match(text) && LENGTH_ASKED.is_match(line))
        }
        Shape::Amount if ASKS_AN_AGE.is_match(line) => age(text),
        Shape::Amount => {
            amount(text)
                && !(PRICE_ASKED.is_match(line) && time(text) && !PRICE_GIVEN.is_match(text))
        }
        Shape::Place => place(text),
    }
}

fn names_the_line(text: &str, line: &str, asked: &[&str]) -> bool {
    if line.is_empty() {
        return true;
    }
    let said = text.to_lowercase();
    SUBJECT_WORD
        .find_iter(&said)
        .any(|word| asked.contains(&word.as_str()))
}

/// Keep the authored order unless the question's answer can lead as a complete sentence.
pub(super) fn apply(wants: Option<&str>, line: &str, reply: &str) -> Option<String> {
    let shape = match wants?.trim_matches(is_space).to_ascii_lowercase().as_str() {
        "time" => Shape::Time,
        "amount" => Shape::Amount,
        "place" => Shape::Place,
        _ => return None,
    };
    let text = reply
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let parts = sentences(&text);
    let (first, rest) = parts.split_first()?;
    if rest.is_empty()
        || is_a_punt(first)
        || OPENS_WITH_SYMPATHY.is_match(first)
        || OPENS_WITH_A_WARNING.is_match(first)
    {
        return None;
    }
    let normalized_line = line
        .to_lowercase()
        .replace('’', "'")
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if gives(first, shape, &normalized_line) {
        return None;
    }
    let lower = line.to_lowercase();
    let asked: Vec<_> = SUBJECT_WORD
        .find_iter(&lower)
        .map(|word| word.as_str())
        .filter(|word| SHARED_WORDS.binary_search(word).is_err())
        .collect();
    if DIGIT.is_match(first) && names_the_line(first, line, &asked) {
        return None;
    }
    let when = WHEN.is_match(line);
    let offset = rest.iter().position(|sentence| {
        (!when || matches(&CLOCK, sentence))
            && !OPENS_WITH_A_WARNING.is_match(sentence)
            && !POINTS_BACKWARDS.is_match(sentence)
            && !sentence.ends_with('?')
            && sentence.split_whitespace().nth(3).is_some()
            && (shape != Shape::Place || !(time(sentence) || amount(sentence)))
            && gives(sentence, shape, &normalized_line)
            && names_the_line(sentence, line, &asked)
    })?;
    let index = offset + 1;
    Some(
        std::iter::once(parts[index])
            .chain(parts[..index].iter().copied())
            .chain(parts[index + 1..].iter().copied())
            .collect::<Vec<_>>()
            .join(" "),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Reading, knowledge::updated_guidance};

    #[test]
    fn lifts_the_first_standalone_sentence_that_names_the_subject() {
        let cases = [
            (
                "time",
                "how long should i hold an ice pack on",
                "Cold for a fresh knock and heat for a stiff, old ache. Ice in the first 48 hours of a sprain: an ice pack wrapped in a tea towel, 15 to 20 minutes at a time. After that, heat loosens a tight muscle.",
                "Ice in the first 48 hours of a sprain: an ice pack wrapped in a tea towel, 15 to 20 minutes at a time. Cold for a fresh knock and heat for a stiff, old ache. After that, heat loosens a tight muscle.",
            ),
            (
                "amount",
                "how much does a washing machine cost",
                "Washing machines differ by model. A new washing machine can last ten years. A new washing machine often costs £400. Allow for delivery.",
                "A new washing machine often costs £400. Washing machines differ by model. A new washing machine can last ten years. Allow for delivery.",
            ),
            (
                "place",
                "where did the family travel",
                "They planned it months ahead. The family stayed in London. They toured a museum.",
                "The family stayed in London. They planned it months ahead. They toured a museum.",
            ),
        ];
        for (want, line, original, expected) in cases {
            assert_eq!(apply(Some(want), line, original).as_deref(), Some(expected));
        }
    }

    #[test]
    fn the_ice_duration_card_leads_with_its_timing_before_repeat_framing() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let input = "how long should i hold an ice pack on";
        let first = bot.turn(input);
        assert_eq!(first.module, Some("answers"));
        assert!(first.reply.starts_with("Ice in the first 48 hours"));
        assert!(first.reply.contains("Cold for a fresh knock"));
        let timing = sentences(&first.reply)[0];
        assert_eq!(bot.turn(input).reply, format!("Same as before: {timing}"));
    }

    #[test]
    fn keeps_warnings_sympathy_and_approved_guidance_in_order() {
        for (line, reply) in [
            (
                "how long should i wait after a gas smell",
                "Do not use switches or flames. The gas engineer could take 20 minutes.",
            ),
            (
                "how long should i wait for medical help",
                "Call 999 now. The emergency team may take twenty minutes on a busy day.",
            ),
            (
                "how long does cremation take",
                "I'm so sorry about your dog. The cremation usually takes two days, and the vet can explain.",
            ),
            (
                "how long does a fit last",
                "Never wait to get help if it is a first seizure. A fit can stop after five minutes.",
            ),
        ] {
            assert_eq!(apply(Some("time"), line, reply), None, "{line}");
        }
        for (domain, id, input, want) in [
            (
                "body",
                "arthritis",
                "how long will my suddenly swollen joint hurt",
                "time",
            ),
            (
                "health",
                "tablet-side-effects",
                "how long will my tablet side effects last",
                "time",
            ),
            (
                "costs",
                "cost-boiler-service",
                "how much does a boiler service cost",
                "amount",
            ),
        ] {
            let reading = Reading::new(input);
            let reply = updated_guidance::registered(domain, id, &reading).unwrap();
            assert_eq!(apply(Some(want), input, &reply), None, "{input}");
        }
    }
}
