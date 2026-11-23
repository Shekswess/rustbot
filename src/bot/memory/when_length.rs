// Changed in the cppalliance fork of RustBot: the time comes from the turn. See "Changes in this fork" in README.md.

//! Recall how long ago a recorded activity or event began.

use crate::bot::{Exchange, Memory, Reading};
use jiff::{ToSpan, civil::Date};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
macro_rules! fancy {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<fancy_regex::Regex> =
            LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}
include!("when_length_data.rs");
pattern!(WORD, r"[a-z0-9']+");
pattern!(DATE_PART, r"[a-z0-9]+");
pattern!(DATE_NUMBER, r"^\d{1,2}(?:st|nd|rd|th)?$");

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];
const COUNTS: [&str; 12] = [
    "a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve",
];

fn normalized(text: &str) -> String {
    text.split(crate::text::is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn same(a: &str, b: &str) -> bool {
    fn root(word: &str) -> &str {
        ["ing", "ed"]
            .iter()
            .find_map(|tail| word.strip_suffix(tail).filter(|s| s.len() >= 2))
            .unwrap_or(word)
    }
    a == b
        || root(a) == root(b)
        || SAME_THING.iter().any(|&(key, forms)| {
            (key == a && forms.contains(&b)) || (key == b && forms.contains(&a))
        })
        || {
            let left = crate::bot::understanding::normalize::forms(a);
            let right = crate::bot::understanding::normalize::forms(b);
            left.iter().any(|form| right.contains(form))
        }
}

fn content(text: &str) -> Vec<String> {
    let clean = text.to_lowercase().replace('’', "'");
    WORD.find_iter(&clean)
        .map(|word| {
            word.as_str()
                .strip_suffix("'s")
                .unwrap_or(word.as_str())
                .replace('\'', "")
        })
        .filter_map(|word| {
            if word.len() < 3
                || word.bytes().all(|c| c.is_ascii_digit())
                || STOPWORDS.binary_search(&word.as_str()).is_ok()
            {
                return None;
            }
            let stem = if let Some(root) = word.strip_suffix("ies").filter(|_| word.len() > 4) {
                format!("{root}y")
            } else if let Some(root) = word
                .strip_suffix("es")
                .filter(|root| word.len() > 3 && root.ends_with(['s', 'x', 'z', 'h']))
            {
                root.to_owned()
            } else if let Some(root) = word
                .strip_suffix('s')
                .filter(|_| word.len() > 3 && !word.ends_with("ss"))
            {
                root.to_owned()
            } else {
                word
            };
            if STOPWORDS.binary_search(&stem.as_str()).is_ok()
                || crate::bot::understanding::normalize::forms(&stem)
                    .iter()
                    .any(|form| STOPWORDS.binary_search(&form.as_str()).is_ok())
            {
                return None;
            }
            Some(stem)
        })
        .collect()
}

fn spelled(n: i64) -> String {
    usize::try_from(n - 1)
        .ok()
        .and_then(|i| COUNTS.get(i))
        .map_or_else(|| n.to_string(), |word| (*word).into())
}

fn count(text: &str) -> Option<i64> {
    let text = text.trim();
    text.parse()
        .ok()
        .or_else(|| {
            NUMBER_WORDS
                .iter()
                .find_map(|&(word, n)| word.eq_ignore_ascii_case(text).then_some(n))
        })
        .filter(|&n| n > 0)
}

fn unit(text: &str) -> Option<&'static str> {
    const TIME: &[&str] = &["day", "week", "fortnight", "month", "year"];
    let original = text.to_lowercase();
    TIME.iter()
        .copied()
        .find(|item| *item == original.trim_end_matches('s'))
}

fn upper(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn reply(length: &str, since: &str) -> String {
    let length = upper(&normalized(length));
    if since.is_empty() {
        format!("{length}.")
    } else {
        format!("{length} - since {since}.")
    }
}

fn month_number(word: &str) -> Option<i8> {
    MONTHS
        .iter()
        .position(|month| month.eq_ignore_ascii_case(word))
        .map(|i| (i + 1) as i8)
}

fn month_name(month: i8) -> &'static str {
    MONTHS.get((month - 1) as usize).copied().unwrap_or("")
}

fn date_in(text: &str, today: Date) -> Option<Date> {
    let text = text.to_lowercase();
    let mut month = None;
    let mut year = None;
    let mut day = None;
    for word in DATE_PART.find_iter(&text).map(|word| word.as_str()) {
        if let Some(number) = month_number(word) {
            month = Some(number);
        } else if word.len() == 4 && word.bytes().all(|c| c.is_ascii_digit()) {
            year = word.parse::<i16>().ok();
        } else if DATE_NUMBER.is_match(word) {
            let digits = word.trim_end_matches(char::is_alphabetic);
            day = digits.parse::<i8>().ok();
        }
    }
    if let Some(year) = year {
        return Date::new(year, month.unwrap_or(1), day.unwrap_or(1)).ok();
    }
    let month = month?;
    for back in 0..3 {
        let date = Date::new(today.year() - back, month, day.unwrap_or(1)).ok()?;
        if date <= today {
            return Some(date);
        }
    }
    None
}

fn date_said(date: Date) -> String {
    let month = month_name(date.month());
    let day = date.day();
    if day == 1 {
        return month.into();
    }
    let end = if (10..=20).contains(&(day % 100)) {
        "th"
    } else {
        match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };
    format!("the {day}{end} of {month}")
}

fn length_in_words(today: Date, start: Date) -> String {
    let days = (today.duration_since(start).as_secs() / 86_400).max(1);
    match days {
        1 => "a day".into(),
        2..=11 => format!("{} days", spelled(days)),
        12..=59 => format!(
            "{} weeks",
            spelled((days as f64 / 7.).round_ties_even() as i64)
        ),
        60..=729 => format!(
            "{} months",
            spelled((days as f64 / 30.44).round_ties_even() as i64)
        ),
        _ => {
            let halves = (days as f64 * 2. / 365.25) as i64;
            if halves % 2 == 0 {
                format!(
                    "{} {}",
                    spelled(halves / 2),
                    if halves == 2 { "year" } else { "years" }
                )
            } else {
                format!("{} and a half years", spelled(halves / 2))
            }
        }
    }
}

fn has_date(text: &str) -> bool {
    FUTURE.is_match(text)
        || SHORT_DATE.is_match(text).unwrap_or(false)
        || PAST_WHEN.is_match(text)
        || BARE_DAY.is_match(text).unwrap_or(false)
}

fn length_from(said: &str, today: Date) -> Option<String> {
    if let Some(ago) = AGO_LENGTH.captures(said) {
        let number = count(&ago["n"])?;
        let unit = unit(&ago["unit"])?;
        let length = ago.get(0)?.as_str().rsplit_once(char::is_whitespace)?.0;
        let since = if unit == "year" {
            (i64::from(today.year()) - number).to_string()
        } else {
            String::new()
        };
        return Some(reply(length, &since));
    }
    if let Some(last) = LAST_LENGTH.captures(said) {
        let since = match unit(&last["unit"])? {
            "year" => (today.year() - 1).to_string(),
            "month" => month_name(if today.month() == 1 {
                12
            } else {
                today.month() - 1
            })
            .into(),
            _ => String::new(),
        };
        return Some(reply(&format!("a {}", last["unit"].to_lowercase()), &since));
    }
    if let Some(held) = FOR_LENGTH.captures(said)
        && let Some(number) = count(&held["n"])
        && let Some(unit) = unit(&held["unit"])
    {
        return Some(reply(
            &format!(
                "{} {unit}{}",
                spelled(number),
                if number == 1 { "" } else { "s" }
            ),
            "",
        ));
    }
    // The date index identifies a dated line. The source reader decodes that
    // entire line, so a year immediately after a matched month still applies.
    if has_date(said)
        && let Some(start) = date_in(said, today)
    {
        return Some(reply(&length_in_words(today, start), &date_said(start)));
    }
    if let Some(weekday) = WEEKDAY_SAID.captures(said)
        && let Some(index) = WEEKDAYS
            .iter()
            .position(|day| day.eq_ignore_ascii_case(&weekday["day"]))
    {
        let back =
            (i64::from(today.weekday().to_monday_zero_offset()) - index as i64).rem_euclid(7);
        let date = today.checked_sub(back.days()).ok()?;
        let label = if date.day() == 1 {
            month_name(date.month())
        } else {
            WEEKDAYS[index]
        };
        return Some(reply(&length_in_words(today, date), label));
    }
    let year = A_YEAR_SAID.find(said)?.as_str();
    let date = date_in(year, today)?;
    Some(reply(&length_in_words(today, date), year))
}

fn corrected(turns: &[Exchange], today: Date) -> Option<String> {
    let [.., before, last] = turns else {
        return None;
    };
    if !BEGAN.is_match(&before.user) {
        return None;
    }
    let last = normalized(&last.user);
    let found = DAY_PUT_RIGHT.captures(&last)?;
    let date = &found["day"];
    length_from(
        &format!(
            "it began {}{date}",
            if A_DAY_FORM.is_match(date) { "" } else { "in " }
        ),
        today,
    )
}

fn answer(text: &str, turns: &[Exchange], today: Date) -> Option<String> {
    let text = normalized(text);
    if !HOW_LONG_ASK.is_match(&text)
        || NOT_A_LENGTH_ASK.is_match(&text)
        || CALENDAR_NOW.is_match(&text)
        || WHEN_SAID.is_match(&text)
    {
        return None;
    }
    if let Some(corrected) = corrected(turns, today) {
        return Some(corrected);
    }
    let named: Vec<_> = content(&text)
        .into_iter()
        .filter(|word| {
            ASK_THROWS.binary_search(&word.as_str()).is_err()
                && ASKED_ABOUT_THE_THING.binary_search(&word.as_str()).is_err()
        })
        .collect();
    turns
        .iter()
        .rev()
        .map(|turn| normalized(&turn.user))
        .filter(|said| !said.ends_with('?') && !ASK_OPENER.is_match(said) && BEGAN.is_match(said))
        .take(4)
        .find_map(|said| {
            let words = content(&said);
            if named
                .iter()
                .all(|want| words.iter().any(|found| same(want, found)))
            {
                length_from(&said, today)
            } else {
                None
            }
        })
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !HOW_LONG_ASK.is_match(reading.raw) {
        return None;
    }
    answer(reading.raw, &memory.turns, reading.now().date())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixture {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/when_length.rs"
        ));
    }

    #[test]
    fn keeps_the_recorded_duration_ahead_of_general_advice() {
        let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
        bot.turn("we got a dog six months ago");
        let result = bot.turn("how long have i been at it");
        assert_eq!(result.module, Some("since_it_began"));
        assert_eq!(result.reply, "Six months.");
    }

    #[test]
    fn elapsed_time_matches_the_reference() {
        let today = Date::new(2026, 11, 17).unwrap();
        let mismatches: Vec<_> = fixture::CASES
            .iter()
            .filter_map(|&(prior, ask, expected)| {
                let turns: Vec<_> = prior
                    .iter()
                    .map(|&user| Exchange {
                        user: user.into(),
                        bot: String::new(),
                        module: "",
                    })
                    .collect();
                let got = answer(ask, &turns, today);
                (got.as_deref() != expected).then_some((prior, ask, expected, got))
            })
            .collect();
        assert!(
            mismatches.is_empty(),
            "{} differences: {:?}",
            mismatches.len(),
            &mismatches[..mismatches.len().min(12)]
        );
    }
}
