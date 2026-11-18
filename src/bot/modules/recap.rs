//! Follow-up questions that ask where the conversation left off.

use std::sync::LazyLock;

use fancy_regex::Regex;

use crate::bot::{Memory, Reading, memory::thread};

static WANTED_AGAIN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\bwhat (?:were|are|was) (?:we|i|you) (?:just )?(?:talking|chatting|saying|on) about\b|",
        r"\bwhat (?:were|are|was) (?:we|i|you) (?:just )?saying\b|",
        r"\bwhat we were (?:just )?(?:talking|chatting|saying) about(?=\s*(?:[?.!,]|$|again\b|earlier\b|before\b|just now\b))|",
        r"\bwhat we were (?:just )?saying(?=\s*(?:[?.!,]|$|again\b|earlier\b|before\b))|",
        r"\bwhere (?:were we|are we)(?=\s*(?:[?.!,]|$|up to\b|now\b|at\b))|",
        r"\bwhere (?:i|we) (?:were|was|are|got to|left off|ended up)\b(?=\s*(?:[?.!,]|$|up to\b|now\b|again\b))|",
        r"\bwhat (?:were|was) (?:we|i) (?:looking at|doing)\b(?=\s*(?:[?.!,]|$|again\b|earlier\b|before\b))|",
        r"\bwhat we were (?:just )?(?:looking at|doing)\b(?=\s*(?:[?.!,]|$|again\b|earlier\b|before\b))|",
        r"\bback to (?:my|your|the) question\b(?=\s*(?:[?.!,]|$))"
    )).expect("valid recap query")
});

pub(super) fn asks_about_chat(text: &str) -> bool {
    WANTED_AGAIN.is_match(text).unwrap_or(false)
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if reading.features.about_bot || !asks_about_chat(&reading.line.clean) {
        return None;
    }
    let recap = thread::recap(memory, asks_about_chat);
    let options = match (recap.subject, recap.asked, recap.said) {
        (Some(topic), Some(ask), _) => vec![
            format!("We were on {topic} — you asked {ask}. Want to carry on?"),
            format!("Still on {topic}: you asked {ask}."),
            format!("The last thing we got into was {topic}, and you asked {ask}."),
        ],
        (_, Some(ask), Some(said)) => {
            let initial: String = said.chars().take(1).flat_map(char::to_uppercase).collect();
            let tail = said.chars().skip(1).collect::<String>();
            vec![
                format!("You said {said}. You asked {ask}. Want to carry on?"),
                format!("You said {said} - and you asked {ask}."),
                format!("{initial}{tail}. Your question was {ask}."),
            ]
        }
        (_, Some(ask), None) => vec![
            format!("You asked {ask}. Want to carry on?"),
            format!("You were asking {ask}."),
        ],
        (_, None, Some(said)) => vec![
            format!("You said {said}. Want to carry on?"),
            format!("You last said {said}."),
            format!("You'd told me {said}."),
            format!("Last thing you said was {said}."),
        ],
        (Some(topic), None, None) => {
            vec![format!("We were on {topic}."), format!("Still {topic}.")]
        }
        (None, None, None) => return None,
    };
    let options: Vec<_> = options.iter().map(String::as_str).collect();
    Some(thread::choose_recap(&options, memory))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bot;

    #[test]
    fn recap_skips_its_own_questions_and_keeps_the_prior_topic() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert!(respond(&Reading::new("where were we up to?"), &Memory::default()).is_none());
        bot.turn("I'm making a birthday cake");
        bot.turn("How long should I bake it?");
        let first = bot.turn("Remind me where we got to");
        let second = bot.turn("Back to my question");
        for turn in [&first, &second] {
            let reply = &turn
                .candidates
                .iter()
                .find(|item| item.module == "recap")
                .expect("recap proposal")
                .reply;
            assert!(
                reply.contains("cake") && reply.contains("how long you should bake it"),
                "{reply}"
            );
            assert!(!reply.contains("remind me"));
        }
        assert_ne!(first.reply, second.reply);
    }
}
