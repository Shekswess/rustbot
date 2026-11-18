//! Source routes for crop damage, pests and wood splitting.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

#[path = "garden_machine.rs"]
pub(super) mod machine;

use super::garden_more::{any, clean_or_typed, garden_words};

macro_rules! pattern {
    ($name:ident, $value:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

pattern!(
    BOLTING,
    r"\bbolt(?:ed|ing)?\b|\brun to seed\b|\bgoing to seed\b|\bgone to seed\b|\bflower(?:ing|ed)?\b|\bwent up\b"
);
pattern!(FORKING, r"\bfork(?:ed|ing)?\b|\btwo legs\b|\bdouble root");
pattern!(WOODY, r"\bwood(?:y|en)\b|\btough\b|\bpithy\b");
pattern!(WOOD_WORDS, super::GARDEN_WOOD_WORDS);
pattern!(SPLITTING, super::GARDEN_SPLITTING);
pattern!(RING_IS_NOT_WOOD, super::GARDEN_RING_IS_NOT_WOOD);
pattern!(OTHER_WOOD, super::GARDEN_OTHER_WOOD);
pattern!(RING_WORD, r"\brings?\b");
pattern!(ROUND, r"\brounds?\b");

static ROOTS: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    super::GARDEN_ROOT_NAMES
        .iter()
        .map(|&(pattern, name)| {
            (
                Regex::new(&format!(r"\b{pattern}\b")).expect("source crop pattern"),
                name,
            )
        })
        .collect()
});

pub(super) fn root_split(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = clean_or_typed(reading).to_lowercase();
    let crop = ROOTS
        .iter()
        .find_map(|(pattern, name)| pattern.is_match(&line).then_some(*name))
        .unwrap_or("A root crop");
    let first = if BOLTING.is_match(&line) {
        "runs to seed when a hot dry spell pushes it up in its first year, and once the stem goes up the root stops swelling and turns woody in the ground."
    } else if FORKING.is_match(&line) {
        "forks when the taproot meets a stone, a hard pan or fresh manure, or when the seedlings were left crowded: the root splits in two to get round it."
    } else if WOODY.is_match(&line) {
        "goes woody when it is left in the ground too long, and in hot dry soil it happens sooner: the root keeps growing and the middle turns hard and stringy."
    } else {
        "splits when it takes a drink after a dry spell: the root swells faster than the skin keeps up, and too much feed does the same."
    };
    Some(Cow::Owned(format!(
        "{crop} {first} Water little and often so the soil stays evenly damp rather than soaking it after a dry week, thin the seedlings so the roots have room, and lift them once they are a good size. The damaged ones are still fine to eat, so use those first."
    )))
}

fn named_pest(line: &str) -> Option<usize> {
    let words = garden_words(line);
    super::GARDEN_PEST_NOUNS
        .iter()
        .position(|names| any(&words, names))
}

pub(super) fn pests(
    reading: &Reading<'_>,
    earlier: &[Exchange],
    replies: &'static [&'static str],
) -> Option<&'static str> {
    let pest = named_pest(clean_or_typed(reading)).or_else(|| {
        earlier
            .iter()
            .rev()
            .take(3)
            .find_map(|turn| named_pest(&turn.user))
    })?;
    // The first source branch is the absent-pest abstention.
    replies.get(pest + 1).copied()
}

pub(super) fn splitting_a_log(reading: &Reading<'_>) -> bool {
    let said = [
        &reading.line.clean,
        &reading.line.norm_text,
        &reading.line.text,
    ]
    .map(String::as_str)
    .join(" ")
    .to_lowercase();
    if RING_WORD.is_match(&said) && !OTHER_WOOD.is_match(&said) && RING_IS_NOT_WOOD.is_match(&said)
    {
        return false;
    }
    WOOD_WORDS.is_match(&said) || (SPLITTING.is_match(&said) && ROUND.is_match(&said))
}

pub(super) fn fruit_setting(reading: &Reading<'_>) -> bool {
    static SMALL_PLACE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(concat!(
            r"\bsmall\s+(?:gardens?|space|plot|patio|balcony|yard|allotment|greenhouse|",
            r"windowsill|window box|pots?|veg patch|vegetable patch|veg plot|back yard)\b"
        ))
        .expect("small garden terms")
    });
    !SMALL_PLACE.is_match(clean_or_typed(reading))
}

pub(super) fn crop_rot(reading: &Reading<'_>) -> bool {
    let line = garden_words(clean_or_typed(reading));
    let kitchen = [
        "cheese",
        "bread",
        "loaf",
        "loaves",
        "toast",
        "jam",
        "meat",
        "ham",
        "bacon",
        "sausage",
        "mince",
        "chicken",
        "fish",
        "salmon",
        "milk",
        "butter",
        "cream",
        "yoghurt",
        "yogurt",
        "leftovers",
        "sandwich",
        "pasta",
        "rice",
        "soup",
        "sauce",
        "gravy",
        "cake",
        "biscuit",
    ];
    let ground = [
        "plant",
        "plants",
        "crop",
        "crops",
        "fruit",
        "fruits",
        "seedling",
        "seedlings",
        "greenhouse",
        "allotment",
        "vine",
        "vines",
        "tree",
        "trees",
        "leaf",
        "leaves",
        "root",
        "roots",
        "compost",
        "soil",
        "ground",
        "pot",
        "pots",
        "bed",
        "beds",
        "garden",
        "flower",
        "flowers",
    ];
    !any(&line, &kitchen) || any(&line, &ground)
}

// https://www.rhs.org.uk/vegetables/courgettes/grow-your-own
// https://www.anses.fr/en/news/beware-inedible-gourds
fn crop_setting(line: &str) -> &'static str {
    const CUCURBITS: &[&str] = &[
        "courgette",
        "courgettes",
        "zucchini",
        "squash",
        "pumpkin",
        "pumpkins",
        "cucumber",
        "cucumbers",
    ];
    let cucurbit = any(line, CUCURBITS);
    let tomatoes = any(line, &["tomato", "tomatoes"]);
    let strawberries = any(line, &["strawberry", "strawberries"]);
    if any(line, &["bitter"]) {
        if cucurbit {
            return "Discard unusually bitter courgettes, cucumbers, squashes or pumpkins. Some can contain high levels of bitter cucurbitacins, and cutting off an end or cooking the flesh cannot make them safe. Seek medical advice if anyone who ate them becomes unwell.";
        }
        if tomatoes || strawberries {
            return "Check whether the variety has reached its normal ripe colour before picking. Keep the plant evenly watered and check for damage or disease if the flavour has changed.";
        }
        return "Check the crop's usual ripe colour and growing conditions. Keep the soil evenly moist and avoid eating fruit that is unusually bitter until you have identified the cause.";
    }
    if any(line, &["small", "tiny", "shrivelled"])
        && !strawberries
        && !any(
            line,
            &[
                "plant",
                "plants",
                "seedling",
                "seedlings",
                "seed",
                "seeds",
                "leaf",
                "leaves",
            ],
        )
    {
        return "Small fruit has already set. Check whether the variety normally produces small fruit, then check watering, available root space and how many fruits are developing. Keep the soil evenly moist and feed according to the crop's needs.";
    }
    if cucurbit {
        return "A female courgette or squash flower has a swollen base that looks like a tiny fruit. If it shrivels before growing, poor pollination or cool, wet conditions can be the cause. Try transferring pollen from a fresh male flower to a female flower, and keep the plant evenly watered. Check the variety before hand-pollinating a cucumber, since some need no pollination.";
    }
    if any(
        line,
        &[
            "runner",
            "french bean",
            "french beans",
            "broad bean",
            "broad beans",
            "beans",
        ],
    ) {
        return "For beans that flower without setting pods, check moisture and pollination. Water thoroughly in dry periods and mulch to keep moisture at the roots. Protect pollinating insects and pick pods regularly once they reach the right size.";
    }
    if any(
        line,
        &[
            "chilli",
            "chillies",
            "chili",
            "pepper",
            "peppers",
            "aubergine",
            "aubergines",
            "eggplant",
            "eggplants",
        ],
    ) {
        return "Chillies, peppers and aubergines may drop flowers in excessive heat, cool conditions or dry soil. Keep watering even, ventilate under glass in hot weather and avoid overfeeding with nitrogen. A gentle shake of the flowers can help pollination under cover.";
    }
    if tomatoes {
        return "If a tomato has no flowers, check how much sun it gets and whether excess nitrogen is encouraging leaves. If flowers open and drop, check for temperature or watering stress and gently tap the flowering truss to help move pollen.";
    }
    if strawberries {
        return "Check that the strawberries get enough sun and consistent moisture, and that flowers are being pollinated. Remove runners if your priority is fruit on the parent plant. Older or unproductive plants can be renewed with healthy replacements.";
    }
    "If a crop flowers without setting fruit, check its temperature, water and specific pollination needs. Check the growing recommendations for its variety before changing feed or attempting hand pollination."
}

// https://www.rhs.org.uk/disease/tomato-blight
// https://www.rhs.org.uk/disease/potato-blight
// https://www.rhs.org.uk/disease/rose-black-spot
fn blight(line: &str) -> &'static str {
    if any(line, &["potato", "potatoes"]) {
        "Potato blight can cause rapidly spreading dark leaf patches during damp weather. If identified, remove the affected tops and keep them out of your home compost. For tubers already in the ground, the RHS advises waiting at least two weeks after removing the foliage before lifting. Discard any tubers showing signs of rot and consider resistant varieties next season."
    } else if any(line, &["rose", "roses"]) {
        "For rose black spot, collect fallen affected leaves and remove heavily affected growth. Keep infected material out of your home compost and improve airflow around the plant. Consider a resistant variety if the problem returns."
    } else if any(line, &["tomato", "tomatoes", "blight", "blighted"]) {
        "Tomato blight can spread rapidly in damp weather, causing dark patches on leaves, stems and fruit. Remove badly affected plants promptly and dispose of infected material outside your home compost. Grow resistant varieties where possible, water at the base and leave plants room to dry. There is no home-garden fungicide for tomato blight in the UK."
    } else {
        "Identify the plant and look closely at the lesions, including the underside of affected leaves. Different plant diseases can cause dark spots. Remove fallen affected material, improve airflow and consult a crop-specific disease guide before treating it."
    }
}

fn rot(line: &str, earlier: &[Exchange]) -> &'static str {
    const BULBS: &[&str] = &[
        "potato", "potatoes", "onion", "onions", "garlic", "bulb", "bulbs",
    ];
    const STONE_OR_POME: &[&str] = &["apple", "apples", "pear", "pears", "plum", "plums"];
    const IMMATURE: &[&str] = &[
        "seedling",
        "seedlings",
        "cutting",
        "cuttings",
        "stem",
        "stems",
    ];
    const FRUIT_TIP: &[&str] = &[
        "blossom end",
        "flower end",
        "black at the bottom",
        "black at the end",
        "bottom is black",
        "bottom of the fruit is black",
        "rot at the end",
        "rotting at the end",
        "end is rotting",
        "end rot",
        "rotting at the flower",
        "rot at the flower",
    ];
    if any(
        line,
        &[
            "eat",
            "eating",
            "edible",
            "safe",
            "use the rest",
            "use them",
        ],
    ) {
        return "Discard fruit and vegetables that are rotten, slimy or mouldy. Keep visibly sound specimens separate, check them carefully as they may spoil faster and wash them before use. Avoid eating green, bitter or badly sprouted potatoes.";
    }
    const OTHER_CROPS: &[&str] = &[
        "courgette",
        "courgettes",
        "cucumber",
        "cucumbers",
        "tomato",
        "tomatoes",
        "pepper",
        "peppers",
        "melon",
        "melons",
        "aubergine",
        "aubergines",
        "squash",
        "pumpkin",
        "pumpkins",
        "strawberry",
        "strawberries",
        "plant",
        "plants",
        "crop",
        "crops",
        "fruit",
        "fruits",
        "dahlia",
        "dahlias",
        "tree",
        "trees",
        "vine",
        "vines",
        "bed",
        "beds",
        "greenhouse",
        "ground",
        "allotment",
        "bottom",
    ];
    let current = [BULBS, STONE_OR_POME, IMMATURE, FRUIT_TIP, OTHER_CROPS]
        .iter()
        .any(|phrases| any(line, phrases));
    let history;
    let line = if current {
        line
    } else {
        history = earlier
            .iter()
            .rev()
            .take(3)
            .map(|turn| garden_words(&turn.user))
            .collect::<Vec<_>>()
            .join(" ");
        &history
    };
    if any(line, IMMATURE) {
        "Seedlings that collapse at the soil line may have damping off. Discard affected seedlings and their compost. Start again in clean trays with fresh compost, avoid waterlogging and give seedlings more airflow."
    } else if any(line, FRUIT_TIP) {
        "A dark, sunken patch at the flower end can be blossom-end rot. Keep moisture even at the roots, particularly for plants in pots, and remove affected fruit. Check symptoms against the crop's guidance because fungal rot and poor pollination can look similar."
    } else if any(line, BULBS) {
        "Separate and discard soft, slimy or smelly bulbs or tubers. Store only sound, dry produce with good airflow and check it regularly. For the next crop, use well-drained soil and identify the disease before changing planting or storage methods."
    } else if any(line, STONE_OR_POME) {
        "Fruit rotting on an apple, pear or plum tree may have brown rot. Collect affected fruit from the tree and ground, including old mummified fruit. Thin crowded clusters where appropriate to reduce rubbing and help fruit dry."
    } else {
        "Remove affected fruit, keep it out of your home compost if disease is suspected and avoid wetting the leaves and fruit as you water. Check drainage and airflow around the plant. Identifying the crop and where the rot starts will narrow down the cause."
    }
}

pub(super) fn condition(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<&'static str> {
    let line = garden_words(clean_or_typed(reading));
    Some(match id {
        "tomato-blight" => blight(&line),
        "crop-not-fruiting" => crop_setting(&line),
        "plant-rot" => rot(&line, earlier),
        _ => return None,
    })
}

#[cfg(test)]
mod condition_tests {
    use super::*;

    #[test]
    fn garden_conditions_keep_the_crop_and_scope_food_safety() {
        assert!(
            crop_setting("why are my courgettes bitter").starts_with("Discard unusually bitter")
        );
        assert!(blight("the potatoes have blight").contains("at least two weeks"));
        assert!(blight("the tomatoes have blight").contains("no home-garden fungicide"));
        assert!(rot("can i still eat the mouldy strawberries", &[]).starts_with("Discard"));
        assert!(!crop_rot(&Reading::new("My loaf is rotting")));
        assert!(!fruit_setting(&Reading::new(
            "Where should I grow tomatoes in a small garden?"
        )));
        let saw = Reading::new("How should I store my chainsaw over winter?");
        assert!(machine::advise(&saw).unwrap().contains("bar cover"));
    }
}
