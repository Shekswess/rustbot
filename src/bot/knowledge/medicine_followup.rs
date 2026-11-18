//! Retain the approved missed-pill referral when the next question asks for a deadline.

use crate::bot::{Memory, Reading};
use std::borrow::Cow;

pub(crate) fn missed_pill_timing(
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<Cow<'static, str>> {
    let last = memory.turns.last()?;
    if last.module != "answers"
        || !matches!(
            reading.clean.trim_end_matches(['?', '.', '!']).trim_end(),
            "how long do i have"
                | "how much time do i have"
                | "when is it too late"
                | "when would it be too late"
        )
    {
        return None;
    }
    let card = super::lookup::by_id("science", "missed-contraceptive-pill")?;
    let prior = crate::bot::memory::unframed_reply(&last.bot).unwrap_or(&last.bot);
    (prior == card.reply).then_some(card.reply)
}

#[cfg(test)]
mod tests {
    use crate::bot::{Bot, knowledge::lookup};

    #[test]
    fn a_bare_deadline_ask_keeps_the_missed_pill_referral() {
        let referral = lookup::by_id("science", "missed-contraceptive-pill").unwrap();
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(
            bot.turn("I forgot my contraceptive pill last night. What should I do?")
                .reply,
            referral.reply
        );
        let followup = bot.turn("How long do I have?");
        assert_eq!(followup.module, Some("answers"));
        assert_eq!(
            crate::bot::memory::unframed_reply(&followup.reply).unwrap_or(&followup.reply),
            referral.reply
        );

        bot.turn("What's the capital of Spain?");
        assert_ne!(bot.turn("How long do I have?").reply, referral.reply);
        bot.reset().unwrap();
        assert_ne!(bot.turn("How long do I have?").reply, referral.reply);
    }
}
