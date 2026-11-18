//! Portion estimates derived from the pinned food and nutrition tables.

use super::nutrition_card_data as data;
use crate::bot::{Reading, understanding::normalize::one_edit};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

pub(super) struct Calories {
    pub(super) name: &'static str,
    pub(super) subject: &'static str,
    pub(super) portion: &'static str,
    pub(super) count: Option<i64>,
}

pub(super) struct Nutrient {
    pub(super) name: &'static str,
    pub(super) subject: &'static str,
    pub(super) portion: &'static str,
    pub(super) amounts: &'static [(&'static str, f64)],
}

macro_rules! pattern {
    ($label:ident, $source:literal) => {
        static $label: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($label)));
    };
}

pattern!(WORD, r"[a-z0-9']+");
pattern!(PORTION_SIZE, r"([0-9]+(?:\.[0-9]+)?)\s*(g|ml)\b");
pattern!(PINT, r"\b(half a pint|a pint|one pint)\b");
pattern!(
    AMOUNT,
    r"[0-9]|\b(?:half|quarter|dozen|pints?|litres?|liters?|bowls?|plates?|glasses?|mugs?|cups?|slices?|pieces?|portions?|servings?|tins?|spoons?|tablespoons?|teaspoons?)\b"
);

struct Scale {
    pattern: Regex,
    multiple: f64,
    unit: &'static str,
}

static MEASURES: LazyLock<Vec<Scale>> = LazyLock::new(|| {
    [
        ("kilograms?|kg", 1000., "g"),
        ("grammes?|grams?|g", 1., "g"),
        ("millilitres?|milliliters?|ml", 1., "ml"),
        ("centilitres?|cl", 10., "ml"),
        ("litres?|liters?|l", 1000., "ml"),
        ("pints?", 568., "ml"),
    ]
    .into_iter()
    .map(|(words, multiple, unit)| Scale {
        pattern: Regex::new(&format!(r"([0-9]+(?:\.[0-9]+)?)\s*(?:{words})\b"))
            .expect("nutrition units"),
        multiple,
        unit,
    })
    .collect()
});

const COUNTS: &[&str] = &[
    "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];
const PROTEIN_SHARES: &[(f64, &str)] = &[
    (0.8, "most"),
    (0.5, "about half"),
    (0.3, "about a third"),
    (0.22, "about a quarter"),
    (0.18, "about a fifth"),
    (0.08, "about a tenth"),
];

fn words(text: &str) -> Vec<&str> {
    WORD.find_iter(text).map(|word| word.as_str()).collect()
}

fn bounded(line: &str, term: &str, plural: bool) -> bool {
    let bytes = line.as_bytes();
    line.match_indices(term).any(|(at, _)| {
        let end = at + term.len();
        let before = bytes.get(at.wrapping_sub(1));
        let after = bytes.get(end);
        let no_left = before.is_none_or(|c| !c.is_ascii_alphanumeric());
        let no_right = after.is_none_or(|c| !c.is_ascii_alphanumeric())
            || (plural
                && after == Some(&b's')
                && bytes
                    .get(end + 1)
                    .is_none_or(|c| !c.is_ascii_alphanumeric()));
        no_left && no_right
    })
}

fn plural(word: &str) -> String {
    if !word.bytes().all(|c| c.is_ascii_alphabetic()) || word.ends_with('s') {
        word.to_owned()
    } else if ["ch", "sh", "ss", "x", "z"]
        .iter()
        .any(|suffix| word.ends_with(suffix))
        || matches!(word, "potato" | "tomato" | "hero")
    {
        format!("{word}es")
    } else if let Some(stem) = word.strip_suffix('y')
        && !stem.ends_with(['a', 'e', 'i', 'o', 'u'])
    {
        format!("{stem}ies")
    } else {
        format!("{word}s")
    }
}

fn count_before(words: &[&str], at: usize) -> f64 {
    let before = &words[at.saturating_sub(4)..at];
    let said = before.join(" ");
    if said.ends_with("couple of") {
        2.
    } else if said.ends_with("a few") {
        3.
    } else if said.ends_with("half a") || said.ends_with("half an") {
        0.5
    } else {
        before
            .iter()
            .rev()
            .find_map(|word| {
                word.parse::<u8>()
                    .ok()
                    .filter(|count| (1..=10).contains(count))
                    .map(f64::from)
                    .or_else(|| {
                        COUNTS
                            .iter()
                            .position(|name| name.eq_ignore_ascii_case(word))
                            .map(|i| (i + 1) as f64)
                    })
            })
            .unwrap_or(1.)
    }
}

fn squash(word: &str) -> Vec<u8> {
    let mut letters = word.as_bytes().to_owned();
    letters.dedup();
    letters
}

fn misspelled(word: &str, names: &[&'static str]) -> Option<&'static str> {
    if word.len() < 5 || !word.bytes().all(|c| c.is_ascii_alphabetic()) || names.contains(&word) {
        return None;
    }
    let squashed = squash(word);
    names.iter().copied().find(|name| {
        !name.contains(' ')
            && name.len() >= 5
            && name.as_bytes().first() == word.as_bytes().first()
            && ((name.len() == word.len() && squashed == squash(name))
                || (name.as_bytes().last() == word.as_bytes().last() && one_edit(name, word)))
    })
}

fn compound(words: &[&str], start: usize, end: usize, dishes: &[&str], joins: &[&str]) -> bool {
    words
        .get(end)
        .is_some_and(|word| dishes.contains(word) || joins.contains(word))
        || start
            .checked_sub(1)
            .is_some_and(|before| joins.contains(&words[before]))
}

fn slipped(words: &[&str], names: &[&'static str]) -> Option<(&'static str, f64)> {
    let (at, name) = words
        .iter()
        .enumerate()
        .find_map(|(at, word)| misspelled(word, names).map(|name| (at, name)))?;
    (!compound(words, at, at + 1, data::NUTRIENT_DISH, data::NUTRIENT_JOIN))
        .then(|| (name, count_before(words, at)))
}

fn initial_upper(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

fn names_food(line: &str) -> bool {
    data::CALORIES
        .iter()
        .chain(data::MEALS)
        .any(|row| line.contains(row.name))
        || data::NUTRIENTS.iter().any(|row| line.contains(row.name))
}

fn meal_tail_is_dish(row: &Calories, words: &[&str]) -> bool {
    let Some(tail) = row.name.split_whitespace().last() else {
        return false;
    };
    words
        .iter()
        .position(|word| *word == tail)
        .and_then(|at| words.get(at + 1))
        .is_some_and(|after| data::CALORIE_DISH.contains(after))
}

fn calorie_answer(reading: &Reading<'_>) -> Option<String> {
    let line = &reading.line;
    let text = if !line.plain.is_empty() && line.plain != line.clean && !names_food(&line.clean) {
        &line.plain
    } else {
        &line.clean
    };
    if !data::CALORIE_ASKS.iter().any(|word| text.contains(word))
        || data::CALORIE_OWN.iter().any(|food| text.contains(food))
        || data::CALORIE_NOT_A_DISH
            .iter()
            .any(|food| bounded(text, food, false))
    {
        return None;
    }
    let words = words(text);
    let longer = |best: &'static Calories, next: &'static Calories| {
        if next.name.len() > best.name.len() {
            next
        } else {
            best
        }
    };
    if let Some(row) = data::MEALS
        .iter()
        .filter(|row| bounded(text, row.name, false) && !meal_tail_is_dish(row, &words))
        .reduce(longer)
    {
        return Some(format!(
            "{} is about {} calories ({}).",
            initial_upper(row.subject),
            row.count?,
            row.portion
        ));
    }
    let matches: Vec<_> = data::CALORIES
        .iter()
        .filter(|row| bounded(text, row.name, true))
        .collect();
    let row = if let Some(row) = matches.iter().copied().reduce(longer) {
        if matches.iter().any(|food| !row.name.contains(food.name)) {
            return None;
        }
        row
    } else {
        let names: Vec<_> = data::CALORIES.iter().map(|row| row.name).collect();
        let (name, _) = slipped(&words, &names)?;
        data::CALORIES.iter().find(|row| row.name == name)?
    };
    let dimensions: Vec<_> = row.name.split_whitespace().collect();
    let tail = dimensions.last()?;
    if let Some(at) = words.iter().rposition(|word| word == tail)
        && compound(
            &words,
            (at + 1).saturating_sub(dimensions.len()),
            at + 1,
            data::CALORIE_DISH,
            data::CALORIE_JOIN,
        )
    {
        return None;
    }
    let verb = if data::CALORIE_PLURAL.contains(&row.name)
        && !(row.subject.starts_with("a ") || row.subject.starts_with("an "))
    {
        "are"
    } else {
        "is"
    };
    Some(format!(
        "{} {verb} about {} calories ({}).",
        initial_upper(row.subject),
        row.count?,
        row.portion
    ))
}

fn column(text: &str) -> Option<&'static str> {
    data::NUTRIENT_WORDS
        .iter()
        .find_map(|&(word, name)| bounded(text, word, false).then_some(name))
}

pub(super) fn names_nutrient_food(text: &str) -> bool {
    nutrient_food(text).is_some()
}

fn nutrient_food(text: &str) -> Option<(&'static Nutrient, f64)> {
    let matches: Vec<_> = data::NUTRIENTS
        .iter()
        .filter(|food| bounded(text, food.name, true))
        .collect();
    let words = words(text);
    let Some(best) = matches.iter().copied().reduce(|best, next| {
        if next.name.len() > best.name.len() {
            next
        } else {
            best
        }
    }) else {
        let names: Vec<_> = data::NUTRIENTS.iter().map(|row| row.name).collect();
        let (name, count) = slipped(&words, &names)?;
        return data::NUTRIENTS
            .iter()
            .find(|food| food.name == name)
            .map(|food| (food, count));
    };
    if matches.iter().any(|food| {
        !best.name.contains(food.name)
            && !best.name.contains(food.name.trim_end_matches('s'))
            && !best.name.contains(&plural(food.name))
    }) {
        return None;
    }
    let names: Vec<_> = best.name.split_whitespace().collect();
    let tail = names.last()?;
    let tail_plural = plural(tail);
    for (at, window) in words.windows(names.len()).enumerate() {
        let end = names.len() - 1;
        if window[..end] != names[..end] || (window[end] != *tail && window[end] != tail_plural) {
            continue;
        }
        if compound(
            &words,
            at,
            at + names.len(),
            data::NUTRIENT_DISH,
            data::NUTRIENT_JOIN,
        ) {
            return None;
        }
        return Some((best, count_before(&words, at)));
    }
    None
}

fn portion_size(portion: &str) -> Option<(f64, &str)> {
    let matched = PORTION_SIZE.captures(portion)?;
    Some((
        matched.get(1)?.as_str().parse().ok()?,
        matched.get(2)?.as_str(),
    ))
}

fn scaled_portion<'a>(text: &'a str, base: (f64, &str)) -> Option<(f64, &'a str)> {
    if let Some((amount, spelling)) = MEASURES
        .iter()
        .filter(|scale| scale.unit == base.1)
        .find_map(|scale| {
            let hit = scale.pattern.captures(text)?;
            let amount: f64 = hit.get(1)?.as_str().parse().ok()?;
            Some((amount * scale.multiple, hit.get(0)?.as_str()))
        })
    {
        return Some((amount / base.0, spelling));
    }
    if base.1 != "ml" {
        return None;
    }
    let spelling = PINT.find(text)?.as_str();
    let amount = if spelling.starts_with("half") {
        284.
    } else {
        568.
    };
    Some((amount / base.0, spelling))
}

fn unsupported_amount(text: &str, row: &Nutrient) -> bool {
    let portion = row.portion.to_lowercase();
    let subject = row.subject.to_lowercase();
    AMOUNT
        .find_iter(text)
        .map(|matched| matched.as_str())
        .any(|word| {
            let plural = plural(word);
            !portion.contains(word)
                && !portion.contains(&plural)
                && !subject.contains(word)
                && !subject.contains(&plural)
        })
}

fn counted_subject(subject: &str, count: f64) -> String {
    let mut parts: Vec<_> = subject.split_whitespace().collect();
    if parts
        .first()
        .is_some_and(|part| matches!(*part, "a" | "an"))
    {
        parts.remove(0);
    }
    let noun = parts
        .iter()
        .position(|&word| word == "of")
        .and_then(|index| index.checked_sub(1))
        .unwrap_or(parts.len().saturating_sub(1));
    let Some(word) = parts.get(noun) else {
        return initial_upper(subject);
    };
    let plural_word = plural(word);
    parts[noun] = &plural_word;
    let label = if count.fract() == 0. && (1.0..=10.0).contains(&count) {
        Cow::Borrowed(COUNTS[count as usize - 1])
    } else {
        Cow::Owned(count.to_string())
    };
    format!("{label} {}", parts.join(" "))
}

fn figure(value: f64) -> String {
    if value >= 10. {
        value.round_ties_even().to_string()
    } else {
        format!("{value:.1}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
    }
}

fn nutrient_answer(reading: &Reading<'_>) -> Option<String> {
    let clean = &reading.line.clean;
    if clean.is_empty() {
        return None;
    }
    let plain = &reading.line.plain;
    let (column, text) = column(clean)
        .map(|column| (column, clean))
        .or_else(|| column(plain).map(|column| (column, plain)))?;
    let alternate = if text == clean { plain } else { clean };
    let (row, count) = nutrient_food(text).or_else(|| {
        (plain != clean && !plain.is_empty())
            .then(|| nutrient_food(alternate))
            .flatten()
    })?;
    let nutrient = row.amounts.iter().find(|(name, _)| *name == column)?.1;
    let mut value = nutrient * count;
    let mut portion = row.portion;
    let mut display = initial_upper(row.subject);
    let mut scaled = false;
    if count > 1.0 {
        display = counted_subject(row.subject, count);
        portion = "";
    } else if count == 1.0 {
        if let Some(base) = portion_size(portion) {
            if let Some((multiple, spelling)) = scaled_portion(clean, base)
                && (multiple - 1.).abs() >= 0.01
            {
                display = initial_upper(spelling);
                if !display.to_lowercase().contains(row.name) {
                    display.push_str(" of ");
                    display.push_str(row.name);
                }
                value *= multiple;
                scaled = true;
            }
        } else if unsupported_amount(clean, row) {
            return None;
        }
    }
    let plural = count > 1.0
        || data::CALORIE_PLURAL.contains(&row.name)
        || !(row.subject.starts_with("a ") || row.subject.starts_with("an "));
    let verb = if plural { "have" } else { "has" };
    let label = if column == "carbs" {
        "carbohydrate"
    } else {
        column
    };
    let unit = if matches!(column, "iron" | "calcium" | "potassium") {
        "mg"
    } else {
        "g"
    };
    let mut answer = if value == 0. {
        format!("{display} {verb} no {label} to speak of")
    } else {
        format!("{display} {verb} about {}{unit} of {label}", figure(value))
    };
    if !scaled
        && !portion.is_empty()
        && portion != "one"
        && !display.to_lowercase().contains(&portion.to_lowercase())
    {
        answer.push_str(&format!(" ({portion})"));
    }
    answer.push('.');
    if column == "protein" {
        if value >= 50. {
            answer.push_str(" That meets or exceeds the 50g adult reference intake used on UK nutrition labels.");
        } else if let Some((_, share)) = PROTEIN_SHARES
            .iter()
            .find(|(fraction, _)| value / 50. >= *fraction)
        {
            answer.push_str(&format!(
                " That is {share} of the 50g adult reference intake used on UK nutrition labels."
            ));
        }
    }
    if let Some((_, context)) = data::CONTEXT.iter().find(|(name, _)| *name == column) {
        answer.push(' ');
        answer.push_str(context);
    }
    Some(answer)
}

pub(super) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "calories-in-food" => calorie_answer(reading).map(Cow::Owned),
        "nutrients-in-food" => nutrient_answer(reading).map(Cow::Owned),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calorie_readers_use_the_named_meal_and_avoid_composite_guesswork() {
        let meal =
            calorie_answer(&Reading::new("How many calories in chicken and chips?")).unwrap();
        assert_eq!(
            meal,
            "A plate of chicken and chips is about 850 calories (a normal takeaway plate)."
        );
        assert!(calorie_answer(&Reading::new("How many calories in a chicken breast?")).is_none());
        assert!(calorie_answer(&Reading::new("How many calories in porridge oats?")).is_none());
        let banana = calorie_answer(&Reading::new("How many calories in a bananna?")).unwrap();
        assert!(banana.starts_with("A banana is about"));
    }

    #[test]
    fn nutrients_scale_portions_and_preserve_subgram_source_amounts() {
        let eggs = nutrient_answer(&Reading::new("How much protein in two eggs?")).unwrap();
        assert!(eggs.starts_with("Two eggs have about 12g of protein."));
        let milk = nutrient_answer(&Reading::new("How much protein in 100ml of milk?")).unwrap();
        assert!(milk.starts_with("100ml of milk has about 3.5g of protein."));
        let salt = nutrient_answer(&Reading::new("How much salt in an egg?")).unwrap();
        assert!(salt.starts_with("An egg has about 0.2g of salt."));
        assert!(
            nutrient_answer(&Reading::new("How much protein in chicken with bananas?")).is_none()
        );
    }
}
