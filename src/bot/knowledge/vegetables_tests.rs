use crate::{bot::Reading, lookup_knowledge};

use super::{roast, watery};

#[test]
fn roast_uses_named_method_and_longest_vegetable_name() {
    assert_eq!(
        roast("roast brussels sprouts").as_deref(),
        Some(
            "Brussels sprouts: 25 to 30 minutes in the oven at 200C (180C fan), halved and laid cut side down, in a single layer with a little oil so it browns rather than steams. It is done when the edges are browned and a knife goes into the thickest piece easily."
        )
    );
    assert_eq!(
        roast("should i steam green beans before i roast them"),
        None
    );
    assert_eq!(roast("roast carrots without garlic"), None);
    assert_eq!(roast("no corn on the cob, roast carrots"), None);
}

#[test]
fn watery_reply_prefers_the_longest_ingredient_and_respects_card_avoids() {
    let line = "my cherry tomatoes and peppers went watery";
    let reply = watery(&Reading::new(line)).expect("watery vegetable reply");
    assert!(reply.starts_with("Tomato goes watery"));
    let card = lookup_knowledge(line).expect("vegetable card");
    assert_eq!((card.domain, card.id), ("vegetables", "veg-watery"));
    for other in [
        "why did my cherry tomatoes go mouldy and watery",
        "why do frozen mushrooms go soggy",
    ] {
        assert_ne!(
            lookup_knowledge(other).map(|card| card.id),
            Some("veg-watery")
        );
    }
}

#[test]
fn vegetable_technique_defers_to_food_safety_questions() {
    for line in [
        "my peppers are watery and there is mould",
        "can i roast carrots if they smell sour",
        "can i still eat my mushy aubergine",
        "i left roasted peppers out overnight and they are soggy",
    ] {
        let card = lookup_knowledge(line);
        assert!(
            !card.is_some_and(|card| matches!(card.id, "veg-roast" | "veg-watery")),
            "{line}"
        );
    }
}

#[test]
fn roast_preserves_the_earlier_parsnip_card_on_a_source_tie() {
    for line in [
        "how long do i roast parsnips",
        "should i roast the parsnips at 200c",
    ] {
        let card = lookup_knowledge(line).expect("parsnip recipe card");
        assert_eq!((card.domain, card.id), ("cooking", "recipe-roast-parsnips"));
    }
}
