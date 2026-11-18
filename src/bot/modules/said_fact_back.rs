//! Repair an unsupported denial before considering a quote of the person's earlier line.
use crate::bot::{
    Memory, Reading, compose,
    memory::{self, attribute_said_again, earlier, lineback},
};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PUNT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
    r"(?i)^(?:i'?m not sure|i am not sure|i don'?t know|i do not know|i can'?t|i cannot|can'?t call|",
    r"that'?s beyond|that is beyond|that'?s all|that is all|there is no more|beyond what i know|not mine to|not a call|not one i can call|",
    r"a quick search|searching for|i'?d rather not|you haven't told me|that'?s a good question|good question|no idea|not something i|",
    r"no (?:figure|number|price|answer|idea|verdict|view|opinion|guess) from me|",
    r"say (?:what (?:you(?:'?re| are)? (?:asking|after|on about|worried about|unsure)|you mean|you want to know|has you (?:asking|wondering))|which|that again|more|me more|a bit more)\b|",
    r"tell me (?:what|which|more) (?:you|has|is|part)\b|no (?:first[- ]hand )?(?:knowledge|idea|clue) (?:of|on|about|here|from me)\b|",
    r"nothing (?:in|about) (?:that|this|it|what you (?:said|told me|described))\b[^.!?]{0,60}?\b(?:say|says|said|tell|tells|told|show|shows|showed|suggest|suggests|indicate|indicates|go on)\b|",
    r"nothing here (?:to go on|that (?:says|tells))\b|hard to (?:put a (?:scale|number) on|say how|judge|tell how|gauge)\b|",
    r"i'?m not sure (?:what|how|which|whether|where) to|i'?m not sure without|",
    r"(?:that'?s|that is|it'?s|it is) (?:your|a) call(?![a-z])|not (?:a )?call (?:i|we) (?:can|could|would)|",
    r"(?:that'?s|that is) (?:one|a question|something) for (?:your|a|the)\b|(?:that'?s|that is) (?:everything|all) i (?:have|know)\b|",
    r"(?:that'?s|that is|it'?s|it is) not (?:mine|for me) to (?:answer|say|tell|call|judge)\b|i would only repeat myself|",
    r"nothing more i can (?:say|add|tell)\b|(?:that'?s|that is) not for me to|",
    r"whether (?:it|that|they|this|[0-9]+|[a-z]+) (?:is|are|was|will be|would be) (?:worth|a lot|too much|too long|too far|any good|safe|normal|fine)\b|",
    r"hold (?:it|that|this|the price|[0-9]+)\b[^.]{0,30}\bagainst (?:one|another|a|other)\b|only you can (?:weigh|judge|say|decide|call)\b|",
    r"(?:that'?s|that is|it'?s|it is) yours to (?:weigh|say|decide|call|judge)\b|that part(?:'?s| is) (?:yours|up to you)\b|",
    r"(?:i'?ll|i will) leave (?:that|this|it) to (?:your|a|the|somebody|someone)\b|",
    r"\s*(?:i wouldn'?t call|i would not call|nothing in that|hard to|no way to|not something i can)\b[^.!?]{0,80}?\b(?:answer|step|thing|doing) above\b)"
)).unwrap()
});

pub(crate) const ANSWERS: &[&str] = &["pets", "family-and-parenting"];

pub(crate) fn observe(_reading: &Reading<'_>, _memory: &mut Memory) {}

pub(crate) fn takes_as_it_comes(reading: &Reading<'_>, memory: &Memory) -> bool {
    let ask = &reading.clean;
    if !earlier::recall_cue(ask) {
        return false;
    }
    let Some(reply) = memory::respond(reading, memory) else {
        return false;
    };
    earlier::repair(&reply, ask, memory).is_some_and(|shown| shown != reply)
        || earlier::another_line_was_asked_for(ask, &reply, memory)
}

fn nothing_to_give(reply: &str) -> bool {
    compose::routes::hedge(reply) || PUNT.is_match(reply).unwrap_or(false)
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let ask = &reading.clean;
    if let Some(reply) = memory::respond(reading, memory) {
        if let Some(shown) = earlier::repair(&reply, ask, memory) {
            return Some(shown);
        }
        let mismatch = earlier::another_line_was_asked_for(ask, &reply, memory)
            && !attribute_said_again::respond(reading, memory)
                .is_some_and(|value| earlier::gives_named_value(ask, &value, memory));
        if !mismatch && !nothing_to_give(&reply) {
            return None;
        }
    }
    if lineback::respond(reading, memory).is_some() {
        return None;
    }
    let reply = earlier::value_from_history(ask, memory)?;
    if !earlier::their_own_words(ask)
        && super::answers::offer(reading, memory)
            .is_some_and(|offer| !offer.reply.trim().is_empty())
    {
        return None;
    }
    Some(reply)
}

#[cfg(test)]
mod tests {
    fn candidate<'a>(turn: &'a crate::Turn, module: &str) -> Option<&'a str> {
        turn.candidates
            .iter()
            .find(|candidate| candidate.module == module)
            .map(|candidate| candidate.reply.as_str())
    }

    #[test]
    fn a_named_sister_without_a_filed_town_repairs_the_store_denial() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        for line in [
            "My name is Ana and my sister is Maya.",
            "Maya lives in Bristol and I live in Leeds.",
            "My favourite fruit is a mango, but I can't eat peanuts.",
            "What is my favourite fruit?",
        ] {
            bot.turn(line);
        }
        let first = bot.turn("where does she live?");
        assert_eq!(
            candidate(&first, "memory_store"),
            Some("You haven't said where your sister lives.")
        );
        assert_eq!(
            candidate(&first, "said_back"),
            Some("You said: \"Maya lives in Bristol and I live in Leeds.\"")
        );
        assert_eq!(first.module, Some("said_fact_back"));
        assert_eq!(
            first.reply,
            "You told me: \"Maya lives in Bristol and I live in Leeds.\"."
        );
        bot.turn("Actually, she moved to Bath.");
        let again = bot.turn("and where does she live now?");
        assert_eq!(
            again.reply,
            "That one you did say: \"Maya lives in Bristol and I live in Leeds.\"."
        );
        assert_eq!(again.module, Some("said_fact_back"));
    }

    #[test]
    fn a_filed_town_needs_no_denial_repair() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My sister Maya lives in Bristol.");
        let turn = bot.turn("Where does Maya live?");
        assert_eq!(candidate(&turn, "said_fact_back"), None);
        assert_eq!(turn.module, Some("memory_store"));
        assert_eq!(turn.reply, "Your sister lives in Bristol.");
    }
}
