//! Rice substitutions and varieties for named dishes, from PyBot's rice domain.

use std::borrow::Cow;

use super::guards::Context;

#[cfg(test)]
#[path = "rice_tests.rs"]
mod tests;

pub(super) struct Variety {
    pub(super) names: &'static [&'static str],
    pub(super) label: &'static str,
    pub(super) water: &'static str,
    pub(super) time: &'static str,
    pub(super) description: &'static str,
    pub(super) substitute: &'static str,
}

use super::{RICE_DISHES, RICE_VARIETIES};

fn variety(name: &str) -> &'static Variety {
    RICE_VARIETIES
        .iter()
        .find_map(|&(key, ref variety)| (key == name).then_some(variety))
        .expect("rice dish names a known variety")
}

fn search_line(text: &str) -> String {
    format!(" {} ", text.to_lowercase())
}

fn dish_named(line: &str) -> Option<(&'static str, &'static str)> {
    RICE_DISHES.iter().find_map(|&(aliases, key)| {
        aliases
            .iter()
            .copied()
            .find(|alias| {
                line.contains(&format!(" {alias} ")) || line.contains(&format!(" {alias}s "))
            })
            .map(|alias| (key, alias))
    })
}

fn varieties_named(line: &str) -> Vec<&'static str> {
    let mut found = Vec::new();
    for (key, variety) in RICE_VARIETIES {
        let mut first = line.len();
        for alias in variety.names {
            for form in [format!(" {alias} "), format!(" {alias}s ")] {
                if let Some(at) = line.find(&form).filter(|&at| at < first) {
                    first = at;
                    break;
                }
            }
        }
        if first < line.len() {
            found.push((first, *key));
        }
    }
    found.sort_unstable();
    found.into_iter().map(|(_, key)| key).collect()
}

fn capitalized(label: &str) -> String {
    let mut label = label.to_owned();
    label[..1].make_ascii_uppercase();
    label
}

pub(super) fn reply(context: &Context<'_>) -> Cow<'static, str> {
    let reading = context.reading;
    let line = if reading.line.clean.is_empty() {
        search_line(&reading.line.words.join(" "))
    } else {
        search_line(&reading.line.clean)
    };
    let named = varieties_named(&line);
    if let [held, wanted, ..] = named.as_slice() {
        let held = variety(held);
        let wanted = variety(wanted);
        return substitute(held, wanted).into();
    }

    let dish = dish_named(&line).or_else(|| {
        context
            .earlier
            .iter()
            .rev()
            .take(4)
            .find_map(|turn| dish_named(&search_line(&turn.user)))
    });
    let held = named.first().copied();
    match (held, dish) {
        (Some(held), Some((wanted, dish))) if held != wanted => {
            let held = variety(held);
            let wanted = variety(wanted);
            format!(
                "You can use {} in {dish}, but {} is the one for it: {}, {}. {} is {}, where {} is {}, so the dish comes out with that grain in it.",
                held.label,
                wanted.label,
                wanted.water,
                wanted.time,
                capitalized(wanted.label),
                wanted.description,
                held.label,
                held.description,
            )
            .into()
        }
        (_, Some((wanted, dish))) => {
            let wanted = variety(wanted);
            let mut answer = format!(
                "For {dish}, {} is the one: {}, {}. It is {}.",
                wanted.label, wanted.water, wanted.time, wanted.description,
            );
            if !wanted.substitute.is_empty() {
                answer.push(' ');
                answer.push_str(wanted.substitute);
            }
            answer.into()
        }
        (Some(held), None) => {
            let held = variety(held);
            format!(
                "{} works in place of white rice: {}, {}. It is {}.",
                capitalized(held.label),
                held.water,
                held.time,
                held.description,
            )
            .into()
        }
        (None, None) => {
            let white = variety("long grain");
            let brown = variety("brown");
            format!(
                "Plain long grain rice is the safe one for most things: {}, {}. Basmati and jasmine are the same water and time with a different grain, and brown rice wants {} for {}.",
                white.water, white.time, brown.water, brown.time,
            )
            .into()
        }
    }
}

fn substitute(held: &Variety, wanted: &Variety) -> String {
    let name = capitalized(held.label);
    if (held.water, held.time) == (wanted.water, wanted.time) {
        return format!(
            "{name} will stand in for {} on the same water and time - {}, {} - and the difference is the grain: it is {}.",
            wanted.label, held.water, held.time, held.description,
        );
    }
    format!(
        "{name} will stand in for {}: it wants {} and {}, where {} wants {} and {}. It is {}, so the dish comes out with that grain instead.",
        wanted.label,
        held.water,
        held.time,
        wanted.label,
        wanted.water,
        wanted.time,
        held.description,
    )
}
