//! Narrow source fences for practical health and workstation questions.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

macro_rules! pattern {
    ($name:ident, $source:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(super::$source).expect(stringify!($source)));
    };
}
static FELL_OUT: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(super::SCIENCE_FELL_OUT).expect("valid falling-out pattern")
});
pattern!(STANDING_THING, SCIENCE_STANDING_THING);
pattern!(PERSON_FALL, SCIENCE_PERSON_FALL);
pattern!(FALL_SUBJECT, SCIENCE_FALL_SUBJECT);
pattern!(FALL_PHRASE, SCIENCE_FALL_PHRASE);
pattern!(TIRED, SCIENCE_TIRED);
pattern!(ANOTHER_ADULT, SCIENCE_ADULT_MEMORY);
pattern!(SOMEONE_ELSE, SCIENCE_OTHER_MEMORY);
pattern!(HABIT_THING, SCIENCE_HABIT_THING);
pattern!(MEMORY_ILLNESS, SCIENCE_MEMORY_ILLNESS);
pattern!(OWN_ERRAND, SCIENCE_OWN_ERRAND);
pattern!(DEBT, SCIENCE_MEMORY_DEBT);
pattern!(BABY_AGE, SCIENCE_BABY_AGE);
pattern!(BABY_WORRY, SCIENCE_BABY_SLEEP_WORRY);

fn has(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|word| text.contains(word))
}

fn public_fall(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let text = format!("{} {} {}", line.clean, line.text, reading.raw).to_lowercase();
    let text: String = text
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '\'' | ' ') {
                ch
            } else {
                ' '
            }
        })
        .collect();
    !FELL_OUT.is_match(&text).unwrap_or(false)
        && !FALL_PHRASE.is_match(&text)
        && (PERSON_FALL.is_match(&text) || !STANDING_THING.is_match(&text))
        && FALL_SUBJECT.captures_iter(&text).all(|m| {
            let subject = &m[1];
            matches!(subject, "i" | "we" | "you")
                || PERSON_FALL.is_match(subject)
                || super::SCIENCE_FALL_NON_SUBJECT.contains(&subject)
        })
}

fn sunscreen(text: &str) -> bool {
    let sun = has(text, &["sun", "spf"]);
    if text.contains("burn") && !sun && !has(text, &["uv", "factor 30", "factor 50"]) {
        return false;
    }
    if !text.contains("factor") || sun {
        return true;
    }
    let numbers: Vec<_> = text
        .split(|ch: char| ch.is_whitespace() || matches!(ch, '?' | ',' | '.'))
        .filter(|word| !word.is_empty())
        .filter(|word| word.bytes().all(|b| b.is_ascii_digit()))
        .collect();
    numbers.len() <= 1
        && (numbers.is_empty()
            || !text
                .split_whitespace()
                .any(|word| matches!(word, "of" | "divisible" | "have" | "has")))
        && numbers
            .iter()
            .all(|number| super::SCIENCE_SUN_FACTORS.contains(number))
        && !has(text, &["multiple", "highest", "common"])
}

pub(super) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    match id {
        "falls-elderly" => public_fall(reading),
        "always-tired" => {
            TIRED.is_match(text)
                || TIRED.is_match(&reading.line.text)
                || TIRED.is_match(&reading.line.words.join(" "))
        }
        "posture" => {
            !text.is_empty()
                && super::technology_setup::not_a_sleepy_body(reading)
                && (!has(text, super::SCIENCE_POSTURE_PLACE)
                    || text
                        .split_whitespace()
                        .any(|word| super::SCIENCE_POSTURE_KIT.contains(&word)))
        }
        "sunscreen" => sunscreen(text),
        "back-tweak" => !super::body::back_is_direction(text),
        _ => false,
    }
}

pub(super) fn memory_guard(id: &str, reading: &Reading<'_>, earlier: &[Exchange]) -> bool {
    let line = &reading.line;
    let text = if reading.reference.followup
        && let Some(previous) = earlier.last()
    {
        Cow::Owned(format!("{} {}", line.clean, previous.user.to_lowercase()))
    } else {
        Cow::Borrowed(line.clean.as_str())
    };
    let routine = !ANOTHER_ADULT.is_match(&text)
        && HABIT_THING.is_match(&text)
        && !MEMORY_ILLNESS.is_match(&text);
    super::family_repair::not_their_deed(reading)
        && !routine
        && !OWN_ERRAND.is_match(&text)
        && !DEBT.is_match(&text)
        && (id != "own-memory-worry" || !SOMEONE_ELSE.is_match(&text))
}

fn integer(word: &str, names: &[(&str, u32)]) -> Option<u32> {
    word.parse().ok().or_else(|| {
        names
            .iter()
            .find_map(|&(name, value)| (name == word).then_some(value))
    })
}

fn headache_duration(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    let text = words.join(" ");
    if has(&text, super::SCIENCE_HEADACHE_DAILY) {
        return true;
    }
    let mut has_duration = false;
    for (index, word) in words.iter().enumerate() {
        let Some(unit) = integer(word, super::SCIENCE_HEADACHE_UNITS) else {
            continue;
        };
        let mut amount = None;
        for before in words[..index].iter().rev().take(3).map(String::as_str) {
            if matches!(before, "a" | "an") {
                amount = Some(1);
                break;
            }
            if let Some(count) = integer(before, super::SCIENCE_HEADACHE_NUMBERS) {
                amount = Some(count);
                break;
            }
            if !matches!(before, "of" | "the" | "this" | "last" | "past" | "few") {
                break;
            }
        }
        let days = amount.map(|count| count.saturating_mul(unit)).or_else(|| {
            if matches!(word.as_str(), "week" | "weeks")
                && has(
                    &text,
                    &[
                        "all week",
                        "past week",
                        "last week",
                        "over a week",
                        "a week",
                        "this past",
                    ],
                )
            {
                Some(49)
            } else if unit == 1 && text.contains("all day") {
                Some(1)
            } else {
                None
            }
        });
        if let Some(days) = days {
            if days >= 3 {
                return true;
            }
            has_duration = true;
        }
    }
    !has_duration && has(&text, super::SCIENCE_HEADACHE_BARE)
}

fn baby_sleep(reading: &Reading<'_>) -> Cow<'static, str> {
    let text = &reading.line.clean;
    let age = BABY_AGE
        .captures(text)
        .and_then(|m| {
            let n = f64::from(integer(&m["n"], super::SCIENCE_BABY_NUMBERS)?);
            if n <= 0.0 {
                return None;
            }
            Some(match m["unit"].trim_start_matches('-') {
                "week" => n / 4.345,
                "year" => n * 12.0,
                _ => n,
            })
        })
        .or_else(|| text.contains("toddler").then_some(18.0));
    let guide = match age {
        Some(age) if (3.0..7.0).contains(&age) => {
            "By around three to six months some babies sleep for longer stretches at night. Patterns vary, and daytime naps still count."
        }
        Some(age) if (7.0..13.0).contains(&age) => {
            "In the second half of the first year, some babies sleep for longer stretches at night and still need daytime naps. Patterns vary."
        }
        Some(age) if (13.0..36.0).contains(&age) => {
            "Toddlers often still nap during the day. Look at their sleep across the full 24 hours and keep a steady bedtime routine."
        }
        Some(age) if (36.0..72.0).contains(&age) => {
            "Many preschool children no longer nap every day. A steady bedtime and wake-up routine can help."
        }
        _ => {
            "Newborns sleep for short stretches across day and night and wake frequently to feed. The amount varies between babies."
        }
    };
    let lead = if BABY_WORRY.is_match(text) {
        "If your child is sleeping much more than usual, consider how alert they are when awake and contact your GP or NHS 111 if you are worried."
    } else {
        "Follow their usual sleep and feeding cues."
    };
    let tail = if age.is_none_or(|age| age < 12.0) {
        " Get urgent advice if your baby feeds poorly or has fewer wet nappies than usual. Call 999 if they are hard to wake, unusually floppy or struggling to breathe."
    } else {
        ""
    };
    Cow::Owned(format!("{guide} {lead}{tail}"))
}

pub(super) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = match id {
        "falls-elderly" => {
            "Ask your GP or local falls service for a falls assessment if you or an older relative keeps falling. They can review balance, eyesight, medicines and home hazards. Call 999 if the person cannot get up or may have injured their head, neck, back or hip."
        }
        "baby-sleep" => return Some(baby_sleep(reading)),
        "always-tired" => {
            "If tiredness has lasted a few weeks, affects daily life, or is unexplained, speak to your GP. Note how much you sleep and whether anything else has changed. A clinician can check possible causes and advise on tests if needed."
        }
        "parent-memory" => {
            "If a relative's memory is changing or affecting daily life, ask them if they would like help arranging a GP appointment. Note several examples and when they started, and offer to go with them. Call 999 for sudden confusion or possible stroke symptoms."
        }
        "own-memory-worry" => {
            "If memory problems keep happening or affect daily life, speak to your GP. Bring a few examples, when they started and a list of medicines. Several treatable problems can affect memory. Call 999 if there is sudden confusion or possible stroke symptoms."
        }
        "posture" => {
            "Set your screen about an arm's length away with the top at eye level, support your lower back, and keep your feet flat or use a footrest. Keep the keyboard and mouse within reach and change position or take short breaks regularly. Ask about a workstation assessment if you work at a screen."
        }
        "sunscreen" => {
            "Use sunscreen with at least SPF 30 and four-star UVA protection or the UVA circle logo. Apply it generously, follow the label and reapply after swimming or sweating. Use shade, clothing and a hat as well. Keep babies under six months out of direct strong sunlight."
        }
        "back-tweak" => {
            "Keep moving gently as you are able after a back strain and avoid staying in bed for long periods. Ask a pharmacist about suitable pain relief. Get urgent advice if pain is severe or getting worse quickly. Go to A&E for new numbness around your genitals or anus, trouble peeing or controlling your bowel or bladder, or numbness or weakness in both legs."
        }
        "headache-days" if headache_duration(reading) => {
            "Contact your GP if a headache lasts several days, keeps returning or is getting worse. Record when it happens and any medicines you have taken. Call 999 for a sudden, extremely painful headache, or a severe headache with new weakness, confusion, or a high temperature and stiff neck."
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wellbeing_guards_keep_unrelated_falls_and_factors_out() {
        for (id, line, expected) in [
            ("falls-elderly", "My nan fell out of bed", true),
            ("falls-elderly", "My nan fell out with my mum", false),
            ("sunscreen", "What factor is 15 and 20?", false),
            ("sunscreen", "Should I use factor 50?", true),
            ("posture", "How high should my monitor be?", true),
            ("posture", "Where should I go in Rome?", false),
        ] {
            assert_eq!(passes(id, &Reading::new(line)), expected, "{line}");
        }
        assert!(
            reply(
                "headache-days",
                &Reading::new("I have had a headache for two days")
            )
            .is_none()
        );
        assert!(
            reply(
                "headache-days",
                &Reading::new("I have had a headache for three days")
            )
            .is_some()
        );
    }
}
