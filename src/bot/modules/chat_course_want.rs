//! The source chat response to a straightforward wish to take a course.

use crate::bot::{Memory, Reading, compose, memory::ack, understanding::kinds::Kind};

fn course(line: &str) -> Option<&str> {
    let (speaker, rest) = line
        .trim_matches([' ', '.', ',', '!', ';', ':'])
        .split_once(' ')?;
    if !matches!(speaker, "i" | "we") {
        return None;
    }
    let want = rest
        .split_once(' ')
        .filter(|(word, _)| matches!(*word, "just" | "still" | "really" | "actually"))
        .map_or(rest, |(_, after)| after);
    let thing = want
        .strip_prefix("want to ")
        .or_else(|| want.strip_prefix("would like to "))?;
    let (action, said) = thing.split_once(' ')?;
    if !matches!(action, "take" | "start" | "do") {
        return None;
    }
    let subject = said
        .strip_prefix("a ")
        .or_else(|| said.strip_prefix("an "))?;
    let words = subject.split_whitespace().collect::<Vec<_>>();
    let end = words
        .iter()
        .position(|word| matches!(*word, "course" | "class"))?;
    if end > 3
        || words[..end].iter().any(|word| {
            !word
                .bytes()
                .all(|letter| letter.is_ascii_lowercase() || letter == b'-')
                || matches!(*word, "if" | "but" | "or" | "without" | "no" | "not")
        })
    {
        return None;
    }
    match &words[end + 1..] {
        []
        | ["online" | "part-time" | "full-time"]
        | ["in", "the", "morning" | "afternoon" | "evening"] => Some(thing),
        _ => None,
    }
}

pub(in crate::bot::modules) fn scheduled_course(reading: &Reading<'_>) -> bool {
    reading.kind == Kind::Statement
        && !reading.line.signals.question_mark
        && reading.features.asks.is_empty()
        && course(&reading.clean).is_some_and(|thing| {
            ["in the morning", "in the afternoon", "in the evening"]
                .into_iter()
                .any(|time| thing.ends_with(time))
        })
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory, words: &[&str]) -> Option<String> {
    if reading.kind != Kind::Statement
        || reading.line.signals.question_mark
        || !reading.features.asks.is_empty()
        || reading.body.is_some()
        || reading.own_topic.is_some_and(|topic| topic.0 == "health")
        || !(3..=16).contains(&words.len())
    {
        return None;
    }
    let thing = course(&reading.clean)?;
    if thing.split_whitespace().count() > 8
        || compose::loss_reader::reads_as_loss(reading.raw)
        || memory
            .turns
            .iter()
            .rev()
            .take(6)
            .any(|turn| compose::loss_reader::reads_as_loss(&turn.user))
        || super::super::answers::offer(reading, memory).is_some()
    {
        return None;
    }
    let said = ack::you(thing);
    let mut chars = said.chars();
    let label: String = chars.next()?.to_uppercase().chain(chars).collect();
    let named = format!("{label} — fair enough. What's stopped it happening so far?");
    let pool = [
        named.as_str(),
        "Then start with the smallest version of it, not the whole thing. What would the first step be?",
        "Doable. Tell me the bit that feels hardest and we'll start there.",
    ];
    Some(compose::pick(&pool, memory, "").to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_a_stated_course_without_a_medical_or_conditional_tail() {
        assert_eq!(
            course("i want to take a maths course in the evening."),
            Some("take a maths course in the evening")
        );
        assert_eq!(
            course("we would like to do an english course online"),
            Some("do an english course online")
        );
        for line in [
            "i want to take a course of antibiotics",
            "i want to take a maths course or learn from a book",
            "i want to take a maths course if it fits around work",
            "where should i take a maths course",
        ] {
            assert!(course(line).is_none(), "{line}");
        }
    }
}
