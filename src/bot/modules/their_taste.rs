//! PyBot bot/modules/their_taste.py and the dislike path of bot/memory/obstacle.py at 7d63c7f.

use super::{greeting, the_detail_the_thread_adds::heavy};
use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

macro_rules! rule {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}

rule!(
    LEAD,
    r"^(?:(?:oh|god,?|honestly|yeah|well|actually|man|christ|jesus|blimey)\s+)*"
);
rule!(
    TONE,
    r"(?:\s+(?:tbh|tho|though|really|honestly|actually|literally|so\s+much|a\s+lot|as\s+well|too|now|anyway|i\s+suppose|i\s+guess|lol|ha|haha|btw|atm|lately|these\s+days|at\s+the\s+moment|right\s+now|still|again))+$"
);
rule!(
    LIKING,
    r"^(?:i|i'm|i've)\s+(?:really like|do like|love|like|adore|am mad about|m mad about|am into|m into|can't get enough of|cant get enough of|can not get enough of|am a fan of|m a fan of|am obsessed with|m obsessed with|have always loved|(?:am|m)\s+(?:a\s+)?(?:\w+\s+){0,2}fan\s+of)\s+(.+)$"
);
rule!(
    DISLIKING,
    r"^(?:i|i'm|i've)\s+(?:really don't like|really dont like|don't like|dont like|hate|dislike|can't stand|cant stand|can not stand|am not a fan of|m not a fan of|am not keen on|m not keen on|have never liked|have gone off)\s+(.+)$"
);
rule!(THING, r"^[a-z][a-z'\-]*(?:\s+[a-z][a-z'\-]*){0,4}$");
rule!(
    A_PERSON,
    r"\b(?:girl|boy|woman|man|guy|lad|lass|bloke|fella|someone|somebody|people|person|her|him|them|she|he|they)\b"
);
rule!(
    THEIRS,
    r"\b(?:mum|mom|mother|dad|father|son|daughter|kid|kids|child|children|wife|husband|partner|baby|gran|grandma|grandmother|nan|nana|grandad|grandpa|grandfather|aunt|uncle|cousin|sister|brother|parents|family|dog|cat|puppy|kitten|pets?|bird|horse|rabbit|hamster|boss|friend|mate|neighbours?|self|life|job|work|body|looks|health|mind|sleep)\b"
);
rule!(
    LIFE_GOING_WRONG,
    r"\b(?:my life|myself|my job|my work|my body|my looks|the world|people|mondays?\s+morning)\b"
);
rule!(
    STORY,
    r"\b(?:but|because|so|though|although|when|whenever|if|since|which|that|keeps?|makes?|gets?|goes?|go|went|got|costs?|cost|is|are|was|were|will|would|can|could|have|has|had|need|needs|want|wants|helps?|works?|tastes?|smells?)\b|\band\s+(?:i|it|its|it's|they|we|you|he|she|there|thats|that's)\b"
);
rule!(
    HEAVY,
    r"(?i)\b(?:ill|unwell|poorly|sick|dying|died|death|passed away|funeral|bereave\w*|griev\w*|cancer|chemo\w*|radiotherap\w*|tumou?r|dementia|alzheim\w*|stroke|sepsis|surgery|operation|transplant|hospice|palliative|terminal|hospital|icu|ambulance|diagnos\w*|bully\w*|bullied|depress\w*|suicid\w*|self harm|miscarr\w*|stillborn|addict\w*|relapse\w*|overdose\w*|sectioned|in pain|suffering)\b"
);
rule!(
    OBSTACLE,
    r"^i\s+(?:(?:just|really)\s+)?(?:hate|dislike|can't stand|cant stand|can not stand|don't like|dont like|(?:am|m)\s+not\s+(?:a fan of|keen on))\s+"
);
rule!(WORD, r"[a-z']+");

const LIKES: &[&str] = &[
    "Can't argue with {thing}. What do you like most?",
    "{Thing} - good taste. What is it about {thing} that you like?",
    "That is a good thing to like. What do you like most about {thing}?",
];
const DISLIKES: &[&str] = &[
    "{Thing} - fair enough. What is it you don't like about {thing}?",
    "Nobody is going to talk you round on {thing}, and nobody needs to. What is it that puts you off?",
    "Plenty of people would say the same about {thing}. What is it you cannot take about {thing}?",
];
const ALTERNATIVES: &[&str] = &[
    "Fair enough - this part doesn't need {thing}: {step}",
    "That's fine, this works without {thing}: {step}",
    "You can leave {thing} out of this one: {step}",
    "This one works without {thing}: {step}",
    "Then skip {thing} - the rest of it stands: {step}",
];
const NOT_A_THING: &[&str] = &[
    "it",
    "that",
    "this",
    "them",
    "these",
    "those",
    "him",
    "her",
    "you",
    "me",
    "us",
    "here",
    "there",
    "everything",
    "anything",
    "nothing",
    "everyone",
    "someone",
    "it all",
    "that one",
    "this one",
    "one",
    "ones",
    "things",
    "stuff",
    "life",
    "my life",
    "myself",
    "my job",
    "my work",
    "people",
    "the world",
    "the news",
    "him too",
    "her too",
    "your advice",
    "advice",
    "your help",
    "this recipe",
    "the recipe",
    "your recipes",
    "this place",
    "this app",
    "chatting to you",
    "talking to you",
    "you too",
    "you both",
    "your voice",
    "much more of this",
    "more of this",
    "much more",
    "any more of this",
    "it any more",
    "it anymore",
    "this any more",
    "this anymore",
];
const PLACEHOLDERS: &[&str] = &[
    "noted",
    "got it",
    "i'm listening",
    "i am listening",
    "thanks for telling me",
    "that's noted",
    "thats noted",
    "ok.",
    "okay.",
    "i don't know enough",
    "i dont know enough",
    "that's beyond what i know",
    "thats beyond what i know",
    "i'm not sure about that one",
    "im not sure about that one",
];

fn plain(raw: &str) -> String {
    let said = raw
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let said = LEAD.replace(&said, "");
    let said = TONE.replace(said.trim(), "");
    for (typed, proper) in [
        ("im ", "i am "),
        ("i m ", "i am "),
        ("i'm ", "i am "),
        ("ive ", "i have "),
        ("i've ", "i have "),
    ] {
        if let Some(rest) = said.strip_prefix(typed) {
            return format!("{proper}{rest}");
        }
    }
    said.into_owned()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Taste {
    Like,
    Dislike,
}

fn taste(said: &str) -> Option<(Taste, &str)> {
    let (kind, found) = if let Some(found) = DISLIKING.captures(said) {
        (Taste::Dislike, found)
    } else {
        (Taste::Like, LIKING.captures(said)?)
    };
    let thing = found.get(1)?.as_str().trim();
    if !THING.is_match(thing)
        || STORY.is_match(thing)
        || NOT_A_THING.contains(&thing)
        || A_PERSON.is_match(thing)
        || (kind == Taste::Dislike && LIFE_GOING_WRONG.is_match(thing))
    {
        return None;
    }
    match thing.split_whitespace().next()? {
        "to" | "your" | "our" | "it" | "that" | "this" | "them" | "here" | "there" | "these"
        | "those" => None,
        "my" if thing == "my" || THEIRS.is_match(thing) => None,
        _ => Some((kind, thing)),
    }
}

fn substantial(reply: &str) -> bool {
    let reply = reply.trim().to_lowercase();
    !reply.is_empty()
        && !PLACEHOLDERS.iter().any(|head| reply.starts_with(head))
        && !compose::routes::hedge(&reply)
}

fn heavy_previous(memory: &Memory) -> bool {
    memory
        .turns
        .last()
        .is_some_and(|turn| HEAVY.is_match(&turn.user) || heavy::thread_is_heavy(&turn.user))
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = plain(reading.raw);
    let (kind, thing) = taste(&said)?;
    if heavy_previous(memory)
        || compose::fallback::respond(reading, memory).is_some_and(|reply| substantial(&reply))
        || greeting::respond(reading, memory).is_some_and(|reply| substantial(&reply))
    {
        return None;
    }
    let thing = thing
        .strip_prefix("my ")
        .map(|rest| format!("your {rest}"))
        .unwrap_or_else(|| thing.to_owned());
    // The original picks the template by subject before substituting it.
    let pool = match kind {
        Taste::Like => LIKES,
        Taste::Dislike => DISLIKES,
    };
    let chosen = compose::pick(pool, memory, &thing);
    let raised = format!("{}{}", thing[..1].to_ascii_uppercase(), &thing[1..]);
    Some(
        chosen
            .replace("{Thing}", &raised)
            .replace("{thing}", &thing),
    )
}

fn names_words(text: &str, wanted: &HashSet<&str>) -> bool {
    WORD.find_iter(text)
        .any(|word| wanted.contains(word.as_str()))
}

/// Reuse a step from the selected prior advice when a dislike is about that advice.
pub(super) fn advice_obstacle(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !memory
        .turns
        .iter()
        .rev()
        .take(2)
        .any(|turn| turn.module == "answers")
    {
        return None;
    }
    let said = plain(reading.raw);
    if !OBSTACLE.is_match(&said) || heavy_previous(memory) {
        return None;
    }
    let (Taste::Dislike, thing) = taste(&said)? else {
        return None;
    };
    let wanted: HashSet<_> = WORD
        .find_iter(thing)
        .map(|word| word.as_str())
        .filter(|word| {
            !matches!(
                *word,
                "the" | "a" | "an" | "my" | "our" | "and" | "of" | "to" | "for" | "with"
            )
        })
        .collect();
    if wanted.is_empty() {
        return None;
    }

    let current = reading.own_topic.map(|(topic, _)| topic);
    let object = Reading::new(thing).own_topic.map(|(topic, _)| topic);
    let latest = memory
        .turns
        .last()
        .and_then(|turn| Reading::new(&turn.user).own_topic)
        .map(|(topic, _)| topic);

    for (back, turn) in memory.turns.iter().rev().take(2).enumerate() {
        if turn.module != "answers"
            || turn.bot.len() < 50
            || turn.bot.ends_with('?')
            || compose::routes::hedge(&turn.bot)
        {
            continue;
        }
        let topic = Reading::new(&turn.user).own_topic.map(|(topic, _)| topic);
        if (back == 1 && (topic.is_none() || topic != latest))
            || (current.is_some() && topic.is_some() && current != topic)
            || (!names_words(&turn.bot.to_lowercase(), &wanted)
                && (topic.is_none() || object != topic))
        {
            continue;
        }
        if let Some(step) = compose::instruction::instructions(&turn.bot)
            .find(|step| !names_words(&step.to_lowercase(), &wanted))
        {
            let said_before = memory
                .turns
                .iter()
                .rev()
                .take(2)
                .any(|other| other.module == "greeting" && other.bot.contains(step));
            if !said_before {
                let choices: Vec<_> = ALTERNATIVES
                    .iter()
                    .map(|lead| lead.replace("{thing}", thing).replace("{step}", step))
                    .map(|line| format!("{line}."))
                    .collect();
                let lines: Vec<_> = choices.iter().map(String::as_str).collect();
                return Some(compose::pick(&lines, memory, reading.raw).into());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn standalone_tastes_keep_their_subject_and_sign() {
        for (line, kind, thing) in [
            ("I really like jazz", Taste::Like, "jazz"),
            ("Im a massive fan of soup", Taste::Like, "soup"),
            ("I'm not a fan of football", Taste::Dislike, "football"),
            ("I love my new kitchen", Taste::Like, "my new kitchen"),
            (
                "I love black and white films",
                Taste::Like,
                "black and white films",
            ),
        ] {
            assert_eq!(taste(&plain(line)), Some((kind, thing)), "{line}");
        }
        for line in [
            "I love thai food but it's expensive",
            "I like a girl at work",
            "I hate my life",
            "Do I really like jazz?",
            "I love that",
        ] {
            assert!(taste(&plain(line)).is_none(), "{line}");
        }
    }

    #[test]
    fn heavy_news_and_stated_objections_keep_their_turn() {
        let heavy = Memory {
            turns: vec![Exchange {
                user: "My son is being bullied".into(),
                bot: "What happened?".into(),
                module: "answers",
            }]
            .into(),
            ..Memory::default()
        };
        assert!(respond(&Reading::new("I love Thai food"), &heavy).is_none());

        let memory = Memory {
            turns: vec![Exchange {
                user: "How can I lose weight?".into(),
                bot: "Build up exercise gradually. Fill up on protein and fibre at each meal."
                    .into(),
                module: "answers",
            }]
            .into(),
            ..Memory::default()
        };
        let text = Reading::new("I hate exercise");
        let answer = greeting::respond(&text, &memory).unwrap();
        assert!(answer.contains("Fill up on protein and fibre"), "{answer}");
        assert!(respond(&text, &memory).is_none());
    }

    #[test]
    fn receipt_floor_leaves_room_for_a_taste() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let answer = bot.turn("I really like jazz");
        assert!(answer.reply.to_lowercase().contains("jazz"), "{answer:?}");
        assert!(answer.reply.ends_with('?'), "{answer:?}");
    }
}
