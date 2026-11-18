use super::super::rows;
use super::*;

#[path = "../../../tests/fixtures/film_shelves.rs"]
mod fixture;

#[test]
fn recommendations_match_pinned_shelves_limited_by_medium_country_and_company() {
    let cards: Vec<_> = rows()
        .filter(|row| matches!(row.id, "pick-plain" | "pick-country" | "pick-limit"))
        .collect();
    for case in fixture::CASES {
        let reading = Reading::new(case.text);
        let prior: Vec<_> = case
            .prior
            .iter()
            .map(|&user| Exchange {
                user: user.into(),
                bot: String::new(),
                module: "",
            })
            .collect();
        let context = Context::new(&reading, &prior);
        let row = cards.iter().find(|r| r.id == case.id).unwrap();
        assert_eq!(
            row.guard.passes(&context, row.id, row.key[0]),
            case.guard,
            "guard of {} for {:?} after {:?}",
            case.id,
            case.text,
            case.prior
        );
        assert_eq!(
            row.answer(&context).as_deref(),
            case.reply,
            "reply of {} for {:?} after {:?}",
            case.id,
            case.text,
            case.prior
        );
    }
}
