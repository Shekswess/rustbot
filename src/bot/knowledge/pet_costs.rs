//! Source-authored UK pet cost estimates selected by species and acquisition route.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::Reading;

use super::{
    PET_COST_BREEDER_LINE, PET_COST_BREEDER_PATTERN, PET_COST_COMPARE_LINE,
    PET_COST_COMPARE_PATTERN, PET_COST_DOG_PATTERN, PET_COST_KITTEN_LINE, PET_COST_KITTEN_PATTERN,
    PET_COST_PUPPY_LINE, PET_COST_RESCUE_LINE, PET_COST_RESCUE_PATTERN, SMALL_PET_BUDGIE_LINE,
    SMALL_PET_BUDGIE_PATTERN, SMALL_PET_FISH_LINE, SMALL_PET_FISH_PATTERN, SMALL_PET_GUINEA_LINE,
    SMALL_PET_GUINEA_PATTERN, SMALL_PET_HAMSTER_LINE, SMALL_PET_HAMSTER_PATTERN,
    SMALL_PET_HORSE_LINE, SMALL_PET_HORSE_PATTERN, SMALL_PET_RABBIT_LINE, SMALL_PET_RABBIT_PATTERN,
};

static DOG: LazyLock<Regex> = LazyLock::new(|| Regex::new(PET_COST_DOG_PATTERN).unwrap());
static KITTEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(PET_COST_KITTEN_PATTERN).unwrap());

static PET_ROUTES: LazyLock<[(Regex, &'static str); 3]> = LazyLock::new(|| {
    [
        (PET_COST_COMPARE_PATTERN, PET_COST_COMPARE_LINE),
        (PET_COST_RESCUE_PATTERN, PET_COST_RESCUE_LINE),
        (PET_COST_BREEDER_PATTERN, PET_COST_BREEDER_LINE),
    ]
    .map(|(pattern, reply)| (Regex::new(pattern).unwrap(), reply))
});

static SMALL_PETS: LazyLock<[(Regex, &'static str); 6]> = LazyLock::new(|| {
    [
        (SMALL_PET_RABBIT_PATTERN, SMALL_PET_RABBIT_LINE),
        (SMALL_PET_HAMSTER_PATTERN, SMALL_PET_HAMSTER_LINE),
        (SMALL_PET_GUINEA_PATTERN, SMALL_PET_GUINEA_LINE),
        (SMALL_PET_BUDGIE_PATTERN, SMALL_PET_BUDGIE_LINE),
        (SMALL_PET_HORSE_PATTERN, SMALL_PET_HORSE_LINE),
        (SMALL_PET_FISH_PATTERN, SMALL_PET_FISH_LINE),
    ]
    .map(|(pattern, reply)| (Regex::new(pattern).unwrap(), reply))
});

fn said(reading: &Reading<'_>) -> String {
    if reading.line.clean.is_empty() {
        reading.line.text.to_lowercase()
    } else {
        reading.line.clean.to_lowercase()
    }
}

pub(super) fn dog_or_cat(reading: &Reading<'_>) -> &'static str {
    let mut said = said(reading);
    if !KITTEN.is_match(&said)
        && !DOG.is_match(&said)
        && let Some(animal) = &reading.animal_in_hand
    {
        said.push(' ');
        said.push_str(&animal.to_lowercase());
    }
    let [(compare, compared), (rescue, rescued), (breeder, bred)] = &*PET_ROUTES;
    if KITTEN.is_match(&said) {
        if DOG.is_match(&said) && compare.is_match(&said) {
            return compared;
        }
        return PET_COST_KITTEN_LINE;
    }
    if rescue.is_match(&said) {
        return rescued;
    }
    if breeder.is_match(&said) {
        return bred;
    }
    PET_COST_PUPPY_LINE
}

pub(super) fn small_pet(reading: &Reading<'_>) -> Option<&'static str> {
    let said = said(reading);
    SMALL_PETS
        .iter()
        .find_map(|(pattern, reply)| pattern.is_match(&said).then_some(*reply))
}
