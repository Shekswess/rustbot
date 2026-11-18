//! Quantities and units from the normalized and original views of a line.
#[path = "number_data.rs"]
mod data;
use super::line::Line;
use data::*;

#[derive(Debug, serde::Serialize)]
pub(crate) struct Quantity {
    pub value: f64,
    pub unit: Option<&'static str>,
    pub text: String,
}
fn get<T: Copy>(table: &[(&'static str, T)], key: &str) -> Option<T> {
    table
        .binary_search_by(|(item, _)| item.cmp(&key))
        .ok()
        .map(|at| table[at].1)
}
fn has(set: &[&str], word: &str) -> bool {
    set.binary_search(&word).is_ok()
}
fn fraction(tail: &[String]) -> (f64, usize) {
    if tail.first().map(String::as_str) != Some("and") {
        return (0.0, 0);
    }
    let index = if tail
        .get(1)
        .is_some_and(|w| matches!(w.as_str(), "a" | "an"))
    {
        2
    } else {
        1
    };
    match tail.get(index).map(String::as_str) {
        Some("half") => (0.5, index + 1),
        Some("quarter") => (0.25, index + 1),
        _ => (0.0, 0),
    }
}
fn not_a_count(norm: &[String], at: usize) -> bool {
    if norm[at] != "one" {
        return false;
    }
    let rest = &norm[at + 1..];
    let following = rest.first().map(String::as_str);
    let mixed = following == Some("and")
        && match rest.get(1).map(String::as_str) {
            Some("half" | "quarter") => true,
            Some("a" | "an") => rest.get(2).is_some_and(|w| w == "half"),
            _ => false,
        };
    if mixed || following.is_some_and(|w| get(UNIT_WORDS, w).is_some()) {
        return false;
    }
    at == 0 || has(ONE_IS_A_THING, &norm[at - 1]) || following.is_none()
}
fn decimal(text: &str) -> Option<f64> {
    let n = text.replace(',', ".").parse::<f64>().ok()?;
    n.is_finite().then_some(n)
}
pub(crate) fn detect(line: &Line) -> Vec<Quantity> {
    let norm = &line.norm;
    let mut out = Vec::new();
    let spoken = norm
        .iter()
        .filter_map(|word| {
            get(WORD_NUMBERS, word).or_else(|| decimal(WORD_DIGITS.find(word)?.as_str()))
        })
        .collect::<Vec<_>>();
    let mut typed = Vec::new();
    let mut seen = Vec::new();
    for m in NUMBER_RE.captures_iter(&line.clean) {
        let all = m.get(0).unwrap();
        let digits = &m[1];
        let Some(value) = decimal(digits) else {
            continue;
        };
        let unit = m.get(2).map_or("", |m| m.as_str());
        if !spoken.contains(&value)
            || has(MIXED_JOINERS, unit) && MIXED_FRACTION.is_match(&line.clean[all.end()..])
        {
            continue;
        }
        let mut canonical = get(UNIT_WORDS, unit)
            .or_else(|| get(UNIT_WORDS, unit.trim_end_matches('s')))
            .or_else(|| get(CLIPPED_UNIT_WORDS, unit));
        if canonical.is_none()
            && unit.is_empty()
            && let Some(t) = CLIPPED_AFTER_A_SLASH.find(&line.clean[all.end()..])
        {
            canonical = Some(get(CLIPPED_UNIT_WORDS, t.as_str().trim()).unwrap_or("years"));
        }
        typed.push((
            line.clean[..all.start()]
                .bytes()
                .filter(|&c| c == b' ')
                .count(),
            value,
        ));
        let key = (value, canonical, None);
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);
        let text = if unit == "%" {
            format!("{digits}%")
        } else if get(UNIT_WORDS, unit).is_some()
            || get(CLIPPED_UNIT_WORDS, unit).is_some()
            || has(FIGURE_SIZE_WORDS, unit)
        {
            let gap = if all.as_str()[digits.len()..].starts_with(char::is_whitespace) {
                " "
            } else {
                ""
            };
            format!("{digits}{gap}{unit}")
        } else {
            digits.to_owned()
        };
        out.push(Quantity {
            value,
            unit: canonical,
            text,
        });
    }
    let mut through = 0;
    for (at, word) in norm.iter().enumerate() {
        let next = norm.get(at + 1).map_or("", String::as_str);
        if matches!(word.as_str(), "a" | "an") {
            let unit = get(UNIT_WORDS, next);
            let key = (1.0, unit, None);
            let owner = at
                .checked_sub(1)
                .and_then(|i| norm.get(i))
                .is_some_and(|w| has(OWNS_THE_UNIT, w));
            if has(A_PORTION_UNITS, next)
                && unit.is_some()
                && !owner
                && !seen.contains(&key)
                && (!has(A_PORTION_NEEDS_OF, next) || norm.get(at + 2).is_some_and(|w| w == "of"))
            {
                seen.push(key);
                out.push(Quantity {
                    value: 1.,
                    unit,
                    text: format!("{word} {next}"),
                });
            }
            continue;
        }
        let Some(base) = get(WORD_NUMBERS, word) else {
            continue;
        };
        if not_a_count(norm, at)
            || at < through
            || typed.iter().any(|&(i, v)| i.abs_diff(at) <= 1 && v == base)
        {
            continue;
        }
        let (part, folded) = fraction(&norm[(at + 1)..]);
        through = at + folded + 1;
        let value = base + part;
        let mut unit = None;
        for word in norm.iter().skip(through).take(5 - folded) {
            unit = get(UNIT_WORDS, word);
            if unit.is_some() || !has(NUMBER_FILLER, word) {
                break;
            }
        }
        let key = (value, unit, Some(word.as_str()));
        if !seen.contains(&key) {
            seen.push(key);
            out.push(Quantity {
                value,
                unit,
                text: word.clone(),
            });
        }
    }
    out
}

pub(super) fn age(word: &str) -> f64 {
    get(WORD_NUMBERS, word)
        .or_else(|| {
            WORD_DIGITS
                .find(word)
                .filter(|m| m.start() == 0)
                .and_then(|m| decimal(m.as_str()))
        })
        .unwrap_or(99.)
}

pub(super) fn word_number(w: &str) -> Option<f64> {
    get(WORD_NUMBERS, w)
}
