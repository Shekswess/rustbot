//! Reply to a short question about whether the step just given is itself the fix.
#[path = "fix_asked_data.rs"]
mod data;
#[cfg(test)]
#[path = "fix_asked_tests.rs"]
mod tests;

use crate::bot::{Exchange, Memory, Reading, compose, knowledge::lookup, modules::answers};
use fancy_regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    ASKS_ABOUT_A_STEP,
    ASKS_FOR_THE_STEP,
    A_FIX_WORD,
    A_WAIT_IN_THE_STEP,
    NOT_A_STEP_TO_WATCH_WORDS,
    WORD,
    OWN_PATTERN,
    ACK_PATTERN,
    A_STEP_ASK,
    PUNT_START,
    PUNT_CHAT,
    PUNT_OFFER,
    METHOD_DOUBT,
    A_CHECK_ASK,
    BARE,
    ENOUGH_ASK,
    A_THINKING_ABOUT,
    A_DECISION_ASK,
    A_PLAN,
    A_HOW_TO_ASK,
    A_LENGTH_IN_THE_ANSWER,
    REFER,
    NAMES_A_CLINICIAN,
    NEGATED,
    AFTER,
);
static CLAUSE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[:;.!?]|,?\s+and\s+").unwrap());
static LETTERS: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"[a-z']+").unwrap());
static BODY_WORD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-z'-]+").unwrap());
static DOING_CLAUSE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r",|;| and | then | so | but ").unwrap());
static CONDITION: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\b(?:if|unless|when|once)\b").unwrap());
static AMOUNT: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\bhow (?:much|many)\b|\bamount\b|\benough\b").unwrap());

fn search(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn start(pattern: &Regex, text: &str) -> bool {
    pattern
        .find(text)
        .ok()
        .flatten()
        .is_some_and(|m| m.start() == 0)
}

fn flat(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    WORD.find_iter(text)
        .filter_map(Result::ok)
        .map(|word| word.as_str())
}

fn order_at_start(line: &str) -> bool {
    let mut words = words(line.trim_matches([' ', ',', '-']));
    words
        .next()
        .is_some_and(|first| data::IMPERATIVES.contains(&first))
        && words.next().is_none_or(|second| {
            !matches!(second, "me" | "us") && !data::AFTER_A_NOUN.contains(&second)
        })
}

fn has_instruction(said: &str) -> bool {
    let flat = flat(said);
    let opening = flat.split([':', ';', '.', '!', '?']).next().unwrap_or("");
    order_at_start(opening) || CLAUSE.split(&flat).any(order_at_start)
}

fn is_punt(turn: &Exchange, said: &str) -> bool {
    matches!(turn.module, "chat" | "greeting" | "fallback")
        && start(&PUNT_START, said)
        && (turn.module != "chat" || search(&PUNT_CHAT, said))
        && (search(&PUNT_OFFER, said)
            || (!said.contains('?') && said.split_whitespace().count() <= 8))
}

fn echo_words(line: &str) -> Vec<String> {
    words(&line.to_lowercase())
        .map(|word| word.replace('\'', ""))
        .collect()
}

fn discarded_echo(turn: &Exchange, prior: Option<&Exchange>) -> bool {
    let said = echo_words(&turn.bot);
    if said.is_empty() {
        return false;
    }
    let theirs: HashSet<_> = echo_words(&turn.user).into_iter().collect();
    if !theirs.is_empty()
        && said.len() <= 12
        && said
            .iter()
            .all(|word| data::ECHO_FILLER.contains(&word.as_str()) || theirs.contains(word))
    {
        return true;
    }
    let Some(prior) = prior.filter(|before| before.module == "answers") else {
        return false;
    };
    let had: HashSet<_> = echo_words(&prior.bot).into_iter().collect();
    said.len() < had.len()
        && said.iter().all(|word| {
            had.contains(word)
                || data::RECAP_FILLER.contains(&word.as_str())
                || data::ECHO_FILLER.contains(&word.as_str())
        })
}

fn only_acknowledges(said: &str) -> bool {
    start(&ACK_PATTERN, said)
        && LETTERS.find_iter(said).count() <= 14
        && !LETTERS
            .find_iter(said)
            .any(|word| data::IMPERATIVES.contains(&word.as_str()))
}

fn last_reply(memory: &Memory, past_the_nod: bool) -> Option<&Exchange> {
    let mut nod = None;
    for (index, turn) in memory.turns.iter().enumerate().rev() {
        if data::UNREADABLE.contains(&turn.module) {
            return nod;
        }
        let said = flat(&turn.bot);
        if (turn.module == "chat" && search(&OWN_PATTERN, &said))
            || data::REPORT_REPLIES.contains(&said.as_str())
            || (data::COMPOSED.contains(&turn.module)
                && (said.trim_end_matches([' ', '.']).ends_with(data::WHY_END)
                    || data::WHY_FRONT
                        .iter()
                        .any(|prefix| said.starts_with(prefix))))
            || (data::READABLE.contains(&turn.module) && only_acknowledges(&said))
            || (turn.module != "answers" && start(&A_STEP_ASK, &flat(&turn.user)))
        {
            continue;
        }
        if is_punt(turn, &said) {
            if !past_the_nod || nod.is_some() {
                return nod.or(Some(turn));
            }
            nod = Some(turn);
            continue;
        }
        let before = index
            .checked_sub(1)
            .and_then(|index| memory.turns.get(index));
        if discarded_echo(turn, before) {
            continue;
        }
        if turn.module == "answers" {
            return Some(turn);
        }
        return nod.or_else(|| data::READABLE.contains(&turn.module).then_some(turn));
    }
    nod
}

fn line_behind(memory: &Memory) -> Option<&str> {
    for (index, turn) in memory.turns.iter().enumerate().rev() {
        if data::UNREADABLE.contains(&turn.module) {
            return None;
        }
        let before = index
            .checked_sub(1)
            .and_then(|index| memory.turns.get(index));
        if (turn.module == "chat" && search(&OWN_PATTERN, &flat(&turn.bot)))
            || (turn.module != "answers" && start(&A_STEP_ASK, &flat(&turn.user)))
            || discarded_echo(turn, before)
        {
            continue;
        }
        return data::READABLE
            .contains(&turn.module)
            .then_some(turn.user.as_str());
    }
    None
}

fn source_domain(memory: &Memory) -> Option<&'static str> {
    let earlier = memory
        .turns
        .iter()
        .rev()
        .find(|turn| turn.module == "answers")?;
    let card = lookup::lookup_reading(&Reading::new(&earlier.user))?;
    (card.reply.as_ref() == earlier.bot
        || memory
            .entries
            .on_screen(&earlier.bot, earlier.module)
            .is_some())
    .then_some(card.domain)
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let ask = flat(reading.raw);
    if ask.split_whitespace().count() > 10
        || !search(&ASKS_ABOUT_A_STEP, &ask)
        || !search(&A_FIX_WORD, &ask)
        || start(&ASKS_FOR_THE_STEP, &ask)
    {
        return None;
    }
    let last = last_reply(memory, false)?;
    if !has_instruction(&last.bot)
        || search(&NOT_A_STEP_TO_WATCH_WORDS, &last.bot)
        || source_domain(memory).is_some_and(|domain| data::NOT_A_STEP_TO_WATCH.contains(&domain))
    {
        return None;
    }
    let pool = if search(&A_WAIT_IN_THE_STEP, &last.bot) {
        data::FIX_ASKED_LINES
    } else {
        data::FIX_ASKED_NOW_LINES
    };
    Some(compose::pick(pool, memory, &ask).into())
}

fn a_how_to(answer: &str) -> bool {
    AFTER
        .split(answer)
        .filter_map(Result::ok)
        .skip(1)
        .chain(super::carried_on::sentences(answer))
        .flat_map(|sentence| DOING_CLAUSE.split(sentence.trim()))
        .any(|part| {
            part.trim_matches([' ', '.', ':', ';', ',', '-'])
                .split_whitespace()
                .next()
                .is_some_and(|word| data::DOING.contains(&word.to_lowercase().as_str()))
        })
}

fn mentions_danger(answer: &str) -> bool {
    data::DANGER.iter().any(|term| {
        answer.match_indices(term).any(|(at, _)| {
            let before = &answer[..at];
            let from = before.char_indices().rev().nth(13).map_or(0, |(at, _)| at);
            let sentence = before.rfind(". ").map_or(before, |at| &before[at..]);
            !search(&NEGATED, &before[from..]) && !CONDITION.is_match(sentence)
        })
    })
}

fn pick_rotated(pool: &[&str], offset: &str, answer: &str, memory: &Memory) -> String {
    let size = pool.len();
    let start = offset.chars().enumerate().fold(0, |hash, (at, letter)| {
        (hash + ((at + 1) % size) * ((letter as usize) % size)) % size
    });
    let variants: Vec<_> = pool[start..]
        .iter()
        .chain(&pool[..start])
        .copied()
        .collect();
    compose::pick(&variants, memory, answer).into()
}

fn clinician(answer: &str) -> bool {
    search(&REFER, answer) || search(&NAMES_A_CLINICIAN, answer)
}

fn check_method(ask: &str, turn: &Exchange, memory: &Memory) -> Option<String> {
    let latest = memory.turns.last()?;
    let behind = flat(&latest.user);
    if [&*A_THINKING_ABOUT, &*A_DECISION_ASK, &*A_PLAN]
        .iter()
        .any(|pattern| search(pattern, &behind))
        || last_reply(memory, false).is_none_or(|turn| turn.module != "answers")
        || is_punt(latest, &flat(&latest.bot))
    {
        return None;
    }
    if (start(&ENOUGH_ASK, ask) || AMOUNT.is_match(ask))
        && let Some(card) = answers::offer(&Reading::new(ask), memory)
    {
        let opening: String = flat(&turn.bot).chars().take(60).collect();
        if !flat(&card.reply).contains(&opening) {
            return None;
        }
    }
    let answer = &turn.bot;
    let body = BODY_WORD
        .find_iter(&behind)
        .any(|word| data::A_BODY_WORD.contains(&word.as_str()));
    let pool = if !body {
        data::CHECK_JOB_LINES
    } else if clinician(answer) {
        data::CHECK_WATCH_LINES
    } else if search(&A_LENGTH_IN_THE_ANSWER, &flat(answer)) {
        data::CHECK_WORKS_LINES
    } else {
        data::CHECK_ONCE_LINES
    };
    Some(pick_rotated(
        pool,
        &format!("{ask} {answer}"),
        answer,
        memory,
    ))
}

pub(super) fn method_doubt(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = flat(reading.raw);
    let ask = clean.trim_matches([' ', '.', '!', '?']);
    let is_method = start(&METHOD_DOUBT, ask);
    let other_check = !start(&BARE, ask) && !start(&ENOUGH_ASK, ask) && search(&A_CHECK_ASK, ask);
    if !is_method && !other_check {
        return None;
    }
    let last = last_reply(memory, true)?;
    if !a_how_to(&last.bot) {
        return None;
    }
    if !is_method {
        return check_method(ask, last, memory);
    }
    let words = line_behind(memory).unwrap_or_default();
    let original = flat(words);
    let read = crate::bot::understanding::line::normalise(words);
    let asked_how = [&*original, &*read.plain, &*read.norm_text]
        .into_iter()
        .any(|text| search(&A_HOW_TO_ASK, text));
    if !asked_how {
        return check_method(ask, last, memory);
    }
    if mentions_danger(&last.bot) {
        return None;
    }
    let checked = check_method(ask, last, memory);
    if clinician(&last.bot) {
        return checked;
    }
    checked.or_else(|| {
        Some(pick_rotated(
            data::METHOD_OK_LINES,
            &last.bot,
            &last.bot,
            memory,
        ))
    })
}
