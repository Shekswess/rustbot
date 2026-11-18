//! Food keeping branches from `bot/knowledge/keeping.py`.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::storage_data as data;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(CUT_OUT, data::CUT_OUT);
pattern!(THAWING, data::THAWING);
pattern!(ASK_HOW_LONG, data::ASK_HOW_LONG);

fn has(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

pub(super) fn cold_cut(reading: &Reading<'_>) -> bool {
    !CUT_OUT.is_match(&reading.line.clean)
}

pub(super) fn cold_thawed(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    !THAWING.is_match(text) || !ASK_HOW_LONG.is_match(text)
}

pub(super) fn oats(reading: &Reading<'_>) -> &'static str {
    let text = format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.norm_text,
        reading.line.norm.join(" ")
    );
    if has(&text, &["cooked", "made"]) || (text.contains("porridge") && !text.contains("oats")) {
        data::OATS_COOKED
    } else {
        data::OATS_BAG
    }
}

fn all_views(reading: &Reading<'_>) -> String {
    format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.norm_text,
        reading.folded()
    )
}

fn ordered_pieces(
    text: &str,
    prefer_first: &[&str],
    first: &str,
    second: &str,
) -> Cow<'static, str> {
    if has(text, prefer_first) {
        Cow::Owned(format!("{first} {second}"))
    } else {
        Cow::Owned(format!("{second} {first}"))
    }
}

pub(super) fn mushroom(reading: &Reading<'_>) -> Cow<'static, str> {
    ordered_pieces(
        &all_views(reading),
        data::EATING_WORDS,
        data::MUSH_VERDICT,
        data::MUSH_FIX,
    )
}

pub(super) fn crunch(reading: &Reading<'_>) -> Cow<'static, str> {
    ordered_pieces(
        &all_views(reading),
        data::CRUNCH_SAFE,
        data::CRUNCH_VERDICT,
        data::CRUNCH_FIX,
    )
}

pub(super) fn yoghurt(reading: &Reading<'_>) -> Cow<'static, str> {
    if has(
        &all_views(reading),
        &["keep", "keeping", "stop", "runny", "watery", "separate"],
    ) {
        Cow::Owned(format!("{} {}", data::YOGHURT_FIX, data::YOGHURT_WHEY))
    } else {
        Cow::Borrowed(data::YOGHURT_WHEY)
    }
}

pub(super) fn flour(reading: &Reading<'_>) -> &'static str {
    if has(
        &all_views(reading),
        &[
            "weevil", "weevils", "mite", "mites", "bug", "bugs", "worm", "worms", "moving",
            "insects", "maggots",
        ],
    ) {
        data::FLOUR_MITES
    } else {
        data::FLOUR_LUMPS
    }
}

pub(super) fn cheese_mould(reading: &Reading<'_>, fallback: &'static str) -> Cow<'static, str> {
    let text = &reading.line.clean;
    if let Some(name) = data::BLUE_CHEESES.iter().find(|name| text.contains(**name)) {
        return Cow::Owned(format!(
            "The blue in {name} is meant to be there. Green or white fuzz on it is not, so a blue cheese that has grown it goes in the bin."
        ));
    }
    if let Some(name) = data::SOFT_CHEESES.iter().find(|name| text.contains(**name)) {
        return Cow::Owned(format!(
            "Mould on {name} means the whole piece goes in the bin: the threads run all the way through a soft cheese, so there is nothing to cut back to."
        ));
    }
    Cow::Borrowed(fallback)
}

pub(super) fn sauce(
    reading: &Reading<'_>,
    replies: &'static [&'static str],
) -> Option<&'static str> {
    let text = format!(" {} ", reading.line.clean);
    let index = if has(&text, &[" freeze", " freezer", " frozen", " freezing"]) {
        0
    } else if has(
        &text,
        &[
            " carton",
            " cartons",
            " tub",
            " fresh pasta sauce",
            " chilled sauce",
            " fresh sauce",
        ],
    ) {
        1
    } else if has(
        &text,
        &[
            " homemade",
            " made it",
            " made my own",
            " made this",
            " batch",
            " i made",
            " my own sauce",
            " left over from",
            " from scratch",
        ],
    ) {
        2
    } else {
        3
    };
    replies.get(index).copied()
}

pub(super) struct ShelfLife {
    pub(super) names: &'static [&'static str],
    pub(super) reply: &'static str,
    pub(super) frozen: Option<&'static str>,
}

fn row_for(text: &str, rows: &'static [ShelfLife]) -> Option<&'static ShelfLife> {
    rows.iter()
        .enumerate()
        .filter_map(|(index, row)| {
            row.names
                .iter()
                .find(|name| text.contains(&format!(" {name} ")))
                .map(|name| (name.chars().count(), std::cmp::Reverse(index), row))
        })
        .max_by_key(|(name_size, index, _)| (*name_size, *index))
        .map(|(_, _, found)| found)
}

pub(super) fn cupboard(reading: &Reading<'_>, fallback: &'static str) -> &'static str {
    let text = format!(" {} ", reading.line.clean);
    let frozen = has(&text, &["freezer", "freeze", "frozen", "defrost", "thaw"]);
    row_for(&text, data::CUPBOARD).map_or(fallback, |row| {
        if frozen {
            row.frozen.unwrap_or(row.reply)
        } else {
            row.reply
        }
    })
}

pub(super) fn freeze_sweet(reading: &Reading<'_>) -> Option<&'static str> {
    let text = format!(" {} ", reading.line.clean);
    row_for(&text, data::SWEETS).map(|row| row.reply)
}

pub(super) fn names_produce(reading: &Reading<'_>) -> bool {
    let line = reading
        .line
        .clean
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let line = format!(" {line} ");
    data::OFF_NAMES
        .iter()
        .any(|name| line.contains(&format!(" {name} ")))
}

fn named_produce(text: &str, names: &[&'static str], categories: bool) -> Option<&'static str> {
    let (_, name) = names
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, name)| text.contains(&format!(" {name} ")))
        .max_by_key(|(index, name)| (name.chars().count(), std::cmp::Reverse(*index)))?;
    (categories || !data::OFF_GROUP_WORDS.contains(&name)).then_some(name)
}

fn one(name: &str) -> Cow<'_, str> {
    if let Some(end) = name.strip_suffix("ies") {
        return Cow::Owned(format!("{end}y"));
    }
    if ["oes", "ches", "shes", "sses"]
        .iter()
        .any(|ending| name.ends_with(ending))
    {
        return Cow::Borrowed(&name[..name.len() - 2]);
    }
    if matches!(name, "leaves" | "leafes") {
        return Cow::Borrowed("leaf");
    }
    Cow::Borrowed(if name.ends_with("ss") {
        name
    } else {
        name.strip_suffix('s').unwrap_or(name)
    })
}

pub(super) fn spoiled_produce(reading: &Reading<'_>, fallback: &'static str) -> Cow<'static, str> {
    let text = format!(" {} ", reading.line.clean);
    if let Some(berry) = named_produce(&text, data::OFF_BERRIES, false) {
        return Cow::Owned(format!(
            "Bin every {} with fuzz on it and the ones touching it, then wash the rest and eat them today. On a berry the mould threads run under the skin where you cannot see them, so there is nothing to cut back to, and a closed punnet passes it on fast. Spread what is sound on kitchen paper in the fridge and it will hold a day or two.",
            one(berry)
        ));
    }
    if named_produce(&text, data::OFF_LEAVES, true).is_some() {
        return Cow::Borrowed(
            "Pick out the slimy or furry leaves and bin them: that slime is bacteria, not water, and cooking will not make it safe to eat. Wash the rest well and eat or cook them today, because in a closed bag it spreads to whatever touches it. Dry the next lot in a tea towel and keep them in a bag with a sheet of kitchen paper, because wet leaves in a sealed bag are what went wrong in the first place.",
        );
    }
    if let Some(citrus) = named_produce(&text, data::OFF_CITRUS, false) {
        return Cow::Owned(format!(
            "Bin the {citrus} and check the rest of the bag. Mould on citrus starts on the skin and the flesh under it is soft and wet, so there is nothing worth cutting back to - and dry the drawer and the bag, because a damp one passes it on. The sound ones keep a couple of weeks loose in the fridge."
        ));
    }
    if let Some(soft) = named_produce(&text, data::OFF_SOFT, false) {
        let (start, pronoun) = if one(soft) == soft {
            ("It is", "it")
        } else {
            ("They are", "them")
        };
        return Cow::Owned(format!(
            "Bin the {soft}. {start} mostly water, so the mould threads run right through {pronoun} and there is nothing to cut back to, and cooking does not make it safe again. A soft patch with no mould on it is just older - trim that away and cook the rest today."
        ));
    }
    if let Some(firm) = named_produce(&text, data::OFF_FIRM, false) {
        return Cow::Owned(format!(
            "Cut the mould off with an inch to spare and use the rest, unless the flesh under it is soft or smells wrong. A firm fruit or root does not carry the threads deep, so a small patch on an apple or a carrot is not the whole of it - and bin the {firm}, because it spreads to what it touches in a drawer."
        ));
    }
    Cow::Borrowed(fallback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeping_uses_the_state_and_the_named_food() {
        assert!(oats(&Reading::new("how long do dry oats keep")).starts_with("Porridge oats keep"));
        assert!(
            oats(&Reading::new("how long does cooked porridge keep"))
                .starts_with("Cooked porridge")
        );
        assert!(
            cheese_mould(&Reading::new("there is mould on my brie"), "hard cheese")
                .starts_with("Mould on brie")
        );
        assert!(!cold_thawed(&Reading::new(
            "how long do i need to thaw chicken"
        )));
        assert!(!cold_cut(&Reading::new("i am not cut out for this job")));
        let fruit = Reading::new("all of my tomatoes have gone mouldy");
        assert!(names_produce(&fruit));
        assert!(spoiled_produce(&fruit, "").starts_with("Bin the tomatoes. They are"));
    }
}
