use super::*;
use crate::bot::Bot;

const VEGAN: &str = "I'd do a potato curry (30 minutes): fry onion with spices, add cubed potato and tomatoes with a splash of water and simmer 20 minutes. Something else that works is vegetable soup.";
const VEGETARIAN: &str = "I'd do chickpea curry (25 minutes): fry an onion with curry powder, add a tin of chickpeas and a tin of tomatoes and simmer for 15 minutes. Something else that works is a lentil dhal.";

#[test]
fn direct_diet_requests_publish_the_second_pair_and_hold_the_third_pick() {
    for (diet, ask, expected, held) in [
        (
            Diet::Vegan,
            "what can i cook for a vegan",
            VEGAN,
            [
                "a potato curry",
                "vegetable soup",
                "tomato pasta",
                "chickpea curry",
            ],
        ),
        (
            Diet::Vegetarian,
            "what can i cook for a vegetarian",
            VEGETARIAN,
            ["chickpea curry", "dhal", "a bean stew", "a potato curry"],
        ),
    ] {
        let mut bot = Bot::new(Some("23")).unwrap();
        let turn = bot.turn(ask);
        assert_eq!(
            (turn.module, turn.reply.as_str()),
            (Some("answers"), expected)
        );
        let plan = bot.memory.meal_plan.unwrap();
        assert_eq!(plan.turn, 0);
        assert_eq!(plan.diet, Some(diet));
        assert_eq!(plan.offered(), held);
    }
}

#[test]
fn dietary_detail_uses_the_matching_fresh_menu_without_committing_a_plan() {
    for (prompt, expected) in [("i am vegan", VEGAN), ("i am vegetarian", VEGETARIAN)] {
        let mut bot = Bot::new(Some("23")).unwrap();
        let turn = bot.turn(prompt);
        assert_eq!(
            (turn.module, turn.reply.as_str()),
            (Some("detail_question"), expected)
        );
        assert!(bot.memory.meal_plan.is_none());
    }
    let mut bot = Bot::new(Some("23")).unwrap();
    let turn = bot.turn("my sister is vegan");
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(turn.reply, DIET_VEGAN_MENU);
    assert!(bot.memory.meal_plan.is_none());
}

#[test]
fn replacing_a_recent_dinner_respects_the_asked_diet_and_previous_offers() {
    let mut bot = Bot::new(Some("23")).unwrap();
    bot.turn("what should i cook tonight");
    let turn = bot.turn("something vegan instead");
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(
        turn.reply,
        "I'd do a lentil dhal (30 minutes): simmer red lentils with onion, garlic and cumin in plenty of water for 25 minutes, until soft, then salt it. Something else that works is a bean stew."
    );
    let plan = bot.memory.meal_plan.as_ref().unwrap();
    assert_eq!(plan.turn, 1);
    assert_eq!(plan.diet, Some(Diet::Vegan));
    assert_eq!(
        plan.offered(),
        ["dhal", "a bean stew", "a potato curry", "vegetable soup"]
    );
}

#[test]
fn dietary_offers_cannot_use_spoiled_or_known_allergenic_food() {
    for (earlier, ask, blocked) in [
        (
            &[][..],
            "what should i cook for a vegan with a chickpea allergy",
            "chickpea",
        ),
        (
            &["i am allergic to chickpeas"][..],
            "what can i cook for a vegan",
            "chickpea",
        ),
        (
            &["my sister is allergic to chickpeas"][..],
            "my sister is vegan",
            "chickpea",
        ),
        (
            &["i am allergic to milk"][..],
            "what can i cook for a vegetarian",
            "feta",
        ),
        (
            &["i have chickpeas and spinach", "the chickpeas smell sour"][..],
            "what can i cook for a vegan",
            "chickpea",
        ),
        (
            &["i have garlic and spinach", "the garlic smells off"][..],
            "what can i cook for a vegan",
            "garlic",
        ),
        (
            &["i have chickpeas and spinach", "the spinach is mouldy"][..],
            "he is vegan",
            "spinach",
        ),
    ] {
        let mut bot = Bot::new(Some("23")).unwrap();
        for line in earlier {
            bot.turn(line);
        }
        let turn = bot.turn(ask);
        assert!(
            turn.candidates
                .iter()
                .filter(|candidate| matches!(
                    candidate.module,
                    "answers" | "chat" | "detail_question"
                ))
                .all(|candidate| !candidate.reply.to_lowercase().contains(blocked)),
            "{ask}: {turn:?}"
        );
    }
}

#[test]
fn an_unrelated_gift_does_not_become_a_dietary_menu() {
    let mut bot = Bot::new(Some("23")).unwrap();
    assert!(
        bot.turn("gift for my brother whos vegetarian")
            .candidates
            .iter()
            .all(|candidate| candidate.module != "detail_question")
    );
    assert!(bot.memory.meal_plan.is_none());
}

#[test]
fn a_vegan_plan_never_turns_into_an_unconstrained_follow_up() {
    let mut bot = Bot::new(Some("23")).unwrap();
    bot.turn("what can i cook for a vegan");
    let follow = Reading::new("another idea");
    assert!(meal_shortlist::another_idea(&follow, &bot.memory).is_none());
    let turn = bot.turn(follow.raw);
    let plan = bot.memory.meal_plan.as_ref().unwrap();
    assert_eq!((plan.turn, plan.diet), (0, Some(Diet::Vegan)));
    assert!(!turn.reply.contains("tuna"), "{turn:?}");
}
