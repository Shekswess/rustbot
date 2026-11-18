//! Compare the person's reported count with the one figure in the preceding answer.

use crate::{
    bot::{Memory, Reading, memory::groupages},
    text::is_space,
};
use regex::{Captures, Regex};
use std::sync::LazyLock;

const WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve",
];
const COUNT: &str =
    r"(?:\d[\d,]*(?:\.\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)";
const UNITS: &[&str] = &[
    "glasses?",
    "steps?",
    "minutes?|mins?",
    "hours?|hrs?",
    "grams?|grammes?|g",
    "milligrams?|mg",
    "micrograms?|mcg",
    "kilometres?|kilometers?|kms?",
    "miles?",
    "sessions?",
];

macro_rules! pattern {
    ($name:ident, $value:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

pattern!(
    REPORT,
    r"(?i)\b(?:i|we)\b[^.?!]{0,24}?\b(?:have|had|get|got|do|eat|drink|walk|run|take|use|manage|average|track|log|hit|only|now|usually|normally|still|am on)\b"
);
pattern!(
    QUESTION,
    r"(?i)\?|^\s*(?:how|what|which|why|when|where|who|should|can|could|do|does|did|is|are|was|were|will|would|any|got any)\b"
);
pattern!(
    LENGTH_OF,
    &format!(r"(?i)\b{COUNT}\s*(?:hours?|hrs?|minutes?|mins?)\s+of\s+\w")
);
pattern!(
    PERIOD,
    r"(?i)^\s*(?:a|an|per|each|every|most)\s+(day|week|night|month)\b"
);
pattern!(
    CEILING,
    r"(?i)\blimits?\b|\bno more than\b|\bat most\b|\bcap(?:ped)?\b|\bsafe (?:limit|amount|upper)\b|\bdo not (?:go )?(?:over|exceed)\b"
);

struct Family {
    single: Regex,
    range: Regex,
}

static FAMILIES: LazyLock<Vec<Family>> = LazyLock::new(|| {
    UNITS
        .iter()
        .map(|unit| Family {
            single: Regex::new(&format!(r"(?i)({COUNT})\s*(?:-\s*)?(?:{unit})\b"))
                .expect("count and unit"),
            range: Regex::new(&format!(
                r"(?i)({COUNT})\s*(?:to|-|–|and|or)\s*({COUNT})\s*(?:-\s*)?(?:{unit})\b"
            ))
            .expect("range and unit"),
        })
        .collect()
});

#[derive(Clone, Copy, PartialEq, Eq)]
enum Period {
    Day,
    Week,
}

fn period(tail: &str) -> Option<Period> {
    let word = PERIOD.captures(tail)?.get(1)?.as_str();
    Some(
        if word.eq_ignore_ascii_case("week") || word.eq_ignore_ascii_case("month") {
            Period::Week
        } else {
            Period::Day
        },
    )
}

struct Figure<'a> {
    value: f64,
    high: f64,
    typed: &'a str,
    period: Option<Period>,
}

fn value(text: &str) -> Option<f64> {
    let word = text.to_ascii_lowercase();
    if let Some(index) = WORDS.iter().position(|&number| number == word) {
        return Some((index + 1) as f64);
    }
    let digits: String = word
        .chars()
        .filter(|&ch| ch != ',')
        .map(|ch| {
            if ch.is_ascii() {
                ch
            } else {
                groupages::decimal(ch.encode_utf8(&mut [0; 4]))
                    .map_or(ch, |digit| char::from(b'0' + digit as u8))
            }
        })
        .collect();
    digits.parse().ok()
}

fn figure<'a>(text: &'a str, found: &Captures<'a>, high: Option<f64>) -> Option<Figure<'a>> {
    let typed = found.get(0)?;
    let value = value(found.get(1)?.as_str())?;
    Some(Figure {
        value,
        high: high.unwrap_or(value),
        typed: typed.as_str().trim_matches([' ', '.', ',', '!', '?']),
        period: period(&text[typed.end()..]),
    })
}

fn report(text: &str) -> Option<(&Family, Figure<'_>)> {
    if text.trim_matches(is_space).is_empty()
        || QUESTION.is_match(text)
        || !REPORT.is_match(text)
        || LENGTH_OF.is_match(text)
    {
        return None;
    }
    let mut found = FAMILIES.iter().flat_map(|family| {
        family
            .single
            .captures_iter(text)
            .filter_map(move |found| Some((family, figure(text, &found, None)?)))
    });
    let report = found.next()?;
    found.next().is_none().then_some(report)
}

fn previous_figure<'a>(answer: &'a str, family: &Family, mine: f64) -> Option<Figure<'a>> {
    let ranges: Vec<_> = family.range.captures_iter(answer).collect();
    let bounds = ranges.iter().filter_map(|found| {
        let high = value(found.get(2)?.as_str())?;
        let bound = figure(answer, found, Some(high))?;
        (bound.value < high).then_some(bound)
    });
    let singles = family.single.captures_iter(answer).filter_map(|found| {
        let single = found.get(0)?;
        if ranges
            .iter()
            .filter_map(|range| range.get(0))
            .any(|range| range.start() <= single.start() && single.end() <= range.end())
        {
            return None;
        }
        figure(answer, &found, None)
    });
    let mut figures = bounds.chain(singles);
    let only = figures.next()?;
    if figures.next().is_some() || only.value == only.high && only.value == mine {
        return None;
    }
    Some(only)
}

fn compare(text: &str, answer: &str) -> Option<String> {
    let (family, report) = report(text)?;
    let target = previous_figure(answer, family, report.value)?;
    if target.value <= 0.0
        || matches!((report.period, target.period), (Some(mine), Some(theirs)) if mine != theirs)
    {
        return None;
    }
    // The unit stays with the range. The period is outside the match so an
    // amount follow-up can read the range on this new reply.
    let first = target.typed.split(' ').next()?;
    let figure = if WORDS.contains(&first.to_ascii_lowercase().as_str()) {
        // Number words occupy ASCII bytes here.
        format!(
            "{}{}",
            char::from(target.typed.as_bytes()[0].to_ascii_lowercase()),
            &target.typed[1..]
        )
    } else {
        target.typed.to_owned()
    };
    let aim = format!("the {figure}");
    let below = report.value < target.value;
    let above = report.value > target.high;
    Some(match (CEILING.is_match(answer), below, above) {
        (true, _, true) => format!("That is over {aim}, which is worth trimming back."),
        (true, true, _) => format!("That is under {aim}, which is where you want to be."),
        (true, false, false) => {
            format!("That is inside {aim}, which is fine - that is a ceiling, not a target.")
        }
        (false, true, _) => format!("That is a fair way under {aim} most people need."),
        (false, _, true) => {
            format!("That is past {aim}, which is fine - more than that is no problem.")
        }
        (false, false, false) => format!("That is inside {aim} most people need."),
    })
}

// Each report is compared with the preceding reply when it is proposed.
pub(crate) fn observe(_reading: &Reading<'_>, _memory: &mut Memory) {}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    compare(reading.raw, &memory.turns.last()?.bot)
}

#[cfg(test)]
mod tests {
    use super::compare;

    #[test]
    fn compares_a_report_with_one_range_in_the_last_answer() {
        assert_eq!(
            compare(
                "i do ٦٠٠٠ steps a day",
                "Aim for 7,000 to 8,000 steps a day."
            )
            .as_deref(),
            Some("That is a fair way under the 7,000 to 8,000 steps most people need.")
        );
        assert_eq!(
            compare("i drink seven glasses", "Six to eight glasses a day.").as_deref(),
            Some("That is inside the six to eight glasses most people need.")
        );
        assert_eq!(
            compare("i do five sessions", "Cap it at four sessions a week.").as_deref(),
            Some("That is over the four sessions, which is worth trimming back.")
        );
    }

    #[test]
    fn passes_questions_unrelated_counts_ambiguity_and_incompatible_periods() {
        assert_eq!(compare("i do 50 minutes?", "Aim for 30 minutes."), None);
        assert_eq!(
            compare("i have 2 hours of meetings", "Try four hours."),
            None
        );
        assert_eq!(
            compare("i have 2 hours per day", "Try four hours each week."),
            None
        );
        assert_eq!(
            compare(
                "i do 20 minutes",
                "20 minutes at a brisk pace, or 15 minutes faster."
            ),
            None
        );
        assert_eq!(compare("i do 20 minutes", "You said 20 minutes."), None);
    }
}
