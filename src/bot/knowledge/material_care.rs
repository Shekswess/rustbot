//! Cleaning advice selected by the product and material named in a source card.

use std::{borrow::Cow, sync::LazyLock};

use regex::RegexSet;

use crate::bot::Reading;

pub(super) fn word_patterns(groups: &[&[&str]]) -> RegexSet {
    RegexSet::new(groups.iter().map(|choices| {
        let choices = choices
            .iter()
            .map(|choice| regex::escape(choice))
            .collect::<Vec<_>>()
            .join("|");
        format!(r"(?i)\b(?:{choices})\b")
    }))
    .expect("material care terms")
}

static BEDDING: LazyLock<RegexSet> = LazyLock::new(|| word_patterns(&[super::CARE_MATTRESS]));
static STONE: LazyLock<RegexSet> = LazyLock::new(|| {
    word_patterns(&[
        &["descaler", "descaling", "limescale remover"],
        &["lemon juice", "lemon"],
        super::CARE_HARD_STONE,
    ])
});

pub(super) fn bleach_on_bedding(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = if BEDDING.is_match(&reading.line.text) {
        "Avoid bleach on a mattress unless its maker specifically allows it. Blot spills promptly, and check whether the cover is removable and washable. Follow its temperature and drying instructions. Keep the foam or padding dry and let the mattress air fully before putting the bedding back."
    } else {
        "Check the care label on the bedding first. The label tells you whether it can be bleached and which wash temperature and drying method to use. For a duvet or pillow, also check the washer load rating and the bedding maker's instructions."
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn acid_on_stone(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let found = STONE.matches(&reading.line.text);
    let product = if found.matched(0) {
        "descaler"
    } else if found.matched(1) {
        "lemon juice"
    } else {
        "vinegar"
    };
    let material = if found.matched(2) {
        "Granite, slate and engineered quartz vary by finish and sealer, so check the supplier's cleaning instructions."
    } else {
        "Acid can etch marble, limestone, travertine and onyx and leave a dull patch."
    };
    Some(Cow::Owned(format!(
        "Avoid cleaning the surface with {product}. {material} Wipe up a splash promptly. Use a mild pH-neutral cleaner approved for the surface, then rinse and dry it."
    )))
}

const DISHWASHER: &[(&[&str], &str)] = &[
    (
        super::CARE_IRON,
        "For a seasoned cast-iron or carbon-steel pan, follow its hand-washing instructions. The wash can strip seasoning and encourage rust. Dry it promptly after cleaning.",
    ),
    (
        super::CARE_WOOD,
        "Hand-wash wooden items unless the maker says they can go in the dishwasher. Prolonged water and heat can swell wood and damage joints and handles.",
    ),
    (
        super::CARE_KNIFE,
        "Hand-wash a kitchen knife unless its maker allows machine washing. Protect the edge and dry the knife promptly. A rack or loose cutlery can damage the edge.",
    ),
    (
        super::CARE_SOFT_METAL,
        "Check the maker's instructions. A dishwasher can discolour or pit some bare metals and damage decorative finishes. Hand-wash and dry the item promptly if the label calls for it.",
    ),
    (
        super::CARE_TRAPPED,
        "Check whether that utensil or flask can go in the machine and whether it needs to be taken apart. Hand-wash it if the maker says so, and drain and dry the parts before reassembling it.",
    ),
    (
        super::CARE_STONEWARE,
        "Check the maker's instructions for the stone or glazed finish. Some pizza stones and unglazed pieces absorb water. If yours calls for hand cleaning, follow its instructions and let it dry fully before heating it.",
    ),
    (
        super::CARE_DELICATE,
        "Check whether the maker allows machine washing. Crystal, painted decoration and metallic rims can cloud, chip or wear. Use the recommended hand-wash method if the item is unsuitable for a dishwasher.",
    ),
    (
        super::CARE_COATING,
        "Check the maker's instructions for the coating. If it calls for hand washing, clean the pan or basket gently with a soft sponge and dry it before putting it back.",
    ),
];
static DISHWASHER_MATERIAL: LazyLock<RegexSet> = LazyLock::new(|| {
    word_patterns(
        &DISHWASHER
            .iter()
            .map(|(terms, _)| *terms)
            .collect::<Vec<_>>(),
    )
});

pub(super) fn what_the_machine_ruins(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let source = &reading.line;
    let line = if source.clean.is_empty() {
        &source.text
    } else {
        &source.clean
    };
    let reply = DISHWASHER_MATERIAL.matches(line).iter().next().map_or(
        "Look for the maker's dishwasher-safe label or care instructions. Wood, sharp knives, uncoated metals and decorated pieces often need hand washing. Check that each item is dishwasher-safe even if you have washed a similar one before.",
        |index| DISHWASHER[index].1,
    );
    Some(Cow::Borrowed(reply))
}
