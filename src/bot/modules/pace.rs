//! The pace or duration that the previous answer already gave.

use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

#[path = "pace_data.rs"]
mod data;
#[cfg(test)]
#[path = "pace_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    PACE_ASK,
    KEEP_UP_ASK,
    A_LENGTH,
    A_RATE_LEAD,
    STEP,
    PACE,
    TEST,
    CONTINUES,
    NOT_A_TAIL,
    DANGLES,
    A_PORTION,
);
static SPLIT: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(r"(?<=[.;!?])\s+|;\s*|,\s*|:\s+|\s+-\s+").unwrap());
static JOINED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:and|but|then|so)\s+").unwrap());
static LEAD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:to start with|first|first of all|for a start)\b").unwrap()
});
static ANSWER_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static CONTENT_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z]{4,}").unwrap());
static AFTER_LENGTH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:of|in|on|for|at)\s+").unwrap());
static ACTION_END: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:is|are|was|were|turns?|helps?|makes?|works?|should|will|but)\b").unwrap()
});

fn collapsed(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn asks(clean: &str, answer: &str, pattern: &Regex, extra: &[&str]) -> bool {
    let said = clean.trim_matches([' ', '?', '!', '.', ',', ';', ':', '\'', '"']);
    let Some(ask) = pattern.find(said) else {
        return false;
    };
    let rest = format!("{} {}", &said[..ask.start()], &said[ask.end()..]).replace("'s", " ");
    let spoken = answer.to_lowercase();
    let in_answer = ANSWER_WORD
        .find_iter(&spoken)
        .map(|word| word.as_str())
        .collect::<HashSet<_>>();
    rest.split_whitespace().all(|word| {
        let word = word
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect::<String>();
        word.is_empty()
            || data::ALLOWED.contains(&word.as_str())
            || extra.contains(&word.as_str())
            || in_answer.contains(word.as_str())
    })
}

fn clauses(answer: &str) -> Vec<String> {
    SPLIT
        .split(answer)
        .filter_map(Result::ok)
        .filter_map(|piece| {
            let text = collapsed(piece.trim_matches([' ', ',', '.', ';']));
            let piece = JOINED.replace(&text, "");
            (!piece.is_empty()).then(|| piece.into_owned())
        })
        .collect()
}

fn trim(text: &str, width: usize) -> String {
    let Some((end, _)) = text.char_indices().nth(width) else {
        return text.to_owned();
    };
    let text = &text[..end];
    let cut = text
        .rfind(',')
        .filter(|&comma| text[..comma].chars().count() > width / 2)
        .or_else(|| {
            text.rfind(' ')
                .filter(|&space| text[..space].chars().count() > width / 2)
        });
    collapsed(cut.map_or(text, |end| &text[..end]))
        .trim_end_matches([',', ';', ':'])
        .to_owned()
}

fn next_step(clauses: &[String]) -> Option<String> {
    clauses
        .iter()
        .map(|clause| trim(clause, 96))
        .find(|clause| {
            !clause.is_empty()
                && !LEAD.is_match(clause)
                && !CONTINUES.is_match(clause)
                && !PACE.is_match(clause)
                && !STEP.is_match(clause)
        })
}

struct Parts {
    pace: Vec<String>,
    test: Option<String>,
    after: Option<String>,
}

fn parts(answer: &str) -> Parts {
    let clauses = clauses(answer);
    let mut stepped = Vec::new();
    let mut timed = Vec::new();
    let mut test = None;
    let mut after = None;
    for (index, clause) in clauses.iter().enumerate() {
        if STEP.is_match(clause) && stepped.len() < 2 {
            stepped.push(trim(clause, 96));
        } else if PACE.is_match(clause) && timed.len() < 2 {
            timed.push(trim(clause, 96));
        } else if test.is_none()
            && let Some(check) = TEST.find(clause)
        {
            test = Some(trim(check.as_str().trim(), 96));
        }
        if (!stepped.is_empty() || !timed.is_empty()) && after.is_none() && !LEAD.is_match(clause) {
            after = next_step(&clauses[index + 1..]);
        }
    }
    stepped.extend(timed);
    let after = after.filter(|next| !stepped.contains(next));
    Parts {
        pace: stepped,
        test,
        after,
    }
}

fn own_pace(answer: &str) -> bool {
    let opening = collapsed(answer).to_lowercase();
    data::LEADS
        .iter()
        .flat_map(|(_, leads)| *leads)
        .any(|lead| {
            let prefix = lead.to_lowercase().chars().take(24).collect::<String>();
            opening.starts_with(&prefix)
        })
}

fn own_duration(answer: &str) -> bool {
    let opening = collapsed(answer).to_lowercase();
    data::KEEP_LEADS.iter().any(|lead| {
        let prefix = lead
            .split_once("{length}")
            .map_or(*lead, |(prefix, _)| prefix);
        opening.starts_with(&prefix.trim().to_lowercase())
    })
}

fn doing(after: &str) -> Option<String> {
    let action = AFTER_LENGTH.replace(after.trim_matches([' ', ',', '.', ';']), "");
    let cut = ACTION_END
        .find(&action)
        .map_or(action.as_ref(), |end| &action[..end.start()]);
    let cut = DANGLES.replace(cut, "");
    let cut = if cut.matches('(').count() > cut.matches(')').count() {
        cut.rsplit_once('(')
            .map_or(cut.as_ref(), |(before, _)| before)
    } else {
        &cut
    };
    let cut = trim(collapsed(cut).trim_matches([' ', ',', '.', ';', '-']), 80);
    (cut.split_whitespace().nth(1).is_some() && !NOT_A_TAIL.is_match(&cut)).then_some(cut)
}

fn content(text: &str) -> impl Iterator<Item = &str> {
    CONTENT_WORD
        .find_iter(text)
        .map(|word| word.as_str())
        .filter(|word| !data::STOPPED.contains(word))
}

fn stem(word: &str) -> &str {
    ["ing", "ed", "es", "s"]
        .iter()
        .find_map(|ending| word.strip_suffix(ending).filter(|root| root.len() > 3))
        .unwrap_or(word)
}

fn length_in(answer: &str, asked: &HashSet<&str>) -> Option<(String, Option<String>)> {
    let mut first = None;
    for clause in clauses(answer) {
        let Some(length) = A_LENGTH.find(&clause) else {
            continue;
        };
        if A_PORTION.is_match(&clause) || A_RATE_LEAD.is_match(&clause[..length.start()]) {
            continue;
        }
        let found = || {
            (
                length.as_str().trim().to_owned(),
                doing(&clause[length.end()..]),
            )
        };
        if !asked.is_empty()
            && content(&clause.to_lowercase()).any(|word| asked.contains(stem(word)))
        {
            return Some(found());
        }
        if first.is_none() {
            first = Some(found());
        }
    }
    first
}

fn advice_length(memory: &Memory, line: &str) -> Option<(String, Option<String>)> {
    let asked = content(line)
        .map(stem)
        .filter(|word| !data::ASK_WORDS.contains(word))
        .collect::<HashSet<_>>();
    let mut answers = memory
        .turns
        .iter()
        .rev()
        .map(|turn| turn.bot.trim())
        .filter(|answer| !answer.is_empty() && !own_pace(answer) && !own_duration(answer))
        .take(3);
    let first = answers.next()?;
    if let Some(length) = length_in(first, &asked) {
        return Some(length);
    }
    let lower = first.to_lowercase();
    let names = content(&lower).collect::<HashSet<_>>();
    answers.find_map(|previous| {
        let older = previous.to_lowercase();
        let older = content(&older).collect::<HashSet<_>>();
        (older.intersection(&names).nth(1).is_some())
            .then(|| length_in(previous, &asked))
            .flatten()
    })
}

fn as_clause(text: &str) -> String {
    let said = text.trim().trim_end_matches('.').trim();
    if said.starts_with("I ") || said.starts_with("I'") {
        return said.to_owned();
    }
    let mut chars = said.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_lowercase)
        .chain(chars)
        .collect()
}

fn keep_reply(clean: &str, raw: &str, last: &str, memory: &Memory) -> Option<String> {
    if own_duration(last) || !asks(clean, last, &KEEP_UP_ASK, data::KEEP_ALLOWED) {
        return None;
    }
    let (length, action) = advice_length(memory, clean)?;
    let lead =
        compose::pick(data::KEEP_LEADS, memory, raw).replace("{length}", &as_clause(&length));
    let mark = compose::pick(data::KEEP_MARK, memory, raw);
    Some(match action {
        Some(action) => format!("{lead} - {mark}, {}.", as_clause(&action)),
        None => format!("{lead} - {mark}."),
    })
}

fn pace_reply(clean: &str, raw: &str, answer: &str, memory: &Memory) -> Option<String> {
    if !asks(clean, answer, &PACE_ASK, &[]) {
        return None;
    }
    let step = parts(answer);
    let pace = step.pace.first()?;
    let prompt = ["slowly", "fast", "soon"]
        .into_iter()
        .find(|word| clean.contains(word))
        .unwrap_or("quickly");
    let leads = data::LEADS
        .iter()
        .find_map(|&(word, options)| (word == prompt).then_some(options))?;
    let lead = compose::pick(leads, memory, raw);
    let next = if let Some(test) = step.test {
        let test = as_clause(&test);
        let opening = test
            .split_whitespace()
            .next()
            .map_or("", |word| word.trim_matches([',', '.', ';', ':']));
        let options = data::TESTED
            .iter()
            .copied()
            .filter(|frame| {
                let prefix = frame
                    .split_once("{test}")
                    .map_or(*frame, |(prefix, _)| prefix);
                !prefix.split_whitespace().any(|word| {
                    word.trim_matches([',', '.', ';', ':'])
                        .eq_ignore_ascii_case(opening)
                })
            })
            .collect::<Vec<_>>();
        compose::pick(
            if options.is_empty() {
                data::TESTED
            } else {
                &options
            },
            memory,
            raw,
        )
        .replace("{test}", &test)
    } else if let Some(rest) = step.pace.get(1) {
        compose::pick(data::REST, memory, raw).replace("{rest}", &as_clause(rest))
    } else {
        compose::pick(data::NEXT, memory, raw).replace("{next}", &as_clause(step.after.as_deref()?))
    };
    Some(format!("{lead}: {}. {next}", as_clause(pace)))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = collapsed(reading.raw).to_lowercase();
    let answer = memory
        .turns
        .iter()
        .rev()
        .map(|turn| turn.bot.trim())
        .find(|answer| !answer.is_empty())?;
    if own_pace(answer) {
        return None;
    }
    if KEEP_UP_ASK.is_match(&clean) {
        keep_reply(&clean, reading.raw, answer, memory)
    } else {
        pace_reply(&clean, reading.raw, answer, memory)
    }
}
