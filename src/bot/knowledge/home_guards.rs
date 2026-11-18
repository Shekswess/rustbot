//! Selection guards for household rules whose keys alone overlap with other subjects.

use crate::bot::Reading;
use fancy_regex::Regex as Lookaround;
use regex::Regex;
use std::sync::LazyLock;

#[path = "home_guard_data.rs"]
mod data;

#[path = "home_replies.rs"]
mod replies;
pub(crate) use replies::registered;

#[path = "home_extra.rs"]
mod extra;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    BOILER_IS_WRONG,
    BUY_NAMED_FIRST,
    THING_IN_HAND,
    VISITOR,
    CALLOUT_FAULT,
    CALLOUT_WORK,
    HOME_SIZE,
    HOME_SIZE_OPENER,
    HOME_SIZE_CARRIED,
    FENCE_WORD,
    DISHWASHER,
    WASHER_MACHINE,
    RESTOCK,
    WHERE_IT_GOES,
    MACHINE_FAULT,
    ANIMAL_IN_MACHINE,
    MACHINE_SYMPTOM,
    ASKS_HOW,
    PART_OF_PERSON,
    GLASSES,
    HEAT_OBJECT,
    REPAIR_VERB,
    NEEDS_MENDING,
    WASHER_AT_JOINT,
    TEMPERATURE,
    BARE_TEMPERATURE,
    WATER_LEAVING,
    WASHER_DONE,
    HAS_GONE,
    SANDING_JOB,
);
static BUY: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::BUY).expect("callout purchase pattern"));
static HEATED_ARGUMENT: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::HEATED_ARGUMENT).expect("heated argument pattern"));
static BOILER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:boiler|boilers|combi)\b").unwrap());
static WORTH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bworth\b").unwrap());
static WASHING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(?:washer|washing)\b").unwrap());
static PACKING_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z]+").unwrap());

pub(crate) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    match id {
        "home/no-room-at-home" => reading.sense.get("room") != Some(&"booking"),
        "home/washer-sound" => {
            let said = reading.line.clean.to_lowercase();
            WASHING.is_match(&said) || !contains_any(&said, &["dryer", "tumble", "dishwasher"])
        }
        "home/boiler-callout" => visit_not_purchase(reading),
        "home/radiator-bleed" => {
            let said = joined(reading, true).to_lowercase();
            !BOILER.is_match(&said) || !BOILER_IS_WRONG.is_match(&said)
        }
        "home/furniture-upstairs" => furniture(reading),
        "home/fence-build" => FENCE_WORD.is_match(&joined(reading, true)),
        "home/ceiling-colour" => ceiling_colour(reading),
        "home/boiler-flow" | "home/heating-fuel" => !sound_named(reading),
        "home/machine-consumables" => machine_consumable(reading),
        "home/wash-cycle-time" | "home/use-the-washer" => machine_can_be_used(reading),
        "home/broken-glass-on-the-floor" => !GLASSES.is_match(
            &[
                &reading.line.clean,
                &reading.line.norm_text,
                &reading.line.text,
            ]
            .map(String::as_str)
            .join(" "),
        ),
        "home/heat-a-room" => heat_a_room(reading),
        "home/a-fixing-and-its-drill-bit" => fixing_question(reading),
        "home/boiler-service-diy" => {
            let said = clean(reading).to_lowercase();
            words_include(reading, data::SERVICE_WORDS) || contains_any(&said, data::SERVICE_WORDS)
        }
        "home/wash-at-30" => wash_is_named(reading),
        "home/washing-machine-size" => washer_is_machine(reading),
        "home/a-washer-two-things" => {
            words_include(reading, data::WASHER_WORDS_OR_PHRASES)
                && !words_include(reading, data::MACHINE_WORDS)
                && !words_include(reading, data::OTHER_WASHER)
                && !WASHER_DONE.is_match(clean(reading))
        }
        "home/washing-machine-leak" => {
            washer_is_machine(reading) && WATER_LEAVING.is_match(&words_or_clean(reading))
        }
        "home/packing-a-kitchen" => {
            reading.reference.followup
                || PACKING_WORD
                    .find_iter(&clean(reading).to_lowercase())
                    .any(|word| data::PACKING_VERBS.contains(&word.as_str()))
        }
        "home/bulk-quantity" => not_a_sanding_job(reading),
        "home/cheapest-hours" => {
            !reading.features.asks.contains(&"where")
                && !contains_any(
                    &format!(" {} ", reading.line.words.join(" ").to_lowercase()),
                    data::WHERE_PHRASES,
                )
        }
        "home/empty-the-kitchen-bin" => {
            !words_include(reading, data::MACHINE_BIN_WORDS)
                && !reading
                    .topics
                    .iter()
                    .any(|&(topic, _)| topic == "technology")
        }
        _ => false,
    }
}

pub(crate) fn not_a_sanding_job(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    !SANDING_JOB.is_match(
        &[line.clean.as_str(), &line.norm_text, &line.plain]
            .join(" ")
            .to_lowercase(),
    )
}

fn clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn words_or_clean(reading: &Reading<'_>) -> String {
    if reading.line.words.is_empty() {
        clean(reading).to_lowercase()
    } else {
        reading.line.words.join(" ").to_lowercase()
    }
}

fn words_include(reading: &Reading<'_>, words: &[&str]) -> bool {
    reading
        .line
        .words
        .iter()
        .any(|word| words.contains(&word.as_str()))
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn joined(reading: &Reading<'_>, with_words: bool) -> String {
    let mut parts = [
        reading.line.clean.as_str(),
        reading.line.text.as_str(),
        reading.raw,
        "",
        reading.line.plain.as_str(),
    ]
    .join(" ");
    if with_words {
        for word in &reading.line.words {
            parts.push(' ');
            parts.push_str(word);
        }
    }
    parts
}

fn visit_not_purchase(reading: &Reading<'_>) -> bool {
    let said = joined(reading, false).to_lowercase();
    !WORTH.is_match(&said)
        || THING_IN_HAND.is_match(&said)
        || VISITOR.is_match(&said)
        || CALLOUT_FAULT.is_match(&said)
        || CALLOUT_WORK.is_match(&said)
        || (!BUY.is_match(&said).unwrap_or(false) && !BUY_NAMED_FIRST.is_match(&said))
}

fn furniture(reading: &Reading<'_>) -> bool {
    let said = reading.line.clean.to_lowercase();
    let Some(size) = HOME_SIZE.find(&said) else {
        return true;
    };
    !HOME_SIZE_OPENER.is_match(&said[..size.start()])
        || HOME_SIZE_CARRIED.is_match(&[&said[..size.start()], &said[size.end()..]].join(" "))
}

fn ceiling_colour(reading: &Reading<'_>) -> bool {
    let said = format!(" {} ", reading.line.clean.to_lowercase());
    !contains_any(&said, data::CEILING_HOW_TO)
        && data::CEILING_COLOURS.iter().any(|word| {
            said.match_indices(word)
                .any(|(start, _)| said[..start].ends_with(' '))
        })
}

fn sound_named(reading: &Reading<'_>) -> bool {
    words_include(reading, data::SOUND_WORDS)
        || contains_any(&clean(reading).to_lowercase(), data::SOUND_STEMS)
}

fn machine_consumable(reading: &Reading<'_>) -> bool {
    let said = if reading.line.plain.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.plain
    };
    (!RESTOCK.is_match(said) || WHERE_IT_GOES.is_match(said))
        && (DISHWASHER.is_match(said)
            || WASHER_MACHINE.is_match(said)
            || contains_any(said, data::ONLY_A_MACHINE))
}

fn machine_can_be_used(reading: &Reading<'_>) -> bool {
    let said = [&reading.line.text[..], &reading.line.clean]
        .join(" ")
        .to_lowercase();
    !ANIMAL_IN_MACHINE.is_match(&said)
        && !PART_OF_PERSON.is_match(&said)
        && (ASKS_HOW.is_match(&said)
            || !(MACHINE_FAULT.is_match(&said) || MACHINE_SYMPTOM.is_match(&said)))
        && !thing_has_gone(reading)
}

pub(super) fn thing_has_gone(reading: &Reading<'_>) -> bool {
    let said = words_or_clean(reading);
    HAS_GONE.captures_iter(&said).any(|found| {
        let state = found[1]
            .split(crate::text::is_space)
            .map(|word| word.trim_matches(['\'', '.', ',', '!', '?']))
            .find(|word| !word.is_empty() && !data::GONE_INTENSIFIER.contains(word));
        state.is_none_or(|state| !data::GONE_STATE.contains(&state))
    })
}

fn heat_a_room(reading: &Reading<'_>) -> bool {
    let typed = if reading.line.text.is_empty() {
        clean(reading)
    } else {
        &reading.line.text
    };
    !HEATED_ARGUMENT.is_match(typed).unwrap_or(false)
        && !HEAT_OBJECT
            .captures_iter(&clean(reading).to_lowercase())
            .any(|item| data::HEATED_PART.contains(&&item[1]))
}

fn fixing_question(reading: &Reading<'_>) -> bool {
    let said = clean(reading).to_lowercase();
    let has_washer = words_include(reading, data::WASHER_WORDS);
    !REPAIR_VERB.is_match(&said)
        && !NEEDS_MENDING.is_match(&said)
        && (!has_washer || words_include(reading, data::FIXING))
        && (!(has_washer || said.contains("washer")) || WASHER_AT_JOINT.is_match(&said))
}

fn wash_is_named(reading: &Reading<'_>) -> bool {
    let words = words_or_clean(reading);
    if words
        .split(|c| c == ',' || crate::text::is_space(c))
        .any(|word| data::WASH_NAMED.contains(&word))
        || TEMPERATURE.is_match(&words)
    {
        return true;
    }
    let mut temperatures = BARE_TEMPERATURE
        .captures_iter(&words)
        .filter_map(|item| item.get(1).or_else(|| item.get(0)));
    temperatures
        .next()
        .is_some_and(|first| temperatures.any(|other| other.as_str() != first.as_str()))
}

fn washer_is_machine(reading: &Reading<'_>) -> bool {
    !words_include(reading, data::WASHER_WORDS_OR_PHRASES)
        || words_include(reading, data::MACHINE_WORDS)
        || !words_include(reading, data::OTHER_WASHER)
}

#[cfg(test)]
#[path = "../../../tests/fixtures/home_guards.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn household_rules_use_the_same_relevance_guards_as_pybot() {
        let mismatches: Vec<_> = fixtures::CASES
            .iter()
            .filter_map(|(id, prompt, expected)| {
                let actual = passes(id, &Reading::new(prompt));
                (actual != *expected).then_some((*id, *prompt, *expected, actual))
            })
            .collect();
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }
}
