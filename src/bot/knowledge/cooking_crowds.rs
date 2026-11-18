//! Pinned cooking_gaps.py roast and table cards.

use crate::bot::{Exchange, Reading};
use fancy_regex::Regex as Lookaround;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use super::super::{dishes, units};

#[path = "cooking_crowds_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $engine:ty, $source:expr) => {
        static $name: LazyLock<$engine> =
            LazyLock::new(|| <$engine>::new($source).expect(stringify!($name)));
    };
}

pattern!(UNUSED_CHAR, Regex, "[^a-z0-9' ]+");
pattern!(
    ROAST_COUNT,
    Lookaround,
    r"(?<![a-z0-9])([0-9]{1,2})(?![a-z0-9])"
);
pattern!(THE_TABLE_NAMED, Regex, data::THE_TABLE_NAMED);
pattern!(THE_MEAL_NAMED, Regex, data::THE_MEAL_NAMED);
pattern!(A_TABLE_PUT_RIGHT, Regex, data::A_TABLE_PUT_RIGHT);
pattern!(A_TABLE_OF_PEOPLE, Lookaround, data::A_TABLE_OF_PEOPLE);
pattern!(CROWD_FOR, Regex, data::CROWD_FOR);
pattern!(CROWD_WITHIN, Regex, data::CROWD_WITHIN);
pattern!(DIGIT_FOR, Regex, r"\bfor\s+([0-9]{1,3})\b");
pattern!(
    DIGIT_WITHIN,
    Regex,
    r"\b([0-9]{1,3})\s+(?:people|of us|adults|guests|coming)\b"
);
pattern!(ORDINAL, Regex, r"\b[0-9]{1,3}(?:st|nd|rd|th)\b");

static NOT_THE_CROWD_CARD: LazyLock<Regex> = LazyLock::new(|| {
    let alternatives = data::NOT_THE_CROWD_CARD
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"\b(?:{alternatives})\b")).expect("roast subjects")
});

fn contains(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

pub(super) fn roast_allowed(reading: &Reading<'_>) -> bool {
    !dishes::named_as_excluded(clean(reading), "sunday roast")
}

pub(super) fn diabetic_bake_asked(reading: &Reading<'_>) -> bool {
    let said = format!(" {} ", reading.line.clean);
    !data::A_MEAL_IS_NAMED
        .iter()
        .any(|word| said.contains(&format!(" {word} ")))
        && !contains(&said, data::ALLOWED_ASKS)
        && !(contains(&said, data::EATING_VERBS)
            && contains(&said, &[" can ", " could ", " should ", " is ", " are "]))
}

pub(super) fn table_announced(reading: &Reading<'_>) -> bool {
    let text = format!(" {} ", clean(reading).to_lowercase());
    A_TABLE_OF_PEOPLE.is_match(&text).unwrap_or(false) && !contains(&text, data::NOT_A_TABLE)
}

fn head_count(raw: &str, words: &[(&str, u32)]) -> Option<u32> {
    raw.parse().ok().or_else(|| {
        words
            .iter()
            .find_map(|&(word, count)| (word == raw).then_some(count))
    })
}

fn earlier_roast_count(earlier: &[Exchange]) -> Option<u32> {
    earlier.iter().rev().take(8).find_map(|turn| {
        let said = turn
            .user
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let found = THE_TABLE_NAMED.captures(&said)?;
        if !THE_MEAL_NAMED.is_match(&said) && !A_TABLE_PUT_RIGHT.is_match(&said) {
            return None;
        }
        let raw = found.name("n").or_else(|| found.name("only"))?.as_str();
        head_count(raw, data::TABLE_WORDS).filter(|count| (2..=30).contains(count))
    })
}

fn explicit_roast_count(line: &str) -> Option<u32> {
    ROAST_COUNT
        .captures_iter(line)
        .find_map(|found| {
            let found = found.ok()?;
            found
                .get(1)?
                .as_str()
                .parse()
                .ok()
                .filter(|count| (4..=30).contains(count))
        })
        .or_else(|| {
            data::COUNT_WORDS.iter().find_map(|&(name, count)| {
                line.match_indices(name)
                    .any(|(at, _)| {
                        let before = line.as_bytes().get(at.wrapping_sub(1));
                        let after = line.as_bytes().get(at + name.len());
                        before.is_none_or(|c| !c.is_ascii_alphanumeric())
                            && after.is_none_or(|c| !c.is_ascii_alphanumeric())
                    })
                    .then_some(count)
            })
        })
}

fn kilos(count: u32) -> String {
    let grams = count * 250;
    match grams % 1000 {
        0 => format!("{}kg", grams / 1000),
        500 => format!("{}.5kg", grams / 1000),
        rest => format!("{}.{:02}kg", grams / 1000, rest / 10),
    }
}

fn duration(grams: u32) -> String {
    let minutes = grams / 25 + 20;
    match (minutes / 60, minutes % 60) {
        (0, minutes) => format!("{minutes} minutes"),
        (1, 0) => "an hour".to_owned(),
        (1, minutes) => format!("an hour and {minutes} minutes"),
        (hours, 0) => format!("{hours} hours"),
        (hours, minutes) => format!("{hours} hours {minutes} minutes"),
    }
}

// Planning times are estimates. Whole poultry, whole pork and stuffing still need a doneness check.
const ROAST_SAFETY: &str = " Check poultry, pork and stuffing at the thickest part with a clean thermometer. Aim for at least 70°C for 2 minutes.";

fn roast_for_a_crowd(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<String> {
    let line = UNUSED_CHAR.replace_all(clean(reading), " ");
    if NOT_THE_CROWD_CARD.is_match(&line) || dishes::named_as_excluded(&line, "sunday roast") {
        return None;
    }
    let count = explicit_roast_count(&line).or_else(|| earlier_roast_count(earlier));
    let after = "Potatoes in an hour before, parboiled and rough in hot fat at 200C for 45 minutes. Stuffing and pigs in blankets 25 minutes before the end, greens 10. The meat comes out to rest 15 minutes under foil, and the gravy is made in the tin while it does.";
    Some(if let Some(count) = count {
        let weight = kilos(count);
        let time = duration(count * 250);
        format!(
            "For {count} people allow about 250g of raw meat each with the bone in, or 150g boneless, so buy about {weight} in total: one joint that size, or two good-sized chickens. Work backwards from the time you want to eat: meat in at 190C (170C fan) roughly {time} before, using 20 minutes per 500g plus 20 as a planning estimate. {after}{ROAST_SAFETY}"
        )
    } else {
        format!(
            "Allow about 250g of raw meat each with the bone in, or 150g boneless: six people want a 2kg chicken or a 1.5kg joint. Work backwards from the time you want to eat: meat in at 190C (170C fan) roughly {} before, using 20 minutes per 500g plus 20 as a planning estimate for the 2kg bird. {after}{ROAST_SAFETY}",
            duration(2000)
        )
    })
}

pub(super) fn counts_for(text: &str) -> Option<u32> {
    [
        (&*DIGIT_FOR, true),
        (&*CROWD_FOR, true),
        (&*DIGIT_WITHIN, false),
        (&*CROWD_WITHIN, false),
    ]
    .into_iter()
    .flat_map(|(pattern, validate_unit)| {
        pattern.captures_iter(text).filter_map(move |found| {
            let after = text[found.get(0)?.end()..]
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches(['.', ',', '!', '?', ';', ':']);
            if validate_unit && data::CROWD_UNITS.contains(&after) {
                return None;
            }
            head_count(found.get(1)?.as_str(), data::CROWD_NUMBERS)
        })
    })
    .chain(units::people_in_groups(text))
    .max()
    .or_else(|| contains(text, data::CROWD_PHRASES).then_some(6))
}

fn trays(count: u32) -> String {
    let said = || {
        data::CROWD_WORDS
            .iter()
            .find_map(|&(word, value)| (count == value).then_some(word))
            .map_or_else(|| count.to_string(), str::to_owned)
    };
    match count {
        0 => "which does four to six, so count the potatoes yourself".to_owned(),
        1..=5 => format!("one tray of it is plenty for {}", said()),
        6..=8 => "which feeds eight from one oven".to_owned(),
        9..=12 => format!("two trays of it, on the top and middle shelves and swapped halfway, to feed {}", said()),
        _ => "two or three trays of it, on the top and middle shelves, or one dish made in the biggest pot you own".to_owned(),
    }
}

fn table_of_people(reading: &Reading<'_>, earlier: &[Exchange]) -> String {
    let text = std::iter::once(clean(reading))
        .chain(
            earlier
                .iter()
                .rev()
                .take(4)
                .rev()
                .map(|turn| turn.user.as_str()),
        )
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let plant_only = contains(&text, &["vegan", "dairy free", "dairy-free", "lactose"]);
    let vegetarian = plant_only || contains(&text, &["vegetarian", "no meat"]);
    let (dish, amount) = if vegetarian {
        let dish = if plant_only {
            "a chickpea and vegetable traybake made in the largest dish you own"
        } else {
            "a big mushroom and lentil lasagne or a chickpea and vegetable traybake, made in the largest dish you own"
        };
        (dish, "about 300g of the dish per person")
    } else {
        (
            "a tray of chicken thighs or sausages with halved potatoes, peppers and onions at 200C (180C fan) for about 45 minutes",
            "about 200g of meat and a big potato each",
        )
    };
    let count = counts_for(&text.replace('-', " ")).unwrap_or(0);
    let side = if contains(&text, &["gluten free", "gluten-free", "coeliac", "celiac"]) {
        "a bowl of salad"
    } else {
        "a loaf of bread and a bowl of salad"
    };
    let mut reply = format!(
        "Cook one big oven dish rather than several pans: {dish}, {}. Allow {amount}, add {side}, and write the times down so nothing needs the oven at once.",
        trays(count)
    );
    if contains(&text, data::OCCASIONS) || ORDINAL.is_match(&text) {
        reply.push_str(" A dish you make that morning, such as a chilli, stew or lasagne, can be cooled promptly, chilled and reheated once until steaming hot throughout.");
    }
    if !vegetarian {
        reply.push_str(" Check the thickest part of the chicken or sausages with a clean thermometer. Aim for at least 70°C for 2 minutes.");
    }
    reply
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    match id {
        "roast-for-a-crowd" | "joint-weight" => roast_for_a_crowd(reading, earlier).map(Cow::Owned),
        "a-table-of-people" => Some(Cow::Owned(table_of_people(reading, earlier))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roast_timing_uses_the_latest_relevant_table_count() {
        let earlier = [
            Exchange {
                user: "I'm doing a roast for 8 people on Sunday".to_owned(),
                bot: String::new(),
                module: "answers",
            },
            Exchange {
                user: "Actually it is 10 people".to_owned(),
                bot: String::new(),
                module: "chat",
            },
        ];
        let reply = registered(
            "roast-for-a-crowd",
            &Reading::new("When do I start?"),
            &earlier,
        )
        .unwrap();
        assert!(reply.contains("For 10 people"));
        assert!(reply.contains("2.5kg"));
        assert!(reply.contains("2 hours"));
        assert!(reply.contains("70°C for 2 minutes"));

        let reply = registered("joint-weight", &Reading::new("Roast for seven"), &earlier).unwrap();
        assert!(reply.contains("1.75kg"));
        assert!(reply.contains("an hour and 30 minutes"));
        assert!(registered("joint-weight", &Reading::new("A beef joint for seven"), &[]).is_none());
        assert!(!roast_allowed(&Reading::new(
            "I don't want a roast, what goes with it instead?"
        )));
    }

    #[test]
    fn numbered_tables_and_baking_permission_have_distinct_guards() {
        assert!(table_announced(&Reading::new(
            "I'm having 12 people over on Saturday."
        )));
        assert!(!table_announced(&Reading::new(
            "12 people applied for the job."
        )));
        assert!(!table_announced(&Reading::new(
            "How much food for 12 people over on Saturday?"
        )));
        assert_eq!(counts_for("for 8 minutes and 4 adults and 3 kids"), Some(7));
        let reply = registered(
            "a-table-of-people",
            &Reading::new("We're having 12 people over on Saturday"),
            &[],
        )
        .unwrap();
        assert!(reply.contains("to feed twelve"));
        assert!(!diabetic_bake_asked(&Reading::new(
            "Can I eat cake if I have diabetes?"
        )));
        assert!(diabetic_bake_asked(&Reading::new(
            "What cake can I bake for someone with diabetes?"
        )));
    }
}
