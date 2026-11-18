//! The choice the person settled on and the question that recalls it.
use crate::bot::{Exchange, Memory, Reading, compose};
use crate::text::is_space;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! rule {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}
include!("decided_data.rs");

const EDGE: &[char] = &[' ', ',', '.', ';', ':', '!', '?'];
const QUESTION_EDGE: &[char] = &[' ', '?', '.', '!', ','];
const ARTICLES: &[&str] = &["a", "an", "the", "some", "my", "our"];
rule!(ONLY_NUMBER, r"^[\d.,]+$");

fn flat(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn low(text: &str) -> String {
    flat(text).to_lowercase()
}

fn has(items: &[&str], word: &str) -> bool {
    items.binary_search(&word).is_ok()
}

fn clean_thing(phrase: &str) -> Option<String> {
    let mut phrase = flat(phrase).trim_matches(EDGE).to_owned();
    for pattern in [&CORRECTING, &INNER_DECIDE, &TAIL] {
        while let std::borrow::Cow::Owned(kept) = pattern.replace(&phrase, "") {
            let kept = kept.trim_matches(EDGE);
            if kept == phrase {
                break;
            }
            phrase = kept.to_owned();
        }
    }
    let words: Vec<_> = phrase.split_whitespace().collect();
    let max_words = if verb_phrase(&phrase) { 8 } else { 4 };
    if words.is_empty()
        || words.len() > max_words
        || ONLY_NUMBER.is_match(&phrase)
        || has(IDIOM, &phrase)
        || words.iter().any(|word| has(NOT_A_THING, word))
    {
        return None;
    }
    Some(phrase)
}

fn verb_phrase(thing: &str) -> bool {
    thing
        .split_once(' ')
        .is_some_and(|(first, _)| first.eq_ignore_ascii_case("to"))
}

fn activity(thing: &str) -> bool {
    thing
        .split_whitespace()
        .map(|word| word.trim_matches(EDGE))
        .find(|word| !ARTICLES.contains(word))
        .is_some_and(|word| has(ACTIVITY, word))
}

fn as_typed(text: &str, thing: String) -> String {
    let pattern = format!(
        r"(?i)\b{}\b",
        thing
            .split_whitespace()
            .map(regex::escape)
            .collect::<Vec<_>>()
            .join(r"\s+")
    );
    Regex::new(&pattern)
        .ok()
        .and_then(|pattern| pattern.find(text))
        .map_or(thing, |matched| matched.as_str().to_owned())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Strength {
    Settled,
    Correction,
    NeedsOptions,
}

#[derive(Debug, PartialEq, Eq)]
struct Choice {
    what: String,
    strength: Strength,
}

fn captured(pattern: &Regex, line: &str) -> Option<String> {
    let found = pattern.captures(line)?;
    clean_thing(found.name("what")?.as_str())
}

fn read(text: &str) -> Option<Choice> {
    let mut line = low(text);
    if line.is_empty()
        || line.contains('?')
        || line.split_whitespace().count() > 14
        || ASK.is_match(&line)
    {
        return None;
    }
    while let std::borrow::Cow::Owned(kept) = LEAD_IN.replace(&line, "") {
        line = kept;
    }
    let groups: &[(&[&LazyLock<Regex>], Strength, bool)] = &[
        (&[&CLAUSE_CHOICE], Strength::Settled, true),
        (CORRECTING_LINE, Strength::Correction, false),
        (GOING, Strength::Settled, true),
        (SETTLING, Strength::Settled, false),
        (CHOOSING, Strength::NeedsOptions, false),
    ];
    for &(patterns, strength, ignores_activities) in groups {
        if let Some(thing) = patterns.iter().find_map(|pattern| {
            captured(pattern, &line).filter(|thing| !ignores_activities || !activity(thing))
        }) {
            return Some(Choice {
                what: as_typed(text, thing),
                strength,
            });
        }
    }
    None
}

fn bare_thing(line: &str) -> bool {
    let line = low(line);
    let line = line.trim_matches(EDGE);
    let words: Vec<_> = line.split_whitespace().collect();
    words.len() <= 2
        && words.last().is_some_and(|word| word.chars().count() > 2)
        && !line.contains('?')
        && words
            .iter()
            .all(|word| !has(NOT_A_THING, word) && word.chars().all(char::is_alphabetic))
}

fn has_options(turns: &[Exchange]) -> bool {
    turns.iter().rev().take(2).any(|turn| {
        let said = &turn.user;
        bare_thing(said) || (!asks(said) && OPTIONS.is_match(said))
    })
}

#[derive(Default)]
pub(crate) struct Decision(Option<Held>);
struct Held {
    what: String,
    line: String,
}

impl Decision {
    pub(super) fn observe(&mut self, text: &str, turns: &[Exchange]) {
        let Some(choice) = read(text) else { return };
        if choice.strength == Strength::Settled
            || (choice.strength == Strength::Correction && self.0.is_some())
            || has_options(turns)
        {
            self.0 = Some(Held {
                what: choice.what,
                line: low(text),
            });
        }
    }
}

pub(crate) fn asks(text: &str) -> bool {
    ASK.is_match(&low(text))
}

fn pair(said: &str) -> Option<String> {
    let line = low(said);
    let low = line.trim_matches(QUESTION_EDGE);
    let (left, right) = low
        .split_once(" between ")
        .and_then(|(_, tail)| tail.split_once(" and "))
        .or_else(|| low.rsplit_once(" or "))?;
    let is_filler = |word: &&str| has(OPTION_FILLER, word.trim_matches([',', ';', ':']));
    let left: Vec<_> = left.split_whitespace().skip_while(is_filler).collect();
    let mut right: Vec<_> = right.split_whitespace().take(3).collect();
    while right.last().is_some_and(is_filler) {
        right.pop();
    }
    let left = left[left.len().saturating_sub(3)..].join(" ");
    let right = right.join(" ");
    let edge = [' ', ',', ';', ':'];
    let left = left.trim_matches(edge);
    let right = right.trim_matches(edge);
    if [left, right]
        .iter()
        .any(|side| side.is_empty() || has(IDIOM, side) || has(NOT_A_THING, side))
    {
        return None;
    }
    Some(format!("{left} or {right}"))
}

fn undecided_pair(turns: &[Exchange], held: Option<&str>) -> Option<String> {
    if held == Some("") {
        return None;
    }
    let limit = if held.is_some() { 6 } else { 3 };
    turns
        .iter()
        .rev()
        .take(limit)
        .take_while(|turn| held.is_none_or(|line| low(&turn.user) != line))
        .find_map(|turn| {
            let said = &turn.user;
            if asks(said) || !OPTIONS.is_match(said) {
                return None;
            }
            pair(said)
        })
}

fn reached_for(turns: &[Exchange], held: Option<&str>) -> bool {
    turns
        .iter()
        .rev()
        .take(3)
        .find_map(|turn| {
            let said = flat(&turn.user);
            if said.is_empty()
                || asks(&said)
                || said.contains('?')
                || OPENING_QUESTION.is_match(&said)
            {
                return None;
            }
            Some(
                held.is_none_or(|line| said.to_lowercase() != line)
                    && REACHES_FOR_A_CHOICE.is_match(&said),
            )
        })
        .unwrap_or(false)
}

fn pending_activity(turns: &[Exchange]) -> bool {
    turns
        .iter()
        .rev()
        .take(3)
        .find_map(|turn| {
            let line = low(&turn.user);
            if line.is_empty() || asks(&line) || line.contains('?') {
                return None;
            }
            Some(
                GOING
                    .iter()
                    .find_map(|pattern| {
                        let found = pattern.captures(&line)?;
                        Some(
                            clean_thing(found.name("what")?.as_str())
                                .is_some_and(|thing| activity(&thing)),
                        )
                    })
                    .unwrap_or(false),
            )
        })
        .unwrap_or(false)
}

fn confirmed_plan(memory: &Memory) -> Option<String> {
    memory
        .stated_plans
        .plans()
        .chain(memory.stated_plans.latest())
        .rev()
        .find_map(|plan| {
            let plan = flat(plan);
            let words = plan.to_lowercase().replace(',', " ");
            let words: Vec<_> = words
                .split_whitespace()
                .filter(|word| word.chars().count() > 2)
                .collect();
            (!words.is_empty()
                && memory.turns.iter().any(|turn| {
                    let said = turn.bot.to_lowercase();
                    words.iter().all(|word| said.contains(word))
                }))
            .then_some(plan)
        })
}

fn guessed(line: &str) -> Option<String> {
    captured(&GUESSING_ASK, low(line).trim_matches(QUESTION_EDGE))
}

fn similar(guess: &str, held: &str) -> bool {
    let a = low(guess);
    let b = low(held);
    fn stripped(s: &str) -> &str {
        let s = s.trim_matches(QUESTION_EDGE);
        s.strip_prefix("to ").unwrap_or(s)
    }
    let a = stripped(&a);
    let b = stripped(&b);
    if a.is_empty() || b.is_empty() {
        return false;
    }
    if a.contains(b) || b.contains(a) {
        return true;
    }
    a.split_once(' ')
        .zip(b.split_once(' '))
        .is_some_and(|((a, rest), (b, other))| {
            rest.contains(' ')
                && rest == other
                && a.chars().count() >= 3
                && b.chars().count() >= 3
                && a.chars().take(3).eq(b.chars().take(3))
        })
}

fn other_subject(line: &str, held: &str) -> bool {
    let said = low(line);
    let held = low(held);
    if COMPANION.is_match(&said) {
        return true;
    }
    let mut names = said
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()))
        .filter(|word| {
            word.chars().count() > 2
                && !word.chars().all(char::is_numeric)
                && !has(ASK_FRAME, word)
                && !has(NOT_A_SUBJECT, word)
                && !has(DOING, word)
        })
        .peekable();
    names.peek().is_some()
        && !names.any(|word| {
            held.contains(word)
                || (word.chars().count() > 3 && word.chars().take(4).eq(held.chars().take(4)))
        })
}

fn perspective(thing: &str) -> String {
    thing
        .split_whitespace()
        .map(|word| {
            let bare = word
                .trim_matches(['.', ',', '!', '?', ';', ':'])
                .to_lowercase();
            TO_YOU
                .binary_search_by_key(&bare.as_str(), |&(key, _)| key)
                .map_or(word, |at| TO_YOU[at].1)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn open_choices(pair: &str) -> [String; 3] {
    ["Nothing settled yet:", "Not yet:", "Neither one yet:"].map(|head| format!("{head} {pair}."))
}

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let text = reading.raw;
    if !asks(text) {
        return None;
    }
    let choose = |replies: &[String]| {
        compose::pick(
            &replies.iter().map(String::as_str).collect::<Vec<_>>(),
            memory,
            text,
        )
        .to_owned()
    };
    let held = memory.decided.0.as_ref();
    if let Some(held) = held
        && let Some(pair) = undecided_pair(&memory.turns, Some(&held.line))
        && !reached_for(&memory.turns, Some(&held.line))
    {
        return Some(choose(&open_choices(&pair)));
    }
    let guess = guessed(text);
    let known = held.map_or("", |held| held.what.as_str());
    if guess.is_none() && other_subject(text, known) {
        return None;
    }
    let Some(held) = held else {
        let deciding = DECIDING_WORD.is_match(text);
        if deciding
            && let Some(plan) = confirmed_plan(memory)
            && !other_subject(text, &plan)
        {
            return Some(choose(&[
                format!("You settled on {plan}."),
                format!("That's settled: {plan}."),
                format!("The chat settled it: {plan}."),
            ]));
        }
        if reached_for(&memory.turns, None) && !(deciding && pending_activity(&memory.turns)) {
            return None;
        }
        if let Some(pair) = undecided_pair(&memory.turns, None) {
            return Some(choose(&open_choices(&pair)));
        }
        return Some(
            compose::pick(
                &[
                    "Nothing settled yet.",
                    "You haven't picked one yet.",
                    "You haven't settled on one yet.",
                ],
                memory,
                text,
            )
            .to_owned(),
        );
    };
    if reached_for(&memory.turns, Some(&held.line)) {
        return None;
    }
    let what = perspective(&held.what);
    let reply = match (verb_phrase(&held.what), guess) {
        (true, None) => choose(&[
            format!("You decided {what}."),
            format!("You decided {what}, in the end."),
        ]),
        (true, Some(guess)) if similar(&guess, &held.what) => choose(&[
            format!("That's right: you decided {what}."),
            format!("Yes - you decided {what}."),
        ]),
        (true, Some(_)) => choose(&[
            format!("Not quite: you decided {what}."),
            format!("The other one: you decided {what}."),
        ]),
        (false, Some(guess)) if similar(&guess, &held.what) => choose(&[
            format!("Yes - {what}, in the end."),
            format!("That's right: {what}."),
            format!("Yes, {what}."),
        ]),
        (false, Some(_)) => choose(&[
            format!("No - {what}, in the end."),
            format!("Not quite: {what}, in the end."),
            format!("The other one: {what}."),
        ]),
        (false, None) => choose(&[
            format!("{}, in the end.", capitalize(&what)),
            format!("You settled on {what}."),
            format!("You went with {what}."),
        ]),
    };
    Some(reply)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prior(text: &str) -> Exchange {
        Exchange {
            user: text.into(),
            bot: "I see.".into(),
            module: "greeting",
        }
    }

    #[test]
    fn files_choices_and_leaves_activities_unsettled() {
        let mut decision = Decision::default();
        decision.observe("I went for a walk", &[]);
        assert!(decision.0.is_none());
        decision.observe("No, a dog", &[]);
        assert!(decision.0.is_none());

        decision.observe(
            "let's go with Spain",
            &[prior("We're between Spain and Italy")],
        );
        let held = decision.0.as_ref().unwrap();
        assert_eq!(held.what, "Spain");
        assert_eq!(held.line, "let's go with spain");

        decision.observe("No, a dog", &[]);
        assert_eq!(decision.0.unwrap().what, "a dog");
    }

    #[test]
    fn separates_a_prior_choice_from_options_now_on_the_table() {
        let mut memory = Memory::default();
        memory.decided.observe("i picked the Orange Coat", &[]);
        memory.turns = vec![
            prior("i picked the Orange Coat"),
            prior("we were thinking italy or spain"),
        ]
        .into();
        let ask = Reading::new("which one did we pick");
        assert!(respond(&ask, &memory).unwrap().ends_with("italy or spain."));
        assert_eq!(
            respond(
                &Reading::new("what did i decide about the boiler"),
                &Memory::default()
            ),
            None
        );
    }
}
