//! A detail of the previous answer, read the way PyBot's memory/repeat.carried_on reads it.
#[path = "carried_on_data.rs"]
mod data;
#[path = "remedy.rs"]
mod remedy;
#[path = "returned.rs"]
mod returned;
#[path = "step_after.rs"]
mod step_after;
#[path = "step_back.rs"]
mod step_back;
pub(in crate::bot) use step_back::steps as ordered_steps;
#[cfg(test)]
#[path = "carried_on_tests.rs"]
mod tests;
#[path = "tried.rs"]
mod tried;

use super::{again, fix_asked, frame_reply};
use crate::bot::{Exchange, Memory, Reading, compose};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};
pub(super) use tried::Attempt;
pub(crate) use tried::{next_step_answer, repeated_step_answer};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Fancy> =
            LazyLock::new(|| Fancy::new(data::$name).unwrap());)+
    };
}
patterns!(
    QUESTION,
    ASKS,
    RESET_LEAD,
    ABOUT_THE_BOT,
    WORDS_WANTED,
    ASKING_TO_START,
    ACK_OPENS,
    QUESTION_MORE,
    REJOINDER,
    REFER_BACK,
    HEDGE,
    ASKING_THEM,
    TROUBLE,
    BROODING,
    OWNED,
    SOMEONE_ELSES_LIMIT,
    TRIED,
    NOT_TRIED,
    DONE,
    AGREES_IN_A_PHRASE,
    ATTEMPT,
    RED_FLAG,
    LIMIT_IN_A_CHAIR,
    LIMIT_AID,
    LIMIT_LIMIT_VERB,
    LIMIT_DISLIKES_THE_WALK,
    LIMIT_FLAT,
    LIMIT_RULE_LEAD,
    LIMIT_NO_WALKS,
    LIMIT_RULE_TAIL,
    LAST_CLAUSE,
    CONTINUES,
    PLAN,
    PLACE_AFTER,
    PLACE_LOWER,
    PLACE_LOWER_NAMED,
);

const WORD_EDGE: &[char] = &['.', ',', '!', ';', ':', '"', '\''];
pub(super) const HAND_BACK_LIMIT: usize = 8;
const OBJECT_EDGE: &[char] = &[' ', '.', ';', ':', '!', '?', '(', ')', '[', ']', '"'];
const CUT_EDGE: &[char] = &[' ', ',', ';', ':', '-'];
static SPLIT_SENTENCES: LazyLock<Fancy> = LazyLock::new(|| Fancy::new(r"(?<=[.!?])\s+").unwrap());
static ANSWER_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static CHAT_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9]+").unwrap());
static REPEAT_LEAD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[^:]{0,80}:\s*").unwrap());
static CLAUSE_BREAK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[,;]\s").unwrap());
static DO_NOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(?:do not|don'?t)\b").unwrap());

struct Carried {
    reply: String,
    note: MemoryNote,
    earlier_handback: Option<String>,
}

enum MemoryNote {
    Reply,
    Whole(String),
    Attempt(Attempt),
    Nothing,
}

impl Carried {
    fn hand_back(reply: String, memory: &Memory) -> Self {
        let low = reply.to_lowercase();
        let trimmed = data::CLAIMS_IT_WAS_SAID.iter().find_map(|head| {
            low.strip_prefix(head)?;
            let rest = reply
                .get(head.len()..)?
                .trim_start_matches([' ', ':', '-', ',', '—'])
                .trim();
            (!rest.is_empty() && !said_in_chat(rest, memory)).then(|| capitalized(rest))
        });
        Self {
            reply: trimmed.unwrap_or(reply),
            note: MemoryNote::Reply,
            earlier_handback: None,
        }
    }
}

fn seen(pattern: &Fancy, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn starts(pattern: &Fancy, text: &str) -> bool {
    pattern
        .find(text)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == 0)
}

fn content(text: &str) -> HashSet<String> {
    again::topic_words(text)
        .into_iter()
        .filter(|word| word.len() > 2 && !again::topic_stopword(word))
        .collect()
}

pub(in crate::bot) fn sentences(text: &str) -> impl Iterator<Item = &str> {
    SPLIT_SENTENCES
        .split(text)
        .filter_map(Result::ok)
        .map(str::trim)
        .filter(|part| !part.is_empty())
}

fn cap(text: &str, uppercase: bool) -> String {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let raised: String = if uppercase {
        first.to_uppercase().collect()
    } else {
        first.to_lowercase().collect()
    };
    raised + chars.as_str()
}

fn capitalized(text: &str) -> String {
    cap(text, true)
}

fn limited(text: &str) -> bool {
    let clean = text
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if [
        &*LIMIT_IN_A_CHAIR,
        &*LIMIT_AID,
        &*LIMIT_LIMIT_VERB,
        &*LIMIT_DISLIKES_THE_WALK,
        &*LIMIT_FLAT,
    ]
    .iter()
    .any(|pattern| seen(pattern, &clean))
    {
        return true;
    }
    let rule = LIMIT_RULE_LEAD
        .find(&clean)
        .ok()
        .flatten()
        .filter(|lead| lead.start() == 0)
        .map_or(clean.as_str(), |lead| &clean[lead.end()..]);
    LIMIT_NO_WALKS
        .find(rule)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == 0 && starts(&LIMIT_RULE_TAIL, &rule[found.end()..]))
}

fn word_in(words: &str, word: &str) -> bool {
    words.split_whitespace().any(|entry| entry == word)
}

fn named_places(text: &str) -> HashSet<String> {
    let mut places = HashSet::new();
    for found in PLACE_AFTER.captures_iter(text).filter_map(Result::ok) {
        if let Some(place) = found.get(1) {
            let place = place.as_str().to_lowercase();
            let first = place.split_whitespace().next().unwrap_or("");
            if !word_in(data::NOT_PLACES, first)
                && !word_in(data::NOT_PLACES, &place)
                && (!word_in(data::PLACE_MIDDLE, first) || place.contains([' ', '-']))
            {
                places.insert(place);
            }
        }
    }
    for found in PLACE_LOWER_NAMED.captures_iter(text).filter_map(Result::ok) {
        if let (Some(first), Some(last)) = (found.get(1), found.get(2)) {
            let last = last.as_str().to_lowercase();
            if !word_in(data::NOT_PLACES, &last) && !word_in(data::PLACE_VERBS, &last) {
                places.insert(format!("{} {last}", first.as_str().to_lowercase()));
            }
        }
    }
    for found in PLACE_LOWER.captures_iter(text).filter_map(Result::ok) {
        if let Some(place) = found.get(1) {
            let place = place.as_str().to_lowercase();
            if !word_in(data::NOT_PLACES, &place)
                && !word_in(data::PLACE_VERBS, &place)
                && !word_in(data::PLACE_MIDDLE, &place)
                && !places.iter().any(|known| {
                    known
                        .strip_prefix(&place)
                        .is_some_and(|rest| rest.starts_with(' '))
                })
            {
                places.insert(place);
            }
        }
    }
    places
}

fn earlier_destinations(text: &str, memory: &Memory) -> HashSet<String> {
    let first_trip = |plan: &str| {
        if !seen(&PLAN, plan) {
            return None;
        }
        let places = named_places(plan);
        (!places.is_empty()).then_some(places)
    };
    let active = memory
        .turns
        .iter()
        .rev()
        .filter(|turn| !turn.user.is_empty())
        .take(2)
        .find_map(|turn| first_trip(&turn.user))
        .or_else(|| {
            std::iter::once(text)
                .chain(memory.turns.iter().rev().map(|turn| turn.user.as_str()))
                .find(|line| seen(&PLAN, line))
                .and_then(first_trip)
        });
    let Some(active) = active else {
        return HashSet::new();
    };
    memory
        .turns
        .iter()
        .map(|turn| turn.user.as_str())
        .chain(std::iter::once(text))
        .flat_map(named_places)
        .filter(|place| !active.contains(place))
        .collect()
}

fn recent_answers<'a>(
    text: &str,
    memory: &'a Memory,
) -> impl Iterator<Item = (usize, &'a Exchange)> {
    let abandoned = earlier_destinations(text, memory);
    memory
        .turns
        .iter()
        .rev()
        .take(data::LOOK_BACK)
        .enumerate()
        .filter_map(move |(index, turn)| {
            if !data::ANSWER_MODULES.contains(&turn.module) {
                return None;
            }
            let said = turn.bot.trim();
            let length = said.chars().count();
            (length >= 50
                && (turn.module == "answers" || length >= data::A_STORE_ANSWER)
                && !seen(&HEDGE, said)
                && !said.ends_with('?')
                && !starts(&QUESTION, said)
                && (abandoned.is_empty()
                    || !again::topic_words(said)
                        .into_iter()
                        .any(|word| abandoned.contains(&word))))
            .then_some((index + 1, turn))
        })
}

fn same_subject(earlier: &Exchange, subject: Option<&str>, memory: &Memory) -> bool {
    if memory
        .turns
        .last()
        .is_some_and(|turn| seen(&HEDGE, &turn.bot))
    {
        return true;
    }
    let subject = subject.map_or_else(HashSet::new, content);
    if subject.is_empty() {
        return true;
    }
    let asked = content(&earlier.user);
    let answer = content(&earlier.bot);
    if !asked.is_empty() && answer.is_disjoint(&asked) {
        return false;
    }
    if !asked.is_disjoint(&subject) || !answer.is_disjoint(&subject) {
        return true;
    }
    memory.turns.iter().rev().take(3).any(|turn| {
        seen(&ATTEMPT, &turn.user)
            && again::topic_words(&turn.user)
                .into_iter()
                .filter(|word| word.len() > 2 && answer.contains(word))
                .collect::<HashSet<_>>()
                .len()
                >= 2
    })
}

fn said_in_chat(part: &str, memory: &Memory) -> bool {
    let lower = part.to_lowercase().replace("...", " ");
    let opening = lower
        .split_whitespace()
        .take(10)
        .collect::<Vec<_>>()
        .join(" ");
    !opening.is_empty()
        && memory.turns.iter().any(|turn| {
            turn.bot
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .contains(&opening)
        })
}

fn already_repeated(part: &str, memory: &Memory, pending: Option<&str>) -> bool {
    let opening = part
        .trim()
        .chars()
        .take(60)
        .collect::<String>()
        .to_lowercase();
    let carried = memory.personal.carried_parts();
    let skip = usize::from(pending.is_some() && carried.len() >= HAND_BACK_LIMIT);
    !opening.is_empty()
        && memory
            .turns
            .iter()
            .map(|turn| turn.bot.as_str())
            .chain(carried.skip(skip))
            .chain(pending)
            .filter(|shown| shown.to_lowercase().contains(&opening))
            .take(2)
            .count()
            == 2
}

fn trailing_offset(text: &str, count: usize) -> usize {
    text.char_indices()
        .nth(count)
        .map_or(text.len(), |(index, _)| index)
}

fn whole_cut(text: &str) -> String {
    let whole = text.trim();
    let boundary = trailing_offset(whole, data::MOST);
    if boundary == whole.len() {
        return whole.into();
    }
    let before = &whole[..boundary];
    let mut cut = before
        .rfind(", ")
        .max(before.rfind("; "))
        .unwrap_or_default();
    let minimum = trailing_offset(whole, 80);
    if cut < minimum {
        cut = before.rfind(' ').unwrap_or_default();
    }
    let cut = cut.max(minimum);
    let kept = whole[..cut].trim_matches(CUT_EDGE);
    let dropped = whole[cut..].trim_matches(CUT_EDGE);
    let Some(hanging) = LAST_CLAUSE
        .captures(kept)
        .ok()
        .flatten()
        .and_then(|found| found.get(1))
    else {
        return kept.into();
    };
    if dropped.is_empty() || starts(&CONTINUES, dropped) {
        return kept.into();
    }
    let after = cut + trailing_offset(&whole[cut..], 2);
    let end = CLAUSE_BREAK
        .find(&whole[after..])
        .map_or(whole.len(), |found| after + found.start());
    let doing = &whole[cut..end];
    if doing.chars().count() <= data::MOST_AFTER && !doing.contains(['\'', '"']) {
        whole[..end].trim_matches(CUT_EDGE).into()
    } else {
        kept[..hanging.start()].trim_matches(CUT_EDGE).into()
    }
}

fn starts_action(sentence: &str) -> bool {
    let Some(first) = sentence
        .trim_matches([' ', '.', ',', ':', ';'])
        .split_whitespace()
        .next()
    else {
        return false;
    };
    let first = first.trim_matches(['.', ',', ';', ':']).to_lowercase();
    if data::IMPERATIVES.contains(&first.as_str()) || data::MOVE_VERBS.contains(&first.as_str()) {
        return true;
    }
    let Some(stem) = first.strip_suffix("ing").filter(|_| first.len() > 4) else {
        return false;
    };
    let doubled = stem
        .as_bytes()
        .windows(2)
        .last()
        .is_some_and(|pair| pair[0] == pair[1])
        && stem.len() > 2;
    data::IMPERATIVES.contains(&stem)
        || data::IMPERATIVES.contains(&format!("{stem}e").as_str())
        || (doubled && stem.is_ascii() && data::IMPERATIVES.contains(&&stem[..stem.len() - 1]))
}

fn doing(sentence: &str) -> String {
    let sentence = sentence.trim();
    let body = sentence
        .split_once(':')
        .filter(|(head, rest)| {
            !starts_action(head)
                && head.split_whitespace().count() <= 12
                && rest.trim().chars().count() >= 20
        })
        .map_or_else(|| sentence.to_owned(), |(_, rest)| capitalized(rest.trim()));
    let mut whole = whole_cut(&body);
    let last = whole.trim_end_matches(['\'', '"', '’', '”']);
    if !whole.is_empty() && !last.ends_with(['.', '!', '?']) {
        whole.push('.');
    }
    whole
}

fn acceptable_sentence(part: &str) -> bool {
    !starts(&RED_FLAG, part) && !seen(&ASKING_THEM, part)
}

fn advice(answer: &str) -> Option<&str> {
    let mut best = None;
    let mut most = 1;
    for sentence in sentences(answer).filter(|part| acceptable_sentence(part)) {
        let count = sentence
            .to_lowercase()
            .split_whitespace()
            .filter(|word| {
                data::IMPERATIVES.contains(&word.trim_matches([' ', ',', ';', ':', '.']))
            })
            .count()
            + usize::from(DO_NOT.is_match(sentence));
        if count > most {
            best = Some(sentence);
            most = count;
        }
    }
    best
}

fn ago_forms(word: &str) -> Vec<String> {
    let mut forms = Vec::new();
    if let Some(stem) = word.strip_suffix("ied").filter(|_| word.len() > 4) {
        forms.push(format!("{stem}y"));
    }
    if let Some(stem) = word.strip_suffix("ed") {
        forms.push(stem.into());
        forms.push(format!("{stem}e"));
        let doubled = stem
            .as_bytes()
            .windows(2)
            .last()
            .is_some_and(|pair| pair[0] == pair[1]);
        if stem.len() > 2 && doubled && stem.is_ascii() {
            forms.push(stem[..stem.len() - 1].to_owned());
        }
    } else if let Some(stem) = word.strip_suffix('d').filter(|_| word.len() > 3) {
        forms.push(stem.into());
    }
    forms
}

fn action(word: &str) -> Option<String> {
    if let Ok(index) = data::IRREGULAR_PAST.binary_search_by_key(&word, |(past, _)| *past) {
        let base = data::IRREGULAR_PAST[index].1;
        return (base == "do" || data::IMPERATIVES.contains(&base)).then(|| base.into());
    }
    ago_forms(word)
        .into_iter()
        .find(|form| data::IMPERATIVES.contains(&form.as_str()))
}

fn object_of(tail: &str) -> Option<HashSet<String>> {
    let tail = tail.to_lowercase().replace(',', " ");
    let mut named = HashSet::new();
    for word in tail
        .split_whitespace()
        .map(|word| word.trim_matches(OBJECT_EDGE))
    {
        if word.is_empty() || data::OBJECT_END.contains(&word) {
            break;
        }
        if data::STANDS_IN.contains(&word) {
            return None;
        }
        if word.len() > 2 && !again::topic_stopword(word) {
            named.insert(word.to_owned());
        }
    }
    (!named.is_empty()).then_some(named)
}

fn step_was_done(line: &str, answer: &str) -> bool {
    let given = content(answer);
    let said = answer.to_lowercase().replace('\'', "");
    DONE.captures_iter(line)
        .filter_map(Result::ok)
        .any(|found| {
            let Some(full) = found.get(0) else {
                return false;
            };
            if line[..full.start()]
                .to_lowercase()
                .split_whitespace()
                .rev()
                .take(3)
                .any(|word| {
                    data::NO_LONGER_TO_DO
                        .contains(&word.trim_matches([' ', ',', '.', ';', ':', '!', '?', '\'']))
                })
            {
                return false;
            }
            let Some(act) = found
                .name("verb")
                .and_then(|found| action(&found.as_str().to_lowercase()))
            else {
                return false;
            };
            if act != "do"
                && !said.split_whitespace().any(|word| {
                    let word = word.trim_matches(OBJECT_EDGE);
                    word == act || ago_forms(word).contains(&act)
                })
            {
                return false;
            }
            object_of(&line[full.end()..]).map_or(act != "do", |named| !named.is_disjoint(&given))
        })
}

pub(in crate::bot) fn later_part(line: &str, answer: &str) -> Option<String> {
    let parts: Vec<_> = sentences(answer).collect();
    let tried = content(line);
    let shared = |part: &str| {
        content(part)
            .into_iter()
            .filter(|word| {
                tried.iter().any(|said| {
                    word == said
                        || (word.len() >= 5 && said.len() >= 5 && word.get(..5) == said.get(..5))
                })
            })
            .count()
    };
    let mut best = None;
    let mut most = 0;
    for (index, part) in parts.iter().enumerate() {
        let count = shared(part);
        if count > most {
            most = count;
            best = Some(index);
        }
    }
    if let Some(index) = best {
        let mut later = parts[index + 1..]
            .iter()
            .filter(|part| acceptable_sentence(part) && shared(part) < most)
            .map(|part| doing(part))
            .filter(|part| part.chars().count() >= 25);
        let first = later.next()?;
        return if starts_action(&first) {
            Some(first)
        } else {
            later.find(|part| starts_action(part)).or(Some(first))
        };
    }
    parts
        .iter()
        .skip(1)
        .rev()
        .filter(|part| acceptable_sentence(part))
        .map(|part| doing(part))
        .find(|part| part.chars().count() >= 25)
}

fn own_subject_missing(line: &str, said: &str) -> bool {
    let line = line.to_lowercase();
    let said = said.to_lowercase();
    let words: HashSet<_> = ANSWER_WORD
        .find_iter(&said)
        .map(|word| word.as_str().trim_end_matches('s'))
        .collect();
    let mut named = OWNED
        .captures_iter(&line)
        .filter_map(Result::ok)
        .filter_map(|found| found.get(1).map(|word| word.as_str().trim_end_matches('s')))
        .peekable();
    named.peek().is_some() && named.all(|word| !words.contains(word))
}

fn names_feeling(line: &str) -> bool {
    seen(&BROODING, line)
        || again::topic_words(line)
            .into_iter()
            .any(|word| data::NEGATIVE_CUES.contains(&word.as_str()))
}

fn their_agreement(line: &str, memory: &Memory) -> Option<Carried> {
    let words: Vec<_> = line
        .split_whitespace()
        .map(|word| word.trim_matches(WORD_EDGE))
        .collect();
    let said_yes = words
        .iter()
        .any(|word| data::AGREES_ON_ITS_OWN.contains(&word.to_lowercase().as_str()))
        || seen(&AGREES_IN_A_PHRASE, line)
        || (words.len() <= 8
            && words
                .last()
                .is_some_and(|word| data::AGREEMENT_TAIL.contains(&word.to_lowercase().as_str())));
    if !said_yes {
        return None;
    }
    let mut parts: Vec<_> = words
        .into_iter()
        .filter(|word| !word.is_empty() && !data::AGREEING.contains(&word.to_lowercase().as_str()))
        .collect();
    let is_opening = |word: &str| {
        let word = word.to_lowercase();
        data::BE_FORM.contains(&word.as_str()) || data::POINTING_AT.contains(&word.as_str())
    };
    while parts.len() > 1 && is_opening(parts[0]) {
        parts.remove(0);
    }
    let first = parts.first()?;
    if parts.len() > 4
        || is_opening(first)
        || data::OPENERS.contains(&first.to_lowercase().as_str())
    {
        return None;
    }
    let name = parts.join(" ");
    let upper = capitalized(&name);
    let lower = cap(&name, false);
    let options = [
        format!("{upper}, yes."),
        format!("Yes — {lower}."),
        format!("{upper} it is."),
        format!("That's it: {lower}."),
        format!("{upper}, that's the main thing."),
    ];
    Some(Carried {
        reply: compose::pick(&options.each_ref().map(String::as_str), memory, &name).into(),
        note: MemoryNote::Nothing,
        earlier_handback: None,
    })
}

fn carried_on(
    text: &str,
    routine: bool,
    subject: Option<&str>,
    memory: &Memory,
    pending: Option<&str>,
) -> Option<Carried> {
    if !memory
        .turns
        .iter()
        .rev()
        .take(data::LOOK_BACK)
        .any(|turn| data::ANSWER_MODULES.contains(&turn.module))
    {
        return None;
    }
    let mut clean = text.trim();
    if let Some(lead) = RESET_LEAD
        .find(clean)
        .ok()
        .flatten()
        .filter(|lead| lead.start() == 0)
    {
        let after = &clean[lead.end()..];
        if starts(&QUESTION, after) || starts(&ASKS, after) {
            clean = after;
        }
    }
    if clean.is_empty()
        || clean.ends_with('?')
        || starts(&QUESTION, clean)
        || starts(&ASKS, clean)
        || clean.split_whitespace().count() > 20
        || seen(&ABOUT_THE_BOT, clean)
        || seen(&WORDS_WANTED, clean)
        || seen(&ASKING_TO_START, clean)
        || starts(&ACK_OPENS, clean)
        || limited(clean)
    {
        return None;
    }
    let mut recent = recent_answers(clean, memory).peekable();
    if !seen(&QUESTION_MORE, clean)
        && ((seen(&TRIED, clean) && !seen(&NOT_TRIED, clean))
            || recent
                .peek()
                .is_some_and(|(back, turn)| *back == 1 && step_was_done(clean, &turn.bot)))
    {
        let (back, turn) = recent.next()?;
        if back != 1 {
            return None;
        }
        let part = later_part(clean, &turn.bot)?;
        return (!already_repeated(&part, memory, pending))
            .then(|| Carried::hand_back(part, memory));
    }
    if routine && !seen(&REJOINDER, clean) {
        return None;
    }
    let asked = content(clean);
    if asked.len() < 2
        || (memory
            .turns
            .last()
            .is_some_and(|turn| seen(&REFER_BACK, &turn.bot))
            && !again::asks_for_move(text))
    {
        return None;
    }
    for (back, turn) in recent {
        if back > 1 && !same_subject(turn, subject, memory) {
            continue;
        }
        let mut best = None;
        let mut shared = 0;
        for part in sentences(&turn.bot).filter(|part| acceptable_sentence(part)) {
            let count = asked.intersection(&content(part)).count();
            if count > shared {
                shared = count;
                best = Some(part);
            }
        }
        if shared < 2 {
            best = if back == 1
                && seen(&TROUBLE, clean)
                && !names_feeling(clean)
                && !own_subject_missing(clean, &turn.bot)
                && !seen(&SOMEONE_ELSES_LIMIT, clean)
            {
                advice(&turn.bot)
            } else {
                None
            };
        } else if shared >= asked.len()
            && let Some(agreement) = their_agreement(clean, memory)
        {
            return Some(agreement);
        }
        if let Some(best) = best {
            let part = doing(best);
            if part.chars().count() >= 25 && !already_repeated(&part, memory, pending) {
                return Some(Carried::hand_back(part, memory));
            }
        }
    }
    None
}

fn chat_words(text: &str) -> String {
    let lower = text.to_lowercase();
    CHAT_WORD
        .find_iter(&lower)
        .map(|word| word.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn repeats_the_chat_answer(reply: &str, text: &str, memory: &Memory) -> bool {
    if text.split_whitespace().count() > 5 {
        return false;
    }
    let flat = chat_words(reply);
    if flat.split_whitespace().count() < 12
        || data::MEANT_LEADS
            .iter()
            .any(|lead| reply.to_lowercase().starts_with(lead))
    {
        return false;
    }
    let after_lead = REPEAT_LEAD
        .find(reply)
        .map(|lead| chat_words(&reply[lead.end()..]))
        .filter(|body| body.split_whitespace().count() >= 12);
    let readings = [Some(flat), after_lead];
    memory.turns.iter().rev().take(4).any(|turn| {
        let before = chat_words(&turn.bot);
        readings.iter().flatten().any(|reading| {
            before == *reading
                || (before.split_whitespace().count() > reading.split_whitespace().count()
                    && before.starts_with(reading))
        })
    })
}

fn after_safety(memory: &Memory) -> bool {
    memory
        .turns
        .last()
        .is_some_and(|turn| matches!(turn.module, "safety" | "urgent"))
}

fn answer_step(text: &str, memory: &Memory) -> Option<Carried> {
    let clean = text.to_lowercase().replace('’', "'");
    let clean = clean.split_whitespace().collect::<Vec<_>>().join(" ");
    let step = (!after_safety(memory))
        .then(|| returned::reply(&clean, memory).or_else(|| tried::reply(&clean, memory)))
        .flatten();
    step.or_else(|| {
        Some(Carried {
            reply: remedy::reply(&clean, memory)
                .or_else(|| tried::acted(&clean, memory))
                .or_else(|| step_after::reply(&clean, memory))?,
            note: MemoryNote::Nothing,
            earlier_handback: None,
        })
    })
}

fn answer(reading: &Reading<'_>, memory: &Memory) -> Option<Carried> {
    if after_safety(memory) {
        return None;
    }
    let routine = reading.context_fields.routine;
    let subject = reading.context_fields.subject.as_deref();
    let raw = reading.raw;
    let typed = raw
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let mut retelling = [
        Some(reading.line.plain.as_str()),
        (reading.line.norm != reading.line.words).then_some(reading.line.norm_text.as_str()),
    ]
    .into_iter()
    .flatten()
    .filter(|line| {
        !line.is_empty() && line.trim() != typed && line.split_whitespace().count() <= 8
    });
    let raw_offer = carried_on(raw, routine, subject, memory, None);
    if raw_offer
        .as_ref()
        .is_some_and(|part| !repeats_the_chat_answer(&part.reply, raw, memory))
    {
        return raw_offer;
    }
    // Spelling retries run within the same provisional chat offer as the raw reading.
    let pending = raw_offer.as_ref().and_then(|part| match part.note {
        MemoryNote::Reply => Some(part.reply.as_str()),
        _ => None,
    });
    if let Some(reply) =
        fix_asked::method_doubt(reading, memory).or_else(|| fix_asked::reply(reading, memory))
    {
        return Some(Carried {
            reply,
            note: MemoryNote::Nothing,
            earlier_handback: pending.map(str::to_owned),
        });
    }
    let mut repaired = retelling.find_map(|line| {
        answer_step(line, memory).or_else(|| carried_on(line, routine, subject, memory, pending))
    })?;
    repaired.earlier_handback = pending.map(str::to_owned);
    Some(repaired)
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    answer(reading, memory).map(|part| part.reply)
}

pub(super) fn respond_step(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    answer_step(reading.raw, memory).map(|part| part.reply)
}

pub(super) fn first_step(reply: &str, text: &str, memory: &Memory) -> Option<String> {
    step_back::hand_back(text, reply, memory)
}

pub(super) fn record(reading: &Reading<'_>, selected: &str, memory: &mut Memory) {
    if let Some(part) = answer_step(reading.raw, memory).or_else(|| answer(reading, memory))
        && (part.reply == selected
            || frame_reply(&part.reply, reading.raw, memory).as_deref() == Some(selected))
    {
        if let Some(earlier) = part.earlier_handback {
            memory.personal.remember_carried(earlier);
        }
        match part.note {
            MemoryNote::Reply => memory.personal.remember_carried(part.reply),
            MemoryNote::Whole(full) => memory.personal.remember_carried(full),
            MemoryNote::Attempt(walk) => memory.personal.remember_attempt(walk),
            MemoryNote::Nothing => (),
        }
    }
}
