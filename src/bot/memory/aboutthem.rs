//! Project a recorded whole-year age to a time the question specifies.

use super::{Personal, personal::Age};
use crate::bot::{Memory, Reading};
macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "aboutthem_data.rs"]
mod data;
use data::*;

pattern!(WORD, r"[a-z0-9]+(?:'[a-z]+)?");

/// Add the child's age to a measured question that refers to them by pronoun.
pub(crate) fn with_the_age(raw: &str, memory: &Memory) -> Option<String> {
    rewrite(raw, &memory.personal)
}

fn rewrite(raw: &str, personal: &Personal) -> Option<String> {
    if THEIR_AGE.is_match(raw) || GIVEN.is_match(raw) || !ASKS.is_match(raw) {
        return None;
    }
    if !raw.contains('?') {
        let low = raw.to_lowercase().replace('’', "'");
        let first = WORD.find(&low)?.as_str();
        let first = first.strip_suffix("'s").unwrap_or(first).replace('\'', "");
        if RETURN_ASKS.binary_search(&first.as_str()).is_err() {
            return None;
        }
    }
    let found = PRONOUN.captures(raw)?;
    let span = found.get(0)?;
    let pronoun = ["who", "own", "obj"]
        .into_iter()
        .find_map(|key| found.name(key))?
        .as_str()
        .to_lowercase();
    let (fits_he, fits_she) = match pronoun.as_str() {
        "she" | "her" => (false, true),
        "he" | "his" => (true, false),
        _ => (true, true),
    };
    let (number, unit) = personal.recorded_ages().find_map(|(who, age)| {
        if !(fits_he && HE.binary_search(&who).is_ok()
            || fits_she && SHE.binary_search(&who).is_ok())
        {
            return None;
        }
        match age {
            Age::Months(months) if (1..BABY_MONTHS).contains(&months) => Some((months, "month")),
            Age::Years(years) if (1..CHILD_AGE).contains(&years) => Some((years, "year")),
            _ => None,
        }
    })?;
    let age = format!("a {} {unit} old", spoken(u16::from(number)));
    let named = if found.name("own").is_some() {
        format!("{age}'s {}", found.name("thing")?.as_str().trim())
    } else {
        age
    };
    let written = format!("{}{named}{}", &raw[..span.start()], &raw[span.end()..]);
    let written = written
        .split(crate::text::is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    (written != raw).then_some(written)
}

fn age_of(personal: &Personal, fits: impl Fn(&str) -> bool) -> Option<u16> {
    personal.recorded_ages().find_map(|(who, age)| {
        if let Age::Years(age) = age
            && (1..CHILD_AGE).contains(&age)
            && fits(who)
        {
            Some(u16::from(age))
        } else {
            None
        }
    })
}

fn reply_for(text: &str, personal: &Personal, year: impl FnOnce() -> i16) -> Option<String> {
    if !AHEAD_ASK.is_match(text) {
        return None;
    }
    let said = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let time = AHEAD_TIME.captures(&said)?;
    let (age, subject) = if let Some(named) = KIN.captures(&said) {
        let who = named["who"].to_lowercase();
        let aliases = KIN_SAME
            .iter()
            .find_map(|(label, fits)| (*label == who).then_some(*fits))?;
        (
            age_of(personal, |role| aliases.contains(&role))?,
            format!("Your {who} will be"),
        )
    } else {
        let pronoun = PRONOUN.captures(&said)?;
        let word = ["who", "own", "obj"]
            .into_iter()
            .find_map(|name| pronoun.name(name))?;
        let (subject, he, she) = match word.as_str().to_lowercase().as_str() {
            "she" | "her" => ("She'll be", false, true),
            "he" | "his" | "him" => ("He'll be", true, false),
            _ => ("They'll be", true, true),
        };
        let age = age_of(personal, |role| {
            (he && HE.binary_search(&role).is_ok()) || (she && SHE.binary_search(&role).is_ok())
        })?;
        (age, subject.to_owned())
    };
    let years = if let Some(next) = time.name("next") {
        next.as_str().eq_ignore_ascii_case("year").then_some(1)?
    } else if let Some(calendar) = time.name("year") {
        u16::try_from(calendar.as_str().parse::<i16>().ok()? - year()).ok()?
    } else {
        let quantity = time.name("years")?.as_str().to_lowercase();
        quantity.parse::<u16>().ok().or_else(|| {
            SPOKEN
                .iter()
                .position(|word| *word == quantity)
                .and_then(|index| u16::try_from(index).ok())
        })?
    };
    let day = if let Some(calendar) = time.name("year") {
        format!("in {}", calendar.as_str())
    } else {
        match years {
            0 => "now".into(),
            1 => "next year".into(),
            n => format!("in {} years", spoken(n)),
        }
    };
    let answer = age + years;
    let answer = if answer <= 12 {
        spoken(answer)
    } else {
        answer.to_string()
    };
    Some(format!("{subject} {answer} {day}."))
}

fn spoken(number: u16) -> String {
    SPOKEN
        .get(usize::from(number))
        .map_or_else(|| number.to_string(), |word| (*word).to_owned())
}

pub(crate) fn an_age_ahead(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    reply_for(reading.raw, &memory.personal, || jiff::Zoned::now().year())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn personal(lines: &[&str]) -> Personal {
        let mut personal = Personal::default();
        for (i, line) in lines.iter().enumerate() {
            personal.observe(&Reading::new(line), i);
        }
        personal
    }

    #[test]
    fn projects_the_earliest_matching_record_and_recognizes_child_aliases() {
        let recorded = personal(&[
            "my son Jack is 6",
            "my son Sam is 9",
            "my little girl is 12",
        ]);
        for (line, expected) in [
            (
                "how old will my son be next year",
                "Your son will be seven next year.",
            ),
            (
                "what age will he be in two years",
                "He'll be eight in two years.",
            ),
            (
                "how old will my daughter be next year",
                "Your daughter will be 13 next year.",
            ),
            ("how old will she be in 2026", "She'll be twelve in 2026."),
            ("how old will he be in 2030", "He'll be ten in 2030."),
            ("how old will he be in 0 years", "He'll be six now."),
        ] {
            assert_eq!(
                reply_for(line, &recorded, || 2026).as_deref(),
                Some(expected),
                "{line}"
            );
        }
    }

    #[test]
    fn leaves_months_unmeasured_dates_and_past_years_to_other_readers() {
        let recorded = personal(&["my daughter is 6 months old", "my son is 6"]);
        for line in [
            "how old will she be next year",
            "how old will he be next summer",
            "how old will he be on his next birthday",
            "how old will he be in a year",
            "how old will he be in 2025",
            "how old is he",
        ] {
            assert_eq!(reply_for(line, &recorded, || 2026), None, "{line}");
        }
    }

    #[test]
    fn files_the_named_child_from_age_clauses_and_keeps_position_separate() {
        let recorded = personal(&[
            "My youngest child is 11.",
            "My son, who is seven, starts school.",
            "My daughter's age is 8.",
        ]);
        for (line, expected) in [
            (
                "how old will my youngest child be next year",
                Some("Your child will be eight next year."),
            ),
            (
                "how old will my son be next year",
                Some("Your son will be eight next year."),
            ),
            (
                "how old will my daughter be in two years",
                Some("Your daughter will be ten in two years."),
            ),
        ] {
            assert_eq!(
                reply_for(line, &recorded, || 2026).as_deref(),
                expected,
                "{line}"
            );
        }
        let positioned = personal(&["my youngest child is 11"]);
        let asks = "how old will my youngest child be in 2 years";
        assert_eq!(reply_for(asks, &positioned, || 2026), None);
    }

    #[test]
    fn rewrites_measured_questions_with_year_month_and_possessive_ages() {
        let recorded = personal(&["my son is 5", "my daughter is 6 months old"]);
        for (line, expected) in [
            (
                "How much sleep does he need?",
                "How much sleep does a five year old need?",
            ),
            (
                "how much sleep does she need",
                "how much sleep does a six month old need",
            ),
            (
                "what time is his bedtime",
                "what time is a five year old's bedtime",
            ),
        ] {
            assert_eq!(
                rewrite(line, &recorded).as_deref(),
                Some(expected),
                "{line}"
            );
        }
        for line in [
            "should he have a nap",
            "how old is she",
            "how much sleep does he need at 5",
            "normally how much sleep he gets",
            "2 how much sleep does he get",
        ] {
            assert_eq!(rewrite(line, &recorded), None, "{line}");
        }
    }

    #[test]
    fn the_month_and_year_facts_keep_their_own_units() {
        let recorded = personal(&["my daughter is 14 months old", "actually my daughter is 2"]);
        let measured = "how much sleep does she need";
        let projection = "how old will she be next year";
        assert_eq!(
            rewrite(measured, &recorded).as_deref(),
            Some("how much sleep does a fourteen month old need")
        );
        assert_eq!(
            reply_for(projection, &recorded, || 2026).as_deref(),
            Some("She'll be three next year.")
        );
        let older = personal(&["my daughter is 2", "now she is 25 months old"]);
        let coming = personal(&["my daughter is 2", "she turns 3 next month"]);
        for recorded in [older, coming] {
            assert_eq!(
                rewrite(measured, &recorded).as_deref(),
                Some("how much sleep does a two year old need")
            );
            assert_eq!(
                reply_for(projection, &recorded, || 2026).as_deref(),
                Some("She'll be three next year.")
            );
        }
    }
}
