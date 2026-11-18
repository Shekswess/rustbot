//! Ordinary family bereavement and adjacent follow-ups from bot/compose/loss.py.
use crate::bot::modules::grief::{self, LossStage};
use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

include!("loss_stage_data.rs");
pattern!(
    MISSING_PRONOUN,
    r"^(?:i miss|i'?m missing|i am missing) (?:her|him|them)[.!]*$"
);
pattern!(
    SAY_GOODBYE,
    r"^how (?:do|can|should) (?:i|we) say good ?bye(?: to (?:him|her|them))?[.!]*$"
);
pattern!(
    PET_DYING_REPORT,
    concat!(
        r"^(?:(?:the|my|our) vet (?:says|said|told (?:me|us))(?: that)? )?",
        r"(?:my|our) (?:[a-z][a-z'-]* ){1,3}(?:",
        r"(?:is|are) (?:dying|at the end(?: of (?:his|her|their|its) life)?|",
        r"(?:being|going to be) put (?:down|to sleep)(?: (?:today|tonight|tomorrow|soon))?)",
        r"|(?:has|have) (?:only |about )?(?:a few|one|two|three|four|\d+) (?:days?|weeks?|months?) left",
        r")[.!]*$"
    )
);
static HELLO_ON_ITS_OWN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?:hi+|hey+|hello+|hiya+|yo|howdy|morning|good morning|afternoon|good afternoon|evening|good evening|greetings)\b[\s,!.]*(?:(?:again|there|all|everyone|everybody|folks|you|mate|love|both)\b[\s,!.]*)*(?:(?:how are (?:you|things|you doing|you keeping)|how'?s it going|hows it going|how are we|you ok|you alright|hope you'?re well|its me)\b[\s,.!?]*)*$"#).unwrap()
});
const LOSS_REPLIES: &[&str] = &[
    "I'm so sorry to hear that. It's a hard thing to carry — I'm here if you want to talk about it, or if anything practical would help.",
    "I'm really sorry. Be gentle with yourself today, and tell me if there's anything you want to talk through.",
    "That's such sad news, I'm sorry. Some days will be heavier than others — I'm here if you want to talk about it, or about anything else.",
    "I'm sorry, that's a real loss. Take today slowly, and shout if there's anything I can help you sort out.",
];
const AFTER_REPLIES: &[&str] = &[
    "I'm here. Take your time with it — tell me what it has been like, if you want to.",
    "There's no rush to talk about it. I'm here when you want to say more.",
    "That sounds heavy to be carrying. I'm listening, however much or little you want to say.",
    "I'm glad you said that out loud. Say as much or as little as you like — I'm here.",
];
const HELLO_AFTER: &[&str] = &[
    "Hello again. I'm still here - how are you doing today?",
    "Hi. No need to say anything in particular - how are you holding up?",
    "Hello. I'm here whenever you want to talk about it, or about anything else.",
    "Hi again. How has the day been?",
];

// Keep this reader on the uncomplicated disclosure. Other contexts (a wish, a
// hypothetical, the person dying now, an animal, or a question) have different upstream readers.
static FAMILY_DEATH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?:(?:sadly|unfortunately|well|so|actually)[ ,]+)?(?:my|our) ",
        r"(?:(?:older|younger|little|big|beloved|best|closest|elder|elderly) )?",
        r"(?:mum|mom|mam|dad|mother|father|brother|sister|son|daughter|wife|husband|partner|",
        r"nan|nana|grandad|granddad|grandma|grandpa|granny|gran|uncle|aunt|auntie|cousin|friend) ",
        r"(?:(?:has|had|just|recently|suddenly|sadly) )*(?:died|passed(?: away| on)?)",
        r"(?: (?:last (?:night|week|month|year|winter|summer)|yesterday|today|recently|",
        r"in (?:january|february|march|april|may|june|july|august|september|october|november|december|\d{4})|",
        r"(?:a|one|two|three|\d+) (?:days?|weeks?|months?|years?) ago))?[.!]*$"
    )).unwrap()
});
static ABOUT_THEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
    r"\b(?:miss(?:ing)? (?:him|her|them)|(?:hard|difficult) to (?:talk|think|speak) about (?:him|her|them)|",
    r"can(?:'?t|not| not) stop (?:crying|thinking about (?:him|her|them))|",
    r"keep thinking about (?:him|her|them)|still (?:cry|crying))\b"
)).unwrap()
});

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let pool = pool(reading, memory)?;
    if pool == SOMEONE_DYING {
        let said_helpers = memory.turns.iter().any(|turn| HELPERS.is_match(&turn.bot));
        let fresh: Vec<_> = pool
            .iter()
            .copied()
            .filter(|reply| {
                !(said_helpers && HELPERS.is_match(reply))
                    && !memory.turns.iter().any(|turn| turn.bot.trim() == *reply)
            })
            .collect();
        if !fresh.is_empty() {
            return Some(compose::pick(&fresh, memory, reading.raw).to_owned());
        }
    }
    Some(compose::pick(pool, memory, reading.raw).into())
}

pub(in crate::bot) fn pool(
    reading: &Reading<'_>,
    memory: &Memory,
) -> Option<&'static [&'static str]> {
    if recognizes(&reading.clean) {
        Some(LOSS_REPLIES)
    } else if PET_DYING_REPORT.is_match(&reading.clean)
        && super::loss_reader::own_subject(&reading.clean).is_some_and(|(_, pet)| pet)
        && super::loss_reader::is_dying_now(&reading.clean)
        && grief::may_answer(reading, memory)
    {
        Some(THE_DAYS_BEFORE)
    } else if OPEN_ASK.is_match(&reading.clean) && grief::may_answer(reading, memory) {
        let loss = grief::remembered(memory)?;
        match loss.stage() {
            LossStage::PersonDying | LossStage::PetDying
                if loss.was_just_reported(memory)
                    || memory
                        .turns
                        .last()
                        .is_some_and(|turn| turn.module == "loss") =>
            {
                Some(if loss.stage() == LossStage::PetDying {
                    THE_DAYS_BEFORE
                } else {
                    SOMEONE_DYING
                })
            }
            LossStage::PersonDied | LossStage::PetDied
                if memory.turns.last().is_some_and(|turn| {
                    matches!(turn.module, "loss" | "grief")
                        || !matches!(turn.module, "" | "greeting" | "safety")
                            && super::loss_reader::reads_as_loss(&turn.user)
                }) =>
            {
                Some(if loss.stage() == LossStage::PersonDied {
                    FIRST_DAYS_PERSON
                } else {
                    FIRST_DAYS_PET
                })
            }
            _ => None,
        }
    } else if MISSING_PRONOUN.is_match(&reading.clean)
        && memory
            .turns
            .last()
            .is_some_and(|turn| turn.module == "grief")
        && grief::remembered(memory).is_some()
        && grief::may_answer(reading, memory)
    {
        Some(MISS_REPLIES)
    } else if SAY_GOODBYE.is_match(&reading.clean)
        && !reading.raw.contains('?')
        && grief::remembered(memory).is_some_and(|loss| {
            loss.stage() == LossStage::PersonDying
                && (loss.was_just_reported(memory)
                    || memory
                        .turns
                        .last()
                        .is_some_and(|turn| turn.module == "loss"))
        })
        && grief::may_answer(reading, memory)
    {
        Some(DYING_SAY_REPLIES)
    } else if memory
        .turns
        .last()
        .is_some_and(|turn| turn.module == "loss")
    {
        if HELLO_ON_ITS_OWN.is_match(reading.clean.trim_end_matches(['.', '!', '?'])) {
            Some(HELLO_AFTER)
        } else if !reading.clean.contains('?') && ABOUT_THEM.is_match(&reading.clean) {
            Some(AFTER_REPLIES)
        } else {
            None
        }
    } else {
        None
    }
}

pub(super) fn recognizes(line: &str) -> bool {
    FAMILY_DEATH.is_match(line) && super::loss_reader::reads_as_loss(line)
}

#[cfg(test)]
mod tests {
    use crate::Bot;

    #[test]
    fn the_first_open_question_after_a_death_gets_registration_guidance() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(bot.turn("my dad died yesterday").module, Some("loss"));
        let turn = bot.turn("What do I do now?");
        assert_eq!(turn.module, Some("loss"), "{turn:?}");
        assert!(turn.reply.to_lowercase().contains("register"), "{turn:?}");
        assert!(turn.reply.contains("GOV.UK"), "{turn:?}");
    }

    #[test]
    fn first_days_guidance_yields_to_named_questions_and_a_changed_thread() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("my dad died yesterday");
        let named = bot.turn("How long does probate take?");
        assert_eq!(named.module, Some("answers"), "{named:?}");

        for (aside, module) in [
            ("What is the capital of Italy?", "answers"),
            ("I have crushing chest pain and cannot breathe", "safety"),
        ] {
            bot.reset().unwrap();
            bot.turn("my dad died yesterday");
            let other = bot.turn(aside);
            assert_eq!(other.module, Some(module), "{other:?}");
            let next = bot.turn("What do I do now?");
            assert!(
                !next
                    .candidates
                    .iter()
                    .any(|candidate| candidate.module == "loss"),
                "{next:?}"
            );
        }
    }

    #[test]
    fn pet_dying_open_followup_stays_with_the_vet() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let report = bot.turn("my dog is dying");
        assert_eq!(report.module, Some("loss"), "{report:?}");
        assert_eq!(
            super::grief::remembered(&bot.memory).map(|loss| loss.stage()),
            Some(super::LossStage::PetDying),
            "{report:?}"
        );
        let next = bot.turn("What do I do now?");
        assert_eq!(next.module, Some("loss"), "{next:?}");
        assert!(next.reply.contains("vet"), "{next:?}");
        bot.reset().unwrap();
        let prognosis = bot.turn("The vet says my cat has a few weeks left.");
        assert_eq!(prognosis.module, Some("loss"), "{prognosis:?}");
        let next = bot.turn("Where do I start?");
        assert_eq!(next.module, Some("loss"), "{next:?}");
        assert!(next.reply.contains("vet"), "{next:?}");
    }

    #[test]
    fn pet_dying_followup_yields_to_breathing_distress_and_new_subjects() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("my dog is dying");
        let urgent = bot.turn("My dog is struggling to breathe");
        assert_eq!(urgent.module, Some("safety"), "{urgent:?}");
        assert!(urgent.reply.contains("vet now"), "{urgent:?}");
        assert!(!urgent.reply.contains("999"), "{urgent:?}");
        let next = bot.turn("What do I do now?");
        assert_eq!(next.module, Some("safety"), "{next:?}");
        assert!(next.reply.contains("vet now"), "{next:?}");

        bot.reset().unwrap();
        bot.turn("my dog is dying");
        bot.turn("What is the capital of Italy?");
        let next = bot.turn("Where do I start?");
        assert!(
            !next
                .candidates
                .iter()
                .any(|candidate| candidate.module == "loss"),
            "{next:?}"
        );
    }
}
