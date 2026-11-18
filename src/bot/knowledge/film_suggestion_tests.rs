use super::super::rows;
use super::*;
use crate::bot::Exchange;

#[path = "../../../tests/fixtures/film_suggestions.rs"]
mod fixtures;

#[test]
fn film_suggestions_match_authored_replies_and_guards() {
    let entertainment: Vec<_> = rows()
        .filter(|row| matches!(row.id, "film-like" | "film-nothing-scary" | "film-war"))
        .collect();
    for case in fixtures::CASES {
        let reading = Reading::new(case.prompt);
        let mut turns: Vec<_> = case
            .topics
            .iter()
            .map(|&topic| Exchange {
                user: topic.into(),
                bot: String::new(),
                module: "",
            })
            .collect();
        if let Some(answer) = case.last {
            turns.push(Exchange {
                user: String::new(),
                bot: answer.into(),
                module: "answers",
            });
        }
        let context = Context::new(&reading, &turns);
        let row = entertainment.iter().find(|row| row.id == case.id).unwrap();
        assert_eq!(
            row.guard.passes(&context, row.id, row.key[0]),
            case.guard,
            "guard of {}: {:?}",
            case.id,
            case.prompt,
        );
        assert_eq!(
            row.answer(&context).as_deref(),
            case.reply,
            "reply of {}: {:?} after {:?}",
            case.id,
            case.prompt,
            case.topics,
        );
    }
}
