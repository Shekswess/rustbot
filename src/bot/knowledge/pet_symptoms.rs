//! Veterinary guidance for the pinned hydration, breathing, and mobility cards.

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(NAMED_PET, super::PET_NAMED_ANIMAL);
pattern!(NAMED_PERSON, super::PET_NAMED_PERSON);
pattern!(PANTING, super::PET_PANTING);
pattern!(BREATHING_SIGN, super::PET_BREATHING_SIGN);
pattern!(SMALL_PET, super::PET_SMALL_ANIMALS);
pattern!(SMALL_LIMP, super::PET_SMALL_LIMP);
pattern!(CLEAR_URINE, super::PET_URINE_CLEAR);
pattern!(CAT_NAMED, super::PET_CAT_NAMED);
pattern!(CAT, r"\b(?:cats?|kittens?)\b");
pattern!(RABBIT, r"\b(?:rabbits?|bunn(?:y|ies))\b");
pattern!(HORSE, r"\b(?:horses?|pon(?:y|ies))\b");
pattern!(WORDS, r"[a-z']+");
pattern!(
    DEHYDRATION_ASK,
    r"dehydrat|how do i know|drinking enough|signs?\b"
);
pattern!(
    SPEAKER_DRINKING,
    r"\b(?:i|we)\s+(?:have\s+|just\s+)?(?:drank|swallowed|drunk|drinking|drink)\b|\b(?:should|can)\s+(?:i|we)\s+drink\b"
);
pattern!(
    PERSON_URINE_RESULT,
    r"\b(?:my|our)\s+(?:blood|urine|test|sample)\b|\b(?:i|we)\s+(?:had|have|got)\s+(?:a|my|our)\s+(?:urine|blood)\b"
);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Animal {
    Cat,
    Rabbit,
    DogOrOther,
}

fn animal(reading: &Reading<'_>) -> Animal {
    let named = reading
        .animal_in_hand
        .as_deref()
        .unwrap_or(&reading.line.clean);
    if CAT.is_match(named) {
        Animal::Cat
    } else if RABBIT.is_match(named) {
        Animal::Rabbit
    } else {
        Animal::DogOrOther
    }
}

pub(super) fn mentioned_person(line: &str) -> bool {
    WORDS
        .find_iter(line)
        .any(|word| super::PET_NAMED_PERSON_WORDS.contains(&word.as_str()))
}

fn pet_in_talk(clean: &str, earlier: &[Exchange]) -> bool {
    if NAMED_PET.is_match(clean) || SMALL_PET.is_match(clean) {
        return true;
    }
    if SPEAKER_DRINKING.is_match(clean) || mentioned_person(clean) {
        return false;
    }
    for turn in earlier.iter().rev().take(3) {
        let said = turn.user.to_lowercase();
        if NAMED_PET.is_match(&said) || SMALL_PET.is_match(&said) {
            return true;
        }
        if NAMED_PERSON.is_match(&said) {
            return false;
        }
    }
    false
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/is-my-dog-drinking-and-weeing-too-much
// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/is-my-cat-drinking-and-weeing-too-much
fn thirst(animal: Animal) -> &'static str {
    match animal {
        Animal::Cat => {
            "Contact your vet if your cat suddenly drinks or pees more than usual. Track how much they drink and any changes in appetite, weight or litter tray visits to share with the vet. Wet food supplies water, so a cat on wet food may drink less from the bowl. Keep fresh water available."
        }
        Animal::Rabbit | Animal::DogOrOther => {
            "Contact your vet if your pet suddenly drinks or pees more than usual. The amount they need varies with their size, diet, activity and health. Keep fresh water available and note their water intake, appetite and any changes in urination to share with the vet."
        }
    }
}

fn drink_more(clean: &str, animal: Animal, earlier: &[Exchange]) -> Option<&'static str> {
    if !pet_in_talk(clean, earlier) {
        return None;
    }
    if DEHYDRATION_ASK.is_match(clean) {
        return Some(if animal == Animal::Cat {
            "Contact a vet promptly if your cat is drinking less and has tacky gums, low energy, poor appetite or less urine than usual. A cat that strains or passes little or no urine needs an emergency vet now. Offer fresh water while arranging veterinary advice."
        } else {
            "Check for tacky gums, low energy, poor appetite or less urine than usual. Contact your vet promptly if you notice these signs, especially with vomiting or diarrhoea. Keep fresh water available and tell the vet when the changes began."
        });
    }
    Some(match animal {
        Animal::Cat => {
            "Switch part of the dry food to wet and add a spoonful of water to the wet food. Offer several clean, wide bowls away from the litter tray and food or try a pet fountain. Contact your vet if your cat starts drinking less than usual or seems unwell."
        }
        Animal::Rabbit => {
            "Check that the bottle flows, offer fresh water in a stable bowl as well, and keep it within easy reach. Contact your vet if drinking has fallen. Call immediately if your rabbit stops eating or producing droppings."
        }
        Animal::DogOrOther => {
            "Offer fresh water in a second clean bowl somewhere your pet passes. For a dog you can also add water to their usual food. Contact a vet if your pet starts drinking less than usual or seems unwell."
        }
    })
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/my-rabbit-isn-t-eating-properly
fn not_drinking(clean: &str, animal: Animal, earlier: &[Exchange]) -> Option<&'static str> {
    if !pet_in_talk(clean, earlier) {
        return None;
    }
    Some(if animal == Animal::Rabbit {
        "Call your vet promptly if your rabbit has stopped drinking. If they have also stopped eating or passing droppings, call an emergency vet immediately. While arranging help, check that the bottle flows and offer fresh water in a sturdy bowl."
    } else if SMALL_PET.is_match(clean) {
        "Call your vet today if your small pet has stopped drinking. Seek urgent help if they are weak, hunched or have also stopped eating. Check that the bottle flows, and offer fresh water in a shallow, sturdy bowl while arranging advice."
    } else if animal == Animal::Cat {
        "Contact a vet if your cat has stopped drinking or is drinking less and seems unwell. A cat on wet food may drink less from a bowl. Offer fresh water in a clean bowl. Call an emergency vet immediately if they are straining to pee or passing little or no urine."
    } else {
        "Contact your vet if your pet has stopped drinking. If they are very weak, repeatedly vomiting or unable to keep water down, seek urgent veterinary advice. Offer fresh water in a clean bowl and tell the vet when your pet last drank."
    })
}

// https://www.merckvetmanual.com/toxicology/salt-toxicosis/salt-toxicosis-in-animals
fn salt_water(clean: &str, earlier: &[Exchange]) -> Option<&'static str> {
    pet_in_talk(clean, earlier).then_some("Call your vet now and tell them roughly how much salt water your pet drank and when. Mention any vomiting, wobbliness, drowsiness or seizures. Prevent further salt-water drinking and ask the vet how to manage water intake while you get help.")
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/other-veterinary-advice/first-aid-for-pets-struggling-to-breathe
fn breathing_fast(clean: &str, animal: Animal) -> Option<&'static str> {
    if animal == Animal::DogOrOther && PANTING.is_match(clean) && !BREATHING_SIGN.is_match(clean) {
        return None;
    }
    Some(if animal == Animal::Cat {
        "Call your vet or an out-of-hours vet now if your cat is breathing fast at rest, panting or struggling to breathe. Keep them calm and handle them as little as possible. Phone before travelling so the vet can advise you on moving them safely."
    } else {
        "Call your vet or an out-of-hours vet now if your pet is struggling to breathe, breathing fast at rest, collapsing or has pale or blue gums. Keep them calm and limit handling. Phone before travelling so the vet can advise you on moving them safely."
    })
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/limping-and-stiffness-in-cats
// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/limping-in-rabbits
fn limp(clean: &str, animal: Animal) -> &'static str {
    if SMALL_LIMP.is_match(clean) || animal == Animal::Rabbit {
        "Contact your vet about the limp. Keep your pet on soft bedding with food and water within reach and limit handling. Seek urgent advice if they cannot use the leg, are in severe pain or stop eating. Give only medicines your vet has prescribed."
    } else if animal == Animal::Cat {
        "Keep your cat indoors with food, water and litter on one level. Call an emergency vet if the limp began suddenly, your cat cannot put weight on the leg or is in severe pain. Contact your vet if a mild limp persists. Give only medicines your vet has prescribed."
    } else {
        "Rest your pet and keep movement to a minimum. Contact a vet immediately if they cannot bear weight, have severe pain or had an injury. Contact your vet if a mild limp persists. Give only medicines your vet has prescribed."
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/other-veterinary-advice/first-aid-for-collapsed-pets
// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/conditions/saddle-thrombus-aortic-thromboembolism-in-cats
fn back_legs(horse: bool) -> &'static str {
    if horse {
        "Call the vet now if your horse cannot use its back legs or get up. Keep people and other horses away and avoid moving or trying to lift the horse unless the vet directs you. Tell the vet when it started and about any injury."
    } else {
        "Call your vet or an out-of-hours vet now if your pet suddenly cannot use their back legs or stand. Keep them still and ask the vet how to move them safely. Tell the vet when it started and whether there was an injury. For a gradually worsening leg drag, arrange a vet examination."
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/head-tilt-in-a-rabbit
// https://www.merck-animal-health-usa.com/horse-owners-and-professionals/equine-health-library/pleasure-horse-life-stage/pleasure-horse-health-conditions/
fn wobbly(horse: bool) -> &'static str {
    if horse {
        "Call your vet now if your horse has become unsteady. Keep others clear of the horse, avoid riding or exercising them, and ask the vet how to keep them safe. Tell the vet when it started and whether the horse has fallen or cannot stand."
    } else {
        "Call your vet today about new unsteadiness. Call an emergency vet now if your pet suddenly cannot stand, is falling over, seems distressed or may have eaten something poisonous. Keep them on a flat, quiet surface away from stairs, and note when the unsteadiness started."
    }
}

const NOT_MOVING: &str = "Call an emergency vet now if your pet cannot move or stand, has collapsed, is unresponsive or is struggling to breathe. If they are moving less than usual and seem unwell, contact your vet today. Keep them quiet and limit handling. If a rabbit has also stopped eating or passing droppings, call immediately.";

fn vet_question(clean: &str) -> &'static str {
    let animal_named = WORDS
        .find_iter(clean)
        .any(|word| super::PET_SUBJECT_WORDS.contains(&word.as_str()))
        || SMALL_PET.is_match(clean);
    let human = !animal_named
        && (mentioned_person(clean)
            || super::PET_SPEAKER_ILL
                .iter()
                .any(|phrase| clean.contains(phrase)));
    if human {
        // https://www.nhs.uk/symptoms/shortness-of-breath/
        "For a person, contact their GP or NHS 111 for urgent medical advice. Call 999 for life-threatening symptoms such as severe breathing difficulty, blue lips or sudden confusion."
    } else {
        "Call an emergency vet now for breathing difficulty, collapse, inability to stand, a hard swollen belly, a seizure, suspected poisoning, or straining to pee with little or no urine. For other symptoms, call your vet and describe what you see and when it began. They can advise how soon your pet needs to be seen."
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/conditions/blocked-bladder-in-cats
fn clear_result(clean: &str, earlier: &[Exchange]) -> Option<&'static str> {
    if !CLEAR_URINE.is_match(clean) || PERSON_URINE_RESULT.is_match(clean) {
        return None;
    }
    let cat = CAT_NAMED.is_match(clean)
        || earlier
            .iter()
            .rev()
            .take(3)
            .map(|turn| turn.user.to_lowercase())
            .take_while(|line| CAT_NAMED.is_match(line) || !PERSON_URINE_RESULT.is_match(line))
            .any(|line| CAT_NAMED.is_match(&line));
    cat.then_some("A clear infection test rules out some causes. Contact your vet again if your cat still urinates outside the tray or seems uncomfortable. Call an emergency vet immediately if they strain and pass little or no urine. If your vet has ruled out a medical problem, an extra clean tray in a quiet place and cleaning accidents with a pet-safe enzyme cleaner may help.")
}

fn says_word(clean: &str, words: &[&str]) -> bool {
    WORDS
        .find_iter(clean)
        .any(|word| words.contains(&word.as_str()))
}

// https://www.bhs.org.uk/horse-care-and-welfare/health-care-management/woundwise-hub/assessing-a-wound/
fn shivering(clean: &str, horse: bool) -> &'static str {
    if horse {
        "Check whether your horse is cold or wet. Call your vet promptly if shivering continues or the horse is stiff, sweating, reluctant to move, or off its food. Avoid making a stiff or distressed horse exercise while you seek advice."
    } else if says_word(clean, super::PET_THIN_COAT) {
        "A thin-coated dog can shiver when cold. Move them to a warm, dry spot and look for signs of pain or illness. Contact your vet if the shivering continues once they are warm, or if they seem weak, sore or off their food."
    } else {
        "Move your pet to a warm, dry, quiet spot if they are cold or frightened. Contact your vet if they keep shivering or appear in pain or unwell. Seek emergency advice for collapse, trouble breathing or suspected poisoning."
    }
}

// https://www.bhs.org.uk/horse-care-and-welfare/health-care-management/woundwise-hub/assessing-a-wound/
fn horse_lame(clean: &str) -> &'static str {
    if says_word(clean, &["hoof", "hooves", "foot", "feet", "sole"]) {
        "Call your vet about your horse's lameness. If they cannot bear weight, keep them as still as possible in a safe place and call the vet immediately. Tell the vet which hoof is affected and whether you noticed heat, swelling or a wound. Keep the horse unridden."
    } else {
        "Call your vet about your horse's lameness and keep the horse unridden. If they cannot bear weight, keep them as still as possible in a safe place and call the vet immediately. Tell the vet which leg is affected and when the limp began."
    }
}

// https://www.bhs.org.uk/horse-care-and-welfare/health-care-management/horse-health/equine-diseases/colic-in-horses-causes-symptoms-and-treatment/
fn horse_off_food(clean: &str) -> &'static str {
    if says_word(
        clean,
        &[
            "eating",
            "eat",
            "eats",
            "eaten",
            "food",
            "feed",
            "hay",
            "colic",
            "rolling",
            "droppings",
            "pooing",
        ],
    ) {
        "Call the vet now if your horse has suddenly stopped eating, is rolling repeatedly or has stopped passing droppings. Keep yourself safe around a distressed horse, and ask the vet what to do about feed, water and movement while you wait. Tell them when the signs began."
    } else {
        "Check that your horse can reach clean, fresh water and that the trough or bucket is working. Contact your vet promptly if they are drinking less than usual or will not drink. Call immediately if they are also off their food, rolling, sweating or passing fewer droppings."
    }
}

pub(in crate::bot) const BREATHING_EMERGENCY: &str = "Call your vet or an out-of-hours vet now for laboured breathing, heaving sides, pale or blue gums, collapse, or breathing that stays fast at rest. Keep your pet calm and limit handling. Phone ahead so the vet can advise on moving them safely.";

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/conditions/boas-breathing-problems-in-flat-faced-dogs
fn snoring(clean: &str) -> &'static str {
    if says_word(clean, super::PET_FLAT_FACE) {
        "Persistent snoring or noisy breathing in a flat-faced pet can indicate narrowed airways. Arrange a vet check. Call an emergency vet immediately if they are working to breathe, have blue gums or collapse."
    } else {
        "Contact your vet if the snoring is new, gets louder, interrupts sleep or occurs while your pet is awake. Call an emergency vet immediately if they struggle to breathe, have blue gums or collapse."
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/other-veterinary-advice/first-aid-for-pets-struggling-to-breathe
fn breathing(clean: &str, animal: Animal) -> Option<&'static str> {
    if super::PET_NOT_HUMAN_BREATHING
        .iter()
        .any(|word| clean.contains(word))
        || !says_word(clean, super::PET_BREATHING_SUBJECT)
    {
        return None;
    }
    let after_exercise = super::PET_BEEN_ACTIVE
        .iter()
        .any(|phrase| clean.contains(phrase));
    let red_flag = [
        "blue", "gasp", "collaps", "struggl", "laboured", "labored", "heaving",
    ]
    .iter()
    .any(|flag| clean.contains(flag));
    Some(
        if after_exercise && animal == Animal::DogOrOther && !red_flag {
            "Panting after exercise can ease as a dog rests somewhere cool with water available. If the breathing stays fast or laboured after resting, or you notice blue gums, weakness or collapse, call a vet immediately."
        } else {
            BREATHING_EMERGENCY
        },
    )
}

// https://www.bhs.org.uk/horse-care-and-welfare/health-care-management/seasonal-care/heat-stroke-in-horses-causes-symptoms-and-treatment/
fn horse_sweating(clean: &str) -> Option<&'static str> {
    if !super::PET_EQUINE.iter().any(|word| clean.contains(word)) {
        return None;
    }
    let after_work = super::PET_HORSE_BEEN_WORKING
        .iter()
        .filter(|&&phrase| !matches!(phrase, "in the field" | "coming back from"))
        .any(|phrase| clean.contains(phrase));
    let red_flag = [
        "unsteady",
        "collapse",
        "stiff",
        "off his feed",
        "off her feed",
        "rolling",
    ]
    .iter()
    .any(|phrase| clean.contains(phrase));
    Some(if after_work && !red_flag {
        "Sweating after work can be part of normal cooling. Stop exercise, offer water and cool your horse with water if they are hot. If they remain distressed, weak or unsteady, or are breathing hard and failing to recover, call the vet immediately and continue cooling."
    } else {
        "Call the vet immediately if your horse is sweating unexpectedly at rest, seems distressed, is reluctant to move or is off their food. If overheating is possible, move them into shade if safe and begin cooling with water while you seek veterinary advice."
    })
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<&'static str> {
    let line = &reading.line;
    let clean = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    let clean = clean.to_lowercase();
    let animal = animal(reading);
    let horse = || {
        HORSE.is_match(&clean)
            || reading
                .animal_in_hand
                .as_deref()
                .is_some_and(|pet| HORSE.is_match(pet))
    };
    match id {
        "pet-limp" => Some(limp(&clean, animal)),
        "pet-thirst" => Some(thirst(animal)),
        "pet-drink-more" => drink_more(&clean, animal, earlier),
        "pet-salt-water" => salt_water(&clean, earlier),
        "pet-not-drinking" => not_drinking(&clean, animal, earlier),
        "pet-breathing-fast" => breathing_fast(&clean, animal),
        "pet-when-to-see-a-vet" => Some(vet_question(&clean)),
        "pet-back-legs" => Some(back_legs(horse())),
        "pet-wobbly" => Some(wobbly(horse())),
        "pet-shivering" => Some(shivering(&clean, says_word(&clean, super::PET_EQUINE))),
        "horse-lame" => Some(horse_lame(&clean)),
        "horse-off-food" => Some(horse_off_food(&clean)),
        "pet-snoring" => Some(snoring(&clean)),
        "pet-breathing" => breathing(&clean, animal),
        "horse-sweating" => horse_sweating(&clean),
        "pet-not-moving" => Some(NOT_MOVING),
        "after-a-clear-urine-result" => clear_result(&clean, earlier),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn turn(user: &str) -> Exchange {
        Exchange {
            user: user.to_owned(),
            bot: String::new(),
            module: "answers",
        }
    }

    #[test]
    fn water_questions_distinguish_patient_and_veterinary_urgency() {
        let earlier = [turn("My dog is at the beach")];
        assert!(
            salt_water("he drank salt water", &earlier)
                .unwrap()
                .starts_with("Call your vet now")
        );
        assert!(salt_water("i drank salt water", &earlier).is_none());
        assert!(
            salt_water(
                "he drank salt water",
                &[
                    turn("My dog is at the beach"),
                    turn("My son is at the beach")
                ]
            )
            .is_none()
        );
        assert!(
            not_drinking("my rabbit stopped drinking", Animal::Rabbit, &[])
                .unwrap()
                .contains("passing droppings")
        );
        assert!(
            not_drinking("my cat will not drink", Animal::Cat, &[])
                .unwrap()
                .contains("emergency vet immediately")
        );
    }

    #[test]
    fn breathing_and_mobility_distinguish_panting_and_acute_symptoms() {
        assert!(breathing_fast("my dog is panting", Animal::DogOrOther).is_none());
        assert!(
            breathing_fast("my cat is panting", Animal::Cat)
                .unwrap()
                .starts_with("Call your vet")
        );
        assert!(
            registered(
                "pet-back-legs",
                &Reading::new("My horse's back legs gave way"),
                &[]
            )
            .unwrap()
            .contains("other horses")
        );
        assert!(
            registered(
                "pet-wobbly",
                &Reading::new("My cat keeps falling over"),
                &[]
            )
            .unwrap()
            .contains("emergency vet")
        );
    }

    #[test]
    fn exercise_reports_do_not_hide_breathing_or_equine_emergencies() {
        assert!(
            breathing("my dog is panting after a run", Animal::DogOrOther)
                .unwrap()
                .starts_with("Panting after exercise")
        );
        assert!(
            breathing(
                "my dog is gasping and collapsing after a run",
                Animal::DogOrOther
            )
            .unwrap()
            .starts_with("Call your vet")
        );
        assert!(breathing("i can't breathe", Animal::DogOrOther).is_none());
        assert!(
            horse_sweating("my horse is sweating after a hack")
                .unwrap()
                .starts_with("Sweating after work")
        );
        assert!(
            horse_sweating("my horse is sweating in the field and rolling")
                .unwrap()
                .starts_with("Call the vet immediately")
        );
        assert!(horse_off_food("my horse keeps rolling").starts_with("Call the vet now"));
    }

    #[test]
    fn vet_and_clear_urine_questions_follow_the_patient() {
        assert!(vet_question("my daughter is ill should i see a vet").contains("NHS 111"));
        assert!(
            !vet_question("my daughter's dog cannot stand should i see a vet").contains("NHS 111")
        );
        let earlier = [turn("My cat keeps weeing outside the tray")];
        let result = clear_result("the test came back clear", &earlier).unwrap();
        assert!(result.contains("Contact your vet again"));
        assert!(clear_result("my urine test came back clear", &earlier).is_none());
    }
}
