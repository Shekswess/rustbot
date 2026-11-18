use crate::bot::{Bot, Exchange, Reading};

use super::{Context, reply};

#[test]
fn rice_questions_keep_source_matcher_order() {
    for (line, domain, id) in [
        ("what rice do i need for sushi", "picking", "pick-rice"),
        ("which rice should i use for sushi", "picking", "pick-rice"),
        ("what rice for biryani", "picking", "pick-rice"),
        ("which rice for sushi", "rice", "rice-for-dish"),
        ("best rice for paella", "rice", "rice-for-dish"),
        (
            "what rice do i need for making sticky rice",
            "rice",
            "rice-for-dish",
        ),
        ("can i use jasmine instead of basmati", "rice", "rice-swap"),
        ("how much rice for a paella", "cooking", "rice-portion"),
        ("can i use rice instead of pasta", "swaps", "swap-pasta"),
        ("which onion for soup", "picking", "pick-onion"),
        ("what kind of apples for crumble", "picking", "pick-apple"),
    ] {
        let card = crate::lookup_knowledge(line).expect(line);
        assert_eq!((card.domain, card.id), (domain, id), "{line}");
    }
}

#[test]
fn rice_reply_uses_only_the_four_most_recent_user_lines() {
    let reading = Reading::new("can i use basmati");
    let earlier = ["im making a paella", "one", "two", "three", "four"].map(|user| Exchange {
        user: user.into(),
        bot: String::new(),
        module: "",
    });
    assert_eq!(
        reply(&Context::new(&reading, &earlier[..4])),
        "You can use basmati in paella, but bomba rice is the one for it: 1 part rice to 2.5 or 3 parts stock, 18 to 20 minutes. Bomba rice is a short grain that swells without bursting, so it stays separate, where basmati is a long grain that cooks up dry and separate, which is why it suits curry, so the dish comes out with that grain in it."
    );
    assert_eq!(
        reply(&Context::new(&reading, &earlier)),
        "Basmati works in place of white rice: 1 part rice to 1.5 parts water, 10 to 12 minutes. It is a long grain that cooks up dry and separate, which is why it suits curry."
    );
}

#[test]
fn common_sushi_prompt_preserves_selected_reply() {
    let turn = Bot::new(Some("0"))
        .unwrap()
        .turn("what rice do i need for sushi");
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(
        turn.reply,
        "Short-grain for sushi - sold as sushi rice, and koshihikari if you want the best: it is sticky enough to hold the roll together where long-grain falls apart. Rinse it until the water runs clear and use a little less water than the packet says."
    );
}
