use super::super::{guards::Context, rows};
use super::Reading;

#[path = "../../../tests/fixtures/sport_practice.rs"]
mod fixture;

#[test]
fn gym_induction_and_training_frequency_respect_the_ask() {
    let turn = crate::bot::Bot::new(Some("0"))
        .unwrap()
        .turn("What happens at a gym induction?");
    assert_eq!(turn.module, Some("answers"));
    assert_eq!(turn.reply, super::data::GYM_INDUCTION);
    for (prompt, expected) in [
        (
            "What should I bring to my gym induction?",
            super::data::GYM_INDUCTION_KIT,
        ),
        (
            "I made a mistake at my gym induction.",
            super::data::GYM_INDUCTION,
        ),
    ] {
        assert_eq!(
            super::gym_induction(&Reading::new(prompt)).as_deref(),
            Some(expected)
        );
    }
    let run = crate::lookup_knowledge("How often should I run?").unwrap();
    assert_eq!((run.domain, run.id), ("sport", "how-often-exercise"));
    assert!(!super::is_not_a_timetable(&Reading::new(
        "How often does the bus run?"
    )));
}

#[test]
fn practice_replies_distinguish_batting_bowling_and_the_other_sports() {
    for &fixture::Case(id, prompt, enabled, answer) in fixture::CASES {
        let row = rows()
            .find(|row| row.domain == "sport" && row.id == id)
            .unwrap();
        let reading = Reading::new(prompt);
        let context = Context::new(&reading, &[]);
        assert_eq!(
            row.guard.passes(&context, id, row.key[0]),
            enabled,
            "{id}: {prompt}"
        );
        assert_eq!(
            row.answer(&context).as_deref(),
            Some(answer),
            "{id}: {prompt}"
        );
    }
}
