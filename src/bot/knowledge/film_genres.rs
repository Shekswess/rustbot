//! Genre shelves, exact running-time limits, and conversation-aware mood picks.

use crate::bot::Exchange;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use super::{
    ENTERTAINMENT_NEGATORS,
    entertainment::{limit, own_texts, plain_words, read_mood, within, word_turned_down},
    film_shelves::{contains_phrase, earlier_lines, line, rendered_picks, stands_down},
    guards::Context,
};

#[path = "film_genres_data.rs"]
mod data;
#[cfg(test)]
#[path = "film_genres_tests.rs"]
mod tests;

static SHELF_NAMES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::SHELVES
        .iter()
        .map(|shelf| {
            Regex::new(&format!(
                r"(?-u:\b){}(?:s|es)?(?-u:\b)",
                regex::escape(shelf.key)
            ))
            .expect("genre name")
        })
        .collect()
});
static A_VERDICT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:is|was|are|were|isnt|wasnt)\b").expect("genre verdict"));
static REFUSED_AFTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::REFUSED_AFTER).expect("genre refusal"));
static TITLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\*([^*]{2,60})\*").expect("emphasized title"));

const ASK_ABOUT: &[&str] = &[
    "one", "ones", "film", "films", "those", "these", "them", "pick", "of", "which", "what",
    "three", "any", "any of",
];

fn good_shelf(words: &str) -> Option<&'static data::Shelf> {
    data::SHELVES
        .iter()
        .zip(SHELF_NAMES.iter())
        .find_map(|(shelf, pattern)| {
            let found = pattern.find(words)?;
            let refused = words[..found.start()]
                .split_whitespace()
                .rev()
                .take(2)
                .any(|word| ENTERTAINMENT_NEGATORS.contains(&word));
            (!refused).then_some(shelf)
        })
}

fn names_other_subject(words: &str) -> bool {
    !contains_phrase(words, data::FILM_SAID) && contains_phrase(words, data::OTHER_SUBJECT)
}

fn refused_shelf(shelf: &data::Shelf, line: &str, earlier: &[Exchange]) -> bool {
    let rejects = |text| word_turned_down(text, |word| shelf.names.contains(&word), &REFUSED_AFTER);
    rejects(line)
        || earlier
            .iter()
            .rev()
            .find(|turn| {
                plain_words(&turn.user)
                    .split_whitespace()
                    .any(|word| shelf.names.contains(&word))
            })
            .is_some_and(|turn| rejects(&turn.user))
}

fn fits(minutes: u16, bound: u16, strict: bool) -> bool {
    minutes < bound || !strict && minutes == bound
}

fn first_upper(text: &str) -> String {
    let mut parts = text.chars();
    let first = parts.next().expect("nonempty authored shelf");
    format!("{}{}", first.to_uppercase(), parts.as_str())
}

fn shortest(shelf: &data::Shelf) -> (&'static str, u16, &'static str) {
    *shelf
        .films
        .iter()
        .min_by_key(|(_, minutes, _)| *minutes)
        .expect("populated shelf")
}

fn shelf_limit(shelf: &data::Shelf, bound: u16, strict: bool) -> Option<String> {
    let count = shelf
        .films
        .iter()
        .filter(|(_, minutes, _)| fits(*minutes, bound, strict))
        .count();
    if count == shelf.films.len() {
        return None;
    }
    let label = shelf.label;
    if count == 0 {
        let (film, minutes, _) = shortest(shelf);
        let when = if strict { "under" } else { "at" };
        return Some(format!(
            "Nothing on the {label} shelf comes in {when} {bound} minutes - the shortest is *{film}* at {minutes}. Say the number again if I have read it wrong, or name another kind."
        ));
    }
    let picks = rendered_picks(
        shelf
            .films
            .iter()
            .filter(|(_, minutes, _)| fits(*minutes, bound, strict))
            .map(|(title, minutes, reason)| format!("*{title}* ({minutes} minutes, {reason})")),
    );

    let head = first_upper(shelf.said_as);
    let when = if strict { "under" } else { "up to" };
    let rest = if shelf.films.len() - count == 1 {
        "one on that shelf runs"
    } else {
        "two on that shelf run"
    };
    Some(format!(
        "{picks}. {head} {when} {bound} minutes, and the other {rest} longer, so say if you would rather have one of those."
    ))
}

fn genre(
    raw: &str,
    plain: &str,
    words: &str,
    shelf: &data::Shelf,
    context: &Context<'_>,
) -> Option<String> {
    if super::entertainment::medium(plain)
        .is_some_and(|medium| medium != super::entertainment::Medium::Film)
        || A_VERDICT.is_match(plain) && contains_phrase(words, &[shelf.key])
        || refused_shelf(shelf, raw, context.earlier)
    {
        return None;
    }
    if contains_phrase(words, data::SHORTEST_WORDS) && contains_phrase(words, ASK_ABOUT) {
        let (film, minutes, why) = shortest(shelf);
        let mut said =
            format!("The shortest on that shelf is *{film}* at {minutes} minutes, {why}");
        if let Some((second, runtime, _)) = shelf.films.iter().find(|(title, ..)| *title != film) {
            use std::fmt::Write;
            write!(said, ". *{second}* runs {runtime} if you have the time.")
                .expect("string write");
        }
        return Some(said);
    }
    if let Some(said) = limit(raw).and_then(|(bound, strict)| shelf_limit(shelf, bound, strict)) {
        return Some(said);
    }
    let titles: Vec<_> = shelf
        .films
        .iter()
        .map(|(title, minutes, why)| format!("*{title}* ({minutes} minutes, {why})"))
        .collect();
    let (last, first) = titles.split_last()?;
    let (first, kind) = (first.join(", "), shelf.said_as);
    Some(format!(
        "For {kind}: {first}, or {last}. Any of them is a good way in, and I can go shorter or pick one if you tell me the mood."
    ))
}

fn has_medium(plain: &str, earlier: &[Exchange]) -> bool {
    super::entertainment::medium(plain).is_some()
        || earlier_lines(plain, earlier).any(|turn| super::entertainment::medium(turn).is_some())
}

fn minutes(
    raw: &str,
    plain: &str,
    words: &str,
    context: &Context<'_>,
) -> Option<Cow<'static, str>> {
    let (bound, strict) = limit(raw)?;
    if let Some(shelf) = good_shelf(words) {
        return genre(raw, plain, words, shelf, context).map(Cow::Owned);
    }
    if read_mood(raw).is_some() {
        return (!has_medium(plain, context.earlier))
            .then(|| mood_pick(context, false))
            .flatten();
    }
    if contains_phrase(words, data::FILM_GENRE_WORDS) {
        return None;
    }
    let carried =
        earlier_lines(plain, context.earlier).find_map(|turn| good_shelf(&plain_words(turn)));
    if let Some(reply) = carried.and_then(|shelf| shelf_limit(shelf, bound, strict)) {
        return Some(Cow::Owned(reply));
    }
    let when = if strict { "under" } else { "up to" };
    let rows = rendered_picks(
        data::FILM_RUNTIME
            .iter()
            .rev()
            .filter(|(_, minutes)| fits(*minutes, bound, strict))
            .take(3)
            .map(|(title, minutes)| format!("*{title}* ({minutes} minutes)")),
    );
    if rows.is_empty() {
        let when = if strict { "under" } else { "at" };
        return Some(Cow::Owned(format!(
            "Nothing I know comes in {when} {bound} minutes. The shortest I have is *A Grand Day Out* at 23 minutes, and *The Snowman* at 27 - say the number again if I have read it wrong."
        )));
    }
    Some(Cow::Owned(format!(
        "{rows} - all {when} {bound} minutes. Say what you feel like and I will pick one."
    )))
}

fn find_medium(text: &str) -> Option<&'static str> {
    let words = format!(" {} ", plain_words(text));
    data::MEDIUM_WORDS
        .iter()
        .flat_map(|(medium, names)| {
            let words = &words;
            names
                .iter()
                .filter_map(move |name| words.find(&format!(" {name} ")).map(|at| (at, *medium)))
        })
        .min_by_key(|(at, _)| *at)
        .map(|(_, medium)| medium)
}

fn title_medium(text: &str) -> Option<&'static str> {
    let words = format!(" {} ", plain_words(text));
    data::PICK_TITLES
        .iter()
        .filter_map(|(title, medium)| words.find(&format!(" {title} ")).map(|at| (at, *medium)))
        .min_by_key(|(at, _)| *at)
        .map(|(_, medium)| medium)
}

fn mood_medium(
    line: &str,
    previous: Option<&Exchange>,
    last_card: Option<&str>,
) -> Option<&'static str> {
    let texts = [
        line,
        previous.map_or("", |turn| &turn.user),
        previous.map_or("", |turn| &turn.bot),
    ];
    texts
        .iter()
        .find_map(|text| find_medium(text))
        .or_else(|| texts.iter().find_map(|text| title_medium(text)))
        .or_else(|| {
            let name = last_card?.split('-').next()?;
            data::MEDIUM_WORDS.iter().find_map(|(medium, words)| {
                words
                    .iter()
                    .any(|word| word.split_whitespace().next() == Some(name))
                    .then_some(*medium)
            })
        })
}

fn shelf_for_mood(medium: &str, mood: &str) -> Option<&'static str> {
    data::MOOD_TABLES
        .iter()
        .find_map(|&(kind, name, reply)| (kind == medium && name == mood).then_some(reply))
}

fn mood_pick(context: &Context<'_>, refused_only: bool) -> Option<Cow<'static, str>> {
    let reading = context.reading;
    let said = format!("{} {}", reading.line.clean, reading.line.norm_text);
    let (mood, refused) = read_mood(&said)?;
    if refused_only && !refused {
        return None;
    }
    let topic = reading.topic.map(|(name, _)| name);
    let prior = context.memory.and_then(|memory| memory.last_knowledge);
    if topic != Some("entertainment")
        && prior.is_none_or(|(domain, _)| domain != "entertainment")
        && (prior.is_some() || topic.is_some_and(|topic| topic != "feelings"))
    {
        return None;
    }
    let medium_spoken = has_medium(&plain_words(&said), context.earlier);
    if mood == "short" && !medium_spoken {
        return None;
    }
    let own = own_texts(reading).collect::<Vec<_>>().join(" ");
    let mood = if let Some((mood, refused)) = read_mood(&own) {
        if refused { "light" } else { mood }
    } else {
        let turns = context
            .earlier
            .iter()
            .rev()
            .filter_map(|turn| read_mood(&turn.user));
        turns
            .clone()
            .find_map(|(mood, refused)| (!refused).then_some(mood))
            .or_else(|| {
                turns
                    .into_iter()
                    .next()
                    .and_then(|(_, refused)| refused.then_some("light"))
            })?
    };
    let medium = if limit(&said).is_some() && !medium_spoken {
        Some("film")
    } else {
        mood_medium(&said, context.earlier.last(), prior.map(|(_, id)| id))
    };
    let rows = medium
        .and_then(|medium| shelf_for_mood(medium, mood))
        .or_else(|| shelf_for_mood("any", mood))?;
    let reply = within(rows, &said);
    if let Some(last) = context
        .earlier
        .iter()
        .rev()
        .find(|turn| turn.module == "answers")
        && plain_words(&reply) == plain_words(&last.bot)
    {
        let title = TITLE.captures(&reply)?.get(1)?.as_str();
        return Some(Cow::Owned(format!("Of those, I'd start with *{title}*.")));
    }
    Some(reply)
}

pub(super) fn registered(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    match id {
        "a-mood-anywhere" => return mood_pick(context, false),
        "a-mood-turned-down" => return mood_pick(context, true),
        _ => {}
    }
    let raw = line(context.reading);
    let plain = plain_words(&raw);
    let words = format!(" {plain} ");
    if stands_down(&words) || names_other_subject(&words) {
        return None;
    }
    match id {
        "film-kind" => genre(&raw, &plain, &words, good_shelf(&words)?, context).map(Cow::Owned),
        "film-length" => minutes(&raw, &plain, &words, context),
        _ => None,
    }
}
