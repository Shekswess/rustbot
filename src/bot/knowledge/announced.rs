//! Read a cooking announcement that also asks for a plan's time, heat or doneness.
use crate::bot::{
    Exchange, Memory, Reading,
    understanding::{
        normalize::{engine_reads, one_edit, tokens},
        topics,
    },
};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::{borrow::Cow, collections::HashSet, sync::LazyLock};

struct Phrase {
    text: &'static str,
    words: &'static [&'static str],
}
struct Plan {
    label: &'static str,
    names: &'static [Phrase],
    avoid: &'static [&'static str],
    time: &'static str,
    temp: &'static str,
    done: &'static str,
    keep: &'static str,
    ahead: &'static str,
    rest: &'static str,
    skin: &'static str,
    stuffing: &'static str,
    frosting: &'static str,
    fill: &'static str,
}
struct Question {
    kind: &'static str,
    priority: u8,
    asks: &'static [&'static [&'static str]],
    helpers: &'static [&'static str],
}
macro_rules! pattern {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Fancy> =
            LazyLock::new(|| Fancy::new($pattern).expect(stringify!($name)));
    };
}
include!("announced_data.rs");
#[path = "doneness_reply.rs"]
pub(super) mod doneness_reply;

static QUESTION_START: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bhow\b|\bwhat temperature\b|\bwhat temp\b|\bis it done\b").unwrap()
});
static SUBJECT_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());

fn has(set: &[&str], word: &str) -> bool {
    set.binary_search(&word).is_ok()
}
fn same(a: &str, b: &str) -> bool {
    a == b
        || [(a, b), (b, a)].iter().any(|&(one, two)| {
            one.len() > 3
                && (["s", "es", "ing", "ed"]
                    .iter()
                    .any(|suffix| one.strip_suffix(suffix) == Some(two))
                    || one
                        .strip_suffix("ing")
                        .is_some_and(|root| two.strip_suffix('e') == Some(root)))
        })
}
fn slip(typed: &str, key: &str) -> bool {
    typed.len() >= 4
        && key.len() >= 4
        && !key.contains(' ')
        && typed.as_bytes()[0] == key.as_bytes()[0]
        && !engine_reads(typed)
        && one_edit(typed, key)
}
fn matches(window: &[String], phrase: &[&str], allow_slip: bool) -> bool {
    window
        .iter()
        .zip(phrase)
        .all(|(word, &key)| same(word, key) || (allow_slip && slip(word, key)))
}
fn contains(line: &[String], phrase: &[&str]) -> bool {
    !phrase.is_empty()
        && line
            .windows(phrase.len())
            .any(|w| matches(w, phrase, false))
}
fn cut(word: &str) -> bool {
    CUTS.iter().any(|&key| {
        if same(word, key) {
            return true;
        }
        if word.len() < 4 || word.len().abs_diff(key.len()) > 1 {
            return false;
        }
        if word.len() == key.len() {
            return word
                .bytes()
                .zip(key.bytes())
                .filter(|(a, b)| a != b)
                .count()
                == 1;
        }
        let (short, long) = if word.len() < key.len() {
            (word, key)
        } else {
            (key, word)
        };
        let differing = short
            .bytes()
            .zip(long.bytes())
            .position(|(a, b)| a != b)
            .unwrap_or(short.len());
        short.as_bytes()[differing..] == long.as_bytes()[differing + 1..]
    })
}
fn ruled_out(words: &[String], start: usize, n: usize) -> bool {
    let bare = |s: &str| s.replace(['\'', '’'], "");
    if words
        .get(start + n)
        .is_some_and(|w| has(INGREDIENT_AFTER, &bare(w)))
    {
        return false;
    }
    words[start.saturating_sub(5)..start]
        .iter()
        .rev()
        .map(|w| bare(w))
        .find(|w| !has(REFUSAL_FILLER, w))
        .is_some_and(|w| has(REFUSED, &w))
}
fn named(words: &[String], text: &str) -> Option<&'static Plan> {
    let plain: Vec<String> = text
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    let elsewhere: HashSet<&str> = plain
        .iter()
        .enumerate()
        .filter(|(i, _)| topics::elsewhere(&plain, *i))
        .map(|(_, w)| w.as_str())
        .collect();
    let mut covered = vec![false; words.len()];
    let mut candidates = Vec::new();
    for plan in PLANS {
        for alias in plan.names {
            let n = alias.words.len();
            for (i, window) in words.windows(n).enumerate() {
                if !matches(window, alias.words, n > 1) {
                    continue;
                }
                if ruled_out(words, i, n) && matches(window, alias.words, false) {
                    covered[i..i + n].fill(true);
                }
                if window.iter().any(|w| elsewhere.contains(w.as_str())) {
                    continue;
                }
                if let Some(after) = words.get(i + n)
                    && (cut(after)
                        || MADE.iter().any(|&d| same(after, d))
                        || plan.avoid.iter().any(|&a| same(after, a) || slip(after, a)))
                {
                    continue;
                }
                candidates.push((n, i, plan));
            }
        }
    }
    let mut best = None;
    for (len, start, plan) in candidates {
        if covered[start..start + len].iter().any(|&v| v) {
            continue;
        }
        if best.as_ref().is_none_or(|&(n, i, _)| (len, start) > (n, i)) {
            best = Some((len, start, plan));
        }
    }
    best.map(|(_, _, plan)| plan)
}
fn doneness(text: &str) -> bool {
    let Some(m) = DONE_ASK
        .find(text)
        .ok()
        .flatten()
        .or_else(|| DONE_VERB.find(text).ok().flatten())
    else {
        return false;
    };
    let span = m.as_str();
    let words = tokens(span);
    DONENESS_FOODS.iter().any(|f| contains(&words, f))
        || SUBJECT_WORD.find_iter(span).all(|m| {
            let w = m.as_str();
            has(ASK_NOISE, w) || has(DONE_WORDS, w) || has(GENERIC_SUBJECT, w)
        })
}
fn question(text: &str, words: &[String]) -> Option<&'static str> {
    let mut best = (0, 0);
    let mut kind = None;
    for q in KINDS {
        let mut score = q
            .asks
            .iter()
            .find(|p| contains(words, p))
            .map_or(0, |p| 2 + p.len());
        if q.kind == "done" && doneness(text)
            || q.kind == "keep" && KEEP.is_match(text).unwrap_or(false)
        {
            score = score.max(6);
        }
        if score == 0 || q.kind == "keep" && FAULT.is_match(text).unwrap_or(false) {
            continue;
        }
        score += usize::from(
            words
                .iter()
                .any(|word| q.helpers.iter().any(|helper| same(word, helper))),
        );
        let rank = (q.priority, score);
        if rank > best {
            best = rank;
            kind = Some(q.kind);
        }
    }
    kind
}
fn another_food(text: &str, plan: &Plan) -> bool {
    let Some(last) = QUESTION_START.find_iter(text).last() else {
        return false;
    };
    tokens(&text[last.end()..]).iter().any(|word| {
        !has(FILLER, word)
            && !word.bytes().any(|b| b.is_ascii_digit())
            && !plan.names.iter().any(|n| n.words.contains(&word.as_str()))
    })
}
fn literal(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(start, _)| {
        let end = start + name.len();
        (start == 0 || !text.as_bytes()[start - 1].is_ascii_alphanumeric())
            && (end == text.len() || !text.as_bytes()[end].is_ascii_alphanumeric())
    })
}
fn start_sentence(plan: &Plan, text: &str) -> String {
    let mut label = plan.label;
    let lower = label.to_lowercase();
    if let Some(name) = plan.names.iter().find(|name| {
        !matches!(name.words.first(), Some(&"a" | &"an" | &"the")) && literal(text, name.text)
    }) {
        let said = name.text;
        if !lower.contains(said)
            && !said.contains(&lower)
            && lower.chars().take(6).ne(said.chars().take(6))
        {
            label = said;
        }
    }
    let mut chars = label.chars();
    let mut out = chars
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_default();
    out.push_str(chars.as_str());
    out.push_str(": ");
    out.push_str(plan.time);
    let temperature = plan.temp.split('C').next().unwrap_or_default().trim();
    if !plan.temp.is_empty() && (plan.time.is_empty() || !plan.time.contains(temperature)) {
        if !plan.time.is_empty() {
            out.push(' ');
        }
        out.push_str(plan.temp);
    }
    out
}

pub(crate) fn part(reading: &Reading<'_>) -> Option<String> {
    let text = if reading.line.norm_text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.norm_text
    };
    if !PERSON.is_match(text).unwrap_or(false) || reading.line.words.iter().any(|w| has(STAGES, w))
    {
        return None;
    }
    let words = tokens(text);
    let kind = question(text, &words)?;
    if !matches!(kind, "done" | "temp" | "time") {
        return None;
    }
    let plan = named(&words, text)?;
    if another_food(text, plan) {
        return None;
    }
    if kind == "done" {
        (!plan.done.is_empty() && !SAUCE.is_match(text).unwrap_or(false))
            .then(|| plan.done.to_owned())
    } else {
        Some(start_sentence(plan, text))
    }
}

pub(in crate::bot::knowledge) fn named_recipe_phrase(text: &str) -> bool {
    named(&tokens(text), text).is_some_and(|plan| {
        plan.names
            .iter()
            .any(|name| name.words.len() > 1 && literal(text, name.text))
    })
}

pub(crate) fn names_dish(text: &str) -> bool {
    plan_label(text).is_some()
}

pub(in crate::bot::knowledge) fn named_as_excluded(text: &str, label: &str) -> bool {
    let words = tokens(text);
    PLANS.iter().filter(|plan| plan.label == label).any(|plan| {
        plan.names.iter().any(|name| {
            words
                .windows(name.words.len())
                .enumerate()
                .any(|(i, window)| {
                    matches(window, name.words, false) && ruled_out(&words, i, name.words.len())
                })
        })
    })
}

pub(in crate::bot::knowledge) fn plan_label(text: &str) -> Option<&'static str> {
    named(&tokens(text), text).map(|plan| plan.label)
}

pub(in crate::bot::knowledge) fn plan_label_in_chat(
    text: &str,
    earlier: &[Exchange],
) -> Option<&'static str> {
    let words = tokens(text);
    plan_in_chat(&words, text, earlier).map(|plan| plan.label)
}

fn plan_in_chat(words: &[String], text: &str, earlier: &[Exchange]) -> Option<&'static Plan> {
    named(words, text).or_else(|| {
        if doneness_reply::names_food(words) {
            return None;
        }
        earlier.iter().rev().take(4).find_map(|turn| {
            let text = turn.user.to_lowercase();
            named(&tokens(&text), &text)
        })
    })
}

pub(in crate::bot::knowledge) fn named_dish_has_heat(text: &str) -> bool {
    named(&tokens(text), text).is_some_and(|plan| !plan.temp.is_empty())
}

pub(in crate::bot::knowledge) fn oven_setting(text: &str) -> Option<&'static str> {
    let words = tokens(text);
    if question(text, &words) != Some("temp") || SAUCE.is_match(text).unwrap_or(false) {
        return None;
    }
    let plan = named(&words, text)?;
    (!plan.temp.is_empty()).then_some(plan.temp)
}

pub(in crate::bot::knowledge) fn practical_plan(
    text: &str,
    earlier: &[Exchange],
    is_timed_deliberation: bool,
) -> Option<String> {
    let words = tokens(text);
    if question(text, &words).is_some()
        || !(PERSON.is_match(text).unwrap_or(false) || is_timed_deliberation)
    {
        return None;
    }
    let plan = plan_in_chat(&words, text, earlier)?;
    Some(start_sentence(plan, text))
}

#[path = "dish_followup.rs"]
pub(in crate::bot::knowledge) mod followup;

#[cfg(test)]
mod tests {
    use super::*;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/announced.rs"
    ));
    #[test]
    fn announced_recipe_parts_match_reference() {
        let mut differences = Vec::new();
        for &(text, expected) in INPUTS {
            let reading = Reading::new(text);
            let result = part(&reading);
            if result.as_deref() != expected.map(|i| RESULTS[i]) {
                differences.push((text, result, expected.map(|i| RESULTS[i])));
            }
        }
        assert!(
            differences.is_empty(),
            "{} differences: {differences:#?}",
            differences.len()
        );
    }
}
