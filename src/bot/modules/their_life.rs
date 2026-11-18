//! Replies to whole-line facts about a home, pets, siblings, age or a relative.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Memory, Reading};

#[path = "their_life_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(data::$name).expect(stringify!($name))
        });)+
    };
}
patterns!(
    NOT_A_SHAPE,
    STORY,
    TONE,
    LIVE,
    PLACE_KINDS,
    PET,
    SIBLINGS,
    AGE,
    NOT_AN_AGE,
    SOMEONE_WORK,
    SOMEONE_HOURS,
    SOMEONE_AGE,
    SOMEONE_KIDS,
    THE_LINE_BEFORE_IS_HEAVY,
);
static SOMEONE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"^(?:my|our)\s+(?P<rel>{})\s+(?P<rest>[a-z0-9 ']+)$",
        data::RELATION
    ))
    .expect("detail about a relative")
});

const NUMBER_WORDS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight",
];

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

pub(super) fn object_pronoun(relation: &str) -> &'static str {
    if data::HER.contains(&relation) {
        "her"
    } else if data::HIM.contains(&relation) {
        "him"
    } else {
        "them"
    }
}

fn count_word(said: &str) -> &str {
    said.parse::<usize>()
        .ok()
        .and_then(|number| NUMBER_WORDS.get(number))
        .copied()
        .unwrap_or(said)
}

fn plural(word: &str) -> String {
    if word.ends_with('s') || matches!(word, "fish" | "sheep" | "deer") {
        word.to_owned()
    } else if let Some(stem) = word.strip_suffix('y') {
        format!("{stem}ies")
    } else {
        format!("{word}s")
    }
}

fn pick(pool: &[&str], memory: &Memory, fill: impl Fn(&str) -> String) -> Option<String> {
    let first = fill(pool.first()?);
    pool.iter()
        .map(|template| fill(template))
        .find(|reply| !memory.turns.iter().any(|turn| turn.bot.trim() == *reply))
        .or(Some(first))
}

fn someone(relation: &str, details: &str, memory: &Memory) -> Option<String> {
    let pronoun = object_pronoun(relation);
    if let Some(found) = SOMEONE_WORK.captures(details) {
        let place = found.name("place")?.as_str();
        if !STORY.is_match(place) {
            return pick(data::SOMEONE_WORK_LINES, memory, |line| {
                let place = if line.starts_with("{Place}") {
                    capitalize(place)
                } else {
                    place.to_owned()
                };
                line.replace("{Place}", &place).replace("{them}", pronoun)
            });
        }
    }
    if let Some(found) = SOMEONE_HOURS.captures(details) {
        let hours = capitalize(found.name("hours")?.as_str());
        return pick(data::SOMEONE_HOURS_LINES, memory, |line| {
            line.replace("{Hours}", &hours).replace("{them}", pronoun)
        });
    }
    if let Some(found) = SOMEONE_AGE.captures(details) {
        let age = found.name("age")?.as_str().parse::<u8>().ok()?;
        if (1..=99).contains(&age) && !data::AGE_READ_ELSEWHERE.contains(&relation) {
            return pick(data::SOMEONE_AGE_LINES, memory, |line| {
                line.replace("{Age}", &age.to_string())
                    .replace("{them}", pronoun)
            });
        }
    }
    if let Some(found) = SOMEONE_KIDS.captures(details) {
        let number = capitalize(count_word(found.name("num")?.as_str()));
        return pick(data::SOMEONE_KIDS_LINES, memory, |line| {
            line.replace("{Num}", &number)
        });
    }
    None
}

fn normalized(text: &str) -> String {
    let line = text.to_lowercase().replace('’', "'");
    let line = line
        .trim()
        .trim_matches([' ', '.', ',', '!', ';', ':', '-', '"', '\''])
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    TONE.replace(&line, "").into_owned()
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if reading.raw.contains('?') {
        return None;
    }
    let line = normalized(reading.raw);
    if line.is_empty()
        || NOT_A_SHAPE.is_match(&line)
        || super::the_detail_the_thread_adds::heavy::thread_is_heavy(&line)
        || memory.turns.last().is_some_and(|turn| {
            matches!(turn.module, "safety" | "grief" | "carers_turn")
                || THE_LINE_BEFORE_IS_HEAVY.is_match(&turn.user.to_lowercase())
                || super::the_detail_the_thread_adds::heavy::thread_is_heavy(
                    &turn.user.to_lowercase(),
                )
        })
    {
        return None;
    }
    if let Some(found) = LIVE.captures(&line) {
        let where_ = found.name("where")?.as_str();
        if PLACE_KINDS.is_match(where_) && !STORY.is_match(where_) {
            return pick(data::PLACES, memory, |reply| {
                reply.replace("{Where}", &capitalize(where_))
            });
        }
    }
    if let Some(found) = SIBLINGS.captures(&line) {
        let siblings = capitalize(found.name("what")?.as_str());
        return pick(data::SIBLING_LINES, memory, |reply| {
            reply.replace("{What}", &siblings)
        });
    }
    if let Some(found) = AGE.captures(&line) {
        let age = found.name("age")?.as_str().parse::<u8>().ok()?;
        if (13..=99).contains(&age) && !NOT_AN_AGE.is_match(&line) {
            return pick(data::AGE_LINES, memory, |reply| {
                reply
                    .replace("{Age}", &age.to_string())
                    .replace("{Next}", &(age + 1).to_string())
            });
        }
    }
    if let Some(found) = PET.captures(&line) {
        let count = found
            .name("num")
            .map_or("", |number| number.as_str())
            .trim();
        let animal = found.name("pet")?.as_str();
        if count.parse::<u32>().is_ok_and(|number| number > 1)
            || ["two", "three", "four", "five", "six"].contains(&count)
        {
            let count = if count.as_bytes().first().is_some_and(u8::is_ascii_digit) {
                "The".to_owned()
            } else {
                capitalize(count)
            };
            let animals = format!("{count} {}", plural(animal));
            return pick(data::PETS, memory, |reply| {
                reply.replace("{Pets}", &animals)
            });
        }
        let article = if animal.starts_with(['a', 'e', 'i', 'o', 'u']) {
            "An"
        } else {
            "A"
        };
        return pick(data::ONE_PET, memory, |reply| {
            reply.replace("{Pet}", &format!("{article} {animal}"))
        });
    }
    let found = SOMEONE.captures(&line)?;
    someone(
        found.name("rel")?.as_str(),
        found.name("rest")?.as_str(),
        memory,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn their_life_reads_whole_facts_and_leaves_sensitive_history_alone() {
        let mut memory = Memory::default();
        for (prompt, start) in [
            ("We live near the park", "Near the park"),
            ("I've got two cats", "Two cats"),
            ("I'm 34", "34"),
            ("I have a sister and a brother", "A sister and a brother"),
            ("My mum is 71", "71"),
            ("My wife works three days a week", "Three days a week"),
        ] {
            let answer =
                respond(&Reading::new(prompt), &memory).unwrap_or_else(|| panic!("{prompt}"));
            assert!(answer.starts_with(start), "{answer}");
        }
        for prompt in [
            "I’m 34 weeks pregnant",
            "My mum is 71 and can’t walk",
            "We live near the park and the gate is broken",
            "We have a dog that keeps barking",
        ] {
            assert!(
                respond(&Reading::new(prompt), &memory).is_none(),
                "{prompt}"
            );
        }
        let fact = Reading::new("We live near the park");
        let first = respond(&fact, &memory).unwrap();
        memory.turns.push(Exchange {
            user: fact.raw.to_owned(),
            bot: first.clone(),
            module: "their_life",
        });
        assert_ne!(respond(&fact, &memory).unwrap(), first);
        memory.turns.push(Exchange {
            user: "My friend is very ill".into(),
            bot: String::new(),
            module: "greeting",
        });
        assert!(respond(&fact, &memory).is_none());
        let turn = Bot::new(Some("0")).unwrap().turn(fact.raw);
        assert!(
            turn.candidates
                .iter()
                .any(|candidate| candidate.module == "their_life" && candidate.priority == 100.0)
        );
    }
}
