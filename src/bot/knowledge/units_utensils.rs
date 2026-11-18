//! Fractions of cups and spoons, including follow-ups about the previous portion.

use crate::bot::Exchange;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
include!("units_utensils_data.rs");

pattern!(SLASH, r"\A(\d)\s*/\s*(\d)\z");
pattern!(OF, r"\bof\s*$");
pattern!(UTENSIL, r"\b(?:cups?|tablespoons?|tbsp|teaspoons?|tsp)\b");

fn normalized(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn value(word: &str) -> Option<f64> {
    if let Ok(i) = SHARE_VALUE.binary_search_by_key(&word, |&(key, _)| key) {
        return Some(SHARE_VALUE[i].1);
    }
    let pair = SLASH.captures(word)?;
    let top: f64 = pair[1].parse().ok()?;
    let bottom: f64 = pair[2].parse().ok()?;
    (bottom != 0.).then_some(top / bottom)
}

fn share_name(value: f64) -> Option<&'static str> {
    [
        (0.25, "a quarter"),
        (0.5, "half"),
        (0.75, "three quarters"),
        (1. / 3., "a third"),
        (2. / 3., "two thirds"),
        (0.2, "a fifth"),
        (1. / 6., "a sixth"),
        (0.125, "an eighth"),
    ]
    .into_iter()
    .find_map(|(known, name)| ((value - known).abs() < 0.001).then_some(name))
}

fn spoons(value: f64) -> String {
    let count = value * 16.;
    let mut whole = count.trunc() as i32;
    let mut rest = ((count - f64::from(whole)) * 3.).round_ties_even() as i32;
    if rest >= 3 {
        whole += 1;
        rest = 0;
    }
    let small = if rest == 1 {
        "a teaspoon".to_owned()
    } else {
        format!("{rest} teaspoons")
    };
    if whole == 0 {
        return small;
    }
    let big = if whole == 1 {
        "1 tablespoon".to_owned()
    } else {
        format!("{whole} tablespoons")
    };
    if rest == 0 {
        big
    } else {
        format!("{big} and {small}")
    }
}

fn tsp(count: f64) -> String {
    let nearest = count.round_ties_even();
    if (count - nearest).abs() < 0.05 {
        if nearest == 1. {
            "a teaspoon".into()
        } else {
            format!("{nearest:.0} teaspoons")
        }
    } else {
        format!("about {count:.1} teaspoons")
    }
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

pub(super) fn answer(clean: &str, with_advice: bool) -> Option<String> {
    if !SHARE_ASK.is_match(clean) || NOT_ABOUT_A_MEASURE.is_match(clean) {
        return None;
    }
    let found = SHARE.captures(clean)?;
    let span = found.get(0)?;
    let word = normalized(&found["word"]);
    let mut portion = value(&word).filter(|&x| x > 0.)?;
    let mut said = share_name(portion).unwrap_or(&word).to_owned();
    let before = &clean[..span.start()];
    let after = &clean[span.end()..];
    if OF.is_match(before)
        && let Some(outer) = SHARE_FRACTION.find_iter(before).last()
    {
        let name = normalized(outer.as_str());
        if let Some(outer) = value(&name).filter(|&x| x > 0.) {
            portion *= outer;
            said = format!("{} of {said}", share_name(outer).unwrap_or(&name));
        }
    }
    if UTENSIL.is_match(before) || UTENSIL.is_match(after) {
        return None;
    }
    let unit = &found["unit"];
    if unit.starts_with("cup") {
        let lead = if said == "half" {
            "Half a cup".to_owned()
        } else {
            format!("{} of a cup", upper(&said))
        };
        let advice = if with_advice {
            ": spoons are the easy way to cut a cup down."
        } else {
            "."
        };
        return Some(format!(
            "{lead} is {}, about {:.0} ml{advice}",
            spoons(portion),
            (portion * 240.).round_ties_even()
        ));
    }
    let lead = upper(&said);
    if unit.starts_with("tablespoon") || unit == "tbsp" {
        return Some(format!(
            "{lead} of a tablespoon is {}, about {:.0} ml.",
            tsp(portion * 3.),
            (portion * 15.).round_ties_even()
        ));
    }
    Some(format!(
        "{lead} of a teaspoon is about {:.0} ml.",
        (portion * 5.).round_ties_even()
    ))
}

pub(super) fn carried(clean: &str, turns: &[Exchange]) -> Option<String> {
    let found = SHARE_THAT.captures(clean)?;
    let span = found.get(0)?;
    let rest = format!("{} {}", &clean[..span.start()], &clean[span.end()..]);
    // The source strips this set of characters around conversational lead-ins.
    let trim = |c| matches!(c, ' ' | '?' | '.' | '!' | ',' | 'a' | 'n' | 'd' | 's' | 'o');
    let mut rest = rest.trim_matches(trim);
    while let Some(lead) = SHARE_LEAD.find(rest) {
        rest = rest[lead.end()..].trim_matches(trim);
    }
    if !rest.is_empty() {
        return None;
    }
    let previous = turns
        .iter()
        .rev()
        .take(4)
        .find(|turn| !turn.user.trim().is_empty())?;
    let line = previous.user.trim().to_lowercase();
    let shared = SHARE.find(&line)?;
    answer(
        &format!(
            "what is {} of {}",
            normalized(&found["word"]),
            &line[shared.start()..]
        ),
        false,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    mod reference {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/unit_utensils.rs"
        ));
    }

    #[test]
    fn fractions_and_follow_ups_match_the_reference() {
        for &(text, expected) in reference::DIRECT {
            assert_eq!(answer(text, true).as_deref(), expected, "{text}");
        }
        for &(prior, text, expected) in reference::CARRIED {
            let turn = Exchange {
                user: prior.into(),
                bot: String::new(),
                module: "",
            };
            assert_eq!(
                carried(text, &[turn]).as_deref(),
                expected,
                "{prior} -> {text}"
            );
        }
    }
}
