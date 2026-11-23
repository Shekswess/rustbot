// Changed in the cppalliance fork of RustBot: the time comes from the turn. See "Changes in this fork" in README.md.

//! Month- and crop-specific garden replies from the source tables.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::{
    GARDEN_CROP_WEEKS, GARDEN_CROP_WEEKS_REST, GARDEN_GARDEN_JOBS, GARDEN_GARDEN_NOW,
    GARDEN_GARDEN_THINGS, GARDEN_MONTH_REPLIES, GARDEN_NEXT_MONTH_WORDS, GARDEN_NOW_WORDS,
    GARDEN_SEASON_REPLIES, GARDEN_SHADE_CROPS, GARDEN_SHADE_DEFAULT, GARDEN_SHADE_POTATOES, garden,
    garden_more::{any, clean_or_typed, garden_words, says},
};

#[derive(Clone, Copy)]
pub(super) enum Rule {
    Sow,
    Jobs,
    Harvest,
    Shade,
}

static SOW_NOW: LazyLock<Regex> = LazyLock::new(|| Regex::new(GARDEN_NOW_WORDS).unwrap());
static NEXT_MONTH: LazyLock<Regex> = LazyLock::new(|| Regex::new(GARDEN_NEXT_MONTH_WORDS).unwrap());
static JOBS_NOW: LazyLock<Regex> = LazyLock::new(|| Regex::new(GARDEN_GARDEN_NOW).unwrap());
static SOW_FOR_WINTER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bfor (the )?winter\b|\bover ?winter\b|\bfor christmas\b|\bbefore winter\b")
        .unwrap()
});

fn is_garden(line: &str, reading: &Reading<'_>) -> bool {
    any(line, GARDEN_GARDEN_THINGS)
        || reading
            .topic
            .is_some_and(|(name, _)| name == "garden-and-plants")
        || reading
            .topics
            .iter()
            .any(|(name, _)| *name == "garden-and-plants")
}

pub(super) fn not_about_now(reading: &Reading<'_>) -> bool {
    let line = garden_words(clean_or_typed(reading));
    !(JOBS_NOW.is_match(&line)
        && any(&line, GARDEN_GARDEN_JOBS)
        && is_garden(&line, reading)
        && !GARDEN_SEASON_REPLIES
            .iter()
            .any(|(_, words, _, _)| any(&line, words)))
}

fn sowing(line: &str, month_now: u8) -> Option<Cow<'static, str>> {
    if let Some(month) = garden::month(line) {
        return Some(Cow::Borrowed(
            GARDEN_MONTH_REPLIES[usize::from(month - 1)].1,
        ));
    }
    for &(season, _, reply, _) in GARDEN_SEASON_REPLIES {
        if says(line, season) {
            return Some(Cow::Borrowed(
                if season == "winter" && SOW_FOR_WINTER.is_match(line) {
                    GARDEN_SEASON_REPLIES
                        .iter()
                        .find(|entry| entry.0 == "autumn")?
                        .2
                } else {
                    reply
                },
            ));
        }
    }
    let next = NEXT_MONTH.is_match(line);
    if !next && !SOW_NOW.is_match(line) {
        return None;
    }
    let index = usize::from(month_now - 1 + u8::from(next)) % 12;
    let (name, reply, _) = GARDEN_MONTH_REPLIES[index];
    Some(Cow::Owned(format!(
        "It's {name}, so here's what goes in. {reply}"
    )))
}

fn jobs(line: &str, reading: &Reading<'_>, month_now: u8) -> Option<&'static str> {
    if !any(line, GARDEN_GARDEN_JOBS) || !is_garden(line, reading) {
        return None;
    }
    GARDEN_SEASON_REPLIES
        .iter()
        .find_map(|&(_, words, _, reply)| any(line, words).then_some(reply))
        .or_else(|| {
            if !JOBS_NOW.is_match(line) {
                return None;
            }
            let season = GARDEN_MONTH_REPLIES[usize::from(month_now - 1)].2;
            GARDEN_SEASON_REPLIES
                .iter()
                .find_map(|entry| (entry.0 == season).then_some(entry.3))
        })
}

fn crop_weeks(reading: &Reading<'_>) -> Cow<'static, str> {
    let line = [
        &reading.line.plain,
        &reading.line.norm_text,
        &reading.line.text,
    ]
    .into_iter()
    .find(|value| !value.is_empty())
    .map_or("", String::as_str);
    let line = format!(" {} ", line.to_lowercase());
    for (forms, reply) in GARDEN_CROP_WEEKS {
        if forms.iter().any(|form| {
            line.contains(&format!(" {form} "))
                || reading.line.words.iter().any(|word| word == form)
        }) {
            return Cow::Owned(format!("{reply} {GARDEN_CROP_WEEKS_REST}"));
        }
    }
    Cow::Borrowed(GARDEN_CROP_WEEKS_REST)
}

fn shade(reading: &Reading<'_>) -> &'static str {
    let line = clean_or_typed(reading).to_lowercase();
    if line.contains("potato") {
        return GARDEN_SHADE_POTATOES;
    }
    GARDEN_SHADE_CROPS
        .iter()
        .find_map(|&(crop, reply)| line.contains(crop).then_some(reply))
        .unwrap_or(GARDEN_SHADE_DEFAULT)
}

pub(super) fn answer(rule: Rule, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    render(rule, reading, reading.now().month() as u8)
}

fn render(rule: Rule, reading: &Reading<'_>, month_now: u8) -> Option<Cow<'static, str>> {
    match rule {
        Rule::Sow => sowing(&garden_words(clean_or_typed(reading)), month_now),
        Rule::Jobs => {
            jobs(&garden_words(clean_or_typed(reading)), reading, month_now).map(Cow::Borrowed)
        }
        Rule::Harvest => Some(crop_weeks(reading)),
        Rule::Shade => Some(Cow::Borrowed(shade(reading))),
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/garden_seasons.rs"]
mod fixtures;

#[cfg(test)]
#[test]
fn garden_seasons_and_named_crops_follow_the_source() {
    for &fixtures::Case(id, text, month, guard, reply) in fixtures::CASES {
        let reading = Reading::new(text);
        let rule = match id {
            "sow-month" => Rule::Sow,
            "garden-jobs-in-a-season" => Rule::Jobs,
            "crop-weeks-named" => Rule::Harvest,
            "veg-in-shade" => Rule::Shade,
            "garden-in-winter" => {
                assert_eq!(not_about_now(&reading), guard, "{id}: {text}");
                continue;
            }
            _ => panic!("unknown forecast fixture {id}"),
        };
        let actual = render(rule, &reading, month);
        assert_eq!(actual.as_deref(), reply, "{id}: {text} [{month}]");
    }
}
