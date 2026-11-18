//! Route a plain statement of trouble to the person who can settle it.

use super::{has, matches};
use crate::bot::{Memory, Reading};

#[path = "trouble_advisor_data.rs"]
mod data;
use data::*;

fn form(word: &str, evidence: &str) -> bool {
    if word == evidence {
        return true;
    }
    if evidence.chars().count() < 4 || has(NOT_A_FORM, word) {
        return false;
    }
    let Some(tail) = word.strip_prefix(evidence) else {
        return false;
    };
    has(ENDINGS, tail)
        || evidence
            .chars()
            .next_back()
            .and_then(|last| tail.strip_prefix(last))
            .is_some_and(|rest| has(ENDINGS, rest))
}

struct Evidence<'a> {
    text: &'a str,
    words: &'a [&'a str],
}

impl Evidence<'_> {
    fn names(&self, evidence: &[&str]) -> bool {
        evidence.iter().any(|term| self.names_one(term))
    }

    fn names_one(&self, term: &str) -> bool {
        if term.contains(' ') {
            self.text.contains(term)
        } else {
            self.words.iter().any(|word| form(word, term))
        }
    }
}

/// Gas signals used by the upstream fallback's action reader.
pub(in crate::bot::compose) fn gas_action(clean: &str) -> bool {
    let text: String = clean
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '\'' {
                c
            } else {
                ' '
            }
        })
        .collect();
    let stripped = text.replace('\'', "");
    let words: Vec<_> = stripped.split_ascii_whitespace().collect();
    let evidence = Evidence {
        text: &text,
        words: &words,
    };
    !evidence.names(&["fluent", "fluency", "fluently", "fluencies"])
        && !evidence.names(&[
            "petrol",
            "diesel",
            "pump",
            "forecourt",
            "filling",
            "fuel",
            "car",
            "van",
            "station",
        ])
        && (evidence.names(&[
            "carbon monoxide",
            "flue",
            "smell of gas",
            "smells of gas",
            "smell gas",
            "smelling gas",
            "gas leak",
            "gas smell",
            "co alarm",
            "gas escape",
            "gas emergency",
        ]) || evidence.names(&["fumes", "fume"])
            && evidence.names(&[
                "boiler",
                "cooker",
                "hob",
                "fire",
                "fireplace",
                "firewood",
                "heater",
                "gas",
                "oven",
                "chimney",
                "flue",
                "woodburner",
                "stove",
            ]))
}

fn grown_up(clean: &str, evidence: &Evidence<'_>) -> bool {
    if evidence.names(GROWN_UP_WORDS) {
        return true;
    }
    let original = format!(" {clean} ");
    let mut left = original.clone();
    for name in BABY_FOOD_NAMES {
        left = left.replace(&format!(" {name} "), " ");
    }
    if left == original || !evidence.names(SELF_EATING_WORDS) {
        return false;
    }
    let words: Vec<_> = super::split(&left).collect();
    !Evidence {
        text: &left,
        words: &words,
    }
    .names(CHILD_AT_HOME)
}

fn who_for(clean: &str, said: &str, words: &[&str], topic: Option<&str>) -> Option<&'static str> {
    if matches(&ASKS_DEFINITION, clean) || matches(&ASKS_ARITHMETIC, clean) {
        return None;
    }
    let evidence = Evidence { text: said, words };
    let names = |terms: &[&str]| evidence.names(terms);
    if names(SCHOOL_WORDS) && names(SCHOOL_TROUBLE) {
        return Some(SCHOOL_WHO);
    }
    if evidence.names_one("password") && names(PASSWORD_TROUBLE) {
        return Some(DEVICE_WHO);
    }
    if names(COMPLAINT_WORDS) && names(TRADER_WORDS) {
        return Some(COMPLAIN_WHO);
    }
    let animal = words
        .iter()
        .any(|word| has(ANIMALS, word) || has(ANIMALS, word.trim_end_matches('s')))
        && !(names(WILD_BIRDS) && names(MESS_PLACES));
    let meds = names(MEDS_WORDS);
    if !animal && meds {
        return Some(HEALTH_WHO);
    }
    if !animal
        && names(FILLING_WORDS)
        && (names(MOUTH_WORDS)
            || !(topic.is_some_and(|topic| has(FOOD_TOPICS, topic)) || names(FILLING_FOODS)))
    {
        return Some(DENTIST_WHO);
    }

    let where_is_a_thing = matches(&ASKS_WHERE, clean) && !names(WHERE_SOMETHING_WRONG);
    let machine_mouse = super::super::loss_reader::names_a_mouse_device(clean);
    let diy = matches(&ASKS_DIY, clean);
    let gift = names(GIFT_WORDS);
    let buying = names(BUYING_WORDS);
    for &(terms, who) in WHO_TO_ASK {
        if (where_is_a_thing || gift) && matches!(who, BABY_WHO | HEALTH_WHO) {
            continue;
        }
        if machine_mouse && who == PEST_WHO {
            continue;
        }
        if animal
            && (who == DENTIST_WHO
                || who != VET_WHO && terms.iter().any(|term| has(SICKNESS, term)))
        {
            continue;
        }
        if who == HEALTH_WHO
            && (matches(&SCHEDULING, clean)
                || matches(&CHOOSING_BETWEEN_TWO, clean)
                || names(JOINT_WORDS)
                    && !matches(&ASKS_ABOUT_THE_JOINT, clean)
                    && !terms
                        .iter()
                        .any(|term| !has(JOINT_WORDS, term) && evidence.names_one(term)))
        {
            continue;
        }
        if diy && matches!(who, TRADES_WHO | DEVICE_WHO) {
            continue;
        }
        if who == BABY_WHO && (grown_up(clean, &evidence) || !names(CHILD_AT_HOME)) {
            continue;
        }
        if who == DEVICE_WHO && (meds || names(NETWORK_WORDS)) {
            continue;
        }
        if who == TENANT_WHO && buying {
            continue;
        }
        if who == WORK_WHO && !names(WORK_ABOUT) {
            continue;
        }
        if names(terms) {
            return Some(who);
        }
    }
    (animal && (meds || names(SICKNESS))).then_some(VET_WHO)
}

pub(crate) fn who_settles(reading: &Reading<'_>) -> Option<&'static str> {
    static MARK: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new("[^a-z0-9']").unwrap());
    let clean = &reading.line.clean;
    let said = format!(" {} ", MARK.replace_all(clean, " "));
    let words: Vec<_> = reading.line.words.iter().map(String::as_str).collect();
    who_for(clean, &said, &words, reading.topic.map(|(topic, _)| topic))
}

pub(in crate::bot::compose) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let words: Vec<_> = reading.line.words.iter().map(String::as_str).collect();
    if !words
        .iter()
        .any(|word| TROUBLE_CUES.iter().any(|cue| word.starts_with(cue)))
    {
        return None;
    }
    let clean = &reading.line.plain;
    let said = format!(" {} ", words.join(" "));
    let who = who_for(clean, &said, &words, reading.topic.map(|(topic, _)| topic))?;
    if who == HEALTH_WHO && !super::super::fallback::asks_a_question(clean, clean, &words) {
        return None;
    }
    let options = UNKNOWN_WHO.map(|line| line.replace("{who}", who));
    let choices = [options[0].as_str(), options[1].as_str()];
    Some(super::super::choose_recent(&choices, memory, clean).to_owned())
}

pub(in crate::bot) fn names_a_route(reply: &str) -> bool {
    WHO_TO_ASK
        .iter()
        .any(|&(_, who)| reply.trim_start().starts_with(who))
}

/// Preserve the composer’s named UK advice route for a request to talk it through.
pub(in crate::bot) fn conversation_referral(
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<String> {
    let who = who_settles(reading)?;
    if !["Citizens Advice", "MoneyHelper"]
        .iter()
        .any(|name| who.contains(name))
    {
        return None;
    }
    let options = UNKNOWN_WHO.map(|line| line.replace("{who}", who));
    let choices = [options[0].as_str(), options[1].as_str()];
    Some(super::super::choose_recent(&choices, memory, &reading.line.clean).to_owned())
}
