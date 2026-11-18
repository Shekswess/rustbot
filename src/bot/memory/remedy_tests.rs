//! Named attempt fixtures from PyBot 7d63c7f.
use super::*;
use crate::bot::Exchange;

struct Case {
    prior: &'static [(&'static str, &'static str, &'static str)],
    turns: &'static [(&'static str, Option<&'static str>)],
}

const CASES: &[Case] = &[
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[
            (
                "i tried a hot bath",
                Some("Hot bath - did that do anything?"),
            ),
            (
                "i tried a heat pad",
                Some("Did the heat pad take the edge off, or nothing?"),
            ),
            (
                "i tried a heat pad",
                Some("And did the heat pad make a difference?"),
            ),
        ],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[("i tried a hot bath and it didnt help", None)],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[(
            "we gave the ice pack a go",
            Some("Ice pack - did that do anything?"),
        )],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[("we gave it a go", None)],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[("i tried that", None)],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[("i tried to get up", None)],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[(
            "i used turning it off and on",
            Some("Turning it off and on - did that do anything?"),
        )],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[(
            "i took some paracetamol",
            Some("Paracetamol - did that do anything?"),
        )],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[("i tried a hot bath again", None)],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[(
            "i tried a hot bath twice",
            Some("Hot bath - did that do anything?"),
        )],
    },
    Case {
        prior: &[(
            "my back hurts",
            "answers",
            "A warm bath can help settle it, and keep moving gently.",
        )],
        turns: &[(
            "i tried the alpha beta gamma delta epsilon",
            Some("Alpha beta gamma delta - did that do anything?"),
        )],
    },
    Case {
        prior: &[
            (
                "my back hurts",
                "answers",
                "A warm bath can help settle it, and keep moving gently.",
            ),
            (
                "i tried a hot bath",
                "chat",
                "Hot bath - did that do anything?",
            ),
        ],
        turns: &[(
            "i tried a hot bath",
            Some("Did the hot bath take the edge off, or nothing?"),
        )],
    },
    Case {
        prior: &[
            (
                "my back hurts",
                "answers",
                "A warm bath can help settle it, and keep moving gently.",
            ),
            ("it was fine", "chat", "Okay."),
            ("it was fine", "chat", "Okay."),
        ],
        turns: &[(
            "i tried a hot bath",
            Some("Hot bath - did that do anything?"),
        )],
    },
    Case {
        prior: &[
            (
                "my back hurts",
                "answers",
                "A warm bath can help settle it, and keep moving gently.",
            ),
            ("it was fine", "chat", "Okay."),
            ("it was fine", "chat", "Okay."),
            ("it was fine", "chat", "Okay."),
        ],
        turns: &[("i tried a hot bath", None)],
    },
    Case {
        prior: &[
            (
                "my back hurts",
                "answers",
                "A warm bath can help settle it, and keep moving gently.",
            ),
            ("it was fine", "chat", "Okay."),
            ("it was fine", "chat", "Okay."),
            ("the sink is clogged", "chat", "A plunger might help."),
        ],
        turns: &[("i used a plunger", Some("Plunger - did that do anything?"))],
    },
    Case {
        prior: &[(
            "my laptop keeps freezing",
            "chat",
            "How long has it been like that?",
        )],
        turns: &[(
            "i tried restarting it",
            Some("Restarting it - did that do anything?"),
        )],
    },
    Case {
        prior: &[(
            "where shall we eat tonight",
            "answers",
            "Try the Thai place near the library.",
        )],
        turns: &[("i tried the new thai place", None)],
    },
    Case {
        prior: &[(
            "my ankle is swollen",
            "safety",
            "Get it checked if you cannot put weight on it.",
        )],
        turns: &[(
            "i used an ice pack",
            Some("Ice pack - did that do anything?"),
        )],
    },
    Case {
        prior: &[],
        turns: &[("i tried a hot bath", None)],
    },
];

#[test]
fn only_a_named_attempt_on_an_active_problem_gets_the_question() {
    for case in CASES {
        let mut memory = Memory::default();
        for &(user, module, bot) in case.prior {
            memory.turns.push(Exchange {
                user: user.into(),
                module,
                bot: bot.into(),
            });
        }
        for &(text, expected) in case.turns {
            let offered = reply(text, &memory);
            assert_eq!(
                offered.as_deref(),
                expected,
                "on {text:?} after {:?}",
                case.prior
            );
            memory.turns.push(Exchange {
                user: text.into(),
                module: "chat",
                bot: offered.unwrap_or_else(|| "Okay.".into()),
            });
        }
    }
}
