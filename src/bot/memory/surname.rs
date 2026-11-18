//! Surname statements and recall, based on PyBot's memory/surname.py.
use super::again;
use crate::bot::{Exchange, Memory};
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

const NAME_WORD: &str =
    r"(?:surname|sur-name|last\s*-?\s*name|family\s*-?\s*name|second\s*-?\s*name)";
const NAME_PARTS: &str =
    "van von der den de da di du dos das del della bin ibn al el la le mc mac st saint o o' ap";
const LOWER_PARTS: &str =
    "van von der den de da di du dos das del della bin ibn al el la le st saint";
const DESCRIBES_A_NAME: &str = "
    hard difficult easy long short weird strange common rare unusual foreign hyphenated double
    spelled spelt misspelled misspelt mispelled changed change changing wrong right same different
    pronounced unpronounceable tricky silly complicated confusing correct incorrect spelled okay ok
    also really actually just still not now too boring ugly nice lovely beautiful awful terrible funny
    embarrassing stupid odd plain old big small";
const NEVER_A_NAME: &str = "
    on in at from of for to with about under over after before by into onto near above below
    and or but if so that this it there when how why the a an";

static SURNAME_WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"(?i)\b{NAME_WORD}\b")).unwrap());
static GIVES_SURNAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"(?i)\b(?:(?:my|our|the)\s+)?{NAME_WORD}\s*(?:is|was|'s|s|:|=)\s*(?P<name>[A-Za-z][A-Za-z'’\-]{{1,30}}(?:\s+[A-Za-z][A-Za-z'’\-]{{1,30}}){{0,3}})"
    ))
    .unwrap()
});

static SURNAME_ASK: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        format!(r"(?i)\b(?:what|whats|what's|which)\s+(?:is\s+|was\s+|were\s+)?(?:my|our|the)?\s*{NAME_WORD}\b"),
        format!(r"(?i)\b(?:which|what)\s+{NAME_WORD}\s+(?:is|was)\s+(?:mine|ours)\b"),
        format!(r"(?i)\b(?:do|did|can|could)\s+(?:you|u|ya)\s+(?:still\s+)?(?:know|remember)\s+(?:what\s+)?(?:my|our)\s+{NAME_WORD}\b"),
        format!(r"(?i)\b(?:tell|remind)\s+(?:me|us)\s+(?:what\s+)?(?:my|our)\s+{NAME_WORD}\b"),
        format!(r"(?i)\b(?:my|our)\s+{NAME_WORD}\s+again\b"),
        format!(r"(?i)\bwhat\s+(?:did|do)\s+i\s+(?:say|tell\s+you)\s+(?:my|our)\s+{NAME_WORD}\b"),
        format!(r"(?i)^(?:and\s+|so\s+)?(?:my|our)?\s*{NAME_WORD}\s*\??$"),
    ])
    .unwrap()
});

macro_rules! rule {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($source).unwrap());
    };
}
rule!(
    NOT_THE_ASK,
    r"(?i)\b(?:how|change|changing|changed|deed\s+poll|register|registering|certificate|passport|marri(?:ed|age|ying)|divorce|spell|spelt|spelled|pronounce|website|domain|form|online)\b"
);
rule!(
    FIRST_NAME_ASK,
    r"(?i)\b(?:what|whats|what's|which|do|does|did|know|remember|again|tell)\b[^?.!]*\b(?:first\s*-?\s*name|given\s*-?\s*name)\b"
);
rule!(
    FULL_NAME_ASK,
    r"(?i)\b(?:what|whats|what's|which|do|does|did|know|remember|again|tell)\b[^?.!]*\bfull\s*-?\s*name\b"
);
rule!(
    ASKS_AGAIN,
    r"(?i)\b(?:what|whats|what's|which|tell|remind|again|said)\b"
);
rule!(POINTS_BACK, r"(?i)\b(?:it|that|them|those)\b");

fn word_in(words: &str, word: &str) -> bool {
    words.split_whitespace().any(|known| known == word)
}

fn write_name(parts: &[&str]) -> String {
    let mut result = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            result.push(' ');
        }
        let lowered = part.to_ascii_lowercase();
        if index < parts.len() - 1 && word_in(LOWER_PARTS, &lowered) {
            result.push_str(&lowered);
            continue;
        }
        let mut capital = true;
        for ch in part.chars() {
            if capital {
                result.extend(ch.to_uppercase());
            } else {
                result.push(ch);
            }
            capital = matches!(ch, '\'' | '’');
        }
    }
    result
}

pub(super) fn given(text: &str) -> Option<String> {
    GIVES_SURNAME.captures_iter(text).find_map(|found| {
        let mut words = found.name("name")?.as_str().split_whitespace();
        let first = words.next()?;
        let low = first.to_ascii_lowercase();
        if word_in(DESCRIBES_A_NAME, &low) || word_in(NEVER_A_NAME, &low) {
            return None;
        }
        let mut parts = vec![first];
        let mut previous_part = word_in(NAME_PARTS, &low);
        for word in words {
            let is_part = word_in(NAME_PARTS, &word.to_ascii_lowercase());
            if is_part
                || word.starts_with(char::is_uppercase)
                || (previous_part && word.chars().all(char::is_alphabetic))
            {
                parts.push(word);
                previous_part = is_part;
            } else {
                break;
            }
        }
        Some(write_name(&parts))
    })
}

fn vary(replies: &[String], memory: &Memory) -> String {
    let options: Vec<_> = replies.iter().map(String::as_str).collect();
    again::vary(&options, memory)
}

fn full_name(first: Option<&str>, last: Option<&str>, memory: &Memory) -> String {
    match (first, last) {
        (Some(first), Some(last)) => vary(
            &[
                format!("{first} {last}."),
                format!("You're {first} {last}."),
                format!("{first} {last} — that's both of the names you gave."),
            ],
            memory,
        ),
        (Some(first), None) => {
            format!("I have your first name, {first}, but you haven't given me a surname.")
        }
        (None, Some(last)) => {
            format!("I have your surname, {last}, but you've not told me your first name.")
        }
        (None, None) => again::vary(
            &[
                "You haven't told me your name yet.",
                "I don't have a name from you — you've not said.",
            ],
            memory,
        ),
    }
}

fn first_name(first: Option<&str>, memory: &Memory) -> String {
    first.map_or_else(
        || {
            again::vary(
                &[
                    "You haven't told me your first name yet.",
                    "I don't have it — you've not said your first name.",
                ],
                memory,
            )
        },
        |first| {
            vary(
                &[
                    format!("{first} — you told me."),
                    format!("It's {first}."),
                    format!("Your first name's {first}."),
                ],
                memory,
            )
        },
    )
}

pub(super) fn awaits_given_name(turn: &Exchange) -> bool {
    turn.module == "memory_store"
        && FULL_NAME_ASK.is_match(&turn.user)
        && turn.bot == "You haven't told me your name yet."
}

pub(super) fn respond(text: &str, held: Option<&str>, memory: &Memory) -> Option<String> {
    let said = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if let Some(held) = held
        && let Some(found) = GIVES_SURNAME.find(&said)
        && found.start() == 0
        && said[found.end()..]
            .trim_matches([' ', '.', '!', '?', ',', ';', '-'])
            .is_empty()
        && given(&said).is_some()
    {
        return Some(vary(
            &[
                format!("{held} — noted."),
                format!("Got it — {held}."),
                format!("{held}, I'll remember that."),
            ],
            memory,
        ));
    }
    if !memory.turns.iter().any(|turn| !turn.user.trim().is_empty()) {
        return None;
    }
    let first = memory.name.as_deref();
    if FULL_NAME_ASK.is_match(&said) {
        return Some(full_name(first, held, memory));
    }
    if FIRST_NAME_ASK.is_match(&said) {
        return Some(first_name(first, memory));
    }
    if NOT_THE_ASK.is_match(&said) {
        return None;
    }
    let count = said.split_whitespace().count();
    let mentions_surname = SURNAME_WORD.is_match(&said);
    if mentions_surname && count <= 12 && SURNAME_ASK.is_match(&said) {
        if given(&said).is_some() {
            return None;
        }
        return Some(held.map_or_else(
            || {
                again::vary(
                    &[
                        "You haven't told me your surname yet.",
                        "I don't have it — you've not said your surname.",
                        "You haven't given me a surname, so I'd only be guessing.",
                    ],
                    memory,
                )
            },
            |held| {
                vary(
                    &[
                        format!("It's {held}."),
                        format!("{held} — that's the one you gave me."),
                        format!("Your surname's {held}."),
                    ],
                    memory,
                )
            },
        ));
    }
    let held = held?;
    let in_context = memory
        .turns
        .iter()
        .rev()
        .filter(|turn| !turn.user.trim().is_empty())
        .take(3)
        .any(|turn| SURNAME_WORD.is_match(&turn.user) || SURNAME_WORD.is_match(&turn.bot));
    if mentions_surname
        || count > 7
        || !ASKS_AGAIN.is_match(&said)
        || !POINTS_BACK.is_match(&said)
        || !in_context
    {
        return None;
    }
    Some(vary(
        &[
            format!("It's {held}."),
            format!("{held} — as you told me."),
            format!("Still {held}."),
        ],
        memory,
    ))
}

#[cfg(test)]
mod tests {
    fn memory_reply(turn: &crate::Turn) -> Option<&str> {
        turn.candidates
            .iter()
            .find(|candidate| candidate.module == "memory_store")
            .map(|candidate| candidate.reply.as_str())
    }

    #[test]
    fn first_family_and_full_names_are_recalled_separately() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My name is Sam.");
        for (line, expected) in [
            ("My last name is okafor.", "Okafor — noted."),
            ("What's my surname?", "It's Okafor."),
            ("What's my first name?", "Sam — you told me."),
            ("What's my full name?", "Sam Okafor."),
            ("And what was it again?", "Still Okafor."),
        ] {
            let turn = bot.turn(line);
            assert_eq!(turn.module, Some("memory_store"), "{line}");
            assert_eq!(turn.reply, expected, "{line}");
        }
        assert_eq!(memory_reply(&bot.turn("How do I change my surname?")), None);
    }

    #[test]
    fn name_descriptions_keep_the_latest_explicit_surname() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        for (line, expected) in [
            (
                "My surname is van der berg.",
                Some("Got it — van der Berg."),
            ),
            ("What is my surname?", Some("It's van der Berg.")),
            ("My surname is hard to spell.", None),
            (
                "What is my surname again?",
                Some("van der Berg — that's the one you gave me."),
            ),
            ("My family name is o'brien.", Some("O'Brien — noted.")),
            (
                "Can you remember my last name?",
                Some("Your surname's O'Brien."),
            ),
            ("My last name is on the form.", None),
            (
                "What is my last name?",
                Some("O'Brien — that's the one you gave me."),
            ),
        ] {
            assert_eq!(memory_reply(&bot.turn(line)), expected, "{line}");
        }
    }

    #[test]
    fn a_missing_full_name_can_prompt_a_bare_first_name() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("Hello");
        assert_eq!(
            bot.turn("What's my full name?").reply,
            "You haven't told me your name yet."
        );
        bot.turn("sam");
        assert_eq!(
            bot.turn("What's my first name?").reply,
            "Sam — you told me."
        );
        assert_eq!(
            bot.turn("What's my full name?").reply,
            "I have your first name, Sam, but you haven't given me a surname."
        );
    }
}
