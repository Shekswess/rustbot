//! The initial and follow-up wording for the choice of fat for the pan.
use super::{COOKING_FAT_CARD, COOKING_FAT_DECIDER};
use crate::bot::{Memory, compose};

fn opening(line: &str) -> &str {
    line.char_indices()
        .nth(40)
        .map_or(line, |(end, _)| &line[..end])
}

pub(super) fn reply(memory: Option<&Memory>) -> Option<&'static str> {
    let previous =
        memory.and_then(|memory| memory.turns.last().map(|turn| (memory, turn.bot.as_str())));
    if let Some((memory, last)) = previous {
        if COOKING_FAT_DECIDER
            .iter()
            .any(|candidate| last.starts_with(opening(candidate)))
        {
            return None;
        }
        if last.starts_with(opening(COOKING_FAT_CARD)) {
            return Some(compose::pick(COOKING_FAT_DECIDER, memory, ""));
        }
    }
    Some(COOKING_FAT_CARD)
}

#[cfg(test)]
#[test]
fn repeats_the_deciding_detail_once() {
    use crate::bot::Exchange;
    let mut memory = Memory::default();
    assert_eq!(reply(None), Some(COOKING_FAT_CARD));
    memory.turns.push(Exchange {
        user: "Should I cook onions in butter or oil?".into(),
        bot: COOKING_FAT_CARD.into(),
        module: "answers",
    });
    let again = reply(Some(&memory)).unwrap();
    assert!(COOKING_FAT_DECIDER.contains(&again));
    memory.turns[0].bot = again.into();
    assert_eq!(reply(Some(&memory)), None);
}
