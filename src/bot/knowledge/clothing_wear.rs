//! Socks, lacing, clothing-size conversions and a wardrobe-weather guard.
#[path = "clothing_wear_data.rs"]
mod data;
#[cfg(test)]
#[path = "clothing_wear_tests.rs"]
mod tests;

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    Socks,
    Lace,
    Size,
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    SOCKS_FORMAL,
    SOCKS_CASUAL,
    SOCKS_COLOUR,
    SOCKS_BLACK_AND_BROWN,
    SOCKS_SPORT,
    ENOUGH_FOR_A_GARMENT,
    A_SIZE_SAID,
    A_WEDDING_SHORTAGE,
    A_NAMED_DRESS_CODE,
);
static WHITE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bwhite\b").unwrap());

fn line_views(reading: &Reading<'_>) -> String {
    format!(
        "{} {} {}",
        reading.line.clean, reading.line.norm_text, reading.line.text
    )
    .to_lowercase()
}

fn mentions(line: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| line.contains(term))
}

fn socks(reading: &Reading<'_>) -> &'static str {
    let line = line_views(reading);
    if SOCKS_SPORT.is_match(&line) {
        data::SOCKS_FOR_SPORT
    } else if SOCKS_BLACK_AND_BROWN.is_match(&line) {
        data::SOCKS_BLACK_BROWN
    } else if SOCKS_FORMAL.is_match(&line) {
        data::SOCKS_SMART
    } else if WHITE.is_match(&line) && SOCKS_CASUAL.is_match(&line) {
        data::SOCKS_WHITE
    } else if SOCKS_CASUAL.is_match(&line) || SOCKS_COLOUR.is_match(&line) {
        data::SOCKS_EVERYDAY
    } else {
        data::SOCKS_DEFAULT
    }
}

fn lace(reading: &Reading<'_>) -> &'static str {
    let words = reading.line.words.join(" ").to_lowercase();
    let skates = mentions(&words, data::LACE_SKATES);
    if !skates && mentions(&words, data::LACE_TEACH) {
        data::LACE_TEACHING
    } else if mentions(&words, data::LACE_BOW) {
        data::LACE_BOW_REPLY
    } else if skates {
        data::LACE_SKATES_REPLY
    } else if mentions(&words, data::LACE_UNDONE) {
        data::LACE_UNDONE_REPLY
    } else {
        data::LACE_KNOT
    }
}

fn size(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<String> {
    let window = &earlier[earlier.len().saturating_sub(3)..];
    let prior = window
        .iter()
        .map(|turn| turn.user.as_str())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let captured = A_SIZE_SAID.captures_iter(&prior).last()?;
    if mentions(&prior, data::A_SHOE_SIZE) {
        return None;
    }
    let number = captured[1].parse::<u8>().ok()?;
    let &(_, eu, us) = data::WOMENS_SIZES.iter().find(|(uk, _, _)| *uk == number)?;
    let line = line_views(reading);
    if mentions(
        &line,
        &["in the us", "in us", "in usa", "in america", "american"],
    ) {
        return Some(format!(
            "A UK {number} is a US {us} - that is the women's conversion, and men's clothes go by chest size instead. A size is only a starting point either way: it drifts a whole step between brands, so go by the brand's own size guide and your measurements rather than the label."
        ));
    }
    Some(format!(
        "A UK {number} is a European {eu} in women's clothes, and a US {us}. Men's clothes go by chest size instead: 36 in chest is EU 46, 38 is 48, 40 is 50, 42 is 52. Sizes drift a whole step between brands, so check the brand's own size guide as well."
    ))
}

pub(super) fn enough_weather(reading: &Reading<'_>) -> bool {
    ENOUGH_FOR_A_GARMENT.is_match(&reading.line.words.join(" ").to_lowercase())
}

pub(super) fn outfit_shortage(reading: &Reading<'_>) -> bool {
    let line = format!(
        "{} {} {} {} {} {}",
        reading.line.clean,
        reading.line.norm_text,
        reading.line.text,
        reading.raw,
        reading.line.plain,
        reading.line.words.join(" "),
    )
    .to_lowercase();
    !A_NAMED_DRESS_CODE.is_match(&line) && A_WEDDING_SHORTAGE.is_match(&line)
}

pub(super) fn reply(
    form: Reply,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let response = match form {
        Reply::Socks => Cow::Borrowed(socks(reading)),
        Reply::Lace => Cow::Borrowed(lace(reading)),
        Reply::Size => Cow::Owned(size(reading, earlier)?),
    };
    Some(response)
}
