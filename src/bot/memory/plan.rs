//! Keep the cursor for ordered instructions already given in the conversation.

use super::carried_on::{ordered_steps, sentences};
use crate::bot::{Exchange, Memory, Reading, compose};
use std::collections::BTreeSet;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "plan_data.rs"]
mod data;
use data::*;

pattern!(WORDS, r"[a-z']{4,}");
pattern!(WORK_DONE, r"(?i)\b[a-z]{3,}ed\b");

#[derive(Clone)]
pub(crate) struct Cursor {
    answer: String,
    steps: Vec<String>,
    at: usize,
    turn: usize,
    topic: Option<String>,
    said: String,
    done: bool,
}

fn last_answer(memory: &Memory) -> Option<&Exchange> {
    memory.turns.iter().rev().find(|turn| !turn.bot.is_empty())
}

fn flat(text: &str) -> String {
    text.split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn with_case(text: &str, upper: bool) -> String {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let start = if upper {
        first.to_uppercase().collect::<String>()
    } else {
        first.to_lowercase().collect()
    };
    start + chars.as_str()
}

fn own_line(answer: &str, cursor: &Cursor) -> bool {
    let said = flat(answer).to_lowercase();
    !said.is_empty()
        && cursor.steps.iter().any(|step| {
            let step = flat(step);
            !step.is_empty() && said.contains(&step.to_lowercase())
        })
}

/// Recover the answer underlying the last step when another reader needs all of it.
pub(crate) fn the_answer_in_hand(memory: &Memory) -> Option<&str> {
    let cursor = memory.steps.as_ref()?;
    let answer = cursor.answer.trim();
    let last = &last_answer(memory)?.bot;
    if answer.is_empty() || last.trim().is_empty() {
        return None;
    }
    (flat(last) == flat(&cursor.said) || own_line(last, cursor)).then_some(answer)
}

fn wording(text: &str) -> impl Iterator<Item = &str> {
    WORDS.find_iter(text).map(|word| word.as_str())
}

fn shared(text: &str, candidate: &str) -> bool {
    let left = text.to_lowercase();
    let right = candidate.to_lowercase();
    let terms: BTreeSet<_> = wording(&left).collect();
    wording(&right).any(|word| terms.contains(word))
}

fn stems(text: &str) -> BTreeSet<String> {
    wording(&text.to_lowercase())
        .map(|word| word[..word.len().min(5)].to_owned())
        .collect()
}

fn same_topic(memory: &Memory, cursor: &Cursor) -> bool {
    let user = memory
        .turns
        .iter()
        .rev()
        .find(|turn| !turn.user.is_empty())
        .map_or("", |turn| turn.user.as_str());
    let answer = last_answer(memory).map_or("", |turn| turn.bot.as_str());
    let mut here = stems(user);
    here.extend(stems(answer));
    cursor
        .steps
        .iter()
        .map(String::as_str)
        .chain(cursor.topic.as_deref())
        .any(|part| !here.is_disjoint(&stems(part)))
}

fn tried(step: &str, memory: &Memory) -> bool {
    let step = step.to_lowercase();
    let mine: BTreeSet<_> = wording(&step)
        .filter(|word| !NOT_THE_THING.contains(word))
        .collect();
    !mine.is_empty()
        && memory.turns.iter().rev().take(3).any(|turn| {
            if !HAD_A_GO.is_match(&turn.user) {
                return false;
            }
            let said = turn.user.to_lowercase();
            wording(&said).any(|word| mine.contains(word))
        })
}

fn comparable_prefix(text: &str) -> &str {
    let end = text
        .char_indices()
        .nth(40)
        .map_or(text.len(), |(index, _)| index);
    &text[..end]
}

fn steps_of(answer: &str) -> Vec<String> {
    let mut steps: Vec<_> = ordered_steps(answer)
        .filter_map(|step| {
            let step = step
                .split_once(": ")
                .filter(|(_, tail)| tail.split_whitespace().count() < 5)
                .map_or(step.as_str(), |(head, _)| head.trim());
            (!step.is_empty()).then(|| step.to_owned())
        })
        .collect();
    if let Some(first) = sentences(answer).next()
        && AMOUNTS.find_iter(first).take(2).count() == 2
        && WORK_DONE.is_match(first)
        && steps.first().is_none_or(|step| {
            !first
                .to_lowercase()
                .contains(comparable_prefix(&step.to_lowercase()))
        })
    {
        steps.insert(0, first.trim_end_matches('.').to_owned());
    }
    if steps.len() < 2 {
        steps.clear();
    }
    steps
}

fn recorded_topic(memory: &Memory) -> Option<String> {
    memory
        .ack_topics
        .stored_topic(&memory.turns)
        .next()
        .map(str::to_owned)
}

fn plan_for(memory: &Memory) -> Option<Cursor> {
    let last = last_answer(memory)?;
    let answer = &last.bot;
    if let Some(cursor) = &memory.steps {
        if cursor.answer == *answer || cursor.said == *answer {
            return Some(cursor.clone());
        }
        let own = own_line(answer, cursor);
        let related = || {
            let fresh = steps_of(answer);
            (fresh.is_empty() || fresh == cursor.steps)
                && memory.turns.len().saturating_sub(cursor.turn) < WHILE_IT_LASTS
                && (same_topic(memory, cursor)
                    || cursor.steps.iter().any(|step| shared(step, answer)))
        };
        if own || related() {
            let reached = cursor
                .steps
                .iter()
                .rposition(|step| shared(step, answer))
                .map_or(0, |position| position + 1);
            let mut cursor = cursor.clone();
            cursor.answer.clone_from(answer);
            cursor.at = cursor.at.max(reached);
            return Some(cursor);
        }
    }
    if NOT_A_JOB.contains(&last.module) {
        return None;
    }
    let steps = steps_of(answer);
    (!steps.is_empty()).then(|| Cursor {
        answer: answer.to_owned(),
        steps,
        at: 0,
        turn: memory.turns.len(),
        topic: recorded_topic(memory),
        said: String::new(),
        done: false,
    })
}

pub(super) fn continuation(text: &str) -> Option<&str> {
    let line = text.trim_matches(crate::text::is_space);
    CARRY_ON.is_match(line).then_some(line)
}

fn propose(reading: &Reading<'_>, memory: &Memory) -> Option<Cursor> {
    let line = continuation(reading.raw)?;
    let mut plan = plan_for(memory)?;
    let at = (plan.at..plan.steps.len())
        .find(|at| !tried(&plan.steps[*at], memory))
        .unwrap_or(plan.steps.len());
    let previous = memory
        .steps
        .as_ref()
        .filter(|before| before.steps == plan.steps);
    let done = previous.is_some_and(|before| before.done);
    let mut say = if let Some(step) = plan.steps.get(at) {
        if OPENS_ON_THE_INSTRUCTION.is_match(step) {
            let mut line = with_case(step, true);
            if !line.ends_with(['.', '!', '?']) {
                line.push('.');
            }
            line
        } else {
            let frame = if at > 0 {
                LEADS[(at - 1) % LEADS.len()]
            } else if AMOUNTS.find(step).is_some_and(|amount| amount.start() == 0) {
                compose::pick(AMOUNTS_LEADS, memory, line)
            } else {
                compose::pick(FIRST_LEADS, memory, line)
            };
            frame.replace("{step}", &with_case(step, true))
        }
    } else if done {
        compose::pick(FINISHED, memory, line).to_owned()
    } else {
        let last = plan.steps.last()?;
        compose::pick(DONE, memory, line).replace("{step}", &with_case(last, false))
    };
    if at == 0
        && let Some((_, count)) = COUNTS
            .iter()
            .find(|(number, _)| *number == plan.steps.len())
    {
        say.push(' ');
        say.push_str(count);
    }
    plan.at = (at + 1).min(plan.steps.len());
    plan.turn = memory.turns.len();
    plan.done = done || at >= plan.steps.len();
    plan.topic = previous
        .and_then(|before| before.topic.clone())
        .or_else(|| recorded_topic(memory));
    plan.said = say;
    Some(plan)
}

pub(crate) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    propose(reading, memory).map(|plan| plan.said)
}

pub(crate) fn record_selected(reading: &Reading<'_>, memory: &mut Memory) {
    if let Some(plan) = propose(reading, memory) {
        memory.steps = Some(plan);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TAP: &str = "Turn the water off at the valve, take the handle off, and take the old part to a hardware shop to match it.";
    const BREAD: &str = "500g strong white flour, 7g dried yeast, 10g salt and 350ml warm water, kneaded for 10 minutes until smooth and stretched. Leave it somewhere warm for an hour. Shape it and give it a second rise. Bake at 220C for half an hour.";

    fn observe(memory: &mut Memory, user: &str, bot: &str, module: &'static str) {
        memory.turns.push(Exchange {
            user: user.to_owned(),
            bot: bot.to_owned(),
            module,
        });
    }

    fn advance(memory: &mut Memory, user: &str) -> Option<String> {
        let reading = Reading::new(user);
        let said = reply(&reading, memory)?;
        assert_eq!(reply(&reading, memory).as_deref(), Some(said.as_str()));
        record_selected(&reading, memory);
        observe(memory, user, &said, "steps");
        Some(said)
    }

    #[test]
    fn hands_over_each_step_then_reports_exhaustion() {
        let mut memory = Memory::default();
        observe(&mut memory, "My tap drips", TAP, "answers");
        assert_eq!(
            reply(&Reading::new("what next for the sink"), &memory),
            None
        );
        for (line, expected) in [
            (
                "what next",
                "Start with this: Turn the water off at the valve. I make it three steps.",
            ),
            ("and then", "Next: Take the handle off."),
            (
                "then what",
                "Then: Take the old part to a hardware shop to match it.",
            ),
            (
                "after that",
                "There is nothing after that bit that I can pick out. The last of it is: take the old part to a hardware shop to match it. Where have you got to?",
            ),
            (
                "go on",
                "Nothing more I can pull out of that answer. How did it go?",
            ),
        ] {
            assert_eq!(
                advance(&mut memory, line).as_deref(),
                Some(expected),
                "{line}"
            );
            assert_eq!(the_answer_in_hand(&memory), Some(TAP));
        }
    }

    #[test]
    fn uses_the_first_ingredient_step_and_restarts_for_a_new_answer() {
        let mut memory = Memory::default();
        observe(&mut memory, "My tap drips", TAP, "answers");
        advance(&mut memory, "what next").unwrap();
        observe(&mut memory, "How do I make a loaf", BREAD, "answers");
        let steps = steps_of(BREAD);
        assert_eq!(steps.len(), 3);
        assert!(steps[0].starts_with("500g strong white flour"));
        assert_eq!(steps[1], "leave it somewhere warm for an hour");
        assert_eq!(steps[2], "bake at 220C for half an hour");
        let first = advance(&mut memory, "what next").unwrap();
        assert!(first.contains(&steps[0]));
        assert_eq!(the_answer_in_hand(&memory), Some(BREAD));
    }

    #[test]
    fn skips_a_step_that_the_user_reports_as_done_without_matching_grammar() {
        let mut memory = Memory::default();
        observe(
            &mut memory,
            "I have already turned the water off",
            "All right.",
            "greeting",
        );
        observe(&mut memory, "My tap drips", TAP, "answers");
        assert_eq!(
            advance(&mut memory, "what next").as_deref(),
            Some("Next: Take the handle off.")
        );
        assert_eq!(the_answer_in_hand(&memory), Some(TAP));
    }

    #[test]
    fn leaves_single_steps_and_emergency_replies_alone() {
        let mut memory = Memory::default();
        observe(
            &mut memory,
            "Is there anything else",
            "Put a hot water bottle on it.",
            "answers",
        );
        assert_eq!(reply(&Reading::new("what next"), &memory), None);
        observe(&mut memory, "There was an emergency", TAP, "safety");
        assert_eq!(reply(&Reading::new("then what"), &memory), None);
    }

    #[test]
    fn the_selected_hook_commits_the_cursor_between_live_turns() {
        let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
        assert_eq!(bot.turn("my bathroom tap drips").module, Some("answers"));
        for (line, expected) in [
            (
                "what next",
                "Start with this: Turn the water off at the isolating valve under the sink. I make it three steps.",
            ),
            ("and then", "Next: Take the handle off."),
            (
                "then what",
                "Then: Take the old part to a hardware shop to match it.",
            ),
        ] {
            let turn = bot.turn(line);
            assert_eq!(turn.module, Some("steps"));
            assert_eq!(turn.reply, expected);
        }
    }
}
