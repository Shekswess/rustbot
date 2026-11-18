//! Household water use, painting tools and appliance care.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::{city_guides, clothing_care, guards::Context, home_upkeep};

#[path = "home_maintenance_data.rs"]
mod data;

use data::*;

static MACHINE_NAMES_WHOLE: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    MACHINE_NAMES
        .iter()
        .map(|(name, _)| whole_name(name, false))
        .collect()
});
static MACHINE_NAMES_PLURAL: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    MACHINE_NAMES
        .iter()
        .map(|(name, _)| whole_name(name, true))
        .collect()
});
static MACHINE_SERVICE_NAMES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    MACHINE_SERVICE
        .iter()
        .map(|(name, _, _)| whole_name(name, true))
        .collect()
});
static STAY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:how long|how many days|how many nights|how much time|how many hours|how many weeks)\b")
        .expect("length of stay question")
});

fn whole_name(name: &str, plural: bool) -> Regex {
    let ending = if plural { "s?" } else { "" };
    Regex::new(&format!(r"\b{}{ending}\b", regex::escape(name))).expect("appliance name")
}

fn ascii_words(line: &str) -> impl Iterator<Item = &str> {
    line.split(|ch: char| !ch.is_ascii_lowercase())
        .filter(|word| !word.is_empty())
}

fn takes_subject(line: &str) -> Option<&'static str> {
    let mut best_len = 0;
    let mut subject = None;
    for (&(alias, machine), pattern) in MACHINE_NAMES.iter().zip(MACHINE_NAMES_WHOLE.iter()) {
        if alias.len() <= best_len {
            continue;
        }
        if pattern.find_iter(line).any(|span| {
            let words =
                || ascii_words(&line[..span.start()]).chain(ascii_words(&line[span.end()..]));
            words().any(|word| matches!(word, "take" | "takes"))
                && words().all(|word| TAKE_WORDS.binary_search(&word).is_ok())
        }) {
            best_len = alias.len();
            subject = Some(machine);
        }
    }
    subject
}

fn longest_machine(line: &str) -> Option<&'static str> {
    let mut longest = 0;
    let mut subject = None;
    for (&(alias, machine), pattern) in MACHINE_NAMES.iter().zip(MACHINE_NAMES_PLURAL.iter()) {
        if alias.len() > longest && pattern.is_match(line) {
            longest = alias.len();
            subject = Some(machine);
        }
    }
    subject
}

pub(super) fn machine_takes(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = first_text(&reading.line.clean, &reading.line.plain).to_lowercase();
    if let Some((_, text)) = takes_subject(&said)
        .and_then(|machine| MACHINE_TAKES.iter().find(|(name, _)| *name == machine))
    {
        return Some(Cow::Borrowed(text));
    }
    (clothing_care::job_left_undone(&said) && longest_machine(&said) == Some("tumble dryer"))
        .then(|| Cow::Borrowed(clothing_care::dryer_heat(reading)))
}

pub(super) fn service_worth(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let asks = &reading.features.asks;
    let said = &reading.line.clean;
    if asks.contains(&"how_much") || contains_any(said, &["how do i", "how can i"]) {
        return None;
    }
    if !asks
        .iter()
        .any(|ask| matches!(*ask, "yesno" | "how_often" | "recommend"))
        && !reading
            .line
            .words
            .iter()
            .any(|word| matches!(word.as_str(), "worth" | "worthwhile" | "point"))
    {
        return None;
    }
    let often = asks.contains(&"how_often") || said.contains("how often");
    let said = said.split_whitespace().collect::<Vec<_>>().join(" ");
    MACHINE_SERVICE_NAMES.iter().zip(MACHINE_SERVICE).find_map(
        |(pattern, &(_, interval, verdict))| {
            pattern
                .is_match(&said)
                .then_some(Cow::Borrowed(if often { interval } else { verdict }))
        },
    )
}

pub(super) fn water_use(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = first_text(&reading.line.text, &reading.line.clean);
    let mut longest = 0;
    let mut reply = WATER_FALLBACK;
    for &(aliases, text) in WATER_USE {
        for &alias in aliases {
            if alias.len() > longest && home_upkeep::any_plural(line, &[alias]) {
                longest = alias.len();
                reply = text;
            }
        }
    }
    Some(Cow::Borrowed(reply))
}

pub(super) fn filter(
    names: &[&str],
    reply: &'static str,
    context: &Context<'_>,
) -> Option<&'static str> {
    let line = first_text(&context.reading.line.text, &context.reading.line.clean).to_lowercase();
    let names_machine = |line: &str| names.iter().any(|name| filter_name_at(line, name));
    let matches = names_machine(&line)
        || context.memory.is_some_and(|memory| {
            let recent = memory.doneness_topics.recent_texts(3).collect::<Vec<_>>();
            names_machine(&recent.join(" ").to_lowercase())
        });
    matches.then_some(reply)
}

fn filter_name_at(line: &str, name: &str) -> bool {
    let boundary = |at: Option<usize>| {
        at.and_then(|at| line.as_bytes().get(at))
            .is_none_or(|ch| !ch.is_ascii_lowercase() && !ch.is_ascii_digit())
    };
    line.match_indices(name)
        .any(|(start, _)| boundary(start.checked_sub(1)) && boundary(Some(start + name.len())))
}

pub(super) fn not_length_of_stay(reading: &Reading<'_>) -> bool {
    let line = [
        &reading.line.clean,
        &reading.line.norm_text,
        &reading.line.plain,
    ]
    .map(String::as_str)
    .join(" ")
    .to_lowercase();
    !STAY.is_match(&line)
        || !CITY_DAYS
            .iter()
            .any(|name| city_guides::named_place(&line, name))
}

pub(super) fn paint_brush_care(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = format!(
        " {} ",
        first_text(&reading.line.clean, &reading.line.text).to_lowercase()
    );
    let oil = contains_any(
        &line,
        &[
            " gloss",
            " oil based",
            " oil-based",
            " white spirit",
            " varnish",
            " oil paint",
            " turps",
        ],
    );
    let roller = contains_any(&line, &[" roller", " sleeve", " sleeves"]);
    let brush = contains_any(&line, &["brush", "bristle"]);
    let reply = if contains_any(
        &line,
        &[
            " gone hard",
            " went hard",
            " hard",
            " stiff",
            " dried out",
            " dry out",
            " save",
            " saved",
            " rescue",
        ],
    ) {
        if roller { HARD_ROLLER } else { HARD_BRUSH }
    } else if contains_any(
        &line,
        &[
            " losing bristles",
            " lose bristles",
            " loses bristles",
            " lost bristles",
            " sheds",
            " shedding",
            " bristles are falling",
            " bristles falling",
            " bristles falling out",
            " bristles coming out",
            " loose bristles",
            " moulting",
            " falling apart",
            " falls apart",
            " come apart",
            " comes apart",
            " coming apart",
        ],
    ) {
        LOOSE_BRISTLES
    } else if roller && !brush {
        if contains_any(
            &line,
            &[
                " between coats",
                " overnight",
                " over night",
                " store",
                " storing",
                " keep",
                " keeping",
                " tonight",
                " tomorrow",
                " next day",
                " wrap",
                " wrapped",
                " cling film",
                " clingfilm",
                " carrier bag",
                " plastic bag",
                " lunchtime",
                " short break",
            ],
        ) && !contains_any(&line, &[" clean", " wash", " rinse"])
        {
            STORE_ROLLER
        } else if oil {
            WASH_OIL_ROLLER
        } else {
            WASH_ROLLER
        }
    } else if contains_any(
        &line,
        &[
            " clean",
            " wash",
            " rinse",
            " after ",
            " finished",
            " done with",
            " out of",
            " get out",
            " got out",
            " remove",
            " removing",
            " off a brush",
            " off my brush",
            " off the brush",
            " off a paintbrush",
            " off my paintbrush",
            " out of a brush",
            " out of my brush",
            " out of a paintbrush",
            " out of my paintbrush",
            " out of the bristles",
            " off a roller",
            " out of a roller",
            " out of the sleeve",
            " get it off",
            " get the paint off",
        ],
    ) {
        if oil { WASH_OIL_BRUSH } else { WASH_BRUSH }
    } else if contains_any(
        &line,
        &[
            " overnight",
            " over night",
            " tomorrow",
            " next day",
            " fridge",
            " until the morning",
        ],
    ) {
        STORE_BRUSH
    } else {
        BETWEEN_COATS
    };
    Some(Cow::Borrowed(reply))
}

fn contains_any(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

fn first_text<'a>(preferred: &'a str, fallback: &'a str) -> &'a str {
    if preferred.is_empty() {
        fallback
    } else {
        preferred
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_takes_only_answers_its_own_job() {
        let r = Reading::new("how long does it take for a fridge to get cold?");
        assert!(machine_takes(&r).unwrap().starts_with("A fridge takes "));
        let r = Reading::new("my tumble dryer takes two goes to dry a load");
        assert!(machine_takes(&r).unwrap().starts_with("Slow drying"));
        for prompt in [
            "how long does a hair dryer take",
            "how long does an oven take to cook a chicken",
        ] {
            assert_eq!(machine_takes(&Reading::new(prompt)), None, "{prompt}");
        }
    }

    #[test]
    fn a_service_interval_and_verdict_have_separate_asks() {
        let r = Reading::new("how often should I service my washing machine?");
        assert_eq!(service_worth(&r).as_deref(), Some(WASHING_MACHINE_OFTEN));
        let r = Reading::new("is servicing my dishwasher worthwhile?");
        assert_eq!(service_worth(&r).as_deref(), Some(DISHWASHER_WORTH));
        assert_eq!(
            service_worth(&Reading::new("how do I service my oven?")),
            None
        );
    }

    #[test]
    fn water_use_respects_travel_and_corrected_figures() {
        assert!(!not_length_of_stay(&Reading::new(
            "how many days should I spend in Bath?"
        )));
        let r = Reading::new("how much water does a bath hold?");
        assert!(not_length_of_stay(&r));
        assert!(water_use(&r).unwrap().starts_with("A full bath"));
        let r = Reading::new("how much water does a toilet use?");
        assert!(water_use(&r).unwrap().contains("420 to 910 litres a week"));
        let r = Reading::new("how much water does a hot tub hold?");
        assert!(
            water_use(&r)
                .unwrap()
                .contains("your model's maintenance instructions")
        );
    }

    #[test]
    fn paint_storage_and_washing_follow_distinct_branches() {
        for prompt in [
            "paintbrush overnight",
            "paint roller overnight",
            "paintbrush between coats",
        ] {
            let answer = paint_brush_care(&Reading::new(prompt)).unwrap();
            assert!(answer.contains("a dry place away from food"), "{prompt}");
        }
        let r = Reading::new("clean gloss paint brush");
        assert_eq!(paint_brush_care(&r).as_deref(), Some(WASH_OIL_BRUSH));
    }

    #[test]
    fn machine_filter_replies_use_the_current_line_or_three_retained_topics() {
        let row = super::super::by_id("home", "washing-machine-filter").unwrap();
        assert!(row.reply.starts_with("Switch off and unplug the machine"));
        let reply = "matching reply";
        let names = ["washing machine", "washer"];
        let read = Reading::new("how do I clean the filter?");
        assert_eq!(filter(&names, reply, &Context::new(&read, &[])), None);
        let read_with_machine = Reading::new("how do I clean the washer filter?");
        assert_eq!(
            filter(&names, reply, &Context::new(&read_with_machine, &[])),
            Some(reply)
        );
        let read_with_other_machine = Reading::new("how do I clean the dishwasher filter?");
        assert_eq!(
            filter(&names, reply, &Context::new(&read_with_other_machine, &[])),
            None
        );

        let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
        bot.turn("my washing machine has stopped draining");
        let reply_from = |bot: &crate::bot::Bot| {
            let mut ctx = Context::new(&read, &[]);
            ctx.memory = Some(&bot.memory);
            filter(&names, reply, &ctx)
        };
        assert_eq!(reply_from(&bot), Some(reply));

        for prompt in [
            "different question: Mars has polar ice caps",
            "new topic: my chocolate cake keeps sinking",
            "unrelated: the neighbours own a terrier",
        ] {
            bot.turn(prompt);
        }
        assert_eq!(
            reply_from(&bot),
            None,
            "{:?}",
            bot.memory
                .doneness_topics
                .recent_texts(3)
                .collect::<Vec<_>>()
        );
    }
}
