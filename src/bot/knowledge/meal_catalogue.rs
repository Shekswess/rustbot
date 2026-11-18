//! Shared catalogue selection for named ingredients, serving plans and meal constraints.

use super::{DISHES, Diet, Dish, MealPlan, Proposal, clock, ranked, recipe_pair, shortlist};
use crate::bot::{
    Memory, Reading,
    compose::seeded_choice,
    understanding::{normalize::tokens, topics},
};
use regex::Regex;
use std::sync::LazyLock;

#[path = "meal_catalogue_data.rs"]
mod data;

pub(crate) const CARDS: &[&str] = &[
    "meal-with",
    "meal-haves",
    "meal-limit",
    "meal-clock",
    "meal-more",
    "meal-use-up",
    "meal-tea",
    "meal-cheaper",
    "meal-out-of",
    "meal-style",
    "meal-for-one",
    "meal-for-two",
];

macro_rules! pattern {
    ($name:ident, $re:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(concat!("(?i)", $re)).expect(stringify!($name)));
    };
}
pattern!(
    ASKS,
    r"\b(?:what|whats|what's|idea|ideas|suggest|something|anything|any|make|cook|meal|dinner|supper|recipe|hungry|starving)\b"
);
pattern!(
    HAVE,
    r"^\s*(?:(?:i|we)(?:['’]ve|ve)?\s+(?:have|got)|there(?:['’]s| is| are))\b"
);
pattern!(
    NOT_DINNER,
    r"\b(?:allerg\w*|anaphyla\w*|intoleran\w*|coeliac|celiac|gift|presents?|reheat|thaw\w*|defrost\w*|spoilt|spoiled|mouldy|moldy|rotten|left out|gone off|gone sour|smells? off|morning after|symptoms?|pregnan\w*)\b"
);
pattern!(
    EATING_TECHNIQUE,
    r"\b(?:how long|how do i|can i use|how much|how many|why|what temperature|gas mark|degrees)\b"
);
pattern!(
    FOOD_NOT_WANTED,
    r"\b(?:no|without|skip|minus|out of|ran out of|run out of|none left of|no more|(?:do not|don't|dont|does not|doesn't|doesnt|won't|wont|will not|cannot|can't|cant|can not|never) (?:eat|eats|have))\s+(?P<what>[^,.?!]+)"
);
pattern!(
    PREFERENCE,
    r"\b(?:with|without|using|got|have|leftover|left over)\b"
);
pattern!(
    TAKEOUT,
    r"\b(?:takeaways?|takeout|deliveroo|restaurant|cafe|pub)\b"
);
pattern!(
    NEW_REQUEST,
    r"\b(?:cheap|cheaper|cheapest|quick|fast|tonight|dinner|supper|cook)\b"
);
pattern!(
    PACK_WITH,
    r"^(?:and )?what (?:can|could|should) (?:i|we) (?:pack|bring|take) (?:with|alongside) (?:it|that|them)[.!?]*$"
);
pattern!(
    PREPARE_AHEAD,
    r"^(?:and )?(?:can|could|should) (?:i|we) (?:make|cook|prepare) (?:it|that|them) (?:ahead(?: of time)?|in advance|the (?:night|day) before)[.!?]*$"
);

fn contains_any(text: &str, cues: &[&str]) -> bool {
    cues.iter().any(|cue| text.contains(cue))
}

fn named_in(raw_words: &[String], normalized: &str) -> Vec<&'static str> {
    let mut words = raw_words.to_vec();
    for word in tokens(normalized) {
        if !words.contains(&word) {
            words.push(word);
        }
    }
    let mut line = words.join(" ");
    let mut named = Vec::new();
    for (aliases, name) in data::INGREDIENTS {
        for phrase in aliases
            .iter()
            .filter(|phrase| !phrase.chars().all(char::is_alphabetic))
        {
            if !line.contains(phrase) {
                continue;
            }
            if !named.contains(name) {
                named.push(*name);
            }
            line = line.replace(phrase, " ");
            words.retain(|word| !phrase.contains(word));
        }
    }
    for (aliases, name) in data::INGREDIENTS {
        if !named.contains(name)
            && aliases.iter().any(|alias| {
                alias.chars().all(char::is_alphabetic) && words.iter().any(|word| word == alias)
            })
        {
            named.push(*name);
        }
    }
    named
}

fn recent(memory: &Memory) -> Option<&MealPlan> {
    memory
        .meal_plan
        .as_ref()
        .filter(|plan| memory.turns.len().saturating_sub(plan.turn) <= 3)
}

#[derive(Default)]
struct Limits {
    diet: Option<Diet>,
    without: Vec<&'static str>,
    minutes: Option<u32>,
}

fn limits(reading: &Reading<'_>, previous: Option<&MealPlan>) -> Option<Limits> {
    let clean = &reading.line.clean;
    let diet = if contains_any(clean, &["vegan"]) || contains_any(clean, data::NO_DAIRY) {
        Some(Diet::Vegan)
    } else if contains_any(clean, data::NO_MEAT) {
        Some(Diet::Vegetarian)
    } else {
        previous.and_then(|plan| plan.diet)
    };
    let mut without = previous.map_or_else(Vec::new, |plan| plan.without.clone());
    for hit in FOOD_NOT_WANTED.captures_iter(clean) {
        let wanted = &hit["what"];
        let foods = named_in(&tokens(wanted), wanted);
        if foods.is_empty()
            && !contains_any(
                wanted,
                &[
                    "meat", "dairy", "fish", "seafood", "time", "money", "fancy", "fuss",
                ],
            )
        {
            return None;
        }
        without.extend(foods);
    }
    if contains_any(clean, data::NO_CHEESE) {
        without.push("cheese");
    }
    if without.contains(&"fish")
        || contains_any(
            clean,
            &["no fish", "without fish", "no seafood", "without seafood"],
        )
    {
        without.extend(["fish", "tuna", "salmon", "prawn", "cod", "haddock"]);
    }
    without.sort_unstable();
    without.dedup();
    let minutes = clock::minutes(reading)
        .or_else(|| {
            contains_any(clean, &["quick", "faster", "fast dinner", "fast meal"]).then_some(20)
        })
        .or_else(|| previous.and_then(|plan| plan.minutes));
    Some(Limits {
        diet,
        without,
        minutes,
    })
}

fn eligible(
    id: &str,
    clean: &str,
    named: &[&str],
    limit: &Limits,
    previous: Option<&MealPlan>,
) -> bool {
    if EATING_TECHNIQUE.is_match(clean) && id != "meal-use-up" {
        return false;
    }
    match id {
        "meal-with" => !named.is_empty() && PREFERENCE.is_match(clean) && ASKS.is_match(clean),
        "meal-haves" => !named.is_empty() && HAVE.is_match(clean),
        "meal-limit" => limit.diet.is_some() || !limit.without.is_empty(),
        "meal-clock" => limit.minutes.is_some() && contains_any(clean, data::MEAL_WORD),
        "meal-more" => previous.is_some() && ASKS.is_match(clean),
        "meal-tea" => {
            contains_any(clean, &["for tea", "teatime", "tea time"]) && ASKS.is_match(clean)
        }
        "meal-cheaper" => previous.is_some() || NEW_REQUEST.is_match(clean),
        "meal-out-of" => {
            FOOD_NOT_WANTED.is_match(clean)
                && !limit.without.is_empty()
                && (previous.is_some() || ASKS.is_match(clean))
        }
        "meal-style" => contains_any(clean, data::DISH_STYLE) && !TAKEOUT.is_match(clean),
        "meal-for-one" | "meal-for-two" => {
            ASKS.is_match(clean) && contains_any(clean, data::MEAL_WORD)
        }
        _ => false,
    }
}

fn pool(
    clean: &str,
    turns: usize,
    names: &[&str],
    offered: &[&str],
    limit: &Limits,
) -> Vec<&'static Dish> {
    let fit = |dish: &Dish| {
        limit.diet.is_none_or(|diet| {
            dish.plant
                .is_some_and(|plant| plant.vegan() || !diet.vegan())
        }) && !limit.without.iter().any(|name| dish.uses.contains(name))
    };
    let within = |dish: &Dish| {
        fit(dish)
            && limit
                .minutes
                .is_none_or(|minutes| u32::from(dish.minutes) <= minutes)
    };
    let mut candidates: Vec<_> = DISHES.iter().filter(|dish| within(dish)).collect();
    if !names.is_empty()
        && !candidates
            .iter()
            .any(|dish| dish.uses.iter().any(|name| names.contains(name)))
    {
        candidates = DISHES.iter().filter(|dish| fit(dish)).collect();
    }
    if contains_any(clean, data::CHEAP_ASK)
        && candidates
            .iter()
            .any(|dish| dish.uses.iter().all(|name| data::CHEAP_SET.contains(name)))
    {
        candidates.retain(|dish| dish.uses.iter().all(|name| data::CHEAP_SET.contains(name)));
    }
    if contains_any(clean, data::LIGHT_ASK)
        && candidates
            .iter()
            .any(|dish| data::LIGHT_SET.contains(&dish.name))
    {
        candidates.retain(|dish| data::LIGHT_SET.contains(&dish.name));
    }
    let styles: Vec<_> = data::DISH_STYLE
        .iter()
        .copied()
        .filter(|cue| clean.contains(cue))
        .chain(
            data::STYLE_CUES
                .iter()
                .filter(|(cue, _)| clean.contains(cue))
                .flat_map(|(_, kinds)| kinds.iter().copied()),
        )
        .collect();
    if candidates
        .iter()
        .any(|dish| contains_any(&dish.name.replace('-', " "), &styles))
    {
        candidates.retain(|dish| contains_any(&dish.name.replace('-', " "), &styles));
    }
    if names.is_empty() {
        let mut rotated = ranked(clean, turns, |dish| {
            candidates.iter().any(|item| item.name == dish.name)
        });
        if contains_any(clean, data::IMPRESS)
            && rotated
                .iter()
                .any(|dish| data::SHOW_OFF.contains(&dish.name))
        {
            rotated.retain(|dish| data::SHOW_OFF.contains(&dish.name));
        }
        rotated.sort_by_key(|dish| offered.contains(&dish.name));
        return rotated;
    }
    candidates.retain(|dish| dish.uses.iter().any(|name| names.contains(name)));
    candidates.sort_by_key(|dish| {
        let used = dish.uses.iter().filter(|name| names.contains(name)).count();
        let extra = dish.uses.iter().filter(|name| !names.contains(name));
        (
            offered.contains(&dish.name),
            std::cmp::Reverse(used),
            !dish.uses.contains(&names[0]),
            extra
                .clone()
                .filter(|food| !data::STAPLE.contains(food))
                .count(),
            extra.count(),
            !names.iter().any(|name| dish.name.contains(name)),
            dish.minutes,
        )
    });
    candidates
}

fn used_up(reading: &Reading<'_>, memory: &Memory) -> Option<Proposal> {
    let clean = &reading.line.clean;
    if !contains_any(
        clean,
        &[
            "leftover",
            "left over",
            "use up",
            "using up",
            "spare",
            "rest of",
        ],
    ) {
        return None;
    }
    let words = &reading.line.words;
    let (food, method) = data::USE_UP
        .iter()
        .find_map(|(foods, method)| {
            let food = foods
                .iter()
                .find(|food| words.iter().any(|word| word == **food))?;
            Some((*food, *method))
        })
        .or_else(|| {
            data::UP_CLASSES.iter().find_map(|(foods, method)| {
                let food = foods.iter().find(|name| {
                    let term: Vec<_> = name.split_whitespace().collect();
                    words
                        .windows(term.len())
                        .any(|window| window.iter().map(String::as_str).eq(term.iter().copied()))
                })?;
                Some((*food, *method))
            })
        })?;
    if super::super::lookup::single_ingredient_meal(reading)
        .is_some_and(|reply| contains_any(&reply.to_lowercase(), data::USE_DISH_WORDS))
        && !data::USE_UP.iter().any(|(names, _)| names.contains(&food))
    {
        return None;
    }
    let method = method
        .replace("it keeps 3 days", "it keeps up to 48 hours")
        .replace(
            "cooked meat only needs heating through",
            "reheat properly chilled cooked meat only once and until it is steaming hot throughout",
        );
    let reply = format!(
        "Leftover {food}: {method}{}",
        if method.ends_with('.') { "" } else { "." }
    );
    if memory.pantry.reply_uses_blocked_food(&reply, memory) {
        return None;
    }
    let plan = MealPlan::after(memory.turns.len(), &[], &[food], None);
    Some(Proposal { reply, plan })
}

fn describe(dish: &Dish, clean: &str, memory: &Memory, without: &[&str]) -> String {
    let approved = clock::approved_method(dish);
    let estimate = approved.is_some();
    let how = approved.unwrap_or_else(|| dish.how.to_owned());
    let how = if without.contains(&"cheese") || without.contains(&"milk") {
        how.replace(", and crumble cheese over it if you eat it", "")
    } else {
        how
    };
    clock::single_recipe(dish, &how, clean, memory, estimate)
        .unwrap_or_else(|| format!("I'd do {}: {how}.", dish.said()))
}

fn named_recipe(
    picks: &[&Dish],
    clean: &str,
    memory: &Memory,
    names: &[&str],
    limits: &Limits,
) -> Option<String> {
    let first = *picks.first()?;
    if names.is_empty()
        && let Some(second) = picks.get(1)
    {
        let approved = clock::approved_method(first);
        let how = approved.as_deref().unwrap_or(first.how);
        return Some(recipe_pair(first, second, how, approved.is_some()));
    }
    let mut reply = describe(first, clean, memory, &limits.without);
    if let Some(second) = picks.get(1)
        && !names.is_empty()
        && second
            .uses
            .iter()
            .filter(|food| names.contains(food))
            .count()
            >= names.len().min(2)
        && second.minutes <= 25.max(first.minutes + 10)
    {
        let variants: &[&str] = if second.minutes.abs_diff(first.minutes) <= 10 {
            &[
                "Something else that works is {alt}.",
                "There is {alt} as well.",
            ]
        } else {
            &[
                "If you'd rather have something else, {alt} takes {n} minutes.",
                "Something else that works is {alt}, at {n} minutes.",
            ]
        };
        let tail = seeded_choice(variants, memory, clean)
            .replace("{alt}", second.said())
            .replace("{n}", &second.minutes.to_string());
        reply.push(' ');
        reply.push_str(&tail);
    }
    Some(reply)
}

pub(super) fn pack_with_cooked_dish(reading: &Reading<'_>, memory: &Memory) -> Option<Proposal> {
    let clean = &reading.line.clean;
    if !PACK_WITH.is_match(clean) {
        return None;
    }
    let mut prior = memory.turns.iter().rev();
    let mut anchor = prior.next()?;
    if PREPARE_AHEAD.is_match(&anchor.user) {
        anchor = prior.next()?;
    }
    if anchor.module != "answers" {
        return None;
    }
    let earlier = Reading::new(&anchor.user);
    let visible = crate::bot::memory::unframed_reply(&anchor.bot).unwrap_or(&anchor.bot);
    let declared = super::super::lookup::pantry_recipe_ingredients(visible)?;

    let mut limit = limits(&earlier, None)?;
    let diet = limit.diet?;
    if memory.pantry.blocks_plant_offer(memory, diet.vegan()) {
        return None;
    }
    if contains_any(&earlier.clean, data::NO_DAIRY) {
        limit
            .without
            .extend(["cheese", "milk", "butter", "cream", "yoghurt"]);
    }
    // The prior dish already pairs the named ingredients. The carried recipe
    // search uses its lead ingredient to offer another dish for the meal.
    let names = [*declared.first()?];
    let choices = pool(clean, memory.turns.len(), &names, &[], &limit);
    let picks = &choices[..choices.len().min(2)];
    let subject = data::INGREDIENTS
        .iter()
        .find(|(_, name)| *name == names[0])
        .and_then(|(aliases, _)| {
            earlier
                .line
                .words
                .iter()
                .find(|word| aliases.contains(&word.as_str()))
        })
        .map_or(names[0], String::as_str);
    let (ask, _) = clean.rsplit_once(' ')?;
    // The source chooses its recipe wording using the question after the
    // referent replaces the pronoun.
    let carried = format!("{ask} the {subject}");
    let reply = named_recipe(picks, &carried, memory, &names, &limit)?;
    if memory.pantry.reply_uses_blocked_food(&reply, memory) {
        return None;
    }
    let offered = picks.iter().map(|dish| dish.name).collect::<Vec<_>>();
    let mut plan = MealPlan::after(memory.turns.len(), &[], &offered, None);
    plan.diet = Some(diet);
    plan.minutes = limit.minutes;
    plan.named = names.to_vec();
    plan.without = limit.without;
    Some(Proposal { reply, plan })
}

pub(crate) fn suggest(id: &str, reading: &Reading<'_>, memory: &Memory) -> Option<Proposal> {
    let clean = &reading.line.clean;
    if super::is_open_dinner(reading)
        || super::plant_ask(reading).is_some()
        || super::plant_instead_ask(reading).is_some()
        || clock::has_scoped_request(reading, memory)
        || NOT_DINNER.is_match(reading.raw)
        || contains_any(clean, data::EMPTY)
        || contains_any(clean, data::GLUTEN)
        || contains_any(clean, data::UNWELL)
        || contains_any(clean, data::CHILD_AT_TABLE)
        || topics::gift(&reading.line.norm)
    {
        return None;
    }
    if id == "meal-use-up" {
        return (!memory.pantry.blocks_typed_offer(memory)).then(|| used_up(reading, memory))?;
    }
    let previous = recent(memory);
    let mut names = named_in(&reading.line.words, &reading.line.norm_text);
    let limit = limits(reading, previous)?;
    if !eligible(id, clean, &names, &limit, previous)
        || if let Some(diet) = limit.diet {
            memory.pantry.blocks_plant_offer(memory, diet.vegan())
        } else {
            memory.pantry.blocks_typed_offer(memory)
        }
    {
        return None;
    }
    if names.is_empty()
        && let Some(plan) = previous
    {
        names.extend(plan.named.iter().copied());
        if names.is_empty()
            && let Some(ingredient) = plan.ingredient
        {
            names.push(ingredient.name());
        }
    }
    names.retain(|food| !limit.without.contains(food));
    let turn = memory.turns.len();
    let older = previous.map_or_else(Vec::new, MealPlan::offered);
    let (reply, offered) = if names.is_empty()
        && limit.diet.is_none()
        && limit.without.is_empty()
        && limit.minutes.is_none()
        && !contains_any(clean, data::IMPRESS)
        && !matches!(id, "meal-cheaper" | "meal-style")
    {
        let first = shortlist(clean, turn, reading.wants, &older);
        (first.reply, first.offered.to_vec())
    } else {
        let choices = pool(clean, turn, &names, &older, &limit);
        let picks = &choices[..choices.len().min(2)];
        let reply = named_recipe(picks, clean, memory, &names, &limit)?;
        (reply, picks.iter().map(|dish| dish.name).collect())
    };
    if memory.pantry.reply_uses_blocked_food(&reply, memory) {
        return None;
    }
    let mut plan = MealPlan::after(turn, &older, &offered, None);
    plan.diet = limit.diet;
    plan.minutes = limit.minutes;
    plan.named = names;
    plan.without = limit.without;
    Some(Proposal { reply, plan })
}

pub(crate) fn indexed(id: &str, reading: &Reading<'_>, memory: Option<&Memory>) -> Option<String> {
    let fresh = Memory::default();
    suggest(id, reading, memory.unwrap_or(&fresh)).map(|proposal| proposal.reply)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn a_packing_followup_keeps_the_dish_and_its_diet_in_context() {
        let start = "I have chickpeas and spinach. What can I make without dairy?";
        let mut bot = Bot::new(Some("0")).unwrap();
        assert!(bot.turn(start).reply.starts_with("Chickpeas and spinach"));
        bot.turn("Could I prepare it in advance?");
        let packed = bot.turn("What should I pack with it?");
        assert_eq!(packed.module, Some("answers"), "{packed:?}");
        assert!(packed.reply.contains("chickpea curry"), "{packed:?}");
        assert!(!packed.reply.contains("cheese"), "{packed:?}");

        bot.reset().unwrap();
        bot.turn(start);
        bot.turn("What is the capital of Italy?");
        assert!(
            pack_with_cooked_dish(&Reading::new("What should I pack with it?"), &bot.memory)
                .is_none()
        );
    }

    #[test]
    fn named_recipes_keep_ingredient_and_diet_constraints() {
        let memory = Memory::default();
        let tuna = suggest(
            "meal-with",
            &Reading::new("what can i make with tuna"),
            &memory,
        )
        .unwrap();
        assert!(tuna.reply.contains("tuna pasta"), "{:?}", tuna.reply);
        assert_eq!(tuna.plan.named, ["tuna"]);
        let excluded = suggest(
            "meal-limit",
            &Reading::new("something without fish for dinner"),
            &memory,
        )
        .unwrap();
        assert!(excluded.plan.without.contains(&"tuna"));
        assert!(!excluded.reply.contains("tuna"), "{:?}", excluded.reply);
        let mut bot = Bot::new(Some("23")).unwrap();
        bot.turn("i am allergic to fish");
        assert!(
            suggest(
                "meal-with",
                &Reading::new("what can i make with tuna"),
                &bot.memory
            )
            .is_none()
        );
    }

    #[test]
    fn leftover_and_numbered_meal_cards_use_the_same_catalogue_safely() {
        let memory = Memory::default();
        let read = Reading::new("how can i use up leftover roast chicken");
        let reply = suggest("meal-use-up", &read, &memory).unwrap().reply;
        assert!(reply.contains("steaming hot throughout"), "{reply}");
        assert!(suggest("meal-for-two", &Reading::new("dinner for two"), &memory).is_some());
        assert!(
            suggest(
                "meal-for-two",
                &Reading::new("board game for two tonight"),
                &memory
            )
            .is_none()
        );
    }
}
