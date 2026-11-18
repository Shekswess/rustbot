//! Falls from height and an acute head injury, from PyBot's safety module.
use crate::bot::{Memory, Reading, compose};
use fancy_regex::Regex;
use std::sync::LazyLock;

macro_rules! patterns {
    ($name:ident, $($pattern:literal),+ $(,)?) => {
        static $name: LazyLock<Vec<Regex>> = LazyLock::new(|| {
            [$($pattern),+].into_iter().map(|p| Regex::new(p).expect(stringify!($name))).collect()
        });
    }
}
include!("fall_data.rs");
patterns!(STILL, r"\b(?:now|worse|still|again|getting\s+worse)\b");
patterns!(STILL_HEAD, r"\bstill\b", r"\bright\s+now\b");
patterns!(
    OWN_HEAD,
    r"\bi\s+(?:'ve\s+|had\s+|got\s+)?(?:bang|hit|knock|fell|fall|slip|trip|smack|crack|bash|smash)\w*"
);
patterns!(
    PARTNER_HURT_SPEAKER,
    r"\bmy\s+(?:(?:ex[- ]?)?(?:partner|boyfriend|girlfriend|husband|wife)|ex)\s+(?:(?:just|has|keeps)\s+){0,2}(?:hit(?:ting|s)?|slap(?:ped|ping|s)?|punch(?:ed|ing|es)?|kick(?:ed|ing|s)?|shov(?:ed|ing|es))\s+me\b(?!\s+(?:up\b|with\s+(?:a\s+)?(?:link|message|text|joke|meme|question)\b))"
);

fn any(patterns: &[Regex], text: &str) -> bool {
    patterns.iter().any(|r| r.is_match(text).unwrap_or(false))
}
#[derive(Clone, Copy)]
enum Patient {
    He,
    She,
}
fn person(line: &str, words: &[&str]) -> Option<Patient> {
    if any(&CHILD_ANON, line) {
        let her = words.iter().any(|w| matches!(*w, "she" | "her"));
        if her
            && !words
                .iter()
                .any(|w| matches!(*w, "his" | "i" | "my" | "me"))
        {
            return Some(Patient::She);
        }
        if !her && words.contains(&"his") {
            return Some(Patient::He);
        }
    }
    match (any(&FEMALE_WORDS, line), any(&MALE_WORDS, line)) {
        (true, false) => Some(Patient::She),
        (false, true) => Some(Patient::He),
        _ => None,
    }
}
fn with_sign(line: &str) -> bool {
    (any(&HEAD_FALL, line) || any(&FALL_REPEAT, line))
        && any(&HEAD_FALL_SIGN, line)
        && ![
            &*FALL_ASLEEP,
            &*FALL_WORD_IDIOM,
            &*FALL_THING,
            &*FALL_CHECK_OFF,
        ]
        .iter()
        .any(|p| any(p, line))
        && (!any(&CONFUSION_NAMED_CAUSE, line) || any(&STILL, line))
}
fn head(line: &str) -> Option<bool> {
    if any(&HEAD_BANG, line) {
        if any(&HEAD_SEEN, line) || (any(&HEAD_PAST, line) && !any(&STILL_HEAD, line)) {
            return None;
        }
        return Some(any(&HEAD_RED, line) || any(&FAINT, line));
    }
    if with_sign(line) {
        return Some(true);
    }
    let knockout = any(&HEAD_KNOCKOUT, line)
        || (!any(&OUT_COLD_ASLEEP, line) && !any(&OUT_COLD_NOW, line) && any(&HEAD_OUT_COLD, line));
    (knockout
        && !any(&HEAD_SEEN, line)
        && (!any(&HEAD_PAST, line) || any(&STILL_HEAD, line))
        && !any(&TOOTH_OUT, line))
    .then_some(true)
}
pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = reading.clean.to_lowercase().replace(['’', '‘'], "'");
    if super::the_detail_the_thread_adds::heavy::only_a_dream(&clean)
        || any(&NOT_NOW_FICTION, &clean)
        || any(&NOT_NOW, &clean)
    {
        return None;
    }
    let partner_assault = any(&PARTNER_HURT_SPEAKER, &clean);
    if (any(&HEAD_BANG, &clean) || partner_assault)
        && !any(&HIT_IN_PLAY, &clean)
        && !any(&HIT_DENIED, &clean)
        && !any(&HIT_THING, &clean)
    {
        if any(&SPEAKER_HIT, &clean) {
            return Some(compose::pick(ABUSE_OWN_REPLIES, memory, "").to_owned());
        }
        if partner_assault || any(&HIT_SPEAKER, &clean) {
            return Some(compose::pick(ABUSE_REPLIES, memory, "").to_owned());
        }
    }
    let words: Vec<_> = clean
        .split(|c: char| !c.is_ascii_lowercase() && c != '\'')
        .filter(|w| !w.is_empty())
        .collect();
    if any(&FALL_CORE, &clean)
        && (!any(&FALL_FINE, &clean) || any(&FALL_RED, &clean))
        && (!any(&FALL_PAST, &clean) || any(&FALL_NOW, &clean))
        && (!any(&FALL_REPEAT, &clean)
            || with_sign(&clean)
            || any(&FALL_NOW, &clean)
            || any(&FALL_RED, &clean)
            || any(&FALL_PATTERN_HURT, &clean))
        && (!any(&NEAR_MISS, &clean) || any(&NEAR_MISS_HARM, &clean))
        && (!any(&PET_FELL, &clean) || any(&HUMAN_TOO, &clean))
        && !any(&FALL_WORD_IDIOM, &clean)
        && !any(&FALL_THING, &clean)
    {
        let who = person(&clean, &words);
        let (base, fitted) = match who {
            Some(Patient::He) => (FALL_THEY, FALL_HE),
            Some(Patient::She) => (FALL_THEY, FALL_SHE),
            None if words.contains(&"i") || words.contains(&"my") => {
                (FALL_SELF_THEY, FALL_SELF_THEY)
            }
            None => (FALL_THEY, FALL_THEY),
        };
        let chosen = compose::pick(base, memory, "");
        let index = base.iter().position(|p| *p == chosen)?;
        return Some(fitted[index].to_owned());
    }
    let red = head(&clean)?;
    if any(&PET_WORDS, &clean) && !any(&HUMAN_ANCHOR, &clean) {
        return None;
    }
    let who = if any(&OWN_HEAD, &clean) {
        None
    } else {
        person(&clean, &words).or_else(|| {
            if words.iter().any(|w| matches!(*w, "his" | "him" | "he")) {
                Some(Patient::He)
            } else if words.iter().any(|w| matches!(*w, "her" | "she")) {
                Some(Patient::She)
            } else {
                None
            }
        })
    };
    let pool = match (red, who.is_some()) {
        (true, true) => HEAD_BANG_RED_OTHER_REPLIES,
        (true, false) => HEAD_BANG_RED_REPLIES,
        (false, true) => HEAD_BANG_OTHER_REPLIES,
        (false, false) => HEAD_BANG_REPLIES,
    };
    let chosen = compose::pick(pool, memory, "");
    Some(match who {
        Some(Patient::He) => chosen.replace("{s}", "he").replace("{o}", "him"),
        Some(Patient::She) => chosen.replace("{s}", "she").replace("{o}", "her"),
        None => chosen.to_owned(),
    })
}

// A sign introduced on the next turn inherits the emergency response already
// delivered. The original fallback consults the selected reply, including when
// another reader proposed a routine response to the initial fall.
pub(super) fn continuing_sign(clean: &str, memory: &Memory) -> bool {
    let Some(last) = memory.turns.last().filter(|t| t.module == "safety") else {
        return false;
    };
    if !any(&HEAD_FALL_SIGN, clean)
        || any(&CONFUSION_NAMED_CAUSE, clean)
        || any(&FALL_CHECK_OFF, clean)
    {
        return false;
    }
    let told_to_call = last.bot.contains("999") || last.bot.contains("911");
    told_to_call
        && (any(&FALL_CORE, &last.user.to_lowercase())
            || any(&HEAD_BANG, &last.user.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;
    mod fixture {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/falls.rs"
        ));
        pub(super) fn cases() -> impl Iterator<Item = (&'static str, Option<&'static str>)> {
            CASES
                .iter()
                .map(|&(line, expected)| (line, expected.map(|i| REPLIES[i])))
        }
    }
    #[test]
    fn source_fall_and_head_injury_proposals() {
        let mut differences = Vec::new();
        for (line, wanted) in fixture::cases() {
            let bot = crate::Bot::new(Some("0")).unwrap();
            let reading = Reading::new(line);
            let got = reply(&reading, &bot.memory);
            if got.as_deref() != wanted {
                differences.push((line, got, wanted));
            }
        }
        assert!(differences.is_empty(), "{differences:#?}");
    }
}
