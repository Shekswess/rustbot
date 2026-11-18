//! Calendar, daylight, plant, and weather replies from the authored nature cards.

use crate::{
    bot::{Exchange, Reading},
    text::is_space,
};
use jiff::{
    SignedDuration, Zoned,
    civil::{DateTime, date},
};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(PLANT_TROUBLE, super::NATURE_PLANT_TROUBLE);
pattern!(PLANT_LINE, r"[^a-z0-9' ]+");

pub(super) struct Produce {
    pub name: &'static str,
    pub part: Option<&'static str>,
    pub fruit: Option<&'static str>,
    pub aggregate: bool,
    pub kitchen: bool,
}

static PRODUCE: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(super::NATURE_PRODUCE.iter().map(|entry| {
        let word = entry.name;
        let (stem, ending) = if let Some(stem) = word.strip_suffix('y') {
            (stem, "(?:y|ies)")
        } else if let Some(stem) = word.strip_suffix('o') {
            (stem, "(?:o|os|oes)")
        } else if ["ch", "sh", "s", "x"]
            .iter()
            .any(|ending| word.ends_with(ending))
        {
            (word, "(?:es)?")
        } else {
            (word, "s?")
        };
        format!(r"\b{}{ending}\b", regex::escape(stem))
    }))
    .expect("authored names of produce")
});

fn has(line: &str, words: &[&str]) -> bool {
    words.iter().any(|word| line.contains(word))
}

fn clean_first<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn typed_first<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    }
}

pub(super) fn leaves(earlier: &[Exchange], reply: &'static str) -> Option<&'static str> {
    let previous = earlier[earlier.len().saturating_sub(3)..]
        .iter()
        .map(|turn| turn.user.as_str())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let said = format!(" {} ", PLANT_LINE.replace_all(&previous, " "));
    (!super::NATURE_PLANTS_IN_HAND
        .iter()
        .any(|word| said.contains(&format!(" {word} ")) || said.contains(&format!(" {word}s "))))
    .then_some(reply)
}

pub(super) fn wet_day(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    const PLAN: &str = "Rain rearranges a day out rather than cancelling it: keep the outdoor parts loose - a walk, a beach, a garden - and put one indoor thing into each day, a museum, a pool, a long lunch or a castle you can go round under cover. Book only what you can move and decide on the morning, when you can see the sky.";
    const WEEK: &str = " If it is set in for the week, pick one proper indoor day rather than a wet walk every day, and keep the evenings for the things that need no weather at all.";
    let said = clean_first(reading).to_lowercase();
    Some(
        if has(
            &said,
            &[
                "all week",
                "every day",
                "a lot",
                "pours",
                "turns",
                "weather is bad",
            ],
        ) {
            Cow::Owned(format!("{PLAN}{WEEK}"))
        } else {
            Cow::Borrowed(PLAN)
        },
    )
}

pub(super) fn daylight(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = clean_first(reading)
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    if has(&line, super::NATURE_DAYLIGHT_SHORT_WORDS) {
        return Some(Cow::Borrowed(super::NATURE_DAYLIGHT_WINTER));
    }
    if has(&line, super::NATURE_DAYLIGHT_LONG_WORDS) {
        let summer = super::NATURE_DAYLIGHT_SUMMER;
        return Some(if has(&line, &["scotland", "scottish", "shetland"]) {
            Cow::Owned(format!("{summer}{}", super::NATURE_DAYLIGHT_SCOTLAND))
        } else {
            Cow::Borrowed(summer)
        });
    }
    let padded = format!(" {line} ");
    for &(name, hours, minutes) in super::NATURE_DAYLIGHT_BY_MONTH {
        let named = if name == "may" {
            has(&padded, &[" in may ", " of may ", " during may "])
                || padded.trim_end().ends_with(" may")
        } else {
            padded.contains(&format!(" {name} "))
        };
        if named {
            return Some(Cow::Owned(format!(
                "In the UK, about the middle of {name} gives roughly {hours} hours {minutes} minutes of daylight, and it is still changing fastest around the equinoxes - a week either side of one moves the sunset by several minutes."
            )));
        }
    }
    Some(Cow::Borrowed(super::NATURE_DAYLIGHT_WINTER))
}

pub(super) fn produce_question(reading: &Reading<'_>) -> bool {
    let line = typed_first(reading).to_lowercase();
    let crop = line
        .split(|character: char| !character.is_ascii_lowercase() && character != '\'')
        .any(|word| super::NATURE_SETTING_CROPS.contains(&word));
    !crop || !PLANT_TROUBLE.is_match(&line)
}

pub(super) fn fruit(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = typed_first(reading).to_lowercase();
    if has(&line, super::NATURE_THE_HARVEST_ASK) {
        return None;
    }
    let Some(index) = PRODUCE.matches(&line).iter().next() else {
        return Some(Cow::Borrowed(
            "Botanically a fruit is the part that grows from the flower and holds the seeds, so tomatoes, cucumbers and peppers are fruits. A berry is a fruit from one ovary with seeds in the flesh, which makes bananas berries but strawberries not.",
        ));
    };
    let entry = &super::NATURE_PRODUCE[index];
    let name = entry.name;
    let article = if name.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "an"
    } else {
        "a"
    };
    let berry = line.contains("berr");
    let vegetable = line.contains("vegetable") && !berry;
    let reply = if let Some(part) = entry.part {
        if vegetable {
            format!(
                "Yes - in the kitchen {article} {name} is a vegetable, but botanically it is {part}, not a fruit: the fruit is the part that holds the seeds."
            )
        } else {
            format!(
                "No - {article} {name} is {part}, not a fruit. The fruit is the part that holds the seeds, so {article} {name} is a vegetable in the kitchen and {part} in the ground."
            )
        }
    } else if berry {
        if entry.aggregate {
            format!(
                "No - {article} {name} is not a true berry: its seeds are on the outside and each one is its own tiny fruit, so botanists call it an aggregate. A banana or a tomato really is a berry, which is where the rule surprises people."
            )
        } else if name == "cherry" {
            return Some(Cow::Borrowed(
                "No - a cherry is a drupe: one seed inside a hard stone, not a berry. A banana or a tomato really is a berry, which is where the rule surprises people.",
            ));
        } else {
            format!(
                "Yes - {article} {name} is a berry, botanically: it grows from one ovary and has its seeds inside the flesh. A strawberry is not, because its seeds sit on the outside."
            )
        }
    } else if let Some(botanical) = entry.fruit {
        format!(
            "Botanically {article} {name} is a {botanical}. Cooks call it a vegetable because it is savoury and goes in mains, but that is a kitchen word, not a plant one."
        )
    } else if entry.kitchen {
        format!(
            "Botanically {article} {name} is a fruit, and in the kitchen too: it grows from the flower and holds its seeds. A vegetable is a different part of the plant - a root, a leaf or a stem."
        )
    } else {
        format!(
            "Botanically {article} {name} is a fruit, because it grows from the flower and holds its seeds. A vegetable is the other parts of the plant, such as roots, leaves and stems."
        )
    };
    Some(Cow::Owned(reply))
}

pub(super) fn full_moon(_: &Reading<'_>) -> Option<Cow<'static, str>> {
    full_moon_at(Zoned::now().datetime()).map(Cow::Owned)
}

pub(super) fn full_moon_at(now: DateTime) -> Option<String> {
    const NEW_MOON: DateTime = date(2000, 1, 6).at(18, 14, 0, 0);
    const SYNODIC_MONTH: f64 = 29.530588853;
    const SECONDS_PER_DAY: f64 = 86_400.0;

    let cycles = now.duration_since(NEW_MOON).as_secs_f64() / SECONDS_PER_DAY / SYNODIC_MONTH;
    let full = (cycles - 0.5).trunc() + 1.0;
    let elapsed = SignedDuration::from_micros(
        ((full + 0.5) * SYNODIC_MONTH * SECONDS_PER_DAY * 1_000_000.0).round_ties_even() as i64,
    );
    let when = NEW_MOON.checked_add(elapsed).ok()?.date();
    let days = when.duration_since(now.date()).as_secs() / 86_400;
    let date_text = when.strftime("%A %-d %B");
    let lead = match days {
        0 => "Tonight is a full moon".to_owned(),
        1 => format!("The next full moon is tomorrow, {date_text}"),
        _ => format!("The next full moon is on {date_text}, in {days} days"),
    };
    Some(format!(
        "{lead}. Full moons come round every 29.5 days, so the one after is about four weeks later; the moon looks biggest when it is low in the sky, which is an illusion rather than the moon being closer."
    ))
}
