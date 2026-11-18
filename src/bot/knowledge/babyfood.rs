//! Feeding ownership, age bands and verdicts from `bot/knowledge/babyfood.py`.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use super::{babyfood_data as data, family_cards::infant_milk_advice, guards::Context};
use crate::bot::Reading;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(AGE_FIGURE, data::AGE_FIGURE);
pattern!(FOOD_INTRO, data::FOOD_INTRO);
pattern!(PEOPLE_INTRO, data::PEOPLE_INTRO);
pattern!(WHILE_FEEDING, data::WHILE_FEEDING);
pattern!(MOTHER_EATING, data::MOTHER_EATING);
pattern!(SHOW_SEASON, data::SHOW_SEASON);
pattern!(ORGAN, data::ORGAN_CONDITION);
static ARGUMENT: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(&format!("(?i){}", data::ARGUMENT)).expect("valid argument pattern")
});

#[derive(Clone, Copy)]
pub(super) enum Reply {
    Fixed,
    Stage,
    Water,
    Milk,
    Drink,
    Snack,
    PlantMilk,
    Allergens,
    Verdict { water_ask: bool },
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Guard {
    FeedsSomeone,
    BabyOrPronoun,
    Weaning,
    NamedOrPronoun,
    NamedOrPronounWithoutArgument,
    ChildNamed,
}

pub(super) struct Line<'a> {
    reading: &'a Reading<'a>,
    text: String,
    padded: String,
}

impl<'a> Line<'a> {
    pub(super) fn new(reading: &'a Reading<'a>) -> Self {
        let text = if reading.line.words.is_empty() {
            reading.line.clean.clone()
        } else {
            reading.line.words.join(" ")
        };
        let padded = format!(" {text} ");
        Self {
            reading,
            text,
            padded,
        }
    }

    fn baby_named(&self) -> bool {
        AGE_FIGURE.is_match(&self.padded) || has_phrase(&self.padded, data::BABY_NAMES)
    }

    fn age(&self) -> Option<u32> {
        self.reading
            .numbers
            .iter()
            .find_map(|number| {
                if number.value < 0.0
                    || number.value.fract() != 0.0
                    || number.value > f64::from(u32::MAX)
                {
                    return None;
                }
                let value = number.value as u32;
                let unit = number.unit?;
                if unit.starts_with("week") {
                    Some(0)
                } else if unit.starts_with("month") {
                    Some(value)
                } else if unit.starts_with("year") {
                    value.checked_mul(12)
                } else {
                    None
                }
            })
            .or_else(|| age_in_text(&self.text))
    }

    fn pronoun(&self) -> bool {
        has_phrase(&self.padded, data::CHILD_PRONOUN)
            && (self.table_word() || !has_phrase(&self.padded, data::ADULT_NAMES))
    }

    fn table_word(&self) -> bool {
        has_phrase(&self.padded, data::TABLE_WORD) || has_phrase(&self.padded, data::TABLE_FOOD)
    }

    fn speaker_is_eating(&self) -> bool {
        if data::FOR_ME.iter().any(|phrase| self.text.contains(phrase)) {
            return true;
        }
        let words = &self.reading.line.words;
        words.iter().enumerate().any(|(index, word)| {
            let rest = &words[index + 1..];
            data::SELF_PRONOUN.contains(&word.as_str())
                && !rest.first().is_some_and(|child| {
                    data::BABY_WORDS.contains(&child.as_str())
                        || data::CHILD_NAMES.contains(&child.as_str())
                })
                && rest
                    .iter()
                    .take(3)
                    .any(|word| data::OWN_EATING.contains(&word.as_str()))
        })
    }
}

fn has_phrase(padded: &str, words: &[&str]) -> bool {
    words.iter().any(|word| {
        padded.match_indices(word).any(|(start, found)| {
            start > 0
                && padded.as_bytes()[start - 1] == b' '
                && padded.as_bytes().get(start + found.len()) == Some(&b' ')
        })
    })
}

fn age_in_text(text: &str) -> Option<u32> {
    let words = text
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>();
    if words.windows(3).any(|words| {
        matches!(
            words,
            ["baby", "is", "one" | "1"] | ["one" | "1", "year", "old"]
        )
    }) {
        return Some(12);
    }
    for (index, word) in data::NUMBER_WORDS.iter().enumerate() {
        let amount = (index + 1) as u32;
        if words
            .windows(2)
            .any(|pair| pair[0] == *word && matches!(pair[1], "month" | "months"))
        {
            return Some(amount);
        }
        if words.windows(3).any(|triple| {
            triple[0] == *word && matches!(triple[1], "year" | "years") && triple[2] == "old"
        }) {
            return Some(12 * amount);
        }
    }
    for (index, word) in words.iter().enumerate() {
        if !data::OLDER_CHILD_WORDS.contains(word) {
            continue;
        }
        let rest = &words[index + 1..];
        let rest = if rest
            .first()
            .is_some_and(|word| matches!(*word, "is" | "are" | "aged" | "at" | "of"))
        {
            &rest[1..]
        } else {
            rest
        };
        if let [age, tail @ ..] = rest
            && let Ok(age @ 2..=19) = age.parse::<u32>()
            && !tail.first().is_some_and(|word| {
                matches!(
                    *word,
                    "month"
                        | "months"
                        | "mth"
                        | "mths"
                        | "week"
                        | "weeks"
                        | "wk"
                        | "wks"
                        | "day"
                        | "days"
                        | "stone"
                        | "kilos"
                        | "kilo"
                        | "kg"
                        | "lbs"
                        | "pounds"
                )
            })
        {
            return Some(12 * age);
        }
    }
    if words
        .iter()
        .any(|word| matches!(*word, "toddler" | "toddlers"))
    {
        return Some(12);
    }
    words
        .iter()
        .any(|word| matches!(*word, "newborn" | "newborns"))
        .then_some(0)
}

fn previous_age(context: &Context<'_>, line: &Line<'_>) -> Option<u32> {
    if !has_phrase(&line.padded, data::CHAT_CHILD_PRONOUN) {
        return None;
    }
    context.earlier.iter().rev().find_map(|turn| {
        let text = turn.user.trim().to_lowercase();
        if text.is_empty() || text == context.reading.line.clean.trim() {
            return None;
        }
        AGE_FIGURE
            .captures_iter(&text)
            .find_map(|age| {
                let value = age[1].parse::<u32>().ok()?;
                match &age[2] {
                    "month" | "months" => Some(value),
                    "year" | "years" => Some(12 * value),
                    _ => None,
                }
            })
            .or_else(|| age_in_text(&text))
    })
}

fn references<'line>(reading: &'line Reading<'_>) -> impl Iterator<Item = &'line str> {
    let reference = &reading.reference;
    [
        reference.he.as_deref(),
        reference.she.as_deref(),
        reference.they.as_deref(),
    ]
    .into_iter()
    .flatten()
}

fn has_word_from(text: &str, names: &[&str]) -> bool {
    text.split(|c: char| !(c.is_ascii_alphabetic() || c == '\''))
        .any(|word| names.contains(&word))
}

fn baby_named_before(context: &Context<'_>) -> bool {
    context.earlier.last().is_some_and(|previous| {
        let earlier = format!(" {} ", previous.user.trim().to_lowercase());
        AGE_FIGURE.is_match(&earlier) || has_phrase(&earlier, data::BABY_NAMES)
    })
}

fn pronoun_is_animal(context: &Context<'_>) -> bool {
    let reading = context.reading;
    references(reading).any(|value| has_word_from(&value.to_lowercase(), data::ANIMALS))
        || reading.animal_in_hand.is_some()
        || context.earlier.last().is_some_and(|turn| {
            let text = turn.user.trim().to_lowercase();
            has_word_from(&text, data::ANIMALS)
                && !has_phrase(&format!(" {text} "), data::BABY_NAMES)
        })
}

pub(super) fn introduces_a_food(line: &Line<'_>) -> bool {
    FOOD_INTRO.is_match(&line.padded)
        && !PEOPLE_INTRO.is_match(&line.padded)
        && !data::NOT_MY_DIET
            .iter()
            .any(|word| line.padded.contains(word))
        && data::ALLERGEN_FOOD.iter().any(|word| {
            !matches!(*word, "bottle" | "formula") && has_phrase(&line.padded, &[*word])
        })
}

fn pronoun_is_adult(reading: &Reading<'_>) -> bool {
    references(reading).any(|reference| {
        let reference = reference.to_lowercase();
        !has_word_from(&reference, data::BABY_SHORT_NAMES)
            && !has_word_from(&reference, data::CHILD_PRONOUN)
            && !has_word_from(&reference, &["one", "baby", "child"])
            && has_word_from(&reference, data::GROWN_UP_NAMES)
    })
}

pub(super) fn feeds_someone(line: &Line<'_>, context: &Context<'_>, use_turns: bool) -> bool {
    let reading = context.reading;
    let pets = reading.topic.is_some_and(|(topic, _)| topic == "pets")
        || reading.own_topic.is_some_and(|(topic, _)| topic == "pets");
    if pets && !line.baby_named() && line.age().is_none() {
        return false;
    }
    if data::FOR_A_GROWN_UP
        .iter()
        .any(|word| line.padded.contains(word))
    {
        return false;
    }
    let mut stripped = line.padded.clone();
    for food in data::BABY_NAME_AS_FOOD {
        stripped = stripped.replace(&format!(" {food} "), " thing ");
    }
    let names_baby = has_phrase(&stripped, data::BABY_NAMES) || AGE_FIGURE.is_match(&stripped);
    let words = &reading.line.words;
    let feeding = data::FEEDING_PHRASES
        .iter()
        .any(|phrase| line.padded.contains(phrase))
        || (words.len() >= 3
            && words
                .iter()
                .any(|word| data::WATER_ASKS.contains(&word.as_str()))
            && data::GIVING_WATER
                .iter()
                .any(|phrase| line.padded.contains(phrase))
            && !has_phrase(&line.padded, data::WATER_NOT_BABY))
        || (FOOD_INTRO.is_match(&line.padded)
            && !data::NOT_MY_DIET
                .iter()
                .any(|word| line.padded.contains(word))
            && data::ALLERGEN_FOOD.iter().any(|word| {
                !matches!(*word, "bottle" | "formula") && has_phrase(&line.padded, &[*word])
            }));
    if !names_baby
        && ((!feeding && line.speaker_is_eating())
            || (WHILE_FEEDING.is_match(&line.text) && MOTHER_EATING.is_match(&line.text)))
    {
        return false;
    }
    if line.baby_named() || line.age().is_some() || feeding {
        return true;
    }
    if words.iter().any(|word| {
        matches!(
            word.as_str(),
            "he" | "she" | "him" | "her" | "his" | "hers" | "they"
        )
    }) {
        return !pronoun_is_animal(context);
    }
    if line.speaker_is_eating() {
        return false;
    }
    has_phrase(&line.padded, data::BABY_ACT)
        || (use_turns
            && context.earlier.iter().rev().any(|turn| {
                let text = turn.user.trim().to_lowercase();
                if text.is_empty() || text == line.text {
                    return false;
                }
                let before = format!(" {text} ");
                AGE_FIGURE.is_match(&before) || has_phrase(&before, data::BABY_NAMES)
            }))
}

impl Guard {
    pub(super) fn passes(self, context: &Context<'_>) -> bool {
        let line = context.baby_line();
        match self {
            Self::FeedsSomeone => {
                context.baby_recipient(false)
                    || (context.reading.reference.followup && baby_named_before(context))
            }
            Self::BabyOrPronoun | Self::Weaning => {
                let baby = line.baby_named()
                    || (has_phrase(&line.padded, data::CHILD_PRONOUN)
                        && !has_phrase(&line.padded, data::ADULT_NAMES));
                baby && (!matches!(self, Self::Weaning) || line.age().is_none_or(|age| age < 12))
            }
            Self::NamedOrPronoun | Self::NamedOrPronounWithoutArgument => {
                (line.baby_named() || line.pronoun())
                    && (!matches!(self, Self::NamedOrPronounWithoutArgument)
                        || ARGUMENT
                            .is_match(context.reading.raw)
                            .is_ok_and(|matched| !matched))
            }
            Self::ChildNamed => {
                has_phrase(&line.padded, data::BABY_WORDS)
                    || has_phrase(&line.padded, data::CHILD_NAMES)
                    || AGE_FIGURE.is_match(&line.padded)
                    || (!SHOW_SEASON.is_match(&line.padded)
                        && has_phrase(&line.padded, data::CHILD_PRONOUN))
            }
        }
    }
}

fn asks_about_an_organ(line: &Line<'_>, names: &[&str]) -> bool {
    let Some(condition) = ORGAN.captures(&line.text) else {
        return false;
    };
    let organ = condition[1].trim_end_matches('s');
    if !names.iter().any(|name| name.contains(organ)) {
        return false;
    }
    let shape = condition.get(0).expect("matched organ condition");
    let rest = format!(
        "{} {}",
        &line.text[..shape.start()],
        &line.text[shape.end()..]
    );
    !ORGAN.is_match(&rest)
        && !names.iter().any(|name| {
            rest.match_indices(name).any(|(at, matched)| {
                let word = |c: char| c.is_alphanumeric() || c == '_';
                !rest[..at].chars().next_back().is_some_and(word)
                    && !rest[at + matched.len()..].chars().next().is_some_and(word)
            })
        })
}

fn water_ask(line: &Line<'_>) -> bool {
    line.padded.contains(" water ")
        && ![
            "i drink",
            "should i drink",
            " a day ",
            "litre",
            "liter",
            "pint",
            "eight glasses",
            "2 litres",
            "2 liters",
        ]
        .iter()
        .any(|word| line.padded.contains(word))
        && [
            "give",
            "giving",
            "offer",
            "when can",
            "how much",
            "how often",
            "sips",
        ]
        .iter()
        .any(|word| line.padded.contains(word))
}

fn wants_verdict(
    context: &Context<'_>,
    line: &Line<'_>,
    names: &[&str],
    own_water_ask: bool,
) -> bool {
    if asks_about_an_organ(line, names) {
        return false;
    }
    if (own_water_ask && water_ask(line)) || line.baby_named() {
        return true;
    }
    let age = previous_age(context, line);
    if line.pronoun() {
        return age.map_or_else(
            || !pronoun_is_adult(line.reading) || line.table_word(),
            |age| age <= 36,
        );
    }
    age.is_some_and(|age| age <= 36)
        || (line.reading.reference.followup
            && context.earlier.last().is_some_and(|last| {
                let before = format!(" {} ", last.user.trim().to_lowercase());
                has_phrase(&before, data::BABY_SHORT_NAMES)
                    || has_phrase(&before, data::CHILD_PRONOUN)
            }))
}

fn age_bands(
    age: Option<u32>,
    bands: [&'static str; 3],
    fallback_offset: usize,
) -> Cow<'static, str> {
    match age {
        Some(..=5) => Cow::Borrowed(bands[0]),
        Some(6..=11) => Cow::Borrowed(bands[1]),
        Some(_) => Cow::Borrowed(bands[2]),
        None => Cow::Owned(bands[fallback_offset..].join(" ")),
    }
}

impl Reply {
    pub(super) fn answer(
        self,
        context: &Context<'_>,
        reply: &'static str,
        subject: &[&str],
    ) -> Option<Cow<'static, str>> {
        if !context.baby_recipient(true) {
            return None;
        }
        let line = context.baby_line();
        let age = || line.age().or_else(|| previous_age(context, line));
        Some(match self {
            Self::Fixed => Cow::Borrowed(reply),
            Self::Stage => {
                if !has_phrase(&line.padded, data::EATING) {
                    return None;
                }
                let stage = line.age().and_then(|age| {
                    data::STAGES.iter().find(|(from, until, _)| {
                        age >= *from && until.is_none_or(|until| age <= until)
                    })
                });
                Cow::Borrowed(stage.map_or(data::FIRST_FOODS, |(_, _, answer)| *answer))
            }
            Self::Water => {
                let reply = match age() {
                    Some(..=5) => data::WATER_UNDER,
                    Some(12..) => data::WATER_OVER,
                    _ => data::WATER_NO_AGE,
                };
                Cow::Owned(format!("{reply}{}", data::WATER_JUICE))
            }
            Self::Milk => match age() {
                Some(..=5) => Cow::Borrowed(data::MILK_UNDER),
                months @ Some(6..=11) => Cow::Borrowed(infant_milk_advice(context.reading, months)),
                Some(_) => Cow::Borrowed(data::MILK_OVER),
                None => Cow::Owned(format!(
                    "{} {} {}",
                    data::MILK_UNDER,
                    infant_milk_advice(context.reading, None),
                    data::MILK_OVER
                )),
            },
            Self::Drink => age_bands(
                age(),
                [data::DRINK_UNDER, data::DRINK_MID, data::DRINK_OVER],
                0,
            ),
            Self::Snack => age_bands(
                age(),
                [data::SNACK_UNDER, data::SNACK_MID, data::SNACK_OVER],
                1,
            ),
            Self::PlantMilk => match (age(), line.text.contains("rice")) {
                (None, _) | (Some(..=11), true) => Cow::Owned(format!(
                    "{} {}",
                    data::PLANT_MILK_BABY,
                    data::PLANT_MILK_RICE
                )),
                (Some(..=11), false) => Cow::Borrowed(data::PLANT_MILK_BABY),
                (Some(_), true) => Cow::Borrowed(data::PLANT_MILK_RICE),
                (Some(_), false) => Cow::Borrowed(data::PLANT_MILK_CHILD),
            },
            Self::Allergens => {
                if PEOPLE_INTRO.is_match(&line.padded)
                    && !has_phrase(&line.padded, data::ALLERGEN_FOOD)
                {
                    return None;
                }
                Cow::Borrowed(data::ALLERGEN_REPLY)
            }
            Self::Verdict { water_ask } => {
                if !wants_verdict(context, line, subject, water_ask) {
                    return None;
                }
                Cow::Borrowed(reply)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn baby_owner_distinguishes_meals_for_a_child_from_the_speakers_food() {
        let child = Reading::new("can my baby have cheese");
        let owner = Context::new(&child, &[]);
        assert!(Guard::FeedsSomeone.passes(&owner));
        assert_eq!(
            Reply::Verdict { water_ask: false }.answer(&owner, "cheese", &["cheese"]),
            Some(Cow::Borrowed("cheese"))
        );
        let adult = Reading::new("can i eat baby rice");
        let adult = Context::new(&adult, &[]);
        assert!(!Guard::FeedsSomeone.passes(&adult));
        let condition = Reading::new("my baby has kidney problems");
        let condition = Context::new(&condition, &[]);
        assert_eq!(
            Reply::Verdict { water_ask: false }.answer(&condition, "offal", &["liver", "kidney"]),
            None
        );
    }

    #[test]
    fn named_age_and_previous_age_choose_the_correct_feeding_bands() {
        let reading = Reading::new("what can my seven month old eat");
        let context = Context::new(&reading, &[]);
        let answer = Reply::Stage.answer(&context, "", &[]).expect("named age");
        assert!(answer.starts_with("At this age milk is still the main food"));
        let history = [Exchange {
            user: "my baby is 11 months old".to_owned(),
            bot: String::new(),
            module: "test",
        }];
        let reading = Reading::new("how much milk should she drink now");
        let context = Context::new(&reading, &history);
        let answer = Reply::Milk.answer(&context, "", &[]).expect("recorded age");
        assert!(answer.contains("about 400 ml a day"));
    }

    #[test]
    fn milk_amount_uses_age_specific_guidance_and_keeps_unrelated_ages() {
        for (line, phrase) in [
            (
                "how much milk does a 9 month old need",
                "about 600 ml a day",
            ),
            (
                "how much milk does a 10 month old need",
                "about 400 ml a day",
            ),
            (
                "how much milk for my 8 month old who stopped taking milk",
                "contact NHS 111",
            ),
            ("how much milk for my 4 month old baby", data::MILK_UNDER),
            ("how much milk does my 12 month old need", data::MILK_OVER),
        ] {
            let reading = Reading::new(line);
            let context = Context::new(&reading, &[]);
            let answer = Reply::Milk.answer(&context, "", &[]).unwrap();
            assert!(answer.contains(phrase), "{line}: {answer}");
            assert!(!answer.contains("500 to 600ml"), "{line}");
        }
        let unspecified = Reading::new("how much milk should my baby drink");
        let context = Context::new(&unspecified, &[]);
        let answer = Reply::Milk.answer(&context, "", &[]).unwrap();
        assert!(answer.contains(data::MILK_UNDER) && answer.contains(data::MILK_OVER));
        assert!(!answer.contains("500 to 600ml"));
    }

    #[test]
    fn registry_reads_age_and_food_introduction_and_declines_a_pet_followup() {
        assert_eq!(
            super::super::lookup("what can my 7 month old eat?").map(|card| card.id),
            Some("babyfood-by-age")
        );
        assert_eq!(
            super::super::lookup("how do i introduce eggs?").map(|card| card.id),
            Some("babyfood-first-taste")
        );
        let dog = [Exchange {
            user: "my dog has been sick".to_owned(),
            bot: String::new(),
            module: "test",
        }];
        let reading = Reading::new("what should i feed him tomorrow?");
        let context = Context::new(&reading, &dog);
        assert!(!Guard::FeedsSomeone.passes(&context));
    }
}
