//! Ask about the first fact when the usual acknowledgement would just repeat it.

use super::upper_first;
use crate::bot::{Memory, compose};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($value).expect(stringify!($name)));
    };
}

#[path = "ack_opening_data.rs"]
mod data;
use data::*;

struct Pronouns {
    subject: &'static str,
    object: &'static str,
    be: &'static str,
    has: &'static str,
}

const YOU: Pronouns = Pronouns {
    subject: "you",
    object: "you",
    be: "are",
    has: "have",
};
const THEY_PRONOUNS: Pronouns = Pronouns {
    subject: "they",
    object: "them",
    be: "are",
    has: "have",
};
const SHE_PRONOUNS: Pronouns = Pronouns {
    subject: "she",
    object: "her",
    be: "is",
    has: "has",
};
const HE_PRONOUNS: Pronouns = Pronouns {
    subject: "he",
    object: "him",
    be: "is",
    has: "has",
};

fn names_relations(names: &[&str]) -> Regex {
    Regex::new(&format!(
        r"\bmy (?:\w+ )?(?:{})(?:s|'s|s')?\b",
        names.join("|")
    ))
    .expect("opening relations")
}

fn who(clean: &str) -> Pronouns {
    pattern!(
        REPORT,
        r"^\s*(?:and |so |but |well )?(?:they|he|she|people|someone|everyone)\s+(?:said|says|say|reckons?|thinks?|told me|tell me)\s+"
    );
    pattern!(
        STARTS_WITH_PRONOUN,
        r"^\s*(?:and |so |but |well )?(he|she|they|theyre|they're)\b"
    );
    static NAMES_THEY: LazyLock<Regex> = LazyLock::new(|| names_relations(THEY));
    static NAMES_SHE: LazyLock<Regex> = LazyLock::new(|| names_relations(SHE));
    static NAMES_HE: LazyLock<Regex> = LazyLock::new(|| names_relations(HE));
    let lower = clean.to_lowercase();
    let lower = REPORT.replace(&lower, "");
    if let Some(pronoun) = STARTS_WITH_PRONOUN.captures(&lower) {
        return match &pronoun[1] {
            "he" => HE_PRONOUNS,
            "she" => SHE_PRONOUNS,
            _ => THEY_PRONOUNS,
        };
    }
    [
        (&*NAMES_THEY, THEY_PRONOUNS),
        (&*NAMES_SHE, SHE_PRONOUNS),
        (&*NAMES_HE, HE_PRONOUNS),
    ]
    .into_iter()
    .find_map(|(pattern, pronouns)| pattern.is_match(&lower).then_some(pronouns))
    .unwrap_or(YOU)
}

fn the_day_is_the_point(clean: &str, after_day: &str) -> bool {
    pattern!(NUMBER, r"\b\d{1,3}\b");
    pattern!(JOIN, r"(?i)\b(?:and|but|n)\b");
    A_DAY_FOR_SOMEONE.find_iter(clean).take(2).count() < 2
        && NUMBER.find_iter(clean).take(2).count() < 2
        && !JOIN.is_match(after_day)
}

pub(super) fn reply(thing: &str, clean: &str, memory: &Memory) -> Option<String> {
    if thing.is_empty()
        || clean.is_empty()
        || memory.turns.iter().any(|turn| !turn.bot.trim().is_empty())
        || [&*A_PAST, &*A_MEASURE, &*A_MONTHS, &*A_CORRECTION, &*MONEY]
            .iter()
            .any(|pattern| pattern.is_match(clean))
        || MONEY.is_match(thing)
        || NOT_AN_OCCASION.is_match(thing)
    {
        return None;
    }
    let pronouns = who(clean);
    let taste = A_TASTE.find(clean).is_some_and(|found| {
        let tail = &clean[found.end()..];
        !A_TASTE_OF_A_PERSON.is_match(thing)
            && !A_CRUSH.is_match(clean)
            && !A_PERSON_AS_OBJECT.is_match(tail)
            && !tail.contains(" but ")
    });
    let pool = if taste {
        A_LIKED_TAILS
    } else if let Some(day) = A_DAY_FOR_SOMEONE.find(clean) {
        if !the_day_is_the_point(clean, &clean[day.end()..]) {
            return None;
        }
        if pronouns.subject == "you" {
            &[
                A_DAY_TAILS[0],
                "{Thing} — anything you're marking it with?",
                A_DAY_TAILS[3],
            ]
        } else {
            A_DAY_TAILS
        }
    } else if A_TIME.is_match(clean) || A_START.is_match(clean) {
        A_DOING_TAILS
    } else if A_STANDING_STATE.is_match(clean) {
        A_STATE_TAILS
    } else {
        return None;
    };
    Some(
        compose::pick(pool, memory, clean)
            .replace("{Thing}", &upper_first(thing))
            .replace("{subj}", pronouns.subject)
            .replace("{them}", pronouns.object)
            .replace("{be}", pronouns.be)
            .replace("{has}", pronouns.has)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    )
}
