//! Everyday pet-care callbacks from bot/knowledge/pets.py.
#[path = "pets_care_data.rs"]
mod data;
#[cfg(test)]
#[path = "pets_care_tests.rs"]
mod tests;

use crate::bot::{Exchange, Reading, understanding::kinds::Kind};
use regex::Regex;
use std::{borrow::Cow, collections::HashSet, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    KittenKit,
    Rescue,
    Bed,
    Litter,
    CatLitter,
    Setup,
    Hamster,
    Alone,
    Handling,
    Loss,
    Lost,
    Baby,
    Checkup,
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    WEIGHING_UP,
    FOR_THE_CAT,
    TWO_OF_THEM,
    CLEANING_ASK,
    WHERE_IT_LANDED,
    A_RABBIT_SAID,
    A_GUINEA_SAID,
    A_RODENT_SAID,
    A_FERRET_SAID,
    A_RODENT_OR_GUINEA,
    A_HOW_TO_HOLD,
    A_WONT_BE_HELD,
    A_HURT_OR_FRIGHT,
    CARE_OF_A_NEW_PET,
    A_SHOPPING_ASK,
);

static STOP_ESCAPE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PET_PREVENT_ESCAPE).expect("pet escape prevention"));
static SOAKING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PET_SOAKING).expect("soaked pet"));
static IN_WATER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PET_IN_WATER).expect("pet in water"));
static PREGNANT_SELF: LazyLock<Regex> = LazyLock::new(|| {
    let phrases = super::PET_PREGNANT_SELF
        .iter()
        .map(|phrase| regex::escape(phrase))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!("(?:^|[^a-z0-9])(?:{phrases})(?:$|[^a-z0-9])")).expect("pregnancy cues")
});

static WORDS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static URINE_SPRAY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::CAT_URINE_SPRAY).expect("cat marking"));
static URINE_DISCOMFORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::CAT_URINE_DISCOMFORT).expect("cat urinary symptoms"));
static URINE_STRAINING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:straining|strains|blocked)\b").expect("urinary difficulty"));
static CAT_REPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:my|our|the|his|her|a)\s+(?:cat|kitten|tomcat)\s+",
        r"(?:is|was|has|keeps|kept|can't|cannot|strains|straining|tries|trying|unable)\b",
    ))
    .expect("cat in a symptom report")
});
static CAT_LITTER_PLACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\blitter (?:tray|box)\b").expect("cat litter place"));
static URINARY_WORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:pee\w*|wee\w*|urine|urinat\w*|spray\w*)\b").expect("urinary subject")
});
static URINE_DROPS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:only|just|passes|passing|produces|producing)\s+(?:only\s+)?(?:a\s+few\s+|a\s+)?drops?\b")
        .expect("very little urine")
});
static HUMAN_URINARY_REPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:i|my (?:father|mother|dad|mum|wife|husband|partner|son|daughter|child))\s+",
        r"(?:(?:am|is|was)\s+)?(?:can\s*not|can'?t|unable to|(?:straining|strains)\s+to)\s+",
        r"(?:pee|wee|urinate|pass urine)\b",
    ))
    .expect("human symptom report")
});
static NEUTER_ALREADY_DONE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PET_NEUTER_ALREADY_DONE).expect("past neutering"));
static NEUTER_NOT_DONE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PET_NEUTER_NOT_DONE).expect("neutering negation"));
static NEUTER_STILL_ASKED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PET_NEUTER_STILL_ASKED).expect("neutering question"));
static URINE_DIFFICULTY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:straining|strains|struggling|trying|crying|yelping)\s+(?:to|while|when)\s+",
        r"(?:(?:he|she|it|they)\s+)?(?:pee\w*|wee\w*|urinat\w*|pass\w*\s+urine)\b",
        r"|\b(?:can\s*not|can'?t|unable to|not able to)\s+(?:pee|wee|urinate|pass urine)\b",
        r"|\b(?:blocked bladder|nothing (?:comes|coming|out)|passing nothing|no wee coming)\b",
        r"|\b(?:little(?: or no)?|no)\s+(?:urine|pee|wee)\b",
        r"|\b(?:pass\w*|produc\w*|only)\s+(?:only\s+)?(?:a\s+few\s+|a\s+)?drops?\b",
        r"|\bdifficult(?:y)?(?:\s+(?:to|in))?\s+(?:pass\w* urine|pee\w*|wee\w*|urinat\w*)\b",
    ))
    .expect("urgent urinary difficulty")
});
static TOPIC_WORDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-z0-9]+(?:'[a-z]+)?").unwrap());
static TOPIC_STOPWORDS: LazyLock<HashSet<&str>> =
    LazyLock::new(|| data::TOPIC_STOPWORDS.split_whitespace().collect());
static PET_NAMES: LazyLock<Vec<(&str, Regex)>> = LazyLock::new(|| {
    data::PET_NAMES
        .iter()
        .map(|(name, pattern)| (*name, Regex::new(pattern).expect(name)))
        .collect()
});

fn said<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.norm_text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.norm_text
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/conditions/blocked-bladder-in-cats
pub(super) const CAT_URINE_NOW: &str = "Call your vet or an out-of-hours vet now. A cat that struggles to pee or passes little or no urine may have a blocked bladder, which is an emergency. Don't wait to see if it improves.";
pub(super) const CAT_URINE_PROMPT: &str = "Contact your vet promptly about the change in your cat's urination. Blood or discomfort can have several causes and needs veterinary assessment. If your cat struggles to pee or passes little or no urine, call an emergency vet immediately.";

pub(super) fn cat_urinary_emergency(reading: &Reading<'_>) -> Option<&'static str> {
    let clean = &reading.line.clean;
    (CAT_REPORT.is_match(clean)
        && (CAT_LITTER_PLACE.is_match(clean) && URINE_DIFFICULTY.is_match(clean)
            || URINARY_WORD.is_match(clean) && URINE_DROPS.is_match(clean))
        && !HUMAN_URINARY_REPORT.is_match(clean))
    .then_some(CAT_URINE_NOW)
}

pub(super) fn cat_urine(reading: &Reading<'_>, marking: &[&'static str; 2]) -> &'static str {
    let line = said(reading);
    if [&reading.line.clean, &reading.line.norm_text]
        .into_iter()
        .any(|text| URINE_DIFFICULTY.is_match(text))
        || !URINE_SPRAY.is_match(line) && URINE_STRAINING.is_match(line)
    {
        return CAT_URINE_NOW;
    }
    if URINE_SPRAY.is_match(line) && !URINE_DISCOMFORT.is_match(line) {
        let words = format!("{} {}", reading.line.norm_text, reading.line.clean);
        let done = NEUTER_ALREADY_DONE.is_match(&words)
            && !NEUTER_NOT_DONE.is_match(&words)
            && !NEUTER_STILL_ASKED.is_match(&words);
        return marking[usize::from(!done)];
    }
    CAT_URINE_PROMPT
}

fn named(line: &str) -> Option<&'static str> {
    PET_NAMES
        .iter()
        .find_map(|(name, pattern)| pattern.is_match(line).then_some(*name))
}

pub(super) fn named_or(
    reading: &Reading<'_>,
    earlier: &[Exchange],
    default: &'static str,
) -> &'static str {
    named(said(reading))
        .or_else(|| {
            earlier
                .iter()
                .rev()
                .take(3)
                .find_map(|turn| named(&turn.user.to_lowercase()))
        })
        .unwrap_or(default)
}

fn get(table: &[(&'static str, &'static str)], name: &str) -> Option<&'static str> {
    table
        .iter()
        .find_map(|(animal, reply)| (*animal == name).then_some(*reply))
}

fn kitten_kit(reading: &Reading<'_>) -> Option<&'static str> {
    let line = if reading.line.clean.is_empty() {
        reading.raw.to_lowercase()
    } else {
        reading.line.clean.to_lowercase()
    };
    let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
    if WEIGHING_UP.is_match(&line) && !FOR_THE_CAT.is_match(&line) && !TWO_OF_THEM.is_match(&line) {
        return None;
    }
    if reading
        .line
        .words
        .iter()
        .any(|word| matches!(word.as_str(), "kitten" | "kittens"))
    {
        Some(data::KITTEN_KIT)
    } else {
        Some(data::CAT_KIT)
    }
}

fn rescue(reading: &Reading<'_>) -> &'static str {
    let line = reading.line.clean.to_lowercase();
    if WORDS.find_iter(&line).any(|word| {
        matches!(
            word.as_str(),
            "cat" | "cats" | "kitten" | "kittens" | "kitty"
        )
    }) {
        data::RESCUE_CAT
    } else {
        data::RESCUE_DOG
    }
}

#[derive(Default)]
struct SleepAnimals {
    has_dog: bool,
    has_cat: bool,
}

impl SleepAnimals {
    fn hear(&mut self, word: &str) {
        self.has_dog |= data::DOG_SLEEP_WORDS.binary_search(&word).is_ok();
        self.has_cat |= data::CAT_SLEEP_WORDS.binary_search(&word).is_ok();
    }

    fn reply(&self) -> &'static str {
        match (self.has_dog, self.has_cat) {
            (true, false) => data::WHERE_THE_DOG_SLEEPS,
            (false, true) => data::WHERE_THE_CAT_SLEEPS,
            _ => data::WHERE_EITHER_SLEEPS,
        }
    }
}

fn topic_stem(word: String) -> String {
    if word.len() > 4
        && let Some(root) = word.strip_suffix("ies")
    {
        return format!("{root}y");
    }
    if word.len() > 3 {
        if let Some(root) = word.strip_suffix("es")
            && root.ends_with(['s', 'x', 'z', 'h'])
        {
            return root.to_owned();
        }
        if let Some(root) = word.strip_suffix('s')
            && !root.ends_with('s')
        {
            return root.to_owned();
        }
    }
    word
}

#[derive(Default)]
struct BedTopics<'a> {
    texts: [Option<&'a str>; 2],
    keys: HashSet<String>,
}

impl<'a> BedTopics<'a> {
    // Recover topic titles from the exchange ledger where the pet is unnamed.
    // Short additions extend the topic's words without replacing its title.
    fn note(&mut self, reading: &Reading<'a>) {
        if matches!(reading.kind, Kind::Empty | Kind::Greeting | Kind::Thanks)
            || reading
                .topics
                .iter()
                .any(|(name, _)| *name == "about-the-bot")
        {
            return;
        }
        let line = reading.raw.to_lowercase().replace('’', "'");
        let words = TOPIC_WORDS
            .find_iter(&line)
            .map(|token| {
                let word = token.as_str();
                word.strip_suffix("'s").unwrap_or(word).replace('\'', "")
            })
            .collect::<Vec<_>>();
        let keys: HashSet<_> = words
            .iter()
            .filter(|word| {
                word.len() >= 3
                    && !TOPIC_STOPWORDS.contains(word.as_str())
                    && !word.bytes().all(|byte| byte.is_ascii_digit())
            })
            .cloned()
            .map(topic_stem)
            .collect();
        if self.texts[0].is_some()
            && !reading.line.change_of_subject
            && (reading.reference.followup
                || (words.len() <= 8 && keys.difference(&self.keys).count() <= 1))
        {
            self.keys.extend(keys);
        } else if keys.len() >= 2 || words.len() >= 4 || !reading.topics.is_empty() {
            self.texts = [Some(reading.raw), self.texts[0]];
            self.keys = keys;
        }
    }
}

fn sleeping(reading: &Reading<'_>, earlier: &[Exchange]) -> &'static str {
    let mut animals = SleepAnimals::default();
    for word in &reading.line.words {
        animals.hear(&word.to_lowercase());
    }
    if !animals.has_dog && !animals.has_cat {
        let mut topics = BedTopics::default();
        for turn in earlier {
            topics.note(&Reading::new(&turn.user));
        }
        topics.note(reading);
        for text in topics.texts.into_iter().flatten() {
            let line = text.to_lowercase();
            for word in WORDS.find_iter(&line) {
                animals.hear(word.as_str());
            }
        }
    }
    animals.reply()
}

fn handling(reading: &Reading<'_>) -> &'static str {
    let line = &reading.line.clean;
    if A_GUINEA_SAID.is_match(line) {
        data::GUINEA_HANDLING
    } else if A_RODENT_SAID.is_match(line) {
        data::RODENT_HANDLING
    } else if A_FERRET_SAID.is_match(line) {
        data::FERRET_HANDLING
    } else {
        data::RABBIT_HANDLING
    }
}

pub(super) fn handling_context(reading: &Reading<'_>) -> bool {
    let line = &reading.line.clean;
    A_RABBIT_SAID.is_match(line)
        || A_FERRET_SAID.is_match(line)
        || !A_RODENT_OR_GUINEA.is_match(line)
        || A_HURT_OR_FRIGHT.is_match(line)
        || !(A_HOW_TO_HOLD.is_match(line) || A_WONT_BE_HELD.is_match(line))
}

pub(super) fn new_kitten_context(reading: &Reading<'_>) -> bool {
    if reading.reference.followup || reading.features.asks.is_empty() {
        return false;
    }
    let line = reading.raw.to_lowercase().replace('’', "'");
    CARE_OF_A_NEW_PET.is_match(&line) && !A_SHOPPING_ASK.is_match(&line)
}

pub(super) fn got_wet(reading: &Reading<'_>) -> bool {
    let line = said(reading);
    IN_WATER.is_match(line) || SOAKING.is_match(line)
}

fn lost(reading: &Reading<'_>) -> Option<String> {
    let clean = reading.line.clean.to_lowercase();
    if STOP_ESCAPE.is_match(&clean) {
        return None;
    }
    let named_lost = |line: &str| {
        let line = format!(" {line} ");
        super::PET_LOST_ANIMALS
            .iter()
            .find(|&&name| line.contains(&format!(" {name} ")))
            .copied()
    };
    let noun = named_lost(&reading.line.words.join(" ").to_lowercase())
        .or_else(|| named_lost(&clean))
        .map_or("pet", |name| if name == "kitten" { "cat" } else { name });
    let council = if matches!(noun, "dog" | "doggy" | "puppy" | "pup") {
        " In England and Wales, report a missing dog to your council's dog warden."
    } else {
        ""
    };
    // https://www.gov.uk/check-council-missing-dog
    // https://www.cats.org.uk/help-and-advice/lost-found-and-feral-cats/lost-a-cat
    Some(format!(
        "If your {noun} is microchipped, report them missing to the microchip database and check your contact details.{council} Call local vets and post a photo and the last known location to local lost-pet groups. Ask neighbours to check sheds and garages. Search nearby calmly and avoid chasing a frightened animal."
    ))
}

// https://www.nhs.uk/conditions/toxoplasmosis/
// https://www.cats.org.uk/help-and-advice/caring-for-your-cat/cats-and-your-family/cats-and-your-pregnancy
const BABY_HOME: &str = "Yes, a cat and a baby can live together. Keep the cat away from the cot and pram, and shut the door to the room where the baby sleeps. Always supervise the cat and baby together. Give your cat a quiet place to retreat, keep their routine as steady as possible and let them get used to the baby's scent before they meet.";
const PREGNANCY_LITTER: &str = "During pregnancy, ask someone else to empty the litter tray daily. If you need to do it yourself, wear gloves and wash your hands afterwards. The parasite that causes toxoplasmosis can be present in infected cat poo and contaminated soil. Wear gloves when gardening, wash fruit and vegetables, and cook meat thoroughly.";

fn baby(reading: &Reading<'_>, earlier: &[Exchange]) -> String {
    let start = earlier.len().saturating_sub(2);
    let lines = std::iter::once(reading.line.clean.as_str())
        .chain(earlier[start..].iter().map(|turn| turn.user.as_str()));
    if lines
        .into_iter()
        .any(|line| PREGNANT_SELF.is_match(&line.to_lowercase()))
    {
        format!("{PREGNANCY_LITTER} {BABY_HOME}")
    } else {
        format!(
            "{BABY_HOME} If you are pregnant, let someone else empty the litter tray and wear gloves in the garden."
        )
    }
}

fn checkup(reading: &Reading<'_>, earlier: &[Exchange]) -> &'static str {
    // https://bsava.com/position-statement/vaccination/
    match named_or(reading, earlier, "other") {
        "rabbit" => {
            "Book a yearly health check for your rabbit and ask your vet about vaccination against myxomatosis and both strains of rabbit haemorrhagic disease. Your vet can advise on the right vaccine and when it needs a booster. The check is also a chance to look at teeth, weight and claws."
        }
        "guinea pig" => {
            "Guinea pigs do not usually need routine vaccinations. A vet who sees guinea pigs can check their teeth, weight, claws and skin and tell you how often they should be seen. Weigh your guinea pig regularly and contact a vet promptly if they stop eating."
        }
        "hamster" => {
            "Hamsters do not usually need routine vaccinations. Register with a vet who sees small animals and ask how often your hamster should be checked. Regular weighing can help you notice weight changes. Ask a vet promptly if your hamster stops eating, loses weight or develops a lump."
        }
        "dog" | "cat" => {
            "Book a routine health check about once a year and ask your vet which vaccines are due. Booster intervals vary with the vaccine, age, lifestyle and exposure. Puppies and kittens usually need an initial course. Older pets may benefit from more frequent checks."
        }
        _ => {
            "Ask a vet who treats your type of pet to recommend a check-up and vaccination schedule. How often they need checks or boosters depends on the species, age, health and lifestyle."
        }
    }
}

pub(super) fn reply(
    kind: Reply,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let response = match kind {
        Reply::KittenKit => kitten_kit(reading)?,
        Reply::Rescue => rescue(reading),
        Reply::Bed => sleeping(reading, earlier),
        Reply::Litter => get(data::PET_LITTER, named_or(reading, earlier, "cat"))
            .or_else(|| get(data::PET_LITTER, "cat"))?,
        Reply::CatLitter if WHERE_IT_LANDED.is_match(said(reading)) => data::LITTER_SPOT,
        Reply::CatLitter => data::LITTER_REPLY,
        Reply::Setup | Reply::Hamster => {
            let animal = if matches!(kind, Reply::Hamster) {
                "hamster"
            } else {
                named_or(reading, earlier, "rabbit")
            };
            let advice = if CLEANING_ASK.is_match(said(reading)) {
                data::PET_CLEANING
            } else {
                data::PET_SETUP
            };
            get(advice, animal)?
        }
        Reply::Alone => get(data::SMALL_PET_ALONE, named_or(reading, earlier, "rabbit"))?,
        Reply::Handling => handling(reading),
        Reply::Loss => super::pet_routines::loss(reading, earlier)?,
        Reply::Lost => return lost(reading).map(Cow::Owned),
        Reply::Baby => return Some(Cow::Owned(baby(reading, earlier))),
        Reply::Checkup => checkup(reading, earlier),
    };
    Some(Cow::Borrowed(response))
}
