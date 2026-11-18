//! Direct reply-framing fixtures from PyBot 7d63c7f memory.again.instead.
use super::{frame_answer_card, instead};
use crate::bot::{Exchange, Memory};

struct Case {
    name: &'static str,
    reply: &'static str,
    text: &'static str,
    turns: &'static [(&'static str, &'static str)],
    wants: Option<&'static str>,
    expected: Option<&'static str>,
}

const CASES: &[Case] = &[
    Case {
        name: r###"new"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"how do i cook rice"###,
        turns: &[],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"short fact"###,
        reply: r###"You live in York."###,
        text: r###"where do i live"###,
        turns: &[(r###"where do i live"###, r###"You live in York."###)],
        wants: None,
        expected: Some(r###"Same as before: You live in York."###),
    },
    Case {
        name: r###"second short frame"###,
        reply: r###"You live in York."###,
        text: r###"where do i live"###,
        turns: &[
            (r###"where do i live"###, r###"You live in York."###),
            (
                r###"where do i live"###,
                r###"Same as before: You live in York."###,
            ),
        ],
        wants: None,
        expected: Some(r###"As I said: You live in York."###),
    },
    Case {
        name: r###"already framed short"###,
        reply: r###"Same as before: You live in York."###,
        text: r###"where do i live"###,
        turns: &[(
            r###"where do i live"###,
            r###"Same as before: You live in York."###,
        )],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"recall wording"###,
        reply: r###"You said: You live in York."###,
        text: r###"what did i say"###,
        turns: &[(
            r###"what did i say"###,
            r###"You said: You live in York."###,
        )],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"short fact followed by bot question"###,
        reply: r###"You live in York."###,
        text: r###"where do i live"###,
        turns: &[(
            r###"i live in york"###,
            r###"You live in York. How is it there?"###,
        )],
        wants: None,
        expected: Some(r###"Same as before: You live in York."###),
    },
    Case {
        name: r###"short recurrence news"###,
        reply: r###"The tap is still leaking."###,
        text: r###"the tap started leaking again this morning"###,
        turns: &[(
            r###"what about the tap"###,
            r###"The tap is still leaking."###,
        )],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"long first sentence"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"how do i cook rice"###,
        turns: &[(
            r###"how do i cook rice"###,
            r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        )],
        wants: None,
        expected: Some(
            r###"Same answer, and it covers this: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
        ),
    },
    Case {
        name: r###"long second frame"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"how do i cook rice"###,
        turns: &[
            (
                r###"how do i cook rice"###,
                r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
            ),
            (
                r###"how do i cook rice"###,
                r###"Same answer, and it covers this: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
            ),
        ],
        wants: None,
        expected: Some(
            r###"That is still the thing to do: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        ),
    },
    Case {
        name: r###"colon separator"###,
        reply: r###"Here is the method: Make one mug of rice, and rinse it until the water runs completely clear before cooking. Set the pan over a low heat once it has boiled."###,
        text: r###"how do i cook rice"###,
        turns: &[(
            r###"how do i cook rice"###,
            r###"Here is the method: Make one mug of rice, and rinse it until the water runs completely clear before cooking. Set the pan over a low heat once it has boiled."###,
        )],
        wants: None,
        expected: Some(
            r###"Same answer, and it covers this — Here is the method: Make one mug of rice, and rinse it until the water runs completely clear before cooking."###,
        ),
    },
    Case {
        name: r###"consecutive shortened body"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"how do i cook rice"###,
        turns: &[
            (
                r###"how do i cook rice"###,
                r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
            ),
            (
                r###"how do i cook rice"###,
                r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
            ),
        ],
        wants: None,
        expected: Some(
            r###"Same answer, and it covers this: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        ),
    },
    Case {
        name: r###"new timing ask"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"how long should i cook rice for"###,
        turns: &[(
            r###"how do i cook rice"###,
            r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        )],
        wants: None,
        expected: Some(
            r###"Same answer, and it covers this: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        ),
    },
    Case {
        name: r###"repeat timing ask"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"how long should i cook rice for"###,
        turns: &[(
            r###"how long should i cook rice for"###,
            r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        )],
        wants: None,
        expected: Some(
            r###"Same answer, and it covers this: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
        ),
    },
    Case {
        name: r###"wording requires script"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"what do i say"###,
        turns: &[(
            r###"how do i cook rice"###,
            r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        )],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"wording script"###,
        reply: r###"Say this to the person in charge: "Hi, I reported the issue last week. Could you tell me when someone can have a look?" Send it today so a reply can reach you before Friday."###,
        text: r###"what do i say"###,
        turns: &[(
            r###"how do i ask the manager"###,
            r###"Say this to the person in charge: "Hi, I reported the issue last week. Could you tell me when someone can have a look?" Send it today so a reply can reach you before Friday."###,
        )],
        wants: None,
        expected: Some(
            r###"Same answer, and it covers this — Say this to the person in charge: "Hi, I reported the issue last week. Could you tell me when someone can have a look?" Send it today so a reply can reach you before Friday."###,
        ),
    },
    Case {
        name: r###"unsupported amount"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"how much does it cost"###,
        turns: &[(
            r###"how do i cook rice"###,
            r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        )],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"step request"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"what should i do first"###,
        turns: &[(
            r###"how do i cook rice"###,
            r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        )],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"new report"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"i moved to bath yesterday"###,
        turns: &[(
            r###"how do i cook rice"###,
            r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        )],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"unrelated recent repeat"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"how do i cook rice"###,
        turns: &[
            (
                r###"how do i cook rice"###,
                r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
            ),
            (r###"What is a daisy?"###, r###"A flower."###),
            (r###"How is the weather?"###, r###"Rain."###),
        ],
        wants: None,
        expected: Some(
            r###"Same answer, and it covers this: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
        ),
    },
    Case {
        name: r###"earlier same question"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"so how do i cook rice then"###,
        turns: &[
            (
                r###"how do i cook rice"###,
                r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
            ),
            (r###"What is a daisy?"###, r###"A flower."###),
            (r###"How is the weather?"###, r###"Rain."###),
            (r###"How are you?"###, r###"I'm fine."###),
        ],
        wants: None,
        expected: Some(
            r###"Back to cook rice: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
        ),
    },
    Case {
        name: r###"earlier unrelated"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"will it be windy"###,
        turns: &[
            (
                r###"how do i cook rice"###,
                r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
            ),
            (r###"What is a daisy?"###, r###"A flower."###),
            (r###"How is the weather?"###, r###"Rain."###),
            (r###"How are you?"###, r###"I'm fine."###),
        ],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"return full named"###,
        reply: r###"In Rome: Start with the Colosseum before lunch and use a timed ticket to avoid the big queue. Book the tower for the evening if you want to see the view."###,
        text: r###"back to rome what should we see"###,
        turns: &[
            (
                r###"what should i see in rome"###,
                r###"In Rome: Start with the Colosseum before lunch and use a timed ticket to avoid the big queue. Book the tower for the evening if you want to see the view."###,
            ),
            (r###"What is a daisy?"###, r###"A flower."###),
            (r###"How is the weather?"###, r###"Rain."###),
            (r###"How are you?"###, r###"I'm fine."###),
        ],
        wants: None,
        expected: Some(
            r###"Back to Rome: Start with the Colosseum before lunch and use a timed ticket to avoid the big queue. Book the tower for the evening if you want to see the view."###,
        ),
    },
    Case {
        name: r###"return with new detail"###,
        reply: r###"In Rome: Start with the Colosseum before lunch and use a timed ticket to avoid the big queue. Book the tower for the evening if you want to see the view."###,
        text: r###"back to rome what about the tower"###,
        turns: &[
            (
                r###"what should i see in rome"###,
                r###"In Rome: Start with the Colosseum before lunch and use a timed ticket to avoid the big queue. Book the tower for the evening if you want to see the view."###,
            ),
            (r###"What is a daisy?"###, r###"A flower."###),
            (r###"How is the weather?"###, r###"Rain."###),
            (r###"How are you?"###, r###"I'm fine."###),
        ],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"alternate refusal"###,
        reply: r###"I'm not sure about that one, sorry — I'd rather not guess. A quick search online will get you a solid answer, and I can help you make sense of what you find."###,
        text: r###"do you know that"###,
        turns: &[(
            r###"do you know that"###,
            r###"I'm not sure about that one, sorry — I'd rather not guess. A quick search online will get you a solid answer, and I can help you make sense of what you find."###,
        )],
        wants: None,
        expected: Some(
            r###"I don't know enough to answer that honestly. A search should sort you out quickly — and I'm happy to help with the rest of it."###,
        ),
    },
    Case {
        name: r###"keep cause"###,
        reply: r###"Clean the filter to make sure the water can run freely before you switch the machine back on. The cause is a clog in the filter stopping the water flowing back out."###,
        text: r###"why does the water not drain"###,
        turns: &[(
            r###"what should i do"###,
            r###"Clean the filter to make sure the water can run freely before you switch the machine back on. The cause is a clog in the filter stopping the water flowing back out."###,
        )],
        wants: Some(r###"reason"###),
        expected: Some(
            r###"Same answer, and it covers this: The cause is a clog in the filter stopping the water flowing back out. Clean the filter to make sure the water can run freely before you switch the machine back on."###,
        ),
    },
    Case {
        name: r###"third person reflection"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"i feel empty still"###,
        turns: &[
            (
                r###"how do i cook rice"###,
                r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
            ),
            (
                r###"how do i cook rice"###,
                r###"Same answer, and it covers this: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
            ),
        ],
        wants: None,
        expected: None,
    },
    Case {
        name: r###"third felt reflection"###,
        reply: r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
        text: r###"i feel still empty"###,
        turns: &[
            (
                r###"how do i cook rice"###,
                r###"Rinse one mug of rice until the water runs clear, then add one and a half mugs of water. Simmer over a low heat for twelve minutes and rest for five."###,
            ),
            (
                r###"how do i cook rice"###,
                r###"Same answer, and it covers this: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
            ),
        ],
        wants: None,
        expected: Some(
            r###"Feeling still empty is the same thing underneath, and the step is the same: Rinse one mug of rice until the water runs clear, then add one and a half mugs of water."###,
        ),
    },
];

#[test]
fn frames_match_the_pinned_engine() {
    for case in CASES {
        let memory = test_memory(case);
        assert_eq!(
            instead(case.reply, case.text, &memory).as_deref(),
            case.expected,
            "{}",
            case.name
        );
    }
}

#[test]
fn a_shortened_card_is_still_a_reply_on_the_earlier_question() {
    let case = CARD_CASES
        .iter()
        .find(|case| case.name == "tomatoes repeat 1")
        .unwrap();
    let memory = test_memory(case);
    let sentence = "Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease.";
    let expected = "Back to tomato plant leaves going yellow then: Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease.";
    assert_eq!(
        instead(sentence, case.text, &memory).as_deref(),
        Some(expected)
    );
}

#[test]
fn card_repeats_match_the_pinned_engine() {
    for case in CARD_CASES {
        let memory = test_memory(case);
        assert_eq!(
            frame_answer_card(case.reply, case.text, &memory)
                .or_else(|| instead(case.reply, case.text, &memory))
                .as_deref(),
            case.expected,
            "{}",
            case.name
        );
    }
}

fn test_memory(case: &Case) -> Memory {
    Memory {
        turns: case
            .turns
            .iter()
            .map(|(user, bot)| Exchange {
                user: (*user).to_owned(),
                bot: (*bot).to_owned(),
                module: "answers",
            })
            .collect(),
        intent_wants: case.wants,
        ..Memory::default()
    }
}

// Repeated questions seen through the answers card lane before the final wrapper.
const CARD_CASES: &[Case] = &[
    Case {
        name: "car repeat 1",
        reply: r###"Listen to what it does: a click or nothing at all is the battery or its connections, a slow groaning crank is a flat battery, and a fast crank that won't fire is fuel or ignition. Try a jump start first; if that works, the battery or alternator is the problem, not the engine."###,
        text: r###"Why will my car not start?"###,
        turns: &[(
            r###"Why will my car not start?"###,
            r###"Listen to what it does: a click or nothing at all is the battery or its connections, a slow groaning crank is a flat battery, and a fast crank that won't fire is fuel or ignition. Try a jump start first; if that works, the battery or alternator is the problem, not the engine."###,
        )],
        wants: None,
        expected: Some(
            r###"Same as before: Try a jump start first; if that works, the battery or alternator is the problem, not the engine."###,
        ),
    },
    Case {
        name: "car repeat 2",
        reply: r###"Listen to what it does: a click or nothing at all is the battery or its connections, a slow groaning crank is a flat battery, and a fast crank that won't fire is fuel or ignition. Try a jump start first; if that works, the battery or alternator is the problem, not the engine."###,
        text: r###"Why will my car not start?"###,
        turns: &[
            (
                r###"Why will my car not start?"###,
                r###"Listen to what it does: a click or nothing at all is the battery or its connections, a slow groaning crank is a flat battery, and a fast crank that won't fire is fuel or ignition. Try a jump start first; if that works, the battery or alternator is the problem, not the engine."###,
            ),
            (
                r###"Why will my car not start?"###,
                r###"Same as before: Try a jump start first; if that works, the battery or alternator is the problem, not the engine."###,
            ),
        ],
        wants: None,
        expected: Some(
            r###"As I said: Try a jump start first; if that works, the battery or alternator is the problem, not the engine."###,
        ),
    },
    Case {
        name: "condensation repeat 1",
        reply: r###"Condensation comes from warm wet air hitting cold surfaces, so ventilate and keep the place a bit warmer: use the extractor fan, open a window for 10 minutes a day, dry clothes outside or in a vented dryer, and leave the heating on low rather than off and on. A dehumidifier helps in a bedroom that gets it badly."###,
        text: r###"How do I stop condensation on windows?"###,
        turns: &[(
            r###"How do I stop condensation on windows?"###,
            r###"Condensation comes from warm wet air hitting cold surfaces, so ventilate and keep the place a bit warmer: use the extractor fan, open a window for 10 minutes a day, dry clothes outside or in a vented dryer, and leave the heating on low rather than off and on. A dehumidifier helps in a bedroom that gets it badly."###,
        )],
        wants: None,
        expected: Some(
            r###"Same as before: Condensation comes from warm wet air hitting cold surfaces, so ventilate and keep the place a bit warmer: use the extractor fan, open a window for 10 minutes a day, dry clothes outside or in a vented dryer, and leave the heating on low rather than off and on."###,
        ),
    },
    Case {
        name: "condensation repeat 2",
        reply: r###"Condensation comes from warm wet air hitting cold surfaces, so ventilate and keep the place a bit warmer: use the extractor fan, open a window for 10 minutes a day, dry clothes outside or in a vented dryer, and leave the heating on low rather than off and on. A dehumidifier helps in a bedroom that gets it badly."###,
        text: r###"How do I stop condensation on windows?"###,
        turns: &[
            (
                r###"How do I stop condensation on windows?"###,
                r###"Condensation comes from warm wet air hitting cold surfaces, so ventilate and keep the place a bit warmer: use the extractor fan, open a window for 10 minutes a day, dry clothes outside or in a vented dryer, and leave the heating on low rather than off and on. A dehumidifier helps in a bedroom that gets it badly."###,
            ),
            (
                r###"How do I stop condensation on windows?"###,
                r###"Same as before: Condensation comes from warm wet air hitting cold surfaces, so ventilate and keep the place a bit warmer: use the extractor fan, open a window for 10 minutes a day, dry clothes outside or in a vented dryer, and leave the heating on low rather than off and on."###,
            ),
        ],
        wants: None,
        expected: Some(
            r###"As I said: Condensation comes from warm wet air hitting cold surfaces, so ventilate and keep the place a bit warmer: use the extractor fan, open a window for 10 minutes a day, dry clothes outside or in a vented dryer, and leave the heating on low rather than off and on."###,
        ),
    },
    Case {
        name: "roast repeat 1",
        reply: r###"About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes. Check the thigh reads 74C on a thermometer and the juices run clear, then rest it for 15 minutes before carving."###,
        text: r###"How do I roast a chicken?"###,
        turns: &[(
            r###"How do I roast a chicken?"###,
            r###"About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes. Check the thigh reads 74C on a thermometer and the juices run clear, then rest it for 15 minutes before carving."###,
        )],
        wants: None,
        expected: Some(
            r###"Same as before: About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes."###,
        ),
    },
    Case {
        name: "roast repeat 2",
        reply: r###"About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes. Check the thigh reads 74C on a thermometer and the juices run clear, then rest it for 15 minutes before carving."###,
        text: r###"How do I roast a chicken?"###,
        turns: &[
            (
                r###"How do I roast a chicken?"###,
                r###"About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes. Check the thigh reads 74C on a thermometer and the juices run clear, then rest it for 15 minutes before carving."###,
            ),
            (
                r###"How do I roast a chicken?"###,
                r###"Same as before: About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes."###,
            ),
        ],
        wants: None,
        expected: Some(
            r###"That is still the thing to do: About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes. Check the thigh reads 74C on a thermometer and the juices run clear, then rest it for 15 minutes before carving."###,
        ),
    },
    Case {
        name: "tomatoes repeat 1",
        reply: r###"Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease. If the yellow sits between green veins, that is magnesium: water evenly, mulch, and give a tomato feed with magnesium in it. If the lowest leaves go pale and drop off, the plant is short of nitrogen or the roots are waterlogged. Pick the yellow leaves off, feed weekly, and water little and often rather than in floods; brown blotches or black bottoms are a different problem."###,
        text: r###"Why are my tomato plant leaves going yellow?"###,
        turns: &[(
            r###"Why are my tomato plant leaves going yellow?"###,
            r###"Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease. If the yellow sits between green veins, that is magnesium: water evenly, mulch, and give a tomato feed with magnesium in it. If the lowest leaves go pale and drop off, the plant is short of nitrogen or the roots are waterlogged. Pick the yellow leaves off, feed weekly, and water little and often rather than in floods; brown blotches or black bottoms are a different problem."###,
        )],
        wants: None,
        expected: Some(
            r###"Same as before: Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease."###,
        ),
    },
    Case {
        name: "tomatoes repeat 2",
        reply: r###"Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease. If the yellow sits between green veins, that is magnesium: water evenly, mulch, and give a tomato feed with magnesium in it. If the lowest leaves go pale and drop off, the plant is short of nitrogen or the roots are waterlogged. Pick the yellow leaves off, feed weekly, and water little and often rather than in floods; brown blotches or black bottoms are a different problem."###,
        text: r###"Why are my tomato plant leaves going yellow?"###,
        turns: &[
            (
                r###"Why are my tomato plant leaves going yellow?"###,
                r###"Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease. If the yellow sits between green veins, that is magnesium: water evenly, mulch, and give a tomato feed with magnesium in it. If the lowest leaves go pale and drop off, the plant is short of nitrogen or the roots are waterlogged. Pick the yellow leaves off, feed weekly, and water little and often rather than in floods; brown blotches or black bottoms are a different problem."###,
            ),
            (
                r###"Why are my tomato plant leaves going yellow?"###,
                r###"Back to tomato plant leaves going yellow then: Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease."###,
            ),
        ],
        wants: None,
        expected: Some(
            r###"As I said: Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease."###,
        ),
    },
];
