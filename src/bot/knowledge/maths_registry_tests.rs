use super::super::{guards::Context, rows};
use super::*;

#[path = "../../../tests/fixtures/maths_registry.rs"]
mod fixture;

#[test]
fn statistics_and_school_guards_match_the_pinned_answers() {
    let entries: Vec<_> = rows()
        .filter(|row| matches!(row.id, "average-mean" | "rounding-how" | "expand-brackets"))
        .collect();
    for case in fixture::CASES {
        let reading = Reading::new(case.text);
        let context = Context::new(&reading, &[]);
        let row = entries.iter().find(|row| row.id == case.id).unwrap();
        assert_eq!(
            row.guard.passes(&context, row.id, row.key[0]),
            case.guard,
            "guard of {} for {:?}",
            case.id,
            case.text
        );
        assert_eq!(
            row.answer(&context).as_deref(),
            case.reply,
            "reply of {} for {:?}",
            case.id,
            case.text
        );
    }
}
