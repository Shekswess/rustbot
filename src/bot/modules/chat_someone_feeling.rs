//! Read and word another person's feelings with the pronouns the user gave.

use super::{flat, mentions, one_of, split};
use crate::bot::{Memory, compose, understanding::kinds::cue_about_person};
use crate::text::is_space;
use regex::Regex;
use std::sync::LazyLock;

#[path = "chat_someone_feeling_data.rs"]
mod data;
use data::*;

pattern!(
    FED_UP_TAIL,
    r"\b(?:sick of|tired of|fed up with|fed up of|done with|had enough of|had it with)\s+[^.!?]*$"
);
pattern!(TO_ME, r"\b(?:with|at|about|of|by)\s+(?:me|us)\b");
pattern!(SHE_IN_TEXT, r"\b(?:she|her|hers)\b");
pattern!(HE_IN_TEXT, r"\b(?:he|him|his)\b");
pattern!(
    FED_UP_A_THING,
    r"\b(?:sick of|done with|had enough of|had it with)\b\s*(.*)$"
);
pattern!(MY_FEELING, r"\b(?:i'm|im|i am|i feel|i've|i have|me|my)\b");
pattern!(WITH_PERSON, r"^\s+with\s+(?:me|us|him|her|them)\b");
pattern!(HAS_A_CAUSE, r"^\s+(?:of|about|with)\s+(.+)$");
pattern!(LOW_IN, r"^\s+(?:on|in)\b");
pattern!(SPENT_ON, r"^\s+(?:up|all|it|my|the|every)\b");
pattern!(MY_BAD_DAY, r"\b(?:i'm|im|i am|i feel|i've|i had)\b");
pattern!(
    CAUSE_ENDS,
    r"\b(?:because|but|so|and|since|when|after|before|which|that)\b"
);
pattern!(
    FED_UP_LEAD,
    r"\b(fed up with|fed up of|fed up|sick and tired of|sick of|tired of|done with|had enough of|had it with)\b"
);

#[derive(Clone, Copy)]
enum Pronouns {
    Her,
    Him,
    Them,
}

enum Feeling {
    Direct(&'static str),
    About(&'static str, String),
    FedUp(String),
}

struct OtherFeeling {
    feeling: Feeling,
    person: Pronouns,
}

fn has(vocabulary: &[&str], word: &str) -> bool {
    vocabulary.binary_search(&word).is_ok()
}

fn pronouns(word: &str) -> Pronouns {
    if has(PLURAL_WORDS, word) {
        Pronouns::Them
    } else if has(SHE_WORDS, word) {
        Pronouns::Her
    } else if has(HE_WORDS, word) {
        Pronouns::Him
    } else {
        Pronouns::Them
    }
}

fn recent_plural(memory: &Memory) -> bool {
    let start = memory.turns.len().saturating_sub(3);
    let said = memory.turns[start..]
        .iter()
        .map(|turn| turn.user.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    !said.is_empty()
        && (split(&said).any(|word| has(PLURAL_WORDS, word)) || mentions(&JOINED_PERSON, &said))
}

fn who<'a>(clean: &'a str, memory: &Memory) -> Option<(&'a str, usize, bool)> {
    if let Ok(Some(matched)) = OTHER_PERSON.captures(clean) {
        return Some((matched.get(1)?.as_str(), matched.get(0)?.end(), false));
    }
    let pronoun = OTHER_PRONOUN.find(clean).ok().flatten()?;
    let said = pronoun.as_str();
    let word = said.split('\'').next()?.trim_end_matches('s');
    if matches!(word, "they" | "them" | "their" | "we" | "us" | "you") && !recent_plural(memory) {
        return None;
    }
    let be = said.len() > 2
        && ["'s", "'re", "s", "re"]
            .iter()
            .any(|tail| said.ends_with(tail));
    Some((word, pronoun.end(), be))
}

fn names_a_thing(before: &str, after: &str) -> bool {
    if !after
        .trim_matches([' ', '.', ',', '!', '?', ';', ':'])
        .trim_matches(is_space)
        .is_empty()
    {
        return false;
    }
    let mut modifiers = split(before)
        .rev()
        .map(|word| word.trim_matches(['.', ',', '!', '?', ';', ':']))
        .filter(|word| !word.is_empty())
        .peekable();
    if modifiers
        .peek()
        .is_some_and(|word| matches!(*word, "a" | "an" | "the"))
    {
        return true;
    }
    for word in modifiers {
        if matches!(word, "a" | "an" | "the") {
            return true;
        }
        if has(HOW_MUCH_OF_IT, word) {
            return false;
        }
    }
    false
}

fn thing(phrase: &str) -> String {
    let phrase = split(phrase).collect::<Vec<_>>().join(" ");
    let phrase = phrase.trim_matches([' ', '.', ',', '!', '?', ';', ':']);
    let head = CAUSE_ENDS
        .find(phrase)
        .ok()
        .flatten()
        .map_or(phrase, |hit| &phrase[..hit.start()]);
    let head = head.trim_matches(is_space);
    if split(head).take(6).count() > 5 {
        String::new()
    } else {
        head.to_owned()
    }
}

fn tired_of(phrase: &str, if_nothing: &'static str) -> Feeling {
    let named = thing(phrase);
    if mentions(&THING_THAT_STAYS, &named) {
        Feeling::FedUp(named)
    } else {
        Feeling::Direct(if_nothing)
    }
}

static CUES: LazyLock<Vec<(&str, &str, Regex)>> = LazyLock::new(|| {
    FEELINGS
        .iter()
        .flat_map(|&(group, words)| {
            words.iter().map(move |&word| {
                let pattern =
                    Regex::new(&format!(r"\b{}\b", regex::escape(word))).expect("escaped mood cue");
                (group, word, pattern)
            })
        })
        .collect()
});

fn other_feeling(clean: &str, memory: &Memory) -> Option<OtherFeeling> {
    let body = FED_UP_TAIL.replace(clean, "");
    let asked = A_HELP_ASK_AT_END.replace_all(clean, "");
    let asked = asked.trim_matches(is_space);
    if clean.ends_with('?') && asked == clean
        || mentions(&OTHER_STOP, &body)
        || mentions(&OTHER_QUESTION, asked)
        || mentions(&CRY_WOLF, clean)
    {
        return None;
    }
    let (mut word, start, be) = who(clean, memory)?;
    let rest = &clean[start..];
    if mentions(&TO_ME, rest) {
        return None;
    }
    if !(has(SHE_WORDS, word) || has(HE_WORDS, word)) {
        if mentions(&SHE_IN_TEXT, rest) {
            word = "she";
        } else if mentions(&HE_IN_TEXT, rest) {
            word = "he";
        }
    }
    let person = pronouns(word);
    let direct = |feeling| OtherFeeling {
        feeling: Feeling::Direct(feeling),
        person,
    };
    if let Ok(Some(cause)) = FED_UP_A_THING.captures(rest) {
        return Some(OtherFeeling {
            feeling: tired_of(&cause[1], "annoyed"),
            person,
        });
    }
    if mentions(&OTHER_BAD_NIGHT, rest) {
        return Some(direct("tired"));
    }

    for &(group, cue, ref pattern) in CUES.iter() {
        let Some(hit) = pattern.find(rest) else {
            continue;
        };
        let before = &rest[..hit.start()];
        let after = &rest[hit.end()..];
        if names_a_thing(before, after) {
            continue;
        }
        let emphasis_only = OTHER_ADVERB
            .replace_all(before, "")
            .trim_matches(is_space)
            .is_empty();
        let speaking = emphasis_only && !be && has(OTHER_FEELING_VERB, cue);
        if !(emphasis_only && (speaking || be) || mentions(&OTHER_VERB, before)) {
            continue;
        }
        if !speaking && !cue_about_person(clean, cue) || has(WEARS_OTHER_PEOPLE_OUT, cue) {
            continue;
        }
        if mentions(&MY_FEELING, before) {
            return None;
        }
        if mentions(&OTHER_NEG, before) {
            continue;
        }
        if mentions(&WITH_PERSON, after) {
            return None;
        }
        if let Ok(Some(cause)) = HAS_A_CAUSE.captures(after) {
            if matches!(
                cue,
                "tired"
                    | "bored"
                    | "sick"
                    | "worn out"
                    | "done in"
                    | "fed up"
                    | "browned off"
                    | "peeved"
            ) {
                let target = if cue == "bored" { "bored" } else { "annoyed" };
                return Some(OtherFeeling {
                    feeling: tired_of(&cause[1], target),
                    person,
                });
            }
            if one_of(ABOUT_REPLIES, group).is_some() {
                let feeling = Feeling::About(group, thing(&cause[1]));
                return Some(OtherFeeling { feeling, person });
            }
            continue;
        }
        if cue == "low" && mentions(&LOW_IN, after) || cue == "spent" && mentions(&SPENT_ON, after)
        {
            continue;
        }
        if mentions(&JOINED_PERSON, &clean[..start + hit.start()]) {
            return Some(OtherFeeling {
                feeling: Feeling::Direct(group),
                person: Pronouns::Them,
            });
        }
        return Some(direct(group));
    }
    (mentions(&ROUGH_DAY, rest) && !mentions(&MY_BAD_DAY, rest)).then(|| direct("tired"))
}

fn words_for(other: OtherFeeling, said: &str, memory: &Memory) -> String {
    let (object, possessive, subject, plate, target, is, cap_is, does, cap_has) = match other.person
    {
        Pronouns::Her => (
            "her",
            "her",
            "she",
            "plate",
            "for one person to be carrying",
            "is",
            "Is",
            "does",
            "Has",
        ),
        Pronouns::Him => (
            "him",
            "his",
            "he",
            "plate",
            "for one person to be carrying",
            "is",
            "Is",
            "does",
            "Has",
        ),
        Pronouns::Them => (
            "them", "their", "they", "plates", "to carry", "are", "Are", "do", "Have",
        ),
    };
    let (pool, thing, lead) = match other.feeling {
        Feeling::Direct("sad") if memory.personal.sole_young_child_age().is_some() => {
            return compose::pick(OTHER_REPLIES_SMALL_CHILD, memory, said).to_owned();
        }
        Feeling::Direct(name) => (
            one_of(OTHER_REPLIES, name).unwrap_or(GENERIC_OTHER_REPLIES),
            String::new(),
            "",
        ),
        Feeling::About(name, thing) => (
            one_of(ABOUT_REPLIES, name).unwrap_or(GENERIC_OTHER_REPLIES),
            thing,
            "",
        ),
        Feeling::FedUp(thing) => {
            let captured = FED_UP_LEAD.captures(said).ok().flatten();
            let phrase = captured.as_ref().map_or("fed up", |parts| &parts[1]);
            let lead = one_of(FEDUP_LEAD, phrase).unwrap_or("Being fed up with");
            let pool = if thing.starts_with("being ") || thing.starts_with("having ") {
                FEDUP_GERUND_SAFE
            } else {
                FEDUP_REPLIES
            };
            (pool, thing, lead)
        }
    };
    let fields = [
        ("{obj}", object),
        ("{poss}", possessive),
        ("{subj}", subject),
        ("{plate}", plate),
        ("{target}", target),
        ("{subj_is}", is),
        ("{Subj_is}", cap_is),
        ("{subj_does}", does),
        ("{Subj_has}", cap_has),
        ("{thing}", &thing),
        ("{lead}", lead),
    ];
    let formatted: Vec<_> = pool
        .iter()
        .map(|line| {
            fields.iter().fold((*line).to_owned(), |line, (key, val)| {
                line.replace(key, val)
            })
        })
        .collect();
    let choices: Vec<_> = formatted.iter().map(String::as_str).collect();
    compose::pick(&choices, memory, "").replace("  ", " ")
}

pub(super) fn reply(clean: &str, repaired: &str, memory: &Memory) -> Option<String> {
    let feeling = other_feeling(clean, memory)?;
    Some(words_for(feeling, &flat(repaired), memory))
}
