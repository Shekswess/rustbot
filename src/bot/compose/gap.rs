//! Offer the relevant decision criteria for questions about unknown prices, terms or rates.
#[path = "gap_data.rs"]
mod data;
use super::{choose_recent, routes};
use crate::bot::Memory;
use data::*;
use fancy_regex::Regex;
use std::sync::LazyLock;

fn hit(regex: &Regex, text: &str) -> bool {
    regex.is_match(text).unwrap_or(false)
}

fn annual_count(text: &str) -> Option<(String, &str, u32)> {
    if !hit(&ASKS_A_COST, text) || hit(&AN_AMOUNT_ALREADY, text) || !hit(&A_LONGER_SPAN, text) {
        return None;
    }
    let (count, whole, span) = if let Some(c) = A_COUNT_IN_A_SPAN.captures(text).ok().flatten() {
        let word = c.get(1)?.as_str().to_lowercase();
        let n = match word.as_str() {
            "one" => "1",
            "two" | "twice" => "2",
            "three" | "three times" => "3",
            "four" | "four times" => "4",
            "five" | "five times" => "5",
            "six" => "6",
            "seven" => "7",
            digits => digits,
        };
        (
            n.trim_start_matches('0').to_owned(),
            c.get(0)?.as_str(),
            c.get(2)?.as_str(),
        )
    } else {
        let c = A_EVERY_SPAN.captures(text).ok().flatten()?;
        ("1".into(), c.get(0)?.as_str(), c.get(1)?.as_str())
    };
    if count.is_empty() {
        return None;
    }
    let factor = match span.to_ascii_lowercase().as_str() {
        "day" => 365,
        "week" => 52,
        "month" => 12,
        _ => return None,
    };
    Some((count, whole, factor))
}
// A decimal count may exceed the machine integer range.
fn scaled_count(count: &str, factor: u32) -> Option<String> {
    let mut digits = Vec::new();
    let mut carry = 0;
    for ch in count.chars().rev() {
        let n = ch.to_digit(10)? * factor + carry;
        digits.push((n % 10) as u8 + b'0');
        carry = n / 10;
    }
    while carry > 0 {
        digits.push((carry % 10) as u8 + b'0');
        carry /= 10;
    }
    let mut written = String::new();
    for (index, digit) in digits.iter().enumerate().rev() {
        written.push(*digit as char);
        if index != 0 && index % 3 == 0 {
            written.push(',')
        }
    }
    Some(written)
}
fn summed(text: &str) -> Option<[String; 2]> {
    let (count, whole, factor) = annual_count(text)?;
    let total = scaled_count(&count, factor)?;
    let price = scaled_count(&count, factor * 5)?;
    let full = whole.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut chars = full.chars();
    let written = format!("{}{}", chars.next()?.to_uppercase(), chars.as_str());
    Some([
        format!(
            "{written} is {total} a year. Put your own price on one - £5 each is £{price} - and that is the figure to weigh against cutting it back."
        ),
        format!(
            "{written} is {total} times a year, so put a price on one: at a fiver each that is £{price}, which is what the habit really costs."
        ),
    ])
}
fn options(s: &str) -> Option<&'static [&'static str]> {
    static PERSONAL_WAIT: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(?:how long|how soon) (?:do|should|shall|can|will) (?:i|we) (?:wait|hold|hang|leave|give|keep|sit)\b").unwrap()
    });
    if s.is_empty() || hit(&NOT_HERE, s) || !(hit(&ASKS_SOMETHING, s) || hit(&NAME_ONLY_ASK, s)) {
        return None;
    }
    kinds().into_iter().find_map(|(kind, pattern, lines)| {
        if !hit(pattern, s) {
            return None;
        }
        let excluded = match kind {
            "howto" => hit(&NOT_A_HOWTO, s) || !hit(&A_HOWTO_HAS_A_SHAPE, s),
            "worth" => !(hit(&A_REPAIR_OR_BUY, s) || hit(&A_BROKEN_THING, s)),
            "rule" => hit(&NOT_A_RULE, s) || (hit(&A_DO_I_NEED, s) && !hit(&A_RULE_SUBJECT, s)),
            "price" => hit(&NOT_A_PRICE, s) || annual_count(s).is_some(),
            "duration" => hit(&BARE_DURATION, s) || hit(&PERSONAL_WAIT, s) || !hit(&A_QUEUE, s),
            "record" => {
                hit(&NOT_A_RECORD, s)
                    || (!hit(&A_NAMED_DOCUMENT, s)
                        && !hit(&THEIR_OWN_PAPER, s)
                        && hit(&A_LOOSE_KIND_WORD, s))
            }
            _ => false,
        };
        (!excluded).then_some(lines)
    })
}
pub(crate) fn replace(clean: &str, reply: &str, memory: &Memory) -> Option<String> {
    let said = reply
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if !(routes::hedge(reply) || hit(&A_POINTER, &said)) || hit(&THE_PAPER_ITSELF, &said) {
        return None;
    }
    if let Some(lines) = summed(clean) {
        return Some(choose_recent(&[&lines[0], &lines[1]], memory, clean).into());
    }
    Some(choose_recent(options(clean)?, memory, clean).into())
}

#[cfg(test)]
#[path = "../../../tests/fixtures/gap.rs"]
mod reference;
#[test]
fn reproduces_pinned_gap_replies() {
    let memory = Memory::default();
    for &(q, expected) in reference::CASES {
        assert_eq!(
            replace(
                q,
                "A quick search online will get you a solid answer.",
                &memory
            )
            .as_deref(),
            expected,
            "{q}"
        );
    }
}
