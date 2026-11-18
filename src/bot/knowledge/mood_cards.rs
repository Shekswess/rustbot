//! The source's mood, panic and ordinary-behaviour questions.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(MACHINE, super::MOOD_MACHINE);
pattern!(FAULT, super::MOOD_MACHINE_FAULT);
pattern!(HOME_FLAT, super::MOOD_HOME_FLAT);
pattern!(HOME, super::MOOD_HOME);
pattern!(COLD_OR_DAMP, super::MOOD_COLD_DAMP);
pattern!(TURNED_DOWN, super::MOOD_TURNED_DOWN);
pattern!(LEFT_FEELING, super::MOOD_HOW_LEFT);
pattern!(DOCTOR, super::MOOD_DOCTOR);
pattern!(SAY_IT, super::MOOD_SAY_IT);
pattern!(
    PANIC_HABIT_VERB,
    r"\b(?:has|have|gets|get|had|having) (?:panic attacks|anxiety attacks|panics)\b"
);

fn has(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn whole(line: &str, options: &[&str]) -> bool {
    options.iter().any(|option| {
        line.match_indices(option).any(|(start, m)| {
            let end = start + m.len();
            (start == 0 || line.as_bytes()[start - 1].is_ascii_whitespace())
                && (end == line.len() || line.as_bytes()[end].is_ascii_whitespace())
        })
    })
}

fn mood_is_persons(reading: &Reading<'_>) -> bool {
    let fall_back = reading.line.clean.split_whitespace();
    let words: Vec<_> = if reading.line.words.is_empty() {
        fall_back
            .filter(|word| !super::MOOD_DETERMINERS.contains(word))
            .collect()
    } else {
        reading
            .line
            .words
            .iter()
            .map(String::as_str)
            .filter(|word| !super::MOOD_DETERMINERS.contains(word))
            .collect()
    };
    if words.contains(&"flat")
        && !words.iter().any(|word| super::MOOD_FEELING.contains(word))
        && has(&words.join(" "), super::MOOD_FLAT_PHRASES)
    {
        return false;
    }
    let mut thing = false;
    for (index, word) in words.iter().enumerate() {
        if !super::MOOD_WORDS.contains(word) {
            continue;
        }
        let before = &words[..index];
        let end = before
            .iter()
            .rposition(|word| !super::MOOD_ADVERB.contains(word));
        let verb = end.and_then(|end| {
            let start = end.saturating_sub(2);
            words[start..=end]
                .iter()
                .rposition(|word| super::MOOD_VERBS.contains(word))
                .map(|offset| start + offset)
        });
        let Some(verb) = verb else {
            if let Some(next) = words.get(index + 1) {
                thing |= !super::MOOD_JOIN.contains(next)
                    && !super::MOOD_TIME.contains(next)
                    && !super::MOOD_PERSON.contains(next);
            }
            continue;
        };
        let between = &words[verb + 1..index];
        let kept = between
            .iter()
            .rposition(|word| !super::MOOD_ADVERB.contains(word))
            .map_or(0, |i| i + 1);
        if Some(verb) != end
            && between[..kept]
                .iter()
                .any(|word| super::MOOD_JOIN.contains(word) || super::MOOD_PERSON.contains(word))
        {
            return true;
        }
        let at =
            if Some(verb) == end || between.iter().any(|word| super::MOOD_ADVERB.contains(word)) {
                words[..verb]
                    .iter()
                    .rposition(|word| !super::MOOD_HELPERS.contains(word))
            } else {
                Some(verb + 1)
            };
        let subject = at.and_then(|at| words.get(at)).copied().unwrap_or("");
        if subject.is_empty()
            || super::MOOD_PERSON.contains(&subject)
            || super::MOOD_VERB_BEFORE.contains(&subject)
            || super::MOOD_JOIN.contains(&subject)
            || super::MOOD_WORDS.contains(&subject)
            || at
                .and_then(|at| at.checked_sub(1))
                .is_some_and(|index| super::MOOD_PLACE_BEFORE.contains(&words[index]))
        {
            return true;
        }
        thing = true;
    }
    !thing
}

pub(super) fn mood_gate(id: &str, reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    match id {
        "lifting-a-low-mood" => {
            mood_is_persons(reading)
                && (!TURNED_DOWN.is_match(text) || LEFT_FEELING.is_match(text))
                && !(HOME.is_match(text) && COLD_OR_DAMP.is_match(text))
        }
        "seasonal-low-mood" => {
            !HOME_FLAT.is_match(text) && !(MACHINE.is_match(text) && FAULT.is_match(text))
        }
        "low-mood" => mood_is_persons(reading) && !(DOCTOR.is_match(text) && SAY_IT.is_match(text)),
        _ => false,
    }
}

pub(super) fn panic_gate(id: &str, reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let text = if line.words.is_empty() {
        Cow::Borrowed(line.clean.as_str())
    } else {
        Cow::Owned(line.words.join(" "))
    };
    if id == "panic-attack-in-the-past" {
        return whole(&text, super::PANIC_MINE)
            || whole(
                &text,
                &[
                    "i had",
                    "ive had",
                    "i have had",
                    "i get",
                    "i used to",
                    "i was having",
                ],
            )
            || whole(&text, super::PANIC_BEHIND_THEM)
            || !(whole(&text, super::PANIC_THEM) || whole(&text, super::PANIC_THEIR_OWN_PAST));
    }
    let once = has(&format!(" {text} "), super::PANIC_HABIT_ONCE);
    let habit =
        PANIC_HABIT_VERB.is_match(&text) || (!once && whole(&text, super::PANIC_HABIT_MARKERS));
    let flat_habit = habit && !whole(&text, super::PANIC_ADVICE);
    if id == "panic-someone-else-habit" {
        flat_habit
    } else {
        !flat_habit
    }
}

pub(super) fn reply(id: &str, _: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = match id {
        "lifting-a-low-mood" => {
            "Try one small activity you can manage today, such as a short walk, a meal or a call to someone you trust. Choose something you can repeat tomorrow. If low mood lasts more than two weeks, you are struggling to cope, or these things do not help, speak to your GP or contact NHS talking therapies."
        }
        "seasonal-low-mood" => {
            "If low mood returns at the same time each year, tell your GP when it starts and how it affects your sleep, energy and daily life. Daylight outdoors, regular activity and a steady sleep routine may help. Ask about treatment options if symptoms keep returning or affect everyday life."
        }
        "low-mood" => {
            "Try talking about how you feel with someone you trust and set a small goal you can manage today. Speak to your GP if low mood lasts more than two weeks, you are struggling to cope, or things you have tried are not helping. Contact NHS 111 for urgent mental health help, or call 999 if someone's life is at immediate risk."
        }
        "panic-someone-else" => {
            "Stay with them if they want you there. Speak calmly, give them space, and ask what usually helps. Encourage slow breathing if they find that useful. Get medical help if you are unsure what is happening, and call 999 for severe breathing difficulty, ongoing chest pain or loss of consciousness."
        }
        "panic-attack-in-the-past" => {
            "After a panic attack you may still feel shaken. Rest somewhere you feel safe and note what happened if that helps. Speak to your GP if attacks recur or you are avoiding activities because you fear another one. New or unexplained symptoms may need medical assessment."
        }
        "panic-someone-else-habit" => {
            "Ask them when they are calm what they would like you to do if it happens again. Agree on a simple plan together and offer to help them talk to a GP or access talking therapies if attacks keep affecting their life."
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}

const ORDINARY_REPLIES: &[&str] = &[
    "Many adults live with their parents. Agree what each person will contribute and what everyone needs for privacy, and set a date to review how it is going.",
    "Talking to yourself on purpose can be a way to organise your thoughts. If you hear voices or sounds that others cannot hear, ask for an urgent GP appointment or call NHS 111. Call 999 if the voices tell you to harm yourself or someone else, you become suddenly confused, or it is getting worse quickly.",
    "A familiar soft toy or blanket can be comforting at bedtime. If it helps you settle and causes no problems, you can keep using it.",
    "Choosing whether to have children is personal. If you are in a relationship, it can help to talk about what each of you wants before either person makes assumptions.",
    "Films can bring out strong emotions. Crying at a film is an ordinary response. If you are often tearful away from films and it is affecting your life, talk to someone you trust or your GP.",
    "It can take time to adjust after a breakup. Talk to someone you trust and seek help from your GP if low mood lasts more than two weeks or you are struggling to cope.",
    "Choosing to skip alcohol is fine. You can simply say, 'Not for me, thanks,' or ask what alcohol-free drinks they have.",
    "There is no deadline for a first relationship. If you want to meet people, choose activities or groups you would enjoy anyway.",
];

pub(super) fn ordinary_answer(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    super::NORMAL_SUBJECT_GROUPS
        .iter()
        .zip(ORDINARY_REPLIES)
        .find_map(|(words, &reply)| has(text, words).then_some(Cow::Borrowed(reply)))
}
