//! Reread a request after removing its search frame, or a continuation with the prior fault.
use regex::Regex;
use std::sync::LazyLock;
macro_rules! pattern {
    ($name:ident,$source:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($source).expect(stringify!($name)));
    };
}
#[path = "reconsider_data.rs"]
mod data;
use data::*;
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new("[A-Za-z']+").unwrap());

fn has(set: &[&str], word: &str) -> bool {
    set.binary_search(&word).is_ok()
}
pub(super) fn intro(raw: &str) -> Option<String> {
    let m = INTRODUCING_A_STATE.captures(raw.trim())?;
    let word = m[1].to_ascii_lowercase();
    has(DESCRIPTIONS, &word).then(|| format!("im {word}"))
}
pub(super) fn errand(raw: &str) -> Option<(&str, bool)> {
    let m = SEARCH_FRAME.captures(raw)?;
    let rest = &raw[m.get(0)?.end()..];
    let rest = NOT_ABOUT_THE_THING
        .find(rest)
        .map_or(rest, |suffix| &rest[..suffix.start()])
        .trim_matches([' ', ',', '.', '?', '!']);
    if rest.split_whitespace().count() < 2 || A_CLAUSE_WITH_NO_SUBJECT.is_match(rest) {
        return None;
    }
    let head = m
        .name("polite")
        .or_else(|| m.name("polite2"))
        .is_none_or(|s| s.as_str().trim().is_empty());
    Some((rest, head))
}
pub(super) fn changes(raw: &str) -> bool {
    A_THING_THAT_CHANGES.is_match(raw)
}
fn second(raw: &str) -> Option<&str> {
    let line = raw.trim();
    let correction = CORRECTION_HEAD.find(line).filter(|m| {
        CORRECTION_MARKER.is_match(line) || A_VALUE_IN_FRONT_OF_NOT.is_match(&line[m.end()..])
    });
    let head = correction.or_else(|| SECOND_JOIN_HEAD.find(line));
    let rest = &line[head.map_or(0, |m| m.end())..];
    let tail = SECOND_JOIN_TAIL.find(rest);
    if head.is_none() && tail.is_none() {
        return None;
    }
    let core = rest[..tail.map_or(rest.len(), |m| m.start())].trim();
    let mut count = 0;
    for word in WORD.find_iter(core) {
        count += 1;
        if count > 5 || has(OWN_VERB, &word.as_str().to_ascii_lowercase()) {
            return None;
        }
    }
    (count > 0).then_some(core)
}
pub(super) fn fault<'a>(
    raw: &str,
    history: impl DoubleEndedIterator<Item = &'a str>,
) -> Option<String> {
    let core = second(raw)?;
    if A_FAULT_TO_LEND.is_match(core) {
        return None;
    }
    for earlier in history.rev().filter(|s| !s.trim().is_empty()).take(3) {
        if let Some(c) = A_FAULT_TO_LEND.captures(earlier) {
            return Some(format!("{core} {}", &c[1]));
        }
        second(earlier)?;
    }
    None
}

fn mapped(table: &[(&'static str, &'static str)], word: &str) -> Option<&'static str> {
    table
        .binary_search_by_key(&word, |p| p.0)
        .ok()
        .map(|i| table[i].1)
}
fn conditional(
    table: &[(&'static str, &'static str, &[&str])],
    word: &str,
    adjacent: &str,
    end_ok: bool,
) -> Option<&'static str> {
    let i = table.binary_search_by_key(&word, |p| p.0).ok()?;
    (end_ok && adjacent.is_empty() || has(table[i].2, adjacent)).then_some(table[i].1)
}
fn describes_self(words: &[(regex::Match<'_>, String)], i: usize) -> bool {
    let mut back = i;
    while back > 0 && has(FEELING_FILLER, &words[back - 1].1) {
        back -= 1
    }
    let Some(before) = back.checked_sub(1) else {
        return false;
    };
    let lead = words[before].1.as_str();
    has(FEELING_LEAD, lead)
        || matches!(
            lead,
            "was" | "wasnt" | "wasn't" | "is" | "isnt" | "isn't" | "get" | "got"
        ) && before > 0
            && matches!(words[before - 1].1.as_str(), "i" | "im" | "i'm" | "iam")
}
type Repairs = Vec<(String, String)>;
pub(super) fn confused(raw: &str) -> Option<(String, Repairs)> {
    let words: Vec<_> = WORD
        .find_iter(raw)
        .map(|m| (m, m.as_str().to_ascii_lowercase()))
        .collect();
    let mut pieces = String::new();
    let mut pairs = Vec::new();
    let mut cut = 0;
    for (i, (span, word)) in words.iter().enumerate() {
        let before = i.checked_sub(1).map_or("", |j| words[j].1.as_str());
        let after = words.get(i + 1).map_or("", |w| w.1.as_str());
        let meant = mapped(CONFUSIONS, word)
            .or_else(|| conditional(CONFUSIONS_AFTER, word, before, false))
            .or_else(|| conditional(CONFUSIONS_BEFORE, word, after, false))
            .or_else(|| {
                conditional(CONFUSIONS_IN_A_FEELING, word, after, true)
                    .filter(|_| describes_self(&words, i))
            })
            .or_else(|| {
                mapped(CONFUSIONS_BEFORE_A_QUESTION, word).filter(|_| has(QUESTION_WORDS, after))
            });
        if let Some(meant) = meant.filter(|m| *m != word) {
            pieces.push_str(&raw[cut..span.start()]);
            pieces.push_str(meant);
            cut = span.end();
            pairs.push((word.clone(), meant.to_owned()));
        }
    }
    if pairs.is_empty() {
        None
    } else {
        pieces.push_str(&raw[cut..]);
        Some((pieces, pairs))
    }
}

fn inflect(phrase: &str, plural: bool) -> String {
    let (head, last) = phrase.rsplit_once(' ').unwrap_or(("", phrase));
    let word = if plural {
        if last.ends_with("ss") || ["ch", "sh", "x", "z"].iter().any(|s| last.ends_with(s)) {
            format!("{last}es")
        } else if last.ends_with('s') {
            last.to_owned()
        } else if let Some(stem) = last.strip_suffix('y')
            && stem
                .chars()
                .next_back()
                .is_some_and(|c| !"aeiou".contains(c))
        {
            format!("{stem}ies")
        } else {
            format!("{last}s")
        }
    } else if let Some(stem) = last.strip_suffix("ies")
        && !stem.is_empty()
    {
        format!("{stem}y")
    } else if ["ses", "xes", "zes", "ches", "shes"]
        .iter()
        .any(|s| last.ends_with(s))
    {
        last[..last.len() - 2].to_owned()
    } else if !last.ends_with("ss") {
        last.strip_suffix('s').unwrap_or(last).to_owned()
    } else {
        last.to_owned()
    };
    if head.is_empty() {
        word
    } else {
        format!("{head} {word}")
    }
}
pub(super) fn borrow_noun<'a>(
    raw: &str,
    read: &super::Reading<'_>,
    history: impl DoubleEndedIterator<Item = &'a str>,
) -> Option<(String, String)> {
    let stand = super::details::which_one(raw)?;
    let topic = read
        .context_fields
        .topic_from
        .iter()
        .any(|(word, score)| *score >= 2 && has(A_DESCRIBING_SUBJECT, &word.to_lowercase()));
    if !topic {
        let mut things = read
            .entities
            .iter()
            .filter(|e| e.kind == "thing")
            .peekable();
        things.peek()?;
        if !things.all(|e| has(A_DESCRIBING_SUBJECT, &e.text.to_lowercase())) {
            return None;
        }
    }
    let plural = raw[stand.clone()].eq_ignore_ascii_case("ones");
    for past in history.rev().filter(|s| !s.trim().is_empty()).take(3) {
        let line = super::line::normalise(past);
        let entities = super::entities::detect(&line);
        let (Some(name), Some("thing")) = super::references::target(&super::references::Prior {
            entities: &entities,
            topic: None,
        }) else {
            continue;
        };
        if name.chars().count() < 3
            || has(STAND_IN_NOT_THINGS, &name)
            || plural && !name.ends_with('s')
        {
            continue;
        }
        let noun = inflect(&name, plural);
        return Some((
            format!("{}{}{}", &raw[..stand.start], noun, &raw[stand.end..]),
            noun,
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::super::{Reading, context::History};
    mod fixtures {
        type Case = (
            &'static [&'static str],
            &'static str,
            usize,
            &'static str,
            &'static str,
        );
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/reconsider.rs"
        ));
    }
    #[test]
    fn alternate_readings_match_the_pinned_conversation_reader() {
        let mut differences = Vec::new();
        for &(prev, raw, i, errand, noun) in fixtures::CASES {
            let mut history = History::default();
            for (index, &earlier) in prev.iter().enumerate() {
                let mut read =
                    Reading::understand(earlier, prev[..index].iter().copied(), None, &history);
                history.retain(&mut read);
            }
            let read = Reading::understand(raw, prev.iter().copied(), None, &history);
            let expected = (fixtures::NORMAL[i], errand, noun);
            let actual = (
                read.line.norm.join(" "),
                read.context_fields.errand.unwrap_or_default(),
                read.stand_in_noun.unwrap_or_default(),
            );
            if (actual.0.as_str(), actual.1.as_str(), actual.2.as_str()) != expected {
                differences.push((prev, raw, expected, actual))
            }
        }
        assert!(
            differences.is_empty(),
            "{} differences: {:#?}",
            differences.len(),
            differences
        );
    }
}
