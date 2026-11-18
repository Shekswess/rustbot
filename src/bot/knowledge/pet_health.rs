//! Injury, appetite and dental cards that distinguish the patient and urgency.

use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

use crate::bot::{Exchange, Reading};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(HURT_PASSIVE, super::PET_HURT_PASSIVE);
pattern!(EAT_VERB, super::PET_EAT_VERB);
pattern!(CAT, r"\b(?:cats?|kittens?)\b");
pattern!(DOG, r"\b(?:dogs?|pups?|pupp(?:y|ies))\b");
pattern!(RABBIT, r"\b(?:rabbits?|bunn(?:y|ies))\b");
pattern!(SMALL_PET, super::PET_SMALL_ANIMALS);
pattern!(
    SPEAKER_SICK,
    r"\b(?:i|we) (?:am |are |was |were |have |has |had |keep |kept )?(?:being sick|throwing up|vomit\w*|feel\w* (?:sick|unwell))\b"
);

fn contains(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

const BITE_ACTIVE: &str = "Call 999 in the UK or your local emergency number now while someone is in immediate danger. Move away and get behind a barrier if you can do so safely. Keep children and other people away and follow the emergency operator's instructions.";
const BITE_WOUND: &str = "Apply firm pressure with a clean cloth if the wound is bleeding heavily. Go to A&E for a deep or large wound or bleeding you cannot stop. For a minor wound, wash it with soap and warm water and cover it with a clean dressing. If the bite broke the skin, seek medical advice, especially for a child, a bite to the hand, foot, face or head, or signs of infection. A clinician can assess tetanus and rabies risk.";
const BITE_DOG: &str = "Keep the dog away from people until you can arrange a vet check and get help from a qualified behaviourist. In the UK, you can report a dangerous or out-of-control dog to your local police or council dog warden.";
const BITE_CHILD: &str = "Keep the dog and child physically separated and arrange a vet check and help from a qualified behaviourist. In the UK, you can report a dangerous or out-of-control dog to your local police or council dog warden.";
const OWN_ANIMAL: &str = "Keep the animal away while the person gets help. If the bite or scratch followed a change in your pet's behaviour, arrange a vet check.";

fn wound_and_after(after: &str) -> Cow<'static, str> {
    Cow::Owned(format!("{BITE_WOUND} {after}"))
}

// https://www.nhs.uk/conditions/animal-and-human-bites/
// https://www.gov.uk/control-dog-public/report-a-dog
// https://www.gov.uk/how-report-antisocial-behaviour/report-police
// https://www.bluecross.org.uk/advice/dog/behaviour-and-training/puppy-biting-and-mouthing
fn dog_bite(words: &[String]) -> Option<Cow<'static, str>> {
    let line = format!(" {} ", words.join(" ").to_lowercase());
    let has = |wanted: &[&str]| {
        wanted
            .iter()
            .any(|want| line.contains(&format!(" {want} ")))
    };
    if (has(&["his", "her", "its", "their"])
        && has(&[
            "paw", "paws", "tail", "leg", "legs", "foot", "feet", "ear", "ears", "fur", "skin",
        ]))
        || (has(&["himself", "herself", "itself"])
            && has(&[
                "scratching",
                "scratches",
                "scratch",
                "itching",
                "itches",
                "itchy",
            ]))
        || has(&[
            "fence",
            "fences",
            "lead",
            "leads",
            "sofa",
            "toy",
            "toys",
            "bed",
            "cushion",
            "blanket",
            "cage",
            "nappy",
            "slipper",
            "slippers",
            "shoe",
            "furniture",
            "wall",
            "stick",
            "sticks",
            "skirting",
        ])
    {
        return None;
    }
    let child = has(super::PET_BITE_CHILD);
    let skin = has(super::PET_BITE_SKIN);
    if has(super::PET_BITE_NOW) {
        return Some(Cow::Borrowed(BITE_ACTIVE));
    }
    if has(super::PET_BITE_TEETH) && !skin && !child {
        return Some(Cow::Borrowed(
            "Offer a chew toy when your puppy mouths your hands. Pause play calmly if the biting gets harder and resume once they settle. Give an overtired puppy a chance to rest. Ask a vet or qualified behaviourist if the biting is getting worse or breaking skin.",
        ));
    }
    let bitten = has(&["bit", "bitten", "nipped"]);
    if child && (has(super::PET_BITE_DEED) || bitten || skin) {
        return Some(wound_and_after(BITE_CHILD));
    }
    if skin || (has(super::PET_BITE_OTHERS) && bitten) {
        return Some(wound_and_after(BITE_DOG));
    }
    let advice = "Use a door or gate to prevent another bite while arranging a vet check and help from a qualified behaviourist. Avoid situations that trigger the biting. A vet can check for pain or illness.";
    if !has(super::PET_BITE_HABIT) {
        return Some(Cow::Borrowed(advice));
    }
    let extra = if child {
        "Keep children and the dog physically separated until you have professional advice."
    } else if has(super::PET_BITE_DOOR) {
        "Put the dog in a separate room before answering the door."
    } else {
        "Keep the dog separated from visitors."
    };
    Some(Cow::Owned(format!("{advice} {extra}")))
}

pub(super) fn person_hurt(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    let person = |word: &String| super::PET_HURT_PEOPLE.contains(&word.as_str());
    words.iter().enumerate().any(|(at, word)| {
        super::PET_HURT_VERBS.contains(&word.as_str()) && words[at + 1..].iter().take(4).any(person)
    }) || (HURT_PASSIVE.is_match(&words.join(" ")) && words.iter().any(person))
}

fn person_wound(words: &[String]) -> Cow<'static, str> {
    let line = words.join(" ");
    let named = |phrases: &[&str]| words.iter().any(|word| phrases.contains(&word.as_str()));
    if DOG.is_match(&line) && named(super::PET_BITE_CHILD) {
        wound_and_after(BITE_CHILD)
    } else if DOG.is_match(&line) && named(super::PET_HURT_STRANGERS) {
        wound_and_after(BITE_DOG)
    } else {
        wound_and_after(OWN_ANIMAL)
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/medications/first-aid-for-cuts-and-grazes
const PET_WOUND: &str = "If your pet is bleeding heavily, apply firm pressure with a clean, dry cloth or dressing and call an emergency vet now. Deep wounds, bites and injuries near the eye should be checked by a vet. Gently rinse a minor surface wound with clean water or saline, prevent licking and watch for swelling, redness or discharge. Call your vet if you are unsure.";

fn paw_licking(reading: &Reading<'_>) -> &'static str {
    if CAT.is_match(
        reading
            .animal_in_hand
            .as_deref()
            .unwrap_or(&reading.line.clean),
    ) {
        "A cat that repeatedly licks one paw may have pain, irritation or an injury. If your cat lets you, look for swelling, a damaged claw or something between the toes. Contact your vet if the licking continues or you notice redness, swelling or a limp. Seek urgent advice if they cannot put weight on the paw or seem in severe pain."
    } else {
        "Repeated paw licking can come from itch, irritation, an injury or an infection. Rinse muddy or salty paws with clean water and dry them. Look between the toes if your dog lets you. Contact your vet if the licking continues, or if the paw is red, swollen, smelly or painful."
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/looking-after-your-pet/all-pets/how-to-give-your-pet-a-health-check-at-home
// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/other-veterinary-advice/first-aid-for-collapsed-pets
fn mouth(reading: &Reading<'_>) -> &'static str {
    let line = &reading.line.clean;
    let says = |phrases: &[&str]| {
        reading
            .line
            .words
            .iter()
            .any(|word| phrases.contains(&word.as_str()))
    };
    if contains(line, &["gums", "gum colour", "gum color"]) {
        let normal_pale_pink = line.contains("pale pink")
            && !says(super::PET_GUMS_CHANGED)
            && !says(&["white", "grey", "gray", "blue", "bleached"]);
        if says(super::PET_GUMS_BAD) && !normal_pale_pink {
            return "If your pet's gums have turned pale, white, grey or blue, call an emergency vet now. Mention any weakness, collapse or breathing difficulty. Keep handling to a minimum.";
        }
        if says(super::PET_GUMS_RED) {
            return "Red, swollen, sore or bleeding gums need a vet check. Avoid brushing painful gums until a vet has assessed them. Let the vet know about bad breath, drooling, or trouble eating.";
        }
        if says(&["black", "dark", "patch", "patches"]) {
            return "Some pets naturally have dark patches on their gums. If a patch is new or changed, or your pet seems unwell, contact your vet. Sudden pale, grey or blue gums need emergency veterinary advice.";
        }
        if says(super::PET_GUMS_COLOUR)
            || says(super::PET_GUMS_FINE)
            || says(super::PET_GUMS_ASKING)
        {
            return if says(super::PET_GUMS_CHANGED) {
                "Contact your vet about a change in gum colour. If the gums have turned pale, white, grey or blue, call an emergency vet now."
            } else {
                "Healthy gums are usually moist and pink, and some pets naturally have dark pigment. Get to know your pet's usual colour. Call a vet promptly if it changes."
            };
        }
    }
    "Brush with a soft pet toothbrush and toothpaste made for pets. Introduce it gradually with short sessions. Bad breath, visible tartar, bleeding gums or trouble eating warrant a vet dental check."
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/my-rabbit-isn-t-eating-properly
// https://www.pdsa.org.uk/pet-help-and-advice/looking-after-your-pet/small-pets/guinea-pig-health
fn off_food(clean: &str) -> Option<&'static str> {
    if let Some(verb) = EAT_VERB.find(clean)
        && super::PET_OFF_FOOD
            .iter()
            .filter_map(|animal| clean.find(animal))
            .min()
            .is_some_and(|first| first > verb.start())
    {
        return None;
    }
    Some(if contains(clean, super::PET_OFF_FOOD_FISH) {
        "Check the tank temperature and test the water for ammonia and nitrite. Avoid adding more food while you work out what changed. Get prompt advice from an aquatic vet if the fish is gasping, has trouble swimming or remains off its food."
    } else if contains(clean, super::PET_OFF_FOOD_REPTILE) {
        "Check the enclosure temperature, humidity and lighting against the requirements for your reptile's species. Contact a reptile vet if the appetite change persists or if you notice weight loss, weakness, discharge or trouble breathing."
    } else if contains(clean, super::PET_OFF_FOOD_BIRD) {
        "Call an avian vet promptly if your bird has stopped eating. Seek urgent help if they are fluffed up, weak, having trouble breathing or sitting on the cage floor. Keep them quiet and offer their usual food and fresh water while arranging help."
    } else if contains(clean, super::PET_OFF_FOOD_RODENT) {
        "Call your vet today if your small pet has stopped eating. Mention any weight loss, drooling, weakness or diarrhoea. Offer their usual food and fresh water while arranging advice."
    } else if contains(clean, super::PET_OFF_FOOD_RABBIT) {
        "Call a vet immediately if your rabbit or guinea pig has stopped eating or producing droppings. These can be signs of a serious gut problem. Keep hay and fresh water within reach while you contact the vet."
    } else {
        "Contact a vet promptly if your pet has stopped eating. Tell them when it began and whether there are other changes such as vomiting, fewer droppings or weakness. Seek urgent help if your pet cannot stand, is struggling to breathe or seems severely unwell."
    })
}

pub(super) fn pets_talk(reading: &Reading<'_>) -> bool {
    reading.subject_topic == Some("pets")
        || reading.topics.iter().any(|&(topic, _)| topic == "pets")
        || reading
            .line
            .words
            .iter()
            .any(|word| super::PET_SUBJECT_WORDS.contains(&word.as_str()))
        || SMALL_PET.is_match(&reading.line.clean)
}

fn sick_animal(line: &str) -> Option<&'static str> {
    if CAT.is_match(line) && !DOG.is_match(line) {
        Some(
            "Call your vet if your cat keeps vomiting, even if they seem well in between. Seek urgent help if they cannot keep water down, have blood in their vomit, are lethargic or may have eaten something harmful.",
        )
    } else if DOG.is_match(line) {
        Some(
            "Call your vet if your dog keeps vomiting. Seek emergency help if they are repeatedly retching without bringing anything up, have a swollen belly, collapse or have blood in their vomit. Keep fresh water available and tell the vet when the vomiting started.",
        )
    } else if RABBIT.is_match(line) {
        Some(
            "Rabbits cannot vomit. Call a vet immediately if your rabbit appears to be choking, is drooling, cannot breathe normally or has stopped eating or passing droppings. Keep them quiet and follow the vet's instructions.",
        )
    } else if SMALL_PET.is_match(line) {
        Some(
            "Call a vet who treats your type of pet today. Retching, drooling or trouble eating can have several causes. Seek urgent help if your pet is weak, struggling to breathe or unable to eat. Tell the vet exactly what you saw.",
        )
    } else {
        None
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/vomiting-in-dogs
// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/vomiting-in-cats
fn sick_again(clean: &str, earlier: &[Exchange]) -> Option<&'static str> {
    if let Some(reply) = sick_animal(clean) {
        return Some(reply);
    }
    if SPEAKER_SICK.is_match(clean) || super::pet_symptoms::mentioned_person(clean) {
        return None;
    }
    for turn in earlier.iter().rev().take(4) {
        let line = turn.user.to_lowercase();
        if line == clean {
            continue;
        }
        if let Some(reply) = sick_animal(&line) {
            return Some(reply);
        }
        if super::pet_symptoms::mentioned_person(&line) || SPEAKER_SICK.is_match(&line) {
            return None;
        }
    }
    None
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
    let result = match id {
        "dog-bite-person" => return dog_bite(&line.words),
        "animal-hurt-a-person" => return Some(person_wound(&line.words)),
        "pet-wound" => PET_WOUND,
        "dog-paw-licking" => paw_licking(reading),
        "pet-teeth" => mouth(reading),
        "pet-off-food" => off_food(&clean)?,
        "pet-sick-again" => sick_again(&clean, earlier)?,
        _ => return None,
    };
    Some(Cow::Borrowed(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bite_reports_preserve_puppy_and_person_distinctions() {
        let direct = |prompt| registered("dog-bite-person", &Reading::new(prompt), &[]);
        let attack = direct("My dog has hold of someone and won't let go").unwrap();
        assert!(attack.contains("999"));
        let puppy = direct("How do I stop my puppy mouthing my hands?").unwrap();
        assert!(puppy.contains("chew toy"));
        assert!(direct("My dog is biting his paw").is_none());

        let bystander = Reading::new("My dog bit the postman");
        assert!(person_hurt(&bystander));
        let reply = registered("animal-hurt-a-person", &bystander, &[]).unwrap();
        assert!(reply.contains("A&E"));
        assert!(reply.contains("report a dangerous"));
    }

    #[test]
    fn veterinary_replies_handle_urgency_and_nonpet_statements() {
        let pet = Reading::new("Why have my dog's gums turned white?");
        assert!(mouth(&pet).contains("emergency vet now"));
        assert!(off_food("my son does not eat fish").is_none());
        assert!(
            off_food("my rabbit wont eat")
                .unwrap()
                .contains("vet immediately")
        );

        let earlier = [Exchange {
            user: "My cat was sick".into(),
            bot: String::new(),
            module: "answers",
        }];
        assert!(
            sick_again("she keeps being sick", &earlier)
                .unwrap()
                .contains("your cat")
        );
        assert!(sick_again("i keep being sick", &earlier).is_none());
    }
}
