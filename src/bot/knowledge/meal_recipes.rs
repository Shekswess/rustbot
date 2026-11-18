//! Single-ingredient recipe ranking for a dinner plan.

use super::{DISHES, Dish};
use crate::bot::{Memory, compose};
use regex::Regex;
use std::sync::LazyLock;

#[path = "meal_recipes_data.rs"]
mod data;
use data::{RECIPES, STAPLES};

struct Recipe {
    dish: &'static Dish,
    uses: &'static [&'static str],
    how: &'static str,
    gives_cook_time: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Ingredient {
    Chickpea,
    Lentil,
    Pea,
    Tuna,
    Mince,
}

impl Ingredient {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Chickpea => "chickpea",
            Self::Lentil => "lentil",
            Self::Pea => "pea",
            Self::Tuna => "tuna",
            Self::Mince => "mince",
        }
    }

    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "chickpea" | "chickpeas" => Self::Chickpea,
            "lentil" | "lentils" => Self::Lentil,
            "pea" | "peas" => Self::Pea,
            "tuna" => Self::Tuna,
            "mince" => Self::Mince,
            _ => return None,
        })
    }
}

static USING_ONE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?xi)\Awhat\s+can\s+i\s+make\s+with\s+([a-z]+)\s*[?.!]*\z")
        .expect("single ingredient dinner ask")
});

pub(super) fn ingredient_in(line: &str) -> Option<Ingredient> {
    let captures = USING_ONE.captures(line.trim())?;
    Ingredient::from_name(&captures[1].to_ascii_lowercase())
}

pub(super) struct Recommendation {
    pub(super) reply: String,
    pub(super) offered: Vec<&'static str>,
}

fn ordered(ingredient: &str, offered: &[&str]) -> Vec<&'static Recipe> {
    let mut recipes: Vec<_> = RECIPES
        .iter()
        .filter(|recipe| recipe.uses.contains(&ingredient))
        .collect();
    recipes.sort_by_key(|recipe| {
        let extra_shopping = recipe
            .uses
            .iter()
            .filter(|&&name| name != ingredient && !STAPLES.contains(&name))
            .count();
        (
            extra_shopping,
            recipe.uses.len(),
            !recipe.dish.name.contains(ingredient),
            recipe.dish.minutes,
        )
    });
    if recipes
        .iter()
        .any(|recipe| !offered.contains(&recipe.dish.name))
    {
        recipes.retain(|recipe| !offered.contains(&recipe.dish.name));
    }
    recipes
}

const OPENERS: &[&str] = &["Try", "I'd do", "Go for", "How about", "Go with"];
const TIMES: &[&str] = &[
    "About {n} minutes.",
    "Roughly {n} minutes in all.",
    "{n} minutes, give or take.",
];
const TOTAL_TIMES: &[&str] = &[
    "Roughly {n} minutes in all.",
    "About {n} minutes in all.",
    "{n} minutes from start to finish.",
    "Roughly {n} minutes, start to finish.",
];

const NEAR_ALTERNATIVES: &[&str] = &[
    "Something else that works is {alt}.",
    "There is {alt} as well.",
];
const ALTERNATIVES: &[&str] = &[
    "If you'd rather have something else, {alt} takes {n} minutes.",
    "Something else that works is {alt}, at {n} minutes.",
];

const GRACE: u8 = 10;

fn capitalized(text: &str) -> String {
    let mut chars = text.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

pub(super) fn recommendation(
    ingredient: &str,
    clean: &str,
    memory: &Memory,
    offered: &[&str],
) -> Option<Recommendation> {
    let recipes = ordered(ingredient, offered);
    let first = *recipes.first()?;
    let second = recipes.get(1).copied();
    let last = memory.turns.last().map_or("", |turn| turn.bot.as_str());
    let openers: Vec<_> = OPENERS
        .iter()
        .copied()
        .filter(|opener| !last.starts_with(opener))
        .collect();
    let opener = compose::seeded_choice(&openers, memory, clean);
    let times = if first.gives_cook_time {
        TOTAL_TIMES
    } else {
        TIMES
    };
    let when = compose::seeded_choice(times, memory, clean)
        .replace("{n}", &first.dish.minutes.to_string());
    let mut reply = format!(
        "{opener} {}: {}. {when}",
        first.dish.said(),
        capitalized(first.how)
    );
    if let Some(second) = second
        && second.dish.minutes <= 25.max(first.dish.minutes + GRACE)
    {
        let variants = if first.dish.minutes.abs_diff(second.dish.minutes) <= GRACE {
            NEAR_ALTERNATIVES
        } else {
            ALTERNATIVES
        };
        let alternative = compose::seeded_choice(variants, memory, clean)
            .replace("{alt}", second.dish.said())
            .replace("{n}", &second.dish.minutes.to_string());
        reply.push(' ');
        reply.push_str(&alternative);
    }
    if let Some((head, _)) = reply.split_once(". ")
        && last.contains(&format!("{head}."))
    {
        reply = format!(
            "{} takes about {} minutes.",
            capitalized(first.dish.said()),
            first.dish.minutes
        );
    }
    Some(Recommendation {
        reply,
        offered: [Some(first), second]
            .into_iter()
            .flatten()
            .map(|recipe| recipe.dish.name)
            .collect(),
    })
}
