//! The speaker's own recent bereavement, and short remarks that still concern it.
use crate::bot::{
    Memory, Reading, compose, compose::loss_reader, memory::history::in_a_safety_thread,
};
use fancy_regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

include!("grief_data.rs");
pattern!(
    ACUTE_DISTRESS,
    r"\b(?:kill myself|harm myself|hurt myself|end my life|take my own life|want to die|better off dead|wish i (?:was|were) dead|self harm|self-harm)\b"
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::bot) enum LossStage {
    PersonDied,
    PetDied,
    PersonDying,
    PetDying,
}

pub(in crate::bot) struct Loss {
    who: String,
    turn: usize,
    stage: LossStage,
}

impl Loss {
    pub(in crate::bot) fn stage(&self) -> LossStage {
        self.stage
    }

    pub(in crate::bot) fn was_just_reported(&self, memory: &Memory) -> bool {
        memory.turns.len().checked_sub(1) == Some(self.turn)
    }

    fn recorded_here(text: &str, turn: usize) -> Option<Self> {
        let clean = words(text);
        if !loss_reader::reads_as_loss(&clean) {
            return None;
        }
        let (subject, pet) = loss_reader::own_subject(&clean)?;
        let stage = match (pet, loss_reader::is_dying_now(&clean)) {
            (false, false) => LossStage::PersonDied,
            (true, false) => LossStage::PetDied,
            (false, true) => LossStage::PersonDying,
            (true, true) => LossStage::PetDying,
        };
        Some(Self {
            who: format!("your {subject}"),
            turn,
            stage,
        })
    }
}

fn words(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .map(|word| word.trim_matches(['.', ',', '!', '?', ';', ':', '"', '\'', '(', ')']))
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(in crate::bot) fn observe(text: &str, memory: &mut Memory) {
    if let Some(loss) = Loss::recorded_here(text, memory.turns.len()) {
        memory.loss = Some(loss);
    }
}

pub(in crate::bot) fn remembered(memory: &Memory) -> Option<&Loss> {
    memory
        .loss
        .as_ref()
        .filter(|loss| memory.turns.len().saturating_sub(loss.turn) <= 3)
}

fn hit(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

pub(in crate::bot) fn may_answer(reading: &Reading<'_>, memory: &Memory) -> bool {
    !in_a_safety_thread(memory)
        && !memory.turns.last().is_some_and(|turn| {
            turn.module == "answers"
                && memory.last_knowledge == Some(("family", "child-cough"))
                && crate::bot::knowledge::lookup::child_cough_emergency(
                    &Reading::new(&turn.user),
                    memory,
                )
        })
        && !hit(&EMERGENCY, &reading.line.clean)
        && !hit(&ACUTE_DISTRESS, &reading.line.clean)
        && !super::safety::has_medical_urgency(reading, memory)
        && !super::safety::current_gas(&reading.line.clean)
}

fn age(text: &str) -> Option<u8> {
    let found = AGE_NUMBER.captures(text).ok().flatten()?;
    if let Some(age) = found.get(1).or_else(|| found.get(2)) {
        return age.as_str().parse().ok();
    }
    found
        .get(3)?
        .as_str()
        .parse::<u8>()
        .ok()
        .map(|decades| decades * 10)
}

fn age_as_word(age: u8) -> String {
    const WORDS: &[&str] = &[
        "0",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
        "Twenty",
    ];
    WORDS
        .get(usize::from(age))
        .map_or_else(|| age.to_string(), |word| (*word).to_owned())
}

fn pick(
    pool: &[&str],
    loss: &Loss,
    age: Option<(&str, u8)>,
    memory: &Memory,
    seed: &str,
) -> String {
    let who = &loss.who;
    let upper = format!("Y{}", &who[1..]);
    let filled: Vec<_> = pool
        .iter()
        .map(|reply| {
            let reply = reply.replace("{who}", who).replace("{Who}", &upper);
            if let Some((animal, age)) = age {
                reply
                    .replace("{animal}", animal)
                    .replace("{Age}", &age_as_word(age))
            } else {
                reply
            }
        })
        .collect();
    let pool: Vec<_> = filled.iter().map(String::as_str).collect();
    compose::pick(&pool, memory, seed).to_owned()
}

fn age_reply(text: &str, loss: &Loss, memory: &Memory) -> Option<String> {
    let age = age(text)?;
    let subject = loss.who.trim_start_matches("your ");
    let animal = ANIMAL_SPAN.iter().find(|(animal, _)| {
        subject == *animal
            || subject
                .strip_prefix(animal)
                .is_some_and(|tail| tail.starts_with(' '))
    });
    let (pool, names) = match animal {
        Some(&(animal, span)) if u16::from(age) * 5 < u16::from(span) * 4 => {
            (PET_YOUNG_REPLIES, Some((animal, age)))
        }
        Some(&(animal, _)) => (PET_OLD_REPLIES, Some((animal, age))),
        None if age < 70 => (YOUNG_AGE_REPLIES, None),
        None => (AGE_REPLIES, None),
    };
    Some(pick(pool, loss, names, memory, ""))
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let loss = remembered(memory)?;
    if matches!(loss.stage, LossStage::PersonDying | LossStage::PetDying)
        || !may_answer(reading, memory)
        || reading.features.about_bot
    {
        return None;
    }
    let clean = words(reading.raw);
    if clean.is_empty()
        || clean.split_whitespace().count() > 16
        || hit(&OTHER_PERSON, &clean)
        || loss_reader::reads_as_loss(&clean)
    {
        return None;
    }
    let sleep = hit(&SLEEP, &clean);
    if !sleep && compose::loss::pool(reading, memory).is_some()
        || reading.topics.iter().any(|(topic, _)| *topic == "health")
    {
        return None;
    }
    if [
        &*WORK_PRESSURE,
        &*WORK,
        &*TELL,
        &*TELLING,
        &*SAY_TO,
        &*REGISTER,
        &*COST,
        &*PROBATE,
        &*PAPERWORK,
        &*ADMIN,
        &*ARRANGEMENTS,
        &*BELONGINGS,
    ]
    .iter()
    .any(|pattern| hit(pattern, &clean))
    {
        return None;
    }
    if hit(&CANT_TALK, &clean) && !hit(&TALK_TO_SOMEONE, &clean) {
        return Some(pick(CANT_TALK_REPLIES, loss, None, memory, &clean));
    }
    for (pattern, pool) in [
        (&*FAMILY, FAMILY_REPLIES),
        (&*EXPECTED, EXPECTED_REPLIES),
        (&*SUDDEN, SUDDEN_REPLIES),
        (&*SLEEP, SLEEP_REPLIES),
        (&*HARD, HARD_REPLIES),
        (&*TALK_ABOUT, TALK_ABOUT_REPLIES),
        (&*GUILT, GUILT_REPLIES),
        (&*CRYING, CRYING_REPLIES),
    ] {
        if hit(pattern, &clean) {
            return Some(pick(pool, loss, None, memory, ""));
        }
    }
    if hit(&AGE, &clean) {
        return age_reply(&clean, loss, memory);
    }
    if hit(&SIGNS, &clean) {
        return None;
    }
    if hit(&MISSING, &clean) {
        return Some(pick(MISSING_REPLIES, loss, None, memory, ""));
    }
    if hit(&THANKS_LINE, &clean)
        && memory
            .turns
            .last()
            .is_some_and(|turn| matches!(turn.module, "loss" | "grief"))
    {
        return Some(pick(COMPANY_REPLIES, loss, None, memory, ""));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Bot;

    #[test]
    fn holds_an_own_loss_without_taking_one_belonging_to_someone_else() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("my friend's mum died");
        assert!(bot.memory.loss.is_none());
        bot.turn("our guinea pig died last night");
        let loss = remembered(&bot.memory).unwrap();
        assert_eq!(loss.who, "your guinea pig");
        assert_eq!(loss.stage(), LossStage::PetDied);
        bot.turn("my mum's cat died");
        assert_eq!(remembered(&bot.memory).unwrap().who, "your guinea pig");
        bot.turn("i bought a phone");
        bot.turn("it died the same day");
        assert!(remembered(&bot.memory).is_none());
    }

    #[test]
    fn early_followups_respond_to_the_same_loss() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(bot.turn("my dad died yesterday").module, Some("loss"));
        let sudden = bot.turn("it was really sudden");
        assert_eq!(sudden.module, Some("grief"));
        assert!(sudden.reply.starts_with("Sudden is the hardest kind"));
        let guilt = bot.turn("i should have visited more");
        assert_eq!(guilt.module, Some("grief"));
        assert!(guilt.reply.contains("your dad"));
        assert_eq!(
            bot.turn("thank you").reply,
            "I'm here. Say as much or as little as you like about your dad."
        );
    }

    #[test]
    fn the_composer_uses_the_recorded_stage() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("my dad is dying");
        assert_eq!(
            remembered(&bot.memory).unwrap().stage(),
            LossStage::PersonDying
        );
        let stage = bot.turn("what do i do now");
        assert_eq!(stage.module, Some("loss"));
        assert!(stage.reply.contains("ask the doctors what to expect"));
        let goodbye = bot.turn("how do i say goodbye to him");
        assert_eq!(goodbye.module, Some("loss"));
        assert!(goodbye.reply.starts_with("Skip the big speech"));

        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("our cats died this year");
        let stage = bot.turn("what do i do now");
        assert_eq!(stage.module, Some("loss"));
        assert!(stage.reply.contains("Ring the vet when you can face it"));
    }

    #[test]
    fn current_and_recent_emergencies_keep_their_routing() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("my dad died yesterday");
        let urgent = bot.turn("i cant sleep and i have crushing chest pain and cannot breathe");
        assert_eq!(urgent.module, Some("safety"));
        assert!(
            !urgent
                .candidates
                .iter()
                .any(|candidate| candidate.module == "grief")
        );

        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("my dad died yesterday");
        let child = bot.turn("my child has a cough and is struggling to breathe");
        assert_eq!(child.module, Some("answers"));
        assert!(child.reply.contains("999"));
        assert!(
            !bot.turn("i cant sleep")
                .candidates
                .iter()
                .any(|candidate| candidate.module == "grief")
        );
    }
}
