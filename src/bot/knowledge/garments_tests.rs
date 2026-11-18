use crate::{bot::Reading, lookup_knowledge};

use super::{fabric, glasses_repair, take_coat, wardrobe};

#[test]
fn fabric_uses_the_first_named_family_in_source_order() {
    let line = "what fabric should i buy for gym shorts and sheets";
    assert!(
        fabric(&Reading::new(line))
            .expect(line)
            .starts_with("For bedding,")
    );
    let line = "which fabric is best for towels";
    let card = lookup_knowledge(line).expect(line);
    assert_eq!((card.domain, card.id), ("fabrics", "which-fabric-for"));
    assert!(fabric(&Reading::new("what fabric should i use for gymnasts")).is_none());
}

#[test]
fn a_clock_qualifies_a_coat_question_without_qualifying_purchase_or_loss() {
    assert!(take_coat(&Reading::new("should i bring a jacket at 8pm")));
    for line in [
        "do i need to buy a coat today",
        "my coat was stolen today",
        "can i wear a coat without a shirt",
    ] {
        assert!(!take_coat(&Reading::new(line)), "{line}");
    }
}

#[test]
fn wardrobe_reply_keeps_the_weight_loss_branch() {
    assert!(
        wardrobe(&Reading::new("none of my clothes fit")).starts_with("Buy the size you are now")
    );
    assert!(
        wardrobe(&Reading::new("i lost weight and none of my clothes fit"))
            .starts_with("If the weight came off without you trying, get it checked first")
    );
}

#[test]
fn loose_glue_ask_and_named_spectacles_use_distinct_replies() {
    assert!(
        glasses_repair(&Reading::new("can i glue it back on"))
            .starts_with("Superglue is the stopgap")
    );
    assert!(
        glasses_repair(&Reading::new("my glasses broke"))
            .starts_with("Take them to an optician first")
    );
    assert_ne!(
        lookup_knowledge("i broke my glass at home").map(|card| card.id),
        Some("glasses-repair")
    );
}
