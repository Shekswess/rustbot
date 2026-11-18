//! Distinguish individual, floor and whole-system heating faults.

use std::{borrow::Cow, sync::LazyLock};

use regex::RegexSet;

use crate::bot::{Reading, modules::safety::current_gas};

use super::{history_cards::whole_words, home_fittings::GAS_EMERGENCY};

#[path = "home_heating_data.rs"]
mod data;

static HOUSE: LazyLock<RegexSet> = LazyLock::new(|| whole_words(data::WHOLE_HOUSE));
static PLURAL: LazyLock<RegexSet> = LazyLock::new(|| whole_words(&["radiators", "rads"]));

fn contains_any(line: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| line.contains(term))
}

fn words_any(words: &[String], terms: &[&str]) -> bool {
    words.iter().any(|word| terms.contains(&word.as_str()))
}

const BOILER_HUM: &str = "A hum can be vibration in a pipe or come from the pump or fan. If it is new or changes when the boiler fires, have an engineer check it. Leave the casing and internal pump controls alone.";
const BOILER_WHISTLE: &str = "A whistle, whine or hiss can come from restricted flow, trapped air or a boiler fault. Check the display and the recommended pressure range in your model's manual. If it recurs or you are unsure whether the boiler is safe, turn it off and get it checked.";
const BOILER_GURGLE: &str = "Gurgling can come from trapped air. If the radiators need bleeding, switch the heating off, let it cool and follow the manufacturer's instructions. Check your boiler manual for its pressure range. If the gurgling returns, have the system checked.";
const BOILER_TICK: &str = "A tick or a tap can be pipes expanding as they warm up. Note where it happens and whether it changes. If the ticking turns into a knock or there are other faults, have the system checked.";
const BOILER_GRIND: &str = "Switch the boiler off and get it checked if it is grinding. The sound can come from a pump or a fan. Note when it happens and leave the case closed.";
const BOILER_BANG: &str = "Banging or rumbling can come from trapped air, poor flow or a pump fault. Check the display and the pressure range in your model's manual. Switch off if there is a burning smell, a fault code or any reason to think the boiler is unsafe. Arrange a Gas Safe registered engineer for a gas boiler.";

pub(super) fn boiler_noise(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = [&reading.line.plain, &reading.line.text, &reading.line.clean]
        .into_iter()
        .find(|text| !text.is_empty())
        .map_or("", String::as_str)
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if current_gas(&line) {
        return Some(Cow::Borrowed(GAS_EMERGENCY));
    }
    let words = &reading.line.words;
    let reply = if contains_any(
        &line,
        &[
            "humming",
            "hummed",
            "buzzing",
            "buzzed",
            "vibrating",
            "droning",
        ],
    ) || words_any(
        words,
        &[
            "hum",
            "hums",
            "humming",
            "hummed",
            "buzz",
            "buzzes",
            "buzzing",
            "buzzed",
            "vibrate",
            "vibrates",
            "vibrating",
            "vibrated",
            "drone",
            "drones",
            "droning",
        ],
    ) {
        BOILER_HUM
    } else if contains_any(
        &line,
        &[
            "whistl", "whine", "whining", "squeal", "squeak", "screech", "hiss", "shriek",
        ],
    ) || words_any(
        words,
        &[
            "squeak",
            "squeaks",
            "squeaking",
            "squeaked",
            "screech",
            "screeches",
            "screeching",
            "screeched",
            "squeal",
            "squeals",
            "squealing",
            "squealed",
        ],
    ) {
        BOILER_WHISTLE
    } else if contains_any(&line, &["gurgl", "gurgling", "glug"]) {
        BOILER_GURGLE
    } else if contains_any(&line, &["tap", "tapping", "tick", "ticking", "click"]) {
        BOILER_TICK
    } else if contains_any(&line, &["grind", "grinding", "scraping"])
        || words_any(
            words,
            &[
                "grind", "grinds", "grinding", "ground", "scrape", "scrapes", "scraping", "scraped",
            ],
        )
    {
        BOILER_GRIND
    } else {
        BOILER_BANG
    };
    Some(Cow::Borrowed(reply))
}

fn nearest(words: &[&str], index: usize, states: &[&str]) -> Option<(usize, usize)> {
    words[index + 1..]
        .iter()
        .position(|word| states.contains(word))
        .map(|offset| (offset + 1, 0))
        .or_else(|| {
            words[..index]
                .iter()
                .rev()
                .position(|word| states.contains(word))
                .map(|offset| (offset + 1, 1))
        })
}

#[derive(PartialEq, Debug)]
enum ColdFloor {
    Up,
    Down,
    Both,
}

fn radiator_floor(line: &str) -> Option<ColdFloor> {
    let spaced = line.replace([',', '.'], " ");
    let words = spaced.split_whitespace().collect::<Vec<_>>();
    let (mut cold_up, mut cold_down) = (false, false);
    for (index, word) in words.iter().enumerate() {
        let up = data::UP.contains(word);
        if !up && !data::DOWN.contains(word) {
            continue;
        }
        let cold = nearest(&words, index, data::COLD);
        let warm = nearest(&words, index, data::WARM);
        let is_cold = cold.is_some_and(|cold| warm.is_none_or(|warm| cold < warm));
        let is_warm = warm.is_some_and(|warm| cold.is_none_or(|cold| warm < cold));
        cold_up |= (up && is_cold) || (!up && is_warm);
        cold_down |= (!up && is_cold) || (up && is_warm);
    }
    match (cold_up, cold_down) {
        (true, true) => Some(ColdFloor::Both),
        (true, false) => Some(ColdFloor::Up),
        (false, true) => Some(ColdFloor::Down),
        (false, false) => None,
    }
}

const COLD_DOWN: &str = "Warm upstairs and cold downstairs can mean the downstairs circuit is not getting enough flow. Check that the downstairs thermostat is asking for heat and that the radiator controls are open. A heating engineer can check the zone valve, pump or system balance if the problem persists.";
const COLD_UP: &str = "Cold upstairs and warm downstairs can mean trapped air or uneven flow. Turn off the heating and let the radiators cool before bleeding them according to the manual. If all the upstairs radiators stay cold, ask a heating engineer to check the system balance.";
const COLD_HOUSE: &str = "If every radiator is cold, check that the thermostat and programmer are calling for heat. Read the display and compare the pressure with the range in the boiler manual. Follow the manual for owner checks. If it will not start or keeps cutting out, get it checked.";
const ALREADY_BLED: &str = "If the radiator stays cold after bleeding, check its controls are open. A stuck radiator valve, a flow problem or sludge can cause this. If the controls are open and it stays cold, ask a heating engineer to check the valve and the system.";
const COLD_RADIATOR: &str = "If one radiator is cold while the others are hot, check both of its external controls. A valve or an imbalance can restrict flow to that radiator. If it is still cold with the controls open, ask a heating engineer to check it.";

pub(super) fn radiator_not_hot(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = if reading.line.clean.is_empty() {
        reading.line.words.join(" ").to_lowercase()
    } else {
        reading.line.clean.to_lowercase()
    };
    if contains_any(&line, data::TOO_HOT) && !contains_any(&line, data::SAYS_COLD) {
        return None;
    }
    let one = contains_any(&line, data::ONE);
    let bled = contains_any(&line, data::ALREADY_BLED);
    let reply = if !one && !bled {
        match radiator_floor(&line) {
            Some(ColdFloor::Down) => COLD_DOWN,
            Some(ColdFloor::Up) => COLD_UP,
            _ if HOUSE.is_match(&line) || PLURAL.is_match(&line) => COLD_HOUSE,
            _ => COLD_RADIATOR,
        }
    } else if bled {
        ALREADY_BLED
    } else {
        COLD_RADIATOR
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heat_fault_distinguishes_one_radiator_from_cold_floors() {
        assert_eq!(
            radiator_floor("radiators cold upstairs but downstairs is warm"),
            Some(ColdFloor::Up)
        );
        assert_eq!(
            radiator_floor("radiators cold downstairs but upstairs is warm"),
            Some(ColdFloor::Down)
        );
        assert_eq!(
            radiator_not_hot(&Reading::new("the radiator in my bedroom is too hot")),
            None
        );
        let reading = Reading::new("my heating is on but one radiator is cold");
        assert_eq!(radiator_not_hot(&reading).as_deref(), Some(COLD_RADIATOR));
    }

    #[test]
    fn boiler_sound_replies_prioritize_gas_and_urgent_faults() {
        let reading = Reading::new("my boiler is grinding");
        assert_eq!(boiler_noise(&reading).as_deref(), Some(BOILER_GRIND));
        let reading = Reading::new("my boiler is humming and smells of gas");
        assert_eq!(boiler_noise(&reading).as_deref(), Some(GAS_EMERGENCY));
        let reading = Reading::new("my boiler is gurgling");
        assert_eq!(boiler_noise(&reading).as_deref(), Some(BOILER_GURGLE));
    }
}
