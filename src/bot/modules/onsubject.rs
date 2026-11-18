//! Resolve a short fallback question against the subject of a recent statement.

use std::{collections::HashSet, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use regex::Regex;

use crate::bot::{Memory, Reading, compose, knowledge::lookup};

#[path = "onsubject_data.rs"]
mod data;
use data::*;

macro_rules! patterns {
    ($($pattern:ident),+ $(,)?) => {
        $(static $pattern: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$pattern).expect(stringify!($pattern)));)+
    };
}

patterns!(
    A_ASK_OPENER,
    A_BACK_REFERENCE,
    A_EMPTY_ASK_OPENER,
    A_POINTER_ASK,
    A_HELP_ASK,
    A_COMPARISON_ASK,
    A_COOKING_TIME_ASK,
    A_FOOD_PRONOUN,
    A_ROASTING_WORD,
    A_BOILING_WORD,
    A_OTHER_METHOD,
    A_NAMED_FOOD,
    A_COOKING_ASK,
    A_LOST_THING,
    A_MACHINE_CUE,
    A_A_QUESTION_ASK,
    A_JUDGEMENT_ASK,
    A_JUDGEMENT_OF_AMOUNT,
    A_WORDS_ASK,
    A_NAMING_ASK,
    A_EXPLANATION,
    A_ITS_OWN_SUBJECT,
    A_DVLA_WORD,
    A_THING_VERB_ASK,
    A_OPEN_ASK,
    A_THERE,
    A_NAMED_PLACE,
    A_REPEAT_LEAD,
    A_EXISTENTIAL_THERE,
);

static A_RECALL_ASK: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::A_RECALL_ASK).expect("stored fact ask"));
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").expect("subject words"));
static SENTENCES: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(r"(?<=[.!?])\s+").expect("answer sentences"));

struct SubjectKind {
    keys: &'static str,
    details: &'static str,
    source: &'static str,
}

const TENANCY: usize = 0;
const CAR: usize = 2;
const BILLS: usize = 3;
const TRAVEL: usize = 9;
const MONEY: usize = 11;

const KINDS: &[SubjectKind] = &[
    SubjectKind {
        keys: "landlord tenancy tenant tenants deposit eviction lease letting lettings rent flat flats",
        details: "Check the tenancy type, the agreement and anything you were sent in writing.",
        source: "Shelter or your council's tenancy team can explain the rules for your area.",
    },
    SubjectKind {
        keys: "vet puppy puppies kitten kittens dog dogs cat cats rabbit rabbits hamster guinea parrot fleas flea litter",
        details: "The animal's age, breed and what's changed can all matter.",
        source: "Your vet can advise about health or behaviour. The breeder or rescue may know their history.",
    },
    SubjectKind {
        keys: "mot tyre tyres brake brakes clutch gearbox engine battery batteries hybrid mileage garage mechanic dvla car cars",
        details: "Have the car's age, mileage and last MOT to hand.",
        source: "A garage can advise about the car. GOV.UK has the official rules for DVLA paperwork.",
    },
    SubjectKind {
        keys: "bill bills electric electricity energy supplier meter standing tariff direct debit arrears",
        details: "Check the account and what the supplier has already been told.",
        source: "Contact the supplier for the account details. Citizens Advice can help if you're disputing a bill.",
    },
    SubjectKind {
        keys: "prescription pharmacist pharmacy symptom symptoms migraine infection swelling swollen rash piles pain dose tablets tablet medicine doctor hospital nurse clinic surgery",
        details: "Note when it began and what you've already tried.",
        source: "Ask a pharmacist about a medicine or minor symptoms. If you need medical advice urgently in the UK, contact your local urgent care service.",
    },
    SubjectKind {
        keys: "sacked redundancy disciplinary warning rota shift shifts overtime payslip employer manager boss hr union interview contract job workplace work leave",
        details: "Have the dates and any written notice or agreement to hand.",
        source: "Acas can advise on employment in Great Britain. In Northern Ireland, contact the Labour Relations Agency.",
    },
    SubjectKind {
        keys: "gcse gcses a-level a-levels exam exams revision homework teacher lesson lessons school college university ucas course term paper papers",
        details: "Check the subject, exam board and dates.",
        source: "Ask the school or consult the exam board's own specification.",
    },
    SubjectKind {
        keys: "boiler radiator radiators thermostat dishwasher washer washing fridge freezer oven hob toilet shower leak leaking damp mould ceiling roof gutter gutters socket wiring plumber electrician limescale drain drains filter pump tap taps",
        details: "Note the model and when the problem started.",
        source: "Check the manual for that model or ask a qualified tradesperson about the fault.",
    },
    SubjectKind {
        keys: "curtain curtains rail drill shelf shelves paint wall walls screw screws rawplug rawplugs tile tiles tiling grout sanding filler sealant flatpack",
        details: "Check its size and what it's fixed to.",
        source: "Check the fitting instructions or ask the manufacturer.",
    },
    SubjectKind {
        keys: "passport visa boarding luggage suitcase adapter plug flight flights airport hotel train booking holiday abroad japan spain france italy greece portugal germany holland turkey egypt mexico canada america australia ireland scotland wales england london paris rome madrid lisbon",
        details: "Check the travel dates and the details of your booking.",
        source: "The operator's website or your booking confirmation can give you the terms.",
    },
    SubjectKind {
        keys: "receipt refund returns warranty guarantee seller order delivery shop store",
        details: "Find the receipt and the date you bought it.",
        source: "Check the seller's policy or contact them in writing.",
    },
    SubjectKind {
        keys: "overdraft mortgage loan benefits universal pension savings debt invoice bill bills tax salary wages wage pay bank credit budget money",
        details: "Check the account and the figures in writing.",
        source: "Ask the bank or organisation that runs the account for the exact terms.",
    },
];

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Words,
    Time,
    Amount,
    How,
    What,
    YesNo,
}

impl Shape {
    fn read(line: &str) -> Self {
        if A_WORDS_ASK.is_match(line) {
            Self::Words
        } else if line.contains("how long") || line.contains("how soon") {
            Self::Time
        } else if line.contains("how much") || line.contains("how many") {
            Self::Amount
        } else {
            match line.split_whitespace().next() {
                Some("how") => Self::How,
                Some("what" | "which" | "who" | "when" | "where") => Self::What,
                _ => Self::YesNo,
            }
        }
    }
}

fn clean(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn words(line: &str) -> Vec<&str> {
    WORD.find_iter(line).map(|word| word.as_str()).collect()
}

fn bare(line: &str) -> bool {
    let words = words(line);
    if words.is_empty() || words.len() > 8 || !(A_ASK_OPENER.is_match(line) || line.ends_with('?'))
    {
        return false;
    }
    let content = words
        .iter()
        .filter(|word| word.len() > 2 && !STOP.split_whitespace().any(|stop| stop == **word))
        .count();
    content <= 2
        && (A_BACK_REFERENCE.is_match(line)
            || words.len() <= 7 && A_EMPTY_ASK_OPENER.is_match(line))
}

fn stem(word: &str) -> &str {
    let word = word.trim_matches('\'');
    let stem = ["ing", "ies", "ied", "es", "ed", "s"]
        .iter()
        .filter_map(|suffix| word.strip_suffix(suffix))
        .find(|base| base.len() > 2)
        .unwrap_or(word);
    stem.strip_suffix('e')
        .filter(|_| stem.len() > 3)
        .unwrap_or(stem)
}

fn named(line: &str) -> Vec<(usize, &'static str)> {
    let words = words(line);
    KINDS
        .iter()
        .enumerate()
        .filter_map(|(kind, data)| {
            let key = data
                .keys
                .split_whitespace()
                .filter(|key| !A_LIGHT_KEY.contains(key))
                .find(|key| {
                    words.iter().enumerate().any(|(index, word)| {
                        if stem(word) != stem(key) {
                            return false;
                        }
                        let before = index
                            .checked_sub(1)
                            .and_then(|index| words.get(index))
                            .copied()
                            .unwrap_or("");
                        let after = words.get(index + 1).copied().unwrap_or("");
                        if *key == "washing"
                            && ![
                                "machine",
                                "machines",
                                "powder",
                                "liquid",
                                "detergent",
                                "basket",
                                "line",
                                "drum",
                                "load",
                                "cycle",
                            ]
                            .iter()
                            .any(|word| *word == before || *word == after)
                        {
                            return false;
                        }
                        let earlier = index
                            .checked_sub(2)
                            .and_then(|index| words.get(index))
                            .copied()
                            .unwrap_or("");
                        !A_PLACE_WORD.contains(key)
                            || !(A_PLACE_PREP.contains(&before)
                                || A_DETERMINER.contains(&before)
                                    && A_PLACE_PREP.contains(&earlier))
                    })
                })?;
            Some((kind, key))
        })
        .collect()
}

fn recently_named(memory: &Memory, present: &str) -> Vec<String> {
    memory
        .turns
        .iter()
        .rev()
        .map(|turn| clean(&turn.user))
        .filter(|line| {
            !line.is_empty()
                && line != present
                && !A_A_QUESTION_ASK.is_match(line)
                && !A_EXPLANATION.is_match(line)
        })
        .take(2)
        .collect()
}

fn place(line: &str) -> Option<&str> {
    A_NAMED_PLACE
        .captures_iter(line)
        .filter_map(|captures| captures.get(1))
        .map(|word| word.as_str())
        .find(|word| {
            word.len() > 2
                && !A_NOT_A_PLACE.contains(word)
                && !STOP.split_whitespace().any(|stop| stop == *word)
        })
}

fn cap(text: &str) -> String {
    let mut letters = text.chars();
    letters.next().map_or_else(String::new, |first| {
        format!("{}{}", first.to_uppercase(), letters.as_str())
    })
}

fn about_place(line: &str, previous: &str, shape: Shape) -> Option<String> {
    if !A_THERE.is_match(line) || A_EXISTENTIAL_THERE.is_match(line) || shape == Shape::Time {
        return None;
    }
    let place = place(previous)?;
    let ask = A_THERE.replace(line, format!("in {place}"));
    let answer = lookup::lookup(&ask)?;
    if !matches!(
        answer.domain,
        "travel" | "geography" | "places" | "weather" | "money"
    ) {
        return None;
    }
    let said = SENTENCES
        .split(&answer.reply)
        .filter_map(Result::ok)
        .take(2)
        .collect::<Vec<_>>()
        .join(" ");
    if A_REPEAT_LEAD.is_match(&said) || !said.to_lowercase().contains(place) {
        return None;
    }
    Some(format!("{}, then. {said}", cap(place)))
}

fn knowledge_sentence(previous: &str, memory: &Memory) -> Option<String> {
    let card = lookup::lookup(previous)?;
    let recent: Vec<String> = memory
        .turns
        .iter()
        .rev()
        .take(3)
        .map(|turn| clean(&turn.bot))
        .collect();
    let seen: HashSet<_> = recent.iter().flat_map(|value| words(value)).collect();
    for sentence in SENTENCES.split(&card.reply).filter_map(Result::ok) {
        let text = sentence.to_lowercase();
        let content: Vec<_> = words(&text)
            .into_iter()
            .filter(|word| word.len() > 3)
            .collect();
        let overlap = content.iter().filter(|word| seen.contains(**word)).count();
        if (40..=250).contains(&sentence.chars().count())
            && !content.is_empty()
            && overlap * 5 < content.len() * 3
        {
            return Some(sentence.to_owned());
        }
    }
    None
}

fn cooking_prompt(line: &str, memory: &Memory) -> Option<String> {
    if !A_COOKING_TIME_ASK.is_match(line)
        || !A_FOOD_PRONOUN.is_match(line)
        || A_NAMED_FOOD.is_match(line)
        || memory.turns.iter().rev().take(3).any(|turn| {
            let line = clean(&turn.user);
            A_COOKING_ASK.is_match(&line)
                || A_NAMED_FOOD.is_match(&line)
                || A_ROASTING_WORD.is_match(&line)
        })
    {
        return None;
    }
    if A_ROASTING_WORD.is_match(line) {
        Some("Roasting time depends on what it is, how much it weighs and the oven temperature. What are you roasting, and how much does it weigh?".to_owned())
    } else if A_BOILING_WORD.is_match(line) {
        Some(
            "Boiling time depends on what it is and how soft you want it. What are you boiling?"
                .to_owned(),
        )
    } else if A_COOKING_ASK.is_match(line) && !A_OTHER_METHOD.is_match(line) {
        Some("Cooking time depends on the food, its size and how you're cooking it. What are you cooking, and how?".to_owned())
    } else {
        None
    }
}

fn reply(line: &str, memory: &Memory) -> Option<String> {
    if !bare(line)
        || A_POINTER_ASK.is_match(line)
        || A_HELP_ASK.is_match(line)
        || A_JUDGEMENT_ASK.is_match(line)
        || A_NAMING_ASK.is_match(line)
        || A_ITS_OWN_SUBJECT.is_match(line)
    {
        return None;
    }
    let recent = recently_named(memory, line);
    let current = named(line);
    let subject = recent.iter().find_map(|previous| {
        let prior = named(previous);
        if prior.len() > 1
            && let Some(here) = current
                .iter()
                .find(|hit| prior.iter().any(|then| hit.0 == then.0))
        {
            return Some((*here, previous.as_str()));
        }
        prior.first().copied().map(|item| (item, previous.as_str()))
    });
    if subject.is_none()
        && let Some(hint) = cooking_prompt(line, memory)
    {
        return Some(hint);
    }
    if A_RECALL_ASK.is_match(line).unwrap_or(false)
        || A_COMPARISON_ASK.is_match(line)
        || A_COOKING_ASK.is_match(line)
    {
        return None;
    }
    let shape = Shape::read(line);
    let Some(((kind, key), earlier)) = subject else {
        return memory
            .turns
            .iter()
            .rev()
            .take(3)
            .map(|turn| clean(&turn.user))
            .filter(|earlier| earlier != line)
            .find_map(|earlier| about_place(line, &earlier, shape));
    };
    if (!current.is_empty() && current.iter().all(|hit| hit.0 != kind))
        || ([BILLS, MONEY].contains(&kind) && A_JUDGEMENT_OF_AMOUNT.is_match(line))
        || (kind == CAR && A_MACHINE_CUE.is_match(earlier))
        || A_LOST_THING.is_match(earlier)
        || (PLACES.contains(&key) && A_THING_VERB_ASK.is_match(line))
    {
        return None;
    }
    if let Some(answer) = about_place(line, earlier, shape) {
        return Some(answer);
    }
    let label = match key {
        "washing" => "the washing machine".to_owned(),
        "sacked" => "the job".to_owned(),
        "mot" => "the MOT".to_owned(),
        "dvla" => "the DVLA paperwork".to_owned(),
        key if PLACES.contains(&key) => cap(key),
        key => format!("the {key}"),
    };
    let topic = &KINDS[kind];
    let garage = kind == CAR && !A_DVLA_WORD.is_match(line) && !A_DVLA_WORD.is_match(earlier);
    let authority = if kind == TENANCY {
        topic.source
    } else if garage {
        "The garage working on the car can give you the details."
    } else if kind == TRAVEL && A_PASSPORT_WORD.contains(&key) {
        "Check the passport or visa rules with the government that issues the document."
    } else {
        topic.source
    };
    if A_OPEN_ASK.is_match(line)
        && current.is_empty()
        && let Some(known) = knowledge_sentence(earlier, memory)
    {
        return Some(format!("{}, then. {known} {authority}", cap(&label)));
    }
    let details = topic.details;
    Some(match shape {
        Shape::Words => format!(
            "{}, then. Say what happened, what you'd like them to do and when you need an answer. {authority}",
            cap(&label)
        ),
        Shape::Amount => format!("I don't have a figure for {label}. {details} {authority}"),
        Shape::Time => format!(
            "{}, then. {details} {authority} Ask them what timeframe applies.",
            cap(&label)
        ),
        Shape::How if garage => format!(
            "{}, then. What settles it is the age, the mileage and the last MOT. I can't talk you through it without seeing it; a garage will tell you over the phone.",
            cap(&label)
        ),
        Shape::How => format!(
            "{}, then. {details} {authority} Ask them which steps apply.",
            cap(&label)
        ),
        Shape::What | Shape::YesNo => format!("{}, then. {details} {authority}", cap(&label)),
    })
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = clean(reading.raw);
    let candidate = reply(&line, memory)?;
    let fallback = compose::fallback::respond(reading, memory)?;
    compose::fallback::unknown_replies()
        .contains(&fallback.as_str())
        .then_some(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Exchange};

    #[test]
    fn onsubject_proposes_only_for_a_short_question_about_the_recent_subject() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("The landlord just put the rent up");
        let turn = bot.turn("Can he do that?");
        assert!(
            turn.candidates
                .iter()
                .any(|offer| offer.module == "onsubject" && offer.reply.contains("tenancy")),
            "{turn:?}"
        );
        let memory = Memory {
            turns: vec![Exchange {
                user: "The car passed its MOT".to_owned(),
                bot: "Good news.".to_owned(),
                module: "greeting",
            }]
            .into(),
            ..Memory::default()
        };
        for line in [
            "Can they sack me now?",
            "How many cars do I have?",
            "Would that be a silly idea?",
        ] {
            assert!(respond(&Reading::new(line), &memory).is_none(), "{line}");
        }
    }

    #[test]
    fn car_followup_uses_the_source_how_shape() {
        let memory = Memory {
            turns: vec![Exchange {
                user: "My car squeaks when I brake in the rain.".to_owned(),
                bot: "Ask a garage.".to_owned(),
                module: "answers",
            }]
            .into(),
            ..Memory::default()
        };
        assert_eq!(
            reply("how do i check the tracking?", &memory).as_deref(),
            Some(
                "The brake, then. What settles it is the age, the mileage and the last MOT. I can't talk you through it without seeing it; a garage will tell you over the phone."
            )
        );
    }

    #[test]
    fn onsubject_can_restore_a_named_destination_to_a_travel_question() {
        let reply = about_place(
            "do i need an adapter there?",
            "we are going to spain",
            Shape::YesNo,
        );
        assert!(reply.is_some_and(|reply| reply.starts_with("Spain, then.")));
    }
}
