//! Ask about a short, already completed part of the person's day.

use crate::bot::modules::the_detail_the_thread_adds::heavy;
use crate::bot::{Exchange, Memory, Reading};
use crate::text::is_space;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($value).expect(stringify!($name)));
    };
}

macro_rules! context_pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($value).expect(stringify!($name)));
    };
}

#[path = "small_day_data.rs"]
mod data;
use super::thing_done::{HARM_ANYWHERE, HARM_NAMED, NEGATIVE_LINE, QUESTION_OPENERS, regret};
use data::*;

fn words(line: &str) -> impl DoubleEndedIterator<Item = &str> {
    line.split(is_space).filter(|word| !word.is_empty())
}

fn flattened(line: &str) -> String {
    words(line).collect::<Vec<_>>().join(" ").to_lowercase()
}

fn matches(pattern: &fancy_regex::Regex, line: &str) -> bool {
    pattern.is_match(line).unwrap_or(false)
}

fn had_a_reply(line: &str, pools: &[&[&str]]) -> bool {
    let neutral = |line: &str| flattened(&line.replace(['—', '–'], "-"));
    let line = neutral(line);
    !line.is_empty()
        && pools.iter().flat_map(|pool| pool.iter()).any(|&item| {
            let item = neutral(item);
            !item.is_empty() && line.starts_with(&item)
        })
}

fn someone_elses_day(line: &str) -> bool {
    A_CHILDHOOD_MEMORY.is_match(line)
        || SOMEONE_ELSE_NAMED.find(line).is_some_and(|other| {
            !A_VISIT.is_match(line)
                && THEIR_OWN_DOING
                    .find(line)
                    .is_none_or(|own| other.start() < own.start())
        })
}

fn happened(line: &str, doing: &fancy_regex::Match<'_>) -> bool {
    words(&line[..doing.start()])
        .chain(words(&line[doing.end()..]))
        .any(|word| word.chars().count() > 2 && !A_BARE_MODIFIER.contains(&word))
}

fn place_secured(line: &str) -> bool {
    A_THING_GOT_AHEAD.is_match(line)
        && !GOT_AHEAD_GONE.is_match(line)
        && !GOT_AHEAD_NOT_A_DATE.is_match(line)
}

fn problem_stated(line: &str) -> bool {
    static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9]+").unwrap());
    let bare = WORD
        .find_iter(line)
        .map(|word| word.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    !bare.is_empty() && (PROBLEM_STATED.is_match(&bare) || SOMETHING_UP.is_match(&bare))
}

fn harm(line: &str) -> bool {
    SENSE_GONE.is_match(line) || matches(&MISHAP_ANYWHERE, line) && !MISHAP_OVER.is_match(line)
}

fn reported_elsewhere(line: &str) -> bool {
    !A_ASKING_WORD.is_match(line)
        && A_REPORTED_NOT_LIVED.is_match(line)
        && (A_NEWS_IS_WEATHER.is_match(line) || A_NEWS_IS_A_CHANGE.is_match(line))
}

fn standing_state(line: &str, has_day: bool) -> bool {
    if has_day || !STATE_NOT_A_DOING.is_match(line) {
        return false;
    }
    let no_state = STATE_NOT_A_DOING.replace_all(line, " ");
    let no_copula = BARE_STATE.replace_all(&no_state, " ");
    !matches(&PAST_DOING, &no_copula)
}

fn done_to_them(line: &str) -> bool {
    if !DONE_TO_ME.is_match(line) || CRACKED_A_JOKE.is_match(line) {
        return false;
    }
    if SMASHED.is_match(line)
        && (!SMASHED_OF_A_THING.is_match(line)
            || A_GROUP_OF_PEOPLE.is_match(line) && A_NIGHT_OUT.is_match(line))
    {
        return false;
    }
    !A_PERSON_OF_THEIRS.is_match(line)
        || A_PERSON_LOST.is_match(line)
        || A_THING_CARRIED_OFF.is_match(line)
}

fn detail_after_hard_news(line: &str, last: Option<&Exchange>) -> bool {
    if !_A_DETAIL_OPENING.is_match(line) {
        return false;
    }
    let Some(last) = last else {
        return false;
    };
    if had_a_reply(&last.bot, &[A_SMALL_DAY]) {
        return false;
    }
    if _A_SYMPATHY.is_match(&last.bot.to_lowercase()) {
        return true;
    }
    let before = flattened(&last.user);
    HARM_ANYWHERE.is_match(&before)
        || NEGATIVE_LINE.is_match(&before)
        || matches(&BAD_DAY_ANYWAY, &before)
        || matches(&A_DAY_WENT_WRONG, &before)
        || _A_BODY_UNWELL.is_match(&before)
}

fn previously_bad(memory: &Memory) -> bool {
    let Some(last) = memory
        .turns
        .iter()
        .rev()
        .find(|turn| !turn.bot.trim().is_empty())
    else {
        return false;
    };
    if had_a_reply(
        &last.bot,
        &[
            LOW_DAY_REPLIES,
            NEGATIVE,
            A_BAD_TIME_TOLD,
            A_THING_CAME_OUT_WRONG,
            A_PLAN_THAT_FELL_THROUGH,
            A_DAY_ASKED_PLAIN,
        ],
    ) {
        return true;
    }
    let head: String = last.bot.chars().take(80).collect();
    if BOT_WAS_SORRY.is_match(&head) {
        return true;
    }
    day_said_badly(&last.user)
}

fn hard_week(memory: &Memory) -> bool {
    memory
        .turns
        .iter()
        .rev()
        .take(3)
        .enumerate()
        .any(|(offset, turn)| {
            let said = flattened(&turn.user);
            super::loss_reader::reads_as_loss(&said)
                || offset < 2
                    && (heavy::bad_news(&said)
                        || heavy::sudden_loss(&said)
                        || HARD_WEEK.is_match(&said))
        })
}

fn verdict(line: &str, memory: &Memory) -> bool {
    let Some(found) = GOOD_VERDICT.captures(line) else {
        return false;
    };
    let mut subject = words(&found["subject"]);
    let first = subject.next().unwrap_or_default();
    let head = subject.next().unwrap_or(first);
    !GRIEVANCE_PEOPLE.contains(&head)
        && !OTHERS.contains(&head)
        && !NOT_A_VERDICT_SUBJECT.contains(&head)
        && !hard_week(memory)
}

fn good_time(line: &str, memory: &Memory) -> bool {
    if line.contains(" but ") || line.contains(" until ") {
        return false;
    }
    if [
        &*GOOD_DAY_OURS,
        &*GOOD_DAY_WENT,
        &*GOOD_DAY_BARE,
        &*GOOD_TIME_IT,
    ]
    .iter()
    .any(|pattern| pattern.is_match(line))
        || verdict(line, memory)
    {
        return true;
    }
    if words(line)
        .next()
        .is_some_and(|word| QUESTION_OPENERS.contains(&word))
    {
        return false;
    }
    static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z]+").unwrap());
    GOOD_TIME_OURS.is_match(line)
        || GOOD_TIME_SUBJECT.is_match(line)
            && WORD
                .find_iter(line)
                .any(|word| GOOD_TIME_WORDS.contains(&word.as_str()))
}

fn shape(line: &str) -> Option<bool> {
    let count = words(line).take(15).count();
    if count == 0 || count > 14 || line.contains('?') || !A_DAY_SUBJECT.is_match(line) {
        return None;
    }
    let doing = PAST_DOING.find(line).ok().flatten()?;
    let has_day = DAY_PAST.is_match(line);
    if !has_day && (count > 10 || !happened(line, &doing)) {
        return None;
    }
    if [
        &*A_BEING_DONE_NOW,
        &*A_DECISION_NOT_A_DAY,
        &*A_WISH,
        &*AN_ATTEMPT_AT_SOMEONE,
        &*A_BETRAYAL_FOUND_OUT,
        &*NOT_A_DAY_DONE,
        &*NOT_A_DAY_SAID,
        &*A_SYMPTOM_PHRASE,
        &*NEGATIVE_LINE,
        &*HARM_NAMED,
        &*A_BAD_THING_NAMED,
        &*A_HAND_OUT,
        &*A_NAME_IS_NOT_A_DAY,
        &*RESULT_WENT_WRONG,
        &*PLAN_THAT_FELL_THROUGH,
    ]
    .iter()
    .any(|pattern| pattern.is_match(line))
        || matches(&A_DAY_WENT_WRONG, line)
        || someone_elses_day(line)
        || regret(line)
        || harm(line)
        || reported_elsewhere(line)
        || standing_state(line, has_day)
    {
        return None;
    }
    Some(has_day)
}

fn account(reading: &Reading<'_>, memory: &Memory, has_day: bool) -> bool {
    let line = &reading.line.clean;
    if reading.features.suggestion.is_some()
        || reading.features.polarity == -1
        || [
            &*A_LOOKING_BACK,
            &*THEIR_NAME_AGE,
            &*THEIR_NAME,
            &*SOMEONE_DONE_WRONG,
        ]
        .iter()
        .any(|pattern| pattern.is_match(line))
        || [&*A_HURT_MYSELF, &*BAD_DAY_ANYWAY]
            .iter()
            .any(|pattern| matches(pattern, line))
        || detail_after_hard_news(line, memory.turns.last())
        || place_secured(line)
        || problem_stated(line)
        || done_to_them(line)
        || !has_day && previously_bad(memory)
        || A_MEDS.is_match(line)
            && A_MEDS_CHANGE.is_match(line)
            && A_MEDS_MINE.is_match(line)
            && !A_MEDS_SYMPTOM.is_match(line)
    {
        return false;
    }
    !good_time(line, memory)
}

fn day_said_badly(line: &str) -> bool {
    let said = flattened(line);
    let said = said.trim_matches([' ', '.', '!', ',']);
    !said.is_empty()
        && !said.contains('?')
        && words(said).take(13).count() <= 12
        && A_DAY_SAID_BADLY.is_match(said)
}

fn bad_time(reading: &Reading<'_>, memory: &Memory, has_day: bool) -> bool {
    let line = &reading.line.clean;
    let detail = memory
        .turns
        .iter()
        .rev()
        .find(|turn| !turn.bot.trim().is_empty())
        .is_some_and(|last| {
            if has_day {
                detail_after_hard_news(line, memory.turns.last())
            } else {
                day_said_badly(&last.user)
            }
        });
    reading.features.polarity == -1
        || detail
        || A_DOING_GONE_BADLY.is_match(line)
        || A_THING_SKIPPED.is_match(line) && !A_MEAL_SKIPPED.is_match(line)
        || A_DAY_SAID_BADLY.is_match(line)
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = &reading.line.clean;
    let has_day = shape(line)?;
    if bad_time(reading, memory, has_day) {
        return Some(super::choose_recent(A_BAD_TIME_TOLD, memory, line).into());
    }
    if !account(reading, memory, has_day)
        || !super::trouble::neutral_tone(reading)
        || memory
            .turns
            .last()
            .is_some_and(|last| had_a_reply(&last.bot, &[A_SMALL_DAY]))
    {
        return None;
    }
    Some(super::choose_recent(A_SMALL_DAY, memory, line).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asks_about_a_plain_account_using_the_source_pool() {
        let memory = Memory::default();
        for (line, wanted) in [
            (
                "im considering a used car",
                "Good to hear. What was the best bit?",
            ),
            (
                "i watched the final together",
                "Sounds alright. Anything good in it?",
            ),
            ("i got my hair cut", "Nice one. How did the rest of it go?"),
        ] {
            assert_eq!(
                reply(&Reading::new(line), &memory).as_deref(),
                Some(wanted),
                "{line}"
            );
        }
    }

    #[test]
    fn declines_on_harm_and_details_of_a_previous_problem() {
        let mut memory = Memory::default();
        for line in [
            "i got glass in my foot",
            "my meds have changed",
            "i am going to look at used cars next month",
        ] {
            assert_eq!(reply(&Reading::new(line), &memory), None, "{line}");
        }

        let reading = Reading::new("it was lifting the shopping");
        assert!(reply(&reading, &memory).is_some());
        memory.turns.push(Exchange {
            user: "i pulled my back".into(),
            bot: "Have you had it looked at?".into(),
            module: "answers",
        });
        assert_eq!(reply(&reading, &memory), None);
    }
}
