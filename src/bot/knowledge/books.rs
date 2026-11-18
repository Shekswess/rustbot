//! Pinned titles, author correction, and related-book suggestions.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::{
    BOOK_AGE_BANDS, BOOK_AGE_WORDS, BOOK_AUTHOR_ALIASES, BOOK_AUTHORS, BOOK_GENRES, BOOK_SIMILAR,
    BOOK_SIMILAR_PATTERN, BOOK_TITLE_DISPLAY, BOOK_TITLE_SMALL, BOOK_TITLE_SPELLING,
    has_ascii_phrase,
};

static LIKE_A_BOOK: LazyLock<Regex> = LazyLock::new(|| Regex::new(BOOK_SIMILAR_PATTERN).unwrap());
static LIKE_A_GENRE: LazyLock<Regex> = LazyLock::new(|| {
    let genres = BOOK_GENRES
        .iter()
        .map(|genre| regex::escape(genre))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(
        r"\b(?:like|similar to|anything like|something like|after)\s+(?:a |an |some |any )?(?:{genres})\b"
    ))
    .unwrap()
});
static CHILD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b\d+ ?year old|\byear olds?\b|\bkids?\b|\bchildren?\b|\btoddlers?\b|\bbab(?:y|ies)\b|",
        r"\b(?:my|our|his|her|their) (?:son|daughter|child|boy|girl|grandson|granddaughter|niece|nephew)\b"
    ))
    .unwrap()
});
static TITLE_END: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:but|that|which|without|and|because)\b").unwrap());
static YEARS_OLD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(\d{1,2})\s*(?:-|\s)?\s*(?:year|years|yr)s?\s*-?\s*old\b").unwrap()
});
static AGED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:aged|age|who is|who's|is)\s+(\d{1,2})\b").unwrap());
static TEEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bteen(?:ager)?s?\b").unwrap());

pub(super) fn for_age(reading: &Reading<'_>) -> Cow<'static, str> {
    let line = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
    .to_lowercase();
    let number = [&*YEARS_OLD, &*AGED]
        .into_iter()
        .find_map(|pattern| pattern.captures(&line)?.get(1)?.as_str().parse::<u8>().ok());
    let age = number
        .or_else(|| {
            BOOK_AGE_WORDS
                .iter()
                .find_map(|&(word, age)| line.contains(word).then_some(age))
        })
        .unwrap_or_default();
    let age = if age == 0 && TEEN.is_match(&line) {
        13
    } else {
        age
    };
    if age != 0
        && let Some((_, _, lead)) = BOOK_AGE_BANDS
            .iter()
            .find(|(low, high, _)| (*low..=*high).contains(&age))
    {
        return format!(
            "{lead} Take them to a bookshop and let them pick the first one themselves: the series they choose is the one they read."
        ).into();
    }
    "Short chapters and a picture every few pages are what a 5 to 7 year old reading alone wants: *The Worst Witch*, *Mr Gum*, *Dog Man* or *Captain Underpants*. For 8 to 9, *The Boy at the Back of the Class*, *Horrible Histories* or *Percy Jackson*. For 10 to 12, *Wonder*, *Artemis Fowl* or *Skulduggery Pleasant*. For 13 and over, *The Hunger Games*, *Northern Lights* or *The Book Thief*. Take them to a bookshop and let them pick the first one themselves; the series they choose is the one they read.".into()
}

pub(super) fn genre_is_named(reading: &Reading<'_>) -> bool {
    let line = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    LIKE_A_GENRE.is_match(&line.to_lowercase())
}

pub(super) fn longest_named(
    line: &str,
    books: &'static [(&str, &str)],
) -> Option<(&'static str, &'static str)> {
    books
        .iter()
        .enumerate()
        .filter(|(_, (title, _))| has_ascii_phrase(line, title))
        .max_by_key(|(index, (title, _))| (title.chars().count(), std::cmp::Reverse(*index)))
        .map(|(_, pair)| *pair)
}

pub(super) fn names_a_book(line: &str) -> bool {
    longest_named(line, BOOK_SIMILAR).is_some()
}

pub(super) fn similar(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = if reading.line.clean.is_empty() {
        &reading.line.norm_text
    } else {
        &reading.line.clean
    }
    .to_lowercase();
    let named = longest_named(&line, BOOK_SIMILAR);
    let relevant = named.map_or_else(
        || Cow::Borrowed(line.as_str()),
        |(title, _)| Cow::Owned(line.replace(title, " ")),
    );
    if CHILD.is_match(&relevant) {
        return None;
    }
    if let Some((_, reply)) = named {
        return Some(Cow::Borrowed(reply));
    }
    let captures = LIKE_A_BOOK.captures(&line)?;
    let delimited = [' ', '.', '?', '!'];
    let title = captures.get(1).or_else(|| captures.get(2))?.as_str();
    let title = TITLE_END
        .split(title.trim_matches(delimited))
        .next()?
        .trim_matches(delimited);
    if title.is_empty() {
        return None;
    }
    let title = title_case(title);
    Some(Cow::Owned(format!(
        "I don't know *{title}* well enough to match it, and a guess would waste your evening. Tell me what you liked about it - the people, the pace, or the writing - and I'll pick something close."
    )))
}

fn title_case(title: &str) -> String {
    let mut previous_letter = false;
    title
        .chars()
        .map(|character| {
            let rendered = if previous_letter {
                character.to_ascii_lowercase()
            } else {
                character.to_ascii_uppercase()
            };
            previous_letter = character.is_ascii_alphabetic();
            rendered
        })
        .collect()
}

fn name_in(clean: &str, title: Option<&str>) -> Option<&'static str> {
    for &(_, author) in BOOK_AUTHORS {
        let Some(surname) = author.split_whitespace().next_back() else {
            continue;
        };
        let surname = surname.to_lowercase();
        let other = BOOK_AUTHOR_ALIASES
            .iter()
            .find_map(|&(key, alias)| (key == surname).then_some(alias));
        if std::iter::once(surname.as_str()).chain(other).any(|name| {
            title.is_none_or(|title| !title.contains(name)) && has_ascii_phrase(clean, name)
        }) {
            return Some(author);
        }
    }
    None
}

fn titles_of(author: &str) -> impl Iterator<Item = &'static str> {
    BOOK_AUTHORS
        .iter()
        .filter_map(move |&(title, writer)| (author == writer).then_some(title))
}

pub(super) fn display(title: &str) -> String {
    if let Some((_, rendered)) = BOOK_TITLE_DISPLAY.iter().find(|(key, _)| *key == title) {
        return (*rendered).to_owned();
    }
    title
        .split_whitespace()
        .enumerate()
        .map(|(index, word)| {
            if index > 0 && BOOK_TITLE_SMALL.contains(&word) {
                return word.to_owned();
            }
            let mut chars = word.chars();
            let first = chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .collect::<String>();
            format!("{first}{}", chars.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn words_list(titles: &[&str]) -> String {
    let rendered: Vec<_> = titles.iter().map(|title| display(title)).collect();
    match rendered.as_slice() {
        [] => String::new(),
        [only] => only.clone(),
        [first @ .., last] => format!("{} and {last}", first.join(", ")),
    }
}

pub(super) fn who_wrote(reading: &Reading<'_>) -> Option<String> {
    let mut line = reading.line.clean.replace(" & ", " and ");
    for &(spelling, standard) in BOOK_TITLE_SPELLING {
        line = line.replace(spelling, standard);
    }
    let found = longest_named(&line, BOOK_AUTHORS);
    let named = name_in(&line, found.map(|(title, _)| title));
    if let Some((title, author)) = found {
        let others: Vec<_> = titles_of(author)
            .filter(|other| *other != title && !other.contains(title) && !title.starts_with(other))
            .take(3)
            .collect();
        let author_name = author.replace('.', "");
        let said_title = display(title);
        let mut reply = match named {
            Some(person) if person != author => {
                format!(
                    "Not quite - {said_title} is by {author_name}, not {}.",
                    person.replace('.', "")
                )
            }
            Some(_) => format!("Yes - {said_title} is by {author_name}."),
            None => format!("{said_title} is by {author_name}."),
        };
        if !others.is_empty() {
            reply.push_str(&format!(
                " Other books by {author_name}: {}.",
                words_list(&others)
            ));
        }
        return Some(reply);
    }
    let named = named?;
    let titles: Vec<_> = titles_of(named).take(6).collect();
    if titles.is_empty() {
        return None;
    }
    let more = if titles.len() > 5 {
        ", among others"
    } else {
        ""
    };
    let shown = words_list(&titles[..titles.len().min(5)]);
    Some(format!("{} wrote {shown}{more}.", named.replace('.', "")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_title_correction_and_unknown_books_take_different_paths() {
        let corrected = who_wrote(&Reading::new("Did Charles Dickens write Emma?"));
        assert!(corrected.is_some_and(|reply| {
            reply.starts_with("Not quite - Emma is by Jane Austen, not Charles Dickens.")
        }));
        assert_eq!(
            who_wrote(&Reading::new("Who wrote the unnamed typewriter manual?")),
            None
        );
    }
}
