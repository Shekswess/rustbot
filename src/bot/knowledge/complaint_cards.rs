//! Dental aftercare and named childhood illness replies.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

static EYE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\beyes?\b").unwrap());

pub(super) fn mouth_socket(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    if !text.contains("socket") {
        return true;
    }
    if EYE.is_match(text) {
        return false;
    }
    super::COMPLAINT_MOUTH_WORDS.iter().any(|term| {
        text.match_indices(term).any(|(offset, matched)| {
            (offset == 0 || text.as_bytes()[offset - 1] == b' ')
                && text
                    .as_bytes()
                    .get(offset + matched.len())
                    .is_none_or(|byte| *byte == b' ')
        })
    })
}

const AFTERCARE: &str = "Follow the aftercare instructions your dentist gave you. Contact them or NHS 111 if pain or swelling gets worse, you have a fever, or bleeding does not stop.";
const FACE: &str = "Call 999 or go to A&E if swelling makes it hard to breathe, swallow or speak, you have swelling around an eye, or you have trouble opening your mouth.";
const AFTER_BRANCHES: &[(&[&str], &str)] = &[
    (
        &[
            "smok",
            "vape",
            "vaping",
            "e-cigarette",
            "ecigarette",
            "cigarette",
            "weed",
            "joint",
        ],
        "Avoid smoking or vaping after the extraction, and check your dentist's advice on how long to wait. Ask a pharmacist about ways to manage nicotine cravings while the socket heals.",
    ),
    (
        &[
            "rins",
            "mouthwash",
            "salt water",
            "saltwater",
            "salt-water",
            "swill",
            "gargl",
        ],
        "Avoid rinsing, spitting or touching the socket for the first 24 hours. Afterwards you can usually rinse gently with warm salt water if your dentist advised it. Brush carefully around the area.",
    ),
    (
        &[
            "exercis", "gym", "run", "runn", "lift", "weights", "swim", "football", "cycl",
            "training", "workout", "work out", "sport",
        ],
        "Avoid strenuous activity for the first day, and follow your dentist's instructions if they advised a longer break. Build back up as you recover.",
    ),
    (
        &[
            "collects food",
            "collect food",
            "food gets stuck",
            "gets stuck",
            "traps food",
            "trapped food",
            "bit of food",
            "bits of food",
            "food stuck",
        ],
        "If a gap or socket keeps trapping food, ask the dentist how to clean it safely and whether it needs treatment. Avoid poking the area or using a syringe unless they show you how.",
    ),
    (
        &[
            "after a week",
            "a week after",
            "a week later",
            "week later",
            "for a week",
            "weeks after",
            "two weeks",
            "ten days",
            "10 days",
            "fortnight",
            "last week",
            "last month",
            "not healing",
            "not healed",
            "hasnt healed",
            "hasn't healed",
            "not getting better",
            "getting worse",
            "worse not better",
        ],
        "Contact your dentist for an assessment if the socket pain is getting worse or is not settling. Tell them when the extraction happened and whether there is swelling, a bad taste or a fever.",
    ),
    (
        &[
            "alcohol",
            "beer",
            "wine",
            "pint",
            "vodka",
            "spirits",
            "get drunk",
        ],
        "Avoid alcohol for at least the first 24 hours unless your dentist advises a longer period. Check any sedation and medicine instructions as well.",
    ),
    (
        &[
            "eat",
            "ate",
            "eating",
            "food",
            "chew",
            "chewing",
            "what can i drink",
            "can i drink",
            "coffee",
            "tea",
            "fizzy",
        ],
        "Choose soft food while chewing is difficult. Avoid hot food and drink until the numbness has worn off, and be careful about biting your cheek or tongue.",
    ),
    (
        &["bleed", "bled", "blood"],
        "Some blood in your saliva can happen after an extraction. For active bleeding, sit up and bite firmly on clean gauze placed over the socket, following your dentist's instructions. Get urgent dental advice if it does not stop. Get emergency help for heavy bleeding you cannot control.",
    ),
    (
        &[
            "dry socket",
            "tastes bad",
            "taste bad",
            "bad taste",
            "foul taste",
            "tastes foul",
            "tastes horrible",
            "tastes awful",
            "smells bad",
            "smell bad",
            "throbbing",
            "throb",
            "worse not better",
            "getting worse",
            "more painful",
        ],
        "Worsening throbbing pain or an unpleasant taste after an extraction can happen with dry socket or another problem. Contact your dentist to have it checked.",
    ),
    (
        &[
            "cant taste",
            "can't taste",
            "cannot taste",
            "no taste",
            "lost my taste",
            "numb",
            "numbness",
            "tingling",
            "tingles",
            "still asleep",
        ],
        "Contact the dental team if lip, chin or tongue numbness lasts beyond the expected anaesthetic period or gets worse. They can check whether you need an assessment.",
    ),
    (
        &[
            "stiff",
            "cant open",
            "can't open",
            "cannot open",
            "lockjaw",
            "lock jaw",
            "sore jaw",
            "jaw ache",
            "jaw aches",
            "jaw hurts",
            "jaw is",
        ],
        "Jaw stiffness can occur after an extraction. Seek urgent advice if it is getting worse or comes with fever or swelling. Get emergency help if you are struggling to breathe or swallow or cannot open your mouth.",
    ),
];

const ILLNESSES: &[&str] = &[
    "Measles can cause fever, cough, sore red eyes and a rash. If you think it may be measles, call your GP or NHS 111 before going in. Keep away from school or nursery for at least four days after the rash starts, and mention any contact with someone pregnant or with a weakened immune system.",
    "Mumps often causes painful swelling at the side of the face and can spread through saliva or droplets. Call your GP or NHS 111 if you think you or your child has it. Stay away from school or nursery for at least five days after the swelling begins.",
    "Rubella can cause a rash and swollen glands. Call your GP or NHS 111 before attending if you think it may be rubella, and follow local exclusion guidance. Contact your midwife or GP straight away if you are pregnant and have been exposed.",
    "Slapped cheek disease is a viral infection that can cause red cheeks or a rash. Children can usually go back to school if they feel well enough. Speak to your GP or midwife if you are pregnant or have a weakened immune system or blood disorder and were exposed.",
    "Scarlet fever can cause a sore throat, fever and a sandpaper-like rash. Contact a GP or NHS 111. Keep the child away from school or nursery until at least 24 hours after they start antibiotics.",
    "Whooping cough can cause long bouts of coughing and is especially serious for young babies. Contact your GP or NHS 111 if you suspect it. In England, the usual exclusion period is 48 hours after starting appropriate antibiotics, or 21 days from the start of symptoms without treatment.",
    "Hand, foot and mouth disease can cause mouth ulcers and spots or blisters on the hands and feet. Encourage fluids and ask for urgent advice if your child cannot drink enough or shows signs of dehydration. They can return to school or nursery when they feel well enough.",
    "Glandular fever can cause a sore throat, swollen glands and fatigue. Contact your GP if you suspect it or symptoms persist. Avoid strenuous exercise and heavy lifting until a clinician advises it is safe. Get emergency help for severe breathing trouble or inability to swallow your saliva.",
    "Impetigo is a contagious skin infection that can cause sores with a golden crust. Ask a pharmacist or GP about treatment. In England, stay away from school or nursery until sores have crusted and healed or until 48 hours after starting treatment.",
    "Laryngitis causes a hoarse voice and often settles within one to two weeks. Rest your voice, drink enough fluids and avoid whispering. Contact your GP if it is not getting better, and get urgent help if swallowing is difficult or breathing is affected.",
];

pub(super) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    let reply = match id {
        "toothache" => {
            let lead = if super::pregnancy_cards::said_pregnant(reading) {
                "Contact your dentist about the toothache and let them know you are pregnant. Ask a clinician or pharmacist about suitable pain relief."
            } else {
                "Contact a dentist if the pain persists, is severe or comes with swelling. If you cannot reach a dentist and need urgent help, use NHS 111. Ask a pharmacist which pain relief is suitable."
            };
            return Some(Cow::Owned(format!("{lead} {FACE}")));
        }
        "dental-infection" => {
            return Some(Cow::Owned(format!(
                "Contact a dentist urgently if you think you have a dental abscess or infection. Use NHS 111 if you cannot get urgent dental care. Tell the dentist if there is fever, pus or facial swelling and mention any pregnancy or medicines. {FACE}"
            )));
        }
        "after-a-tooth-out" => {
            let says = |options: &[&str]| {
                options.iter().any(|option| {
                    if option.contains([' ', '-']) {
                        text.contains(option)
                    } else {
                        text.split_whitespace().any(|word| word.starts_with(option))
                    }
                })
            };
            let lead = AFTER_BRANCHES
                .iter()
                .find_map(|(options, lead)| says(options).then_some(*lead))
                .or_else(|| {
                    if says(&["how long", "how soon", "how much time"])
                        && says(&["take", "last", "appointment", "procedure", "surgery", "in the chair"])
                        && !says(&["heal", "ache", "pain", "sore", "swell", "bruis", "numb"])
                    {
                        Some("Ask your dentist how long to allow for the extraction and recovery. Timing depends on the tooth and the procedure.")
                    } else if says(&["heal", "how long", "normal", "days", "weeks", "week", "still"]) {
                        Some("Pain and swelling can last a few days after an extraction. Healing time depends on the procedure. Ask your dentist to check if pain or swelling gets worse rather than easing.")
                    } else {
                        None
                    }
                });
            return Some(lead.map_or(Cow::Borrowed(AFTERCARE), |lead| {
                Cow::Owned(format!("{lead} {AFTERCARE}"))
            }));
        }
        "childhood-illness" => {
            let (_, reply) = super::CHILDHOOD_ILLNESS_GROUPS
                .iter()
                .zip(ILLNESSES)
                .filter_map(|(aliases, &reply)| {
                    aliases
                        .iter()
                        .filter_map(|name| text.find(name))
                        .min()
                        .map(|offset| (offset, reply))
                })
                .min_by_key(|(offset, _)| *offset)?;
            reply
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}
