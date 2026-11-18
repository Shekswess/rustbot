//! Confirm a figure already given or identify a revised cooking figure.

use super::measurement;
use crate::bot::{Exchange, Memory, Reading, compose, understanding::normalize::tokens};
use fancy_regex::Regex as ContextRegex;
use regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

macro_rules! rule {
    ($name:ident, $regex:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($regex).expect(stringify!($name)));
    };
}

const UNITS: &str = "week month day year hour minute second mile km kilo stone pound percent session run portion time glass cup litre pint degree inch cm metre gram ml";
const SMALL_NUMBERS: &[&str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
];
const NOTHING_NAMED: &str = "so and but or then ok okay right still is are was were am that this it they those these the a an of in on at for to do does did will would can could should my i me you we our yeah yes no anymore any more both quite really actually true correct stand stands work works hold holds apply applies changed change mean means all that's its it's again now today currently nowadays even just also too much very bit little";
const CHANGE_WORDS: &str = "that this it which change changed changes affect affects alter alters impact matter difference make makes made a an any to for the on of in with much long time cooking oven";
const NON_SUBJECT: &str = "minute minutes hour hours second seconds day days week weeks month months degree degrees oven ovens tray tins pan pans tin dish dishes cook cooking cooked bake baked roast roasted about until when then they this that with from your yours each every before after while total same time times heat high medium low fan";

const STANDS: &[&str] = &[
    "Still right: {reason}",
    "That has not changed: {reason}",
    "Yes - and that is where it comes from: {reason}",
];
const CONFIRMS: &[&str] = &[
    "Still right - {figure}.",
    "Yes, {figure} is still the number.",
    "That holds: {figure}, as before.",
];
const TIME_MOVED: &[&str] = &[
    "The revised time I gave was {figure}.",
    "For this version, the time I gave was {figure}.",
];
const HEAT_MOVED: &[&str] = &[
    "The revised temperature I gave was {figure}.",
    "For this version, the temperature I gave was {figure}.",
];
const TIME_HELD: &[&str] = &[
    "The original time was {figure}. Check the recipe for your substitution and check that the dish is cooked through.",
    "I gave {figure} for the original version. Check the cooking instructions for the substituted ingredient.",
];
const HEAT_HELD: &[&str] = &[
    "The original oven temperature was {figure}. Check the recipe for your substitution.",
    "I gave {figure} for the original version. Check the cooking instructions for the substituted ingredient.",
];

rule!(
    ASKING,
    r"\b(?:still|right|correct|true|ok|okay|the same|sure)\b|\?"
);
rule!(
    CHANGE_ASK,
    r"\b(?:does|do|will|would|should|did)\b[^.?!]{0,24}?\b(?:change|affect|alter|make (?:a|any) difference to|matter (?:to|for)|impact)\b"
);
rule!(
    ATTRIBUTE,
    r"\b(?:the|my|our|its)\s+(time|long|temperature|heat|cooking time|oven time)\b"
);
rule!(
    SETTING,
    r"\b(?:boiler|thermostat|programmer|clock|timer|heating)\b"
);
rule!(
    CHANGE,
    r"(?i)\b(?:instead|rather than|without|swap\w*|switch\w*|replace\w*|substitut\w*|veggie|vegetarian|vegan|gluten|dairy|free from|omitted|skipped|cut out|left out|leave out|made it with|no [a-z]+)\b"
);
rule!(
    METHOD,
    r"(?i)\b(?:slow cooker|crock ?pot|pressure cooker|instant pot|air ?fryer|microwave|steam\w*|sous ?vide|barbecue|bbq|grill\w*|poach\w*|deep[- ]?fr(?:y|ied|ying)|boil\w*|on the hob|in the oven instead|oven instead)\b"
);
rule!(
    SIZE,
    r"(?i)\b(?:bigger|larger|smaller|double|doubled|halve|halved|batch|twice as|half as)\b|\b(?:more|extra|less|fewer)\s+(?:people|guests?|kids?|portions?|slices|servings?|adults?|children)\b|\b(?:for|feed|feeding|now|there are|there will be)\s+(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|\d+)\b|\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|\d+)\s*(?:of us|people|guests?|kids?|portions?|slices|servings?|adults?|children)\b"
);

static FIGURE: LazyLock<Regex> = LazyLock::new(|| {
    let numbers = SMALL_NUMBERS.join("|");
    let units = UNITS.split_whitespace().collect::<Vec<_>>().join("|");
    Regex::new(&format!(
        r"(?i)\b(\d[\d,]*(?:\.\d+)?|{numbers}|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|couple|few|once|twice|thrice)\b(?:\s+({units})s?\b)?"
    )).expect("valid repeated figure phrase")
});

static COOK_FIGURE: LazyLock<ContextRegex> = LazyLock::new(|| {
    ContextRegex::new(concat!(
        r"(?i)\b(\d[\d,.]*(?:\s*(?:to|or|-)\s*\d[\d,.]*)?)\s*(hours?|hrs?)\s+(\d[\d,.]*)\s*(minutes?|mins?)?\b",
        r"(?!\s*(?:g|kg|ml|cl|lb|lbs|oz|cm|degrees?)\b)|",
        r"\b(\d[\d,.]*(?:\s*(?:to|or|-)\s*\d[\d,.]*)?)\s*(minutes?|mins?|hours?|hrs?|days?|weeks?|months?|degrees?|C\b|°C)"
    )).expect("valid cooking figure phrase")
});

fn one_of(word: &str, words: &str) -> bool {
    words.split_whitespace().any(|item| word == item)
}

fn number(text: &str) -> Option<f64> {
    let text = text.trim_matches(['.', ',']).to_ascii_lowercase();
    if let Ok(number) = text.replace(',', "").parse::<f64>() {
        return Some(number);
    }
    SMALL_NUMBERS
        .iter()
        .position(|word| *word == text)
        .map(|index| index as f64)
        .or_else(|| {
            Some(match text.as_str() {
                "thirty" => 30.0,
                "forty" => 40.0,
                "fifty" => 50.0,
                "sixty" => 60.0,
                "seventy" => 70.0,
                "eighty" => 80.0,
                "ninety" => 90.0,
                "hundred" => 100.0,
                "once" => 1.0,
                "couple" | "twice" => 2.0,
                "few" | "thrice" => 3.0,
                _ => return None,
            })
        })
}

struct Figure<'a> {
    amount: f64,
    unit: String,
    start: usize,
    end: usize,
    said: &'a str,
}

fn figures(text: &str) -> impl Iterator<Item = Figure<'_>> {
    FIGURE.captures_iter(text).filter_map(|capture| {
        let found = capture.get(0)?;
        Some(Figure {
            amount: number(&capture[1])?,
            unit: capture
                .get(2)
                .map_or_else(String::new, |unit| unit.as_str().to_lowercase()),
            start: found.start(),
            end: found.end(),
            said: found.as_str(),
        })
    })
}

fn bare(text: &str, max_words: usize, extra: &str) -> bool {
    text.split_whitespace().take(max_words + 1).count() <= max_words
        && text.split_whitespace().all(|word| {
            let word = word.trim_matches(['.', ',', '!', '?', ';', ':']);
            word.is_empty()
                || one_of(word, NOTHING_NAMED)
                || one_of(word.trim_end_matches("'s"), NOTHING_NAMED)
                || one_of(word.trim_end_matches('s'), UNITS)
                || one_of(word, extra)
        })
}

fn same_figure(line: &str, asked: &Figure<'_>) -> bool {
    figures(line).any(|given| {
        given.amount == asked.amount && (asked.unit.is_empty() || given.unit == asked.unit)
    })
}

fn confirm(line: &str, memory: &Memory) -> Option<String> {
    if line.split_whitespace().take(10).count() > 9 || !ASKING.is_match(line) {
        return None;
    }
    let mut asked = figures(line);
    let one = asked.next()?;
    if asked.next().is_some() {
        return None;
    }
    let other = format!("{} {}", &line[..one.start], &line[one.end..]);
    if !bare(&other, 9, "per each every") {
        return None;
    }
    let recent = memory.turns.iter().rev().take(5).filter(|turn| {
        !matches!(turn.module, "safety" | "urgent" | "a_count_of_packets")
            && same_figure(&turn.bot, &one)
    });
    recent.clone().next()?;
    let reason = recent
        .flat_map(|turn| measurement::sentences(&turn.bot))
        .find(|sentence| {
            (6..=45).contains(&sentence.split_whitespace().count()) && same_figure(sentence, &one)
        });
    Some(match reason {
        Some(reason) => compose::pick(STANDS, memory, line).replace("{reason}", reason),
        None => compose::pick(CONFIRMS, memory, line).replace("{figure}", one.said),
    })
}

#[derive(Clone, Copy)]
enum Family {
    Time,
    Heat,
}

fn time_in_minutes(count: &str, unit: &str) -> f64 {
    let amount = count
        .split_whitespace()
        .next()
        .and_then(number)
        .unwrap_or_default();
    amount
        * match unit.trim_end_matches('s') {
            "hour" | "hr" => 60.0,
            "day" => 1440.0,
            "week" => 10080.0,
            "month" => 43200.0,
            _ => 1.0,
        }
}

fn best_cook(source: &str, family: Family) -> Option<String> {
    let figures = COOK_FIGURE.captures_iter(source).filter_map(|found| {
        let found = found.ok()?;
        if let Some(hour) = found.get(1) {
            let minute = found.get(3)?.as_str();
            let unit = found.get(4).map_or("minutes", |unit| unit.as_str());
            return matches!(family, Family::Time).then(|| {
                let amount =
                    time_in_minutes(hour.as_str(), "hour") + time_in_minutes(minute, "minute");
                let said = format!("{} {} {minute} {unit}", hour.as_str(), &found[2]);
                (amount, said)
            });
        }
        let unit = found.get(6)?.as_str().to_ascii_lowercase();
        let value = found.get(5)?.as_str();
        let is_heat = matches!(unit.trim_end_matches('s'), "degree" | "c" | "°c");
        match (family, is_heat) {
            (Family::Heat, true) => Some((
                0.0,
                if unit.ends_with('c') {
                    format!("{value}C")
                } else {
                    format!("{value} {unit}")
                },
            )),
            (Family::Time, false) => {
                Some((time_in_minutes(value, &unit), format!("{value} {unit}")))
            }
            _ => None,
        }
    });
    let picked = match family {
        Family::Heat => figures.into_iter().next(),
        Family::Time => figures.reduce(|old, new| if new.0 > old.0 { new } else { old }),
    };
    picked.map(|(_, said)| said)
}

fn digits(text: &str) -> impl Iterator<Item = char> + '_ {
    text.chars().filter(char::is_ascii_digit)
}

fn shares_subject(old: &Exchange, new: &Exchange) -> bool {
    let old: HashSet<_> = tokens(&old.bot)
        .into_iter()
        .filter(|word| word.len() >= 4 && !one_of(word, NON_SUBJECT))
        .collect();
    tokens(&new.bot).into_iter().any(|word| old.contains(&word))
}

fn changed(line: &str, memory: &Memory) -> Option<String> {
    if !CHANGE_ASK.is_match(line) || SETTING.is_match(line) || line.split_whitespace().count() > 12
    {
        return None;
    }
    let recent = memory.turns.iter().rev().take(3);
    if recent
        .clone()
        .any(|turn| SIZE.is_match(&turn.user) || METHOD.is_match(&turn.user))
        || !recent.clone().any(|turn| CHANGE.is_match(&turn.user))
    {
        return None;
    }
    let found = ATTRIBUTE.captures(line)?.get(1)?;
    let rest = format!("{} {}", &line[..found.start()], &line[found.end()..]);
    if !bare(&rest, 12, CHANGE_WORDS) {
        return None;
    }
    let family = match found.as_str() {
        "heat" | "temperature" => Family::Heat,
        _ => Family::Time,
    };
    let (new, earlier) = memory.turns.split_last()?;
    let previous = earlier.last();
    let last = best_cook(&new.bot, family);
    let old = previous.and_then(|turn| best_cook(&turn.bot, family));
    let moved = match (&last, &old, previous) {
        (Some(last), Some(old), Some(previous)) => {
            !digits(last).eq(digits(old)) && shares_subject(previous, new)
        }
        (Some(_), None, _) => true,
        _ => false,
    };
    let figure = if moved { last? } else { old? };
    let options = match (family, moved) {
        (Family::Time, true) => TIME_MOVED,
        (Family::Heat, true) => HEAT_MOVED,
        (Family::Time, false) => TIME_HELD,
        (Family::Heat, false) => HEAT_HELD,
    };
    Some(compose::pick(options, memory, line).replace("{figure}", &figure))
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if memory.turns.is_empty() {
        return None;
    }
    let line = reading.raw.replace('’', "'").to_lowercase();
    let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
    confirm(&line, memory).or_else(|| changed(&line, memory))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn turn(user: &str, bot: &str) -> Exchange {
        Exchange {
            user: user.into(),
            bot: bot.into(),
            module: "answers",
        }
    }

    #[test]
    fn confirms_the_same_figure_and_unit_the_bot_actually_said() {
        let memory = Memory {
            turns: vec![turn(
                "How long should I train?",
                "Eight weeks gives you time to build your distance steadily.",
            )]
            .into(),
            ..Memory::default()
        };
        let answer = respond(&Reading::new("So is eight weeks still right?"), &memory).unwrap();
        assert!(answer.contains("Eight weeks gives you time to build"));
        assert!(respond(&Reading::new("Is eight hours still right?"), &memory).is_none());
        assert!(respond(&Reading::new("Is eight weeks right for my baby?"), &memory).is_none());
    }

    #[test]
    fn distinguishes_a_new_cooking_figure_from_a_change_of_method_or_size() {
        let mut memory = Memory {
            turns: vec![
                turn("I'm making rice", "Cook white rice for 10 to 12 minutes."),
                turn(
                    "I'm using brown rice instead",
                    "Cook brown rice for 25 to 30 minutes.",
                ),
            ]
            .into(),
            ..Memory::default()
        };
        let answer = respond(&Reading::new("Does that change the time?"), &memory).unwrap();
        assert!(answer.contains("25 to 30 minutes"));
        memory.turns.push(turn(
            "I swapped to a slow cooker instead",
            "A new method takes a different time.",
        ));
        assert!(respond(&Reading::new("Does that change the time?"), &memory).is_none());
    }
}
