//! Direct source fixtures captured from PyBot 7d63c7f.
use super::*;
use crate::bot::understanding::context::History;

#[test]
fn shortage_only_admits_an_unnamed_wedding_outfit() {
    for (line, expected) in [
        ("I have nothing to wear to a wedding.", true),
        ("I need an outfit for a wedding. Any ideas?", true),
        ("What should I wear to a wedding?", false),
        ("I have no idea what to wear to an afternoon wedding.", true),
        ("I haven't got anything for my cousin's wedding.", true),
        ("I need a suit for a wedding.", true),
        ("I have nothing in my wardrobe for a wedding.", true),
        ("I don't have an outfit for next weekend's wedding.", true),
        (
            "A hotel wedding. What about outfits, any suggestions?",
            true,
        ),
        ("Help with a wedding outfit?", true),
        ("I have nothing to wear for a black tie wedding.", false),
        ("I need something for a cocktail dress code.", false),
        ("Ideas for shoes to a wedding?", false),
        ("Which hat should I wear to a wedding?", false),
        (
            "What colour should I wear to the wedding? Any ideas?",
            false,
        ),
        ("I need a jumpsuit for this wedding.", false),
        (
            "I don't have anything to wear to a white tie wedding.",
            false,
        ),
        ("Need an outfit for a morning dress wedding.", false),
        ("Do I need a tie for this wedding?", false),
        ("No outfit yet for the wedding; tell me what to wear.", true),
        ("What should I wear? This is a smart casual wedding.", false),
        ("I haven't got nothing for a wedding.", true),
        ("What should I wear this weekend?", false),
        ("I have nothing at all to wear to the wedding.", true),
        ("Ideas for a white dress to a wedding?", true),
    ] {
        assert_eq!(outfit_shortage(&Reading::new(line)), expected, "{line:?}");
    }
}

type Case = (
    Reply,
    &'static [&'static str],
    &'static str,
    Option<&'static str>,
);

#[test]
fn clothing_wear_callbacks_match_the_reference() {
    let cases: &[Case] = &[
        (
            Reply::Socks,
            &[],
            "What socks should I wear for a marathon?",
            Some(SPORT),
        ),
        (
            Reply::Socks,
            &[],
            "Which socks are best for mountain hiking?",
            Some(SPORT),
        ),
        (
            Reply::Socks,
            &[],
            "What socks should I wear with a suit?",
            Some(FORMAL),
        ),
        (
            Reply::Socks,
            &[],
            "What socks for an interview?",
            Some(FORMAL),
        ),
        (
            Reply::Socks,
            &[],
            "Can I wear black socks with brown shoes?",
            Some(BLACK_BROWN),
        ),
        (
            Reply::Socks,
            &[],
            "Can I wear BROWN shoes with BLACK socks?",
            Some(BLACK_BROWN),
        ),
        (
            Reply::Socks,
            &[],
            "Which socks with brown shoes and a black suit?",
            Some(BLACK_BROWN),
        ),
        (
            Reply::Socks,
            &[],
            "Are white socks fine with trainers?",
            Some(WHITE),
        ),
        (
            Reply::Socks,
            &[],
            "Can I wear white socks with smart trousers?",
            Some(FORMAL),
        ),
        (
            Reply::Socks,
            &[],
            "Which socks should I wear with jeans?",
            Some(CASUAL),
        ),
        (
            Reply::Socks,
            &[],
            "What socks work with navy shoes?",
            Some(FORMAL),
        ),
        (Reply::Socks, &[], "What socks do I need?", Some(GENERAL)),
        (
            Reply::Socks,
            &[],
            "Which socks for a wedding and gym?",
            Some(SPORT),
        ),
        (
            Reply::Socks,
            &[],
            "What colour socks go with dress boots?",
            Some(FORMAL),
        ),
        (
            Reply::Socks,
            &[],
            "Which socks are best for my football kit?",
            Some(SPORT),
        ),
        (
            Reply::Lace,
            &[],
            "How do I tie my shoelaces?",
            Some(LACE_KNOT),
        ),
        (
            Reply::Lace,
            &[],
            "How should I tie a bow on a present?",
            Some(LACE_RIBBON),
        ),
        (
            Reply::Lace,
            &[],
            "How do I teach a child to tie a bow?",
            Some(LACE_TEACHING),
        ),
        (
            Reply::Lace,
            &[],
            "How do I teach a child to tie ice skates?",
            Some(LACE_SKATES),
        ),
        (
            Reply::Lace,
            &[],
            "How do I lace up skates?",
            Some(LACE_SKATES),
        ),
        (
            Reply::Lace,
            &[],
            "My shoelaces keep coming undone",
            Some(LACE_UNDONE),
        ),
        (
            Reply::Lace,
            &[],
            "How do I double knot my laces?",
            Some(LACE_KNOT),
        ),
        (
            Reply::Lace,
            &[],
            "How should I tie a ribbon bow for a gift?",
            Some(LACE_RIBBON),
        ),
        (
            Reply::Lace,
            &[],
            "How do I keep the lace ends from trailing?",
            Some(LACE_UNDONE),
        ),
        (
            Reply::Lace,
            &[],
            "My daughter is learning to tie her shoes",
            Some(LACE_TEACHING),
        ),
        (
            Reply::Lace,
            &[],
            "Laces in my ice skates always come undone",
            Some(LACE_SKATES),
        ),
        (
            Reply::Lace,
            &[],
            "How do I tie lace on a present with skates in it?",
            Some(LACE_RIBBON),
        ),
        (
            Reply::Size,
            &["I'm a size 14 in the UK"],
            "What is that in Europe?",
            Some(EU_14),
        ),
        (
            Reply::Size,
            &["I'm a size 12 in the UK"],
            "What is that in the US?",
            Some(US_12),
        ),
        (
            Reply::Size,
            &["My size 10 skirt no longer fits so I'm size 16."],
            "What is that in America?",
            Some(US_16),
        ),
        (
            Reply::Size,
            &["UK 6 is too small."],
            "What is that in the EU?",
            Some(EU_6),
        ),
        (
            Reply::Size,
            &["UK 8 is too small."],
            "What is that in American sizes?",
            Some(US_8),
        ),
        (
            Reply::Size,
            &["I'm a size 18."],
            "What is that in Europe?",
            Some(EU_18),
        ),
        (
            Reply::Size,
            &["I'm a size 20."],
            "What is that in US sizes?",
            Some(US_20),
        ),
        (
            Reply::Size,
            &["My shoe size is UK 8."],
            "What is that in Europe?",
            None,
        ),
        (
            Reply::Size,
            &["I'm a size 14.", "I also need a pair of trainers."],
            "What size is that in the US?",
            None,
        ),
        (
            Reply::Size,
            &["I'm a size 13."],
            "What is that in Europe?",
            None,
        ),
        (
            Reply::Size,
            &["I wear a size 22."],
            "What is that in the US?",
            None,
        ),
        (
            Reply::Size,
            &[
                "I'm a size 14.",
                "Hello.",
                "That's fine.",
                "Here's another thing.",
            ],
            "What is that in Europe?",
            None,
        ),
        (
            Reply::Size,
            &["I'm a size 14.", "Thanks.", "Hello."],
            "What is that in Europe?",
            Some(EU_14),
        ),
        (
            Reply::Size,
            &["I've just bought a top."],
            "What is that in Europe?",
            None,
        ),
        (Reply::Size, &[], "What is that in Europe?", None),
        (
            Reply::Size,
            &["I'm an EU 42."],
            "What is that in the US?",
            None,
        ),
        (
            Reply::Size,
            &[
                "I first bought shoes.",
                "My UK 18 dress looks good.",
                "Right.",
                "Fine.",
            ],
            "What is that in the US?",
            Some(US_18),
        ),
    ];
    for &(kind, before, line, expected) in cases {
        let mut runtime_history = History::default();
        let mut prior: Vec<Exchange> = Vec::new();
        for user in before {
            let mut reading = Reading::in_context(
                user,
                prior.iter().map(|turn| turn.user.as_str()),
                None,
                &runtime_history,
            );
            runtime_history.retain(&mut reading);
            prior.push(Exchange {
                user: (*user).into(),
                bot: "All right.".into(),
                module: "greeting",
            });
        }
        let reading = Reading::in_context(
            line,
            prior.iter().map(|turn| turn.user.as_str()),
            None,
            &runtime_history,
        );
        let actual = reply(kind, &reading, &prior);
        assert_eq!(
            actual.as_deref(),
            expected,
            "{kind:?} for {line:?} after {before:?}"
        );
    }
}

#[test]
fn enough_guard_matches_the_reference() {
    for (line, expected) in [
        ("Is it cold enough for a coat?", true),
        ("Is it warm enough for shorts?", true),
        ("Is it wet enough for wellies?", true),
        ("Is it hot enough for a fan?", false),
        ("Is it windy enough for a jacket?", true),
        ("Is the coat warm enough?", false),
        ("Is it cold enough for my thick coat?", true),
        ("Is it mild enough for a t-shirt?", false),
        ("Is it chilly enough for her coat?", false),
        ("Is it sunny enough for a sun hat?", true),
        ("Is it windy enough for the gloves?", true),
    ] {
        let read = Reading::new(line);
        assert_eq!(enough_weather(&read), expected, "{line:?}");
    }
}

#[test]
fn today_guard_matches_the_reference() {
    for (line, expected) in [
        ("What should I wear today?", true),
        ("Do I need a coat today?", true),
        ("What should I put on today?", true),
        ("Can I put on the kettle today?", false),
        ("Can I put on some music today?", false),
        ("I am putting on a coat", true),
        ("I'm putting on both the music and my jacket", false),
        ("I want to put on weight today", true),
    ] {
        let read = Reading::new(line);
        assert_eq!(
            super::super::guards::put_on_is_clothes(&read.line.words),
            expected,
            "{line:?}"
        );
    }
}

const SPORT: &str = "For running, walking or the gym, wear a thin synthetic or merino-wool sock with a flat seam across the toe, in the same shoe you train in: cotton holds sweat against the skin and that is what blisters come from. Two thin pairs are warmer and rub less than one thick pair, and the sock should be tight enough not to bunch under the arch.";

const FORMAL: &str = "Dark socks that match the trousers rather than the shoes, long enough to cover the calf so no bare leg shows when you sit down: navy, charcoal or black in wool or a wool blend, plain unless the suit is patterned. Black socks with brown shoes is the one pairing that looks wrong, so match the trousers and let the shoes be their own colour.";

const BLACK_BROWN: &str = "Black socks with brown shoes is the pairing to avoid: the sock should match the trousers, so with brown shoes wear a dark brown, navy or charcoal sock, or keep the shoes for a lighter suit. Nobody notices a sock that matches the trousers and everybody notices one that fights the shoes.";

const WHITE: &str = "White socks are fine with trainers, shorts and gym kit, and wrong with a suit or smart trousers: with those, wear a dark sock that matches the trousers, long enough that the calf stays covered when you sit down.";

const CASUAL: &str = "Go by the trousers rather than the shoes: a plain sock a shade darker than the trousers is the safe pairing, which means dark grey or navy with jeans and charcoal or black with smart trousers. Cotton for everyday, wool or a wool blend when the foot will be in a shoe all day, because cotton stays wet against the skin.";

const GENERAL: &str = "Match the trousers rather than the shoes, dark navy, charcoal or black, and long enough that no bare leg shows when you sit down. Cotton for everyday and wool or a wool blend for a shoe you are in all day, and thin wool rather than thick cotton with anything smart, because a thick sock makes a well-fitted shoe tight.";

const LACE_KNOT: &str = "Make two loops and tie them like a bow: cross the laces once, take the top one under and through the gap so the base knot is snug, then bend each lace into a loop, cross the two loops over, and push one loop down through the hole beneath them. Pull the loops outward rather than up, and it sits flat. Two things make the rest easy: lace from the bottom up so every row lies flat, and leave about 10cm of lace after the bow - a long tail is what drags and comes undone.";

const LACE_RIBBON: &str = "Use a ribbon rather than string, because it holds its shape: make a loop in each hand, cross the right loop over the left, tuck it behind and bring it up through the gap underneath, then pull both loops outward until the bow is the size you want. Tape the ribbon to the box underneath first if the present is heavy, so it cannot slide, and curl the tails over a scissor blade and cut them on the angle.";

const LACE_TEACHING: &str = "About five or six, and it comes quicker if the lace is the practice rather than the shoe: tie a lace round a table leg, a ribbon round a chair back, or one big shoe on the floor, and teach the bunny-ears version first - two loops, cross, push one through. Say one step at a time and let them repeat it five times before adding the next, and be slow in front of them with your own laces. Fifteen minutes on a Saturday works better than twenty rushed tries on the way to school, and being allowed to be bad at it for a few weeks is part of learning it.";

const LACE_SKATES: &str = "Skates want a firmer lace than shoes and nothing left trailing: lace from the bottom, keep the foot snug through the middle, then pull the top two tight at the ankle so the heel cannot lift, double knot it, and tuck the ends inside the top of the boot. Do them up with thin socks or warm feet, because tight laces on cold feet is how toes go numb, and loosen the top two before you take them off.";

const LACE_UNDONE: &str = "It is the knot rather than your tying: a bow on a smooth lace works loose. Tie the first knot twice - a double knot - for round, waxed or nylon laces, or switch to flat laces with a bit of texture, and check you have about 10cm of tail after the bow because a long end swings and pulls itself free. Running for a bus or a bike ride: tuck the ends and loops under the criss-cross above them and they will stay put all day.";

const EU_14: &str = "A UK 14 is a European 42 in women's clothes, and a US 10. Men's clothes go by chest size instead: 36 in chest is EU 46, 38 is 48, 40 is 50, 42 is 52. Sizes drift a whole step between brands, so check the brand's own size guide as well.";

const US_12: &str = "A UK 12 is a US 8 - that is the women's conversion, and men's clothes go by chest size instead. A size is only a starting point either way: it drifts a whole step between brands, so go by the brand's own size guide and your measurements rather than the label.";

const US_16: &str = "A UK 16 is a US 12 - that is the women's conversion, and men's clothes go by chest size instead. A size is only a starting point either way: it drifts a whole step between brands, so go by the brand's own size guide and your measurements rather than the label.";

const EU_6: &str = "A UK 6 is a European 34 in women's clothes, and a US 2. Men's clothes go by chest size instead: 36 in chest is EU 46, 38 is 48, 40 is 50, 42 is 52. Sizes drift a whole step between brands, so check the brand's own size guide as well.";

const US_8: &str = "A UK 8 is a US 4 - that is the women's conversion, and men's clothes go by chest size instead. A size is only a starting point either way: it drifts a whole step between brands, so go by the brand's own size guide and your measurements rather than the label.";

const EU_18: &str = "A UK 18 is a European 46 in women's clothes, and a US 14. Men's clothes go by chest size instead: 36 in chest is EU 46, 38 is 48, 40 is 50, 42 is 52. Sizes drift a whole step between brands, so check the brand's own size guide as well.";

const US_20: &str = "A UK 20 is a US 16 - that is the women's conversion, and men's clothes go by chest size instead. A size is only a starting point either way: it drifts a whole step between brands, so go by the brand's own size guide and your measurements rather than the label.";

const US_18: &str = "A UK 18 is a US 14 - that is the women's conversion, and men's clothes go by chest size instead. A size is only a starting point either way: it drifts a whole step between brands, so go by the brand's own size guide and your measurements rather than the label.";
