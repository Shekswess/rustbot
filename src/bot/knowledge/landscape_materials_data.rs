//! Pinned `bot/knowledge/materials.py` material classes and bag estimates.

pub(super) const NUMBERS: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub(super) const SLAB_BASE: &str = "100mm of compacted MOT Type 1, then 30 to 40mm of sharp sand mixed 5 to 1 with cement as the bed";

type Loose = (
    &'static [&'static str],
    f64,
    f64,
    &'static str,
    &'static str,
);

pub(super) const LOOSE: &[Loose] = &[
    (
        &[
            "gravel",
            "shingle",
            "chippings",
            "chips",
            "pea gravel",
            "pea shingle",
            "pebbles",
            "stones",
            "slate",
            "bark",
            "bark chippings",
            "woodchip",
            "wood chips",
            "mulch",
            "aggregate",
        ],
        50.0,
        10.0,
        "A 850kg bulk bag of {thing} covers about 10 square metres at 50mm, and a 25kg bag about 0.3 square metres",
        "Put a weed membrane down first if you do not want to hoe it, and keep 20mm below the damp course on a wall so the rain can drain away.",
    ),
    (
        &["topsoil", "soil", "loam"],
        100.0,
        7.0,
        "A 850kg bulk bag of {thing} (about 0.7 cubic metres) covers about 7 square metres at 100mm",
        "Screened topsoil, not subsoil, for anything you want to grow; it settles about a fifth, so fill to slightly proud of the level you want.",
    ),
    (
        &[
            "compost",
            "multipurpose compost",
            "manure",
            "composted manure",
        ],
        50.0,
        12.0,
        "A 60 litre bag of {thing} covers about 1 square metre at 50mm, and a 145 litre bag about 2.5 square metres",
        "For a raised bed, fill the top 300mm with compost and the rest with rougher soil or rubble at the bottom; a bed one metre by two at 300mm deep takes about 6 bags of 60 litres.",
    ),
    (
        &["sand", "play sand", "sharp sand", "builder's sand"],
        50.0,
        0.6,
        "A 25kg bag of {thing} covers about 0.6 square metres at 50mm, and about a tonne and a half fills a cubic metre",
        "Play sand, washed and graded, for a sandpit rather than builder's sand; cover it at night so cats and rain leave it alone.",
    ),
];

pub(super) const BULK_WEIGHT: &[(&[&str], &str)] = &[
    (
        &["bark", "woodchip", "wood chip", "mulch"],
        "about 350kg to 400kg, and it fills close to a cubic metre and a half",
    ),
    (
        &["compost", "soil", "topsoil", "loam"],
        "about 700kg to 750kg, which is roughly 0.7 of a cubic metre",
    ),
    (
        &["sand", "ballast", "sharp sand", "grit sand"],
        "about 850kg, which is about half a cubic metre",
    ),
    (
        &[
            "slate",
            "pebbles",
            "pebble",
            "shingle",
            "chippings",
            "gravel",
            "aggregate",
            "stone",
            "stones",
        ],
        "about 850kg, which is about half a cubic metre at 20mm",
    ),
];
