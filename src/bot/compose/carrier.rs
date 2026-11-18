//! Direct the user to the part of their own object or paperwork that answers an ask.
#[path = "carrier_data.rs"]
mod data;
use crate::bot::{
    Exchange, Memory, Reading,
    compose::{choose_recent, pick},
    knowledge::lookup,
    memory::measurement,
};
use data::*;
use fancy_regex::Regex;

fn hit(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}
struct Thing<'a> {
    label: String,
    kind: &'static str,
    determiner: Option<&'a str>,
}
fn named(text: &str) -> Option<Thing<'_>> {
    let mut best = None;
    for (pattern, kind) in kinds() {
        for found in pattern.find_iter(text).flatten() {
            if best
                .as_ref()
                .is_none_or(|(word, _, _): &(&str, &str, usize)| found.as_str().len() > word.len())
            {
                best = Some((found.as_str(), kind, found.start()));
            }
        }
    }
    let (mut word, mut kind, mut start) = best?;
    if let Ok(Some(paper)) = PAPERWORK.find(text) {
        word = paper.as_str();
        kind = "paperwork";
        start = paper.start();
    }
    let determiner = DETERMINER
        .captures(&text[..start])
        .ok()
        .flatten()
        .and_then(|m| m.get(1))
        .map(|m| m.as_str());
    let article = match determiner {
        Some("my" | "our") => "your",
        Some(d) => d,
        None => "the",
    };
    Some(Thing {
        label: format!("{article} {word}"),
        kind,
        determiner,
    })
}
fn in_thread(memory: &Memory) -> Option<Thing<'static>> {
    let recent = memory
        .turns
        .iter()
        .rev()
        .filter(|turn| !turn.user.trim().is_empty())
        .take(2);
    let latest = recent.clone().next()?;
    for (back, turn) in recent.enumerate() {
        let said = turn
            .user
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if let Some(thing) = named(&said) {
            let word = thing
                .label
                .split_once(' ')
                .filter(|(d, _)| {
                    matches!(
                        *d,
                        "my" | "our" | "your" | "his" | "her" | "their" | "the" | "a" | "an"
                    )
                })
                .map_or(thing.label.as_str(), |(_, noun)| noun);
            if back > 0 && word == "kettle" && !kettle_note(latest) {
                return None;
            }
            return Some(Thing {
                label: format!("the {word}"),
                kind: thing.kind,
                determiner: Some("the"),
            });
        }
    }
    None
}
// The machine lead and route pools are from PyBot's bot/compose/not_knowing.py.
const DESCALING: (&str, &str) = ("cleaning", "descale-kettle");
const MACHINE_LEADS: &[&str] = &[
    "The model number and the exact code on the display are what settle these - the number is on a sticker on the machine, or in the settings screen under About.",
    "For a machine, the maker's own sheet is written for your exact model: the sticker on the back or inside the door has the number, and the code on the display is the other half.",
];
const MACHINE_ROUTES: &[&str] = &[
    "Put the model number into the maker's own support page and it gives the manual and what that code means.",
    "The maker's support page with that model has the manual, the fault codes and the reset sequence.",
];
static DESCALING_DURATION: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(concat!(
        r"^(?:(?:and|so|but|okay|ok|then|also)\s+)*how long",
        r"(?:\s+(?:(?:does|will|would)\s+(?:it|this|that)\s+take|",
        r"(?:do|should)\s+(?:i|we)\s+(?:leave\s+(?:it|this|that)|",
        r"wait(?:\s+before\s+rinsing)?|let\s+(?:it|this|that)\s+(?:sit|soak))))?[ ?.!]*$"
    ))
    .expect("bare descaling duration")
});

fn kettle_note(turn: &Exchange) -> bool {
    matches!(turn.module, "chat" | "greeting")
        && ["thanks", "thank you", "okay", "ok"].iter().any(|phrase| {
            turn.user
                .trim()
                .trim_end_matches(['.', '!', '?'])
                .eq_ignore_ascii_case(phrase)
        })
}

/// The authored machine route, or the duration sentence of the selected descaling answer.
pub(crate) fn descaling(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if DESCALING_DURATION.is_match(&reading.line.clean) {
        // PyBot's repeat.the_figure_asked retrieves a recent answer's sentence.
        // Check the selected card's identity and the intervening turn before quoting it.
        let anchor = memory.last_answer_anchor?;
        if anchor.card != Some(DESCALING) {
            return None;
        }
        let (source, after) = memory.turns.get(anchor.turn..)?.split_first()?;
        if source.module != "answers"
            || after.len() > 1
            || after.first().is_some_and(|turn| !kettle_note(turn))
        {
            return None;
        }
        return measurement::duration_sentence(&source.bot).map(str::to_owned);
    }
    if !reading.line.words.iter().any(|word| word == "kettle")
        || lookup::lookup_in_context(reading, memory)
            .is_none_or(|card| (card.domain, card.id) != DESCALING)
    {
        return None;
    }
    let clean = &reading.line.clean;
    Some(format!(
        "{} {}",
        choose_recent(MACHINE_LEADS, memory, clean),
        choose_recent(MACHINE_ROUTES, memory, clean)
    ))
}

pub(crate) fn reply(raw: &str, memory: &Memory) -> Option<String> {
    let said = raw
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if said.is_empty()
        || [
            &*NOT_HERE,
            &NOT_ABOUT_THE_CONVERSATION,
            &NOT_A_LOOKUP_BUT_A_REQUEST,
            &A_JUDGEMENT_ASK,
            &A_PLAN_ASK,
            &A_COOKING_LINE,
            &A_READING_JUST_GIVEN,
            &A_WHY_ASK,
            &A_METHOD_OR_ADVICE_ASK,
            &A_DECISION_ABOUT_GOING_ON,
            &A_NOW_OR_NEVER_ASK,
            &NOT_ABOUT_THE_VISIT,
        ]
        .iter()
        .any(|p| hit(p, &said))
    {
        return None;
    }
    let current = named(&said);
    let from_thread = current.is_none();
    let thing = current.or_else(|| in_thread(memory))?;
    let ask = asks()
        .into_iter()
        .find(|(p, _)| hit(p, &said))
        .map(|(_, a)| a)
        .or_else(|| (from_thread && !hit(&NOT_ON_THE_PLATE, &said)).then_some("first_step"))?;
    if !(from_thread
        || THEIR_COPY_ASKS.contains(&ask)
        || thing.determiner.is_some_and(|d| THEIR_OWN.contains(&d))
        || hit(&A_RECEIPT, &said)
        || hit(&A_JOB_ON_IT, &said))
    {
        return None;
    }
    let middle = carriers(ask, thing.kind)?;
    let questions = question_kind(ask, thing.kind).or_else(|| question(ask))?;
    let opener = pick(
        if ask == "get_out" {
            OPENER_TERMS
        } else {
            OPENER
        },
        memory,
        &said,
    )
    .replace("{S}", &thing.label);
    Some(format!(
        "{opener}. {}. {}",
        pick(middle, memory, &said),
        pick(questions, memory, &said)
    ))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/carrier.rs"]
mod reference;
#[test]
fn answers_reference_queries_in_context() {
    let mut memory = Memory::default();
    for &(prior, line, idx) in reference::CASES {
        memory.turns.clear();
        if !prior.is_empty() {
            memory.turns.push(crate::bot::Exchange {
                user: prior.to_owned(),
                bot: "Understood.".to_owned(),
                module: "greeting",
            });
        }
        assert_eq!(
            reply(line, &memory).as_deref(),
            idx.map(|i| reference::REPLIES[i]),
            "{prior} / {line}"
        );
    }
}

#[cfg(test)]
mod kettle_tests {
    use crate::bot::Bot;

    const OPENER: &str = "my kettle is full of limescale, how do i get rid of it?";
    const FOLLOW_UP: &str = "how long does that take";
    const MEASURE: &str = "Half fill the kettle with equal parts water and white vinegar, or a sachet of citric acid, boil it and leave it half an hour, then tip it out and boil fresh water twice before you use it again.";
    const DEVICE_ROUTE: &str = "The model number and the exact code on the display are what settle these - the number is on a sticker on the machine, or in the settings screen under About. Put the model number into the maker's own support page and it gives the manual and what that code means.";

    #[test]
    fn kettle_uses_the_pinned_route_and_selected_duration_sentence() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("hi, i'm Sam");
        let open = bot.turn(OPENER);
        assert_eq!(open.module, Some("answers"), "{open:?}");
        assert!(
            open.candidates
                .iter()
                .any(|item| { item.module == "greeting" && item.reply == DEVICE_ROUTE }),
            "{open:?}"
        );

        let follow = bot.turn(FOLLOW_UP);
        assert_eq!(follow.module, Some("answers"), "{follow:?}");
        assert_eq!(follow.reply, "Half an hour.");
        assert!(
            follow
                .candidates
                .iter()
                .any(|item| { item.module == "greeting" && item.reply == MEASURE }),
            "{follow:?}"
        );
    }

    #[test]
    fn kettle_duration_does_not_cross_to_a_new_subject() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn(OPENER);
        let changed = bot.turn("how long does it take to roast a chicken?");
        assert!(
            changed
                .candidates
                .iter()
                .all(|item| !item.reply.contains("kettle")),
            "{changed:?}"
        );

        bot.reset().unwrap();
        bot.turn(OPENER);
        bot.turn("what is the capital of Italy");
        let changed = bot.turn(FOLLOW_UP);
        assert!(
            changed
                .candidates
                .iter()
                .all(|item| !item.reply.contains("kettle")),
            "{changed:?}"
        );
    }
}
