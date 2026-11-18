//! Emergency signs with a reported fever in a child.

use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $regex:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($regex).expect(stringify!($name)));
    };
}

pattern!(
    CHILD,
    concat!(
        r"\b(?:baby|babies|toddler|newborn|infant|child|children|kid|kids|son|daughter|little (?:boy|girl|one))\b",
        r"|\b(?:[1-9]|1[0-7])[- ]?(?:year|yr)s?[- ]?old\b|\b[1-9][0-9]?[- ]?(?:week|month)s?[- ]?old\b"
    )
);
pattern!(FEVER, r"\b(?:fever(?:ish)?|temperature|temp)\b");
pattern!(
    EARLIER,
    r"\b(?:last (?:night|week|month|year)|yesterday|days? ago|used to|previously)\b"
);
pattern!(
    RECOVERED,
    r"\b(?:fine now|back to normal|fully recovered|completely better)\b"
);
pattern!(NOT_RECOVERED, r"\b(?:still|again|right now|just now)\b");
pattern!(
    PET_PATIENT,
    r"\b(?:dog|cat|puppy|kitten|rabbit|hamster|pet)\b[^,.?!]{0,24}\b(?:fever|temperature|temp)\b"
);
pattern!(WOMAN, r"\b(?:daughter|sister|girl|mum|mother|she|her)\b");
pattern!(MAN, r"\b(?:son|brother|boy|dad|father|he|him|his)\b");
pattern!(
    NEGATED,
    r"\b(?:no|not|never|without|doesn'?t have|don'?t have|hasn'?t got|haven'?t got|does not have|do not have) (?:longer |any |signs? of |currently |really |very |having |an? )*$"
);
pattern!(
    UNRESPONSIVE,
    r"\b(?:(?:can(?:not|'?t)|couldn'?t) wake|won'?t wake(?: up)?|is unresponsive|is unconscious|not responding|(?:is|are) (?:limp|floppy))\b"
);
pattern!(
    FIRST_FIT,
    r"\b(?:first (?:febrile )?(?:seizure|fit)|(?:seizure|fit) for the first time)\b"
);
pattern!(
    COLOUR,
    concat!(
        r"\b(?:blue|grey|gray|pale|blotchy) (?:lips?|tongue|skin|face)\b|\b(?:going|turning|gone|turned) blue\b",
        r"|\b(?:lips?|tongue|skin|face) (?:is |are |looks? |going |turns? |turning |went |has gone |have gone )*",
        r"(?:very |unusually )?(?:blue|grey|gray|pale|blotchy)\b"
    )
);
pattern!(
    BREATHING,
    concat!(
        r"\b(?:struggl(?:e|es|ing|ed) (?:to breathe|for breath)|(?:can(?:not|'?t)|couldn'?t) (?:breathe|catch (?:his|her|their) breath)|",
        r"(?:hard|difficult) (?:for (?:him|her|them) )?to breathe|(?:difficulty|trouble|problems?) (?:with )?breathing|breathing (?:difficulties|difficulty)|",
        r"(?:breathing|working) (?:very |really )?hard|too breathless to (?:feed|talk|drink)|",
        r"(?:ribs?|tummy|chest) (?:is |are )?(?:sucking|pulling) (?:in|under)|",
        r"grunting (?:with each breath|when breathing)|unusually quiet and still|",
        r"drooling and (?:cannot|can'?t|struggling to) swallow)\b"
    )
);
pattern!(DROWSY, r"\b(?:hard|difficult) to wake\b");
pattern!(VERY_FAST, r"\bbreathing (?:very|really) fast\b");
pattern!(BLUE_OR_GREY, r"\b(?:blue|grey|gray)\b");
pattern!(
    SEPSIS,
    concat!(
        r"\b(?:stiff neck|neck (?:is |feels? )?stiff|non[ -]?blanching rash|",
        r"rash (?:that )?(?:does not|doesn'?t|will not|won'?t) (?:fade|go away)|",
        r"unusually cold hands and feet|light hurts (?:their|his|her) eyes|",
        r"(?:suddenly|extremely) confused|unable to swallow (?:spit|saliva))\b"
    )
);

const SEPSIS_REPLIES: &[&str] = &[
    "That's a 999 or 911 call now: say you're worried about sepsis or meningitis, and give the temperature and how long it has been going on. While you wait, press a clear glass on the rash - if it doesn't fade, tell them that too. Don't give food or drink if they are drowsy.",
    "Ring 999 or 911 and say you're worried about sepsis or meningitis - that is the word that gets them there fast. Keep them warm, don't leave them alone, and don't give them anything to eat or drink while they are drowsy.",
];
const COLOUR_REPLIES: &[&str] = &[
    "Blue or grey lips or face mean not enough oxygen is getting through: ring 999 or 911 now, keep them sitting up if that is easier for their breathing, and stay with them. Say when it started and whether there is a rash that does not fade under a glass.",
    "That colour change is a 999 or 911 call now, not a wait-and-see. Keep them sitting up if that is easier for their breathing, stay with them, and say when it started and whether there is a rash that does not fade under a glass.",
];
const UNRESPONSIVE_REPLY: &str = "Call 999 now in the UK, or your local emergency number elsewhere. Say your child has a fever and you cannot wake them. Check whether they're breathing normally and follow the call handler's instructions. Stay with them.";
const BREATHING_REPLY: &str = "Call 999 now in the UK, or your local emergency number elsewhere. Tell the call handler your child has a fever and describe how they're breathing. Stay with your child and follow the call handler's instructions.";
const FIT_REPLY: &str = "Call 999 now in the UK, or your local emergency number elsewhere. Say this was your child's first seizure and tell the call handler how long it lasted. Stay with your child and follow the call handler's instructions.";
const OTHER_COLOUR_REPLY: &str = "Call 999 now in the UK, or your local emergency number elsewhere. Tell the call handler about your child's fever and the change in their skin colour. Stay with them and follow the call handler's instructions.";

fn reported_sign(pattern: &Regex, said: &str) -> bool {
    pattern
        .find_iter(said)
        .any(|sign| !NEGATED.is_match(&said[..sign.start()]))
}

#[derive(Clone, Copy)]
enum RedFlag {
    Unresponsive,
    FirstFit,
    Colour,
    Breathing,
    Sepsis,
}

fn red_flag(said: &str, fever: bool) -> Option<RedFlag> {
    [
        (RedFlag::Unresponsive, &*UNRESPONSIVE, true),
        (RedFlag::FirstFit, &*FIRST_FIT, fever),
        (RedFlag::Colour, &*COLOUR, true),
        (RedFlag::Breathing, &*BREATHING, true),
        (RedFlag::Breathing, &*VERY_FAST, fever),
        (RedFlag::Sepsis, &*DROWSY, true),
        (RedFlag::Sepsis, &*SEPSIS, fever),
    ]
    .into_iter()
    .find_map(|(flag, pattern, applies)| (applies && reported_sign(pattern, said)).then_some(flag))
}

pub(in crate::bot) fn breathing_or_colour_emergency(said: &str) -> bool {
    red_flag(said, false).is_some()
}

pub(in crate::bot) fn fever_red_flags(said: &str) -> bool {
    red_flag(said, reported_sign(&FEVER, said)).is_some()
}

fn fit(reply: &str, said: &str) -> String {
    match (WOMAN.is_match(said), MAN.is_match(said)) {
        (true, false) => reply.replace("they are", "she is").replace("them", "her"),
        (false, true) => reply.replace("they are", "he is").replace("them", "him"),
        _ => reply.to_owned(),
    }
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = &reading.line.clean;
    if !CHILD.is_match(said)
        || !reported_sign(&FEVER, said)
        || PET_PATIENT.is_match(said)
        || EARLIER.is_match(said) && RECOVERED.is_match(said) && !NOT_RECOVERED.is_match(said)
    {
        return None;
    }
    let answer = match red_flag(said, true)? {
        RedFlag::Unresponsive => UNRESPONSIVE_REPLY.into(),
        RedFlag::FirstFit => FIT_REPLY.into(),
        RedFlag::Colour if BLUE_OR_GREY.is_match(said) => {
            compose::pick(COLOUR_REPLIES, memory, "").to_owned()
        }
        RedFlag::Colour => OTHER_COLOUR_REPLY.into(),
        RedFlag::Breathing => BREATHING_REPLY.into(),
        RedFlag::Sepsis => fit(compose::pick(SEPSIS_REPLIES, memory, ""), said),
    };
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn reported_childhood_fever_with_red_flags_wins_the_safety_band() {
        for (line, expected) in [
            (
                "my son is off school with a fever and a stiff neck, when can he go back",
                "Ring 999 or 911 and say you're worried about sepsis or meningitis",
            ),
            (
                "my child has a fever and a rash that does not fade when pressed",
                "Ring 999 or 911 and say you're worried about sepsis or meningitis",
            ),
            (
                "my son is off school with a fever and he is hard to wake",
                "Ring 999 or 911 and say you're worried about sepsis or meningitis",
            ),
            (
                "my daughter is off school with a fever and i cannot wake her",
                "Call 999 now in the UK",
            ),
            (
                "my son is off school with a fever and his lips are going blue",
                "That colour change is a 999 or 911 call now",
            ),
            (
                "my son is off school with a fever and is breathing very fast",
                "Call 999 now in the UK",
            ),
            (
                "my toddler has a fever and had a seizure for the first time",
                "Call 999 now in the UK",
            ),
            (
                "my son has fever and a stiff neck, our dog is beside him",
                "Ring 999 or 911 and say you're worried about sepsis or meningitis",
            ),
        ] {
            let result = Bot::new(Some("0")).unwrap().turn(line);
            assert_eq!(result.module, Some("safety"), "{line}: {}", result.reply);
            assert!(
                result.reply.starts_with(expected),
                "{line}: {}",
                result.reply
            );
        }
    }

    #[test]
    fn emergency_hook_declines_ordinary_negated_historical_and_adult_cases() {
        for line in [
            "my son is off school with a fever but has no stiff neck",
            "my child is off school with a fever and has a runny nose",
            "my son has no fever, just a stiff neck after sleeping badly",
            "my child had fever and a stiff neck last week but is fine now",
            "my dad is off work with a fever and a stiff neck",
            "my daughter's dog has a fever and a stiff neck",
        ] {
            assert!(
                respond(&Reading::new(line), &Memory::default()).is_none(),
                "{line}"
            );
        }
    }
}
