//! School-break replies with local calendar dates and a recent regional context.

use crate::bot::{Exchange, Reading};
use jiff::{Zoned, civil::Date};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "school_break_data.rs"]
mod data;
use data::*;

#[derive(Clone, Copy)]
pub(super) enum Break {
    Summer,
    Winter,
    Easter,
    HalfTerm,
    BackToSchool,
    General,
}

static NATION_PATTERNS: LazyLock<Vec<(Regex, Option<&str>)>> = LazyLock::new(|| {
    NATIONS
        .iter()
        .map(|(names, year)| {
            let alternatives = names
                .iter()
                .map(|name| regex::escape(name))
                .collect::<Vec<_>>()
                .join("|");
            let pattern =
                Regex::new(&format!(r"\b(?:{alternatives})\b")).expect("school nation names");
            (pattern, *year)
        })
        .collect()
});

static ASK_LENGTH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\bhow long (?:is|are|do|does|will)\b|\bhow many weeks off\b|\bhow long (?:a|an|the)\b|\blast(?:s|ed)?\b").unwrap()
});
static ASK_WAIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:how (?:long|many)|days|weeks|till|until|countdown|left)\b").unwrap()
});

fn nation_year(text: &str) -> Option<Option<&'static str>> {
    let text = text.to_lowercase();
    NATION_PATTERNS
        .iter()
        .find_map(|(nation, year)| nation.is_match(&text).then_some(*year))
}

fn nation_reply<'a>(text: &str, prior: impl DoubleEndedIterator<Item = &'a str>) -> Option<String> {
    nation_year(text)
        .or_else(|| prior.rev().take(6).find_map(nation_year))
        .flatten()
        .map(|year| format!("{year}{DATES_ARE_LOCAL}"))
}

fn next_date(today: Date, month: i8, day: i8) -> Date {
    let this = Date::new(today.year(), month, day).expect("valid school break date");
    if this >= today {
        this
    } else {
        Date::new(today.year() + 1, month, day).unwrap_or(this)
    }
}

fn date_said(date: Date, today: Date) -> String {
    let weekday = date.strftime("%A");
    let month = date.strftime("%B");
    let day = date.day();
    if date.year() == today.year() {
        format!("{weekday} {day} {month}")
    } else {
        format!("{weekday} {day} {month} {}", date.year())
    }
}

fn this_or_next(date: Date, today: Date) -> String {
    let words = date_said(date, today);
    if date.year() == today.year() {
        format!("this year that is {words}")
    } else {
        format!("the next one is {words}")
    }
}

fn wait_days(today: Date, until: Date) -> String {
    let days = today.duration_until(until).as_secs() / 86400;
    if days <= 10 {
        format!("{days} days")
    } else {
        format!("{days} days (about {} weeks)", (days + 3) / 7)
    }
}

fn asks_wait(text: &str) -> bool {
    !ASK_LENGTH.is_match(text) && ASK_WAIT.is_match(text)
}

fn summer(today: Date, clean: &str) -> String {
    let date = next_date(today, 7, 21);
    if ASK_LENGTH.is_match(clean) {
        format!(
            "Six weeks off: {SUMMER}. The exact two days are on your school's term dates, because a school can start back a day or two either side."
        )
    } else if asks_wait(clean) {
        format!(
            "{} until the summer holidays: {SUMMER}, so they break up on {}. The exact day is on your school's term dates, and it can sit a day or two either side.",
            wait_days(today, date),
            date_said(date, today)
        )
    } else {
        format!(
            "{SUMMER} - {}. The exact day is on your school's term dates, and it can sit a day or two either side.",
            this_or_next(date, today)
        )
    }
}

fn winter(today: Date, clean: &str) -> String {
    let date = next_date(today, 12, 19);
    let intro = this_or_next(date, today);
    if asks_wait(clean) {
        format!(
            "{} until the Christmas holidays: schools usually break up on about 19 December, and {intro}, then go back the first Tuesday or Wednesday in January.",
            wait_days(today, date)
        )
    } else {
        format!(
            "{WINTER}: the break-up is usually about 19 December, {intro}, and back on the first Tuesday or Wednesday in January. Your school's term dates give the exact two days."
        )
    }
}

fn back_to_school(clean: &str) -> Cow<'static, str> {
    for (words, reply) in TERM_STARTS {
        if words.iter().any(|word| clean.contains(word)) {
            return Cow::Owned(format!("{reply}. {STARTS_LOCAL}"));
        }
    }
    Cow::Borrowed(RETURN)
}

fn general(today: Date, clean: &str) -> String {
    let summer = next_date(today, 7, 21);
    let winter = next_date(today, 12, 19);
    let (date, description) = if winter < summer {
        (
            winter,
            format!(
                "the Christmas break, from about 19 December ({}) to the first days of January",
                date_said(winter, today)
            ),
        )
    } else {
        (
            summer,
            format!(
                "the summer break, from the third week of July ({}) to the first days of September",
                date_said(summer, today)
            ),
        )
    };
    if asks_wait(clean) {
        return format!(
            "{} until the next break - {description}. The exact dates come from your school or your council.",
            wait_days(today, date)
        );
    }
    format!(
        "The next one is {description}. English state schools have six weeks off in summer, two at Christmas, two at Easter and a week's half term in October, February and May; the exact dates come from your school or your council."
    )
}

fn render<'a>(
    which: Break,
    clean: &str,
    prior: impl DoubleEndedIterator<Item = &'a str>,
    today: Date,
) -> Cow<'static, str> {
    if let Some(reply) = nation_reply(clean, prior) {
        return Cow::Owned(reply);
    }
    match which {
        Break::Summer => Cow::Owned(summer(today, clean)),
        Break::Winter => Cow::Owned(winter(today, clean)),
        Break::Easter => Cow::Borrowed(EASTER),
        Break::HalfTerm => Cow::Borrowed(HALF_TERM),
        Break::BackToSchool => back_to_school(clean),
        Break::General => Cow::Owned(general(today, clean)),
    }
}

pub(super) fn answer(
    which: Break,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Cow<'static, str> {
    render(
        which,
        &reading.line.clean,
        earlier.iter().map(|turn| turn.user.as_str()),
        Zoned::now().date(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_the_break_date_count_or_length_as_requested() {
        let date = Date::new(2026, 7, 14).unwrap();
        let reply = |which, text| render(which, text, std::iter::empty(), date);
        assert!(reply(Break::Summer, "when are the summer holidays").contains("Tuesday 21 July"));
        assert!(reply(Break::Summer, "days till the summer holidays").starts_with("7 days until"));
        assert!(
            reply(Break::Summer, "how long are the summer holidays").starts_with("Six weeks off")
        );
        assert!(
            reply(Break::General, "how many days until the kids break up")
                .starts_with("7 days until the next break")
        );
        assert!(
            reply(Break::BackToSchool, "when does the spring term start")
                .starts_with("The spring term starts")
        );
        assert!(
            reply(Break::Easter, "when are the easter holidays").starts_with("The Easter break")
        );
    }

    #[test]
    fn reads_the_latest_nation_and_keeps_older_context_outside_the_window() {
        let date = Date::new(2026, 11, 18).unwrap();
        assert!(
            render(
                Break::HalfTerm,
                "when is half term in wales",
                ["i live in glasgow"].into_iter(),
                date
            )
            .starts_with("Welsh schools")
        );
        assert!(
            render(
                Break::BackToSchool,
                "when do schools go back",
                ["i live in london", "we moved to dundee"].into_iter(),
                date
            )
            .starts_with("Scottish schools")
        );
        let old =
            std::iter::once("i live in belfast").chain(std::iter::repeat_n("good to know", 6));
        assert!(
            render(Break::Winter, "when is christmas break", old, date)
                .starts_with("The Christmas break")
        );
        assert!(
            render(
                Break::Summer,
                "when are the summer holidays in england",
                ["i live in cork"].into_iter(),
                date
            )
            .contains("Wednesday 21 July 2027")
        );
    }
}
