//! Direct PyBot 7d63c7f the_move fixtures.
use super::*;
use crate::bot::Exchange;

struct Case {
    name: &'static str,
    text: &'static str,
    reply: &'static str,
    turns: &'static [(&'static str, &'static str)],
    expected: Option<&'static str>,
}

const CASES: &[Case] = &[
    Case {
        name: r####"ordinary start"####,
        text: r####"where do i start"####,
        reply: r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        )],
        expected: Some(
            r####"The first thing to do: Clean the filter and detergent drawer before the next wash."####,
        ),
    },
    Case {
        name: r####"first question"####,
        text: r####"what should i do first"####,
        reply: r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        )],
        expected: Some(
            r####"The thing to do first: Clean the filter and detergent drawer before the next wash."####,
        ),
    },
    Case {
        name: r####"change wording"####,
        text: r####"what should i change first"####,
        reply: r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        )],
        expected: Some(
            r####"The first thing I would change: Clean the filter and detergent drawer before the next wash."####,
        ),
    },
    Case {
        name: r####"wording declines"####,
        text: r####"how do i word it"####,
        reply: r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        )],
        expected: None,
    },
    Case {
        name: r####"figure declines"####,
        text: r####"how long does it take"####,
        reply: r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        )],
        expected: None,
    },
    Case {
        name: r####"new subject declines"####,
        text: r####"how do i start running"####,
        reply: r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        )],
        expected: None,
    },
    Case {
        name: r####"pure statement declines"####,
        text: r####"Pay is at least the level agreed in the contract."####,
        reply: r####"Rinse the rice until the water runs clear."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Rinse the rice until the water runs clear."####,
        )],
        expected: None,
    },
    Case {
        name: r####"not a step"####,
        text: r####"where do i start"####,
        reply: r####"Pay is at least the level agreed in the contract. Most schemes vary by employer."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Pay is at least the level agreed in the contract. Most schemes vary by employer."####,
        )],
        expected: None,
    },
    Case {
        name: r####"conditional"####,
        text: r####"where do i start"####,
        reply: r####"If the door is open, check which seal is damaged. Take a photo of the make and serial number."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"If the door is open, check which seal is damaged. Take a photo of the make and serial number."####,
        )],
        expected: Some(
            r####"The first thing to do: Take a photo of the make and serial number."####,
        ),
    },
    Case {
        name: r####"self-labeled first step"####,
        text: r####"what should i do first"####,
        reply: r####"Start with ten minutes at a pace you can talk through. Add another minute each week."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Start with ten minutes at a pace you can talk through. Add another minute each week."####,
        )],
        expected: Some(
            r####"The one to do now: Start with ten minutes at a pace you can talk through."####,
        ),
    },
    Case {
        name: r####"method stays with step"####,
        text: r####"what should i do first"####,
        reply: r####"Start with run-walk: one minute of easy jogging, two minutes of walking, repeated for 20 minutes three times a week. The pace can be easy."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Start with run-walk: one minute of easy jogging, two minutes of walking, repeated for 20 minutes three times a week. The pace can be easy."####,
        )],
        expected: Some(
            r####"The one to do now: Start with run-walk: one minute of easy jogging, two minutes of walking, repeated for 20 minutes three times a week."####,
        ),
    },
    Case {
        name: r####"thing list step"####,
        text: r####"what should i do first"####,
        reply: r####"The things that work: same wake-up time every day, no caffeine after early afternoon, a cool dark room. Track how often you wake in the night."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"The things that work: same wake-up time every day, no caffeine after early afternoon, a cool dark room. Track how often you wake in the night."####,
        )],
        expected: Some(r####"The thing to do first: Same wake-up time every day."####),
    },
    Case {
        name: r####"list of dishes is not a step"####,
        text: r####"what should i do first"####,
        reply: r####"Learn four things and the rest follows: pasta with a sauce, an omelette, a tray of roasted vegetables. Weigh everything out before you start."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Learn four things and the rest follows: pasta with a sauce, an omelette, a tray of roasted vegetables. Weigh everything out before you start."####,
        )],
        expected: Some(r####"The thing to do first: Weigh everything out before you start."####),
    },
    Case {
        name: r####"bracket stays whole"####,
        text: r####"what should i do first"####,
        reply: r####"Turn off the water at the stopcock (not at the tap, which might still leak), and put a container under the joint."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Turn off the water at the stopcock (not at the tap, which might still leak), and put a container under the joint."####,
        )],
        expected: Some(r####"The thing to do first: Turn off the water at the stopcock."####),
    },
    Case {
        name: r####"qualifier cuts"####,
        text: r####"what should i do first"####,
        reply: r####"Run a hot 60C service wash with a washing-machine cleaner because the usual short programme will not clear the filter."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Run a hot 60C service wash with a washing-machine cleaner because the usual short programme will not clear the filter."####,
        )],
        expected: Some(
            r####"The thing to do first: Run a hot 60C service wash with a washing-machine cleaner."####,
        ),
    },
    Case {
        name: r####"complement is kept"####,
        text: r####"what should i do first"####,
        reply: r####"Put the immersion and any room heaters on a timer for the hours you actually use."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Put the immersion and any room heaters on a timer for the hours you actually use."####,
        )],
        expected: Some(
            r####"The thing to do first: Put the immersion and any room heaters on a timer for the hours you actually use."####,
        ),
    },
    Case {
        name: r####"leading role"####,
        text: r####"what should i do first"####,
        reply: r####"Half fill the kettle with equal parts water and white vinegar. Boil it and leave it half an hour."####,
        turns: &[(
            r####"my washing machine smells"####,
            r####"Half fill the kettle with equal parts water and white vinegar. Boil it and leave it half an hour."####,
        )],
        expected: Some(
            r####"The thing to do first: Half fill the kettle with equal parts water and white vinegar."####,
        ),
    },
    Case {
        name: r####"tried step declines"####,
        text: r####"where do i start"####,
        reply: r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        turns: &[
            (
                r####"my washing machine smells"####,
                r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
            ),
            (
                r####"ive already tried cleaning the filter"####,
                r####"Got it."####,
            ),
        ],
        expected: None,
    },
    Case {
        name: r####"already handed back"####,
        text: r####"where do i start"####,
        reply: r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
        turns: &[
            (
                r####"my washing machine smells"####,
                r####"The seal is where most of the damp sits. Clean the filter and detergent drawer before the next wash. Leave the door open after you finish."####,
            ),
            (
                r####"what should i try first"####,
                r####"Clean the filter and detergent drawer before the next wash."####,
            ),
        ],
        expected: None,
    },
];

#[test]
fn first_step_matches_pybot() {
    for case in CASES {
        let memory = Memory {
            turns: case
                .turns
                .iter()
                .map(|(user, bot)| Exchange {
                    user: (*user).into(),
                    bot: (*bot).into(),
                    module: "answers",
                })
                .collect(),
            ..Memory::default()
        };
        assert_eq!(
            hand_back(case.text, case.reply, &memory).as_deref(),
            case.expected,
            "{}",
            case.name
        );
    }
}
