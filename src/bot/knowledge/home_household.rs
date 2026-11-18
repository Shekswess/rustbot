//! Household leak, mould and smoke alarm readers.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::Reading;

use super::{guards::Context, history_cards::whole_words};

#[path = "left_on.rs"]
pub(super) mod left_on;

#[path = "home_household_data.rs"]
mod data;

macro_rules! pattern {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
pattern!(
    A_MARK_WORD,
    THE_WATER_IS_MOVING,
    ALARM_STEAM,
    ALARM_NO_CAUSE
);

macro_rules! words {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<RegexSet> = LazyLock::new(|| whole_words(data::$name));)+
    };
}
words!(
    OUTBUILDING,
    MOULD_BODY,
    MOULD_SAID,
    MOULD_SURFACE,
    MOULD_DEVICE,
    MOULD_IN_THE_BATHROOM
);

pub(super) fn water_hammer(reading: &Reading<'_>) -> bool {
    static APPLIANCES: LazyLock<RegexSet> =
        LazyLock::new(|| whole_words(&["washing machine", "washer", "dishwasher", "tumble dryer"]));
    static PLUMBING: LazyLock<RegexSet> = LazyLock::new(|| {
        whole_words(&[
            "pipe",
            "pipes",
            "tap",
            "taps",
            "shower",
            "toilet",
            "cistern",
            "stop tap",
            "valve",
            "valves",
            "water hammer",
            "mains",
            "plumbing",
            "radiator",
            "boiler",
        ])
    });
    let line = &reading.line.clean;
    !APPLIANCES.is_match(line) || PLUMBING.is_match(line)
}

fn fallback<'a>(choices: &[&'a str]) -> &'a str {
    choices
        .iter()
        .copied()
        .find(|text| !text.is_empty())
        .unwrap_or_default()
}

pub(super) fn not_a_body(reading: &Reading<'_>) -> bool {
    let said = fallback(&[
        &reading.line.clean,
        &reading.line.text,
        &reading.line.norm_text,
    ])
    .to_lowercase();
    !MOULD_BODY.is_match(&said) || MOULD_SAID.is_match(&said) || MOULD_SURFACE.is_match(&said)
}

pub(super) fn mould(context: &Context<'_>) -> Option<&'static str> {
    let line = &context.reading.line;
    let said = fallback(&[&line.norm_text, &line.text]);
    let surface = MOULD_SURFACE.is_match(said);
    let named_mould = MOULD_SAID.is_match(said);
    if MOULD_DEVICE.is_match(said) && !surface {
        return named_mould.then_some(MOULD_DEVICE_ADVICE);
    }
    if !named_mould && !surface {
        let mut heard = Vec::new();
        if let Some(memory) = context.memory {
            heard.extend(memory.doneness_topics.current_text());
            heard.extend(
                memory
                    .doneness_topics
                    .current_words()
                    .iter()
                    .map(String::as_str),
            );
        }
        let first = context.earlier.len().saturating_sub(4);
        heard.extend(
            context.earlier[first..]
                .iter()
                .map(|turn| turn.user.as_str()),
        );
        if MOULD_BODY.is_match(&heard.join(" ").to_lowercase()) {
            return None;
        }
    }
    Some(if MOULD_IN_THE_BATHROOM.is_match(said) {
        MOULD_BATHROOM_ADVICE
    } else {
        MOULD_ROOM_ADVICE
    })
}

const MOULD_DEVICE_ADVICE: &str = "Switch the device off and unplug it before cleaning. Use the manufacturer's instructions for the screen or case and keep moisture out of the openings. If mould returns, check for damp around where you keep the device.";
const MOULD_BATHROOM_ADVICE: &str = "For a small patch on a hard surface, use a suitable mould cleaner as directed, wear the recommended protection, and let the surface dry. Run the extractor during and after a shower and wipe off condensation. If the mould covers a large area or comes back, report it if you rent and arrange for someone qualified to check the source of the damp.";
const MOULD_ROOM_ADVICE: &str = "For a small patch on a hard surface, use a suitable mould cleaner as directed and let the surface dry. Check for leaks and condensation and improve ventilation where you can. If mould covers a large area or keeps coming back, arrange a professional assessment of the damp and report it to your landlord if you rent.";

fn leak_line(reading: &Reading<'_>) -> String {
    fallback(&[&reading.line.clean, &reading.line.text]).to_lowercase()
}

pub(super) fn live_mark(reading: &Reading<'_>) -> bool {
    let line = leak_line(reading);
    !A_MARK_WORD.is_match(&line) || THE_WATER_IS_MOVING.is_match(&line)
}

const HOUSE_LEAK: &str = "Keep clear of sagging plaster, and don't touch wet electrical fittings or appliances. If water is near the electrics, switch off at the consumer unit only if you can get to it safely and it is dry. Call a registered electrician before using affected circuits again. Put a bucket under the drip only if the ceiling and electrics are safe. Stay off the wet roof and out of a loft with wet boards. Photograph the damage if safe, report it promptly in writing if you rent, and contact a roofer and your insurer.";
const OUTBUILDING_LEAK: &str = "Stay off the roof and keep away from damaged areas, especially fragile or wet roofing. Move valuables only if the area and electrics are safe. If water has reached electrics, keep clear and call a registered electrician. Photograph the damage if safe and contact a roofer about a temporary cover or repair.";

pub(super) fn roof_leak(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = if OUTBUILDING.is_match(&leak_line(reading)) {
        OUTBUILDING_LEAK
    } else {
        HOUSE_LEAK
    };
    Some(Cow::Borrowed(reply))
}

const FIRE_CHECK: &str = "If the alarm is sounding now and you aren't sure why, or you see signs of fire, get out, stay out and call the fire service from outside (999 in the UK). ";
const ALARM_CARE: &str = "Never disable the alarm or take its battery out to silence it. When you know it was a false alarm, check the manual for cleaning, testing and the replacement date. Get recurring false alarms checked.";
const ALARM_UNKNOWN: &str = "Dust, insects or an ageing sensor can cause false alarms. ";
const ALARM_BATHROOM: &str = "Steam can set off a smoke alarm. After a confirmed false alarm, try keeping the bathroom door shut and using the extractor. Ask your local fire service or a qualified installer to check placement if it keeps happening. ";
const ALARM_COOKING: &str = "Toast or frying can set off a smoke alarm. After a confirmed false alarm, use the extractor before cooking. Ask your local fire service or a qualified installer about a suitable kitchen heat alarm and placement of the smoke alarms. ";

pub(super) fn alarm_nuisance(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = &reading.line.clean;
    let cause = if ALARM_STEAM.is_match(line) {
        ALARM_BATHROOM
    } else if ALARM_NO_CAUSE.is_match(line) {
        ALARM_UNKNOWN
    } else {
        ALARM_COOKING
    };
    Some(Cow::Owned(format!("{FIRE_CHECK}{cause}{ALARM_CARE}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roof_leaks_distinguish_old_marks_and_current_hazards() {
        assert!(!live_mark(&Reading::new("there is a stain on the ceiling")));
        let reading = Reading::new("there is a wet patch by the ceiling light");
        assert!(live_mark(&reading));
        assert!(
            roof_leak(&reading)
                .unwrap()
                .contains("only if you can get to it safely")
        );
        let reading = Reading::new("rain is leaking through the shed roof");
        assert!(
            roof_leak(&reading)
                .unwrap()
                .starts_with("Stay off the roof")
        );
    }

    #[test]
    fn mould_distinguishes_devices_surfaces_and_body_parts() {
        let reading = Reading::new("my screen keeps turning black");
        assert_eq!(mould(&Context::new(&reading, &[])), None);
        let reading = Reading::new("my screen has black mould");
        assert_eq!(
            mould(&Context::new(&reading, &[])),
            Some(MOULD_DEVICE_ADVICE)
        );
        let reading = Reading::new("my ankle has turned black");
        assert!(!not_a_body(&reading));
        let reading = Reading::new("my bathroom wall has turned black");
        assert!(not_a_body(&reading));
        assert_eq!(
            mould(&Context::new(&reading, &[])),
            Some(MOULD_BATHROOM_ADVICE)
        );
    }

    #[test]
    fn smoke_alarm_routes_unknown_cause_before_maintenance() {
        for (line, cause) in [
            ("smoke alarm went off for no reason", ALARM_UNKNOWN),
            ("smoke alarm goes off every time I shower", ALARM_BATHROOM),
            ("smoke alarm sounds when I burn toast", ALARM_COOKING),
        ] {
            let answer = alarm_nuisance(&Reading::new(line)).unwrap();
            assert!(answer.starts_with(FIRE_CHECK));
            assert!(answer.contains(cause));
            assert!(answer.contains(ALARM_CARE));
        }
    }
}
