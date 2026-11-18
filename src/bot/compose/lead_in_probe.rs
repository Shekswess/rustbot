//! Source fallback that distinguishes whole-line and stripped-question probes.

use super::{choose_recent, fallback};
use crate::bot::understanding::kinds::Kind;
use crate::bot::{Memory, Reading};
use fancy_regex::Regex;
use std::sync::LazyLock;

#[path = "lead_in_probe_data.rs"]
mod data;
#[path = "lead_in_history.rs"]
mod history;
pub(super) use history::{hedged_recently, method_after_answer};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    A_BARE_ONLINE,
    BARE_PICK,
    BETWEEN_US_ASK,
    LEAD_IN,
    LIVE_HOURS_ASK,
    LIVE_LISTING_ASK,
    LIVE_VENUE_RE,
    LIVE_YES_NO_OPEN,
    NOT_WHAT_I_SAID,
    ONLINE_ASK,
    OWN_DEVICE,
    OWN_TIME_ASK,
    OWN_VERB_OPEN,
    PRIVACY_ASK,
    PRIVACY_NOT_THIS,
    PRIVACY_SHAPES,
);
static POINTER_WORD: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[a-z0-9]+").unwrap());

fn hit(pattern: &Regex, line: &str) -> bool {
    pattern.is_match(line).unwrap_or(false)
}

fn says(line: &str, phrase: &str) -> bool {
    let bytes = line.as_bytes();
    line.match_indices(phrase).any(|(start, _)| {
        (start == 0 || !bytes[start - 1].is_ascii_lowercase())
            && !bytes
                .get(start + phrase.len())
                .is_some_and(u8::is_ascii_lowercase)
    })
}

fn says_any(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| says(line, phrase))
}

fn self_line(clean: &str) -> String {
    let text = clean.replace(['’', '\''], "");
    match LEAD_IN.find(&text).ok().flatten() {
        Some(lead) if lead.end() < text.len() => text[lead.end()..].to_owned(),
        _ => text,
    }
}

fn about(clean: &str) -> Option<&'static [&'static str]> {
    let first = self_line(clean);
    let again = self_line(&first);
    let privacy = [first.as_str(), again.as_str()];
    let names_private = privacy.iter().any(|text| {
        (hit(&PRIVACY_ASK, text) || hit(&PRIVACY_SHAPES, text) || hit(&BETWEEN_US_ASK, text))
            && !hit(&NOT_WHAT_I_SAID, text)
    });
    if names_private
        || hit(&PRIVACY_ASK, &first)
            && !hit(&PRIVACY_NOT_THIS, &first)
            && !hit(&NOT_WHAT_I_SAID, &first)
    {
        return Some(data::PRIVACY_LINES);
    }
    (hit(&ONLINE_ASK, &again) && !hit(&OWN_DEVICE, &again) && !hit(&A_BARE_ONLINE, &first))
        .then_some(data::ONLINE_LINES)
}

fn asks_hours(reading: &Reading<'_>, clean: &str) -> bool {
    if reading.time.0.value() == Some(false)
        || says_any(clean, data::LIVE_NOT)
        || says_any(clean, data::LIVE_NOT_HOURS)
    {
        return false;
    }
    let words = reading
        .line
        .words
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let asking = reading.kind == Kind::Question
        || reading.kind == Kind::Statement && fallback::asks_a_question(clean, reading.raw, &words);
    if !asking && !says_any(clean, data::LIVE_STRONG) {
        return false;
    }
    if hit(&LIVE_VENUE_RE, clean) && (hit(&LIVE_HOURS_ASK, clean) || hit(&LIVE_YES_NO_OPEN, clean))
    {
        return true;
    }
    if hit(&OWN_TIME_ASK, clean)
        && (!(says_any(clean, data::OWN_PLACE) || hit(&OWN_VERB_OPEN, clean))
            || says_any(clean, data::OWN_PERSON))
    {
        return false;
    }
    !(says_any(clean, data::LIVE_SPORT) && hit(&LIVE_LISTING_ASK, clean))
        && says_any(clean, data::HOURS_ASK_WORDS)
}

fn is_bare_do(clean: &str) -> bool {
    let Some(found) = BARE_PICK.captures(clean).ok().flatten() else {
        return false;
    };
    found.name("verb").is_some_and(|verb| verb.as_str() == "do")
        && !clean.trim_start().starts_with("which")
}

fn points_at_nothing(clean: &str) -> bool {
    let clean = clean.replace('\'', "");
    let words = POINTER_WORD
        .find_iter(&clean)
        .map(|word| word.as_str())
        .collect::<Vec<_>>();
    if words.is_empty()
        || words.len() > 6
        || words
            .iter()
            .any(|word| !data::POINTING_WORDS.contains(word))
    {
        return false;
    }
    let names = |options: &[&str]| words.iter().any(|word| options.contains(word));
    let asks_meaning = names(data::MEAN_WORDS);
    if !asks_meaning && names(&["you", "u"]) {
        return false;
    }
    if !asks_meaning
        && !names(data::POINTERS)
        && !data::POINTING_IDIOMS.contains(&words.join(" ").as_str())
    {
        return false;
    }
    asks_meaning || names(data::POINTING_ASK)
}

pub(crate) fn specific(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = reading.clean.trim();
    let pool = about(clean)
        .or_else(|| asks_hours(reading, clean).then_some(data::HOURS_LINES))
        .or_else(|| history::more(clean, memory))
        .or_else(|| {
            memory
                .turns
                .is_empty()
                .then(|| {
                    if is_bare_do(clean) {
                        Some(data::BARE_PICK_DO)
                    } else if points_at_nothing(clean) || points_at_nothing(&reading.line.norm_text)
                    {
                        Some(data::POINTING_LINES)
                    } else {
                        None
                    }
                })
                .flatten()
        })?;
    Some(choose_recent(pool, memory, clean).into())
}
