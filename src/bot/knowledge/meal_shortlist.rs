//! Fresh dinner shortlists from `bot/knowledge/meals.py`.

use std::sync::LazyLock;

use crate::bot::{Memory, Reading};
use regex::Regex;

#[path = "meal_shortlist_data.rs"]
mod data;
use data::DISHES;

#[path = "meal_recipes.rs"]
mod recipes;
use recipes::Ingredient;

#[path = "meal_clock.rs"]
pub(crate) mod clock;

#[path = "meal_catalogue.rs"]
pub(crate) mod catalogue;

#[derive(Clone, Copy)]
struct Dish {
    name: &'static str,
    said_as: Option<&'static str>,
    minutes: u8,
    favourite: bool,
    plant: Option<Diet>,
    uses: &'static [&'static str],
    how: &'static str,
    shape: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Diet {
    Vegetarian,
    Vegan,
}

impl Diet {
    pub(crate) fn named(text: &str) -> Option<Self> {
        match text {
            "vegetarian" => Some(Self::Vegetarian),
            "vegan" => Some(Self::Vegan),
            _ => None,
        }
    }

    pub(crate) fn vegan(self) -> bool {
        self == Self::Vegan
    }
}

impl Dish {
    fn said(self) -> &'static str {
        self.said_as.unwrap_or(self.name)
    }
}

/// A reply proposal and the source dish names it offers in the same order.
pub(crate) struct MealShortlist {
    pub(crate) reply: String,
    pub(crate) offered: [&'static str; 3],
}

/// The source's last four offers, with the latest offer's first dish as its pick.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MealPlan {
    pub(crate) turn: usize,
    older: Vec<&'static str>,
    latest: Vec<&'static str>,
    ingredient: Option<Ingredient>,
    pub(crate) diet: Option<Diet>,
    minutes: Option<u32>,
    named: Vec<&'static str>,
    without: Vec<&'static str>,
}

impl MealPlan {
    fn after(
        turn: usize,
        older: &[&'static str],
        latest: &[&'static str],
        ingredient: Option<Ingredient>,
    ) -> Self {
        Self {
            turn,
            older: retained(older, latest.len()).to_vec(),
            latest: latest.to_vec(),
            ingredient,
            diet: None,
            minutes: None,
            named: Vec::new(),
            without: Vec::new(),
        }
    }

    pub(crate) fn offered(&self) -> Vec<&'static str> {
        self.older.iter().chain(&self.latest).copied().collect()
    }
}

pub(crate) struct Proposal {
    pub(crate) reply: String,
    pub(crate) plan: MealPlan,
}

static OPEN_DINNER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?xi)\A
        (?:please\s+)?
        (?:
            what\s+(?:can|could|shall|should)\s+(?:i|we)\s+(?:cook|make|eat)
                (?:\s+(?:for\s+(?:dinner|supper|tea)(?:\s+tonight)?|tonight|this\s+evening))?
          | what\s+(?:can|could|shall|should)\s+(?:i|we)\s+have
                \s+for\s+(?:dinner|supper|tea)(?:\s+tonight)?
          | what(?:'s|’s|s|\s+is)\s+for\s+(?:dinner|supper|tea)(?:\s+tonight)?
          | what\s+to\s+(?:cook|make)(?:\s+(?:for\s+(?:dinner|supper|tea)|tonight))?
          | (?:any\s+)?(?:dinner|supper|tea)\s+(?:ideas|suggestions|recommendations)(?:\s+for\s+tonight)?
          | (?:any\s+)?(?:ideas|suggestions|recommendations)\s+for\s+(?:dinner|supper|tea)(?:\s+tonight)?
          | (?:suggest|recommend)\s+(?:a\s+)?(?:dinner|supper)(?:\s+(?:for\s+)?tonight)?
          | can\s+you\s+suggest\s+a\s+meal\s+for\s+tonight
        )
        (?:\s*,?\s+please)?
        \s*[,?.!]*\z",
    )
    .expect("open dinner question pattern")
});

static PLANT_MEAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?xi)\A(?:please\s+)?what\s+(?:can|could|shall|should)\s+i\s+(?:cook|make)\s+for\s+a\s+(vegetarian|vegan)(?:\s*,?\s+please)?\s*[,?.!]*\z",
    )
    .expect("request for a vegetarian or vegan meal")
});

static PLANT_INSTEAD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?xi)\A(?:something|anything)\s+(vegetarian|vegan)\s+instead(?:\s*,?\s+please)?\s*[,?.!]*\z",
    )
    .expect("vegetarian or vegan meal follow-up")
});

fn request_diet(pattern: &Regex, reading: &Reading<'_>) -> Option<Diet> {
    let said = |text| Diet::named(&pattern.captures(text)?[1].to_ascii_lowercase());
    let requested = said(reading.raw.trim())?;
    (said(&reading.line.clean) == Some(requested)).then_some(requested)
}

pub(crate) fn plant_ask(reading: &Reading<'_>) -> Option<Diet> {
    request_diet(&PLANT_MEAL, reading)
}

pub(crate) fn plant_instead_ask(reading: &Reading<'_>) -> Option<Diet> {
    request_diet(&PLANT_INSTEAD, reading)
}

static OTHER_LENGTH_PROBE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)\A(?:please\s+)?(?:
            # occasions/bring-dish
            what\s+(?:(?:can|shall|should)\s+(?:i|we)|to)\s+(?:cook|make)\s+for\s+dinner
            # travel/food-in-the-country and travel/city-next
          | what\s+(?:can|could|shall|should)\s+i\s+eat(?:\s+this\s+evening)?\s*[,?.!]*\z
          | what\s+(?:can|could|shall|should)\s+we\s+eat(?:\s+(?:tonight|this\s+evening|for\s+dinner))?\s*[,?.!]*\z
            # dishes/plan-start
          | what\s+(?:can|could|shall|should)\s+we\s+(?:cook|make)\s+this\s+evening\s*[,?.!]*\z
            # cooking/tea-brew
          | what\s+(?:can|could)\s+i\s+have\s+for\s+tea\s*[,?.!]*\z
        )",
    )
    .expect("open dinner preflight competition pattern")
});

static STANDS_DOWN_FOR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)\A(?:please\s+)?(?:
            # The bare could-eat travel readings get no dinner fallback.
            what\s+could\s+(?:i|we)\s+eat(?:\s+this\s+evening)?
            # dishes/eating-out-choose takes this shared choice.
          | what\s+shall\s+we\s+eat\s+tonight
            # drinks/tea-types answers the bare should-for-tea ask.
          | what\s+should\s+(?:i|we)\s+(?:cook|eat|have)\s+for\s+tea
        )\s*[,?.!]*\z",
    )
    .expect("competing open-ask directions")
});

static ANOTHER_IDEA: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?xi)\A(?:
            another\s+idea
          | (?:could\s+you\s+suggest\s+)?another\s+dinner\s+idea
          | another\s+idea\s+for\s+dinner
          | a\s+different\s+(?:dinner|meal)\s+idea
        )(?:\s*,?\s+please)?\s*[,?.!]*\z",
    )
    .expect("follow-up dinner question pattern")
});

pub(crate) fn is_another_idea_ask(reading: &Reading<'_>) -> bool {
    ANOTHER_IDEA.is_match(reading.raw.trim()) && ANOTHER_IDEA.is_match(&reading.line.clean)
}

static COOK_SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\Awhat\s+(?:(?:can|could|shall|should)\s+i|to)\s+cook\b")
        .expect("subject of a cooking question")
});

static COOK_TOGETHER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\Awhat\s+(?:can|shall)\s+we\s+cook\s+tonight\s*[?.!]*\z")
        .expect("a shared dinner ask")
});

static DINNER_SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\A(?:what's for dinner|(?:any )?(?:dinner ideas|ideas for dinner)(?: for tonight)?)\s*[?.!]*\z",
    )
    .expect("subject of an open dinner question")
});

/// Offer the initial shortlist for an open dinner ask.
///
/// The caller owns eligibility from session history, including remembered diets,
/// ingredients, rejected suggestions, and conversations about an existing dish.
pub(crate) fn open_dinner(reading: &Reading<'_>, turns: usize) -> Option<MealShortlist> {
    let clean = &reading.line.clean;
    is_open_dinner(reading).then(|| shortlist(clean, turns, reading.wants, &[]))
}

pub(crate) fn is_open_dinner(reading: &Reading<'_>) -> bool {
    // Reconsidering a clause can leave a constraint in the original text only.
    OPEN_DINNER.is_match(reading.raw.trim())
        && OPEN_DINNER.is_match(&reading.line.clean)
        && !STANDS_DOWN_FOR.is_match(&reading.line.clean)
}

/// Identify the bring-dish and travel keys that can own the preliminary length probe.
pub(crate) fn other_length_probe(reading: &Reading<'_>) -> bool {
    OTHER_LENGTH_PROBE.is_match(&reading.line.clean)
}

pub(crate) fn propose(reading: &Reading<'_>, turn: usize, preflight: bool) -> Proposal {
    let another = |offered: &[&str]| shortlist(&reading.line.clean, turn, reading.wants, offered);
    let first = another(&[]);
    // PyBot's length probe can call the callback before _offer builds the visible list.
    let (visible, held) = if preflight {
        let second = another(&first.offered);
        let held = append(first.offered[2], second.offered);
        (second, another(&held))
    } else {
        let held = another(&first.offered);
        (first, held)
    };
    // The final usability probe modifies only this candidate's proposed plan.
    Proposal {
        reply: visible.reply,
        plan: MealPlan::after(turn, &visible.offered, &held.offered, None),
    }
}

fn append(previous: &'static str, latest: [&'static str; 3]) -> [&'static str; 4] {
    [previous, latest[0], latest[1], latest[2]]
}

fn retained<'a>(older: &'a [&'static str], count: usize) -> &'a [&'static str] {
    &older[older.len().saturating_sub(4usize.saturating_sub(count))..]
}

fn extend_offers(older: &[&'static str], latest: &[&'static str]) -> Vec<&'static str> {
    retained(older, latest.len())
        .iter()
        .chain(latest)
        .copied()
        .collect()
}

pub(crate) fn with_ingredient(reading: &Reading<'_>, memory: &Memory) -> Option<Proposal> {
    let previous = memory.meal_plan.as_ref()?;
    let turn = memory.turns.len();
    // No intervening turn has introduced a condition or moved the conversation on.
    if turn.checked_sub(1) != Some(previous.turn)
        || previous.diet.is_some()
        || previous.minutes.is_some()
        || !previous.named.is_empty()
        || !previous.without.is_empty()
    {
        return None;
    }
    let ingredient = recipes::ingredient_in(reading.raw)?;
    let clean = &reading.line.clean;
    if recipes::ingredient_in(clean) != Some(ingredient) || previous.ingredient == Some(ingredient)
    {
        return None;
    }
    let trial = |held: &[&str]| recipes::recommendation(ingredient.name(), clean, memory, held);
    let held = previous.offered();
    let preflight = trial(&held)?;
    let held = extend_offers(&held, &preflight.offered);
    let visible = trial(&held)?;
    let held = extend_offers(&held, &visible.offered);
    let checked = trial(&held)?;
    Some(Proposal {
        reply: visible.reply,
        plan: MealPlan::after(turn, &held, &checked.offered, Some(ingredient)),
    })
}

pub(crate) fn another_idea(reading: &Reading<'_>, memory: &Memory) -> Option<Proposal> {
    if let Some(proposal) = catalogue::pack_with_cooked_dish(reading, memory) {
        return Some(proposal);
    }
    // This path carries a fresh unconstrained dinner through its next turn. An intervening
    // turn can introduce a food restriction or change subjects.
    let [opening] = memory.turns.as_slice() else {
        return None;
    };
    let previous = memory.meal_plan.as_ref()?;
    if previous.ingredient.is_some()
        || previous.diet.is_some()
        || previous.minutes.is_some()
        || !previous.named.is_empty()
        || !previous.without.is_empty()
        || !is_another_idea_ask(reading)
    {
        return None;
    }
    let start = Reading::new(&opening.user);
    let (subject, preflight) = continuation_subject(&start)?;
    let clean = &reading.line.clean;
    let cleaned = clean.trim_end_matches(['?', '.', '!']).trim_end();
    let asks_by_reference = matches!(
        cleaned.trim_end_matches(','),
        "another idea" | "another idea please"
    );
    let carried = if asks_by_reference {
        format!("{cleaned} {subject}")
    } else {
        clean.to_owned()
    };
    let turn = memory.turns.len();
    let previous = previous.offered();
    let offered = if preflight || !asks_by_reference {
        let probe = shortlist(clean, turn, reading.wants, &previous);
        append(previous[3], probe.offered).to_vec()
    } else {
        previous
    };
    let visible = shortlist(&carried, turn, reading.wants, &offered);
    let offered = append(offered[3], visible.offered);
    let committed = shortlist(&carried, turn, reading.wants, &offered);
    Some(Proposal {
        reply: visible.reply,
        plan: MealPlan::after(turn, &visible.offered, &committed.offered, None),
    })
}

fn continuation_subject(opening: &Reading<'_>) -> Option<(&'static str, bool)> {
    let clean = &opening.line.clean;
    let words = &opening.line.words;
    let phrase = |first: &str, second: &str| words.windows(2).any(|pair| pair == [first, second]);
    let subject = if COOK_SUBJECT.is_match(clean) {
        if phrase("for", "supper") {
            "cook supper"
        } else if phrase("for", "tea") {
            "cook tea"
        } else if phrase("could", "i") && phrase("for", "dinner") {
            "cook dinner"
        } else {
            "cook"
        }
    } else if COOK_TOGETHER.is_match(clean) {
        return Some(("cook", false));
    } else if DINNER_SUBJECT.is_match(clean) {
        if words.last().is_some_and(|last| last == "tonight") {
            "dinner tonight"
        } else {
            "dinner"
        }
    } else {
        return None;
    };
    Some((subject, true))
}

fn ranked(clean: &str, turns: usize, fits: impl Fn(&Dish) -> bool) -> Vec<&'static Dish> {
    let seed = turns as u128 * 7919 + clean.chars().map(|ch| ch as u128).sum::<u128>();
    let mut order = Vec::with_capacity(DISHES.len());
    for favourite in [true, false] {
        let start = order.len();
        order.extend(
            DISHES
                .iter()
                .filter(|dish| dish.favourite == favourite && fits(dish)),
        );
        let group = &mut order[start..];
        if !group.is_empty() {
            group.rotate_left((seed % group.len() as u128) as usize);
        }
    }
    order
}

fn spread(order: &[&'static Dish], offered: &[&str]) -> [&'static Dish; 3] {
    let mut used_shapes = Vec::with_capacity(3);
    [(0, 15), (16, 25), (26, 95)].map(|(low, high)| {
        let chosen = order
            .iter()
            .copied()
            .filter(|dish| (low..=high).contains(&dish.minutes) && !offered.contains(&dish.name))
            .min_by_key(|dish| (used_shapes.contains(&dish.shape), !dish.favourite))
            .expect("pinned meal table covers every timing band");
        used_shapes.push(chosen.shape);
        chosen
    })
}

/// Render a shortlist for an unconstrained dinner, excluding the plan's last four offers.
pub(crate) fn shortlist(
    clean: &str,
    turns: usize,
    wants: Option<&str>,
    offered: &[&str],
) -> MealShortlist {
    let offered = &offered[offered.len().saturating_sub(4)..];
    let picks = spread(&ranked(clean, turns, |_| true), offered);
    let [first, second, third] = picks;
    let reply = if wants == Some("pick") {
        format!(
            "Here's my pick: {} in {} minutes, or {} in {}, or {} in {}. Tell me what is in the fridge and I will narrow it down.",
            first.said(),
            first.minutes,
            second.said(),
            second.minutes,
            third.said(),
            third.minutes,
        )
    } else {
        format!(
            "Quick: {} in {}, {} in {}, or {} if you have {} minutes. Tell me what is in the fridge and I will pick properly.",
            first.said(),
            first.minutes,
            second.said(),
            second.minutes,
            third.said(),
            third.minutes,
        )
    };
    MealShortlist {
        reply,
        offered: picks.map(|dish| dish.name),
    }
}

fn plant_pair(
    diet: Diet,
    clean: &str,
    turn: usize,
    offered: &[&str],
) -> Option<(String, [&'static str; 2])> {
    let ranked = ranked(clean, turn, |dish| {
        dish.plant
            .is_some_and(|plant| plant.vegan() || !diet.vegan())
    });
    let mut fresh = ranked
        .into_iter()
        .filter(|dish| !offered.contains(&dish.name));
    let first = fresh.next()?;
    let second = fresh.next()?;
    let reply = recipe_pair(first, second, first.how, false);
    Some((reply, [first.name, second.name]))
}

fn recipe_pair(first: &Dish, second: &Dish, how: &str, estimate: bool) -> String {
    let tail = match i16::from(second.minutes) - i16::from(first.minutes) {
        ..=-11 => format!(
            "If you want something quicker, {} takes {} minutes.",
            second.said(),
            second.minutes
        ),
        -10..=10 => format!("Something else that works is {}.", second.said()),
        _ => format!(
            "Something else that works is {}, at {} minutes.",
            second.said(),
            second.minutes
        ),
    };
    let about = if estimate { "about " } else { "" };
    format!(
        "I'd do {} ({about}{} minutes): {how}. {tail}",
        first.said(),
        first.minutes,
    )
}

fn plant_clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    let clean = &reading.line.clean;
    clean
        .strip_suffix(" please")
        .map_or(clean, |line| line.trim_end_matches([',', ' ']))
}

pub(super) fn plant_card(reading: &Reading<'_>) -> Option<String> {
    plant_pair(plant_ask(reading)?, plant_clean(reading), 0, &[]).map(|(reply, _)| reply)
}

pub(crate) fn plant_proposal(reading: &Reading<'_>, memory: &Memory) -> Option<Proposal> {
    let turn = memory.turns.len();
    let (diet, previous) = if turn == 0 {
        (plant_ask(reading)?, Vec::new())
    } else {
        let diet = plant_instead_ask(reading)?;
        let plan = memory.meal_plan.as_ref()?;
        if turn.checked_sub(1) != Some(plan.turn)
            || plan.ingredient.is_some()
            || plan.minutes.is_some()
            || plan
                .diet
                .is_some_and(|earlier| earlier.vegan() && !diet.vegan())
        {
            return None;
        }
        (diet, plan.offered())
    };
    if memory.pantry.blocks_plant_offer(memory, diet.vegan()) {
        return None;
    }
    let clean = if turn == 0 {
        plant_clean(reading)
    } else {
        &reading.line.clean
    };
    let trial = |held: &[&str]| plant_pair(diet, clean, turn, held);
    let (_, first) = trial(&previous)?;
    let held = extend_offers(&previous, &first);
    let (reply, visible) = trial(&held)?;
    let held = extend_offers(&held, &visible);
    let (_, checked) = trial(&held)?;
    let mut plan = MealPlan::after(turn, &held, &checked, None);
    plan.diet = Some(diet);
    Some(Proposal { reply, plan })
}

#[cfg(test)]
#[path = "meal_shortlist_tests.rs"]
mod tests;
