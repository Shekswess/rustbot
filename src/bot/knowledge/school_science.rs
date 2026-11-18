//! Reading and branch order for the authored science teaching cards.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::Reading;

#[derive(Clone, Copy)]
pub(super) enum Rule {
    Mass,
    Energy,
    CellDivision,
    Frequency,
    Velocity,
    Structure,
}

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(NEWTON_EACH, super::SCIENCE_NEWTON_EACH);
pattern!(GARDEN_PLANTS, super::SCIENCE_GARDEN_PLANTS);
pattern!(
    ICE_SLIP,
    r"slippery|slippy|slips? |slip on|skate|skating|glide"
);
pattern!(CRANE, r"\bcranes?\b");
pattern!(BRIDGE, r"\bbridges?\b");
pattern!(CHIMNEY, r"\bchimneys?\b");
pattern!(TREE, r"\btrees?\b|\bforest\b|\bwoods\b");
pattern!(
    PLANT,
    r"\bplants?\b|\bhouseplants?\b|\bflowers?\b|\bhanging basket\b"
);

fn typed_or_clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    }
}

fn clean_or_typed<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn combined_lines(reading: &Reading<'_>) -> String {
    format!(
        "{} {}",
        reading.line.clean.to_lowercase(),
        reading.line.text.to_lowercase()
    )
}

fn named<'a>(said: &str, words: &'a [&'a str]) -> Option<&'a str> {
    words
        .iter()
        .filter_map(|&word| {
            said.find(&format!(" {word} "))
                .or_else(|| said.find(word))
                .map(|at| (at, word))
        })
        .min_by_key(|(at, _)| *at)
        .map(|(_, word)| word)
}

pub(super) fn asks_about_mass(reading: &Reading<'_>) -> bool {
    let said = format!(" {} ", typed_or_clean(reading).to_lowercase());
    named(&said, super::SCIENCE_MASS_OR_WEIGHT_WORDS).is_some()
        || (named(&said, super::SCIENCE_MASS_OR_WEIGHT_UNITS).is_some()
            && named(&said, super::SCIENCE_DEFINE).is_some())
}

pub(super) fn water_is_the_subject(reading: &Reading<'_>) -> bool {
    reading.line.words.iter().all(|word| {
        super::SCIENCE_WATER_IS_THE_SUBJECT
            .binary_search(&word.as_str())
            .is_ok()
    })
}

fn stands_up(reading: &Reading<'_>) -> usize {
    let lines = combined_lines(reading);
    [
        &*CRANE,
        &*BRIDGE,
        &*CHIMNEY,
        &*TREE,
        &*GARDEN_PLANTS,
        &*PLANT,
    ]
    .iter()
    .position(|pattern| pattern.is_match(&lines))
    .unwrap_or(6)
}

pub(super) fn reply(
    rule: Rule,
    reading: &Reading<'_>,
    replies: &'static [&'static str],
) -> Option<&'static str> {
    let said = format!(" {} ", clean_or_typed(reading).to_lowercase());
    let index = match rule {
        Rule::Mass => usize::from(
            named(&said, &["weight", "weigh"]) != Some("weight")
                || named(&said, &["mass"]).is_some(),
        ),
        Rule::Energy => usize::from(named(&said, &["potential", "kinetic"]) != Some("potential")),
        Rule::CellDivision => match named(&said, &["meiosis", "cell cycle", "mitosis"]) {
            Some("meiosis") => 0,
            Some("cell cycle") => 1,
            _ => 2,
        },
        Rule::Frequency => match named(&said, &["wavelength", "amplitude", "frequency"]) {
            Some("wavelength") => 0,
            Some("amplitude") => 1,
            _ => 2,
        },
        Rule::Velocity => {
            usize::from(named(&said, &["velocity"]).is_none() || named(&said, &["speed"]).is_some())
        }
        Rule::Structure => stands_up(reading),
    };
    replies.get(index).copied()
}

pub(super) fn freezing_point(reading: &Reading<'_>) -> &'static str {
    let said = clean_or_typed(reading).to_lowercase();
    if ICE_SLIP.is_match(&said) {
        "Ice is slippery because a skate or a boot melts a thin film of water under it, so you are sliding on water rather than on ice, and the film keeps refreezing as you go over it. Water freezes at 0C (32F); salt lowers that, which is why roads are gritted."
    } else {
        super::SCIENCE_WATER_FREEZES
    }
}

pub(super) fn newtons_laws(reading: &Reading<'_>) -> &'static str {
    let said = reading.line.text.to_lowercase();
    let index = NEWTON_EACH
        .captures(&said)
        .and_then(|match_| {
            (1..=3)
                .find_map(|group| match_.get(group))
                .map(|word| word.as_str())
        })
        .map_or(1, |word| match word {
            "first" | "1st" => 0,
            "third" | "3rd" => 2,
            _ => 1,
        });
    super::SCIENCE_NEWTON_REPLIES[index]
}

fn name_at(said: &str, name: &str) -> Option<usize> {
    let bytes = said.as_bytes();
    said.match_indices(name).find_map(|(at, _)| {
        let whole = (at == 0 || !bytes[at - 1].is_ascii_lowercase())
            && !bytes
                .get(at + name.len())
                .is_some_and(u8::is_ascii_lowercase);
        whole.then_some(at)
    })
}

pub(super) fn planet_size(reading: &Reading<'_>) -> Option<&'static str> {
    let said = combined_lines(reading);
    let (first, reply) = super::SCIENCE_PLANET_SIZES
        .iter()
        .filter_map(|&(name, reply)| name_at(&said, name).map(|position| (position, reply)))
        .min()?;
    (!["sun", "moon", "earth"]
        .iter()
        .any(|body| name_at(&said, body).is_some_and(|position| position < first)))
    .then_some(reply)
}
