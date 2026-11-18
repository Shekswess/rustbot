//! Trouble types, their first and next questions, and the gates that keep other answers.

use crate::bot::{
    Memory, Reading,
    knowledge::lookup,
    modules::{chat, the_detail_the_thread_adds::heavy},
};
use crate::text::is_space;
use fancy_regex::{Match, Regex};
use std::borrow::Cow;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $pattern:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| {
                fancy_regex::Regex::new($pattern).expect(stringify!($name))
            });
    };
}

#[path = "trouble_data.rs"]
mod data;
use data::*;

#[path = "trouble_advisor.rs"]
pub(in crate::bot) mod advisor;
#[path = "trouble_machine.rs"]
pub(in crate::bot) mod machine;
#[path = "trouble_mood.rs"]
mod mood;
pub(super) use mood::turns_round;
pub(crate) use mood::{reads_like_symptom, without_sick_figure};
#[path = "trouble_named.rs"]
pub(in crate::bot) mod named;
#[path = "trouble_setback.rs"]
mod setback;
#[path = "trouble_taste.rs"]
mod taste;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Shape {
    Thing,
    Sore,
    Strain,
    Vague,
}

fn split(text: &str) -> impl DoubleEndedIterator<Item = &str> + Clone {
    text.split(is_space).filter(|word| !word.is_empty())
}

fn words(text: &str) -> impl Iterator<Item = &str> + '_ {
    static WORD: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"[a-z']+").unwrap());
    WORD.find_iter(text).map(|word| word.as_str())
}

fn flat(line: &str) -> String {
    split(line).collect::<Vec<_>>().join(" ").to_lowercase()
}

fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn found<'a>(pattern: &Regex, text: &'a str) -> Option<Match<'a>> {
    pattern.find(text).ok().flatten()
}

fn has(vocabulary: &[&str], word: &str) -> bool {
    vocabulary.binary_search(&word).is_ok()
}

fn includes(words: &[&str], vocabulary: &[&str]) -> bool {
    words.iter().any(|word| has(vocabulary, word))
}

fn another(word: &str) -> bool {
    has(OTHERS, word)
        || OWNER_END.iter().any(|end| {
            word.strip_suffix(end)
                .is_some_and(|base| base.chars().count() > 2 && has(OTHERS, base))
        })
}

pub(crate) fn someone_elses(words: &[&str]) -> bool {
    words.iter().any(|word| another(word))
}

fn someone_is_subject(text: &str) -> bool {
    let words: Vec<_> = words(text).collect();
    words.iter().enumerate().any(|(at, word)| {
        another(word)
            && (!has(ANIMAL_MODIFIERS, word)
                || words
                    .get(at + 1)
                    .is_none_or(|next| has(NOT_A_MODIFIER, next)))
    })
}

fn fault_of_a_thing(text: &str) -> bool {
    let padded = format!(" {text} ");
    matches(&THING, &padded) && matches(&OWNS, &padded) && !someone_is_subject(text)
}

fn heads_of_faults(text: &str) -> Vec<&str> {
    [&*THING, &*CANT_GET, &*FLAT]
        .iter()
        .flat_map(|pattern| {
            pattern.find_iter(text).flatten().filter_map(|one| {
                words(&text[..one.start()])
                    .filter(|word| !has(FAULT_FILLERS, word))
                    .last()
            })
        })
        .collect()
}

fn before_noun(text: &str, at: usize) -> (&str, Vec<&str>) {
    let mut before: Vec<_> = words(&text[..at]).collect();
    while before.last().is_some_and(|word| has(JOINING, word)) {
        before.pop();
    }
    (before.pop().unwrap_or(""), before)
}

fn owned_noun(text: &str, at: usize) -> &str {
    let (noun, before) = before_noun(text, at);
    for word in before.into_iter().rev().take(3) {
        if matches!(word, "my" | "our") {
            return noun;
        }
        if has(NOT_OWNED, word) {
            return "";
        }
    }
    ""
}

fn keeps_doing_it(text: &str) -> bool {
    if matches(&ASKS_TO_CHOOSE, text) {
        return false;
    }
    KEEPS_AT_IT.captures_iter(text).flatten().any(|capture| {
        let Some(doing) = capture.get(1) else {
            return false;
        };
        if has(KEEPS_FINE, doing.as_str()) {
            return false;
        }
        let before: Vec<_> = words(&text[..capture.get(0).unwrap().start()]).collect();
        if !includes(&before[before.len().saturating_sub(3)..], KEEPS_THEIRS) {
            return false;
        }
        let phrase: Vec<_> = before
            .into_iter()
            .filter(|word| !has(FAULT_FILLERS, word) && !has(NOT_THE_THING, word))
            .collect();
        let Some(&head) = phrase.last() else {
            return false;
        };
        !has(OTHERS, head)
            && !has(NOT_A_THING, head)
            && !has(BODY_PART, head)
            && !has(KEEPS_NOT_A_THING, head)
            && !has(KEEPS_NOT_THEIR_THING, head)
            && matches(&OWNS, text)
            && (includes(&phrase, KEEPS_POSSESSIVE) || includes(&phrase, KEEPS_OF_A_HOUSE))
    })
}

fn shape(clean: &str, supplied: &[&str]) -> Option<Shape> {
    static ACHES: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
        r"\b(?:dead|killing|kills|hurts?|banging|thumping|pounding|splitting|had\s+it|done\s+for)\b"
    ).unwrap()
    });
    let text = flat(clean);
    if text.is_empty() || text.contains('?') {
        return None;
    }
    let mapped = supplied
        .is_empty()
        .then(|| words(&text).collect::<Vec<_>>());
    let terms = mapped.as_deref().unwrap_or(supplied);
    let mine = format!(" {text} ");
    let state = found(&THING_STATE_ONLY, &mine);
    let growing = state.is_some_and(|one| has(GROWING, before_noun(&mine, one.start()).0));
    let body = includes(terms, BODY_PART);
    let thing_state = state.is_some() && !growing && !body;
    if someone_elses(terms) && !fault_of_a_thing(&text)
        || matches(&PLEA, &text)
        || matches(&DEATHWISH, &text)
        || matches(&ACUTE, &text) && !thing_state
        || matches(&FRACTURE, &text) && (body || includes(terms, BONE))
        || matches(&BREAKS, &text)
            && (includes(terms, NOT_A_THING) || includes(&heads_of_faults(&text), NOT_A_THING))
    {
        return None;
    }
    let ache = !thing_state && matches(&BODY, &mine) || body && matches(&ACHES, &mine);
    if matches(&MINE, &text) && ache {
        let figurative = matches(&FIGURATIVE, &mine)
            && matches(&NUISANCE, &text)
            && !words(&text).any(|word| has(BODY_PART, word));
        return (!figurative).then_some(Shape::Sore);
    }
    if matches(&FLAT, &mine) && includes(&heads_of_faults(&mine), FLAT_SUBJECT) {
        return Some(Shape::Strain);
    }
    if !body && setback::recognizes(&text) {
        return Some(Shape::Vague);
    }
    if let Some(trouble) = found(&THING, &mine)
        && matches(&OWNS, &mine)
    {
        let owner = owned_noun(&mine, trouble.start());
        if has(NOT_A_THING, owner) {
            return Some(Shape::Strain);
        }
        let same_state = state.is_some_and(|one| one.start() == trouble.start());
        if has(BODY_PART, owner) && same_state {
            return Some(Shape::Sore);
        }
        if growing && same_state
            || has(BODY_PART, owner) && matches(&A_DECLINE, &mine[trouble.start()..])
        {
            return None;
        }
        return Some(Shape::Thing);
    }
    (!body && (matches(&CANT_GET, &mine) && matches(&OWNS, &mine) || keeps_doing_it(&mine)))
        .then_some(Shape::Thing)
}

fn without_idioms(text: &str) -> Cow<'_, str> {
    let text = WANTING_OR_PRAISE.replace_all(text, " ");
    if !split(&text).any(|word| word == "dead") {
        return text;
    }
    let mut terms = split(&text).peekable();
    let mut kept = Vec::new();
    while let Some(word) = terms.next() {
        if word != "dead"
            || !terms.peek().is_some_and(|next| {
                has(
                    DEAD_AS_VERY_AFTER,
                    next.trim_matches(|ch| ODD_MARK.contains(ch)),
                )
            })
        {
            kept.push(word);
        }
    }
    Cow::Owned(kept.join(" "))
}

/// Recognize a fault of the speaker's own thing, body, or circumstances.
pub(crate) fn reads_as_trouble(clean: &str, words: &[&str], repaired: &str) -> Option<Shape> {
    let clean = flat(clean);
    let text = without_idioms(&clean);
    if text.is_empty() || text.contains('?') || heavy::someone_died(&text) {
        return None;
    }
    shape(&text, words).or_else(|| {
        if !repaired.is_empty() && repaired != text {
            shape(&without_idioms(repaired), &[])
        } else {
            None
        }
    })
}

fn only_decline(text: &str) -> bool {
    let Some(decline) = found(&A_DECLINE, text) else {
        return false;
    };
    let rest = format!("{} {}", &text[..decline.start()], &text[decline.end()..]);
    ![&*THING, &*CANT_GET, &*BODY, &*FLAT]
        .iter()
        .any(|pattern| matches(pattern, &rest))
}

fn said_already(lines: Vec<String>, memory: &Memory) -> Vec<String> {
    let said: Vec<_> = memory
        .turns
        .iter()
        .map(|turn| flat(&turn.bot))
        .filter(|line| !line.is_empty())
        .collect();
    let Some(last) = said.last() else {
        return lines;
    };
    let mut fresh: Vec<_> = lines
        .iter()
        .filter(|line| !said.contains(&flat(line)))
        .cloned()
        .collect();
    if fresh.is_empty() {
        fresh = lines
            .iter()
            .filter(|line| flat(line) != *last)
            .cloned()
            .collect();
    }
    if fresh.is_empty() { lines } else { fresh }
}

/// Questions appropriate to a trouble, excluding ones already answered or asked.
pub(crate) fn pool(shape: Shape, clean: &str, memory: &Memory) -> Vec<String> {
    if shape == Shape::Vague {
        return said_already(setback::pool(clean), memory);
    }
    let text = format!(" {} ", clean.to_lowercase());
    let duration = matches(&DURATION, &text);
    let decline = shape == Shape::Thing && only_decline(&text);
    let lines = match (shape, duration, decline) {
        (Shape::Sore, false, _) => SORE,
        (Shape::Sore, true, _) => SORE_TIME,
        (Shape::Strain, false, _) => STRAIN,
        (Shape::Strain, true, _) => STRAIN_TIME,
        (Shape::Thing, false, true) => DECLINE,
        (Shape::Thing, true, true) => DECLINE_TIME,
        (Shape::Thing, false, false) => BROKEN,
        (Shape::Thing, true, false) => BROKEN_TIME,
        (Shape::Vague, _, _) => unreachable!(),
    };
    let still_failing = !decline || matches(&STILL_FAILING, &text);
    let tried = matches(&ALREADY_TRIED, &text);
    said_already(
        lines
            .iter()
            .filter(|line| {
                (still_failing || !line.contains("on the way out"))
                    && (!tried || !line.contains("tried so far"))
            })
            .map(|line| (*line).to_owned())
            .collect(),
        memory,
    )
}

fn shape_on_table(memory: &Memory) -> Option<Shape> {
    let last = flat(&memory.turns.last()?.bot);
    [
        (Shape::Sore, SORE, SORE_TIME),
        (Shape::Thing, BROKEN, BROKEN_TIME),
        (Shape::Strain, STRAIN, STRAIN_TIME),
        (Shape::Sore, SORE_NEXT, &[]),
        (Shape::Thing, BROKEN_NEXT, &[]),
        (Shape::Strain, STRAIN_NEXT, &[]),
    ]
    .into_iter()
    .find_map(|(shape, lines, second)| {
        lines
            .iter()
            .chain(second)
            .any(|line| last.contains(&flat(line)))
            .then_some(shape)
    })
}

fn pointed_at(text: &str, memory: &Memory) -> bool {
    if matches(&IT, text) {
        return true;
    }
    let old = memory
        .turns
        .last()
        .map_or_else(String::new, |turn| flat(&turn.user));
    old.is_empty()
        || MINE_AGAIN
            .captures_iter(text)
            .flatten()
            .filter_map(|parts| parts.get(1))
            .any(|noun| {
                regex::Regex::new(&format!(r"\b{}\b", regex::escape(noun.as_str())))
                    .expect("escaped trouble noun")
                    .is_match(&old)
            })
}

fn answered_shape(memory: &Memory) -> Option<Shape> {
    let last = memory.turns.last()?;
    let bot = flat(&last.bot);
    if bot.is_empty() || bot.ends_with('?') {
        return None;
    }
    shape(&last.user, &[])
}

fn length_answered(text: &str, memory: &Memory) -> bool {
    matches(&A_LENGTH_OF_TIME, text)
        && chat::respond(&Reading::new(text), memory).is_some_and(|reply| {
            let reply = reply.to_lowercase();
            STANDING_HEADS.iter().any(|head| reply.contains(head))
        })
}

fn strain_answer(text: &str, memory: &Memory) -> bool {
    let terms: Vec<_> = split(text).collect();
    (2..=9).contains(&terms.len())
        && !text.contains('?')
        && !matches(&NOT_A_STRAIN_DETAIL, text)
        && !matches(&A_SENTENCE, text)
        && !terms
            .iter()
            .all(|word| has(A_HEDGE, word.trim_matches([',', '.'])))
        && lookup::lookup(text).is_none()
        && !length_answered(text, memory)
        && reads_as_trouble(text, &[], "").is_none()
}

/// Whether a short line answers the strain question immediately above it.
pub(crate) fn answers_a_strain_question(clean: &str, memory: &Memory) -> bool {
    shape_on_table(memory) == Some(Shape::Strain) && strain_answer(&flat(clean), memory)
}

/// Continue the trouble from an earlier question or a whole reply about the same subject.
pub(crate) fn carried_on(clean: &str, supplied: &[&str], memory: &Memory) -> Option<Vec<String>> {
    let prior = shape_on_table(memory);
    let shape = prior.or_else(|| answered_shape(memory))?;
    let text = flat(clean);
    if text.is_empty() || text.contains('?') {
        return None;
    }
    let generated = supplied
        .is_empty()
        .then(|| words(&text).collect::<Vec<_>>());
    let words = generated.as_deref().unwrap_or(supplied);
    let answers_strain = shape == Shape::Strain && prior.is_some() && strain_answer(&text, memory);
    if someone_elses(words) && !fault_of_a_thing(&text) && !answers_strain {
        return None;
    }
    let padded = format!(" {text} ");
    if !answers_strain
        && ![&*DETAIL, &*TRIGGER, &*DURATION]
            .iter()
            .any(|pattern| matches(pattern, &padded))
        || prior.is_none() && !pointed_at(&padded, memory)
    {
        return None;
    }
    let next = match shape {
        Shape::Sore => SORE_NEXT,
        Shape::Strain => STRAIN_NEXT,
        _ => BROKEN_NEXT,
    };
    let mut lines = said_already(next.iter().map(|line| (*line).to_owned()).collect(), memory);
    for line in pool(shape, clean, memory) {
        if !STRAIN.contains(&line.as_str()) && !lines.contains(&line) {
            lines.push(line);
        }
    }
    lines.retain(|line| !matches(&WHAT_IT_DOES, line));
    (!lines.is_empty()).then_some(lines)
}

fn choose(lines: Vec<String>, clean: &str, memory: &Memory) -> Option<String> {
    let choices: Vec<_> = lines.iter().map(String::as_str).collect();
    (!choices.is_empty()).then(|| super::choose_recent(&choices, memory, clean).to_owned())
}

pub(super) fn continue_here(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = &reading.line.clean;
    let words: Vec<_> = reading.line.words.iter().map(String::as_str).collect();
    choose(carried_on(clean, &words, memory)?, clean, memory)
}

fn tone(clean: &str, words: &[&str], repaired: &str) -> mood::Mood {
    let typed = mood::read(words, clean);
    if typed == mood::Mood::Neutral && repaired != clean {
        return mood::read(&self::words(repaired).collect::<Vec<_>>(), repaired);
    }
    typed
}

fn reading_tone(reading: &Reading<'_>) -> mood::Mood {
    let words: Vec<_> = reading.line.words.iter().map(String::as_str).collect();
    tone(&reading.line.clean, &words, &reading.line.plain)
}

pub(super) fn negative_tone(reading: &Reading<'_>) -> bool {
    reading_tone(reading) == mood::Mood::Negative
}

pub(super) fn neutral_tone(reading: &Reading<'_>) -> bool {
    reading_tone(reading) == mood::Mood::Neutral
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = &reading.line.clean;
    let words: Vec<_> = reading.line.words.iter().map(String::as_str).collect();
    let repaired = &reading.line.plain;
    let shape = reads_as_trouble(clean, &words, repaired)?;
    match tone(clean, &words, repaired) {
        mood::Mood::Positive => return None,
        mood::Mood::Negative
            if shape != Shape::Sore || matches(&CAUGHT, clean) || matches(&FRIGHTENED, clean) =>
        {
            return None;
        }
        _ => {}
    }
    choose(pool(shape, clean, memory), clean, memory)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn distinguish_machine_sore_strain_and_setback() {
        for (text, expected) in [
            ("my phone is on its way out", Some(Shape::Thing)),
            ("me knee is killing me", Some(Shape::Sore)),
            ("my career is stuck", Some(Shape::Strain)),
            ("my cake was a disaster", Some(Shape::Vague)),
            ("my new bed is dead comfortable", None),
            ("my baby keeps coughing", None),
            ("why is my cooker dead?", None),
        ] {
            let reading = Reading::new(text);
            let words: Vec<_> = reading.line.words.iter().map(String::as_str).collect();
            assert_eq!(
                reads_as_trouble(&reading.line.clean, &words, &reading.line.plain),
                expected,
                "{text}"
            );
        }
        let reading = Reading::new("my cake was a disaster");
        assert_eq!(
            reply(&reading, &Memory::default()).as_deref(),
            Some("That's rotten — what happened with your cake?")
        );
    }

    #[test]
    fn trouble_detail_skips_duration_and_questions_already_asked() {
        let memory = Memory {
            turns: vec![
                Exchange {
                    user: "my knee has been sore for two weeks".into(),
                    bot: SORE[0].into(),
                    module: "greeting",
                },
                Exchange {
                    user: "it gets worse when i run".into(),
                    bot: SORE_NEXT[0].into(),
                    module: "greeting",
                },
            ]
            .into(),
            ..Memory::default()
        };
        let line = "it is worse in the morning for two weeks now";
        let follow = carried_on(line, &[], &memory).unwrap();
        assert_eq!(follow.first().map(String::as_str), Some(SORE_NEXT[1]));
        assert!(
            !follow
                .iter()
                .any(|line| line == SORE[0] || line == SORE_NEXT[0])
        );
        assert!(!follow.iter().any(|line| line.contains("How long")));
        assert!(carried_on("can i treat that?", &[], &memory).is_none());
    }

    #[test]
    fn mood_follows_the_meaning_of_a_cue() {
        use mood::Mood::{Negative, Neutral, Positive};
        for (text, expected) in [
            ("i hate cats", Neutral),
            ("i hate my job", Negative),
            ("this phone is rubbish", Neutral),
            ("it was a rubbish week", Negative),
            ("i almost lost my keys", Neutral),
            ("i want to lose weight", Neutral),
            ("i dont want to lose weight", Negative),
            ("i broke a record", Positive),
            ("my boiler finally died", Neutral),
        ] {
            assert_eq!(
                mood::read(&words(text).collect::<Vec<_>>(), text),
                expected,
                "{text}"
            );
        }
    }

    #[test]
    fn named_trouble_and_machine_proposals_are_registered() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let named = bot.turn("the problem is the car");
        assert_eq!(named.module, Some("the_trouble_named"));
        assert_eq!(
            named.reply,
            "A vehicle goes wrong in two ways that matter - something is not right with it, or what putting it right costs - and the first step is different for each. Which is it?"
        );
        let given_advice = bot.turn("the problem is the council");
        assert!(
            given_advice
                .reply
                .starts_with("Citizens Advice covers this free")
        );
        assert!(
            given_advice
                .candidates
                .iter()
                .all(|c| c.module != "the_trouble_named")
        );

        let mut bot = Bot::new(Some("0")).unwrap();
        let machine = bot.turn("my airpods are not showing up");
        let candidate = machine
            .candidates
            .iter()
            .find(|c| c.module == "the_trouble_it_has")
            .unwrap();
        assert_eq!(candidate.priority, -999.0);
        assert_eq!(
            candidate.reply,
            "My airpods - every time, or only sometimes?"
        );
        let follow = bot.turn("only when it spins");
        assert!(
            follow
                .candidates
                .iter()
                .all(|c| c.module != "the_trouble_it_has")
        );
        let hazard = bot.turn("my electric blanket is sparking");
        assert!(
            hazard
                .candidates
                .iter()
                .all(|c| c.module != "the_trouble_it_has")
        );
    }
}
