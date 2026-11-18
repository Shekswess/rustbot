//! Ownership and wording for everyday relationship and household habit questions.

use std::{borrow::Cow, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use regex::{Regex, RegexSet};

use crate::{
    bot::{Reading, memory::groupages::decimal},
    text::is_space,
};

#[path = "social_guard_data.rs"]
mod data;

use data::*;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(RELATIVE, A_RELATIVE);
pattern!(ELSE_FORGETS, SOMEBODY_ELSE);
pattern!(FORGETTING, SOMETHING_FORGOTTEN);
pattern!(OWN_PHONE, MY_OWN_SCREENS);
pattern!(THEIR_PHONE, THEIR_SCREENS);
pattern!(BODY_HURT, SOMEONE_HURT_A_PART);
pattern!(SMOKE, RAISE_SMOKE);
pattern!(HOUSE, RAISE_HOUSE);
pattern!(INTERRUPTS, TALKS_OVER);
pattern!(MOVES_OUT, MOVING_OUT);
pattern!(MOVES_AWAY, MOVING_AWAY);
pattern!(EXPECTING, BABY);
pattern!(HOUSEWORK, MESS);
pattern!(HOUSEMATES, SHARED_HOUSE);
pattern!(FINANCES, MONEY);
pattern!(BILL_SHARE, SHARE_MONEY);
pattern!(PARENT, PARENT_MONEY);
pattern!(HOUSEHOLD, HOUSEHOLD_MONEY);
pattern!(NEWS, SAID_IT_IS_NEWS);
pattern!(QUIET, NO_FUSS);
pattern!(COLLEAGUE, A_COLLEAGUE);
pattern!(CLOSE, A_CLOSE_ONE);
pattern!(REQUESTS_WORDS, ASKS_FOR_THE_WORDS);
pattern!(ORDINAL, AGE_ORDINAL);
pattern!(AGE_AFTER_VERB, AGE_IN_FRONT);
pattern!(AGE_BEFORE_DATE, AGE_AHEAD);
pattern!(YEARS_OLD, AGE_YEARS);

static CAT_OUTSIDE: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(RAISE_CAT_OUTSIDE).expect("cat and garden in one sentence"));
static EXAMPLE_TOPICS: LazyLock<RegexSet> = LazyLock::new(|| {
    // The garden example is checked first because it needs a lookahead.
    RegexSet::new([
        RAISE_CAT,
        RAISE_ANIMAL,
        RAISE_CHILD,
        RAISE_SMOKE,
        RAISE_HOUSE,
        RAISE_MONEY,
    ])
    .expect("authored example topics")
});

fn said<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        reading.raw
    } else {
        &reading.line.clean
    }
}

fn collapsed(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

pub(super) fn hard_conversation(reading: &Reading<'_>) -> bool {
    let words = collapsed(&reading.line.words.join(" "));
    let text = format!(" {words} ");
    PERSON_OR_TALK.iter().any(|phrase| text.contains(phrase))
        && super::family_repair::not_their_deed(reading)
}

pub(super) fn writer_is_forgetting(reading: &Reading<'_>) -> bool {
    let text = collapsed(said(reading));
    !ELSE_FORGETS.is_match(&text)
        && !RELATIVE
            .find(&text)
            .is_some_and(|person| FORGETTING.is_match(&text[person.end()..]))
}

pub(super) fn screens_are_somebody_elses(reading: &Reading<'_>) -> bool {
    let text = collapsed(said(reading));
    if text.is_empty()
        || OWN_PHONE.is_match(&text)
        || super::family_guards::screens_are_childs(reading)
    {
        return false;
    }
    let text = format!(" {text} ");
    THEIR_PHONE.is_match(&text)
        || SOMEBODY_AT_HOME.iter().any(|person| {
            text.match_indices(person)
                .any(|(offset, found)| text[offset + found.len()..].starts_with(' '))
        })
}

pub(super) fn not_a_hurt_body(reading: &Reading<'_>) -> bool {
    !BODY_HURT.is_match(said(reading))
}

fn raise_example(text: &str, household_only: bool) -> Option<&'static str> {
    if CAT_OUTSIDE.is_match(text).unwrap_or(false) {
        return Some(RAISE_EXAMPLES[0]);
    }
    let index = EXAMPLE_TOPICS
        .matches(text)
        .into_iter()
        .find(|&index| !household_only || index < 4)?;
    Some(RAISE_EXAMPLES[index + 1])
}

pub(super) fn raise_problem(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let example = raise_example(said(reading), false).unwrap_or(RAISE_PLAIN);
    Some(Cow::Owned(format!(
        "Say what happened, how it made you feel and what you want to change, once: {example}. One topic at a time, no list of past grievances, and ask for their side."
    )))
}

pub(super) fn telling_them(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = said(reading);
    let reply = if INTERRUPTS.is_match(text) {
        TELL_TALKS_OVER
    } else if MOVES_OUT.is_match(text) {
        TELL_MOVING_OUT
    } else if MOVES_AWAY.is_match(text) {
        TELL_MOVING_AWAY
    } else if EXPECTING.is_match(text) {
        TELL_BABY
    } else if SMOKE.is_match(text) || HOUSE.is_match(text) {
        let example = raise_example(text, true).unwrap_or(RAISE_EXAMPLES[5]);
        return Some(Cow::Owned(format!(
            "{TELL_HOUSEHOLD_START}{example}{TELL_HOUSEHOLD_END}"
        )));
    } else if HOUSEWORK.is_match(text) {
        TELL_MESS
    } else if HOUSEMATES.is_match(text) && (FINANCES.is_match(text) || BILL_SHARE.is_match(text)) {
        TELL_SHARE_MONEY
    } else if PARENT.is_match(text) {
        return None;
    } else if FINANCES.is_match(text) && HOUSEHOLD.is_match(text) {
        TELL_JOINT_MONEY
    } else if NEWS.is_match(text) {
        TELL_NEWS
    } else {
        return None;
    };
    Some(Cow::Borrowed(reply))
}

fn birthday_age(text: &str) -> Option<u16> {
    let text = text.to_lowercase();
    let claim = [&*ORDINAL, &*AGE_AFTER_VERB, &*AGE_BEFORE_DATE, &*YEARS_OLD]
        .into_iter()
        .find_map(|pattern| pattern.captures(&text))?;
    decimal(&claim[1]).filter(|age| (1..=120).contains(age))
}

fn ordinal(age: u16) -> String {
    let suffix = match (age % 100, age % 10) {
        (11..=13, _) => "th",
        (_, 1) => "st",
        (_, 2) => "nd",
        (_, 3) => "rd",
        _ => "th",
    };
    format!("{age}{suffix}")
}

pub(super) fn card_age_shape(reading: &Reading<'_>) -> bool {
    birthday_age(&reading.line.words.join(" ")).is_some()
}

pub(super) fn card_has_no_age(reading: &Reading<'_>) -> bool {
    birthday_age(said(reading)).is_none()
}

pub(super) fn card_wording_shape(reading: &Reading<'_>) -> bool {
    REQUESTS_WORDS.is_match(&reading.line.words.join(" ").to_lowercase())
}

pub(super) fn birthday_card(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = said(reading);
    let age = birthday_age(text);
    let label = age.map_or_else(|| "birthday".to_owned(), ordinal);
    if QUIET.is_match(text) {
        return Some(Cow::Owned(BIRTHDAY_QUIET.replace("%s", &label)));
    }
    let example = match age {
        Some(age) if age < 13 => BIRTHDAY_CHILD,
        Some(age) if age < 18 => BIRTHDAY_TEEN,
        _ if COLLEAGUE.is_match(text) => BIRTHDAY_COLLEAGUE,
        _ if CLOSE.is_match(text) => BIRTHDAY_CLOSE,
        _ => BIRTHDAY_UNKNOWN,
    };
    Some(Cow::Owned(format!(
        "{BIRTHDAY_OPENING}{}",
        example.replace("%s", &label)
    )))
}

pub(super) fn plain_card(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = if QUIET.is_match(said(reading)) {
        PLAIN_CARD_QUIET
    } else {
        PLAIN_CARD
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_raised_problem_has_a_person_or_conversation_and_belongs_to_the_speaker() {
        for (line, expected) in [
            ("how do i talk about my feelings", true),
            ("how do i tell my sister how i feel", true),
            ("how do i stop feeling sick", false),
            ("i hurt my husbands feelings", false),
        ] {
            assert_eq!(hard_conversation(&Reading::new(line)), expected, "{line}");
        }
        assert!(!not_a_hurt_body(&Reading::new("my friend hurt his arm")));
        let reply = raise_problem(&Reading::new("my friend's cat keeps using my garden")).unwrap();
        assert!(reply.contains(RAISE_EXAMPLES[0]));
    }

    #[test]
    fn own_child_and_other_adult_keep_distinct_phone_and_habit_ownership() {
        for (line, expected) in [
            ("my husband is always on his phone", true),
            ("my girlfriend says im always on my phone", false),
            ("my grandson is always on his tablet", false),
            ("they keep picking up their phone at dinner", true),
            ("i keep using my phone at dinner", false),
        ] {
            assert_eq!(screens_are_somebody_elses(&Reading::new(line)), expected);
        }
        assert!(!writer_is_forgetting(&Reading::new(
            "my mum keeps forgetting appointments"
        )));
        assert!(writer_is_forgetting(&Reading::new(
            "i keep forgetting to ring my mum"
        )));
    }

    #[test]
    fn news_uses_the_named_subject_and_declines_generic_parent_or_apology_requests() {
        assert_eq!(
            telling_them(&Reading::new("how do i tell my parents im moving abroad")).as_deref(),
            Some(TELL_MOVING_AWAY)
        );
        assert_eq!(
            telling_them(&Reading::new(
                "how do i tell my flatmate to pay their share"
            ))
            .as_deref(),
            Some(TELL_SHARE_MONEY)
        );
        assert!(telling_them(&Reading::new("how do i say sorry to my wife")).is_none());
        assert!(telling_them(&Reading::new("how do i bring up money with my parents")).is_none());
    }

    #[test]
    fn card_wording_uses_the_named_age_and_respects_a_quiet_occasion() {
        for (line, number, example) in [
            (
                "what do i write in a card for a 6 year old",
                "6th",
                "finally beat me at cards",
            ),
            (
                "what shall i write in a birthday card for my daughter turning 13",
                "13th",
                "stuck in a car with",
            ),
            (
                "my nan is 90 next month what do i write in her card",
                "90th",
                "drove us home at 3am",
            ),
            ("my mate turns 22 and hates a fuss", "22nd", "no fuss"),
        ] {
            let reading = Reading::new(line);
            let reply = birthday_card(&reading).unwrap();
            assert!(
                reply.contains(number) && reply.contains(example),
                "{line}: {reply}"
            );
            assert!(card_age_shape(&reading));
            assert!(!card_has_no_age(&reading));
        }
        let reading = Reading::new("what shall i write in a card for my mate who hates a fuss");
        assert!(card_has_no_age(&reading) && card_wording_shape(&reading));
        assert_eq!(plain_card(&reading).as_deref(), Some(PLAIN_CARD_QUIET));
        assert_eq!(birthday_age("turning ٢٢"), Some(22));
        assert_eq!(birthday_age("0th birthday, turning 21"), None);
    }

    #[test]
    fn selected_cards_rank_the_occasion_and_allow_a_contained_key() {
        let birthday = "what should i write in a birthday card";
        let plain = "what do i write in a card for my neighbour";
        for (text, expected) in [
            (birthday, birthday_card(&Reading::new(birthday)).unwrap()),
            (plain, Cow::Borrowed(PLAIN_CARD)),
        ] {
            let turn = crate::Bot::new(Some("0")).unwrap().turn(text);
            assert_eq!(turn.module, Some("answers"), "{text}");
            assert_eq!(turn.reply, expected, "{text}");
        }
        let turn = crate::Bot::new(Some("0"))
            .unwrap()
            .turn("i have no motivation at my job");
        assert_eq!(turn.module, Some("answers"));
        assert!(
            turn.reply
                .starts_with("Shrink the job until it is almost silly")
        );
    }
}
