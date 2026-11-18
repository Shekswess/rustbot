//! Material counts from the dimensions in this turn or the last four user turns.

use super::guards::Context;
use crate::bot::Reading;
use regex::{Captures, Regex};
use std::{borrow::Cow, sync::LazyLock};

use super::{
    MATERIAL_AS_LENGTH, MATERIAL_AS_SQUARE, MATERIAL_CAME_AWAY, MATERIAL_DOUBLE_DUTY,
    MATERIAL_HIGH_LONG, MATERIAL_JOB_WORDS, MATERIAL_LONG_HIGH, MATERIAL_LOOKING_BACK,
    MATERIAL_OURS, MATERIAL_SIDE_BY_SIDE, MATERIAL_SUBJECT_MODAL, MATERIAL_UNIT_WORDS,
};

macro_rules! pattern {
    ($name:ident, $value:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

pattern!(SIDE_BY_SIDE, MATERIAL_SIDE_BY_SIDE);
pattern!(LONG_HIGH, MATERIAL_LONG_HIGH);
pattern!(HIGH_LONG, MATERIAL_HIGH_LONG);
pattern!(SQUARE, MATERIAL_AS_SQUARE);
pattern!(LENGTH, MATERIAL_AS_LENGTH);
pattern!(PARTITIVE, r"\b([a-z]+)\s+of\s+([a-z]+)");
pattern!(CONTAINER, r"\bin\s+(?:a|an|the|one)\s+([a-z]+)");
pattern!(IN_BOXES, r"\b(?:in|per|a|one) box\b|\bboxes\b");
pattern!(PANELS, r"\b(\d+)\s+(?:fence\s+|6ft\s+)?panels?\b");
pattern!(BLOCKS, r"\bblocks?\b");
pattern!(CEILING_OR_ROOF, r"\bceilings?\b|\broof\b");
pattern!(CEILING_BEFORE, super::MATERIAL_CEILING_BEFORE);
pattern!(CEILING_AFTER, super::MATERIAL_CEILING_AFTER);
pattern!(CEILING, super::MATERIAL_CEILING_THING);
pattern!(ONE_SURFACE, super::MATERIAL_ONE_SURFACE);
pattern!(ROOM, super::MATERIAL_ROOM);
pattern!(ROOM_WALLS, r"\brooms?\b|\bwalls?\b");

static DOING_WORDS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    MATERIAL_DOUBLE_DUTY
        .iter()
        .map(|word| Regex::new(&format!(r"\b{word}(?:s|es|ed|ing)?\b")).unwrap())
        .collect()
});

#[derive(Clone, Copy)]
pub(super) enum Material {
    Tiles,
    Decking,
    Fence,
    Bricks,
    Floorboards,
    Skirting,
    Plasterboard,
    Paint,
    Wallpaper,
}

fn line(reading: &Reading<'_>) -> String {
    [&reading.line.clean, &reading.line.plain]
        .into_iter()
        .find(|value| !value.is_empty())
        .map_or(reading.raw, String::as_str)
        .to_lowercase()
}

fn material_word(word: &str) -> bool {
    MATERIAL_OURS.contains(&word)
        || (word.len() > 3
            && word
                .strip_suffix('s')
                .is_some_and(|stem| MATERIAL_OURS.contains(&stem)))
        || (word.len() > 4
            && word
                .strip_suffix("es")
                .is_some_and(|stem| MATERIAL_OURS.contains(&stem)))
}

pub(super) fn is_subject(reading: &Reading<'_>) -> bool {
    let said = line(reading);
    if MATERIAL_CAME_AWAY
        .iter()
        .chain(MATERIAL_LOOKING_BACK.iter())
        .any(|phrase| said.contains(phrase))
    {
        return false;
    }
    for word in DOING_WORDS.iter() {
        for hit in word.find_iter(&said) {
            if said[..hit.start()]
                .split_whitespace()
                .next_back()
                .is_some_and(|before| MATERIAL_SUBJECT_MODAL.contains(&before))
                && !word.is_match(&format!("{} {}", &said[..hit.start()], &said[hit.end()..]))
            {
                return false;
            }
        }
    }
    if PARTITIVE.captures_iter(&said).any(|found| {
        let holder = &found[1];
        (MATERIAL_UNIT_WORDS.contains(&holder) || material_word(holder))
            && !material_word(&found[2])
    }) {
        return false;
    }
    !CONTAINER.captures_iter(&said).any(|found| {
        let holder = &found[1];
        !MATERIAL_UNIT_WORDS.contains(&holder)
            && !MATERIAL_JOB_WORDS.contains(&holder)
            && !MATERIAL_OURS.contains(&holder)
    })
}

fn number(raw: &str) -> Option<f64> {
    let value = match raw {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        "eleven" => 11,
        "twelve" => 12,
        "twenty" => 20,
        "thirty" => 30,
        "forty" => 40,
        "fifty" => 50,
        "sixty" => 60,
        _ => {
            return raw
                .parse::<f64>()
                .ok()
                .filter(|x| x.is_finite() && *x != 0.0);
        }
    };
    Some(f64::from(value))
}

fn ratio(unit: &str) -> f64 {
    match unit {
        "mm" => 0.001,
        "cm" => 0.01,
        "ft" | "feet" | "foot" => 0.3048,
        "in" | "inch" | "inches" => 0.0254,
        "yd" | "yard" | "yards" => 0.9144,
        _ => 1.0,
    }
}

fn group<'a>(found: &Captures<'a>, index: usize) -> &'a str {
    found.get(index).map_or("", |value| value.as_str())
}

fn pair(found: &Captures<'_>, share_unit: bool) -> Option<(f64, f64)> {
    let (unit1, unit2) = (group(found, 2), group(found, 4));
    let second_unit = if share_unit && unit2.is_empty() {
        unit1
    } else {
        unit2
    };
    Some((
        number(group(found, 1))? * ratio(unit1),
        number(group(found, 3))? * ratio(second_unit),
    ))
}

fn rounded(value: f64, decimals: usize) -> f64 {
    format!("{value:.decimals$}").parse().unwrap_or(value)
}

#[derive(Default)]
struct Measure {
    area: Option<f64>,
    run: Option<f64>,
}

impl Measure {
    fn is_empty(&self) -> bool {
        self.area.is_none_or(|area| area == 0.0) && self.run.is_none_or(|run| run == 0.0)
    }
}

fn measures(text: &str) -> Measure {
    let valid = |n| (0.3..=40.0).contains(&n);
    for found in SIDE_BY_SIDE.captures_iter(text) {
        if let Some((first, second)) = pair(&found, true)
            && valid(first)
            && valid(second)
        {
            return Measure {
                area: Some(rounded(first * second, 2)),
                run: Some(rounded(2.0 * (first + second), 2)),
            };
        }
    }
    for (pattern, reverse) in [(&*LONG_HIGH, false), (&*HIGH_LONG, true)] {
        if let Some(found) = pattern.captures(text)
            && let Some((first, second)) = pair(&found, false)
            && valid(first)
            && valid(second)
        {
            return Measure {
                area: Some(rounded(first * second, 2)),
                run: Some(rounded(if reverse { second } else { first }, 2)),
            };
        }
    }
    if let Some(found) = SQUARE.captures(text)
        && let Some(value) = number(&found[1])
    {
        let unit = group(&found, 2);
        return Measure {
            area: Some(rounded(value * ratio(unit).powi(2), 2)),
            run: None,
        };
    }
    if let Some(found) = LENGTH.captures(text)
        && let Some(value) = number(&found[1])
    {
        return Measure {
            area: None,
            run: Some(rounded(value * ratio(&found[2]), 2)),
        };
    }
    Measure::default()
}

fn piece_size(text: &str) -> Option<(f64, f64)> {
    for found in SIDE_BY_SIDE.captures_iter(text) {
        let Some((mut first, mut second)) = pair(&found, true) else {
            continue;
        };
        if group(&found, 2).is_empty()
            && group(&found, 4).is_empty()
            && 1.5 < first
            && first <= 3000.0
        {
            first /= 1000.0;
            second /= 1000.0;
        }
        if (0.05..=1.5).contains(&first) && (0.05..=1.5).contains(&second) {
            return Some((first, second));
        }
    }
    None
}

fn whole(value: f64) -> String {
    let digits = format!("{value:.0}");
    let mut grouped = String::new();
    for (index, ch) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(ch);
    }
    grouped
}

fn counted(number: f64, noun: &str, plural: &str) -> String {
    if number.round_ties_even() == 1.0 {
        format!("1 {noun}")
    } else {
        format!("{} {plural}", whole(number))
    }
}

fn boxes(tiles: f64) -> String {
    let boxes = (tiles / 12.0).ceil();
    let prefix = if boxes > 1.0 { "around " } else { "" };
    format!("{prefix}{} of a dozen", counted(boxes, "box", "boxes"))
}

fn piece(size: (f64, f64)) -> String {
    let dimension = |n| {
        if n < 1.0 {
            format!("{}mm", whole(n * 1000.0))
        } else {
            format!("{n}m")
        }
    };
    format!("{} by {}", dimension(size.0), dimension(size.1))
}

fn about_area(area: f64) -> String {
    if format!("{area:.1}") == "1.0" {
        "One square metre".to_owned()
    } else if area.fract() != 0.0 {
        format!("{area} square metres")
    } else {
        format!("{} square metres", whole(area))
    }
}

pub(super) fn answer(material: Material, context: &Context<'_>) -> Cow<'static, str> {
    let mut clean = line(context.reading);
    let mut measure = measures(&clean);
    let prior = if measure.is_empty() {
        context
            .earlier
            .iter()
            .rev()
            .take(4)
            .map(|turn| turn.user.trim().to_lowercase())
            .find(|prior| !measures(prior).is_empty())
    } else {
        None
    };
    if matches!(material, Material::Paint | Material::Wallpaper) {
        let inherited = prior.is_some();
        let surface = prior.as_deref().unwrap_or(&clean);
        let area = wall_area(surface);
        return match material {
            Material::Paint => paint(area),
            Material::Wallpaper => wallpaper(surface, inherited, area),
            _ => unreachable!("material has a separate calculation"),
        };
    }
    if let Some(prior) = prior {
        clean = format!("{clean} {prior}").trim().to_owned();
        measure = measures(&clean);
    }
    let area = measure.area.filter(|area| *area != 0.0);
    let run = measure.run.filter(|run| *run != 0.0);
    match material {
        Material::Tiles => tiles(&clean, area),
        Material::Decking => decking(&clean, area),
        Material::Fence => fence(&clean, run),
        Material::Bricks => bricks(&clean, area),
        Material::Floorboards => floorboards(area),
        Material::Skirting => skirting(run),
        Material::Plasterboard => plasterboard(&clean, area),
        Material::Paint | Material::Wallpaper => unreachable!("surface already evaluated"),
    }
}

fn tiles(clean: &str, area: Option<f64>) -> Cow<'static, str> {
    let size = piece_size(clean);
    let in_boxes = IN_BOXES.is_match(clean);
    match (area, size) {
        (Some(area), Some(size)) => {
            let count = area / (size.0 * size.1);
            let buy = (count * 1.1).ceil();
            Cow::Owned(format!(
                "{} of {} tiles is {}, so buy {} for the cuts - {}. Line the cuts up in the corner nobody looks at and check a spare tile is left over for a repair.",
                about_area(area),
                piece(size),
                counted(count.round_ties_even(), "tile", "tiles"),
                counted(buy, "tile", "tiles"),
                boxes(buy),
            ))
        }
        (Some(area), None) if in_boxes => Cow::Owned(format!(
            "A box says the square metres it covers on the label, and most cover about one, so {} is about {} boxes, or {} with the cuts. A 600mm tile comes three to the square metre, so a box of those covers more and the count drops: read the m2 on the box rather than the number of tiles in it.",
            about_area(area).to_lowercase(),
            whole(area.ceil()),
            whole((area * 1.1).ceil()),
        )),
        (None, Some(size)) => Cow::Owned(format!(
            "A {} tile is about {} to the square metre, so add 10% to the area for the cuts. How big is the wall or floor - length and width in metres?",
            piece(size),
            whole((1.0 / (size.0 * size.1)).ceil()),
        )),
        (None, None) if in_boxes => Cow::Borrowed(
            "A box of tiles covers about a square metre - it says the m2 on the box - so the count follows from the tile size: a 300mm tile is about 11 to the square metre, and a 600mm one just under three. Buy 10% more than the area needs for the cuts at the edges. How big is the wall or floor?",
        ),
        (None, None) => Cow::Borrowed(
            "A 300mm tile covers about a tenth of a square metre, so count about 11 to the square metre and add 10% for cuts. How big is the wall or floor - length and width in metres?",
        ),
        (Some(area), None) => {
            let count = area * 11.1;
            let buy = (count * 1.1).ceil();
            Cow::Owned(format!(
                "{} at about 11 tiles to the square metre is {}, so buy {} to allow for cuts - {}. A 600mm tile is just under three to the square metre, so that size needs about a quarter of the count.",
                about_area(area),
                counted(count.round_ties_even(), "tile", "tiles"),
                counted(buy, "tile", "tiles"),
                boxes(buy),
            ))
        }
    }
}

static SURFACES: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    super::MATERIAL_SURFACES
        .iter()
        .map(|&(word, surface)| (Regex::new(&format!(r"\b{word}\b")).unwrap(), surface))
        .collect()
});

fn decking(clean: &str, area: Option<f64>) -> Cow<'static, str> {
    let Some(area) = area else {
        let surface = SURFACES
            .iter()
            .find_map(|(word, name)| word.is_match(clean).then_some(*name))
            .unwrap_or("deck");
        return Cow::Owned(format!(
            "A 4.8m board of 145mm decking covers about 0.7 of a square metre, so a square metre is about a board and a half. How big is the {surface} - length and width in metres?",
        ));
    };
    let count = area / 0.696;
    Cow::Owned(format!(
        "{} at about 0.7 of a square metre a board is {} boards of 4.8m, so buy {} with the cuts. The joists are the other count: at 400mm centres a 4m by 3m deck needs about 9 runs, and the boards run across them with a 5mm gap.",
        about_area(area),
        whole(count.round_ties_even()),
        whole((count * 1.1).ceil()),
    ))
}

fn fence(clean: &str, run: Option<f64>) -> Cow<'static, str> {
    if let Some(run) = run {
        let panels = (run / 1.83).ceil();
        return Cow::Owned(format!(
            "{} metres at 1.83m for a standard 6ft panel is {} panels, and {} posts if the run starts and ends on one. Put a gravel board under each panel so the wood does not sit in the wet, and set the posts 600mm into the ground in concrete.",
            whole(run),
            whole(panels),
            whole(panels + 1.0),
        ));
    }
    if let Some(found) = PANELS.captures(clean)
        && let Ok(panels) = found[1].parse::<u64>()
    {
        let panels = panels.max(1) as f64;
        let run = panels * 1.83;
        let tenths = rounded(run, 1);
        return Cow::Owned(format!(
            "{} panels at 1.83m is {} metres of run, so you want {} posts if it starts and ends on one. Set each post 600mm into the ground in concrete and put a gravel board under every panel so the wood does not sit in the wet.",
            whole(panels),
            whole(tenths),
            whole(panels + 1.0),
        ));
    }
    Cow::Borrowed(
        "A standard 6ft fence panel is 1.83m wide, so 20 metres needs 11 panels and 12 posts. How long is the run in metres?",
    )
}

fn bricks(clean: &str, area: Option<f64>) -> Cow<'static, str> {
    let blocks = BLOCKS.is_match(clean);
    match (area, blocks) {
        (None, true) => Cow::Borrowed(
            "A concrete block is 440 by 215mm, so a square metre of single-skin wall is about 10 blocks. How big is the wall - length and height in metres?",
        ),
        (None, false) => Cow::Borrowed(
            "A standard brick with its mortar joint covers about 60 to the square metre of single-skin wall. How big is the wall - length and height in metres?",
        ),
        (Some(area), true) => {
            let count = area * 10.0;
            Cow::Owned(format!(
                "{} of single-skin wall at about 10 blocks to the square metre is {} blocks, so order {} with the waste. A cavity wall takes two skins, so double it and add wall ties.",
                about_area(area),
                whole(count.round_ties_even()),
                whole((count * 1.05).ceil()),
            ))
        }
        (Some(area), false) => {
            let count = area * 60.0;
            Cow::Owned(format!(
                "{} of single-skin wall at about 60 bricks to the square metre is {} bricks, so order {} with the waste - and double it for a cavity wall. A 25kg bag of mortar mix lays about 25 bricks.",
                about_area(area),
                whole(count.round_ties_even()),
                whole((count * 1.05).ceil()),
            ))
        }
    }
}

fn floorboards(area: Option<f64>) -> Cow<'static, str> {
    let Some(area) = area else {
        return Cow::Borrowed(
            "A 2.4m floorboard 120mm wide covers about 0.29 of a square metre, so a square metre is about three and a half boards. How big is the room - length and width in metres?",
        );
    };
    let count = area / 0.288;
    Cow::Owned(format!(
        "{} at about 0.29 of a square metre a board is {} boards of 2.4m by 120mm, so buy {} for cuts. Run them across the joists and leave an 8 to 10mm gap at each wall for movement.",
        about_area(area),
        whole(count.round_ties_even()),
        whole((count * 1.1).ceil()),
    ))
}

fn skirting(run: Option<f64>) -> Cow<'static, str> {
    let Some(run) = run else {
        return Cow::Borrowed(
            "Skirting comes in 2.4m lengths, so add up the room's perimeter and divide: a 4m by 3m room is 14m round, which is six lengths. How big is the room - length and width in metres?",
        );
    };
    let lengths = (run / 2.4).ceil();
    Cow::Owned(format!(
        "{} metres round at 2.4m a length is {} lengths, so buy {} to allow for the mitres and a mistake. Work the perimeter out as twice (length + width), and add a length for each doorway you have to cut round.",
        whole(run),
        whole(lengths),
        whole(lengths + 1.0),
    ))
}

fn plasterboard(clean: &str, area: Option<f64>) -> Cow<'static, str> {
    let Some(area) = area else {
        return Cow::Borrowed(
            "A plasterboard sheet is 2.4m by 1.2m, so just under three square metres, and you want about 10% more than the wall for the cuts and the joints. How big is the wall or ceiling - length and width in metres?",
        );
    };
    let thick = if CEILING_OR_ROOF.is_match(clean) {
        "12.5mm"
    } else {
        "9.5mm"
    };
    Cow::Owned(format!(
        "{} is {} sheets of 2.4m by 1.2m {thick} plasterboard once it is covered, so buy {} with the cuts at the edges and keep the offcut for a patch. Stagger the joints, leave a 3mm gap between the sheets, and screwing it into the joists or studs holds better than gluing it on its own.",
        about_area(area),
        whole(area / 2.88),
        whole((area * 1.1 / 2.88).ceil()),
    ))
}

enum WallArea {
    Wall(f64),
    Ceiling(f64),
    Room { area: f64, ceiling: f64 },
    Unknown,
}

fn ceiling_height(text: &str) -> Option<f64> {
    for pattern in [&*CEILING_BEFORE, &*CEILING_AFTER] {
        if let Some(found) = pattern.captures(text)
            && let Some(value) = number(group(&found, 1))
        {
            let height = value * ratio(group(&found, 2));
            if (1.8..=5.0).contains(&height) {
                return Some(rounded(height, 2));
            }
        }
    }
    None
}

fn wall_area(text: &str) -> WallArea {
    let measure = measures(text);
    let Some(area) = measure.area.filter(|area| *area != 0.0) else {
        return WallArea::Unknown;
    };
    let Some(run) = measure.run else {
        return WallArea::Wall(area);
    };
    if LONG_HIGH.is_match(text) || HIGH_LONG.is_match(text) {
        return WallArea::Wall(area);
    }
    let height = ceiling_height(text);
    if CEILING.is_match(text) && height.is_none() && !ROOM_WALLS.is_match(text) {
        return WallArea::Ceiling(area);
    }
    if ONE_SURFACE.is_match(text) && !ROOM.is_match(text) {
        return WallArea::Wall(area);
    }
    let ceiling = height.unwrap_or(2.4);
    WallArea::Room {
        area: rounded(run * ceiling, 2),
        ceiling,
    }
}

fn wall_dimensions(text: &str, inherited: bool) -> Option<(f64, f64)> {
    if !inherited
        && ((CEILING.is_match(text) && ceiling_height(text).is_none())
            || !ONE_SURFACE.is_match(text)
            || ROOM.is_match(text))
    {
        return None;
    }
    let valid = |n| (0.3..=40.0).contains(&n);
    for found in SIDE_BY_SIDE.captures_iter(text) {
        if let Some((first, second)) = pair(&found, true)
            && valid(first)
            && valid(second)
        {
            return Some((rounded(first, 2), rounded(second, 2)));
        }
    }
    for (pattern, reverse) in [(&*LONG_HIGH, false), (&*HIGH_LONG, true)] {
        if let Some(found) = pattern.captures(text)
            && let Some((first, second)) = pair(&found, false)
            && valid(first)
            && valid(second)
        {
            return Some(if reverse {
                (rounded(second, 2), rounded(first, 2))
            } else {
                (rounded(first, 2), rounded(second, 2))
            });
        }
    }
    None
}

fn noun(value: f64, singular: &str) -> String {
    if rounded(value, 1) == 1.0 {
        format!("{value} {singular}")
    } else {
        format!("{value} {singular}s")
    }
}

fn paint(surface: WallArea) -> Cow<'static, str> {
    let (area, place) = match surface {
        WallArea::Unknown => return Cow::Borrowed(super::MATERIAL_PAINT_WITH_NO_SIZE),
        WallArea::Wall(area) => (area, "wall"),
        WallArea::Ceiling(area) => (area, "ceiling"),
        WallArea::Room { area, .. } => (area, "room"),
    };
    let litres = ((area * 2.0 / 11.0 * 2.0 + 0.999).floor() / 2.0).max(1.0);
    let area = rounded(area, 1);
    let area_name = noun(area, "square metre");
    let coats = rounded(area * 2.0, 1);
    let detail = if let WallArea::Room { ceiling, .. } = surface {
        format!(
            "A room that size is {} round and {} to the ceiling, so about {area_name} of wall, and two coats makes {coats}. Take off roughly 1.7 square metres for a door and 1.2 for a window if they are in it, allow a third coat when a dark colour goes over a light one, and give the ceiling and any second colour their own sum.",
            noun(rounded(area / ceiling, 1), "metre"),
            noun(ceiling, "metre")
        )
    } else if place == "ceiling" {
        format!(
            "A ceiling that size is {area_name}, two coats makes {coats}, and a litre of emulsion covers 10 to 12. Give the walls their own sum, and go a coat further when a white ceiling is going over a colour rather than over white."
        )
    } else {
        format!(
            "That's {area_name}, two coats makes {coats}, and a litre of emulsion covers 10 to 12. Allow a third coat when a dark colour goes over a light one, and give the ceiling and any second colour their own sum."
        )
    };
    Cow::Owned(format!("About {}: {detail}", noun(litres, "litre")))
}

fn wallpaper(text: &str, inherited: bool, surface: WallArea) -> Cow<'static, str> {
    let (width, height, wall) = match surface {
        WallArea::Wall(_) => {
            let Some((width, height)) = wall_dimensions(text, inherited) else {
                return Cow::Borrowed(super::MATERIAL_WALLPAPER_WITH_NO_SIZE);
            };
            (width, height, true)
        }
        WallArea::Room { area, ceiling } => (area / ceiling, ceiling, false),
        _ => return Cow::Borrowed(super::MATERIAL_WALLPAPER_WITH_NO_SIZE),
    };
    let drops = (width / 0.52).ceil();
    let plain = (10.0 / height).floor().max(1.0);
    let patterned = (10.0 / (height + 0.3)).floor().max(1.0);
    let plain_rolls = (drops / plain).ceil();
    let patterned_rolls = (drops / patterned).ceil();
    if wall {
        return Cow::Owned(format!(
            "About {}: a wall {} wide and {} high takes {} at the 52cm width of a standard roll, and a 10m roll gives {} of a plain paper or {} of one with a big repeat - so {} plain and {} patterned. Buy one extra for a mistake and matching, and keep the same batch number on every roll.",
            noun(plain_rolls, "roll"),
            noun(rounded(width, 1), "metre"),
            noun(rounded(height, 1), "metre"),
            noun(drops, "drop"),
            noun(plain, "drop"),
            noun(patterned, "drop"),
            noun(plain_rolls, "roll"),
            noun(patterned_rolls, "roll"),
        ));
    }
    let length = significant(width);
    Cow::Owned(format!(
        "About {plain_rolls} rolls: a room that size is {length} metres round and {height} metres to the ceiling, so {drops} drops at the 52cm width of a standard roll, and a 10m roll gives {plain} drops of a plain paper or {patterned} of one with a big repeat - so {plain_rolls} rolls plain and {patterned_rolls} patterned. Buy one extra for a mistake and matching, and keep the same batch number on every roll."
    ))
}

fn significant(value: f64) -> f64 {
    let digits = value.abs().log10().floor() as isize + 1;
    rounded(value, 6isize.saturating_sub(digits).max(0) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn separates_tile_and_floor_dimensions_and_carries_the_previous_size() {
        let turns = [Exchange {
            user: "The floor is 3m by 2m.".to_owned(),
            bot: String::new(),
            module: "answers",
        }];
        for text in [
            "How many 600 by 600 tiles do I need?",
            "How many 600 by 600 tiles for a 3 by 2 metre floor?",
        ] {
            let reading = Reading::new(text);
            assert!(is_subject(&reading));
            let reply = answer(Material::Tiles, &Context::new(&reading, &turns));
            assert!(
                reply.starts_with(
                    "6 square metres of 600mm by 600mm tiles is 17 tiles, so buy 19 tiles"
                ),
                "{reply}"
            );
        }
        let current = Reading::new("How much skirting for a 4m by 3m room?");
        assert!(
            answer(Material::Skirting, &Context::new(&current, &turns))
                .starts_with("14 metres round"),
        );
    }

    #[test]
    fn guard_yields_for_containers_verb_uses_and_past_counts() {
        for text in [
            "How many blocks of butter do I need?",
            "How many bricks are in a Lego set?",
            "Should I tile the hallway?",
            "How many tiles fell off the roof?",
        ] {
            assert!(!is_subject(&Reading::new(text)), "{text}");
        }
    }

    #[test]
    fn paint_and_paper_distinguish_room_wall_and_ceiling() {
        for (material, text, prefix) in [
            (
                Material::Paint,
                "How much paint for a 4m by 3m ceiling?",
                "About 2.5 litres: A ceiling that size is 12 square metres",
            ),
            (
                Material::Paint,
                "How much paint for a room 4m by 3m with 2.7m ceilings?",
                "About 7 litres: A room that size is 14 metres round and 2.7 metres to the ceiling",
            ),
            (
                Material::Wallpaper,
                "How much wallpaper for a 1m by 1m wall?",
                "About 1 roll: a wall 1 metre wide and 1 metre high takes 2 drops",
            ),
        ] {
            let reading = Reading::new(text);
            let reply = answer(material, &Context::new(&reading, &[]));
            assert!(reply.starts_with(prefix), "{reply}");
        }
    }
}
