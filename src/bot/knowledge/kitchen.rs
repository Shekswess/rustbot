//! Recipes and substitutions when a named kitchen appliance is unavailable.
#[path = "kitchen_data.rs"]
mod data;
use super::super::understanding::line::Line;
use data::*;
use regex::Regex;
use std::{collections::BTreeSet, sync::LazyLock};

fn says_gone(line: &str, phrase: &str) -> bool {
    line.split('|').any(|part| {
        part.match_indices(phrase).any(|(i, _)| {
            let after = part[i + phrase.len()..]
                .trim()
                .split(' ')
                .next()
                .unwrap_or("")
                .trim_matches(&['.', ',', '!', '?', ';', ':'][..]);
            LACK_ATTACHED.binary_search(&after).is_err()
        })
    })
}
fn missing(line: &Line) -> BTreeSet<&'static str> {
    let words = format!("{} {}", line.clean, line.norm_text);
    let mut gone = BTreeSet::new();
    for &(appliance, phrases) in LACKS {
        if phrases.iter().any(|p| says_gone(&words, p)) {
            gone.insert(appliance);
        }
    }
    for &(appliance, phrase) in LACK_SAID {
        if !gone.contains(appliance) && says_gone(&words, phrase) {
            gone.insert(appliance);
        }
    }
    gone
}
fn includes(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}
fn available(text: &str, gone: &BTreeSet<&str>) -> BTreeSet<&'static str> {
    if let Some(&(name, _)) = ONLY_ONE.iter().find(|(_, terms)| includes(text, terms)) {
        return BTreeSet::from([name]);
    }
    let named: BTreeSet<_> = HAVE_ONE
        .iter()
        .filter(|(name, terms)| !gone.contains(name) && includes(text, terms))
        .map(|(name, _)| *name)
        .collect();
    if !named.is_empty() {
        return named;
    }
    ["oven", "hob", "microwave", "kettle"]
        .into_iter()
        .filter(|name| !gone.contains(name))
        .collect()
}
fn griddle(s: &str) -> String {
    static WRAP: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(?:wrap|wraps|tortilla|tortillas|pitta|pittas|pita|pitas)\b").unwrap()
    });
    if let Some(item) = GRIDDLE_BATTER.find(s) {
        return BATTER_REPLY.replace("{name}", item.as_str().trim_end_matches('s'));
    }
    if let Some(item) = GRIDDLE_DOUGH.find(s) {
        let name = item.as_str().trim_end_matches('s');
        return if matches!(name, "scone" | "drop scone" | "pikelet") {
            SCONE_REPLY.to_owned()
        } else {
            DOUGH_REPLY.replace("{name}", name)
        };
    }
    let name = WRAP
        .find(s)
        .map_or("wrap", |m| m.as_str().trim_end_matches('s'));
    (if name.starts_with("pitt") {
        PITTA_REPLY
    } else {
        WRAP_REPLY
    })
    .replace("{name}", name)
}
fn no_baking(s: &str) -> String {
    if GRIDDLE_BAKES.is_match(s) {
        return griddle(s);
    }
    let bread = BREAD.is_match(s);
    let mut base = if bread && !CAKEISH.is_match(s) {
        NO_OVEN_BREAD.to_owned()
    } else if bread {
        format!("{NO_BAKING} {NO_OVEN_BREAD}")
    } else {
        NO_BAKING.to_owned()
    };
    if let Some(m) = CROWD.find(s) {
        base.push_str(&format!(" For {} people make two tins of it and cut it small, or set one big cheesecake in a deep tin and chill it overnight.",m.as_str()));
    }
    base
}
pub(crate) fn without(line: &Line) -> Option<String> {
    static ASK: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(&format!(
            r"\b(?:{})\b",
            COOKING_ASKS
                .iter()
                .map(|p| regex::escape(p))
                .collect::<Vec<_>>()
                .join("|")
        ))
        .unwrap()
    });
    let gone = missing(line);
    if gone.is_empty() {
        return None;
    }
    let text = line.words.join(" ").to_lowercase();
    if !ASK.is_match(&text) && !includes(&text, LISTING) {
        let named = HAVE_ONE
            .iter()
            .chain(ONLY_ONE.iter().filter(|(a, _)| *a != "oven"))
            .any(|(a, t)| {
                matches!(*a, "hob" | "microwave" | "kettle" | "oven") && includes(&text, t)
            });
        if !named || text.split_whitespace().count() > 8 {
            return None;
        }
    }
    const BAKING: &[&str] = &[
        "bake",
        "baking",
        "cake",
        "cakes",
        "bread",
        "pie",
        "pastry",
        "pastries",
        "biscuit",
        "biscuits",
        "cookie",
        "cookies",
        "meringue",
        "meringues",
        "yorkshire",
        "scone",
        "scones",
        "loaf",
        "loaves",
        "muffin",
        "muffins",
        "brownies",
        "flapjack",
        "flapjacks",
    ];
    let bake =
        includes(&text, BAKING) || !text.contains("sweet potato") && SWEET_ASK.is_match(&text);
    if gone.contains("oven") && !gone.contains("hob") {
        if bake {
            return Some(no_baking(&text));
        }
        if GRIDDLE_BAKES.is_match(&text) {
            return Some(griddle(&text));
        }
    }
    let have = available(&text, &gone);
    let answer = if gone.contains("hob") || !have.contains("hob") {
        if have.contains("oven") {
            NO_OVEN_ONLY
        } else if have.contains("microwave") {
            NO_HOB
        } else {
            NO_KETTLE
        }
    } else if gone.contains("oven") || !have.contains("oven") {
        NO_OVEN
    } else if gone.contains("microwave") && !bake {
        NO_MICROWAVE
    } else {
        return None;
    };
    Some(answer.to_owned())
}
#[cfg(test)]
#[path = "../../../tests/fixtures/kitchen.rs"]
mod tests;
