//! Fittings and the repair trades named by their jobs.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Reading, modules::safety::current_gas};

use super::guards::Context;

#[path = "home_fittings_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));
    };
}
pattern!(TRADE_ASKS);
pattern!(NOT_A_TRADE);
pattern!(WINDOW_COST_RE);

static TRADES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    data::TRADES
        .iter()
        .map(|(names, reply)| (any_whole(names), *reply))
        .collect()
});

static FITTINGS: LazyLock<Vec<(&'static str, Regex)>> = LazyLock::new(|| {
    data::FITTINGS
        .iter()
        .map(|(name, words)| (*name, any_whole(words)))
        .collect()
});

static CUPBOARD: LazyLock<Regex> = LazyLock::new(|| any_whole(data::CUPBOARD_WORDS));

fn any_whole(names: &[&str]) -> Regex {
    let alternatives = names
        .iter()
        .map(|name| regex::escape(name))
        .collect::<Vec<_>>();
    Regex::new(&format!(r"\b(?:{})\b", alternatives.join("|")))
        .expect("whole fitting or trade name")
}

fn mentions(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

fn clean(line: &str) -> String {
    line.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn fitting_repair(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.text,
        reading.line.words.join(" ")
    )
    .to_lowercase();
    if TRADE_ASKS.is_match(&said) || mentions(&said, data::FITTING_SHARED_STANDDOWN) {
        return None;
    }
    let kind = FITTINGS
        .iter()
        .find_map(|(kind, names)| names.is_match(&said).then_some(*kind))
        .or_else(|| {
            let subject = reading.context_fields.subject.as_deref()?.to_lowercase();
            let words = subject.split(|ch: char| !ch.is_ascii_lowercase());
            data::FITTINGS.iter().find_map(|(kind, names)| {
                words
                    .clone()
                    .any(|word| names.contains(&word))
                    .then_some(*kind)
            })
        })
        .or_else(|| {
            if mentions(&said, data::STAY_SHUT) {
                Some("catch")
            } else if mentions(&said, data::CROOKED) {
                Some("crooked")
            } else {
                CUPBOARD.is_match(&said).then_some("cupboard")
            }
        })?;
    if kind == "oven" && !mentions(&said, data::OVEN_PART) {
        return None;
    }
    let &(_, avoid, reply) = data::FITTING_REPLY
        .iter()
        .find(|(name, _, _)| *name == kind)?;
    (!mentions(&said, avoid)).then_some(Cow::Borrowed(reply))
}

pub(super) fn broken_window(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = clean(&reading.line.clean);
    if TRADE_ASKS.is_match(&line) {
        return None;
    }
    if WINDOW_COST_RE.is_match(&line) || mentions(&line, data::WINDOW_ASKS) {
        return Some(Cow::Owned(format!(
            "{}{}",
            data::WINDOW_COST,
            data::WINDOW_SAFE
        )));
    }
    Some(Cow::Borrowed(data::WINDOW_SAFE))
}

pub(super) const GAS_EMERGENCY: &str = "If you smell gas or suspect a leak, leave the building and call your local gas emergency service from outside. In Great Britain, call 0800 111 999.";

fn trade_for(line: &str) -> Option<&'static str> {
    if NOT_A_TRADE.is_match(line) {
        return None;
    }
    if current_gas(line) {
        return Some(GAS_EMERGENCY);
    }
    TRADES
        .iter()
        .find_map(|(pattern, reply)| pattern.is_match(line).then_some(*reply))
}

pub(super) fn trade_to_call(context: &Context<'_>) -> Option<&'static str> {
    let line = clean(if context.reading.line.clean.is_empty() {
        &context.reading.line.text
    } else {
        &context.reading.line.clean
    });
    if !TRADE_ASKS.is_match(&line) || NOT_A_TRADE.is_match(&line) {
        return None;
    }
    if let Some(reply) = trade_for(&line) {
        return Some(reply);
    }
    let last_turn = context.earlier.len().checked_sub(1)?;
    if let Some(reply) = trade_for(&clean(&context.earlier[last_turn].user)) {
        return Some(reply);
    }
    if let Some(memory) = context.memory {
        if memory.doneness_topics.recorded_on_turn(last_turn) {
            return None;
        }
        if context.reading.reference.followup
            && let Some(topic) = memory.doneness_topics.current_text()
            && let Some(reply) = trade_for(&clean(topic))
        {
            return Some(reply);
        }
    }
    let previous_turn = last_turn.checked_sub(1)?;
    trade_for(&clean(&context.earlier[previous_turn].user))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_broken_window_separates_prices_repairs_and_the_trade() {
        let price = Reading::new("how much does it cost to replace a broken window?");
        assert!(
            broken_window(&price)
                .unwrap()
                .starts_with("Ask a local glazier")
        );
        let fault = Reading::new("someone broke my window");
        assert!(
            broken_window(&fault)
                .unwrap()
                .starts_with("Keep people and pets away")
        );
        assert_eq!(fitting_repair(&fault), None);
        let trade = Reading::new("who fixes a broken window?");
        assert_eq!(broken_window(&trade), None);
        assert!(
            trade_to_call(&Context::new(&trade, &[]))
                .unwrap()
                .starts_with("A glazier")
        );
    }

    #[test]
    fn a_fitting_names_a_part_and_stands_down_for_other_jobs() {
        let shelf = Reading::new("my cupboard shelf has snapped");
        assert!(
            fitting_repair(&shelf)
                .unwrap()
                .starts_with("A snapped shelf")
        );
        let door = Reading::new("my oven door will not close");
        assert!(fitting_repair(&door).unwrap().contains("let it cool"));
        assert_eq!(fitting_repair(&Reading::new("my oven is broken")), None);
    }

    #[test]
    fn trade_uses_shared_hazard_routing_and_prior_fault_context() {
        let urgent = Reading::new("who do I call about a gas leak?");
        assert_eq!(
            trade_to_call(&Context::new(&urgent, &[])),
            Some(GAS_EMERGENCY)
        );
        let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
        bot.turn("my roof is leaking behind the gutter");
        let ask = Reading::new("who do I call?");
        let mut context = Context::new(&ask, &bot.memory.turns);
        context.memory = Some(&bot.memory);
        assert!(trade_to_call(&context).unwrap().starts_with("A roofer"));
    }
}
