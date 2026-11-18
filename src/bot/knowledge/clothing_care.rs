//! Fabric drying, wash settings, pressing and waterproof-care eligibility.
#[path = "clothing_care_data.rs"]
mod data;
#[cfg(test)]
#[path = "clothing_care_tests.rs"]
mod tests;

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    NoIron,
    Dry,
    WashTemperature,
    Mould,
    EngineOil,
    Blister,
    Standing,
    HairColour,
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(NO_IRON, NO_BOARD, NEEDS_SIXTY);

macro_rules! source_pattern {
    ($name:ident, $source:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(super::$source).expect(stringify!($name)));
    };
}
source_pattern!(HEIGHT, CLOTHING_HEIGHT);
source_pattern!(ON_FEET, CLOTHING_ON_FEET);
source_pattern!(BURN_BEFORE, CLOTHING_BURN_BEFORE);
source_pattern!(BLISTER_SORE, CLOTHING_BLISTER_SORE);
source_pattern!(PREVENT_BLISTER, CLOTHING_PREVENT_BLISTER);
source_pattern!(RUBBING, CLOTHING_RUBBING);
source_pattern!(BREAKING_IN, CLOTHING_BREAKING_IN);
source_pattern!(BLISTER_NAMED, CLOTHING_BLISTER_NAMED);

static MOULD_HIDE: LazyLock<Regex> = LazyLock::new(|| {
    let alternatives = super::CLOTHING_MOULD_SOFT
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"\b(?:{alternatives})\b")).expect("leather and suede words")
});
static BLISTER_CARE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?i)\b(?:pop|burst|drain|bust)\b|\bstop\s+(?:it|them|the\s+blister)\s+(?:from\s+)?(?:hurting|aching|pain)\b"
).expect("blister treatment questions")
});

fn line_views(reading: &Reading<'_>) -> String {
    format!(
        "{} {} {}",
        reading.line.clean, reading.line.norm_text, reading.line.text
    )
    .to_lowercase()
}

fn mentions(line: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| line.contains(term))
}

fn no_iron(reading: &Reading<'_>) -> Option<&'static str> {
    let line = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    if NO_IRON.is_match(line) {
        Some(data::NO_IRON_TEXT)
    } else if NO_BOARD.is_match(line) {
        Some(data::NO_BOARD_TEXT)
    } else {
        None
    }
}

pub(super) fn drying_laundry(reading: &Reading<'_>) -> bool {
    let line = if !reading.line.words.is_empty() {
        reading.line.words.join(" ").to_lowercase()
    } else if !reading.line.clean.is_empty() {
        reading.line.clean.to_lowercase()
    } else {
        reading.line.text.to_lowercase()
    };
    mentions(&line, data::LAUNDRY_WORDS) || !mentions(&line, data::BUILDING_WORDS)
}

fn dryer_fault(words: &str) -> bool {
    mentions(words, data::A_DRYER)
        && !mentions(words, &["hair dryer", "hairdryer"])
        && mentions(words, data::A_JOB_LEFT_UNDONE)
}

pub(super) fn dryer_heat(reading: &Reading<'_>) -> &'static str {
    if reading
        .line
        .words
        .iter()
        .any(|word| data::DRYER_SLOW_WORDS.binary_search(&word.as_str()).is_ok())
    {
        data::DRYER_SLOW
    } else {
        data::DRYER_COLD
    }
}

pub(super) fn job_left_undone(line: &str) -> bool {
    data::A_JOB_LEFT_UNDONE
        .iter()
        .any(|phrase| line.contains(phrase))
}

fn drying(reading: &Reading<'_>) -> &'static str {
    let words = reading.line.words.join(" ").to_lowercase();
    let down = mentions(&words, data::DRY_DOWN_WORDS)
        || (words.split_whitespace().any(|word| word == "down")
            && mentions(&words, &["jacket", "coat", "gilet", "bag"]));
    if dryer_fault(&words) {
        dryer_heat(reading)
    } else if down {
        data::DRY_DOWN
    } else if mentions(&words, data::DRY_HANGER_WORDS) {
        data::DRY_HANGER
    } else if mentions(&words, data::DRY_FLAT_WORDS) {
        data::DRY_FLAT
    } else if mentions(&words, data::DRY_FLUFFY_WORDS) {
        data::DRY_FLUFFY
    } else {
        data::DRY_INSIDE
    }
}

fn wash_temperature(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let prior = earlier
        .iter()
        .map(|turn| turn.user.as_str())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let line = line_views(reading);
    if mentions(&prior, data::A_FAULTY_MACHINE)
        || mentions(
            &line,
            &[
                "towel rail",
                "towel radiator",
                "heated towel rail",
                "ladder rail",
                "radiator",
                "radiators",
            ],
        )
    {
        return None;
    }
    if mentions(&line, &["90", "95"]) {
        Some(data::WASH_HIGH)
    } else if line.contains("60") {
        Some(data::WASH_SIXTY)
    } else if !mentions(&line, &["30", "40", "60", "90", "95"]) && NEEDS_SIXTY.is_match(&line) {
        Some(data::WASH_BEDDING)
    } else {
        Some(data::WASH_NORMAL)
    }
}

pub(super) fn faulty_coat(reading: &Reading<'_>) -> bool {
    let line = line_views(reading);
    mentions(&line, data::COAT_FAULT) || !mentions(&line, data::COAT_BUYING)
}

pub(super) fn mould_is_on_fabric(reading: &Reading<'_>) -> bool {
    const CONTAINERS_AND_MATERIALS: &[&str] = &[
        "bag",
        "bags",
        "handbag",
        "handbags",
        "drawer",
        "drawers",
        "cupboard",
        "cupboards",
        "leather",
        "suede",
        "fabric",
    ];
    !(super::guards::food_is_being_kept(reading)
        && super::guards::only_a_container(reading, CONTAINERS_AND_MATERIALS))
}

pub(super) fn feet_not_height(reading: &Reading<'_>) -> bool {
    let line = format!(
        "{} {} {} {}",
        line_views(reading),
        reading.raw,
        reading.line.plain,
        reading.line.words.join(" ")
    )
    .to_lowercase();
    !HEIGHT.is_match(&line) || ON_FEET.is_match(&line)
}

fn mould(reading: &Reading<'_>) -> &'static str {
    let line = &reading.line;
    let text = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    let regular = text
        .to_ascii_lowercase()
        .replace(|ch: char| !ch.is_ascii_alphanumeric() && ch != '\'', " ");
    if MOULD_HIDE.is_match(&regular) {
        "For leather or suede, check the manufacturer's care instructions or ask a leather cleaner. Keep the item dry and away from direct heat. Treat a small patch only with a cleaner approved for that finish, and get specialist advice if mould reaches the padding or lining."
    } else {
        "Wash the item with detergent at the temperature on its care label, or have it dry-cleaned if the label calls for it. Avoid brushing mould indoors. Dry the item fully and deal with any damp where it was stored. Ask a cleaner about padding or material that cannot be washed."
    }
}

fn engine_oil(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let line = &reading.line;
    let typed = format!("{} {} {}", line.clean, line.norm.join(" "), line.plain);
    if !mentions(&typed, super::CLOTHING_STAIN_SAID)
        && !earlier
            .iter()
            .any(|turn| mentions(&turn.user.to_lowercase(), super::CLOTHING_STAIN_SAID))
    {
        return None;
    }
    Some(
        "Blot up excess oil and check the garment's care label. For a small mark on washable material, use a laundry pretreatment suitable for grease. Heavily oil-soaked clothes or anything treated with a flammable solvent need specialist cleaning or disposal advice. Keep them out of the tumble dryer.",
    )
}

fn blister(reading: &Reading<'_>, earlier: &[Exchange]) -> &'static str {
    let line = &reading.line;
    let text = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    let prior = earlier.iter().rev().take(2);
    if BURN_BEFORE.is_match(text) || prior.clone().any(|turn| BURN_BEFORE.is_match(&turn.user)) {
        return "For a recent burn or scald, cool it under running cool water for 20 minutes. Leave the blister intact. Ask NHS 111 for advice on a burn blister. Call 999 or go to A&E for a large or deep burn, one on the face, genitals or bottom, or a chemical or electrical burn.";
    }
    if BLISTER_SORE.is_match(text) || prior.clone().any(|turn| BLISTER_SORE.is_match(&turn.user)) {
        return "Gently wash the blister and cover it with a clean padded dressing. Leave the skin intact. If it looks infected, feels very painful or keeps coming back, contact NHS 111.";
    }
    if BLISTER_CARE.is_match(text) {
        return "Let the blister heal without popping it. Keep it clean and covered with a soft plaster or padded dressing. Leave the shoes that caused it off until it heals. A pharmacist can help you find a blister dressing.";
    }
    if PREVENT_BLISTER.is_match(text)
        || ((RUBBING.is_match(text) || BREAKING_IN.is_match(text)) && !BLISTER_NAMED.is_match(text))
    {
        return "Check that the shoes fit well and have enough room where they rub. Wear new shoes for short stretches at first with comfortable socks, and change shoes when you feel a hot spot forming.";
    }
    "Wash the blister gently and cover it with a padded dressing. Leave it intact and give the skin time to heal before wearing the shoes again."
}

fn hair_colour(reading: &Reading<'_>) -> Cow<'static, str> {
    let line = line_views(reading);
    if mentions(
        &line,
        &[
            "how long",
            "how many minutes",
            "leave it on",
            "leave on for",
        ],
    ) && !mentions(
        &line,
        &[
            "last", "lasts", "keep", "keeps", "shelf", "expire", "go off",
        ],
    ) {
        return Cow::Borrowed(
            "Use the timings on the dye's instructions, which vary by product. Do the recommended allergy and strand tests in advance, and rinse at the stated time. Stop if the colour causes burning, itching or other signs of a reaction.",
        );
    }
    let opener = if mentions(
        &line,
        &["grey", "gray", "greys", "grays", "white hair", "going grey"],
    ) {
        "Choose a colour labelled for grey coverage. A permanent dye can give more coverage and a semi-permanent colour can blend it. Test on a strand to see the result."
    } else if mentions(&line, super::CLOTHING_HAIR_DARK_WORDS)
        && !mentions(&line, &["lighter", "blonde", "blond", "bleach", "lift"])
    {
        "Going darker changes the colour without lightening it first. Compare the shade guide with your current hair and test a strand, since porous ends can come out darker."
    } else {
        "Check the product's shade guide for the change it can make on your starting colour. Ask a colourist about going much lighter, especially on already-dyed or damaged hair."
    };
    Cow::Owned(format!(
        "{opener} Wear gloves, follow the product instructions, and do its allergy and strand tests before using it. Stop and wash the dye off if you get irritation. Call 999 if you have trouble breathing or your lips, mouth, throat or tongue suddenly swell."
    ))
}

pub(super) fn reply(
    form: Reply,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let response = match form {
        Reply::NoIron => no_iron(reading)?,
        Reply::Dry => drying(reading),
        Reply::WashTemperature => wash_temperature(reading, earlier)?,
        Reply::Mould => mould(reading),
        Reply::EngineOil => engine_oil(reading, earlier)?,
        Reply::Blister => blister(reading, earlier),
        Reply::Standing => {
            "Break up standing with short seated breaks and changes of position when the job allows. Wear well-fitting, supportive shoes and ask about a secure anti-fatigue mat or a seat at the workstation. Get persistent or worsening foot pain checked."
        }
        Reply::HairColour => return Some(hair_colour(reading)),
    };
    Some(Cow::Borrowed(response))
}
