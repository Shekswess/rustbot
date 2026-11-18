//! Food spoilage guards and raw vegetable replies.

use super::{FOOD_INGREDIENT_WORDS, still_edible_data as data};
use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

static DAYS_AGO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::DAYS_AGO).expect("valid leftover age pattern"));
static MONEY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::MONEY_IN_THE_LINE).expect("valid leftovers money fence"));
static DISH_FAULT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::A_FAULT_IN_THE_DISH).expect("valid dish-fault fence"));
static ASK_END: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::ENDS_ON_AN_ASK).expect("valid leftovers verdict ending"));

const KEEP_LEFTOVERS: &str = "Cool cooked leftovers within two hours and keep them covered in the fridge at 5°C or below. Use them within 48 hours or freeze them. If reheating, heat only the part you will eat, once, until it is steaming hot throughout. For cooked rice, cool it within an hour and use it within 24 hours.";

fn contains(text: &str, names: &[&str]) -> bool {
    names.iter().any(|name| text.contains(name))
}

fn names(text: &str, names: &[&str]) -> bool {
    names.iter().any(|name| text.contains(&format!(" {name} ")))
}

fn word_line(text: &str) -> String {
    let plain: String = text
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '\'' {
                ch.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    format!(
        " {} ",
        plain.split_whitespace().collect::<Vec<_>>().join(" ")
    )
}

pub(super) fn not_mince_keeping(reading: &Reading<'_>) -> bool {
    let line = word_line(&reading.line.text);
    !names(&line, data::MINCE_WORDS)
        || names(&line, data::MINCE_VERDICT_WORDS)
        || !names(&line, data::MINCE_KEEPING_WORDS)
}

pub(super) fn meat(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = word_line(&reading.line.clean);
    let base = "Follow the meat's use-by date and any instructions for storage after opening. Keep raw meat covered on the bottom shelf of the fridge. A colour change alone does not tell you it is safe.";
    let spoilage = names(
        &line,
        &[
            "sour",
            "putrid",
            "slimy",
            "tacky",
            "green patches",
            "smells off",
            "gone off",
            "gone bad",
        ],
    );
    let reply = if spoilage {
        format!("Discard meat that smells off or feels slimy. {base}")
    } else if names(&line, data::MINCE_WORDS) {
        format!(
            "{base} Cook mince thoroughly, and discard it if its use-by date has passed or you cannot verify how it was stored."
        )
    } else {
        format!("{base} Discard it if it smells sour, feels slimy or is past its use-by date.")
    };
    Some(Cow::Owned(reply))
}

pub(super) fn no_named_food(reading: &Reading<'_>) -> bool {
    let line = word_line(&reading.line.clean);
    !reading
        .line
        .words
        .iter()
        .any(|word| data::KNOWN_FOODS.binary_search(&word.as_str()).is_ok())
        && !names(&line, data::KNOWN_FOODS)
        && !names(&line, FOOD_INGREDIENT_WORDS)
}

pub(super) fn unnamed(_: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(
        "Discard food that smells sour or otherwise off. Cooking cannot make spoiled food safe. Food can also be unsafe without a change in smell, so follow the use-by date and consider how long it has been stored and at what temperature.",
    ))
}

pub(super) fn raw_veg(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = word_line(&reading.line.clean.replace("'s", " "));
    data::RAW_VEGETABLES
        .iter()
        .find(|(name, _)| names(&line, &[*name]))
        .map(|(_, reply)| Cow::Borrowed(*reply))
}

pub(super) fn not_money(reading: &Reading<'_>) -> bool {
    [&reading.line.clean, &reading.line.text, &reading.line.plain]
        .into_iter()
        .all(|text| !MONEY.is_match(&text.to_lowercase()))
}

pub(super) fn long_leftovers(_: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Owned(format!(
        "Discard the leftovers if they have been in the fridge for a week or longer. Reheating does not make old leftovers safe. {KEEP_LEFTOVERS}"
    )))
}

pub(super) fn wants_keeping(reading: &Reading<'_>) -> bool {
    let line = &reading.line.clean;
    !DISH_FAULT.is_match(line)
        && !contains(line, data::COOKING_ASK)
        && (contains(line, data::KEEPING_IN_THE_LINE) || contains(line, data::PAST_KEEPING))
}

fn days_ago(line: &str) -> Option<u32> {
    if let Some(capture) = DAYS_AGO.captures(line) {
        let said = capture.iter().skip(1).flatten().next()?.as_str();
        return said.parse().ok().or_else(|| {
            data::COUNT_WORDS
                .iter()
                .find(|(word, _)| *word == said)
                .map(|(_, count)| *count)
        });
    }
    data::DAYS_SAID
        .iter()
        .find(|(phrase, _)| line.contains(phrase))
        .map(|(_, days)| *days)
}

pub(super) fn leftovers(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = &reading.line.clean;
    let asked =
        reading.raw.contains('?') || ASK_END.is_match(line) || contains(line, data::VERDICT_ASKS);
    if !asked {
        return Some(Cow::Borrowed(KEEP_LEFTOVERS));
    }
    let verdict = match days_ago(line) {
        Some(0 | 1) => "Use it if it was cooled promptly and kept chilled.",
        Some(2) => {
            "Check when it was first cooked. Use it only if it was cooled promptly, stayed chilled and is still within 48 hours. Cooked rice has a shorter limit."
        }
        Some(_) => {
            "Discard it. It has been in the fridge longer than the 48-hour limit for cooked leftovers, and reheating cannot make it safe."
        }
        None => return Some(Cow::Borrowed(KEEP_LEFTOVERS)),
    };
    Some(Cow::Owned(format!("{verdict} {KEEP_LEFTOVERS}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_veg_uses_longer_names_and_package_guidance() {
        assert!(
            raw_veg(&Reading::new("Can I eat raw sweet potato?"))
                .unwrap()
                .contains("sweet potato")
        );
        assert!(
            raw_veg(&Reading::new("Can I eat raw cassava?"))
                .unwrap()
                .contains("cyanide")
        );
        assert!(
            raw_veg(&Reading::new("Can I eat dried kidney beans raw?"))
                .unwrap()
                .contains("30 minutes")
        );
    }

    #[test]
    fn spoiling_food_disambiguates_named_meat_from_generic_and_finances() {
        assert!(!no_named_food(&Reading::new(
            "The soured cream smells off, can I use it?"
        )));
        assert!(no_named_food(&Reading::new(
            "It smells off, can I still cook it?"
        )));
        assert!(!not_money(&Reading::new("I get £200 left over each month")));
        assert!(!not_mince_keeping(&Reading::new(
            "Is mince ok the next day?"
        )));
        assert!(not_mince_keeping(&Reading::new(
            "Is this mince still safe?"
        )));
    }

    #[test]
    fn leftover_verdict_enforces_the_48_hour_limit_and_yields_to_recipes() {
        assert!(
            leftovers(&Reading::new("Can I eat stew from 3 days ago?"))
                .unwrap()
                .starts_with("Discard it")
        );
        assert!(
            leftovers(&Reading::new("Can I eat stew from 2 days ago?"))
                .unwrap()
                .starts_with("Check when")
        );
        assert!(!wants_keeping(&Reading::new(
            "What can I make with leftover curry?"
        )));
        assert!(wants_keeping(&Reading::new("Is leftover stew still good?")));
    }
}
