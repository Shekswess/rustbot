//! Ask about an unresolved choice the person has already put into words.

use crate::bot::{Memory, Reading};

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($value).expect(stringify!($name)));
    };
}

macro_rules! context_pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($value).expect(stringify!($name)));
    };
}

#[path = "dither_data.rs"]
mod data;
use data::*;

fn unresolved(line: &str) -> bool {
    !FACT_BETWEEN.is_match(line)
        && !PERSON_NOT_A_DECISION.is_match(line)
        && !PRONOUN_NOT_A_DECISION.is_match(line)
        && !HELP_SERVICE.is_match(line)
        && DITHER.is_match(line).unwrap_or(false)
        && (DITHER_STRONG.is_match(line).unwrap_or(false)
            || DITHER_SUBJECT.is_match(line).unwrap_or(false))
}

fn pool(typed: &str, repaired: &str, words: &[String]) -> Option<&'static [&'static str]> {
    if !(unresolved(typed) || repaired != typed && unresolved(repaired))
        || !DITHER_TWO_WAY.is_match(typed)
            && (LIFE_DIRECTION.is_match(typed) || A_STRETCH_OF_TIME.is_match(typed))
    {
        return None;
    }
    Some(
        if words
            .iter()
            .any(|word| HEALTH_WORDS.contains(&word.as_str()))
            && !HEALTH_MODIFIER.is_match(typed)
            || CHECK_PHRASES.iter().any(|phrase| typed.contains(phrase))
        {
            UNKNOWN_HEALTH
        } else if DITHER_YES_NO.is_match(typed) {
            DITHER_DECIDE
        } else if DITHER_OPEN.is_match(typed) {
            DITHER_OPEN_LINES
        } else {
            DITHER_TWO_WAY_LINES
        },
    )
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let typed = &reading.line.clean;
    let pool = pool(typed, &reading.line.plain, &reading.line.words)?;
    super::trouble::neutral_tone(reading).then(|| super::choose_recent(pool, memory, typed).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn draws_for_a_choice_and_avoids_repeating_it() {
        let reading = Reading::new("im torn between two laptops");
        let mut memory = Memory::default();
        let first = reply(&reading, &memory).unwrap();
        assert_eq!(
            first,
            "What's pulling you each way? Even a rough version will do, and I'll lay both out with you."
        );
        memory.turns.push(Exchange {
            user: reading.raw.into(),
            bot: first,
            module: "greeting",
        });
        assert_eq!(
            reply(&reading, &memory).as_deref(),
            Some(
                "Say both versions out loud, the one where you do it and the one where you don't, and I'll help you find the one you'd regret less."
            )
        );
    }

    #[test]
    fn reads_a_repaired_choice_and_passes_on_other_uncertainty() {
        let memory = Memory::default();
        let reading = Reading::new("i dont no what to do about my job");
        assert_eq!(
            reply(&reading, &memory).as_deref(),
            Some(
                "Say what you'd want if nothing was in the way, and we can work back to the first step."
            )
        );
        for line in [
            "im not sure what to do with my life",
            "i dont know the difference between the two",
            "im not sure if i should call 111",
            "im not sure what to do about my wife",
        ] {
            assert_eq!(reply(&Reading::new(line), &memory), None, "{line}");
        }
    }
}
