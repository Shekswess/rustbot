//! Priority ordering and repeat avoidance. Specialized upstream routing is not yet ported.
use crate::bot::{Candidate, Memory, Reading, compose};

#[path = "arbiter_fit.rs"]
mod fit;
#[path = "arbiter_possession.rs"]
mod possession;

pub(crate) fn choose<'a>(
    reading: &Reading<'_>,
    memory: &Memory,
    proposals: &'a [Candidate],
    score_carried_subject: bool,
    answer_card: Option<(&str, &str)>,
) -> Option<&'a Candidate> {
    let picked = choose_by_priority(
        reading,
        memory,
        proposals,
        score_carried_subject,
        answer_card,
    )?;
    let health_answer = answer_card.is_some_and(|(domain, _)| domain == "health")
        || reading.topic.is_some_and(|(topic, _)| topic == "health");
    if picked.module == "answers"
        && let Some(urgent) = proposals
            .iter()
            .find(|proposal| proposal.module == "urgent")
        && super::urgent::overrides_answer(reading, health_answer)
    {
        return Some(urgent);
    }
    if picked.module == "answers"
        && let Some(carer) = proposals
            .iter()
            .find(|proposal| proposal.module == "carers_turn")
    {
        return Some(carer);
    }
    if picked.module == "answers"
        && super::talkto::yields_small_talk(answer_card)
        && let Some(opener) = proposals
            .iter()
            .find(|candidate| candidate.module == "talkto")
    {
        return Some(opener);
    }
    if picked.module == "chat"
        && answer_card == Some(("family", "teenager-talk"))
        && super::chat::row_yields_to_teenager_card(reading, &picked.reply, memory)
        && let Some(card) = proposals
            .iter()
            .find(|proposal| proposal.module == "answers")
    {
        return Some(card);
    }
    if picked.priority < 9000.0
        && super::super::memory::history::implies_earlier_turn(
            &picked.reply,
            memory,
            Some(reading.raw),
        )
    {
        return proposals
            .iter()
            .filter(|p| p.priority >= 0.0)
            .find(|p| {
                !super::super::memory::history::implies_earlier_turn(
                    &p.reply,
                    memory,
                    Some(reading.raw),
                )
            })
            .or(Some(picked));
    }
    Some(picked)
}

fn choose_by_priority<'a>(
    reading: &Reading<'_>,
    memory: &Memory,
    proposals: &'a [Candidate],
    score_carried_subject: bool,
    answer_card: Option<(&str, &str)>,
) -> Option<&'a Candidate> {
    let first = proposals.first()?;
    if super::arbiter_echo::should_step_back(reading, first, &proposals[1..], memory) {
        return choose_by_priority(
            reading,
            memory,
            &proposals[1..],
            score_carried_subject,
            answer_card,
        );
    }
    // Urgent response bands and direct live-service answers keep the turn.
    if first.priority >= 9000.0 || matches!(first.module, "live_times" | "the_month_they_have") {
        return Some(first);
    }
    if let Some(fallback) = proposals.iter().find(|p| p.module == "greeting") {
        if compose::after_emergency::needs_evacuation(reading, &fallback.reply) {
            return Some(fallback);
        }
        if compose::after_emergency::claims(reading, memory, &fallback.reply) {
            let topic = fit::Fit::new(reading);
            if fit::points_at_thread(reading.raw)
                || !proposals
                    .iter()
                    .any(|p| p.priority >= 0.0 && topic.owns(p.module))
            {
                return Some(fallback);
            }
        }
    }
    if first.priority < 0.0
        && let Some(trouble) = proposals.iter().find(|p| p.module == "the_trouble_named")
        && super::the_trouble_named::takes_as_it_comes(reading, memory)
        && memory
            .turns
            .last()
            .is_none_or(|t| compose::said_again(&t.bot) != compose::said_again(&trouble.reply))
    {
        return Some(trouble);
    }
    if first.priority < 0.0
        && !reading.raw.contains('?')
        && compose::trouble::answers_a_strain_question(&reading.line.clean, memory)
        && let Some(greeting) = proposals.iter().find(|p| p.module == "greeting")
        && memory
            .turns
            .last()
            .is_none_or(|t| compose::said_again(&t.bot) != compose::said_again(&greeting.reply))
    {
        return Some(greeting);
    }
    // an_age_ahead declares both TAKES_AS_IT_COMES and CLAIMS_INSTEAD. They
    // call the same reader as respond, so the proposal already proves the claim.
    if first.module == "an_age_ahead" {
        return Some(first);
    }
    if let Some(claim) = proposals.iter().find(|proposal| {
        let relevant = match proposal.module {
            "since_it_began"
            | "keeping"
            | "a_return_brings_back_the_advice"
            | "who_to_see"
            | "dreams" => true,
            "aboutachild" => super::aboutachild::claims(reading, memory),
            "familyask" => super::familyask::claims(reading, memory),
            "group_ages" => super::group_ages::claims(reading, memory),
            "household" => super::household::claims(reading, memory),
            "already" => crate::bot::memory::already::claims(reading, memory),
            "a_figure_beside_the_state" => true,
            "saving_followup" | "a_shock_at_a_bill" => true,
            _ => false,
        };
        relevant
            && memory.turns.last().is_none_or(|last| {
                compose::said_again(&last.bot) != compose::said_again(&proposal.reply)
            })
    }) {
        return Some(claim);
    }
    if (first.priority < 2000.0 || first.module == "memory_store")
        && let Some(confirmed) = proposals
            .iter()
            .find(|proposal| matches!(proposal.module, "figure_asked_again" | "taking_asked_again"))
    {
        return Some(confirmed);
    }
    // A stored count answers a numerical household question even when the
    // household hook can also name the people who live there.
    if first.module == "household"
        && let Some(count) = crate::bot::memory::household::count_reply(reading, memory)
        && let Some(store) = proposals
            .iter()
            .find(|proposal| proposal.module == "memory_store" && proposal.reply == count)
    {
        return Some(store);
    }
    // A recall that identifies the actual owner answers the question. Quoting
    // the same relative's words would lose the distinction between the two people.
    if first.module == "said_fact_back"
        && let Some(store) = proposals.iter().find(|p| p.module == "memory_store")
        && ((store.reply.contains("the allergy to ") && store.reply.ends_with("'s, not yours."))
            || store
                .reply
                .contains("haven't told me about an allergy of your "))
    {
        return Some(store);
    }
    if let Some(fact) = proposals.iter().find(|c| c.module == "said_fact_back")
        && (super::said_fact_back::takes_as_it_comes(reading, memory)
            || (first.priority <= fact.priority
                && reading
                    .topics
                    .iter()
                    .any(|(t, _)| super::said_fact_back::ANSWERS.contains(t))))
        && memory
            .turns
            .last()
            .is_none_or(|t| compose::said_again(&t.bot) != compose::said_again(&fact.reply))
    {
        return Some(fact);
    }
    // The quotation answers what the person did with the object.
    if let Some(recall) = proposals.iter().find(|p| p.module == "thing_i_did_with")
        && memory
            .turns
            .last()
            .is_none_or(|last| compose::said_again(&last.bot) != compose::said_again(&recall.reply))
    {
        return Some(recall);
    }
    if let Some(card) = possession::candidate(reading, memory, proposals, first) {
        return Some(card);
    }
    let generic_quote = first.module == "memory_store"
        && [
            "You told me: “",
            "That was you, not me: “",
            "From your own words: “",
        ]
        .iter()
        .any(|prefix| first.reply.starts_with(prefix));
    if (first.priority < 7000.0 || generic_quote)
        && let Some(attribute) = proposals
            .iter()
            .find(|p| p.module == "attribute_said_again")
        && crate::bot::memory::attribute_said_again::takes_as_it_comes(reading, memory)
        && memory
            .turns
            .last()
            .is_none_or(|t| compose::said_again(&t.bot) != compose::said_again(&attribute.reply))
    {
        return Some(attribute);
    }
    if let Some(chat) = proposals.iter().find(|candidate| {
        candidate.module == "chat"
            && crate::bot::memory::frequency::is_frequency_reply(
                reading.raw,
                &candidate.reply,
                memory,
            )
    }) {
        return proposals
            .iter()
            .find(|candidate| {
                candidate.module == "answers"
                    && crate::bot::memory::frequency::answers_owns_frequency(
                        &candidate.reply,
                        memory,
                    )
            })
            .or(Some(chat));
    }
    if let Some(handed) = super::arbiter_hand_back::candidate(reading, memory, proposals) {
        return Some(handed);
    }
    if matches!(first.module, "answers" | "chat" | "greeting")
        && let Some(limit) = proposals
            .iter()
            .find(|proposal| proposal.module == "ruled_out")
    {
        return Some(limit);
    }
    if matches!(first.module, "chat" | "greeting")
        && let Some(nudge) = proposals.iter().find(|proposal| proposal.module == "nudge")
    {
        return Some(nudge);
    }
    let Some(previous) = memory.turns.last() else {
        return Some(first);
    };
    if previous.user.eq_ignore_ascii_case(reading.raw) {
        return Some(first);
    }
    // An explicit transcript position still has the same answer after all
    // of its wording variants have been used.
    if first.module == "memory_store"
        && first.priority >= 7000.0
        && crate::bot::memory::earlier::chronology_ask(&reading.clean)
    {
        return Some(first);
    }
    let lodging_question = matches!(
        answer_card,
        Some(("travel", "where-to-stay" | "where-to-stay-there"))
    ) && reading.features.asks.contains(&"where");
    if first.module == "chat"
        && reading.kind == crate::bot::understanding::kinds::Kind::Question
        && (lodging_question
            || !reading.line.words.iter().any(|word| {
                matches!(
                    word.as_str(),
                    "i" | "me"
                        | "my"
                        | "mine"
                        | "we"
                        | "us"
                        | "our"
                        | "ours"
                        | "again"
                        | "else"
                        | "remember"
                        | "remind"
                        | "said"
                        | "told"
                )
            }))
        && let Some(card) = proposals
            .iter()
            .find(|proposal| proposal.module == "answers" && proposal.reply == first.reply)
    {
        // The reference lets recall keep its chat proposal. For a direct
        // question with identical wording, prefer the better declared fit.
        let topic = fit::Fit::new(reading);
        // A named lodging card supplies the geographic topic for a question such as
        // "where should we stay?" even when the words only contain its ask.
        let carried = fit::Fit::carried(
            memory
                .ack_topics
                .stored_topic(&memory.turns)
                .chain(lodging_question.then_some("geography and travel")),
        );
        if topic.score(card.module, &carried) > topic.score(first.module, &carried)
            && memory.turns.first_reply(&card.reply).is_none()
        {
            return Some(card);
        }
    }
    // A carried subject can give both chat and the indexed card a fresh reply.
    // Try the declared topic while keeping a direct reply to the wording.
    if first.module == "chat"
        && (score_carried_subject
            || crate::bot::memory::next_step_answer(reading.raw, memory).is_some())
    {
        let topic = fit::Fit::new(reading);
        let carried = fit::Fit::carried(memory.ack_topics.stored_topic(&memory.turns));
        let mut best = first;
        let mut best_score = topic.score(first.module, &carried);
        for candidate in proposals.iter().skip(1).take_while(|p| p.priority >= 0.0) {
            let score = topic.score(candidate.module, &carried);
            let carried_tariff = score_carried_subject
                && candidate.module == "answers"
                && answer_card == Some(("home", "cheapest-hours"));
            if score > best_score
                && memory.turns.first_reply(&candidate.reply).is_none()
                && !(carried_tariff && topic.preserves_subject(&best.reply, &candidate.reply))
            {
                best = candidate;
                best_score = score;
            }
        }
        return Some(best);
    }
    let novelty_floor = if first.priority >= 0.0 {
        0.0
    } else {
        f64::NEG_INFINITY
    };
    proposals
        .iter()
        .take_while(|p| p.priority >= novelty_floor)
        .find(|p| memory.turns.first_reply(&p.reply).is_none())
        .or(Some(first))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn normalized_novelty_preserves_priority_bands_and_urgent_replies() {
        let mut memory = Memory::default();
        for reply in ["She\tcalled them.", "A later reply."] {
            memory.turns.record(Exchange {
                user: "An earlier input".into(),
                bot: reply.into(),
                module: "answers",
            });
        }
        let reading = Reading::new("A different input.");
        let mut proposals = [
            Candidate {
                module: "answers",
                priority: 1.0,
                reply: "He called her.".into(),
            },
            Candidate {
                module: "greeting",
                priority: 0.0,
                reply: "Entirely new wording.".into(),
            },
        ];
        assert_eq!(
            choose(&reading, &memory, &proposals, false, None),
            Some(&proposals[1])
        );
        proposals[1].priority = -1.0;
        assert_eq!(
            choose(&reading, &memory, &proposals, false, None),
            Some(&proposals[0])
        );
        proposals[1].priority = 0.0;
        proposals[0].module = "safety";
        proposals[0].priority = 9000.0;
        assert_eq!(
            choose(&reading, &memory, &proposals, false, None),
            Some(&proposals[0])
        );
    }

    #[test]
    fn gas_evacuation_beats_an_unrelated_answer_while_the_caller_is_inside() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("I smell gas in my kitchen.");
        let reading = Reading::new("I called the gas emergency line, I'm still inside.");
        let proposals = [
            Candidate {
                module: "answers",
                priority: 0.0,
                reply: "Tell me which key it is on and I will name it.".into(),
            },
            Candidate {
                module: "greeting",
                priority: -1000.0,
                reply: "Get everyone out first and call from outside; tell the operator exactly what is burning or leaking and where.".into(),
            },
        ];
        let chosen = choose(&reading, &bot.memory, &proposals, false, None).unwrap();
        assert_eq!(chosen, &proposals[1]);
    }

    #[test]
    fn stored_headcount_beats_listing_the_household() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("there are three people here");
        bot.turn("me and my kids");
        let turn = bot.turn("how many of us are there");
        assert_eq!(turn.module, Some("memory_store"));
        assert_eq!(turn.reply, "There are three of you.");
        assert!(
            turn.candidates
                .iter()
                .any(|candidate| candidate.module == "household")
        );
    }

    #[test]
    fn the_capital_card_owns_an_identical_chat_proposal() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("what is the capital of chile?");
        let answer = bot.turn("what is the capital of new south wales?");
        assert_eq!(answer.module, Some("answers"));
        assert_eq!(
            answer.reply,
            "Sydney, not Canberra - Canberra is the capital of the country."
        );
        assert!(
            answer
                .candidates
                .iter()
                .any(|proposal| { proposal.module == "chat" && proposal.reply == answer.reply })
        );
    }

    #[test]
    fn duration_question_selects_the_card_about_the_cake() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("im making a birthday cake");
        let turn = bot.turn("how long will it take");
        assert_eq!(turn.module, Some("answers"));
        assert_eq!(
            turn.reply,
            "A 20cm sponge bakes at 180C (160C fan) for 25 to 30 minutes; a loaf tin needs 45 to 55. It's done when a skewer in the middle comes out clean, and it's worth checking 5 minutes early the first time. About an hour and a half start to finish, counting mixing and cooling."
        );
    }

    #[test]
    fn next_step_competes_with_the_card_that_described_the_job() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("My boiler keeps losing pressure");
        bot.turn("I topped it up yesterday");
        let next = bot.turn("and then what");
        assert_eq!(next.module, Some("answers"), "{:?}", next.candidates);
        assert_eq!(
            next.reply,
            "Same answer, and it covers this: Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day."
        );
    }
}
