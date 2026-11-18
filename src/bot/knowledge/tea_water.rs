//! Kettle water quantities from the number of mugs or people.

use std::{borrow::Cow, sync::LazyLock};

use crate::bot::Reading;
use regex::Regex;

use super::{TEA_COUNT_WORDS, TEA_CUPS_SAID, TEA_FULL_KETTLE, TEA_FULL_KETTLE_ASK, TEA_ML_PER_MUG};

static CUPS: LazyLock<Regex> = LazyLock::new(|| Regex::new(TEA_CUPS_SAID).expect("cups pattern"));
static FULL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(TEA_FULL_KETTLE_ASK).expect("full kettle pattern"));

pub(super) fn reply(reading: &Reading<'_>) -> Cow<'static, str> {
    let text = if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    }
    .to_lowercase();
    let count = reading.numbers.iter().find_map(|number| {
        let value = number.value.trunc();
        if value < 1.0 || value >= u64::MAX as f64 {
            return None;
        }
        match number.unit? {
            "cup" | "cups" | "mug" | "mugs" | "cuppa" | "cuppas" => Some((value as u64, false)),
            "people" | "person" | "of us" | "guest" | "guests" => Some((value as u64, true)),
            _ => None,
        }
    });
    let count = count.or_else(|| {
        let hit = CUPS.captures(&text)?;
        let said = &hit[1];
        let value = said.parse::<u64>().ok().or_else(|| {
            TEA_COUNT_WORDS
                .iter()
                .find_map(|&(word, value)| (word == said).then_some(value))
        })?;
        let unit = &hit[2];
        let people = ["people", "person", "of us"]
            .iter()
            .any(|word| unit.starts_with(word));
        (value > 0).then_some((value, people))
    });
    let Some((count, people)) = count else {
        return if FULL.is_match(&text) {
            Cow::Borrowed(TEA_FULL_KETTLE)
        } else {
            Cow::Borrowed(
                "A mug of tea is about 300 ml, so count the mugs you are making and fill the kettle just past that: two cups is 600 ml, and a full 1.7-litre kettle is about five mugs. Boiling only what you will drink is the cheapest habit in the kitchen.",
            )
        };
    };
    let amount = u128::from(count) * u128::from(TEA_ML_PER_MUG);
    let measure = match (amount / 1000, amount % 1000 / 100) {
        (0, _) => format!("{amount} ml"),
        (litres, 0) => format!("{litres} litres"),
        (litres, tenths) => format!("{litres}.{tenths} litres"),
    };
    let what = if people {
        format!("{count} people, a mug each")
    } else if count == 1 {
        "one cup".to_owned()
    } else {
        format!("{count} cups")
    };
    format!("About {measure} for {what}, since a mug holds roughly 300 ml and a splash more covers the element. Boiling only what you drink is the cheapest habit in the kitchen.").into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kettle_counts_respect_people_and_measurement_units() {
        assert!(
            reply(&Reading::new(
                "how much water in the kettle for four people"
            ))
            .starts_with("About 1.2 litres for 4 people, a mug each")
        );
        assert!(
            reply(&Reading::new("how much water in the kettle for one cup"))
                .starts_with("About 300 ml for one cup")
        );
        assert_eq!(
            reply(&Reading::new("how many mugs does a full kettle make")),
            TEA_FULL_KETTLE
        );
    }
}
