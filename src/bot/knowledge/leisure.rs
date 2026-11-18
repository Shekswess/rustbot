//! The free-outing question's subject and activity filters.

use super::{Form, Key, RawRow};

use crate::{
    bot::{Reading, knowledge::dishes, understanding::details},
    text::is_space,
};
use regex::{Regex, RegexSet};
use std::{
    borrow::Cow,
    cmp::Reverse,
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

macro_rules! pattern {
    ($name:ident, $body:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($body).expect(stringify!($name)));
    };
}
pattern!(FREE_CUE, super::LEISURE_FREE_TIME_CUE_PATTERN);
pattern!(AT_HOME, super::LEISURE_AT_HOME_PATTERN);
pattern!(NO_IDEA, super::LEISURE_IDEA_NONE_PATTERN);
pattern!(IDEAS_ASK, super::LEISURE_IDEAS_ASK_PATTERN);
pattern!(IDEAS_FOR, super::LEISURE_IDEAS_FOR_PATTERN);
pattern!(MUSING, super::LEISURE_MUSING_PATTERN);
pattern!(COLD, super::LEISURE_COLD_PATTERN);
pattern!(COLD_WEATHER, super::LEISURE_COLD_WEATHER_PATTERN);
pattern!(SHORT_RUN, super::LEISURE_SHORT_RUN_PATTERN);
pattern!(PURPOSE, super::LEISURE_PURPOSE_AFTER_RUN_PATTERN);

static ABOUT_IT: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        super::LEISURE_ASKS_ABOUT_IT
            .iter()
            .map(|word| format!(r"\b{}\b", regex::escape(word))),
    )
    .expect("authored complaint frames")
});
static BLOCKS: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| blocks(super::LEISURE_FREE_TIME.iter().copied()));
static LEAVE_BLOCKS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    blocks(
        super::LEISURE_FREE_TIME
            .iter()
            .chain(super::LEISURE_WEEK_OFF)
            .copied(),
    )
});

fn words(line: &str) -> impl Iterator<Item = &str> + Clone {
    line.split(|character: char| !character.is_ascii_lowercase() && character != '\'')
        .filter(|word| !word.is_empty())
}

fn blocks(words: impl Iterator<Item = &'static str>) -> Vec<&'static str> {
    let mut phrases: Vec<_> = words.filter(|word| !word.is_empty()).collect();
    phrases.sort_by_key(|phrase| Reverse(phrase.chars().count()));
    phrases
}

fn bounded_at(line: &str, phrase: &str) -> Option<usize> {
    line.match_indices(phrase).find_map(|(start, _)| {
        let end = start + phrase.len();
        let bytes = line.as_bytes();
        (start
            .checked_sub(1)
            .is_none_or(|before| !bytes[before].is_ascii_alphanumeric())
            && bytes
                .get(end)
                .is_none_or(|after| !after.is_ascii_alphanumeric()))
        .then_some(start)
    })
}

fn covered_words(line: &str, phrases: &[&'static str]) -> HashMap<&'static str, usize> {
    let mut line = line.to_owned();
    let mut covered = HashMap::new();
    for &phrase in phrases {
        while let Some(start) = bounded_at(&line, phrase) {
            line.replace_range(start..start + phrase.len(), " ");
            for word in phrase.split(is_space).filter(|word| !word.is_empty()) {
                *covered.entry(word).or_default() += 1;
            }
        }
    }
    covered
}

fn place_words(line: &str, block: bool) -> HashSet<&str> {
    let words: Vec<_> = words(line).collect();
    let mut places = HashSet::new();
    for (position, preposition) in words.iter().enumerate() {
        if !(super::LEISURE_AT_THE_PLACE_WORDS.contains(preposition)
            || block && super::LEISURE_FOR_THE_PLACE_WORDS.contains(preposition))
        {
            continue;
        }
        let after = &words[position + 1..];
        let after = if after
            .first()
            .is_some_and(|word| super::LEISURE_THE_OWNER.contains(word))
        {
            &after[1..]
        } else {
            after
        };
        if let [first, second, ..] = after
            && super::LEISURE_AT_HOME.contains(&format!("{first} {second}").as_str())
        {
            places.extend([*first, *second]);
        } else if let Some(first) = after
            .first()
            .filter(|word| super::LEISURE_AT_HOME.contains(word))
        {
            places.insert(*first);
        }
    }
    places
}

fn says_free_time(reading: &Reading<'_>, line: &str) -> bool {
    let named = reading
        .context_fields
        .subject
        .as_deref()
        .into_iter()
        .chain(reading.entities.iter().map(|entity| entity.text.as_str()))
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    let free = |word| {
        super::LEISURE_FREE_TIME_WORDS.contains(&word)
            || super::LEISURE_FREE_TIME_WHO.contains(&word)
    };
    let time = |word| super::LEISURE_FREE_TIME_TIME.contains(&word);
    if FREE_CUE.is_match(line)
        && words(&named)
            .all(|word| free(word) || time(word) || super::LEISURE_A_DAY_CONDITION.contains(&word))
    {
        return true;
    }
    let named = AT_HOME.replace_all(&named, " ");
    let holiday = words(&named).any(time);
    let covered = covered_words(line, &BLOCKS);
    let named_time = words(line).any(time);
    let places = place_words(line, !covered.is_empty() || named_time);
    let stays_in_scope = words(&named)
        .filter(|word| !holiday || !super::LEISURE_HOLIDAY_PREFIX.contains(word))
        .all(|word| free(word) || covered.contains_key(word) || places.contains(word));
    stays_in_scope
        && (named_time
            || AT_HOME.is_match(line)
            || !covered.is_empty()
            || reading
                .own_topic
                .is_none_or(|(topic, _)| topic == "small-talk"))
}

fn ideas_about_a_thing(line: &str) -> bool {
    if line.is_empty() || NO_IDEA.is_match(line) {
        return false;
    }
    if IDEAS_ASK.is_match(line) {
        return true;
    }
    let Some(found) = IDEAS_FOR.captures(line) else {
        return false;
    };
    let rest = &found[1];
    let mut choices = words(rest).filter(|word| !super::LEISURE_IDEA_FILLER.contains(word));
    if choices
        .clone()
        .any(|word| matches!(word, "away" | "somewhere" | "abroad"))
    {
        return true;
    }
    let covered = covered_words(rest, &LEAVE_BLOCKS);
    let places = place_words(line, true);
    choices
        .find(|word| !covered.contains_key(word) && !places.contains(word))
        .is_some_and(|word| {
            !super::LEISURE_TIME_WORDS.contains(&word) && !super::LEISURE_NO_SUBJECT.contains(&word)
        })
}

fn about_a_complaint(reading: &Reading<'_>) -> bool {
    if reading.context_fields.subject.is_none()
        || reading
            .own_topic
            .is_none_or(|(topic, _)| !super::LEISURE_A_THING_THE_PERSON_HAS.contains(&topic))
    {
        return false;
    }
    let said = format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.text,
        reading.line.words.join(" ")
    )
    .to_lowercase();
    ABOUT_IT.is_match(&said)
}

pub(super) fn cheap_day(reading: &Reading<'_>) -> bool {
    let line = reading.line.clean.to_lowercase();
    !ideas_about_a_thing(&line) && !about_a_complaint(reading) && says_free_time(reading, &line)
}

static WORK_BLOCKS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    blocks(
        LEAVE_BLOCKS
            .iter()
            .chain(super::LEISURE_WORK_BLOCK)
            .copied(),
    )
});
static ASKED_BLOCKS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    blocks(
        super::LEISURE_FREE_TIME
            .iter()
            .chain(super::LEISURE_WEEK_OFF)
            .chain(super::LEISURE_FREE_TIME_ASK)
            .chain(super::LEISURE_ASKS)
            .chain(super::LEISURE_WORK_BLOCK)
            .copied(),
    )
});
static CITIES: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        super::LEISURE_CITIES
            .iter()
            .map(|city| format!(r"(?:^|[^a-z]){}(?:$|[^a-z])", regex::escape(city))),
    )
    .expect("authored cities with guides")
});
static RECIPES: LazyLock<Vec<(RegexSet, RegexSet)>> = LazyLock::new(|| {
    let set = |words: &[&str]| {
        RegexSet::new(
            words
                .iter()
                .map(|word| format!(r"\b{}\b", regex::escape(word))),
        )
        .expect("authored recipe names and exclusions")
    };
    super::LEISURE_RECIPE_KEYS
        .iter()
        .map(|&(keys, fences)| (set(keys), set(fences)))
        .collect()
});

pub(super) fn free_time(reading: &Reading<'_>) -> bool {
    says_free_time(reading, &reading.line.clean.to_lowercase())
}

fn has(line: &str, choices: &[&str]) -> bool {
    choices.iter().any(|choice| line.contains(choice))
}

fn block_word(word: &str) -> bool {
    [
        super::LEISURE_FREE_BLOCK_WORDS,
        super::LEISURE_ASK_WORDS,
        super::LEISURE_BLOCK_QUALIFIERS,
        super::LEISURE_COUNT_WORDS,
    ]
    .iter()
    .any(|words| words.contains(&word))
}

fn not_wanted(line: &str, word: &str) -> bool {
    let Some(start) = bounded_at(line, word) else {
        return false;
    };
    line[..start]
        .split(is_space)
        .rev()
        .filter(|word| !word.is_empty())
        .take(2)
        .any(|word| super::LEISURE_NOT_THE_THING.contains(&word))
}

fn another_subject(reading: &Reading<'_>, clean: &str) -> bool {
    let mut keywords = details::keywords(&reading.line.norm).map(|word| word.replace('\'', ""));
    if MUSING.is_match(clean)
        || keywords
            .clone()
            .any(|word| super::LEISURE_JOB_WORDS.contains(&word.as_str()))
    {
        return false;
    }
    let block = !covered_words(clean, &WORK_BLOCKS).is_empty()
        || words(clean).any(|word| super::LEISURE_FREE_TIME_TIME.contains(&word));
    let places = place_words(clean, block);
    if let Some(subject) = &reading.context_fields.subject
        && subject
            .to_lowercase()
            .split(|character: char| !character.is_ascii_alphanumeric() && character != '\'')
            .filter(|word| !word.is_empty() && !places.contains(word))
            .any(|word| !block_word(word))
    {
        return true;
    }
    let mut covered = covered_words(clean, &ASKED_BLOCKS);
    keywords.any(|word| {
        if block_word(&word) || word.chars().all(char::is_numeric) || places.contains(word.as_str())
        {
            return false;
        }
        if let Some(count) = covered.get_mut(word.as_str()).filter(|count| **count != 0) {
            *count -= 1;
            return false;
        }
        !not_wanted(clean, &word)
    })
}

fn short_run(line: &str) -> bool {
    SHORT_RUN
        .find(line)
        .is_some_and(|found| !PURPOSE.is_match(&line[found.end()..]))
}

pub(super) fn plan(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = reading.line.words.join(" ").to_lowercase();
    let clean = reading.line.clean.to_lowercase();
    let line = format!("{said} {clean}");
    let named_dish = has(&clean, super::LEISURE_DISH_PLAN_MARKER)
        && !has(&clean, super::LEISURE_DISH_IDEA_ASK)
        && dishes::names_dish(&clean);
    if named_dish
        || super::LEISURE_UNWELL
            .iter()
            .any(|word| bounded_at(&line, word).is_some())
        || COLD.is_match(&line) && !COLD_WEATHER.is_match(&line)
        || has(&line, super::LEISURE_WHERE_GO_PHRASES)
            && [
                super::LEISURE_MONEY_WORDS,
                super::LEISURE_MONEY_ONLY,
                super::LEISURE_WHEN_AWAY,
            ]
            .iter()
            .any(|words| has(&line, words))
        || has(&line, super::LEISURE_TRIP_PHRASES)
        || has(&line, super::LEISURE_MONTHS)
        || CITIES.is_match(&line)
    {
        return None;
    }
    let known_recipe = format!("{clean} {} {said}", reading.line.text.to_lowercase());
    if RECIPES
        .iter()
        .any(|(keys, fences)| keys.is_match(&known_recipe) && !fences.is_match(&known_recipe))
        || [
            super::LEISURE_NOTE_ASK,
            super::LEISURE_EVENT_AWAY,
            super::LEISURE_EVENT_EVENING,
            super::LEISURE_MEDIUM_NAMED,
        ]
        .iter()
        .any(|words| has(&line, words))
    {
        return None;
    }
    if has(
        &line,
        &[
            "newborn",
            "new baby",
            "new-born",
            "just had a baby",
            "had a baby",
            "baby is due",
            "baby due",
            "paternity",
            "maternity leave",
            "on maternity",
        ],
    ) {
        return Some(Cow::Borrowed(super::LEISURE_NEWBORN_WEEK));
    }
    let short = short_run(&line);
    if has(&line, super::LEISURE_REQUESTING_TIME)
        || has(&line, super::LEISURE_WORK_BLOCK) && !short && !has(&line, super::LEISURE_ASKS)
        || ideas_about_a_thing(&clean)
        || about_a_complaint(reading)
        || another_subject(reading, &clean)
        || has(&line, super::LEISURE_GIFT_ASK)
    {
        return None;
    }
    let half_day = has(
        &line,
        &["this afternoon", "free afternoon", "afternoon off"],
    );
    let counted_days = (short || has(&line, &["few days", "couple of days"]))
        && !has(
            &line,
            &[
                "week off",
                "a week off",
                "weeks off",
                "whole week",
                "week free",
            ],
        );
    let plan = if half_day {
        super::LEISURE_DAY_PLAN
    } else if has(&line, &["day off", "days off", "free day"]) {
        super::LEISURE_DAY_OFF_PLAN
    } else if has(&line, super::LEISURE_THE_EVENING) {
        super::LEISURE_EVENING_PLAN
    } else if counted_days {
        super::LEISURE_DAY_PLAN
    } else if has(&line, super::LEISURE_WEEK_OFF) {
        super::LEISURE_WEEK_PLAN
    } else {
        super::LEISURE_WEEKEND_PLAN
    };
    let money = has(&line, super::LEISURE_MONEY_WORDS);
    let mut answer = Cow::Borrowed(plan);
    if plan == super::LEISURE_WEEK_PLAN {
        if money || has(&line, super::LEISURE_MONEY_ONLY) {
            answer = Cow::Borrowed(super::LEISURE_WEEK_PLAN_SKINT);
        } else if has(&line, &["summer", "hot", "heat", "sunny", "sunshine"]) {
            answer = Cow::Owned(format!("{plan} {}", super::LEISURE_SUMMER_WEEK));
        } else if has(&line, &["winter", "autumn", "frost", "snow", "dark"]) {
            answer = Cow::Owned(format!("{plan} {}", super::LEISURE_WINTER_WEEK));
        }
    }
    if money {
        let answer = answer.to_mut();
        answer.push(' ');
        answer.push_str(super::LEISURE_NO_MONEY);
    }
    Some(answer)
}

pub(super) fn rank(
    row: &RawRow,
    groups: Key<'_>,
    line: &[Vec<Form>],
    words: &[String],
) -> Option<i32> {
    const ASK_WORDS: &[&str] = &[
        "what", "whats", "which", "how", "where", "when", "who", "why", "should", "shall", "could",
        "would", "can",
    ];
    let tail = words
        .iter()
        .enumerate()
        .skip(4)
        .rfind(|(_, word)| ASK_WORDS.contains(&word.as_str()))
        .map(|(index, _)| index);
    let best = groups.iter().zip(row.key).filter_map(|(group, &authored)| {
        group
            .iter()
            .zip(authored)
            .enumerate()
            .filter_map(|(index, (phrase, wording))| {
                let (start, end) = phrase.spans(line).next()?;
                let frame = wording
                    .split_whitespace()
                    .all(|word| super::LEISURE_RANK_ASK_FRAME.binary_search(&word).is_ok());
                let weight = if end - start > 1 { 3 } else { 2 };
                Some((
                    weight,
                    !frame,
                    std::cmp::Reverse(index),
                    start,
                    wording,
                    phrase.unique_in_domain,
                ))
            })
            .max_by_key(|&(weight, not_frame, index, ..)| (weight, not_frame, index))
    });
    let mut ranked = [(0, false); 2];
    let mut said = 0;
    for (weight, not_frame, _, start, wording, specific) in best {
        let late = !not_frame && tail.is_some_and(|tail| start >= tail);
        if !late {
            said += weight;
        }
        let has_subject = wording.split_whitespace().any(|word| {
            super::LEISURE_RANK_A_SAYS_NOTHING
                .binary_search(&word)
                .is_err()
        });
        let (total, subject) = &mut ranked[usize::from(late)];
        *total += weight + i32::try_from(specific).unwrap_or(i32::MAX);
        *subject |= has_subject;
    }
    let [(total, subject), (asking, asking_subject)] = ranked;
    let subject = subject || said <= 2 && asking_subject;
    subject.then_some(total + if said > 2 { 0 } else { asking } + row.bias)
}
