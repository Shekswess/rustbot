//! Fault and repair replies from PyBot's cooking and kitchen kit registries.

use crate::bot::Reading;
use regex::Regex;
use std::sync::LazyLock;

#[path = "cooking_repairs_data.rs"]
mod data;
#[path = "cooking_hob.rs"]
pub(super) mod hob;

fn phrase_pattern(phrases: &[&str]) -> Regex {
    let alternatives = phrases
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>();
    Regex::new(&format!(r"\b(?:{})\b", alternatives.join("|"))).expect("source cooking phrases")
}

macro_rules! phrase_patterns {
    ($($name:ident => $source:expr),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| phrase_pattern($source));)+
    };
}

phrase_patterns! {
    BURNT_EATING => data::BURNT_EATING_ASK,
    PAN_EATING => data::BURNT_EATING_SAFETY,
    BATTER => data::BURNT_BATTER,
    VEGETABLE => data::BURNT_VEG,
    MEAT_EXCLUDES_VEGETABLE => &["chicken", "beef", "lamb", "pork", "turkey", "gammon",
        "duck", "steak", "sausage", "sausages", "bacon", "ham", "joint", "brisket", "meatballs"],
    ROAST_PARTS => data::ROAST_PARTS,
    ROAST_MEAT => data::BURNT_MEAT,
    ROAST_EXCLUDE => &[data::PAN_WORDS, &["sauce", "gravy", "soup", "stew", "curry", "rice"]].concat(),
    PAN_DISH => data::PAN_DISH,
    PAN_WORDS => data::PAN_WORDS,
    TIN_OR_OVEN => data::TIN_OR_OVEN,
    BAKE_NOUNS => data::BAKE_NOUNS,
    BAKED_BURNT => data::BAKED_BURNT,
    NOT_A_PAN => data::NOT_A_PAN,
    A_PAN => data::A_PAN,
    PAN_STUCK => data::PAN_STUCK,
    PAN_BURNT => data::PAN_BURNT,
    PAN_SOMETHING => data::PAN_SOMETHING,
    PAN_CLEAN => data::PAN_CLEAN,
    FOOD => data::FOOD,
    TIN_CLEAN => &["off my tray", "off the tray", "off my tin", "off the tin",
        "get the burnt off", "off the sheet"],
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::$name)
            .expect("source cooking pattern"));)+
    };
}

patterns! { VESSEL_PHRASES_PATTERN, BLANK_STUFF_PATTERN }

static BAKE_BOTTOM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::BAKE_BOTTOM_PATTERN).unwrap());
static BAKE_STOP: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::BAKE_STOP_PATTERN).unwrap());
static BAKE_DONE: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::BAKE_DONE_PATTERN).unwrap());
static HOME_FLAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("(?i){}", data::FLAT_YOU_LIVE_IN_PATTERN)).expect("source flat guard")
});

pub(super) fn reads_flat_bake(reading: &Reading<'_>) -> bool {
    !HOME_FLAT.is_match(reading.raw)
}

pub(super) fn registered(id: &str, reading: &Reading<'_>) -> Option<&'static str> {
    let words = &reading.line.words;
    match id {
        "fix-falls-apart" => falls_apart(words, &reading.line.clean),
        "egg-dish-fault" => egg_fault(words),
        "fix-undercooked" => Some(undercooked(words)),
        "fix-greasy" => Some(greasy(words)),
        "bread-make" => Some(bread(words)),
        "flat-bake" => Some(flat(words)),
        "dip-fault" => Some(dip(&reading.line.clean)),
        "bake-burning" => bake_burning(reading),
        "fix-burnt" => burnt_dish(&combined_line(reading, false)),
        "burnt-pan" => burnt_pan(&combined_line(reading, false)),
        "burnt-tin" => burnt_tin(&combined_line(reading, true), words),
        "kit-damaged" => Some(kit_damaged(reading)),
        _ => hob::registered(id, reading),
    }
}

fn prefix_word(words: &[String], candidates: &[&str]) -> bool {
    words.iter().any(|word| {
        candidates
            .iter()
            .any(|candidate| word.starts_with(candidate))
    })
}

fn contains(text: &str, parts: &[&str]) -> bool {
    parts.iter().any(|part| text.contains(part))
}

fn words_or_phrases(words: &[String], said: &str, candidates: &[&str]) -> bool {
    prefix_word(words, candidates)
        || candidates
            .iter()
            .any(|part| part.contains(' ') && said.contains(part))
}

fn falls_apart(words: &[String], clean: &str) -> Option<&'static str> {
    let said = words.join(" ");
    let has = |wanted| prefix_word(words, wanted);
    let why = words
        .iter()
        .any(|word| matches!(word.as_str(), "why" | "howcome"))
        || contains(
            &said,
            &[
                "what did i do wrong",
                "what went wrong",
                "what am i doing wrong",
            ],
        );
    if has(data::FISH_NAMES) {
        Some(if has(&["pie", "pies", "fish pie"]) {
            data::FALLS_FISH_PIE
        } else if why {
            data::FALLS_FISH_WHY
        } else {
            data::FALLS_FISH
        })
    } else if has(&[
        "meatballs",
        "meatball",
        "kofta",
        "koftas",
        "rissoles",
        "falafel",
        "sausage meat",
        "meatloaf",
        "patties",
        "patty",
    ]) {
        Some(data::FALLS_SHAPED)
    } else if has(&["tofu", "paneer", "halloumi", "tempeh"]) {
        Some(data::FALLS_TOFU)
    } else if has(&["potatoes", "potato", "mash"]) {
        Some(data::FALLS_POTATO)
    } else if has(&["dumplings", "dumpling", "gnocchi"]) {
        Some(data::FALLS_DUMPLING)
    } else if has(&[
        "cake",
        "cakes",
        "sponge",
        "bread",
        "loaf",
        "loaves",
        "pastry",
        "scone",
        "scones",
        "biscuit",
        "biscuits",
        "cookie",
        "cookies",
        "brownie",
        "brownies",
        "muffin",
        "muffins",
        "traybake",
        "yorkshire pudding",
        "yorkshires",
        "burger",
        "burgers",
        "rice",
        "peas",
        "pasta",
        "noodles",
        "omelette",
        "pancake",
        "pancakes",
    ]) {
        None
    } else if has(&[
        "chicken", "turkey", "lamb", "beef", "pork", "roast", "joint", "shoulder", "brisket",
        "ham", "gammon",
    ]) {
        Some(data::FALLS_MEAT)
    } else if has(&["stew", "casserole", "tagine", "goulash", "braise", "pulled"]) {
        Some(data::FALLS_STEW)
    } else if contains(&format!(" {said}  {clean} "), data::COMING_APART_ON_PURPOSE) {
        Some(data::FALLS_EXPECTED)
    } else if has(data::MUSHY_OWN_ROW) {
        None
    } else {
        Some(if why {
            data::FALLS_WHY
        } else {
            data::FALLS_GENERAL
        })
    }
}

fn egg_fault(words: &[String]) -> Option<&'static str> {
    let said = words.join(" ");
    let named = |names| contains(&said, names);
    if named(data::EGG_SAUCE) {
        named(&[
            "grainy",
            "gritty",
            "split",
            "splits",
            "curdled",
            "curdles",
            "lumpy",
            "scrambled",
        ])
        .then_some(data::EGG_CUSTARD)
    } else if named(data::EGG_QUICHE) {
        named(&[
            "rubbery",
            "rubber",
            "tough",
            "dense",
            "overcooked",
            "over cooked",
        ])
        .then_some(data::EGG_QUICHE_RUBBERY)
    } else if named(data::EGG_FRIED) {
        Some(data::EGG_FRIED_RUBBERY)
    } else if named(data::EGG_PAN) || named(data::EGG_FRITTATA) {
        if named(&[
            "fell apart",
            "falls apart",
            "falling apart",
            "wouldnt fold",
            "wont fold",
            "broke",
            "tears",
            "tore",
        ]) {
            Some(data::EGG_FOLD)
        } else if named(&["soggy", "watery", "wet"]) {
            Some(data::EGG_WATERY)
        } else {
            named(&[
                "rubbery",
                "rubber",
                "tough",
                "leathery",
                "overcooked",
                "over cooked",
                "chewy",
            ])
            .then_some(data::EGG_RUBBERY)
        }
    } else {
        None
    }
}

fn undercooked(words: &[String]) -> &'static str {
    let said = words.join(" ");
    [
        (data::UNDER_BREAD, data::UNDER_LOAF),
        (data::UNDER_BAKE, data::UNDER_BAKED),
        (data::UNDER_STARCH, data::UNDER_GRAIN),
        (data::UNDER_VEG, data::UNDER_VEGETABLE),
    ]
    .into_iter()
    .find_map(|(names, reply)| words_or_phrases(words, &said, names).then_some(reply))
    .unwrap_or(data::UNDER_DEFAULT)
}

fn greasy(words: &[String]) -> &'static str {
    let said = words.join(" ");
    if words_or_phrases(words, &said, data::FRIED_GREASY) {
        data::GREASY_FRIED
    } else if prefix_word(
        words,
        &[
            "roast potatoes",
            "roasties",
            "potatoes",
            "potato",
            "wedges",
            "hash browns",
        ],
    ) && prefix_word(words, &["roast", "roasted", "oven"])
    {
        data::GREASY_ROAST
    } else {
        [
            (data::PASTRY_GREASY, data::GREASY_PASTRY),
            (data::MEAT_GREASY, data::GREASY_MEAT),
            (data::POT_GREASY, data::GREASY_POT),
        ]
        .into_iter()
        .find_map(|(names, reply)| words_or_phrases(words, &said, names).then_some(reply))
        .unwrap_or(data::GREASY_DEFAULT)
    }
}

fn bread(words: &[String]) -> &'static str {
    if words.iter().any(|word| {
        matches!(
            word.as_str(),
            "wholemeal" | "wholewheat" | "whole" | "brown" | "granary" | "malted"
        )
    }) || contains(
        &words.join(" "),
        &["whole wheat", "whole-wheat", "wholemeal flour"],
    ) {
        data::BREAD_WHOLEMEAL
    } else {
        data::BREAD_WHITE
    }
}

fn flat(words: &[String]) -> &'static str {
    if prefix_word(words, &["pancake", "pancakes", "batter", "crepe", "crepes"]) {
        data::FLAT_PANCAKE
    } else if prefix_word(words, &["yorkshire", "puddings"]) {
        data::FLAT_YORKSHIRE
    } else if prefix_word(words, &["scon", "scones"]) {
        data::FLAT_SCONE
    } else {
        data::FLAT_DEFAULT
    }
}

fn dip(text: &str) -> &'static str {
    if contains(text, data::DIP_FIXES[0].0)
        && contains(text, &["watery", "too thin", "runny", " thin"])
    {
        return data::DIP_WATERY;
    }
    data::DIP_FIXES
        .iter()
        .find_map(|&(names, reply)| contains(text, names).then_some(reply))
        .unwrap_or(data::DIP_PLAIN)
}

fn combined_line(reading: &Reading<'_>, join_norm: bool) -> String {
    let line = &reading.line;
    let norm = if join_norm {
        line.norm.join(" ")
    } else {
        format!("{:?}", line.norm)
    };
    format!("{} {} {norm}", line.clean, line.norm_text).to_lowercase()
}

fn bake_burning(reading: &Reading<'_>) -> Option<&'static str> {
    let text = combined_line(reading, true);
    let baked = VESSEL_PHRASES_PATTERN.replace_all(&text, " ");
    if !BAKE_NOUNS.is_match(&baked) && !BAKED_BURNT.is_match(&baked) && TIN_OR_OVEN.is_match(&text)
    {
        return None;
    }
    let said = combined_line(reading, false);
    Some(if BAKE_BOTTOM.is_match(&said) {
        data::BAKE_BOTTOM_REPLY
    } else if BAKE_STOP.is_match(&said) || said.contains("burning") {
        data::BAKE_PREVENTION
    } else if BAKE_DONE.is_match(&said) {
        data::BAKE_BURNT
    } else {
        data::BAKE_PREVENTION
    })
}

fn burnt_dish(said: &str) -> Option<&'static str> {
    if BURNT_EATING.is_match(said) {
        return Some(data::BURNT_EATING_REPLY);
    }
    if BATTER.is_match(said) {
        return Some(data::BURNT_BATTER_REPLY);
    }
    if VEGETABLE.is_match(said) && !MEAT_EXCLUDES_VEGETABLE.is_match(said) {
        return Some(data::BURNT_VEG_REPLY);
    }
    let without_vessel = VESSEL_PHRASES_PATTERN.replace_all(said, " ");
    if ROAST_PARTS.is_match(&without_vessel) {
        return Some(data::ROAST_PART_REPLY);
    }
    if ROAST_MEAT.is_match(&without_vessel) && !ROAST_EXCLUDE.is_match(&without_vessel) {
        return Some(data::BURNT_MEAT_REPLY);
    }
    (PAN_DISH.is_match(&without_vessel)
        || (!PAN_WORDS.is_match(said) && !TIN_OR_OVEN.is_match(&without_vessel)))
    .then_some(data::BURNT_DISH_REPLY)
}

fn burnt_pan(said: &str) -> Option<&'static str> {
    if PAN_EATING.is_match(said) {
        return Some(data::BURNT_EATING_REPLY);
    }
    if (NOT_A_PAN.is_match(said) && !A_PAN.is_match(said))
        || (PAN_STUCK.is_match(said) && !PAN_BURNT.is_match(said) && !PAN_SOMETHING.is_match(said))
    {
        return None;
    }
    (PAN_CLEAN.is_match(said) || !PAN_DISH.is_match(said)).then_some(data::BURNT_PAN_REPLY)
}

fn burnt_tin(said: &str, words: &[String]) -> Option<&'static str> {
    let without_vessel = VESSEL_PHRASES_PATTERN.replace_all(said, " ");
    let baked = BLANK_STUFF_PATTERN.replace_all(&without_vessel, " ");
    if FOOD.is_match(&baked) && !PAN_CLEAN.is_match(said) && !TIN_CLEAN.is_match(said) {
        return None;
    }
    let has = |terms: &[&str]| words.iter().any(|word| terms.contains(&word.as_str()));
    Some(
        if has(&[
            "coating",
            "coatings",
            "non",
            "nonstick",
            "teflon",
            "scratch",
            "scratched",
            "scratches",
            "scouring",
            "scour",
            "scourer",
            "wool",
            "scraper",
        ]) {
            data::TIN_COATING
        } else if has(&["shelf", "shelves", "rack", "racks"]) {
            data::TIN_SHELF
        } else if has(&["tray", "trays", "sheet", "sheets"]) {
            data::TIN_TRAY
        } else {
            data::TIN_DEFAULT
        },
    )
}

fn kit_damaged(reading: &Reading<'_>) -> &'static str {
    let line = &reading.line;
    let text = format!(" {} {} {} ", line.text, line.norm_text, line.clean).to_lowercase();
    let has = |terms: &[&str]| terms.iter().any(|term| text.contains(&format!(" {term}")));
    if has(&[
        "microwave plate",
        "turntable",
        "glass plate",
        "plate in the microwave",
    ]) {
        data::KIT_MICROWAVE
    } else if has(&["jug", "blender", "carafe", "glass lid", "lid"]) {
        data::KIT_JUG
    } else if has(&["board", "chopping board", "cutting board", "wooden board"]) {
        data::KIT_BOARD
    } else if has(&["enamel", "enamelled", "chipped", "chipping"]) {
        data::KIT_ENAMEL
    } else if has(&[
        "coating",
        "non stick",
        "non-stick",
        "teflon",
        "flaking",
        "peeling",
    ]) {
        data::KIT_COATING
    } else if has(&[
        "warped",
        "warping",
        "buckled",
        "buckling",
        "dented",
        "bent",
        "wobbles",
        "wobbling",
        "rocks",
        "rocking",
        "spins",
        "doesnt sit flat",
        "doesn't sit flat",
        "wont sit flat",
        "won't sit flat",
        "not flat",
        "bowed",
        "dished",
        "domed",
        "bulging",
        "bulged",
    ]) {
        if has(&[
            "tin",
            "tins",
            "tray",
            "trays",
            "sheet",
            "springform",
            "mould",
            "muffin",
        ]) {
            data::KIT_BENT_TIN
        } else {
            data::KIT_WARPED
        }
    } else if has(&["kettle"]) {
        data::KIT_KETTLE
    } else if has(&[
        "slow cooker",
        "casserole",
        "stoneware",
        "ceramic",
        "pot",
        "pots",
        "dish",
        "baking dish",
        "oven dish",
    ]) {
        data::KIT_POT
    } else {
        data::KIT_DEFAULT
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/cooking_repairs.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooking_and_kit_reply_branches_follow_the_pinned_source() {
        for &(id, prompt, expected) in fixtures::CASES {
            assert_eq!(
                registered(id, &Reading::new(prompt)),
                expected,
                "{id}: {prompt}"
            );
        }
    }

    #[test]
    fn flat_bake_guard_reads_the_determiner() {
        assert!(reads_flat_bake(&Reading::new("My scones came out flat.")));
        assert!(!reads_flat_bake(&Reading::new(
            "Can I bake in my small flat?"
        )));
    }
}
