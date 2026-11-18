//! PyBot 7d63c7f direct carried_on fixtures and state transitions.
use super::*;

#[test]
fn a_stood_down_opening_counts_in_spelling_retries_only() {
    let mut memory = Memory::default();
    let opening = "Write the one sentence you want people to remember, then build three slides around it and cut the rest.";
    memory.turns.push(Exchange {
        user: "How do I give a talk at work?".into(),
        bot: format!("{opening} Practise out loud twice, standing up, timing it, and take a one-page note instead of reading the slides; the first 30 seconds scripted and the rest in headings is what keeps a talk calm."),
        module: "answers",
    });
    let text = "I've never done one before.";
    assert_eq!(
        carried_on(text, false, None, &memory, None)
            .map(|part| part.reply)
            .as_deref(),
        Some(opening)
    );
    let reading = Reading::new(text);
    assert_eq!(respond(&reading, &memory), None);
    record(&reading, "A different answer won.", &mut memory);
    assert_eq!(memory.personal.carried_parts().len(), 0);
}

struct Case {
    name: &'static str,
    text: &'static str,
    turns: &'static [(&'static str, &'static str, &'static str)],
    repeated: &'static [&'static str],
    routine: bool,
    expected: Option<&'static str>,
}
const CASES: &[Case] = &[
    Case {
        name: r###"plain detail"###,
        text: r###"the detergent drawer looks dirty"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: Some(
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder."###,
        ),
    },
    Case {
        name: r###"reported attempt"###,
        text: r###"ive tried the filter and the drawer already"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: Some(r###"Run a hot empty wash with machine cleaner."###),
    },
    Case {
        name: r###"past action from answer"###,
        text: r###"i cleaned the filter"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: Some(r###"Run a hot empty wash with machine cleaner."###),
    },
    Case {
        name: r###"negative attempt"###,
        text: r###"i havent cleaned the filter"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"irregular past act"###,
        text: r###"i already ran a hot empty wash"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: Some(
            r###"Leave the door and drawer open between washes; this stops the damp smell coming back."###,
        ),
    },
    Case {
        name: r###"unrelated object"###,
        text: r###"i topped up my phone"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"agreement"###,
        text: r###"ok so hot empty wash then"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: Some(r###"Hot empty wash, yes."###),
    },
    Case {
        name: r###"thanks closes subject"###,
        text: r###"thanks. the detergent drawer is dirty"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"reset and new ask"###,
        text: r###"nvm what is happening with the detergent drawer"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"mobility limit"###,
        text: r###"she cant walk far"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"new owned thing"###,
        text: r###"i keep losing my keys"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"brooding"###,
        text: r###"i keep going over it in my head at 2am"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"bare worry"###,
        text: r###"i honestly cant get back off"###,
        turns: &[(
            r###"i keep waking early"###,
            r###"Take one small thing off the list for today and set a reminder to have a proper break. Keep the light off and do not check your phone when you wake up, then get up and sit somewhere dim if you are still awake."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: Some(
            r###"Keep the light off and do not check your phone when you wake up, then get up and sit somewhere dim if you are still awake."###,
        ),
    },
    Case {
        name: r###"question mark matters"###,
        text: r###"Why are my tomato plant leaves going yellow?"###,
        turns: &[(
            r###"Why are my tomato plant leaves going yellow?"###,
            r###"Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease. If the yellow sits between green veins, that is magnesium: water evenly, mulch, and give a tomato feed with magnesium in it. If the lowest leaves go pale and drop off, the plant is short of nitrogen or the roots are waterlogged. Pick the yellow leaves off, feed weekly, and water little and often rather than in floods; brown blotches or black bottoms are a different problem."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"normalized why input"###,
        text: r###"why are my tomato plant leaves going yellow"###,
        turns: &[(
            r###"Why are my tomato plant leaves going yellow?"###,
            r###"Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease. If the yellow sits between green veins, that is magnesium: water evenly, mulch, and give a tomato feed with magnesium in it. If the lowest leaves go pale and drop off, the plant is short of nitrogen or the roots are waterlogged. Pick the yellow leaves off, feed weekly, and water little and often rather than in floods; brown blotches or black bottoms are a different problem."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: Some(
            r###"Yellow leaves on a tomato are usually a feeding or watering problem rather than a disease."###,
        ),
    },
    Case {
        name: r###"empty history"###,
        text: r###"the detergent drawer is dirty"###,
        turns: &[],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"stock acknowledgment"###,
        text: r###"the detergent drawer is dirty"###,
        turns: &[(
            r###"i had a problem"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"greeting"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"stored long factual answer"###,
        text: r###"the detergent drawer looks dirty"###,
        turns: &[(
            r###"washer update"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"memory_store"###,
        )],
        repeated: &[],
        routine: false,
        expected: Some(
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder."###,
        ),
    },
    Case {
        name: r###"past step but empty next"###,
        text: r###"ive tried the machine cleaner"###,
        turns: &[(
            r###"washer update"###,
            r###"Run a hot empty wash with machine cleaner for a full hour and leave the machine closed."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"answer two turns back"###,
        text: r###"the detergent drawer looks dirty"###,
        turns: &[
            (
                r###"my washing machine smells"###,
                r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
                r###"answers"###,
            ),
            (
                r###"thanks"###,
                r###"Glad that helped."###,
                r###"greeting"###,
            ),
        ],
        repeated: &[],
        routine: false,
        expected: Some(
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder."###,
        ),
    },
    Case {
        name: r###"suppress already given"###,
        text: r###"the detergent drawer looks dirty"###,
        turns: &[
            (
                r###"my washing machine smells"###,
                r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
                r###"answers"###,
            ),
            (
                r###"the detergent drawer looks dirty"###,
                r###"Clean the filter and wipe the detergent drawer until it is free of old powder."###,
                r###"chat"###,
            ),
        ],
        repeated: &[],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"suppress saved raw"###,
        text: r###"the detergent drawer looks dirty"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder."###,
        ],
        routine: false,
        expected: None,
    },
    Case {
        name: r###"routine report"###,
        text: r###"the detergent drawer looks dirty"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: true,
        expected: None,
    },
    Case {
        name: r###"routine rejoinder"###,
        text: r###"the detergent drawer still looks dirty"###,
        turns: &[(
            r###"my washing machine smells"###,
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder. Run a hot empty wash with machine cleaner. Leave the door and drawer open between washes; this stops the damp smell coming back."###,
            r###"answers"###,
        )],
        repeated: &[],
        routine: true,
        expected: Some(
            r###"Clean the filter and wipe the detergent drawer until it is free of old powder."###,
        ),
    },
];

fn memory(case: &Case) -> Memory {
    let mut memory = Memory {
        turns: case
            .turns
            .iter()
            .map(|(user, bot, module)| Exchange {
                user: (*user).into(),
                bot: (*bot).into(),
                module,
            })
            .collect(),
        ..Memory::default()
    };
    for part in case.repeated {
        memory.personal.remember_carried((*part).into());
    }
    memory
}

#[test]
fn carried_decisions_match_pybot() {
    for case in CASES {
        let memory = memory(case);
        assert_eq!(
            carried_on(case.text, case.routine, None, &memory, None)
                .map(|part| part.reply)
                .as_deref(),
            case.expected,
            "{}",
            case.name,
        );
    }
}

#[test]
fn records_the_raw_handback_only_for_a_matching_reply() {
    let case = CASES
        .iter()
        .find(|case| case.name == "plain detail")
        .unwrap();
    let mut memory = memory(case);
    assert_eq!(respond(&Reading::new(case.text), &memory), None);
    let read = Reading::new("the detergent drawer still looks really dirty");
    let raw = case.expected.unwrap();
    assert_eq!(respond(&read, &memory).as_deref(), Some(raw));
    record(&read, "Some other candidate won.", &mut memory);
    assert_eq!(memory.personal.carried_parts().count(), 0);
    record(&read, raw, &mut memory);
    assert_eq!(memory.personal.carried_parts().collect::<Vec<_>>(), [raw]);
    assert_eq!(respond(&read, &memory), None);
}

#[test]
fn a_changed_destination_leaves_the_old_place_answer_behind() {
    let answer = "Rome has mild days at that time of year, but the evenings are cooler than the afternoon and it is worth taking a layer.";
    let line = "the evenings are cooler than the afternoon";
    let mut memory = Memory {
        turns: vec![Exchange {
            user: "I am going to Rome next week".into(),
            bot: answer.into(),
            module: "answers",
        }]
        .into(),
        ..Memory::default()
    };
    assert_eq!(
        carried_on(line, false, None, &memory, None).map(|part| part.reply),
        Some(answer.into())
    );
    memory.turns.push(Exchange {
        user: "I am off to Cornwall next week".into(),
        bot: "Got it. Which part of Cornwall?".into(),
        module: "greeting",
    });
    assert!(carried_on(line, false, None, &memory, None).is_none());
}
