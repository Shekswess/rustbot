//! Bedtime and waking time as reported across the last six turns.
use crate::bot::{Memory, compose};
use crate::text::is_space;
use fancy_regex::Regex;
macro_rules! pattern {
    ($name:ident,$source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}
#[path = "asleep_data.rs"]
mod data;
use data::*;

const HOURS: [&str; 15] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve", "thirteen", "fourteen",
];

fn matches(re: &Regex, line: &str) -> bool {
    re.is_match(line).unwrap_or(false)
}
fn end(marker: &Regex, line: &str, bed: bool) -> Option<f64> {
    for hit in marker.find_iter(line).flatten() {
        let rest = &line[hit.end()..];
        let window = &rest[..rest.char_indices().nth(26).map_or(rest.len(), |(i, _)| i)];
        let Some(atom) = ATOM.captures(window).ok().flatten() else {
            continue;
        };
        if bed && matches(&WAKE_MARKER, &window[..atom.get(0)?.start()]) {
            continue;
        }
        if let Some(clock) = atom.name("clock") {
            let h = clock.as_str().parse::<u8>().ok()?;
            if let Some(meridiem) = atom.name("when") {
                // The source returns the meridiem-qualified hour before applying minutes.
                return Some(f64::from(match meridiem.as_str().as_bytes()[0] {
                    b'p' if h < 12 => h + 12,
                    b'a' if h == 12 => 0,
                    _ => h,
                }));
            }
            let minutes = atom
                .name("minutes")
                .map_or(Some(0), |m| m.as_str().parse::<u8>().ok())?;
            let hour = match (bed, h) {
                (true, 12) => 0,
                (true, 6..=11) => h + 12,
                (false, 4..=11) => h,
                _ => return None,
            };
            return Some(f64::from(hour) + f64::from(minutes) / 60.0);
        }
        let named = if let Some(h) = atom.name("half") {
            let hour = HOURS.iter().position(|w| *w == h.as_str())?;
            hour as f64 + if hour >= 12 { 0.5 } else { 12.5 }
        } else if let Some(h) = atom.name("word") {
            HOURS.iter().position(|w| *w == h.as_str())? as f64
        } else if &atom["atom"] == "midnight" {
            0.0
        } else {
            12.0
        };
        if bed {
            return Some(if (6.0..=11.0).contains(&named) {
                named + 12.0
            } else {
                named
            });
        }
        return (4.0..=11.0).contains(&named).then_some(named);
    }
    None
}

fn ends(line: &str) -> (Option<f64>, Option<f64>) {
    let padded = format!(" {} ", line.to_lowercase());
    (
        end(&BED_MARKER, &padded, true),
        end(&WAKE_MARKER, &padded, false),
    )
}
fn someone_else(low: &str) -> bool {
    matches(&SOMEONE_ELSE, low) || matches(&THEY_NOT_ME, low)
}
fn last_bed(memory: &Memory) -> Option<f64> {
    let (mut bed, mut wake) = (None, None);
    for turn in memory.turns.iter().rev().take(6) {
        if turn.user.trim_matches(is_space).is_empty() {
            continue;
        }
        if someone_else(&turn.user.to_lowercase()) {
            return None;
        }
        let (b, w) = ends(&turn.user);
        bed = bed.or(b);
        wake = wake.or(w);
        if bed.is_some() && wake.is_some() {
            break;
        }
    }
    bed
}

pub(crate) fn answer(raw: &str, memory: &Memory) -> Option<String> {
    let words: Vec<_> = raw.split(is_space).filter(|w| !w.is_empty()).collect();
    if words.is_empty() || words.len() > 14 {
        return None;
    }
    let said = words.join(" ");
    let low = said.to_lowercase();
    if matches(&ASKS_A_BEDTIME, &low) || matches(&SOMEONE_ELSE, &low) {
        return None;
    }
    let (bed, wake) = ends(&said);
    let wake = wake?;
    let bed = bed.or_else(|| {
        let short = matches(&I_WAKE, &low) && words.len() <= 6;
        if matches(&THEY_NOT_ME, &low)
            || matches(&NIGHT_WAKING, &low)
            || !(matches(&CARRIES_ON, &low) || !matches(&HAS_A_SUBJECT, &low) || short)
        {
            return None;
        }
        last_bed(memory)
    })?;
    let hours = (wake - bed).rem_euclid(24.0);
    if !(2.0..=14.0).contains(&hours) {
        return None;
    }
    let whole = hours as usize;
    let minutes = ((hours.fract()) * 60.0).round_ties_even() as u8;
    let length = match minutes {
        0 => format!("{} hours", HOURS[whole]),
        30 => format!("{} and a half hours", HOURS[whole]),
        n => format!("{} hours and {n} minutes", HOURS[whole]),
    };
    let pool = if hours < 7.0 {
        SHORT
    } else if hours <= 9.0 {
        FINE
    } else {
        LONG
    };
    let reply = compose::pick(pool, memory, &said).replace("{length}", &length);
    let mut chars = reply.chars();
    Some(chars.next()?.to_uppercase().chain(chars).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;
    mod cases {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/asleep.rs"
        ));
    }
    #[test]
    fn clocks_and_ownership_match_the_reference() {
        let mut memory = Memory::default();
        for &(before, raw, i) in cases::CASES {
            memory.turns.clear();
            memory.turns.extend(before.iter().map(|&user| Exchange {
                user: user.into(),
                bot: String::new(),
                module: "greeting",
            }));
            assert_eq!(
                answer(raw, &memory).as_deref(),
                cases::EXPECTED[i],
                "{raw} after {before:?}"
            );
        }
    }
}
