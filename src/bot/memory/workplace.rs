//! The place somebody named for work, in the frame of their previous job.

use super::occupations::Work;
use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::sync::LazyLock;

use super::workplace_data as data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    I_WORK,
    NOT_MINE,
    A_QUESTION,
    NOT_A_PLACE,
    A_PLACE,
    WITH_THE_DETAIL,
    USED_TO
);
static PLACE_CLAUSE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:and|but|because|so|when|while|although|though|which|where|if)\b")
        .expect("workplace clause")
});
static PLACE_FILLER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\s+(?:anyway|now|though|too|still)$").expect("workplace filler")
});

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let text = reading.raw;
    let (prep, place) = named_place(text)?;
    let job = memory.occupations.earlier_job(text)?;
    let start = capitalize(&with_article(job));
    if prep.eq_ignore_ascii_case("from")
        || place.eq_ignore_ascii_case("home")
        || place.eq_ignore_ascii_case("the house")
    {
        let ending = compose::pick(data::FROM_HOME_TAILS, memory, text);
        return Some(format!("{start} working from home - {ending}"));
    }
    let ending = compose::pick(data::TAILS, memory, text);
    Some(format!(
        "{start} {} {place} - {ending}",
        prep.to_lowercase()
    ))
}

pub(super) fn observe(work: &mut Work, reading: &Reading<'_>) {
    work.observe(reading.raw, &reading.line.plain);
}

fn named_place(text: &str) -> Option<(&str, String)> {
    let said = text.trim_matches(crate::text::is_space);
    let found = I_WORK.captures(said)?;
    if said.ends_with('?')
        || A_QUESTION.is_match(said)
        || USED_TO.is_match(said)
        || NOT_MINE.is_match(said)
    {
        return None;
    }
    let rest = found
        .name("rest")?
        .as_str()
        .trim_matches(crate::text::is_space);
    if NOT_A_PLACE.is_match(rest) {
        return None;
    }
    let place = A_PLACE.captures(rest)?;
    let place_raw = place.name("place")?.as_str();
    if WITH_THE_DETAIL.is_match(place_raw) {
        return None;
    }
    let value = clean_place(place_raw);
    let tail = &rest[place.get(0)?.end()..];
    let tail = tail.trim_matches([' ', ',', '.', ';', ':', '!', '?', '\'', '’']);
    (value.chars().count() >= 3 && words(tail).take(4).count() <= 3)
        .then_some((place.name("prep")?.as_str(), value))
}

fn clean_place(place: &str) -> String {
    let before = PLACE_CLAUSE
        .find(place)
        .map_or(place, |join| &place[..join.start()]);
    let before = before.trim_matches(crate::text::is_space);
    let before = before.trim_end_matches(|ch| {
        crate::text::is_space(ch) || matches!(ch, ',' | '.' | ';' | ':' | '!' | '?' | '\'' | '’')
    });
    let without_filler = PLACE_FILLER.replace(before, "");
    if words(&without_filler).take(6).count() > 5 {
        words(&without_filler).take(5).collect::<Vec<_>>().join(" ")
    } else {
        without_filler
            .trim_matches(crate::text::is_space)
            .to_owned()
    }
}

pub(super) fn words(line: &str) -> impl Iterator<Item = &str> {
    line.split(crate::text::is_space)
        .filter(|word| !word.is_empty())
}

pub(super) fn with_article(job: &str) -> String {
    let article = if job.to_lowercase().starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    };
    format!("{article} {job}")
}

pub(super) fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first
        .to_uppercase()
        .chain(chars.as_str().to_lowercase().chars())
        .collect()
}

#[cfg(test)]
#[path = "../../../tests/fixtures/workplace.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use crate::bot::Bot;

    #[test]
    fn workplace_and_duration_followups_use_the_recorded_job() {
        for session in super::fixtures::SESSIONS {
            let mut bot = Bot::new(Some("0")).unwrap();
            for super::fixtures::Turn(prompt, workplace, duration) in *session {
                let turn = bot.turn(prompt);
                for (hook, expected) in [("my_work", workplace), ("years_in", duration)] {
                    let proposal = turn
                        .candidates
                        .iter()
                        .find(|candidate| candidate.module == hook)
                        .map(|candidate| candidate.reply.as_str());
                    assert_eq!(proposal, *expected, "{hook}: {prompt}");
                    if expected.is_some() {
                        assert_eq!(turn.module, Some(hook), "{prompt}");
                    }
                }
            }
        }
    }
}
