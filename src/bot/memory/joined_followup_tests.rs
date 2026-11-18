//! Conditioned source decisions from PyBot 7d63c7f.
use super::*;

const CASES: &[Case] = &[
    Case {
        name: "driving",
        ask: "how many lessons will i need",
        expected: None,
        prior: &[(
            "im learning to drive",
            "chat",
            "Worth doing. What would you like help with first?",
        )],
    },
    Case {
        name: "driving_cost",
        ask: "how much do they cost",
        expected: None,
        prior: &[(
            "How do I learn to drive?",
            "answers",
            "Most people need around 45 hours of lessons plus 20 hours of private practice before they pass, so two lessons a week gets you there in about six months at 40 just as at 18. Lessons run 25 to 40 pounds an hour, and a block with one instructor beats single lessons with three. Do the theory test early, because the pass lasts two years, and ask the price of a test-day car hire before you book the test.",
        )],
    },
    Case {
        name: "running",
        ask: "how often should i go",
        expected: None,
        prior: &[(
            "i want to start running",
            "answers",
            "Start with run-walk: one minute of easy jogging, two minutes of walking, repeated for 20 minutes, three times a week with a rest day between. Each week add a minute to the jog and take one off the walk, and in about eight weeks you will run 20 minutes without stopping. Go slowly enough to speak in short sentences - that is the pace that builds the habit.",
        )],
    },
    Case {
        name: "cake",
        ask: "how long will it take",
        expected: Some(
            "Cake: A 20cm sponge bakes for 25 to 30 minutes and a loaf tin for 45 to 55; about an hour and a half start to finish once you count mixing and cooling. 180C, or 160C in a fan oven.",
        ),
        prior: &[(
            "im making a birthday cake",
            "answers",
            "A two-layer chocolate or madeira sponge is the easy crowd-pleaser: bake it the day before, wrap it well and fill it with buttercream on the day. One 20cm tin serves 8 to 10 people generously.",
        )],
    },
    Case {
        name: "oven",
        ask: "is that safe",
        expected: None,
        prior: &[(
            "how do i clean the oven glass",
            "answers",
            "Spread a paste of bicarbonate of soda and water on the glass, leave it twenty minutes and wipe it off with a damp cloth; burnt specks lift with a plastic scraper or a razor blade held almost flat to the glass. Keep caustic oven cleaner off the rubber seal, and wipe the inside of the door too, because most of the brown film sits between the panes.",
        )],
    },
    Case {
        name: "dairy_free",
        ask: "what can i do with a courgette",
        expected: Some("Slice the courgette thick, oil and salt it, and grill 4 minutes a side."),
        prior: &[(
            "im dairy free",
            "answers",
            "Dairy free is easy to cook around: oat, soya or almond milk in place of milk, olive oil instead of butter, and a tin of coconut milk where a recipe wants cream. Watch the labels for whey, casein, milk powder and lactose, which sit in stock cubes, bread, biscuits and some crisps, and check the pesto and the chocolate too.",
        )],
    },
    Case {
        name: "flight",
        ask: "how much is a return ticket",
        expected: None,
        prior: &[(
            "im going to london for the weekend",
            "answers",
            "In London: pick one area a day rather than criss-crossing: South Bank, the museums in South Kensington, and the markets in the East End. The Tube is quickest; walking between central stops is often as fast.",
        )],
    },
    Case {
        name: "owned",
        ask: "where is my phone",
        expected: None,
        prior: &[(
            "how do i find a film to watch",
            "answers",
            "For a film: *Paddington 2* (warm, funny, and good at any age), *Knives Out* (a whodunnit everyone can follow, with jokes all the way through) or *The King's Speech* (a true story that gives everyone something to talk about afterwards). Say the mood and I'll narrow it down.",
        )],
    },
    Case {
        name: "park",
        ask: "where is the best place to park",
        expected: None,
        prior: &[(
            "how do i give a talk at work",
            "answers",
            "Write the first thirty seconds word for word and say it out loud five times; after that the nerves settle, because starting is the hard part. One point per slide, headings only, and look at one friendly face per sentence. Slow down: a pause reads as confidence and nobody notices it half as much as you do.",
        )],
    },
    Case {
        name: "fence",
        ask: "how often should i go",
        expected: None,
        prior: &[(
            "i went for a swim before work",
            "chat",
            "A swim — what was it like?",
        )],
    },
    Case {
        name: "tomato",
        ask: "Do tomatoes need blanching first?",
        expected: None,
        prior: &[(
            "Can I freeze tomatoes?",
            "answers",
            "Yes, and no blanching needed: freeze them whole on a tray, or halve them and bag them. The skins split and slip off as they thaw, so they are for sauces, soups and stews rather than salads, and they keep about 3 months.",
        )],
    },
    Case {
        name: "slugs",
        ask: "does beer work",
        expected: None,
        prior: &[(
            "i have slugs in my garden",
            "answers",
            "Go out after dark with a torch and pick them off, which beats most products; copper tape round pots and a gritty barrier round seedlings help too. Encourage birds and frogs, and water in the morning rather than the evening.",
        )],
    },
    Case {
        name: "correction",
        ask: "how much does it cost",
        expected: None,
        prior: &[(
            "i had a panic attack",
            "answers",
            "Then it is behind you, and getting through one is the thing to be pleased about rather than something to keep watch on. If one starts again, the out-breath is what helps - out for longer than in - and leaving the place it started rather than fighting it. If they come back often enough to shape your week, ask your GP about CBT: it is the treatment with the best evidence for panic.",
        )],
    },
    Case {
        name: "bot",
        ask: "who am i talking to",
        expected: None,
        prior: &[(
            "i have a work presentation next week",
            "answers",
            "Write the one sentence you want people to remember, then build three slides around it and cut the rest. Practise out loud twice, standing up, timing it, and take a one-page note instead of reading the slides; the first 30 seconds scripted and the rest in headings is what keeps a talk calm.",
        )],
    },
    Case {
        name: "long",
        ask: "how much would it cost to call a national company to come and look at it",
        expected: None,
        prior: &[(
            "my boiler keeps losing pressure",
            "answers",
            "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        )],
    },
    Case {
        name: "offering",
        ask: "what about food",
        expected: None,
        prior: &[(
            "im making a birthday cake",
            "answers",
            "A two-layer chocolate or madeira sponge is the easy crowd-pleaser: bake it the day before, wrap it well and fill it with buttercream on the day. One 20cm tin serves 8 to 10 people generously.",
        )],
    },
];

struct Case {
    name: &'static str,
    ask: &'static str,
    expected: Option<&'static str>,
    prior: &'static [(&'static str, &'static str, &'static str)],
}

#[test]
fn presentation_after_earlier_school_topic_and_first_time_follow_up() {
    let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
    for line in [
        "Will my parcel come before Friday?",
        "What time does my son finish school?",
        "How many calories in my stew?",
    ] {
        bot.turn(line);
    }
    let talk = bot.turn("How do I give a talk at work?");
    assert_eq!(talk.module, Some("chat"));
    assert_eq!(
        talk.reply,
        "Write the first thirty seconds word for word and say it out loud five times; after that the nerves settle, because starting is the hard part. One point per slide, headings only, and look at one friendly face per sentence. Slow down: a pause reads as confidence and nobody notices it half as much as you do."
    );
    let first = bot.turn("I've never done one before.");
    assert_eq!(first.module, Some("a_first_time"));
    assert_eq!(
        first.reply,
        "Nobody is calm for their first one, and the nerves go in the first two minutes once you are actually in it. Walk it through out loud once beforehand rather than reading it over again in your head. What is it you are facing?"
    );
    assert!(
        !first
            .candidates
            .iter()
            .any(|candidate| candidate.module == "chat")
    );
}

#[test]
fn finite_source_contrasts() {
    for case in CASES {
        let mut memory = Memory::default();
        for &(user, module, bot) in case.prior {
            memory.turns.push(Exchange {
                user: user.into(),
                module,
                bot: bot.into(),
            });
        }
        assert_eq!(
            reply(&Reading::new(case.ask), &memory).as_deref(),
            case.expected,
            "PyBot follow-up {:?}",
            case.name,
        );
    }
}
