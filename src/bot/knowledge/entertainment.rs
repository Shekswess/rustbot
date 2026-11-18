//! Film duration filters and medium-specific mood replies.

use crate::bot::{Memory, Reading};
use regex::Regex;
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use super::{
    ENTERTAINMENT_BOOK_MOODS, ENTERTAINMENT_CONTRACTIONS, ENTERTAINMENT_FILM_MINUTES,
    ENTERTAINMENT_HOUR_WORDS, ENTERTAINMENT_MOOD_SIGNALS, ENTERTAINMENT_MOODS,
    ENTERTAINMENT_NEGATORS, ENTERTAINMENT_SOFT, ENTERTAINMENT_WANTING, FILM_LIMIT_CUE_PATTERN,
    FILM_LIMIT_HOURS_PATTERN, FILM_LIMIT_UNDER_PATTERN, FILM_NOT_TURN_DOWN_PATTERN,
    FILM_SCARE_REFUSED_AFTER_PATTERN, FILM_SCARE_TURNED_DOWN, FILM_SCARE_WORDS, FILM_STRICT_LIMIT,
    FILM_TURNED_DOWN_WANT, FILM_TURNS_BACK, FILM_WITH_CHILDREN,
};
use super::{books, entertainment_titles, guards::Context};

#[cfg(test)]
#[path = "film_suggestion_tests.rs"]
mod suggestion_tests;
#[path = "film_suggestion_data.rs"]
mod suggestions;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(LIMIT_CUE, FILM_LIMIT_CUE_PATTERN);
pattern!(UNDER, FILM_LIMIT_UNDER_PATTERN);
pattern!(HOURS, FILM_LIMIT_HOURS_PATTERN);
pattern!(NUMBER, r"^(\d{2,3})\b");
pattern!(TITLE, r"\*([^*]{2,60})\*");
pattern!(PICK_TITLE, r"\*([^*\n]{2,60})\*");
pattern!(PICK_WORD, r"[a-z']+");
pattern!(PICK_LEAD, r"(?i)^for (?:a|an|the)[^:\n]{0,40}:\s*");
pattern!(NEGATION_APOSTROPHE, r"n'\s*t\b");
pattern!(NOT_A_REFUSAL, FILM_NOT_TURN_DOWN_PATTERN);
pattern!(SCARE_REFUSED_AFTER, FILM_SCARE_REFUSED_AFTER_PATTERN);
pattern!(FRESH_LIST, suggestions::FRESH_LIST);
pattern!(FILM_ASK, suggestions::FILM_ASK);
pattern!(
    LIKE_VERB,
    r"\b(?:like|likes|liked|loves?|loved|enjoyed|similar)\b"
);
pattern!(
    LIKED_TITLE,
    r"\b(?:film|movie|show|series|programme)s?\s+(?:like|similar to)\s+([a-z0-9'&: -]{2,40})"
);
pattern!(
    CUT_LIKED_TITLE,
    r"\b(?:but|that|which|without|and|or|please)\b|\b(?:film|movie|show|series|programme)s?\b"
);
pattern!(FILM_OR_MOVIE, r"\b(?:films?|movies?)\b");
pattern!(SEEN_CONTRACTION, r"(?i)\bisn't\b");
pattern!(
    TEEN_VIEWER,
    r"\bteen\w*|\badolescen\w*|\b1[3-9]\s*(?:year|yr)|\b(?:aged|age)\s*1[3-9]\b"
);
pattern!(
    VIEWER_AGE,
    r"\b(\d{1,2})\s*(?:year|yr)s?[ -]old\b|\bage[d]?\s*(\d{1,2})\b|\b(?:is|are)\s+(\d{1,2})\b"
);

fn split(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
}

pub(super) fn plain_words(line: &str) -> String {
    let lower = line.to_lowercase().replace('’', "'").replace('-', " ");
    let plain = NEGATION_APOSTROPHE
        .replace_all(&lower, "nt")
        .replace('\'', "");
    split(&plain)
        .map(|word| {
            ENTERTAINMENT_CONTRACTIONS
                .iter()
                .find_map(|&(from, to)| (word == from).then_some(to))
                .unwrap_or(word)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn limit(line: &str) -> Option<(u16, bool)> {
    let plain = format!(" {} ", plain_words(line));
    let (found, strict) = if let Some(found) = UNDER.captures(&plain) {
        let strict = FILM_STRICT_LIMIT.contains(&&found[1]);
        (found, strict)
    } else {
        (LIMIT_CUE.captures(&plain)?, false)
    };
    let tail = found[2].trim();
    let minutes = if let Some(number) = NUMBER.captures(tail) {
        number[1].parse().ok()?
    } else if let Some((_, minutes)) = ENTERTAINMENT_HOUR_WORDS
        .iter()
        .find(|(phrase, _)| tail.starts_with(phrase))
    {
        *minutes
    } else {
        let hours = HOURS.captures(tail)?;
        if hours.get(0)?.start() != 0 {
            return None;
        }
        (hours[1].parse::<f64>().ok()? * 60.0).round_ties_even() as u16
    };
    (minutes != 0).then_some((minutes, strict))
}

fn film_minutes(title: &str) -> Option<u16> {
    ENTERTAINMENT_FILM_MINUTES
        .iter()
        .find_map(|&(name, minutes)| (name == title).then_some(minutes))
}

pub(super) fn within(reply: &'static str, line: &str) -> Cow<'static, str> {
    let Some((limit, strict)) = limit(line) else {
        return Cow::Borrowed(reply);
    };
    let mut named = Vec::new();
    for title in TITLE
        .captures_iter(reply)
        .filter_map(|matched| matched.get(1))
        .map(|matched| matched.as_str())
    {
        let Some(minutes) = film_minutes(title) else {
            return Cow::Borrowed(reply);
        };
        if !named.iter().any(|(name, _)| *name == title) {
            named.push((title, minutes));
        }
    }
    let fit = named
        .iter()
        .filter(|(_, minutes)| {
            if strict {
                *minutes < limit
            } else {
                *minutes <= limit
            }
        })
        .map(|(title, minutes)| format!("*{title}* ({minutes} minutes)"))
        .collect::<Vec<_>>();
    let over = named.len() - fit.len();
    if over == 0 {
        return Cow::Borrowed(reply);
    }
    let lead = reply
        .split_once(':')
        .map(|(lead, _)| lead.trim())
        .filter(|lead| !lead.is_empty() && lead.chars().count() <= 40 && !lead.contains('*'));
    if fit.is_empty() {
        let (shortest, minutes) = named
            .iter()
            .min_by_key(|(_, time)| time)
            .expect("nonempty film list");
        let prefix = lead.map_or_else(
            || "Nothing in that list".to_owned(),
            |lead| format!("{lead}, nothing there"),
        );
        let wording = if strict { "under" } else { "at" };
        return Cow::Owned(format!(
            "{prefix} comes in {wording} {limit} minutes - the shortest is *{shortest}* at {minutes}. Say another mood and I will look again."
        ));
    }
    let (last, leading) = fit.split_last().expect("nonempty fitted film list");
    let body = if leading.is_empty() {
        last.clone()
    } else {
        format!("{} or {last}", leading.join(", "))
    };
    let wording = if strict { "under" } else { "up to" };
    let runs = if over == 1 { "runs" } else { "run" };
    Cow::Owned(match lead {
        Some(lead) => {
            let others = if over == 1 { "one" } else { "two" };
            format!(
                "{lead}, {wording} {limit} minutes: {body}. The other {others} {runs} longer, so say if you would rather have one of those."
            )
        }
        None => format!(
            "Films that fit {wording} {limit} minutes: {body}. The rest of that list {runs} longer, so say if you would rather have one of those."
        ),
    })
}

struct Moods {
    phrases: Vec<(&'static str, Vec<String>)>,
    signals: HashSet<&'static str>,
    by_word: HashMap<&'static str, &'static str>,
}

static MOODS: LazyLock<Moods> = LazyLock::new(|| {
    let mut phrases = Vec::new();
    for &(mood, branches) in ENTERTAINMENT_MOODS {
        for phrase in branches {
            let normalized = phrase.replace(['-', '\''], " ");
            phrases.push((mood, split(&normalized).map(str::to_owned).collect()));
        }
    }
    let signals = ENTERTAINMENT_MOOD_SIGNALS.iter().copied().collect();
    let mut by_word = HashMap::new();
    for &(mood, branches) in ENTERTAINMENT_MOODS {
        for &word in branches.iter().filter(|word| !word.contains(' ')) {
            let plain = plain_words(word);
            if phrase_mood(&phrases, &plain.split_whitespace().collect::<Vec<_>>())
                == Some((mood, false))
            {
                by_word.entry(word).or_insert(mood);
            }
        }
    }
    Moods {
        phrases,
        signals,
        by_word,
    }
});

fn negated(words: &[&str], start: usize, first: &str) -> bool {
    let negator = |word| ENTERTAINMENT_NEGATORS.contains(&word);
    if start == 0 || negator(first) {
        return false;
    }
    negator(words[start - 1])
        || (start > 1
            && negator(words[start - 2])
            && ENTERTAINMENT_SOFT.contains(&words[start - 1]))
        || (1..=4).any(|back| {
            start > back
                && ENTERTAINMENT_WANTING.contains(&words[start - back])
                && negator(words[start - back - 1])
        })
}

fn phrase_mood(
    phrases: &[(&'static str, Vec<String>)],
    words: &[&str],
) -> Option<(&'static str, bool)> {
    let mut refused = false;
    for (mood, phrase) in phrases.iter().filter(|(_, phrase)| !phrase.is_empty()) {
        for (start, run) in words.windows(phrase.len()).enumerate() {
            if !run.iter().zip(phrase).all(|(word, wanted)| *word == wanted) {
                continue;
            }
            if negated(words, start, &phrase[0]) {
                refused = true;
            } else {
                return Some((mood, false));
            }
        }
    }
    refused.then_some(("light", true))
}

pub(super) fn read_mood(line: &str) -> Option<(&'static str, bool)> {
    let line = plain_words(line);
    let words = line.split_whitespace().collect::<Vec<_>>();
    let mut refused = match phrase_mood(&MOODS.phrases, &words) {
        Some((mood, false)) => return Some((mood, false)),
        found => found.is_some(),
    };
    for (at, &word) in words
        .iter()
        .enumerate()
        .filter(|(_, word)| MOODS.signals.contains(*word))
    {
        if negated(&words, at, word) {
            refused = true;
            continue;
        }
        if let Some(mood) = MOODS.by_word.get(word) {
            return Some((mood, false));
        }
    }
    refused.then_some(("light", true))
}

fn mood(line: &str) -> Option<&'static str> {
    read_mood(line).map(|(mood, _)| mood)
}

pub(super) fn reads_mood(reading: &Reading<'_>) -> bool {
    mood(&reading.line.words.join(" ")).is_some()
}

pub(super) fn own_texts<'a>(reading: &'a Reading<'_>) -> impl Iterator<Item = &'a str> {
    let choices = if reading.raw.trim().is_empty() {
        [reading.line.norm_text.as_str(), reading.line.clean.as_str()]
    } else {
        [reading.raw, ""]
    };
    choices.into_iter().filter(|text| !text.is_empty())
}

pub(super) fn refuses_scare(reading: &Reading<'_>) -> bool {
    own_texts(reading).any(|own| {
        word_turned_down(
            own,
            |word| FILM_SCARE_WORDS.contains(&word),
            &SCARE_REFUSED_AFTER,
        )
    })
}

pub(super) fn word_turned_down(
    text: &str,
    named: impl Fn(&str) -> bool,
    refused_after: &Regex,
) -> bool {
    let plain = plain_words(text);
    let mut plain = NOT_A_REFUSAL.replace_all(&plain, " ").into_owned();
    for phrase in FILM_TURNED_DOWN_WANT {
        plain = plain.replace(phrase, " not ");
    }
    let words = plain.split_whitespace().collect::<Vec<_>>();
    words.iter().enumerate().any(|(at, word)| {
        if !named(word) && !named(word.trim_end_matches('s')) {
            return false;
        }
        let before = &words[at.saturating_sub(8)..at];
        let after_turn = before
            .iter()
            .rposition(|word| FILM_TURNS_BACK.contains(word))
            .map_or(0, |index| index + 1);
        before[after_turn..].iter().any(|word| {
            ENTERTAINMENT_NEGATORS.contains(word) || FILM_SCARE_TURNED_DOWN.contains(word)
        }) || refused_after.is_match(&words[at + 1..(at + 9).min(words.len())].join(" "))
    })
}

fn line(reading: &Reading<'_>) -> String {
    format!("{} {}", reading.line.clean, reading.line.norm_text)
}

pub(super) fn film_list(reading: &Reading<'_>, reply: &'static str) -> Cow<'static, str> {
    within(reply, &line(reading))
}

pub(super) fn family_film(
    reading: &Reading<'_>,
    replies: &'static [&'static str],
) -> Option<Cow<'static, str>> {
    let &[everyone, teens, youngest, younger, older] = replies else {
        return None;
    };
    let clean = &reading.line.clean;
    let said = format!("{clean} {}", reading.raw);
    if !FILM_WITH_CHILDREN
        .iter()
        .chain(["family", "teenager", "teenagers", "teen", "teens"].iter())
        .any(|word| said.contains(word))
    {
        return Some(within(everyone, &said));
    }
    if TEEN_VIEWER.is_match(clean) {
        return Some(Cow::Borrowed(teens));
    }
    let youngest_age = VIEWER_AGE
        .captures_iter(clean)
        .filter_map(|found| {
            (1..=3)
                .find_map(|index| found.get(index))
                .and_then(|digits| digits.as_str().parse::<u8>().ok())
        })
        .filter(|age| (1..=16).contains(age))
        .min();
    Some(Cow::Borrowed(match youngest_age {
        Some(1..=4) => youngest,
        Some(5..=7) => younger,
        _ => older,
    }))
}

pub(super) fn family_film_qualifier(
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<&'static str> {
    let anchor = memory.last_answer_anchor?;
    let ("entertainment", id @ ("film-family-funny" | "watch-family")) = anchor.card? else {
        return None;
    };
    if anchor.turn + 1 != memory.turns.len() {
        return None;
    }
    let clean = reading.clean.trim_end_matches(['.', '?', '!']).trim();
    let tail = clean
        .strip_prefix("something light that ")
        .or_else(|| clean.strip_prefix("anything light that "))?;
    let prior = memory.turns.last()?.user.to_lowercase();
    if FILM_WITH_CHILDREN.iter().any(|word| prior.contains(word)) || TEEN_VIEWER.is_match(&prior) {
        return None;
    }
    match tail {
        "isnt too long" | "is not too long" => Some(id),
        // The source sees the apostrophized verb after "that" as a new
        // thing if the conversation has never mentioned that token.
        "isn't too long"
            if memory.turns.iter().any(|turn| {
                SEEN_CONTRACTION.is_match(&turn.user) || SEEN_CONTRACTION.is_match(&turn.bot)
            }) =>
        {
            Some(id)
        }
        _ => None,
    }
}

pub(super) fn pick_mood(
    reading: &Reading<'_>,
    picks: &'static [(&'static str, &'static str)],
) -> Option<Cow<'static, str>> {
    let line = line(reading);
    let mood = mood(&line)?;
    picks
        .iter()
        .find_map(|&(kind, reply)| (kind == mood).then(|| within(reply, &line)))
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Medium {
    Book,
    Series,
    Podcast,
    Film,
}

pub(super) fn medium(line: &str) -> Option<Medium> {
    plain_words(line)
        .split_whitespace()
        .find_map(|word| match word {
            "book" | "books" | "novel" | "novels" | "read" | "reading" => Some(Medium::Book),
            "series" | "boxset" | "show" | "shows" | "telly" | "episode" | "episodes" => {
                Some(Medium::Series)
            }
            "podcast" | "podcasts" => Some(Medium::Podcast),
            "film" | "films" | "movie" | "movies" | "watch" | "watching" => Some(Medium::Film),
            _ => None,
        })
}

const PICK_ORDER: &[&str] = &[
    "light",
    "uplifting",
    "funny",
    "think",
    "gripping",
    "twist",
    "true",
    "short",
    "romance",
    "scary",
    "sad",
];

fn nearby_moods(mood: &str) -> &'static [&'static str] {
    match mood {
        "funny" => &["light", "uplifting", "twist"],
        "light" => &["uplifting", "funny", "short"],
        "uplifting" => &["light", "funny", "romance"],
        "sad" => &["romance", "think", "uplifting"],
        "think" => &["twist", "true", "gripping"],
        "gripping" => &["twist", "think", "true"],
        "twist" => &["gripping", "think", "true"],
        "true" => &["think", "gripping", "uplifting"],
        "short" => &["light", "funny", "romance"],
        "romance" => &["uplifting", "sad", "light"],
        "scary" => &["think", "gripping", "twist"],
        _ => &[],
    }
}

fn pick_titles(reply: &str) -> impl Iterator<Item = &str> {
    PICK_TITLE
        .captures_iter(reply)
        .filter_map(|captures| captures.get(1))
        .map(|title| title.as_str().trim_matches(crate::text::is_space))
}

fn pick_medium(text: &str) -> Option<&'static [(&'static str, &'static str)]> {
    const FILM: &[&str] = &[
        "film", "films", "movie", "movies", "watch", "watching", "netflix", "telly", "tv",
        "series", "cinema",
    ];
    const BOOK: &[&str] = &[
        "book",
        "books",
        "read",
        "reading",
        "novel",
        "novels",
        "author",
        "audiobook",
    ];
    let mut has_book = false;
    for word in PICK_WORD.find_iter(text) {
        let word = word.as_str();
        if FILM.contains(&word) {
            return Some(suggestions::FILM_MOODS);
        }
        has_book |= BOOK.contains(&word);
    }
    has_book.then_some(ENTERTAINMENT_BOOK_MOODS)
}

pub(super) fn other_pick(context: &Context<'_>) -> Option<String> {
    const SERIES_SIDE: &[&str] = &[
        "series",
        "sitcom",
        "episodes",
        "episode",
        "netflix",
        "iplayer",
        "prime",
        "tv",
        "telly",
        "channel",
        "streaming",
        "broadcast",
    ];
    let last = context
        .earlier
        .iter()
        .rev()
        .find(|turn| !turn.bot.is_empty())?;
    pick_titles(&last.bot).nth(1)?;
    let asked = context.reading.raw.to_lowercase();
    if PICK_WORD
        .find_iter(&asked)
        .any(|word| SERIES_SIDE.contains(&word.as_str()))
    {
        return None;
    }
    let table = pick_medium(&asked).or_else(|| pick_medium(&last.bot.to_lowercase()))?;
    let named: HashSet<String> = context
        .earlier
        .iter()
        .flat_map(|turn| pick_titles(&turn.bot))
        .map(str::to_lowercase)
        .collect();

    let before = context.earlier.split_last()?.1;
    let latest = before
        .last()
        .map_or_else(String::new, |turn| turn.user.to_lowercase());
    let earlier = before
        .iter()
        .map(|turn| turn.user.as_str())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let mut moods = Vec::with_capacity(PICK_ORDER.len());
    for part in [&asked, &latest, &earlier] {
        let words: HashSet<_> = PICK_WORD
            .find_iter(part)
            .map(|word| word.as_str())
            .collect();
        for &(mood, _) in table {
            if words.contains(mood) && !moods.contains(&mood) {
                moods.push(mood);
                for &near in nearby_moods(mood) {
                    if table.iter().any(|&(name, _)| name == near) && !moods.contains(&near) {
                        moods.push(near);
                    }
                }
            }
        }
    }
    let mut seen = HashSet::new();
    for mood in moods
        .into_iter()
        .chain(PICK_ORDER.iter().copied())
        .chain(table.iter().map(|&(mood, _)| mood))
    {
        if !seen.insert(mood) {
            continue;
        }
        let Some(&(_, reply)) = table.iter().find(|&&(name, _)| name == mood) else {
            continue;
        };
        let mut titles = pick_titles(reply).peekable();
        if titles.peek().is_some() && !titles.any(|title| named.contains(&title.to_lowercase())) {
            return Some(format!(
                "Something different then: {}",
                PICK_LEAD.replace(reply, "")
            ));
        }
    }
    None
}

#[cfg(test)]
#[test]
fn the_next_pick_uses_history_without_repeating_any_title() {
    use crate::bot::Exchange;
    let prior = [Exchange {
        user: "Suggest a book".to_owned(),
        bot: "*The Thursday Murder Club*, *The Midnight Library*".to_owned(),
        module: "answers",
    }];
    let books = Reading::new("Any other books?");
    assert_eq!(
        other_pick(&Context::new(&books, &prior)).as_deref(),
        Some(
            "Something different then: *Klara and the Sun* (what makes someone human), *The Remains of the Day* (a whole life looked back on) or *Sapiens* if you would rather have ideas than a story."
        )
    );
    assert_eq!(other_pick(&Context::new(&books, &[])), None);
    let series = Reading::new("Any other series?");
    assert_eq!(other_pick(&Context::new(&series, &prior)), None);
}

pub(super) fn asks_for_a_film(reading: &Reading<'_>) -> bool {
    own_texts(reading).any(|text| {
        plain_words(text)
            .split_whitespace()
            .any(|word| suggestions::FILM_ASK_WORDS.contains(&word))
    }) || refuses_scare(reading)
}

fn mood_shelf(kind: &str) -> Option<&'static str> {
    suggestions::FILM_MOODS
        .iter()
        .find_map(|&(mood, reply)| (mood == kind).then_some(reply))
}

fn gentle_list(context: &Context<'_>) -> Option<&'static str> {
    let prior = context
        .earlier
        .iter()
        .rev()
        .find(|turn| turn.module == "answers" && !turn.bot.is_empty());
    if prior.is_none_or(|turn| plain_words(&turn.bot) != plain_words(suggestions::GENTLE_LIST)) {
        return Some(suggestions::GENTLE_LIST);
    }
    let own = own_texts(context.reading).collect::<Vec<_>>().join(" ");
    match medium(&own) {
        Some(Medium::Book | Medium::Podcast | Medium::Series) => Some(suggestions::GENTLE_LIST),
        Some(Medium::Film) | None => mood_shelf("funny"),
    }
}

fn title_case(text: &str) -> String {
    let mut after_letter = false;
    text.chars()
        .map(|character| {
            let output = if after_letter {
                character
            } else {
                character.to_ascii_uppercase()
            };
            after_letter = character.is_ascii_alphabetic();
            output
        })
        .collect()
}

fn a_similar_film(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let reading = context.reading;
    let clean = &reading.line.clean;
    let own = if reading.raw.is_empty() {
        clean
    } else {
        reading.raw
    }
    .to_lowercase();
    if FRESH_LIST.is_match(&own)
        && !LIKE_VERB.is_match(&own)
        && !books::names_a_book(&own)
        && !entertainment_titles::names_a_work(&own)
    {
        let in_chat = context
            .earlier
            .iter()
            .rev()
            .find_map(|turn| match read_mood(&turn.user) {
                Some((mood, false)) => Some(mood),
                _ => None,
            });
        return Some(
            in_chat
                .and_then(mood_shelf)
                .map_or(Cow::Borrowed(suggestions::EASY_FILM_LIST), |reply| {
                    within(reply, clean)
                }),
        );
    }
    let texts = [clean.to_lowercase(), reading.line.norm_text.to_lowercase()];
    if let Some(reply) = texts
        .iter()
        .find_map(|text| entertainment_titles::following_title(text))
    {
        return Some(Cow::Borrowed(reply));
    }
    for text in &texts {
        let Some(found) = LIKED_TITLE.captures(text).and_then(|found| found.get(1)) else {
            continue;
        };
        let found = found.as_str();
        let title = CUT_LIKED_TITLE
            .find(found)
            .map_or(found, |cut| &found[..cut.start()])
            .trim_matches([' ', '.', '?', '!']);
        if !title.is_empty() {
            let title = title_case(title);
            return Some(Cow::Owned(format!(
                "I don't know *{title}* well enough to match it. Tell me what you liked about it: the story, the jokes, or how tense it was, and I'll pick something close."
            )));
        }
    }
    (FILM_ASK.is_match(&own) && FILM_OR_MOVIE.is_match(&own))
        .then_some(Cow::Borrowed(suggestions::ASK_BACK))
}

pub(super) fn suggestion(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    match id {
        "film-like" => a_similar_film(context),
        "film-nothing-scary" => gentle_list(context).map(Cow::Borrowed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_films_use_the_youngest_age_and_decline_symptom_watch() {
        use super::super::{guards::Context, rows};
        let family = rows()
            .find(|row| row.id == "watch-family")
            .expect("family film card");
        let reading = Reading::new("What is a film for my children age 6 and age 4?");
        let context = Context::new(&reading, &[]);
        assert!(
            family
                .answer(&context)
                .is_some_and(|reply| reply.starts_with("For a four-year-old"))
        );
        let reading = Reading::new("My kids fell over. What should I watch for?");
        let context = Context::new(&reading, &[]);
        assert!(!family.guard.passes(&context, family.id, family.key[0]));
    }

    #[test]
    fn film_limits_keep_only_twelve_angry_men_when_equality_is_allowed() {
        let reply = "Drama: *12 Angry Men*, *Arrival* or *Ex Machina*.";
        assert_eq!(within(reply, "up for a drama"), reply);
        assert!(within(reply, "under 96 minutes").contains("the shortest is *12 Angry Men* at 96"));
        let exact = within(reply, "at most 96 minutes");
        assert!(exact.contains("*12 Angry Men* (96 minutes)"));
        assert!(!exact.contains("*Arrival*"));
    }
}
