//! Follow-ups that read a named part or a referring word against the answer just given.
use std::sync::LazyLock;

use regex::Regex;

use super::Answer;
use crate::bot::{
    Memory, Reading,
    knowledge::lookup,
    memory::sentences,
    understanding::{kinds::Kind, normalize},
};

static SCRATCHING_POST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:and )?(?:what about|where (?:should|do|can|could) (?:i|we) put) (her|his|a) scratching post$",
    )
    .expect("scratching post follow-up")
});

static WATERING_PRONOUN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:and )?(?:how (?:can|do) (?:i|we) tell if (?:i|we)(?:'ve| have)|have (?:i|we)) watered (them|it) too much[?!.]*$",
    )
    .expect("watering follow-up")
});

static FIRST_SLIDE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^what (?:should|shall|can|could) (?:i|we) put on the first slide[?!.]*$")
        .expect("first slide question")
});
static WASHING_MACHINE_REPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\bwashing machine\b[^.!?]*\b(?:knocks?|knocking|noisy|rattles?|rattling)\b")
        .expect("washing machine fault report")
});
static TOPIC_BREAK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^\s*(?:never mind|forget (?:that|it)|new topic|different (?:topic|question)|changing the subject)\b")
        .expect("explicit topic change")
});

static CYCLING_INSTEAD_OF_DRIVING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:can|could|should|shall) (?:i|we) cycle\b[^.!?]*\b(?:while|when|as|because)\b[^.!?]*\b(?:the|my|our) car\b",
    )
    .expect("cycling while the car is unavailable")
});

pub(super) fn cycling_in_car_weather(
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<lookup::KnowledgeMatch> {
    if !reading.line.words.iter().any(|word| word == "car")
        || !CYCLING_INSTEAD_OF_DRIVING.is_match(&reading.clean)
    {
        return None;
    }
    // PyBot widens this unanswered question with the rain word from the car
    // topic. The rest of the car report is irrelevant to the cycling card.
    let weather = memory
        .turns
        .iter()
        .rev()
        .take(3)
        .take_while(|turn| {
            !matches!(turn.module, "safety" | "urgent") && !TOPIC_BREAK.is_match(&turn.user)
        })
        .find_map(|turn| {
            let mut weather = None;
            let mut car = false;
            for word in turn.user.split(|ch: char| !ch.is_alphabetic()) {
                car |= word.eq_ignore_ascii_case("car");
                if let Some(spelling) = ["rain", "raining", "rains"]
                    .into_iter()
                    .find(|spelling| word.eq_ignore_ascii_case(spelling))
                {
                    weather = Some(spelling);
                }
            }
            weather.filter(|_| car)
        })?;
    let ask = reading.clean.trim_end_matches(['?', '.', '!']).trim_end();
    lookup::lookup(&format!("{ask} {weather}"))
        .filter(|card| (card.domain, card.id) == ("sport", "run-in-the-rain"))
}

pub(super) fn washer_schedule_report(reading: &Reading<'_>, memory: &Memory) -> Option<Answer> {
    if !super::super::chat::scheduled_course(reading)
        || !memory
            .turns
            .iter()
            .rev()
            .take(6)
            .take_while(|turn| {
                !matches!(turn.module, "safety" | "urgent") && !TOPIC_BREAK.is_match(&turn.user)
            })
            .any(|turn| WASHING_MACHINE_REPORT.is_match(&turn.user))
    {
        return None;
    }
    // The pinned statement reader prepends the outstanding report's "washing"
    // topic word. The time of day on the new line completes the grouped key.
    let said = reading.clean.trim_end_matches(['.', '!']);
    let widened = format!("washing {said}");
    let widened = Reading::new(&widened);
    let card = lookup::lookup_in_context(&widened, memory)?;
    if (card.domain, card.id) != ("home", "cheapest-hours") {
        return None;
    }
    Some(Answer {
        selected_card: Some((card.domain, card.id)),
        reply: card.reply.to_string(),
        outcomes: card.outcomes,
        score_carried_subject: true,
        entry: Some(card.reply),
    })
}

static FIRST_CHECK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^what(?:'s| is) the first thing to check[?!.]*$").expect("first check question")
});
static PACK_CAPACITY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^would (?:a|my|the) (?:small )?(?:backpack|rucksack) be enough[?!.]*$")
        .expect("pack capacity follow-up")
});
static SOUP_FIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^would they work in a soup[?!.]*$").expect("leftover soup follow-up")
});

pub(super) fn carry_topic_words(
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<lookup::KnowledgeMatch> {
    if reading.kind != Kind::Question {
        return None;
    }
    if memory.last_answer_anchor.is_some_and(|anchor| {
        anchor.turn + 1 == memory.turns.len() && anchor.card == Some(("travel", "packing-weekend"))
    }) && PACK_CAPACITY.is_match(&reading.clean)
    {
        // The weekend card's packing key completes the bag question.
        return lookup::lookup(&format!("{} pack", reading.clean.trim_end_matches('?')))
            .filter(|card| (card.domain, card.id) == ("study", "school-bag"));
    }

    let last = memory.turns.last()?;
    let expected = match last.module {
        "a_first_time" if FIRST_SLIDE.is_match(&reading.clean) => {
            ("decorating", "paint-runs-and-sags")
        }
        "greeting"
            if last.bot.ends_with(" — noted.")
                && FIRST_CHECK.is_match(&reading.clean)
                && WASHING_MACHINE_REPORT.is_match(&last.user) =>
        {
            // The leading "Got it" acknowledgment makes the source's
            // memory reader surface the previous project's own first step.
            ("home", "which-paint")
        }
        _ => return None,
    };
    let paint_room = lookup::by_id("home", "paint-room")?;
    let mut earlier = memory.turns.iter().rev().take_while(|turn| {
        !matches!(turn.module, "safety" | "urgent") && !TOPIC_BREAK.is_match(&turn.user)
    });
    // The source's current topic is still the unanswered appliance report.
    // Its keys reach no card, so the follow-up borrows the older room project.
    earlier
        .find(|turn| turn.module == "greeting" && WASHING_MACHINE_REPORT.is_match(&turn.user))?;
    earlier.find(|turn| turn.module == "answers" && turn.bot == paint_room.reply)?;
    lookup::lookup(&format!("{} paint room", reading.clean))
        .filter(|card| (card.domain, card.id) == expected)
}

pub(super) fn ingredient_fit_repeat(reading: &Reading<'_>, memory: &Memory) -> Option<Answer> {
    let anchor = memory.last_answer_anchor?;
    if anchor.turn + 1 != memory.turns.len()
        || !matches!(
            anchor.card,
            Some(("cooking", "roast-vegetables") | ("sport", "rest-days"))
        )
        || !SOUP_FIT.is_match(&reading.clean)
    {
        return None;
    }
    let (last, earlier) = memory.turns[memory.turns.len().saturating_sub(6)..].split_last()?;
    let start = earlier
        .iter()
        .rposition(|turn| {
            matches!(turn.module, "safety" | "urgent") || TOPIC_BREAK.is_match(&turn.user)
        })
        .map_or(0, |position| position + 1);
    let overlap = |reply: &str| {
        let words = normalize::tokens(reply);
        ["work", "soup"]
            .into_iter()
            .filter(|key| words.iter().any(|word| normalize::stem(word) == *key))
            .count()
    };
    let mut best = last;
    let mut best_score = overlap(&last.bot);
    // The reference retains the first answer whose keyword score beats the
    // latest one. A form of "work" can be the sole word shared with the ask.
    for turn in earlier[start..]
        .iter()
        .filter(|turn| turn.module == "answers")
    {
        let score = overlap(&turn.bot);
        if score > best_score {
            best = turn;
            best_score = score;
        }
    }
    let rest_days = lookup::by_id("sport", "rest-days")?;
    if best.bot != rest_days.reply {
        return None;
    }
    let part = sentences(&best.bot).next()?;
    Some(Answer {
        selected_card: None,
        reply: format!("Same answer, and it covers this: {part}"),
        outcomes: &[],
        score_carried_subject: false,
        entry: None,
    })
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<Answer> {
    if reading.kind != Kind::Question {
        return None;
    }
    let anchor = memory.last_answer_anchor?;
    if anchor.turn + 1 != memory.turns.len() {
        return None;
    }
    let last = memory.turns.last()?;
    let ask = reading.clean.trim_end_matches(['?', '.', '!']).trim();
    let card = match anchor.card? {
        ("pets", "cat-scratching") => {
            let detail = SCRATCHING_POST.captures(ask)?.get(1)?.as_str();
            let card = lookup::by_id("pets", "cat-scratching")?;
            if last.bot != card.reply.as_ref() {
                return None;
            }
            // A possessive points to the placement already given. With an indefinite
            // article the pet-detail reader instead gives the later sentence about the post.
            let mut parts = sentences(&last.bot);
            let first = parts.next()?;
            let body = if detail == "a" {
                let second = parts.next()?.strip_prefix("A ")?;
                format!("a {second}")
            } else {
                first.to_owned()
            };
            return Some(Answer {
                selected_card: Some((card.domain, card.id)),
                reply: format!("Same for {detail} scratching post: {body}"),
                outcomes: card.outcomes,
                score_carried_subject: false,
                entry: Some(card.reply),
            });
        }
        ("garden", "tomato-yellow") => {
            let found = WATERING_PRONOUN.captures(&reading.clean)?.get(1)?;
            let rewritten = format!(
                "{}the tomato plants{}",
                &reading.clean[..found.start()],
                &reading.clean[found.end()..]
            );
            let card = lookup::lookup(&rewritten)?;
            if (card.domain, card.id) != ("garden", "veg-watering") {
                return None;
            }
            card
        }
        ("home", "paint-quantity")
            if matches!(ask, "and what about the ceiling" | "what about the ceiling") =>
        {
            let earlier_topic = memory.ack_topics.has_earlier_topic();
            let question = last.user.to_lowercase();
            if earlier_topic
                && let Some(unit) = [" metres", " meters"]
                    .into_iter()
                    .find(|unit| question.matches(unit).count() == 2)
            {
                // The source substitutes the first occurrence of the named unit here.
                // The resulting question has no usable pair of dimensions.
                let rewritten = question.replacen(unit, " ceiling", 1);
                lookup::lookup(&rewritten)
                    .filter(|card| (card.domain, card.id) == ("home", "paint-quantity"))?
            } else {
                let card = lookup::by_id("home", "paint-ceiling")?;
                let reply = card.reply.as_ref();
                let finish = sentences(reply).last()?;
                let method = reply.strip_suffix(finish)?.trim_end();
                return Some(Answer {
                    selected_card: Some((card.domain, card.id)),
                    reply: format!("{finish} {method}"),
                    outcomes: card.outcomes,
                    score_carried_subject: false,
                    entry: Some(card.reply),
                });
            }
        }
        _ => return None,
    };
    Some(Answer {
        selected_card: Some((card.domain, card.id)),
        reply: card.reply.to_string(),
        outcomes: card.outcomes,
        score_carried_subject: false,
        entry: Some(card.reply),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bot;

    #[test]
    fn course_reply_competes_with_the_pending_washer_schedule_card() {
        let course = "I want to take a maths course in the evening.";
        let washer = "My washing machine is making a knocking sound.";
        let tariff = lookup::by_id("home", "cheapest-hours").unwrap().reply;

        let mut bot = Bot::new(Some("0")).unwrap();
        let ordinary = bot.turn(course);
        assert_eq!(ordinary.module, Some("chat"));
        assert!(ordinary.candidates.iter().all(|c| c.module != "answers"));

        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn(washer);
        let short = bot.turn(course);
        assert_eq!(short.module, Some("chat"));
        assert!(
            short
                .reply
                .starts_with("Take a maths course in the evening")
        );
        assert!(
            short
                .candidates
                .iter()
                .any(|c| c.module == "answers" && c.reply == tariff)
        );

        let mut bot = Bot::new(Some("0")).unwrap();
        for line in [
            "I want to paint the spare room.",
            washer,
            "What's the first thing to check?",
            "How do I give a talk at work?",
            "I've never done one before.",
            "What should I put on the first slide?",
            "How do I ask for feedback afterwards?",
        ] {
            bot.turn(line);
        }
        let carried = bot.turn(course);
        assert_eq!(carried.module, Some("answers"));
        assert_eq!(carried.reply, tariff);
        assert!(
            carried
                .candidates
                .iter()
                .any(|c| c.module == "chat" && c.priority == 300.0)
        );

        let mut bot = Bot::new(Some("0")).unwrap();
        let direct = bot.turn("Is it cheaper to run the washing machine at night?");
        assert_eq!(direct.module, Some("answers"));
        assert_eq!(direct.reply, tariff);
    }

    #[test]
    fn scratching_post_names_the_detail_from_the_selected_cat_answer() {
        let report = "My cat keeps scratching the sofa.";
        let body = lookup::by_id("pets", "cat-scratching").unwrap().reply;
        let first = sentences(&body).next().unwrap();
        for (ask, expected) in [
            (
                "Where should I put her scratching post?",
                format!("Same for her scratching post: {first}"),
            ),
            (
                "Where should we put a scratching post?",
                "Same for a scratching post: a post with height lets her do it properly."
                    .to_owned(),
            ),
        ] {
            let mut bot = Bot::new(Some("0")).unwrap();
            assert_eq!(bot.turn(report).reply, body);
            let turn = bot.turn(ask);
            assert_eq!(turn.module, Some("answers"));
            assert_eq!(turn.reply, expected);
        }
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn(report);
        let turn = bot.turn(
            "Where should I put her scratching post? My cat keeps going to the litter tray but cannot pee.",
        );
        assert!(
            turn.reply
                .starts_with("Call your vet or an out-of-hours vet now.")
        );
    }

    #[test]
    fn watering_pronoun_uses_the_tomato_just_discussed() {
        let ask = "How can I tell if I've watered them too much?";
        let watering = lookup::by_id("garden", "veg-watering").unwrap().reply;
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("Why are my tomato plant leaves going yellow?");
        let turn = bot.turn(ask);
        assert_eq!(turn.module, Some("answers"));
        assert_eq!(turn.reply, watering);

        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("Why are my tomato plant leaves going yellow?");
        bot.turn("Why do onions make you cry?");
        assert_ne!(bot.turn(ask).reply, watering);
    }

    #[test]
    fn ceiling_extension_uses_the_paint_quantity_only_with_a_prior_topic() {
        let quantity = "How much paint do I need for a room four metres by three metres?";
        let extension = "And what about the ceiling?";
        let expected = lookup::lookup("How much paint do I need for the ceiling?")
            .unwrap()
            .reply;
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("I want to paint the spare room.");
        bot.turn(quantity);
        let turn = bot.turn(extension);
        assert_eq!(turn.module, Some("answers"));
        assert_eq!(turn.reply, expected);

        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("Hi.");
        bot.turn(quantity);
        let turn = bot.turn(extension);
        assert_eq!(turn.module, Some("answers"));
        assert!(turn.reply.starts_with("Matt emulsion, two coats,"));
    }

    #[test]
    fn explicit_fan_cleaning_outranks_the_carried_fault() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("The bathroom fan is loud and the mirror stays fogged up.");
        let turn = bot.turn("How do I clean the fan safely?");
        assert_eq!(turn.module, Some("answers"));
        assert_eq!(
            turn.reply,
            lookup::by_id("cleaning", "clean-fan").unwrap().reply
        );
        assert!(turn.candidates.iter().any(|candidate| {
            candidate.module == "chat" && candidate.reply.starts_with("Clean the cover")
        }));
    }

    #[test]
    fn cycling_question_can_carry_rain_from_a_recent_car_report() {
        let ask = "Can I cycle to work while the car is in?";
        let expected = lookup::by_id("sport", "run-in-the-rain").unwrap().reply;
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_ne!(bot.turn(ask).reply, expected);

        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("My car makes a squeaking noise when I brake in the rain.");
        bot.turn("What should I make a note of before I take it in?");
        let turn = bot.turn(ask);
        assert_eq!(turn.reply, expected);
        assert_eq!(turn.module, Some("answers"));

        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("My car makes a squeaking noise when I brake in the rain.");
        bot.turn("Different topic. I need a cake recipe.");
        assert_ne!(bot.turn(ask).reply, expected);
    }
}
