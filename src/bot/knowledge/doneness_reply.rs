//! The doneness part of a dish plan, before the generic card is rendered.
use super::{Phrase, contains, named, question, same, tokens};
use crate::bot::{Memory, Reading, memory::history};
use fancy_regex::Regex as Fancy;
use std::sync::LazyLock;

struct ReadyNote {
    names: &'static [Phrase],
    reply: &'static str,
}

include!("doneness_reply_data.rs");

fn matches(pattern: &Fancy, line: &str) -> bool {
    pattern.is_match(line).unwrap_or(false)
}

fn left_out(line: &str) -> bool {
    LEFT_OUT_PHRASES.iter().any(|phrase| line.contains(phrase)) || matches(&LEAVING_OUT, line)
}

fn with_food(line: &str, food: &str) -> bool {
    line.match_indices(food).any(|(start, _)| {
        let end = start + food.len();
        let word = |ch: char| ch.is_alphanumeric() || ch == '_';
        !line[..start].chars().next_back().is_some_and(word)
            && !line[end..].chars().next().is_some_and(word)
    })
}

fn asks_if_still_ok(line: &str, earlier: &str) -> bool {
    if !STILL_OK_ASK.iter().any(|phrase| line.contains(phrase)) || !left_out(earlier) {
        return false;
    }
    let new_food: Vec<_> = LEFT_OUT_FOODS
        .iter()
        .filter(|food| with_food(line, food))
        .collect();
    LEFT_OUT_FOODS
        .iter()
        .any(|food| with_food(earlier, food) && (new_food.is_empty() || new_food.contains(&food)))
}

pub(super) fn names_food(words: &[String]) -> bool {
    words
        .iter()
        .any(|word| FOOD_WORDS.iter().any(|food| same(word, food)))
}

fn ready_note(words: &[String]) -> Option<&'static str> {
    let mut best = None;
    for note in READY {
        for alias in note.names {
            if best.is_none_or(|(len, _)| alias.text.len() > len) && contains(words, alias.words) {
                best = Some((alias.text.len(), note.reply));
            }
        }
    }
    best.map(|(_, reply)| reply)
}

fn answer<'a>(
    text: &str,
    raw: &str,
    recent: impl Iterator<Item = &'a str> + Clone,
    needs_named_plan: bool,
) -> Option<&'static str> {
    let raw = raw.to_lowercase();
    if left_out(&raw)
        || recent
            .clone()
            .next()
            .is_some_and(|earlier| asks_if_still_ok(&raw, &earlier.to_lowercase()))
    {
        return None;
    }
    let words = tokens(text);
    if question(text, &words) != Some("done") {
        return None;
    }
    let own = named(&words, text);
    if own.is_none()
        && (needs_named_plan
            || matches(&PERSON_STATE, text)
            || matches(&PERSON_REST, text)
            || matches(&BODY_TEMPERATURE, text))
    {
        return None;
    }
    let plan = own.or_else(|| {
        if names_food(&words) {
            return None;
        }
        recent.clone().find_map(|earlier| {
            let earlier = earlier.to_lowercase();
            named(&tokens(&earlier), &earlier)
        })
    });
    if let Some(plan) = plan
        && !matches(&NAMES_A_SAUCE, text)
        && !plan.done.is_empty()
    {
        return Some(plan.done);
    }
    if own.is_some() {
        return None;
    }
    ready_note(&words).or_else(|| {
        recent
            .take(4)
            .find_map(|earlier| ready_note(&tokens(earlier)))
    })
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<&'static str> {
    let needs_named_plan = reading.lexical_words().is_none();
    if needs_named_plan
        && (reading.kind != crate::bot::understanding::kinds::Kind::Question
            || memory.pantry.current_spoilage()
            || STILL_OK_ASK
                .iter()
                .any(|phrase| reading.clean.contains(phrase))
            || !reading.line.words.iter().any(|word| {
                matches!(
                    word.as_str(),
                    "tell"
                        | "know"
                        | "check"
                        | "test"
                        | "see"
                        | "ready"
                        | "done"
                        | "cooked"
                        | "through"
                )
            })
            || !reading
                .line
                .words
                .iter()
                .any(|word| matches!(word.as_str(), "bread" | "dough" | "loaf")))
    {
        return None;
    }
    if history::in_a_safety_thread(memory) {
        return None;
    }
    let recent = memory
        .turns
        .iter()
        .rev()
        .take(4)
        .map(|turn| turn.user.as_str())
        .chain(memory.doneness_topics.current_text());
    // A named test reaches the same dish planner as an observer-filled
    // question. Only the latter may borrow its dish from the conversation.
    answer(
        &reading.line.norm_text,
        reading.raw,
        recent,
        needs_named_plan,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_the_plan_or_the_named_egg_method() {
        let earlier = ["im roasting a chicken"].into_iter();
        let ask = "chicken how can i tell when its done";
        assert_eq!(
            answer(ask, ask, earlier, false),
            Some(
                "74C in the thickest part of the thigh on a digital thermometer, pushed in without touching bone, and the juices run clear with the leg moving freely in the socket; 75C for a moment is the same thing."
            )
        );
        let ask = "are they cooked yet";
        assert_eq!(
            answer(ask, ask, ["im boiling eggs"].into_iter(), false),
            Some(
                "It depends what you want: 4 minutes for a runny yolk, 6 for a soft one and 8 to 9 for a firm one, counted from the moment they go into water that is already boiling, then into cold water so they stop cooking."
            )
        );
        assert_eq!(
            answer(ask, ask, ["im poaching eggs"].into_iter(), false),
            Some(
                "A poached egg is done when the white is opaque and set and the yolk still gives when you press it lightly: 3 minutes in barely simmering water for a runny one, 4 for a set white. Lift it out with a slotted spoon and drain it on kitchen paper."
            )
        );
    }

    #[test]
    fn leaves_food_safety_and_other_subjects_to_their_own_reader() {
        let earlier = ["im roasting a chicken"].into_iter();
        for (lexical, raw) in [
            (
                "chicken how can i tell when its done",
                "i left it out overnight, is it done",
            ),
            (
                "how can i tell if my boiler is ready",
                "how can i tell if my boiler is ready",
            ),
            (
                "how can i tell if the chicken sauce is cooked",
                "how can i tell if the chicken sauce is cooked",
            ),
        ] {
            assert_eq!(answer(lexical, raw, earlier.clone(), false), None, "{raw}");
        }
        let query = "is it still safe to eat when its cooked";
        assert_eq!(
            answer(
                query,
                query,
                ["i left cooked chicken on the side for four hours"].into_iter(),
                false,
            ),
            None
        );
    }

    #[test]
    fn a_named_bread_test_does_not_need_the_observer_to_supply_its_subject() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("Why did my bread turn out dense?");
        let ready = bot.turn("How can I tell when the dough has risen enough?");
        assert_eq!(ready.module, Some("answers"));
        assert_eq!(
            ready.reply,
            "It sounds hollow when you tap the base, and a thermometer reads 95C inside."
        );
        let proof = bot.turn("How long should I leave the bread dough to rise?");
        assert!(!proof.reply.contains("tap the base"), "{proof:?}");
    }
}
