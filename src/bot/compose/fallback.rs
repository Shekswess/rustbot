//! Core fallback, courtesy, and conversation-opening rules from PyBot.

pub(in crate::bot) use super::choose_recent as choose;
use super::pick;
use crate::bot::{Memory, Reading};
use crate::text::is_space;
use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

#[path = "fallback_data.rs"]
mod data;
use data::*;

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9']+").unwrap());
static LETTER_WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static PRIVACY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
    r"^(?:is (?:this|it|our chat|this chat) (?:private|confidential)|",
    r"(?:who|can anyone|can anybody) (?:can )?(?:see|read) (?:this chat|what i (?:say|type)|our chat)|",
    r"(?:do|will|can) you (?:save|log|store|keep|share|record) (?:this|our) (?:conversation|chat))\b"
)).unwrap()
});
static LIMITS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"^(?:what (?:cant|cannot|can not) you (?:do|help with|answer)|what are your (?:limits|limitations))\b"
).unwrap()
});
static ONLINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"^(?:are you connected to (?:the )?(?:internet|web)|(?:can|do) you (?:browse|search|access|use) (?:the )?(?:internet|web)|can you go online)\b"
).unwrap()
});
static METHOD_OBTAIN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^ (?:get|find|buy|hire|rent|book|order|locate|pick up|put|keep|store)\b").unwrap()
});
static METHOD_OBTAIN_EXCEPTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"^ (?:up|better|going|started|through|on|out|round|back|dressed|ready|organised|there|in|to)\b"
).unwrap()
});
static METHOD_OTHER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:round here|near me|nearby|in my area|locally|you|your|yourself)\b").unwrap()
});
static ASK_WAY_IN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"^(?:(?:i )?(?:dont|don't|do not) know |(?:i have )?no idea )?(?:what (?:should|can|do|could) i do|what to do)(?: (?:now|next|anymore|any more|about (?:it|this|that)))*[.!?]*$"
).unwrap()
});
static PRESENCE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?:can|could|may|might) i ask (?:you )?(?:something|a question|one thing)",
        r"(?: please)?[.!?]*$|^(?:are you (?:online|there)|hello|hi)\s*\?+$"
    ))
    .unwrap()
});
static IMPERATIVE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"^(?:tell|explain|show|give|suggest|recommend|find|list|write|make) (?:me\b|us\b|how\b|why\b)"
).unwrap()
});
static FAREWELL_QUESTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:when|what|where|which|why|who|how|can|could|should|would|do|does|did|is|are|will)\b",
    )
    .unwrap()
});

const NAME_NOT_NOTED: &[&str] = &[
    "Remind me of the name and I'll keep it from here.",
    "Say the name again and I'll remember it from here on.",
    "I haven't got it in front of me — tell me the name and I'll hold on to it.",
];
const ASK_FOR_NAME: &[&str] = &[
    "What's your name? You've not said, so only you know - tell me and I'll remember it.",
    "What's your name? I can't look that up: it's about you. Say it once and I'll keep it.",
];
const SUDDEN_LOSS: &[&str] = &[
    "I'm so sorry. That's a real loss, and it deserves to be taken seriously — who have you got around you this week?",
    "I'm really sorry. Take the time you need, and tell me what would help today.",
    "That's heartbreaking, and I'm sorry. Is there someone you can be with today?",
];

/// Answer courtesy and low-priority asks that specialist modules did not claim.
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(reply) = super::after_emergency::reply(reading, memory) {
        return Some(reply);
    }
    if crate::bot::modules::safety::current_gas(&reading.clean)
        && !crate::bot::modules::the_detail_the_thread_adds::heavy::only_a_dream(&reading.clean)
        && super::trouble::advisor::gas_action(&reading.clean)
    {
        return Some(super::after_emergency::evacuate().to_owned());
    }
    if let Some(reply) = super::leaving::reply(reading, memory) {
        return Some(reply);
    }
    if let Some(reply) = super::morning_after::reply(reading, memory) {
        return Some(reply);
    }
    if crate::bot::memory::asks_for_name(reading) {
        let corrected = memory
            .turns
            .iter()
            .rev()
            .take(2)
            .any(|turn| turn.module == "name_taken_back");
        let pool = if corrected {
            ASK_FOR_NAME
        } else {
            NAME_NOT_NOTED
        };
        return Some(choose(pool, memory, &reading.clean).to_owned());
    }
    if super::loss::recognizes(&reading.clean) {
        return Some(choose(SUDDEN_LOSS, memory, &reading.clean).to_owned());
    }
    if let Some(value) = memory.personal.favourite_value_for(&reading.clean) {
        let mut chars = value.chars();
        let capitalized: String = chars
            .next()
            .into_iter()
            .flat_map(char::to_uppercase)
            .chain(chars)
            .collect();
        let options = [
            format!("You said {value} - is that still the one?"),
            format!("{capitalized}, as far as I have it. Has that changed?"),
            format!("{capitalized}, you told me. Still the one?"),
        ];
        return Some(
            choose(
                &options.each_ref().map(String::as_str),
                memory,
                &reading.clean,
            )
            .to_owned(),
        );
    }
    if memory.turns.is_empty() {
        let clean = flatten(reading.raw).to_lowercase();
        if REFERS_BACK.is_match(&clean) {
            return Some(choose(NOTHING_BEFORE_IT, memory, &clean).into());
        }
    }
    let clean = cleaned(&reading.clean);
    let reply = fallback_line(&clean, reading, memory);
    if is_neutral(&reply)
        && let Some(receipt) = super::medical_news::reply(reading, memory)
            .or_else(|| super::trouble::advisor::reply(reading, memory))
            .or_else(|| super::trouble::continue_here(reading, memory))
            .or_else(|| super::their_people::reply(reading, memory))
            .or_else(|| super::occasion::reply(reading, memory))
            .or_else(|| super::day_ahead::reply(reading, memory))
            .or_else(|| super::dither::reply(reading, memory))
            .or_else(|| super::thing_done::plan_remark(reading, memory))
            .or_else(|| super::thing_done::completed_plan(reading, memory))
            .or_else(|| super::thing_done::reply(reading, memory))
            .or_else(|| crate::bot::memory::ack::reply(reading, memory))
            .or_else(|| super::trouble::reply(reading, memory))
            .or_else(|| super::small_day::reply(reading, memory))
    {
        return Some(receipt);
    }
    Some(reply)
}

/// Offer a clean opening when the fallback would begin with an empty receipt.
pub(crate) fn first_line_open(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if memory.turns.iter().any(|turn| !turn.bot.trim().is_empty()) {
        return None;
    }
    let reply = crate::bot::modules::greeting::respond(reading, memory)?;
    A_RECEIPT_OF_NOTHING
        .is_match(&reply)
        .then(|| choose(COLD_OPEN, memory, &flatten(reading.raw)).into())
}

fn fallback_line(clean: &str, reading: &Reading<'_>, memory: &Memory) -> String {
    let raw = reading.raw;
    let answer = |pool| choose(pool, memory, clean).to_owned();
    if let Some(news) = super::news_ask::reply(clean) {
        return news.to_owned();
    }
    if (A_TALK_AT_A_TIME_ASK.is_match(clean) || A_COMPANY_ASK.is_match(clean))
        && !A_TALK_AT_A_TIME_HARM.is_match(clean)
    {
        return answer(TALK_AT_A_TIME);
    }
    if let Some(thing) = continuation(clean) {
        if !thing.is_empty() {
            return answer(RETURNED_TO_A_THING).replace("{thing}", &thing);
        }
        return answer(if memory.turns.is_empty() {
            RETURNED_TO_IT_FIRST
        } else {
            RETURNED_TO_IT
        });
    }

    let words: Vec<_> = WORD.find_iter(clean).map(|word| word.as_str()).collect();
    if words.is_empty()
        || words
            .iter()
            .all(|word| word.chars().all(|character| character == '\''))
    {
        return answer(EMPTY);
    }
    let spoken = words.join(" ");
    if words.len() <= 6 && FLAT_LINE.is_match(&spoken) {
        return answer(FLAT);
    }
    if let Some(pool) = reaction_pool(clean) {
        return answer(pool);
    }

    let repairs = words
        .iter()
        .map(|word| chat_spelling(word))
        .collect::<Vec<_>>()
        .join(" ");
    let asking = asks_a_question(clean, raw, &words)
        || words == ["sup"]
        || greeting_tail(&words).is_some_and(|tail| matches!(tail, ["quick", "question" | "one"]));
    let thanks = thanks_outcome(clean);
    if thanks
        .is_some_and(|said| THANKS_WENT_BADLY.is_match(said) && !THANKS_GOOD_AGAIN.is_match(said))
    {
        return answer(THANKS_NO_BETTER);
    }
    if !asking && a_thanks(&words) {
        return answer(farewell_pool(clean).unwrap_or(THANKS));
    }
    let hand = HAND_ASK.is_match(&spoken) || HAND_ASK.is_match(&repairs);
    if !asking
        && !hand
        && (reading.kind == crate::bot::understanding::kinds::Kind::Greeting
            || greeting_only(clean, &words))
    {
        return answer(GREETING);
    }
    if let Some(reply) = handoff(clean, memory) {
        return reply;
    }
    if hand {
        return answer(HAND);
    }
    if let Some(pool) = about_pool(clean, &repairs) {
        return answer(pool);
    }
    if let Some(pool) = farewell_pool(clean) {
        return answer(pool);
    }
    if how_are_you(clean, &repairs) {
        return answer(HOW_ARE_YOU);
    }
    if SORRY_ASKING.is_match(clean) && only_words(clean, MANNERS_ONLY) {
        return answer(SORRY_FOR_ASKING);
    }
    if ACKNOWLEDGES_ALONE.is_match(clean) {
        return answer(THANKS);
    }
    if thanks.is_some_and(|said| THANKS_GOOD_AGAIN.is_match(said) || !asking) {
        return answer(THANKS_THEN_SAID);
    }
    if ASK_WAY_IN.is_match(clean) || ASK_WAY_IN.is_match(&repairs) {
        return answer(BARE_PICK_DO);
    }
    if asking || asking_for_help(clean) || IMPERATIVE.is_match(clean) {
        let pool = unknown_pool(clean, memory);
        if pool == UNKNOWN
            && let Some(day) = super::for_the_day::for_the_ask(clean, &repairs)
        {
            return answer(day);
        }
        if pool == UNKNOWN
            && let Some(descaling) = super::carrier::descaling(reading, memory)
        {
            return descaling;
        }
        if (pool == UNKNOWN || pool == UNKNOWN_DECIDE)
            && let Some(routed) = super::what_i_do_know::reply_for(raw, memory)
        {
            return routed;
        }
        return answer(pool);
    }
    if words.len() <= 2 && words.iter().any(|word| ACKS.contains(word)) {
        return answer(ACK);
    }
    if A_SORRY_ON_ITS_OWN.is_match(&spoken) || A_SORRY_ON_ITS_OWN.is_match(clean) {
        return answer(SORRY_ON_ITS_OWN);
    }
    if A_DISMISSAL.is_match(clean) {
        return answer(NEVER_MIND);
    }
    if words.iter().any(|word| THANK_WORDS.contains(word)) && only_words(clean, THANKS_ONLY) {
        return answer(THANKS);
    }
    if greeting_tail(&words).is_some() || GREETING_OPEN.is_match(clean) {
        return answer(GREETING);
    }
    // The reference seeds its neutral pool with the understanding reader's clean
    // text, including its accent folding and original quotation marks.
    choose(NEUTRAL, memory, &reading.line.clean).to_owned()
}

pub(super) fn is_neutral(reply: &str) -> bool {
    fn plain(s: &str) -> String {
        flatten(&s.to_lowercase().replace(['—', '–'], "-"))
    }
    static HEADS: std::sync::LazyLock<Vec<String>> =
        std::sync::LazyLock::new(|| NEUTRAL.iter().map(|line| plain(line)).collect());
    let line = plain(reply);
    !line.is_empty() && HEADS.iter().any(|head| line.starts_with(head))
}

pub(super) fn placeholder(reply: &str) -> bool {
    let head = flatten(reply)
        .to_lowercase()
        .chars()
        .take(40)
        .collect::<String>();
    is_neutral(reply)
        || unknown_replies()
            .iter()
            .any(|line| line.to_lowercase().chars().take(40).eq(head.chars()))
}

/// Replies used by the fallback when no more specific answer is available.
pub(crate) fn unknown_replies() -> &'static [&'static str] {
    UNKNOWN
}

fn flatten(text: &str) -> String {
    text.split(is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn cleaned(text: &str) -> String {
    let text = short_hello(text);
    let mut output = String::with_capacity(text.len());
    for word in text.split(' ') {
        if word.is_empty() {
            continue;
        }
        let said = table_value(SAID_OUT, word).unwrap_or(word);
        let said = if output
            .split(' ')
            .next_back()
            .is_some_and(|last| SUBJECT_SAID.contains(&last))
        {
            said.strip_prefix("i ")
                .or_else(|| said.strip_prefix("I "))
                .unwrap_or(said)
        } else {
            said
        };
        if !output.is_empty() {
            output.push(' ');
        }
        output.push_str(said);
    }
    output
}

fn short_hello(text: &str) -> Cow<'_, str> {
    for &(typed, meant) in HELLO_SAID_SHORT {
        let Some(rest) = text.strip_prefix(typed) else {
            continue;
        };
        if rest.is_empty() {
            return Cow::Borrowed(meant);
        }
        if rest.starts_with([' ', ',', '!']) {
            let words: Vec<_> = rest
                .trim_matches([' ', ',', '!', '.'])
                .split(' ')
                .skip_while(|word| HELLO_TAIL.contains(&word.trim_matches([',', '.', '!'])))
                .collect();
            let rest = words.join(" ");
            let rest = rest.trim_matches([' ', ',', '!', '.']);
            return if rest.is_empty() {
                Cow::Borrowed(meant)
            } else {
                Cow::Owned(rest.into())
            };
        }
    }
    Cow::Borrowed(text)
}

fn table_value<'a>(table: &[(&str, &'a str)], key: &str) -> Option<&'a str> {
    table
        .binary_search_by(|(candidate, _)| candidate.cmp(&key))
        .ok()
        .map(|index| table[index].1)
}

fn chat_spelling(word: &str) -> &str {
    match word {
        "u" | "ya" => "you",
        "ur" => "your",
        "pls" | "plz" => "please",
        "hows" | "howz" | "how's" => "how is",
        "whats" | "what's" | "wats" => "what is",
        "r" => "are",
        _ => word,
    }
}

fn continuation(clean: &str) -> Option<String> {
    if !POINTS_BACK.is_match(clean) {
        return None;
    }
    let left = POINTS_BACK.replacen(clean, 3, " ");
    let tokens: Vec<_> = left
        .split_whitespace()
        .filter_map(|part| {
            let word: String = part
                .chars()
                .filter(|ch| ch.is_ascii_alphanumeric() || *ch == '\'')
                .collect();
            (!word.is_empty()).then_some(word)
        })
        .collect();
    let lead = tokens
        .iter()
        .position(|word| A_THING_MARKER.contains(&word.as_str()))
        .unwrap_or(tokens.len());
    if !tokens[..lead]
        .iter()
        .all(|word| A_POINTING_FILLER.contains(&word.as_str()))
    {
        return None;
    }
    if lead == tokens.len() {
        return Some(String::new());
    }
    let thing = &tokens[lead + 1..];
    (1..=4).contains(&thing.len()).then(|| thing.join(" "))
}

fn greeting_tail<'words, 'text>(words: &'words [&'text str]) -> Option<&'words [&'text str]> {
    GREETINGS
        .iter()
        .filter_map(|phrase| {
            let count = phrase.split(' ').count();
            (words.len() >= count && words[..count].iter().copied().eq(phrase.split(' ')))
                .then(|| &words[count..])
        })
        .min_by_key(|rest| rest.len())
}

fn greeting_only(clean: &str, words: &[&str]) -> bool {
    let Some(rest) = greeting_tail(words) else {
        return false;
    };
    rest.iter()
        .all(|word| GREETING_TAIL.contains(word) || POLITE.contains(word))
        || rest.len() <= 2 && rest.iter().all(|word| !NEEDS_ANSWER.contains(word))
        || (rest.starts_with(&["im"])
            || rest.starts_with(&["i'm"])
            || rest.starts_with(&["my", "name", "is"]))
            && rest.len() <= 4
            && !HAND_ASK.is_match(clean)
}

pub(super) fn asks_a_question(clean: &str, raw: &str, words: &[&str]) -> bool {
    if raw.contains('?') {
        return true;
    }
    let plain = words
        .iter()
        .map(|word| word.replace('\'', ""))
        .collect::<Vec<_>>();
    if let Some(first) = plain.first() {
        let first = table_value(WORD_FIXES, first).unwrap_or(first);
        if QUESTION_WORDS.contains(&first) {
            return true;
        }
    }
    let joined = format!(" {} ", plain.join(" "));
    LATER_QUESTION.is_match(&INDIRECT.replace_all(clean, " "))
        || ASK_SHAPES.is_match(&INDIRECT.replace_all(&joined, " "))
}

fn only_words(clean: &str, allow: &[&str]) -> bool {
    let mut words = LETTER_WORD.find_iter(clean).peekable();
    words.peek().is_some() && words.all(|word| allow.contains(&word.as_str()))
}

fn a_thanks(words: &[&str]) -> bool {
    let rest = words
        .iter()
        .position(|word| {
            ![
                "oh", "ah", "aha", "aah", "ahh", "ohh", "ooh", "aw", "aww", "mm", "mmm",
            ]
            .contains(word)
        })
        .map_or(&[][..], |at| &words[at..]);
    let Some(tail) = THANKS_PHRASES
        .iter()
        .filter_map(|phrase| {
            let count = phrase.split(' ').count();
            (rest.len() >= count && rest[..count].iter().copied().eq(phrase.split(' ')))
                .then(|| &rest[count..])
        })
        .min_by_key(|tail| tail.len())
    else {
        return false;
    };
    if tail.iter().any(|word| NEEDS_ANSWER.contains(word)) {
        return false;
    }
    tail.len() <= 3
        || tail
            .first()
            .is_some_and(|word| ["for", "very", "so"].contains(word) || THANKS_ABOUT.contains(word))
        || tail
            .iter()
            .all(|word| THANKS_ONLY.contains(word) || ADDRESSED.contains(word))
}

fn thanks_outcome(clean: &str) -> Option<&str> {
    let said = THANKS_OPENS
        .captures(clean.trim_end_matches(['.', '!']).trim())?
        .name("said")?
        .as_str()
        .trim_matches([' ', '.', ',', '!', ';', ':']);
    (LETTER_WORD.find_iter(said).take(2).count() == 2
        && !THANKS_TAKES_A_QUESTION.is_match(&THANKS_GOOD_AGAIN.replace_all(said, " ")))
    .then_some(said)
}

fn reaction_pool(clean: &str) -> Option<&'static [&'static str]> {
    if !REACTION_LINE.is_match(clean) {
        return None;
    }
    [
        (&*REACTION_SIGN_0, REACTION_RELIEF),
        (&*REACTION_SIGN_1, REACTION_OUCH),
        (&*REACTION_SIGN_2, REACTION_MISHAP),
        (&*REACTION_SIGN_3, REACTION_DISBELIEF),
        (&*REACTION_SIGN_4, REACTION_REPEAT),
    ]
    .into_iter()
    .find_map(|(signature, pool)| signature.is_match(clean).then_some(pool))
}

fn handoff(clean: &str, memory: &Memory) -> Option<String> {
    if let Some(thing) = HANDOFF_ABOUT
        .captures(clean)
        .and_then(|capture| capture.name("thing"))
    {
        let thing = thing
            .as_str()
            .trim_matches([' ', ',', '.', '?', '!', ':', '-']);
        let first = thing.split(' ').next().unwrap_or("");
        if !first.is_empty()
            && !POINTS_AT_NOTHING.contains(&first)
            && !POINTS_AT_NOTHING.contains(&thing)
            && !HANDOFF_NOT_SUBJECT.contains(&first)
        {
            let mut chars = thing.chars();
            let title = chars.next().unwrap().to_uppercase().to_string() + chars.as_str();
            let variants: Vec<_> = ABOUT_THE_THING
                .iter()
                .map(|line| line.replace("{thing}", thing).replace("{Thing}", &title))
                .collect();
            return Some(
                pick(
                    &variants.iter().map(String::as_str).collect::<Vec<_>>(),
                    memory,
                    clean,
                )
                .to_owned(),
            );
        }
    }
    HANDOFF.is_match(clean).then(|| {
        pick(
            if HANDOFF_QUESTION.is_match(clean) {
                ASK_AWAY
            } else {
                GO_ON
            },
            memory,
            clean,
        )
        .into()
    })
}

fn about_pool(clean: &str, repaired: &str) -> Option<&'static [&'static str]> {
    let words: Vec<_> = LETTER_WORD
        .find_iter(clean)
        .map(|word| word.as_str())
        .filter(|word| !FILLERS.contains(word))
        .collect();
    let words = words.join(" ");
    let phrase = words
        .trim_start_matches("so ")
        .trim_start_matches("ok ")
        .trim_start_matches("okay ");
    let opens = |phrases: &[&str]| {
        phrases.iter().any(|start| {
            phrase == *start
                || phrase
                    .strip_prefix(start)
                    .is_some_and(|rest| rest.starts_with(' '))
        })
    };
    if (PRIVACY.is_match(phrase) || BETWEEN_US_ASK.is_match(phrase))
        && !NOT_WHAT_I_SAID.is_match(phrase)
    {
        Some(ABOUT_PRIVACY)
    } else if TRUST_ASK.is_match(phrase) {
        Some(ABOUT_TRUST)
    } else if SECRET_ASK.is_match(phrase) {
        Some(ABOUT_SECRET)
    } else if JUDGING_ASK.is_match(phrase) {
        Some(ABOUT_JUDGING)
    } else if LIMITS.is_match(phrase) {
        Some(ABOUT_LIMITS)
    } else if ONLINE.is_match(phrase) {
        Some(ABOUT_ONLINE)
    } else if MODEL_ASK.is_match(phrase) {
        Some(ABOUT_MODEL)
    } else if WORKINGS_ASK.is_match(phrase) {
        Some(ABOUT_WORKINGS)
    } else if matches!(phrase, "what are you" | "whats a chatbot") || opens(IDENTITY) {
        Some(ABOUT_IDENTITY)
    } else if phrase == "what do you remember"
        || phrase.ends_with(" what do you remember")
        || opens(MEMORY)
    {
        Some(ABOUT_MEMORY)
    } else if matches!(phrase, "why are you here" | "why do you exist") {
        Some(ABOUT_PURPOSE)
    } else if HELP.iter().any(|start| {
        phrase.strip_prefix(start).is_some_and(|tail| {
            tail.is_empty()
                || tail.starts_with(' ')
                    && tail
                        .split_whitespace()
                        .all(|word| HELP_TAIL.contains(&word))
        })
    }) || HELP_ELSE.captures(repaired).is_some_and(|groups| {
        groups.get(1).or_else(|| groups.get(2)).is_some_and(|tail| {
            tail.as_str()
                .split_whitespace()
                .all(|word| HELP_TAIL.contains(&word))
        })
    }) {
        Some(ABOUT_HELP)
    } else {
        None
    }
}

fn how_are_you(clean: &str, repaired: &str) -> bool {
    let plain = clean.replace('\'', "");
    for form in [plain.as_str(), repaired] {
        let tags = |end: usize| {
            LETTER_WORD
                .find_iter(&form[end..])
                .all(|word| HOW_ARE_YOU_TAGS.contains(&word.as_str()))
        };
        if [&*YOUR_DAY_ASK, &*YOUR_TIME_ASK]
            .into_iter()
            .any(|pattern| pattern.find_iter(form).any(|found| tags(found.end())))
        {
            return true;
        }
        for phrase in HOW_ARE_YOU_ASKS {
            let phrase = phrase.replace('\'', "");
            if form
                .match_indices(&phrase)
                .any(|(at, text)| word_edges(form, at, text.len()) && tags(at + text.len()))
            {
                return true;
            }
        }
        for phrase in HOW_ARE_YOU_GOING_ON {
            if form.match_indices(phrase).any(|(at, text)| {
                let after = &form[at + text.len()..];
                if !word_edges(form, at, text.len()) || after.chars().any(char::is_numeric) {
                    return false;
                }
                let mut words = LETTER_WORD.find_iter(after);
                words.next().is_none_or(|word| word.as_str() == "else") && words.next().is_none()
            }) {
                return true;
            }
        }
    }
    let mut norm: Vec<_> = repaired
        .split(' ')
        .filter(|word| !word.is_empty())
        .collect();
    if let Some(rest) = greeting_tail(&norm) {
        norm = rest.to_vec();
    }
    while norm.len() > 2
        && norm[norm.len() - 2] == "with"
        && GREETING_ASK_SECOND_PERSON.contains(norm.last().unwrap())
    {
        norm.truncate(norm.len() - 2);
    }
    if norm.is_empty()
        || norm.len() > 8
        || !norm.iter().any(|word| GREETING_ASK_SUBJECT.contains(word))
    {
        return false;
    }
    match norm[0] {
        "what" => norm.iter().all(|word| GREETING_WHAT_SAYING.contains(word)),
        "how" => norm.iter().all(|word| GREETING_ASK_WORDS.contains(word)),
        _ => {
            norm.len() <= 4
                && norm
                    .iter()
                    .position(|word| GREETING_ASK_STATE.contains(word))
                    .is_some_and(|index| {
                        norm[..index]
                            .iter()
                            .any(|word| GREETING_ASK_SECOND_PERSON.contains(word))
                            && norm
                                .get(index + 1)
                                .is_none_or(|after| !GREETING_ASK_ABOUT_SOMETHING.contains(after))
                    })
        }
    }
}

fn word_edges(text: &str, start: usize, length: usize) -> bool {
    !text
        .as_bytes()
        .get(start.wrapping_sub(1))
        .is_some_and(u8::is_ascii_lowercase)
        && !text
            .as_bytes()
            .get(start + length)
            .is_some_and(u8::is_ascii_lowercase)
}

fn farewell_pool(clean: &str) -> Option<&'static [&'static str]> {
    let clauses: Vec<_> = clean
        .split([',', ';', '.', '!', '?'])
        .map(str::trim)
        .filter(|clause| !clause.is_empty())
        .collect();
    if clean.contains('?')
        || clauses
            .first()
            .is_some_and(|first| FAREWELL_QUESTION.is_match(first))
    {
        return None;
    }
    let index = clauses
        .iter()
        .position(|clause| a_farewell_clause(clause))?;
    if clauses[index + 1..]
        .iter()
        .any(|clause| !a_farewell_clause(clause) && !only_words(clause, LEAVING_THANKS))
    {
        return Some(REASON_POOL);
    }
    Some(
        if NITE.is_match(clauses[index]) && !NOT_A_BEDTIME.is_match(clauses[index]) {
            NIGHT_GOODBYE_POOL
        } else {
            GOODBYE
        },
    )
}

fn a_farewell_clause(clause: &str) -> bool {
    if KEEPS.is_match(clause)
        || ASKS_OPENER.is_match(clause)
        || (AWAY_OUT.is_match(clause) && AWAY_PLAN.is_match(clause))
        || (BED_LEAVE.is_match(clause) && NEXT_DAY.is_match(clause))
    {
        return false;
    }
    if FAREWELLS.iter().any(|phrase| {
        clause.strip_prefix(phrase).is_some_and(|rest| {
            rest.is_empty()
                || rest.starts_with(' ')
                    && rest.split_whitespace().all(|word| {
                        FAREWELL_TAIL.contains(&word) || SIGN_OFF_VOCATIVE.contains(&word)
                    })
        })
    }) {
        return true;
    }
    if !LEAVE.is_match(clause) {
        return false;
    }
    let tail = LEAVE.replace_all(clause, " ");
    let allowed = |text: &str| {
        LETTER_WORD.find_iter(text).all(|word| {
            [
                LEAVING_ALLOWED,
                SIGN_OFF_WORDS,
                LEAVE_SHORTHAND,
                SIGN_OFF_VOCATIVE,
            ]
            .into_iter()
            .any(|group| group.contains(&word.as_str()))
        })
    };
    allowed(&tail)
        || (A_SIGN_OFF.is_match(clause)
            && A_LEAVE_TIME.is_match(&tail)
            && allowed(&A_LEAVE_TIME.replace_all(&tail, " ")))
}

fn asking_for_help(clean: &str) -> bool {
    TIPS_ASKS.is_match(clean) && !NOT_A_TIPS_ASK.is_match(clean)
}

fn unknown_pool(clean: &str, memory: &Memory) -> &'static [&'static str] {
    if PRESENCE_ASK.is_match(clean) {
        return UNKNOWN;
    }
    if asking_for_help(clean) {
        return TIPS_LINES;
    }
    if DECIDE_OPENING.is_match(clean) && !DEFINITION.is_match(clean) {
        return UNKNOWN_DECIDE;
    }
    if let Some(opening) = METHOD_OPENING.find(clean) {
        let tail = &clean[opening.end()..];
        let obtaining = METHOD_OBTAIN
            .find(tail)
            .is_some_and(|verb| !METHOD_OBTAIN_EXCEPTION.is_match(&tail[verb.end()..]));
        if !obtaining
            && !METHOD_OTHER.is_match(clean)
            && !HOW_TO_TELL.is_match(clean)
            && !DEFINITION.is_match(clean)
            && !LETTER_WORD
                .find_iter(clean)
                .any(|word| PRODUCT_WORDS.contains(&word.as_str()))
        {
            return super::lead_in_probe::method_after_answer(memory).unwrap_or(TIPS_LINES);
        }
    }
    UNKNOWN
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reply(text: &str) -> String {
        respond(&Reading::new(text), &Memory::default()).unwrap()
    }

    #[test]
    fn a_drink_report_and_a_bad_day_take_their_own_fallback() {
        assert_eq!(
            reply("i drank too much last night"),
            "How bad is it - banging head, or just tired and skint?"
        );
        assert_eq!(
            reply("i skipped the gym all week"),
            "That is a nuisance. Is there anything that would help?"
        );
    }

    #[test]
    fn keeps_upstream_courtesy_seeds_and_reply_pools() {
        assert_eq!(
            reply("??"),
            "Take your time. What would you like to talk about?"
        );
        assert_eq!(reply("GM"), "Hi! What's up?");
        assert_eq!(
            reply("thx"),
            "No problem at all. Shout if you need anything else."
        );
        assert_eq!(
            reply("Thanks, its worse today"),
            "Sorry to hear that — thanks for coming back with it."
        );
        assert_eq!(
            reply("one more thing about the boiler"),
            "Go on — what about the boiler?"
        );
    }

    #[test]
    fn first_line_open_replaces_only_the_generic_receipt() {
        let mut memory = Memory::default();
        let neutral = Reading::new("glorp");
        assert_eq!(
            first_line_open(&neutral, &memory).as_deref(),
            Some("Go on - what's been happening?")
        );
        assert_eq!(
            first_line_open(&Reading::new("what is a froblen?"), &memory),
            None
        );
        memory.turns.push(crate::bot::Exchange {
            user: "hi".into(),
            bot: "Hi!".into(),
            module: "chat",
        });
        assert_eq!(first_line_open(&neutral, &memory), None);
    }
}
