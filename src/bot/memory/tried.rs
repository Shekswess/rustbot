//! The next instruction after the user reports following part of the preceding answer.
#[path = "tried_data.rs"]
mod data;
#[cfg(test)]
#[path = "tried_tests.rs"]
mod tests;
use super::{ASKING_THEM, Carried, DO_NOT, MemoryNote, RED_FLAG, cap, seen, sentences, starts};
use crate::bot::{Memory, compose, memory::ack};
use fancy_regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).unwrap());)+
    };
}
patterns!(
    WORD,
    STEP_TAIL_NOUN,
    NOT_WORKED,
    IMPROVED,
    WORSE,
    STEP_ALREADY,
    STEP_TRIED,
    STEP_LAST,
    STEP_DONE,
    STEP_DID,
    PAST_STEP,
    PAST_STEP_ED,
    DOING_WORD,
    MOVES,
    A_CONDITION,
    CLAUSE,
    PRONOUN_TAIL,
    TAIL_WORD,
    KEEPS_IT_RIGHT,
    BUYING,
    PAST_WORD,
    NEXT_ASK,
);

const TRIM: &[char] = &[' ', '.', ',', ';', ':'];
const ATTEMPT_TRIM: &[char] = &[' ', '.', ',', '!', '\''];
static CONTRAST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\s+(but|and|though|although|yet)\s+").unwrap());
static ALREADY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+already$").unwrap());
static TRY_USED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:tried|used)\s+").unwrap());
static FINAL_THEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r",?\s*\bthen$").unwrap());
static JOIN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^(?:and|but|or)\s+").unwrap());
static ACTED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?:i|we)\s+(?:(?:have|had|already|just|finally|actually)\s+)*",
        r"(?:told|done|did|sent|texted|messaged|emailed|called|rang|phoned|asked|",
        r"said\s+(?:(?:it|that)\s+)?to|spoke\s+(?:to|with)|talked\s+(?:to|with)|",
        r"said\s+what\s+you\s+(?:suggested|said)|",
        r"used\s+(?:it|that|your\s+(?:wording|words|line|suggestion|advice|text)))\b",
    ))
    .unwrap()
});
static SAY_ADVICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:what|how|which)\s+(?:(?:should|shall|do|can|could|would)\s+)?(?:i|we)\s+",
        r"(?:say|tell|text|message|write|email|call|ask|talk|speak|word)\b",
        r"|\bwhat\s+to\s+(?:say|tell|text|message|write|email)\b",
        r"|\b(?:how|what)\s+(?:would|could|should)\s+you\s+(?:say|word|phrase)\b",
    ))
    .unwrap()
});
static TOLD_THE_BOT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:told|said|texted|messaged|emailed|called|asked)\s+you\b").unwrap()
});
const ACTED_REPLIES: &[&str] = &[
    "How did it land?",
    "What did they say?",
    "Did it come across the way you meant?",
    "How did that go over?",
    "Did it help?",
];

#[derive(Clone)]
pub(in crate::bot::memory) struct Attempt {
    turn: usize,
    step: String,
    answer: String,
    seen: Vec<String>,
}

fn flat(text: &str) -> String {
    text.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches(TRIM)
        .into()
}

fn words(text: &str) -> Vec<String> {
    let low = text.to_lowercase().replace('’', "'");
    WORD.find_iter(&low)
        .filter_map(Result::ok)
        .map(|w| w.as_str().to_owned())
        .collect()
}

fn report_on_attempt(line: &str) -> bool {
    seen(&NOT_WORKED, line) || seen(&IMPROVED, line) || seen(&WORSE, line)
}

fn named_attempt(text: &str) -> Option<(String, bool)> {
    let line = text
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let line = line.trim_matches(['.', ',', '!']);
    if line.is_empty()
        || line.contains('?')
        || line.split_whitespace().count() > 14
        || report_on_attempt(line)
    {
        return None;
    }
    for (pattern, tried) in [
        (&*STEP_ALREADY, false),
        (&*STEP_TRIED, true),
        (&*STEP_LAST, false),
        (&*STEP_DONE, false),
        (&*STEP_DID, false),
    ] {
        let Some(found) = pattern.captures(line).ok().flatten() else {
            continue;
        };
        if found.get(0).is_none_or(|found| found.start() != 0) {
            continue;
        }
        let Some(what) = found.name("what") else {
            continue;
        };
        let what = ALREADY.replace(what.as_str().trim_matches(ATTEMPT_TRIM), "");
        let what = TRY_USED.replace(&what, "");
        let said = CONTRAST
            .captures(&what)
            .ok()
            .flatten()
            .and_then(|joint| {
                let full = joint.get(0)?;
                let joining = joint.get(1)?.as_str();
                if joining == "and" && starts(&STEP_TAIL_NOUN, what[full.end()..].trim()) {
                    return None;
                }
                Some(&what[..full.start()])
            })
            .unwrap_or(&what);
        let said = said.trim_matches(ATTEMPT_TRIM);
        if words(said).iter().any(|word| {
            !data::STEP_EMPTY.contains(&word.as_str()) && !data::STEP_VAGUE.contains(&word.as_str())
        }) {
            return Some((said.into(), tried));
        }
    }
    None
}

fn past_attempt(text: &str, answer: &str) -> Option<String> {
    let line = text
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let line = line.trim_matches(['.', ',', '!']);
    if line.is_empty() || answer.is_empty() || line.contains('?') || report_on_attempt(line) {
        return None;
    }
    let found = PAST_STEP.captures(line).ok().flatten()?;
    let said = ALREADY.replace(found.name("what")?.as_str().trim_matches(ATTEMPT_TRIM), "");
    let head = CONTRAST
        .find(&said)
        .ok()
        .flatten()
        .map_or(said.as_ref(), |found| &said[..found.start()])
        .trim_matches(ATTEMPT_TRIM);
    let kept: Vec<_> = words(head)
        .into_iter()
        .filter(|word| !data::STEP_EMPTY.contains(&word.as_str()))
        .collect();
    let first = kept.first()?;
    (kept.len() >= 2
        && (data::PAST_IRREGULAR.contains(&first.as_str())
            || data::SHORT_PAST.contains(&first.as_str())
            || starts(&PAST_STEP_ED, first))
        && in_answer(head, answer))
    .then(|| head.to_owned())
}

fn stem(word: &str) -> &str {
    for ending in ["ing", "ed", "es", "s"] {
        if let Some(base) = word.strip_suffix(ending).filter(|base| base.len() >= 3) {
            if matches!(ending, "ing" | "ed")
                && base.len() > 3
                && base
                    .as_bytes()
                    .windows(2)
                    .last()
                    .is_some_and(|tail| tail[0] == tail[1])
            {
                return &base[..base.len() - 1];
            }
            return base;
        }
    }
    word
}

fn meaningful(word: &str) -> bool {
    !data::STEP_EMPTY.contains(&word) && !data::STEP_VAGUE.contains(&word)
}

fn stems(thing: &str) -> HashSet<String> {
    words(thing)
        .into_iter()
        .filter(|word| meaningful(word))
        .map(|word| stem(&word).to_owned())
        .collect()
}

fn shares_stem(got: &str, tried: &str) -> bool {
    got == tried
        || (got.len() >= 5
            && tried
                .strip_prefix(got)
                .is_some_and(|tail| data::SAME_STEP.contains(&tail)))
        || (tried.len() >= 5
            && got
                .strip_prefix(tried)
                .is_some_and(|tail| data::SAME_STEP.contains(&tail)))
}

fn names(part: &str, stems: &HashSet<String>) -> bool {
    words(part)
        .iter()
        .any(|word| stems.iter().any(|tried| shares_stem(stem(word), tried)))
}

fn imperative(word: &str) -> bool {
    super::data::IMPERATIVES.contains(&word)
}

fn opens_step(part: &str) -> bool {
    let is_action = |first: &str| imperative(first) || data::MORE_STEP_VERBS.contains(&first);
    if words(part).first().is_some_and(|first| is_action(first)) {
        return true;
    }
    part.trim().split_once(", ").is_some_and(|(head, tail)| {
        starts(&A_CONDITION, head) && words(tail).first().is_some_and(|first| is_action(first))
    })
}

fn says_action(part: &str) -> bool {
    let lower = part.to_lowercase();
    let mut explains = false;
    for word in lower.split_whitespace().map(|word| word.trim_matches(TRIM)) {
        if imperative(word) {
            return !explains || DO_NOT.is_match(part);
        }
        explains |= matches!(word, "is" | "are" | "was" | "were" | "means" | "sounds");
    }
    DO_NOT.is_match(part)
}

fn tail_of_step(part: &str) -> bool {
    starts(&PRONOUN_TAIL, part) || (starts(&TAIL_WORD, part) && !opens_step(part))
}

fn clause_pieces(text: &str) -> Vec<String> {
    let cleaned = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut parts = Vec::new();
    let mut piece = String::new();
    let mut depth: isize = 0;
    for chunk in CLAUSE.split(&cleaned).filter_map(Result::ok) {
        depth += chunk.matches('(').count() as isize - chunk.matches(')').count() as isize;
        if !piece.is_empty() && depth > 0 {
            piece.push_str(", ");
            piece.push_str(chunk);
            continue;
        }
        if !piece.trim_matches(TRIM).is_empty() {
            parts.push(piece.trim_matches(TRIM).to_owned());
        }
        piece.clear();
        piece.push_str(chunk);
    }
    if !piece.trim_matches(TRIM).is_empty() {
        parts.push(piece.trim_matches(TRIM).to_owned());
    }
    parts
}

fn step_piece(text: &str) -> Option<String> {
    let mut piece = text.trim_matches(TRIM).to_owned();
    if piece.split_whitespace().count() > data::STEP_WORDS {
        let head = piece.split(": ").next()?.trim_matches(TRIM);
        if !head.is_empty() && head != piece {
            piece = head.to_owned();
        }
    }
    if piece.split_whitespace().count() > data::STEP_WORDS
        && let Some((head, tail)) = piece.split_once(", ")
        && starts(&A_CONDITION, head)
    {
        let first = tail.split(", ").next()?.trim_matches(TRIM);
        if !first.is_empty() && opens_step(first) {
            piece = format!("{head}, {first}");
        }
    }
    (!piece.is_empty()
        && piece.split_whitespace().count() <= data::STEP_WORDS
        && piece.chars().count() <= 180)
        .then_some(piece)
}

fn steps(answer: &str) -> Vec<String> {
    let mut parts = Vec::new();
    for piece in sentences(answer)
        .flat_map(clause_pieces)
        .filter_map(|clause| step_piece(&clause))
    {
        if !parts.contains(&piece) {
            parts.push(piece);
        }
    }
    parts
}

fn list_heads(answer: &str) -> HashSet<String> {
    sentences(answer)
        .filter_map(|sent| sent.split_once(':'))
        .filter(|(_, tail)| !tail.trim().is_empty())
        .flat_map(|(head, _)| clause_pieces(head))
        .filter_map(|piece| step_piece(&piece))
        .map(|step| flat(&step))
        .collect()
}

fn in_answer(thing: &str, said: &str) -> bool {
    let known = stems(thing);
    if known.is_empty() {
        return false;
    }
    let steps = steps(said);
    if !steps.iter().any(|part| names(part, &known)) {
        return false;
    }
    let objects: HashSet<_> = words(thing)
        .into_iter()
        .filter(|word| {
            meaningful(word)
                && !data::STEP_TIMING.contains(&word.as_str())
                && !data::PAST_IRREGULAR.contains(&word.as_str())
                && !data::SHORT_PAST.contains(&word.as_str())
                && !starts(&PAST_STEP_ED, word)
                && !starts(&DOING_WORD, word)
        })
        .map(|word| stem(&word).to_owned())
        .collect();
    objects.is_empty() || steps.iter().any(|piece| names(piece, &objects))
}

fn move_parts(part: &str, wanted: &HashSet<String>) -> (String, String) {
    if starts(&A_CONDITION, part.trim()) {
        return (String::new(), String::new());
    }
    let parts: Vec<_> = MOVES
        .split(part)
        .filter_map(Result::ok)
        .map(|bit| bit.trim_matches(TRIM))
        .filter(|bit| !bit.is_empty())
        .collect();
    let Some((index, _)) = parts
        .iter()
        .enumerate()
        .map(|(i, part)| {
            let actual = words(part);
            let hits = wanted
                .iter()
                .filter(|word| actual.iter().any(|found| shares_stem(found, word)))
                .count();
            (i, hits)
        })
        .filter(|(_, hits)| *hits > 0)
        .max_by_key(|(_, hits)| *hits)
    else {
        return (String::new(), String::new());
    };
    let left = parts[index + 1..].join(" and ");
    let rest = if !tail_of_step(&left) && opens_step(&left) {
        left
    } else {
        String::new()
    };
    (parts[index].into(), rest)
}

fn advice_after(parts: &[String]) -> Option<String> {
    parts
        .iter()
        .rev()
        .find(|part| {
            words(part).len() >= 5
                && !seen(&ASKING_THEM, part)
                && !starts(&RED_FLAG, part)
                && !tail_of_step(part)
                && seen(&KEEPS_IT_RIGHT, part)
        })
        .cloned()
}

fn with_tail(part: &str, parts: &[String], index: usize) -> String {
    if index + 2 != parts.len() {
        return part.into();
    }
    let next = &parts[index + 1];
    if !says_action(next)
        || seen(&ASKING_THEM, next)
        || starts(&A_CONDITION, next)
        || list_heads(next).contains(&flat(next))
        || words(part).len() + words(next).len() > 40
    {
        return part.into();
    }
    format!("{}, and {}", part.trim_end_matches(TRIM), cap(next, false))
}

fn walk_answer(said: &str, thing: &str, wrap: bool) -> Option<String> {
    let parts = steps(said);
    let wanted = stems(thing);
    if parts.is_empty() || wanted.is_empty() {
        return None;
    }
    let heads = list_heads(said);
    let (at, tried) = parts
        .iter()
        .enumerate()
        .find(|(_, part)| names(part, &wanted))?;
    let (named, ahead) = move_parts(tried, &wanted);
    if !ahead.is_empty() && says_action(&ahead) {
        return Some(ahead);
    }
    if !opens_step(tried) && !opens_step(&named) {
        return if wrap {
            None
        } else {
            advice_after(&parts[at + 1..])
        };
    }
    for (index, part) in parts.iter().enumerate().skip(at + 1) {
        if names(part, &wanted) {
            let (_, next) = move_parts(part, &wanted);
            if !next.is_empty() && says_action(&next) {
                return Some(next);
            }
            continue;
        }
        if !seen(&ASKING_THEM, part)
            && !starts(&RED_FLAG, part)
            && !tail_of_step(part)
            && !heads.contains(&flat(part))
            && opens_step(part)
        {
            return Some(with_tail(part, &parts, index));
        }
    }
    if !wrap {
        return advice_after(&parts[at + 1..]);
    }
    parts[..at]
        .iter()
        .find(|part| {
            !tail_of_step(part)
                && !heads.contains(&flat(part))
                && opens_step(part)
                && says_action(part)
        })
        .cloned()
}

fn first_left(said: &str, seen_parts: &[String], previous: &str) -> Option<String> {
    let heads = list_heads(said);
    let parts = steps(said);
    let where_ = flat(previous);
    let start = parts
        .iter()
        .position(|part| {
            let piece = flat(part);
            !where_.is_empty() && (piece.contains(&where_) || where_.contains(&piece))
        })
        .map_or(0, |at| at + 1);
    let already_seen = |part: &str| {
        let normalized = flat(part);
        heads.contains(&normalized) || seen_parts.iter().any(|seen| normalized.contains(seen))
    };
    for part in parts[start..].iter().filter(|part| !already_seen(part)) {
        if opens_step(part) && says_action(part) {
            return Some(part.clone());
        }
        let lower = flat(part);
        let joined = JOIN.replace(&lower, "");
        if starts(&A_CONDITION, &joined) && says_action(&joined) {
            return Some(if joined != lower {
                cap(&joined, true)
            } else {
                lower
            });
        }
    }
    if previous.is_empty() {
        return None;
    }
    parts[start..]
        .iter()
        .find(|part| !already_seen(part) && !seen(&ASKING_THEM, part) && says_action(part))
        .cloned()
}

fn as_lead(lead: &str, thing: &str, step: &str) -> String {
    let thing = ack::you(thing);
    let step = FINAL_THEN.replace(step.trim_matches(TRIM), "");
    let step = step.trim_end_matches([' ', ',', ';', ':']);
    let first = step.split_whitespace().next().unwrap_or("");
    let preserve = matches!(first, "I" | "I'm" | "I've" | "I'd")
        || (first.len() > 1
            && first.chars().any(char::is_uppercase)
            && !first.chars().any(char::is_lowercase));
    let step = if preserve {
        step.to_owned()
    } else {
        cap(step, false)
    };
    let reply = lead
        .replace("{thing}", &thing)
        .replace("{Thing}", &cap(&thing, true))
        .replace("{step}", &step)
        .replace("{Step}", &cap(&step, true));
    format!("{}.", reply.trim_end_matches([' ', '.']))
}

fn knowledge_answer(memory: &Memory) -> Option<&str> {
    let turn = memory.turns.last()?;
    (turn.module == "answers").then_some(turn.bot.as_str())
}

fn attempted(text: &str, memory: &Memory) -> Option<Carried> {
    let answer = knowledge_answer(memory)?;
    let last = memory.turns.last()?;
    let (thing, said_tried, past) = if let Some((named, tried)) = named_attempt(text) {
        if !in_answer(&named, answer) {
            return None;
        }
        (named, tried, false)
    } else {
        (past_attempt(text, answer)?, false, true)
    };
    if seen(&BUYING, &words(&last.user).join(" ")) {
        return None;
    }
    let step = walk_answer(answer, &thing, true)?;
    let doing = starts(&DOING_WORD, &thing) && !said_tried;
    let first = thing.split_whitespace().next().unwrap_or("");
    let habitual = doing && !starts(&PAST_WORD, first) && !data::PAST_IRREGULAR.contains(&first);
    let options = if past {
        data::DONE_LEADS
    } else if habitual {
        data::PRESENT_LEADS
    } else if doing {
        data::ALREADY_LEADS
    } else {
        data::STEP_LEADS
    };
    let reply = as_lead(compose::pick(options, memory, text), &thing, &step);
    let mut seen = memory
        .personal
        .last_attempt()
        .map_or_else(Vec::new, |old| old.seen.clone());
    seen.push(flat(&step));
    Some(Carried {
        reply,
        earlier_handback: None,
        note: MemoryNote::Attempt(Attempt {
            turn: memory.turns.len(),
            step,
            answer: answer.to_owned(),
            seen,
        }),
    })
}

fn asks_for_next(text: &str) -> bool {
    let line = text
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let line = line.trim_matches(['.', ',', '!']);
    !line.is_empty()
        && line.split_whitespace().count() <= 7
        && starts(&NEXT_ASK, line)
        && words(line)
            .iter()
            .all(|word| data::NEXT_ASK_WORDS.contains(&word.as_str()))
}

fn pending_next<'a>(text: &str, memory: &'a Memory) -> Option<&'a Attempt> {
    if !asks_for_next(text) {
        return None;
    }
    let last = memory.personal.last_attempt()?;
    if last.turn.checked_add(1) != Some(memory.turns.len())
        || last.step.trim().is_empty()
        || last.answer.is_empty()
    {
        return None;
    }
    Some(last)
}

pub(crate) fn next_step_answer<'a>(text: &str, memory: &'a Memory) -> Option<&'a str> {
    pending_next(text, memory).map(|last| last.answer.as_str())
}

/// Recover the registered paragraph when a step question follows its shortened repeat.
pub(crate) fn repeated_step_answer(text: &str, memory: &Memory) -> Option<String> {
    let last = memory.turns.last()?;
    if last.module != "answers" || super::super::plan::continuation(text).is_none() {
        return None;
    }
    let heard = super::super::again::unframed(&last.bot)?;
    let card = memory.entries.on_screen(heard, last.module)?;
    super::super::again::instead(card, text, memory)
}

fn what_next(text: &str, memory: &Memory) -> Option<Carried> {
    let last = pending_next(text, memory)?;
    let next = walk_answer(&last.answer, &last.step, false)
        .or_else(|| first_left(&last.answer, &last.seen, &last.step))?;
    if flat(&next) == flat(&last.step) {
        return None;
    }
    let mut seen = last.seen.clone();
    let flattened = flat(&next);
    if !seen.contains(&flattened) {
        seen.push(flattened);
    }
    Some(Carried {
        reply: as_lead(compose::pick(data::NEXT_LEADS, memory, text), "", &next),
        earlier_handback: None,
        note: MemoryNote::Attempt(Attempt {
            turn: memory.turns.len(),
            step: next,
            answer: last.answer.clone(),
            seen,
        }),
    })
}

pub(super) fn reply(text: &str, memory: &Memory) -> Option<Carried> {
    attempted(text, memory).or_else(|| what_next(text, memory))
}

pub(super) fn acted(text: &str, memory: &Memory) -> Option<String> {
    let last = memory.turns.last()?;
    let line = words(text).join(" ");
    (last.module != "safety"
        && seen(&ACTED, &line)
        && !seen(&TOLD_THE_BOT, &line)
        && seen(&SAY_ADVICE, &words(&last.user).join(" ")))
    .then(|| compose::pick(ACTED_REPLIES, memory, "").to_owned())
}
