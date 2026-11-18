//! Count the user's own children from the facts held across the conversation.

use crate::bot::{Memory, Reading, compose};
use fancy_regex::{Captures, Regex};
use mt19937::MT19937;
use std::{borrow::Cow, collections::BTreeSet};

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "kidcount_data.rs"]
mod data;
use data::*;

pattern!(DIGITS, r"[0-9]{1,3}");
pattern!(WORDS, r"[a-z']+");
pattern!(
    OWN_CHILD,
    r"(?i)\b(?P<who>daughter|son|twins|stepson|stepdaughter)s?\b"
);
pattern!(
    AGED_RELATION,
    r"(?i)\b(?:my|our)\s+(?:little\s+|big\s+|eldest\s+|youngest\s+)?(?P<who>son|daughter|child|kid|boy|girl|baby|twins?)\s+(?:is|are|was|were|turned)\s+(?P<age>[0-9]{1,3})\b"
);
pattern!(TALLY_LEAD, r"(?i)\b(?:i|we)(?:'ve|ve|\s+have|\s+got)\b");
pattern!(
    AGE_NOT_A_MEASURE,
    r"(?i)^\s*(?:foot|feet|ft|cm|kilos?|kg|lbs?|pounds?|miles?|weeks?|months?|minutes?|days?|hours?)\b"
);

#[derive(Default)]
pub(crate) struct Facts {
    explicit: Option<usize>,
    corrected_this_turn: Option<usize>,
    relations: Vec<&'static str>,
    group_names: BTreeSet<String>,
}

fn capture<'a>(found: &Captures<'a>, name: &str) -> Option<&'a str> {
    Some(found.name(name)?.as_str())
}

fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn match_start(pattern: &Regex, text: &str) -> bool {
    pattern
        .find(text)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == 0)
}

fn number(text: &str) -> Option<usize> {
    let low = text.to_lowercase();
    if low.starts_with("a couple") {
        Some(2)
    } else {
        low.parse()
            .ok()
            .or_else(|| NUMBER_WORDS.iter().position(|word| *word == low))
    }
}

fn word(number: usize) -> String {
    NUMBER_WORDS
        .get(number)
        .map_or_else(|| number.to_string(), |word| (*word).to_owned())
}

fn upper_first(word: &str) -> String {
    let mut chars = word.chars();
    chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(chars)
        .collect()
}

fn fit_child(word: &str) -> Option<&'static str> {
    CHILD_RELATIONS.iter().find(|role| **role == word).copied()
}

fn capture_number(matches: &Captures<'_>) -> Option<usize> {
    number(capture(matches, "n").or_else(|| capture(matches, "m"))?)
}

fn ages(group: &str) -> Vec<usize> {
    DIGITS
        .find_iter(group)
        .flatten()
        .filter_map(|age| age.as_str().parse().ok())
        .collect()
}

pub(super) fn figures_as_digits(text: &str) -> Cow<'_, str> {
    fn figure(word: &str) -> Option<usize> {
        FIGURE_VALUES
            .iter()
            .position(|value| value.eq_ignore_ascii_case(word))
    }
    fn convert(text: &str) -> Cow<'_, str> {
        FIGURE_WORD.replace_all(text, |found: &Captures<'_>| {
            figure(&found[0]).map_or_else(|| found[0].to_owned(), |number| number.to_string())
        })
    }
    match FIGURE_PAIR.replace_all(text, |found: &Captures<'_>| {
        figure(&found[1]).map_or_else(|| found[0].to_owned(), |number| (20 + number).to_string())
    }) {
        Cow::Borrowed(text) => convert(text),
        Cow::Owned(text) => Cow::Owned(convert(&text).into_owned()),
    }
}

fn count_from_ages(text: &str) -> Option<usize> {
    let repaired = figures_as_digits(text);
    let text = repaired.as_ref();

    if let Some(group) = KID_AGES
        .captures(text)
        .ok()
        .flatten()
        .or_else(|| KID_AGES_AFTER_A_COMMA.captures(text).ok().flatten())
    {
        let count = ages(capture(&group, "ages")?)
            .into_iter()
            .filter(|age| *age <= 110)
            .count();
        if count > 0 {
            return Some(count);
        }
    }

    if let Some(group) = KID_GROUP_AGES_ARE.captures(text).ok().flatten() {
        let found = group.get(0)?;
        if match_start(&AGE_TAIL, &text[found.end()..])
            && !matches(&KID_AGES_NOT, &text[..group.name("ages")?.start()])
        {
            let distinct = ages(capture(&group, "ages")?)
                .into_iter()
                .filter(|age| *age <= 120)
                .collect::<BTreeSet<_>>();
            if !distinct.is_empty() {
                return Some(distinct.len());
            }
        }
    }

    let separate: Vec<_> = AGED_RELATION
        .captures_iter(text)
        .flatten()
        .filter_map(|capture| {
            let tail = &text[capture.get(0)?.end()..];
            if match_start(&AGE_NOT_A_MEASURE, tail) {
                return None;
            }
            let age: usize = self::capture(&capture, "age")?.parse().ok()?;
            Some(age)
        })
        .collect();
    if separate.len() >= 2 && separate.iter().any(|age| *age > 0) {
        let distinct = separate.iter().collect::<BTreeSet<_>>();
        return Some(if distinct.len() >= 2 {
            distinct.len()
        } else {
            separate.len()
        });
    }

    // An age without a named relative can count a child when the speaker explicitly has them.
    if !separate.is_empty() || !matches(&TALLY_LEAD, text) {
        return None;
    }
    let old = YEAR_OLD
        .captures_iter(text)
        .flatten()
        .filter(|found| {
            let Some(span) = found.get(0) else {
                return false;
            };
            let head = &text[..span.start()];
            let age = capture(found, "age").and_then(|age| age.parse::<usize>().ok());
            if !age.is_some_and(|age| age < 18 || matches(&PEOPLE_NAMED_ANYWHERE, text))
                || matches(&OWN_AGE_BEFORE_IT, head)
                || matches(&A_PERSON_AGED, head)
                || matches(&YEAR_OLD_ANIMAL, head)
            {
                return false;
            }
            let tail = &text[span.end()..];
            let neighbour = AFTER_YEAR_OLD.captures(tail).ok().flatten();
            neighbour.is_none_or(|word| {
                capture(&word, "word").is_some_and(|word| {
                    AFTER_YEAR_OLD_OKAY
                        .binary_search(&word.to_lowercase().as_str())
                        .is_ok()
                })
            })
        })
        .count();
    (old > 0).then_some(old)
}

fn stated_count(text: &str) -> Option<usize> {
    if match_start(&KID_ASKING, text) {
        return None;
    }
    if let Some(none) = KID_NONE.find(text).ok().flatten()
        && !matches(&KID_NONE_NOT_AFTER_ALL, &text[none.end()..])
    {
        return Some(0);
    }
    let number = KID_COUNT
        .captures(text)
        .ok()
        .flatten()
        .or_else(|| KID_COUNT_AND.captures(text).ok().flatten());
    match number {
        Some(found) => {
            let tail = &text[found.get(0)?.end()..];
            if match_start(&KID_ELSEWHERE, tail) {
                None
            } else {
                capture_number(&found)
            }
        }
        None => count_from_ages(text),
    }
}

fn corrected_count(text: &str, previous: &str, known: bool) -> Option<usize> {
    if match_start(&KID_ASKING, text) {
        return None;
    }
    let found = COUNT_OF_THEM.captures(text).ok().flatten()?;
    let count = capture_number(&found).filter(|count| (2..=12).contains(count))?;
    if match_start(&COUNT_OF_THEM_ELSEWHERE, &text[found.get(0)?.end()..]) {
        return None;
    }
    let previous = previous.to_lowercase();
    let terms: Vec<_> = WORDS
        .find_iter(&previous)
        .flatten()
        .map(|term| term.as_str())
        .collect();
    let has_children = terms.iter().any(|word| CHILDREN_WORDS.contains(word));
    let has_pets = terms.iter().any(|word| {
        matches!(
            *word,
            "cats" | "cat" | "dogs" | "dog" | "pets" | "pet" | "fish" | "birds"
        )
    });
    (has_children || known && !has_pets).then_some(count)
}

fn bare_count_correction(text: &str, previous: &str) -> Option<usize> {
    let parsed = BARE_NEW_COUNT.captures(text).ok().flatten()?;
    let said = capture(&parsed, "count")?;
    if said.len() > 2 && said.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let new = number(said).filter(|new| (1..=30).contains(new))?;
    let previous_count = stated_count(previous)?;
    if new == previous_count || ANY_NUMBER.find_iter(previous).flatten().take(2).count() > 1 {
        return None;
    }
    Some(new)
}

fn owning_clause(head: &str) -> &str {
    let mut edges = vec![0];
    edges.extend(CLAUSE_EDGE.find_iter(head).flatten().map(|edge| edge.end()));
    edges
        .iter()
        .rev()
        .take(5)
        .copied()
        .find_map(|start| {
            let first = head[start..]
                .split([',', ' '])
                .find(|word| !word.is_empty())?;
            let first: String = first
                .to_lowercase()
                .chars()
                .filter(|c| c.is_ascii_alphabetic() || *c == '\'')
                .collect();
            ANY_SUBJECT
                .contains(&first.as_str())
                .then_some(&head[start..])
        })
        .unwrap_or(&head[edges[edges.len().saturating_sub(5)]..])
}

fn relation_mentions(text: &str) -> impl Iterator<Item = &'static str> + '_ {
    OWN_CHILD.captures_iter(text).flatten().filter_map(|found| {
        let hit = found.get(0)?;
        let head = &text[..hit.start()];
        let tail = &text[hit.end()..];
        if match_start(&IN_LAW_AFTER, tail) {
            return None;
        }
        let possessive = tail.starts_with("'s") || tail.starts_with("s'");
        let following = if possessive { &tail[2..] } else { tail };
        let next = following
            .split_whitespace()
            .map(|word| word.trim_matches(['.', ',', '!', '?', ';', ':', '\'']))
            .find(|word| !word.is_empty())
            .unwrap_or("")
            .to_lowercase();
        if COMPOUND_AFTER.binary_search(&next.as_str()).is_ok()
            && !((possessive || hit.as_str().to_lowercase().ends_with('s'))
                && COMPOUND_THAT_IS_THEIRS
                    .binary_search(&next.as_str())
                    .is_ok())
        {
            return None;
        }
        let before = owning_clause(head);
        if matches(&SOMEONE_ELSE_HAS, before) || matches(&A_RELATION_OWNED, before) {
            return None;
        }
        fit_child(&capture(&found, "who")?.to_lowercase())
    })
}

impl Facts {
    fn remember_relation(&mut self, role: &'static str) {
        if !self.relations.contains(&role) {
            self.relations.push(role);
        }
    }

    pub(crate) fn observe(&mut self, reading: &Reading<'_>, previous: Option<&str>) {
        let text = reading.raw.replace(['’', '‘', 'ʼ'], "'");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        let count = stated_count(&text);
        let corrected = if count.is_none_or(|count| count == 0) {
            corrected_count(
                &text,
                previous.unwrap_or(""),
                self.explicit.is_some_and(|count| count > 0),
            )
        } else {
            None
        };
        self.corrected_this_turn = self
            .explicit
            .and(previous)
            .and_then(|before| bare_count_correction(&text, before));
        if let Some(update) = corrected.or(count).or(self.corrected_this_turn) {
            self.explicit = Some(update);
        }

        for relation in relation_mentions(&text) {
            self.remember_relation(relation);
        }
        if let Some(group) = GROUP_NAMED_NAMES.captures(&text).ok().flatten()
            && let Some(who) = capture(&group, "who").and_then(|who| fit_child(&who.to_lowercase()))
        {
            self.remember_relation(who);
        }
        let repaired = figures_as_digits(&text);
        for found in KID_NAME_AGES.captures_iter(&repaired).flatten() {
            let (Some(hit), Some(rest), Some(role)) = (
                found.get(0),
                capture(&found, "rest"),
                capture(&found, "who").and_then(|who| fit_child(&who.to_lowercase())),
            ) else {
                continue;
            };
            if matches(&FIGURE_AFTER, &repaired[hit.end()..]) {
                continue;
            }
            let pairs: Vec<_> = KID_NAME_AGE.captures_iter(rest).flatten().collect();
            if ages(rest).len() != pairs.len() {
                continue;
            }
            for pair in pairs {
                let (Some(name), Some(age)) = (
                    capture(&pair, "name"),
                    capture(&pair, "age").and_then(number),
                ) else {
                    continue;
                };
                let name = name.to_lowercase();
                if age > 120 || NAMED_GROUP_WORDS.binary_search(&name.as_str()).is_ok() {
                    continue;
                }
                self.remember_relation(role);
                self.group_names.insert(name);
            }
        }
    }
}

fn asks(text: &str) -> bool {
    let Some(rest) = THE_ASK.captures(text).ok().flatten() else {
        return false;
    };
    let Some(rest) = capture(&rest, "rest") else {
        return false;
    };
    matches(&A_CHILD, rest)
        && !matches(&SOMEONE_ELSE, text)
        && !matches(&NOT_THEIR_OWN, text)
        && (matches(&THEIR_OWN, text) || rest.split_whitespace().count() <= 2)
}

fn count(memory: &Memory) -> Option<usize> {
    let family = &memory.family_counts;
    if family.explicit.is_some_and(|count| count > 0) {
        return None;
    }
    let mut relations: BTreeSet<_> = family.relations.iter().copied().collect();
    let mut names = family.group_names.clone();
    for (role, name, _) in memory.personal.family_people() {
        if let Some(child) = fit_child(role) {
            relations.insert(child);
            if let Some(name) = name {
                names.insert(name.trim().to_lowercase());
            }
        }
    }
    let held = relations.len().max(names.len());
    (held >= 2).then_some(held)
}

fn vary<const N: usize>(options: [&str; N], memory: &Memory) -> String {
    let fresh: Vec<_> = options
        .iter()
        .copied()
        .filter(|line| {
            !memory
                .turns
                .iter()
                .rev()
                .take(5)
                .any(|turn| turn.bot == *line)
        })
        .collect();
    let choices = if fresh.is_empty() {
        &options[..]
    } else {
        &fresh
    };
    let turn = memory.turns.len() as u64;
    let words = [turn as u32, (turn >> 32) as u32];
    let mut random = MT19937::new_with_slice_seed(&words[..if words[1] == 0 { 1 } else { 2 }]);
    choices[compose::draw(&mut random, choices.len())].to_owned()
}

pub(crate) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !asks(reading.raw) {
        return None;
    }
    let held = word(count(memory)?);
    let titled = upper_first(&held);
    Some(vary(
        [
            &format!("You've got {held} children."),
            &format!("{titled} children — that's what you've told me."),
            &format!("That makes {held} children you've mentioned."),
        ],
        memory,
    ))
}

pub(crate) fn store_reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(value) = memory.family_counts.corrected_this_turn {
        let noun = if value == 1 { "kid" } else { "kids" };
        return Some(format!("{value} {noun} — got it."));
    }
    let found = KIDS_ASK.find(&reading.line.clean).ok().flatten()?;
    if matches(&KIDS_SOMEONE_ELSE, &reading.line.clean[found.end()..])
        || matches(&KIDS_A_DECISION, &reading.line.clean)
    {
        return None;
    }
    let options = match memory.family_counts.explicit {
        Some(0) => vary(
            [
                "You told me you don't have any kids.",
                "None — you said you don't have any.",
                "You said you haven't got any kids.",
            ],
            memory,
        ),
        Some(1) => vary(
            [
                "You've got one kid.",
                "One kid, as you told me.",
                "You said you've got one kid.",
            ],
            memory,
        ),
        Some(held) => {
            let noun = if held == 2 { "children" } else { "kids" };
            let held = word(held);
            let titled = upper_first(&held);
            vary(
                [
                    &format!("You've got {held} {noun}."),
                    &format!("{titled} {noun} — that's what you told me."),
                    &format!("You said you've got {held} {noun}."),
                ],
                memory,
            )
        }
        None => {
            let named = memory
                .personal
                .family_people()
                .find_map(|(role, _, _)| fit_child(role))
                .or_else(|| memory.family_counts.relations.first().copied());
            if let Some(role) = named {
                vary(
                    [
                        &format!(
                            "You've mentioned your {role} — how many have you got altogether?"
                        ),
                        &format!(
                            "I know about your {role}, but not a number. How many have you got in all?"
                        ),
                    ],
                    memory,
                )
            } else {
                vary(
                    [
                        "You haven't told me how many kids you have yet.",
                        "I don't have a number from you — how many have you got?",
                        "You haven't said how many children you've got.",
                    ],
                    memory,
                )
            }
        }
    };
    Some(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn distinguishes_explicit_counts_from_ages_or_other_peoples_children() {
        for (line, count) in [
            ("i have two kids", Some(2)),
            ("i've got a dog and four kids", Some(4)),
            ("i dont have any kids", Some(0)),
            ("we have a 3 year old and a 7 year old", Some(2)),
            ("my kids are four and seven", Some(2)),
            ("my two girls are 6 and 8", Some(2)),
            ("my son is 6 and my daughter is 8", Some(2)),
            ("how many kids do i have", None),
            ("i dont have kids at home", None),
            ("i have four kids in my class", None),
            ("my washing machine is a 6 year old model", None),
        ] {
            assert_eq!(stated_count(line), count, "{line}");
        }
        assert_eq!(relation_mentions("my sister's son is six").next(), None);
        assert_eq!(
            relation_mentions("my brother has a son and a daughter").count(),
            0
        );
        assert_eq!(
            bare_count_correction("actually just one", "i have three kids"),
            Some(1)
        );
        assert_eq!(
            bare_count_correction("no, 4", "i have two kids and 3 dogs"),
            None
        );
    }

    #[test]
    fn count_recalls_named_siblings_once_and_varies_the_pinned_wording() {
        let mut bot = Bot::new(Some("0")).unwrap();
        for line in ["my son Jack is 6", "my son Sam is 9", "my son Jack is 6"] {
            bot.turn(line);
        }
        for expected in [
            "You've got two children.",
            "Two children — that's what you've told me.",
            "That makes two children you've mentioned.",
        ] {
            let turn = bot.turn("how many kids do i have");
            assert_eq!(turn.module, Some("kid_tally"));
            assert_eq!(turn.reply, expected);
        }
        let turn = bot.turn("how many kids does my sister have");
        assert!(
            turn.candidates
                .iter()
                .all(|candidate| candidate.module != "kid_tally")
        );
    }

    #[test]
    fn an_explicit_count_keeps_the_original_store_route() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("my son is 6");
        bot.turn("my daughter is 9");
        bot.turn("i have three kids");
        let recall = bot.turn("how many kids do i have");
        assert_eq!(recall.module, Some("memory_store"));
        assert_eq!(recall.reply, "You've got three kids.");
        assert!(
            recall
                .candidates
                .iter()
                .all(|candidate| candidate.module != "kid_tally")
        );
    }

    #[test]
    fn an_unambiguous_count_correction_replaces_the_explicit_count() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("i have two kids");
        let corrected = bot.turn("actually three");
        assert_eq!(corrected.module, Some("memory_store"));
        assert_eq!(corrected.reply, "3 kids — got it.");
        assert_eq!(
            bot.turn("how many kids do i have").reply,
            "You've got three kids."
        );
    }
}
