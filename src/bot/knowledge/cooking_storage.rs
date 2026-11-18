//! Contextual storage, freezing, bake follow-up, pulse and stock cards.

use super::cooking_pairings;
use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "cooking_storage_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(data::$name).expect(stringify!($name))
        });)+
    };
}

patterns! {PULSES_FRESH, PULSES_TIN, PULSES_DRIED, PULSES_APPLIANCE,
PULSES_HARD, PULSES_NO_SOAK, PULSES_KIDNEY}

static PAIRING_FOODS: LazyLock<Regex> = LazyLock::new(|| {
    let alternatives = data::PAIRING_FOODS
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"\b(?:{alternatives})\b")).expect("pinned pairing foods")
});

fn contains(line: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| line.contains(term))
}

fn mentions(line: &str, wanted: &[&str]) -> bool {
    wanted
        .iter()
        .any(|term| line.contains(&format!(" {term} ")))
}

fn words_of(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    format!("{} {} {}", line.clean, line.norm_text, line.norm.join(" "))
}

fn use_text(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    if line.text.is_empty() {
        &line.clean
    } else {
        &line.text
    }
    .to_lowercase()
}

fn recent_users(earlier: &[Exchange], count: usize) -> impl Iterator<Item = String> + '_ {
    earlier
        .iter()
        .rev()
        .take(count)
        .map(|turn| turn.user.to_lowercase())
}

fn container_keep(earlier: &[Exchange]) -> Option<String> {
    let food = recent_users(earlier, 2).find_map(|line| {
        data::CONTAINER_FOODS
            .iter()
            .copied()
            .find(|term| line.contains(term))
    })?;
    Some(format!(
        "Follow the label’s instructions for refrigerating and using {food} after opening. Discard it if the lid is bulging, the contents look spoiled or the use-by date has passed."
    ))
}

fn baked_keep(line: &str) -> Option<&'static str> {
    data::BAKED_GOODS
        .iter()
        .find_map(|&(name, reply)| line.contains(name).then_some(reply))
}

fn pointed_at(words: &[String]) -> bool {
    let clean: Vec<_> = words
        .iter()
        .map(|word| {
            word.trim_matches(['.', ',', '!', '?'])
                .replace('\'', "")
                .to_lowercase()
        })
        .collect();
    (1..=3).any(|width| {
        clean.windows(width).enumerate().any(|(start, window)| {
            if !data::FREEZE_POINTS_AT.contains(&window.join(" ").as_str()) {
                return false;
            }
            let next = start + width;
            [
                start.checked_sub(1),
                start.checked_sub(2),
                Some(next),
                Some(next + 1),
            ]
            .into_iter()
            .flatten()
            .filter_map(|at| clean.get(at))
            .any(|word| data::FREEZE_WORD_FORMS.contains(&word.as_str()))
        })
    })
}

fn freeze_facts(words: &[&str]) -> Vec<((bool, usize, usize), &'static str)> {
    let mut units: Vec<_> = words.iter().copied().map(Cow::Borrowed).collect();
    for width in 2..=3 {
        units.extend(
            words
                .windows(width)
                .map(|window| Cow::Owned(window.join(" "))),
        );
    }
    let mut facts = Vec::new();
    for &(names, fact) in data::FREEZE_FACTS {
        for name in names {
            let word_count = name.split_whitespace().count();
            for (at, word) in units.iter().enumerate() {
                if word.starts_with(name) {
                    facts.push(((word == name, word_count, at), fact));
                }
            }
        }
    }
    facts
}

fn freeze_food(words: &[String], earlier: &[Exchange]) -> Option<&'static str> {
    if words.iter().any(|word| {
        data::NOT_A_FREEZER.contains(&word.trim_matches(['.', ',', '!', '?', '"', '\'']))
    }) {
        return None;
    }
    let parts: Vec<_> = words.iter().map(String::as_str).collect();
    let best = freeze_facts(&parts)
        .into_iter()
        .fold(None, |best, (score, fact)| {
            if best.is_none_or(|(before, _)| score > before) {
                Some((score, fact))
            } else {
                best
            }
        });
    if let Some((_, fact)) = best {
        return Some(fact);
    }
    if !pointed_at(words) {
        return None;
    }
    let earlier = recent_users(earlier, 4).find_map(|line| {
        let words: Vec<_> = line.split_whitespace().collect();
        let mut matches = freeze_facts(&words).into_iter().map(|(_, fact)| fact);
        let first = matches.next()?;
        matches.all(|fact| fact == first).then_some(first)
    });
    Some(earlier.unwrap_or(data::FREEZE_ANY))
}

fn keep_bake(bakes: &str) -> &'static str {
    if contains(
        bakes,
        &[
            "scone",
            "scones",
            "biscuit",
            "biscuits",
            "cookie",
            "cookies",
            "shortbread",
        ],
    ) {
        data::KEEP_BISCUIT
    } else if contains(
        bakes,
        &[
            "bread",
            "loaf",
            "loaves",
            "sourdough",
            "bun",
            "buns",
            "roll",
            "rolls",
            "focaccia",
            "baguette",
        ],
    ) {
        data::KEEP_BREAD
    } else if contains(bakes, &["cupcake", "muffin"]) {
        data::KEEP_MUFFIN
    } else {
        data::KEEP_CAKE
    }
}

fn bake_keep(line: &str, earlier: &[Exchange]) -> Option<&'static str> {
    if !contains(line, data::BAKE_KEEP_SHAPES) {
        return None;
    }
    let bake = recent_users(earlier, 3).find(|text| contains(text, data::BAKED))?;
    contains(&bake, &["cupcake", "muffin"]).then(|| keep_bake(&bake))
}

fn upper_first(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

fn bake_follow(line: &str, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    if !contains(line, data::BAKE_FOLLOW_SHAPES) {
        return None;
    }
    let bakes: Vec<_> = recent_users(earlier, 3)
        .filter(|text| contains(text, data::BAKED))
        .collect();
    if bakes.is_empty() {
        return None;
    }
    let use_it = contains(line, data::USE_SHAPES);
    let family = std::iter::once(line)
        .chain(bakes.iter().map(String::as_str))
        .find_map(|said| {
            data::BAKE_FAMILIES
                .iter()
                .find(|&&(terms, ..)| contains(said, terms))
        });
    let Some(&(terms, cause, cure, rescue)) = family else {
        return (use_it || contains(line, data::BAKE_KEEP_SHAPES))
            .then(|| Cow::Borrowed(keep_bake(&bakes.join(" "))));
    };
    Some(Cow::Owned(if use_it {
        if matches!(terms.first().copied(), Some("sank" | "gummy")) {
            format!("{}.", upper_first(rescue))
        } else {
            format!("It is still worth eating: {rescue}.")
        }
    } else if contains(line, data::NEXT_SHAPES) {
        format!("Next time, {cure}. {}.", upper_first(cause))
    } else {
        format!("{cause}. {}.", upper_first(cure))
    }))
}

fn milk_kept(reading: &Reading<'_>) -> &'static str {
    let line = &reading.line;
    let said = format!(" {} {} {:?} ", line.clean, line.norm_text, line.norm).to_lowercase();
    if contains(&said, &["formula", "made up", "made-up"]) {
        data::MILK_FORMULA
    } else if contains(&said, data::BREAST_MILK) {
        data::MILK_BREAST
    } else if said.contains("fresh") || contains(&said, data::STORE_ASKS) {
        data::MILK_KEEP
    } else {
        data::MILK_SPOILT
    }
}

pub(super) fn cheese_keeping_asked(reading: &Reading<'_>) -> bool {
    let said = &reading.line.clean;
    !(cooking_pairings::asks_to_pair(said) && PAIRING_FOODS.is_match(said))
}

fn cheese_kept(line: &str) -> &'static str {
    let said = format!(" {line} ");
    if mentions(&said, data::CHEESE_TROUBLE_WORDS)
        || data::CHEESE_TROUBLE_WORDS
            .iter()
            .any(|word| line.starts_with(word))
    {
        return data::CHEESE_MOULD;
    }
    [
        (data::BLUE_CHEESES, data::CHEESE_BLUE),
        (data::SOFT_CHEESES, data::CHEESE_SOFT),
        (data::CHEESE_NAMES, data::CHEESE_HARD),
    ]
    .into_iter()
    .find_map(|(names, reply)| {
        names
            .iter()
            .any(|word| said.contains(&format!(" {word}")))
            .then_some(reply)
    })
    .unwrap_or(data::CHEESE_DEFAULT)
}

fn freeze_cooked(reading: &Reading<'_>) -> String {
    let said = format!(" {} ", reading.line.words.join(" "));
    let keeps = data::FREEZE_DISH_TIMES.iter().find_map(|&(names, keeps)| {
        names
            .iter()
            .any(|name| said.contains(&format!(" {name} ")) || said.contains(&format!(" {name}s ")))
            .then_some(keeps)
    });
    let step = data::FREEZE_COOKED_STEP;
    if let Some(keeps) = keeps {
        format!(
            "Yes - it keeps {keeps} in the freezer, so cool it within an hour, then pack it flat in bags or tubs so it freezes fast. {step}"
        )
    } else {
        format!(
            "Yes - cooked dishes keep about three months frozen, and a month for mash, so cool it within an hour, then pack it flat in bags or tubs so it freezes fast. {step}"
        )
    }
}

fn pulses(reading: &Reading<'_>) -> Option<&'static str> {
    let said = words_of(reading);
    let hard = PULSES_HARD.is_match(&said);
    let tinned = PULSES_TIN.is_match(&said);
    let kidney = PULSES_KIDNEY.is_match(&said);
    if (PULSES_FRESH.is_match(&said)
        && !said.contains("split peas")
        && !PULSES_DRIED.is_match(&said))
        || (tinned && !hard)
        || (PULSES_APPLIANCE.is_match(&said) && !kidney)
    {
        return None;
    }
    Some(if kidney && !tinned {
        data::KIDNEY_INSTRUCTIONS
    } else if hard {
        data::PULSES_HARD_REPLY
    } else if PULSES_NO_SOAK.is_match(&said) {
        data::PULSES_NO_SOAK_REPLY
    } else {
        data::PULSES_DEFAULT
    })
}

fn stock(words: &[String]) -> Option<&'static str> {
    if words.is_empty() {
        return None;
    }
    let said = format!(" {} ", words.join(" "));
    let has = |terms| mentions(&said, terms);
    let kind = if has(data::STOCK_VEG) {
        "veg"
    } else if has(data::STOCK_FISH) {
        "fish"
    } else if has(data::STOCK_BROWN) {
        "brown"
    } else {
        "chicken"
    };
    let &(_, time, method) = data::STOCK_METHODS.iter().find(|row| row.0 == kind)?;
    let keeps = has(data::STOCK_KEEP_ASK);
    if has(data::STOCK_CUBES_ASK) {
        Some(if keeps {
            data::STOCK_CUBE_KEEP
        } else {
            data::STOCK_CUBES
        })
    } else if keeps {
        Some(data::STOCK_KEEP)
    } else {
        Some(
            [
                (data::STOCK_SLOW_ASK, data::STOCK_SLOW),
                (data::STOCK_SKIM_ASK, data::STOCK_SKIM),
                (data::STOCK_HEALTH_ASK, data::STOCK_HEALTH),
                (data::STOCK_FLAVOUR_ASK, data::STOCK_FLAVOUR),
                (data::STOCK_AMOUNT_ASK, data::STOCK_AMOUNT),
                (data::STOCK_TIME_ASK, time),
            ]
            .into_iter()
            .find_map(|(terms, reply)| has(terms).then_some(reply))
            .unwrap_or(method),
        )
    }
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    match id {
        "container-keep-follow" => container_keep(earlier).map(Cow::Owned),
        "baked-keep" => baked_keep(&use_text(reading)).map(Cow::Borrowed),
        "freeze-food" | "freeze-anything" => {
            freeze_food(&reading.line.words, earlier).map(Cow::Borrowed)
        }
        "bake-keep-follow" => bake_keep(&use_text(reading), earlier).map(Cow::Borrowed),
        "bake-follow" => bake_follow(&use_text(reading), earlier),
        "milk" => Some(Cow::Borrowed(milk_kept(reading))),
        "cheese-store" => Some(Cow::Borrowed(cheese_kept(&reading.line.clean))),
        "freeze-cooked" => Some(Cow::Owned(freeze_cooked(reading))),
        "pulses-soak" => pulses(reading).map(Cow::Borrowed),
        "stock-carcass" => stock(&reading.line.words).map(Cow::Borrowed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(said: &str) -> Exchange {
        Exchange {
            user: said.to_owned(),
            bot: "Have some cheese with it.".to_owned(),
            module: "answers",
        }
    }

    #[test]
    fn freezing_uses_specific_names_and_only_users_for_recall() {
        let earlier = [user("There is half a loaf left.")];
        let reply = registered(
            "freeze-anything",
            &Reading::new("Can I freeze it?"),
            &earlier,
        )
        .unwrap();
        assert!(reply.contains("straight in the toaster"));
        let mixed = [user("I have apples and a pack of broccoli.")];
        assert_eq!(
            registered("freeze-anything", &Reading::new("Can I freeze it?"), &mixed).as_deref(),
            Some(data::FREEZE_ANY)
        );
        assert!(
            registered(
                "freeze-anything",
                &Reading::new("Should I freeze my card?"),
                &earlier
            )
            .is_none()
        );
        let cream = registered(
            "freeze-food",
            &Reading::new("Can I freeze cream cheese?"),
            &[],
        )
        .unwrap();
        assert!(cream.starts_with("Soft cheese freezes"));
    }

    #[test]
    fn storage_callbacks_apply_specific_guidance_to_context_and_safety() {
        let earlier = [user("I baked some muffins.")];
        let reply = registered(
            "bake-keep-follow",
            &Reading::new("How long do they keep?"),
            &earlier,
        )
        .unwrap();
        assert_eq!(reply, data::KEEP_MUFFIN);
        let earlier = [user("My cake is wet in the middle.")];
        let reply = registered(
            "bake-follow",
            &Reading::new("Can I still eat it?"),
            &earlier,
        )
        .unwrap();
        assert!(reply.contains("bake it fully"));
        let earlier = [user("I opened a jar of jam.")];
        let reply = registered(
            "container-keep-follow",
            &Reading::new("How long will it keep?"),
            &earlier,
        )
        .unwrap();
        assert!(reply.contains("jam after opening"));
        assert!(!cheese_keeping_asked(&Reading::new(
            "What pairs best with blue cheese?"
        )));
        assert!(cheese_keeping_asked(&Reading::new(
            "How do I store blue cheese?"
        )));
        assert_eq!(
            pulses(&Reading::new(
                "Can I cook dried kidney beans in a slow cooker?"
            )),
            Some(data::KIDNEY_INSTRUCTIONS)
        );
        assert!(pulses(&Reading::new("How long do I cook green beans?")).is_none());
        let stock = Reading::new("How long will homemade stock keep in the fridge?");
        assert_eq!(super::stock(&stock.line.words), Some(data::STOCK_KEEP));
    }
}
