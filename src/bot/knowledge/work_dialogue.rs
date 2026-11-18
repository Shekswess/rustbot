//! The work cards' checks for whose situation and which part of it was asked about.

use std::borrow::Cow;

use crate::bot::{Exchange, Reading};
use fancy_regex::Regex as Lookaround;
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

#[path = "work_catalogue.rs"]
mod catalogue;
#[path = "work_dialogue_data.rs"]
mod data;
#[path = "work_employment.rs"]
mod employment;
#[path = "work_hours.rs"]
mod hours;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    CRY_WORD,
    FIRST_PERSON,
    ANOTHER_PERSON,
    NOBODY_ELSE,
    LOST_IN_WHAT_I_WANT,
    SUIT,
    THE_WORK_NAMED,
    SCHOOL_LEAVER,
    JOB_PLACE_LANDMARK,
    JOB_PLACE_AS_A_CONSTRAINT,
);
static A_NAMED_WAR: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::A_NAMED_WAR).expect("named war"));
static PUNCTUATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^a-z0-9' ]+").expect("work-card punctuation"));
static WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-z']+").expect("words around crying"));
static SAY_NO_FOOD: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        data::SAY_NO_FOOD
            .iter()
            .map(|food| format!(r"\b{}\b", regex::escape(food))),
    )
    .expect("food and drink refusals")
});
static SMELLS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::SMELLS
        .iter()
        .map(|(pattern, _, _)| Regex::new(pattern).expect("named smell"))
        .collect()
});

pub(crate) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    let text = said_text(reading);
    match id {
        "two-people-who-clash" => !A_NAMED_WAR.is_match(text).unwrap_or(false),
        "crying-at-work" => tears(text) != Some(Tears::Theirs),
        "someone-crying" => tears(text) != Some(Tears::Mine),
        "commute-long" => {
            let said = cleaned(text);
            !uses(&said, &["each way"]) || uses(&said, data::EACH_WAY_TIME)
        }
        "dress-for-work" => {
            let said = cleaned(text);
            uses(&said, data::A_JOB_IN_LINE)
                || (!uses(&said, data::A_DRESS_CODE_IN_LINE)
                    && !uses(&said, data::AN_OCCASION_IN_LINE))
        }
        "wrong-job" => {
            let lower = text.to_lowercase();
            let said = lower
                .split(crate::text::is_space)
                .filter(|word| !word.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            !LOST_IN_WHAT_I_WANT.is_match(&said)
                && (!SUIT.is_match(&said) || THE_WORK_NAMED.is_match(&said))
                && !SCHOOL_LEAVER.is_match(&said)
        }
        "write-a-reference" => uses(&cleaned(text), data::WRITING_A_REFERENCE),
        "noisy-office" => {
            let clean = reading.line.clean.to_lowercase();
            !uses(&clean, data::WITH_SOMEONE_AT_WORK) || uses(&clean, data::NOISE_WORDS)
        }
        "job-search-plan" => job_place(&cleaned(text)).is_none(),
        _ => employment::passes(id, text),
    }
}

pub(crate) fn answer(
    id: &str,
    reading: &Reading<'_>,
    turns: &[Exchange],
) -> Option<Cow<'static, str>> {
    if id == "what-a-job-pays" {
        return catalogue::pay(reading);
    }
    if id == "holiday-entitlement" {
        return Some(hours::holiday(reading, turns));
    }
    registered(id, reading, turns).map(Cow::Borrowed)
}

fn registered(id: &str, reading: &Reading<'_>, turns: &[Exchange]) -> Option<&'static str> {
    let text = said_text(reading);
    let reply = match id {
        "interview-prep" => {
            let said = cleaned(text);
            if uses(&said, data::SHAPE_OF_THE_INTERVIEW) {
                data::INTERVIEW_SHAPE
            } else if uses(&said, data::NERVES) {
                data::INTERVIEW_NERVOUS
            } else {
                data::INTERVIEW_PLAIN
            }
        }
        "say-no-politely" => {
            if SAY_NO_FOOD.is_match(&text.to_lowercase()) {
                data::REFUSE_FOOD
            } else {
                data::REFUSE_WORK
            }
        }
        "first-day" => first_day(&cleaned(text))?,
        "defensive-talk" => {
            let revisiting = || {
                let earlier = turns[turns.len().saturating_sub(3)..]
                    .iter()
                    .map(|turn| turn.user.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                uses(&cleaned(&earlier), data::REVIEW_ANCHOR)
            };
            return (uses(&cleaned(text), data::REVIEW_ANCHOR) || revisiting())
                .then_some(data::DEFENSIVE);
        }
        "colleague-smell" => smell(&reading.line.clean.to_lowercase()),
        "noisy-office" => quiet(&reading.line.clean.to_lowercase()),
        "job-in-a-place" => job_place(&cleaned(text))?,
        "work-word" => catalogue::definition(reading)?,
        "what-a-job-is-like" => catalogue::day(reading)?,
        "going-back-to-work" => hours::going_back(text),
        "pay-and-hours" => hours::pay_and_hours(text)?,
        _ => return employment::reply(id, reading),
    };
    Some(reply)
}

fn said_text<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn cleaned(text: &str) -> String {
    PUNCTUATION
        .replace_all(&text.to_lowercase(), " ")
        .into_owned()
}

fn uses(said: &str, phrases: &[&str]) -> bool {
    phrases
        .iter()
        .any(|phrase| phrase_offset(said, phrase).is_some())
}

fn phrase_offset(said: &str, phrase: &str) -> Option<usize> {
    let bytes = said.as_bytes();
    said.match_indices(phrase).find_map(|(start, matching)| {
        let before_word = start
            .checked_sub(1)
            .and_then(|at| bytes.get(at))
            .is_some_and(u8::is_ascii_alphanumeric);
        let after_word = bytes
            .get(start + matching.len())
            .is_some_and(u8::is_ascii_alphanumeric);
        (!before_word && !after_word).then_some(start)
    })
}

fn job_place(said: &str) -> Option<&'static str> {
    if JOB_PLACE_AS_A_CONSTRAINT.is_match(said) {
        return None;
    }
    data::JOB_PLACES.iter().find_map(|&(places, reply)| {
        places
            .iter()
            .any(|place| {
                phrase_offset(said, place)
                    .is_some_and(|offset| !JOB_PLACE_LANDMARK.is_match(&said[..offset]))
            })
            .then_some(reply)
    })
}

#[derive(PartialEq, Eq)]
enum Tears {
    Mine,
    Theirs,
}

fn tears(text: &str) -> Option<Tears> {
    let lower = text.to_lowercase();
    let words: Vec<_> = WORD.find_iter(&lower).map(|word| word.as_str()).collect();
    for (index, word) in words.iter().enumerate() {
        if !CRY_WORD.is_match(word)
            && !words
                .get(index + 1)
                .is_some_and(|next| CRY_WORD.is_match(&format!("{word} {next}")))
        {
            continue;
        }
        let before = &words[index.saturating_sub(4)..index];
        let near = before.join(" ");
        let another = ANOTHER_PERSON.is_match(&near);
        if matches!(before.last(), Some(&("me" | "us" | "myself")))
            || NOBODY_ELSE.is_match(&near)
            || (FIRST_PERSON.is_match(&near) && !another)
        {
            return Some(Tears::Mine);
        }
        if another {
            return Some(Tears::Theirs);
        }
    }
    None
}

fn first_day(said: &str) -> Option<&'static str> {
    if uses(said, data::NERVES)
        && uses(said, data::ANOTHER_PERSON_WORDS)
        && !uses(said, data::FIRST_DAY_ASKS)
    {
        return None;
    }
    if uses(said, data::NEW_START) {
        return Some(data::DAY_LOST);
    }
    Some(
        match (
            uses(said, data::FIRST_DAY_QUESTIONS),
            uses(said, data::GOOD_NEWS),
        ) {
            (true, true) => data::DAY_QUESTIONS_NEWS,
            (true, false) => data::DAY_QUESTIONS,
            (false, true) => data::DAY_LIST_NEWS,
            (false, false) => data::DAY_LIST,
        },
    )
}

fn smell(clean: &str) -> &'static str {
    let home = uses(clean, data::PEOPLE_AT_HOME);
    if let Some(index) = SMELLS.iter().position(|pattern| pattern.is_match(clean)) {
        let (_, at_home, at_work) = data::SMELLS[index];
        if home { at_home } else { at_work }
    } else if home {
        data::SMELL_HOME_DEFAULT
    } else {
        data::SMELL_WORK_DEFAULT
    }
}

fn quiet(clean: &str) -> &'static str {
    if uses(clean, data::BABY_AT_HOME) {
        data::QUIET_BABY
    } else if !uses(clean, data::WITH_SOMEONE_AT_WORK) {
        data::QUIET_ROOM
    } else if uses(clean, data::PLAYING_NOISE) {
        data::QUIET_RADIO
    } else if uses(
        clean,
        &[
            "phone",
            "calls",
            "call",
            "talking",
            "talks",
            "talks loudly",
            "chatter",
        ],
    ) {
        data::QUIET_VOICE
    } else {
        data::QUIET_NOISE
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/work_dialogue.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn employer_advice_stands_down_for_a_landmark_and_childcare_hours() {
        for input in [
            "How do I get a job near the school?",
            "How do I get a job that fits round school pickup?",
            "How do I get a job after being in hospital?",
        ] {
            let reading = Reading::new(input);
            assert!(passes("job-search-plan", &reading));
            assert!(registered("job-in-a-place", &reading, &[]).is_none());
        }
        let reading = Reading::new("How do I get a job in a charity shop?");
        assert!(!passes("job-search-plan", &reading));
        assert!(
            registered("job-in-a-place", &reading, &[])
                .is_some_and(|reply| reply.starts_with("Volunteer"))
        );
    }

    #[test]
    fn work_guards_and_replies_reproduce_the_pinned_inputs() {
        for &fixtures::Case(id, raw, guard, reply) in fixtures::CASES {
            let reading = Reading::new(raw);
            if let Some(expected) = guard {
                assert_eq!(passes(id, &reading), expected, "{id}: {raw}");
            } else {
                assert_eq!(registered(id, &reading, &[]), reply, "{id}: {raw}");
            }
        }
        for &(id, raw, earlier, guard, reply) in fixtures::EXTRA {
            let reading = Reading::new(raw);
            let turns = earlier
                .iter()
                .map(|&user| Exchange {
                    user: user.into(),
                    bot: String::new(),
                    module: "answers",
                })
                .collect::<Vec<_>>();
            if let Some(expected) = guard {
                assert_eq!(passes(id, &reading), expected, "{id}: {raw}");
            }
            assert_eq!(
                registered(id, &reading, &turns),
                reply,
                "{id}: {raw} after {earlier:?}"
            );
        }
    }
}
