use super::{Reading, open_dinner, shortlist};
use crate::bot::Bot;

#[test]
fn shortlist_keeps_source_names_and_opening_shape() {
    let answer = shortlist("what should i cook tonight?", 0, Some("pick"), &[]);
    assert_eq!(answer.offered, ["tuna pasta", "a frittata", "dhal"]);
    assert_eq!(
        answer.reply,
        "Here's my pick: tuna pasta in 15 minutes, or a frittata in 20, or a lentil dhal in 30. Tell me what is in the fridge and I will narrow it down."
    );

    let answer = shortlist("what should i cook tonight?", 1, None, &[]);
    assert_eq!(
        answer.offered,
        ["tuna pasta", "a frittata", "vegetable soup"]
    );
    assert_eq!(
        answer.reply,
        "Quick: tuna pasta in 15, a frittata in 20, or vegetable soup if you have 30 minutes. Tell me what is in the fridge and I will pick properly."
    );
}

#[test]
fn shortlist_can_exclude_previous_offers() {
    let answer = shortlist(
        "what should i cook tonight",
        0,
        Some("pick"),
        &["tuna pasta", "a frittata", "a salmon traybake"],
    );
    assert_eq!(
        answer.offered,
        ["pea and mint soup", "tomato pasta", "a potato curry"]
    );
    assert_eq!(
        answer.reply,
        "Here's my pick: pea and mint soup in 15 minutes, or tomato pasta in 20, or a potato curry in 30. Tell me what is in the fridge and I will narrow it down."
    );
}

#[test]
fn open_shortlist_declines_constrained_meal_asks() {
    assert!(open_dinner(&Reading::new("What should I cook tonight?"), 0).is_some());
    for line in [
        "what can i cook with a tin of tomatoes",
        "what should i cook for dinner without dairy",
        "what can i make for eight people",
        "what can i make in 20 minutes",
        "what is safe to eat after my rice sat out all night",
        "is the sour smelling chicken safe to cook for dinner",
        "what can i eat for dinner if i have a nut allergy",
        "what can i make for dinner? i have a peanut allergy",
        "the chicken smells sour. what can i cook tonight?",
        "what can i make for lunch",
        "what can i have tonight",
        "what should we have this evening?",
        "what should i cook for tea",
        "what could we eat this evening",
        "what shall we eat tonight",
        "what should i wear to a dinner party",
    ] {
        assert!(open_dinner(&Reading::new(line), 0).is_none(), "{line}");
    }
}

#[test]
fn another_idea_excludes_the_held_offers_during_its_proposal_passes() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("what should i cook tonight");
    let turn = bot.turn("another idea");
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(
        turn.reply,
        "Quick: tuna pasta in 15, macaroni cheese in 25, or a sausage traybake if you have 45 minutes. Tell me what is in the fridge and I will pick properly."
    );
    let plan = bot.memory.meal_plan.as_ref().unwrap();
    assert_eq!(plan.turn, 1);
    assert_eq!(
        plan.offered(),
        [
            "a sausage traybake",
            "pea and mint soup",
            "chickpea curry",
            "a chicken traybake"
        ]
    );
}

#[test]
fn a_food_constraint_keeps_an_older_shortlist_out_of_the_follow_up() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("what should i cook tonight");
    bot.turn("i am allergic to nuts");
    assert!(super::another_idea(&Reading::new("another idea"), &bot.memory).is_none());
    let turn = bot.turn("another idea");
    assert_eq!(bot.memory.meal_plan.as_ref().unwrap().turn, 0, "{turn:?}");
}

#[test]
fn asks_with_one_ingredient_commit_the_authored_pick() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("what should i cook tonight");
    let turn = bot.turn("what can i make with chickpeas");
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(
        turn.reply,
        "Try a chickpea salad: Toss a drained tin of chickpeas with chopped tomato and cucumber, dress it with lemon juice, olive oil and salt, and crumble cheese over it if you eat it. About 10 minutes. If you'd rather have something else, spinach and chickpea curry takes 25 minutes."
    );
    let plan = bot.memory.meal_plan.as_ref().unwrap();
    assert_eq!(plan.turn, 1);
    assert_eq!(plan.ingredient, Some(super::recipes::Ingredient::Chickpea));
    assert_eq!(plan.latest[0], "a sweet potato curry");
    assert_eq!(
        plan.offered(),
        [
            "a squash and chickpea traybake",
            "a chickpea salad",
            "spinach and chickpea curry",
            "a sweet potato curry"
        ]
    );
}

#[test]
fn ingredient_ask_defers_to_an_allergy_or_other_constraint() {
    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("what should i cook tonight");
    for prompt in [
        "what can i make with chickpeas without dairy",
        "what can i make with peas and tuna",
        "what can i make with tuna that smells sour",
    ] {
        assert!(
            super::with_ingredient(&Reading::new(prompt), &bot.memory).is_none(),
            "{prompt}"
        );
    }
    bot.turn("i am allergic to fish");
    assert!(
        super::with_ingredient(&Reading::new("what can i make with tuna"), &bot.memory).is_none()
    );
    let turn = bot.turn("what can i make with tuna");
    assert_eq!(bot.memory.meal_plan.as_ref().unwrap().turn, 0, "{turn:?}");
}
