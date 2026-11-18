//! Remove a bare temperature figure before reading the person's own state.

use std::sync::LazyLock;

use fancy_regex::Regex as Lookaround;
use regex::Regex;

use crate::bot::{Memory, Reading, compose::body_state};

const NUMBER: &str = r"(?:\d+(?:\.\d+)?|zero|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred)";
const UNIT: &str =
    r"(?:degrees?\b|deg\b|°(?:\s*[cf]\b)?|celsius\b|centigrade\b|fahrenheit\b|[cf]\b)";
const LEAD: &str = "its|it's|about|around|only|just|currently|still|nearly|almost|minus|negative|plus|and|but|so|then|is|be";

static FIGURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"\b(?:(?:{LEAD})\s+)*{NUMBER}\s*{UNIT}(?:\s+(?:outside|out there|in here|here|today|tonight|now|already|still))?")).expect("temperature figure")
});
static BELOW_ZERO: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(&format!(r"(?<![\w-])(?:minus\s+|[-−]\s?){NUMBER}(?:\s*{UNIT})?(?:\s*(?:outside|out there|out|in here|today|tonight|now))?")).expect("negative temperature figure")
});
static AT_START: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[\s,;:-]*(?:(?:it's|its|it|is|was|there|and|but|so|then|about|around|only|just|still|currently|be)\b[\s,;:-]*)+").expect("before bare temperature")
});
static AT_END: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[\s,;:-]+(?:(?:it's|its|it|is|was|there|about|around|only|just|still|currently|and|but|so|then)\b[\s,;:-]*)+$").expect("after bare temperature")
});

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.raw.to_lowercase().replace('’', "'");
    if !FIGURE.is_match(&line) && !BELOW_ZERO.is_match(&line).unwrap_or(false) {
        return None;
    }
    // Strip negatives as a unit before stripping the positive figure inside one.
    let without_negative = BELOW_ZERO.replace_all(&line, " ");
    let without_figure = FIGURE.replace_all(&without_negative, " ");
    let tidy = without_figure
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let cold = AT_START.replace(tidy.trim(), "");
    let cold = AT_END.replace(cold.trim_matches([' ', ',', ';', ':', '-']), "");
    body_state::reply(cold.trim(), memory, &reading.line.clean)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn figure_beside_the_state_leaves_named_things_to_their_specialists() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("It's 20 degrees and I'm freezing");
        assert_eq!(turn.module, Some("a_figure_beside_the_state"), "{turn:?}");
        for line in [
            "I'm freezing, it's -3 outside",
            "it's 30°C and I'm hungry",
            "it's -3°C and I'm freezing",
        ] {
            assert!(
                respond(&Reading::new(line), &Memory::default()).is_some(),
                "{line}"
            );
        }
        for line in [
            "The office is 20 degrees and I'm freezing",
            "What temperature does water freeze?",
            "I'm roasting a chicken at 180 degrees",
            "It is 20 degrees and I'm hot and cold",
        ] {
            assert!(
                respond(&Reading::new(line), &Memory::default()).is_none(),
                "{line}"
            );
        }
    }
}
