//! Body-region reading used by the pinned ache and injury cards.

use crate::bot::Reading;
use regex::Regex;
use std::sync::LazyLock;

use super::{
    BODY_ALSO_THE_BACK, BODY_BACK_CLAUSE_JOIN, BODY_BACK_DESCRIBED, BODY_BACK_IS_A_NOUN_AFTER,
    BODY_BACK_IS_A_RETURN, BODY_BACK_TAKING_A_THING, BODY_BACK_THEIRS, BODY_BACK_VERB_BEFORE,
    BODY_DETERMINERS, BODY_HURT_WORDS, BODY_KEPT_AWAKE_PATTERN, BODY_OBJECT_OWNERS,
    BODY_PERSON_NOUNS, BODY_POSSESSIVES, BODY_REGION_PARTS, BODY_REPORTED_COMPLAINT_PATTERN,
    BODY_STANDING_TROUBLE, BODY_THING_PARTS, BODY_WORKSHOP_NOUNS,
};
use super::{
    BODY_BACK_AFTER_VERB_PATTERN, BODY_BACK_BEFORE_PLACE_PATTERN, BODY_BACK_HOME_PATTERN,
    BODY_BACK_OWNED_PATTERN, BODY_BACK_WITH_VERB_PATTERN, BODY_CANNOT_USE_PATTERN,
    BODY_NAMED_PARTS, BODY_OTHER_OWNERS, BODY_OWN_WORDS, BODY_PREGNANT_WORDS, BODY_TREATMENTS,
    guards::Context,
};

pub(super) struct Treatment {
    pub(super) part: &'static str,
    pub(super) weeks: [&'static str; 3],
    pub(super) persistent: &'static str,
}

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(CANNOT_USE, BODY_CANNOT_USE_PATTERN);
pattern!(BACK_WITH_VERB, BODY_BACK_WITH_VERB_PATTERN);
pattern!(BACK_OWNED, BODY_BACK_OWNED_PATTERN);
pattern!(BACK_AFTER_VERB, BODY_BACK_AFTER_VERB_PATTERN);
pattern!(BACK, r"\bback\b");
pattern!(WORD, r"[a-z']+");
pattern!(MONTH, r"\bmonth");
pattern!(YEAR, r"\byear");

static BACK_BEFORE_PLACE: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(BODY_BACK_BEFORE_PLACE_PATTERN)
        .expect("valid return to a place pattern")
});
static BACK_HOME: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(BODY_BACK_HOME_PATTERN).expect("valid return home pattern")
});

fn any_word(choices: &[&str]) -> Regex {
    let alternation = choices
        .iter()
        .map(|word| regex::escape(word))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"\b(?:{alternation})\b")).expect("valid known body words pattern")
}

static WORKSHOP: LazyLock<Regex> = LazyLock::new(|| any_word(BODY_WORKSHOP_NOUNS));
static REGIONS: LazyLock<Regex> = LazyLock::new(|| any_word(BODY_REGION_PARTS));
static KEPT_AWAKE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(BODY_KEPT_AWAKE_PATTERN).expect("valid activity loss pattern"));
static REPORTED_COMPLAINT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("(?i){BODY_REPORTED_COMPLAINT_PATTERN}"))
        .expect("valid reported complaint pattern")
});

fn trimmed(word: &str) -> &str {
    word.trim_matches(|c: char| c.is_ascii_punctuation())
}

fn owns_a_part(word: &str) -> bool {
    let word = trimmed(word);
    let word = word.strip_suffix("'s").unwrap_or(word);
    BODY_OBJECT_OWNERS.contains(&word)
        || word
            .strip_suffix('s')
            .is_some_and(|bare| BODY_OBJECT_OWNERS.contains(&bare))
}

pub(super) fn person_part(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    for (index, word) in words.iter().enumerate() {
        if !BODY_THING_PARTS.contains(&trimmed(word)) {
            continue;
        }
        if index > 0 && owns_a_part(&words[index - 1]) {
            return false;
        }
        let tail = &words[index + 1..];
        let next = tail
            .iter()
            .take(3)
            .map(|word| trimmed(word))
            .find(|word| !word.is_empty() && !BODY_DETERMINERS.contains(word));
        if next.is_some_and(|word| !BODY_THING_PARTS.contains(&word) && owns_a_part(word)) {
            return false;
        }
        let between: Vec<_> = tail.iter().take(4).map(|word| trimmed(word)).collect();
        if let Some(cut) = ["in", "inside"]
            .iter()
            .find_map(|prep| between.iter().position(|word| word == prep))
            && !between[..cut]
                .iter()
                .any(|word| BODY_HURT_WORDS.contains(word))
            && between[cut + 1..]
                .iter()
                .copied()
                .find(|word| {
                    !word.is_empty()
                        && !BODY_DETERMINERS.contains(word)
                        && !BODY_POSSESSIVES.contains(word)
                })
                .is_some_and(owns_a_part)
        {
            return false;
        }
        if between.first() == Some(&"of") {
            let rest: Vec<_> = between[1..]
                .iter()
                .copied()
                .filter(|word| !word.is_empty() && !BODY_DETERMINERS.contains(word))
                .collect();
            let skipped = rest
                .iter()
                .take(rest.len().saturating_sub(1))
                .take_while(|word| BODY_POSSESSIVES.contains(word))
                .count();
            let rest = &rest[skipped..];
            if rest.first().is_some_and(|owner| {
                !BODY_PERSON_NOUNS.contains(owner)
                    && !BODY_THING_PARTS.contains(owner)
                    && !["my", "your", "his", "her", "their", "our", "whose"].contains(owner)
            }) || rest.iter().any(|owner| owns_a_part(owner))
            {
                return false;
            }
        }
    }
    true
}

pub(super) fn person_nail(reading: &Reading<'_>) -> bool {
    let line = &reading.line.clean;
    !line.contains("nail") || !WORKSHOP.is_match(line)
}

pub(super) fn part_stops_activity(words: &str) -> bool {
    KEPT_AWAKE.is_match(words) && REGIONS.is_match(words)
}

pub(super) fn back_is_a_body_part(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    if words.is_empty()
        || words
            .iter()
            .any(|word| BODY_ALSO_THE_BACK.contains(&trimmed(word)))
    {
        return true;
    }
    for (index, word) in words.iter().enumerate() {
        if trimmed(word) != "back" {
            continue;
        }
        let before = index
            .checked_sub(1)
            .map(|prev| trimmed(&words[prev]))
            .unwrap_or("");
        let after = words.get(index + 1).map(|word| trimmed(word)).unwrap_or("");
        if BODY_BACK_IS_A_NOUN_AFTER.contains(&after) || BODY_BACK_THEIRS.contains(&before) {
            return true;
        }
        if BODY_BACK_VERB_BEFORE.contains(&before) {
            continue;
        }
        if BODY_BACK_DESCRIBED.contains(&before) {
            return true;
        }
        if BODY_BACK_IS_A_RETURN.contains(&after) {
            continue;
        }
        let went_back = words[..index]
            .iter()
            .rev()
            .take(5)
            .map(|word| trimmed(word))
            .take_while(|word| !BODY_BACK_CLAUSE_JOIN.contains(word))
            .any(|word| BODY_BACK_TAKING_A_THING.contains(&word));
        if !went_back {
            return true;
        }
    }
    false
}

pub(super) fn region(reading: &Reading<'_>, is_back: bool) -> bool {
    let line = &reading.line.clean;
    person_part(reading)
        && (!is_back || back_is_a_body_part(reading))
        && (REPORTED_COMPLAINT.is_match(line)
            || !BODY_STANDING_TROUBLE.iter().any(|word| line.contains(word)))
}

fn part_named(
    words: impl IntoIterator<Item = impl AsRef<str>>,
    skip_back: bool,
) -> Option<&'static str> {
    let words = words.into_iter().collect::<Vec<_>>();
    BODY_NAMED_PARTS.iter().find_map(|&(alias, part)| {
        ((!skip_back || alias != "back") && words.iter().any(|word| word.as_ref() == alias))
            .then_some(part)
    })
}

pub(super) fn part_named_with_background(reading: &Reading<'_>) -> Option<&'static str> {
    let words = &reading.line.words;
    let skip_back = !back_is_a_body_part(reading);
    part_named(words, skip_back).or_else(|| {
        let background = crate::bot::understanding::topics::background_words(&reading.line.norm);
        part_named(
            words.iter().map(String::as_str).chain(background),
            skip_back,
        )
    })
}

fn treatment(part: &str) -> &'static Treatment {
    BODY_TREATMENTS
        .iter()
        .find(|entry| entry.part == part)
        .expect("generated body treatment")
}

pub(super) fn own_back(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    !BACK_WITH_VERB.is_match(clean) || BACK_OWNED.is_match(&BACK_WITH_VERB.replace_all(clean, "  "))
}

pub(super) fn usable_part(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    (words
        .iter()
        .any(|word| BODY_OWN_WORDS.contains(&word.as_str()))
        || !words
            .iter()
            .any(|word| BODY_OTHER_OWNERS.contains(&word.as_str())))
        && !CANNOT_USE.is_match(&reading.line.clean)
}

pub(super) fn named_usable_part(reading: &Reading<'_>) -> bool {
    let text = reading.folded().replace('’', "'");
    usable_part(reading)
        && !REPORTED_COMPLAINT.is_match(&text)
        && part_named(WORD.find_iter(&text).map(|word| word.as_str()), false).is_some()
}

pub(super) fn back_is_direction(text: &str) -> bool {
    let said = BACK.find_iter(text).count();
    said != 0
        && (BACK_AFTER_VERB.find_iter(text).count()
            + BACK_BEFORE_PLACE
                .find_iter(text)
                .filter_map(Result::ok)
                .count()
            + BACK_HOME.find_iter(text).filter_map(Result::ok).count())
            >= said
}

pub(super) fn weeks(context: &Context<'_>) -> Option<&'static str> {
    let reading = context.reading;
    let line = &reading.line;
    let last = context
        .earlier
        .last()
        .map_or("", |turn| turn.user.as_str())
        .to_lowercase();
    if [line.clean.as_str(), reading.folded(), &line.plain, &last]
        .into_iter()
        .any(|said| BODY_PREGNANT_WORDS.iter().any(|word| said.contains(word)))
    {
        return None;
    }
    let part = part_named(&line.words, !back_is_a_body_part(reading)).unwrap_or("part");
    if part == "back"
        && back_is_direction(&line.clean)
        && !context.earlier.iter().any(|turn| {
            let said = turn.user.to_lowercase();
            BACK.is_match(&said) && !back_is_direction(&said)
        })
    {
        return None;
    }
    let time = if YEAR.is_match(&line.clean) {
        2
    } else {
        usize::from(MONTH.is_match(&line.clean))
    };
    Some(treatment(part).weeks[time])
}

pub(super) fn persistent(reading: &Reading<'_>) -> &'static str {
    let part = part_named(&reading.line.words, false).unwrap_or("joint");
    treatment(part).persistent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_guards_require_a_persons_part_and_positive_activity_evidence() {
        assert!(person_part(&Reading::new("my back hurts in the car")));
        assert!(!person_part(&Reading::new("the leg of my table is broken")));
        assert!(part_stops_activity("my knee is stopping me walking"));
        assert!(!part_stops_activity("the noise is keeping me awake"));
    }

    #[test]
    fn weeks_reply_reads_the_scale_and_stands_down_for_pregnancy() {
        let reading = Reading::new("my knee has been sore for months");
        let reply = weeks(&Context::new(&reading, &[])).expect("a sore knee answer");
        assert!(reply.starts_with("A knee that has been sore for months is usually"));

        let pregnancy = Reading::new("my knee has been sore for months during pregnancy");
        assert!(weeks(&Context::new(&pregnancy, &[])).is_none());
    }
}
