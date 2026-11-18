//! Rewrite an underspecified measure question around the person's established subject.
#[path = "measurement_subject_data.rs"]
mod data;
#[cfg(test)]
#[path = "measurement_subject_tests.rs"]
mod tests;

use super::{again, frame_reply};
use crate::bot::{
    Exchange, Memory, Reading,
    knowledge::lookup::{self, KnowledgeMatch},
    modules::answers::{Answer, ordinary_offer},
    understanding::kinds::Kind,
};
use fancy_regex::Regex;
use std::sync::LazyLock;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    MEASURE_ASK,
    IN_PROGRESS,
    THE_LENGTH_FOR_IT,
    LENGTH_NOT,
    JOURNEY,
    PROGRESS_ASK,
    A_HOW_GOOD,
    A_GERUND,
    AND_AT_THE_START,
    A_PURPOSE,
    A_TIME_IN_A_REPLY,
    A_REPAIR,
    A_HOW_LONG,
    A_FIGURE,
    A_DOING_NAMED,
    A_PLAN_TO_THE_DOING,
    A_FIGURE_ON_THE_END,
);
static WORD: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"[a-z']+").unwrap());
static NAME_WORD: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"[a-z]+").unwrap());
static PRONOUN: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"\b(?:it|that|this|they|them|those|these|one|ones)\b").unwrap()
});
static PLAN_WORD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-z0-9']+").unwrap());
static DOUBLES: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[^aeiou][aeiou][bdgklmnprst]$").unwrap());

const EDGE: &[char] = &[' ', '.', '!', '?', ','];
const PUNCTUATION: &[char] = &['.', ',', ';', ':', '!', '?'];

fn seen(rule: &Regex, line: &str) -> bool {
    rule.is_match(line).unwrap_or(false)
}

fn starts(rule: &Regex, line: &str) -> bool {
    rule.find(line)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == 0)
}

fn compact(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn flat(text: &str) -> String {
    compact(&text.to_lowercase().replace('’', "'"))
}

fn nouns(text: &str) -> impl DoubleEndedIterator<Item = &str> {
    let words: Vec<_> = NAME_WORD
        .find_iter(text)
        .map(|word| word.as_str())
        .collect();
    words.into_iter().filter(|word| {
        word.len() > 2
            && !matches!(
                *word,
                "the" | "and" | "for" | "with" | "some" | "any" | "new"
            )
            && ![data::NOT_THE_THING, data::MEASURE_WORDS, data::A_FAULT]
                .iter()
                .any(|group| group.contains(word))
    })
}

fn subject_name(topic: &Reading<'_>) -> Option<String> {
    let owned = flat(topic.raw);
    let subject = topic.context_fields.subject.as_deref().unwrap_or(&owned);
    let words: Vec<_> = nouns(subject).collect();
    let inferred = words.last().copied();
    if seen(&IN_PROGRESS, &owned)
        || WORD
            .find_iter(&owned)
            .any(|word| data::A_FAULT.contains(&word.as_str()))
    {
        return nouns(&owned).next_back().or(inferred).map(str::to_owned);
    }
    if words.iter().all(|word| data::A_GETTING.contains(word))
        || inferred.is_some_and(|word| data::PLACE_WORDS.contains(&word))
    {
        nouns(&owned).next_back().or(inferred).map(str::to_owned)
    } else {
        inferred.map(str::to_owned)
    }
}

fn only_measure(line: &str) -> bool {
    WORD.find_iter(line).all(|word| {
        [
            data::MEASURE_WORDS,
            data::LIGHT_DOING,
            data::PLACE_PREPOSITIONS,
            data::PLACE_ARTICLES,
        ]
        .iter()
        .any(|group| group.contains(&word.as_str()))
    })
}

fn subject(memory: &Memory) -> Option<(&Exchange, String)> {
    for turn in memory.turns.iter().rev() {
        let low = flat(&turn.user);
        if low.is_empty() || only_measure(&low) {
            continue;
        }
        let reading = Reading::new(&turn.user);
        if matches!(reading.kind, Kind::Empty | Kind::Greeting | Kind::Thanks)
            || reading
                .topics
                .iter()
                .any(|(topic, _)| *topic == "about-the-bot")
            || (PRONOUN.is_match(low.split_whitespace().next().unwrap_or(""))
                && seen(&A_PURPOSE, &low))
            || low
                .split_whitespace()
                .next()
                .is_some_and(|word| matches!(word, "it" | "that" | "this"))
        {
            continue;
        }
        if let Some(name) = subject_name(&reading) {
            return Some((turn, name));
        }
    }
    None
}

fn newer_thing(last: &str, subject: &str) -> bool {
    let text = flat(last);
    if seen(&A_PURPOSE, &text) {
        return false;
    }
    text.split_whitespace().next_back().is_some_and(|tail| {
        let tail = tail.trim_matches(PUNCTUATION);
        tail.len() >= 4
            && ![
                data::VERBS,
                data::NOT_A_THING_END,
                data::POINTS_BACK,
                data::A_FAULT,
                data::ASPECT_NOUNS,
            ]
            .iter()
            .any(|group| group.contains(&tail))
            && !tail.contains(subject)
            && !subject.contains(tail)
    })
}

fn faulty(topic: &str, memory: &Memory) -> bool {
    let low = flat(topic);
    WORD.find_iter(&low)
        .any(|word| data::A_FAULT.contains(&word.as_str()))
        && memory
            .turns
            .last()
            .is_none_or(|turn| !seen(&A_TIME_IN_A_REPLY, &turn.bot))
}

fn last_person_line<'a>(ask: &str, memory: &'a Memory) -> Option<&'a str> {
    memory
        .turns
        .iter()
        .rev()
        .map(|turn| turn.user.as_str())
        .find(|line| !line.trim().is_empty() && flat(line) != ask)
}

fn length_about_doing(ask: &str, last: &str) -> Option<String> {
    if ask.split_whitespace().count() > 8
        || !starts(&THE_LENGTH_FOR_IT, ask)
        || seen(&LENGTH_NOT, ask)
        || !WORD.find_iter(ask).all(|word| {
            [
                data::MEASURE_WORDS,
                data::MEASURE_VERBS,
                data::POINTS_BACK,
                data::LENGTH_FILLERS,
                data::PLACE_WORDS,
                data::PLACE_PREPOSITIONS,
                data::PLACE_ARTICLES,
            ]
            .iter()
            .any(|group| group.contains(&word.as_str()))
        })
    {
        return None;
    }
    let said = compact(last);
    let captured = IN_PROGRESS.captures(&said).ok().flatten()?;
    let doing = captured
        .get(1)?
        .as_str()
        .trim_matches([' ', ',', ';', ':', '-']);
    (!doing.is_empty() && doing.split_whitespace().count() <= 7 && !starts(&JOURNEY, doing))
        .then(|| format!("how long does it take to {doing}"))
}

fn place_phrase(low: &[&str]) -> Option<String> {
    let end = low
        .iter()
        .position(|word| data::PLACE_WORDS.contains(word))?;
    let mut first = end;
    if end > 0 && data::PLACE_PAIRS.contains(&(low[end - 1], low[end])) {
        first -= 1;
    }
    while first > 0
        && (data::PLACE_PREPOSITIONS.contains(&low[first - 1])
            || data::PLACE_ARTICLES.contains(&low[first - 1]))
    {
        first -= 1;
    }
    Some(low[first..=end].join(" "))
}

fn doing_verb(line: &str) -> Option<&'static str> {
    line.split_whitespace().find_map(|word| {
        let word = word.trim_matches(PUNCTUATION);
        let at = data::DOING_BASE
            .binary_search_by_key(&word, |(source, _)| *source)
            .ok()?;
        Some(data::DOING_BASE[at].1)
    })
}

fn place_matches(reading: &Reading<'_>, card: &KnowledgeMatch) -> bool {
    let Some(place) = reading.context_fields.subject.as_deref() else {
        return true;
    };
    if !place
        .split_whitespace()
        .any(|word| data::PLACE_WORDS.contains(&word))
    {
        return true;
    }
    let Some((kind, _)) = Reading::new(place).topic else {
        return true;
    };
    domain_answers(kind, card.domain)
}

pub(super) fn domain_answers(topic: &str, domain: &str) -> bool {
    data::DOMAIN_TOPICS
        .iter()
        .find(|(known, _)| *known == domain)
        .is_some_and(|(_, topics)| topics.contains(&topic))
}

fn aspect(low: &[&str], subject: &str) -> Option<String> {
    let mut rest = low
        .iter()
        .filter(|word| !data::ASPECT_FILLERS.contains(word));
    let first = *rest.next()?;
    if rest.next().is_some() {
        return None;
    }
    data::ASPECT_ASKS.iter().find_map(|(names, template)| {
        (names.contains(&first)
            && (!template.starts_with("what do i feed") || data::FED_THINGS.contains(&subject)))
        .then(|| template.replace("{thing}", subject))
    })
}

fn progress_form(word: &str, topic: &str) -> Option<String> {
    let topic = flat(topic);
    let named = A_DOING_NAMED
        .captures_iter(&topic)
        .filter_map(Result::ok)
        .any(|found| {
            found
                .name("doing")
                .or_else(|| found.name("also"))
                .is_some_and(|said| {
                    said.as_str() == word
                        || said.as_str().strip_suffix("ing").is_some_and(|stem| {
                            stem.trim_end_matches('n') == word.trim_end_matches('n')
                        })
                })
        })
        || A_PLAN_TO_THE_DOING
            .captures_iter(&topic)
            .filter_map(Result::ok)
            .any(|found| {
                found
                    .name("action")
                    .is_some_and(|action| action.as_str() == word)
            });
    if !named {
        return None;
    }
    if word.ends_with("ing") {
        return Some(word.to_owned());
    }
    if let Some(base) = word.strip_suffix('e').filter(|base| {
        base.len() >= 3
            && base
                .as_bytes()
                .last()
                .is_some_and(|letter| !b"aeiou".contains(letter))
    }) {
        return Some(format!("{base}ing"));
    }
    if word.len() <= 5 && DOUBLES.is_match(word) {
        return Some(format!("{word}{}ing", word.chars().last()?));
    }
    Some(format!("{word}ing"))
}

fn measured_doing(word: &str, topic: &str) -> bool {
    let topic = flat(topic);
    let words: Vec<_> = PLAN_WORD
        .find_iter(&topic)
        .map(|word| word.as_str())
        .collect();
    words
        .windows(2)
        .any(|pair| pair[0] == word && starts(&A_FIGURE_ON_THE_END, pair[1]))
}

fn rewrite(
    ask: &str,
    latest: &str,
    previous: &str,
    topic: &str,
    subject: &str,
    faulty: bool,
) -> Option<String> {
    let asked = AND_AT_THE_START.replace(ask, "");
    let ask = asked.trim_matches(EDGE);
    if ask.is_empty() || ask.split_whitespace().count() > 12 {
        return None;
    }
    if let Some(length) = length_about_doing(ask, latest) {
        return Some(length);
    }
    let low: Vec<_> = ask
        .split_whitespace()
        .map(|word| word.trim_matches(PUNCTUATION))
        .collect();
    if let Some(named_aspect) = aspect(&low, subject) {
        return Some(named_aspect);
    }
    if !seen(&MEASURE_ASK, ask) {
        return None;
    }
    let place = place_phrase(&low);
    let points = low.iter().any(|word| data::POINTS_BACK.contains(word));
    if place.is_none() && !points && !only_measure(ask) {
        return None;
    }
    let progress = seen(&PROGRESS_ASK, ask);
    let asked = if faulty {
        low.iter()
            .take_while(|word| data::A_CONNECTIVE.contains(word))
            .count()
    } else {
        0
    };
    let low = &low[asked..];
    if !low.iter().enumerate().all(|(index, word)| {
        [
            data::MEASURE_WORDS,
            data::POINTS_BACK,
            data::PLACE_WORDS,
            data::PLACE_PREPOSITIONS,
            data::PLACE_ARTICLES,
        ]
        .iter()
        .any(|group| group.contains(word))
            || (!points && data::LIGHT_DOING.contains(word))
            || (faulty && data::MEASURE_VERBS.contains(word))
            || (progress && data::PROGRESS_WORDS.contains(word))
            || low
                .get(index + 1)
                .is_some_and(|next| data::PLACE_PAIRS.contains(&(*word, *next)))
    }) {
        return None;
    }
    if let Some(place) = place {
        let previous = flat(previous);
        let thing = WORD
            .find_iter(&previous)
            .map(|word| word.as_str())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .find(|word| {
                word.len() >= 3
                    && ![
                        data::MEASURE_WORDS,
                        data::PLACE_WORDS,
                        data::POINTS_BACK,
                        data::PLACE_PREPOSITIONS,
                    ]
                    .iter()
                    .any(|group| group.contains(word))
                    && doing_verb(word).is_none()
            })
            .unwrap_or(subject);
        let action = doing_verb(&previous)?;
        return (low.starts_with(&["how", "long"])
            && low.len() > 2
            && !data::PLACE_WORDS.contains(&thing)
            && (data::PLACE_PREPOSITIONS.contains(&low[2])
                || data::PLACE_ARTICLES.contains(&low[2])
                || data::PLACE_WORDS.contains(&low[2])))
        .then(|| format!("how long does it take to {action} {thing} {place}"));
    }
    let ask = if asked > 0 {
        low.join(" ")
    } else {
        ask.to_owned()
    };
    if !points {
        return Some(format!("{ask} {subject}"));
    }
    let doing = progress.then(|| progress_form(subject, topic)).flatten();
    if doing.is_some()
        && !seen(&A_HOW_GOOD, &ask)
        && measured_doing(subject, topic)
        && seen(&A_HOW_LONG, &ask)
    {
        return None;
    }
    let named = doing.unwrap_or_else(|| {
        if progress && starts(&A_GERUND, subject) {
            subject.to_owned()
        } else {
            format!("the {subject}")
        }
    });
    Some(PRONOUN.replacen(&ask, 1, named.as_str()).into_owned())
}

fn contains_subject(card: &KnowledgeMatch, subject: &str) -> bool {
    card.keys()
        .iter()
        .flat_map(|group| group.iter())
        .flat_map(|phrase| NAME_WORD.find_iter(phrase))
        .any(|word| word.as_str().starts_with(subject))
}

fn repair_entry(card: &KnowledgeMatch) -> bool {
    card.keys()
        .iter()
        .flat_map(|group| group.iter())
        .any(|phrase| seen(&A_REPAIR, phrase))
}

fn repair_questions(text: &str, topic: &str, subject: &str) -> Vec<String> {
    let low = flat(topic);
    let thing = NAME_WORD
        .find_iter(&low)
        .map(|word| word.as_str())
        .filter(|word| {
            ![data::A_FAULT, data::NOT_THE_THING, data::MEASURE_WORDS]
                .iter()
                .any(|group| group.contains(word))
                && !matches!(*word, "the" | "and" | "for" | "with")
        })
        .take(4)
        .collect::<Vec<_>>()
        .join(" ");
    if thing.is_empty() {
        return vec![format!("{text} repair")];
    }
    let named = format!("the {subject}");
    let replaced = text.replacen(&named, &format!("the {thing}"), 1);
    [
        replaced.replacen(&format!("the {thing}"), &format!("the {thing} repair"), 1),
        format!("{replaced} repair"),
        format!("how many days for a {thing} repair"),
    ]
    .into_iter()
    .collect()
}

fn short_length(answer: &str, ask: &str) -> bool {
    let words: Vec<_> = WORD.find_iter(ask).map(|word| word.as_str()).collect();
    words
        .iter()
        .any(|word| matches!(*word, "long" | "many" | "much"))
        && words.iter().all(|word| data::LENGTH_SAID.contains(word))
        && answer.split_whitespace().count() <= 12
        && seen(&A_FIGURE, answer)
}

pub(crate) fn respond(
    reading: &Reading<'_>,
    memory: &Memory,
    alone: Option<&str>,
) -> Option<Answer> {
    let ask = flat(reading.raw);
    if (!seen(&MEASURE_ASK, &ask)
        && !WORD
            .find_iter(&ask)
            .any(|word| data::ASPECT_NOUNS.contains(&word.as_str())))
        || memory.turns.is_empty()
        || memory
            .turns
            .last()
            .is_some_and(|turn| matches!(turn.module, "safety" | "urgent"))
    {
        return None;
    }
    let (topic, noun) = subject(memory)?;
    let latest = memory.turns.last()?.user.as_str();
    if newer_thing(latest, &noun) {
        return None;
    }
    let faulty = faulty(&topic.user, memory);
    let previous = last_person_line(&ask, memory).unwrap_or(latest);
    let written = rewrite(&ask, latest, previous, &topic.user, &noun, faulty)?;
    let faulty = faulty && seen(&A_HOW_LONG, &written);
    let read = |line: &str| lookup::lookup_in_context(&Reading::new(line), memory);
    let mut reached = if faulty {
        repair_questions(&written, &topic.user, &noun)
            .into_iter()
            .find_map(|attempt| {
                let card = read(&attempt)?;
                (contains_subject(&card, &noun)
                    && repair_entry(&card)
                    && alone.is_none_or(|alone| flat(alone) != flat(&card.reply)))
                .then_some((attempt, card))
            })
    } else {
        None
    };
    if reached.is_none() {
        reached = read(&written).map(|card| (written.clone(), card));
    }
    if faulty && reached.as_ref().is_none_or(|(_, card)| !repair_entry(card)) {
        return None;
    }
    if reached
        .as_ref()
        .is_none_or(|(_, card)| !contains_subject(card, &noun))
        && seen(&IN_PROGRESS, &topic.user)
    {
        let wider = format!("{written} {}", topic.user);
        reached = read(&wider).map(|card| (wider, card));
    }
    let (text, card) = reached?;
    if !contains_subject(&card, &noun) || !place_matches(reading, &card) {
        return None;
    }
    let repairs = faulty && repair_entry(&card);
    if !repairs
        && (lookup::lookup_in_context(reading, memory)
            .is_some_and(|only| contains_subject(&only, &noun))
            || alone.is_some_and(|alone| flat(alone).contains(&noun) || short_length(alone, &ask)))
    {
        return None;
    }
    let wide = Reading::new(&text);
    let mut offered = ordinary_offer(&wide, memory)?;
    if again::unframed(&offered.reply).is_some() {
        return None;
    }
    offered.score_carried_subject = true;
    if let Some(framed) = frame_reply(&offered.reply, reading.raw, memory) {
        offered.reply = framed;
        return Some(offered);
    }
    (!memory
        .turns
        .iter()
        .any(|turn| flat(&turn.bot) == flat(&offered.reply)))
    .then_some(offered)
}
