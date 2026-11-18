//! Convert a named ingredient's weight and cup or spoon volume.

use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

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

#[derive(Clone, Copy)]
struct Measure {
    id: &'static str,
    plural: &'static str,
    singular: &'static str,
    factor: f64,
    is_mass: bool,
}
include!("units_ingredient_data.rs");

pattern!(MUG, r"\bmugs?\b");
pattern!(
    SPOON,
    r"\b(?:tablespoons?|tbsp|teaspoons?|tsp|dessertspoons?|dsp)\b"
);
pattern!(DESSERTSPOON, r"\b(?:dessertspoons?|dsp)\b");
pattern!(TABLESPOON, r"\b(?:tablespoons?|tbsp)\b");
pattern!(TEASPOON, r"\b(?:teaspoons?|tsp)\b");
pattern!(CUP, r"\bcups?\b");
pattern!(
    WEIGHT,
    r"\b(?:grams?|g|kg|kilograms?|oz|ounces?|pounds?|lbs?)\b"
);

fn food_matcher(table: &[(&str, f64)]) -> RegexSet {
    RegexSet::new(
        table
            .iter()
            .map(|(name, _)| format!(r"\b{}\b", regex::escape(name))),
    )
    .expect("unit ingredient names")
}

pattern!(
    PRICE_OR_NUTRITION,
    r"\b(?:cost|costs|price|prices|worth|sell|buy|buying|calories|protein)\b"
);
static LIQUIDS: LazyLock<RegexSet> = LazyLock::new(|| food_matcher(LIQUID_DENSITIES));
static CUP_FOODS: LazyLock<RegexSet> = LazyLock::new(|| food_matcher(GRAMS_PER_CUP));
static SPOON_FOODS: LazyLock<RegexSet> = LazyLock::new(|| food_matcher(GRAMS_PER_TSP));

fn find_food(
    text: &str,
    matcher: &RegexSet,
    names: &'static [(&'static str, f64)],
) -> Option<(&'static str, f64)> {
    // Equal-length aliases retain their original order.
    matcher
        .matches(text)
        .iter()
        .map(|index| names[index])
        .fold(None::<(&str, f64)>, |best, row| {
            if best.is_none_or(|(name, _)| row.0.len() > name.len()) {
                Some(row)
            } else {
                best
            }
        })
}

fn name_as_said(food: &str) -> &str {
    SAID_FOOD
        .iter()
        .find_map(|&(name, said)| (food == name).then_some(said))
        .unwrap_or(food)
}

fn number_before(text: &str, start: usize) -> Option<f64> {
    let begin = text[..start]
        .char_indices()
        .rev()
        .nth(23)
        .map_or(0, |(i, _)| i);
    let window = &text[begin..start];
    let last = NUMBER
        .find_iter(window)
        .flatten()
        .filter(|hit| window[hit.end()..].trim().is_empty())
        .last()?;
    match last.as_str() {
        "a" | "an" | "one" => Some(1.),
        number => super::degrees(&number.replace(',', "")),
    }
}

fn measure_names(text: &str) -> impl Iterator<Item = (usize, Measure)> + '_ {
    UNIT_WORD.find_iter(text).flatten().filter_map(|hit| {
        let i = UNIT_NAMES
            .binary_search_by_key(&hit.as_str(), |&(alias, _)| alias)
            .ok()?;
        Some((hit.start(), UNIT_NAMES[i].1?))
    })
}

fn grams(value: f64) -> String {
    if value.abs() >= 10. {
        crate::bot::knowledge::maths::number((value + 1e-9).round_ties_even())
    } else {
        format!("{value:.1}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
    }
}

fn spoons(value: f64) -> String {
    let rounded = (value * 2.).round_ties_even() / 2.;
    if rounded.fract() == 0. {
        format!("{rounded:.0}")
    } else {
        format!("{:.0} 1/2", rounded.trunc())
    }
}

fn cup_phrase(value: f64, unit: Measure, with_spoons: bool) -> String {
    if unit.id == "cup" {
        if let Some(&(_, said)) = FRACTIONS
            .iter()
            .find(|&&(count, _)| (value - count).abs() < 0.06)
        {
            return said.into();
        }
        if (value - 1.).abs() < 0.06 {
            return "1 cup".into();
        }
        if with_spoons && 0. < value && value < 0.5 {
            let count = value * 16.;
            if count < 2. {
                return format!("{} teaspoons", spoons(count * 3.));
            }
            if count < 10. {
                return format!("{} tablespoons", spoons(count));
            }
        }
        if value >= 0.1 {
            return format!("{} cups", super::say((value * 10.).round_ties_even() / 10.));
        }
    }
    format!("{} {}", super::say(value), unit.plural)
}

fn amount_phrase(value: f64, unit: Measure) -> String {
    if !unit.is_mass {
        let phrase = cup_phrase(value, unit, false);
        if phrase != format!("{} {}", super::say(value), unit.plural) {
            return phrase;
        }
    }
    if let Some(leading) = [
        (0.25, "a quarter of"),
        (0.5, "half"),
        (0.75, "three quarters of"),
    ]
    .iter()
    .find_map(|&(amount, word)| (value == amount).then_some(word))
    {
        let article = if unit.singular.starts_with(['a', 'e', 'i', 'o', 'u']) {
            "an"
        } else {
            "a"
        };
        return format!("{leading} {article} {}", unit.singular);
    }
    format!(
        "{} {}",
        super::say(value),
        if value == 1. {
            unit.singular
        } else {
            unit.plural
        }
    )
}

fn upper(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn named_conversion(clean: &str, food: &str, grams_per_cup: f64) -> Option<String> {
    let mentions: Vec<_> = measure_names(clean).collect();
    let (_, first_volume) = mentions.iter().copied().find(|(_, unit)| !unit.is_mass)?;
    let (_, first_mass) = mentions.iter().copied().find(|(_, unit)| unit.is_mass)?;
    for (offset, unit) in mentions {
        let Some(value) = number_before(clean, offset) else {
            continue;
        };
        let food = name_as_said(food);
        let source = upper(&format!("{} of {food}", amount_phrase(value, unit)));
        let text = if unit.is_mass {
            let volume = value * unit.factor / (grams_per_cup * first_volume.factor);
            cup_phrase(volume, first_volume, true)
        } else {
            let weight = value * grams_per_cup * unit.factor / first_mass.factor;
            let count = if first_mass.id == "g" {
                grams(weight)
            } else {
                super::say(weight)
            };
            format!("{count} {}", first_mass.plural)
        };
        return Some(format!("{source} is about {text}."));
    }
    None
}

fn named_weight(clean: &str, food: Option<(&str, f64)>) -> Option<String> {
    if PER_CUP.is_match(clean) {
        return None;
    }
    let (measure, share) = if MUG.is_match(clean) {
        ("mug", 240. / 250.)
    } else if TABLESPOON.is_match(clean) {
        ("tablespoon", 16.)
    } else if TEASPOON.is_match(clean) {
        ("teaspoon", 48.)
    } else if DESSERTSPOON.is_match(clean) {
        ("dessertspoon", 24.)
    } else if CUP.is_match(clean) {
        ("cup", 1.)
    } else {
        return None;
    };
    if SPOON.is_match(clean)
        && !matches!(measure, "mug" | "cup")
        && let Some((name, grams)) = find_food(clean, &SPOON_FOODS, GRAMS_PER_TSP)
    {
        return Some(format!(
            "A {measure} of {name} is about {} g.",
            super::say(grams * (240. / share) / 5.)
        ));
    }
    let (food, density) = food?;
    if POUNDS_IS_MONEY.is_match(clean) || !(WEIGH.is_match(clean) || WEIGHT.is_match(clean)) {
        return None;
    }
    let amount = if share == 1. {
        density
    } else {
        (density / share).round_ties_even().max(1.)
    };
    Some(format!(
        "A {measure} of {} is about {} g.",
        name_as_said(food),
        grams(amount)
    ))
}

pub(super) fn liquid_weight(clean: &str) -> Option<String> {
    if PER_CUP.is_match(clean)
        || POUNDS_IS_MONEY.is_match(clean)
        || PRICE_OR_NUTRITION.is_match(clean)
        || !(WEIGH.is_match(clean) || WEIGHT.is_match(clean))
    {
        return None;
    }
    let matched = LIQUIDS.matches(clean).iter().next()?;
    let (liquid, density) = LIQUID_DENSITIES[matched];
    let volume = UNIT_WORD.find_iter(clean).flatten().find_map(|hit| {
        let i = LIQUID_UNIT_NAMES
            .binary_search_by_key(&hit.as_str(), |&(alias, _)| alias)
            .ok()?;
        let unit = LIQUID_UNIT_NAMES[i].1;
        let value = number_before(clean, hit.start())?;
        let said = if value == 1. {
            format!("a {}", unit.singular)
        } else {
            amount_phrase(value, unit)
        };
        Some((value * unit.factor, said))
    });
    let (millilitres, said) =
        volume.or_else(|| MUG.is_match(clean).then(|| (250., "a mug".into())))?;
    let mass = millilitres * density / 1000.;
    let weight = if mass < 1000. {
        format!("{} g", grams(mass))
    } else {
        let kilos = mass / 1000.;
        let precision = if kilos < 2. { 2 } else { 1 };
        let rendered = format!("{kilos:.precision$}");
        // The source keeps one fractional digit at large weights and inserts
        // a separator for thousands before removing trailing decimal zeroes.
        let formatted = if kilos >= 1000. {
            let (whole, fraction) = rendered.split_once('.')?;
            format!(
                "{}.{fraction}",
                crate::bot::knowledge::maths::number(whole.parse().ok()?)
            )
        } else {
            rendered
        };
        format!(
            "{} kg",
            formatted.trim_end_matches('0').trim_end_matches('.')
        )
    };
    Some(format!("{} of {liquid} weighs {weight}.", upper(&said)))
}

fn together(food: &str, other: &str, short_name: bool) -> bool {
    other == food
        || other.strip_suffix('s') == Some(food)
        || short_name && food.split_whitespace().next_back() == Some(other)
}

fn read_amount(clean: &str) -> Option<Cow<'_, str>> {
    let component = COMPONENT_MEASURED.captures(clean).filter(|hit| {
        hit.name("again")
            .is_none_or(|other| together(&hit["component"], other.as_str(), false))
    });
    if let Some(dry) = DRY_IN_A_MEASURE.captures(clean)
        && dry
            .name("again")
            .is_none_or(|other| together(&dry["food"], other.as_str(), true))
    {
        return Some(Cow::Owned(format!(
            "how many grams in a {} of {}",
            &dry["measure"], &dry["food"]
        )));
    }
    if let Some(component) = component {
        if component["measure"].starts_with("cup") {
            return Some(Cow::Owned(format!(
                "how many grams in a {} of {}",
                &component["measure"], &component["component"]
            )));
        }
    } else if super::COMPONENT.is_match(clean) {
        return None;
    }
    Some(Cow::Borrowed(clean))
}

pub(super) fn answer(clean: &str) -> Option<String> {
    if !CUP.is_match(clean) && !MUG.is_match(clean) && !SPOON.is_match(clean) {
        return None;
    }
    let clean = read_amount(clean)?;
    let food = find_food(&clean, &CUP_FOODS, GRAMS_PER_CUP);
    food.and_then(|(name, weight)| named_conversion(&clean, name, weight))
        .or_else(|| named_weight(&clean, food))
}

#[cfg(test)]
mod tests {
    mod reference {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/unit_ingredients.rs"
        ));
    }
    #[test]
    fn liquid_weights_match_the_reference() {
        for &(text, expected) in reference::LIQUIDS {
            assert_eq!(super::liquid_weight(text).as_deref(), expected, "{text}");
        }
    }

    #[test]
    fn ingredient_measurements_match_the_reference() {
        let differences: Vec<_> = reference::CASES
            .iter()
            .filter_map(|&(text, expected)| {
                let actual = super::answer(text);
                (actual.as_deref() != expected).then_some((text, expected, actual))
            })
            .collect();
        assert!(
            differences.is_empty(),
            "{} differences: {:?}",
            differences.len(),
            &differences[..differences.len().min(18)]
        );
    }
}
