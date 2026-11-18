//! Quantities for the pinned materials cards and their dimension conventions.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Captures, Regex};

use crate::bot::{Exchange, Reading};

#[path = "landscape_materials_data.rs"]
mod data;
use data::{BULK_WEIGHT, LOOSE, NUMBERS, SLAB_BASE};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(PAIR, super::LANDSCAPE_PAIR);
pattern!(SQUARE, super::LANDSCAPE_SQUARE);
pattern!(FEET, super::LANDSCAPE_FEET);
pattern!(UNIT, super::LANDSCAPE_UNIT);
pattern!(DEPTH, super::LANDSCAPE_DEPTH);
pattern!(PERSONS_WEIGHT, super::LANDSCAPE_A_PERSONS_WEIGHT);
pattern!(MATERIAL_WEIGHT, super::LANDSCAPE_A_MATERIAL_WEIGHT);
pattern!(CONCRETE_JOB, super::LANDSCAPE_CONCRETE_JOB);
pattern!(
    SLAB_SIZE,
    r"(\d+(?:\.\d+)?)\s*(?:mm)?\s*(?:x|by)\s*(\d+(?:\.\d+)?)\s*(?:mm|cm)?"
);
pattern!(
    POST,
    r"\b(?:fence|gate|post|posts|hole|holes|postcrete|post mix|postmix)\b"
);

fn clean_or_typed<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn numbers(pair: &Captures<'_>) -> Option<(f64, f64)> {
    let one: f64 = pair[1].parse().ok()?;
    let two: f64 = pair[2].parse().ok()?;
    (one.is_finite() && two.is_finite()).then_some((one, two))
}

fn pair_sides(pair: &Captures<'_>) -> Option<(f64, f64)> {
    let (one, two) = numbers(pair)?;
    let metres = if FEET.is_match(&pair[0]) { 0.3048 } else { 1.0 };
    Some((one * metres, two * metres))
}

fn valid_area(value: f64) -> Option<f64> {
    (0.2..=20_000.0)
        .contains(&value)
        .then(|| round_tenth(value))
}

fn area(line: &str) -> Option<f64> {
    if let Some(pair) = PAIR.captures(line)
        && UNIT.is_match(&pair[0])
        && let Some((one, two)) = pair_sides(&pair)
        && let Some(value) = valid_area(one * two)
    {
        return Some(value);
    }
    let square = SQUARE.captures(line)?;
    let amount: f64 = square[1].parse().ok()?;
    valid_area(
        amount
            * if FEET.is_match(&square[0]) {
                0.0929
            } else {
                1.0
            },
    )
}

fn asked_area(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<f64> {
    area(clean_or_typed(reading)).or_else(|| {
        // Runtime exchanges record the bot text in `bot`; this source reader's old
        // `reply` key is absent from a completed exchange.
        earlier
            .iter()
            .rev()
            .take(3)
            .find_map(|turn| area(&turn.user))
    })
}

fn depth_mm(line: &str, default: f64) -> f64 {
    DEPTH
        .captures(line)
        .and_then(|measurement| {
            let amount: f64 = measurement[1].parse().ok()?;
            let factor = match &measurement[2] {
                "mm" => 1.0,
                "cm" => 10.0,
                _ => 25.0,
            };
            let depth = amount * factor;
            (10.0..=400.0).contains(&depth).then_some(depth)
        })
        .unwrap_or(default)
}

fn round_tenth(value: f64) -> f64 {
    format!("{value:.1}").parse().unwrap_or(value)
}

fn sizes(value: f64) -> String {
    let whole = value.round_ties_even();
    if (value - whole).abs() < 0.05 {
        format!("{whole:.0}")
    } else {
        format!("{value:.1}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
    }
}

fn count(value: f64) -> String {
    let word = if value >= 20.0 {
        format!("{:.0}", (value / 10.0).round_ties_even() * 10.0)
    } else if value >= 4.0 {
        format!("{:.0}", value.round_ties_even())
    } else {
        let halves = (value * 2.0).round_ties_even() / 2.0;
        if halves.fract() == 0.0 && halves >= 1.0 {
            NUMBERS[halves as usize].to_owned()
        } else {
            sizes(halves)
        }
    };
    format!("about {word}")
}

fn loose_count(bags: f64, unit: &str) -> String {
    let quarters = (bags * 4.0).round_ties_even() as usize;
    match quarters {
        0 => format!("about a tenth of a {unit}"),
        1 => format!("about a quarter of a {unit}"),
        2 => format!("about half a {unit}"),
        3 => format!("about three quarters of a {unit}"),
        38.. => {
            let whole = if bags < 20.0 {
                bags.round_ties_even()
            } else {
                (bags / 10.0).round_ties_even() * 10.0
            };
            format!("about {whole:.0} {unit}s")
        }
        _ => {
            let whole = quarters / 4;
            let part = quarters % 4;
            match part {
                0 if whole == 1 => format!("about one {unit}"),
                0 => format!("about {} {unit}s", NUMBERS[whole]),
                3 => format!("about {} {unit}s", NUMBERS[whole + 1]),
                _ => format!(
                    "about {} and {} {unit}s",
                    NUMBERS[whole],
                    if part == 1 { "a quarter" } else { "a half" }
                ),
            }
        }
    }
}

fn longest<'a>(names: &'a [&'a str], matches: impl Fn(&str) -> bool) -> Option<&'a str> {
    names
        .iter()
        .copied()
        .filter(|name| matches(name))
        .reduce(
            |best, term| {
                if term.len() > best.len() { term } else { best }
            },
        )
}

fn loose(index: usize, line: &str) -> String {
    let (names, depth, covers, lead, extra) = LOOSE[index];
    let mut thing = longest(names, |name| line.contains(name)).unwrap_or(names[0]);
    let mut lead = lead;
    if thing == "sand"
        && ["sandpit", "sand pit", "play sand"]
            .iter()
            .any(|item| line.contains(item))
    {
        thing = "play sand";
        lead = "A 25kg bag of play sand covers about 0.6 square metres at 50mm, so a 1.2m square sandpit filled to 150mm takes about 10 bags";
    }
    let lead = lead.replace("{thing}", thing);
    if let Some(area) = area(line) {
        let asked = depth_mm(line, depth);
        let per_bag = covers * (depth / asked);
        let unit = if names[0] == "sand" {
            "bag"
        } else {
            "bulk bag"
        };
        let bags = loose_count(area / per_bag, unit);
        return format!(
            "{} square metres at {}mm needs {bags} of {thing}. {lead}, and it is worth a tenth more than the sums say. {extra}",
            sizes(area),
            asked as u32
        );
    }
    format!(
        "{lead}. Measure the area and work out the volume at the depth you want, then divide by what a bag holds, and order a tenth more than the sums say - a bag never spreads as far as the label claims. {extra}"
    )
}

fn bulk_weight(reading: &Reading<'_>) -> String {
    let line = format!(" {} ", clean_or_typed(reading));
    let (what, figures) = BULK_WEIGHT
        .iter()
        .find_map(|(names, reply)| {
            longest(names, |name| {
                line.contains(&format!(" {name} ")) || line.contains(&format!(" {name}s "))
            })
            .map(|name| (name, *reply))
        })
        .unwrap_or((
            "gravel or sand",
            "about 850kg, which is about half a cubic metre",
        ));
    format!(
        "A bulk bag of {what} is {figures} - the bags sold as a tonne bag are filled to about 850kg for stone and sand, because that is what the bag and a lorry will take, so buy by the bag rather than by the tonne if the weight matters. That is the same figure the coverage sums use: 850kg of gravel covers about 10 square metres at 50mm. A 25kg bag is the same material in a bag you can lift, so 34 of them make a bulk bag."
    )
}

pub(super) fn not_a_persons_weight(reading: &Reading<'_>) -> bool {
    let line = clean_or_typed(reading);
    MATERIAL_WEIGHT.is_match(line) || !PERSONS_WEIGHT.is_match(line)
}

pub(super) fn is_concrete(reading: &Reading<'_>) -> bool {
    let line = format!(
        " {} {} ",
        reading.line.clean.to_lowercase(),
        reading.line.text.to_lowercase()
    );
    super::LANDSCAPE_CONCRETE_SURE
        .iter()
        .any(|term| line.contains(term))
        || CONCRETE_JOB.is_match(&line)
}

fn slabs(line: &str) -> String {
    let Some(area) = area(line) else {
        return format!(
            "Work it out at four 500mm slabs to the square metre, six of 400mm and five and a half of 600 x 300, then add a tenth for cuts and breakages. Under them: {SLAB_BASE}."
        );
    };
    let per_square = SLAB_SIZE
        .captures(line)
        .and_then(|size| numbers(&size))
        .and_then(|(one, two)| {
            let mm = |size| if size < 20.0 { size * 1000.0 } else { size };
            let (one, two) = (mm(one), mm(two));
            ((200.0..=1200.0).contains(&one) && (200.0..=1200.0).contains(&two))
                .then_some(1e6 / (one * two))
        })
        .unwrap_or(4.0);
    let side = (1000.0 / per_square.sqrt()).round_ties_even();
    format!(
        "{} square metres needs {} slabs of {side:.0}mm, working at {per_square:.1} to the square metre, with a tenth spare for cuts and breakages. Under them: {SLAB_BASE}.",
        sizes(area),
        count(area * per_square * 1.1)
    )
}

fn turf(line: &str) -> Cow<'static, str> {
    area(line).map_or(Cow::Borrowed("Turf rolls cover about a square metre each, so the area in square metres is the number of rolls, plus a few for the edges and any that go thin. Lay it within a day of delivery, water it the same evening, and keep off it for a fortnight."), |area| {
        Cow::Owned(format!("{} square metres is {} rolls of turf, ordered a few extra for the edges and any that go thin. Lay it within a day of delivery, water it the same evening, and keep off it for a fortnight.", sizes(area), count(area * 1.05)))
    })
}

#[derive(Clone, Copy)]
pub(super) enum Rule {
    Gravel,
    Topsoil,
    Compost,
    Sand,
    BulkWeight,
    Slabs,
    Turf,
    Concrete,
    Underlay,
    Grout,
    Adhesive,
    Flooring,
}

pub(super) fn answer(rule: Rule, reading: &Reading<'_>, earlier: &[Exchange]) -> Cow<'static, str> {
    let clean = &reading.line.clean;
    match rule {
        Rule::Gravel => Cow::Owned(loose(0, clean)),
        Rule::Topsoil => Cow::Owned(loose(1, clean)),
        Rule::Compost => Cow::Owned(loose(2, clean)),
        Rule::Sand => Cow::Owned(loose(3, clean)),
        Rule::BulkWeight => Cow::Owned(bulk_weight(reading)),
        Rule::Slabs => Cow::Owned(slabs(clean)),
        Rule::Turf => turf(clean),
        Rule::Concrete => Cow::Owned(concrete(clean, reading, earlier)),
        Rule::Underlay | Rule::Grout | Rule::Adhesive => {
            Cow::Owned(bags(rule, asked_area(reading, earlier)))
        }
        Rule::Flooring => flooring(clean),
    }
}

const UNDERLAY: &str = "Roll it across the room rather than along it, butt the seams without overlapping and tape the joins. Foam crumb is the cheap default; rubber crumb or felt is worth it under a thin carpet and over a hard floor.";
const GROUT: &str = "Mix it to the thickness of toothpaste and only as much as you can use in half an hour, because it goes off in the bucket.";
const ADHESIVE: &str = "Comb it out in one direction, press each tile on across the ridges, and lift one now and then to check the back is covered.";

fn bags(rule: Rule, area: Option<f64>) -> String {
    let Some(area) = area else {
        return match rule {
            Rule::Underlay => format!(
                "Underlay is sold by area: a roll is usually 15 square metres (1.37m by 11m), so measure the floor and add a tenth for the cuts. {UNDERLAY}"
            ),
            Rule::Grout => format!(
                "A 5kg bag of grout does about 4 to 5 square metres of 30cm tiles with 3mm joints, so a bathroom floor is one bag and a kitchen wall two; mosaic or wide joints take about twice as much for the same area. {GROUT}"
            ),
            Rule::Adhesive => format!(
                "A 20kg bag of tile adhesive covers about 4 to 5 square metres with a 6mm notched trowel, so a small bathroom wall is one bag; large tiles, a 10mm notch or back-buttering take it nearer 3 square metres a bag. {ADHESIVE}"
            ),
            _ => unreachable!(),
        };
    };
    let covers = if matches!(rule, Rule::Underlay) {
        15.0
    } else {
        4.5
    };
    let bags = (area * 1.1 / covers).ceil().max(1.0);
    let plural = if bags == 1.0 { "" } else { "s" };
    let bags = count(bags);
    let area = sizes(area);
    match rule {
        Rule::Underlay => format!(
            "{area} square metres needs {bags} roll{plural} of underlay, a roll being 15 square metres (1.37m by 11m), and the tenth extra covers the cuts round the edges and under the doorway. {UNDERLAY}"
        ),
        Rule::Grout => format!(
            "{area} square metres needs {bags} bag{plural} of grout, 5kg each: a bag does 4 to 5 square metres of 30cm tiles with 3mm joints, and small tiles or wide joints take about twice as much. {GROUT}"
        ),
        Rule::Adhesive => format!(
            "{area} square metres needs {bags} bag{plural} of tile adhesive, 20kg each: a bag covers 4 to 5 square metres with a 6mm notched trowel, and large tiles or a deeper notch take it nearer 3. {ADHESIVE}"
        ),
        _ => unreachable!(),
    }
}

fn flooring(line: &str) -> Cow<'static, str> {
    if [
        "skirting",
        "skirting board",
        "skirting boards",
        "coving",
        "dado rail",
        "dado rails",
    ]
    .iter()
    .any(|kind| line.contains(kind))
    {
        return match PAIR.captures(line).and_then(|pair| pair_sides(&pair)) {
            Some((one, two)) => {
                let around = 2.0 * (one + two);
                let boards = (around / 2.4).floor() + 1.0;
                Cow::Owned(format!(
                    "A room {one:.1}m by {two:.1}m has {around:.1} metres round it, so {:.1} metres once a couple of doorways are taken off - and boards come in 2.4m lengths, so buy {boards:.0} of them. Cut the joints where two boards meet over a straight bit of wall, because the joint shows less there than at a corner.",
                    around - 1.6
                ))
            }
            None => Cow::Borrowed(
                "Skirting, coving and dado rail are bought by the length, not the area: run a tape right round the room and take off the doorways, about 0.8m each. A doorway takes the skirting away and the coving carries on over it. Boards come in 2.4m lengths, so buy whole boards and join them over a straight run, and add one board for the corners and the cuts.",
            ),
        };
    }
    let Some(area) = area(line) else {
        return Cow::Borrowed(
            "Measure each room on the floor, not on the plan: length times width in metres, and an L-shaped room as two rectangles added together. Add a tenth for the cuts and the doorway, so 12 square metres of floor takes 13.2 square metres of carpet, or six packs of laminate at 2.2 square metres a pack. A carpet roll is 4m or 5m wide, so the shop wants the longest and the widest measurement of each room as well as the area, and a room wider than the roll needs a join.",
        );
    };
    let with_spare = area * 1.1;
    Cow::Owned(format!(
        "{} square metres needs {} square metres of carpet or flooring with the tenth spare for the cuts and the doorway, which is {} packs of laminate at about 2.2 square metres a pack. A carpet roll is 4m or 5m wide, which is why the shop asks for the longest and widest measurement of each room rather than the area alone: a room under 4m wide comes out of one piece, across the width of the roll, and a wider one needs a join. Keep the offcuts, because a spare pack for a repair later is worth more than the few pounds it costs.",
        sizes(area),
        sizes(round_tenth(with_spare)),
        count(with_spare / 2.2)
    ))
}

fn concrete(line: &str, reading: &Reading<'_>, earlier: &[Exchange]) -> String {
    const MIXING: &str = "Concrete is one part cement to six of ballast by volume - six shovels of ballast to one of cement, mixed dry first, then water until it holds together - and a cubic metre takes about 1.75 tonnes of ballast and six or seven 25kg bags of cement, which is two bulk bags of ballast.";
    if POST.is_match(line) {
        return format!(
            "One 20kg bag of Postcrete sets one post: the hole wants to be about 250 to 300mm across and 600mm (two foot) deep, with 50 to 75mm of gravel in the bottom for drainage. Stand the post, check it upright with a level on two sides, pour the dry mix in and add the water on top of it. A thicker post, a gate post or a hole much wider than that takes two bags. {MIXING}"
        );
    }
    if let Some(area) = asked_area(reading, earlier) {
        let depth = depth_mm(line, 100.0);
        let volume = area * (depth / 1000.0);
        return format!(
            "{} square metres at {}mm is {volume:.2} cubic metres of concrete: about {:.1} tonnes of ballast and {:.0} bags of cement, or {:.0} 25kg bags of ready-mixed concrete. {MIXING}",
            sizes(area),
            depth as u32,
            volume * 1.75,
            (volume * 6.5).ceil(),
            (volume * 2400.0 / 25.0).ceil()
        );
    }
    format!(
        "Work it out as a volume: length times width times depth in metres, so a base 100mm thick takes 0.1 cubic metres for every square metre, and a 6 by 8 foot shed is about 4.5 square metres - 0.45 cubic metres, which is one 850kg bulk bag of ballast and three 25kg bags of cement. A single fence post is one 20kg bag of postcrete. {MIXING}"
    )
}
