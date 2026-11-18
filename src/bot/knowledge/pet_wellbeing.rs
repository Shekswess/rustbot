//! Neutering, weight, and coat cards from the pinned pets registry.

use regex::{Regex, RegexSet};
use std::{borrow::Cow, cmp::Reverse, sync::LazyLock};

use crate::bot::{Exchange, Reading};

#[path = "pet_weight_data.rs"]
mod weight_data;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(NEUTER_AGAINST, super::PET_NEUTER_AGAINST);
pattern!(NEUTER_AFTERCARE, super::PET_NEUTER_AFTERCARE);
pattern!(NEUTER_CONFIRM, super::PET_NEUTER_CONFIRM);
pattern!(NEUTER_BEHAVIOUR, super::PET_NEUTER_BEHAVIOUR);
pattern!(NEUTER_NOT_YET, super::PET_NEUTER_NOT_YET);
pattern!(NEUTER_OTHER, super::PET_NEUTER_OTHER_ASK);
pattern!(NEUTER_ALREADY_DONE, super::PET_NEUTER_ALREADY_DONE);
pattern!(NEUTER_NOT_DONE, super::PET_NEUTER_NOT_DONE);
pattern!(NEUTER_STILL_ASKED, super::PET_NEUTER_STILL_ASKED);
pattern!(CAT_SHED, super::PET_CAT_SHED);
pattern!(CAT_NOT_WASHING, super::PET_CAT_NOTWASH);
pattern!(RABBIT, r"\b(?:rabbits?|bunn(?:y|ies))\b");
pattern!(
    CAT,
    r"\b(?:cats?|kittens?|toms?|tomcats?|queens?|mogg(?:y|ie|ies))\b"
);
pattern!(DOG, r"\b(?:dogs?|pups?|pupp(?:y|ies)|bitch(?:es)?)\b");
pattern!(NEUTER_WHEN, r"\b(?:should|when|what age|how old)\b");
pattern!(
    NEUTER_DONE_WORD,
    r"\b(?:neutered|spayed|castrated|the snip|done)\b"
);
pattern!(
    NEUTER_COST,
    r"\b(?:costs?|prices?|much does it|how much (?:is|will|would)|cheapest|afford|pay for)\b"
);
pattern!(
    NEUTER_RECOVERY,
    r"\b(?:recover|recovery|recovering|after the (?:operation|op)|stitches|cones?|how long (?:does|till|until).*(?:take|recover)|come home)\b"
);
pattern!(
    MALE,
    r"\b(?:tom|castrat\w*|buck|he|hes|he's|him|his|boy|male|sire)\b"
);
pattern!(
    FEMALE,
    r"\b(?:spay|spays|spayed|spaying|queen|bitch|doe|she|shes|she's|her|hers|girl|female)\b"
);
pattern!(YOUNG, r"\b(?:kitten|kittens|puppy|pup|puppies)\b");

static OTHER_SPECIES: LazyLock<RegexSet> =
    LazyLock::new(|| phrase_set(super::PET_NEUTER_OTHER_SPECIES, r"\b"));
static WEIGH_ASK: LazyLock<RegexSet> = LazyLock::new(|| phrase_set(super::PET_WEIGH_WORDS, ""));
static WEIGH_DIRECTION: LazyLock<RegexSet> =
    LazyLock::new(|| phrase_set(super::PET_WEIGH_DIRECTION, ""));
static THIN: LazyLock<RegexSet> = LazyLock::new(|| phrase_set(super::PET_GOING_THIN, ""));
static WEIGHT_UP: LazyLock<RegexSet> = LazyLock::new(|| phrase_set(super::PET_WEIGHT_UP, ""));
static THIN_WORDS: LazyLock<Vec<(Regex, Regex)>> = LazyLock::new(|| {
    super::PET_THIN_WORDS
        .iter()
        .map(|word| {
            let word = regex::escape(word);
            (
                Regex::new(&format!(r"\b{word}\b")).expect("weight loss word"),
                Regex::new(&format!(r"\b{word} (?:coat|fur|hair|skin)")).expect("coat question"),
            )
        })
        .collect()
});

fn phrase_set<'a>(phrases: impl IntoIterator<Item = &'a &'a str>, suffix: &str) -> RegexSet {
    RegexSet::new(
        phrases
            .into_iter()
            .map(|phrase| format!(r"\b{}{suffix}", regex::escape(phrase))),
    )
    .expect("pet care phrases")
}

// https://www.bva.co.uk/take-action/our-policies/neutering-of-cats-and-dogs/
// https://www.bsava.com/position-statement/neutering-of-dogs-cats-rabbits-and-ferrets/
fn neuter_why(line: &str) -> Option<&'static str> {
    let against = NEUTER_AGAINST.is_match(line);
    Some(if RABBIT.is_match(line) {
        if against {
            "Discuss the anaesthetic and recovery risks with a vet experienced in rabbits. Neutering can prevent unwanted litters and some womb diseases in females, and may reduce hormonal mounting or aggression. The best time depends on your rabbit's breed and maturity."
        } else {
            "Neutering rabbits prevents unwanted litters and helps prevent womb diseases in females. It can also reduce some hormonally driven behaviour. Ask a rabbit-experienced vet about timing and the risks for your rabbit."
        }
    } else if CAT.is_match(line) {
        if against {
            "Neutering a cat involves an anaesthetic and surgery, so ask your vet about the risks for your cat and how to manage recovery. It prevents unwanted litters and may reduce roaming and urine marking."
        } else {
            "Neutering a cat prevents unwanted litters and may reduce roaming and urine marking. Ask your vet about timing. For kittens, UK veterinary guidance generally recommends around four months."
        }
    } else if !DOG.is_match(line) && OTHER_SPECIES.is_match(line) {
        return None;
    } else if against {
        "Ask your vet to weigh the benefits against the risks for your dog. Sex, breed, maturity, health and lifestyle can change the decision. Some dogs benefit from waiting until physical maturity, and the effects on behaviour vary."
    } else {
        "The benefits and risks of neutering a dog depend on sex, breed, maturity, health and lifestyle. It prevents pregnancy and can reduce some reproductive health risks. Ask your vet whether it is suitable for your dog and what timing makes sense."
    })
}

fn neuter_sex(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let spoken = format!("{} {}", reading.line.clean, reading.line.text).to_lowercase();
    let lines = std::iter::once(spoken.as_str())
        .chain(reading.animal_in_hand.as_deref())
        .chain(earlier.iter().rev().take(2).map(|turn| turn.user.as_str()));
    for line in lines {
        let line = line.to_lowercase();
        if FEMALE.is_match(&line) {
            return Some("female");
        }
        if MALE.is_match(&line) {
            return Some("male");
        }
    }
    None
}

fn neuter(reading: &Reading<'_>, clean: &str, earlier: &[Exchange]) -> Option<&'static str> {
    let said = format!("{clean} {}", reading.line.norm_text).to_lowercase();
    if NEUTER_AFTERCARE.is_match(&said)
        && !NEUTER_WHEN.is_match(&said)
        && !NEUTER_RECOVERY.is_match(clean)
    {
        return None;
    }
    if NEUTER_CONFIRM.is_match(&said)
        && NEUTER_DONE_WORD.is_match(&said)
        && !NEUTER_BEHAVIOUR.is_match(&said)
    {
        return Some(
            "Ask the vet who has your pet's records, or book a vet examination to check whether your pet has been neutered. Tell the vet if they were adopted and share any records you have.",
        );
    }
    let species = if RABBIT.is_match(clean) {
        "rabbit"
    } else if CAT.is_match(clean) {
        "cat"
    } else if DOG.is_match(clean) {
        "dog"
    } else if OTHER_SPECIES.is_match(clean) {
        "other"
    } else {
        super::pets_care::named_or(reading, earlier, "other")
    };
    let behaviour = NEUTER_BEHAVIOUR.is_match(&said);
    if NEUTER_ALREADY_DONE.is_match(&said)
        && !NEUTER_NOT_DONE.is_match(&said)
        && !NEUTER_STILL_ASKED.is_match(&said)
        && !NEUTER_NOT_YET.is_match(&said)
    {
        if NEUTER_OTHER.is_match(&said) && !behaviour {
            return None;
        }
        let carried_behaviour = behaviour
            || earlier
                .iter()
                .rev()
                .take(3)
                .any(|turn| NEUTER_BEHAVIOUR.is_match(&turn.user.to_lowercase()));
        return Some(match (species, carried_behaviour) {
            ("cat", true) => {
                "A neutered cat can still spray or wee outside the tray. Ask your vet about a change in urination, and call an emergency vet immediately if the cat strains and passes little or no urine. If your vet rules out a medical cause, clean the area with a pet-safe enzyme cleaner and check whether the trays are clean, accessible and in quiet places."
            }
            ("dog", true) => {
                "A neutered dog can still mount, mark or roam. Ask your vet to check any sudden changes. A qualified behaviourist can help with persistent habits and show you what to reward instead."
            }
            ("rabbit", _) => {
                "Neutering prevents unwanted litters and can help with some hormonal behaviours. It also reduces the risk of womb diseases in female rabbits. Ask your vet about aftercare and maintaining a healthy weight."
            }
            _ => {
                "Some behaviours can persist after neutering. Ask your vet about any new symptoms, and follow their instructions on recovery, feeding and exercise."
            }
        });
    }
    if NEUTER_COST.is_match(clean) {
        return Some(
            "Ask local practices for a written quote covering the operation, anaesthetic, pain relief and any follow-up or protective collar. The price depends on the animal and the practice. If cost is a concern, ask your vet whether a local charity or low-cost scheme might be available.",
        );
    }
    if NEUTER_RECOVERY.is_match(clean) {
        return Some(
            "Follow your vet's instructions on rest, wound care and any protective collar. Keep your pet quiet and check the wound regularly. Call the vet if it opens, bleeds, swells or discharges, or if your pet is very quiet, vomits or stops eating. A rabbit that stops eating or producing droppings needs immediate veterinary advice.",
        );
    }
    Some(match species {
        "rabbit" => {
            "Ask a rabbit-experienced vet when to neuter your rabbit. Timing depends on sex, breed, maturity and health. Neutering prevents unwanted litters, and spaying can reduce the risk of womb disease."
        }
        "cat" => match neuter_sex(reading, earlier) {
            Some("female") => {
                "Cats can become fertile by four months. Ask your vet about spaying at around four months and what timing suits her health."
            }
            Some("male") => {
                "Ask your vet about neutering him at around four months. Neutering prevents breeding and may reduce roaming and urine marking."
            }
            _ => {
                "Ask your vet about neutering at around four months. Timing depends on your cat's health and circumstances."
            }
        },
        "dog" => {
            "Ask your vet about the right timing for your dog. Sex, breed, physical maturity, health and lifestyle all affect the decision. Dogs mature at different ages, and delaying neutering can be appropriate for some."
        }
        _ => {
            "Ask a vet who treats your animal's species about whether and when to neuter. They can explain the benefits, risks and recovery for your pet."
        }
    })
}

type WeightRows = &'static [(&'static str, f32, f32)];

struct BreedWeights {
    entries: Vec<(&'static str, f32, f32)>,
    breeds: RegexSet,
}

impl BreedWeights {
    fn new(rows: WeightRows) -> Self {
        let mut entries = rows.to_vec();
        entries.sort_by_key(|(breed, _, _)| Reverse(breed.len()));
        let names = entries.iter().map(|(breed, _, _)| breed);
        let breeds = phrase_set(names, r"\b");
        Self { entries, breeds }
    }

    fn find(&self, line: &str) -> Option<(&'static str, f32, f32)> {
        self.breeds
            .matches(line)
            .iter()
            .next()
            .map(|index| self.entries[index])
    }
}

static DOG_WEIGHTS: LazyLock<BreedWeights> =
    LazyLock::new(|| BreedWeights::new(weight_data::DOG_KG));
static CAT_WEIGHTS: LazyLock<BreedWeights> =
    LazyLock::new(|| BreedWeights::new(weight_data::CAT_KG));

fn losing_weight(line: &str) -> bool {
    let thin = THIN.is_match(line)
        || THIN_WORDS
            .iter()
            .any(|(word, coat)| word.is_match(line) && !coat.is_match(line));
    thin && !(WEIGHT_UP.is_match(line)
        && super::PET_WEIGHT_ASK.iter().any(|word| line.contains(word)))
}

// https://wsava.org/Global-Guidelines/Global-Nutrition-Guidelines/
// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/conditions/obesity-in-cats
fn weight(line: &str, is_cat: bool) -> Cow<'static, str> {
    if WEIGH_ASK.is_match(line) && !WEIGH_DIRECTION.is_match(line) {
        if YOUNG.is_match(line) {
            return Cow::Borrowed(
                "Weigh your growing pet regularly on the same scales and record the results. Ask your vet to check their growth against a chart for their age and size. Call the vet if their weight stops increasing or falls.",
            );
        }
        let table = if is_cat { &*CAT_WEIGHTS } else { &*DOG_WEIGHTS };
        if let Some((breed, low, high)) = table.find(line) {
            return Cow::Owned(format!(
                "A rough adult range for a {breed} is {low} to {high} kg. Build and size vary. You should usually be able to feel the ribs under a thin covering and see a waist from above. Ask your vet what weight and body condition are right for your pet."
            ));
        }
        return Cow::Borrowed(
            "Healthy adult weight varies by breed and build. You should usually be able to feel the ribs under a thin covering and see a waist from above. Weigh your pet on the same scales to track changes, and ask your vet to check their body condition.",
        );
    }
    if losing_weight(line) {
        return Cow::Borrowed(
            "Unplanned weight loss needs a vet check, even if your pet is still eating. Record their weight, appetite and any changes in thirst, urination or stools. Contact the vet promptly if the weight loss is rapid or your pet has stopped eating or seems unwell.",
        );
    }
    Cow::Borrowed(if is_cat {
        "Ask your vet for a target weight and a gradual feeding plan. Measure meals, count treats and encourage play. Cats need gradual, supervised weight loss. Call your vet if your cat stops eating."
    } else {
        "Ask your vet for a target weight and daily feeding plan. Measure your dog's meals, count treats and use exercise that suits their age and health. Track their weight and ask your vet about unexpected changes."
    })
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/hair-loss-alopecia-in-cats
fn coat(line: &str) -> &'static str {
    if CAT_NOT_WASHING.is_match(line) {
        "Arrange a vet check if your cat stops grooming or develops a greasy or matted coat. Pain, illness or difficulty reaching part of the body can all affect grooming. Mention any changes in appetite, movement or behaviour."
    } else if CAT_SHED.is_match(line)
        && !["bald", "bare skin", "patch", "sore", "scab"]
            .iter()
            .any(|word| line.contains(word))
    {
        "Loose fur with an even, healthy-looking coat is often normal shedding. Gentle regular brushing can help. Ask your vet if you see bare patches, redness, itching or a sudden change in the coat."
    } else {
        "Ask your vet to check new bald patches, thinning fur or repeated licking. Fleas, skin disease and pain can all cause hair loss. Use only a flea treatment that a vet recommends for your cat."
    }
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let line = &reading.line;
    let clean = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    let clean = clean.to_lowercase();
    let text = match id {
        "neuter-why" => neuter_why(&clean)?,
        "pet-neuter" => neuter(reading, &clean, earlier)?,
        "cat-hair-loss" => coat(&clean),
        "cat-weight" | "cat-weight-loss" | "dog-weight" => {
            let said = if line.norm_text.is_empty() {
                &clean
            } else {
                &line.norm_text
            };
            return Some(weight(said, id != "dog-weight"));
        }
        _ => return None,
    };
    Some(Cow::Borrowed(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clinic_advice_keeps_neutering_timing_behaviour_and_coat_separate() {
        assert!(
            neuter_why("why should i neuter my rabbit")
                .unwrap()
                .contains("womb diseases")
        );
        assert!(neuter_why("why should i neuter my horse").is_none());
        let r = Reading::new("When should I neuter my male kitten?");
        assert!(
            neuter(&r, &r.line.clean, &[])
                .unwrap()
                .contains("around four months")
        );
        let r = Reading::new("How can I tell if my cat was spayed?");
        assert!(neuter(&r, &r.line.clean, &[]).unwrap().contains("records"));
        assert!(coat("my cat is shedding").contains("brushing"));
        assert!(coat("my cat has a bald patch").contains("a vet recommends"));
    }

    #[test]
    fn weight_cards_distinguish_breed_ranges_and_unintended_loss() {
        assert!(
            weight("how much should a jack russell weigh", false)
                .contains("jack russell is 5 to 8")
        );
        assert!(weight("how much should a puppy weigh", false).contains("growing pet"));
        assert!(weight("my cat has lost weight", true).contains("Unplanned weight loss"));
        assert!(weight("how do i help my dog lose weight", false).contains("feeding plan"));
        assert!(!losing_weight("my cat has thin fur"));
    }
}
