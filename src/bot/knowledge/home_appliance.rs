//! Fault and part dispatch for household appliances.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::Reading;

use super::history_cards::whole_words;

#[path = "home_appliance_data.rs"]
mod data;

static MICROWAVE: LazyLock<RegexSet> =
    LazyLock::new(|| whole_words(&["microwave", "microwaves", "microwave oven"]));
static KETTLE: LazyLock<RegexSet> = LazyLock::new(|| whole_words(&["kettle", "kettles"]));
static PUMP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bpumps?\b").expect("separate pump part"));
static CUTOUT: LazyLock<Vec<RegexSet>> = LazyLock::new(|| {
    data::CUTTING_OUT
        .iter()
        .map(|(words, _)| whole_words(words))
        .collect()
});
static MACHINES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::MACHINE_NAMES
        .iter()
        .map(|(alias, _)| {
            Regex::new(&format!(r"\b{}s?\b", regex::escape(alias))).expect("machine alias")
        })
        .collect()
});

const MICROWAVE_COLD: &str = "Check the manual for the power setting and make sure the microwave door closes properly. If its light and turntable work but the food stays cold, stop using it and contact the manufacturer or an appliance engineer. Leave the casing closed. Internal parts can still hold a dangerous charge after unplugging.";
const KETTLE_COLD: &str = "Switch off and unplug the kettle before checking whether it sits firmly on the base. If you see damage, signs of heat or corrosion, stop using it. If it still won't heat after the checks in the manual, ask the retailer or manufacturer about repair or replacement.";
const TOASTER_COLD: &str = "Switch off and unplug the toaster. Check the timer setting and, once it has cooled, empty the crumb tray as the manual directs. Leave the casing closed and keep fingers and tools out of the slots. If it still won't heat, ask the retailer or manufacturer about repair or replacement.";

pub(super) fn not_heating(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = reading.line.clean.to_lowercase();
    let reply = if MICROWAVE.is_match(&line) {
        MICROWAVE_COLD
    } else if KETTLE.is_match(&line) {
        KETTLE_COLD
    } else {
        TOASTER_COLD
    };
    Some(Cow::Borrowed(reply))
}

const CUTOUT_FALLBACK: &str = "An appliance can stop part way if it overheats. Switch it off and let it cool before checking the filter or vents as the manual directs. If it keeps stopping, have it checked.";
const CUTOUT_ELECTRICS: &str = "If the plug or socket is hot, scorched, buzzing or sparking, stop using it and get it checked before using it again.";

pub(super) fn cutting_out(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = reading.line.clean.to_lowercase();
    let lead = CUTOUT
        .iter()
        .zip(data::CUTTING_OUT)
        .find_map(|(aliases, &(_, reply))| aliases.is_match(&line).then_some(reply))
        .unwrap_or(CUTOUT_FALLBACK);
    Some(Cow::Owned(format!("{lead} {CUTOUT_ELECTRICS}")))
}

fn names_have(line: &str, names: &[&str]) -> bool {
    names
        .iter()
        .any(|name| line.contains(&format!(" {name} ")) || line.contains(&format!(" {name}s ")))
}

fn has_drain_fault(line: &str) -> bool {
    data::A_DRAIN_FAULT.iter().any(|word| line.contains(word))
}

pub(super) fn not_washer_drain(reading: &Reading<'_>) -> bool {
    let said = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    let line = said
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    !has_drain_fault(&line) || !(line.contains("washing machine") || line.contains("washer"))
}

pub(super) fn machine_part(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = format!(" {} ", reading.line.words.join(" ").to_lowercase());
    let &(parts, advice) = data::MACHINE_PARTS
        .iter()
        .find(|(parts, _)| names_have(&line, parts))?;
    let machine = MACHINES
        .iter()
        .zip(data::MACHINE_NAMES)
        .find_map(|(alias, &(_, name))| alias.is_match(&line).then_some(name))?;
    if parts[0] == "pump" && data::DRYER_KINDS.iter().any(|kind| line.contains(kind)) {
        let mut rest = line.clone();
        for kind in data::DRYER_KINDS {
            rest = rest.replace(kind, " ");
        }
        if !PUMP.is_match(&rest) && !has_drain_fault(&line) {
            return None;
        }
    }
    let end = if machine == "machine" {
        "The model number on the rating plate is what the parts shop needs.".to_owned()
    } else {
        format!(
            "Yours is a {machine}, so the model number on the rating plate is what the parts shop needs."
        )
    };
    Some(Cow::Owned(format!("{advice} {end}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_heat_pump_is_a_kind_of_dryer_beside_a_drain_fault() {
        let read = Reading::new("is a heat pump dryer cheaper to run?");
        assert_eq!(machine_part(&read), None);
        let read = Reading::new("my heat pump dryer is not draining");
        assert!(machine_part(&read).unwrap().starts_with("The drain pump"));
        let read = Reading::new("my washing machine is not draining");
        assert!(!not_washer_drain(&read));
        let read = Reading::new("my washing machine's belt snapped");
        assert!(machine_part(&read).unwrap().contains("user-replaceable"));
    }

    #[test]
    fn house_appliance_fault_suggestions_keep_live_parts_closed() {
        let read = Reading::new("my toaster won't heat");
        assert!(
            not_heating(&read)
                .unwrap()
                .contains("keep fingers and tools out")
        );
        let read = Reading::new("my microwave runs and won't heat");
        assert!(
            not_heating(&read)
                .unwrap()
                .contains("Leave the casing closed")
        );
        let read = Reading::new("my washer stops mid-cycle");
        let reply = cutting_out(&read).unwrap();
        assert!(reply.starts_with("A washer that stops mid-cycle"));
        assert!(reply.contains("error code for your exact model"));
        assert!(reply.contains("stop using it"));
    }
}
