//! Fractions and mixed amounts of a fixed-length time unit.

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
include!("units_timespan_data.rs");

pattern!(FRACTIONS, r"\bfractions?\b");
pattern!(AND_A_HALF, r"^\s*and\s+a\s+half\b");

#[derive(Clone, Copy, Eq, PartialEq)]
enum Unit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
}

impl Unit {
    fn read(text: &str) -> Option<Self> {
        Some(match text {
            "second" | "seconds" | "sec" | "secs" => Self::Second,
            "minute" | "minutes" | "min" | "mins" => Self::Minute,
            "hour" | "hours" | "hr" | "hrs" => Self::Hour,
            "day" | "days" => Self::Day,
            "week" | "weeks" => Self::Week,
            _ => return None,
        })
    }

    fn seconds(self) -> f64 {
        match self {
            Self::Second => 1.,
            Self::Minute => 60.,
            Self::Hour => 3600.,
            Self::Day => 86400.,
            Self::Week => 604800.,
        }
    }

    fn plural(self) -> &'static str {
        match self {
            Self::Second => "seconds",
            Self::Minute => "minutes",
            Self::Hour => "hours",
            Self::Day => "days",
            Self::Week => "weeks",
        }
    }

    fn smaller(self) -> Option<Self> {
        match self {
            Self::Second => None,
            Self::Minute => Some(Self::Second),
            Self::Hour => Some(Self::Minute),
            Self::Day => Some(Self::Hour),
            Self::Week => Some(Self::Day),
        }
    }

    fn article(self) -> &'static str {
        match self {
            Self::Second => "a second",
            Self::Minute => "a minute",
            Self::Hour => "an hour",
            Self::Day => "a day",
            Self::Week => "a week",
        }
    }
}

const NUMERALS: [&str; 12] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve",
];
const NAMED_FRACTIONS: &[(u8, u8, &str)] = &[
    (1, 4, "a quarter"),
    (1, 2, "half"),
    (3, 4, "three quarters"),
    (1, 3, "a third"),
    (2, 3, "two thirds"),
    (1, 5, "a fifth"),
    (1, 6, "a sixth"),
    (1, 8, "an eighth"),
    (1, 7, "a seventh"),
    (1, 9, "a ninth"),
    (2, 7, "two sevenths"),
    (1, 10, "a tenth"),
    (1, 12, "a twelfth"),
    (1, 24, "a twenty-fourth"),
    (5, 6, "five sixths"),
    (5, 8, "five eighths"),
    (3, 8, "three eighths"),
    (7, 8, "seven eighths"),
    (1, 20, "a twentieth"),
];

fn numeral(text: &str) -> Option<f64> {
    let value = match text {
        "a" | "an" => 1.,
        "fifteen" => 15.,
        "twenty" => 20.,
        "thirty" => 30.,
        "forty" => 40.,
        "fifty" => 50.,
        _ => NUMERALS
            .iter()
            .position(|word| *word == text)
            .map(|i| (i + 1) as f64)
            .or_else(|| text.parse().ok())?,
    };
    value.is_finite().then_some(value)
}

fn fraction_value(word: &str) -> Option<f64> {
    let unfolded = word.replace('-', " ");
    let value = if word == "half" {
        0.5
    } else if let Some((top, bottom)) = word.split_once('/') {
        let bottom = numeral(bottom.trim())?;
        if bottom == 0. {
            return None;
        }
        numeral(top.trim())? / bottom
    } else if word.contains('.') {
        numeral(word)?
    } else {
        let mut words = unfolded.split_whitespace();
        let count = numeral(words.next()?)?;
        let divisor = match words.next()? {
            "half" | "halves" => 2.,
            "third" | "thirds" => 3.,
            "quarter" | "quarters" => 4.,
            "fifth" | "fifths" => 5.,
            "sixth" | "sixths" => 6.,
            "eighth" | "eighths" => 8.,
            "tenth" | "tenths" => 10.,
            "twelfth" | "twelfths" => 12.,
            "twentieth" | "twentieths" => 20.,
            _ => return None,
        };
        if words.next().is_some() {
            return None;
        }
        count / divisor
    };
    (0. < value && value < 1.).then_some(value)
}

fn fraction_name(value: f64) -> Option<&'static str> {
    NAMED_FRACTIONS.iter().find_map(|&(top, bottom, said)| {
        ((value - f64::from(top) / f64::from(bottom)).abs() < 0.001).then_some(said)
    })
}

fn upper(text: &str) -> String {
    let mut letters = text.chars();
    letters
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(letters)
        .collect()
}

// The Python reader uses six significant digits and scientific notation outside
// the normal range of the 'g' format.
fn general(value: f64) -> String {
    let scientific = format!("{value:.5e}");
    let (mantissa, exponent) = scientific.split_once('e').expect("scientific number");
    let exponent: i32 = exponent.parse().expect("scientific exponent");
    let trim = |s: String| {
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_owned()
        } else {
            s
        }
    };
    if !(-4..6).contains(&exponent) {
        return format!("{}e{exponent:+03}", trim(mantissa.to_owned()));
    }
    trim(format!(
        "{value:.precision$}",
        precision = (5 - exponent) as usize
    ))
}

fn amount(value: f64, unit: Unit) -> String {
    if (value - value.round()).abs() < 0.0001 {
        let rounded = value.round();
        return if rounded == 1. {
            format!("one {}", unit.plural().trim_end_matches('s'))
        } else {
            format!(
                "{} {}",
                crate::bot::knowledge::maths::number(rounded),
                unit.plural()
            )
        };
    }
    if (value.fract() - 0.5).abs() < 0.0001 {
        return format!("{:.0} and a half {}", value.trunc(), unit.plural());
    }
    format!("{} {}", general(value), unit.plural())
}

fn count_word(value: f64) -> String {
    if (1.0..=12.0).contains(&value) {
        NUMERALS[value as usize - 1].into()
    } else {
        format!("{:.0}", value.trunc())
    }
}

fn share_of_unit(value: f64, unit: Unit) -> String {
    let article = unit.article();
    if (value - 0.5).abs() < 0.0001 {
        return format!("Half {article}");
    }
    if value > 1. - 0.0001 {
        let whole = value.trunc();
        let rest = ((value - whole) * 10000.).round_ties_even() / 10000.;
        if rest.abs() < 0.0001 {
            return if whole == 1. {
                upper(article)
            } else {
                format!("{} {}", count_word(whole), unit.plural())
            };
        }
        if (rest - 0.5).abs() < 0.0001 {
            return if whole == 1. {
                format!("{} and a half", upper(article))
            } else {
                format!("{} and a half {}", count_word(whole), unit.plural())
            };
        }
        if whole == 1. {
            let rest_name = fraction_name(rest).map_or_else(|| general(rest), str::to_owned);
            return format!("{} and {rest_name}", upper(article));
        }
        return format!(
            "{} {} and {}",
            count_word(whole),
            unit.plural(),
            share_of_unit(rest, unit).to_lowercase()
        );
    }
    let name = fraction_name(value).map_or_else(|| general(value), str::to_owned);
    format!("{} of {article}", upper(&name))
}

fn fraction_of_time(clean: &str) -> Option<String> {
    for captures in [FRACTION_OF.captures(clean), FRACTION_AS.captures(clean)]
        .into_iter()
        .flatten()
    {
        let unit = Unit::read(&captures["unit"])?;
        let small = Unit::read(&captures["small"])?;
        let number = numeral(&captures["amount"])?;
        if number == 0. || unit.smaller() != Some(small) {
            continue;
        }
        let ratio = number * small.seconds() / unit.seconds();
        let said = upper(&amount(number, small));
        let part = fraction_name(ratio).map_or_else(
            || format!("about {:.0}%", (100. * ratio).round_ties_even()),
            str::to_owned,
        );
        return Some(format!("{said} is {part} of {}.", unit.article()));
    }
    None
}

fn span_amount(seconds: f64, unit: Unit, clean: &str) -> Option<String> {
    let target = UNIT_ASKED
        .captures(clean)
        .and_then(|capture| Unit::read(&capture[1]))
        .filter(|wanted| wanted.seconds() < unit.seconds())
        .or_else(|| unit.smaller())?;
    Some(amount(seconds / target.seconds(), target))
}

fn mixed_parts(clean: &str) -> Option<(Unit, f64, f64)> {
    let found = MIXED_SPAN
        .captures(clean)
        .or_else(|| HALF_SPAN.captures(clean))?;
    let unit = Unit::read(&found["unit"])?;
    let whole = numeral(&found["whole"])?;
    let rest = found
        .name("n")
        .map_or(Some(0.5), |n| numeral(n.as_str()).map(|n| n / 60.))?;
    Some((unit, whole, rest))
}

pub(super) fn answer(clean: &str) -> Option<String> {
    let asks = SPAN_ASK.is_match(clean);
    let target = TARGET_UNIT.is_match(clean);
    if !asks && !target && !FRACTION_ASK.is_match(clean) && !FRACTION_AS.is_match(clean) {
        return None;
    }
    if FRACTIONS.is_match(clean)
        && let Some(fraction) = fraction_of_time(clean)
    {
        return Some(fraction);
    }
    if let Some(share) = FRACTION_OF_A_MIXED.captures(clean).ok().flatten()
        && let Some(value) = fraction_value(&share["word"])
        && let Some((unit, whole, extra)) = mixed_parts(&share["span"])
    {
        let part = (whole + extra) * value;
        return Some(format!(
            "{} is {}.",
            share_of_unit(part, unit),
            span_amount(part * unit.seconds(), unit, clean)?
        ));
    }
    if let Some((unit, whole, extra)) = mixed_parts(clean) {
        let said = if (extra - 0.5).abs() < 0.0001 {
            share_of_unit(whole + 0.5, unit)
        } else {
            let head = if whole == 1. {
                upper(unit.article())
            } else {
                format!("{} {}", count_word(whole), unit.plural())
            };
            format!("{head} and {}", amount(extra * 60., Unit::Minute))
        };
        return Some(format!(
            "{} is {}.",
            upper(&said),
            span_amount((whole + extra) * unit.seconds(), unit, clean)?
        ));
    }
    if target || !asks {
        return None;
    }
    if let Some(whole) = WHOLE_UNIT.captures(clean) {
        let unit = Unit::read(&whole["unit"])?;
        return Some(format!(
            "{} is {}.",
            share_of_unit(1., unit),
            span_amount(unit.seconds(), unit, clean)?
        ));
    }
    let found = TIME_SHARE.captures(clean).ok().flatten()?;
    let unit = Unit::read(&found["unit"])?;
    let mut value = fraction_value(&found["word"])?;
    let end = found.get(0)?.end();
    let tail: String = clean[end..].chars().take(14).collect();
    if AND_A_HALF.is_match(&tail) {
        value += 0.5;
    }
    Some(format!(
        "{} is {}.",
        share_of_unit(value, unit),
        span_amount(value * unit.seconds(), unit, clean)?
    ))
}

#[cfg(test)]
mod tests {
    mod reference {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/unit_timespan.rs"
        ));
    }

    #[test]
    fn spans_match_the_reference() {
        let mismatches: Vec<_> = reference::CASES
            .iter()
            .filter_map(|&(text, expected)| {
                let actual = super::answer(text);
                (actual.as_deref() != expected).then_some((text, expected, actual))
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
