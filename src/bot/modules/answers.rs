use crate::bot::{
    Memory, Reading,
    knowledge::{
        airport_arrival, cooking, dishes, geography, lookup, maths, meal_shortlist,
        outcomes::{self, Outcomes},
        travel,
    },
};

use std::borrow::Cow;

#[path = "answers_followup.rs"]
mod followup;

pub(in crate::bot) fn observe(reading: &Reading<'_>, memory: &mut Memory) {
    memory.birth_facts.observe(reading);
    memory.pantry.observe(
        reading,
        memory.turns.len(),
        memory.turns.last().map(|turn| turn.bot.as_str()),
    );
}

pub(crate) struct Answer {
    pub selected_card: Option<(&'static str, &'static str)>,
    pub reply: String,
    pub outcomes: Outcomes,
    pub score_carried_subject: bool,
    pub entry: Option<Cow<'static, str>>,
}

pub(in crate::bot) fn offer_for_turn(
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<(Answer, Option<meal_shortlist::MealPlan>)> {
    if let Some(reply) = memory.pantry.recipe(reading, memory) {
        return Some((
            Answer {
                selected_card: None,
                reply: reply.to_owned(),
                outcomes: &[],
                score_carried_subject: true,
                entry: Some(Cow::Borrowed(reply)),
            },
            None,
        ));
    }
    if let Some(proposal) = meal_shortlist::another_idea(reading, memory)
        .or_else(|| meal_shortlist::with_ingredient(reading, memory))
    {
        return Some(meal_offer(proposal, None));
    }
    let answer = offer(reading, memory);
    if let Some(card) = answer.as_ref().and_then(|answer| answer.selected_card)
        && card.0 == "meals"
        && meal_shortlist::catalogue::CARDS.contains(&card.1)
        && let Some(proposal) = meal_shortlist::catalogue::suggest(card.1, reading, memory)
    {
        return Some(meal_offer(proposal, Some(card)));
    }
    if let Some((proposal, card)) = meal_shortlist::clock::propose(reading, memory) {
        let selected = answer.as_ref().and_then(|answer| answer.selected_card);
        if meal_shortlist::clock::can_replace(card, selected) {
            return Some(meal_offer(proposal, Some(card)));
        }
    }
    if let Some(proposal) = meal_shortlist::plant_proposal(reading, memory) {
        let selected = answer.as_ref().and_then(|answer| answer.selected_card);
        let intended = if memory.turns.is_empty() {
            ("cooking", "meal-ideas")
        } else {
            ("meals", "meal-diet-again")
        };
        let fixed_menu = memory.turns.is_empty()
            && matches!(
                selected,
                Some((
                    "cooking",
                    "vegan-main" | "vegetarian-main" | "vegetarian-guest"
                ))
            );
        if selected.is_none_or(|card| card == intended) || fixed_menu {
            return Some(meal_offer(proposal, Some(intended)));
        }
    }
    if memory.turns.is_empty()
        && meal_shortlist::is_open_dinner(reading)
        && answer.as_ref().is_none_or(|answer| {
            answer
                .selected_card
                .is_some_and(|(domain, id)| lookup::is_meal_shortlist(domain, id))
        })
    {
        let proposal = meal_shortlist::propose(
            reading,
            memory.turns.len(),
            lookup::probes_meal_shortlist(reading),
        );
        let selected_card = answer.and_then(|answer| answer.selected_card);
        return Some(meal_offer(proposal, selected_card));
    }
    answer
        .or_else(|| followup::washer_schedule_report(reading, memory))
        .map(|answer| (answer, None))
}

fn meal_offer(
    proposal: meal_shortlist::Proposal,
    selected_card: Option<(&'static str, &'static str)>,
) -> (Answer, Option<meal_shortlist::MealPlan>) {
    (
        Answer {
            selected_card,
            entry: Some(Cow::Owned(proposal.reply.clone())),
            reply: proposal.reply,
            outcomes: &[],
            score_carried_subject: false,
        },
        Some(proposal.plan),
    )
}

pub(crate) fn offer(reading: &Reading<'_>, memory: &Memory) -> Option<Answer> {
    if let Some(referral) =
        crate::bot::knowledge::medicine_followup::missed_pill_timing(reading, memory)
    {
        return Some(Answer {
            selected_card: None,
            reply: referral.to_string(),
            score_carried_subject: false,
            outcomes: &[],
            entry: Some(referral),
        });
    }
    let alone = ordinary_offer(reading, memory);
    crate::bot::memory::measurement_subject::respond(
        reading,
        memory,
        alone.as_ref().map(|answer| answer.reply.as_str()),
    )
    .or(alone)
}

fn travel_followup(text: &str, memory: &Memory, required: Option<&str>) -> Option<Answer> {
    let widened = Reading::new(text);
    let card = lookup::lookup_in_context(&widened, memory)?;
    if card.domain != "travel"
        || required.is_some_and(|id| id != card.id)
        || crate::bot::memory::bare_subject::answer_recently_said(&card.reply, memory)
    {
        return None;
    }
    Some(Answer {
        selected_card: Some((card.domain, card.id)),
        reply: lookup::apply_lead(&widened, &card, &card.reply)
            .unwrap_or_else(|| card.reply.to_string()),
        outcomes: card.outcomes,
        score_carried_subject: false,
        entry: Some(card.reply),
    })
}

pub(crate) fn ordinary_offer(reading: &Reading<'_>, memory: &Memory) -> Option<Answer> {
    if let Some(reply) = lookup::cat_urinary_emergency(reading) {
        return Some(Answer {
            selected_card: Some(("pets", "cat-urine")),
            reply: reply.to_owned(),
            outcomes: &[],
            score_carried_subject: false,
            entry: Some(Cow::Borrowed(reply)),
        });
    }
    if !memory.active_outcomes.is_empty() {
        let same_subject = memory.turns.last().is_some_and(|turn| {
            crate::bot::knowledge::outcomes::only_followup_words(&reading.line.clean, &turn.user)
        });
        if let Some(reply) = outcomes::owed(memory.active_outcomes, reading.folded(), same_subject)
        {
            return Some(Answer {
                selected_card: None,
                reply: reply.to_owned(),
                score_carried_subject: false,
                outcomes: memory.active_outcomes,
                entry: None,
            });
        }
    }
    if let Some(answer) = specific_answer(reading, memory) {
        return Some(Answer {
            selected_card: None,
            reply: answer,
            score_carried_subject: false,
            outcomes: &[],
            entry: None,
        });
    }
    if memory
        .turns
        .last()
        .is_some_and(|turn| turn.module == "safety")
        && reading.kind != crate::bot::understanding::kinds::Kind::Question
        && !reading.line.signals.question_mark
    {
        return crate::bot::memory::bare_subject::named_request(reading, memory);
    }
    if let Some(reply) = crate::bot::memory::a_year_of_birth::reply(reading, memory) {
        return Some(Answer {
            selected_card: None,
            reply,
            score_carried_subject: false,
            outcomes: &[],
            entry: None,
        });
    }
    if let Some(part) = lookup::trip_question(reading.raw, memory)
        && let Some(reply) = travel_followup(&part, memory, None)
    {
        return Some(reply);
    }
    if let Some(answer) = followup::respond(reading, memory) {
        return Some(answer);
    }
    let Some(card) = lookup::lookup_in_context(reading, memory)
        .or_else(|| followup::cycling_in_car_weather(reading, memory))
        .or_else(|| followup::carry_topic_words(reading, memory))
    else {
        if let Some(answer) = followup::ingredient_fit_repeat(reading, memory) {
            return Some(answer);
        }
        // A bare question after carrying out a step can still reach the original
        // indexed card through the conversation. Only offer it if the repeat
        // reader supplies wording and it was actually a registered answer.
        if reading.kind == crate::bot::understanding::kinds::Kind::Question
            && let Some(body) = crate::bot::memory::next_step_answer(reading.raw, memory)
            && let Some(card) = memory.entries.on_screen(body, "answers")
            && let Some(reply) = crate::bot::memory::frame_reply(card, reading.raw, memory)
        {
            return Some(Answer {
                selected_card: None,
                reply,
                score_carried_subject: false,
                outcomes: &[],
                entry: None,
            });
        }
        if let Some(reply) = crate::bot::memory::repeated_step_answer(reading.raw, memory) {
            return Some(Answer {
                selected_card: None,
                reply,
                score_carried_subject: false,
                outcomes: &[],
                entry: None,
            });
        }
        if let Some(carried) = crate::bot::memory::bare_subject::respond(reading, memory) {
            return Some(carried);
        }
        if let Some(carried) = crate::bot::memory::bare_subject::named_request(reading, memory) {
            return Some(carried);
        }
        if let Some(sight) = lookup::trip_sight_question(reading.raw, memory)
            && let Some(reply) = travel_followup(&sight, memory, Some("city-next"))
        {
            return Some(reply);
        }
        if let Some(reply) = crate::bot::memory::frequency::behind_frequency(reading, memory) {
            return Some(Answer {
                selected_card: None,
                reply,
                score_carried_subject: false,
                outcomes: &[],
                entry: None,
            });
        }
        return crate::bot::knowledge::kitchen::without(&reading.line)
            .or_else(|| lookup::single_ingredient_meal(reading).map(str::to_owned))
            .map(|reply| Answer {
                selected_card: None,
                reply,
                score_carried_subject: false,
                outcomes: &[],
                entry: None,
            });
    };
    if card.domain == "home"
        && !card.fault
        && let Some(widened) =
            crate::bot::memory::bare_subject::unfinished_plan_after_residence_quote(reading, memory)
        && let Some(reply) = travel_followup(&widened, memory, Some("city-guide"))
    {
        return Some(reply);
    }
    if card.fault
        && let Some(kitchen) = crate::bot::knowledge::kitchen::without(&reading.line)
    {
        return Some(Answer {
            selected_card: None,
            reply: kitchen,
            score_carried_subject: false,
            outcomes: &[],
            entry: None,
        });
    }
    let seen = memory
        .turns
        .iter()
        .rev()
        .take(3)
        .any(|t| t.bot == card.reply.as_ref());
    let outcome = seen
        .then(|| outcomes::owed(card.outcomes, reading.folded(), true))
        .flatten();
    let raw = outcome.unwrap_or(card.reply.as_ref());
    let shaped = lookup::move_answer_sentence(reading, raw);
    let framed = if outcome.is_none() {
        crate::bot::memory::frame_answer_card(shaped.as_deref().unwrap_or(raw), reading.raw, memory)
    } else {
        None
    };
    let reply = framed.or(shaped).unwrap_or_else(|| raw.to_owned());
    let reply = lookup::apply_lead(reading, &card, &reply).unwrap_or(reply);
    Some(Answer {
        selected_card: Some((card.domain, card.id)),
        reply,
        // Score the fan the user named against the carried bathroom fault.
        score_carried_subject: (card.domain, card.id) == ("cleaning", "clean-fan"),
        outcomes: card.outcomes,
        entry: Some(card.reply),
    })
}

fn detail_in_last_card(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let anchor = memory.last_answer_anchor?;
    if anchor.turn + 1 != memory.turns.len() {
        return None;
    }
    let last = &memory.turns.last()?.bot;
    let ask = reading.clean.trim_end_matches(['?', '.', '!']).trim();
    match anchor.card? {
        ("cooking", "egg-boil") => {
            let condition = ask
                .strip_prefix("and if i want ")
                .or_else(|| ask.strip_prefix("what if i want "))
                .or_else(|| ask.strip_prefix("if i want "))?;
            if !matches!(condition, "the yolk soft" | "a soft yolk") {
                return None;
            }
            let (_, cases) = last.split_once("boiling water: ")?;
            let (soft, _) = cases.split_once(", 9 to 10 for hard boiled")?;
            Some(format!("{soft}."))
        }
        ("technology", "phone-not-charging") => {
            let action = ask
                .strip_prefix("should i ")
                .or_else(|| ask.strip_prefix("can i "))
                .or_else(|| ask.strip_prefix("could i "))?;
            if !matches!(action, "try a different cable" | "try another cable") {
                return None;
            }
            let (step, _) = last.split_once(", then ")?;
            Some(format!("{step}."))
        }
        _ => None,
    }
}

fn specific_answer(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(detail) = detail_in_last_card(reading, memory) {
        return Some(detail);
    }
    if let Some(reply) = dishes::observed_doneness(reading, memory) {
        return Some(reply.to_owned());
    }
    if let Some(dish) = dishes::announced_part(reading) {
        return Some(dish);
    }
    if let Some(answer) = lookup::dog_feed_followup(reading, memory) {
        return Some(answer.to_owned());
    }
    if let Some(dish) = dishes::definition(&reading.clean) {
        return Some(dish);
    }
    if reading.kind == crate::bot::understanding::kinds::Kind::Question
        && let Some(answer) = geography::answer(&reading.clean)
    {
        return Some(answer);
    }
    if let Some(capital) = travel::capital(&reading.clean) {
        return Some(capital.to_owned());
    }
    if !crate::bot::memory::history::in_a_safety_thread(memory)
        && airport_arrival::arrival_calculation(reading).is_none()
        && let Some(answer) =
            maths::solve(&reading.clean).or_else(|| maths::carried_measure(&reading.clean, memory))
    {
        return Some(answer);
    }
    if let Some(answer) = cooking::roast_chicken(&reading.clean) {
        return Some(
            crate::bot::memory::frame_answer_card(&answer, reading.raw, memory).unwrap_or(answer),
        );
    }

    if let Some(answer) = crate::bot::memory::shorter::respond(reading, memory) {
        return Some(answer);
    }
    if let Some(answer) = lookup::pantry_combination(reading, memory) {
        return Some(answer.to_owned());
    }
    if let Some(answer) = crate::bot::memory::measurement::follow_up(reading, memory) {
        return Some(answer);
    }
    None
}
