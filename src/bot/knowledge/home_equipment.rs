//! Recognize household machine faults before choosing fault-specific advice.

use std::{borrow::Cow, sync::LazyLock};

use regex::RegexSet;

use crate::bot::Reading;

use super::history_cards::whole_words;

#[path = "home_equipment_data.rs"]
mod data;

macro_rules! words {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<RegexSet> = LazyLock::new(|| whole_words(data::$name));)+
    };
}
words!(
    MACHINE_BURNING,
    MACHINE_LEAKING,
    DEAD_ALREADY_ANSWERED,
    BURNING_ALREADY_ANSWERED
);

static NOISE_NAMES: LazyLock<Vec<RegexSet>> = LazyLock::new(|| {
    data::NOISE_ROWS
        .iter()
        .map(|row| whole_words(row.names))
        .collect()
});
static NOISE_KINDS: LazyLock<Vec<RegexSet>> = LazyLock::new(|| sound_patterns(data::NOISE_KINDS));
static FIRST_NOISE: LazyLock<Vec<Vec<RegexSet>>> = LazyLock::new(|| {
    data::NOISE_ROWS
        .iter()
        .map(|row| sound_patterns(row.first))
        .collect()
});
static LEAK_NAMES: LazyLock<Vec<RegexSet>> = LazyLock::new(|| {
    data::LEAK_CHECKS
        .iter()
        .map(|(names, _)| whole_words(names))
        .collect()
});
static MACHINE_NAMES: LazyLock<Vec<RegexSet>> = LazyLock::new(|| {
    data::MACHINE_CHECKS
        .iter()
        .map(|(names, _, _)| whole_words(names))
        .collect()
});

fn sound_patterns(kinds: &[(&str, &[&str])]) -> Vec<RegexSet> {
    kinds
        .iter()
        .map(|(_, stems)| {
            RegexSet::new(stems.iter().map(|stem| format!(r"\b{stem}"))).expect("noise word stem")
        })
        .collect()
}

fn noise_kind(
    line: &str,
    patterns: &[RegexSet],
    kinds: &[(&'static str, &[&str])],
) -> Option<&'static str> {
    patterns
        .iter()
        .zip(kinds)
        .find_map(|(pattern, (kind, _))| pattern.is_match(line).then_some(*kind))
}

const AFTER_BANG: &str = "A bang can mean something is loose or being hit. Switch off and unplug the appliance before checking for a loose item or whether the machine is level and clear of the cabinet. If it still bangs, stop using it and have it checked.";

pub(super) fn appliance_noise(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = reading.line.clean.to_lowercase();
    let index = NOISE_NAMES
        .iter()
        .position(|pattern| pattern.is_match(&line))?;
    let row = &data::NOISE_ROWS[index];
    let kind = noise_kind(&line, &FIRST_NOISE[index], row.first)
        .or_else(|| noise_kind(&line, &NOISE_KINDS, data::NOISE_KINDS));
    let advice = kind.and_then(|kind| {
        row.kinds
            .iter()
            .find_map(|(name, reply)| (*name == kind).then_some(*reply))
            .or_else(|| (kind == "knock").then_some(AFTER_BANG))
    });
    let mut parts = vec![row.normal];
    if let Some(advice) = advice {
        if kind == Some("loud") {
            parts.push(advice);
        } else {
            parts.insert(0, advice);
        }
    }
    parts.push(row.urgent);
    Some(Cow::Owned(parts.join(" ")))
}

const LEAK_FALLBACK: &str = "Stop using the leaking machine and unplug it if you can do so safely. A leak can come from a seal, a gasket or a split tank. Check the model's manual before inspecting any removable parts.";
const FIRE_LEAD: &str = "Switch off and unplug it only if you can do so safely. If there is a fire or smoke is building up, get out, stay out and call the fire service from outside (999 in the UK). ";
const BURNING_FALLBACK: &str = "Stop using it and have the retailer, manufacturer or a qualified repairer check it before you use it again.";
const DEAD_FALLBACK: &str = "Check the plug and socket for visible damage and follow the manual for basic power checks. If the appliance stays dead, contact the retailer or manufacturer for advice.";

pub(super) fn kitchen_machine_fault(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    let line = if clean.is_empty() {
        &reading.line.norm_text
    } else {
        clean
    }
    .to_lowercase();
    let burning = MACHINE_BURNING.is_match(&line);
    if !burning && MACHINE_LEAKING.is_match(&line) {
        let reply = LEAK_NAMES
            .iter()
            .zip(data::LEAK_CHECKS)
            .find_map(|(pattern, &(_, reply))| pattern.is_match(&line).then_some(reply))
            .unwrap_or(LEAK_FALLBACK);
        return Some(Cow::Borrowed(reply));
    }
    if (burning && BURNING_ALREADY_ANSWERED.is_match(&line))
        || (!burning && DEAD_ALREADY_ANSWERED.is_match(&line))
    {
        return None;
    }
    let reply =
        MACHINE_NAMES
            .iter()
            .zip(data::MACHINE_CHECKS)
            .find_map(|(pattern, &(_, hot, dead))| {
                pattern
                    .is_match(&line)
                    .then_some(if burning { hot } else { dead })
            });
    if burning {
        return Some(Cow::Owned(format!(
            "{FIRE_LEAD}{}",
            reply.unwrap_or(BURNING_FALLBACK)
        )));
    }
    Some(Cow::Borrowed(reply.unwrap_or(DEAD_FALLBACK)))
}

const MOWER_CORDLESS: &str = "For a cordless mower, use the manufacturer-approved charger and let the battery reach the temperature range in the manual. Check that the battery is seated and read any fault light. Stop charging a damaged, swollen or overheating battery and ask the manufacturer what to do.";
const MOWER_BATTERY_BUY: &str = "Use the mower's model number and the old battery label to find a manufacturer-approved replacement. Check the stated voltage, capacity, dimensions and terminal layout with the supplier.";
const MOWER_BATTERY: &str = "Use the charge method and charger specified for the mower's battery. If the battery repeatedly goes flat or won't charge, get the battery and charging system checked. Follow the mower manual for jump-starting or terminal maintenance because designs vary.";
const MOWER_PETROL: &str = "On a petrol mower that won't start, check the fuel and oil levels, air filter and spark plug as the manual directs. Work outdoors, switch off and let it cool before handling fuel. Disconnect the spark plug lead before clearing anything underneath the deck. For an electric mower, unplug it or remove the battery before inspection.";
const MOWER_BLADE: &str = "Switch off and wait for the blade to stop. Unplug a corded mower, remove the battery from a cordless mower, or disconnect a petrol mower's spark plug lead as directed in the manual. Follow the manual for removing, sharpening or replacing the blade. Replace a damaged blade with the correct part or ask a mower service shop.";
const MOWER_OIL: &str = "The engine manual gives the oil type, capacity and service interval. Stop the engine and disconnect the spark plug lead as directed before draining the oil. Follow the model's instructions for positioning the mower and checking the level. A two-stroke engine may use a fuel-oil mix instead of a separate sump.";
const MOWER_FILTER: &str = "Check the engine manual for the filter type and service interval. Switch off and let the engine cool before taking the filter out. Some foam filters can be washed and re-oiled, while paper filters usually need replacing when clogged.";

fn mower_phrase(line: &str, phrases: &[&str]) -> bool {
    phrases
        .iter()
        .any(|phrase| line.contains(&format!(" {phrase} ")))
}

pub(super) fn mower(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = reading.line.words.join(" ").to_lowercase();
    let line = format!(" {said} ");
    let reply = if mower_phrase(&line, data::MOWER_CORDLESS) {
        MOWER_CORDLESS
    } else if mower_phrase(&line, data::MOWER_BATTERY_BUY) {
        MOWER_BATTERY_BUY
    } else if mower_phrase(&line, data::MOWER_BATTERY) {
        MOWER_BATTERY
    } else if mower_phrase(&line, data::MOWER_PETROL) {
        MOWER_PETROL
    } else if mower_phrase(&line, data::MOWER_BLADE) {
        MOWER_BLADE
    } else if mower_phrase(&line, &["oil", "oili", "change the oil"]) {
        MOWER_OIL
    } else {
        MOWER_FILTER
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_noises_put_specific_checks_before_ordinary_sounds() {
        let shake = appliance_noise(&Reading::new("my tumble dryer is shaking")).unwrap();
        assert!(shake.starts_with("A dryer that shakes"));
        let oven = appliance_noise(&Reading::new("my oven fan is noisy")).unwrap();
        assert!(oven.starts_with("The fan running on"));
        let knock = appliance_noise(&Reading::new("my dishwasher bangs")).unwrap();
        assert!(knock.starts_with("A bang"));
        assert_eq!(appliance_noise(&Reading::new("my mouse squeals")), None);
    }

    #[test]
    fn kitchen_fault_leaks_and_burning_have_distinct_first_actions() {
        assert!(
            kitchen_machine_fault(&Reading::new("my kettle leaks"))
                .unwrap()
                .starts_with("Stop using the leaking kettle")
        );
        assert!(
            kitchen_machine_fault(&Reading::new("my slow cooker smells of burning"))
                .unwrap()
                .starts_with(FIRE_LEAD)
        );
        assert_eq!(
            kitchen_machine_fault(&Reading::new("my air fryer is smoking")),
            None
        );
        assert_eq!(
            kitchen_machine_fault(&Reading::new("my kettle stopped working")),
            None
        );
    }

    #[test]
    fn mower_keeps_service_and_battery_branches_bounded() {
        assert_eq!(
            mower(&Reading::new("how do I sharpen the lawn mower blade")).as_deref(),
            Some(MOWER_BLADE)
        );
        assert_eq!(
            mower(&Reading::new("which battery for my ride on mower")).as_deref(),
            Some(MOWER_BATTERY_BUY)
        );
        assert_eq!(
            mower(&Reading::new("my cordless mower is not starting")).as_deref(),
            Some(MOWER_CORDLESS)
        );
        assert_eq!(
            mower(&Reading::new("how do i change the oil in my mower")).as_deref(),
            Some(MOWER_OIL)
        );
    }
}
