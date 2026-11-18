use super::*;
use crate::bot::Bot;

#[test]
fn timed_requests_publish_the_expected_pair_and_keep_the_clock_and_offers() {
    let prompt = Reading::new("what can i cook in 15 minutes");
    let memory = Memory::default();
    let (offer, card) = propose(&prompt, &memory).unwrap();
    assert_eq!(card, ("cooking", "meal-ideas"));
    assert_eq!(offer.plan.minutes, Some(15));
    assert_eq!(
        offer.plan.offered(),
        [
            "pesto pasta",
            "mushrooms on toast",
            "a tuna salad",
            "a chickpea salad"
        ],
    );
    assert_eq!(
        offer.reply,
        "I'd do pesto pasta (12 minutes): stir pesto through hot pasta with a spoonful of the cooking water to loosen it. Something else that works is mushrooms on toast."
    );
    let mut bot = Bot::new(Some("23")).unwrap();
    let turn = bot.turn(prompt.raw);
    assert_eq!((turn.module, turn.reply), (Some("answers"), offer.reply));
    assert_eq!(bot.memory.meal_plan, Some(offer.plan));
}

#[test]
fn faster_follow_up_reads_the_previous_pick_and_holds_the_latest_source_state() {
    let mut bot = Bot::new(Some("23")).unwrap();
    bot.turn("what should i cook tonight");
    let turn = bot.turn("quicker please");
    assert_eq!(turn.module, Some("answers"));
    assert!(turn.reply.starts_with("Try beans on toast: Warm the beans"));
    let plan = bot.memory.meal_plan.as_ref().unwrap();
    assert_eq!(plan.minutes, Some(7));
    assert_eq!(
        plan.offered(),
        ["a cheese toastie", "beans on toast", "dhal", "a bean stew"]
    );
}

#[test]
fn approved_methods_keep_the_source_dish_and_timing_as_an_estimate() {
    for (ask, dish, pieces) in [
        (
            "something on the table in 15 minutes",
            "a prawn stir-fry",
            &["firm and opaque"][..],
        ),
        (
            "i need something on the table in 15 minutes",
            "egg fried rice",
            &[
                "ideally within 1 hour",
                "no more than 24 hours",
                "Reheat it only once",
                "steaming hot throughout",
            ][..],
        ),
    ] {
        let mut bot = Bot::new(Some("23")).unwrap();
        let turn = bot.turn(ask);
        assert_eq!(turn.module, Some("answers"));
        assert!(turn.reply.contains(&format!("{dish} (about 15 minutes)")));
        assert!(
            pieces.iter().all(|piece| turn.reply.contains(piece)),
            "{turn:?}"
        );
    }
    let chicken = DISHES
        .iter()
        .find(|dish| dish.name == "a chicken stir-fry")
        .unwrap();
    assert!(
        approved_method(chicken)
            .unwrap()
            .contains("at or above 70°C for 2 minutes")
    );
}

#[test]
fn a_clock_leaves_technical_questions_and_sessions_with_food_limits_to_existing_routes() {
    let default = Memory::default();
    for ask in [
        "the meeting is only ten minutes",
        "how long do i boil eggs for 15 minutes",
        "dinner in 15 minutes",
        "what can i make in 5 minutes",
    ] {
        assert!(propose(&Reading::new(ask), &default).is_none(), "{ask}");
    }
    let mut bot = Bot::new(Some("23")).unwrap();
    bot.turn("what should i cook tonight");
    bot.turn("i am allergic to fish");
    assert!(propose(&Reading::new("quick 15 minutes please"), &bot.memory).is_none());
}
