//! Medium, audience and country selection for recommendation cards.

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use super::{
    ENTERTAINMENT_NEGATORS,
    entertainment::{Medium, medium, plain_words, read_mood},
    guards::Context,
};

#[path = "film_shelves_data.rs"]
mod data;
#[cfg(test)]
#[path = "film_shelves_tests.rs"]
mod tests;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(data::$name).expect(stringify!($name))
        });)+
    };
}
patterns!(REFUSED_KIND, WATCHES_SOMETHING, BOOK_AS_VERB);

static SUBS_TURNED_DOWN: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(data::SUBS_TURNED_DOWN).expect("subtitles refusal pattern")
});

#[derive(Clone, Copy, PartialEq, Eq)]
enum Constraint {
    Free,
    NoSubtitles,
}

fn contains_one(text: &str, parts: &[&str]) -> bool {
    parts.iter().any(|part| text.contains(part))
}

pub(super) fn contains_phrase(text: &str, phrases: &[&str]) -> bool {
    phrases
        .iter()
        .any(|phrase| text.contains(&format!(" {phrase} ")))
}

pub(super) fn line(reading: &Reading<'_>) -> String {
    [reading.raw, &reading.line.clean, &reading.line.norm_text]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn country_is_literal(reading: &Reading<'_>) -> bool {
    !reading.line.words.windows(2).any(|pair| {
        data::COUNTRIES
            .iter()
            .any(|(word, ..)| pair[0].eq_ignore_ascii_case(word))
            && data::COUNTRY_AFTER_A_LITTLE_WORD
                .iter()
                .chain(data::COUNTRY_BEFORE_A_THING)
                .any(|word| pair[1].eq_ignore_ascii_case(word))
    })
}

pub(super) fn stands_down(words: &str) -> bool {
    BOOK_AS_VERB.is_match(words)
        || contains_one(words, data::PICK_OTHER_MEDIUM)
        || contains_phrase(words, data::PICK_SUBJECT)
        || contains_phrase(words, data::PICK_REACT)
        || contains_one(words, &["family", "the kids", "kids", "children", "child"])
}

pub(super) fn earlier_lines<'a>(
    mine: &'a str,
    turns: &'a [Exchange],
) -> impl Iterator<Item = &'a str> {
    turns[turns.len().saturating_sub(2)..]
        .iter()
        .map(|turn| turn.user.as_str())
        .filter(move |line| !line.is_empty() && plain_words(line) != mine)
}

fn mentions_limit(raw: &str, plain: &str) -> Option<Constraint> {
    let both = format!(" {} {plain} ", raw.to_lowercase().replace('’', "'"));
    if contains_one(&both, data::SUBS_TELLS) {
        Some(Constraint::NoSubtitles)
    } else if contains_one(&both, data::FREE_TELLS) {
        Some(Constraint::Free)
    } else {
        None
    }
}

fn limit(line: &str, plain: &str, earlier: &[Exchange]) -> Option<Constraint> {
    mentions_limit(line, plain).or_else(|| {
        earlier_lines(plain, earlier).find_map(|prior| mentions_limit(prior, &plain_words(prior)))
    })
}

fn wants_subtitles(words: &str) -> bool {
    !contains_one(words, data::SUBS_TELLS)
        && !SUBS_TURNED_DOWN.is_match(words).unwrap_or(false)
        && (contains_one(words, data::SUBS_WANT)
            || WATCHES_SOMETHING.is_match(words)
                && contains_one(words, data::SUBS_WANT_IF_WATCHING))
}

fn cinema(words: &str) -> Option<(&'static str, &'static str, &'static str)> {
    data::COUNTRIES
        .iter()
        .find_map(|&(word, adjective, country, language)| {
            let start = words.find(&format!(" {word} "))?;
            let rejected = words[..start]
                .split_whitespace()
                .rev()
                .take(2)
                .any(|word| ENTERTAINMENT_NEGATORS.contains(&word));
            (!rejected).then_some((adjective, country, language))
        })
}

fn picks(kind: Medium, who: &str, limit: Option<Constraint>) -> Vec<&'static data::Pick> {
    let mut rows: Vec<_> = data::PICKS
        .iter()
        .filter(|row| row.medium == kind)
        .collect();
    if matches!(kind, Medium::Film | Medium::Series)
        && let Some(limit) = limit
    {
        let fits = |row: &&data::Pick| match limit {
            Constraint::Free => row.free,
            Constraint::NoSubtitles => row.no_subtitles,
        };
        if rows.iter().any(fits) {
            rows.retain(fits);
        }
    }
    if !who.is_empty() {
        rows.sort_by_key(|row| !row.company.contains(&who));
    }
    rows.truncate(3);
    rows
}

fn for_whom(words: &str) -> &'static str {
    if contains_one(
        words,
        &[
            "parents",
            " my mum",
            " my dad",
            " my mother",
            " my father",
            "in laws",
            "in law",
        ],
    ) {
        "parents"
    } else if contains_one(
        words,
        &[
            "partner",
            "wife",
            "husband",
            "girlfriend",
            "boyfriend",
            "my other half",
        ],
    ) {
        "partner"
    } else if contains_one(
        words,
        &[
            " friends",
            " mate",
            "mates",
            "everyone",
            "the girls",
            "the lads",
        ],
    ) {
        "friends"
    } else {
        ""
    }
}

pub(super) fn rendered_picks(rows: impl Iterator<Item = String>) -> String {
    let titles: Vec<_> = rows.collect();
    match titles.split_last() {
        Some((last, [])) => last.clone(),
        Some((last, before)) => format!("{} or {last}", before.join(", ")),
        None => String::new(),
    }
}

fn pick_list(rows: &[&data::Pick]) -> String {
    rendered_picks(
        rows.iter()
            .map(|row| format!("*{}* ({})", row.title, row.reason)),
    )
}

fn after_picks(kind: Medium, who: &str, limit: Option<Constraint>) -> &'static str {
    if matches!(kind, Medium::Film | Medium::Series) {
        match limit {
            Some(Constraint::Free) => {
                return " All three are free-to-air rather than behind a subscription, and iPlayer and ITVX are both free - rights move, so check iPlayer first.";
            }
            Some(Constraint::NoSubtitles) => {
                return " Nothing there needs subtitles: all of them are in English and none of them is harder to follow than a chat show.";
            }
            None => {}
        }
    }
    if who == "parents" {
        return match kind {
            Medium::Book => {
                " All of them are safe to hand to a parent, and none needs any explaining."
            }
            Medium::Podcast => {
                " All of them are safe to put on with a parent, and none needs any explaining."
            }
            Medium::Film | Medium::Series => {
                " All of them are safe to watch with a parent, and none needs any explaining."
            }
        };
    }
    if kind == Medium::Book {
        " Tell me the last one you enjoyed and I'll match the next."
    } else {
        " Say the mood and I'll narrow it down."
    }
}

fn reply(kind: Medium, who: &str, limit: Option<Constraint>, rows: &[&data::Pick]) -> String {
    let head = match kind {
        Medium::Film => "For a film",
        Medium::Series => "For a series",
        Medium::Book => "For a book",
        Medium::Podcast => "For a podcast",
    };
    let company = match who {
        "parents" => " with your parents",
        "partner" => " for the two of you",
        "friends" => " with friends",
        _ => "",
    };
    format!(
        "{head}{company}: {}.{}",
        pick_list(rows),
        after_picks(kind, who, limit)
    )
}

fn from_country(adjective: &str, country: &str, language: &str) -> String {
    let found: Vec<_> = data::NATIONAL_CINEMA
        .iter()
        .filter(|(_, from, _)| *from == country)
        .take(3)
        .collect();
    let count = found.len();
    if found.is_empty() {
        let have = data::KNOWN_COUNTRIES;
        return format!(
            "I have not got a {adjective} list, and I would rather say so than hand you the wrong film. I do have {have} - tell me a {adjective} film you liked and I will match the next one."
        );
    }
    let picks = rendered_picks(
        found
            .into_iter()
            .map(|(title, _, why)| format!("*{title}* ({why})")),
    );
    let article = if adjective
        .to_ascii_lowercase()
        .starts_with(['a', 'e', 'i', 'o', 'u'])
    {
        "an"
    } else {
        "a"
    };
    let count = match count {
        1 => "It is",
        2 => "Both are",
        _ => "All three are",
    };
    format!(
        "For {article} {adjective} film: {picks}. {count} in {language} with subtitles. Say the mood and I'll narrow it down."
    )
}

fn non_english() -> String {
    let rows = rendered_picks(
        data::NATIONAL_CINEMA
            .iter()
            .take(9)
            .step_by(4)
            .map(|(title, country, reason)| format!("*{title}* ({country}: {reason})")),
    );
    format!("Subtitled, then: {rows}. Tell me a country and I will give you three of its films.")
}

fn plain(line: &str, plain: &str, words: &str, earlier: &[Exchange]) -> Option<String> {
    if read_mood(line).is_some() {
        return None;
    }
    let carried = || earlier_lines(plain, earlier).find_map(medium);
    let mut kind = medium(plain).or_else(carried)?;
    if let Some(refused) = REFUSED_KIND
        .captures(words)
        .and_then(|found| medium(&found["kind"]))
        && refused == kind
    {
        kind = carried()
            .filter(|prior| *prior != refused)
            .or(match refused {
                Medium::Film => Some(Medium::Series),
                Medium::Series => Some(Medium::Film),
                _ => None,
            })?;
    }
    if kind == Medium::Podcast {
        return None;
    }
    if kind == Medium::Film {
        if let Some((adjective, country, language)) = cinema(words) {
            return Some(from_country(adjective, country, language));
        }
        if wants_subtitles(words) {
            return Some(non_english());
        }
    }
    let who = for_whom(words);
    let limit = limit(line, plain, earlier);
    let rows = picks(kind, who, limit);
    (!rows.is_empty()).then(|| reply(kind, who, limit, &rows))
}

fn constrained(line: &str, plain: &str, earlier: &[Exchange]) -> Option<String> {
    let limit = limit(line, plain, earlier)?;
    match (medium(plain), limit) {
        (Some(Medium::Book), _) => Some(reply(
            Medium::Book,
            "",
            None,
            &picks(Medium::Book, "", None),
        )),
        (_, Constraint::NoSubtitles) => {
            let rows = picks(Medium::Film, "", Some(limit));
            Some(format!(
                "Nothing that needs subtitles, then: {}. All three are in English, so nobody has to read the screen to follow them.",
                pick_list(&rows)
            ))
        }
        (kind, Constraint::Free) => {
            let kind = kind
                .filter(|kind| matches!(kind, Medium::Film | Medium::Series))
                .unwrap_or(Medium::Series);
            let rows = picks(kind, "", Some(limit));
            Some(format!(
                "Then it is iPlayer and ITVX, and both are free. {}. All three are BBC, so they are on iPlayer, and a film on ITV turns up most weeks too - check iPlayer rather than trusting a listing, because rights move.",
                pick_list(&rows)
            ))
        }
    }
}

pub(super) fn registered(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    let line = line(context.reading);
    let normalized = plain_words(&line);
    let words = format!(" {normalized} ");
    if stands_down(&words) {
        return None;
    }
    match id {
        "pick-plain" | "pick-country" => plain(&line, &normalized, &words, context.earlier),
        "pick-limit" => constrained(&line, &normalized, context.earlier),
        _ => None,
    }
    .map(Cow::Owned)
}
