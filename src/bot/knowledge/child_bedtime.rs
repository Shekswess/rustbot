//! Age and evening-clock rendering for the `family/child-bedtime` card.

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    AGE_DIGITS,
    concat!(
        r"\b([0-9]{1,2})\s*(?:-|\s)?\s*(?:year|yr)s?\s*(?:-|\s)?\s*old\b",
        r"|\baged?\s*([0-9]{1,2})\b|\b([0-9]{1,2})\s*(?:yo|y/o)\b"
    )
);
pattern!(
    AGE_WORD_FORM,
    concat!(
        r"\b(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|",
        r"fifteen|sixteen|seventeen)\s*(?:-|\s)?\s*(?:year|yr)s?\s*(?:-|\s)?\s*old\b"
    )
);
pattern!(
    AGE_TOO_YOUNG,
    r"\b(?:week|weeks|month|months)\s*old\b|\bnewborn\b|\bbaby\b"
);
pattern!(
    TEEN_SAID,
    r"\b(?:teenager|teenagers|teen|teens|teenaged|teenage)\b"
);
pattern!(
    BED_CLOCK,
    concat!(
        r"\b([0-9]{1,2})(?:[.:]([0-9]{2}))?\s*(pm|p\.m\.)\b",
        r"|\b([0-9]{1,2})(?:[.:]([0-9]{2}))?\s*o'?clock\b"
    )
);

const AGE_WORDS: &[&str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
];

struct Band {
    last_age: usize,
    hours: &'static str,
    window: &'static str,
    earliest: u16,
    latest: u16,
}

const BANDS: &[Band] = &[
    Band {
        last_age: 2,
        hours: "eleven to fourteen hours",
        window: "6.30 and 7.30pm",
        earliest: 6 * 60 + 30,
        latest: 7 * 60 + 30,
    },
    Band {
        last_age: 5,
        hours: "ten to thirteen hours",
        window: "6.30 and 8pm",
        earliest: 6 * 60 + 30,
        latest: 8 * 60,
    },
    Band {
        last_age: 12,
        hours: "nine to twelve hours",
        window: "7.30 and 9pm",
        earliest: 7 * 60 + 30,
        latest: 9 * 60,
    },
    Band {
        last_age: 18,
        hours: "eight to ten hours",
        window: "8.30 and 10pm",
        earliest: 8 * 60 + 30,
        latest: 10 * 60,
    },
];

const CHART: &str = "Match it to the age: a one year old needs 11 to 14 hours and is ready at 6.30 to \
    7.30pm; two to five, 10 to 13 hours and 6.30 to 8pm; six to twelve, 9 to 12 hours \
    and 7.30 to 9pm. Work back from the morning wake-up rather than the clock, keep \
    the last hour calm and screen-free, and let it move half an hour either way as \
    long as the mornings work. Teenagers need 8 to 10 hours, so a 10pm bedtime is not \
    early for them.";

fn child_age(clean: &str) -> Option<usize> {
    if let Some(captures) = AGE_DIGITS.captures(clean) {
        return captures
            .iter()
            .skip(1)
            .flatten()
            .next()?
            .as_str()
            .parse()
            .ok();
    }
    let captures = AGE_WORD_FORM.captures(clean)?;
    AGE_WORDS
        .iter()
        .position(|word| *word == &captures[1])
        .map(|index| index + 1)
}

fn said_bedtime(clean: &str) -> Option<u16> {
    let captures = BED_CLOCK.captures(clean)?;
    let hour = captures
        .get(1)
        .or_else(|| captures.get(4))?
        .as_str()
        .parse()
        .ok()?;
    let minute = captures
        .get(2)
        .or_else(|| captures.get(5))
        .map_or(Some(0), |value| value.as_str().parse().ok())?;
    (captures.get(3).is_some() || matches!(hour, 6..=11)).then_some(hour * 60 + minute)
}

fn clock_words(minutes: u16) -> String {
    let hour = minutes / 60;
    let minute = minutes % 60;
    if minute == 0 {
        format!("{hour}pm")
    } else {
        format!("{hour}.{minute:02}pm")
    }
}

fn render(clean: &str) -> Cow<'static, str> {
    let explicit_age = child_age(clean);
    let teen = explicit_age.is_none() && TEEN_SAID.is_match(clean);
    let age = explicit_age.or(teen.then_some(15));
    let Some((age, band)) = age.and_then(|age| {
        BANDS
            .iter()
            .find(|band| age <= band.last_age)
            .map(|band| (age, band))
    }) else {
        return Cow::Borrowed(CHART);
    };
    if AGE_TOO_YOUNG.is_match(clean) {
        return Cow::Borrowed(CHART);
    }

    let age_word = age
        .checked_sub(1)
        .and_then(|index| AGE_WORDS.get(index))
        .copied();
    let (article, person) = match (teen, age_word) {
        (true, _) => ("a", Cow::Borrowed("teenager")),
        (false, Some(word @ ("eight" | "eleven"))) => {
            ("an", Cow::Owned(format!("{word} year old")))
        }
        (false, Some(word)) => ("a", Cow::Owned(format!("{word} year old"))),
        (false, None) => ("a", Cow::Owned(format!("{age} year old"))),
    };
    let Band {
        hours,
        window,
        earliest,
        latest,
        ..
    } = *band;
    if let Some(said) = said_bedtime(clean) {
        let clock = clock_words(said);
        return Cow::Owned(if said > latest {
            format!(
                "{clock} is past the window for {article} {person}, who is usually ready between {window} and needs {hours}. It \
                works if the morning starts late enough; if it does not, move lights out \
                fifteen minutes earlier for a fortnight rather than arguing tonight."
            )
        } else if said < earliest {
            format!(
                "{clock} is early for {article} {person}, who is usually ready between {window}, not before. A child who is not \
                tired fights it, so keep the last hour calm and dim and work back to {clock} over \
                a week or two."
            )
        } else {
            format!(
                "{clock} is inside the window for {article} {person} - {window}, with {hours} of sleep - so it is fine as long as \
                the mornings are not a fight. Keep the wake-up time fixed and the last hour calm \
                and screen-free."
            )
        });
    }
    if age >= 13 {
        return Cow::Owned(format!(
            "Teenagers need eight to ten hours and their body clock runs late, so lights out \
            between {window} is right and 10pm is not late. Fix the wake-up time and keep it at \
            weekends, get daylight early, and keep the phone out of the last half hour."
        ));
    }
    let article = if article == "an" { "An" } else { "A" };
    Cow::Owned(format!(
        "{article} {person} needs {hours}, so lights out between {window}, earlier on a school morning. Work back from the \
        wake-up time rather than the clock, keep the last hour calm and screen-free, and let \
        it move half an hour either way as long as the mornings work."
    ))
}

pub(super) fn answer(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    let text;
    let clean = if clean.is_empty() {
        text = reading.line.text.to_lowercase();
        text.as_str()
    } else {
        clean.as_str()
    };
    Some(render(clean))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_the_named_age_band_or_the_full_chart() {
        assert_eq!(
            render("how much sleep does a five year old need"),
            "A five year old needs ten to thirteen hours, so lights out between 6.30 and 8pm, earlier on a school morning. Work back from the wake-up time rather than the clock, keep the last hour calm and screen-free, and let it move half an hour either way as long as the mornings work."
        );
        assert!(
            render("what time should a child aged 8 go to bed")
                .starts_with("An eight year old needs nine to twelve hours")
        );
        assert!(
            render("what time should a 14yo go to bed")
                .starts_with("Teenagers need eight to ten hours")
        );
        assert_eq!(render("what time should my baby go to bed"), CHART);
        assert_eq!(render("what time should my 21 year old go to bed"), CHART);
    }

    #[test]
    fn compares_the_named_time_with_both_window_edges() {
        assert!(
            render("is 10pm too late for a seven year old")
                .starts_with("10pm is past the window for a seven year old")
        );
        assert!(
            render("is 7.30pm too early for a 7-year-old")
                .starts_with("7.30pm is inside the window")
        );
        assert!(
            render("is 6 o'clock too early for an eleven year old")
                .starts_with("6pm is early for an eleven year old")
        );
        assert!(
            render("is 5 o'clock too early for an eleven year old")
                .starts_with("An eleven year old needs")
        );
    }
}
