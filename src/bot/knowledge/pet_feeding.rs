//! Feeding guidance for a named dog or the dog already under discussion.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Exchange, Reading};

use super::{
    PETS_DOG_FEED_ASK, PETS_DOG_NOT_DOG, PETS_DOG_SICK_NOW, PETS_DOG_SWITCH_WORDS, PETS_DOG_WORD,
    PETS_ILL_ANIMAL_WORDS, PETS_ILLNESS_WORDS,
};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(FEED_ASK, PETS_DOG_FEED_ASK);
pattern!(SICK_NOW, PETS_DOG_SICK_NOW);
pattern!(DOG, PETS_DOG_WORD);
pattern!(NOT_DOG, PETS_DOG_NOT_DOG);
pattern!(PRONOUN, r"\b(?:him|her|his|he|she|it)\b");

fn recent_turns(turns: &[Exchange]) -> &[Exchange] {
    &turns[turns.len().saturating_sub(3)..]
}

pub(super) fn ate_the_other_pets_dinner(words: &[String]) -> bool {
    let line = format!(" {} ", words.join(" "));
    let has = |names: &[&str]| names.iter().any(|name| line.contains(&format!(" {name} ")));
    has(super::PETS_OTHER_DINNER)
        && !has(super::PETS_PERSON_ATE)
        && !has(super::PETS_ANIMAL_ILL)
        && (has(super::PETS_DINNER_STOPPED)
            || (!has(super::PETS_DINNER_ASKED_OR_REFUSED) && has(super::PETS_DINNER_TAKEN)))
}

pub(super) fn followup(reading: &Reading<'_>, turns: &[Exchange]) -> Option<&'static str> {
    let said = reading.folded();
    if !FEED_ASK.is_match(said)
        || SICK_NOW.is_match(said)
        || DOG.is_match(said)
        || !PRONOUN.is_match(said)
    {
        return None;
    }
    let recent = recent_turns(turns);
    let latest_dog = recent
        .iter()
        .rposition(|turn| DOG.is_match(&turn.user.to_lowercase()))?;
    if recent[latest_dog + 1..]
        .iter()
        .any(|turn| NOT_DOG.is_match(&turn.user.to_lowercase()))
    {
        return None;
    }
    Some(dog_feed(said, turns))
}

pub(super) fn dog_feed(clean: &str, turns: &[Exchange]) -> &'static str {
    if PETS_DOG_SWITCH_WORDS
        .iter()
        .any(|word| clean.contains(word))
    {
        return "Most dogs move onto adult food at about a year. Small breeds can go at nine to twelve months; the big ones, a Labrador upwards, do better on puppy food until eighteen months to two years, because their joints are still growing. Change over a week or two rather than overnight: a quarter of the new food for two or three days, then half, then three quarters, and watch his poo - loose means slow down. Keep roughly the same amount and go by his ribs and waist rather than the packet.";
    }
    let earlier = recent_turns(turns)
        .iter()
        .map(|turn| turn.user.to_lowercase())
        .collect::<Vec<_>>();
    let replies = std::iter::once(clean).chain(earlier.iter().map(String::as_str));
    if replies.clone().any(|line| {
        PETS_ILLNESS_WORDS.iter().any(|word| line.contains(word))
            && line
                .split(|c: char| !(c.is_ascii_lowercase() || c == '\''))
                .any(|word| PETS_ILL_ANIMAL_WORDS.contains(&word))
    }) {
        return "Keep tomorrow bland and small: plain chicken and rice, or his normal food soaked soft, in four little meals rather than two big ones. Over a day or two mix his usual food back in. If the sickness comes back, or he stops drinking, ring the vet rather than waiting.";
    }
    if replies
        .clone()
        .any(|line| line.contains("puppy") || line.contains("pup"))
    {
        return "Puppies need more than the adult 2% guide: feed a complete puppy food and follow the packet's weight and age chart. Four small meals a day up to about four months, then three, then two after six months. You should feel his ribs easily but not see them; ask the vet to weigh him at each vaccination visit and adjust from there.";
    }
    "As a rough guide, an adult dog eats about 2% of his body weight a day: 400g of food for a 20kg dog, split into two meals. Use the bag or tin as a starting point, then go by condition: you should feel ribs easily but not see them, and see a waist from above. Puppies need more, and a vet can weigh him and give you an exact amount."
}

#[cfg(test)]
mod tests {
    use super::*;

    fn turn(user: &str) -> Exchange {
        Exchange {
            user: user.to_owned(),
            bot: String::new(),
            module: "test",
        }
    }

    #[test]
    fn pronoun_follows_the_dog_unless_the_subject_changed() {
        let reading = Reading::new("what should i feed him tomorrow?");
        let dog = turn("my dog has been sick");
        assert!(
            followup(&reading, &[dog])
                .is_some_and(|reply| reply.starts_with("Keep tomorrow bland"))
        );
        assert!(
            followup(
                &reading,
                &[turn("my dog has been sick"), turn("my rabbit is hungry")]
            )
            .is_none()
        );
    }
}
