//! Recognize a report of work already done and choose a follow-up about that step.
#[path = "already_data.rs"]
mod data;
use crate::bot::{Memory, Reading, compose};
use data::*;
use regex::Regex;
use std::sync::LazyLock;

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static WORD_OR_NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9']+").unwrap());
fn has(list: &[&str], word: &str) -> bool {
    list.binary_search(&word).is_ok()
}
fn present(step: &str) -> &str {
    IRREGULAR
        .binary_search_by_key(&step, |&(past, _)| past)
        .map(|at| IRREGULAR[at].1)
        .unwrap_or(step)
}
fn stem(word: &str, min_len: usize) -> &str {
    if word.len() >= min_len {
        word.strip_suffix("ed")
            .or_else(|| word.strip_suffix('e'))
            .unwrap_or(word)
    } else {
        word
    }
}

fn reported(low: &str) -> Option<&str> {
    let (cap, leading) = ALREADY
        .captures(low)
        .map(|c| (c, true))
        .or_else(|| TRAILING.captures(low).map(|c| (c, false)))?;
    let step = cap.get(1)?.as_str();
    if has(TELLING, step) && APOLOGY.is_match(low) {
        return Some("apologise");
    }
    if !has(PAST, step) && !step.ends_with("ed") && !has(A_STEP_TAKEN, step) {
        return None;
    }
    if leading
        && has(SAME_AS_PRESENT, step)
        && WORD_OR_NUMBER
            .find_iter(&low[cap.get(0)?.end()..])
            .all(|w| has(POINTS_AT, w.as_str()))
    {
        return None;
    }
    Some(step)
}

fn did_step(low: &str) -> Option<&str> {
    let cap = DID.captures(low)?;
    let step = cap.get(1)?.as_str();
    (has(DID_STEPS, step)
        && WORD
            .find_iter(&low[cap.get(0)?.end()..])
            .all(|w| has(DID_TAIL, w.as_str())))
    .then_some(step)
}

fn opening(answer: &str) -> &str {
    for (i, c) in answer.char_indices() {
        if matches!(c, '.' | '!' | '?') && answer[i + 1..].starts_with(' ') {
            return &answer[..i + 1];
        }
    }
    answer
}
fn finished<'a>(low: &'a str, answer: &str) -> Option<&'a str> {
    for (pattern, act) in [
        (&*FINISHED_ACT, true),
        (&*FINISHED_ITSELF, false),
        (&*BARE_DONE, false),
        (&*DONE_THAT, false),
    ] {
        let Some(cap) = pattern.captures(low) else {
            continue;
        };
        let step = cap.get(1)?.as_str();
        if act && !has(FINISHED_STEPS, step) && step != "did" {
            continue;
        }
        if WORD
            .find_iter(&low[cap.get(0)?.end()..])
            .any(|w| !has(MORE_PADDING, w.as_str()))
        {
            continue;
        }
        let step = if act && step == "did" { "done" } else { step };
        return (!has(BARE_COMPLETION, step) || !KEPT_UP.is_match(opening(answer))).then_some(step);
    }
    None
}

enum Subject {
    Unnamed,
    Named(String),
    DifferentMeaning,
}
fn named(before: &str) -> Subject {
    let mut words = before
        .split_whitespace()
        .rev()
        .map(|s| s.trim_matches(['.', ',', '!', '\'']))
        .skip_while(|s| has(OVER_BETWEEN, s) || has(OVER_STOP, s));
    let mut thing = Vec::new();
    for w in words.by_ref() {
        if thing.is_empty() && has(OVER_NO_THING, w) {
            return Subject::Unnamed;
        }
        if [OVER_LEAD, OVER_STOP, OVER_BETWEEN, OVER_NEVER_A_THING]
            .iter()
            .any(|set| has(set, w))
        {
            break;
        }
        if has(OVER_NO_THING, w) {
            return Subject::Unnamed;
        }
        thing.push(w);
        if thing.len() == 2 {
            break;
        }
    }
    let Some(&head) = thing.first() else {
        return Subject::Unnamed;
    };
    thing.reverse();
    let name = thing.join(" ");
    if NOT_A_THING_SAID_BACK.is_match(&name)
        || thing.iter().all(|s| s.chars().all(char::is_numeric))
    {
        return Subject::Unnamed;
    }
    if [OVER_NOT_A_THING_NAMED, OVER_A_MEAL, OVER_STYLED]
        .iter()
        .any(|set| has(set, head))
    {
        return Subject::DifferentMeaning;
    }
    Subject::Named(name)
}
fn resolved(low: &str) -> Option<(&str, Subject)> {
    if OVER_ASKING.is_match(low) {
        return None;
    }
    let words = low.split_whitespace().collect::<Vec<_>>();
    for cap in OVER_WORD.captures_iter(low) {
        let found = cap.get(0)?;
        let at = low[..found.start()].split_whitespace().count();
        if words
            .get(at + 1)
            .is_some_and(|w| has(OVER_ADJECTIVE, w.trim_matches(['.', ',', '!'])))
        {
            continue;
        }
        let near = words[at.saturating_sub(4)..at]
            .iter()
            .map(|s| s.trim_matches(['.', ',', '!', '\'']))
            .collect::<Vec<_>>();
        let got = near
            .iter()
            .position(|w| has(OVER_GOT, w))
            .unwrap_or_default();
        if near[got..].iter().any(|w| has(OVER_PEOPLE, w)) {
            continue;
        }
        let mut start = 0;
        let mut end = low.len();
        for join in CLAUSE_BREAK.find_iter(low) {
            if join.end() <= found.start() {
                start = join.end();
            } else if join.start() >= found.start() {
                end = join.start();
                break;
            }
        }
        if OVER_NOT.is_match(&low[start..end]) || !OVER_STATE.is_match(&near.join(" ")) {
            continue;
        }
        let subject = named(&low[..found.start()]);
        if !matches!(subject, Subject::DifferentMeaning) {
            return Some((cap.get(1)?.as_str(), subject));
        }
    }
    None
}

fn says_step(step: &str, answer: &str) -> bool {
    if step.starts_with("apolog") {
        return SAYS_APOLOGY.is_match(answer);
    }
    if has(AN_OUTCOME, step) {
        return compose::instruction::first_instruction(answer).is_some();
    }
    let word = present(step);
    let base = stem(word, 4);
    WORD.find_iter(&answer.to_lowercase())
        .any(|w| w.as_str().starts_with(base) || w.len() >= 4 && word.starts_with(w.as_str()))
}
fn changed_item(step: &str, low: &str) -> bool {
    if !has(CHANGED_OF_A_THING, step) {
        return false;
    }
    let mut words = WORD.find_iter(low).map(|w| w.as_str());
    words.any(|w| w == step)
        && !words.any(|w| has(A_LIFE_OBJECT, w) || has(CHANGED_NOTHING_BROKEN, w))
}

static AND_STEP: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(r#"(?i)\s+(?:and|then|so|but)\s+(?=(?:start|begin|use|put|place|leave|let|keep|wipe|rub|mix|make|add|take|coat|scrub|brush|soak|pour|drain|wash|rinse|blot|hang|air|open|close|undo|unscrew|pull|lift|fill|top|cut|trim|cover|seal|spray|check|turn|hold|run|go|get|do|give|ask|book|call|wait|set|sit|stand|watch|pick|dig|move|remove|replace|clean|unplug|vacuum|sweep|boil|simmer|bake|rest|chill|store|freeze|test|look|try|screw|tighten|loosen|slide)\b)"#).unwrap()
});

static SENTENCES: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(r"(?<=[.!?])\s+").unwrap());
static CLAUSES: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(r",\s+(?=[a-z])").unwrap());
fn after_step(step: &str, answer: &str, line: &str) -> Vec<String> {
    let word = present(step);
    let base = if word.len() > 4 && word.ends_with("ed") {
        &word[..word.len() - 2]
    } else if word.len() > 3 && word.ends_with('e') {
        &word[..word.len() - 1]
    } else {
        word
    };
    let wanted: std::collections::HashSet<_> = WORD
        .find_iter(line)
        .map(|w| w.as_str())
        .filter(|w| {
            w.len() > 1
                && !has(NOT_A_THING_WORD, w)
                && !has(REPORT_FILLER, w)
                && !w.starts_with(base)
        })
        .collect();
    let steps: Vec<_> = SENTENCES
        .split(answer)
        .flatten()
        .flat_map(|s| CLAUSES.split(s.trim()).flatten())
        .flat_map(|s| AND_STEP.split(s).flatten())
        .map(|s| s.trim_matches([' ', '.', ',', ';', ':']))
        .filter(|s| !s.is_empty())
        .collect();
    let mut best = None;
    let mut best_score = 0;
    for (i, s) in steps.iter().enumerate() {
        let low = s.to_lowercase();
        let words = WORD
            .find_iter(&low)
            .map(|w| w.as_str())
            .collect::<std::collections::HashSet<_>>();
        let score = words.intersection(&wanted).count() + usize::from(low.contains(base));
        if score > best_score {
            best_score = score;
            best = Some(i);
        }
    }
    let Some(best) = best.filter(|_| wanted.is_empty() || best_score >= 2) else {
        return Vec::new();
    };
    let mut next = Vec::new();
    let mut condition = false;
    for &step in steps.iter().skip(best + 1).take(4) {
        let step = step
            .split_once(':')
            .filter(|(head, _)| head.split_whitespace().count() >= 2)
            .map_or(step, |(head, _)| {
                head.trim_end_matches([' ', '.', ',', ';'])
            });
        let Some(first) = step.chars().next() else {
            break;
        };
        if next.is_empty()
            && step
                .split_whitespace()
                .next()
                .is_some_and(|w| has(NOT_A_STEP_START, &w.to_lowercase()))
        {
            break;
        }
        if first.is_uppercase() {
            if condition || !CONDITION_START.is_match(step) {
                break;
            }
            condition = true;
        }
        next.push(format!(
            "{}{}",
            first.to_lowercase(),
            &step[first.len_utf8()..]
        ));
    }
    next
}

fn previous_reply(memory: &Memory) -> &str {
    memory
        .turns
        .iter()
        .rev()
        .find(|turn| !turn.bot.is_empty())
        .map_or("", |turn| turn.bot.as_str())
}

fn answer_in_hand(memory: &Memory) -> &str {
    super::plan::the_answer_in_hand(memory).unwrap_or_else(|| previous_reply(memory))
}

fn step_report<'a>(
    low: &'a str,
    last: &str,
    memory: &Memory,
) -> Option<(&'a str, Option<Subject>, bool)> {
    if low.is_empty()
        || low.contains('?')
        || low.split_whitespace().count() > 9
        || TO_THE_BOT.is_match(low)
    {
        return None;
    }
    if memory
        .turns
        .iter()
        .rev()
        .take(2)
        .any(|t| HARD.is_match(&t.user) || HARD.is_match(&t.bot))
    {
        return None;
    }
    let explicit = reported(low).or_else(|| did_step(low));
    if let Some((step, subject)) = resolved(low) {
        return Some((step, Some(subject), explicit.is_some()));
    }
    if ASKING.is_match(low) && !DID.is_match(low) {
        return None;
    }
    let step = match explicit {
        Some(step) => step,
        None if !last.is_empty() => finished(low, last)?,
        None => return None,
    };
    Some((step, None, explicit.is_some()))
}

pub(crate) fn claims(reading: &Reading<'_>, memory: &Memory) -> bool {
    let answer = answer_in_hand(memory);
    step_report(reading.folded(), answer, memory)
        .is_some_and(|(step, done, _)| done.is_some() || says_step(step, answer))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let low = reading.folded();
    let last = answer_in_hand(memory);
    let (step, done, explicit) = step_report(low, last, memory)?;
    if let Some(subject) = done {
        if !explicit && last.is_empty() && finished(low, "").is_some() {
            return None;
        }
        if memory.turns.iter().rev().take(2).any(|t| {
            CRISIS_WORDS
                .iter()
                .any(|w| t.bot.to_lowercase().contains(w))
        }) {
            return None;
        }
        if let Subject::Named(thing) = subject {
            let pool = if has(OVER_A_JOB_DONE, step) {
                OVER_NAMED_JOB_REPLIES
            } else {
                OVER_NAMED_REPLIES
            };
            return Some(
                compose::choose_mixed(pool, memory, reading.raw).replace("{thing}", &thing),
            );
        }
        return Some(compose::choose_mixed(OVER_REPLIES, memory, reading.raw).into());
    }
    if has(SAID_TO_ME, step) && !memory.turns.iter().any(|t| !t.user.trim().is_empty()) {
        return Some(compose::pick(FIRST_LINE_REPLIES, memory, reading.raw).into());
    }
    if !has(BARE_COMPLETION, step) && !last.is_empty() {
        let next = after_step(step, last, low);
        if !next.is_empty() {
            let joined = next.join(", ");
            let said = joined.trim_end_matches([' ', '.']);
            let said = said
                .strip_prefix("and then ")
                .or_else(|| said.strip_prefix("then "))
                .unwrap_or(said);
            return Some(compose::choose_recent(STEPS_LEADS, memory, said).replace("{said}", said));
        }
    }
    if has(A_STEP_TAKEN, step)
        && !has(CONTACT, step)
        && !has(TELLING, step)
        && !last.is_empty()
        && !says_step(step, opening(last))
    {
        return None;
    }
    let pool = if has(NOBODY_TO_ANSWER, step) || has(CONTACT, step) && AN_APOLOGY.is_match(low) {
        REPORT_REPLIES
    } else if has(CONTACT, step) {
        CONTACT_REPLIES
    } else if has(REPAIRED, step) || changed_item(step, low) {
        REPAIRED_REPLIES
    } else if has(CHANGED, step) {
        CHANGED_REPLIES
    } else if has(SETTLED, step) {
        SETTLED_REPLIES
    } else {
        REPORT_REPLIES
    };
    Some(compose::choose_recent(pool, memory, previous_reply(memory)).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn already_uses_the_pet_acknowledgement_to_seed_the_council_reply() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let first = bot.turn("i already called the plumber");
        assert_eq!(first.reply, "Good. Have they said anything back yet?");
        let second = bot.turn("i already have a dog");
        assert_eq!(second.module, Some("greeting"));
        assert_eq!(second.reply, "A dog. What's it like to live with?");
        let third = bot.turn("i already told the council");
        assert_eq!(third.module, Some("already"));
        assert_eq!(third.reply, "Right. Did they give you an answer?");
    }

    #[test]
    fn already_claims_only_a_step_the_previous_answer_named_or_a_resolution() {
        let memory = Memory {
            turns: vec![Exchange {
                user: "my tap is leaking".to_owned(),
                bot: "Call a plumber and tell them where the tap is leaking.".to_owned(),
                module: "answers",
            }]
            .into(),
            ..Memory::default()
        };
        assert!(claims(
            &Reading::new("i already called the plumber"),
            &memory
        ));
        assert!(!claims(
            &Reading::new("i already emailed the council"),
            &memory
        ));
        assert!(claims(&Reading::new("my tap is fixed"), &memory));
    }
}
