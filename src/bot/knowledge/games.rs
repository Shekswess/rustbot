//! Game picks, child party activities, and first steps into a hobby.
#[path = "games_data.rs"]
mod data;
#[cfg(test)]
#[path = "games_tests.rs"]
mod tests;

use crate::bot::{Exchange, Reading};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::{borrow::Cow, collections::HashSet, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    Party,
    Card,
    Activity,
    GamePick,
    Solo,
}

macro_rules! patterns {
    ($type:ident; $($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<$type> =
           LazyLock::new(|| $type::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(Regex; CHESS_WORDS, CASTLING_VERB, A_THING_IS_BEING_BAKED, FISH_IS_DINNER,
    PAINTED_THINGS, PAINT_AS_A_THING, PAINT_COLOURS, BUYING_THE_MACHINE,
    ANIMAL_AVOID, ANIMAL_ASKS);
patterns!(Fancy; PET_OR_PROFESSIONAL, PHYSICAL_HARM, GRIEF, A_BABY, REGRET_FIRST,
    NOT_A_DEED, NOBODY_TO_REPAIR);
static AGES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    data::AGE_SHAPES
        .iter()
        .map(|shape| Regex::new(shape).unwrap())
        .collect()
});
static DEEDS: LazyLock<Vec<Fancy>> = LazyLock::new(|| read_patterns(data::DEEDS));
static LAPSES: LazyLock<Vec<Fancy>> = LazyLock::new(|| read_patterns(data::LAPSES));
static CHILD_AGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(\d{1,2})\s*(?:years? old|year old|yo)\b").unwrap());
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());

const CARD_NAMES: &[(&str, &str)] = &[
    ("uno", "uno"),
    ("solitaire", "solitaire"),
    ("patience", "solitaire"),
    ("rummy", "rummy"),
    ("snap", "snap"),
    ("go fish", "go fish"),
    ("blackjack", "blackjack"),
    ("pontoon", "blackjack"),
    ("twenty one", "blackjack"),
];
const IMPROVEMENT: &[&str] = &[
    " get better at ",
    " get good at ",
    " how do i improve ",
    " how can i improve ",
];

fn read_patterns(set: &[&str]) -> Vec<Fancy> {
    set.iter()
        .map(|p| Fancy::new(p).expect("compiled repair cue"))
        .collect()
}

fn contains(line: &str, choices: &[&str]) -> bool {
    choices.iter().any(|word| line.contains(word))
}

fn matches(pattern: &Fancy, line: &str) -> bool {
    pattern.is_match(line).unwrap_or(false)
}

fn get<T: Copy>(table: &[(&str, T)], key: &str) -> Option<T> {
    table
        .iter()
        .find_map(|(name, value)| (*name == key).then_some(*value))
}

pub(super) fn chess_context(reading: &Reading<'_>) -> bool {
    let line = format!(
        "{} {} {}",
        reading.line.clean, reading.line.norm_text, reading.raw
    )
    .to_lowercase();
    CHESS_WORDS.is_match(&line) || CASTLING_VERB.is_match(&line)
}

pub(super) fn party_context(reading: &Reading<'_>) -> bool {
    let line = reading
        .line
        .clean
        .to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if line.is_empty()
        || [
            &*PET_OR_PROFESSIONAL,
            &*PHYSICAL_HARM,
            &*GRIEF,
            &*A_BABY,
            &*REGRET_FIRST,
        ]
        .into_iter()
        .any(|cue| matches(cue, &line))
    {
        return true;
    }
    if LAPSES.iter().any(|cue| matches(cue, &line)) {
        return false;
    }
    matches(&NOT_A_DEED, &line)
        || matches(&NOBODY_TO_REPAIR, &line)
        || !DEEDS.iter().any(|cue| matches(cue, &line))
}

fn party(clean: &str) -> Option<Cow<'static, str>> {
    if !contains(clean, data::PARTY_CHILD_WORDS)
        || contains(clean, &["adult", "grown up", "grown-up"])
    {
        return None;
    }
    let age = AGES
        .iter()
        .find_map(|cue| {
            let found: u8 = cue.captures(clean)?.get(1)?.as_str().parse().ok()?;
            (1..=16).contains(&found).then_some(found)
        })
        .or_else(|| {
            contains(clean, &["toddler", "toddlers", "baby", "babies", "nursery"]).then_some(2)
        });
    let answer = match age {
        Some(1..=4) => data::PARTY_TINY,
        Some(5..=7) => data::PARTY_MIDDLE,
        Some(_) => data::PARTY_OLDER,
        None => data::PARTY_ANY,
    };
    if (age.is_some() || clean.contains("indoor"))
        && contains(clean, &["indoor", "indoors", "inside", "flat"])
    {
        return Some(Cow::Owned(
            answer.replace("round the garden", "round the house"),
        ));
    }
    Some(Cow::Borrowed(answer))
}

fn fold(word: &str) -> String {
    let word = word.to_lowercase().replace('’', "'");
    let word = word.trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"']);
    let size = word.chars().count();
    if let Some(root) = word.strip_suffix("ies").filter(|_| size > 4) {
        return format!("{root}y");
    }
    if let Some(root) = word.strip_suffix("es").filter(|root| {
        size > 4 && root.ends_with('o') || size > 3 && root.ends_with(['s', 'x', 'z', 'h'])
    }) {
        return root.into();
    }
    if size > 2 && !word.ends_with("ss") {
        return word.strip_suffix('s').unwrap_or(word).into();
    }
    word.into()
}

fn dish_is_named(reading: &Reading<'_>) -> bool {
    let clean = format!(
        " {} ",
        reading
            .line
            .words
            .iter()
            .map(|word| fold(word))
            .collect::<Vec<_>>()
            .join(" ")
    );
    data::DISH_ALIASES
        .iter()
        .any(|alias| clean.contains(&format!(" {alias} ")))
}

fn activity(reading: &Reading<'_>) -> Option<&'static str> {
    if dish_is_named(reading) {
        return None;
    }
    let text = format!("{} {}", reading.line.clean, reading.line.norm_text);
    if A_THING_IS_BEING_BAKED
        .captures_iter(&text)
        .any(|found| !data::NOTHING_TO_BAKE.contains(&&found[1]))
    {
        return None;
    }
    let terms: Vec<_> = WORD.find_iter(&text).map(|word| word.as_str()).collect();
    let padded = format!(" {} ", terms.join(" "));
    let terms: HashSet<_> = terms.into_iter().collect();
    let any_named = |words: &[&str]| words.iter().any(|word| terms.contains(word));
    if any_named(data::PAINT_VERBS)
        && (any_named(data::PAINTED_SURFACES)
            || any_named(data::PAINTED_FAULTS)
            || contains(&padded, data::PAINTED_PHRASES))
    {
        return None;
    }
    if let Some((_, said)) = data::ACTIVITY_START
        .iter()
        .find(|(name, _)| padded.contains(&format!(" {name} ")))
    {
        return Some(*said);
    }
    if let Some((verb, named)) = data::ACTIVITY_VERBS
        .iter()
        .find(|(verb, _)| terms.contains(verb))
    {
        if data::FISH_VERBS.contains(verb) && FISH_IS_DINNER.is_match(&text)
            || data::PAINT_VERBS.contains(verb)
                && [&*PAINTED_THINGS, &*PAINT_AS_A_THING, &*PAINT_COLOURS]
                    .into_iter()
                    .any(|cue| cue.is_match(&text))
        {
            return None;
        }
        return get(data::ACTIVITY_START, named);
    }
    let (_, alias) = data::ACTIVITY_OTHER_NAMES
        .iter()
        .find(|(name, _)| padded.contains(&format!(" {name} ")))?;
    get(data::ACTIVITY_START, alias)
}

fn child_in_chat(earlier: &[Exchange]) -> bool {
    earlier.iter().rev().take(8).any(|turn| {
        let text = turn.user.to_lowercase();
        contains(&text, data::CHILD_WORDS)
            && CHILD_AGE
                .captures_iter(&text)
                .filter_map(|age| age[1].parse::<u8>().ok())
                .min()
                .is_none_or(|age| age < 13)
    })
}

fn word_at(line: &str, name: &str) -> bool {
    line.match_indices(name).any(|(at, term)| {
        !line[..at]
            .chars()
            .next_back()
            .is_some_and(|letter| letter.is_ascii_lowercase())
            && !line[at + term.len()..]
                .chars()
                .next()
                .is_some_and(|letter| letter.is_ascii_lowercase())
    })
}

pub(super) fn pick_context(reading: &Reading<'_>, earlier: &[Exchange]) -> bool {
    if reading
        .topic
        .is_some_and(|(topic, _)| topic == "family-and-parenting")
    {
        return false;
    }
    let clean = format!(" {} ", reading.line.clean.replace('-', " "));
    let relies_on_chat = reading.reference.followup || reading.line.words.len() <= 3;
    if relies_on_chat
        && earlier
            .last()
            .is_some_and(|last| contains(&last.user.to_lowercase(), data::A_CROWD))
    {
        return false;
    }
    if contains(&clean, data::PLATFORMS)
        && (BUYING_THE_MACHINE.is_match(&clean) || contains(&clean, data::MACHINE_SHAPES))
    {
        return false;
    }
    !contains(&clean, IMPROVEMENT)
        && (!contains(&clean, data::GAME_JUDGED)
            || !data::NAMED_GAME_TITLES
                .iter()
                .any(|title| word_at(&clean, title)))
}

fn game(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let line = reading.raw;
    if !ANIMAL_AVOID.is_match(line) && ANIMAL_ASKS.is_match(line) {
        return None;
    }
    if child_in_chat(earlier) {
        return Some(data::CHILD_GAME);
    }
    let line = format!("{} {}", reading.line.clean, reading.line.norm_text);
    Some(
        data::GAME_PICKS
            .iter()
            .find_map(|(words, reply)| contains(&line, words).then_some(*reply))
            .unwrap_or(data::DEFAULT_GAME),
    )
}

pub(super) fn reply(
    kind: Reply,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let reply = match kind {
        Reply::Party => return party(&reading.line.clean),
        Reply::Card => {
            let said = format!("{} {}", reading.line.clean, reading.line.norm_text);
            let (_, key) = CARD_NAMES.iter().find(|(key, _)| said.contains(key))?;
            get(data::CARD_GAMES, key)?
        }
        Reply::Activity => activity(reading)?,
        Reply::GamePick => game(reading, earlier)?,
        Reply::Solo if child_in_chat(earlier) => data::CHILD_SOLO,
        Reply::Solo => data::DEFAULT_SOLO,
    };
    Some(Cow::Borrowed(reply))
}
