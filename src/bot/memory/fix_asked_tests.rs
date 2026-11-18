//! Source decisions from PyBot 7d63c7f.
use super::*;

struct Case {
    prior: &'static [(&'static str, &'static str, &'static str)],
    ask: &'static str,
    expected: Option<&'static str>,
}
const CASES: &[Case] = &[
    Case {
        ask: "Will that be enough?",
        expected: Some(
            "Try it straight away. If it does what it should, that was the fix; if it behaves the same, the step was not the answer and something else is doing it.",
        ),
        prior: &[(
            "how do i clear the sink",
            "answers",
            "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
        )],
    },
    Case {
        ask: "and does that fix the pressure too",
        expected: Some(
            "Try it and watch what happens next. If it stays away, that was the whole of it; if it does not, the thing behind it needs seeing to.",
        ),
        prior: &[(
            "why does my boiler keep losing pressure",
            "answers",
            "Top it up using the filling loop, then check the pressure again the next day.",
        )],
    },
    Case {
        ask: "will that sort it out",
        expected: Some(
            "Do it now and check it the way you checked before. If nothing has changed at all, that step was not the fix and the next thing to look at is further in.",
        ),
        prior: &[(
            "why is the radiator cold at the top",
            "chat",
            "There is air in the radiator: open the bleed valve at the top with a radiator key.",
        )],
    },
    Case {
        ask: "will it work",
        expected: None,
        prior: &[(
            "why is it leaking",
            "answers",
            "Water at the front usually means the door seal has split.",
        )],
    },
    Case {
        ask: "does that fix it",
        expected: None,
        prior: &[(
            "my baby is not feeding",
            "answers",
            "Try a little water first and ring 111 if the baby keeps refusing to feed.",
        )],
    },
    Case {
        ask: "will that work",
        expected: None,
        prior: &[(
            "there is smoke from the boiler",
            "safety",
            "Leave it off and call an engineer.",
        )],
    },
    Case {
        ask: "does that help",
        expected: None,
        prior: &[(
            "this is awkward",
            "chat",
            "Tell me what has gone wrong so I can help.",
        )],
    },
    Case {
        ask: "how do i fix it",
        expected: None,
        prior: &[(
            "how do i clear the sink",
            "answers",
            "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
        )],
    },
    Case {
        ask: "is that enough to clear it after years of build up and trouble last time",
        expected: None,
        prior: &[(
            "how do i clear the sink",
            "answers",
            "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
        )],
    },
    Case {
        ask: "how long will that take",
        expected: None,
        prior: &[(
            "how do i clear the sink",
            "answers",
            "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
        )],
    },
    Case {
        ask: "will that fix it",
        expected: Some(
            "Try it straight away. If it does what it should, that was the fix; if it behaves the same, the step was not the answer and something else is doing it.",
        ),
        prior: &[
            ("it is blocked", "chat", "Right, it is blocked."),
            (
                "how do i clear the sink",
                "answers",
                "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
            ),
        ],
    },
    Case {
        ask: "will that fix it",
        expected: Some(
            "Try it straight away. If it does what it should, that was the fix; if it behaves the same, the step was not the answer and something else is doing it.",
        ),
        prior: &[
            (
                "how do i clear the sink",
                "answers",
                "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
            ),
            ("it is blocked", "chat", "Right, it is blocked."),
        ],
    },
    Case {
        ask: "will that fix it",
        expected: None,
        prior: &[
            (
                "how do i clear the sink",
                "answers",
                "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
            ),
            (
                "i see",
                "greeting",
                "Got it. Is there anything you'd like a hand with?",
            ),
        ],
    },
    Case {
        ask: "will that fix it",
        expected: Some(
            "Try it straight away. If it does what it should, that was the fix; if it behaves the same, the step was not the answer and something else is doing it.",
        ),
        prior: &[
            (
                "how do i clear the sink",
                "answers",
                "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
            ),
            (
                "how do i start",
                "chat",
                "The first thing: pull the hair out of the trap.",
            ),
        ],
    },
    Case {
        ask: "will that fix it",
        expected: None,
        prior: &[
            (
                "how do i clear the sink",
                "answers",
                "Pull the hair and soap out of the trap. Run the tap to see how it drains.",
            ),
            (
                "my school times",
                "a_day_somebody_else_keeps",
                "Check the timetable in the school app.",
            ),
        ],
    },
    Case {
        ask: "will it work",
        expected: Some(
            "Do it and give it a few days. Coming back after that is the sign it is not something the step sorts out on its own.",
        ),
        prior: &[(
            "how do i clean the stain",
            "answers",
            "Cover the mark in a mild paste and leave it overnight.",
        )],
    },
    Case {
        ask: "will it work",
        expected: None,
        prior: &[],
    },
];

#[test]
fn recognizes_the_fix_question_and_reads_when_the_step_can_be_checked() {
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
            "{:?} after {:?}",
            case.ask,
            case.prior,
        );
    }
}

const METHOD_CASES: &[Case] = &[
    Case {
        ask: "will it work",
        expected: Some(
            "That is how I would go about it, and it is the right thing to do first. Tell me how you get on and I will pick it up from there.",
        ),
        prior: &[(
            "how do i descale a kettle",
            "answers",
            "Half fill the kettle with equal parts water and white vinegar, or a sachet of citric acid, boil it and leave it half an hour, then tip it out and boil fresh water twice before you use it again. In a hard-water area do it every couple of months: the scale forms because hard water leaves calcium behind as it boils, and a thick layer makes the kettle slow and puts a scum on the tea.",
        )],
    },
    Case {
        ask: "would that actually work?",
        expected: Some(
            "That is the right first move, and what comes back from it decides the next step.",
        ),
        prior: &[(
            "how do i descale a kettle",
            "answers",
            "Half fill the kettle with equal parts water and white vinegar, or a sachet of citric acid, boil it and leave it half an hour, then tip it out and boil fresh water twice before you use it again. In a hard-water area do it every couple of months: the scale forms because hard water leaves calcium behind as it boils, and a thick layer makes the kettle slow and puts a scum on the tea.",
        )],
    },
    Case {
        ask: "does that fix it",
        expected: Some(
            "Yes, do it that way - it is the sensible order to do it in. Say how it goes.",
        ),
        prior: &[(
            "my baby is not feeding",
            "answers",
            "A baby under three months who stops feeding needs looking at today: ring 111 or your GP now rather than waiting for the next feed. Keep offering small feeds and note the wet nappies - it is the count over the day that tells you how they are doing. If there is also a temperature, they are floppy or hard to wake, they have a rash, or the nappies are dry, that is 999.",
        )],
    },
    Case {
        ask: "and does that fix the pressure too",
        expected: None,
        prior: &[(
            "my boiler keeps losing pressure",
            "answers",
            "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        )],
    },
    Case {
        ask: "will that sort it out",
        expected: None,
        prior: &[(
            "how do i unblock a slow bath",
            "answers",
            "Pull the hair and soap out of the trap first (a bent wire or tweezers does it), then pour a kettle of hot water down with a spoonful of bicarbonate of soda and vinegar; a plunger over the plughole clears the rest. Skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back. The reason is hair, soap and grease caught in the trap and the bend.",
        )],
    },
    Case {
        ask: "i blotted it, will that be enough?",
        expected: None,
        prior: &[(
            "it happened last night and i blotted it",
            "chat",
            "Good - blotting first is right. Now pour on cold water and blot again.",
        )],
    },
    Case {
        ask: "will that help",
        expected: Some(
            "That is the standard way of doing it and it works - tell me how it goes and we will take it from there.",
        ),
        prior: &[(
            "my back hurts",
            "answers",
            "Try a warm bath now, and keep moving gently throughout the day.",
        )],
    },
    Case {
        ask: "will it work",
        expected: Some(
            "It is the right thing to try first, and it works for most people. Give it a week or two and tell me how it goes.",
        ),
        prior: &[(
            "my knee aches when I run",
            "answers",
            "Ease off for a few weeks and build up gradually.",
        )],
    },
    Case {
        ask: "does that help",
        expected: Some(
            "That is the thing to try first, and most people do well on it. If it is no better in a week or two, that is when it wants someone to look rather than more of the same.",
        ),
        prior: &[(
            "my neck hurts",
            "answers",
            "Try a warm compress for twenty minutes. See a GP if the pain keeps coming back.",
        )],
    },
    Case {
        ask: "does it work",
        expected: None,
        prior: &[("where do i live", "memory_store", "You haven't said.")],
    },
    Case {
        ask: "would that work",
        expected: None,
        prior: &[(
            "I'm going to quit my job",
            "chat",
            "Work through what happens with rent and the timing first.",
        )],
    },
    Case {
        ask: "would that be worth it",
        expected: None,
        prior: &[(
            "we are thinking about buying a dog",
            "answers",
            "Look at the size of your garden and check whether the local park allows dogs.",
        )],
    },
    Case {
        ask: "will it work",
        expected: None,
        prior: &[(
            "something smells like gas",
            "safety",
            "Leave the building and call the gas emergency service from outside.",
        )],
    },
    Case {
        ask: "does that work",
        expected: None,
        prior: &[
            (
                "how do i descale a kettle",
                "answers",
                "Half fill the kettle with equal parts water and white vinegar, or a sachet of citric acid, boil it and leave it half an hour, then tip it out and boil fresh water twice before you use it again. In a hard-water area do it every couple of months: the scale forms because hard water leaves calcium behind as it boils, and a thick layer makes the kettle slow and puts a scum on the tea.",
            ),
            (
                "ok fair enough",
                "greeting",
                "Got it. Is there anything you'd like a hand with?",
            ),
        ],
    },
];

#[test]
fn whether_a_method_works_is_a_separate_check_from_the_fix() {
    for case in METHOD_CASES {
        let mut memory = Memory::default();
        for &(user, module, bot) in case.prior {
            memory.turns.push(Exchange {
                user: user.into(),
                module,
                bot: bot.into(),
            });
        }
        assert_eq!(
            method_doubt(&Reading::new(case.ask), &memory).as_deref(),
            case.expected,
            "{:?} after {:?}",
            case.ask,
            case.prior,
        );
    }
}
