//! Fabric uses, taking a coat, wardrobe fit, and spectacle repairs from PyBot.

use std::{borrow::Cow, sync::LazyLock};

use crate::bot::Reading;
use regex::{Regex, RegexSet};

use super::{
    COAT_CLOCK, COAT_LOST_OR_TAKEN, COAT_NOT_ABOUT_TODAY, COAT_OUT_WHEN, FABRIC_USES, GLASSES_GLUE,
    GLASSES_GLUE_WORDS, GLASSES_NAMING_WORDS, GLASSES_REPAIR, WARDROBE_ADVICE,
    WARDROBE_WEIGHT_COMING_OFF,
};

#[cfg(test)]
#[path = "garments_tests.rs"]
mod tests;

static FABRIC_USE: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(FABRIC_USES.iter().map(|&(names, _)| {
        let names = names
            .iter()
            .map(|name| regex::escape(name))
            .collect::<Vec<_>>();
        format!(r"\b(?:{})\b", names.join("|"))
    }))
    .expect("fabric names are literals")
});
static COAT_DAY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(COAT_OUT_WHEN).expect("coat day pattern"));
static COAT_TIME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(COAT_CLOCK).expect("coat clock pattern"));
static LOST_COAT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(COAT_LOST_OR_TAKEN).expect("lost garment pattern"));
static LOST_WEIGHT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(WARDROBE_WEIGHT_COMING_OFF).expect("weight loss pattern"));

pub(super) fn fabric(reading: &Reading<'_>) -> Option<&'static str> {
    let text = if !reading.line.clean.is_empty() {
        Cow::Borrowed(reading.line.clean.as_str())
    } else if !reading.raw.is_empty() {
        Cow::Owned(reading.raw.to_lowercase())
    } else {
        Cow::Owned(reading.line.words.join(" ").to_lowercase())
    };
    let index = FABRIC_USE.matches(&text).into_iter().next()?;
    Some(FABRIC_USES[index].1)
}

pub(super) fn take_coat(reading: &Reading<'_>) -> bool {
    // PyBot converts the word list to its Python string form and replaces the quotes
    // and commas with spaces. Its clean and original text still carry whole phrases.
    let said = format!(
        "  {}   {} {}",
        reading.line.words.join("    "),
        reading.line.clean,
        reading.raw,
    )
    .to_lowercase()
    .replace(['[', ']', ',', '\''], " ");
    !["how do i", "how do you", "how can i", "how to"]
        .iter()
        .chain(COAT_NOT_ABOUT_TODAY)
        .any(|phrase| said.contains(phrase))
        && !LOST_COAT.is_match(&said)
        && (COAT_DAY.is_match(&said) || COAT_TIME.is_match(&said))
}

fn line_views(reading: &Reading<'_>) -> String {
    format!(
        "{} {} {}",
        reading.line.clean, reading.line.norm_text, reading.raw
    )
    .to_lowercase()
}

pub(super) fn wardrobe(reading: &Reading<'_>) -> Cow<'static, str> {
    if !LOST_WEIGHT.is_match(&line_views(reading)) {
        return Cow::Borrowed(WARDROBE_ADVICE);
    }
    let mut advice = WARDROBE_ADVICE.to_owned();
    advice[..1].make_ascii_lowercase();
    format!("If the weight came off without you trying, get it checked first: that is worth one appointment, and weight that comes off by itself shouldn't be ignored. Then, when you restock, {advice}").into()
}

pub(super) fn glasses_repair(reading: &Reading<'_>) -> &'static str {
    let said = line_views(reading);
    if GLASSES_GLUE_WORDS.iter().any(|word| said.contains(word))
        && !GLASSES_NAMING_WORDS.iter().any(|word| said.contains(word))
    {
        GLASSES_GLUE
    } else {
        GLASSES_REPAIR
    }
}
