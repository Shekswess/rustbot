//! The price lead and route in PyBot's `compose/not_knowing.py`.

use super::{choose_recent, fallback, routes};
use crate::bot::Memory;
use regex::Regex;
use std::sync::LazyLock;

static PRICE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?:^|[^a-z])(?:how much (?:is|does|do|would|will|are|should)|what does .{1,30} cost|",
        r"the cost of|price of|prices for|how expensive|cheapest|going rate|quote|",
        r"charge|charges|fee|fees|fare|fares|how much (?:will|would) it cost)(?:$|[^a-z])",
    ))
    .expect("price question words")
});
static DECISION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^\s*(?:so |and |ok |okay |but |well )*(?:should|shall|ought) i(?:$|[^a-z])|",
        r"(?:^|[^a-z])(?:do i bother|do i go for|is it worth|worth (?:it|fixing|replacing)|",
        r"better to|which (?:one|is better)|what would you do)(?:$|[^a-z])",
    ))
    .expect("decision wording")
});

const PRICE_LEADS: &[&str] = &[
    "Prices move week to week and shop to shop, so a number from me would be out of date the moment I said it.",
    "The price depends on where you buy and when, which is not something I can see.",
];
const PRICE_ROUTES: &[&str] = &[
    "The shop's own page has today's price, and a comparison site shows what the others are charging this week.",
    "Look at two or three sellers' own pages - the spread is usually bigger than people expect.",
];

// A bare search pointer can give way to a price lead after the named readers.
pub(crate) fn replace(clean: &str, reply: &str, memory: &Memory) -> Option<String> {
    if !fallback::unknown_replies().contains(&reply)
        || !PRICE.is_match(clean)
        || DECISION.is_match(clean)
        || routes::has_route(clean)
    {
        return None;
    }
    let lead = choose_recent(PRICE_LEADS, memory, clean);
    let route = choose_recent(PRICE_ROUTES, memory, clean);
    Some(format!("{lead} {route}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn a_price_lead_uses_the_reference_pools_without_taking_named_routes_or_decisions() {
        let mut memory = Memory::default();
        for _ in 0..58 {
            memory.turns.push(Exchange {
                user: "A prior turn".into(),
                bot: "Its reply.".into(),
                module: "answers",
            });
        }
        assert_eq!(
            replace(
                "what's the difference between interest and an account fee?",
                fallback::unknown_replies()[0],
                &memory
            ),
            Some(format!("{} {}", PRICE_LEADS[0], PRICE_ROUTES[0]))
        );
        assert_eq!(
            replace("what is the fee?", "Ask the retailer directly.", &memory),
            None
        );
        for line in [
            "what difference does coffee make?",
            "should i pay the fee?",
            "what is the cancellation policy for this booking fee?",
        ] {
            assert_eq!(
                replace(line, fallback::unknown_replies()[0], &memory),
                None,
                "{line}"
            );
        }
    }
}
