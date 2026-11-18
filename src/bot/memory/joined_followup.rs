//! Read an unanswered short ask with a recent subject, as in PyBot's chat._follow_up_reading.
#[path = "joined_followup_data.rs"]
mod data;
#[cfg(test)]
#[path = "joined_followup_tests.rs"]
mod tests;

use super::{again, frame_reply, measurement};
use crate::bot::{
    Exchange, Memory, Reading, knowledge::lookup, modules::answers, understanding::kinds::Kind,
};
use fancy_regex::Regex;
use std::{collections::HashSet, sync::LazyLock};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    QUESTION_OPEN,
    YES_NO_OPEN,
    OFFERING,
    OWN_THING,
    LOOKING_FOR,
    FOUND_THING,
    AN_ARTICLE,
    ASKS_ABOUT_A_THING,
    ASKS_WHAT_TO_DO_WITH,
    DISTRESS,
    TIME_ASK,
    NO_REASON,
    DONENESS,
    ABOUT_THE_CHAT,
);

type KindPatterns = Vec<(&'static str, Regex)>;
static ASK_KIND: LazyLock<KindPatterns> = LazyLock::new(|| table(data::ASKS));
static FIGURE_SPANS: LazyLock<KindPatterns> = LazyLock::new(|| table(data::SPANS));
static WORD: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"[a-z]+").unwrap());
static FLAT_WORD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[0-9a-z]+").unwrap());
static DURATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::DURATION).unwrap());

fn table(data: &'static [(&'static str, &'static str)]) -> KindPatterns {
    data.iter()
        .map(|(kind, pattern)| (*kind, Regex::new(pattern).expect(kind)))
        .collect()
}

fn seen(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn clean(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn content(text: &str) -> bool {
    !seen(&ABOUT_THE_CHAT, text)
        && again::topic_words(text).iter().any(|word| {
            !again::topic_stopword(word)
                && ![data::ASK_ONLY, data::ASK_MORE, data::STAND_IN]
                    .iter()
                    .any(|group| group.contains(&word.as_str()))
        })
}

fn bare(ask: &str) -> bool {
    let count = ask.split_whitespace().count();
    count <= 8
        && !seen(&OFFERING, ask)
        && (seen(&QUESTION_OPEN, ask) || (count <= 5 && seen(&YES_NO_OPEN, ask)))
}

fn singular(word: &str) -> &str {
    let stem = word.trim_end_matches('s');
    if stem.is_empty() { word } else { stem }
}

fn names(pattern: &Regex, sentence: &str) -> Vec<String> {
    pattern
        .captures_iter(sentence)
        .filter_map(Result::ok)
        .filter_map(|found| found.get(1).map(|name| name.as_str().to_owned()))
        .collect()
}

fn unrelated_named_thing(ask: &str, topic: &str) -> bool {
    let owns = names(&OWN_THING, ask);
    let looking = seen(&LOOKING_FOR, ask).then(|| names(&FOUND_THING, ask));
    let mut things = owns;
    if let Some(looking) = looking {
        things.extend(
            looking
                .into_iter()
                .filter(|word| !data::NOT_A_THING.contains(&word.as_str())),
        );
    }
    if things.is_empty() && seen(&ASKS_ABOUT_A_THING, ask) && !seen(&ASKS_WHAT_TO_DO_WITH, ask) {
        things.extend(names(&AN_ARTICLE, ask).into_iter().filter(|word| {
            !data::NOT_A_THING.contains(&word.as_str())
                && !data::A_CONTAINER.contains(&word.as_str())
        }));
    }
    !things.is_empty()
        && !WORD.find_iter(topic).any(|word| {
            things
                .iter()
                .any(|thing| singular(thing) == singular(word.as_str()))
        })
}

fn in_safety_thread(memory: &Memory) -> bool {
    memory
        .turns
        .iter()
        .rev()
        .take(3)
        .find(|turn| {
            !matches!(
                turn.module,
                "carers_turn" | "safety_cards" | "greeting" | ""
            )
        })
        .is_some_and(|turn| matches!(turn.module, "safety" | "urgent"))
}

fn known_topics(memory: &Memory) -> Vec<(usize, &Exchange, Reading<'_>)> {
    let mut prior = HashSet::new();
    let mut topics = Vec::new();
    for (index, turn) in memory.turns.iter().enumerate().rev() {
        if matches!(turn.module, "record_due" | "a_figure_of_a_dish") || !content(&turn.user) {
            continue;
        }
        let reading = Reading::new(&turn.user);
        if matches!(reading.kind, Kind::Empty | Kind::Greeting | Kind::Thanks)
            || reading
                .topics
                .iter()
                .any(|(topic, _)| *topic == "about-the-bot")
            || (reading.reference.followup && reading.context_fields.subject.is_none())
            || !prior.insert(clean(&turn.user))
        {
            continue;
        }
        topics.push((index, turn, reading));
        if topics.len() == 3 {
            break;
        }
    }
    topics
}

fn useful_kind(ask: &str, answer: &str) -> bool {
    let Some((kind, _)) = ASK_KIND.iter().find(|(_, pattern)| seen(pattern, ask)) else {
        return true;
    };
    if *kind == "date" && seen(&DONENESS, ask) {
        return true;
    }
    FIGURE_SPANS
        .iter()
        .find(|(property, _)| property == kind)
        .is_none_or(|(_, pattern)| seen(pattern, answer))
}

fn words_only(text: &str) -> String {
    let low = text.to_lowercase();
    FLAT_WORD
        .find_iter(&low)
        .map(|word| word.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn said_before(said: &str, memory: &Memory) -> bool {
    let flat = words_only(said);
    if flat.is_empty() {
        return true;
    }
    let parts: Vec<_> = measurement::sentences(said)
        .into_iter()
        .map(words_only)
        .filter(|part| part.split_whitespace().count() >= 4)
        .collect();
    memory.turns.iter().any(|turn| {
        let earlier = words_only(&turn.bot);
        !earlier.is_empty()
            && (flat == earlier
                || (earlier.split_whitespace().count() >= 4
                    && (flat.contains(&earlier) || earlier.contains(&flat)))
                || parts.iter().any(|part| earlier.contains(part)))
    })
}

fn proves(topic: &Reading<'_>, ask: &str, keys: &[&[&str]]) -> bool {
    let tokens = |group: &[&str]| {
        group
            .iter()
            .flat_map(|choice| choice.split_whitespace())
            .map(|word| singular(&word.to_lowercase()).to_owned())
            .collect::<HashSet<_>>()
    };
    let named = topic.context_fields.subject.as_deref().unwrap_or("");
    let all: HashSet<_> = keys.iter().flat_map(|group| tokens(group)).collect();
    if WORD
        .find_iter(named)
        .any(|word| all.contains(singular(word.as_str())))
    {
        return true;
    }
    let Some(first) = keys.first() else {
        return false;
    };
    let name = tokens(first);
    let original = topic.raw.to_lowercase();
    let earlier: HashSet<_> = WORD
        .find_iter(&original)
        .map(|word| singular(word.as_str()))
        .collect();
    let added: HashSet<_> = WORD
        .find_iter(ask)
        .map(|word| singular(word.as_str()))
        .filter(|word| !earlier.contains(word))
        .collect();
    topic
        .entities
        .iter()
        .filter(|entity| matches!(entity.kind, "thing" | "person"))
        .any(|entity| {
            WORD.find_iter(&entity.text.to_lowercase()).any(|word| {
                let word = singular(word.as_str());
                name.contains(word) && !added.contains(word)
            })
        })
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let ask = clean(reading.raw);
    if memory.turns.is_empty()
        || !bare(&ask)
        || in_safety_thread(memory)
        || reading.someone_new
        || reading
            .topics
            .iter()
            .any(|(topic, _)| *topic == "about-the-bot")
        || seen(&DISTRESS, &ask)
        || super::callback::has_update(reading.raw, memory)
    {
        return None;
    }
    let topics = known_topics(memory);
    let &(newest, first, _) = topics.first()?;
    if first.module == "greeting"
        || topics
            .iter()
            .any(|(_, turn, _)| seen(&DISTRESS, &clean(&turn.user)))
        || (reading.kind == Kind::Question
            && memory
                .turns
                .iter()
                .rev()
                .take(3)
                .any(|turn| seen(&NO_REASON, &clean(&turn.user))))
    {
        return None;
    }
    let can_try_older = !content(&ask) && newest + 1 != memory.turns.len();
    let mut joined = topics
        .iter()
        .filter_map(|(index, turn, topic)| {
            let text = clean(&turn.user);
            (text != ask && !unrelated_named_thing(&ask, &text)).then_some((*index, text, topic))
        })
        .peekable();
    let mut first_pass = true;
    while let Some((_, prior, topic)) = joined.next() {
        if !first_pass && !can_try_older {
            return None;
        }
        first_pass = false;
        let text = format!("{prior} {ask}");
        let wide = Reading::understand(
            &text,
            memory.turns.iter().map(|turn| turn.user.as_str()),
            memory.intent_wants,
            &memory.prior_readings,
        );
        let Some(offered) = answers::offer(&wide, memory) else {
            continue;
        };
        let body = frame_reply(&offered.reply, &text, memory).unwrap_or(offered.reply);
        if said_before(&body, memory)
            || again::unframed(&body).is_some_and(|inner| said_before(inner, memory))
        {
            if joined
                .peek()
                .is_some_and(|(next, _, _)| memory.turns.len() - next <= 3)
            {
                continue;
            }
            return None;
        }
        if !useful_kind(&ask, &body) {
            return None;
        }
        if (seen(&TIME_ASK, &body)
            && memory
                .turns
                .iter()
                .any(|turn| seen(&DURATION, &format!(" {} ", clean(&turn.user)))))
            || lookup::lookup_in_context(&wide, memory)
                .is_some_and(|card| !proves(topic, &ask, card.keys()))
        {
            continue;
        }
        return Some(body);
    }
    None
}
