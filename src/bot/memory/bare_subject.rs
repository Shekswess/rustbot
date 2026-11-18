//! Look up a short question using the conversation's preceding subject.
#[path = "bare_subject_data.rs"]
mod data;
#[cfg(test)]
#[path = "bare_subject_tests.rs"]
mod tests;

use super::{again, frame_reply};
use crate::bot::{
    Exchange, Memory, Reading,
    compose::fallback,
    knowledge::lookup::{self, KnowledgeMatch},
    modules::answers::Answer,
    understanding::kinds::{Intent, Kind},
};
use fancy_regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    BARE_ASK,
    TIME_WORD,
    ASKS_A_QUANTITY,
    CORRECTION_LEAD,
    PHRASAL_RE,
    RELATION,
    FRAME_ASK,
    WITHOUT_CLAUSE,
    LEAD,
    LEAD_QUESTION,
    WISH_RE,
    BUYING_A_STATE,
    CONSIDERING,
    HANDOFF_THROAT,
    BARE_VERDICT,
);
static WORD: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"[a-z0-9']+").unwrap());
static TIME_SPLIT: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"\b(?:for|tonight|today|tomorrow|yesterday|this|next)\b").unwrap()
});
static NUMBER: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^\d+(?:s|st|nd|rd|th)?$").unwrap());
static ANOTHER_SUBJECT: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?i)^\s*(?:never ?mind|nvm|new topic|different (?:topic|question)|(?:change|changing) (?:the )?subject)\b")
        .unwrap()
});
static RESIDENCE_RECALL: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)^(?:(?:and|so)[ ,]+)?where (?:does (?:she|he)|do they) live(?: now)?[?.!]*$",
    )
    .unwrap()
});
static PAIRED_RESIDENCES: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?i)\blives? in\b[^.?!]{1,90}\band i live in\b").unwrap());
static UNFINISHED_PLAN: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"^i was going to [^.?!]+ but i (?:haven'?t|have not) done (?P<reference>it) yet[.!]*$",
    )
    .unwrap()
});
const EDGE: &[char] = &[' ', ',', '.', ';', ':', '!', '?', '-'];
const MAX_SUBJECT: usize = 4;

fn seen(expression: &Regex, line: &str) -> bool {
    expression.is_match(line).unwrap_or(false)
}

fn flat(text: &str) -> String {
    text.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn shorten(word: &str) -> String {
    if let Some(kept) = word.strip_suffix("ies").filter(|_| word.len() > 4) {
        return format!("{kept}y");
    }
    if let Some(kept) = word
        .strip_suffix("es")
        .filter(|kept| word.len() > 3 && kept.ends_with(['s', 'x', 'z', 'h']))
    {
        return kept.to_owned();
    }
    if word.len() > 3 && !word.ends_with("ss") {
        return word.strip_suffix('s').unwrap_or(word).to_owned();
    }
    word.to_owned()
}

fn keys(text: &str) -> Vec<String> {
    let mut used = HashSet::new();
    again::topic_words(text)
        .into_iter()
        .filter(|word| {
            word.len() >= 3
                && !again::topic_stopword(word)
                && !word.bytes().all(|c| c.is_ascii_digit())
        })
        .map(|word| shorten(&word))
        .filter(|word| used.insert(word.clone()))
        .collect()
}

fn strip_recurring(expression: &Regex, mut source: String) -> String {
    while let std::borrow::Cow::Owned(stripped) = expression.replace(&source, "") {
        if stripped.trim().is_empty() || stripped == source {
            break;
        }
        source = stripped;
    }
    source
}

fn retained(source: &str) -> Vec<String> {
    let mut words = keys(source);
    words.retain(|word| !data::ASK_ONLY.contains(&word.as_str()));
    while words
        .last()
        .is_some_and(|word| data::TIME_WORDS.contains(&word.as_str()))
    {
        words.pop();
    }
    words
}

fn only_intro(line: &str) -> bool {
    matches!(
        HANDOFF_THROAT.replace(line, "").trim_matches(EDGE),
        "one more thing" | "quick question" | "another thing" | "a quick question"
    )
}

fn gist(line: &str) -> String {
    let low = flat(line);
    let corrected = CORRECTION_LEAD.replace(&low, "");
    let switched = data::SWITCHERS
        .iter()
        .find_map(|prefix| corrected.strip_prefix(prefix))
        .map_or(corrected.as_ref(), |tail| tail.trim_start_matches(EDGE));
    let unphrasal = PHRASAL_RE.replace_all(switched, " ");
    let whole = RELATION
        .split(&unphrasal)
        .find_map(Result::ok)
        .unwrap_or_default();
    let mut source = whole.into();
    if seen(&FRAME_ASK, whole) {
        let trimmed = WITHOUT_CLAUSE.replace(whole, "");
        let trimmed = trimmed.trim_matches(EDGE);
        if !trimmed.is_empty() && !keys(trimmed).is_empty() {
            source = trimmed.to_owned();
        }
    }
    source = strip_recurring(&CONSIDERING, source);
    let buying = seen(&BUYING_A_STATE, &source);
    let mut wished = false;
    if !buying {
        for _ in 0..3 {
            let stripped = WISH_RE.replace(&source, "");
            if stripped == source || stripped.trim_matches(EDGE).is_empty() {
                break;
            }
            source = stripped.into_owned();
            wished = true;
        }
    }
    let before_split = source;
    let mut segments = TIME_SPLIT.split(&before_split);
    let first = segments.next().unwrap_or_default();
    let first = if !wished && !buying {
        strip_recurring(
            &LEAD_QUESTION,
            LEAD.replace(first.trim_matches(EDGE), "").into_owned(),
        )
    } else {
        first.to_owned()
    };
    let first = strip_recurring(&CONSIDERING, first);
    let clause = first
        .split_once(',')
        .filter(|(head, tail)| !tail.trim().is_empty() && only_intro(head))
        .map_or_else(
            || first.split([',', ';']).next().unwrap_or(""),
            |(_, tail)| tail,
        );
    let mut kept = retained(clause);
    if !kept.is_empty()
        && kept
            .iter()
            .all(|word| data::A_FIGURE.contains(&word.as_str()))
        && let Some(later) = segments.map(retained).find(|later| {
            !later.is_empty()
                && later
                    .iter()
                    .any(|word| !data::A_FIGURE.contains(&word.as_str()))
        })
    {
        kept = later;
    }
    if kept.is_empty() && wished {
        kept = [before_split.as_str(), whole]
            .into_iter()
            .map(retained)
            .find(|words| !words.is_empty())
            .unwrap_or_default();
    }
    kept.truncate(MAX_SUBJECT);
    kept.join(" ")
}

fn known(word: &str, heard: &HashSet<String>) -> bool {
    heard.contains(word)
        || word
            .strip_suffix('s')
            .is_some_and(|base| heard.contains(base))
        || heard.iter().any(|said| said.starts_with(word))
}

fn describes(word: &str) -> bool {
    data::NO_THING_NAMED.contains(&word)
        || data::ADJECTIVE_ENDINGS
            .iter()
            .any(|ending| word.ends_with(ending) && word.len() > ending.len() + 2)
}

fn names_its_own_thing(ask: &str, memory: &Memory) -> bool {
    if seen(&ASKS_A_QUANTITY, ask) {
        return false;
    }
    let heard: HashSet<_> = memory
        .turns
        .iter()
        .flat_map(|turn| [&turn.user, &turn.bot])
        .flat_map(|line| {
            WORD.find_iter(&line.to_lowercase())
                .map(|word| word.as_str().to_owned())
                .collect::<Vec<_>>()
        })
        .collect();
    let words: Vec<_> = WORD.find_iter(ask).map(|word| word.as_str()).collect();
    words.iter().enumerate().any(|(i, &word)| {
        if NUMBER.is_match(word)
            || describes(word)
            || known(word, &heard)
            || [data::DESCRIBES_ONE, data::VERB_FORMS, data::AUXILIARY]
                .iter()
                .any(|set| set.contains(&word))
            || word
                .strip_suffix('e')
                .is_some_and(|root| data::DESCRIBES_ONE.contains(&format!("{root}ing").as_str()))
            || words
                .iter()
                .skip(i + 1)
                .take(2)
                .any(|word| matches!(*word, "one" | "ones"))
        {
            return false;
        }
        let before = &words[i.saturating_sub(2)..i];
        match before {
            [place, determiner]
                if data::THING_AFTER.contains(determiner)
                    && data::AFTER_A_PLACE.contains(place) =>
            {
                false
            }
            [.., determiner] if data::THING_AFTER.contains(determiner) => true,
            [determiner, adjective]
                if data::THING_AFTER.contains(determiner)
                    && !data::THING_AFTER.contains(adjective)
                    && !data::AUXILIARY.contains(adjective)
                    && !describes(adjective) =>
            {
                true
            }
            _ => false,
        }
    })
}

fn subject(memory: &Memory) -> Option<(&Exchange, String)> {
    for turn in memory.turns.iter().rev() {
        if matches!(turn.module, "safety" | "urgent") {
            return None;
        }
        let earlier = Reading::new(&turn.user);
        let changed = earlier.line.change_of_subject || ANOTHER_SUBJECT.is_match(&turn.user);
        let thing = gist(&turn.user);
        let empty = thing.is_empty() || seen(&TIME_WORD, &thing);
        if changed {
            return (!empty).then_some((turn, thing));
        }
        if empty
            || seen(&BARE_ASK, &turn.user)
            || matches!(earlier.kind, Kind::Empty | Kind::Greeting | Kind::Thanks)
            || earlier
                .topics
                .iter()
                .any(|(topic, _)| *topic == "about-the-bot")
            || fallback::unknown_replies().contains(&turn.bot.as_str())
            || (earlier.context_fields.subject.is_none()
                && thing.split_whitespace().count() == 1
                && !matches!(turn.module, "answers" | "chat"))
        {
            continue;
        }
        if earlier.context_fields.subject.is_none()
            && thing.split_whitespace().count() == 1
            && thing
                .split_whitespace()
                .all(|word| data::VERBISH.contains(&word) || word.ends_with("ed"))
        {
            continue;
        }
        return Some((turn, thing));
    }
    None
}

fn singular(word: &str) -> &str {
    let head = word.trim_end_matches('s');
    if head.is_empty() { word } else { head }
}

fn verb(word: &str) -> bool {
    data::VERBISH.contains(&word)
        || (word.len() > 4 && (word.ends_with("ing") || word.ends_with("ed")))
}

fn about(card: &KnowledgeMatch, thing: &str) -> bool {
    let keys: HashSet<_> = card
        .keys()
        .iter()
        .flat_map(|group| group.iter())
        .flat_map(|phrase| phrase.split_whitespace())
        .map(singular)
        .collect();
    let mut named: Vec<_> = thing
        .split_whitespace()
        .filter(|word| !data::DESCRIBING.contains(word))
        .collect();
    if named.is_empty() {
        named.extend(thing.split_whitespace());
    }
    let Some(head) = named.last().map(|word| singular(word)) else {
        return false;
    };
    keys.contains(head)
        && (!verb(head)
            || named
                .iter()
                .any(|word| !verb(word) && keys.contains(singular(word))))
}

fn opening(reply: &str) -> String {
    let said = BARE_VERDICT.replace(reply.trim(), "");
    said.to_lowercase()
        .split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn answer_recently_said(reply: &str, memory: &Memory) -> bool {
    let first = opening(reply);
    !first.is_empty()
        && memory
            .turns
            .iter()
            .rev()
            .take(3)
            .any(|turn| opening(&turn.bot) == first)
}

/// The pinned answers reader lets an immediately recalled pair of residences
/// supply the subject of a short unfinished plan with no named entity.
pub(crate) fn unfinished_plan_after_residence_quote(
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<String> {
    if reading.kind != Kind::Statement
        || reading.line.change_of_subject
        || reading.context_fields.subject.is_some()
        || reading
            .line
            .words
            .iter()
            .take(12)
            .any(|word| matches!(word.as_str(), "my" | "our" | "his" | "her" | "their"))
        || reading.raw.split_whitespace().count() > 14
        || super::history::in_a_safety_thread(memory)
    {
        return None;
    }
    let last = memory.turns.last()?;
    if !matches!(last.module, "said_fact_back" | "said_back")
        || !RESIDENCE_RECALL.is_match(last.user.trim())
    {
        return None;
    }
    let line = &reading.clean;
    let reference = UNFINISHED_PLAN.captures(line)?.name("reference")?;
    let quoted = last.bot.split_once('"')?.1.split_once('"')?.0;
    if !PAIRED_RESIDENCES.is_match(quoted) || names_its_own_thing(line, memory) {
        return None;
    }
    let record = memory
        .turns
        .iter()
        .rev()
        .skip(1)
        .take_while(|turn| {
            !matches!(turn.module, "safety" | "urgent") && !ANOTHER_SUBJECT.is_match(&turn.user)
        })
        .find(|turn| turn.user.trim() == quoted)?;
    let subject = gist(&record.user);
    if subject.is_empty()
        || subject
            .split_whitespace()
            .any(|word| line.split_whitespace().any(|typed| typed == word))
    {
        return None;
    }
    Some(format!(
        "{}the {subject}{}",
        &line[..reference.start()],
        &line[reference.end()..]
    ))
}

fn corrected_word(text: &str) -> Option<&str> {
    let opener = CORRECTION_LEAD.find(text).ok().flatten();
    let after = opener.map_or(text, |matched| &text[matched.end()..]);
    let named = after.strip_prefix("the word ")?;
    let named = named.strip_prefix("is ").unwrap_or(named);
    Some(named.strip_prefix("actually ").unwrap_or(named))
}

pub(crate) fn named_request(reading: &Reading<'_>, memory: &Memory) -> Option<Answer> {
    let [request] = reading.line.words.as_slice() else {
        return None;
    };
    if reading.kind != Kind::Statement
        || reading.features.intent != Intent::Request
        || reading.own_topic.is_some()
        || reading.line.change_of_subject
        || !reading.reference.followup
        || data::ASKS_FOR_AN_ANSWER.contains(&request.as_str())
    {
        return None;
    }
    let (at, anchor) = memory.ack_topics.active(&memory.turns)?;
    let original = memory.turns.get(at)?;
    let main = flat(&original.user);
    let term = corrected_word(&main);
    let held = if matches!(original.module, "safety" | "urgent") {
        // The explicit word correction has a subject even when the safety
        // reply owned that turn. The preceding topic gives the correction
        // its context.
        term?;
        let preceding = at.checked_sub(1).and_then(|i| memory.turns.get(i))?;
        if matches!(preceding.module, "safety" | "urgent") {
            return None;
        }
        gist(&preceding.user)
    } else {
        if term.is_some_and(|term| term.split_whitespace().count() != 1) {
            return None;
        }
        gist(term.unwrap_or(anchor))
    };
    if held.split_whitespace().count() != 1 || held == *request || seen(&TIME_WORD, &held) {
        return None;
    }
    let text = format!("what about {request} {held}");
    let widened = Reading::new(&text);
    let card = lookup::lookup_in_context(&widened, memory)?;
    if !about(&card, &held)
        || !card
            .keys()
            .iter()
            .skip(1)
            .any(|group| group.contains(&"what"))
        || answer_recently_said(&card.reply, memory)
    {
        return None;
    }
    let reply_words = card.reply.to_lowercase();
    let names_request = WORD.find_iter(&reply_words).any(|word| {
        let word = word.as_str();
        word.len() >= 4 && (word.starts_with(request) || request.starts_with(word))
    });
    // A rewritten reply may add a word the original never used. That should
    // not turn a late candidate into the reply to a one-word request.
    let names_request_as_written = names_request
        && lookup::by_id(card.domain, card.id).is_some_and(|entry| entry.reply == card.reply);
    Some(Answer {
        selected_card: Some((card.domain, card.id)),
        reply: card.reply.to_string(),
        outcomes: card.outcomes,
        score_carried_subject: names_request_as_written,
        entry: Some(card.reply),
    })
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<Answer> {
    let ask = flat(reading.raw);
    if reading.kind != Kind::Question
        || reading.line.change_of_subject
        || ANOTHER_SUBJECT.is_match(&ask)
        || reading.someone_new
        || !seen(&BARE_ASK, &ask)
        || names_its_own_thing(&ask, memory)
    {
        return None;
    }
    let (earlier, subject) = subject(memory)?;
    if subject.split_whitespace().any(|word| ask.contains(word)) {
        return None;
    }
    let text = format!("{} {subject}", ask.trim_end_matches(['?', '.', '!', ' ']));
    let widened = Reading::new(&text);
    let card = lookup::lookup_in_context(&widened, memory)?;
    let anchor = Reading::new(&earlier.user);
    let head = anchor.context_fields.subject.as_deref().unwrap_or(&subject);
    if !(about(&card, &subject) || about(&card, head))
        || anchor.topic.is_some_and(|(topic, _)| {
            !super::measurement_subject::domain_answers(topic, card.domain)
        })
    {
        return None;
    }
    if answer_recently_said(&card.reply, memory) {
        return None;
    }
    let base =
        lookup::apply_lead(&widened, &card, &card.reply).unwrap_or_else(|| card.reply.to_string());
    let reply = frame_reply(&base, reading.raw, memory)
        .or_else(|| again::already_had(&base, memory))
        .unwrap_or(base);
    Some(Answer {
        selected_card: Some((card.domain, card.id)),
        reply,
        outcomes: card.outcomes,
        score_carried_subject: false,
        entry: Some(card.reply),
    })
}
