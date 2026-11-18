//! Follow up on a gas emergency after a selected safety reply.

use super::{safety, the_detail_the_thread_adds::heavy};
use crate::bot::{Exchange, Memory, Reading, compose, memory};
use crate::text::is_space;
use fancy_regex::Regex;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "gas_followup_data.rs"]
mod data;
use data::*;

pattern!(EMERGENCY_WORDS, r"(?i)\b(?:999|911|emergency)\b");
pattern!(
    OUTSIDE,
    r"\b(?:i(?:'m|m| am)|we(?:'re| are)|they(?:'re| are)|he(?:'s| is)|she(?:'s| is))\s+(?:already\s+|all\s+)?(?:outside|out of the (?:house|flat|building)|out(?! of\b)|clear)\b"
);
pattern!(THEY_WORDS, r"\b(?:they're|them|their)\b");
pattern!(HE_WORDS, r"\b(?:he's|himself|him|his|he|them|their)\b");
pattern!(SHE_WORDS, r"\b(?:she's|herself|hers|her|she|them|their)\b");
pattern!(BODY_WORDS, r"[a-z]+\b");

fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

pub(crate) fn waiting(outside: bool) -> &'static str {
    if outside { GAS_AFTER } else { GAS_WAITING }
}

pub(crate) fn reports_outside(clean: &str) -> bool {
    matches(&OUTSIDE, clean)
}

fn lower(line: &str) -> String {
    line.to_lowercase()
        .replace(['\u{2018}', '\u{2019}'], "'")
        .split(is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(Clone, Copy)]
enum Person {
    She,
    He,
}

fn pronoun(line: &str) -> Option<Person> {
    if matches(&CHILD_ANON, line) {
        let words: Vec<_> = BODY_WORDS
            .find_iter(line)
            .flatten()
            .map(|word| word.as_str())
            .collect();
        let she = words.contains(&"she") || words.contains(&"her");
        if she
            && !["his", "i", "my", "me"]
                .iter()
                .any(|word| words.contains(word))
        {
            return Some(Person::She);
        }
        if !she && words.contains(&"his") {
            return Some(Person::He);
        }
    }
    match (matches(&FEMALE_WORDS, line), matches(&MALE_WORDS, line)) {
        (true, false) => Some(Person::She),
        (false, true) => Some(Person::He),
        _ => None,
    }
}

fn name_person(text: &str, person: Person) -> String {
    THEY_WORDS
        .replace_all(text, |parts: &fancy_regex::Captures<'_>| {
            match (person, &parts[0]) {
                (Person::She, "they're") => "she's",
                (Person::She, _) => "her",
                (Person::He, "they're") => "he's",
                (Person::He, "their") => "his",
                (Person::He, _) => "him",
            }
        })
        .into_owned()
}

fn corrected_person(text: &str, person: Person) -> String {
    match person {
        Person::She => HE_WORDS
            .replace_all(text, |parts: &fancy_regex::Captures<'_>| match &parts[0] {
                "he's" => "she's",
                "he" => "she",
                "himself" => "herself",
                _ => "her",
            })
            .into_owned(),
        Person::He => SHE_WORDS
            .replace_all(text, |parts: &fancy_regex::Captures<'_>| match &parts[0] {
                "she's" => "he's",
                "she" => "he",
                "herself" => "himself",
                "hers" | "their" => "his",
                _ => "him",
            })
            .into_owned(),
    }
}

const GENERIC: &[&[&str]] = &[
    WHEN_HELP_REPLY,
    FRIGHTENED_REPLY,
    CALL_HELP_REPLY,
    DETAIL_REPLY,
];

fn generic_reply(text: &str) -> bool {
    GENERIC
        .iter()
        .flat_map(|pool| pool.iter())
        .any(|candidate| {
            text == *candidate
                || [Person::She, Person::He]
                    .iter()
                    .any(|person| text == name_person(candidate, *person))
        })
}

pub(crate) struct GasHistory<'a> {
    last: &'a Exchange,
    at: usize,
    pub(crate) outside: bool,
    person: Option<Person>,
}

/// Recover the gas case through generic safety replies from the same incident.
pub(crate) fn gas_history(turns: &[Exchange]) -> Option<GasHistory<'_>> {
    let mut history = turns
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, turn)| turn.module == "safety");
    let (index, last) = history.next()?;
    let mut person = pronoun(&lower(&last.user));
    let mut current = (index, last);
    loop {
        let base = memory::unframed_reply(&current.1.bot).unwrap_or(&current.1.bot);
        if safety::is_initial_gas_reply(base) || matches!(base, GAS_AFTER | GAS_WAITING) {
            return Some(GasHistory {
                last,
                at: index,
                outside: base == GAS_AFTER,
                person,
            });
        }
        if !generic_reply(base) {
            return None;
        }
        let previous = history.next()?;
        if current.0 - previous.0 > 2 {
            return None;
        }
        current = previous;
        person = person.or_else(|| pronoun(&lower(&current.1.user)));
    }
}

pub(crate) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let gas = gas_history(&memory.turns)?;
    if memory.turns.len() - gas.at > 2 {
        return None;
    }
    let typed = lower(reading.raw);
    if heavy::only_a_dream(&typed) {
        return None;
    }
    let repaired = lower(&reading.line.plain);
    for line in [&typed, &repaired] {
        let named = pronoun(line);
        if let Some(pool) = [
            (&WHEN_HELP_COMES, WHEN_HELP_REPLY),
            (&FRIGHTENED, FRIGHTENED_REPLY),
            (&CALL_HELP, CALL_HELP_REPLY),
        ]
        .into_iter()
        .find_map(|(pattern, pool)| matches(pattern, line).then_some(pool))
        {
            let picked = compose::pick(pool, memory, "");
            return Some(
                named
                    .or(gas.person)
                    .map_or_else(|| picked.to_owned(), |person| name_person(picked, person)),
            );
        }
        if matches(&CORRECTION_OF_PERSON, line) && matches(&EMERGENCY_WORDS, &gas.last.bot) {
            return Some(named.map_or_else(
                || gas.last.bot.clone(),
                |person| corrected_person(&gas.last.bot, person),
            ));
        }
        if matches(&DETAIL_AFTER_ESCALATION, line) && !matches(&ESCALATED_ALREADY, line) {
            let picked = compose::pick(DETAIL_REPLY, memory, "");
            return Some(
                named
                    .or(gas.person)
                    .map_or_else(|| picked.to_owned(), |person| name_person(picked, person)),
            );
        }
        if !matches(&FOLLOW_UP, line)
            || matches(&HELP_ARRIVED, line) && !matches(&HELP_ASK_MORE, line)
            || !(matches(&FOLLOW_UP_ANCHOR, line) || matches(&CASE_WORDS, line))
        {
            continue;
        }
        let outside = gas.outside || matches(&SAFE_NOW, line);
        return Some(waiting(outside).to_owned());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn the_settled_dream_preserves_the_last_card_for_a_recent_correction() {
        let mut bot = Bot::new(Some("0")).unwrap();
        for line in [
            "HELLO!!!",
            "   ",
            "...?!",
            "hiiii ☀️",
            "Salut, je m’appelle Léa.",
            "i dnt knw wat 2 do",
            "I'm sad and I just need someone to listen. No advice please.",
        ] {
            bot.turn(line);
        }
        let first = bot.turn("I smell gas in my kitchen. What should I do?");
        assert_eq!(first.module, Some("safety"));
        let settled = bot.turn("It was in a dream, I am safe now.");
        assert_eq!(settled.module, Some("greeting"));
        assert_eq!(
            settled.reply,
            "Good — that's the main thing. Keep an eye on it for a bit, and ring again if anything changes."
        );
        assert!(
            settled
                .candidates
                .iter()
                .all(|proposal| proposal.module != "safety")
        );
        let correction = bot.turn("No, I meant the word ‘dream’.");
        assert_eq!(correction.module, Some("safety"));
        assert_eq!(correction.reply, first.reply);
        for line in ["/help", "/reset", "\tthanks\t"] {
            assert!(
                bot.turn(line)
                    .candidates
                    .iter()
                    .all(|proposal| proposal.module != "safety")
            );
        }
    }

    #[test]
    fn a_correction_does_not_repeat_an_aged_out_gas_card() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(
            bot.turn("I smell gas in my kitchen.").module,
            Some("safety")
        );
        bot.turn("What should I cook tonight?");
        bot.turn("How much is council tax?");
        let correction = bot.turn("No, I meant the meter.");
        assert!(
            correction
                .candidates
                .iter()
                .all(|proposal| proposal.module != "safety")
        );
    }

    #[test]
    fn getting_out_changes_the_follow_up_and_stays_in_selected_history() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(
            bot.turn("I smell gas in my kitchen.").module,
            Some("safety")
        );
        let outside = bot.turn("I'm outside now.");
        assert_eq!(outside.module, Some("safety"));
        assert_eq!(outside.reply, waiting(true));
        let follow = bot.turn("What do I do now?");
        assert_eq!(follow.module, Some("safety"));
        assert_eq!(
            follow.reply,
            format!("Same answer, and it covers this: {}", waiting(true))
        );
        let later = bot.turn("Operator said wait, what now?");
        assert_eq!(later.module, Some("safety"));
        assert_eq!(
            later.reply,
            format!("That is still the thing to do: {}", waiting(true))
        );
    }
}
