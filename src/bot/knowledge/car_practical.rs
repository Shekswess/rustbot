//! Vehicle advice that chooses between typed frames or looks back for a bicycle.

use crate::bot::{Exchange, Reading, understanding::line::normalise};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

#[path = "car_practical_data.rs"]
mod data;

static CARS: LazyLock<RegexSet> =
    LazyLock::new(|| phrase_set(data::A_CAR_ONLY.iter().chain(data::CAR_MODELS).copied()));
static ROOMS: LazyLock<RegexSet> =
    LazyLock::new(|| phrase_set(data::A_HOUSE_SURFACE.iter().copied()));
static DRIVING_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:driv(?:e|es|er|ers|ing|en)|cars?|roads?|motorways?|dual carriageway|",
        r"roundabout|junctions?|headlights?|headlamps?|main beam|full beam|high beam|dipped|dip|dazzl\w*|",
        r"windscreen|wing mirror|mirrors?|lane|laning|traffic|vehicle|van|lorry|lorries|truck|",
        r"steer(?:ing)?|wheel|brakes?|braking|gears?|gearstick|clutch|accelerator|",
        r"park(?:ed|ing)?|revers(?:e|ing)|overtak\w*|tailgat\w*|aquaplan\w*|skid(?:s|ded|ding)?|",
        r"mot|dashboard|petrol|diesel|engine|bonnet|boot|tyres?|horn)\b"
    ))
    .expect("driving terms")
});
static CHILD_SEAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:seats?|isofix|cars?|child|children|kids?|toddler|baby|son|daughter|",
        r"height|135|law|legal|rear facing|backless|high[- ]backed|front seat|cushion|maxi)\w*"
    ))
    .expect("child seat terms")
});
static VEHICLE_TERM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:cars?|van|mot|motor|engine|vehicle|lorry|truck)\b")
        .expect("motor vehicle terms")
});
static BICYCLE_TERM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:bike|bicycle|cycle|pushbike)s?\b").expect("bicycle terms"));
static ABS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\babs\b").expect("ABS warning"));
static DASH_TERMS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:lights?|lit|warning|symbol|dash|dashboard|cars?|brake|brakes|braking|mot|",
        r"garage|wheels?|sensor|comes?|came|flashing|flashes|flickering)\b"
    ))
    .expect("dashboard terms")
});

#[derive(Clone, Copy, Debug)]
pub(super) enum Gate {
    BrakePrice,
    ChildSeat,
    Dashboard,
    NightDriving,
    RepairPrice,
}

fn phrase_set(phrases: impl IntoIterator<Item = &'static str>) -> RegexSet {
    RegexSet::new(
        phrases
            .into_iter()
            .map(|phrase| format!(" {} ", regex::escape(phrase))),
    )
    .expect("literal vehicle or room phrases")
}

pub(crate) fn car_damp(reading: &Reading<'_>) -> bool {
    let said = format!(" {} ", reading.line.text.to_lowercase());
    CARS.is_match(&said) || !ROOMS.is_match(&said)
}

fn contains_any(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

fn has_phrase(padded: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| {
        padded.match_indices(phrase).any(|(start, matched)| {
            padded[..start].ends_with(' ') && padded[start + matched.len()..].starts_with(' ')
        })
    })
}

fn names_garage(clean: &str) -> bool {
    let padded = format!(" {clean} ");
    has_phrase(&padded, &["car", "cars"])
        || has_phrase(&padded, data::CAR_MODELS)
        || has_phrase(&padded, data::OTHER_VEHICLES)
        || has_phrase(&padded, data::CAR_PARTS)
        || has_phrase(&padded, data::GARAGE_TRADE_EXTRA)
        || (has_phrase(&padded, data::CAR_WEAR) && !has_phrase(&padded, data::HOME_SURFACE))
}

fn names_house(clean: &str) -> bool {
    let padded = format!(" {clean} ");
    has_phrase(&padded, data::HOME_SURFACE) || has_phrase(&padded, data::HOUSE_TRADE_EXTRA)
}

impl Gate {
    pub(super) fn passes(self, reading: &Reading<'_>, earlier: &[Exchange]) -> bool {
        let clean = &reading.line.clean;
        let previous = || earlier.last().map(|turn| normalise(&turn.user).clean);
        match self {
            Self::BrakePrice => {
                VEHICLE_TERM.is_match(clean)
                    || !previous()
                        .is_some_and(|line| line.contains("cable") && BICYCLE_TERM.is_match(&line))
            }
            Self::ChildSeat => {
                CHILD_SEAT.is_match(clean)
                    || previous().is_some_and(|line| CHILD_SEAT.is_match(&line))
            }
            Self::Dashboard => !ABS.is_match(clean) || DASH_TERMS.is_match(clean),
            Self::NightDriving => {
                clean.is_empty()
                    || DRIVING_WORDS.is_match(clean)
                    || CARS.is_match(&format!(" {clean} "))
            }
            Self::RepairPrice => {
                if reading.topic.is_some_and(|(name, _)| name == "cars") || names_garage(clean) {
                    return true;
                }
                let lean = previous().unwrap_or_default();
                names_garage(&lean) || !(names_house(clean) || names_house(&lean))
            }
        }
    }
}

fn asked_frequency(clean: &str) -> bool {
    contains_any(
        clean,
        &[
            "how often",
            "how frequently",
            "how regularly",
            "how many times",
            "often",
            "frequently",
            "regularly",
            "how long between",
            "every how long",
            "frequency",
        ],
    )
}

// https://www.gov.uk/guidance/the-highway-code/annex-6-vehicle-maintenance-safety-and-security
fn tyres(clean: &str) -> Cow<'static, str> {
    const PRESSURE: &str = "Use the pressures specified for your model and load in the handbook or on the driver's door or fuel-flap label. Measure with a gauge while the tyres are cold.";
    const INTERVAL: &str = "Check tyre pressures weekly and before a long journey.";
    if asked_frequency(clean) {
        Cow::Owned(format!("{INTERVAL} {PRESSURE}"))
    } else {
        Cow::Owned(format!("{PRESSURE} {INTERVAL}"))
    }
}

// https://www.gov.uk/check-vehicle-safe
// https://www.volkswagen.co.uk/en/owners-and-services/my-car/warning-light/hybrid-engine-oil-pressure.html
// https://www.gov.uk/guidance/the-highway-code/annex-6-vehicle-maintenance-safety-and-security
fn oil(clean: &str, earlier: &[Exchange]) -> &'static str {
    const LEVEL: &str = "Park on level ground, switch off the engine and wait as directed by the handbook. Check the dipstick or electronic reading. If you need to top up, add the specified oil a little at a time and recheck without exceeding the maximum mark.";
    const INTERVAL: &str = "Follow the manufacturer's oil and filter change schedule for your model and driving conditions. Check whether the handbook gives a mileage limit, a time limit or an oil-life indicator.";
    const AMOUNT: &str = "Look up the exact oil capacity for your engine and whether the filter is being replaced. For a top-up, add only enough to correct the level. Recheck while adding small amounts to avoid overfilling.";
    if contains_any(
        clean,
        &[
            "oil pressure light",
            "oil pressure warning",
            "red oil light",
        ],
    ) {
        return "If the oil pressure warning comes on while driving, stop as soon as it is safe and switch the engine off. Check the handbook and call roadside assistance if the warning persists or the cause is unclear.";
    }
    for (words, reply) in [
        (
            data::OIL_COST_WORDS,
            "Ask for a quote for your exact model and engine. Check that it includes the correct oil, the filter, labour and VAT. An interim service may include extra checks.",
        ),
        (
            data::OIL_FILTER_WORDS,
            "Check the manufacturer's schedule for the oil and filter. Use the specified part and follow the fitting instructions, including any torque requirement.",
        ),
        (
            data::OIL_GRADE_WORDS,
            "Use the viscosity and manufacturer approval specified for your exact engine. Check both on the label. Two oils with the same viscosity can have different manufacturer approvals.",
        ),
        (
            data::OIL_JOB_WORDS,
            "Engine oil lubricates moving parts, helps carry heat away and holds contaminants that the filter can remove. The manufacturer specifies the grade to keep the oil film working across the engine's operating temperatures.",
        ),
        (
            data::OIL_OVERFILL_WORDS,
            "Recheck the oil level using the handbook procedure on level ground. If it remains above the maximum, ask a garage how to remove the excess before driving. If the engine is smoking or running roughly, stop safely and arrange help.",
        ),
        (data::OIL_AMOUNT_WORDS, AMOUNT),
    ] {
        if contains_any(clean, words) {
            return reply;
        }
    }
    if contains_any(clean, data::OIL_CHECK_WORDS) {
        if asked_frequency(clean) {
            return "Check the level at least weekly and before a long trip. Park on level ground, switch off the engine and wait as directed by the handbook. Use the dipstick or electronic reading, and add only enough of the specified oil to bring it within the marked range.";
        }
        return LEVEL;
    }
    if contains_any(clean, data::OIL_INTERVAL_WORDS) {
        return INTERVAL;
    }
    earlier
        .last()
        .and_then(|turn| {
            [INTERVAL, AMOUNT]
                .into_iter()
                .find(|answer| turn.bot.starts_with(answer))
        })
        .unwrap_or(LEVEL)
}

// https://www.volkswagen.co.uk/en/owners-and-services/my-car/warning-light.html
fn dashboard(clean: &str) -> &'static str {
    let padded = format!(" {clean} ");
    for (cues, reply) in [
        (
            &["brake pad", "pad wear", "wear indicator"][..],
            "A brake pad wear light calls for a prompt brake inspection. If braking feels abnormal, the pedal feels soft or a red brake warning is also on, stop safely and arrange recovery.",
        ),
        (
            &[
                "brake light",
                "brake warning",
                "handbrake",
                "parking brake",
                "brake fluid",
            ],
            "If a red brake warning stays on after you release the parking brake, or braking feels different, stop safely and arrange recovery. Check the symbol and instructions in the handbook.",
        ),
        (
            &["abs", "anti-lock", "anti lock"],
            "Check your handbook for the ABS warning. If there is also a red brake warning or braking feels wrong, stop safely and call for recovery. If the ABS warning is alone, arrange a prompt inspection and follow the handbook's driving instructions.",
        ),
        (
            &[
                "dpf",
                "particle filter",
                "diesel particle",
                "exhaust filter",
            ],
            "The diesel particulate filter warning can have different instructions by model. Use the handbook's regeneration procedure only if it applies to this warning and you can do it safely and legally. If other warnings appear or it stays on, seek advice from a garage or roadside assistance.",
        ),
        (
            &["airbag", "air bag", "srs"],
            "An airbag or SRS warning can mean part of the restraint system may fail in a crash. Check the handbook and arrange inspection promptly. A passenger airbag OFF indicator can also be intentional, depending on the seating setup.",
        ),
        (
            &["battery", "charging", "alternator"],
            "A charging-system warning can mean the vehicle may lose electrical power. Move somewhere safe, stop and contact roadside assistance. Keep the lights and other safety equipment you need while you get to safety.",
        ),
        (
            &[
                "tyre pressure",
                "tpms",
                "tyre light",
                "tire pressure",
                "tire light",
                "flat tyre",
                "flat tire",
            ],
            "Stop somewhere safe and look for a flat tyre or damage. Check cold pressures against the manufacturer's figures. If a tyre is flat or damaged, use roadside assistance unless you can safely follow the vehicle's approved repair procedure. A persistent warning can also be a sensor fault.",
        ),
        (
            &["power steering", "steering light"],
            "If steering has become heavy or difficult, stop safely and arrange recovery. Check the warning symbol and the instructions for your car in the handbook.",
        ),
        (
            &["traction", "esp", "esc", "skid light", "slippery"],
            "A traction or stability light may flash while the system is working. Reduce speed to suit conditions. If it stays on, check whether the system is switched off or has a fault and follow the handbook.",
        ),
        (
            &["glow plug", "glowplug", "preheat"],
            "A glow-plug light may appear briefly before a diesel engine starts. If it stays on or flashes while driving, check the handbook and arrange a diagnostic check.",
        ),
        (
            &["adblue", "ad blue", "urea light"],
            "Check whether the AdBlue message says the level is low or reports a fault. If low, use the product and amount specified in the handbook. Take any restart countdown seriously and seek help if a fault remains.",
        ),
        (
            &[
                "service light",
                "spanner",
                "service indicator",
                "service due",
            ],
            "Check the handbook for the exact service symbol. It may be a maintenance reminder or a fault warning, depending on the vehicle. Arrange the indicated service or diagnostic check.",
        ),
        (
            &[
                "washer fluid",
                "washer warning",
                "screen wash",
                "washer light",
            ],
            "Check the fluid level and top up with suitable screenwash as directed by the handbook. Clear visibility matters, so sort out a persistent warning or blocked washer before driving in conditions where you will need it.",
        ),
        (
            &["seatbelt", "seat belt", "belt light"],
            "Check that everyone is properly belted before setting off. Check for heavy items on an empty seat. If it stays on with all occupied seats correctly belted, check the handbook and arrange an inspection.",
        ),
        (
            &[
                "key symbol",
                "immobiliser",
                "immobilizer",
                "key light",
                "key warning",
            ],
            "Check what the key or immobiliser warning means for your model. Follow the handbook's procedure and try the spare key if it recommends doing so. Ask the manufacturer or roadside assistance if the vehicle will not start.",
        ),
    ] {
        if has_phrase(&padded, cues) {
            return reply;
        }
    }
    "Identify the warning symbol, colour and any message using your vehicle handbook. If a red warning stays on while driving, or braking or steering changes, stop safely and get assistance."
}

pub(crate) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let line = &reading.line;
    let said = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    let reply = match id {
        "tyre-price" => {
            let prior = earlier[earlier.len().saturating_sub(4)..]
                .iter()
                .map(|turn| turn.user.as_str())
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase();
            if data::A_BICYCLE.iter().any(|word| prior.contains(word)) {
                return None;
            }
            data::TYRE_PRICE
        }
        "repair-or-scrap" => return Some(repair_value(reading)),
        "tyre-pressure" => return Some(tyres(said)),
        "repair-cost" => {
            "Ask the garage for an itemised quote for your exact car and fault. Check the labour rate, estimated hours, parts, diagnostic charge and VAT. Ask them to call before doing extra work and compare a second quote for a significant repair."
        }
        "driving-at-night" => {
            "Use dipped headlights in built-up areas at night and switch from main beam before it can dazzle an oncoming driver, someone ahead or another road user. Drive at a speed that lets you stop within the distance you can see. Keep the glass clean. If headlights dazzle you, slow down and stop safely if you cannot see well enough to continue."
        }
        "child-seats" => {
            "In the UK, children normally need an approved child seat or booster until they turn 12 or reach 135 cm, whichever happens first. Then they use a seat belt. A height-based seat must face rearwards until the child is over 15 months old. Follow the seat's labelled height or weight limits and fitting instructions. Disable the front airbag before fitting a rear-facing seat in the front."
        }
        "oil-check" => oil(said, earlier),
        "dash-light" => dashboard(said),
        "brakes-cost" => {
            if contains_any(
                said,
                &[
                    "last",
                    "mileage",
                    "life",
                    "how often",
                    "how long",
                    "how many miles",
                ],
            ) {
                "Brake pad life varies with the car, load and driving. Use periodic inspections and the manufacturer's wear limit to decide when to replace them. If you get a wear warning or hear grinding, arrange a prompt inspection. If braking is impaired or the pedal feels soft, stop safely and arrange recovery."
            } else {
                "Ask for a fitted quote for your registration and the affected axle. Check whether it includes pads, discs if needed, fitting and VAT. The price depends on the vehicle and which parts are worn."
            }
        }
        "stalling" => {
            if earlier
                .iter()
                .rev()
                .take(6)
                .any(|turn| contains_any(&turn.user.to_lowercase(), data::LEARNING))
            {
                return None;
            }
            "If the engine repeatedly cuts out without a driver error, arrange an inspection before driving it again. If it stalls in traffic and you cannot clear the road safely, use hazard lights where appropriate and call for assistance from a safe position."
        }
        "clicking-on-a-turn" => {
            let said = if said.is_empty() { &line.plain } else { said };
            let said = said.to_lowercase();
            if data::A_DEFINITION_ASK
                .iter()
                .any(|shape| said.contains(shape))
            {
                data::CV_JOINT_DEFINITION
            } else {
                data::THE_CLICK_ON_A_TURN_REPLY
            }
        }
        _ => {
            let reply = match id {
                "clutch-biting-point" => data::CLUTCH_BITING_POINT,
                "change-gear-smoothly" => data::CHANGE_GEAR_SMOOTHLY,
                "three-point-turn" => data::THREE_POINT_TURN,
                "driving-in-the-dark" => data::DRIVING_IN_THE_DARK,
                _ => return None,
            };
            let said = said.to_lowercase();
            if data::WRONG_WITH_IT.iter().any(|shape| said.contains(shape)) {
                return None;
            }
            reply
        }
    };
    Some(Cow::Borrowed(reply))
}

fn repair_value(reading: &Reading<'_>) -> Cow<'static, str> {
    let mut figures = reading
        .numbers
        .iter()
        .map(|number| number.value)
        .filter(|amount| amount.is_finite() && *amount >= 150.0);
    let Some(figure) = figures.next() else {
        return Cow::Borrowed(data::REPAIR_OR_SCRAP);
    };
    if figures.next().is_some() {
        return Cow::Borrowed(data::REPAIR_OR_SCRAP);
    }
    let figure = format!("{:.0}", figure.trunc());
    let mut twice = Vec::with_capacity(figure.len() + 1);
    let mut carry = 0;
    for digit in figure.bytes().rev() {
        let next = (digit - b'0') * 2 + carry;
        twice.push(b'0' + next % 10);
        carry = next / 10;
    }
    if carry != 0 {
        twice.push(b'0' + carry);
    }
    twice.reverse();
    let value = with_thousands(figure.as_bytes());
    let twice = with_thousands(&twice);
    Cow::Owned(format!(
        "£{value} of work is the number to judge it by: if the car would sell for less than about £{twice}, the repair is the bigger number, so let it go. Scrap value is only £150 to £400, and a 'dangerous' MOT item like brakes, steering or rust near a mounting is not worth fixing on a cheap car; tyres, a clutch or a cambelt on an otherwise sound car usually are."
    ))
}

fn with_thousands(digits: &[u8]) -> String {
    let mut formatted = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.iter().enumerate() {
        if index != 0 && (digits.len() - index).is_multiple_of(3) {
            formatted.push(',');
        }
        formatted.push(char::from(*digit));
    }
    formatted
}

#[cfg(test)]
#[path = "../../../tests/fixtures/car_practical.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn vehicle_responses_and_damp_guard_match_pinned_pybot() {
        for fixtures::Case(id, prompt, history, guard, reply) in fixtures::CASES {
            let reading = Reading::new(prompt);
            if let Some(guard) = guard {
                assert_eq!(car_damp(&reading), *guard, "{id}: {prompt}");
            } else {
                let earlier: Vec<_> = history
                    .iter()
                    .map(|line| Exchange {
                        user: (*line).to_owned(),
                        bot: String::new(),
                        module: "answers",
                    })
                    .collect();
                assert_eq!(
                    registered(id, &reading, &earlier).as_deref(),
                    *reply,
                    "{id}: {prompt}"
                );
            }
        }
    }

    #[test]
    fn registers_scoped_vehicle_safety_advice() {
        for (prompt, expected) in [
            (
                "The battery warning light came on in my car",
                "contact roadside assistance",
            ),
            (
                "How often should I check my car tyre pressure?",
                "Check tyre pressures weekly",
            ),
            (
                "At what age does my child no longer need a car seat?",
                "135 cm",
            ),
        ] {
            let mut bot = Bot::new(Some("0")).unwrap();
            let turn = bot.turn(prompt);
            assert_eq!(turn.module, Some("answers"), "{prompt}: {turn:?}");
            assert!(turn.reply.contains(expected), "{prompt}: {}", turn.reply);
        }
        assert!(!Gate::Dashboard.passes(&Reading::new("What are abs?"), &[]));
        assert!(!Gate::ChildSeat.passes(&Reading::new("When do I need a booster?"), &[]));
        let boiler = [Exchange {
            user: "Is boiler cover worth it?".into(),
            bot: String::new(),
            module: "answers",
        }];
        assert!(!Gate::RepairPrice.passes(&Reading::new("How much does a repair cost?"), &boiler));
    }
}
