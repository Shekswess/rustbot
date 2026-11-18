//! Baby equipment and room-layout planning.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::{Exchange, Reading};

use super::super::history_cards::whole_words;

static ROOM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:kitchens?|bathrooms?|baths?|showers?|toilets?|loos?|ensuite|en suite|wet room|",
        r"utility|laundry|boot room|small room|spare room|dining room)\b"
    ))
    .expect("rooms with plans")
});
static BATHROOM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:bathrooms?|baths?|showers?|toilets?|loos?|ensuite|en suite|wet room)\b")
        .expect("bathroom terms")
});
static UTILITY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:utility|laundry|boot room)\b").expect("utility room terms"));

// https://www.worcester-bosch.co.uk/support/literature/download/release/6720813278/12691
pub(super) fn room(reading: &Reading<'_>) -> Option<&'static str> {
    let clean = &reading.line.clean;
    if UTILITY.is_match(clean) {
        Some(
            "Start with appliance dimensions, plumbing, ventilation and room for doors to open. Leave service access and working space at the appliances. If there is a gas boiler, have a Gas Safe registered engineer confirm its model's clearance, ventilation and access requirements before you design any enclosure. Keep vents and flue access clear.",
        )
    } else if BATHROOM.is_match(clean) {
        Some(
            "Mark the toilet's existing soil pipe and the door swing first. Place the bath or shower and leave clear access to the basin and toilet. Check the exact fixture dimensions, drainage falls, safe electrical zones and ventilation with the installer before ordering.",
        )
    } else if ROOM.is_match(clean) {
        Some(
            "Start with the sink, hob and fridge, leaving clear routes between the work areas and safe landing space beside the hob. Mark pipes, power points, door swings and the space needed to open appliances and drawers. Tape out the layout and walk through how you would use it before ordering.",
        )
    } else {
        None
    }
}

const GEAR: &[(&[&str], &str)] = &[
    (
        &[
            "pram",
            "prams",
            "pushchair",
            "pushchairs",
            "buggy",
            "buggies",
            "stroller",
            "strollers",
            "travel system",
        ],
        "Choose a pram that fits your space and has a lie-flat configuration approved for newborns if needed. For a used pram, check the frame, folding locks, brakes, harness and recall status. Compare the price including any parts you need.",
    ),
    (
        &[
            "carrier",
            "carriers",
            "sling",
            "slings",
            "baby carrier",
            "baby sling",
        ],
        "A carrier leaves your hands free. Choose one for the baby's size and make sure the face stays visible, the chin stays off the chest and the back is supported. Follow the NHS TICKS guidance.",
    ),
    (
        &[
            "cot",
            "cots",
            "cot bed",
            "cotbed",
            "crib",
            "cribs",
            "moses basket",
            "travel cot",
        ],
        "Choose a suitable cot, crib or Moses basket with a firm, flat, waterproof mattress that fits without gaps. The Lullaby Trust recommends a new mattress if possible. Check a used cot for damage and confirm that it has all its parts and instructions.",
    ),
    (
        &[
            "car seat",
            "car seats",
            "carseat",
            "carseats",
            "i size",
            "isofix",
            "infant carrier",
        ],
        "Choose an approved seat that fits both the child and your vehicle. Buy new if you cannot verify a used seat's full history. It must have all the required parts and instructions and should never have been in a crash. Check whether it has been recalled.",
    ),
    (
        &["high chair", "highchair", "high chairs"],
        "Choose a stable high chair when the baby can sit upright for feeding, typically around six months. Check the harness and inspect any used chair for damage and missing parts.",
    ),
    (
        &["baby monitor", "monitor", "monitors"],
        "A baby monitor is optional. Place the device and its cables well out of reach of the cot, and follow the manufacturer's mounting instructions.",
    ),
    (
        &["bouncy chair", "bouncer"],
        "Choose a bouncer appropriate for the baby's size and use it on the floor while the baby is awake and supervised. Use a firm, flat sleep space if the baby falls asleep.",
    ),
];

static GEAR_NAMES: LazyLock<Vec<RegexSet>> =
    LazyLock::new(|| GEAR.iter().map(|(words, _)| whole_words(words)).collect());
static FAULT: LazyLock<RegexSet> = LazyLock::new(|| {
    whole_words(&[
        "squeak",
        "squeaks",
        "squeaking",
        "squeaky",
        "sticking",
        "stuck",
        "jammed",
        "jam",
        "wobbly",
        "loose",
        "broken",
        "broke",
        "snapped",
        "wont fold",
        "wont push",
        "dragging",
        "rusty",
        "rust",
        "cracked",
        "torn",
        "rips",
        "ripped",
        "fraying",
        "buckle",
        "buckles",
        "harness",
        "brake",
        "brakes",
        "wheel",
        "wheels",
        "axle",
        "axles",
        "handle",
    ])
});
static COMPARISON: LazyLock<RegexSet> =
    LazyLock::new(|| whole_words(&["which", "better", "or", "versus", "vs"]));

// https://www.nhs.uk/baby/caring-for-a-newborn/what-you-will-need-for-your-baby/
// https://www.lullabytrust.org.uk/baby-safety/baby-product-information/second-hand-baby-products/
pub(super) fn baby_gear(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    let fault = FAULT.is_match(clean);
    let matching = |line: &str| {
        GEAR_NAMES
            .iter()
            .enumerate()
            .filter_map(|(index, names)| names.is_match(line).then_some(GEAR[index].1))
            .take(2)
            .collect::<Vec<_>>()
    };
    let mut matches = matching(clean);
    if fault
        && matches.is_empty()
        && let Some(turn) = earlier.last()
    {
        matches = matching(&turn.user.to_lowercase());
    }
    if fault && !matches.is_empty() {
        return Some(Cow::Borrowed(
            "Stop using the item if its brake, harness, frame or locking mechanism is faulty. Check the manufacturer instructions and recall notices, and use a manufacturer-approved repair or replacement part before putting it back into use.",
        ));
    }
    Some(match matches.as_slice() {
        [first, second] if COMPARISON.is_match(clean) => Cow::Owned(format!("{first} {second}")),
        [one] => Cow::Borrowed(one),
        _ => Cow::Borrowed(
            "Start with a suitable flat sleep space and mattress, clothing and nappies, and an approved car seat if you travel by car. Choose a pram or carrier that fits your needs. Check second-hand items for recalls, damage and complete instructions before using them.",
        ),
    })
}

pub(super) fn asks_for_gear(reading: &Reading<'_>) -> bool {
    let line = &reading.line.clean;
    !["car seat", "carseat", "booster seat"]
        .iter()
        .any(|seat| line.contains(seat))
        || ![
            "what age",
            "how old",
            "no longer need",
            "stop using",
            "law",
            "legal",
            "until when",
        ]
        .iter()
        .any(|ask| line.contains(ask))
}
