//! Clause walking fixtures from PyBot 7d63c7f.
use super::*;
use crate::bot::Exchange;

struct Case {
    question: &'static str,
    answer: &'static str,
    module: &'static str,
    line: &'static str,
    expected: Option<&'static str>,
}

const CASES: &[Case] = &[
    Case {
        question: "how do i get red wine out of a cream carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "answers",
        line: "it happened last night and i blotted it",
        expected: Some("Good - blotting first is right. Now pour on cold water and blot again."),
    },
    Case {
        question: "how do i get red wine out of a cream carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "answers",
        line: "and i blotted it",
        expected: Some("Good - blotting first is right. Now pour on cold water and blot again."),
    },
    Case {
        question: "how do i get red wine out of a cream carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "answers",
        line: "i rubbed it",
        expected: None,
    },
    Case {
        question: "how do i get red wine out of a cream carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "answers",
        line: "i blotted it, will that be enough",
        expected: None,
    },
    Case {
        question: "how do i get red wine out of a cream carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "answers",
        line: "i hate this carpet",
        expected: None,
    },
    Case {
        question: "how do i get red wine out of a cream carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "answers",
        line: "i blotted it but spilled it",
        expected: None,
    },
    Case {
        question: "how do i get red wine out of a cream carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "answers",
        line: "i dont have kitchen paper",
        expected: None,
    },
    Case {
        question: "how do i get red wine out of a cream carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "chat",
        line: "it happened last night and i blotted it",
        expected: None,
    },
    Case {
        question: "how to descale a kettle",
        answer: "Fill it with half water and half vinegar, or a sachet of citric acid. Leave it for an hour, then rinse it and boil fresh water twice.",
        module: "answers",
        line: "i used the vinegar",
        expected: Some("That was the right first move. Next, rinse it and boil fresh water twice."),
    },
    Case {
        question: "how to descale a kettle",
        answer: "Fill it with half water and half vinegar, or a sachet of citric acid. Leave it for an hour, then rinse it and boil fresh water twice.",
        module: "answers",
        line: "it boiled over",
        expected: None,
    },
    Case {
        question: "how to descale a kettle",
        answer: "Fill it with half water and half vinegar, or a sachet of citric acid. Leave it for an hour, then rinse it and boil fresh water twice.",
        module: "answers",
        line: "i boiled it dry",
        expected: None,
    },
    Case {
        question: "how to descale a kettle",
        answer: "Fill it with half water and half vinegar, or a sachet of citric acid. Leave it for an hour, then rinse it and boil fresh water twice.",
        module: "answers",
        line: "i rinsed it",
        expected: None,
    },
    Case {
        question: "whats the best way to clean my oven",
        answer: "Spread bicarbonate of soda inside the oven, not on the elements, then leave it overnight. Wipe it clean with a damp cloth, and dry it in daylight.",
        module: "answers",
        line: "i did the bicarbonate bit",
        expected: Some("That was the right first move. Next, wipe it clean with a damp cloth."),
    },
    Case {
        question: "whats the best way to clean my oven",
        answer: "Spread bicarbonate of soda inside the oven, not on the elements, then leave it overnight. Wipe it clean with a damp cloth, and dry it in daylight.",
        module: "answers",
        line: "i wiped it clean with a damp cloth",
        expected: None,
    },
    Case {
        question: "any tips for ants in the kitchen",
        answer: "Put the gel near the trail, then give the bait a week or two: the ants keep coming for the first few days and then the stream thins out. Wash the surface later.",
        module: "answers",
        line: "i put down the gel",
        expected: None,
    },
    Case {
        question: "any tips for ants in the kitchen",
        answer: "Put the gel near the trail, then give the bait a week or two: the ants keep coming for the first few days and then the stream thins out. Wash the surface later.",
        module: "answers",
        line: "i tried the gel and waited",
        expected: None,
    },
    Case {
        question: "how to do it",
        answer: "Spray from the bottom up, above that. Wipe with a cloth.",
        module: "answers",
        line: "i sprayed it",
        expected: None,
    },
    Case {
        question: "how to use my filter",
        answer: "Remove the cover, then GP guidance says to keep the old filter so you can compare its dimensions when you go back to the shop and ask someone to help you find the next size up for your unit.",
        module: "answers",
        line: "i removed the cover",
        expected: None,
    },
    Case {
        question: "what happened to the carpet",
        answer: "Blot it, don't rub, with kitchen paper, then pour on cold water and blot again, repeating until it stops coming out. If it still shows, use a carpet cleaner safe for the material.",
        module: "answers",
        line: "it happened last night and i blotted it",
        expected: None,
    },
    Case {
        question: "how to use my filter",
        answer: "Lift the cover, then GP guidance says to keep the old filter so you can compare its dimensions when you go back to the shop and ask someone to help you find the next size up for your unit.",
        module: "answers",
        line: "i lifted the cover",
        expected: Some(
            "That was the right first move. Next, GP guidance says to keep the old filter so you can compare its dimensions when you go back to the shop.",
        ),
    },
    Case {
        question: "how to use my filter",
        answer: "Lift the cover, then A shop that stocks that kind of filter can help find the right one. Clean the case and swap it over.",
        module: "answers",
        line: "i lifted the cover",
        expected: Some(
            "That was the right first move. Next, a shop that stocks that kind of filter can help find the right one.",
        ),
    },
    Case {
        question: "how to use my filter",
        answer: "Lift the cover, then give the filter a short test: the diagnostic instructions say you can watch the display for five minutes and check it again after that period.",
        module: "answers",
        line: "i lifted the cover",
        expected: Some("Right - lifting is the first bit done. Now give the filter a short test."),
    },
];

#[test]
fn reads_the_clause_after_a_reported_action() {
    for case in CASES {
        let mut memory = Memory::default();
        memory.turns.push(Exchange {
            user: case.question.into(),
            bot: case.answer.into(),
            module: case.module,
        });
        assert_eq!(
            reply(case.line, &memory).as_deref(),
            case.expected,
            "{:?} following {:?}",
            case.line,
            case.question,
        );
    }
}
