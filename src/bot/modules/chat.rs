//! Conversation openings, greetings, and simple social acknowledgements.
use crate::bot::{Memory, Reading, compose};
#[path = "chat_course_want.rs"]
mod course_want;
#[path = "chat_row.rs"]
mod row;
#[path = "chat_trip_intention.rs"]
mod trip_intention;
pub(super) use course_want::scheduled_course;
pub(in crate::bot) use row::Row;

pub(in crate::bot) fn observe(reading: &Reading<'_>, memory: &mut Memory) {
    row::observe(reading, memory);
}

pub(in crate::bot) fn record_selected(reading: &Reading<'_>, reply: &str, memory: &mut Memory) {
    row::record_selected(reading, reply, memory);
}

pub(super) fn row_yields_to_teenager_card(
    reading: &Reading<'_>,
    reply: &str,
    memory: &Memory,
) -> bool {
    row::yields_to_teenager_card(reading, reply, memory)
}

pub(crate) fn after(text: &str, reply: &str, memory: &mut Memory) {
    memory.frequency_reply = crate::bot::memory::frequency::is_frequency_reply(text, reply, memory);
}

const GREET: &[&str] = &[
    "Hi! What's on your mind?",
    "Morning! How's your day going?",
    "Hello! What can I help with?",
    "Hey! What are we working on?",
    "Hi there — what do you need?",
];
const CHECK_IN: &[&str] = &[
    "Hi — still here. Where were we?",
    "Hello again. What's next?",
    "Hi. Anything changed since we last spoke?",
    "Still here, if you want to carry on.",
    "Hello again — carry on where we left off?",
];
const CHECK_IN_NAME: &[&str] = &[
    "Hi {name} — still here. Where were we?",
    "{name} — hello again. What's next?",
    "Hi {name}. Anything changed since we last spoke?",
];
const MEET: &[&str] = &[
    "Hi {name} — good to meet you. What's on your mind?",
    "Nice to meet you, {name}. What can I help with?",
    "Hello {name}! What are we doing today?",
];
const THANKS: &[&str] = &[
    "Any time.",
    "Glad that helped.",
    "You're welcome — shout if anything else comes up.",
    "No problem at all.",
    "Happy to help.",
    "No worries at all.",
];
const BYE: &[&str] = &[
    "Bye! Take care.",
    "See you — good luck with it.",
    "Take care. Come back any time.",
    "Bye for now!",
    "Take it easy!",
];
const NIGHT: &[&str] = &[
    "Night! Sleep well.",
    "Goodnight — sleep well.",
    "Sleep well. I'll be here tomorrow.",
    "Night then. Sleep well.",
];
const HOW_ARE_YOU: &[&str] = &[
    "Doing well, thanks — what about you?",
    "All good here. What's keeping you busy?",
    "Can't complain. What's on your mind?",
];
const HOW_ARE_YOU_MID: &[&str] = &[
    "Still here, thanks — how are you getting on?",
    "Doing fine, thanks. How's your day going?",
    "All good here. What about you?",
];
const HELP: &[&str] = &[
    "Happy to help. What's the thing you're stuck on?",
    "Of course. Tell me what you need a hand with.",
    "Say what it is you're stuck on and we'll take it from there.",
    "Right — what is it you need help with?",
];
const ACK: &[&str] = &[
    "Sounds good.",
    "Alright then — good luck with it.",
    "Nice one.",
    "Fair enough.",
    "Hope it works out.",
];

fn greeting_word(word: &str) -> bool {
    matches!(
        word,
        "hi" | "hiya"
            | "hey"
            | "heya"
            | "hello"
            | "yo"
            | "howdy"
            | "morning"
            | "afternoon"
            | "evening"
            | "greetings"
            | "hallo"
            | "helo"
    ) || word
        .strip_prefix("hi")
        .is_some_and(|rest| rest.bytes().all(|b| b == b'i'))
        || word
            .strip_prefix("he")
            .is_some_and(|rest| !rest.is_empty() && rest.bytes().all(|b| b == b'y'))
}

fn choose(pool: &[&str], memory: &Memory, name: Option<&str>, seed_text: &str) -> String {
    if let Some(name) = name {
        let options: Vec<_> = pool
            .iter()
            .map(|line| line.replace("{name}", name))
            .collect();
        let refs: Vec<_> = options.iter().map(String::as_str).collect();
        compose::pick(&refs, memory, seed_text).into()
    } else {
        compose::pick(pool, memory, seed_text).into()
    }
}

fn thanks_only(words: &[&str]) -> bool {
    const CORE: &[&str] = &[
        "thanks",
        "thank",
        "thx",
        "ty",
        "cheers",
        "ta",
        "appreciated",
        "appreciate",
        "thnx",
        "thks",
    ];
    const FILLER: &[&str] = &[
        "one",
        "nice",
        "try",
        "ill",
        "i'll",
        "will",
        "give",
        "go",
        "it",
        "lets",
        "let's",
        "today",
        "tomorrow",
        "later",
        "now",
        "anything",
        "else",
        "that",
        "this",
        "was",
        "is",
        "brill",
        "yeah",
        "yep",
        "earlier",
        "before",
        "sorted",
        "worked",
        "works",
        "far",
        "helped",
        "super",
        "legend",
        "ace",
        "so",
        "much",
        "again",
        "for",
        "you",
        "u",
        "very",
        "a",
        "lot",
        "the",
        "help",
        "thats",
        "that's",
        "its",
        "it's",
        "they",
        "were",
        "are",
        "im",
        "i'm",
        "i",
        "am",
        "welcome",
        "well",
        "good",
        "okay",
        "alright",
        "helping",
        "perfect",
        "great",
        "brilliant",
        "lovely",
        "awesome",
        "amazing",
        "excellent",
        "handy",
        "helpful",
        "really",
        "too",
        "all",
        "and",
        "ok",
        "man",
        "mate",
        "friend",
        "idea",
        "do",
        "grand",
        "chill",
        "content",
        "mint",
        "sound",
    ];
    words.iter().any(|word| CORE.contains(word))
        && words
            .iter()
            .all(|word| CORE.contains(word) || FILLER.contains(word))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(reply) = crate::bot::memory::meant::respond(reading, memory) {
        return Some(reply);
    }
    let words = crate::bot::understanding::normalize::tokens(reading.raw);
    let words: Vec<_> = words.iter().map(String::as_str).collect();
    let joined = words.join(" ");
    let name = memory.name.as_deref();

    if memory.introduced_now {
        return Some(choose(MEET, memory, name, ""));
    }
    if let Some(reply) = memory.personal.respond_home(memory) {
        return Some(reply);
    }
    if let Some(reply) = row::respond(reading, memory) {
        return Some(reply);
    }
    if let Some(reply) = super::chat_someone::status_reply(reading, memory) {
        return Some(reply);
    }
    if let Some(reply) = crate::bot::memory::frequency::reply(reading, memory) {
        return Some(reply);
    }
    if reading.context_fields.presence || reading.context_fields.opening {
        return super::chat_presence::opening(reading, memory);
    }
    let (pool, name): (&[&str], Option<&str>) = if words.iter().any(|word| greeting_word(word))
        && words.iter().all(|word| {
            greeting_word(word)
                || matches!(
                    *word,
                    "good"
                        | "day"
                        | "all"
                        | "everyone"
                        | "folks"
                        | "guys"
                        | "team"
                        | "there"
                        | "again"
                        | "x"
                        | "everybody"
                        | "you"
                        | "lot"
                        | "the"
                )
        }) {
        match () {
            _ if words.contains(&"morning") => (
                &[
                    "Morning! How's your day going?",
                    "Morning — what's on your mind?",
                    "Morning! What can I help with?",
                ],
                None,
            ),
            _ if words.contains(&"afternoon") => (
                &[
                    "Afternoon! What's on your mind?",
                    "Afternoon — what can I help with?",
                    "Afternoon! How's your day going?",
                ],
                None,
            ),
            _ if words.contains(&"evening") => (
                &[
                    "Evening! What's on your mind?",
                    "Evening — what can I help with?",
                    "Evening! How's your day been?",
                ],
                None,
            ),
            _ if memory.turns.is_empty() => (GREET, None),
            _ if name.is_some() => (CHECK_IN_NAME, name),
            _ => (CHECK_IN, None),
        }
    } else if matches!(
        joined.as_str(),
        "how are you"
            | "how are u"
            | "how's it going"
            | "hows it going"
            | "how are you doing"
            | "how was your day"
            | "how is your day"
            | "u ok"
    ) {
        (
            if memory.turns.is_empty() {
                HOW_ARE_YOU
            } else {
                HOW_ARE_YOU_MID
            },
            None,
        )
    } else if thanks_only(&words) {
        if memory
            .turns
            .last()
            .is_none_or(|turn| turn.bot.trim().ends_with('?'))
        {
            let options: Vec<_> = THANKS
                .iter()
                .copied()
                .filter(|line| !line.contains("helped"))
                .collect();
            return Some(choose(&options, memory, None, ""));
        }
        (THANKS, None)
    } else if matches!(
        joined.as_str(),
        "bye"
            | "goodbye"
            | "bye bye"
            | "cya"
            | "cya later"
            | "see you"
            | "see you later"
            | "cheerio"
            | "talk later"
            | "later"
    ) {
        (BYE, None)
    } else if matches!(
        joined.as_str(),
        "night" | "good night" | "goodnight" | "night night"
    ) {
        (NIGHT, None)
    } else if matches!(
        joined.as_str(),
        "help" | "help me" | "help please" | "please help" | "help me please"
    ) {
        (HELP, None)
    } else if matches!(
        joined.as_str(),
        "ok" | "okay"
            | "alright"
            | "right"
            | "fine"
            | "sounds good"
            | "cool"
            | "nice"
            | "nice one"
            | "great"
    ) {
        (ACK, None)
    } else {
        return crate::bot::memory::chat_answer_step(reading, memory)
            .or_else(|| super::chat_someone::respond(reading, memory))
            .or_else(|| trip_intention::respond(reading, memory, &words))
            .or_else(|| course_want::respond(reading, memory, &words))
            .or_else(|| crate::bot::memory::carry_chat_reply(reading, memory))
            .or_else(|| crate::bot::memory::chat_joined_followup(reading, memory));
    };
    Some(choose(pool, memory, name, ""))
}
