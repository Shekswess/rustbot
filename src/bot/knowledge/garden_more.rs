//! Source garden callbacks with ordered condition branches.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::Reading;
use crate::bot::understanding::kinds::StateOwner;

use super::{
    GARDEN_A_FRUIT_THAT_ROTS, GARDEN_A_GARDEN_THING, GARDEN_A_PART_OF_THE_PLANT,
    GARDEN_A_RAISED_BED_AMOUNT, GARDEN_A_RAISED_BED_BOTTOM, GARDEN_A_RAISED_BED_BUILD,
    GARDEN_A_RAISED_BED_DRY, GARDEN_A_RAISED_BED_GRASS, GARDEN_A_RAISED_BED_GROW,
    GARDEN_A_RAISED_BED_PAVING, GARDEN_A_RAISED_BED_SOIL, GARDEN_A_ROOM_OR_WALL, GARDEN_BARE_LOSS,
    GARDEN_BARE_STEMS, GARDEN_BRASSICAS, GARDEN_DRY_IS_A_VERB, GARDEN_FROST_AFTERMATH,
    GARDEN_FROST_ALREADY, GARDEN_GARDEN_IN_THE_LINE, GARDEN_HERB_WINTER_WORDS,
    GARDEN_HERB_WOODY_NAMES, GARDEN_LEAF_WORDS, GARDEN_PRUNE_NAMED, GARDEN_SOFT_HERBS,
    GARDEN_SOMETHING_TO_GROW_IN, GARDEN_THE_GARDEN_IN_THE_COLD, GARDEN_WHEN_ASKS,
};

#[derive(Clone, Copy)]
pub(super) enum Rule {
    RaisedBed,
    Bean,
    Brassica,
    Courgette,
    Lawn,
    PruneFruit,
    PruneShrub,
    Herbs,
    Seedlings,
    Cactus,
    Dig,
    DryBed,
    Rose,
    Tree,
    Cuttings,
}

static NON_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9' ]+").unwrap());
static STONE_FRUIT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bplum|\bcherr|\bdamson").unwrap());
static WHY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(?:why|how come)\b").unwrap());
static DRY_IS_A_VERB: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(GARDEN_DRY_IS_A_VERB).unwrap());
static POTS_ALREADY_FROZEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(?:have|has|had|got|been|is|are|was|were)\s+(?:been\s+)?",
        r"(?:frozen|caught|blackened|dead)\b|\b(?:blackened|dead|died|killed)\b"
    ))
    .expect("outdoor pots already damaged by frost")
});
static SHRUBS: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    GARDEN_PRUNE_NAMED
        .iter()
        .map(|(source, text)| {
            (
                Regex::new(source).expect("source shrub pruning pattern"),
                *text,
            )
        })
        .collect()
});

pub(super) fn says(line: &str, term: &str) -> bool {
    let bytes = line.as_bytes();
    line.match_indices(term).any(|(start, _)| {
        (start == 0 || !bytes[start - 1].is_ascii_alphanumeric())
            && !bytes
                .get(start + term.len())
                .is_some_and(u8::is_ascii_alphanumeric)
    })
}

pub(super) fn any(line: &str, words: &[&str]) -> bool {
    words.iter().any(|word| says(line, word))
}

fn first_of(line: &str, groups: &[&[&str]]) -> usize {
    groups
        .iter()
        .position(|words| any(line, words))
        .unwrap_or(groups.len())
}

fn lawn(line: &str) -> usize {
    let definite: &[&[&str]] = &[
        &[
            "aerate",
            "aerating",
            "aeration",
            "aerated",
            "spiking",
            "spike",
            "hollow tine",
            "hollow tines",
            "compacted",
            "compaction",
        ],
        &[
            "scarify",
            "scarifying",
            "scarified",
            "scarifier",
            "dethatch",
            "dethatching",
            "thatch",
            "thatching",
            "spongy",
            "raking out",
        ],
        &[
            "reseed",
            "reseeding",
            "re seed",
            "oversow",
            "over sow",
            "sow grass",
            "grass seed",
        ],
    ];
    let index = first_of(line, definite);
    if index < definite.len() {
        return index;
    }
    let colour = any(
        line,
        &[
            "yellow",
            "yellowing",
            "brown",
            "browning",
            "patchy",
            "patches",
            "dying",
            "dead",
            "died",
            "killed",
        ],
    );
    let green = any(
        line,
        &[
            "green",
            "greener",
            "greenest",
            "lush",
            "thicker",
            "thick",
            "dull",
            "pale",
            "washed out",
            "faded",
            "tired",
            "yellowish",
            "pale green",
            "not very green",
            "thin",
            "feed",
            "feeding",
            "fertiliser",
            "fertilizer",
            "starved",
        ],
    );
    if green && !colour {
        3
    } else if any(
        line,
        &["patch", "patches", "patchy", "in places", "here and there"],
    ) {
        4
    } else if any(line, &["dead", "died", "killed"]) {
        5
    } else if colour {
        6
    } else {
        7
    }
}

fn herbs(line: &str) -> usize {
    let named = any(line, GARDEN_SOFT_HERBS) || any(line, &["herb", "herbs"]);
    let cutting = any(
        line,
        &[
            "prune",
            "pruning",
            "pinch",
            "pinching",
            "tip",
            "tips",
            "top",
            "topping",
            "trim",
            "trimming",
            "cut",
            "cutting",
            "snip",
            "snipping",
            "back",
            "harvest",
            "harvesting",
            "pick",
            "picking",
        ],
    );
    if named
        && any(line, GARDEN_HERB_WINTER_WORDS)
        && !any(
            line,
            &["grow", "growing", "plant", "planting", "sow", "sowing"],
        )
    {
        0
    } else if says(line, "mint")
        && any(
            line,
            &[
                "spread",
                "spreads",
                "spreading",
                "spread out",
                "taken over",
                "took over",
                "taking over",
                "everywhere",
                "invasive",
                "contain",
                "control",
                "stop it",
                "keep it in",
                "border",
            ],
        )
    {
        1
    } else if !named {
        7
    } else if any(
        line,
        &[
            "woody",
            "wooden",
            "leggy",
            "flowered",
            "flowering",
            "flower",
            "flowers",
            "bolted",
            "bolting",
            "gone to seed",
        ],
    ) {
        2
    } else if cutting && any(line, GARDEN_WHEN_ASKS) {
        if any(line, GARDEN_HERB_WOODY_NAMES) {
            3
        } else {
            4
        }
    } else if cutting {
        5
    } else {
        6
    }
}

pub(super) fn reply(
    rule: Rule,
    reading: &Reading<'_>,
    texts: &'static [&'static str],
) -> Option<&'static str> {
    let line = garden_words(clean_or_typed(reading));
    let index = match rule {
        Rule::RaisedBed => first_of(
            &line,
            &[
                GARDEN_A_RAISED_BED_AMOUNT,
                GARDEN_A_RAISED_BED_DRY,
                GARDEN_A_RAISED_BED_BOTTOM,
                GARDEN_A_RAISED_BED_GRASS,
                GARDEN_A_RAISED_BED_PAVING,
                GARDEN_A_RAISED_BED_GROW,
                GARDEN_A_RAISED_BED_SOIL,
                GARDEN_A_RAISED_BED_BUILD,
            ],
        ),
        Rule::Bean => usize::from(!any(&line, &["broad", "broad bean", "broad beans"])),
        Rule::Brassica => {
            if let Some(crop) = [
                "broccoli",
                "cauliflower",
                "sprouts",
                "sprout",
                "kale",
                "cabbage",
            ]
            .into_iter()
            .find(|crop| says(&line, crop))
            {
                let crop = if crop == "sprout" { "sprouts" } else { crop };
                return GARDEN_BRASSICAS
                    .iter()
                    .find_map(|&(name, reply)| (name == crop).then_some(reply));
            }
            0
        }
        Rule::Courgette => usize::from(!any(&line, &["pumpkin", "pumpkins", "squash"])),
        Rule::Lawn => lawn(&line),
        Rule::PruneFruit => usize::from(!STONE_FRUIT.is_match(&line)),
        Rule::PruneShrub => {
            if let Some((_, reply)) = SHRUBS.iter().find(|(pattern, _)| pattern.is_match(&line)) {
                return Some(*reply);
            }
            0
        }
        Rule::Herbs => herbs(&line),
        Rule::Seedlings => usize::from(!any(
            &line,
            &[
                "leggy",
                "spindly",
                "straggly",
                "stretched",
                "stretching",
                "tall and thin",
                "fallen over",
                "falling over",
                "flopped",
                "flopping",
            ],
        )),
        Rule::Cactus => usize::from(!any(
            &line,
            &[
                "soft",
                "squishy",
                "soggy",
                "mushy",
                "translucent",
                "rotting",
                "rotted",
                "collapse",
                "collapsed",
            ],
        )),
        Rule::Dig => usize::from(!(any(&line, GARDEN_WHEN_ASKS) && says(&line, "dig"))),
        Rule::DryBed => {
            if DRY_IS_A_VERB.is_match(&line).unwrap_or(false) {
                0
            } else if WHY.is_match(&line) {
                1
            } else {
                2
            }
        }
        Rule::Rose => {
            let index = first_of(
                &line,
                &[
                    &["water", "watering", "how often", "hosepipe"],
                    &["feed", "feeding", "fertiliser", "fertilizer"],
                ],
            );
            if index < 2 {
                index
            } else {
                2 + usize::from(any(
                    &line,
                    &[
                        "ground",
                        "garden",
                        "bush",
                        "bushes",
                        "border",
                        "pot",
                        "soil",
                        "beds",
                        "bed",
                        "shrub",
                        "climbing",
                        "patio",
                        "plotted",
                        "planted",
                        "allotment",
                        "leaves",
                        "leaf",
                        "stems",
                        "stem",
                        "roots",
                        "root",
                        "buds",
                        "bud",
                        "shoots",
                        "hedge",
                    ],
                ))
            }
        }
        Rule::Tree => first_of(
            &line,
            &[
                &["water", "watering", "how often", "soak"],
                &["prune", "pruning", "cut back", "shape"],
            ],
        ),
        Rule::Cuttings => usize::from(!any(&line, GARDEN_WHEN_ASKS)),
    };
    texts.get(index).copied().filter(|text| !text.is_empty())
}

pub(super) fn clean_or_typed<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

pub(super) fn garden_words(line: &str) -> String {
    let lower = line.to_lowercase();
    NON_WORD.replace_all(&lower, " ").into_owned()
}

fn names_word_or_phrase(line: &str, names: &[&str]) -> bool {
    names.iter().any(|name| {
        if name.contains(' ') {
            line.contains(name)
        } else {
            line.split(|c: char| !c.is_ascii_lowercase() && c != '\'')
                .any(|word| word == *name)
        }
    })
}

pub(super) fn somewhere_to_grow(reading: &Reading<'_>) -> bool {
    let line = if reading.line.clean.is_empty() {
        &reading.line.norm_text
    } else {
        &reading.line.clean
    };
    let line = line
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    !names_word_or_phrase(&line, GARDEN_A_ROOM_OR_WALL)
        || names_word_or_phrase(&line, GARDEN_SOMETHING_TO_GROW_IN)
}

pub(super) fn the_plant_is_the_subject(reading: &Reading<'_>) -> bool {
    reading.features.state_owner != Some(StateOwner::Speaker)
}

pub(super) fn bare_stems(reading: &Reading<'_>) -> bool {
    let line = clean_or_typed(reading)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    names_word_or_phrase(&line, GARDEN_BARE_STEMS)
        || (names_word_or_phrase(&line, GARDEN_BARE_LOSS)
            && names_word_or_phrase(&line, GARDEN_A_PART_OF_THE_PLANT))
}

pub(super) fn garden_frost(reading: &Reading<'_>) -> bool {
    let line = clean_or_typed(reading)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    names_word_or_phrase(&line, GARDEN_THE_GARDEN_IN_THE_COLD)
}

const COLD_POND_WORDS: &[&str] = &[
    "freeze",
    "freezes",
    "freezing",
    "frozen",
    "ice",
    "iced",
    "icy",
    "winter",
    "winters",
    "overwinter",
    "winterise",
    "winterize",
    "cold",
    "snow",
    "frost",
];

pub(super) fn winter_pond(reading: &Reading<'_>) -> bool {
    let line = garden_words(clean_or_typed(reading));
    any(&line, COLD_POND_WORDS)
        || any(
            &line,
            &["wintertime", "overwintering", "winterised", "winterising"],
        )
}

pub(super) fn summer_pond(reading: &Reading<'_>) -> bool {
    !any(&garden_words(clean_or_typed(reading)), COLD_POND_WORDS)
}

const CHRISTMAS_TREES: &[&str] = &["christmas", "xmas", "spruce", "fir", "nordmann"];

pub(super) fn christmas_tree(reading: &Reading<'_>) -> bool {
    let line = garden_words(clean_or_typed(reading));
    any(&line, CHRISTMAS_TREES)
        || any(&line, &["needles", "needle", "tinsel", "decorations"])
        || !any(&line, GARDEN_A_GARDEN_THING)
}

pub(super) fn growing_tree(reading: &Reading<'_>) -> bool {
    let line = garden_words(clean_or_typed(reading));
    !any(&line, CHRISTMAS_TREES) && any(&line, &["tree", "trees", "sapling", "saplings"])
}

pub(super) fn frost_damage(reading: &Reading<'_>) -> bool {
    let line = clean_or_typed(reading).to_lowercase().replace('’', "'");
    names_word_or_phrase(&line, GARDEN_FROST_ALREADY)
        && names_word_or_phrase(&line, GARDEN_FROST_AFTERMATH)
}

pub(super) fn frost_ahead_for_pots(reading: &Reading<'_>) -> bool {
    let text = if reading.raw.is_empty() {
        &reading.line.clean
    } else {
        reading.raw
    };
    !POTS_ALREADY_FROZEN.is_match(text)
}

pub(super) fn names_garden(reading: &Reading<'_>) -> bool {
    any(&garden_words(&reading.line.text), GARDEN_GARDEN_IN_THE_LINE)
}

pub(super) fn tomato_fruit(reading: &Reading<'_>) -> bool {
    let line = garden_words(clean_or_typed(reading));
    !any(&line, GARDEN_LEAF_WORDS) || any(&line, GARDEN_A_FRUIT_THAT_ROTS)
}

#[cfg(test)]
#[test]
fn the_pot_card_answers_a_move_ahead_of_frost() {
    let card = crate::lookup_knowledge("Do I need to bring my pots in tonight?").unwrap();
    assert_eq!(
        (card.domain, card.id),
        ("containers_frost", "a-pot-is-not-a-border")
    );
    assert!(card.reply.starts_with("Bring them in if you can"));
    for prompt in [
        "Should I move my pots because they have frozen?",
        "Should I move my blackened geraniums before tonight?",
    ] {
        assert!(!frost_ahead_for_pots(&Reading::new(prompt)), "{prompt}");
    }
}
