//! Look up the question implied by a dietary, portion, trip, or purchase detail.

use crate::bot::{Memory, Reading, knowledge, modules};
use crate::text::is_space;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(concat!("(?i)", $source)).expect(stringify!($name)));
    };
}

pattern!(
    ASKS,
    r"^\W*(?:how|what|whats|what's|which|when|where|who|why)\b|^\W*(?:is|are|was|were|do|does|did|can|could|should|would|will|have|has|had|am)\s+(?:i|you|we|they|it|that|this|there|he|she|any|my|your|his|her|their|the|a|an)\b|\b(?:what|how|which|who|when|where) about\b"
);
pattern!(
    DIET,
    r"\b(vegan|vegetarian|coeliac|celiac|dairy[- ]free|gluten[- ]free|pescatarian)\b"
);
pattern!(
    COUNT,
    r"\b(?:there(?:'?s| is| are)\s+)?(\d+|one|two|three|four|five|six|seven|eight|nine|ten)\s+(?:of us|people|adults|kids|children|guests|coming|more|extra|others|mouths)\b"
);
pattern!(ADDITIONAL, r"\b(?:more|extra|another)\b");
pattern!(
    GOING,
    r"\b(?:off|going|heading|away|flying|driving|travelling|traveling)\s+to\s+([a-z][a-z' -]{2,24}?)(?:\s+(?:in|on|for|next|at)\b|$)"
);
pattern!(
    MONTH,
    r"\b(january|february|march|april|may|june|july|august|september|october|november|december|spring|summer|autumn|winter)\b"
);
pattern!(
    CONSIDERING,
    r"\b(?:looking at|thinking (?:of|about)|considering|about to (?:buy|get|order)|going to (?:buy|get|order)|planning (?:to get|on getting))\s+(?:an\s+|the\s+|some\s+|a\s+)?([a-z][a-z' -]{2,30}?)(?:\s+(?:for|in|because|to|but)\b|[.,!?]|$)"
);
pattern!(
    BUYING_PREFIX,
    r"^(?:(?:buying|getting|ordering)\s+)?(?:a|an|the)\s+"
);
pattern!(
    FOOD,
    r"\b(?:food|cook|cooking|cooked|eat|eats|eating|dish|recipe|meal|meals|menu|portion|portions|serves|serving|dinner|roast|bake|baking|ingredient|ingredients|cheese|chicken|pasta|rice|soup|salad)\b"
);
pattern!(WORD, r"[a-z']{4,}");

enum Detail<'a> {
    Diet(&'a str),
    Count(&'a str),
    Additional,
    Going(&'a str),
    Considering(&'a str),
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(is_space).filter(|part| !part.is_empty())
}

fn capture<'a>(pattern: &Regex, text: &'a str) -> Option<&'a str> {
    pattern
        .captures(text)?
        .get(1)
        .map(|part| part.as_str().trim())
}

fn detail(clean: &str) -> Option<Detail<'_>> {
    if ASKS.is_match(clean.split(',').next()?) {
        return None;
    }
    if let Some(diet) = capture(&DIET, clean) {
        return Some(Detail::Diet(diet));
    }
    if let Some(count) = capture(&COUNT, clean) {
        return Some(if ADDITIONAL.is_match(clean) {
            Detail::Additional
        } else {
            Detail::Count(count)
        });
    }
    capture(&GOING, clean).map(Detail::Going).or_else(|| {
        capture(&CONSIDERING, clean).map(|thing| {
            Detail::Considering(
                BUYING_PREFIX
                    .find(thing)
                    .map_or(thing, |prefix| &thing[prefix.end()..]),
            )
        })
    })
}

fn about_food(memory: &Memory) -> bool {
    memory
        .entries
        .latest()
        .is_some_and(|answer| FOOD.is_match(answer))
        || memory
            .turns
            .iter()
            .rev()
            .find(|turn| !turn.bot.is_empty())
            .is_some_and(|turn| FOOD.is_match(&turn.bot))
}

pub(crate) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if reading.raw.len() < 3
        || reading.raw.contains('?')
        || words(reading.raw).take(17).count() > 16
    {
        return None;
    }
    let clean = words(reading.raw)
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let detail = detail(&clean)?;
    let question = match detail {
        Detail::Diet(diet) => format!("what can i cook for a {diet}"),
        Detail::Count(count) if about_food(memory) => format!("how much food for {count} people"),
        Detail::Additional if about_food(memory) => "how much food for one more person".to_owned(),
        Detail::Going(place) => capture(&MONTH, &clean).map_or_else(
            || format!("what should i know before a trip to {place}"),
            |month| format!("what should i know before a trip to {place} in {month}"),
        ),
        Detail::Considering(thing) => format!("what do i need to know before buying {thing}"),
        Detail::Count(_) | Detail::Additional => return None,
    };
    let implied = Reading::new(&question);
    if knowledge::withheld::applies(&question, Some(memory)) {
        return None;
    }
    let (answer, plan) = modules::answers::offer_for_turn(&implied, &Memory::default())?;
    if let Detail::Diet(diet) = detail
        && !knowledge::lookup::dietary_menus::detail_answer(
            diet,
            reading,
            answer.selected_card,
            plan.as_ref(),
            memory,
        )
    {
        return None;
    }
    let answer = answer.reply;
    if matches!(
        detail,
        Detail::Diet(_) | Detail::Count(_) | Detail::Additional
    ) && memory.pantry.blocks_typed_offer(memory)
        && memory.pantry.reply_uses_blocked_food(&answer, memory)
    {
        return None;
    }
    let said = words(&answer).collect::<Vec<_>>().join(" ");
    if said.chars().take(40).count() < 40
        || super::routes::hedge(&said)
        || super::fallback::is_neutral(&said)
        || memory
            .turns
            .iter()
            .any(|turn| words(&turn.bot).eq(words(&said)))
    {
        return None;
    }
    if let Detail::Going(noun) | Detail::Considering(noun) = detail {
        let mut named = WORD.find_iter(noun).peekable();
        if named.peek().is_some() {
            let low = answer.to_lowercase();
            if !named.any(|word| low.contains(word.as_str())) {
                return None;
            }
        }
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use crate::bot::Bot;

    #[test]
    fn a_headcount_implies_portions_in_a_meal_conversation() {
        let prompt = "theres eight of us at home";
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("im doing the food for a party");
        let turn = bot.turn(prompt);
        assert_eq!(turn.module, Some("detail_question"));
        assert_eq!(
            turn.reply,
            "About 300 g of the dish each, so 2.4 kg for eight."
        );
        assert!(
            !bot.turn(prompt)
                .candidates
                .iter()
                .any(|row| row.module == "detail_question")
        );

        let mut unrelated = Bot::new(Some("0")).unwrap();
        unrelated.turn("my boiler is noisy");
        assert!(
            !unrelated
                .turn(prompt)
                .candidates
                .iter()
                .any(|row| row.module == "detail_question")
        );
    }
}
