//! Pick up a detail when the fallback composer would leave it unanswered.

use crate::bot::{Memory, Reading, compose};
use crate::text::is_space;
use regex::Regex;
use std::collections::BTreeSet;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "thread_detail_data.rs"]
mod data;
use data::*;

#[path = "thread_detail_heavy.rs"]
pub(in crate::bot) mod heavy;

static TIDY_WORDS: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    TIDY.iter()
        .map(|(pattern, word)| (Regex::new(pattern).expect("detail spelling pattern"), *word))
        .collect()
});

fn split(text: &str) -> impl Iterator<Item = &str> {
    text.split(is_space).filter(|word| !word.is_empty())
}

fn collapse(text: &str) -> String {
    split(text).collect::<Vec<_>>().join(" ")
}

fn matchable(text: &str) -> String {
    collapse(&text.to_lowercase().replace(['\'', '’'], ""))
}

fn uppercase_first(word: &str) -> String {
    let mut letters = word.chars();
    letters
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(letters)
        .collect()
}

fn clause(clean: &str) -> String {
    let mut text = clean.trim_matches([' ', '.', ',', '!', '?', '-']);
    while let Some(lead) = LEAD.find(text) {
        text = &text[lead.end()..];
    }
    let text = TIDY_WORDS
        .iter()
        .fold(text.to_owned(), |text, (pattern, word)| {
            pattern.replace_all(&text, *word).into_owned()
        });
    uppercase_first(&text)
}

fn second_person(clause: &str) -> String {
    split(clause)
        .enumerate()
        .map(|(at, word)| {
            let core = word.trim_end_matches([',', '.', '?', '!']);
            let replacement = match core.to_lowercase().replace('’', "'").as_str() {
                "i" | "me" => "you",
                "i'm" | "im" => "you're",
                "i've" | "ive" => "you've",
                "i'll" | "ill" => "you'll",
                "i'd" | "id" => "you'd",
                "my" => "your",
                "mine" => "yours",
                "myself" => "yourself",
                _ => return word.to_owned(),
            };
            let selected = if at == 0 {
                uppercase_first(replacement)
            } else {
                replacement.into()
            };
            format!("{selected}{}", &word[core.len()..])
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn detail(clean: &str) -> Option<[&'static str; 3]> {
    if A_LIMIT.is_match(clean) && A_PERSON.is_match(clean) && !A_MACHINE_VERB.is_match(clean) {
        Some(LIMIT_REPLIES)
    } else if AN_ATTEMPT.is_match(clean) {
        Some(ATTEMPT_REPLIES)
    } else if A_HISTORY.is_match(clean) {
        Some(HISTORY_REPLIES)
    } else {
        None
    }
}

fn detail_pool(clean: &str, memory: &Memory) -> Option<[String; 3]> {
    if split(clean).take(3).count() < 3 || clean.contains('?') || QUESTION_OPEN.is_match(clean) {
        return None;
    }
    let previous = memory.turns.last()?;
    let endings = detail(clean)?;
    let transcript = memory
        .turns
        .iter()
        .map(|turn| turn.user.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    if SERIOUS.is_match(&transcript) || heavy::heavy_news(&previous.user) {
        return None;
    }
    let clause = clause(clean);
    if split(&clause).take(3).count() < 3 {
        return None;
    }
    let said = second_person(&clause);
    Some(endings.map(|ending| format!("{said}{ending}")))
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    WORDS.find_iter(text).map(|word| word.as_str())
}

pub(in crate::bot) fn says_nothing_about_the_line(reply: &str, text: &str) -> bool {
    let mut parts = split(reply);
    if parts.next().is_none() {
        return true;
    }
    if parts.take(22).count() == 22 {
        return false;
    }
    if HEDGE.is_match(reply) {
        return true;
    }
    let reply = reply.to_lowercase();
    let text = text.to_lowercase();
    let mine: BTreeSet<_> = words(&text).collect();
    let said: BTreeSet<_> = words(&reply).collect();
    if !mine.is_empty() && mine.intersection(&said).count() * 5 >= mine.len() * 3 {
        return true;
    }
    let about: Vec<_> = mine
        .into_iter()
        .filter(|word| word.len() >= 4 && FRAME.binary_search(word).is_err())
        .collect();
    let stem = |word: &str| word.len().min(5);
    !about.is_empty()
        && !about.iter().any(|&word| {
            said.iter()
                .any(|&other| other.len() >= 4 && word[..stem(word)] == other[..stem(other)])
        })
}

/// Offer a single relevant question when a statement adds a detail to a previous turn.
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let source = if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    };
    let clean = matchable(&source.replace(['—', '–'], "-"));
    let options = detail_pool(&clean, memory)?;
    let fallback = super::greeting::respond(reading, memory)?;
    let typed = matchable(reading.raw);
    if !says_nothing_about_the_line(&fallback, &typed) {
        return None;
    }
    Some(compose::choose_recent(&options.each_ref().map(String::as_str), memory, &typed).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    const MODULE: &str = "the_detail_the_thread_adds";

    fn history(user: &str, bot: &str) -> Exchange {
        Exchange {
            user: user.into(),
            bot: bot.into(),
            module: "greeting",
        }
    }

    #[test]
    fn candidates_take_up_three_kinds_of_detail() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        for (line, expected) in [
            ("i called a plumber", None),
            (
                "i already told the council",
                Some("You already told the council, so what did they say?"),
            ),
            (
                "it was fine for two years",
                Some("It was fine for two years - how often is it happening now?"),
            ),
            (
                "she doesnt like surprises",
                Some("She doesn't like surprises - how far does the plan have to go round it?"),
            ),
            ("my bike wont start", None),
            ("did you call the plumber?", None),
        ] {
            let turn = bot.turn(line);
            let detail = turn
                .candidates
                .iter()
                .find(|candidate| candidate.module == MODULE);
            assert_eq!(detail.map(|candidate| candidate.reply.as_str()), expected);
            assert_eq!(
                detail.map(|candidate| candidate.priority),
                expected.map(|_| -999.0)
            );
        }
    }

    #[test]
    fn a_care_or_loss_thread_keeps_its_own_reply() {
        for prior in [
            "my uncle passed last year",
            "my cat was run over last night",
            "the doctor said there is nothing more they can do",
            "i have a biopsy on friday",
        ] {
            let mut memory = Memory::default();
            memory.turns.push(history(prior, ""));
            assert!(
                detail_pool("i already rang the council", &memory).is_none(),
                "{prior}"
            );
        }
        let mut memory = Memory::default();
        memory.turns.push(history("my laptop would not charge", ""));
        assert!(detail_pool("i already rang the council", &memory).is_some());
    }

    #[test]
    fn an_answer_that_names_the_subject_keeps_the_turn() {
        assert!(!says_nothing_about_the_line(
            "Plumbers can ask for photos before giving a quote.",
            "i already called the plumber",
        ));
        assert!(says_nothing_about_the_line(
            "You already called the plumber.",
            "i already called the plumber",
        ));
        assert!(says_nothing_about_the_line(
            "I'm listening - say more if it helps.",
            "it was fine for two years",
        ));
    }

    #[test]
    fn a_mention_of_their_food_limit_defers_to_the_composer() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        for line in [
            "I'm planning a surprise party for my grandma",
            "she doesnt like surprises",
            "one of us doesnt drink",
        ] {
            bot.turn(line);
        }
        let turn = bot.turn("my dad wont eat pork");
        assert!(
            !turn
                .candidates
                .iter()
                .any(|candidate| candidate.module == MODULE)
        );
        assert!(turn.candidates.iter().any(|candidate| {
            candidate.module == "greeting"
                && candidate.reply == "Fair enough — pork is not for him."
        }));
    }

    #[test]
    fn wording_preserves_plural_speakers_and_avoids_repeating_the_last_reply() {
        let mut memory = Memory::default();
        memory.turns.push(history("hello", "welcome"));
        let reading = Reading::new("we already tried that on saturday");
        let first = respond(&reading, &memory).unwrap();
        assert!(first.starts_with("We already tried that on Saturday"));
        memory.turns.push(history(reading.raw, &first));
        let second = respond(&reading, &memory).unwrap();
        assert_ne!(first, second);
    }
}
