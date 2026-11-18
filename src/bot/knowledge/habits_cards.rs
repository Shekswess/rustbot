//! Starting a business, daily routines after retirement, and personal reminders.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    PETS,
    r"\b(?:dogs?|pupp(?:y|ies)|pups?|cats?|kittens?|rabbits?|bunn(?:y|ies)|hamsters?|guinea pigs?|gerbils?|ferrets?|parrots?|birds?|horses?|pon(?:y|ies)|tortoises?|pets?|litter tray)\b"
);
pattern!(
    MY_AGE,
    r"\b(?:im|i am|i'm)\s+(?:nearly |almost |just |only )?(?:\d{2}|forty|fifty|sixty|seventy|eighty|ninety)\b|\b(?:at my age|my age|getting older|as i get older|since i turned)\b"
);
pattern!(
    OTHER_ERRAND,
    r"\b(?:mum|mom|mother|dad|father|parents?|grandma|grandmother|grandad|grandfather|gran|nan|nana|granny|wife|husband|partner|spouse|other half|mother in law|father in law|aunt|auntie|uncle|brother|sister)\b[^.?!]{0,40}\b(?:forgets?|forgetting|forgetful|forgot|keeps forgetting|repeats|repeating|confused|confusion)\b"
);
pattern!(
    SOMEONE_ELSE,
    r"\b(?:my|his|her|our|their|the)\s+(?:\w+\s+){0,2}(?:mum|mom|mam|mother|dad|father|parents?|husband|wife|partner|spouse|other half|nan|nana|gran|granny|grandma|grandad|grandpa|grandmother|grandfather|son|daughter|child|kid|brother|sister|aunt|auntie|uncle|cousin|in law|mother in law|father in law|friend|neighbour|neighbor|boss|colleague|coworker|flatmate|housemate|landlord|carer|pal|chum|mate|nephew|niece|grandchild|grandson|granddaughter|fiance|fiancee|stepmum|stepdad|sister in law|brother in law|missus|girlfriend|boyfriend)\b[^.!?]{0,30}?\b(?:forget|forgets|forgetting|forgot|forgotten|forgetful|forgetfulness|memory|remembers?|repeating|repeats|confused|confusion|getting confused|scatty|absent[- ]minded|away with it)\b|\b(?:he|she|they)\s+(?:keeps?|is|are|has|have|always|never|used to)\b[^.!?]{0,25}?\b(?:forget|forgets|forgetting|forgot|forgotten|forgetful|forgetfulness|memory|remembers?|repeating|repeats|confused|confusion|getting confused|scatty|absent[- ]minded|away with it)\b"
);
pattern!(TO, r"\bto\b");

const TRADES: &[&str] = &[
    "builder",
    "plumber",
    "electrician",
    "roofer",
    "decorator",
    "tradesman",
    "handyman",
    "plasterer",
    "carpenter",
    "joiner",
    "painter",
    "landscaper",
    "locksmith",
    "gas engineer",
    "heating engineer",
];
const GOING_ALONE: &[&str] = &[
    "my own",
    "for myself",
    "self employed",
    "self-employed",
    "sole trader",
    "my own boss",
    "go it alone",
    "going solo",
    "go solo",
    "freelance",
    "in business",
    "working for myself",
];
const BUSINESS: &[&str] = &[
    "business",
    "trade",
    "company",
    "firm",
    "shop",
    "sole trader",
];

const START_BUSINESS: &str = "Start with one clear product or service and find out what customers would pay for it. Before trading, check any licence, insurance or registration your activity requires. Keep receipts and invoices from day one and consider a separate account for business money. In the UK, you normally need to register for Self Assessment as a sole trader if gross trading income exceeds £1,000 in a tax year. If you need to register for that tax year, the usual deadline is 5 October after it ends. Check GOV.UK for the rules that apply to you.";
const RETIREMENT: &str = "Leaving work can take away the routine, the people you see and a reason to get out of the house. Start with one regular thing that gives your week some shape, such as a class, a volunteer shift, a walk with someone or a standing coffee date. Keep some time for things you want to do for yourself. If low mood lasts more than two weeks or you're struggling to cope, speak to your GP.";
const MOUTH_TIP: &str = "Notice when it happens and put a substitute there before you start. If you chew pens or bite your lip at your desk, put the pen out of reach and try sugar-free gum if you can chew it safely. If your hands need something to do, keep a small fidget object nearby and use it when you notice the urge.";

fn has(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

pub(crate) fn owns_the_business(reading: &Reading<'_>) -> bool {
    let text = &reading.clean;
    let going_alone = has(text, GOING_ALONE);
    (!has(text, TRADES) || going_alone)
        && (!crate::bot::memory::starting_out_names_something(text)
            || going_alone
            || has(text, BUSINESS))
}

fn references<'a>(reading: &'a Reading<'_>) -> impl Iterator<Item = &'a str> {
    let reference = &reading.reference;
    [
        reference.it.as_deref(),
        reference.that.as_deref(),
        reference.they.as_deref(),
        reference.there.as_deref(),
        reference.he.as_deref(),
        reference.she.as_deref(),
        reading.animal_in_hand.as_deref(),
    ]
    .into_iter()
    .flatten()
}

pub(crate) fn the_speakers_errand(reading: &Reading<'_>) -> bool {
    let text = &reading.clean;
    if OTHER_ERRAND.is_match(text) && !TO.is_match(text) {
        return false;
    }
    let with_references = std::iter::once(text.as_str())
        .chain(references(reading))
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
        .replace('’', "'");
    !SOMEONE_ELSE.is_match(&with_references) && !MY_AGE.is_match(&with_references)
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "start-a-business" => Some(Cow::Borrowed(START_BUSINESS)),
        "retired-and-at-a-loss" => Some(Cow::Borrowed(RETIREMENT)),
        "stop-mouth" => {
            if let Some(carried) =
                references(reading).find(|part| PETS.is_match(&part.to_lowercase()))
                && super::lookup::pet_card_covers(reading, carried)
            {
                return None;
            }
            Some(Cow::Borrowed(MOUTH_TIP))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn business_and_reminders_require_the_intended_subject() {
        assert!(!owns_the_business(&Reading::new(
            "how do i start out in gardening"
        )));
        assert!(owns_the_business(&Reading::new(
            "how do i start out on my own as a plumber"
        )));
        assert!(!the_speakers_errand(&Reading::new(
            "my mom keeps forgetting things"
        )));
        assert!(the_speakers_errand(&Reading::new(
            "i keep forgetting to ring my mum"
        )));
        assert!(!the_speakers_errand(&Reading::new(
            "im 50 and i keep forgetting things"
        )));
    }
}
