use super::*;
use crate::bot::Bot;

#[test]
fn replenishes_and_expires_stock_separately_from_meal_offers() {
    let mut bot = Bot::new(Some("0")).unwrap();
    let first = bot.turn("i have chickpeas and spinach");
    assert_eq!(first.module, Some("answers"), "{first:?}");
    assert_eq!(
        first.reply,
        "Chickpeas and spinach: fry onion, garlic, ginger and cumin, add the chickpeas and a splash of stock, then wilt the spinach through at the end. Ten minutes, and it's as good cold as hot."
    );
    let pantry = &bot.memory.pantry;
    assert_eq!(pantry.meal.as_ref().unwrap().have, ["chickpea", "spinach"]);
    assert!(bot.memory.meal_plan.is_none());
    let turn = bot.turn("what can i make?");
    assert_eq!(turn.module, Some("answers"), "{turn:#?}");
    assert_eq!(
        turn.reply,
        "A quick chickpea curry: fry onion, garlic and curry powder for 5 minutes, add the tinned tomatoes and chickpeas, and simmer for 15. Spinach or yoghurt stirred in at the end rounds it off."
    );
    assert_eq!(
        bot.memory.pantry.meal.as_ref().unwrap().offered,
        ["chickpea-spinach"]
    );
    bot.turn("hello");
    bot.turn("what is the capital of france");
    assert!(bot.memory.pantry.recent(bot.memory.turns.len()).is_none());
}

#[test]
fn retains_stock_without_reactivating_a_spoiled_item() {
    for (stock, problem, food) in [
        (
            "i have some eggs in the fridge",
            "my eggs are mouldy",
            "egg",
        ),
        ("i have a tin of tuna", "my tuna smells sour", "tuna"),
        ("i have a tin of tuna", "my tuna sat out overnight", "tuna"),
    ] {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn(stock);
        bot.turn(problem);
        let plan = bot.memory.pantry.meal.as_ref().unwrap();
        assert_eq!((plan.turn, plan.have.as_slice()), (0, &[food][..]));
        assert!(bot.memory.pantry.blocked(food, &bot.memory));
        let ask = Reading::new("what can i make?");
        assert!(bot.memory.pantry.recipe(&ask, &bot.memory).is_none());
        assert!(
            bot.memory
                .pantry
                .blocks_reply(&ask, &format!("Cook the {food}."), &bot.memory)
        );
    }
}

#[test]
fn allergy_names_follow_actual_food_categories() {
    for (allergen, food) in [
        ("egg", "egg"),
        ("eggs", "egg"),
        ("fish", "tuna"),
        ("fish", "salmon"),
        ("dairy", "milk"),
        ("milk", "yoghurt"),
        ("shellfish", "prawn"),
    ] {
        assert!(conflicts(allergen, food), "{allergen} / {food}");
    }
    assert!(!conflicts("peanuts", "pea"));
    assert!(!conflicts("fish", "prawn"));
    assert!(!conflicts("milk", "coconut milk"));
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("i am allergic to fish");
    let turn = bot.turn("i have tuna in the fridge");
    assert!(
        !turn
            .candidates
            .iter()
            .any(|proposal| proposal.reply.contains("Eat it cold")),
        "{turn:?}"
    );
    assert!(bot.memory.pantry.blocked("tuna", &bot.memory));
    assert!(
        bot.memory
            .pantry
            .recipe(&Reading::new("what can i make?"), &bot.memory)
            .is_none()
    );
}

#[test]
fn no_dairy_filters_a_recipe_without_destroying_the_stock_record() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("i have pasta and cheese");
    bot.turn("no dairy please");
    let plan = bot.memory.pantry.meal.as_ref().unwrap();
    assert_eq!(plan.have, ["pasta", "cheese"]);
    assert_eq!(plan.avoid, DAIRY);
    let turn = bot.turn("what can i make?");
    assert_eq!(turn.module, Some("answers"), "{turn:#?}");
    assert_eq!(
        turn.reply,
        "A 15-minute tomato pasta: soften sliced garlic in olive oil, add the tinned tomatoes and a pinch of salt, and simmer while the pasta cooks. Tear in a few basil leaves at the end and loosen it with a splash of pasta water so it clings."
    );
}

#[test]
fn a_later_smell_report_does_not_make_a_previously_flagged_food_available() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("i have a tin of tuna");
    let problem = bot.turn("my tuna smells sour");
    assert_eq!(problem.module, Some("answers"), "{problem:?}");
    bot.turn("it smells normal now");
    assert!(bot.memory.pantry.blocked("tuna", &bot.memory));
    let turn = bot.turn("what can i make?");
    assert!(
        !turn
            .candidates
            .iter()
            .any(|proposal| proposal.reply.to_lowercase().contains("tuna pasta")),
        "{turn:?}"
    );
}

#[test]
fn an_allergy_stays_separate_from_pantries_that_expire() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("i am allergic to peanuts");
    bot.turn("i have peas in the fridge");
    assert_eq!(bot.memory.pantry.meal.as_ref().unwrap().have, ["pea"]);
    assert!(!bot.memory.pantry.blocked("pea", &bot.memory));
    assert!(bot.memory.pantry.blocked("peanut", &bot.memory));
    bot.turn("hello");
    bot.turn("what is the capital of france");
    assert!(bot.memory.pantry.recent(bot.memory.turns.len()).is_none());
    assert!(bot.memory.pantry.blocked("peanut", &bot.memory));
}

#[test]
fn typed_follow_up_cannot_publish_with_a_remembered_milk_allergy() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("what should i cook tonight");
    // Keep the selected meal in place while installing a food fact held by the personal store.
    bot.memory.personal.observe(
        &Reading::new("i am allergic to milk"),
        bot.memory.turns.len(),
    );
    let reading = Reading::new("another idea");
    let proposal =
        crate::bot::knowledge::meal_shortlist::another_idea(&reading, &bot.memory).unwrap();
    assert!(proposal.reply.contains("macaroni cheese"));
    assert!(bot.memory.pantry.blocks_typed_offer(&bot.memory));

    let turn = bot.turn("another idea");
    assert!(
        !turn
            .candidates
            .iter()
            .any(|candidate| candidate.reply == proposal.reply)
    );
    assert_eq!(bot.memory.meal_plan.as_ref().unwrap().turn, 0);
}

#[test]
fn recent_meal_context_applies_the_fish_exclusion_to_generic_replies() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("what should i cook tonight");
    bot.turn("i am allergic to fish");
    let continued = Reading::new("another idea");
    assert!(
        bot.memory
            .pantry
            .blocks_reply(&continued, "Try a tuna salad.", &bot.memory)
    );
    assert!(!bot.memory.pantry.blocks_reply(
        &Reading::new("what is the capital of france"),
        "Try a tuna salad.",
        &bot.memory,
    ));
    let turn = bot.turn("another idea");
    assert!(
        !turn
            .candidates
            .iter()
            .any(|proposal| proposal.reply.contains("tuna"))
    );
    assert_eq!(bot.memory.meal_plan.as_ref().unwrap().turn, 0);
}
