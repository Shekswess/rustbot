//! Dish nutrition verdicts and food to bring to someone else's table.

use super::{food_occasions_data as data, guards::Context, meal_windows::Limits, travel_airport};
use crate::bot::Reading;
use std::borrow::Cow;

fn phrase(text: &str, word: &str) -> bool {
    text.contains(&format!(" {word} ")) || text.contains(&format!(" {word}s "))
}

fn any_substring(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

pub(super) fn verdict(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let plain: String = reading
        .line
        .clean
        .to_ascii_lowercase()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '\'' {
                ch
            } else {
                ' '
            }
        })
        .collect();
    let said = format!(
        " {} ",
        plain.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    let mut remainder = said.clone();
    for (name, _) in data::VERDICTS {
        if phrase(&remainder, name) {
            remainder = remainder
                .replace(&format!(" {name} "), " ")
                .replace(&format!(" {name}s "), " ");
        }
    }
    if data::VERDICT_FENCE
        .iter()
        .any(|word| phrase(&remainder, word))
    {
        return None;
    }
    data::VERDICTS
        .iter()
        .find(|(name, _)| phrase(&said, name))
        .map(|(_, reply)| Cow::Borrowed(*reply))
}

fn equivalent(actual: &str, expected: &str) -> bool {
    let actual = actual.trim_matches('\'');
    actual == expected
        || [(actual, expected), (expected, actual)]
            .into_iter()
            .any(|(longer, shorter)| {
                longer.strip_suffix("s") == Some(shorter)
                    || longer.strip_suffix("es") == Some(shorter)
            })
}

fn names(words: &[&str], name: &str) -> bool {
    let name: Vec<_> = name.split_whitespace().collect();
    words.windows(name.len()).any(|words| {
        words
            .iter()
            .zip(&name)
            .all(|(actual, expected)| equivalent(actual, expected))
    })
}

fn bring_hint(reading: &Reading<'_>) -> Option<&'static str> {
    let clean = reading.line.clean.to_lowercase();
    if any_substring(&clean, data::NOT_A_DISH)
        || any_substring(&clean, data::HOST_OWN)
        || reading.wants == Some("place")
    {
        return None;
    }
    let clean_words = clean
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '\'')
        .filter(|word| !word.is_empty());
    let words: Vec<_> = reading
        .line
        .words
        .iter()
        .map(String::as_str)
        .chain(clean_words)
        .collect();
    if any_substring(&clean, data::MAKE_ASKS)
        && clean
            .split_whitespace()
            .any(|word| data::SPREAD_WORDS.contains(&word))
        && !words
            .iter()
            .any(|word| equivalent(word, "bring") || equivalent(word, "take"))
    {
        return None;
    }
    let carries = data::CARRY_WORDS.iter().any(|word| names(&words, word));
    if !carries && any_substring(&clean, data::COOKING_ASKS) {
        return None;
    }
    for (group, hint) in data::OCCASIONS {
        if group
            .iter()
            .any(|name| (carries || !data::HOST_NAMES.contains(name)) && names(&words, name))
        {
            return Some(hint);
        }
    }
    if any_substring(&clean, data::BRING_ASKS)
        && !any_substring(&clean, data::NOT_A_DISH_TO_CARRY)
        && !any_substring(&clean, data::TRIP)
        && !travel_airport::food_border(reading)
        && data::GENERIC_FOOD
            .iter()
            .any(|food| words.iter().any(|word| equivalent(word, food)))
    {
        return Some(data::ANY_DISH);
    }
    None
}

pub(super) fn bring(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let reply = bring_hint(context.reading)?;
    let limits = Limits::new(context, false);
    if limits.restricted() {
        return Some(Cow::Borrowed(
            "Ask the host which dishes and ingredients fit everyone's dietary requirements. Bring a dish you can put together with ingredients they have confirmed, and include the ingredient list. If anyone has a food allergy, check labels and ask what precautions they need to avoid cross-contact. Keep perishable food chilled until serving.",
        ));
    }
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_picks_the_long_alias_and_excludes_the_person_or_pet() {
        assert!(
            verdict(&Reading::new("Is fish and chips good for you?"))
                .unwrap()
                .starts_with("Fish and chips")
        );
        assert!(
            verdict(&Reading::new("Is a hot dog bad for you?"))
                .unwrap()
                .starts_with("A hot dog")
        );
        assert!(verdict(&Reading::new("Is a hot dog bad for my dog?")).is_none());
        assert!(
            verdict(&Reading::new("Is a vegan burger good for you?"))
                .unwrap()
                .starts_with("Vegan burgers")
        );
    }

    #[test]
    fn bring_prefers_the_first_occasion_and_checks_known_constraints() {
        let ask = Reading::new("What should I bring to a wake and a picnic?");
        assert!(bring_hint(&ask).unwrap().starts_with("Sandwiches"));
        assert!(bring_hint(&Reading::new("I'm hosting a barbecue, what should I make?")).is_none());
        assert!(
            bring_hint(&Reading::new(
                "What food can I bring into the UK from France?"
            ))
            .is_none()
        );
        let ask = Reading::new("What food should I bring to a picnic with a nut allergy?");
        assert!(
            bring(&Context::new(&ask, &[]))
                .unwrap()
                .contains("cross-contact")
        );
    }
}
