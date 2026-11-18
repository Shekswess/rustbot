//! Direct recurrence branches and records from PyBot 7d63c7f.
use super::*;
use crate::bot::Exchange;

struct Case {
    name: &'static str,
    text: &'static str,
    turns: &'static [(&'static str, &'static str)],
    expected: Option<&'static str>,
    record: Option<&'static str>,
}

const CASES: &[Case] = &[
    Case {
        name: r####"recurrence"####,
        text: r####"its dropped again"####,
        turns: &[(
            r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
            r####"answers"####,
        )],
        expected: Some(r####"Stop topping it up and get a heating engineer out."####),
        record: Some(
            r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
        ),
    },
    Case {
        name: r####"skip the acknowledgement"####,
        text: r####"It has dropped again"####,
        turns: &[
            (
                r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
                r####"answers"####,
            ),
            (
                r####"Got it. You topped it up yesterday."####,
                r####"chat"####,
            ),
        ],
        expected: Some(r####"Stop topping it up and get a heating engineer out."####),
        record: Some(
            r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
        ),
    },
    Case {
        name: r####"foreground question"####,
        text: r####"will it drop again?"####,
        turns: &[(
            r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
            r####"answers"####,
        )],
        expected: None,
        record: None,
    },
    Case {
        name: r####"a person came back"####,
        text: r####"i went back to work today"####,
        turns: &[(
            r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
            r####"answers"####,
        )],
        expected: None,
        record: None,
    },
    Case {
        name: r####"an unrelated action"####,
        text: r####"should i give it back to the shop"####,
        turns: &[(
            r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
            r####"answers"####,
        )],
        expected: None,
        record: None,
    },
    Case {
        name: r####"no branch"####,
        text: r####"its dropped again"####,
        turns: &[(
            r####"Check the radiator pressure and note the reading."####,
            r####"answers"####,
        )],
        expected: None,
        record: None,
    },
    Case {
        name: r####"recurrence without again"####,
        text: r####"it came back"####,
        turns: &[(
            r####"Dry the patch before wiping it down, and check whether the seal outside is carrying water in. Fix the seal first if it comes back after rain."####,
            r####"answers"####,
        )],
        expected: Some(r####"Fix the seal first if it comes back after rain."####),
        record: Some(
            r####"Dry the patch before wiping it down, and check whether the seal outside is carrying water in. Fix the seal first if it comes back after rain."####,
        ),
    },
    Case {
        name: r####"mid-sentence condition stays"####,
        text: r####"the damp came back"####,
        turns: &[(
            r####"Dry the patch before wiping it down, and check whether the seal outside is carrying water in. Fix the seal first if it comes back after rain."####,
            r####"answers"####,
        )],
        expected: Some(r####"Fix the seal first if it comes back after rain."####),
        record: Some(
            r####"Dry the patch before wiping it down, and check whether the seal outside is carrying water in. Fix the seal first if it comes back after rain."####,
        ),
    },
    Case {
        name: r####"visible question asks another"####,
        text: r####"where will it go when it comes back?"####,
        turns: &[(
            r####"Dry the patch before wiping it down, and check whether the seal outside is carrying water in. Fix the seal first if it comes back after rain."####,
            r####"answers"####,
        )],
        expected: None,
        record: None,
    },
    Case {
        name: r####"conditional already in full"####,
        text: r####"what if it gets worse"####,
        turns: &[(
            r####"Check the lid if it gets worse."####,
            r####"answers"####,
        )],
        expected: None,
        record: None,
    },
    Case {
        name: r####"conditional with helpful branch"####,
        text: r####"and if it does not get better in two weeks"####,
        turns: &[(
            r####"The mark should fade once the room has dried out in the daytime for a full week. If it does not clear up, arrange for someone to check where the damp is coming in."####,
            r####"answers"####,
        )],
        expected: Some(r####"Arrange for someone to check where the damp is coming in."####),
        record: Some(r####"Arrange for someone to check where the damp is coming in."####),
    },
    Case {
        name: r####"do not cut a condition list"####,
        text: r####"the damp came back"####,
        turns: &[(
            r####"Watch the seal for a while. If it has been more than six weeks, keeps swelling up, or comes back every time, tell the landlord about the pipe."####,
            r####"answers"####,
        )],
        expected: None,
        record: None,
    },
    Case {
        name: r####"bare branch not repeated"####,
        text: r####"it came back"####,
        turns: &[(
            r####"Fix the seal first if it comes back after rain."####,
            r####"answers"####,
        )],
        expected: None,
        record: None,
    },
    Case {
        name: r####"safety boundary"####,
        text: r####"its dropped again"####,
        turns: &[
            (
                r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
                r####"answers"####,
            ),
            (r####"This is an emergency."####, r####"safety"####),
            (r####"Noted."####, r####"greeting"####),
        ],
        expected: None,
        record: None,
    },
    Case {
        name: r####"look back four"####,
        text: r####"its dropped again"####,
        turns: &[
            (
                r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
                r####"answers"####,
            ),
            (r####"Noted."####, r####"greeting"####),
            (r####"Fine."####, r####"greeting"####),
            (r####"Alright."####, r####"greeting"####),
        ],
        expected: Some(r####"Stop topping it up and get a heating engineer out."####),
        record: Some(
            r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
        ),
    },
    Case {
        name: r####"beyond four"####,
        text: r####"its dropped again"####,
        turns: &[
            (
                r####"Check the radiator pressure and write down the gauge reading before making any adjustments. If it keeps dropping, stop topping it up and get a heating engineer out."####,
                r####"answers"####,
            ),
            (r####"Noted."####, r####"greeting"####),
            (r####"Fine."####, r####"greeting"####),
            (r####"Alright."####, r####"greeting"####),
            (r####"Okay."####, r####"greeting"####),
        ],
        expected: None,
        record: None,
    },
];

#[test]
fn returned_decisions_match_pybot() {
    for case in CASES {
        let memory = Memory {
            turns: case
                .turns
                .iter()
                .map(|(bot, module)| Exchange {
                    user: String::new(),
                    bot: (*bot).into(),
                    module,
                })
                .collect(),
            ..Memory::default()
        };
        let decided = reply(case.text, &memory);
        assert_eq!(
            decided.as_ref().map(|part| part.reply.as_str()),
            case.expected,
            "{}",
            case.name
        );
        assert_eq!(
            decided.as_ref().and_then(|part| match &part.note {
                MemoryNote::Whole(whole) => Some(whole.as_str()),
                _ => None,
            }),
            case.record,
            "{}",
            case.name
        );
    }
}
