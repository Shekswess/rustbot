//! Read a bare follow-up as the preceding question with one typed value replaced.
use super::{
    Reading, context, entities,
    kinds::{Intent, Kind},
    line,
};
#[path = "asked_again_data.rs"]
mod data;

fn has(set: &[&str], value: &str) -> bool {
    set.binary_search(&value).is_ok()
}

fn thing(mut words: &[String]) -> Option<&[String]> {
    while let Some(size) = data::FOLLOW_OPENER.iter().find_map(|opener| {
        let size = opener.split_ascii_whitespace().count();
        (words.len() >= size
            && words[..size]
                .iter()
                .map(String::as_str)
                .eq(opener.split_ascii_whitespace()))
        .then_some(size)
    }) {
        words = &words[size..];
    }
    while words
        .first()
        .is_some_and(|w| has(data::CORRECTION_OPENER, w))
    {
        words = &words[1..];
    }
    (!words.is_empty()
        && words.len() <= 5
        && !words.iter().any(|w| has(data::NOT_A_THING, w))
        && words
            .iter()
            .any(|w| w.chars().count() > 2 || line::common_name(w)))
    .then_some(words)
}

fn digit(word: &str) -> bool {
    word.chars().any(|c| c.is_numeric())
}

fn same_kind(a: &str, b: &str) -> Option<bool> {
    if [
        entities::is_relation,
        line::common_name,
        line::body_word,
        line::count_unit,
    ]
    .iter()
    .any(|test| test(a) && test(b))
    {
        Some(false)
    } else {
        (digit(a) && digit(b)).then_some(true)
    }
}

pub(super) fn replacement<'a>(
    read: &Reading<'_>,
    mut earlier: impl DoubleEndedIterator<Item = &'a str>,
) -> Option<String> {
    if matches!(
        read.features.intent,
        Intent::Empty
            | Intent::Greeting
            | Intent::Farewell
            | Intent::Thanks
            | Intent::Ack
            | Intent::Smalltalk
            | Intent::Laugh
            | Intent::AboutBot
    ) || !read.reference.followup && read.own_topic.is_some()
        || context::shifts(&read.line)
    {
        return None;
    }
    let phrase = thing(&read.line.norm)?;
    let previous = earlier.rfind(|s| !s.trim().is_empty())?;
    let before = line::normalise(previous).norm;
    if !before
        .iter()
        .any(|w| !has(data::STOPWORDS, w) && (w.chars().count() > 1 || digit(w)))
    {
        return None;
    }
    let (start, numeric) = phrase.iter().find_map(|word| {
        before
            .iter()
            .enumerate()
            .find_map(|(i, old)| same_kind(word, old).map(|numeric| (i, numeric)))
    })?;
    if numeric && read.kind != Kind::Question {
        return None;
    }
    let mut end = start + 1;
    if digit(&before[start]) {
        while end < before.len()
            && (line::count_unit(&before[end])
                || matches!(before[end].as_str(), "a" | "an" | "each" | "per" | "every"))
        {
            end += 1;
        }
    }
    let skip = usize::from(
        start > 0
            && has(data::DETERMINER_TAIL, &before[start - 1])
            && has(data::DETERMINER_TAIL, &phrase[0]),
    );
    let insert = &phrase[skip..];
    if insert.is_empty() || insert == &before[start..end] {
        return None;
    }
    Some(
        before[..start]
            .iter()
            .chain(insert)
            .chain(&before[end..])
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" "),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    mod fixture {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/asked_again.rs"
        ));
    }
    #[test]
    fn bare_followups_match_earlier_typed_values() {
        let mut mismatches = Vec::new();
        for &(before, next, expected) in fixture::CASES {
            let mut history = context::History::default();
            history.retain(&mut Reading::new(before));
            let actual = Reading::understand(next, [before].into_iter(), None, &history);
            if actual.context_fields.asked_again.as_deref() != expected {
                mismatches.push((before, next, expected, actual.context_fields.asked_again));
            }
        }
        assert!(
            mismatches.is_empty(),
            "{} mismatches: {:?}",
            mismatches.len(),
            &mismatches[..mismatches.len().min(15)]
        );
    }
}
