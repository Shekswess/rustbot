//! Timed dinners and faster revisions to a selected, unconstrained meal plan.

use super::{DISHES, Dish, MealPlan, Proposal, extend_offers, is_open_dinner, ranked, recipe_pair};
use crate::bot::{Memory, Reading, compose::seeded_choice};
use regex::Regex;
use std::sync::LazyLock;

type Card = (&'static str, &'static str);

#[derive(Clone, Copy)]
enum Limit {
    Minutes(u32),
    Faster,
}

struct Request {
    card: Card,
    limit: Limit,
    preflight: bool,
}

const MINUTES: &str = r"(?:\d+(?:[.,]\d+)?|[a-z]+(?:[-\s][a-z]+)?)\s+min(?:ute)?s?";
const LENGTH: [&str; 3] = [
    "About {n} minutes.",
    "Roughly {n} minutes in all.",
    "{n} minutes, give or take.",
];
const TOTAL: [&str; 4] = [
    "Roughly {n} minutes in all.",
    "About {n} minutes in all.",
    "{n} minutes from start to finish.",
    "Roughly {n} minutes, start to finish.",
];
const OPENERS: [&str; 5] = ["Try", "I'd do", "Go for", "How about", "Go with"];

macro_rules! request {
    ($name:ident, $re:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(&format!(
                r"(?ix)\A(?:{})(?:\s*,?\s+please)?\s*[,.?!]*\z",
                $re
            ))
            .expect(stringify!($name))
        });
    };
}

request!(
    MEAL_QUESTION,
    format!(
        r"(?:please\s+)?what\s+(?:can|could|shall|should)\s+i\s+(?:cook|make)\s+in\s+(?:{MINUTES}|half\s+an\s+hour)"
    )
);
request!(
    HURRY_CLOCK,
    format!(r"(?:(?:i\s+)?need\s+)?something\s+on\s+the\s+table\s+in\s+{MINUTES}")
);
request!(QUICK_CLOCK, format!(r"(?:quick|fast)\s+{MINUTES}"));
request!(ONLY_CLOCK, format!(r"only\s+{MINUTES}\s*,?\s+please"));
request!(
    QUICK_DINNER,
    r"(?:any\s+)?(?:quick|fast)\s+(?:meal|dinner)\s+ideas|a\s+quick\s+(?:dinner|meal)|something\s+quick\s+for\s+dinner"
);
request!(HURRY, r"(?:i\s+)?need\s+something\s+(?:fast|quick)");
request!(FAST_FOOD, r"(?:quick|fast)\s+food\s+please");
request!(FAST_FOLLOW, r"(?:something\s+)?fast\s+please");
request!(FASTER_FOLLOW, r"quicker\s+please|something\s+quicker");
request!(QUICK_FOLLOW, r"and\s+quick");
static METHOD_HAS_TIME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d+\s*(?:to\s*\d+\s*)?(?:min|minute|hour|hr)")
        .expect("time mentioned in a recipe method")
});

pub(super) fn minutes(reading: &Reading<'_>) -> Option<u32> {
    reading
        .numbers
        .iter()
        .find(|amount| amount.unit == Some("minutes") && amount.value >= 1.0)
        .map(|amount| amount.value as u32)
        .or_else(|| reading.line.clean.contains("half an hour").then_some(30))
}

fn opening_plan(memory: &Memory) -> Option<&MealPlan> {
    let [opening] = memory.turns.as_slice() else {
        return None;
    };
    let plan = memory.meal_plan.as_ref()?;
    (plan.turn == 0
        && plan.ingredient.is_none()
        && plan.diet.is_none()
        && plan.minutes.is_none()
        && plan.named.is_empty()
        && plan.without.is_empty()
        && is_open_dinner(&Reading::new(&opening.user)))
    .then_some(plan)
}

fn requested(reading: &Reading<'_>, memory: Option<&Memory>) -> Option<Request> {
    let entered = reading.raw.trim();
    let matches =
        |pattern: &Regex| pattern.is_match(entered) && pattern.is_match(&reading.line.clean);
    let previous = memory.and_then(opening_plan);
    if memory.is_some_and(|memory| {
        memory.pantry.blocks_typed_offer(memory) || !memory.turns.is_empty() && previous.is_none()
    }) {
        return None;
    }
    let (card, limit, preflight) = if matches(&MEAL_QUESTION) && previous.is_none() {
        (
            ("cooking", "meal-ideas"),
            Limit::Minutes(minutes(reading)?),
            true,
        )
    } else if matches(&HURRY_CLOCK) && previous.is_none() {
        (
            ("meals", "meal-hurry"),
            Limit::Minutes(minutes(reading)?),
            true,
        )
    } else if matches(&QUICK_CLOCK) {
        (
            ("meals", "meal-quick"),
            Limit::Minutes(minutes(reading)?),
            true,
        )
    } else if matches(&ONLY_CLOCK) && previous.is_none() {
        (
            ("meals", "meal-limit-only"),
            Limit::Minutes(minutes(reading)?),
            false,
        )
    } else if matches(&QUICK_DINNER) && previous.is_none() {
        let preflight = !reading.line.clean.starts_with("something quick");
        (("cooking", "meal-ideas"), Limit::Minutes(20), preflight)
    } else if matches(&HURRY) && previous.is_none() {
        let preflight = !reading.line.clean.starts_with("need something quick");
        (("meals", "meal-hurry"), Limit::Minutes(20), preflight)
    } else if matches(&FAST_FOOD) && previous.is_none() {
        (("meals", "meal-hurry-quick"), Limit::Minutes(20), true)
    } else if previous.is_some() && matches(&FAST_FOLLOW) {
        let id = if reading.line.clean.starts_with("something") {
            "meal-hurry"
        } else {
            "meal-hurry-quick"
        };
        (("meals", id), Limit::Minutes(20), true)
    } else if previous.is_some() && matches(&FASTER_FOLLOW) {
        (("meals", "meal-hurry-quick"), Limit::Faster, true)
    } else if previous.is_some() && matches(&QUICK_FOLLOW) {
        (("meals", "meal-faster"), Limit::Minutes(20), false)
    } else {
        return None;
    };
    Some(Request {
        card,
        limit,
        preflight,
    })
}

fn budget(limit: Limit, pick: Option<&str>) -> Option<u32> {
    match limit {
        Limit::Minutes(minutes) => Some(minutes),
        Limit::Faster => DISHES
            .iter()
            .find(|dish| Some(dish.name) == pick)
            .map(|dish| u32::from(dish.minutes) - 1),
    }
}

fn ranked_for_clock(
    clean: &str,
    turn: usize,
    max: u32,
    offered: &[&str],
    previous_turn: bool,
) -> Vec<&'static Dish> {
    let fits = |dish: &Dish| u32::from(dish.minutes) <= max;
    let all_held = !DISHES
        .iter()
        .any(|dish| fits(dish) && !offered.contains(&dish.name));
    let fit_exists = DISHES.iter().any(fits);
    // Only offers from an earlier turn can widen the source pool to get a new dish.
    let widen = !fit_exists || previous_turn && all_held;
    let order = ranked(clean, turn, |dish| widen || fits(dish));
    let fresh: Vec<_> = order
        .iter()
        .copied()
        .filter(|dish| !offered.contains(&dish.name))
        .collect();
    if fresh.is_empty() { order } else { fresh }
}

struct Draft {
    picks: Vec<&'static Dish>,
    minutes: u32,
}

impl Draft {
    fn held(&self) -> Vec<&'static str> {
        self.picks.iter().map(|dish| dish.name).collect()
    }
}

fn draft(
    clean: &str,
    turn: usize,
    limit: Limit,
    pick: Option<&str>,
    offered: &[&str],
    previous_turn: bool,
) -> Option<Draft> {
    let minutes = budget(limit, pick)?;
    let picks: Vec<_> = ranked_for_clock(clean, turn, minutes, offered, previous_turn)
        .into_iter()
        .take(2)
        .collect();
    (!picks.is_empty()).then_some(Draft { picks, minutes })
}

pub(super) fn approved_method(dish: &Dish) -> Option<String> {
    let chicken = "Cook the chicken until the thickest part stays at or above 70°C for 2 minutes";
    const RICE: &str = "Use rice chilled quickly after cooking, ideally within 1 hour, and refrigerated for no more than 24 hours. Reheat it only once and serve it steaming hot throughout";
    Some(match dish.name {
        "a chicken salad" => format!(
            "fry or griddle a chicken breast, starting with about 5 minutes a side. {chicken}. Rest it for 5 minutes, then slice it over leaves and tomato with oil and lemon"
        ),
        "a chicken stir-fry" => format!(
            "slice the chicken thin and fry it hot with sliced peppers, starting with about 5 minutes. {chicken}. Add the cooked noodles and soy sauce and toss them through"
        ),
        "a chicken traybake" | "chicken curry" => format!("{}. {chicken}", dish.how),
        "a chicken and rice" => format!(
            "fry strips of chicken with an onion, then add a tin of tomatoes or a spoon of curry paste. {chicken}. Stir through freshly cooked rice, or reheat refrigerated rice with the chicken. {RICE}"
        ),
        "a prawn stir-fry" => "fry raw prawns until firm and opaque, then add peppers, cooked noodles and soy sauce and toss until hot".to_owned(),
        "egg fried rice" => format!("{}. {RICE}", dish.how.replacen(" for 5 minutes", "", 1)),
        _ => return None,
    })
}

fn render(draft: &Draft, clean: &str, memory: &Memory) -> Option<String> {
    let first = *draft.picks.first()?;
    if u32::from(first.minutes) > draft.minutes {
        return None;
    }
    let approved = approved_method(first);
    let how = approved.as_deref().unwrap_or(first.how);
    if let Some(&second) = draft.picks.get(1) {
        return Some(recipe_pair(first, second, how, approved.is_some()));
    }
    single_recipe(first, how, clean, memory, approved.is_some())
}

pub(super) fn single_recipe(
    first: &Dish,
    how: &str,
    clean: &str,
    memory: &Memory,
    estimate: bool,
) -> Option<String> {
    let last = memory.turns.last().map_or("", |turn| turn.bot.as_str());
    let openers: Vec<&str> = OPENERS
        .into_iter()
        .filter(|opener| !last.starts_with(opener))
        .collect();
    let opener = seeded_choice(&openers, memory, clean);
    let time = if estimate {
        format!("Allow about {} minutes.", first.minutes)
    } else {
        let options: &[&str] = if METHOD_HAS_TIME.is_match(first.how) {
            &TOTAL
        } else {
            &LENGTH
        };
        seeded_choice(options, memory, clean).replace("{n}", &first.minutes.to_string())
    };
    let mut method = how.chars();
    let capitalized = method.next()?.to_uppercase().to_string() + method.as_str();
    let reply = format!("{opener} {}: {capitalized}. {time}", first.said());
    Some(reply)
}

fn clock_line(reading: &Reading<'_>) -> String {
    reading
        .line
        .clean
        .split_whitespace()
        .map(|word| match word {
            "min" | "mins" => "minutes",
            _ => word,
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn has_scoped_request(reading: &Reading<'_>, memory: &Memory) -> bool {
    requested(reading, Some(memory)).is_some()
}

pub(crate) fn indexed_request(
    domain: &str,
    id: &str,
    reading: &Reading<'_>,
    memory: Option<&Memory>,
) -> bool {
    requested(reading, memory).is_some_and(|request| request.card == (domain, id))
}

pub(crate) fn indexed_reply(
    domain: &str,
    id: &str,
    reading: &Reading<'_>,
    memory: Option<&Memory>,
) -> Option<String> {
    if let Some(memory) = memory {
        let (proposal, card) = propose(reading, memory)?;
        return (card == (domain, id)).then_some(proposal.reply);
    }
    let request = requested(reading, None)?;
    if request.card != (domain, id) {
        return None;
    }
    let clean = clock_line(reading);
    let proposal = draft(&clean, 0, request.limit, None, &[], false)?;
    render(&proposal, &clean, &Memory::default())
}

pub(crate) fn can_replace(intended: Card, selected: Option<Card>) -> bool {
    selected.is_none_or(|card| card == intended || card == ("cooking", "time-budget"))
}

pub(crate) fn propose(reading: &Reading<'_>, memory: &Memory) -> Option<(Proposal, Card)> {
    let request = requested(reading, Some(memory))?;
    let turn = memory.turns.len();
    let previous = opening_plan(memory);
    let mut held = previous.map_or_else(Vec::new, MealPlan::offered);
    let mut pick = previous.and_then(|plan| plan.latest.first()).copied();
    let clean = clock_line(reading);

    if request.preflight {
        let first = draft(&clean, turn, request.limit, pick, &held, previous.is_some())?;
        held = extend_offers(&held, &first.held());
        pick = first.picks.first().map(|dish| dish.name);
    }
    let visible = draft(
        &clean,
        turn,
        request.limit,
        pick,
        &held,
        !request.preflight && previous.is_some(),
    )?;
    let reply = render(&visible, &clean, memory)?;
    held = extend_offers(&held, &visible.held());
    let picked = visible.picks.first().map(|dish| dish.name);
    let final_draft = draft(&clean, turn, request.limit, picked, &held, false)?;
    let mut plan = MealPlan::after(turn, &held, &final_draft.held(), None);
    plan.minutes = Some(final_draft.minutes);
    Some((Proposal { reply, plan }, request.card))
}

#[cfg(test)]
#[path = "meal_clock_tests.rs"]
mod tests;
