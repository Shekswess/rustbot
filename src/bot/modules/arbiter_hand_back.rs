//! Keep a matched clarification of the last answer over a speculative lead-in reply.

use crate::bot::{Candidate, Memory, Reading};
use fancy_regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

#[path = "arbiter_hand_back_data.rs"]
mod data;
#[cfg(test)]
#[path = "arbiter_hand_back_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(HOW_TO, KEEP_CLAUSE, LESS, MEAN, REPEAT);
static WORD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(data::WORD).expect("previous answer word"));
static MEASURES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::MEASURE_ASKS
        .iter()
        .map(|pattern| Regex::new(pattern).expect("measure request pattern"))
        .collect()
});

fn hit(pattern: &Regex, line: &str) -> bool {
    pattern.is_match(line).unwrap_or(false)
}

fn words(line: &str) -> Vec<&str> {
    WORD.find_iter(line).map(|word| word.as_str()).collect()
}

fn brevity(word: &str) -> bool {
    data::BREVITY_WORDS.contains(&word)
}

fn shorter(line: &str) -> bool {
    let clean = line.split_whitespace().collect::<Vec<_>>().join(" ");
    if clean.is_empty() || hit(&HOW_TO, &clean) {
        return false;
    }
    let asked = KEEP_CLAUSE.replace_all(&clean, "").replace('’', "'");
    let said = words(&asked)
        .into_iter()
        .map(|word| word.strip_suffix("'s").unwrap_or(word).replace('\'', ""))
        .collect::<Vec<_>>();
    if said
        .iter()
        .any(|word| !data::LESS_ALLOWED.contains(&word.as_str()))
        || !(hit(&LESS, &clean) || said.iter().any(|word| brevity(word)))
    {
        return false;
    }
    said.iter()
        .rev()
        .find(|word| !data::LESS_IGNORED.contains(&word.as_str()))
        .is_some_and(|last| data::LESS_WORDS.contains(&last.as_str()) || brevity(last))
}

fn quotes_an_answer(previous: &[&str], candidate: &[&str]) -> bool {
    if previous.len() < 12 || candidate.len() < 12 {
        return false;
    }
    (0..=(candidate.len() - 12).min(6)).any(|lead| {
        previous
            .windows(candidate.len() - lead)
            .any(|run| run == &candidate[lead..])
    })
}

pub(crate) fn candidate<'a>(
    reading: &Reading<'_>,
    memory: &Memory,
    proposals: &'a [Candidate],
) -> Option<&'a Candidate> {
    if proposals.first()?.module != "after_a_lead_in" {
        return None;
    }
    let last = memory.turns.last()?;
    let previous = last.bot.to_lowercase();
    let previous = words(&previous);
    if previous.is_empty() || !data::ANSWER_BANDS.contains(&last.module) {
        return None;
    }
    let line = reading.raw.trim().to_lowercase();
    // This is the source's MEAN/REPEAT branch, where the whole ask concerns
    // the previous answer. Other store repeat requests have separate readers.
    if !(hit(&MEAN, &line) || hit(&REPEAT, &line))
        || shorter(&line)
        || MEASURES.iter().any(|pattern| hit(pattern, &line))
    {
        return None;
    }
    let mut band = proposals.iter().filter(|proposal| {
        proposal.priority >= 0.0 && data::MEMORY_MODULES.contains(&proposal.module)
    });
    if let Some(whole) = band
        .clone()
        .find(|proposal| quotes_an_answer(&previous, &words(&proposal.reply.to_lowercase())))
    {
        return Some(whole);
    }
    let seen = previous.into_iter().collect::<HashSet<_>>();
    band.find(|proposal| {
        let said = proposal.reply.to_lowercase();
        let said = words(&said);
        !said.is_empty()
            && said.iter().filter(|word| seen.contains(**word)).count() * 2 >= said.len()
    })
}
