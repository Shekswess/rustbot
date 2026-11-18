//! Give someone wording for a recurring behaviour that affects them.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Memory, Reading, compose, memory::ack};

#[path = "habit_data.rs"]
mod data;
use data::*;

type Pool = &'static [&'static str];

macro_rules! patterns {
    ($($pattern:ident),+ $(,)?) => {
        $(static $pattern: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$pattern).expect(stringify!($pattern)));)+
    };
}

patterns!(
    SOMEONE,
    RECURS,
    DONE_TO_ME,
    ASK_FOR,
    NOT_ASKED,
    UNSEEN,
    NOT_HELPED,
    MY_THING,
    MESS,
    CRITICISED,
    CRITICISED_MY,
    BAILS,
    KEEPS_THE_ARRANGEMENT,
    ALWAYS_DOES_IT,
    THANKED,
    COURTESY,
    COURTESY_DENIED,
    NOT_SINCERE,
    A_SHARED_OCCASION,
    ARRIVES_LATE,
    GOING_ON,
    EMAIL_HABIT,
    SAYS_ABOUT_ME,
    CREDIT,
);

static A_PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"\b(?:my|our|the)\s+(?:[a-z-]+\s+){{0,2}}?({})(?:s|es)?\b",
        PEOPLE.join("|")
    ))
    .expect("named person with a habit")
});
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").expect("habit words"));
static BOT_SELF: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:you|your|the bot|this bot)\b").expect("bot mention"));
static ASK_WORDING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:what|how)\b[^?]{0,24}\b(?:say|tell|word|phrase|put it|raise it)\b")
        .expect("asks for wording")
});
static FOLLOWUP_WORDING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:what|how)\b[^?]{0,30}\b(?:say|tell|word|phrase|put it|bring it up)\b")
        .expect("asks for new wording")
});
static A_PROMISE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:says? (?:she|he|they)'?ll|said (?:she|he|they)'?d|promised|says? (?:she|he|they) (?:will|would)|doesn'?t|didn'?t|won'?t)\b")
        .expect("promise")
});
static PRONOUN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:he|she|they|him|her|them)\b").expect("person in thread"));
static LEAVING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:leaves?|left|leaving|drops?|dropped|dumping|dumps|put|puts|putting)\b")
        .expect("makes a mess")
});

const OPENING_POOLS: &[Pool] = &[
    ASKED,
    TAKEN,
    MESSY,
    LEFT_OUT,
    TAKEN_FOR_GRANTED,
    CRITICISED_LINES,
    WORK_LINES,
    BAILING,
    LATE_LINES,
    SUBJECT_AGAIN,
    RAISE,
    ABOUT_ME,
    GOOD_HABIT_APPRECIATED,
    GOOD_HABIT_INCLUDED,
    WORDS_NOT_MEANT,
];

fn contains_phrase(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

fn opening_selected(memory: &Memory) -> bool {
    memory
        .turns
        .iter()
        .rev()
        .take(3)
        .find(|turn| !turn.bot.trim().is_empty())
        .is_some_and(|turn| {
            turn.module == "a_habit_is_raised"
                && OPENING_POOLS
                    .iter()
                    .flat_map(|pool| pool.iter())
                    .any(|reply| turn.bot == *reply)
        })
}

fn follow_up(line: &str, count: usize, memory: &Memory) -> Option<Pool> {
    if BOT_SELF.is_match(line) || !opening_selected(memory) {
        return None;
    }
    if contains_phrase(line, SELF_DOUBT) {
        return Some(DOUBT_POOL);
    }
    if count <= 12 && contains_phrase(line, ALWAYS_COUNT) {
        return Some(EVERY_TIME_POOL);
    }
    if FOLLOWUP_WORDING.is_match(line) {
        return None;
    }
    if count <= 20
        && (contains_phrase(line, THE_SAYING)
            || contains_phrase(line, AFTER_A_SAYING)
            || A_PROMISE.is_match(line))
    {
        return Some(SAID_IT_POOL);
    }
    (count <= 12 && !line.contains('?') && PRONOUN.is_match(line)).then_some(HOLD_THE_LINE)
}

fn pool(line: &str, memory: &Memory) -> Option<Pool> {
    let count = line.split_whitespace().count();
    if !(1..=30).contains(&count)
        || WORD
            .find_iter(line)
            .any(|word| NOT_THIS.contains(&word.as_str()))
    {
        return None;
    }
    if let Some(reply) = follow_up(line, count, memory) {
        return Some(reply);
    }

    let insincere = NOT_SINCERE.is_match(line);
    if !RECURS.is_match(line) && !insincere {
        return None;
    }
    let person = A_PERSON
        .captures(line)
        .and_then(|found| found.get(1))
        .or_else(|| SOMEONE.find(line))?
        .as_str();
    if WORD.find_iter(line).any(|word| {
        let word = word.as_str();
        NOT_A_GRUMBLE.contains(&word) || GOOD_WORDS.contains(&word)
    }) || line.ends_with('?') && !ASK_WORDING.is_match(line)
    {
        return None;
    }
    let at_work = WORK_PEOPLE.contains(&person);
    let work_topic = at_work && contains_phrase(line, WORK_WORDS);
    let affected = DONE_TO_ME.is_match(line);
    let taken = MY_THING.is_match(line);
    let mess = MESS.is_match(line);
    let excluded = NOT_ASKED.is_match(line);
    let unnoticed = UNSEEN.is_match(line);
    let no_help = NOT_HELPED.is_match(line);
    let criticism = CRITICISED_MY.is_match(line);
    let about_me = SAYS_ABOUT_ME.is_match(line);
    let credit = CREDIT.is_match(line);
    let strain = contains_phrase(line, STRAIN);
    let bails = BAILS.is_match(line);
    let late = ARRIVES_LATE.is_match(line);
    let keeps_on = GOING_ON.is_match(line);
    let email = EMAIL_HABIT.is_match(line);
    let courtesy = COURTESY.is_match(line);
    let recurring = ALWAYS_DOES_IT.is_match(line);

    if ![
        affected, taken, mess, excluded, unnoticed, no_help, criticism, about_me, credit, strain,
        insincere, bails, late, keeps_on, email, courtesy, work_topic, recurring,
    ]
    .contains(&true)
    {
        return None;
    }
    let request = ASK_FOR.is_match(line);
    let criticises = CRITICISED.is_match(line);
    if insincere {
        return Some(WORDS_NOT_MEANT);
    }
    if recurring && !bails && (A_SHARED_OCCASION.is_match(line) || !request)
        || courtesy
            && !COURTESY_DENIED.is_match(line)
            && !(bails || strain || criticises || excluded || unnoticed)
    {
        return Some(if THANKED.is_match(line) {
            GOOD_HABIT_APPRECIATED
        } else {
            GOOD_HABIT_INCLUDED
        });
    }
    if at_work && (work_topic || affected) {
        return Some(WORK_LINES);
    }
    if about_me {
        return Some(ABOUT_ME);
    }
    if criticises && (affected || criticism || strain) {
        return Some(CRITICISED_LINES);
    }
    if credit {
        return Some(CREDIT_LINES);
    }
    if excluded {
        return Some(LEFT_OUT);
    }
    if unnoticed {
        return Some(TAKEN_FOR_GRANTED);
    }
    if request {
        return Some(ASKED);
    }
    if mess && LEAVING.is_match(line) {
        return Some(MESSY);
    }
    if taken {
        return Some(TAKEN);
    }
    if no_help {
        return Some(RAISE);
    }
    if !KEEPS_THE_ARRANGEMENT.is_match(line) {
        if bails {
            return Some(BAILING);
        }
        if late {
            return Some(LATE_LINES);
        }
        if keeps_on && !CHILD_PEOPLE.contains(&person) {
            return Some(SUBJECT_AGAIN);
        }
    }
    if email && at_work {
        return Some(RAISE);
    }
    if affected {
        return Some(CRITICISED_LINES);
    }
    strain.then_some(RAISE)
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading
        .raw
        .to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let replies = pool(&line, memory)?;
    let seed_fallback = compose::fallback::respond(reading, memory)?;
    let placeholder = compose::fallback::placeholder(&seed_fallback)
        || compose::routes::hedge(&seed_fallback)
        || ack::reply(reading, memory).is_some_and(|receipt| receipt == seed_fallback);
    placeholder.then(|| compose::pick(replies, memory, &line).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn habit_is_raised_keeps_distinct_requests_and_selected_followups() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("My brother keeps asking me for money");
        assert!(
            turn.candidates
                .iter()
                .any(|offer| offer.module == "a_habit_is_raised"
                    && ASKED.contains(&offer.reply.as_str())),
            "{turn:?}"
        );
        let mut memory = Memory::default();
        assert!(pool("every single time", &memory).is_none());
        memory.turns.push(Exchange {
            user: "my brother keeps asking me for money".to_owned(),
            bot: ASKED[0].to_owned(),
            module: "a_habit_is_raised",
        });
        assert_eq!(pool("every single time", &memory), Some(EVERY_TIME_POOL));
        assert_eq!(
            pool("i already told him and he did it again", &memory),
            Some(SAID_IT_POOL)
        );
        assert!(pool("my brother keeps hitting me", &memory).is_none());
    }
}
