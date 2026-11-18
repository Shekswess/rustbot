//! Fallback pools for the kind of answer requested, including conditions that disqualify a pool.
#[path = "shape_data.rs"]
mod data;
use crate::bot::{Exchange, Memory, Reading};
use data::*;
use fancy_regex::Regex;
use std::sync::LazyLock;
fn matches(pattern: &Regex, s: &str) -> bool {
    pattern.is_match(s).unwrap_or(false)
}
fn has(set: &[&str], s: &str) -> bool {
    set.binary_search(&s).is_ok()
}
fn recent(memory: &Memory, n: usize) -> impl Iterator<Item = &Exchange> {
    memory.turns.iter().rev().take(n)
}
fn said(memory: &Memory, n: usize) -> impl Iterator<Item = &str> {
    recent(memory, n).flat_map(|t| [t.user.as_str(), t.bot.as_str()])
}
fn options_named(line: &str, memory: &Memory) -> bool {
    std::iter::once(line).chain(said(memory, 4)).any(|v| {
        TWO_THINGS
            .captures_iter(&v.to_lowercase())
            .flatten()
            .any(|m| !has(NOT_AN_OPTION, &m[1]) && !has(NOT_AN_OPTION, &m[2]))
    })
}
fn count_just_named(line: &str, memory: &Memory) -> bool {
    let Some(found) = HOW_MANY.captures(line).ok().flatten() else {
        return false;
    };
    let word = &found[1];
    if has(NOT_THE_THING, word) {
        return false;
    }
    let word = word.trim_end_matches('s');
    said(memory, 4).any(|s| {
        s.to_lowercase()
            .split(|c: char| !c.is_ascii_lowercase())
            .any(|w| w.strip_prefix(word).is_some_and(|tail| tail.len() <= 2))
    })
}
fn about_body(said: &str) -> bool {
    static WORDS: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new("[a-z']+").unwrap());
    let lowered = said.to_lowercase();
    let no_forecast = FORECAST.replace(&lowered, " ");
    let no_body = BODY_WORD_FOR_A_THING.replace_all(&no_forecast, " ");
    let words: Vec<_> = WORDS.find_iter(&no_body).map(|m| m.as_str()).collect();
    let mut ignore = vec![false; words.len()];
    for idiom in IDIOM_PHRASES {
        let size = idiom.split_ascii_whitespace().count();
        for (i, part) in words.windows(size).enumerate() {
            if part.iter().copied().eq(idiom.split_ascii_whitespace()) {
                ignore[i..i + size].fill(true);
            }
        }
    }
    words.iter().zip(ignore).any(|(w, skip)| {
        !skip
            && [SYMPTOM_WORDS, BODY_PARTS, HEALTH_WORDS]
                .iter()
                .any(|set| has(set, w))
    })
}
fn care_thread(memory: &Memory) -> bool {
    for turn in recent(memory, 12) {
        if has(CARE_MODULES, turn.module) {
            return true;
        }
        if !has(QUIET_MODULES, turn.module) {
            break;
        }
    }
    if recent(memory, 4)
        .any(|t| super::loss::recognizes(&t.user.to_lowercase()) || about_body(&t.user))
    {
        return true;
    }
    let mut intervening = Vec::new();
    for turn in recent(memory, 12) {
        if turn.module == "safety" {
            let answer = turn.bot.to_lowercase();
            if !CRISIS_WORDS.iter().any(|w| answer.contains(w)) {
                return false;
            }
            let words = |text: &str| {
                text.split(|c: char| !c.is_ascii_alphabetic() && c != '\'')
                    .filter(|w| w.len() >= 5)
                    .map(str::to_lowercase)
                    .collect::<std::collections::HashSet<_>>()
            };
            let original = words(&answer);
            return intervening
                .into_iter()
                .all(|t: &Exchange| words(&t.bot).intersection(&original).take(3).count() == 3);
        } else if turn.module != "greeting" {
            intervening.push(turn)
        }
    }
    false
}
pub(crate) fn pool(
    want: Option<&str>,
    line: &str,
    memory: &Memory,
) -> Option<&'static [&'static str]> {
    let mut want = want.unwrap_or("");
    let test = |p: &Regex| matches(p, line);
    if want == "steps" {
        if test(&A_PICK_ASK) {
            want = "pick"
        } else if test(&A_WORK_OUT) {
            want = "amount"
        } else if test(&A_STRAIGHT_YESNO) {
            want = "yesno"
        }
    }
    if want == "amount" && test(&A_TIME_WORD) {
        want = "time"
    }
    if test(&A_DATE_ON_IT)
        && (test(&A_WHOSE_THING)
            || recent(memory, 3).any(|t| {
                let s = t.user.to_lowercase();
                matches(&A_WHOSE_THING, &s) && matches(&THEIR_THINGS, &s)
            }))
    {
        want = "paperwork"
    }
    if matches!(want, "time" | "amount") && (test(&A_FREQUENCY) || test(&A_FACT_ABOUT_TIME))
        || matches!(want, "pick" | "options") && !test(&AN_ASK_AT_THE_FRONT) && !test(&TWO_THINGS)
        || want == "amount" && test(&A_STORED_COUNT)
    {
        return None;
    }
    let past = !memory.turns.is_empty();
    if test(&A_WHAT_WAS_IT) {
        if past {
            return None;
        }
        if test(&A_THEY_SAID_SO) {
            want = "recall"
        }
    }
    if test(&A_VAGUE) {
        want = "vague"
    }
    if matches!(want, "pick" | "options") && (test(&WHICH_I_NEED) || test(&WHAT_TO_DO_WITH)) {
        return None;
    }
    if !LINES.contains(&want) || recent(memory, 3).any(|t| matches(&HEAVY, &t.user.to_lowercase()))
    {
        return None;
    }
    let leans = test(&LEANS_ON_THE_THREAD);
    if past
        && (want != "yesno" && leans
            || matches!(want, "pick" | "steps" | "options")
                && test(&BARE_ASK)
                && recent(memory, 2).any(|t| t.bot.split_whitespace().take(25).count() == 25))
    {
        return None;
    }
    if want == "yesno" {
        if test(&AN_ENOUGH_ASK) {
            return Some(AMOUNT_LINES);
        }
        if test(&A_FIT_STATE) && test(&A_POINTS_AT_A_THING)
            || leans && !test(&A_VERDICT_WORD)
            || !past && !test(&A_DECISION)
        {
            return None;
        }
    }
    if matches!(want, "amount" | "time")
        && (test(&ON_PAPER)
            || said(memory, 4).any(|s| matches(&ON_PAPER, &s.to_lowercase()))
            || said(memory, 2).any(|s| s.chars().any(|c| c.is_numeric())))
    {
        return None;
    }
    if count_just_named(line, memory) {
        return None;
    }
    match want {
        "pick" if options_named(line, memory) => Some(PICK_LINES_NAMED),
        "pick" => Some(PICK_LINES),
        "options" => Some(OPTIONS_LINES),
        "amount" => Some(AMOUNT_LINES),
        "time" => Some(TIME_LINES),
        "yesno" => Some(YESNO_LINES),
        "reason" => Some(REASON_LINES),
        "advice" => Some(ADVICE_LINES),
        "vague" => Some(VAGUE_LINES),
        "recall" => Some(RECALL_LINES),
        "paperwork" => Some(PAPERWORK_LINES),
        _ => None,
    }
}
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.line.clean.trim();
    let options = pool(reading.wants, line, memory)?;
    if care_thread(memory) {
        return None;
    }
    let reply = super::fallback::respond(reading, memory)?;
    if !super::fallback::placeholder(&reply) {
        return None;
    }
    let floor = crate::bot::modules::greeting::respond(reading, memory)?;
    super::fallback::placeholder(&floor)
        .then(|| super::choose_recent(options, memory, line).to_owned())
}
