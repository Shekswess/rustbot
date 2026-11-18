//! Serving sizes, egg counts, and shared-oven questions.

use crate::bot::{Reading, memory::groupages::decimal};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

#[path = "cake_planning_data.rs"]
mod data;

static SERVINGS: LazyLock<[Regex; 4]> = LazyLock::new(|| {
    [
        r"\b(\d{1,3})\s+(?:people|guests|kids|children|adults|persons|portions|slices|of us|head)\b",
        r"\bfor\s+(\d{1,3})\b",
        r"\b(?:feeds|feed|serves|serve)\s+(\d{1,3})\b",
        r"\bparty\s+of\s+(\d{1,3})\b",
    ]
    .map(|pattern| Regex::new(pattern).expect("cake serving number"))
});
static COUNT_WORDS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(data::CAKE_COUNTS.iter().map(|(words, _)| whole(words)))
        .expect("cake serving words")
});
static CAKE_DOUBLE: LazyLock<RegexSet> = LazyLock::new(|| cues(data::CAKE_DOUBLE_WORDS));
static CAKE_DECOR: LazyLock<RegexSet> = LazyLock::new(|| cues(data::CAKE_DECOR_WORDS));
static CUPCAKE: LazyLock<RegexSet> = LazyLock::new(|| cues(data::CUPCAKE_WORDS));
static CAKE_SIZE: LazyLock<RegexSet> = LazyLock::new(|| cues(data::CAKE_SIZE_WORDS));
static SKIP_EGGS: LazyLock<RegexSet> = LazyLock::new(|| {
    cues(&[
        "needed",
        "necessary",
        "essential",
        "required",
        "have to",
        "can i skip",
        "skip the egg",
        "skip eggs",
        "do without",
    ])
});
static EGG_BATTER: LazyLock<RegexSet> =
    LazyLock::new(|| cues(&["yorkshire", "batter", "toad in the hole", "popover"]));
static EGG_PANCAKE: LazyLock<RegexSet> =
    LazyLock::new(|| cues(&["pancake", "pancakes", "crepe", "crepes", "drop scone"]));
static EGG_MERINGUE: LazyLock<RegexSet> =
    LazyLock::new(|| cues(&["meringue", "meringues", "pavlova"]));
static TIN_DIMENSIONS: LazyLock<[Regex; 4]> = LazyLock::new(|| {
    [
        r"\b20\s*cm\b",
        r"\b23\s*cm\b",
        r"\b25\s*cm\b",
        r"\b30\s*x\s*20\s*cm\b",
    ]
    .map(|pattern| Regex::new(pattern).expect("cake tin dimensions"))
});

struct Tin {
    name: &'static str,
    plural: &'static str,
    low: u16,
    high: u16,
}

const TINS: [Tin; 4] = [
    Tin {
        name: "20cm tin",
        plural: "20cm tins",
        low: 8,
        high: 10,
    },
    Tin {
        name: "23cm tin",
        plural: "23cm tins",
        low: 12,
        high: 14,
    },
    Tin {
        name: "25cm tin",
        plural: "25cm tins",
        low: 18,
        high: 20,
    },
    Tin {
        name: "30 x 20cm traybake",
        plural: "30 x 20cm traybakes",
        low: 15,
        high: 18,
    },
];

pub(crate) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    match id {
        "cake-size" => cake_size(clean),
        "cake-eggs" => Some(Cow::Borrowed(eggs(clean))),
        "two-dishes-one-oven" => {
            let in_the_oven = clean.contains("cake")
                && (clean.contains("roast")
                    || clean
                        .split(' ')
                        .any(|word| data::TWO_OVEN_BEEF.contains(&word)));
            Some(Cow::Borrowed(if in_the_oven {
                data::CAKE_AND_ROAST
            } else {
                data::TWO_OVEN
            }))
        }
        _ => None,
    }
}

fn whole(phrase: &str) -> String {
    format!(r"\b{}\b", regex::escape(phrase))
}

fn cues(phrases: &[&str]) -> RegexSet {
    RegexSet::new(phrases.iter().map(|phrase| {
        if phrase.contains(' ') {
            regex::escape(phrase)
        } else {
            whole(phrase)
        }
    }))
    .expect("authored cake cues")
}

fn headcount(text: &str) -> u16 {
    let digits = SERVINGS.iter().flat_map(|pattern| {
        pattern.captures_iter(text).filter_map(|found| {
            let finished = found.get(0)?.end();
            let tail = text[finished..].trim_matches(crate::text::is_space);
            let after = tail.split(' ').next()?.trim_matches([',', '.']);
            (!data::CAKE_CLOCK.contains(&after))
                .then(|| decimal(&found[1]))
                .flatten()
        })
    });
    let matches = COUNT_WORDS.matches(text);
    let words = matches.iter().map(|index| data::CAKE_COUNTS[index].1);
    digits.chain(words).max().unwrap_or_default()
}

fn named_tin(clean: &str) -> Option<usize> {
    let exact = clean.replace(" cm", "cm");
    [3, 0, 1, 2]
        .into_iter()
        .find(|&index| TIN_DIMENSIONS[index].is_match(&exact))
}

fn bare_size(clean: &str) -> Cow<'static, str> {
    if let Some(index) = named_tin(clean) {
        let tin = &TINS[index];
        let feeds = if index == 3 {
            format!(
                "A {} cut into squares feeds {} to {}. ",
                tin.name, tin.low, tin.high
            )
        } else {
            let size = tin.name.trim_end_matches(" tin");
            format!(
                "A {size} two-layer sponge feeds {} to {}.",
                tin.low, tin.high
            )
        };
        return Cow::Owned(format!(
            "{feeds} {} {}",
            data::CAKE_DAY_BEFORE,
            data::CAKE_DOUBLE
        ));
    }
    if CAKE_DOUBLE.is_match(clean) {
        Cow::Borrowed(data::CAKE_DOUBLE)
    } else if CAKE_DECOR.is_match(clean) {
        Cow::Borrowed(data::CAKE_DECOR)
    } else if CUPCAKE.is_match(clean) {
        Cow::Borrowed(data::CUPCAKE_KEEP)
    } else if CAKE_SIZE.is_match(clean) {
        Cow::Owned([data::CAKE_SIZE, data::CAKE_DAY_BEFORE, data::CAKE_DOUBLE].join(" "))
    } else {
        Cow::Borrowed(data::CAKE_BASE)
    }
}

fn eggs(clean: &str) -> &'static str {
    if SKIP_EGGS.is_match(clean) {
        data::EGG_SKIP
    } else if EGG_BATTER.is_match(clean) {
        data::EGG_BATTER
    } else if EGG_PANCAKE.is_match(clean) {
        data::EGG_PANCAKE
    } else if EGG_MERINGUE.is_match(clean) {
        data::EGG_MERINGUE
    } else {
        data::EGG_SPONGE
    }
}

struct TinPlan {
    score: (u16, usize, u16),
    counts: [u16; 4],
    low: u16,
    high: u16,
}

fn tins_for(count: u16) -> Option<TinPlan> {
    if let Some(index) = TINS[..3].iter().position(|shape| count <= shape.high) {
        let mut counts = [0; 4];
        counts[index] = 1;
        let tin = &TINS[index];
        return Some(TinPlan {
            score: (1, 1, tin.high - count),
            counts,
            low: tin.low,
            high: tin.high,
        });
    }
    let mut best: Option<TinPlan> = None;
    for n20 in 0..=8 {
        for n23 in 0..=8 {
            for n25 in 0..=8 {
                for n_tray in 0..=8 {
                    let counts = [n20, n23, n25, n_tray];
                    let total = counts.iter().sum::<u16>();
                    if !(1..=12).contains(&total) {
                        continue;
                    }
                    let high = counts
                        .iter()
                        .zip(&TINS)
                        .map(|(n, tin)| n * tin.high)
                        .sum::<u16>();
                    if high < count {
                        continue;
                    }
                    let kinds = counts.iter().filter(|&&n| n > 0).count();
                    let score = (total, kinds, high - count);
                    if best.as_ref().is_none_or(|old| score < old.score) {
                        let low = counts.iter().zip(&TINS).map(|(n, tin)| n * tin.low).sum();
                        best = Some(TinPlan {
                            score,
                            counts,
                            low,
                            high,
                        });
                    }
                }
            }
        }
    }
    best
}

fn cake_size(clean: &str) -> Option<Cow<'static, str>> {
    let count = headcount(clean);
    if count == 0 {
        return Some(bare_size(clean));
    }
    if CUPCAKE.is_match(clean) {
        let trays = (count * 2).div_ceil(24);
        let mut reply = format!(
            "{} cupcakes for {count} people: two a head, baked at 180C for 12 to 18 minutes and iced on the day.",
            count * 2
        );
        if trays > 1 {
            reply.push_str(&format!(
                " That is {trays} trays of 24, and a muffin tray takes 12 at a time."
            ));
        }
        return Some(Cow::Owned(reply));
    }
    let TinPlan {
        score: (total, _, _),
        counts,
        low,
        high,
    } = tins_for(count)?;
    let lead = if total == 1 {
        let index = counts.iter().position(|&n| n == 1)?;
        let shape = TINS[index].name;
        if index == 3 {
            format!("One {shape} cut into squares feeds {low} to {high}, which covers {count}.")
        } else {
            let size = shape.trim_end_matches(" tin");
            format!("A {size} two-layer sponge feeds {low} to {high}, so one tin covers {count}.")
        }
    } else {
        let mut pieces = Vec::with_capacity(TINS.len());
        for (&n, tin) in counts.iter().zip(&TINS).filter(|(n, _)| **n != 0) {
            // PyBot's wording table ends at six. Its responder skips this
            // callback when a larger count runs past that table.
            let number = data::CAKE_NUMBERS.get(usize::from(n))?;
            let shape = if n == 1 { tin.name } else { tin.plural };
            pieces.push(format!("{number} {shape}"));
        }
        let tail = pieces.pop()?;
        let mut listed = if pieces.is_empty() {
            tail
        } else {
            format!("{} and {tail}", pieces.join(", "))
        };
        listed.get_mut(..1)?.make_ascii_uppercase();
        format!("{listed} feed {low} to {high} between them, which covers {count}.")
    };
    Some(Cow::Owned(format!(
        "{lead} {} Double every weight and split it between the tins with the same heat and time.",
        data::CAKE_DAY_BEFORE
    )))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/cake_planning.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_pinned_cake_plans_and_egg_choices() {
        for &(id, raw, expected) in fixtures::CASES {
            let reply = registered(id, &Reading::new(raw));
            assert_eq!(reply.as_deref(), expected, "{id}: {raw}");
        }
    }
}
