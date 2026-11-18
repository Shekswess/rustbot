//! Bin, lens and stain readers matching the authored cleaning card keys.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::{Exchange, Reading};

use super::has_space_delimited_phrase;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(LOAF_WENT_OFF, super::CLEANING_STALE_LOAF);
pattern!(BIN_VERB, super::CLEANING_BIN_VERB);
pattern!(PET_NAMED, super::CLEANING_PET_NAMED);
pattern!(PET_ACCIDENT, super::CLEANING_PET_ACCIDENT);
pattern!(PLACE_TO_WASH, super::CLEANING_PLACE_TO_WASH);
pattern!(OFF_PHRASE, super::CLEANING_OFF_PHRASE);
pattern!(PRONOUN_BEFORE_OFF, super::CLEANING_PRONOUN_BEFORE_OFF);
pattern!(FOOD_TOKENS, "[a-z']+");

const SURFACE_CLEANING: &str = "Blot the mark with a clean white cloth, working inward from the edge. Follow the material's care label. If it allows water-based cleaning, use a barely damp cloth and a little diluted mild detergent. Test in a hidden spot first, and blot dry without soaking the area.";

fn raw_line(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    (if line.text.is_empty() {
        &line.clean
    } else {
        &line.text
    })
    .to_lowercase()
}

fn contains(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

pub(super) fn bin_is_the_subject(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let said = format!("{} {} {}", line.clean, line.norm_text, line.plain).to_lowercase();
    if LOAF_WENT_OFF.is_match(&said) {
        return false;
    }
    !BIN_VERB.is_match(&said)
        || !FOOD_TOKENS
            .find_iter(&said)
            .any(|token| super::FOOD_NAMED.contains(&token.as_str()))
}

pub(super) fn bin_smell(_reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(
        "Empty the bin, wash it with warm soapy water, rinse it and let it dry before putting a new liner in. Take food waste out regularly. If you use a disinfectant, follow its label and keep it separate from other cleaners.",
    ))
}

pub(super) fn spectacles_fog(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = if reading.features.asks.contains(&"why") {
        "Warm, damp air condenses when it touches a cooler lens. A lens-safe anti-fog product can help. Check the cleaner works with your lens coating and follow its instructions. If a mask is sending breath up at the lenses, adjust the nose bridge for a closer fit."
    } else {
        "Try an anti-fog product suitable for your lens coating, applied with the recommended cloth. If a mask sends breath up at the lenses, adjust the nose bridge for a closer fit. Fog forms when warm, damp air touches the cooler lens."
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn the_place_a_pet_used(
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let said = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    if !PLACE_TO_WASH.is_match(said)
        || !earlier
            .iter()
            .rev()
            .take(3)
            .any(|turn| PET_NAMED.is_match(&turn.user) && PET_ACCIDENT.is_match(&turn.user))
    {
        return None;
    }
    Some(Cow::Borrowed(
        "Blot up what you can, then use a pet-safe enzyme cleaner suitable for the surface. Test it in a hidden spot first and follow the label. Keep the pet away until the patch is dry. Avoid ammonia cleaners for urine and keep bleach separate from other cleaners.",
    ))
}

static SOFT_MARKS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(super::CLEANING_SOFT_STAINS.iter().map(|(words, _)| {
        words
            .iter()
            .map(|word| {
                let escaped = regex::escape(word);
                if word.contains(' ') {
                    escaped
                } else {
                    format!(r"\b{escaped}\b")
                }
            })
            .collect::<Vec<_>>()
            .join("|")
    }))
    .expect("soft surface marks")
});

static NAMED_MARK: LazyLock<Regex> = LazyLock::new(|| {
    let more = [
        "stain",
        "stains",
        "stained",
        "smudge",
        "smudges",
        "smudged",
        "splodge",
        "splodges",
        "splat",
        "blob",
        "blobs",
        "patch",
        "patches",
        "dirt",
        "grime",
        "filth",
        "soot",
        "ash",
        "residue",
        "scum",
        "spill",
        "spills",
        "spilled",
        "spilt",
        "mess",
        "crumb",
        "crumbs",
        "dust",
        "scuff",
        "scuffs",
        "fingerprint",
        "fingerprints",
        "mark",
        "marks",
        "marked",
        "bolognaise",
        "lasagne",
        "lasagna",
        "beetroot",
        "blueberry",
        "raspberry",
        "strawberry",
        "chocolate",
        "cheese",
        "sauce",
        "chips",
        "butter",
        "milk",
        "cola",
        "coke",
        "red bull",
        "suncream",
        "sun cream",
        "sunscreen",
        "fake tan",
        "hair oil",
        "baby oil",
        "engine oil",
        "white spirit",
        "primer",
        "undercoat",
        "gloss",
        "emulsion",
        "wood stain",
        "superglue",
        "blu tack",
        "bluetack",
        "plasticine",
        "playdough",
        "play dough",
        "slime",
        "silly putty",
    ];
    let alternatives = super::CLEANING_STAIN_HAS_CARD
        .iter()
        .copied()
        .chain(
            super::CLEANING_SOFT_STAINS
                .iter()
                .flat_map(|(words, _)| words.iter().copied()),
        )
        .chain(more)
        .map(regex::escape)
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"\b(?:{alternatives})\b")).expect("named marks")
});

pub(super) fn a_mark_on_a_soft_surface(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = raw_line(reading);
    let reply = SOFT_MARKS
        .matches(&text)
        .iter()
        .next()
        .map_or(SURFACE_CLEANING, |index| {
            super::CLEANING_SOFT_STAINS[index].1
        });
    Some(Cow::Borrowed(reply))
}

pub(super) fn says_how_it_comes_off(reading: &Reading<'_>) -> bool {
    let text = raw_line(reading);
    contains(&text, super::CLEANING_COMES_OFF)
        || (OFF_PHRASE.is_match(&text)
            && (PRONOUN_BEFORE_OFF.is_match(&text) || NAMED_MARK.is_match(&text)))
        || !super::diet_plan::is_not_a_mark(reading)
}

pub(super) fn a_mark_on_the_person(reading: &Reading<'_>) -> bool {
    let text = raw_line(reading);
    let named = |terms: &[&str]| {
        terms
            .iter()
            .any(|term| has_space_delimited_phrase(&text, term))
    };
    !named(super::CLEANING_NOT_SKIN)
        && named(super::CLEANING_MARK_ON_PERSON)
        && contains(&text, super::CLEANING_MARK_WORDS)
}

pub(super) fn stain_nobody_taught_it(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = raw_line(reading);
    let skin = contains(
        &text,
        &[
            "fingers", "finger", "hands", "hand", "skin", "arms", "arm", "face", "thumbs", "thumb",
        ],
    );
    let hard = contains(
        &text,
        &[
            "worktop",
            "work top",
            "counter",
            "countertop",
            "table",
            "floor",
            "floors",
            "wall",
            "walls",
            "slab",
            "slabs",
            "paving",
            "pavement",
            "path",
            "drive",
            "driveway",
            "patio",
            "decking",
            "brick",
            "bricks",
            "concrete",
            "render",
            "tile",
            "tiles",
            "grout",
            "bath",
            "sink",
            "basin",
            "toilet",
            "hob",
            "glass",
            "mirror",
            "window",
            "windows",
            "fridge",
            "cupboard",
            "door",
            "car",
            "car seat",
        ],
    );
    if skin
        && !hard
        && super::CLEANING_SURFACE_STATE
            .iter()
            .any(|word| has_space_delimited_phrase(&text, word))
    {
        return None;
    }
    let reply = if contains(&text, &["chewing gum", "chewed gum", "gum", "bubble gum"]) {
        if skin {
            "Hold a cool wrapped pack against the gum until it firms up, then gently work it free with a little cooking oil. Wash with mild soap afterwards. Stop if it pulls at the skin."
        } else {
            "Harden the gum with an ice cube in a bag, then lift it carefully with a blunt plastic edge. For the residue, use a cleaner the care label allows. Test it on a hidden spot first."
        }
    } else if contains(
        &text,
        &[
            "superglue",
            "super glue",
            "glue",
            "glued",
            "adhesive",
            "resin",
            "silicone",
            "sealant",
            "blu tack",
            "blu-tack",
            "blutack",
            "putty",
            "plasticine",
            "play dough",
            "playdough",
        ],
    ) {
        if skin
            && contains(
                &text,
                &[
                    "silicone",
                    "sealant",
                    "putty",
                    "play dough",
                    "playdough",
                    "plasticine",
                ],
            )
        {
            "Check the product label for skin cleanup. Wash the residue off gently with mild soap and water if the label allows it. Avoid scraping your skin. Get advice if it keeps stinging or the skin is damaged."
        } else if skin {
            "For superglue on intact skin, soak the area in warm soapy water until it softens, then gently work the skin apart. Stop if it hurts. Get medical advice if the bond will not release or the glue is near an eye or mouth."
        } else {
            "Check the adhesive's label and the material's care instructions. Lift hardened residue gently with a blunt plastic edge if the surface permits it. Test any approved remover out of sight before using it on the mark."
        }
    } else if contains(
        &text,
        &[
            "tar",
            "tarmac",
            "bitumen",
            "creosote",
            "resin",
            "paint",
            "varnish",
            "dye",
            "hair dye",
            "henna",
            "wood stain",
            "rust",
            "oil paint",
            "graffiti",
        ],
    ) {
        if skin {
            "Follow the product's skin-contact instructions and rinse away ordinary residue with mild soap and water. Avoid solvents and scrubbing. If the product can cause chemical burns or the skin is burning, get emergency help and follow the product's first-aid instructions."
        } else if hard {
            "Look up the cleaner approved for both the stain and the surface. Test it in a hidden spot, then dab gently with a cloth. Ventilate if the label says to and keep solvents away from heat or flame."
        } else {
            "Check the fabric care label for a remover suitable for that mark. Test it in a hidden area. For delicate fabric or a stubborn paint or tar mark, take the garment to a cleaner and tell them what caused it."
        }
    } else if skin
        && contains(
            &text,
            &[
                "sauce",
                "tomato",
                "ketchup",
                "curry",
                "juice",
                "coffee",
                "tea",
                "wine",
                "gravy",
                "soup",
                "chocolate",
                "jam",
            ],
        )
    {
        "Wash the residue off gently with running water and mild soap. If a food stain leaves a faint tint, give it time to fade."
    } else if contains(
        &text,
        &[
            "grease",
            "greasy",
            "oil",
            "butter",
            "fat",
            "sauce",
            "sunscreen",
            "sun cream",
            "suncream",
            "cream",
            "lotion",
            "lipstick",
            "makeup",
            "make-up",
            "mayonnaise",
            "dressing",
        ],
    ) {
        if skin {
            "Wash with warm water and mild soap, working it in gently with your fingertips. Rinse and dry your hands. Check the product's first-aid label if this was a chemical spill."
        } else {
            "Lift excess grease first, then use a cleaner suitable for the material. For washable fabric, pretreat with a little laundry detergent and wash as the label allows. For upholstery or a hard surface, use a barely damp cloth if the material can take it."
        }
    } else if contains(
        &text,
        &[
            "egg", "milk", "blood", "sick", "vomit", "poo", "pee", "urine", "spit", "phlegm",
        ],
    ) {
        if skin {
            "Rinse ordinary residue off intact skin, then wash well with soap and running water. If someone else's blood reached broken skin or an eye, get prompt medical advice."
        } else {
            "Rinse the stain from the back with cool water if the fabric allows it. Pretreat with an approved detergent and wash as the label directs. Check the mark before using a dryer."
        }
    } else if skin {
        "Use warm water and mild soap. Rub gently with your fingertips, rinse and pat dry. Check the label if this is a chemical or adhesive, and get advice if the skin becomes sore or irritated."
    } else if contains(
        &text,
        &[
            "book",
            "paper",
            "card",
            "page",
            "pages",
            "letter",
            "document",
            "photo",
            "photograph",
        ],
    ) {
        "Blot surface moisture off the paper gently with a clean dry cloth. Avoid getting the paper wetter or rubbing ink off. For a photograph, valuable document or book, check conservation advice for the material."
    } else if contains(
        &text,
        &[
            "carpet",
            "rug",
            "mat",
            "sofa",
            "couch",
            "settee",
            "armchair",
            "mattress",
            "upholstery",
            "cushion",
            "curtain",
            "car seat",
        ],
    ) {
        SURFACE_CLEANING
    } else if hard {
        "Lift any loose material with a soft cloth or a blunt plastic scraper. Wipe with a damp cloth and a cleaner approved for that surface. Test stronger products out of sight first."
    } else {
        "Blot excess liquid, then rinse washable fabric with cool water from the back of the mark. Pretreat with a detergent suitable for the fabric and follow the care label. Check the stain before tumble drying."
    };
    Some(Cow::Borrowed(reply))
}
