//! Matched prior-answer candidates from the PyBot 7d63c7f arbiter.

use super::*;
use crate::bot::Exchange;

const PRIOR_REPLY: &str = "Most adult dogs need 30 minutes to an hour a day. Leave a gap between the walk and feeding, and keep the normal route to begin with.";
const CASES: &[(&str, &str, usize, Option<usize>)] = &[
    ("and what does that mean", "answers", 0, Some(1)),
    ("and what does that mean", "answers", 1, None),
    ("and what does that mean", "answers", 2, Some(2)),
    ("and what does that mean", "answers", 3, Some(2)),
    ("and what does that mean", "answers", 4, Some(1)),
    ("and what does that mean", "answers", 5, None),
    ("and what does that mean", "answers", 6, Some(1)),
    ("and what does that mean", "greeting", 0, None),
    ("and what does that mean", "greeting", 1, None),
    ("and what does that mean", "greeting", 2, None),
    ("and what does that mean", "greeting", 3, None),
    ("and what does that mean", "greeting", 4, None),
    ("and what does that mean", "greeting", 5, None),
    ("and what does that mean", "greeting", 6, None),
    ("and what does that mean", "memory_store", 0, Some(1)),
    ("and what does that mean", "memory_store", 1, None),
    ("and what does that mean", "memory_store", 2, Some(2)),
    ("and what does that mean", "memory_store", 3, Some(2)),
    ("and what does that mean", "memory_store", 4, Some(1)),
    ("and what does that mean", "memory_store", 5, None),
    ("and what does that mean", "memory_store", 6, Some(1)),
    ("And what do you mean?", "answers", 0, Some(1)),
    ("And what do you mean?", "answers", 1, None),
    ("And what do you mean?", "answers", 2, Some(2)),
    ("And what do you mean?", "answers", 3, Some(2)),
    ("And what do you mean?", "answers", 4, Some(1)),
    ("And what do you mean?", "answers", 5, None),
    ("And what do you mean?", "answers", 6, Some(1)),
    ("And what do you mean?", "greeting", 0, None),
    ("And what do you mean?", "greeting", 1, None),
    ("And what do you mean?", "greeting", 2, None),
    ("And what do you mean?", "greeting", 3, None),
    ("And what do you mean?", "greeting", 4, None),
    ("And what do you mean?", "greeting", 5, None),
    ("And what do you mean?", "greeting", 6, None),
    ("And what do you mean?", "memory_store", 0, Some(1)),
    ("And what do you mean?", "memory_store", 1, None),
    ("And what do you mean?", "memory_store", 2, Some(2)),
    ("And what do you mean?", "memory_store", 3, Some(2)),
    ("And what do you mean?", "memory_store", 4, Some(1)),
    ("And what do you mean?", "memory_store", 5, None),
    ("And what do you mean?", "memory_store", 6, Some(1)),
    ("and what exactly do you mean by that?", "answers", 0, None),
    ("and what exactly do you mean by that?", "greeting", 0, None),
    (
        "and what exactly do you mean by that?",
        "memory_store",
        0,
        None,
    ),
    ("and what does that mean then", "answers", 0, Some(1)),
    ("and what does that mean then", "greeting", 0, None),
    ("and what does that mean then", "memory_store", 0, Some(1)),
    ("and what does that mean in practice", "answers", 0, Some(1)),
    ("and what does that mean in practice", "greeting", 0, None),
    (
        "and what does that mean in practice",
        "memory_store",
        0,
        Some(1),
    ),
    ("and can you explain that", "answers", 0, Some(1)),
    ("and can you explain that", "greeting", 0, None),
    ("and can you explain that", "memory_store", 0, Some(1)),
    ("and i don't follow", "answers", 0, Some(1)),
    ("and i don't follow", "greeting", 0, None),
    ("and i don't follow", "memory_store", 0, Some(1)),
    ("and that is confusing", "answers", 0, Some(1)),
    ("and that is confusing", "greeting", 0, None),
    ("and that is confusing", "memory_store", 0, Some(1)),
    ("and can you say it again", "answers", 0, Some(1)),
    ("and can you say it again", "answers", 1, None),
    ("and can you say it again", "answers", 2, Some(2)),
    ("and can you say it again", "answers", 3, Some(2)),
    ("and can you say it again", "answers", 4, Some(1)),
    ("and can you say it again", "answers", 5, None),
    ("and can you say it again", "answers", 6, Some(1)),
    ("and can you say it again", "greeting", 0, None),
    ("and can you say it again", "greeting", 1, None),
    ("and can you say it again", "greeting", 2, None),
    ("and can you say it again", "greeting", 3, None),
    ("and can you say it again", "greeting", 4, None),
    ("and can you say it again", "greeting", 5, None),
    ("and can you say it again", "greeting", 6, None),
    ("and can you say it again", "memory_store", 0, Some(1)),
    ("and can you say it again", "memory_store", 1, None),
    ("and can you say it again", "memory_store", 2, Some(2)),
    ("and can you say it again", "memory_store", 3, Some(2)),
    ("and can you say it again", "memory_store", 4, Some(1)),
    ("and can you say it again", "memory_store", 5, None),
    ("and can you say it again", "memory_store", 6, Some(1)),
    ("and what does that mean for my dog", "answers", 0, Some(1)),
    ("and what does that mean for my dog", "greeting", 0, None),
    (
        "and what does that mean for my dog",
        "memory_store",
        0,
        Some(1),
    ),
    ("and what does that mean for the router", "answers", 0, None),
    (
        "and what does that mean for the router",
        "greeting",
        0,
        None,
    ),
    (
        "and what does that mean for the router",
        "memory_store",
        0,
        None,
    ),
    ("and are you sure", "answers", 0, None),
    ("and are you sure", "greeting", 0, None),
    ("and are you sure", "memory_store", 0, None),
    (
        "and what does it mean when the tyre leaks",
        "answers",
        0,
        None,
    ),
    (
        "and what does it mean when the tyre leaks",
        "greeting",
        0,
        None,
    ),
    (
        "and what does it mean when the tyre leaks",
        "memory_store",
        0,
        None,
    ),
    ("and what about it", "answers", 0, None),
    ("and what about it", "greeting", 0, None),
    ("and what about it", "memory_store", 0, None),
    ("and how long did you say", "answers", 0, None),
    ("and how long did you say", "greeting", 0, None),
    ("and how long did you say", "memory_store", 0, None),
    ("and what time was that", "answers", 0, None),
    ("and what time was that", "greeting", 0, None),
    ("and what time was that", "memory_store", 0, None),
    ("and how many times", "answers", 0, None),
    ("and how many times", "greeting", 0, None),
    ("and how many times", "memory_store", 0, None),
    ("and can you explain that more simply", "answers", 0, None),
    ("and can you explain that more simply", "greeting", 0, None),
    (
        "and can you explain that more simply",
        "memory_store",
        0,
        None,
    ),
    (
        "and what does that mean in simple terms",
        "answers",
        0,
        None,
    ),
    (
        "and what does that mean in simple terms",
        "greeting",
        0,
        None,
    ),
    (
        "and what does that mean in simple terms",
        "memory_store",
        0,
        None,
    ),
    ("and can you put that another way", "answers", 0, None),
    ("and can you put that another way", "greeting", 0, None),
    ("and can you put that another way", "memory_store", 0, None),
    ("and can you say it more briefly", "answers", 0, None),
    ("and can you say it more briefly", "greeting", 0, None),
    ("and can you say it more briefly", "memory_store", 0, None),
    ("and why is that", "answers", 0, None),
    ("and why is that", "greeting", 0, None),
    ("and why is that", "memory_store", 0, None),
    ("and could you explain that more", "answers", 0, None),
    ("and could you explain that more", "greeting", 0, None),
    ("and could you explain that more", "memory_store", 0, None),
];

type Proposal = (&'static str, f64, &'static str);
const PROPOSALS: &[&[Proposal]] = &[
    &[
        (
            "after_a_lead_in",
            320.0,
            "'That' has nothing behind it here.",
        ),
        (
            "chat",
            300.0,
            "Just this: Most adult dogs need 30 minutes to an hour a day.",
        ),
        ("greeting", -1000.0, "That is what I said a moment ago."),
    ],
    &[
        (
            "after_a_lead_in",
            320.0,
            "'That' has nothing behind it here.",
        ),
        (
            "chat",
            300.0,
            "This is a different answer about the garden.",
        ),
        (
            "greeting",
            -1000.0,
            "Most adult dogs need 30 minutes to an hour a day.",
        ),
    ],
    &[
        (
            "after_a_lead_in",
            320.0,
            "'That' has nothing behind it here.",
        ),
        ("memory_store", 7000.0, "Your dog weighs 15 kilograms."),
        (
            "chat",
            300.0,
            "Just this: Most adult dogs need 30 minutes to an hour a day.",
        ),
    ],
    &[
        (
            "after_a_lead_in",
            320.0,
            "'That' has nothing behind it here.",
        ),
        ("chat", 300.0, "Most adult dogs need exercise."),
        (
            "said_back",
            350.0,
            "I said this: Most adult dogs need 30 minutes to an hour a day. Leave a gap between the walk and feeding, and keep the normal route to begin with.",
        ),
    ],
    &[
        (
            "after_a_lead_in",
            320.0,
            "'That' has nothing behind it here.",
        ),
        ("chat", 300.0, "Most need something else."),
        ("said_back", 350.0, "Dogs need something."),
        (
            "greeting",
            -1000.0,
            "Most adult dogs need 30 minutes to an hour a day.",
        ),
    ],
    &[
        (
            "after_a_lead_in",
            320.0,
            "'That' has nothing behind it here.",
        ),
        (
            "answers",
            0.0,
            "Most adult dogs need 30 minutes to an hour a day. Leave a gap between the walk and feeding, and keep the normal route to begin with.",
        ),
    ],
    &[
        (
            "after_a_lead_in",
            320.0,
            "'That' has nothing behind it here.",
        ),
        (
            "chat",
            300.0,
            "Do you mean whether adult dogs need 30 minutes?",
        ),
    ],
];

#[test]
fn clarification_compares_the_prior_subject_and_preserves_the_reply_band() {
    for &(line, previous_module, group, index) in CASES {
        let reading = Reading::new(line);
        let memory = Memory {
            turns: vec![Exchange {
                user: "My dog is scared of walks.".to_owned(),
                bot: PRIOR_REPLY.to_owned(),
                module: previous_module,
            }]
            .into(),
            ..Memory::default()
        };
        let proposals = PROPOSALS[group]
            .iter()
            .map(|&(module, priority, reply)| Candidate {
                module,
                priority,
                reply: reply.to_owned(),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            candidate(&reading, &memory, &proposals),
            index.map(|i| &proposals[i]),
            "{line:?} {previous_module} {group}"
        );
    }
}

#[test]
fn a_losing_lead_in_leaves_the_answer_and_earlier_fact_in_the_conversation() {
    let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
    assert_eq!(
        bot.turn("My dog is scared of walks.").module,
        Some("answers")
    );

    let clarified = bot.turn("and what does that mean");
    assert_eq!(clarified.module, Some("chat"), "{:?}", clarified.candidates);
    assert_eq!(
        clarified.reply,
        "Just this: Most adult dogs need 30 minutes to an hour a day."
    );
    assert!(clarified.candidates.iter().any(|candidate| {
        candidate.module == "after_a_lead_in"
            && candidate.reply == "'That' could be a lot of things and I've none of them in front of me - say the thing you mean and I'll come at it properly."
    }));

    let recall = bot.turn("What did I say about my dog?");
    assert_eq!(recall.module, Some("memory_store"));
    assert_eq!(
        recall.reply,
        "That was you, not me: “My dog is scared of walks.”"
    );
}
