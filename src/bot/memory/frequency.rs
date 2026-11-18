//! Read a bare frequency against the trouble in the preceding turn.

use super::{again, measurement};
use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        pub(super) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($value).expect(stringify!($name)));
    };
}

#[path = "frequency_data.rs"]
mod data;
use data::*;

fn clean(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn pool(line: &str, memory: &Memory) -> Option<Vec<String>> {
    let frequency = FREQUENCY_ONLY.captures(line)?;
    let last = memory.turns.last()?;
    if memory.urgent_medical.is_some() || matches!(last.module, "safety" | "urgent") {
        return None;
    }
    let said = clean(&last.user);
    if said.is_empty()
        || WORRY.is_match(&said)
        || NOT_MY_NIGHT.is_match(&said)
        || memory
            .turns
            .iter()
            .rev()
            .take(3)
            .any(|turn| LOSS.is_match(&turn.user.to_lowercase()))
    {
        return None;
    }

    static NIGHT_PHRASE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"\b(?:night|nights|morning|mornings|nightly|all\s+(?:the\s+)?time|constantly)\b",
        )
        .unwrap()
    });
    let frequency = frequency[1].trim();
    let night = NIGHT_PHRASE.is_match(frequency) && MY_NIGHT.is_match(&said);
    if !night && !(SYMPTOM.is_match(&said) || compose::trouble::reads_like_symptom(&said)) {
        return None;
    }
    static ASKED_LENGTH: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)\b(?:how long|how often|weeks?|months?)\b").unwrap());
    let options = match (night, ASKED_LENGTH.is_match(&last.bot)) {
        (true, true) => NIGHT_FREQUENCY_KEPT,
        (true, false) => NIGHT_FREQUENCY,
        (false, true) => BODY_FREQUENCY_KEPT,
        (false, false) => BODY_FREQUENCY,
    };
    let mut letters = frequency.chars();
    let first = letters.next()?;
    let often = first.to_uppercase().chain(letters).collect::<String>();
    Some(
        options
            .iter()
            .map(|line| line.replace("{often}", &often))
            .collect(),
    )
}

pub(in crate::bot) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = clean(reading.raw);
    if [&*END_OF_ROPE, &*OTHER_AWAKE, &*LOSS]
        .iter()
        .any(|pattern| pattern.is_match(&line))
    {
        return None;
    }
    let pool = pool(&line, memory)?;
    let pool: Vec<_> = pool.iter().map(String::as_str).collect();
    Some(compose::pick(&pool, memory, "").to_owned())
}

pub(in crate::bot) fn is_frequency_reply(raw: &str, reply: &str, memory: &Memory) -> bool {
    pool(&clean(raw), memory).is_some_and(|pool| pool.iter().any(|line| line == reply))
}

pub(in crate::bot) fn answers_owns_frequency(reply: &str, memory: &Memory) -> bool {
    let reply = clean(reply);
    if reply.is_empty() || _A_CHILD_OR_THEM.is_match(&reply) || reply.starts_with("same answer") {
        return false;
    }
    !memory.turns.last().is_some_and(|last| {
        let prior: String = clean(&last.bot).chars().take(60).collect();
        !prior.is_empty() && reply.contains(&prior)
    })
}

pub(in crate::bot) fn behind_frequency(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !memory.frequency_reply
        || !reading.reference.followup
        || reading.line.change_of_subject
        || reading.context_fields.subject.is_some()
        || reading.line.asking.iter().any(|ask| ask == "recommend")
        || reading
            .line
            .words
            .iter()
            .any(|word| A_CALL_ASK.contains(&word.as_str()))
    {
        return None;
    }
    let anchor = memory.turns.iter().rev().nth(1)?;
    if anchor.module != "answers" {
        return None;
    }
    let answer = anchor.bot.trim();
    let answer = again::unframed(answer).unwrap_or(answer);
    let sentences = measurement::sentences(answer);
    let first = *sentences.first()?;
    let short = if first.chars().count() < 60 && sentences.len() > 1 {
        sentences[..2].join(" ")
    } else {
        first.to_owned()
    };
    Some(format!("Same as before: {short}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn records_only_a_selected_frequency_and_reuses_the_earlier_card_once() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let answers = bot.turn("i have a cough");
        assert_eq!(answers.module, Some("answers"));
        let frequency = bot.turn("its every night");
        assert_eq!(frequency.module, Some("chat"));
        assert_eq!(
            frequency.reply,
            "Every night, then. That is enough to get it looked at if it keeps on, and a note of when it is worst helps."
        );
        assert!(bot.memory.frequency_reply);

        let followup = bot.turn("what do i do now?");
        assert_eq!(followup.module, Some("answers"));
        let initial = measurement::sentences(&answers.reply)[0];
        assert_eq!(followup.reply, format!("Same as before: {initial}"));
        assert!(!bot.memory.frequency_reply);
    }

    #[test]
    fn picks_the_sleep_pool_and_excludes_another_person_and_recent_loss() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("i keep waking up at 3am");
        let response = bot.turn("its every night");
        assert_eq!(response.module, Some("chat"));
        assert_eq!(
            response.reply,
            "Every night is a lot. How long has it been like that?"
        );
        assert!(bot.memory.frequency_reply);

        let reading = Reading::new("its every night");
        for user in [
            "my son keeps waking up",
            "i keep waking up worrying about work",
        ] {
            let mut memory = Memory::default();
            memory.turns.push(Exchange {
                user: user.into(),
                bot: "What happens next?".into(),
                module: "answers",
            });
            assert_eq!(reply(&reading, &memory), None, "{user}");
        }
        bot.reset().unwrap();
        bot.turn("my mum died last week");
        bot.turn("i keep waking up at 3am");
        assert!(
            !bot.turn("its every night")
                .candidates
                .iter()
                .any(|c| c.module == "chat")
        );
        assert!(!bot.memory.frequency_reply);
    }

    #[test]
    fn approved_exception_does_not_record_a_frequency_after_emergency() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(
            bot.turn("i have severe chest pain and cant breathe").module,
            Some("safety")
        );
        let turn = bot.turn("its every night");
        assert_eq!(turn.module, Some("safety"));
        assert!(!turn.candidates.iter().any(|c| c.module == "chat"));
        assert!(!bot.memory.frequency_reply);
    }
}
