//! Reply decoration and repeat counting from PyBot's memory/again.py.
#[path = "again_data.rs"]
mod data;
#[cfg(test)]
#[path = "again_tests.rs"]
mod tests;

use super::super::measurement;
use crate::bot::{Exchange, Memory, compose::fallback, understanding};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

const SHORT: usize = 14;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Fancy> =
            LazyLock::new(|| Fancy::new(data::$name).unwrap());)+
    };
}

patterns!(
    A_DETAIL,
    ANOTHER_ONE,
    A_MONEY_ASK,
    MONEY_SAID,
    AN_AMOUNT,
    A_REPORT,
    AN_ASK,
    A_STILL_GOING_ON,
    A_SCRIPT,
    SAID_BACK,
    RULED_OUT,
    FEELING,
    CANNOT,
    BEING,
    NOTHING,
    CLAUSE_ONLY,
    OPENING_FILLER,
    WORDING_ASKS,
    WORDING_SOMEONE,
    WORDING_SAYS,
    RETURNED_RECURRENCE,
    RETURNED_REMIND_ME,
    RETURNED_AN_ASK,
    RETURNED_ASKING,
    RETURNED_RETURNING,
    RETURNED_AGAIN,
    RETURNED_RECURRING,
    RETURNED_SELF_EVIDENT,
    START_ASK,
    STARTING_WHAT,
    STARTING_OUT,
    HOW_IT,
    HELP_SHAPE,
    HELP_ASK_QUESTION_FIRST,
    NAMED_AFTER_IT,
    MORE_ASK,
    IDEAS_ONLY,
    NAMED_CAUSE,
    RETURN_LEAD,
    CASUAL_LEAD,
    NAME_END,
    OPENS_ON_A_FIGURE,
    CARD_DONENESS,
    CARD_JOINED_ON,
    START_AT_A_LOSS,
);

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9]+(?:'[a-z]+)?").unwrap());
static SLOT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\{[a-z_]+\}").unwrap());
static WH_WITH_S: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(what|how|when|where|why|who|which)s\b").unwrap());
static CARD_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9']+").unwrap());
static CARD_FIGURE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+(?:\.\d+)?").unwrap());
static CARD_NARROW: LazyLock<Vec<(&str, Fancy)>> = LazyLock::new(|| {
    data::CARD_NARROW
        .iter()
        .map(|(kind, regex)| (*kind, Fancy::new(regex).unwrap()))
        .collect()
});
static FRAME_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::FRAMES
        .iter()
        .filter_map(|frame| frame.split_once("{reply}"))
        .map(|(head, _)| {
            let head = head.trim_end();
            let separator = head.strip_suffix([':', '—', '–', '-']).map(str::trim_end);
            let chunks: Vec<_> = SLOT
                .split(separator.unwrap_or(head))
                .map(regex::escape)
                .collect();
            let tail = if separator.is_some() {
                r"\s*(?::|—|–|-)\s*"
            } else {
                ""
            };
            Regex::new(&format!("^{}{tail}", chunks.join(".+?"))).unwrap()
        })
        .collect()
});
static LEAD_WORDS: LazyLock<Vec<String>> = LazyLock::new(|| {
    data::FRAMES
        .iter()
        .map(|frame| flat(frame.split_once('{').map_or(frame, |(head, _)| head)))
        .filter(|head| head.contains(' '))
        .collect()
});

fn matches(pattern: &Fancy, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

pub(in crate::bot::memory) fn words(text: &str) -> Vec<String> {
    let text = text.to_lowercase().replace('’', "'");
    WORD.find_iter(&text)
        .map(|found| {
            let word = found.as_str();
            word.strip_suffix("'s").unwrap_or(word).replace('\'', "")
        })
        .filter(|word| !word.is_empty())
        .collect()
}

pub(in crate::bot::memory) fn topic_stopword(word: &str) -> bool {
    data::STOPWORDS.binary_search(&word).is_ok()
}

fn flat(text: &str) -> String {
    words(text).join(" ")
}

pub(in crate::bot::memory) fn unframed(reply: &str) -> Option<&str> {
    FRAME_PATTERNS.iter().find_map(|pattern| {
        let found = pattern.find(reply)?;
        let body = reply[found.end()..].trim();
        (!body.is_empty()).then_some(body)
    })
}

fn carries_lead(mine: &str) -> bool {
    LEAD_WORDS.iter().any(|lead| {
        mine.strip_prefix(lead)
            .is_some_and(|rest| rest.starts_with(' '))
    })
}

fn starts_with_words(text: &str, prefix: &str) -> bool {
    text.strip_prefix(prefix)
        .is_some_and(|rest| rest.starts_with(' '))
}

pub(in crate::bot::memory) fn already_had(reply: &str, memory: &Memory) -> Option<String> {
    let mine = flat(reply);
    if mine.split_whitespace().count() <= SHORT || carries_lead(&mine) {
        return None;
    }
    memory
        .turns
        .iter()
        .any(|turn| {
            let said = flat(&turn.bot);
            said == mine || starts_with_words(&said, &mine)
        })
        .then(|| data::AGAIN[0].replace("{reply}", reply.trim()))
}

fn times_on_screen(mine: &str, memory: &Memory) -> usize {
    memory
        .turns
        .iter()
        .rev()
        .take_while(|turn| {
            flat(&turn.bot) == mine
                || unframed(&turn.bot).is_some_and(|body| {
                    let body = flat(body);
                    !body.is_empty() && mine.starts_with(&body)
                })
        })
        .count()
}

fn recent_match(mine: &str, memory: &Memory) -> bool {
    memory
        .turns
        .iter()
        .rev()
        .map(|turn| flat(&turn.bot))
        .filter(|body| !body.is_empty())
        .take(3)
        .any(|body| body == mine)
}

fn earlier_match<'a>(mine: &str, memory: &'a Memory) -> Option<(&'a Exchange, String)> {
    memory.turns.iter().find_map(|turn| {
        let said = flat(&turn.bot);
        (said == mine || starts_with_words(&said, mine)).then_some((turn, said))
    })
}

fn returned_report(text: &str) -> bool {
    let low = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    if low.is_empty() || matches(&RETURNED_REMIND_ME, &low) || matches(&RETURNED_AN_ASK, &low) {
        return false;
    }
    reads_returning_clean(&low) || matches(&RETURNED_RECURRENCE, &low)
}

fn reads_returning_clean(low: &str) -> bool {
    !low.is_empty()
        && low.split_whitespace().count() <= 16
        && !matches(&RETURNED_ASKING, low)
        && (matches(&RETURNED_RETURNING, low)
            || (matches(&RETURNED_AGAIN, low)
                && (matches(&RETURNED_RECURRING, low) || matches(&RETURNED_SELF_EVIDENT, low))))
}

pub(in crate::bot::memory) fn reads_as_returning(text: &str) -> bool {
    let low = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    reads_returning_clean(&low)
}

fn word_in(words: &str, word: &str) -> bool {
    words.split_whitespace().any(|known| known == word)
}

fn a_content_word(word: &str) -> bool {
    word.trim_end_matches('s').len() > 2
        && data::STOPWORDS.binary_search(&word).is_err()
        && data::ASK_ONLY.binary_search(&word).is_err()
}

fn stems(text: &str) -> HashSet<String> {
    words(text)
        .into_iter()
        .filter(|word| a_content_word(word))
        .map(|word| word.trim_end_matches('s').to_owned())
        .collect()
}

fn same_question(mine: &str, text: &str, memory: &Memory) -> Option<String> {
    let asked = stems(text);
    if asked.len() < 2 {
        return None;
    }
    for turn in &memory.turns {
        let said = flat(&turn.bot);
        if said != mine && !starts_with_words(&said, mine) {
            continue;
        }
        let earlier = stems(&turn.user);
        if earlier.len() < 2
            || asked.intersection(&earlier).count() * 4 < asked.union(&earlier).count() * 3
        {
            continue;
        }
        let head = RULED_OUT
            .find(text)
            .ok()
            .flatten()
            .map_or(text, |stop| &text[..stop.start()]);
        let mut best = Vec::new();
        let mut run = Vec::new();
        for word in words(head) {
            if earlier.contains(word.trim_end_matches('s')) {
                if !run.contains(&word) {
                    run.push(word);
                    if run.len() > best.len() {
                        best.clone_from(&run);
                    }
                }
            } else if a_content_word(&word) {
                run.clear();
            }
        }
        return (best.len() > 1).then(|| best.join(" "));
    }
    None
}

pub(in crate::bot::memory) fn return_target(text: &str) -> Option<(String, bool)> {
    let low = flat(text);
    let low = WH_WITH_S.replace_all(&low, "${1} is");
    let rest = if let Some(found) = RETURN_LEAD.find(&low).ok().flatten() {
        &low[found.end()..]
    } else {
        let found = CASUAL_LEAD.find(&low).ok().flatten()?;
        let rest = &low[found.end()..];
        let first = rest.split_whitespace().next()?;
        if !word_in(data::CASUAL_OPEN, first) {
            return None;
        }
        rest
    };
    let mut parts = rest.split_whitespace().peekable();
    let mut target = Vec::new();
    while let Some(word) = parts.next() {
        if word_in(data::RETURN_ASKS, word) {
            if word_in(data::HELPER_ASKS, word)
                && !word_in(data::MODALS, word)
                && let Some(after) = parts.peek()
                && (word_in(data::VERB_PHRASE_AFTER, after)
                    || word_in(data::ING_VERBS, after)
                    || after.ends_with("ed"))
            {
                return None;
            }
            break;
        }
        target.push(word);
    }
    while target.last().is_some_and(|word| {
        word_in(data::CASUAL_OPEN, word) || data::STOPWORDS.binary_search(word).is_ok()
    }) {
        target.pop();
    }
    if target.is_empty()
        || target.len() > 3
        || target
            .iter()
            .any(|word| word_in(data::CASUAL_VERB, word) || word_in(data::RETURN_TIME, word))
    {
        return None;
    }
    let target = target.join(" ");
    let rest = rest.strip_prefix(&target).unwrap_or(rest);
    let asks = rest
        .split_whitespace()
        .any(|word| word_in(data::RETURN_ASKS, word));
    Some((target, asks))
}

fn title_first(word: &str) -> String {
    let mut chars = word.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn as_name(thing: &str) -> String {
    thing
        .split_whitespace()
        .map(|word| {
            if data::PLACES.binary_search(&word).is_ok() {
                title_first(word)
            } else {
                word.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn without_subject_lead(reply: &str, topic: &str) -> String {
    let reply = reply.trim();
    let escaped = regex::escape(topic);
    let prefix = Regex::new(&format!(
        r"(?i)^(?:(?:in|at|on|for|about|with|around)\s+{escaped}\s*[:,]?\s+|{escaped}\s*[:,]\s+)"
    ))
    .unwrap();
    prefix
        .find(reply)
        .and_then(|found| {
            let rest = &reply[found.end()..];
            (!rest.is_empty()).then(|| title_first(rest))
        })
        .unwrap_or_else(|| reply.to_owned())
}

fn return_frame(topic: &str, reply: &str, memory: &Memory) -> String {
    data::RETURN[memory.turns.len() % data::RETURN.len()]
        .replace("{detail}", topic)
        .replace("{reply}", reply)
}

fn topic_on_screen(memory: &Memory) -> Option<String> {
    memory.turns.iter().rev().take(2).find_map(|turn| {
        let (topic, _) = return_target(&turn.user)?;
        Some(
            topic
                .strip_prefix("my ")
                .map_or_else(|| topic.clone(), |mine| format!("your {mine}")),
        )
    })
}

fn adds_nothing(topic: &str, text: &str, earlier: &str) -> bool {
    flat(text).split_whitespace().all(|word| {
        word.len() <= 2
            || !word.chars().all(char::is_alphabetic)
            || data::FRAME_WORDS.binary_search(&word).is_ok()
            || data::CORE_WORDS.binary_search(&word).is_ok()
            || topic.split_whitespace().any(|one| one == word)
            || !earlier.split_whitespace().any(|one| one == word)
    })
}

pub(crate) fn starting_out_names_something(text: &str) -> bool {
    let Some(out) = STARTING_OUT.captures(text).ok().flatten() else {
        return false;
    };
    let Some(first) = out.get(1) else {
        return false;
    };
    let first = first.as_str();
    if data::STARTING_INTO.contains(&first) || data::STARTING_AT.contains(&first) {
        out.get(2)
            .is_some_and(|second| !data::STARTING_BACK.contains(&second.as_str()))
    } else {
        !data::STARTING_BACK.contains(&first)
    }
}

fn ask_for_start(text: &str) -> bool {
    if !matches(&START_ASK, text) || starting_out_names_something(text) {
        return false;
    }
    let Some(found) = STARTING_WHAT.captures(text).ok().flatten() else {
        let at_a_loss = matches(&START_AT_A_LOSS, text);
        return words(text)
            .into_iter()
            .filter(|word| {
                let word = word.as_str();
                word.len() > 2
                    && !word.bytes().all(|byte| byte.is_ascii_digit())
                    && !topic_stopword(word)
                    && data::START_WORDS.binary_search(&word).is_err()
                    && !(at_a_loss && data::START_LOSS_WORDS.binary_search(&word).is_ok())
            })
            .count()
            < 2;
    };
    let Some(first) = found.get(1).map(|value| value.as_str()) else {
        return false;
    };
    if data::STARTING_AT.contains(&first) {
        return found
            .get(2)
            .is_none_or(|second| data::STARTING_BACK.contains(&second.as_str()));
    }
    data::STARTING_BACK.contains(&first)
}

pub(in crate::bot::memory) fn asks_for_move(text: &str) -> bool {
    if ask_for_start(text) || matches(&HOW_IT, text) {
        return true;
    }
    if !matches(&HELP_SHAPE, text) {
        return false;
    }
    if matches(&HELP_ASK_QUESTION_FIRST, text) && !matches(&NAMED_AFTER_IT, text) {
        return true;
    }
    if matches(&MORE_ASK, text) || matches(&IDEAS_ONLY, text) {
        return false;
    }
    understanding::kinds::read(&understanding::line::normalise(text))
        .1
        .help_ask
}

fn money_missing(text: &str, reply: &str) -> bool {
    matches(&A_MONEY_ASK, text) && matches(&MONEY_SAID, text) && !matches(&AN_AMOUNT, reply)
}

fn quote_open(text: &str) -> bool {
    !text.matches('"').count().is_multiple_of(2)
        || text.matches('“').count() > text.matches('”').count()
}

fn card_sentences(reply: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for part in measurement::sentences(reply) {
        if let Some(previous) = result.last_mut()
            && (quote_open(previous)
                || (matches(&NAME_END, previous) && matches(&OPENS_ON_A_FIGURE, part)))
        {
            previous.push(' ');
            previous.push_str(part);
        } else {
            result.push(part.to_owned());
        }
    }
    result
}

fn named_cause(part: &str) -> bool {
    part.split_whitespace().count() >= 6
        && NAMED_CAUSE
            .find(part)
            .ok()
            .flatten()
            .is_some_and(|found| found.start() <= 2)
}

fn cause_said_again(reply: &str, memory: &Memory) -> Option<String> {
    if memory.intent_wants != Some("reason") {
        return None;
    }
    let lead = reply
        .split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let recent = memory
        .turns
        .iter()
        .rev()
        .filter(|turn| !turn.bot.is_empty())
        .take(3);
    let in_hand = (lead.len() >= 25)
        .then(|| {
            recent
                .clone()
                .find(|turn| turn.bot.to_lowercase().contains(&lead))
                .map(|turn| turn.bot.as_str())
        })
        .flatten();
    let behind = recent.clone().find(|turn| {
        card_sentences(&turn.bot)
            .iter()
            .any(|part| named_cause(part))
    });
    for candidate in [Some(reply), in_hand, behind.map(|turn| turn.bot.as_str())]
        .into_iter()
        .flatten()
    {
        let parts = card_sentences(candidate);
        for (index, part) in parts.iter().enumerate() {
            if !named_cause(part) {
                continue;
            }
            let low = part.to_lowercase();
            let shown = memory
                .turns
                .iter()
                .any(|turn| turn.bot.to_lowercase().starts_with(&low));
            if shown {
                return (parts.len() > 1).then(|| parts[1..].join(" "));
            }
            if index == 0 {
                return None;
            }
            return Some(format!("{part} {}", parts[0]));
        }
    }
    None
}

fn short(reply: &str, memory: &Memory, short_first: bool) -> String {
    if let Some(cause) = cause_said_again(reply, memory) {
        return cause;
    }
    let parts = card_sentences(reply);
    let Some(first) = parts.first() else {
        return reply.to_owned();
    };
    let threshold = if short_first { 5 } else { SHORT };
    let keeps = if first.split_whitespace().count() >= threshold || parts.len() == 1 {
        1
    } else {
        2
    };
    let answer = parts[..keeps].join(" ");
    if quote_open(&answer) {
        reply.trim().to_owned()
    } else {
        answer
    }
}

fn recast(word: &str) -> Option<&'static str> {
    Some(match word {
        "i" | "me" | "we" | "us" => "you",
        "my" | "our" => "your",
        "mine" => "yours",
        "myself" => "yourself",
        "am" => "are",
        "i'm" | "im" => "you're",
        "i've" => "you've",
        "i'll" => "you'll",
        "i'd" => "you'd",
        "was" => "were",
        _ => return None,
    })
}

fn for_the_person(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let bare = word
                .trim_matches(|ch: char| ch.is_ascii_punctuation())
                .to_lowercase();
            let Some(turned) = recast(&bare) else {
                return word.to_owned();
            };
            let end = word
                .trim_end_matches(|ch: char| ch.is_ascii_punctuation())
                .len();
            let turned = if word.starts_with(char::is_uppercase) {
                title_first(turned)
            } else {
                turned.to_owned()
            };
            format!("{turned}{}", &word[end..])
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn phrase(text: &str) -> Option<(bool, String)> {
    let said = text.trim().trim_end_matches(['.', '!', ',']);
    let line = OPENING_FILLER.replace(said, "");
    let line = line.trim();
    if line.is_empty() || line.contains('?') || line.split_whitespace().count() > 8 {
        return None;
    }
    if let Some(found) = NOTHING.captures(line).ok().flatten() {
        let verb = found.get(1)?.as_str().to_lowercase();
        return (verb.ends_with('s') && !matches!(verb.as_str(), "is" | "was" | "has"))
            .then(|| (false, format!("Nothing {verb} you")));
    }
    for (pattern, lead) in [(&*FEELING, "Feeling "), (&*CANNOT, "Not being able to ")] {
        if let Some(found) = pattern.captures(line).ok().flatten() {
            let said = found.get(1)?.as_str();
            let rest = for_the_person(said);
            if rest.is_empty()
                || rest.split_whitespace().count() > 7
                || rest.contains([',', ';'])
                || said.split_whitespace().any(|word| {
                    recast(
                        &word
                            .trim_matches(|ch: char| ch.is_ascii_punctuation())
                            .to_lowercase(),
                    )
                    .is_some()
                })
                || rest
                    .split_whitespace()
                    .next()
                    .is_some_and(|word| matches!(word, "to" | "like"))
            {
                return None;
            }
            return Some((true, format!("{lead}{rest}")));
        }
    }
    if matches(&CLAUSE_ONLY, line) {
        return Some((false, title_first(&for_the_person(line))));
    }
    let found = BEING.captures(line).ok().flatten()?;
    let rest = for_the_person(found.get(1)?.as_str());
    let words: Vec<_> = rest.split_whitespace().collect();
    let last = words.last()?;
    if rest.contains(',')
        || words.len() > 3
        || last.to_lowercase().ends_with("ing")
        || matches!(
            words[0].to_lowercase().as_str(),
            "going" | "gonna" | "about" | "not"
        )
        || words.iter().any(|word| {
            matches!(
                word.to_lowercase().as_str(),
                "to" | "the" | "a" | "an" | "my" | "your"
            )
        })
    {
        return None;
    }
    Some((true, format!("Being {rest}")))
}

fn underneath(text: &str, reply: &str, times: usize, memory: &Memory) -> Option<String> {
    if times < 2
        || reply.trim().is_empty()
        || carries_lead(&flat(reply))
        || unframed(reply).is_some()
    {
        return None;
    }
    static CLAUSE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"[,;]|\band\b|\bbut\b|\bso\b").unwrap());
    let (said, detail) = phrase(text).or_else(|| {
        CLAUSE
            .split(text)
            .filter(|part| !part.trim().is_empty())
            .skip(1)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .find_map(phrase)
    })?;
    let pool = if said {
        data::UNDERNEATH_SAID
    } else {
        data::UNDERNEATH_CLAUSE
    };
    Some(
        pool[memory.turns.len() % pool.len()]
            .replace("{said}", &detail)
            .replace("{reply}", reply.trim()),
    )
}

fn alternate_hedge(mine: &str, memory: &Memory) -> Option<String> {
    let pool = fallback::unknown_replies();
    let which = pool.iter().position(|line| flat(line) == mine)?;
    let mut other = pool
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != which)
        .map(|(_, reply)| *reply);
    other
        .nth(memory.turns.len() % (pool.len() - 1))
        .map(str::to_owned)
}

fn framed_as_repeat(
    reply: &str,
    text: &str,
    mine: &str,
    times: usize,
    memory: &Memory,
) -> Option<String> {
    if returned_report(text) {
        return None;
    }
    if mine.split_whitespace().count() <= SHORT {
        let lead = data::SHORT_AGAIN[times.min(data::SHORT_AGAIN.len()) - 1];
        return Some(lead.replace("{reply}", reply.trim()));
    }
    let report =
        matches(&A_REPORT, text) && !matches(&AN_ASK, text) && !matches(&A_STILL_GOING_ON, text);
    let wording = matches(&WORDING_ASKS, text)
        || matches(&WORDING_SOMEONE, text)
        || matches(&WORDING_SAYS, text);
    if report
        || (wording && !matches(&A_SCRIPT, reply))
        || asks_for_move(text)
        || money_missing(text, reply)
    {
        return None;
    }
    let short = short(reply, memory, false);
    if let Some(made) = underneath(text, &short, times, memory) {
        return Some(made);
    }
    let short_flat = flat(&short);
    let read_the_short = short_flat != mine && times_on_screen(&short_flat, memory) > 0;
    let asks_for_detail = matches(&A_DETAIL, text) || matches(&ANOTHER_ONE, text);
    let earlier_ask = memory
        .turns
        .iter()
        .rev()
        .take(3)
        .find(|turn| flat(&turn.bot) == mine)
        .map_or("", |turn| &turn.user);
    let body = if read_the_short || (asks_for_detail && !matches(&A_DETAIL, earlier_ask)) {
        reply
    } else {
        &short
    };
    if let Some(alternate) = alternate_hedge(mine, memory) {
        return Some(alternate);
    }
    let lead = data::AGAIN[times.min(data::AGAIN.len()) - 1];
    let lead = if body.contains(':') {
        lead.replace(": {reply}", " — {reply}")
    } else {
        lead.to_owned()
    };
    Some(lead.replace("{reply}", body))
}

fn repeat_reply(reply: &str, text: &str, memory: &Memory, only_short: bool) -> Option<String> {
    if memory.turns.is_empty() {
        return None;
    }
    let mine = flat(reply);
    let short = mine.split_whitespace().count() <= SHORT;
    if (only_short && (!short || unframed(reply).is_some()))
        || mine.is_empty()
        || carries_lead(&mine)
        || (short && matches(&SAID_BACK, reply.trim()))
    {
        return None;
    }
    let times = times_on_screen(&mine, memory);
    if times > 0 {
        return framed_as_repeat(reply, text, &mine, times, memory);
    }
    let last = memory.turns.last()?.bot.trim();
    if recent_match(&mine, memory) || (last.ends_with('?') && starts_with_words(&flat(last), &mine))
    {
        return framed_as_repeat(reply, text, &mine, 1, memory);
    }
    if short {
        return None;
    }
    let (earlier, said) = earlier_match(&mine, memory)?;
    if let Some((topic, asks)) = return_target(text)
        && (flat(&earlier.user).contains(&topic) || said.contains(&topic))
    {
        if !asks || !adds_nothing(&topic, text, &said) {
            return None;
        }
        let topic = as_name(&topic);
        return Some(return_frame(
            &topic,
            &without_subject_lead(reply, &topic),
            memory,
        ));
    }
    let shared = same_question(&mine, text, memory)?;
    let topic = topic_on_screen(memory).unwrap_or(shared);
    Some(return_frame(
        &as_name(&topic),
        &self::short(reply, memory, true),
        memory,
    ))
}

pub(in crate::bot::memory) fn instead(reply: &str, text: &str, memory: &Memory) -> Option<String> {
    repeat_reply(reply, text, memory, false)
}

pub(in crate::bot::memory) fn frame_short(reply: &str, memory: &Memory) -> Option<String> {
    repeat_reply(reply, "", memory, true)
}

fn card_flat(text: &str) -> String {
    let lower = text.to_lowercase();
    CARD_WORD
        .find_iter(&lower)
        .map(|item| item.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn card_opening(text: &str) -> String {
    card_flat(text)
        .split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ")
}

fn has_changed_figures(reply: &str, seen: &str) -> bool {
    let mine: HashSet<_> = CARD_FIGURE
        .find_iter(reply)
        .map(|item| item.as_str())
        .collect();
    let theirs: HashSet<_> = CARD_FIGURE
        .find_iter(seen)
        .map(|item| item.as_str())
        .collect();
    !theirs.is_empty() && !mine.is_subset(&theirs)
}

fn same_card(reply: &str, mine: &str, seen: &str) -> bool {
    let theirs = card_flat(seen);
    if theirs.is_empty() {
        return false;
    }
    if theirs.contains(&card_opening(mine))
        || card_sentences(reply).iter().any(|sentence| {
            let part = card_flat(sentence);
            part.len() > 20 && theirs.contains(&part)
        })
    {
        return !has_changed_figures(reply, seen);
    }
    seen.split(':').any(|clause| {
        let cut = card_flat(clause);
        cut.split_whitespace().count() >= 6 && mine.contains(&cut)
    })
}

fn card_body(reply: &str, text: &str) -> String {
    let parts = card_sentences(reply);
    let Some(first) = parts.first() else {
        return reply.to_owned();
    };
    let clean = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let asks_part = CARD_NARROW
        .iter()
        .find(|(_, pattern)| matches(pattern, &clean))
        .is_some_and(|(kind, _)| *kind != "date" || !matches(&CARD_DONENESS, &clean));
    if parts.len() > 1 && asks_part {
        let needed = card_flat(text);
        let needed: HashSet<_> = needed
            .split_whitespace()
            .filter(|word| word.len() > 2 && data::CARD_CONTENT_STOP.binary_search(word).is_err())
            .collect();
        let known = |part: &str| {
            let flat = card_flat(part);
            let shared: HashSet<_> = flat
                .split_whitespace()
                .filter(|word| needed.contains(word))
                .collect();
            shared.len()
        };
        if let Some((_, part, score)) = parts
            .iter()
            .enumerate()
            .map(|(index, part)| (index, part, known(part)))
            .max_by_key(|(index, part, score)| {
                (
                    *score,
                    std::cmp::Reverse(part.chars().count()),
                    std::cmp::Reverse(*index),
                )
            })
            && score > 0
        {
            let part = part.trim();
            return CARD_JOINED_ON
                .find(part)
                .ok()
                .flatten()
                .map_or_else(|| part.to_owned(), |head| title_first(&part[head.end()..]));
        }
    }
    if first.chars().count() >= 60 || parts.len() == 1 {
        first.to_owned()
    } else {
        format!("{first} {}", parts[1])
    }
}

pub(in crate::bot::memory) fn frame_answer_card(
    reply: &str,
    text: &str,
    memory: &Memory,
) -> Option<String> {
    if memory.turns.is_empty() {
        return None;
    }
    let mine = card_flat(reply);
    if mine.split_whitespace().count() <= SHORT
        || unframed(reply).is_some()
        || returned_report(text)
    {
        return None;
    }
    let wanted = card_flat(text);
    let times = memory
        .turns
        .iter()
        .rev()
        .take_while(|turn| card_flat(&turn.user) == wanted && same_card(reply, &mine, &turn.bot))
        .count();
    if times == 0 || money_missing(text, reply) {
        return None;
    }
    let opening = card_opening(reply);
    if !memory
        .turns
        .iter()
        .rev()
        .take(3)
        .any(|turn| card_opening(&turn.bot) == opening)
    {
        return None;
    }
    if asks_for_move(text) {
        return super::super::first_step(reply, text, memory);
    }
    if times >= 3 {
        return Some(data::CARD_ASK_AGAIN_HELP.replace("{reply}", reply));
    }
    Some(data::CARD_LEADS[times - 1].replace("{reply}", &card_body(reply, text)))
}
