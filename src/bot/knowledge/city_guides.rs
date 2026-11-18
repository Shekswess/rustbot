//! City guides, existing-guide detail and the next card for a trip.
#[path = "city_guide_data.rs"]
mod data;
#[cfg(test)]
#[path = "city_guide_tests.rs"]
mod tests;
#[path = "trip_part.rs"]
mod trip_part;

use crate::bot::{Exchange, Reading};
use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::{borrow::Cow, collections::BTreeSet, sync::LazyLock};

pub(super) use trip_part::asked_part as trip_question;
pub(super) use trip_part::changed_sight as trip_sight_question;

struct City {
    name: &'static str,
    title: &'static str,
    guide: &'static str,
    food: Option<&'static str>,
    areas: Option<&'static str>,
    day_trips: Option<&'static str>,
    other: &'static [&'static str],
    days: usize,
}

macro_rules! patterns {
    ($type:ident; $($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<$type> =
            LazyLock::new(|| $type::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(Regex; BOOK_ASK, WORTH_ASK, SIGHTSEEING_ASK, QUESTION_OPENS,
    WHICH_FIRST, GOING_TO_A_PLACE, MONTH_RANGE, SEASON_IN_TEXT);
patterns!(Fancy; EAT_ASK, MONTH_IN_TEXT);
patterns!(Regex; EATING, VALUE_EATING);
static DAY_TRIP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bday\s+(?:trip|trips|out)\b").unwrap());
static DAYS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"\b(\d{{1,2}}|{})\s+(?:days|nights)\b",
        data::NUMBER_WORD.join("|")
    ))
    .unwrap()
});

fn word_at(line: &str, word: &str) -> bool {
    line.match_indices(word).any(|(i, word)| {
        let before = i.checked_sub(1).and_then(|i| line.as_bytes().get(i));
        let after = line.as_bytes().get(i + word.len());
        !before.is_some_and(u8::is_ascii_lowercase) && !after.is_some_and(u8::is_ascii_lowercase)
    })
}

pub(super) fn guide_city_named_except(line: &str, excluded: &str) -> bool {
    data::CITIES
        .iter()
        .any(|city| city.name != excluded && word_at(line, city.name))
}

fn title(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

static NAMED_PLACES: LazyLock<[Regex; 3]> = LazyLock::new(|| {
    ["bath", "split", "cork"].map(|name| {
        Regex::new(&format!(concat!(
            r"\b(?:go|goes|going|went|get|getting|travel|travelling|traveling|trip|holiday|",
            r"visit|visiting|stay|staying|fly|flying|from|in|near|see|seeing)\s+(?:to\s+)?{0}\b",
            r"|(?:^|[^a-z]){0}\s+(?:is\s+)?worth|\bis\s+{0}\s+worth|\bworth\s+(?:visiting|seeing|going to)\s+{0}\b",
        ), name)).unwrap()
    })
});

pub(super) fn named_place(line: &str, name: &str) -> bool {
    let ambiguous = match name {
        "bath" => Some(0),
        "split" => Some(1),
        "cork" => Some(2),
        _ => None,
    };
    word_at(line, name) && ambiguous.is_none_or(|index| NAMED_PLACES[index].is_match(line))
}

fn city_named(line: &str) -> Option<&'static City> {
    data::CITIES
        .iter()
        .filter(|city| named_place(line, city.name))
        .fold(None, |best: Option<&City>, city| {
            if best.is_none_or(|old| old.name.len() < city.name.len()) {
                Some(city)
            } else {
                best
            }
        })
}

pub(super) fn names_time(text: &str) -> bool {
    !time_words(text, true).is_empty()
}

pub(super) fn seasonal_reply<'a>(
    text: &'a str,
    clean: &str,
    plan: Option<&str>,
    earlier: &[Exchange],
) -> &'a str {
    let mut when = time_words(clean, true);
    let planned = plan.map(|line| time_words(line, true));
    if let Some(planned) = planned.filter(|time| !time.is_empty()) {
        when.merge(planned);
    } else {
        for turn in earlier.iter().rev().take(2) {
            when.merge(time_words(&turn.user, true));
        }
    }
    remove_other_season(text, &when)
}

fn month(word: &str) -> Option<u16> {
    data::MONTHS
        .iter()
        .find_map(|&(name, n)| (name == word).then_some(n))
}

#[derive(Default)]
struct When {
    months: u16,
    seasons: u8,
}

impl When {
    fn is_empty(&self) -> bool {
        self.months == 0 && self.seasons == 0
    }

    fn merge(&mut self, other: Self) {
        self.months |= other.months;
        self.seasons |= other.seasons;
    }
}

fn time_words(text: &str, loose: bool) -> When {
    let line = text
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let mut time = When::default();
    for captures in MONTH_IN_TEXT.captures_iter(&line).filter_map(Result::ok) {
        if let Some(n) = captures.get(1).and_then(|m| month(m.as_str())) {
            time.months |= 1 << n;
        }
    }
    for &(name, n) in data::MONTHS {
        if (loose && !data::AMBIGUOUS_MONTHS.contains(&name) && word_at(&line, name))
            || (name.len() > 3
                && text.match_indices(&title(name)).any(|(i, name)| {
                    !text[..i].ends_with(char::is_alphabetic)
                        && !text[i + name.len()..].starts_with(char::is_alphabetic)
                }))
        {
            time.months |= 1 << n;
        }
    }
    for (i, (season, _)) in data::SEASON_MONTHS.iter().enumerate() {
        if (season == &"high summer" && line.contains("high summer"))
            || SEASON_IN_TEXT
                .find_iter(&line)
                .any(|m| m.as_str() == *season)
        {
            time.seasons |= 1 << i;
        }
    }
    for captures in MONTH_RANGE.captures_iter(&line) {
        let Some(mut first) = month(&captures[1]) else {
            continue;
        };
        let Some(last) = month(&captures[2]) else {
            continue;
        };
        for _ in 0..12 {
            time.months |= 1 << first;
            if first == last {
                break;
            }
            first = first % 12 + 1;
        }
    }
    time
}

fn time_in_conversation(earlier: &[Exchange], clean: &str) -> When {
    let mut when = time_words(clean, true);
    for turn in earlier.iter().rev().take(2) {
        let added = time_words(&turn.user, true);
        let lower = turn.user.to_lowercase();
        let correction = ["sorry", "no,", "actually"]
            .iter()
            .any(|word| lower.starts_with(word));
        let has_time = !added.is_empty();
        when.merge(added);
        if correction && has_time {
            break;
        }
    }
    when
}

fn fits_season(sentence: &str, when: &When) -> bool {
    let time = time_words(sentence, false);
    time.is_empty()
        || time.months & when.months != 0
        || data::SEASON_MONTHS
            .iter()
            .enumerate()
            .any(|(i, (_, months))| {
                time.seasons & (1 << i) != 0
                    && (months.iter().any(|month| when.months & (1 << month) != 0)
                        || (when.months == 0 && when.seasons & (1 << i) != 0))
            })
}

fn remove_other_season<'a>(text: &'a str, when: &When) -> &'a str {
    if when.is_empty() {
        return text;
    }
    let Some((i, _)) = text
        .match_indices(' ')
        .rev()
        .find(|(i, _)| text[..*i].ends_with(['.', '!', '?']))
    else {
        return text;
    };
    if fits_season(&text[i + 1..], when) {
        text
    } else {
        &text[..i]
    }
}

fn has_any(line: &str, choices: &[&str]) -> bool {
    line.split_whitespace().any(|word| choices.contains(&word))
}

fn said_in_chat(reply: &str, earlier: &[Exchange]) -> bool {
    let head: String = reply.chars().take(40).collect::<String>().to_lowercase();
    !head.is_empty()
        && earlier
            .iter()
            .any(|turn| turn.bot.to_lowercase().contains(&head))
}

fn has_guided(city: &City, earlier: &[Exchange], asked: impl Fn(&str) -> bool) -> bool {
    let head = city
        .guide
        .chars()
        .take(40)
        .collect::<String>()
        .to_lowercase();
    earlier.iter().any(|turn| {
        let said = turn.user.to_lowercase();
        city_named(&said).is_some_and(|named| named.name == city.name)
            && asked(&said)
            && turn.bot.to_lowercase().contains(&head)
    })
}

fn whole_guide(city: &City, body: &str) -> Cow<'static, str> {
    Cow::Owned(format!("In {}: {body}", city.title))
}

fn first_sight(city: &City) -> Option<Cow<'static, str>> {
    let head = city.guide.split(". ").next()?.trim_end_matches('.');
    let mut sights = head
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            if part.to_lowercase().starts_with("and ") {
                &part[4..]
            } else {
                part
            }
        });
    let first = sights.next()?;
    let also = sights
        .next()
        .map_or_else(String::new, |second| format!(", then {second}"));
    let capital = first.chars().next().is_some_and(char::is_uppercase);
    let article = data::A_PLACE_OPENS
        .iter()
        .any(|lead| first.to_lowercase().starts_with(lead));
    let reply = if capital || article {
        format!("Start with {first}{also}.")
    } else {
        format!("{}{also}.", title(first))
    };
    Some(Cow::Owned(reply))
}

fn sight_detail(city: &City, clean: &str) -> Option<Cow<'static, str>> {
    if has_any(clean, data::DETAIL_PARTS[0].0)
        && let Some(food) = city.food
    {
        return Some(Cow::Borrowed(food));
    }
    for &(ask, reply) in data::DETAIL_PARTS {
        if clean
            .split_whitespace()
            .any(|word| ask.contains(&word) && !data::ASKING_ONLY.contains(&word))
            && let Some(sentence) = city
                .guide
                .split(". ")
                .find(|s| has_any(&s.to_lowercase(), reply))
        {
            return Some(Cow::Owned(format!(
                "In {}: {}.",
                city.title,
                sentence.trim_end_matches('.')
            )));
        }
    }
    None
}

fn has_time_to_leave(city: &City, earlier: &[Exchange]) -> bool {
    earlier.iter().any(|turn| {
        let line = turn.user.to_lowercase();
        let count = data::TRIP_UNITS
            .iter()
            .find_map(|&(word, count)| line.contains(word).then_some(count))
            .or_else(|| {
                let captures = DAYS.captures(&line)?;
                let word = &captures[1];
                word.parse().ok().or_else(|| {
                    data::NUMBER_WORD
                        .iter()
                        .position(|&n| n == word)
                        .map(|i| i + 1)
                })
            });
        count.is_some_and(|count| count >= (2 * city.days).max(2))
    })
}

fn next_card(city: &City, clean: &str, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let more = has_any(clean, data::MORE_PLACES)
        || DAY_TRIP.is_match(clean)
        || has_time_to_leave(city, earlier);
    let order = if !more && has_any(clean, data::SEEING) {
        [city.areas, city.day_trips]
    } else {
        [city.day_trips, city.areas]
    };
    order
        .into_iter()
        .flatten()
        .chain(city.other.iter().copied())
        .find(|reply| !said_in_chat(reply, earlier))
        .map(Cow::Borrowed)
}

fn about(
    city: &City,
    reading: &Reading<'_>,
    clean: &str,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    if WHICH_FIRST.is_match(clean)
        && let Some(reply) = first_sight(city)
    {
        return Some(reply);
    }
    if let Some(reply) = sight_detail(city, clean) {
        return Some(reply);
    }
    if !said_in_chat(city.guide, earlier) {
        return Some(whole_guide(city, city.guide));
    }
    if reading.features.asks.is_empty() && !QUESTION_OPENS.is_match(clean) {
        let time = time_in_conversation(earlier, clean);
        return Some(whole_guide(city, remove_other_season(city.guide, &time)));
    }
    next_card(city, clean, earlier)
}

fn ask_words(line: &str) -> BTreeSet<String> {
    line.to_lowercase()
        .split([' ', '?', ','])
        .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()))
        .filter(|word| {
            word.len() >= 3
                && word.chars().all(char::is_alphabetic)
                && !data::NOT_THE_ASK_WORDS.contains(word)
                && !data::CITIES.iter().any(|city| city.name == *word)
        })
        .map(str::to_owned)
        .collect()
}

fn same_ask(clean: &str, earlier: &[Exchange]) -> bool {
    let saved: BTreeSet<_> = earlier
        .iter()
        .filter(|turn| {
            let words = turn.user.to_lowercase();
            GOING_TO_A_PLACE.is_match(&words)
                && !QUESTION_OPENS.is_match(&words)
                && !turn.user.contains('?')
        })
        .flat_map(|turn| ask_words(&turn.user))
        .collect();
    let raw = ask_words(clean);
    let wanted: Vec<_> = raw.difference(&saved).collect();
    !wanted.is_empty()
        && earlier.iter().any(|turn| {
            !turn.bot.is_empty()
                && ask_words(&turn.user)
                    .difference(&saved)
                    .eq(wanted.iter().copied())
        })
}

fn place_before(earlier: &[Exchange], require_travel: bool, depth: usize) -> Option<&'static City> {
    earlier.iter().rev().take(depth).find_map(|turn| {
        let said = turn.user.to_lowercase();
        (!require_travel || GOING_TO_A_PLACE.is_match(&said))
            .then(|| city_named(&said))
            .flatten()
    })
}

pub(super) fn guide(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    if BOOK_ASK.is_match(clean)
        || (WORTH_ASK.is_match(clean) && !time_words(clean, true).is_empty())
    {
        return None;
    }
    let Some(city) = city_named(clean) else {
        return if SIGHTSEEING_ASK.is_match(clean) {
            next(reading, earlier)
        } else {
            None
        };
    };
    if city.food.is_some() && EAT_ASK.is_match(clean).unwrap_or(false) {
        return None;
    }
    let time = time_in_conversation(earlier, clean);
    let body = remove_other_season(city.guide, &time);
    if !reading.features.asks.is_empty()
        && said_in_chat(city.guide, earlier)
        && !same_ask(reading.raw, earlier)
        && !has_guided(city, earlier, |_| true)
    {
        return about(city, reading, clean, earlier);
    }
    Some(whole_guide(city, body))
}

pub(super) fn next(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    if clean
        .split_whitespace()
        .filter(|word| word.chars().all(char::is_alphabetic))
        .count()
        > 8
        || city_named(clean).is_some()
    {
        return None;
    }
    let said = earlier.last()?.bot.to_lowercase();
    if let Some(city) = data::CITIES
        .iter()
        .find(|city| said.contains(&format!("in {}:", city.name)))
    {
        return about(city, reading, clean, earlier);
    }
    let city = place_before(earlier, true, 2).or_else(|| {
        SIGHTSEEING_ASK
            .is_match(clean)
            .then(|| place_before(earlier, true, earlier.len()))
            .flatten()
    })?;
    if said_in_chat(city.guide, earlier) {
        about(city, reading, clean, earlier)
    } else {
        Some(whole_guide(city, city.guide))
    }
}

pub(super) fn is_eating(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    EATING.is_match(clean) || VALUE_EATING.is_match(clean)
}

pub(super) fn cheap_eats(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    let named = data::CHEAP_EATS_ORDER
        .iter()
        .find(|city| clean.contains(**city));
    let some_food = named
        .and_then(|name| data::CITIES.iter().find(|city| city.name == *name)?.food)
        .or_else(|| {
            earlier.iter().rev().find_map(|turn| {
                let said = turn.user.to_lowercase();
                if !GOING_TO_A_PLACE.is_match(&said) {
                    return None;
                }
                city_named(&said)?.food
            })
        });
    if let Some(food) = some_food {
        return Some(Cow::Borrowed(food));
    }
    (!clean.contains("eat there")).then_some(Cow::Borrowed(data::GENERAL_EATS))
}

pub(super) fn followup(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    if city_named(clean).is_some() || !data::FOLLOW_MARKS.iter().any(|mark| clean.contains(mark)) {
        return None;
    }
    let sightseeing = SIGHTSEEING_ASK.is_match(clean);
    let city = place_before(earlier, false, 2).or_else(|| {
        sightseeing
            .then(|| place_before(earlier, true, earlier.len()))
            .flatten()
    })?;
    if sightseeing
        && has_guided(city, earlier, |line| {
            QUESTION_OPENS.is_match(line.trim_start()) || line.contains('?')
        })
    {
        return Some(whole_guide(city, city.guide));
    }
    about(city, reading, clean, earlier)
}

pub(super) fn warm_week_pack(reading: &Reading<'_>) -> &'static str {
    const SUMMER: u16 = (1 << 6) | (1 << 7) | (1 << 8);
    const SHOULDER: u16 = (1 << 5) | (1 << 9) | (1 << 10);
    let said = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    let when = time_words(said, true);
    let summer = data::SEASON_MONTHS
        .iter()
        .enumerate()
        .any(|(index, &(season, _))| {
            matches!(season, "summer" | "high summer") && when.seasons & (1 << index) != 0
        });
    if summer || when.months & SUMMER != 0 {
        data::WARM_SUMMER
    } else if when.months & SHOULDER != 0 {
        data::WARM_SHOULDER
    } else {
        data::WARM_WEEK
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/travel_pack.rs"]
mod pack_fixtures;

#[cfg(test)]
mod pack_tests {
    use super::*;

    #[test]
    fn warm_week_uses_the_same_travel_season_as_pybot() {
        for &(prompt, reply) in pack_fixtures::CASES {
            assert_eq!(warm_week_pack(&Reading::new(prompt)), reply, "{prompt}");
        }
    }
}
