//! Named creative requests and seeded answer pools from `bot/knowledge/fun.py`.

use std::borrow::Cow;

use crate::bot::{Memory, compose};

use super::{fun_data as data, guards::Context};

fn has_phrase(text: &str, phrase: &str) -> bool {
    text.match_indices(phrase).any(|(start, found)| {
        (start == 0 || !text.as_bytes()[start - 1].is_ascii_lowercase())
            && text
                .as_bytes()
                .get(start + found.len())
                .is_none_or(|next| !next.is_ascii_lowercase())
    })
}

pub(super) fn not_counting(text: &str) -> bool {
    !data::COUNTED.iter().any(|word| has_phrase(text, word))
}

pub(super) fn only_ask(text: &str) -> bool {
    text.split(|character: char| !(character.is_ascii_lowercase() || character == '\''))
        .filter(|word| !word.is_empty())
        .all(|word| data::ASK_WORDS.contains(&word))
}

pub(super) fn something_to_hear(context: &Context<'_>) -> Option<&'static str> {
    use crate::bot::compose::loss_reader::{is_dying_now, reads_as_loss};

    if context
        .earlier
        .iter()
        .rev()
        .take(6)
        .any(|turn| is_dying_now(&turn.user))
        || context
            .earlier
            .iter()
            .rev()
            .take(2)
            .any(|turn| reads_as_loss(&turn.user))
    {
        return None;
    }
    let text = &context.reading.line.clean;
    let pool = if text.contains("entertain") || text.contains("amuse") {
        data::RIDDLES
    } else {
        data::FUN_FACTS
    };
    Some(pick(context, pool, text))
}

pub(super) fn fact(context: &Context<'_>) -> Option<&'static str> {
    let text = format!(" {} ", context.reading.line.clean);
    if !data::FACT_ASKS.iter().any(|ask| has_phrase(&text, ask))
        || data::FACT_NOT.iter().any(|word| has_phrase(&text, word))
    {
        return None;
    }
    let (_, facts) = data::FACTS
        .iter()
        .find(|(names, _)| names.iter().any(|name| has_phrase(&text, name)))?;
    Some(pick(context, facts, &text))
}

pub(super) fn pick(
    context: &Context<'_>,
    pool: &'static [&'static str],
    seed: &str,
) -> &'static str {
    if let Some(memory) = context.memory {
        compose::pick(pool, memory, seed)
    } else {
        compose::pick(pool, &Memory::default(), seed)
    }
}

pub(super) fn toast(context: &Context<'_>) -> &'static str {
    let text = context.reading.line.words.join(" ");
    let pool = data::TOASTS
        .iter()
        .find_map(|(occasion, pool)| {
            occasion
                .iter()
                .any(|word| text.contains(word))
                .then_some(*pool)
        })
        .unwrap_or(data::PLAIN_TOASTS);
    pick(context, pool, &text)
}

pub(super) fn group_game(context: &Context<'_>) -> &'static str {
    let text = format!(" {} ", context.reading.line.clean);
    for (marks, reply) in [
        (
            &["two truths", "2 truths", "truths and a lie", "truth or lie"][..],
            data::GROUP_GAMES[0],
        ),
        (&["would you rather"][..], data::GROUP_GAMES[1]),
        (
            &["twenty questions", "20 questions"][..],
            data::GROUP_GAMES[2],
        ),
        (&["alphabet"][..], data::GROUP_GAMES[3]),
    ] {
        if marks.iter().any(|mark| text.contains(mark)) {
            return reply;
        }
    }
    let travelling = [
        " car", " journey", " travel", " flight", " drive", " road", " trip", " train", " plane",
        " airport",
    ]
    .iter()
    .any(|word| text.contains(word));
    let games = if travelling {
        data::GROUP_GAMES
    } else {
        &data::GROUP_GAMES[..3]
    };
    pick(context, games, &text)
}

pub(super) fn name_ideas(context: &Context<'_>) -> Cow<'static, str> {
    let text = format!(" {} ", context.reading.line.clean).replace("'s", "");
    let thing = data::THINGS
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, thing)| text.contains(&format!(" {thing} ")))
        .max_by_key(|(index, thing)| (thing.chars().count(), std::cmp::Reverse(*index)))
        .map(|(_, thing)| thing);
    let names = match thing {
        Some("boat" | "boats" | "yacht" | "kayak" | "canoe") => data::BOAT_NAMES,
        Some("band" | "bands" | "quiz team" | "team") => data::BAND_NAMES,
        Some("podcast") => data::PODCAST_NAMES,
        Some("pub" | "cafe" | "café" | "shop" | "business" | "company" | "startup") => {
            data::SHOP_NAMES
        }
        _ => data::TECH_NAMES,
    };
    Cow::Owned(format!(
        "Short, sayable and easy to spell beats clever: {}, {} or {}. Say it out loud in a sentence - 'this is {}' - before you commit, and check nobody selling the same thing already has it.",
        names[0],
        names[1],
        names[2],
        names[0].to_lowercase()
    ))
}

#[cfg(test)]
mod tests {
    use crate::bot::{Exchange, Reading};

    use super::*;

    #[test]
    fn creative_callbacks_distinguish_a_requested_fact_or_quiz_from_a_report() {
        let request = Reading::new("tell me a fun fact about dogs");
        let context = Context::new(&request, &[]);
        assert!(fact(&context).is_some());
        assert!(!not_counting("did you know octopuses have three hearts"));
        assert!(
            !Reading::new("she gave me a question to answer")
                .features
                .wants_a_quiz
        );
        let quiz = super::super::lookup("quiz me").expect("explicit request for a quiz");
        assert_eq!(quiz.id, "quiz-question");
    }

    #[test]
    fn generic_entertainment_yields_after_recent_loss() {
        let reading = Reading::new("tell me something interesting");
        let earlier = [Exchange {
            user: "my dad died last week".to_owned(),
            bot: String::new(),
            module: "test",
        }];
        assert!(only_ask(reading.folded()));
        assert!(something_to_hear(&Context::new(&reading, &[])).is_some());
        assert!(something_to_hear(&Context::new(&reading, &earlier)).is_none());
    }
}
