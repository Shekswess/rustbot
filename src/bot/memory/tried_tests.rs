//! Reply and cursor from PyBot 7d63c7f, with a clean prior answer for each scenario.
use super::*;
use crate::bot::Exchange;

struct Case {
    question: &'static str,
    answer: &'static str,
    turns: &'static [(&'static str, Option<&'static str>, Option<&'static str>)],
}

const CASES: &[Case] = &[
    Case {
        question: "My boiler keeps losing pressure",
        answer: "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        turns: &[
            (
                "I topped it up yesterday",
                Some(
                    "Since you've already topped it up yesterday, bleed the radiators and check it again the next day.",
                ),
                Some("bleed the radiators and check it again the next day"),
            ),
            (
                "and then what",
                Some(
                    "Next: if it keeps dropping, stop topping it up and get a heating engineer out.",
                ),
                Some("If it keeps dropping, stop topping it up and get a heating engineer out"),
            ),
            (
                "then what",
                Some("Then bleed the radiators and check it again the next day."),
                Some("bleed the radiators and check it again the next day"),
            ),
            (
                "what else can i try",
                Some(
                    "Next: if it keeps dropping, stop topping it up and get a heating engineer out.",
                ),
                Some("If it keeps dropping, stop topping it up and get a heating engineer out"),
            ),
        ],
    },
    Case {
        question: "My boiler keeps losing pressure",
        answer: "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        turns: &[
            (
                "I already bleed the radiators",
                Some(
                    "Then if it keeps dropping, stop topping it up and get a heating engineer out.",
                ),
                Some("If it keeps dropping, stop topping it up and get a heating engineer out"),
            ),
            (
                "what else can I try",
                Some("Next: bleed the radiators and check it again the next day."),
                Some("bleed the radiators and check it again the next day"),
            ),
        ],
    },
    Case {
        question: "My boiler keeps losing pressure",
        answer: "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        turns: &[
            (
                "I topped it up yesterday",
                Some(
                    "Since you've already topped it up yesterday, bleed the radiators and check it again the next day.",
                ),
                Some("bleed the radiators and check it again the next day"),
            ),
            (
                "what else about the boiler",
                None,
                Some("bleed the radiators and check it again the next day"),
            ),
        ],
    },
    Case {
        question: "My boiler keeps losing pressure",
        answer: "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        turns: &[("I've done that", None, None)],
    },
    Case {
        question: "My boiler keeps losing pressure",
        answer: "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        turns: &[("I topped up my kettle", None, None)],
    },
    Case {
        question: "My boiler keeps losing pressure",
        answer: "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        turns: &[
            (
                "I topped it up yesterday",
                Some(
                    "Since you've already topped it up yesterday, bleed the radiators and check it again the next day.",
                ),
                Some("bleed the radiators and check it again the next day"),
            ),
            (
                "never mind",
                None,
                Some("bleed the radiators and check it again the next day"),
            ),
            (
                "and then what",
                None,
                Some("bleed the radiators and check it again the next day"),
            ),
        ],
    },
    Case {
        question: "Why is my bathroom sink draining slowly?",
        answer: "Start with a plunger over the plughole, then undo the trap underneath (bucket first) and pull the gunk out. A kettle of hot water with a spoonful of bicarbonate and vinegar clears the rest. Skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back.",
        turns: &[
            (
                "ive tried the plunger",
                Some("Then undo the trap underneath (bucket first) and pull the gunk out."),
                Some("undo the trap underneath (bucket first) and pull the gunk out"),
            ),
            (
                "and then what",
                Some("Next: pull the gunk out."),
                Some("pull the gunk out"),
            ),
            (
                "then what",
                Some(
                    "Then skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back.",
                ),
                Some(
                    "Skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back",
                ),
            ),
            (
                "anything else?",
                Some("Then this: pull the gunk out."),
                Some("pull the gunk out"),
            ),
        ],
    },
    Case {
        question: "Why is my bathroom sink draining slowly?",
        answer: "Start with a plunger over the plughole, then undo the trap underneath (bucket first) and pull the gunk out. A kettle of hot water with a spoonful of bicarbonate and vinegar clears the rest. Skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back.",
        turns: &[
            (
                "I pulled the gunk out",
                Some(
                    "Then skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back.",
                ),
                Some(
                    "Skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back",
                ),
            ),
            (
                "then what",
                Some("Then this: pull the gunk out."),
                Some("pull the gunk out"),
            ),
        ],
    },
    Case {
        question: "Why is my bathroom sink draining slowly?",
        answer: "Start with a plunger over the plughole, then undo the trap underneath (bucket first) and pull the gunk out. A kettle of hot water with a spoonful of bicarbonate and vinegar clears the rest. Skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back.",
        turns: &[("I have undone the trap", None, None)],
    },
    Case {
        question: "Why is my bathroom sink draining slowly?",
        answer: "Start with a plunger over the plughole, then undo the trap underneath (bucket first) and pull the gunk out. A kettle of hot water with a spoonful of bicarbonate and vinegar clears the rest. Skip the chemical cleaner until last, and fill the trap with a hair collector so it does not come back.",
        turns: &[("I tried the brush", None, None)],
    },
    Case {
        question: "Why does my washing machine smell?",
        answer: "Run a hot service wash with washing machine cleaner, then clean the filter and the rubber seal. Leaving the door and drawer open between washes is what stops the smell coming back.",
        turns: &[
            (
                "ive cleaned the filter",
                Some("Then run a hot service wash with washing machine cleaner."),
                Some("Run a hot service wash with washing machine cleaner"),
            ),
            (
                "and then what",
                Some(
                    "Next: clean the filter and the rubber seal, and leaving the door and drawer open between washes is what stops the smell coming back.",
                ),
                Some(
                    "clean the filter and the rubber seal, and leaving the door and drawer open between washes is what stops the smell coming back",
                ),
            ),
            (
                "what else?",
                Some(
                    "Next: leaving the door and drawer open between washes is what stops the smell coming back.",
                ),
                Some(
                    "Leaving the door and drawer open between washes is what stops the smell coming back",
                ),
            ),
        ],
    },
    Case {
        question: "Why does my washing machine smell?",
        answer: "Run a hot service wash with washing machine cleaner, then clean the filter and the rubber seal. Leaving the door and drawer open between washes is what stops the smell coming back.",
        turns: &[(
            "i ran a hot wash with vinegar",
            Some(
                "You ran a hot wash with vinegar. Now clean the filter and the rubber seal, and leaving the door and drawer open between washes is what stops the smell coming back.",
            ),
            Some(
                "clean the filter and the rubber seal, and leaving the door and drawer open between washes is what stops the smell coming back",
            ),
        )],
    },
    Case {
        question: "Why does my washing machine smell?",
        answer: "Run a hot service wash with washing machine cleaner, then clean the filter and the rubber seal. Leaving the door and drawer open between washes is what stops the smell coming back.",
        turns: &[("ive cleaned the bathroom", None, None)],
    },
    Case {
        question: "My laptop won't switch on",
        answer: "Check the charger light first, then try another socket and cable; if the charger lights up but the laptop is still dead, then leave it with a repair shop.",
        turns: &[
            (
                "ive checked the charger",
                Some("Since you've already checked the charger, try another socket and cable."),
                Some("try another socket and cable"),
            ),
            (
                "what else can i try",
                Some("Next: leave it with a repair shop."),
                Some("leave it with a repair shop"),
            ),
        ],
    },
    Case {
        question: "What laptop should I buy?",
        answer: "Start with a laptop that has 16 GB of RAM, then check the size of the SSD. Compare the screens outside if you can.",
        turns: &[("ive tried the laptop", None, None)],
    },
    Case {
        question: "How do I fix a dripping tap?",
        answer: "A dripping tap is nearly always a worn washer or cartridge. Turn the water off at the isolating valve and take the handle off. Change the washer and put the handle back.",
        turns: &[("ive changed the washer", None, None)],
    },
    Case {
        question: "How do I get to sleep early?",
        answer: "Keep your phone away from the bed, then dim the lights an hour before bed. Put the clock where you can't see it.",
        turns: &[("i already keep my phone away", None, None)],
    },
    Case {
        question: "How can I stop the hinge from squeaking?",
        answer: "Spray the hinge with lubricant, then wipe any extra from the floor. Cover the floor next time to keep it clean.",
        turns: &[
            (
                "i sprayed the hinge",
                Some(
                    "Right, so you sprayed the hinge. Then wipe any extra from the floor, and cover the floor next time to keep it clean.",
                ),
                Some(
                    "wipe any extra from the floor, and cover the floor next time to keep it clean",
                ),
            ),
            (
                "and then what",
                Some("Next: cover the floor next time to keep it clean."),
                Some("Cover the floor next time to keep it clean"),
            ),
        ],
    },
];

#[test]
fn named_and_past_attempts_walk_only_the_preceding_answer() {
    for case in CASES {
        let mut memory = Memory::default();
        memory.turns.push(Exchange {
            user: case.question.to_owned(),
            module: "answers",
            bot: case.answer.to_owned(),
        });
        for &(text, expected, step) in case.turns {
            let offer = reply(text, &memory);
            assert_eq!(
                offer.as_ref().map(|part| part.reply.as_str()),
                expected,
                "after {:?}, asked {text:?}",
                case.question
            );
            let spoken = offer.map_or_else(
                || "Okay.".to_owned(),
                |part| {
                    let MemoryNote::Attempt(cursor) = part.note else {
                        panic!("a later step requires a cursor");
                    };
                    memory.personal.remember_attempt(cursor);
                    part.reply
                },
            );
            assert_eq!(
                memory
                    .personal
                    .last_attempt()
                    .map(|walk| walk.step.as_str()),
                step,
                "saved step after {text:?}"
            );
            memory.turns.push(Exchange {
                user: text.to_owned(),
                module: "chat",
                bot: spoken,
            });
        }
    }
}

#[test]
fn a_report_on_wording_advice_asks_how_it_went() {
    let cases = [
        (
            "what should i say to her",
            "greeting",
            "I already told her that",
            Some("What did they say?"),
        ),
        (
            "what should i say to her",
            "answers",
            "I've already told her that",
            None,
        ),
        (
            "what should i say to her",
            "safety",
            "I already told her that",
            None,
        ),
        (
            "what should i say to her",
            "greeting",
            "I already told you that",
            None,
        ),
        (
            "what should i say to her",
            "greeting",
            "I said what you suggested",
            Some("What did they say?"),
        ),
        (
            "what should i say to her",
            "chat",
            "we used your wording",
            Some("What did they say?"),
        ),
        ("what should i say to her", "chat", "I left it alone", None),
        (
            "what did i say about my trip",
            "answers",
            "I asked her",
            None,
        ),
        (
            "how would you word it",
            "answers",
            "I sent it",
            Some("What did they say?"),
        ),
        (
            "what to text my sister",
            "chat",
            "i texted her",
            Some("What did they say?"),
        ),
        ("my boss blamed me", "chat", "i told her", None),
        (
            "what should i say to her",
            "chat",
            "I spoke with her",
            Some("What did they say?"),
        ),
        (
            "how should I clean it",
            "answers",
            "I called the plumber",
            None,
        ),
    ];
    for (asked, module, line, expected) in cases {
        let mut memory = Memory::default();
        memory.turns.push(Exchange {
            user: asked.into(),
            module,
            bot: "I'm listening.".into(),
        });
        assert_eq!(
            acted(line, &memory).as_deref(),
            expected,
            "{line:?} after {asked:?}"
        );
    }
}
