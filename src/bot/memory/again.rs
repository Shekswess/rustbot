//! Repeat framing for short facts already on the screen, after PyBot's memory/again.py.
#[path = "again_frames.rs"]
mod frames;
use super::measurement;
use crate::bot::understanding::normalize::{stem, tokens};
use crate::bot::{Exchange, Memory, Reading, compose};
pub(crate) use frames::starting_out_names_something;
pub(super) use frames::{
    already_had, asks_for_move, frame_answer_card, frame_short, instead, reads_as_returning,
    return_target, topic_stopword, unframed, words as topic_words,
};
use regex::Regex;
use std::sync::LazyLock;

const ANSWER_MODULES: &[&str] = &["answers", "loss", "grief", "safety", "urgent"];

static REPLAY_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
    r"^(?:(?:and|so|okay|ok|oh|right|well|sorry|please)\s+)*(?:",
    r"(?:(?:can|could|would|will) you )?(?:(?:say|read|repeat)(?: (?:it|that|this|the last answer))? (?:again|once more|slower|slowly|simpler|clearer)|",
    r"repeat(?: (?:it|that|this|the last answer))?|say (?:it|that))|",
    r"what did you (?:just )?(?:say|tell me)(?: (?:again|earlier|before|previously|now|so far))?|",
    r"what was that|pardon|come again|i (?:missed|didn't catch|didnt catch) (?:that|it)|one more time|",
    r"explain (?:it|that|this)(?: (?:again|once more|slower|more slowly|simply|more simply))?|",
    r"(?:go|walk|talk)(?: me)? (?:over|through) (?:it|that|this)(?: again)?|",
    r"run (?:it|that|this) (?:by me|again|once more)|(?:the )?last (?:bit|part)(?: again)?|",
    r"you(?:'ve| have)? lost me(?: there)?|i lost you)",
    // PyBot folds the conversation wrapper before recognizing a replay request.
    r"(?:[ ,]+(?:of|in|from|for|to)\s+(?:this|our|the)\s+(?:chat|conversation|talk|session|thread)",
    r"(?:\s+(?:so far|today|here|with you|we have had))?)?",
    r"(?:[ ,]+(?:please|now|then|thanks))*[?.!]*$"
)).unwrap()
});
static WHOLE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?:(?:and|so|but|ok|okay|oh|right|well|yes|yeah|ta|please|then)\s+)*(?:",
        r"in (?:the )?full|the full (?:thing|version|answer)|the whole (?:thing|answer|lot|of it)|",
        r"all of (?:it|that|this)|both of (?:them|those)|the lot|",
        r"(?:start (?:again )?)?from the (?:very )?(?:top|start|beginning)|",
        r"(?:say|tell me|give me) (?:it all|it in full|all of it|all of that|the lot))",
        r"(?:[ ,]+(?:please|then|now|thanks|ta))*[?.!]*$"
    ))
    .unwrap()
});
static LAST_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(?:what did you just say|what was that|pardon|come again|i missed (?:that|it)|did(?:n't|nt) catch)\b"
).unwrap()
});
static CHAT_RECALL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?i)^\s*(?:what (?:did|have) i (?:say|said|tell you|told you)|what do you (?:know|remember) about me|(?:can you )?(?:recap|summari[sz]e)|what did we talk about|where were we)\b"
).unwrap()
});

static NAMED_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
    r"^(?:(?:and|so|okay|ok|please) )*(?:what (?:did you (?:say|tell me)|have you said)|remind me what you (?:said|told me)) ",
    r"about (?P<topic>[^?.!]+?)(?: again| earlier| before| please)?[?.!]*$"
)).unwrap()
});

fn nod(line: &str) -> bool {
    const WORDS: &str = "thanks ta thx ty thank you cheers much so very really that this helps helped for the lot got it ok okay k kk great nice brilliant awesome perfect lovely cool fair enough right alright indeed yes yeah yep sure fine good ah oh";
    let words = tokens(line);
    !words.is_empty()
        && words.len() <= 4
        && words
            .iter()
            .all(|word| WORDS.split_ascii_whitespace().any(|nod| nod == word))
}

pub(super) fn asks_for_repeat(text: &str) -> bool {
    let line = text.trim().to_lowercase();
    REPLAY_ASK.is_match(&line) || WHOLE_ASK.is_match(&line)
}

pub(super) fn about_the_chat(line: &str) -> bool {
    let line = line.to_lowercase();
    REPLAY_ASK.is_match(&line)
        || WHOLE_ASK.is_match(&line)
        || CHAT_RECALL.is_match(&line)
        || NAMED_ASK.is_match(&line)
        || super::earlier::chronology_ask(&line)
}

pub(super) fn contentless(line: &str) -> bool {
    let clean = tokens(line).join(" ");
    clean.is_empty()
        || nod(line)
        || matches!(
            clean.as_str(),
            "hello" | "hi" | "hey" | "hiya" | "hiii" | "good morning" | "good evening"
        )
}

fn took_back(line: &str) -> bool {
    let clean = tokens(line).join(" ");
    clean.starts_with("forget the ")
        || clean.starts_with("never mind ")
        || matches!(clean.as_str(), "forget it" | "never mind" | "scratch that")
}

fn answer_to_replay<'a>(memory: &'a Memory, ask: &str) -> Option<&'a Exchange> {
    let history = memory
        .turns
        .iter()
        .rev()
        .filter(|turn| !turn.bot.trim().is_empty());
    let last = history.clone().next()?;
    if LAST_ASK.is_match(ask) {
        return Some(last);
    }
    let active = history
        .clone()
        .find(|turn| !nod(&turn.user) && !about_the_chat(&turn.user) && !took_back(&turn.user))
        .unwrap_or(last);
    if !contentless(&active.user) {
        return Some(active);
    }
    history
        .clone()
        .find(|turn| ANSWER_MODULES.contains(&turn.module))
        .or_else(|| {
            history.clone().find(|turn| {
                !about_the_chat(&turn.user) && turn.bot.split_whitespace().count() >= 8
            })
        })
}

pub(super) fn vary(options: &[&str], memory: &Memory) -> String {
    let recent = memory.turns.iter().rev().take(5);
    let unused: Vec<_> = options
        .iter()
        .copied()
        .filter(|option| !recent.clone().any(|turn| turn.bot == *option))
        .collect();
    let options = if unused.is_empty() {
        options
    } else {
        unused.as_slice()
    };
    compose::seeded_choice(options, memory, "").to_owned()
}

fn named_reply(topic: &str, memory: &Memory) -> Option<String> {
    fn subject_words(text: &str) -> Vec<String> {
        tokens(text)
            .into_iter()
            .filter(|word| {
                !matches!(
                    word.as_str(),
                    "a" | "an"
                        | "the"
                        | "my"
                        | "our"
                        | "your"
                        | "their"
                        | "this"
                        | "that"
                        | "it"
                        | "of"
                        | "for"
                        | "with"
                        | "in"
                        | "on"
                        | "and"
                )
            })
            .map(|word| stem(&word).to_owned())
            .collect()
    }
    let wanted = subject_words(topic);
    if wanted.is_empty() {
        return None;
    }
    for turn in memory.turns.iter().rev() {
        if matches!(turn.module, "safety" | "urgent" | "grief" | "loss") {
            return None;
        }
        if about_the_chat(&turn.user) || contentless(&turn.user) {
            continue;
        }
        let said = turn.bot.trim();
        if !ANSWER_MODULES.contains(&turn.module) && said.split_whitespace().count() < 8 {
            continue;
        }
        let parts = measurement::sentences(said);
        let Some(first) = parts.first().copied() else {
            continue;
        };
        let asked = subject_words(&turn.user);
        let opening = subject_words(first);
        if !(wanted.iter().all(|word| opening.contains(word))
            || wanted.iter().all(|word| asked.contains(word)))
            || opening.iter().all(|word| asked.contains(word))
        {
            continue;
        }
        let opening = if opening.len() <= 2 && parts.len() > 1 {
            format!("{first} {}", parts[1])
        } else {
            first.to_owned()
        };
        let words = opening.split_whitespace();
        let sentence = if words.clone().count() > 45 {
            format!(
                "{}...",
                words
                    .take(45)
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim_end_matches([',', ';', ':'])
            )
        } else {
            opening
        };
        let options = [
            format!("On {topic}: {sentence}"),
            format!("I said: {sentence}"),
            format!("What I said about {topic}: {sentence}"),
        ];
        return Some(compose::pick(&options.each_ref().map(String::as_str), memory, "").into());
    }
    if wanted.len() > 4 {
        return None;
    }
    let mut letters = topic.chars();
    let headline: String = letters
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(letters)
        .collect();
    let options = [
        format!("I don't think we've been over {topic} yet - what do you want to know?"),
        format!("We haven't got to {topic} yet. Ask me and I'll say what I know."),
        format!("{headline} hasn't come up yet - ask away."),
    ];
    Some(compose::pick(&options.each_ref().map(String::as_str), memory, "").into())
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(asked) = NAMED_ASK.captures(&reading.clean) {
        return named_reply(&asked["topic"], memory);
    }
    let whole = WHOLE_ASK.is_match(&reading.clean);
    let explicit_measurement = measurement::explicit_request(&reading.clean);
    if explicit_measurement && measurement::follow_up(reading, memory).is_some() {
        return None;
    }
    if !whole && !explicit_measurement && !REPLAY_ASK.is_match(&reading.clean) {
        return None;
    }
    if memory.turns.is_empty() {
        return (!whole).then(|| {
            vary(
                &[
                    "I haven't said anything yet — you go first.",
                    "Nothing from me so far. What would you like to talk about?",
                ],
                memory,
            )
        });
    }
    let Some(turn) = answer_to_replay(memory, &reading.clean) else {
        return (!whole).then(|| {
            vary(
                &[
                    "Nothing from me on that yet. What were you asking about?",
                    "I haven't given you an answer on that yet. What is it you want to know?",
                    "Nothing of mine to give back there yet - what was the question?",
                ],
                memory,
            )
        });
    };
    let base = turn.bot.trim();
    let base = frames::unframed(base).unwrap_or(base);
    let base = ["Sure — ", "Again — ", "Of course — ", "Here it is again: "]
        .iter()
        .find_map(|lead| base.strip_prefix(lead))
        .unwrap_or(base);
    if !ANSWER_MODULES.contains(&turn.module) {
        return (!whole).then(|| base.to_owned());
    }
    if whole {
        let options = [
            format!("All of it: {base}"),
            format!("Here it is in full: {base}"),
            format!("The whole of it, then: {base}"),
        ];
        return Some(vary(&options.each_ref().map(String::as_str), memory));
    }
    let options = [
        format!("Sure — {base}"),
        format!("Again — {base}"),
        format!("Here it is again: {base}"),
        base.to_owned(),
    ];
    Some(vary(&options.each_ref().map(String::as_str), memory))
}

#[cfg(test)]
mod tests {
    #[test]
    fn explicit_replay_passes_over_acknowledgments_and_does_not_stack_leads() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        assert_eq!(bot.turn("What is 17% of 240?").reply, "17% of 240 is 40.8.");
        bot.turn("Thanks!");
        for line in [
            "Can you say that again?",
            "Can you repeat it?",
            "In full, please.",
        ] {
            let turn = bot.turn(line);
            assert_eq!(turn.module, Some("memory_store"), "{turn:?}");
            assert!(turn.reply.ends_with("17% of 240 is 40.8."), "{turn:?}");
            assert!(!turn.reply.contains("— Sure —") && !turn.reply.contains("— Again —"));
        }
    }

    #[test]
    fn late_replay_skips_the_transcript_lookup() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        let first = bot.turn("What is 17% of 240?");
        assert_eq!(first.reply, "17% of 240 is 40.8.");
        let transcript = bot.turn("What did I say at the start of this chat?");
        assert_eq!(transcript.module, Some("memory_store"), "{transcript:?}");
        assert!(transcript.reply.contains("of 240?"), "{transcript:?}");
        for ask in [
            "What did you say earlier in this chat?",
            "What did you tell me before in our conversation?",
        ] {
            let repeat = bot.turn(ask);
            assert_eq!(repeat.module, Some("memory_store"), "{repeat:?}");
            assert!(repeat.reply.ends_with(&first.reply), "{repeat:?}");
        }
        assert!(!super::asks_for_repeat(
            "What did you say earlier in this chat about Sam?"
        ));
    }
}
