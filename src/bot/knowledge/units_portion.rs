//! Per-person food allowances and totals for the headcount on the line.

use crate::bot::memory::groupages;
use fancy_regex::Regex as Lookaround;
use regex::Regex;
use std::sync::LazyLock;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Unit {
    Mass,
    Volume,
    Count,
    Bottle,
}

struct Allowance {
    keys: &'static [&'static str],
    amount: f64,
    unit: Unit,
    noun: &'static str,
}

struct Dish {
    keys: &'static [&'static str],
    main: &'static [&'static str],
    side: &'static [&'static str],
    tomatoes: bool,
    spice: bool,
}

macro_rules! pattern {
    ($name:ident, $engine:ty, $value:expr) => {
        static $name: LazyLock<$engine> =
            LazyLock::new(|| <$engine>::new($value).expect(stringify!($name)));
    };
}

include!("units_portion_data.rs");

pattern!(ASK, Regex, r"\bhow (?:much|many|big|large)\b|\bwhat size\b");
pattern!(
    NEED,
    Regex,
    r"\bi (?:need|want)\b|\bi(?:'?m| am) (?:cooking|making)\b"
);
pattern!(TIME, Regex, r"\b(?:minutes?|mins?|hours?|hrs?|seconds?)\b");
pattern!(NEED_A, Regex, r"\bi (?:need|want)\s+(?:a|an|one|some)\b");
pattern!(EGG, Regex, r"\begg");
pattern!(BETWEEN_US, Regex, r"\bbetween\s+us\b");
pattern!(
    AFTER_FOOD,
    Regex,
    r"^\s+(?:dressing|sauce|dip|oil|mayo|mayonnaise|vinaigrette|juice)"
);
pattern!(
    MAKING,
    Regex,
    r"\bi (?:need|want)\s+to\b|\bi(?:'?m| am) (?:cooking|making|baking|roasting|prepping)\b"
);
pattern!(BIRD_SIZE, Regex, r"\bhow (?:big|large)\b|\bwhat size\b");

fn numeric(word: &str) -> Option<u32> {
    groupages::decimal(word)
        .map(u32::from)
        .or_else(|| {
            PEOPLE
                .iter()
                .position(|&name| name == word)
                .map(|i| (i + 1) as u32)
        })
        .or_else(|| (word == "a dozen").then_some(12))
}

pub(in crate::bot::knowledge) fn people_in_groups(clean: &str) -> Option<u32> {
    let grouped: u32 = TABLE_GROUPS
        .captures_iter(clean)
        .filter_map(|hit| numeric(hit.get(1)?.as_str()))
        .sum();
    (grouped > 0).then_some(grouped)
}

fn wanted_people(clean: &str) -> Option<u32> {
    if BETWEEN_US.is_match(clean) {
        return Some(2);
    }
    let grouped = people_in_groups(clean).unwrap_or(0);
    let plain = PEOPLE_COUNT.captures(clean).ok().flatten().and_then(|hit| {
        let said = hit.iter().skip(1).flatten().next()?.as_str();
        (said != "a dozen").then(|| numeric(said)).flatten()
    });
    if grouped == 0 {
        plain
    } else {
        Some(grouped.max(plain.unwrap_or(0)))
    }
}

fn said_people(people: u32) -> String {
    people
        .checked_sub(1)
        .and_then(|index| PEOPLE.get(index as usize))
        .map_or_else(|| people.to_string(), |word| (*word).to_owned())
}

fn word(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

fn name_in<'a>(clean: &'a str, name: &str, allows_tail: impl Fn(&str) -> bool) -> Option<&'a str> {
    clean.match_indices(name).find_map(|(start, found)| {
        let before = clean[..start].chars().next_back();
        let tail = &clean[start + found.len()..];
        (before.is_none_or(|ch| ch != '.' && !word(ch))
            && tail.chars().next().is_none_or(|ch| !word(ch))
            && allows_tail(tail))
        .then_some(found)
    })
}

fn decimal(value: f64) -> String {
    format!("{value:.1}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

fn grams(value: f64) -> String {
    if value.abs() >= 10.0 {
        return format!("{:.0}", value + 1e-9);
    }
    decimal(value)
}

fn scaled(value: f64, small: &str, large: &str) -> String {
    if value >= 1_000.0 {
        return format!("{} {large}", decimal(value / 1_000.0));
    }
    let shown = if small == "g" {
        grams(value)
    } else {
        value.to_string()
    };
    format!("{shown} {small}")
}

fn each(amount: f64, noun: &str, people: Option<u32>, each_person: bool) -> String {
    let plural = if noun.ends_with('s') || noun.contains(' ') {
        noun.to_owned()
    } else {
        format!("{noun}s")
    };
    let count = if amount > 1.0 {
        format!("{amount} {plural}")
    } else {
        format!("1 {noun}")
    };
    if let Some(people) = people {
        let count_all = if amount > 1.0 {
            (amount * f64::from(people)).to_string()
        } else {
            said_people(people)
        };
        return format!(
            "About {count} each, so {count_all} {plural} for {}.",
            said_people(people)
        );
    }
    if each_person {
        format!("About {count} each.")
    } else {
        format!("A portion is about {count}.")
    }
}

fn bottles(amount: f64, noun: &str, people: Option<u32>) -> String {
    let per = if amount == 0.5 {
        "half a bottle".to_owned()
    } else {
        format!("{amount} bottles")
    };
    if let Some(people) = people {
        let whole = (amount * f64::from(people)).ceil();
        let unit = if whole == 1.0 { "bottle" } else { "bottles" };
        return format!(
            "Allow about {per} of {noun} a guest, so {whole} {unit} for {}.",
            said_people(people)
        );
    }
    format!("Allow about {per} of {noun} a guest.")
}

fn measured(amount: f64, noun: &str, kind: Unit, people: Option<u32>, each_person: bool) -> String {
    let (small, large) = if kind == Unit::Mass {
        ("g", "kg")
    } else {
        ("ml", "litres")
    };
    let per = if kind == Unit::Mass {
        grams(amount)
    } else {
        amount.to_string()
    };
    if let Some(people) = people {
        return format!(
            "About {per} {small} of {noun} each, so {} for {}.",
            scaled(amount * f64::from(people), small, large),
            said_people(people)
        );
    }
    if each_person {
        format!("About {per} {small} of {noun} each.")
    } else {
        format!("A portion of {noun} is about {per} {small}.")
    }
}

fn mass_allowance(keys: &[&str]) -> Option<(f64, &'static str)> {
    PER_PERSON
        .iter()
        .chain(PER_PERSON_WHEN_COUNTED)
        .find_map(|row| {
            (row.unit == Unit::Mass
                && row.amount != 0.0
                && keys.iter().any(|key| row.keys.contains(key)))
            .then_some((row.amount, row.noun))
        })
}

fn dish_for_a_table(clean: &str, people: Option<u32>) -> Option<String> {
    let count = people.filter(|value| (2..=60).contains(value))?;
    if A_TIME_IS_THE_ASK.is_match(clean) {
        return None;
    }
    let (found, row) = TABLE_DISHES
        .iter()
        .flat_map(|dish| {
            dish.keys.iter().filter_map(move |name| {
                name_in(clean, name, |rest| {
                    !A_DISH_PART.find(rest).is_some_and(|hit| hit.start() == 0)
                })
                .map(|found| (found, dish))
            })
        })
        .min_by_key(|(name, _)| std::cmp::Reverse(name.len()))?;
    if matches!(found, "chilli" | "chili") && CHILLI_THE_VEGETABLE.is_match(clean) {
        return None;
    }
    let parts: Vec<_> = [row.main, row.side]
        .into_iter()
        .filter_map(mass_allowance)
        .collect();
    if parts.is_empty() {
        return None;
    }
    let each = parts
        .iter()
        .map(|(amount, noun)| format!("{amount} g of {noun}"))
        .collect::<Vec<_>>()
        .join(" and ");
    let total = parts
        .iter()
        .map(|(amount, noun)| {
            let weight = *amount * f64::from(count);
            let weight = if weight >= 1_000.0 {
                format!("{} kg", decimal((weight / 100.0).round_ties_even() / 10.0))
            } else {
                format!("{weight} g")
            };
            format!("{weight} of {noun}")
        })
        .collect::<Vec<_>>()
        .join(" and ");
    let mut reply = format!("For {count}, {found} is about {each} each: {total} in all.");
    if row.tomatoes {
        let tins = (f64::from(count) / 4.0).round_ties_even().max(1.0);
        let noun = if tins == 1.0 { "tin" } else { "tins" };
        reply.push_str(&format!(
            " One 400 g tin of tomatoes does four, so {tins} {noun}."
        ));
    }
    if row.spice {
        reply
            .push_str(" Use a little more spice than the recipe says - a big batch tastes milder.");
    }
    Some(reply)
}

fn per_person(clean: &str, people: Option<u32>, each_person: bool) -> Option<String> {
    let asks = ASK.is_match(clean);
    let portions = PORTION_WORD.is_match(clean);
    if !(asks || NEED.is_match(clean))
        || TIME.is_match(clean)
        || NEED_A.is_match(clean)
        || PORTION_AVOID.is_match(clean)
        || BAKING_PORTION.is_match(clean) && EGG.is_match(clean)
        || people.is_none() && !each_person && (!portions || PORTION_COUNT.is_match(clean))
    {
        return None;
    }
    let counted = if people.is_some() && !CONTAINER.is_match(clean) {
        PER_PERSON_WHEN_COUNTED
    } else {
        &[]
    };
    let found = PER_PERSON
        .iter()
        .chain(counted)
        .flat_map(|row| {
            row.keys.iter().filter_map(move |name| {
                name_in(clean, name, |rest| !AFTER_FOOD.is_match(rest)).map(|name| (name, row))
            })
        })
        .min_by_key(|(name, _)| std::cmp::Reverse(name.len()));
    let Some((name, row)) = found else {
        let people = people.filter(|_| portions)?;
        return Some(format!(
            concat!(
                "A main is about 300 g a head, a plate of pasta or rice 75 to 100 g dry and ",
                "a buffet six to eight pieces a guest, so for {} plan on {} of the main dish. ",
                "Tell me what you are cooking and I will do the sum for the shopping."
            ),
            said_people(people),
            scaled(300.0 * f64::from(people), "g", "kg")
        ));
    };
    if MADE_NOT_MEASURED.contains(&row.noun) && !asks && MAKING.is_match(clean) {
        return None;
    }
    let (amount, noun) = if BIRDS.contains(&name) && BIRD_SIZE.is_match(clean) {
        (BIRD_GRAMS, name)
    } else {
        (row.amount, row.noun)
    };
    let people = people.filter(|value| *value != 0);
    Some(match row.unit {
        Unit::Count => each(amount, noun, people, each_person),
        Unit::Bottle => bottles(amount, noun, people),
        kind => measured(amount, noun, kind, people, each_person),
    })
}

/// Let earlier readers keep weight, oven and package questions outside the portion shapes.
pub(super) fn serving_question(
    clean: &str,
    weights: impl FnOnce() -> Option<String>,
) -> Option<String> {
    let people = wanted_people(clean);
    let each_person = PEOPLE_WORD.is_match(clean).unwrap_or(false);
    if people.is_some() || each_person {
        if (FEEDS_ASK.is_match(clean)
            && WHOLE_SERVES
                .iter()
                .any(|food| name_in(clean, food, |_| true).is_some()))
            || people.is_some() && COUNT_OF_BIRDS.is_match(clean)
        {
            // Whole joints and birds have their own food allowance in the source.
            return weights();
        }
        return dish_for_a_table(clean, people)
            .or_else(|| per_person(clean, people, each_person))
            .or_else(weights);
    }
    weights().or_else(|| per_person(clean, people, each_person))
}

#[cfg(test)]
mod tests {
    use super::serving_question;

    #[test]
    fn scales_an_explicit_table_with_the_pinned_noun_and_units() {
        assert_eq!(
            serving_question("how much food for eight people", || None).as_deref(),
            Some("About 300 g of the dish each, so 2.4 kg for eight.")
        );
        assert_eq!(
            serving_question("how much rice for 3 adults and 2 kids", || None).as_deref(),
            Some("About 75 g of dry rice each, so 375 g for five.")
        );
        assert_eq!(
            serving_question("how many sausages per person", || None).as_deref(),
            Some("About 2 sausages each.")
        );
    }

    #[test]
    fn leaves_weighing_and_whole_birds_to_their_own_readers() {
        assert_eq!(
            serving_question("how much food weighs a kilogram", || None),
            None
        );
        assert_eq!(
            serving_question("how many chickens for 20 people", || None),
            None
        );
        assert_eq!(
            serving_question("how much chilli powder for 8 people", || None),
            None
        );
    }
}
