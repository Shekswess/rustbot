//! Wording for requests, interview follow-ups, and spelling.

use std::{borrow::Cow, collections::HashMap, sync::LazyLock};

use crate::{
    bot::{
        Exchange, Reading,
        understanding::{context, details, line},
    },
    text::is_space,
};

use super::{
    COMFORT_LAST, COMFORT_REPLIES, COMFORT_UNNAMED, EULOGY_LENGTH_WORDS, EULOGY_START_WORDS,
};

#[path = "wording_practical_data.rs"]
mod data;

use data::*;

fn has_any(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| text.contains(phrase))
}

fn said<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    }
}

pub(super) fn asks_for_time_off(reading: &Reading<'_>) -> bool {
    let line = said(reading).to_lowercase();
    !has_any(&line, ALREADY_HAD)
        && !has_any(&line, AWAY_OR_HOME)
        && !has_any(&line, USE_THE_WEEK)
        && (!(has_any(&line, MONTHS_NAMED) || has_any(&line, HOW_LONG))
            || has_any(&line, ASKS_TO_HAVE))
        && has_any(&line, ASKING_FRAME)
}

pub(super) fn time_off(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = said(reading).to_lowercase();
    let reply = match (has_any(&line, EARLY_WORDS), has_any(&line, YES_WORDS)) {
        (true, true) => EARLY_SCRIPT_YES,
        (true, false) => EARLY_SCRIPT,
        (false, true) => TIME_OFF_SCRIPT_YES,
        (false, false) => TIME_OFF_SCRIPT,
    };
    Some(Cow::Borrowed(reply))
}

fn word_char(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

fn has_words(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| {
        text.match_indices(phrase).any(|(at, matched)| {
            !text[..at].chars().next_back().is_some_and(word_char)
                && !text[at + matched.len()..]
                    .chars()
                    .next()
                    .is_some_and(word_char)
        })
    })
}

pub(super) fn eulogy(reading: &Reading<'_>, replies: &'static [&'static str; 3]) -> &'static str {
    let text = reading.line.clean.to_lowercase();
    if has_any(&text, EULOGY_LENGTH_WORDS) {
        replies[0]
    } else if has_any(&text, EULOGY_START_WORDS) {
        replies[1]
    } else {
        replies[2]
    }
}

fn comfort_about(text: &str, previous_words: &str) -> Option<&'static str> {
    let current = format!(" {} ", text.to_lowercase());
    let previous = format!(" {previous_words} ");
    let both = format!("{current}{previous}");
    let matches = |word: &&str| {
        if word.contains(' ') {
            let phrase = format!(" {word} ");
            current.contains(&phrase) || previous.contains(&phrase)
        } else {
            has_words(&both, std::slice::from_ref(word))
        }
    };
    if let Some((_, reply)) = COMFORT_REPLIES
        .iter()
        .find(|(words, _)| words.iter().any(matches))
    {
        return Some(reply);
    }
    (has_words(&both, COMFORT_UNNAMED)
        || COMFORT_REPLIES
            .iter()
            .any(|(words, _)| has_words(&both, words)))
    .then_some(COMFORT_LAST)
}

pub(super) fn comfort(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let prior = if (reading.reference.followup || reading.own_topic.is_none())
        && !reading.features.asks.is_empty()
        && !context::shifts(&reading.line)
        && let Some(turn) = earlier.last()
    {
        details::keywords(&line::normalise(&turn.user).norm)
            .take(8)
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        String::new()
    };
    comfort_about(&reading.line.clean, &prior)
}

pub(super) fn follow_up(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = if reading.line.norm_text.is_empty() {
        reading.raw
    } else {
        &reading.line.norm_text
    };
    let after_interview = has_words(line, INTERVIEW_WORDS);
    if has_words(line, WHEN_WORDS) {
        return Some(Cow::Borrowed(if after_interview {
            FOLLOW_UP_AFTER_INTERVIEW
        } else {
            FOLLOW_UP_CHASE
        }));
    }
    Some(Cow::Owned(
        if !after_interview && has_words(line, CHASE_WORDS) {
            format!("{FOLLOW_UP_CHASE} {FOLLOW_UP_THANKS}")
        } else {
            format!("{FOLLOW_UP_THANKS} {FOLLOW_UP_CHASE}")
        },
    ))
}

static TIPS: LazyLock<HashMap<String, &'static str>> = LazyLock::new(|| {
    let mut tips = HashMap::new();
    for &(right, slips, tip) in SPELLINGS {
        for word in std::iter::once(right).chain(slips.iter().copied()) {
            tips.insert(word.to_owned(), tip);
            if word.len() <= 3 || word.contains(' ') {
                continue;
            }
            for suffix in ["ly", "s", "es", "ing", "ed"] {
                tips.insert(format!("{word}{suffix}"), tip);
            }
            if let Some(stem) = word.strip_suffix('e') {
                for suffix in ["ing", "ed", "ly"] {
                    tips.insert(format!("{stem}{suffix}"), tip);
                }
            }
        }
    }
    tips
});

pub(super) fn spelling(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = reading.line.clean.to_lowercase();
    for word in line.split(|ch| matches!(ch, '?' | ',' | ':') || is_space(ch)) {
        if let Some(tip) = TIPS.get(word.trim_matches('.')) {
            return Some(Cow::Borrowed(tip));
        }
    }
    // The source checks multiword slips in the order of the spelling table.
    SPELLINGS.iter().find_map(|&(_, slips, _)| {
        slips
            .iter()
            .find(|slip| slip.contains(' ') && line.contains(*slip))
            .and_then(|slip| TIPS.get(*slip))
            .map(|tip| Cow::Borrowed(*tip))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comfort_declines_without_a_named_situation() {
        assert_eq!(comfort_about("what should i say to her", ""), None);
        assert!(
            comfort_about("what should i say to her", "friend made redundant")
                .expect("wording for lost work")
                .starts_with("Lead with them")
        );
    }

    #[test]
    fn eulogy_length_precedes_opening_when_both_are_asked() {
        let card =
            crate::lookup_knowledge("give me a short eulogy with an opening").expect("eulogy card");
        assert_eq!(card.id, "write-a-eulogy");
        assert!(
            card.reply
                .starts_with("A eulogy runs about three to five minutes")
        );
    }

    #[test]
    fn having_time_off_and_requesting_it_are_distinct() {
        for (text, expected) in [
            ("can i leave early on friday", true),
            ("id like a day off next week", true),
            ("i need to book a week off in june", true),
            ("i booked a week off for june", false),
            ("i have two weeks off in july", false),
            ("how do i use a week off well", false),
            ("i want a week off the booze", false),
            ("i want to go away for my week off", false),
        ] {
            assert_eq!(asks_for_time_off(&Reading::new(text)), expected, "{text}");
        }
        for (text, expected) in [
            ("can i leave early on friday", EARLY_SCRIPT_YES),
            ("how do i tell my boss i need to leave early", EARLY_SCRIPT),
            ("can i take a day off next week", TIME_OFF_SCRIPT_YES),
            ("i need to book a week off in june", TIME_OFF_SCRIPT),
        ] {
            assert_eq!(time_off(&Reading::new(text)).as_deref(), Some(expected));
        }
    }

    #[test]
    fn interview_and_timing_each_change_which_email_comes_first() {
        for (line, opening) in [
            ("how do i chase a job application", FOLLOW_UP_CHASE),
            ("how do i follow up after an interview", FOLLOW_UP_THANKS),
            (
                "how long after my interview should i follow up",
                FOLLOW_UP_AFTER_INTERVIEW,
            ),
            ("when do i chase my job application", FOLLOW_UP_CHASE),
        ] {
            assert!(
                follow_up(&Reading::new(line)).unwrap().starts_with(opening),
                "{line}"
            );
        }
        assert!(!has_words(
            "nowhere to interviewers",
            &["interview", "where"]
        ));
    }

    #[test]
    fn spelling_prefers_the_first_recognised_word_then_multiword_slips() {
        for (line, opening) in [
            ("how do you spell it: separately?", "Separate has"),
            ("how do i spell definately?", "Definitely has"),
            ("which is correct, wierd or weird?", "Weird breaks"),
            ("how do i spell your welcome", "You're means"),
            ("loose weight followed by your welcome", "You're means"),
            (
                "how do i spell loose weight and correctly write recieve",
                "Receive is",
            ),
        ] {
            assert!(
                spelling(&Reading::new(line)).unwrap().starts_with(opening),
                "{line}"
            );
        }
        assert!(spelling(&Reading::new("how do i spell a completely different word")).is_none());
    }
}
