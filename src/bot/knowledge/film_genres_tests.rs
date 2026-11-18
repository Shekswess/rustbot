use super::super::rows;
use super::*;
use crate::bot::{Memory, Reading};

#[path = "../../../tests/fixtures/film_genres.rs"]
mod fixture;

#[test]
fn genre_duration_and_mood_callbacks_match_the_pinned_rules() {
    let cards: Vec<_> = rows()
        .filter(|row| {
            matches!(
                row.id,
                "film-length" | "film-kind" | "a-mood-anywhere" | "a-mood-turned-down"
            )
        })
        .collect();
    for case in fixture::CASES {
        let reading = Reading::new(case.user);
        let mut memory = Memory {
            last_knowledge: case.saved,
            ..Memory::default()
        };
        memory
            .turns
            .extend(case.earlier.iter().map(|&user| Exchange {
                user: user.to_owned(),
                bot: String::new(),
                module: "answers",
            }));
        if let Some(reply) = case.answer
            && let Some(last) = memory.turns.last_mut()
        {
            last.bot.push_str(reply);
        }
        let mut context = Context::new(&reading, &memory.turns);
        context.memory = Some(&memory);
        let card = cards.iter().find(|row| row.id == case.id).unwrap();
        assert_eq!(
            card.answer(&context).as_deref(),
            case.expected,
            "{} for {:?} after {:?}",
            case.id,
            case.user,
            case.earlier
        );
    }
}
