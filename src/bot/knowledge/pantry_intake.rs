//! Intake for the cooking observer's pantry, separate from offered meal shortlists.

use super::{PANTRY_DISHES, PANTRY_INGREDIENT_ALIASES, food_fold, guards, pantry_names};
use crate::bot::{Memory, Reading, understanding::normalize::tokens};
use regex::Regex;
use std::sync::LazyLock;

#[derive(Default)]
pub(crate) struct PantryIntake {
    meal: Option<Holdings>,
    set_aside: Vec<&'static str>,
    unresolved_spoilage: Option<usize>,
    spoilage_this_turn: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct Holdings {
    started: usize,
    turn: usize,
    have: Vec<&'static str>,
    avoid: Vec<&'static str>,
    only: bool,
    minutes: Option<u32>,
    offered: Vec<&'static str>,
}

const MEAL_IDS: &[(&[&str], &str)] = &[
    (&["chickpea", "spinach"], "chickpea-spinach"),
    (&["chickpea", "tomato"], "chickpea-curry"),
    (&["pasta", "tomato"], "tomato-pasta"),
];

static STOCK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)\Ai\s+(?:(?P<only>only)\s+have|have|(?:have\s+)?got|(?P<empty>don't\s+have|dont\s+have|do\s+not\s+have))\s+(?P<foods>.+?)\z",
    )
    .expect("first-person stock declaration")
});
static STOCK_CONTRACTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?x)\Ai(?:'|’)?ve\s+got\s+(?P<foods>.+?)\z")
        .expect("contracted first-person stock declaration")
});
static BETWEEN_FOODS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\s*,\s*(?:and\s+)?|\s+(?:and|or)\s+").expect("food list separator")
});
static TIME_LIMIT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\Ai\s+(?:only\s+)?have\s+([0-9]+)\s*(?:minutes?|mins?)\z")
        .expect("explicit pantry time limit")
});
static MEAL_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?xi)\Awhat\s+(?:can|should|shall|could)\s+(?:i|we)\s+(?:cook|make)(?:\s+with\s+.+)?\s*[.?!]*\z")
        .expect("scoped meal direction")
});
static BARE_MEAL_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?xi)\Awhat\s+can\s+i\s+(?:make|cook)\s*[.?!]*\z").expect("bare meal follow-up")
});
static LEFT_OUT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:sat|left|forgot|stayed|been)\b(?:\W+\w+){0,6}?\W+out\b(?:\s+(?:all night|overnight|since|for\b|on\b|of (?:the )?(?:fridge|freezer)))")
        .expect("reported food left out of safe storage")
});
static SPOILAGE_REPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?xi)\A(?:(?:my|the|these|those)\s+)?(?P<food>[a-z]+(?:\s+[a-z]+){0,3}?)\s+
        (?:(?:is|are|went|has\s+gone|have\s+gone)\s+(?:mouldy|moldy|rotten|off|bad|sour)|(?:smells?|smelt|smelled)\s+(?:sour|off|rotten|bad))\s*[.!,?]*\z",
    )
    .expect("named food spoilage report")
});
static RECIPE_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:try|cook|fry|simmer|bake|roast|toss|serve|stir|eat it|eat them|use it|use them|here's my pick|something else that works)\b")
        .expect("positive food-use wording")
});

const DAIRY: &[&str] = &["butter", "cheese", "cream", "milk", "yoghurt"];
const DAIRY_NAMES: &[&str] = &[
    "casein",
    "cheddar",
    "creme fraiche",
    "feta",
    "ghee",
    "halloumi",
    "parmesan",
    "whey",
];
const FISH: &[&str] = &[
    "anchovy",
    "bass",
    "cod",
    "flounder",
    "haddock",
    "hake",
    "herring",
    "mackerel",
    "pollock",
    "salmon",
    "sardine",
    "sea bass",
    "tilapia",
    "trout",
    "tuna",
    "white fish",
];
const CRUSTACEANS: &[&str] = &["crab", "crayfish", "lobster", "prawn", "shrimp"];
const MOLLUSCS: &[&str] = &["clam", "mussel", "oyster", "scallop", "squid"];

fn canonical_name(text: &str) -> Option<&'static str> {
    let words: Vec<_> = tokens(text)
        .iter()
        .map(|word| food_fold::fold(word))
        .collect();
    PANTRY_INGREDIENT_ALIASES
        .iter()
        .find(|(parts, _)| words.iter().map(String::as_str).eq(parts.iter().copied()))
        .map(|(_, food)| *food)
}

fn reported_stock(line: &str) -> Option<(Vec<&'static str>, bool, bool)> {
    let hit = STOCK
        .captures(line)
        .or_else(|| STOCK_CONTRACTION.captures(line))?;
    let raw = hit.name("foods")?.as_str();
    let raw = [
        " in the fridge",
        " in the freezer",
        " in the cupboard",
        " in the pantry",
        " in my fridge",
        " in",
        " at home",
    ]
    .into_iter()
    .find_map(|ending| raw.strip_suffix(ending))
    .unwrap_or(raw);
    let foods = BETWEEN_FOODS
        .split(raw)
        .map(|food| {
            let food = [
                "any ",
                "some ",
                "a tin of ",
                "a can of ",
                "tinned ",
                "canned ",
            ]
            .into_iter()
            .find_map(|starter| food.strip_prefix(starter))
            .unwrap_or(food);
            canonical_name(food)
        })
        .collect::<Option<Vec<_>>>()?;
    Some((
        foods,
        hit.name("only").is_some(),
        hit.name("empty").is_some(),
    ))
}

fn names_in(reading: &Reading<'_>) -> Vec<&'static str> {
    let words = reading
        .line
        .words
        .iter()
        .map(|word| food_fold::fold(word))
        .collect::<Vec<_>>();
    pantry_names::ingredients(reading, &words)
}

/// Match declared food and category names without treating similar spellings as the same food.
fn conflicts(allergen: &str, food: &str) -> bool {
    let allergen = food_fold::fold(allergen.trim());
    let exact = allergen == food || canonical_name(&allergen) == Some(food);
    exact
        || match allergen.as_str() {
            "fish" | "finfish" | "fin fish" => FISH.contains(&food),
            "dairy" | "milk" | "cow's milk" | "cows milk" => {
                DAIRY.contains(&food) || DAIRY_NAMES.contains(&food)
            }
            "crustacean" => CRUSTACEANS.contains(&food),
            "shellfish" => CRUSTACEANS.contains(&food) || MOLLUSCS.contains(&food),
            "seafood" => {
                FISH.contains(&food) || CRUSTACEANS.contains(&food) || MOLLUSCS.contains(&food)
            }
            _ => false,
        }
}

pub(super) fn uses_excluded_food(reading: &Reading<'_>, reply: &str, exclusion: &Regex) -> bool {
    let mut clauses = exclusion.captures_iter(reading.raw).peekable();
    if clauses.peek().is_none() {
        return false;
    }
    let reply_names = names_in(&Reading::new(reply));
    let reply_words: Vec<_> = tokens(reply)
        .iter()
        .map(|word| food_fold::fold(word))
        .collect();
    clauses.any(|hit| {
        let cut = &hit[1];
        let removed = names_in(&Reading::new(cut));
        let categories = tokens(cut)
            .into_iter()
            .filter(|word| matches!(word.as_str(), "dairy" | "seafood" | "shellfish" | "fish"));
        removed
            .into_iter()
            .map(str::to_owned)
            .chain(categories)
            .any(|food| {
                reply_names.iter().any(|name| conflicts(&food, name))
                    || reply_words.iter().any(|word| conflicts(&food, word))
            })
    })
}

fn might_be_spoiled(reading: &Reading<'_>, names: &[&str], recent: bool) -> bool {
    let line = &reading.line.clean;
    guards::food_has_gone_off(line)
        || SPOILAGE_REPORT
            .captures(line)
            .and_then(|hit| canonical_name(&hit["food"]))
            .is_some_and(|reported| names.contains(&reported))
        || (!names.is_empty() || recent && reading.line.words.iter().any(|word| word == "it"))
            && LEFT_OUT.is_match(line)
}

fn keep_once(names: &mut Vec<&'static str>, fresh: impl IntoIterator<Item = &'static str>) {
    for food in fresh {
        if !names.contains(&food) {
            names.push(food);
        }
    }
}

impl PantryIntake {
    pub(crate) fn observe(&mut self, reading: &Reading<'_>, turn: usize, last_reply: Option<&str>) {
        let clean = reading
            .line
            .clean
            .trim_end_matches(['.', '!', '?', ','])
            .trim();
        let mut mentioned = names_in(reading);
        self.spoilage_this_turn =
            might_be_spoiled(reading, &mentioned, self.recent(turn).is_some());
        if self.spoilage_this_turn {
            if mentioned.is_empty()
                && let Some(plan) = self.recent(turn)
                && let [food] = plan.have.as_slice()
            {
                mentioned.push(*food);
            }
            if mentioned.len() != 1
                && let Some(plan) = self.recent(turn)
            {
                self.unresolved_spoilage = Some(plan.started);
            }
            keep_once(&mut self.set_aside, mentioned);
            return;
        }

        let stock = (reading.kind == crate::bot::understanding::kinds::Kind::Statement)
            .then(|| reported_stock(clean))
            .flatten();
        if let Some((names, only, empty)) = stock {
            if self.recent(turn).is_none() {
                self.meal = Some(Holdings {
                    started: turn,
                    turn,
                    have: Vec::new(),
                    avoid: Vec::new(),
                    only: false,
                    minutes: None,
                    offered: Vec::new(),
                });
            }
            let plan = self.meal.as_mut().expect("verified pantry plan");
            keep_once(
                if empty {
                    &mut plan.avoid
                } else {
                    &mut plan.have
                },
                names,
            );
            plan.only |= only && !plan.have.is_empty();
            plan.turn = turn;
            plan.keep_previous_offer(last_reply);
            return;
        }

        if self.recent(turn).is_none() {
            return;
        }
        let plan = self.meal.as_mut().expect("recent pantry plan");
        let trimmed = clean.strip_suffix(" please").unwrap_or(clean);
        let exclusion = match trimmed {
            "no dairy" | "without dairy" | "dairy free" | "dairy-free" => Some(DAIRY),
            "without cheese" | "no cheese" => Some(&["cheese"][..]),
            "without fish" | "no fish" => Some(&["fish"][..]),
            _ => None,
        };
        if let Some(avoid) = exclusion {
            keep_once(&mut plan.avoid, avoid.iter().copied());
            plan.turn = turn;
        } else if let Some(limit) = TIME_LIMIT
            .captures(clean)
            .and_then(|found| found[1].parse().ok())
        {
            plan.minutes = Some(limit);
            plan.turn = turn;
        } else if BARE_MEAL_ASK.is_match(reading.raw.trim()) {
            plan.turn = turn;
        } else {
            return;
        }
        plan.keep_previous_offer(last_reply);
    }

    fn recent(&self, turn: usize) -> Option<&Holdings> {
        self.meal
            .as_ref()
            .filter(|plan| turn.saturating_sub(plan.turn) <= 2)
    }

    fn blocked(&self, name: &str, memory: &Memory) -> bool {
        self.set_aside.contains(&name)
            || memory
                .personal
                .reported_allergens()
                .any(|allergen| conflicts(allergen, name))
            || self
                .recent(memory.turns.len())
                .is_some_and(|plan| plan.avoid.iter().any(|avoid| conflicts(avoid, name)))
    }

    pub(crate) fn current_spoilage(&self) -> bool {
        self.spoilage_this_turn
    }

    /// These plans have only partial ingredient data. Keep the stock and allergy rules in force.
    fn unverified_recipe(&self, memory: &Memory) -> bool {
        self.spoilage_this_turn
            || !self.set_aside.is_empty()
            || self.recent(memory.turns.len()).is_some_and(|plan| {
                !plan.avoid.is_empty() || self.unresolved_spoilage == Some(plan.started)
            })
    }

    pub(crate) fn blocks_typed_offer(&self, memory: &Memory) -> bool {
        self.unverified_recipe(memory) || memory.personal.has_food_limits()
    }

    pub(crate) fn blocks_plant_offer(&self, memory: &Memory, vegan: bool) -> bool {
        self.unverified_recipe(memory) || !memory.personal.allows_plant_menu(vegan)
    }

    /// Preserve cooking suggestions that use the ingredients in a verified pantry path.
    pub(crate) fn recipe(&self, reading: &Reading<'_>, memory: &Memory) -> Option<&'static str> {
        let turn = memory.turns.len();
        let plan = self.recent(turn)?;
        let clean = reading.line.clean.trim_end_matches(['.', '!', '?']).trim();
        if self.spoilage_this_turn
            || self.unresolved_spoilage == Some(plan.started)
            || memory.personal.has_food_limits()
            || plan.only
            || plan.minutes.is_some()
            || plan.have.iter().any(|food| self.set_aside.contains(food))
        {
            return None;
        }
        let has = |a, b| plan.have.len() == 2 && plan.have.contains(&a) && plan.have.contains(&b);
        let needs = if plan.started == turn
            && reported_stock(clean).is_some()
            && has("chickpea", "spinach")
            && plan.avoid.is_empty()
        {
            &["chickpea", "spinach"][..]
        } else if !BARE_MEAL_ASK.is_match(reading.raw.trim()) || plan.started == turn {
            return None;
        } else if has("chickpea", "spinach") && plan.avoid.is_empty() {
            &["chickpea", "tomato"][..]
        } else if has("pasta", "cheese") && DAIRY.iter().all(|food| plan.avoid.contains(food)) {
            &["pasta", "tomato"][..]
        } else {
            return None;
        };
        PANTRY_DISHES.iter().find_map(|dish| {
            (dish.uses == needs
                && !dish.uses.iter().any(|name| self.blocked(name, memory))
                && MEAL_IDS
                    .iter()
                    .any(|(foods, id)| *foods == needs && !plan.offered.contains(id)))
            .then_some(dish.reply)
        })
    }

    /// Check explicit meal requests without interfering with a safety question on the current food.
    pub(crate) fn blocks_reply(&self, reading: &Reading<'_>, reply: &str, memory: &Memory) -> bool {
        if let Some(diet) = super::dietary_menus::requested_meal(reading, Some(memory))
            && RECIPE_WORDS.is_match(reply)
            && !super::dietary_menus::lets_recipe_through(reading, reply, Some(memory), diet)
        {
            return true;
        }
        let allergens = memory.personal.reported_allergens().next().is_some();
        let avoids = self
            .recent(memory.turns.len())
            .is_some_and(|plan| !plan.avoid.is_empty());
        let unresolved = self
            .recent(memory.turns.len())
            .is_some_and(|plan| self.unresolved_spoilage == Some(plan.started));
        if self.set_aside.is_empty() && !allergens && !avoids && !unresolved {
            return false;
        }
        let turn = memory.turns.len();
        let recent_meal = self.recent(turn).is_some()
            || memory
                .meal_plan
                .as_ref()
                .is_some_and(|plan| turn.saturating_sub(plan.turn) <= 2);
        let meal_ask = MEAL_ASK.is_match(reading.raw.trim())
            || crate::bot::knowledge::meal_shortlist::is_open_dinner(reading)
            || recent_meal && crate::bot::knowledge::meal_shortlist::is_another_idea_ask(reading);
        let stock = reported_stock(reading.line.clean.trim_end_matches(['.', '!', '?']).trim());
        if !meal_ask && stock.as_ref().is_none_or(|(_, _, absent)| *absent) {
            return false;
        }
        if unresolved && BARE_MEAL_ASK.is_match(reading.raw.trim()) {
            return true;
        }
        let names = if let Some((stock, _, _)) = stock {
            stock
        } else {
            names_in(reading)
        };
        if names.iter().any(|food| self.blocked(food, memory)) && RECIPE_WORDS.is_match(reply) {
            return true;
        }
        if !meal_ask {
            return false;
        }
        self.reply_uses_blocked_food(reply, memory)
    }

    pub(crate) fn reply_uses_blocked_food(&self, reply: &str, memory: &Memory) -> bool {
        let reply = Reading::new(reply);
        names_in(&reply)
            .iter()
            .any(|food| self.blocked(food, memory))
    }
}

impl Holdings {
    fn keep_previous_offer(&mut self, reply: Option<&str>) {
        let Some(reply) = reply else { return };
        for (foods, id) in MEAL_IDS {
            if PANTRY_DISHES
                .iter()
                .any(|dish| dish.uses == *foods && reply.contains(dish.reply))
            {
                keep_once(&mut self.offered, [*id]);
            }
        }
        let expired = self.offered.len().saturating_sub(4);
        self.offered.drain(..expired);
    }
}

#[cfg(test)]
#[path = "pantry_intake_tests.rs"]
mod tests;
