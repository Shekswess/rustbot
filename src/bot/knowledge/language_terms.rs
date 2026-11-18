//! Language definitions and writing requests with wording shared across reference cards.

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "language_terms_data.rs"]
mod data;

#[path = "language_words.rs"]
mod words;

#[path = "language_writing.rs"]
mod writing;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    A_DASH_OF_SOMETHING,
    WRITING_BRACKETS,
    NOT_A_PUNCTUATION_BRACKET,
    BUYING_BRACKETS,
    NOT_MATHS_BRACKETS,
    SOMEBODY_FORGETTING,
    VERSE_INSTANCE_LEAD,
    EVERY_MINUTE,
    A_COUNTED_MINUTE,
);

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static PHOTO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:photo|picture|pic|snap|shot|image)\s+(?:of|showing)\s+(.+)$").unwrap()
});
static PRONOUN_PHOTO: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(?:me|us|we)\b").unwrap());
static LOWER_I: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bi\b").unwrap());

pub(crate) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    match id {
        "brackets-punctuation" => {
            let said = plain(reading).to_lowercase();
            WRITING_BRACKETS.is_match(&said)
                || !(NOT_A_PUNCTUATION_BRACKET.is_match(&said)
                    || BUYING_BRACKETS.is_match(&said)
                    || NOT_MATHS_BRACKETS.is_match(&said))
        }
        "dash-hyphen" => !A_DASH_OF_SOMETHING.is_match(&plain(reading).to_lowercase()),
        "which-plural-of-a-us-word" => line.words.iter().any(|word| {
            word.trim_matches(|ch: char| ch.is_ascii_punctuation())
                .eq_ignore_ascii_case("or")
        }),
        "grammar-term" => !SOMEBODY_FORGETTING.is_match(&collapsed(if line.clean.is_empty() {
            &line.text
        } else {
            &line.clean
        })),
        "write-a-limerick" => {
            let words = &line.words;
            words
                .iter()
                .any(|word| data::VERSE_METHOD_WORDS.contains(&word.as_str()))
                || !VERSE_INSTANCE_LEAD.is_match(&words.join(" ").to_lowercase())
        }
        "meeting-minutes" => {
            let said = collapsed(&line.clean);
            EVERY_MINUTE.find_iter(&said).count() > A_COUNTED_MINUTE.find_iter(&said).count()
        }
        _ => words::passes(id, reading) || writing::passes(id, reading),
    }
}

pub(crate) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = reading.line.clean.to_lowercase();
    let reply = match id {
        "dash-hyphen" => named_variant(&said, data::DASH_LOOKS).unwrap_or(data::DASH_DEFAULT),
        "metaphor-simile" => {
            let here = combined(reading, true);
            match (here.contains("metaphor"), here.contains("simile")) {
                (true, false) => data::METAPHOR,
                (false, true) => data::SIMILE,
                _ => data::BOTH_FIGURES_DEFAULT,
            }
        }
        "figure-of-speech" => {
            let here = combined(reading, false);
            data::FIGURES
                .iter()
                .find_map(|&(keys, reply)| {
                    keys.iter().any(|word| here.contains(word)).then_some(reply)
                })
                .unwrap_or(data::FIGURE_DEFAULT)
        }
        "grammar-term" => grammar_term(reading),
        "poem-form" => poem_form(reading),
        "speak-clearly" => {
            if mentions(&said, data::SPEAK_WORDS_0) {
                data::SPEAK_REPLY_0
            } else if mentions(&said, data::SPEAK_WORDS_1) {
                data::SPEAK_REPLY_1
            } else {
                data::SPEAK_DEFAULT
            }
        }
        "mark-unnamed" => mark(&said),
        "write-a-caption" => return Some(caption(&said)),
        _ => return words::registered(id, reading).or_else(|| writing::registered(id, reading)),
    };
    Some(Cow::Borrowed(reply))
}

fn plain<'a>(reading: &'a Reading<'_>) -> &'a str {
    [&reading.line.plain, &reading.line.clean, &reading.line.text]
        .into_iter()
        .find(|text| !text.is_empty())
        .map_or("", String::as_str)
}

fn collapsed(text: &str) -> String {
    text.split(crate::text::is_space)
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn combined(reading: &Reading<'_>, normal: bool) -> String {
    [
        reading.line.clean.as_str(),
        reading.line.text.as_str(),
        reading.raw,
        if normal {
            reading.line.norm_text.as_str()
        } else {
            ""
        },
    ]
    .join(" ")
    .to_lowercase()
}

fn mentions(said: &str, words: &[&str]) -> bool {
    WORD.find_iter(said).any(|token| {
        words.iter().any(|word| {
            token
                .as_str()
                .strip_prefix(word)
                .is_some_and(|tail| matches!(tail, "" | "s" | "es" | "ing" | "ed"))
        })
    })
}

fn named_variant(said: &str, rows: &'static [(&[&str], &str)]) -> Option<&'static str> {
    rows.iter().find_map(|&(keys, reply)| {
        (mentions(said, keys)
            || keys
                .iter()
                .any(|word| word.contains(' ') && said.contains(word)))
        .then_some(reply)
    })
}

fn grammar_term(reading: &Reading<'_>) -> &'static str {
    let flat = reading.line.words.join(" ").to_lowercase();
    data::GRAMMAR_TERMS
        .iter()
        .find_map(|&(key, plural, reply)| {
            (reading
                .line
                .words
                .iter()
                .any(|word| word.eq_ignore_ascii_case(key) || word.eq_ignore_ascii_case(plural))
                || (key.contains(' ') && flat.contains(key)))
            .then_some(reply)
        })
        .unwrap_or(data::GRAMMAR_DEFAULT)
}

fn poem_form(reading: &Reading<'_>) -> &'static str {
    let flat = reading.line.words.join(" ").to_lowercase();
    data::POEM_FORMS
        .iter()
        .find_map(|&(keys, reply)| {
            keys.iter()
                .any(|&key| {
                    reading
                        .line
                        .words
                        .iter()
                        .any(|word| word.eq_ignore_ascii_case(key))
                        || (key.contains(' ') && flat.contains(key))
                })
                .then_some(reply)
        })
        .unwrap_or(data::POEM_DEFAULT)
}

fn mark(said: &str) -> &'static str {
    const SIGNS: &[(char, &str)] = &[
        ('#', "hashtag"),
        ('@', "at sign"),
        ('|', "pipe"),
        ('~', "tilde"),
        ('*', "asterisk"),
        ('^', "caret"),
        ('_', "underscore"),
    ];
    if let Some(reply) = named_variant(said, data::MARK_LOOKS) {
        return reply;
    }
    let named = SIGNS.iter().find(|(sign, _)| said.contains(*sign));
    data::GRAMMAR_TERMS
        .iter()
        .find_map(|&(name, _, reply)| {
            let matches = named.map_or_else(|| said.contains(name), |&(_, sign)| name == sign);
            matches.then_some(reply)
        })
        .unwrap_or(data::MARK_DEFAULT)
}

fn caption(said: &str) -> Cow<'static, str> {
    let Some(photo) = PHOTO.captures(said) else {
        return Cow::Borrowed(data::CAPTION_DEFAULT);
    };
    let mut words: Vec<&str> = WORD
        .find_iter(&photo[1])
        .map(|word| word.as_str())
        .take_while(|word| {
            ![
                "and", "but", "that", "which", "please", "for", "from", "on", "at",
            ]
            .contains(word)
        })
        .collect();
    while words
        .last()
        .is_some_and(|word| ["i", "me", "my", "the", "a", "an", "just", "that"].contains(word))
    {
        words.pop();
    }
    if words.len() > 6
        || words
            .iter()
            .all(|word| ["my", "the", "a", "an", "our", "some"].contains(word))
    {
        return Cow::Borrowed(data::CAPTION_DEFAULT);
    }
    let mut subject = words.join(" ");
    let padded = format!(" {subject} ");
    let has = |pool: &[&str]| {
        pool.iter().any(|word| {
            padded.match_indices(word).any(|(start, found)| {
                padded[..start].ends_with(' ') && padded[start + found.len()..].starts_with(' ')
            })
        })
    };
    let detail = if has(data::PEOPLE_PHOTO) || PRONOUN_PHOTO.is_match(&subject) {
        data::PHOTO_DETAIL_PEOPLE
    } else {
        data::PHOTO_DETAIL
            .iter()
            .find_map(|&(keys, detail)| has(keys).then_some(detail))
            .unwrap_or(data::PHOTO_DETAIL_ANY)
    };
    subject[..1].make_ascii_uppercase();
    let typed = LOWER_I.replace_all(&subject, "I");
    Cow::Owned(format!(
        "{}{typed}, {detail}{}",
        data::CAPTION_BEFORE,
        data::CAPTION_AFTER
    ))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/language_terms.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_explanations_and_guards_match_pinned_pybot() {
        let mut differences = Vec::new();
        for &fixtures::Case(id, raw, guard, phrase) in fixtures::CASES {
            let reading = Reading::new(raw);
            let actual = registered(id, &reading);
            if guard.is_some_and(|expected| passes(id, &reading) != expected)
                || actual.as_deref() != phrase
            {
                differences.push((id, raw, guard, passes(id, &reading), phrase, actual));
            }
        }
        assert!(differences.is_empty(), "{differences:#?}");
    }
}
