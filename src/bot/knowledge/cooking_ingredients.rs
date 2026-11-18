//! Staple methods, ingredient forms and gluten facts from the pinned registry.

use super::super::{food_fold::fold, ingredient_card_data as data};
use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

macro_rules! pattern {
    ($label:ident, $source:expr) => {
        static $label: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($label)));
    };
}

pattern!(GLUTEN_OPENING, data::GLUTEN_FACT_OPENING);
pattern!(GLUTEN_MAKING, data::GLUTEN_FACT_MAKING);
pattern!(WORD, r"[a-z']+");
pattern!(HEAT, data::HEAT_OR_TIME);
pattern!(MEASURE, data::HOW_MUCH);
pattern!(SIDE, data::A_SIDE);
pattern!(OVEN, data::OVEN_WORD);

fn get(name: &str, rows: &'static [(&'static str, &'static str)]) -> Option<&'static str> {
    rows.iter()
        .find(|(key, _)| *key == name)
        .map(|(_, value)| *value)
}

fn has(text: &str, cues: &[&str]) -> bool {
    cues.iter().any(|cue| text.contains(cue))
}

fn gluten_food(text: &str) -> Option<(usize, usize, &'static str, bool)> {
    let words: Vec<_> = WORD.find_iter(text).collect();
    for size in [2, 1] {
        for pair in words.windows(size) {
            let name = pair
                .iter()
                .map(|word| word.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            if let Some(&(food, gluten)) = data::GLUTEN_LOOKUP.iter().find(|(key, _)| *key == name)
            {
                return Some((pair.first()?.start(), pair.last()?.end(), food, gluten));
            }
        }
    }
    None
}

fn gluten_fact(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    if !GLUTEN_OPENING.is_match(text) {
        return None;
    }
    let (start, end, food, gluten) = gluten_food(text)?;
    if GLUTEN_MAKING.is_match(&text.replace(food, " ")) {
        return None;
    }
    if let Some(reply) = get(food, data::GLUTEN_FACT_NOT_PLAIN) {
        return Some(Cow::Borrowed(reply));
    }
    let mut previous = text[..start].split_whitespace().rev();
    let article = previous.next();
    if article == Some("free")
        && previous
            .next()
            .is_some_and(|word| word.bytes().all(|c| c.is_ascii_lowercase()))
    {
        return None;
    }
    if matches!(
        food,
        "oats" | "porridge" | "oatmeal" | "oat milk" | "oat drink"
    ) {
        return Some(Cow::Borrowed(
            "Choose a product labelled gluten-free. Ordinary oats can pick up gluten from wheat, rye or barley during growing and processing. Oats themselves can also be a problem for some people with coeliac disease.",
        ));
    }
    if let Some(reply) = get(food, data::NOT_SETTLED_BY_THE_RECIPE) {
        return Some(Cow::Borrowed(reply));
    }
    let following = WORD.find(&text[end..]).map(|word| word.as_str());
    let another_food =
        following.is_some_and(|word| data::GLUTEN_LOOKUP.iter().any(|(name, _)| *name == word));
    let named = match (another_food, article) {
        (false, Some(article @ ("a" | "an" | "the"))) => format!("{article} {food}"),
        _ => food.to_owned(),
    };
    let tail = food.split_whitespace().last()?;
    let plural =
        tail.ends_with('s') && !tail.ends_with("ss") && !data::SINGULAR_IN_S.contains(&tail);
    if gluten {
        let mut reply = if matches!(food, "barley" | "rye" | "soy sauce" | "soya sauce") {
            format!("No: {named} usually contains gluten.")
        } else {
            let predicate = if plural {
                "are made with wheat flour"
            } else {
                "is made with wheat flour"
            };
            format!("No: {named} {predicate}, so it is not gluten free.")
        };
        if let Some(swap) = get(food, data::GLUTEN_FACT_NAMED_SWAP) {
            reply.push(' ');
            reply.push_str(if matches!(food, "soy sauce" | "soya sauce") {
                "Look for tamari labelled gluten-free and check the ingredients."
            } else {
                swap
            });
        } else if data::GLUTEN_FACT_SWAPS.contains(&food) {
            reply.push_str(" Rice, potatoes or a corn pasta do the same job.");
        } else {
            reply.push_str(" The gluten-free one is a different product, so check the label.");
        }
        return Some(Cow::Owned(reply));
    }
    let predicate = if plural {
        "have no gluten in them. The plain ones are gluten free"
    } else {
        "has no gluten in it. The plain one is gluten free"
    };
    Some(Cow::Owned(format!(
        "Yes: {named} {predicate}; check the packet on flavoured or own-brand versions."
    )))
}

pub(in crate::bot::knowledge::lookup) fn asks_gluten_fact(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    GLUTEN_OPENING.is_match(text)
        && has(text, data::GLUTEN_FACT_ASKS)
        && gluten_food(text)
            .is_some_and(|(_, _, food, _)| !GLUTEN_MAKING.is_match(&text.replace(food, " ")))
}

const FORMS: &[(&str, &[&str])] = &[
    ("frozen", &["frozen", "freezer"]),
    ("tinned", &["tinned", "canned", "tin of", "can of"]),
    ("dried", &["dried", "dry"]),
    ("jarred", &["jarred", "jar of"]),
    ("fresh", &["fresh"]),
];

fn form_for(
    words: &[String],
    start: usize,
    size: usize,
) -> Option<(&'static str, &'static [&'static str])> {
    words[start..start + size]
        .iter()
        .chain(words[start.saturating_sub(2)..start].iter())
        .find_map(|word| {
            FORMS
                .iter()
                .copied()
                .find(|(_, forms)| forms.contains(&word.as_str()))
        })
}

fn form_swap(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words: Vec<_> = reading.line.words.iter().map(|word| fold(word)).collect();
    if !has(&words.join(" "), data::SWAP_ASK) {
        return None;
    }
    let mut used = vec![false; words.len()];
    let mut found = None;
    let mut seen = Vec::new();
    for &(parts, food, alias) in data::FORM_ALIASES {
        let width = parts.len();
        for (start, window) in words.windows(width).enumerate() {
            if used[start..start + width].iter().any(|used| *used)
                || !window.iter().map(String::as_str).eq(parts.iter().copied())
            {
                continue;
            }
            used[start..start + width].fill(true);
            seen.push(food);
            if found.is_none()
                && let Some((form, forms)) = form_for(&words, start, width)
            {
                found = Some((start, width, food, alias, form, forms));
            }
            break;
        }
    }
    let Some((start, width, food, alias, form, forms)) = found else {
        return words
            .iter()
            .enumerate()
            .any(|(at, word)| {
                data::FRUIT_WORDS.contains(&word.as_str())
                    && form_for(&words, at, 1).is_some_and(|(form, _)| form == "frozen")
            })
            .then_some(Cow::Borrowed(data::FROZEN_FRUIT));
    };
    if !data::FORM_FOODS.contains(&food) || seen.iter().any(|other| *other != food) {
        return None;
    }
    let mut parts = alias.splitn(2, ' ');
    let shown = if parts
        .next()
        .is_some_and(|word| forms.contains(&word.to_lowercase().as_str()))
    {
        parts.next().unwrap_or(alias)
    } else {
        alias
    };
    let reply = if has(&words[..start + width].join(" "), data::OUT_ASKS) {
        get(form, data::OUT_OF_FORM)?
    } else if form == "frozen" && data::MEAT.contains(&food) {
        "Frozen {food} can substitute for fresh. Defrost it completely in the fridge in a covered container on the bottom shelf, then cook as directed. Cook it from frozen only when the pack or recipe gives safe instructions for doing so."
    } else if form == "frozen" && data::FRUIT_WORDS.contains(&food) {
        data::FROZEN_FRUIT
    } else {
        get(form, data::FORM_ADVICE)?
    };
    Some(Cow::Owned(reply.replace("{food}", shown)))
}

pub(in crate::bot::knowledge::lookup) fn method_asked(reading: &Reading<'_>) -> bool {
    let said = [&reading.line.clean, &reading.line.norm_text]
        .into_iter()
        .find(|line| !line.is_empty());
    let joined;
    let text = match said {
        Some(text) => text,
        None => {
            joined = reading.line.words.join(" ");
            &joined
        }
    };
    HEAT.is_match(text) || (!MEASURE.is_match(text) && !SIDE.is_match(text))
}

fn mentions(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(&format!(" {word} ")))
}

fn corn(words: &[String]) -> &'static str {
    let line = format!(" {} ", words.join(" "));
    if mentions(
        &line,
        &[
            "microwave",
            "microwaved",
            "microwaving",
            "microwaves",
            "micro",
        ],
    ) {
        "In the microwave a cob takes 3 to 4 minutes on full power for one and 5 to 6 for two: leave it in its husk, or wrap it in a damp piece of kitchen paper, and turn it once. It carries on cooking as it stands, so give it a minute before you butter it. Loose kernels in a covered microwave-safe bowl with a splash of water take 2 to 3 minutes, stirred halfway. Follow the cooking directions on the pack for frozen sweetcorn."
    } else if mentions(&line, &["air fryer", "airfryer", "air fried", "airfry"]) {
        "In the air fryer rub the cobs with a little oil and give them 10 to 12 minutes at 190C, turning once: the kernels blister at the tips and that is the point. Two cobs fit in a normal basket; more than that and they steam instead of browning."
    } else if mentions(
        &line,
        &[
            "grill", "grilled", "grilling", "barbecue", "bbq", "char", "charred",
        ],
    ) {
        "Under a hot grill or on the barbecue, 10 to 12 minutes, turning every few minutes until the kernels start to char in patches - rub them with a little butter or oil first so they colour rather than dry. Boiled instead, 6 to 8 minutes in unsalted water; salt toughens it."
    } else {
        "Boil it in unsalted water for 6 to 8 minutes - salt toughens it - or grill it, turning it now and then, for 10 to 12 minutes until the kernels start to char. In the microwave a cob takes 3 to 4 minutes on full power. Butter, salt and pepper, and it needs nothing else."
    }
}

fn appliance_and_cut(text: &str, words: &[String]) -> bool {
    (OVEN.is_match(text) || mentions(text, data::APPLIANCE_NAMES))
        && words
            .iter()
            .any(|word| data::CUT_WORDS.contains(&word.as_str()))
}

fn staple(reading: &Reading<'_>) -> Option<&'static str> {
    let original = &reading.line.words;
    let words: Vec<_> = original.iter().map(|word| fold(word)).collect();
    let joined = format!(" {} ", words.join(" "));
    if has(&joined, data::NOT_A_METHOD)
        || (has(&joined, data::KNOWS_READY)
            && !mentions(&joined, data::DONE_NOT)
            && mentions(&joined, data::HARVEST_CROPS))
    {
        return None;
    }
    let sauce = words
        .iter()
        .any(|word| matches!(word.as_str(), "sauce" | "saucy"));
    for &(names, reply) in data::STAPLES {
        for name in names {
            if sauce
                && !matches!(
                    *name,
                    "white sauce" | "cheese sauce" | "bechamel" | "béchamel"
                )
            {
                continue;
            }
            let wanted: Vec<_> = name.split_whitespace().map(fold).collect();
            for (start, window) in words.windows(wanted.len()).enumerate() {
                if window != wanted {
                    continue;
                }
                let end = start + wanted.len();
                let before = start
                    .checked_sub(1)
                    .and_then(|index| words.get(index))
                    .map_or("", String::as_str);
                let after = words.get(end).map_or("", String::as_str);
                if data::DISH_BEFORE.contains(&before) || data::DISH_AFTER.contains(&after) {
                    continue;
                }
                let own = fold(reply);
                let guards = data::NOT_WANTED
                    .iter()
                    .filter(|(word, _)| *word == names[0] || *word == *name)
                    .flat_map(|(_, terms)| terms.iter());
                let blocked = guards.into_iter().any(|guard| {
                    if guard.contains(' ') {
                        let phrase = fold(guard);
                        joined.contains(&phrase)
                            && !own.contains(&phrase)
                            && (!data::AN_APPLIANCE.contains(guard)
                                || !appliance_and_cut(&joined, &words))
                    } else {
                        words[..start]
                            .iter()
                            .chain(&words[end..])
                            .any(|word| word == guard)
                    }
                });
                if !blocked {
                    return Some(if reply.is_empty() {
                        corn(original)
                    } else {
                        reply
                    });
                }
            }
        }
    }
    None
}

pub(super) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "gluten-free-fact" => gluten_fact(reading),
        "form-swap" => form_swap(reading),
        "cook-a-staple" => staple(reading).map(Cow::Borrowed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facts_and_methods_distinguish_named_food_and_portion_questions() {
        let rice = Reading::new("Is rice gluten free?");
        assert!(asks_gluten_fact(&rice));
        assert!(gluten_fact(&rice).unwrap().contains("rice has no gluten"));
        let soy = Reading::new("Is soy sauce gluten free?");
        assert!(
            gluten_fact(&soy)
                .unwrap()
                .contains("tamari labelled gluten-free")
        );
        let oats = Reading::new("Does oat milk have gluten?");
        assert!(gluten_fact(&oats).unwrap().contains("labelled gluten-free"));
        assert!(!asks_gluten_fact(&Reading::new(
            "Can I make bread without gluten?"
        )));
        assert!(!method_asked(&Reading::new(
            "How much rice for four people?"
        )));
        assert!(method_asked(&Reading::new(
            "How many minutes to cook rice?"
        )));
        let quinoa = Reading::new("How long does quinoa take to cook?");
        assert!(
            staple(&quinoa)
                .unwrap()
                .starts_with("Quinoa takes 12 to 15 minutes")
        );
        let microwave_corn = Reading::new("How do I microwave corn on the cob?");
        assert!(
            staple(&microwave_corn)
                .unwrap()
                .starts_with("In the microwave")
        );
        assert!(staple(&Reading::new("How do I know when my onions are ready?")).is_none());
    }

    #[test]
    fn swaps_tell_substitution_apart_from_shortage_and_pairing() {
        let spinach = Reading::new("Can I use frozen spinach instead of fresh?");
        assert!(form_swap(&spinach).unwrap().starts_with("Frozen spinach"));
        let dried_lentils = Reading::new("We have run out of dried lentils");
        assert!(
            form_swap(&dried_lentils)
                .unwrap()
                .contains("tinned is the stand-in")
        );
        let pair = Reading::new("Can I use fresh tomatoes instead of spinach?");
        assert!(form_swap(&pair).is_none());
    }
}
