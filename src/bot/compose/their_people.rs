//! Acknowledgments for plain facts about somebody the user knows.

use crate::bot::{Memory, Reading, understanding::line};
use crate::text::{decimal_digit, is_space};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "their_people_data.rs"]
mod data;
use data::*;

fn has(words: &[&str], word: &str) -> bool {
    words.binary_search(&word).is_ok()
}

fn capitalized(word: &str) -> String {
    let mut chars = word.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = line::without_leaders(&reading.line.clean);
    if !clean.starts_with("my ")
        || clean.contains('?')
        || reading.line.words.iter().any(|word| has(NOT_PLAIN, word))
    {
        return None;
    }
    let padded = format!(" {clean} ");
    if CRISIS
        .iter()
        .any(|phrase| padded.contains(&format!(" {phrase} ")))
    {
        return None;
    }
    taste(clean, memory).or_else(|| name_and_age(clean, memory))
}

fn pronouns(person: &str) -> (&'static str, &'static str, &'static str) {
    if has(SHE_PEOPLE, person) {
        ("she", "her", "is")
    } else if has(HE_PEOPLE, person) {
        ("he", "him", "is")
    } else {
        ("they", "them", "are")
    }
}

fn name_and_age(clean: &str, memory: &Memory) -> Option<String> {
    let parts = THEIR_NAME_AGE.captures(clean)?;
    let age = parts["age"]
        .chars()
        .try_fold(0u8, |age, digit| Some(age * 10 + decimal_digit(digit)?))?;
    let name: String = parts["name"]
        .chars()
        .scan(true, |capitalize, ch| {
            let letter = if *capitalize {
                ch.to_ascii_uppercase()
            } else {
                ch
            };
            *capitalize = !ch.is_ascii_alphabetic();
            Some(letter)
        })
        .collect();
    let (they, _, are) = pronouns(&parts["person"]);
    let age = age.to_string();
    // Fill the pool before picking so the previous reply can be excluded.
    let options = THEIR_NAME_AGE_LINES.map(|line| {
        line.replace("{name}", &name)
            .replace("{n}", &age)
            .replace("{they}", they)
            .replace("{are}", are)
    });
    Some(super::pick(&options.each_ref().map(String::as_str), memory, clean).into())
}

fn taste(clean: &str, memory: &Memory) -> Option<String> {
    static ARTICLES: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(?:the|a|an|his|her|their|its) ").unwrap());
    static POSSESSIVES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(?:my|our)\b").unwrap());
    for (pattern, pool) in [
        (&*THEIR_DISLIKE, THEIR_DISLIKE_LINES),
        (&*THEIR_HATE, THEIR_HATE_LINES),
    ] {
        let Some(parts) = pattern.captures(clean) else {
            continue;
        };
        let thing = parts["thing"].to_lowercase();
        let thing = thing
            .split(is_space)
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let bare = ARTICLES.replace(&thing, "");
        let words: Vec<_> = bare.split(' ').filter(|word| !word.is_empty()).collect();
        let Some(head) = words.first() else { continue };
        let framed = format!(" {thing} ");
        if THE_THING_IS_A_CLAUSE
            .iter()
            .any(|hang| framed.contains(hang))
            || words.len() > 4
            || has(NOT_A_THING, &bare)
            || words.iter().any(|word| has(NOT_A_TASTE_AS_A_THING, word))
            || has(AGE_WORDS, head)
            || bare.bytes().all(|byte| byte.is_ascii_digit())
        {
            continue;
        }
        let thing = POSSESSIVES.replace_all(&thing, "your");
        let (they, them, are) = pronouns(&parts["person"]);
        let initial = capitalized(&thing);
        let options: Vec<_> = pool
            .iter()
            .map(|line| {
                line.replace("{Thing}", &initial)
                    .replace("{thing}", &thing)
                    .replace("{they}", they)
                    .replace("{them}", them)
                    .replace("{are}", are)
            })
            .collect();
        let options = options.iter().map(String::as_str).collect::<Vec<_>>();
        return Some(super::pick(&options, memory, clean).into());
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::bot::Bot;

    #[test]
    fn a_childs_name_and_age_stay_together() {
        let mut bot = Bot::new(Some("0")).unwrap();
        for reply in [
            "Tom at 9 — that is a good age. What is he into at the moment?",
            "Tom, 9. What is he like just now?",
        ] {
            let turn = bot.turn("and my sons called tom, hes 9");
            assert_eq!(turn.module, Some("greeting"));
            assert_eq!(turn.reply, reply);
        }
        for (input, reply) in [
            (
                "My second daughter is called Mia and she is 06",
                "Mia, 6. What is she like just now?",
            ),
            (
                "My son is named Jo-An and he is 7",
                "Jo-An, 7. What is he like just now?",
            ),
            (
                "my son is called rohan and he is ٩",
                "Rohan, 9. What is he like just now?",
            ),
        ] {
            let mut bot = Bot::new(Some("0")).unwrap();
            let turn = bot.turn(input);
            assert_eq!(turn.module, Some("greeting"), "{input}");
            assert_eq!(turn.reply, reply, "{input}");
        }
    }
}
