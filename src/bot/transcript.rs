use super::{Exchange, compose};
use std::{collections::HashMap, ops::Deref};

/// Complete exchanges with an exact first-occurrence index over normalized replies.
#[derive(Default)]
pub(super) struct Transcript {
    exchanges: Vec<Exchange>,
    first_replies: Option<HashMap<Box<str>, usize>>,
}

impl Transcript {
    pub(super) fn record(&mut self, exchange: Exchange) {
        let first_replies = self.first_replies.get_or_insert_with(|| {
            let mut first_replies = HashMap::new();
            for (index, turn) in self.exchanges.iter().enumerate() {
                first_replies
                    .entry(compose::said_again(&turn.bot).into_boxed_str())
                    .or_insert(index);
            }
            first_replies
        });
        first_replies
            .entry(compose::said_again(&exchange.bot).into_boxed_str())
            .or_insert(self.exchanges.len());
        self.exchanges.push(exchange);
    }

    pub(super) fn first_reply(&self, reply: &str) -> Option<usize> {
        if self.exchanges.is_empty() {
            return None;
        }
        let normalized = compose::said_again(reply);
        if let Some(first_replies) = &self.first_replies {
            return first_replies.get(normalized.as_str()).copied();
        }
        self.exchanges
            .iter()
            .position(|turn| compose::said_again(&turn.bot) == normalized)
    }
}

impl Deref for Transcript {
    type Target = Vec<Exchange>;

    fn deref(&self) -> &Self::Target {
        &self.exchanges
    }
}

impl<'a> IntoIterator for &'a Transcript {
    type Item = &'a Exchange;
    type IntoIter = std::slice::Iter<'a, Exchange>;

    fn into_iter(self) -> Self::IntoIter {
        self.exchanges.iter()
    }
}

// Fixtures may build or edit the raw history. Index it on the next recorded
// exchange, and use the original exact scan until then.
#[cfg(test)]
impl std::ops::DerefMut for Transcript {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.first_replies = None;
        &mut self.exchanges
    }
}

#[cfg(test)]
impl From<Vec<Exchange>> for Transcript {
    fn from(exchanges: Vec<Exchange>) -> Self {
        Self {
            exchanges,
            first_replies: None,
        }
    }
}

#[cfg(test)]
impl FromIterator<Exchange> for Transcript {
    fn from_iter<T: IntoIterator<Item = Exchange>>(iter: T) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    fn exchange(reply: &str) -> Exchange {
        Exchange {
            user: "A test input".into(),
            bot: reply.into(),
            module: "answers",
        }
    }

    #[test]
    fn retains_earliest_equal_reply_through_a_long_transcript() {
        let mut history = Transcript::from(vec![exchange("SHE\u{00a0}called them."), exchange("")]);
        assert_eq!(history.first_reply("He called her. "), Some(0));
        for number in 0..4096 {
            history.record(exchange(&format!("Distinct reply {number}.")));
        }
        history.record(exchange("He called him."));
        assert_eq!(history.len(), 4099);
        assert_eq!(history.first_reply("They called him."), Some(0));
        assert_eq!(history.first_reply(" \t"), Some(1));
        assert_eq!(history.first_reply("Distinct reply 4095."), Some(4097));
        assert_eq!(history.first_reply("They called him!"), None);
        assert_eq!(history[0].bot, "SHE\u{00a0}called them.");
    }

    #[test]
    fn replacing_fixture_replies_discards_stale_keys_before_rebuilding() {
        let mut history = Transcript::default();
        history.record(exchange("Old wording."));
        history[0].bot = "Replacement wording.".into();
        assert_eq!(history.first_reply("Old wording."), None);
        assert_eq!(history.first_reply("Replacement wording."), Some(0));
        history.record(exchange("REPLACEMENT wording."));
        assert_eq!(history.first_reply("REPLACEMENT wording."), Some(0));
        assert_eq!(history.first_replies.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn resetting_a_live_bot_clears_replies_and_their_index() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let reply = bot.turn("What is the capital of Italy?").reply;
        assert_eq!(bot.memory.turns.first_reply(&reply), Some(0));
        bot.reset().unwrap();
        assert!(bot.memory.turns.is_empty());
        assert_eq!(bot.memory.turns.first_reply(&reply), None);
        assert_eq!(bot.turn("What is the capital of Italy?").reply, reply);
    }
}
