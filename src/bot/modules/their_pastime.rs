//! A scoped conversation reply to a reported hobby, role or visit.

use std::sync::LazyLock;

use regex::{Captures, Regex};

use crate::bot::{Memory, Reading, compose, memory::occupations};

#[path = "their_pastime_data.rs"]
mod data;

const ADDITIONAL_PEOPLE: &str = "father in law|mother in law|son in law|daughter in law|sister in law|brother in law|friend|best friend|neighbour|neighbor|mate|boss|aunty|auntie|boyfriend|girlfriend|other half|flatmate|housemate|fella|missus|stepmum|stepdad|grandson|granddaughter";

fn fact_pattern(suffix: &str) -> Regex {
    Regex::new(&format!(
        r"^my (?P<person>{}|{ADDITIONAL_PEOPLE})(?:'?s)? {suffix}$",
        data::THEIR_PERSON
    ))
    .expect("whole-line fact about another person")
}

macro_rules! fact {
    ($name:ident, $pattern:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| fact_pattern($pattern));
    };
}

fact!(
    ROLE,
    r"(?:is|works as) (?:a|an) (?P<role>[a-z][a-z'\-]{2,20})"
);
fact!(
    INTO,
    r"(?:is|are) (?:really |well |properly |totally |so )?into (?P<thing>[a-z][a-z' \-]{1,22})"
);
fact!(
    LOVES,
    r"(?:loves|adores|likes|is obsessed with|is mad about|is nuts about) (?P<thing>[a-z][a-z' \-]{1,22})"
);
fact!(
    DOES,
    r"(?:plays|does|collects|makes|teaches|runs|swims|sings|paints|draws|bakes|cooks|gardens|reads|writes|dances) (?:in |with |for |at )?(?P<thing>[a-z][a-z' \-]{1,22})"
);
fact!(
    GOOD_AT,
    r"(?:is|are) (?:really |very |so |quite )?good at (?P<thing>[a-z][a-z' \-]{1,22})"
);
fact!(
    KEEPS,
    r"keeps (?:some |a few |two |three )?(?P<thing>[a-z]{2,12}s)"
);
fact!(
    TOOK_UP,
    r"(?:has|have) (?:just |recently )?(?:taken up|got into|started) (?P<thing>[a-z][a-z' \-]{1,22})"
);
fact!(
    VISIT,
    r"(?:is|are) (?:coming|visiting|staying) (?:on |at |this |next |for |in |over )[a-z0-9' \-]{1,24}"
);

static WORDS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9']+").expect("pastime words"));
static HEAVY_IN_THE_THREAD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::HEAVY_IN_THE_THREAD).expect("serious news"));

const PLACEHOLDER_OPENINGS: &[&str] = &[
    "noted",
    "i'm listening",
    "i am listening",
    "got it",
    "ok.",
    "that's noted",
    "thanks for telling me",
    "i don't know enough",
    "i dont know enough",
    "that's beyond what i know",
    "thats beyond what i know",
    "i'm not sure about that one",
    "im not sure about that one",
];

fn normalized(text: &str) -> String {
    let text = text.to_lowercase().replace('’', "'");
    WORDS
        .find_iter(&text)
        .map(|word| word.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalized(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn past_feeling(clean: &str, memory: &Memory) -> bool {
    if clean.split_whitespace().count() > 12 {
        return false;
    }
    let mut turns = memory.turns.iter().rev();
    let Some(mut prior) = turns.next() else {
        return false;
    };
    if matches!(
        normalized(&prior.user).as_str(),
        "dont know why" | "don't know why" | "i don't know why" | "dunno why" | "no idea why"
    ) && let Some(before) = turns.next()
    {
        prior = before;
    }
    let prior = Reading::new(&prior.user);
    prior
        .features
        .emotion
        .is_some_and(|emotion| emotion != "confused")
        && prior.features.polarity <= 0
}

fn problem(thing: &str) -> bool {
    data::NOT_TAKEN_UP_PHRASES
        .iter()
        .any(|phrase| thing.contains(phrase))
        || thing
            .split_whitespace()
            .any(|word| data::NOT_TAKEN_UP.contains(&word))
}

fn thing_pronoun(thing: &str) -> &'static str {
    if thing.split_whitespace().last().is_some_and(|word| {
        word.len() >= 4
            && word.ends_with('s')
            && !["ss", "us", "is", "ous"]
                .iter()
                .any(|suffix| word.ends_with(suffix))
            && !data::NOT_PLURAL.contains(&word)
    }) {
        "them"
    } else {
        "it"
    }
}

fn watched(thing: &str) -> bool {
    data::BEFORE_AN_AUDIENCE.contains(&thing)
        || thing
            .split_whitespace()
            .any(|word| data::BEFORE_AN_AUDIENCE.contains(&word))
}

fn fill(line: &str, person: &str, thing: &str) -> String {
    let (they, them, their, do_, is, has) = if data::SHE_PEOPLE.contains(&person) {
        ("she", "her", "her", "does", "is", "has")
    } else if data::HE_PEOPLE.contains(&person) {
        ("he", "him", "his", "does", "is", "has")
    } else {
        ("they", "them", "their", "do", "are", "have")
    };
    capitalized(
        &line
            .replace("{they}", they)
            .replace("{them}", them)
            .replace("{their}", their)
            .replace("{do}", do_)
            .replace("{is}", is)
            .replace("{has}", has)
            .replace("{it}", thing_pronoun(thing)),
    )
}

fn filled_pick(
    pool: &[&str],
    found: &Captures<'_>,
    thing: &str,
    job: Option<&str>,
    clean: &str,
    memory: &Memory,
) -> Option<String> {
    let person = found.name("person")?.as_str();
    let may_watch = watched(thing);
    let choices: Vec<String> = pool
        .iter()
        .filter(|line| may_watch || !line.contains("watch"))
        .map(|line| {
            let line = if let Some(job) = job {
                line.replace("{Job}", &capitalized(job))
            } else {
                (*line).to_owned()
            };
            fill(&line, person, thing)
        })
        .collect();
    let replies: Vec<&str> = choices.iter().map(String::as_str).collect();
    Some(compose::pick(&replies, memory, clean).to_owned())
}

fn read(clean: &str, memory: &Memory) -> Option<String> {
    if past_feeling(clean, memory) {
        return None;
    }
    if let Some(found) = ROLE.captures(clean) {
        let role = found.name("role")?.as_str();
        if occupations::is_job_word(role) {
            return filled_pick(data::ROLE_LINES, &found, "", Some(role), clean, memory);
        }
    }
    for (pattern, replies) in [
        (&*INTO, data::INTO_LINES),
        (&*LOVES, data::LOVES_LINES),
        (&*DOES, data::DOES_LINES),
        (&*GOOD_AT, data::GOOD_AT_LINES),
        (&*KEEPS, data::DOES_LINES),
        (&*TOOK_UP, data::DOES_LINES),
        (&*VISIT, data::VISIT_LINES),
    ] {
        if let Some(found) = pattern.captures(clean) {
            let thing = found.name("thing").map_or("", |thing| thing.as_str());
            if !problem(thing) {
                return filled_pick(replies, &found, thing, None, clean, memory);
            }
        }
    }
    None
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if reading.raw.contains('?') {
        return None;
    }
    let clean = normalized(reading.raw);
    if !clean.starts_with("my ")
        || memory.turns.last().is_some_and(|turn| {
            matches!(turn.module, "safety" | "grief" | "carers_turn")
                || HEAVY_IN_THE_THREAD.is_match(&turn.user.to_lowercase())
                || super::the_detail_the_thread_adds::heavy::thread_is_heavy(&turn.user)
        })
    {
        return None;
    }
    let reply = read(&clean, memory)?;
    if let Some(greeting) = compose::fallback::respond(reading, memory) {
        let greeting = greeting.trim().to_lowercase().replace('’', "'");
        if !PLACEHOLDER_OPENINGS
            .iter()
            .any(|start| greeting.starts_with(start))
        {
            return None;
        }
    }
    Some(reply)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn their_pastime_uses_whole_safe_statements_and_keeps_specific_replies() {
        let mut memory = Memory::default();
        for text in [
            "my sister is into yoga",
            "my brother plays the guitar",
            "my mother in law is coming on sunday",
            "my best friend is a vet",
            "my daughter loves dinosaurs",
            "my uncle keeps bees",
        ] {
            assert!(read(text, &memory).is_some(), "{text}");
        }
        for text in [
            "my son is into drugs",
            "my husband is into another woman",
            "my dad is a nightmare",
            "my mum is coming home from hospital",
        ] {
            assert!(read(text, &memory).is_none(), "{text}");
        }
        assert_eq!(thing_pronoun("dinosaurs"), "them");
        assert_eq!(thing_pronoun("chess"), "it");
        assert!(!watched("the guitar"));
        memory.turns.push(Exchange {
            user: "I feel really sad".into(),
            bot: String::new(),
            module: "chat",
        });
        assert!(read("my sister is into yoga", &memory).is_none());
        let turn = Bot::new(Some("0")).unwrap().turn("My sister is into yoga");
        assert!(turn.candidates.iter().any(|candidate| candidate.module == "their_pastime" && candidate.priority == 300.0), "{turn:?}");
    }
}
