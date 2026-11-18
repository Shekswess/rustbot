//! Greet the occasion someone says is theirs or belongs to someone close.

use crate::bot::{Memory, Reading};
use crate::text::is_space;
use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($value).expect(stringify!($name)));
    };
}

#[path = "occasion_data.rs"]
mod data;
use data::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Occasion {
    Birthday,
    Anniversary,
    Wedding,
    Wish,
}

fn doubted(line: &str) -> bool {
    let words: Vec<_> = line
        .split_whitespace()
        .map(|word| {
            word.trim_matches(['.', ',', '!', '?', ';', ':', '"', '\''])
                .replace('’', "")
        })
        .collect();
    words.iter().enumerate().any(|(i, word)| {
        if DOUBTED.contains(&word.as_str()) {
            return true;
        }
        if !SURE.contains(&word.as_str()) {
            return false;
        }
        let before: Vec<_> = words[i.saturating_sub(4)..i]
            .iter()
            .map(String::as_str)
            .collect();
        super::trouble::turns_round(&before)
            && !(DOUBT.contains(&word.as_str()) && before.iter().any(|word| CANNOT.contains(word)))
            && !(BELIEVE_IT.contains(&word.as_str())
                && words.get(i + 1).is_some_and(|word| word == "it"))
    })
}

fn read(line: &str) -> Option<Occasion> {
    let line = line
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if line.is_empty()
        || line.contains('?')
        || [&*NOT_AN_OCCASION, &*LET_DOWN, &*SOMEONE_LEFT, &*BAD]
            .iter()
            .any(|pat| pat.is_match(&line))
        || doubted(&line)
    {
        return None;
    }
    [
        (&*BIRTHDAY, Occasion::Birthday),
        (&*ANNIVERSARY, Occasion::Anniversary),
        (&*WEDDING, Occasion::Wedding),
        (&*WISH, Occasion::Wish),
        (&*ORDINAL_DAY, Occasion::Birthday),
    ]
    .into_iter()
    .find_map(|(pattern, occasion)| pattern.is_match(&line).then_some(occasion))
}

fn family(word: &str) -> bool {
    let word = word.trim().trim_matches('\'');
    FAMILY.contains(&word.strip_suffix('s').unwrap_or(word))
}

fn someone_elses(line: &str) -> bool {
    [&*RELATION_BEFORE_DAY, &*ORDINAL_DAY]
        .into_iter()
        .flat_map(|pattern| pattern.captures_iter(line))
        .filter_map(|found| found.name("whose"))
        .any(|whose| whose.as_str().split(is_space).any(family))
}

fn pronoun(line: &str) -> Option<&'static str> {
    static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
    let line = line.replace('’', "'");
    let mut she = false;
    let mut he = false;
    for word in WORD.find_iter(&line).map(|found| found.as_str()) {
        let stem = word
            .strip_suffix('s')
            .filter(|stem| SHE.contains(stem) || HE.contains(stem))
            .unwrap_or(word);
        if SHE.contains(&stem) {
            she = true;
        } else if HE.contains(&stem) {
            he = true;
        }
    }
    match (she, he) {
        (true, false) => Some("her"),
        (false, true) => Some("him"),
        _ => None,
    }
}

fn pool(occasion: Occasion, typed: &str) -> Vec<Cow<'static, str>> {
    let (own_pool, someone_elses_pool) = match occasion {
        Occasion::Birthday => (BIRTHDAY_REPLIES, BIRTHDAY_FOR_SOMEONE_ELSE),
        Occasion::Anniversary => (ANNIVERSARY_REPLIES, ANNIVERSARY_FOR_SOMEONE_ELSE),
        Occasion::Wedding => (WEDDING_REPLIES, &[][..]),
        Occasion::Wish => (WISH_REPLIES, &[][..]),
    };
    if !someone_elses_pool.is_empty() && someone_elses(typed) {
        let apart = APART.is_match(typed);
        let pronoun = pronoun(typed);
        static THEM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bthem\b").unwrap());
        let kept: Vec<_> = someone_elses_pool
            .iter()
            .filter(|line| apart || !PLACES_THEM.is_match(line))
            .map(|&line| pronoun.map_or(Cow::Borrowed(line), |who| THEM.replace_all(line, who)))
            .collect();
        if !kept.is_empty() {
            return kept;
        }
        return someone_elses_pool
            .iter()
            .map(|&line| Cow::Borrowed(line))
            .collect();
    }
    let now = HAPPENING_NOW.is_match(typed);
    let has_date = DATE_GIVEN.is_match(typed);
    let kept: Vec<_> = own_pool
        .iter()
        .filter(|line| {
            let lower = line.to_lowercase();
            !(now && lower.contains("when it comes") || has_date && lower.contains("how far off"))
        })
        .map(|&line| Cow::Borrowed(line))
        .collect();
    if kept.is_empty() {
        return own_pool.iter().map(|&line| Cow::Borrowed(line)).collect();
    }
    kept
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let typed = &reading.line.clean;
    let repaired = &reading.line.plain;
    let occasion = read(typed).or_else(|| (repaired != typed).then(|| read(repaired)).flatten())?;
    if super::trouble::negative_tone(reading) {
        return None;
    }
    let pool = pool(occasion, typed);
    let options: Vec<_> = pool.iter().map(Cow::as_ref).collect();
    Some(super::choose_recent(&options, memory, typed).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_the_occasion_and_addresses_the_person_it_belongs_to() {
        let memory = Memory::default();
        for (line, expected) in [
            (
                "im looking into a party for our anniversary",
                "Happy anniversary when it comes. Are you doing anything for it?",
            ),
            (
                "my mums 70th is in march",
                "Happy birthday to her — are you doing something for it?",
            ),
            (
                "my sisters birthday is next monday and im visiting",
                "Lovely. Are you seeing her for it?",
            ),
        ] {
            assert_eq!(
                reply(&Reading::new(line), &memory).as_deref(),
                Some(expected),
                "{line}"
            );
        }
    }

    #[test]
    fn leaves_doubted_or_unhappy_occasions_to_other_readers() {
        let memory = Memory::default();
        for line in [
            "i dont think its our anniversary",
            "my mums birthday is friday but im upset",
            "what can i get for our anniversary",
        ] {
            assert_eq!(reply(&Reading::new(line), &memory), None, "{line}");
        }
    }
}
