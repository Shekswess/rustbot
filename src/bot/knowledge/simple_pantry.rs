//! The pinned pantry's recipe combinations and dairy-restricted single-ingredient fallback.

use super::{PANTRY_DISHES, SINGLE_INGREDIENT_ALIASES, SINGLE_INGREDIENT_REPLIES, food_fold};
use crate::bot::{Memory, Reading, understanding::normalize::tokens};

pub(super) struct PantryDish {
    pub(super) uses: &'static [&'static str],
    pub(super) reply: &'static str,
    pub(super) has_dairy: bool,
}

const MEAL_QUESTIONS: &[&str] = &[
    "what can i make",
    "what can i cook",
    "what should i make",
    "what should i cook",
    "what can i do",
];

pub(super) fn combination(reading: &Reading<'_>, memory: &Memory) -> Option<&'static str> {
    let question = &reading.line.clean;
    if !MEAL_QUESTIONS.iter().any(|ask| question.contains(ask)) {
        return None;
    }
    let normalized = tokens(reading.raw).join(" ");
    let restricted = normalized.strip_suffix(" without dairy");
    let sentence = restricted.unwrap_or(&normalized);
    let pantry = MEAL_QUESTIONS.iter().find_map(|ask| {
        let (before, after) = sentence.split_once(ask)?;
        match (before.trim(), after.trim()) {
            ("", tail) => tail.strip_prefix("with "),
            (list, "") => ["i have ", "ive got ", "i've got ", "we have "]
                .into_iter()
                .find_map(|lead| list.strip_prefix(lead)),
            _ => None,
        }
    })?;
    let dairy_free = restricted.is_some();
    let ingredients = fresh_list(pantry)?;
    // The full pantry carries exclusions through the last four turns. Leave those
    // cases, and repeated suggestions, to the existing contextual readers.
    if memory.turns.iter().rev().take(4).any(|turn| {
        tokens(&turn.user).iter().any(|word| {
            matches!(
                word.as_str(),
                "allergic"
                    | "allergy"
                    | "avoid"
                    | "cant"
                    | "can't"
                    | "cannot"
                    | "coeliac"
                    | "free"
                    | "gluten"
                    | "intolerant"
                    | "no"
                    | "vegan"
                    | "vegetarian"
                    | "without"
            )
        })
    }) {
        return None;
    }
    PANTRY_DISHES
        .iter()
        .find(|dish| {
            ingredients.len() == dish.uses.len()
                && (!dairy_free || !dish.has_dairy)
                && dish.uses.iter().all(|food| ingredients.contains(food))
                && memory
                    .turns
                    .last()
                    .is_none_or(|turn| turn.bot != dish.reply)
        })
        .map(|dish| dish.reply)
}

fn fresh_list(pantry: &str) -> Option<Vec<&'static str>> {
    let mut names = Vec::new();
    let mut tinned = false;
    for word in pantry.split_whitespace() {
        if matches!(word, "tinned" | "canned") {
            tinned = true;
            continue;
        }
        if matches!(word, "a" | "and" | "an" | "some" | "the") {
            tinned = false;
            continue;
        }
        let word = food_fold::fold(word);
        let index = SINGLE_INGREDIENT_ALIASES
            .binary_search_by_key(&word.as_str(), |(alias, _)| alias)
            .ok()?;
        let name = SINGLE_INGREDIENT_ALIASES[index].0;
        if name == "bean" && !tinned {
            return None;
        }
        tinned = false;
        if !names.contains(&name) {
            names.push(name);
        }
    }
    (names.len() >= 2).then_some(names)
}

pub(super) fn dairy_free(reading: &Reading<'_>) -> Option<&'static str> {
    let words = tokens(reading.raw);
    let sentence = words.join(" ");
    let (restriction, ingredient) = [
        " what can i do with ",
        " what do i do with ",
        " what to do with ",
        " what can i make with ",
        " what can i cook with ",
    ]
    .into_iter()
    .find_map(|ask| sentence.split_once(ask))?;
    let restriction = ["i'm ", "i am ", "im ", "we're ", "we are ", "were "]
        .into_iter()
        .find_map(|lead| restriction.strip_prefix(lead))
        .unwrap_or(restriction);
    if !matches!(
        restriction,
        "dairy free" | "dairyfree" | "lactose intolerant" | "no dairy" | "milk free"
    ) {
        return None;
    }
    let ingredient = ["a ", "an ", "the ", "some "]
        .into_iter()
        .find_map(|lead| ingredient.strip_prefix(lead))
        .unwrap_or(ingredient);
    let ingredient = ingredient
        .split_whitespace()
        .map(food_fold::fold)
        .collect::<Vec<_>>()
        .join(" ");
    let alias = SINGLE_INGREDIENT_ALIASES
        .binary_search_by_key(&ingredient.as_str(), |(alias, _)| alias)
        .ok()?;
    SINGLE_INGREDIENT_REPLIES[SINGLE_INGREDIENT_ALIASES[alias].1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn recipe_pairs_match_the_pinned_answers() {
        let cases = [
            (
                "I have chickpeas and spinach. What can I make without dairy?",
                "Chickpeas and spinach: fry onion, garlic, ginger and cumin, add the chickpeas and a splash of stock, then wilt the spinach through at the end. Ten minutes, and it's as good cold as hot.",
            ),
            (
                "What should I make with tomatoes and pasta?",
                "A 15-minute tomato pasta: soften sliced garlic in olive oil, add the tinned tomatoes and a pinch of salt, and simmer while the pasta cooks. Tear in a few basil leaves at the end and loosen it with a splash of pasta water so it clings.",
            ),
            (
                "What can I make with aubergine and pasta?",
                "Aubergine pasta: fry cubed aubergine in plenty of olive oil for 10 minutes until soft, add garlic and a tin of tomatoes, simmer 10 minutes and toss with the pasta.",
            ),
            (
                "What can I cook with garlic and pasta?",
                "Aglio e olio: warm 4 sliced garlic cloves and a little chilli in 4 tablespoons of olive oil until they just colour, then toss with the cooked spaghetti and a splash of pasta water. Ten minutes, and salt it properly.",
            ),
            (
                "What can I make with chickpeas and tomatoes?",
                "A quick chickpea curry: fry onion, garlic and curry powder for 5 minutes, add the tinned tomatoes and chickpeas, and simmer for 15. Spinach or yoghurt stirred in at the end rounds it off.",
            ),
            (
                "What can I cook with tinned beans and tomatoes?",
                "A quick bean stew: soften onion and garlic, add a tin of beans and a tin of tomatoes with paprika and cumin, and simmer for 15 minutes. Good on toast, rice or a jacket potato.",
            ),
            (
                "What can I cook with peppers and broccoli?",
                "Stir-fry the pepper and broccoli in a very hot pan for 4 minutes with garlic and soy sauce. Add noodles or rice, and keep everything moving so it chars rather than steams.",
            ),
        ];
        for (prompt, expected) in cases {
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            assert_eq!(turn.module, Some("answers"), "{prompt}");
            assert_eq!(turn.reply, expected, "{prompt}");
        }
    }

    #[test]
    fn recipe_pairs_defer_when_ingredients_or_restrictions_need_more_context() {
        for prompt in [
            "What can I make with dried beans and tomatoes?",
            "What can I make with chickpeas and tomatoes without dairy?",
            "What can I cook with cooked spinach and chickpeas?",
            "What can I make with chickpeas and spinach without dairy or onions?",
            "Can I eat chickpeas and spinach that I left out overnight?",
        ] {
            assert_eq!(
                combination(&Reading::new(prompt), &Memory::default()),
                None,
                "{prompt}"
            );
        }
        let mut memory = Memory::default();
        memory.turns.push(crate::bot::Exchange {
            user: "I can't eat onions".to_owned(),
            bot: String::new(),
            module: "greeting",
        });
        assert_eq!(
            combination(
                &Reading::new("What can I make with spinach and chickpeas?"),
                &memory
            ),
            None
        );
    }

    #[test]
    fn simple_meal_uses_the_source_dairy_free_variant_and_preserves_restrictions() {
        assert_eq!(
            dairy_free(&Reading::new(
                "I'm dairy free. What can I do with a courgette?"
            )),
            Some("Slice the courgette thick, oil and salt it, and grill 4 minutes a side."),
        );
        for text in [
            "I'm dairy free. What can I make with some cheese?",
            "I'm not dairy free. What can I do with a courgette?",
            "I'm dairy free. What can I do with spinach and courgettes?",
        ] {
            assert_eq!(dairy_free(&Reading::new(text)), None, "{text}");
        }
    }
}
