//! Gates for the family cards, kept separate from their authored keys and replies.

use crate::bot::{
    Reading,
    modules::safety::breathing_or_colour_emergency,
    understanding::kinds::{StateOwner, broken_heart},
};
use fancy_regex::Regex as Lookaround;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "family_guard_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    ADULT_AGE,
    A_PICKUP_NOT_A_WAKING,
    SMALL_CHILD_AGE,
    A_NAME_IS_ASKED,
    GIVING_UP_ON,
    ASKS_THE_AGE,
    A_FIGURE_IN_THE_LINE,
    A_PERSON_OR_CHILD_PHRASE,
    A_WEAR_STRETCH,
    THE_JOB,
    A_GROWN_AGE,
    OTHER_PUMP,
    A_FOOD_LIMIT,
    A_VISIT_AS_A_GRIEVANCE,
    AN_HOUR,
    A_LENGTH_OF_STAY,
    A_VISIT_I_AM_MAKING,
    I_ASK_TO_STOP_THE_ROW,
    A_GROWN_UP_IN_THE_ROW,
    A_CHILD_IN_THE_ROW,
    I_AM_IN_THE_ROW,
    A_TRIP_STILL_BEING_WEIGHED,
    ASKING_FOR_THE_WORDS,
    THE_LENGTH_ASK,
    TELLING_A_CHILD,
    GETTING_OVER,
    AN_ENDING_OR_A_PERSON,
    A_PERSON_ALREADY_THERE,
    WORDS_FOR_SOMEONE,
    LEFT_THE_COUNTRY,
    MONEY_IN_THE_LINE,
    A_CHILD_REFUSING_A_GARMENT,
);

macro_rules! lookarounds {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Lookaround> =
            LazyLock::new(|| Lookaround::new(data::$name).expect(stringify!($name)));)+
    };
}

lookarounds!(TAKING_SOMEBODY, TEEN_AGE, OTHER_ADULT_AGE);
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static BATH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bbath").unwrap());
static FORGETS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:he|she|they|hubby|husband|wife|partner|boyfriend|girlfriend|mum|mom|dad|mother|father|son|daughter|brother|sister|mate|friend|boss|workmate)",
        r"\s+(?:keeps?|always|never|does not|doesn't|did not|didn't|never ever)\s+(?:remember|forget|forgot|forgotten)"
    )).unwrap()
});
static GETTING_OVER_FILLER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^[\s,.!?]*(?:(?:a|an|the|my|your|his|her|their|our|this|that|these|those|being|having|all|it|about|with|from|over|of)\b[\s,.]*)*")
        .unwrap()
});

fn clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        reading.line.text.as_str()
    } else {
        reading.line.clean.as_str()
    }
}

fn typed<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.text.is_empty() {
        reading.line.clean.as_str()
    } else {
        reading.line.text.as_str()
    }
}

fn contains(text: &str, pool: &[&str]) -> bool {
    pool.iter().any(|item| text.contains(item))
}

fn words_include(reading: &Reading<'_>, pool: &[&str]) -> bool {
    reading
        .line
        .words
        .iter()
        .any(|word| pool.contains(&word.as_str()))
}

fn collapsed(text: &str) -> String {
    text.split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn teenager(text: &str) -> bool {
    contains(text, data::TEEN_WORDS) || TEEN_AGE.is_match(text).unwrap_or(false)
}

pub(super) fn teenager_named(reading: &Reading<'_>) -> bool {
    teenager(&reading.line.words.join(" ").to_lowercase())
}

pub(super) fn toddler_in_morning(reading: &Reading<'_>) -> bool {
    let words = reading.line.words.join(" ").to_lowercase();
    !teenager(&words)
        && !contains(&words, data::ADULT_WORDS)
        && !ADULT_AGE.is_match(&words)
        && !A_PICKUP_NOT_A_WAKING.is_match(clean(reading))
}

pub(super) fn small_child_age(reading: &Reading<'_>) -> bool {
    SMALL_CHILD_AGE.is_match(&reading.line.words.join(" ").to_lowercase())
}

pub(super) fn child_is_not_grown(reading: &Reading<'_>) -> bool {
    let words = reading.line.words.join(" ").to_lowercase();
    !contains(&words, data::GROWN_UP_SAID) && !OTHER_ADULT_AGE.is_match(&words).unwrap_or(false)
}

pub(super) fn child_cough(reading: &Reading<'_>) -> bool {
    !A_GROWN_AGE
        .captures_iter(&reading.line.clean)
        .any(|capture| {
            let name = &capture[1];
            name.parse::<u16>().map_or_else(
                |_| {
                    data::YEAR_WORDS
                        .iter()
                        .any(|&(word, number)| word == name && number >= 19)
                },
                |number| number >= 19,
            )
        })
}

// The original entry key and guard are retained. Triage follows NHS croup guidance.
static CROUP_SIGNS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:croup|bark(?:s|y|ing)? (?:cough|like a seal)|(?:cough|coughing) (?:sounds? )?like a (?:bark|seal)|stridor|",
        r"(?:high[- ]pitched|rasping|harsh) (?:noise|sound) (?:when |while |on )?(?:breathing in|inhaling))\b"
    )).expect("croup signs")
});
pub(super) const CHILD_COUGH_EMERGENCY: &str = "If your child is struggling to breathe or has the signs you've described now, call 999 immediately in the UK. If you're elsewhere, call your local emergency number.";

pub(super) fn child_cough_reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = collapsed(&typed(reading).to_lowercase().replace('’', "'"));
    let reply = if breathing_or_colour_emergency(&said) {
        CHILD_COUGH_EMERGENCY
    } else if CROUP_SIGNS.is_match(&said) {
        "A barking cough or a harsh noise when breathing in can be croup. Call NHS 111 or ask for an urgent GP appointment. Keep your child upright and calm while you get help. Do not use steam. If they struggle to breathe or their lips, tongue or skin turn blue, grey or unusually pale, call 999 immediately."
    } else {
        "If your child is breathing, eating and drinking normally and isn't wheezing, offer fluids and keep an eye on them. Contact a GP if the cough lasts more than three weeks, or NHS 111 if the symptoms are worsening and you are worried. If they struggle to breathe or their lips, tongue or skin turn blue, grey or unusually pale, call 999 immediately."
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn not_word_for_a_name(reading: &Reading<'_>) -> bool {
    !A_NAME_IS_ASKED.is_match(&reading.line.words.join(" ").to_lowercase())
}

pub(super) fn giving_up_on_person(reading: &Reading<'_>) -> bool {
    let text = typed(reading).to_lowercase();
    !GIVING_UP_ON.is_match(&text)
        || contains(
            &text,
            &["marriage", "divorce", "husband", "wife", "partner"],
        )
        || GIVING_UP_ON.captures_iter(&text).any(|capture| {
            capture[1]
                .split_whitespace()
                .any(|word| data::GIVING_UP_A_PERSON.contains(&word))
        })
}

pub(super) fn baby_in_water(reading: &Reading<'_>) -> bool {
    if BATH.is_match(&typed(reading).to_lowercase()) {
        return false;
    }
    if reading.line.words.is_empty() {
        return reading
            .line
            .clean
            .split_whitespace()
            .any(|word| data::BABY_IN_THE_WATER.contains(&word));
    }
    words_include(reading, data::BABY_IN_THE_WATER)
}

pub(super) fn more_than_age(reading: &Reading<'_>) -> bool {
    let text = collapsed(&reading.line.clean).to_lowercase();
    text.is_empty()
        || text.contains('?')
        || ASKS_THE_AGE.is_match(&text)
        || !A_FIGURE_IN_THE_LINE.is_match(&text)
        || text.split_whitespace().any(|word| {
            !data::THE_FRAME_OF_AN_AGE.contains(&word)
                && !A_FIGURE_IN_THE_LINE
                    .find(word)
                    .is_some_and(|m| m.range() == (0..word.len()))
        })
}

pub(super) fn child_dark(reading: &Reading<'_>) -> bool {
    let words: Vec<_> = reading
        .line
        .words
        .iter()
        .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()))
        .filter(|word| !word.is_empty())
        .collect();
    if words
        .iter()
        .any(|word| data::A_PERSON_OR_CHILD_WORD.contains(word))
    {
        return true;
    }
    let text = if words.is_empty() {
        collapsed(typed(reading)).to_lowercase()
    } else {
        words.join(" ")
    };
    A_PERSON_OR_CHILD_PHRASE.is_match(&text)
        || !(A_WEAR_STRETCH.is_match(&text) || THE_JOB.is_match(&text))
}

pub(super) fn milk_pump(reading: &Reading<'_>) -> bool {
    if OTHER_PUMP.is_match(&collapsed(&reading.line.clean).to_lowercase()) {
        return false;
    }
    words_include(reading, data::MILK_SAID)
        || reading
            .line
            .norm
            .iter()
            .any(|word| data::MILK_SAID.contains(&word.as_str()))
        || [&reading.line.clean, &reading.line.text]
            .iter()
            .any(|text| {
                WORD.find_iter(&text.to_lowercase())
                    .any(|item| data::MILK_SAID.contains(&item.as_str()))
            })
}

pub(super) fn caffeine_drink(reading: &Reading<'_>) -> bool {
    [
        &reading.line.clean,
        &reading.line.text,
        &reading.line.norm_text,
    ]
    .iter()
    .any(|text| {
        WORD.find_iter(&text.to_lowercase())
            .any(|item| data::CAFFEINE_SAID.contains(&item.as_str()))
    })
}

pub(super) fn debt_is_money(reading: &Reading<'_>) -> bool {
    MONEY_IN_THE_LINE.is_match(typed(reading))
}

pub(super) fn not_debt_or_trip(reading: &Reading<'_>) -> bool {
    let said = if reading.line.norm_text.is_empty() {
        reading.line.text.as_str()
    } else {
        reading.line.norm_text.as_str()
    };
    let said = said.to_lowercase();
    !debt_is_money(reading)
        && (contains(&said, data::HANDED_BACK) || !TAKING_SOMEBODY.is_match(&said).unwrap_or(false))
}

pub(super) fn parents_visiting(reading: &Reading<'_>) -> bool {
    let said = clean(reading);
    !(A_FOOD_LIMIT.is_match(said)
        || A_VISIT_AS_A_GRIEVANCE.is_match(said)
        || AN_HOUR.is_match(said) && !A_LENGTH_OF_STAY.is_match(said)
        || A_VISIT_I_AM_MAKING.is_match(said)
        || A_VISIT_I_AM_MAKING.is_match(&reading.line.text)
        || ASKING_FOR_THE_WORDS.is_match(said))
}

pub(super) fn visit_i_am_making(reading: &Reading<'_>) -> bool {
    let said = clean(reading);
    (A_VISIT_I_AM_MAKING.is_match(said) || A_VISIT_I_AM_MAKING.is_match(&reading.line.text))
        && !A_TRIP_STILL_BEING_WEIGHED.is_match(said)
}

pub(super) fn not_a_fight_about_clothes(reading: &Reading<'_>) -> bool {
    let said = format!(
        "{} {} {}",
        reading.line.clean, reading.line.norm_text, reading.line.text
    )
    .to_lowercase();
    !A_CHILD_REFUSING_A_GARMENT.is_match(&said)
}

pub(super) fn sibling_fighting(reading: &Reading<'_>) -> bool {
    let said = clean(reading);
    let inside = if I_ASK_TO_STOP_THE_ROW.is_match(said) {
        A_GROWN_UP_IN_THE_ROW.is_match(said) && !A_CHILD_IN_THE_ROW.is_match(said)
    } else {
        I_AM_IN_THE_ROW.is_match(said)
    };
    !inside && not_a_fight_about_clothes(reading)
}

pub(super) fn screens_are_childs(reading: &Reading<'_>) -> bool {
    let said = collapsed(clean(reading)).to_lowercase();
    if said.is_empty() {
        return true;
    }
    let padded = format!(" {said} ");
    contains(&padded, data::ONLY_SAID_ABOUT_A_CHILD)
        || data::A_CHILD_NAMED.iter().any(|phrase| {
            if phrase.contains(' ') {
                padded.contains(phrase)
            } else {
                WORD.find_iter(&padded).any(|word| word.as_str() == *phrase)
            }
        })
}

pub(super) fn forgets_themself(reading: &Reading<'_>) -> bool {
    !FORGETS.is_match(&collapsed(clean(reading)).to_lowercase())
}

pub(super) fn friend_forgot_birthday(reading: &Reading<'_>) -> bool {
    let words = &reading.line.norm;
    !words.is_empty()
        && !(words.iter().any(|word| word == "birthday")
            && !words.iter().any(|word| {
                matches!(
                    word.as_str(),
                    "forgot" | "forgotten" | "missed" | "was" | "were" | "yesterday" | "last"
                )
            })
            && words.windows(2).any(|pair| {
                matches!(pair[0].as_str(), "my" | "our")
                    && matches!(pair[1].as_str(), "mate" | "mates" | "friend" | "friends")
            }))
}

pub(super) fn someone_elses_drinking(reading: &Reading<'_>) -> bool {
    reading.line.words.iter().any(|word| {
        data::SOMEONE_ELSE.contains(&word.trim_matches(|ch: char| ch.is_ascii_punctuation()))
    })
}

pub(super) fn reading_length(reading: &Reading<'_>) -> bool {
    THE_LENGTH_ASK.is_match(clean(reading))
}

pub(super) fn telling_a_child(reading: &Reading<'_>) -> bool {
    let said = if reading.line.words.is_empty() {
        reading.line.text.to_lowercase()
    } else {
        reading.line.words.join(" ").to_lowercase()
    };
    let said = format!(" {said} ");
    !contains(&said, &[" tell if ", " tell whether ", " tell when "])
        && TELLING_A_CHILD.is_match(&said)
        && data::A_CHILD_MONEY_TROUBLE
            .iter()
            .any(|word| said.contains(&format!(" {word}")))
}

pub(super) fn heart_is_own(reading: &Reading<'_>) -> bool {
    reading.features.state_owner != Some(StateOwner::Other)
}

pub(super) fn broken_heart_shape(reading: &Reading<'_>) -> bool {
    broken_heart(&reading.line.words.join(" "))
}

pub(super) fn a_person_is_what_they_get_over(reading: &Reading<'_>) -> bool {
    let said = clean(reading);
    let Some(phrase) = GETTING_OVER.find(said) else {
        return true;
    };
    let names_ending =
        |text: &str| AN_ENDING_OR_A_PERSON.is_match(text) || A_PERSON_ALREADY_THERE.is_match(text);
    if names_ending(said) {
        return true;
    }
    let rest = GETTING_OVER_FILLER.replace(&said[phrase.end()..], "");
    let rest = rest.trim_matches([' ', '?', '!', '.', ',']).to_lowercase();
    let head = WORD.find(&rest).map_or("", |word| word.as_str());
    matches!(
        head,
        "" | "it"
            | "this"
            | "that"
            | "them"
            | "him"
            | "her"
            | "what"
            | "everything"
            | "anything"
            | "all"
            | "which"
            | "whom"
            | "people"
            | "someone"
            | "somebody"
            | "badly"
            | "now"
            | "situation"
            | "past"
            | "behaved"
            | "guilt"
            | "guilty"
            | "shame"
            | "regret"
            | "myself"
            | "how"
            | "why"
            | "whole"
            | "the"
    ) || [
        &reading.reference.it,
        &reading.reference.that,
        &reading.reference.she,
        &reading.reference.he,
        &reading.reference.they,
    ]
    .into_iter()
    .flatten()
    .any(|who| names_ending(who))
}

pub(super) fn not_a_hard_conversation(reading: &Reading<'_>) -> bool {
    let words;
    let mut said = clean(reading);
    if said.is_empty() {
        words = reading.line.words.join(" ");
        said = &words;
    }
    !WORDS_FOR_SOMEONE.is_match(said)
        || !A_PERSON_ALREADY_THERE.is_match(said)
            && ![
                &reading.reference.she,
                &reading.reference.he,
                &reading.reference.they,
                &reading.reference.it,
            ]
            .into_iter()
            .flatten()
            .any(|who| A_PERSON_ALREADY_THERE.is_match(who))
}

pub(super) fn another_country_said(reading: &Reading<'_>) -> bool {
    let said = format!(" {} {} ", reading.line.text, reading.line.clean).to_lowercase();
    LEFT_THE_COUNTRY.is_match(&said)
        || crate::bot::knowledge::geography::source_country_named(&said)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn separates_toddler_waking_from_older_children_and_pickups() {
        assert!(toddler_in_morning(&Reading::new(
            "my three year old wakes up at 5"
        )));
        assert!(!toddler_in_morning(&Reading::new(
            "my daughter is 13 and won't wake up"
        )));
        assert!(!toddler_in_morning(&Reading::new(
            "collect my toddler up at half five"
        )));
        assert!(small_child_age(&Reading::new(
            "my child is 4 and refuses to sleep"
        )));
    }

    #[test]
    fn preserves_child_cough_guard_and_sends_emergency_signs_to_immediate_help() {
        assert!(child_cough(&Reading::new("my four year old has a cough")));
        assert!(!child_cough(&Reading::new(
            "my son is nineteen years old and coughing"
        )));
        for line in [
            "my toddler's lips are blue and she has a cough",
            "my little boy is struggling to breathe and coughing",
            "my baby has a barky cough and severe breathing difficulties",
            "my baby is hard to wake and has a cough",
            "my toddler is coughing and is too breathless to drink",
            "my baby is coughing and turning blue",
        ] {
            let reply = child_cough_reply(&Reading::new(line)).unwrap();
            assert!(
                reply.contains("call 999 immediately") && !reply.contains("111"),
                "{line}"
            );
        }
    }

    #[test]
    fn recognizes_possible_croup_and_preserves_ordinary_cough_advice() {
        let croup = child_cough_reply(&Reading::new(
            "my toddler has a barking cough but is not struggling to breathe",
        ))
        .unwrap();
        assert!(croup.contains("Call NHS 111"));
        assert!(croup.contains("Do not use steam"));
        let ordinary = child_cough_reply(&Reading::new(
            "my child has a cough but no blue lips and seems fine",
        ))
        .unwrap();
        assert!(ordinary.starts_with("If your child is breathing, eating and drinking normally"));
    }

    #[test]
    fn child_cough_proposals_and_selected_replies_keep_the_triage_branch() {
        for (line, expect) in [
            (
                "my toddler has a cough and her lips are blue",
                "call 999 immediately",
            ),
            (
                "my child has a cough and is struggling to breathe",
                "call 999 immediately",
            ),
            (
                "my toddler has a barking cough",
                "can be croup. Call NHS 111",
            ),
            (
                "my child is coughing and has stridor",
                "can be croup. Call NHS 111",
            ),
            (
                "my baby is 9 months old and has a cough",
                "isn't wheezing, offer fluids",
            ),
        ] {
            let turn = Bot::new(Some("0")).unwrap().turn(line);
            assert_eq!(turn.module, Some("answers"), "{line}");
            assert!(turn.reply.contains(expect), "{line}: {}", turn.reply);
            let answer = turn
                .candidates
                .iter()
                .find(|proposed| proposed.module == "answers")
                .unwrap();
            assert_eq!(answer.reply, turn.reply, "{line}");
            assert!(!turn.reply.contains("steamy"), "{line}");
        }
        let unrelated = Bot::new(Some("0"))
            .unwrap()
            .turn("i have a dry cough at night");
        assert!(!unrelated.candidates.iter().any(|proposal| {
            proposal.reply.contains("can be croup. Call NHS 111")
                || proposal
                    .reply
                    .contains("If your child is breathing, eating and drinking")
        }));
    }
}
