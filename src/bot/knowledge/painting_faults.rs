//! Native guards for the four `painting_faults` cards.
//!
//! Source: `bot/knowledge/painting_faults.py` at `7d63c7fbebb44c7e466e5c8b5c1f341ab767929e`.
//! The four word-list guards below are ported 1:1 from that file (`_words`,
//! `_paint_is_the_subject`, `_a_finish_with_no_surface`, `_the_paint_has_sealed_it`,
//! `_the_trim_is_what_is_painted`, `_a_stain_under_the_coat`) together with the source
//! `a_food_is_being_kept` test from `bot/knowledge/lookup.py`.
//!
//! The rows register through the `*_gate` functions, which add one RustBot-only hold on top of
//! each source guard: every one of the four replies tells the reader to sand paint, and the
//! source has no reading for paint that is known or suspected to contain lead. The cards decline
//! those lines because sanding can create lead dust. See https://www.hse.gov.uk/lead/faqs.htm.

use regex::Regex;
use std::sync::LazyLock;

use crate::bot::Reading;

// The paint going on, in the forms a person writes it before they say what it is on.
const PAINT_WORD: &[&str] = &[
    "paint",
    "paints",
    "painted",
    "painting",
    "repaint",
    "repainting",
    "re-paint",
    "gloss",
    "glossing",
    "glossed",
    "satin",
    "eggshell",
    "undercoat",
    "undercoating",
    "primer",
    "priming",
    "primed",
    "varnish",
    "varnishing",
    "varnished",
    "coating",
];

// A finish is what is already on the wood.
const A_FINISH: &[&str] = &[
    "gloss",
    "glossy",
    "satin",
    "eggshell",
    "sheen",
    "matt",
    "matte",
    "undercoat",
    "primer",
    "primed",
    "varnish",
    "varnished",
    "lacquer",
    "lacqered",
    "lacquered",
    "oil",
    "oiled",
    "mdf",
];

// A surface word hands the line back to the card that owns that surface.
const A_SURFACE: &[&str] = &[
    "door",
    "doors",
    "cupboard",
    "cupboards",
    "kitchen",
    "skirting",
    "skirtings",
    "banister",
    "banisters",
    "bannister",
    "bannisters",
    "woodwork",
    "radiator",
    "radiators",
    "wall",
    "walls",
    "ceiling",
    "ceilings",
    "room",
    "rooms",
    "floor",
    "floors",
    "floorboard",
    "floorboards",
    "window",
    "windows",
    "frame",
    "frames",
    "sill",
    "sills",
    "windowsill",
    "furniture",
    "table",
    "tables",
    "chair",
    "chairs",
    "shelf",
    "shelves",
    "shed",
    "fence",
    "fences",
    "gate",
    "gates",
    "bedroom",
    "bathroom",
    "hallway",
    "stairs",
    "stair",
    "handrail",
    "architrave",
    "trim",
    "wardrobe",
    "wardrobes",
    "cabinet",
    "cabinets",
    "drawer",
    "drawers",
    "toy",
    "toys",
    "car",
    "cars",
    "bike",
    "bicycle",
];

// Paint that has landed somewhere it was not meant to go.
const A_ACCIDENT: &[&str] = &[
    "carpet",
    "carpets",
    "rug",
    "rugs",
    "sofa",
    "sofas",
    "couch",
    "settee",
    "curtain",
    "curtains",
    "cushion",
    "cushions",
    "clothes",
    "clothing",
    "shirt",
    "shirts",
    "jeans",
    "trousers",
    "jacket",
    "jackets",
    "hands",
    "hand",
    "hair",
    "skin",
    "fingers",
    "fingernails",
    "nails",
    "washing machine",
    "bath",
    "sink",
    "tiles",
    "tile",
    "tiling",
    "grass",
    "patio",
    "drive",
    "driveway",
];

// A coat that has to dry has its own wait.
const A_WAIT: &[&str] = &[
    "how long",
    "take to dry",
    "takes to dry",
    "dry",
    "dries",
    "drying",
    "dried",
    "recoat",
    "re-coat",
    "between coats",
    "touch dry",
    "hard",
    "cure",
    "curing",
    "seconds",
    "minutes",
    "hours",
    "days",
];

// Taking a finish off is not painting over it.
const A_STRIPPER: &[&str] = &[
    "remove",
    "removing",
    "removal",
    "get off",
    "take off",
    "taking off",
    "strip",
    "stripping",
    "stripper",
    "sand off",
    "sanding off",
    "peel",
    "peeling",
    "peeled",
    "flake",
    "flaking",
    "bubble",
    "bubbling",
    "blister",
    "blistering",
];

// The trim in a room, in the forms a person says it.
const THE_TRIM: &[&str] = &[
    "skirting",
    "skirtings",
    "skirting board",
    "skirting boards",
    "banister",
    "banisters",
    "bannister",
    "bannisters",
    "woodwork",
    "architrave",
    "handrail",
    "handrails",
    "spindle",
    "spindles",
    "balustrade",
    "picture rail",
    "dado rail",
    "window sill",
    "windowsill",
    "window sills",
    "windowsills",
    "sill",
    "sills",
    "trim",
    "door frame",
    "door frames",
];

// The preposition that puts the trim on the receiving end of the paint.
const ON_THE_WAY: &[&str] = &[
    "on", "onto", "off", "getting", "goes", "going", "got", "spilt", "spilled", "splash",
    "splashes", "splashed", "drips", "dripping", "runs", "running", "from", "under", "behind",
    "against", "down", "near", "round", "around",
];

// A mark in the surface that a coat goes over.
const A_MARK: &[&str] = &[
    "stain",
    "stains",
    "stained",
    "mark",
    "marks",
    "knot",
    "knots",
    "rust",
    "rusty",
    "nicotine",
    "tar",
    "smoke",
    "grease",
    "oily",
    "biro",
    "pen",
    "marker",
    "crayon",
    "felt tip",
    "felt pen",
    "watermark",
    "watermarks",
    "tide mark",
    "tide marks",
];

// Oil, water or smoke behind a wall is the damp card's question.
const A_LEAKING: &[&str] = &[
    "leak",
    "leaks",
    "leaking",
    "leaked",
    "damp",
    "damp patch",
    "damp patches",
    "water",
    "rain",
    "raining",
    "roof",
    "gutter",
    "gutters",
    "pipe",
    "pipes",
    "burst",
    "condensation",
    "mould",
    "mold",
    "mouldy",
    "moldy",
    "flood",
    "flooded",
];

// The coat that goes over the mark.
const A_OVER: &[&str] = &[
    "paint over",
    "painting over",
    "paint on top of",
    "cover",
    "covering",
    "showing through",
    "shows through",
    "showing up",
    "coming through",
    "come through",
    "bleeding through",
    "bleeds through",
    "seeping through",
    "block",
    "blocking",
    "stain block",
    "stainblock",
    "hiding",
];

/// `_words`: the line's words, lowercased, with `,.;:!?` stripped from both ends.
fn words(reading: &Reading<'_>) -> Vec<String> {
    reading
        .line
        .words
        .iter()
        .map(|word| {
            word.to_lowercase()
                .trim_matches([',', '.', ';', ':', '!', '?'])
                .to_owned()
        })
        .collect()
}

fn is_in(list: &[&str], word: &str) -> bool {
    list.contains(&word)
}

/// `_paint_is_the_subject`.
fn paint_is_the_subject(reading: &Reading<'_>) -> bool {
    !super::guards::food_is_being_kept(reading)
}

/// `_a_finish_with_no_surface`.
pub(super) fn a_finish_with_no_surface(reading: &Reading<'_>) -> bool {
    if !paint_is_the_subject(reading) {
        return false;
    }
    let words = words(reading);
    if !words.iter().any(|word| is_in(A_FINISH, word)) {
        return false;
    }
    if words.iter().any(|word| is_in(A_SURFACE, word)) {
        return false;
    }
    !words
        .iter()
        .any(|word| is_in(A_ACCIDENT, word) || is_in(A_WAIT, word) || is_in(A_STRIPPER, word))
}

/// `_the_paint_has_sealed_it`.
pub(super) fn the_paint_has_sealed_it(reading: &Reading<'_>) -> bool {
    paint_is_the_subject(reading) && words(reading).iter().any(|word| is_in(PAINT_WORD, word))
}

/// `_the_trim_is_what_is_painted`: the trim within four words after a paint word, unless a
/// word on the way comes first.
pub(super) fn the_trim_is_what_is_painted(reading: &Reading<'_>) -> bool {
    if !paint_is_the_subject(reading) {
        return false;
    }
    let words = words(reading);
    for (at, word) in words.iter().enumerate() {
        if !is_in(PAINT_WORD, word) {
            continue;
        }
        for said in words.iter().skip(at + 1).take(4) {
            if is_in(ON_THE_WAY, said) {
                break;
            }
            if is_in(THE_TRIM, said) {
                return true;
            }
        }
    }
    false
}

/// `_a_stain_under_the_coat`.
pub(super) fn a_stain_under_the_coat(reading: &Reading<'_>) -> bool {
    if !paint_is_the_subject(reading) {
        return false;
    }
    let words = words(reading);
    if !words.iter().any(|word| is_in(A_MARK, word)) {
        return false;
    }
    if words
        .iter()
        .any(|word| is_in(A_ACCIDENT, word) || is_in(A_LEAKING, word))
    {
        return false;
    }
    words.iter().any(|word| {
        is_in(A_OVER, word) || is_in(A_FINISH, word) || word == "paint" || word == "painting"
    })
}

// ------------------------------------------------------------------------------------------
// Every reply in this domain recommends sanding paint. Dry-sanding paint that contains lead
// releases lead dust, so a line that says the paint is, might be, or could be lead does not
// register any of these cards. The test is deliberately broad: any whole word `lead`,
// `leaded` or `leadbased` in the typed, cleaned or tokenised line holds the card, including
// the verb ("will this lead to peeling") and "lead-free". Over-holding only falls back to
// the pre-port RustBot behaviour for that line.
static LEAD_NAMED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(?:lead|leaded|leadbased)\b").expect("valid lead pattern"));

/// True when the line names lead, so a sanding reply must not be registered for it.
fn names_lead_paint(reading: &Reading<'_>) -> bool {
    [
        reading.raw,
        reading.line.text.as_str(),
        reading.line.clean.as_str(),
        reading.line.norm_text.as_str(),
    ]
    .iter()
    .any(|text| LEAD_NAMED.is_match(text))
}

/// Row gate for `painting_faults/painted-shut`: source guard, held on lead lines.
pub(super) fn painted_shut_gate(reading: &Reading<'_>) -> bool {
    the_paint_has_sealed_it(reading) && !names_lead_paint(reading)
}

/// Row gate for `painting_faults/a-coat-over-a-finish`: source guard, held on lead lines.
pub(super) fn coat_over_a_finish_gate(reading: &Reading<'_>) -> bool {
    a_finish_with_no_surface(reading) && !names_lead_paint(reading)
}

/// Row gate for `painting_faults/the-trim-is-what-is-painted`: source guard, held on lead lines.
pub(super) fn trim_is_painted_gate(reading: &Reading<'_>) -> bool {
    the_trim_is_what_is_painted(reading) && !names_lead_paint(reading)
}

/// Row gate for `painting_faults/a-stain-under-the-coat`: source guard, held on lead lines.
pub(super) fn stain_under_the_coat_gate(reading: &Reading<'_>) -> bool {
    a_stain_under_the_coat(reading) && !names_lead_paint(reading)
}

#[cfg(test)]
#[path = "../../../tests/fixtures/painting_faults.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    fn source_guard(name: &str) -> fn(&Reading<'_>) -> bool {
        match name {
            "the_paint_has_sealed_it" => the_paint_has_sealed_it,
            "a_finish_with_no_surface" => a_finish_with_no_surface,
            "the_trim_is_what_is_painted" => the_trim_is_what_is_painted,
            "a_stain_under_the_coat" => a_stain_under_the_coat,
            other => panic!("unknown painting guard {other}"),
        }
    }

    fn gate(id: &str) -> fn(&Reading<'_>) -> bool {
        match id {
            "painted-shut" => painted_shut_gate,
            "a-coat-over-a-finish" => coat_over_a_finish_gate,
            "the-trim-is-what-is-painted" => trim_is_painted_gate,
            "a-stain-under-the-coat" => stain_under_the_coat_gate,
            other => panic!("unknown painting card {other}"),
        }
    }

    #[test]
    fn painting_guards_follow_the_pinned_source() {
        let mismatches: Vec<_> = fixtures::SOURCE_GUARD_CASES
            .iter()
            .filter_map(|&(name, prompt, expected)| {
                let actual = source_guard(name)(&Reading::new(prompt));
                (actual != expected).then_some((name, prompt, expected, actual))
            })
            .collect();
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }

    #[test]
    fn painting_cards_hold_every_lead_line() {
        let mismatches: Vec<_> = fixtures::LEAD_HOLD_CASES
            .iter()
            .filter_map(|&(id, source, prompt)| {
                let reading = Reading::new(prompt);
                let held = names_lead_paint(&reading);
                let source_passes = source_guard(source)(&reading);
                let registered = gate(id)(&reading);
                // The hold must be what stands the card down: the source guard still passes.
                (!held || !source_passes || registered).then_some((
                    id,
                    prompt,
                    held,
                    source_passes,
                    registered,
                ))
            })
            .collect();
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }

    #[test]
    fn the_hold_leaves_lines_without_lead_alone() {
        for &(name, prompt, expected) in fixtures::SOURCE_GUARD_CASES {
            let reading = Reading::new(prompt);
            if names_lead_paint(&reading) {
                continue;
            }
            let id = match name {
                "the_paint_has_sealed_it" => "painted-shut",
                "a_finish_with_no_surface" => "a-coat-over-a-finish",
                "the_trim_is_what_is_painted" => "the-trim-is-what-is-painted",
                _ => "a-stain-under-the-coat",
            };
            assert_eq!(gate(id)(&reading), expected, "{id}: {prompt}");
        }
    }

    #[test]
    fn the_four_cards_register_in_source_order() {
        let ids: Vec<_> = super::super::knowledge_ids().collect();
        let at = ids
            .iter()
            .position(|&(domain, _)| domain == "painting_faults")
            .expect("painting_faults is registered");
        assert_eq!(
            &ids[at..at + 4],
            &[
                ("painting_faults", "painted-shut"),
                ("painting_faults", "a-coat-over-a-finish"),
                ("painting_faults", "the-trim-is-what-is-painted"),
                ("painting_faults", "a-stain-under-the-coat"),
            ]
        );
        assert_eq!(ids[at - 1].0, "opinions");
        assert_eq!(ids[at + 4], ("panic_talk", "panic-someone-else"));
        assert_eq!(
            ids.iter()
                .filter(|&&(domain, _)| domain == "painting_faults")
                .count(),
            4
        );
    }

    #[test]
    fn lead_lines_never_reach_a_painting_card() {
        for &(_, _, prompt) in fixtures::LEAD_HOLD_CASES {
            let found = super::super::lookup(prompt).map(|card| (card.domain, card.id));
            assert!(
                found.is_none_or(|(domain, _)| domain != "painting_faults"),
                "{prompt}: {found:?}"
            );
        }
    }

    #[test]
    fn lead_is_read_from_the_typed_line() {
        for prompt in fixtures::LEAD_NAMED {
            assert!(names_lead_paint(&Reading::new(prompt)), "{prompt}");
        }
        for prompt in fixtures::LEAD_NOT_NAMED {
            assert!(!names_lead_paint(&Reading::new(prompt)), "{prompt}");
        }
    }
}
