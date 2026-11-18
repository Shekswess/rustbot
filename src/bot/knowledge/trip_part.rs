//! Put the held destination into an elliptical question about one part of a trip.
use super::{City, city_named, data};
use crate::bot::Exchange;
use regex::Regex;
use std::{collections::BTreeSet, sync::LazyLock};

static PARTS: LazyLock<Vec<(Regex, &str)>> = LazyLock::new(|| {
    [
    (r####"\b(?:stay|staying|hotel|hotels|accommodation|sleep|sleeping|hostel|hostels)\b"####, "where should i stay in {place}"),
    (r####"\b(?:eat|eats|eating|food|restaurant|restaurants|dinner|lunch|breakfast|drink|drinks|bar|bars|cafe|cafes|coffee|pizza|gelato)\b"####, "where should i eat in {place}"),
    (r####"\b(?:see|sights|sightseeing|visit|visiting|museum|museums|gallery|galleries|ruins|tour|tours)\b"####, "what should i see in {place}"),
    (r####"\b(?:days?|long enough|enough time|how long)\b"####, "how many days in {place}"),
    (r####"\b(?:when|month|months|season|weather|climate|temperature|january|february|april|june|july|august|september|october|november|december|spring|summer|autumn|winter)\b"####, "when should i go to {place}"),
    (r####"\b(?:expensive|pricey|cost|costs|costing|afford|budget|spend|how much)\b"####, "is {place} expensive"),
].into_iter().map(|(pattern, ask)| (Regex::new(pattern).unwrap(), ask)).collect()
});
static GOING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r####"\b(?:going|go|goes|off|heading|trip|trips|holiday|holidays|travel|travelling|traveling|visit|visiting|fly|flying|flight|flights|weekend|weekends|break|away|abroad|booked|booking|planning|thinking about|considering|looking at)\b"####).unwrap()
});
static LIVING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r####"\b(?:live|lives|lived|living|moved|moves|moving|relocate|relocated|relocating|grew up|born|home|hometown|my flat|my house)\b"####).unwrap()
});
const FILLER: &[&str] = &[
    "a",
    "about",
    "advice",
    "also",
    "an",
    "and",
    "any",
    "anywhere",
    "are",
    "area",
    "areas",
    "at",
    "be",
    "been",
    "best",
    "but",
    "can",
    "could",
    "couple",
    "decent",
    "did",
    "do",
    "does",
    "eight",
    "eighteen",
    "eleven",
    "enough",
    "few",
    "fifteen",
    "five",
    "for",
    "four",
    "fourteen",
    "get",
    "give",
    "gives",
    "go",
    "going",
    "good",
    "got",
    "had",
    "has",
    "have",
    "here",
    "how",
    "i",
    "in",
    "info",
    "is",
    "it",
    "like",
    "many",
    "me",
    "more",
    "most",
    "much",
    "my",
    "need",
    "needed",
    "needs",
    "neighborhood",
    "neighbourhood",
    "nice",
    "nine",
    "nineteen",
    "now",
    "of",
    "ok",
    "okay",
    "on",
    "one",
    "or",
    "our",
    "part",
    "parts",
    "place",
    "places",
    "pleasant",
    "please",
    "recommend",
    "recommendation",
    "recommendations",
    "recommends",
    "require",
    "required",
    "seven",
    "seventeen",
    "several",
    "should",
    "six",
    "sixteen",
    "so",
    "some",
    "somewhere",
    "spot",
    "spots",
    "stay",
    "suggest",
    "suggestion",
    "suggestions",
    "take",
    "takes",
    "tell",
    "ten",
    "that",
    "the",
    "then",
    "there",
    "thirteen",
    "this",
    "three",
    "to",
    "too",
    "twelve",
    "twenty",
    "two",
    "us",
    "want",
    "wanted",
    "wants",
    "was",
    "we",
    "were",
    "what",
    "whats",
    "when",
    "where",
    "which",
    "who",
    "why",
    "will",
    "worth",
    "would",
    "you",
    "your",
];
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static LETTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z]+").unwrap());

fn city_opening(line: &str) -> Option<&'static City> {
    let tail = line.trim_start().strip_prefix("In ")?;
    data::CITIES.iter().find(|city| {
        tail.strip_prefix(city.title).is_some_and(|rest| {
            rest.starts_with([':', ',', '.', '!', '?'])
                || [" the ", " a ", " an ", " is ", " it ", " you ", " we "]
                    .iter()
                    .any(|start| rest.starts_with(start))
        })
    })
}

fn trip_place(earlier: &[Exchange]) -> Option<&'static City> {
    let mut latest_card = None;
    for turn in earlier.iter().rev() {
        latest_card = latest_card.or_else(|| city_opening(&turn.bot));
        let text = turn.user.to_lowercase();
        let Some(city) = city_named(&text) else {
            continue;
        };
        if GOING.is_match(&text) {
            return latest_card.or(Some(city));
        }
        if LIVING.is_match(&text) {
            return None;
        }
    }
    if earlier
        .iter()
        .any(|turn| LIVING.is_match(&turn.user.to_lowercase()))
    {
        return None;
    }
    latest_card
}

fn question_part(line: &str) -> Option<(&'static str, BTreeSet<&str>)> {
    PARTS
        .iter()
        .filter_map(|(pattern, ask)| Some((pattern.find(line)?.start(), pattern, *ask)))
        .min_by_key(|(at, _, _)| *at)
        .map(|(_, pattern, ask)| {
            let words = pattern
                .find_iter(line)
                .flat_map(|found| LETTER.find_iter(found.as_str()))
                .map(|found| found.as_str())
                .collect();
            (ask, words)
        })
}

pub(in crate::bot::knowledge) fn asked_part(text: &str, earlier: &[Exchange]) -> Option<String> {
    let city = trip_place(earlier)?;
    let line = text
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let line = line.trim_matches([' ', '.', '!', '?']);
    if line.split_whitespace().count() > 8 {
        return None;
    }
    let (template, own_words) = question_part(line)?;
    let words: Vec<_> = WORD.find_iter(line).map(|word| word.as_str()).collect();
    if words
        .windows(2)
        .any(|pair| matches!(pair[0], "a" | "an") && own_words.contains(pair[1]))
    {
        return None;
    }
    if words.iter().any(|word| {
        word.len() >= 3
            && !FILLER.contains(word)
            && !own_words.contains(word)
            && !(template.starts_with("how many days") && matches!(*word, "day" | "days"))
    }) {
        return None;
    }
    Some(template.replace("{place}", city.title))
}

static SWAP_LEAD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r####"(?i)^\W*(?:(?:ok|okay|right|so|yes|yeah|and)\b[\s,]*)*(?:(?:and|but|or)\s+)?(?:(?:what|how)\s+about|what\s+of|and|also|plus)\b[\s,]*"####).unwrap()
});
static SWAP_THING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r####"(?i)^(?:(?:a|an|the|some|any|another)\s+)?([a-z][a-z' -]{0,24})$"####).unwrap()
});
static SWAP_TAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r####"(?i)[\s,]*(?:\b(?:too|as well|also|instead|then|now|though|please|thanks)\b[\s,.!?]*)+$"####).unwrap()
});
static SWAP_HEDGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r####"(?i)[\s,]*(?:\b(?:probably|maybe|perhaps|possibly|definitely|likely|obviously|actually|really|sure|suppose|supposedly|hopefully|honestly|frankly)\b|\b(?:i|we)\s+(?:think|guess|suppose|reckon|imagine|hope)\b|\bof course\b|\bfor now\b|\bright\b|\bok(?:ay)?\b)[\s,.!?]*$"####).unwrap()
});
const SWAP_STOP: &[&str] = &[
    "a",
    "about",
    "after",
    "also",
    "am",
    "an",
    "and",
    "any",
    "anything",
    "are",
    "as",
    "at",
    "be",
    "before",
    "best",
    "better",
    "but",
    "by",
    "can",
    "could",
    "did",
    "do",
    "does",
    "done",
    "for",
    "from",
    "get",
    "give",
    "good",
    "got",
    "had",
    "has",
    "have",
    "he",
    "help",
    "her",
    "here",
    "him",
    "his",
    "how",
    "hows",
    "i",
    "i'm",
    "idea",
    "ideas",
    "if",
    "im",
    "in",
    "instead",
    "into",
    "is",
    "it",
    "it's",
    "its",
    "just",
    "kind",
    "kinds",
    "know",
    "less",
    "like",
    "long",
    "make",
    "many",
    "may",
    "me",
    "might",
    "mine",
    "more",
    "most",
    "much",
    "must",
    "my",
    "need",
    "no",
    "not",
    "now",
    "of",
    "on",
    "one",
    "ones",
    "or",
    "over",
    "please",
    "really",
    "rest",
    "said",
    "same",
    "say",
    "shall",
    "she",
    "should",
    "so",
    "some",
    "something",
    "sort",
    "sorts",
    "tell",
    "than",
    "that",
    "the",
    "the other",
    "the same",
    "them",
    "then",
    "there",
    "these",
    "they",
    "thing",
    "things",
    "think",
    "this",
    "those",
    "time",
    "times",
    "to",
    "us",
    "very",
    "want",
    "was",
    "way",
    "ways",
    "we",
    "were",
    "what",
    "whatre",
    "whats",
    "when",
    "whens",
    "where",
    "wheres",
    "which",
    "who",
    "whos",
    "why",
    "whys",
    "will",
    "with",
    "would",
    "yes",
    "you",
    "your",
    "yours",
];

fn swap_phrase(text: &str) -> Option<&str> {
    let line = text.trim_matches([' ', '?', '!', '.']);
    let lead = SWAP_LEAD.find(line)?;
    let rest = SWAP_TAIL
        .find(&line[lead.end()..])
        .map_or(&line[lead.end()..], |tail| {
            &line[lead.end()..lead.end() + tail.start()]
        });
    let rest = rest.trim_matches([' ', '?', '!', '.', ',']);
    let rest = SWAP_HEDGE
        .find(rest)
        .map_or(rest, |tail| &rest[..tail.start()]);
    let name = SWAP_THING.captures(rest.trim())?.get(1)?.as_str();
    let mut words = name.split_whitespace();
    let head = words.next()?;
    (words.next().is_none()
        && data::DETAIL_PARTS[1].0.contains(&head)
        && !data::ASKING_ONLY.contains(&head))
    .then_some(head)
}

fn stem(word: &str) -> &str {
    if word.len() <= 4 {
        return word;
    }
    ["ies", "es", "s"]
        .into_iter()
        .find_map(|end| word.strip_suffix(end))
        .unwrap_or(word)
}

/// Try the preceding sightseeing question with its detail slot replaced.
pub(in crate::bot::knowledge) fn changed_sight(text: &str, earlier: &[Exchange]) -> Option<String> {
    trip_place(earlier)?;
    let lower = text.to_lowercase();
    let asked = swap_phrase(&lower)?;
    for use_answer in [true, false] {
        for turn in earlier.iter().rev().take(4) {
            let prior = turn
                .user
                .to_lowercase()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if turn.bot.is_empty()
                || !super::SIGHTSEEING_ASK.is_match(&prior)
                || city_named(&prior).is_some()
            {
                continue;
            }
            let answer = turn.bot.to_lowercase();
            let replaced = prior
                .split_whitespace()
                .rev()
                .map(|word| word.trim_matches(['?', '.', '!', ',', ';', ':']))
                .find(|word| {
                    word.len() >= 3
                        && word.chars().all(char::is_alphabetic)
                        && SWAP_STOP.binary_search(word).is_err()
                        && *word != asked
                        && (!use_answer || answer.contains(stem(word)))
                });
            if let Some(replaced) = replaced {
                return Some(prior.replacen(replaced, asked, 1));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn travel_parts_stay_on_the_held_destination() {
        let cases = [
            (
                "Where should I stay?",
                &[(
                    "I am going to Rome next week.",
                    "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner.",
                )][..],
                Some("where should i stay in Rome"),
            ),
            (
                "How many days should I stay?",
                &[(
                    "I am going to Rome next week.",
                    "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner.",
                )][..],
                Some("how many days in Rome"),
            ),
            (
                "Where should I stay for three days?",
                &[(
                    "I am going to Rome next week.",
                    "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner.",
                )][..],
                None,
            ),
            (
                "And in August?",
                &[(
                    "I am going to Rome next week.",
                    "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner.",
                )][..],
                Some("when should i go to Rome"),
            ),
            (
                "Is it expensive?",
                &[(
                    "I am going to Rome next week.",
                    "In Rome: the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner.",
                )][..],
                Some("is Rome expensive"),
            ),
            (
                "What should I eat?",
                &[(
                    "I am staying in Lisbon for the weekend.",
                    "In Lisbon: Alfama on foot and the 28 tram.",
                )][..],
                Some("where should i eat in Lisbon"),
            ),
            (
                "What should I eat in Florence?",
                &[(
                    "I am staying in Lisbon for the weekend.",
                    "In Lisbon: Alfama on foot and the 28 tram.",
                )][..],
                None,
            ),
            (
                "I want to stay in a hostel.",
                &[(
                    "I am staying in Lisbon for the weekend.",
                    "In Lisbon: Alfama on foot and the 28 tram.",
                )][..],
                None,
            ),
            (
                "Can you help me pick a place to stay that would be near a museum?",
                &[(
                    "I am staying in Lisbon for the weekend.",
                    "In Lisbon: Alfama on foot and the 28 tram.",
                )][..],
                None,
            ),
            (
                "Is it expensive?",
                &[("I live in Rome.", "In Rome: the Colosseum is popular.")][..],
                None,
            ),
            (
                "Where should I eat?",
                &[(
                    "Tell me about Lisbon.",
                    "In Lisbon: Alfama on foot and the 28 tram.",
                )][..],
                Some("where should i eat in Lisbon"),
            ),
            ("What should I see?", &[][..], None),
        ];
        for (ask, prior, expected) in cases {
            let earlier: Vec<_> = prior
                .iter()
                .map(|&(user, bot)| Exchange {
                    user: user.into(),
                    bot: bot.into(),
                    module: "answers",
                })
                .collect();
            assert_eq!(asked_part(ask, &earlier).as_deref(), expected, "{ask}");
        }
    }

    #[test]
    fn a_new_sight_replaces_the_slot_in_the_existing_question() {
        let earlier = [
            Exchange {
                user: "I am going to Rome next week.".into(),
                bot: "In Rome: the Colosseum with a timed ticket.".into(),
                module: "answers",
            },
            Exchange {
                user: "What should I see first?".into(),
                bot: "Start with the Colosseum with a timed ticket, then the Pantheon.".into(),
                module: "answers",
            },
        ];
        assert_eq!(
            changed_sight("And what about the museums?", &earlier).as_deref(),
            Some("what should i see museums?")
        );
        assert!(changed_sight("What about museum opening times?", &earlier).is_none());
        assert!(changed_sight("And what about the museums?", &earlier[1..]).is_none());
    }
}
