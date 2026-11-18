//! Wording-dependent behaviour and pet-loss cards from the pinned pets table.

use super::{
    PET_CAT_CHILDREN, PET_CAT_WARNING, PET_CHILD_WORRY, PET_LEAD_PHRASES, PET_LEAD_PULL,
    PET_LEAD_SLIP, PET_LEAD_SUBJECTS, PET_LEAD_TIGHT, PET_LOSS_PREY_REPLY, PET_LOSS_REPLY,
    PET_SCRATCH_DOGS, PET_SCRATCH_REPLIES,
};
use crate::bot::{Exchange, Reading, understanding::kinds::Kind};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(WORDS, r"[a-z']+");
pattern!(PLAIN_TEXT, r"[^a-z0-9' ]+");
pattern!(OTHER_DEATH, super::PET_LOSS_OTHER_DEATH);
pattern!(ATE_OR_ILL, super::PET_LOSS_ATE_OR_ILL);
pattern!(DYING, super::PET_LOSS_DYING);
pattern!(LOSS_SAID, super::PET_LOSS_SAID);
pattern!(PUT_TO_SLEEP, super::PET_LOSS_SLEEP);
pattern!(PREY, super::PET_LOSS_PREY);
pattern!(HUNTING, super::PET_LOSS_HUNTING);
pattern!(THEIR_PET, super::PET_LOSS_THEIR_PET);
pattern!(REPEATED, super::PET_LOSS_REPEATED);
pattern!(BARE, super::PET_LOSS_BARE);
pattern!(NAMED, super::PET_LOSS_NAMED);
pattern!(CABLE_SHOCKED, super::PET_CABLE_SHOCKED);
pattern!(CABLE_CHEWED, super::PET_CABLE_CHEWED);
pattern!(CABLE_LIVE, super::PET_CABLE_LIVE);
pattern!(
    CABLE_UNPLUGGED,
    r"\b(?:was (?:unplugged|disconnected|not plugged in)|wasn't plugged in|already unplugged|(?:unplugged|disconnected) (?:cable|charger|wire|lead|cord))\b"
);
pattern!(
    ELECTRIC_SHOCK,
    r"\b(?:electric(?:al)? shock|electrocuted|got (?:a )?shock|was shocked)\b"
);

fn names_one(line: &str, phrases: &[&str], word_byte: impl Fn(&u8) -> bool + Copy) -> bool {
    phrases.iter().any(|phrase| {
        line.match_indices(phrase).any(|(start, matched)| {
            let before = start
                .checked_sub(1)
                .and_then(|index| line.as_bytes().get(index));
            let after = line.as_bytes().get(start + matched.len());
            !before.is_some_and(word_byte) && !after.is_some_and(word_byte)
        })
    })
}

pub(super) fn names_a_lead(reading: &Reading<'_>) -> bool {
    let line = &reading.line.clean;
    (line
        .split_whitespace()
        .any(|word| PET_LEAD_SUBJECTS.contains(&word))
        || PET_LEAD_PHRASES.iter().any(|phrase| line.contains(phrase)))
        && reading
            .context_fields
            .subject
            .as_deref()
            .is_none_or(|subject| {
                let subject = subject.to_lowercase();
                subject.trim().is_empty()
                    || WORDS
                        .find_iter(&subject)
                        .any(|word| PET_LEAD_SUBJECTS.contains(&word.as_str()))
            })
}

pub(super) fn dog_pulling(reading: &Reading<'_>, options: &[&'static str; 3]) -> &'static str {
    let line = &reading.line.clean;
    if PET_LEAD_SLIP.iter().any(|slip| line.contains(slip)) {
        return options[0];
    }
    let says = |cues: &[&str]| line.split_whitespace().any(|word| cues.contains(&word));
    if line.contains("collar") && says(PET_LEAD_TIGHT) && !says(PET_LEAD_PULL) {
        options[1]
    } else {
        options[2]
    }
}

pub(super) fn cat_biting(reading: &Reading<'_>, options: &[&'static str; 2]) -> &'static str {
    options[usize::from(names_one(
        &reading.line.clean,
        PET_CAT_WARNING,
        u8::is_ascii_alphanumeric,
    ))]
}

pub(super) fn moult(reading: &Reading<'_>, options: &[&'static str; 2]) -> &'static str {
    let rabbit = names_one(
        &reading.line.clean,
        &["rabbit", "rabbits", "bunny", "bunnies"],
        u8::is_ascii_lowercase,
    );
    options[usize::from(!rabbit)]
}

pub(super) fn cat_and_children(reading: &Reading<'_>) -> Option<&'static str> {
    if reading.kind == Kind::Statement {
        let text = format!(
            " {} {} ",
            reading.line.clean,
            reading.line.text.to_lowercase()
        );
        let plain = PLAIN_TEXT.replace_all(&text, " ");
        if !PET_CHILD_WORRY
            .iter()
            .any(|worry| plain.contains(&format!(" {worry} ")))
        {
            return None;
        }
    }
    Some(PET_CAT_CHILDREN)
}

pub(super) fn scratching(reading: &Reading<'_>) -> Option<&'static str> {
    let words = &reading.line.words;
    let line = words.join(" ");
    let target = [
        (
            &[
                "door",
                "doors",
                "back door",
                "front door",
                "bedroom door",
                "gate",
            ][..],
            "door",
        ),
        (
            &[
                "floor", "floors", "carpet", "carpets", "rug", "rugs", "bedding", "bed", "blanket",
                "cushion", "cushions", "mat", "mattress",
            ][..],
            "floor",
        ),
        (
            &[
                "sofa",
                "sofas",
                "settee",
                "couch",
                "armchair",
                "chair",
                "chairs",
                "curtain",
                "curtains",
                "furniture",
                "table",
                "tables",
                "woodwork",
                "wallpaper",
                "wall",
            ][..],
            "furniture",
        ),
        (
            &[
                "himself",
                "herself",
                "themselves",
                "skin",
                "raw",
                "sore",
                "itchy",
                "itching",
                "scratching",
                "so much",
                "all the time",
                "constantly",
                "bleeding",
                "bald",
                "patch",
                "patches",
            ][..],
            "skin",
        ),
    ]
    .into_iter()
    .find_map(|(keys, target)| keys.iter().any(|key| line.contains(key)).then_some(target))
    .unwrap_or("");
    let target = if target == "furniture"
        && words
            .iter()
            .any(|word| PET_SCRATCH_DOGS.contains(&word.as_str()))
    {
        "furniture-dog"
    } else {
        target
    };
    PET_SCRATCH_REPLIES
        .iter()
        .find_map(|&(key, reply)| (key == target).then_some(reply))
}

pub(super) fn loss(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let line = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    let line = line.to_lowercase();
    if line.is_empty() {
        return Some(PET_LOSS_REPLY);
    }
    if OTHER_DEATH.is_match(&line) || ATE_OR_ILL.is_match(&line) && !DYING.is_match(&line) {
        return None;
    }
    if earlier.last().is_some_and(|turn| {
        let previous = format!("{} {}", turn.user, turn.bot).to_lowercase();
        PREY.is_match(&previous)
            && HUNTING.is_match(&previous)
            && (PREY.is_match(&line) && !THEIR_PET.is_match(&line)
                || REPEATED.is_match(&line)
                || BARE.is_match(&line) && !NAMED.is_match(&line))
    }) {
        return Some(PET_LOSS_PREY_REPLY);
    }
    (LOSS_SAID.is_match(&line) || PUT_TO_SLEEP.is_match(&line)).then_some(PET_LOSS_REPLY)
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/other-veterinary-advice/first-aid-for-an-electric-shock-in-pets
const CABLE_EMERGENCY: &str = "Keep clear of your pet and the cable until the power is off. If you can reach the main switch safely, turn off the power there. Keep others away and call an emergency vet now, even if your pet looks well, because injuries can show up later.";
const CABLE_UNKNOWN_POWER: &str = "If the cable may have been live when your pet bit it, keep clear of both until the power is safely off and call an emergency vet now. Injuries can show up later. Keep your pet away from the damaged cable and replace it before using the appliance again.";
// https://www.rspca.org.uk/adviceandwelfare/pets/rabbits/environment
const CABLE_PREVENTION: &str = "Route electrical cables where your pet cannot reach them, using secure cable trunking or barriers. Remove damaged cables from use and replace them before reconnecting anything. Give your pet something safe to chew, such as a suitable chew toy or hay for a rabbit.";

pub(super) fn cables(reading: &Reading<'_>) -> &'static str {
    let line = PLAIN_TEXT.replace_all(&reading.line.clean, " ");
    let chewed = CABLE_CHEWED.is_match(&line);
    let unplugged = CABLE_UNPLUGGED.is_match(&line);
    if CABLE_SHOCKED.is_match(&line)
        || ELECTRIC_SHOCK.is_match(&line)
        || chewed && !unplugged && CABLE_LIVE.is_match(&line)
    {
        CABLE_EMERGENCY
    } else if chewed && !unplugged {
        CABLE_UNKNOWN_POWER
    } else {
        CABLE_PREVENTION
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/pet_routines.rs"]
mod fixtures;

#[cfg(test)]
#[test]
fn pet_routine_callbacks_match_pinned_replies_and_guards() {
    use super::{guards, rows};
    let mut failures = Vec::new();
    for &fixtures::Case(id, prior, prompt, allowed, expected) in fixtures::CASES {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        for line in prior {
            bot.turn(line);
        }
        let reading = Reading::new(prompt);
        let context = guards::Context::new(&reading, &bot.memory.turns);
        let row = rows()
            .find(|row| row.domain == "pets" && row.id == id)
            .unwrap();
        let actual = row.answer(&context);
        let guard = row
            .guard
            .passes(&context, id, row.key.first().copied().unwrap_or_default());
        if (guard, actual.as_deref()) != (allowed, expected) {
            failures.push(format!("{id} {prompt:?} after {prior:?}: expected {allowed} {expected:?}, got {guard} {actual:?}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[cfg(test)]
#[test]
fn electrical_pet_care_uses_the_reported_exposure() {
    for (prompt, expected) in [
        (
            "How do I stop my house rabbit chewing wires?",
            CABLE_PREVENTION,
        ),
        (
            "My rabbit bit a live cable and is shaking.",
            CABLE_EMERGENCY,
        ),
        (
            "My puppy chewed the charger, and then I unplugged it.",
            CABLE_UNKNOWN_POWER,
        ),
        ("My puppy chewed an unplugged charger.", CABLE_PREVENTION),
        ("My kitten bit a switched on wire.", CABLE_EMERGENCY),
        (
            "My dog chewed the mains cable but looks fine.",
            CABLE_EMERGENCY,
        ),
    ] {
        let reading = Reading::new(prompt);
        assert_eq!(cables(&reading), expected, "{prompt}");
    }
    for (prompt, expected) in [
        (
            "How do I stop my house rabbit chewing wires?",
            CABLE_PREVENTION,
        ),
        (
            "My rabbit bit a live cable and is shaking.",
            CABLE_EMERGENCY,
        ),
    ] {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        let answer = bot.turn(prompt);
        assert_eq!(answer.module, Some("answers"), "{prompt}: {answer:?}");
        assert_eq!(answer.reply, expected, "{prompt}");
    }
}
