//! Conditioned bare questions from PyBot 7d63c7f.
use super::*;

#[test]
fn the_pinned_unfinished_plan_can_carry_an_immediately_quoted_residence() {
    let mut bot = crate::Bot::new(Some("0")).unwrap();
    for line in [
        "My name is Ana and my sister is Maya.",
        "Maya lives in Bristol and I live in Leeds.",
        "My favourite fruit is a mango, but I can't eat peanuts.",
        "What is my favourite fruit?",
        "where does she live?",
        "Actually, she moved to Bath.",
        "and where does she live now?",
    ] {
        bot.turn(line);
    }
    let plan = "I was going to paint the spare room but I haven't done it yet.";
    let turn = bot.turn(plan);
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(
        turn.reply,
        concat!(
            "In Bristol: the harbourside and the SS Great Britain, the Clifton Suspension Bridge, ",
            "and Wapping Wharf or Stokes Croft for food. It is a walking city; the buses are the weak point."
        )
    );
    let recalled = bot.turn("What did I say I had to do?");
    assert_eq!(recalled.module, Some("memory_store"));
    assert!(recalled.reply.contains("the spare room"));
    assert!(!recalled.reply.contains("Bristol"));

    bot.reset().unwrap();
    assert!(bot.turn(plan).reply.starts_with("Before paint"));
}

#[test]
fn a_quoted_residence_stops_at_an_owned_subject_or_topic_boundary() {
    let mut memory = Memory::default();
    memory.turns.push(Exchange {
        user: "Maya lives in Cardiff and I live in York.".into(),
        bot: String::new(),
        module: "chat",
    });
    memory.turns.push(Exchange {
        user: "Where does she live now?".into(),
        bot: "You told me: \"Maya lives in Cardiff and I live in York.\".".into(),
        module: "said_fact_back",
    });
    let reading = Reading::new("I was going to paint the spare room but I haven't done it yet.");
    let widened = unfinished_plan_after_residence_quote(&reading, &memory).unwrap();
    let selected = lookup::lookup_in_context(&Reading::new(&widened), &memory).unwrap();
    assert_eq!((selected.domain, selected.id), ("travel", "city-guide"));
    assert!(selected.reply.starts_with("In Cardiff:"));

    let owned = Reading::new("I was going to paint my spare room but I haven't done it yet.");
    assert!(unfinished_plan_after_residence_quote(&owned, &memory).is_none());
    memory.turns.insert(
        1,
        Exchange {
            user: "Different topic.".into(),
            bot: String::new(),
            module: "greeting",
        },
    );
    assert!(unfinished_plan_after_residence_quote(&reading, &memory).is_none());
    memory.turns[1].user = "I smell gas in my kitchen.".into();
    memory.turns[1].module = "safety";
    assert!(unfinished_plan_after_residence_quote(&reading, &memory).is_none());
}

#[test]
fn a_short_request_reaches_the_held_topic_when_the_card_answers_it() {
    let mut bot = crate::Bot::new(Some("0")).unwrap();
    let chat = bot.turn("/help");
    assert_eq!(chat.module, Some("chat"));
    assert!(chat.candidates.iter().all(|item| item.module != "answers"));

    bot.reset().unwrap();
    bot.turn("No, I meant the word ‘photosynthesis’.");
    let offered = bot.turn("help");
    assert_eq!(offered.module, Some("chat"));
    assert!(
        offered.candidates.iter().any(|item| {
            item.module == "answers" && item.reply.starts_with("Plants use sunlight")
        })
    );

    bot.reset().unwrap();
    bot.turn("No, I meant the word ‘arthritis’.");
    assert_eq!(bot.turn("help").module, Some("chat"));

    bot.reset().unwrap();
    bot.turn("I smell gas in my kitchen. What should I do?");
    bot.turn("It was in a dream, I am safe now.");
    bot.turn("No, I meant the word ‘dream’.");
    let offered = bot.turn("/help");
    assert_eq!(offered.module, Some("answers"));
    assert!(
        offered
            .reply
            .starts_with("Dreaming happens mostly in REM sleep")
    );
}

struct Case {
    name: &'static str,
    prior: &'static [(&'static str, &'static str, &'static str)],
    ask: &'static str,
    expected: Option<&'static str>,
}

const CASES: &[Case] = &[
    Case {
        name: "boiler",
        prior: &[
            (
                "My boiler keeps losing pressure",
                "answers",
                "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
            ),
            (
                "I topped it up yesterday",
                "chat",
                "You've already topped it up yesterday. Bleed the radiators and check it again the next day.",
            ),
            (
                "and then what",
                "answers",
                "Same answer, and it covers this: Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day.",
            ),
            (
                "then what",
                "steps",
                "Start with this: Top the pressure up to between 1 and 1.5 bar using the filling loop. I make it two steps.",
            ),
        ],
        ask: "what else can i try",
        expected: Some(
            "Same answer, and it covers this: Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        ),
    },
    Case {
        name: "boiler-recent",
        prior: &[(
            "My boiler keeps losing pressure",
            "answers",
            "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        )],
        ask: "what else can i try",
        expected: None,
    },
    Case {
        name: "new-ask",
        prior: &[(
            "My boiler keeps losing pressure",
            "answers",
            "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        )],
        ask: "Where should my sister eat?",
        expected: None,
    },
    Case {
        name: "new-topic-marker",
        prior: &[
            (
                "My boiler keeps losing pressure",
                "answers",
                "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
            ),
            (
                "Changing subject.",
                "answers",
                "Check the entry requirements on the university or college page for the exact course, because they differ more than people expect - most want five GCSEs at grade 4 or above including English and maths, and A-levels or a BTEC in a subject that fits. Pick the subjects that keep two doors open rather than one, and ask a teacher or someone doing the job now what they actually needed.",
            ),
        ],
        ask: "what else can i try",
        expected: None,
    },
    Case {
        name: "reset-on-ask",
        prior: &[(
            "My boiler keeps losing pressure",
            "answers",
            "Top the pressure up to between 1 and 1.5 bar using the filling loop, then bleed the radiators and check it again the next day. The pressure falls because water is escaping somewhere: a radiator valve, a joint, or the pressure relief pipe outside. If it keeps dropping, stop topping it up and get a heating engineer out.",
        )],
        ask: "Never mind, what should I try?",
        expected: None,
    },
    Case {
        name: "first",
        prior: &[],
        ask: "What should I try?",
        expected: None,
    },
    Case {
        name: "job",
        prior: &[(
            "my kids wont listen to me",
            "answers",
            "Say the rule before the trouble, not while it is happening, in one sentence: 'we do not hit; if you hit, the tablet goes away'. Then give the same consequence every time, because the sameness is what does the work, and keep the telling-off to about ten seconds with the reason inside it. And don't hit - it teaches that hitting works. If the same thing blows up every day, look for the pattern first: tired, hungry, right after school, or screens.",
        )],
        ask: "what can i do",
        expected: None,
    },
];

#[test]
fn bare_subject_observes_recent_answers_and_topic_changes() {
    for case in CASES {
        let memory = Memory {
            turns: case
                .prior
                .iter()
                .map(|&(user, module, bot)| Exchange {
                    user: user.into(),
                    module,
                    bot: bot.into(),
                })
                .collect(),
            ..Memory::default()
        };
        let proposal = respond(&Reading::new(case.ask), &memory);
        assert_eq!(
            proposal.as_ref().map(|answer| answer.reply.as_str()),
            case.expected,
            "{}",
            case.name
        );
    }
}

#[test]
fn bare_question_returns_the_older_boiler_answer_after_steps() {
    let expected = &CASES[0];
    let mut bot = crate::Bot::new(Some("0")).unwrap();
    let repeated = format!("Same answer, and it covers this — {}", expected.prior[0].2);
    for &(line, module, reply) in expected.prior {
        let turn = bot.turn(line);
        assert_eq!(turn.module, Some(module), "{line}");
        assert_eq!(turn.reply, reply, "{line}");
        if module == "steps" {
            assert!(
                turn.candidates.iter().any(|candidate| {
                    candidate.module == "answers" && candidate.reply == repeated
                })
            );
        }
    }
    let turn = bot.turn(expected.ask);
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(Some(turn.reply.as_str()), expected.expected);
}

#[test]
fn alternate_remedy_is_not_an_ordered_step_after_a_shortened_repeat() {
    let expected = &CASES[0];
    let mut bot = crate::Bot::new(Some("0")).unwrap();
    for &(line, _, _) in &expected.prior[..3] {
        bot.turn(line);
    }
    let turn = bot.turn(expected.ask);
    assert!(!turn.candidates.iter().any(|pick| pick.module == "answers"));
}

#[test]
fn bare_subject_cannot_rewrite_approved_medicine_followups() {
    for (prior, referral_fragment) in [
        (
            "Can I take antacids with my tablets?",
            "ask a pharmacist about the exact products",
        ),
        (
            "Can I take ibuprofen with sertraline?",
            "ask a pharmacist before taking the painkiller",
        ),
        (
            "I forgot my contraceptive pill last night. What should I do?",
            "Check the missed-pill instructions",
        ),
    ] {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        let reply = bot.turn(prior);
        assert_eq!(reply.module, Some("answers"), "{prior}");
        assert!(reply.reply.contains(referral_fragment), "{prior}");
        for ask in [
            "What should I take?",
            "What should I do?",
            "How long should I wait?",
            "How long do I have?",
        ] {
            assert!(
                respond(&Reading::new(ask), &bot.memory).is_none(),
                "{prior} / {ask}"
            );
        }
    }
}
