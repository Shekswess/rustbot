//! Short, first-person reports about how someone feels.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Memory, compose};

static PERSONAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:(?:so|oh|ah|god|honestly|well|and|also|tbh|ok|okay|literally|seriously|man|ooh|ugh|yeah|no)[ ,]+)*(?:i(?:'m|m|am)|am|i (?:am|feel|feeling|feelin)) +(?P<state>.+)$").expect("first-person body state")
});
static PART: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^my (?:nose|ears?|hands?|feet|fingers?|toes?|face|head|cheeks?|skin|arms?|legs?|knuckles?|wrists?|ankles?|lips?|eyes?|neck|back|chest|tummy|stomach) (?:is|are|feel|feels|look|looks) (?P<state>.+)$").expect("state of a body part")
});

const LEAD: &str = "really so very proper properly absolutely completely totally always still bit little a dead stone bloody permanently bitterly that quite pretty fairly";
const TAIL: &str = "today tonight now still again though honestly already outside here actually really so very absolutely bloody thanks";
const ALSO: &str =
    "tired knackered exhausted shattered grumpy sore achy sweaty miserable worn wet hungry thirsty";
const SITUATIONS: &[&str] = &[
    "all the time",
    "all day",
    "all night",
    "all morning",
    "all afternoon",
    "all evening",
    "all week",
    "all weekend",
    "in here",
    "in this house",
    "in this weather",
    "at night",
    "at work",
    "in the night",
    "right through",
    "to the bone",
    "through to the bone",
    "all day today",
    "all day yesterday",
    "all night long",
];

#[derive(Clone, Copy, PartialEq)]
enum State {
    Cold,
    Hot,
    Hungry,
    Thirsty,
    LightHead,
    Queasy,
    Bunged,
    Clammy,
    Pale,
    Puffy,
    Spotty,
}

impl State {
    fn from_word(word: &str) -> Option<Self> {
        Some(match word {
            "cold" | "freezing" | "frozen" | "chilly" | "chilled" | "perished" | "baltic"
            | "icy" | "shivering" | "shivery" => Self::Cold,
            "boiling" | "hot" | "roasting" | "sweating" | "sweaty" | "sweltering" | "baking"
            | "melting" => Self::Hot,
            "hungry" | "starving" | "starved" | "famished" | "peckish" | "ravenous" => Self::Hungry,
            "thirsty" | "parched" | "dehydrated" => Self::Thirsty,
            "faint" | "fainted" | "fainting" | "faintish" | "woozy" | "dizzy" | "giddy"
            | "wobbly" | "shaky" | "lightheaded" | "groggy" => Self::LightHead,
            "queasy" | "queezy" | "nauseous" | "nauseated" | "sickly" | "bilious" => Self::Queasy,
            "bunged" | "bungedup" | "blockedup" | "sniffly" | "sniffles" | "sniffing"
            | "croaky" | "hoarse" | "sneezing" | "congested" | "husky" | "snotty" => Self::Bunged,
            "clammy" | "clamy" | "clammyish" | "coldsweat" | "coldsweats" => Self::Clammy,
            "pale" | "pallid" | "ashen" | "grey" | "gray" | "greyish" | "grayish" | "mottled"
            | "blotchy" | "washedout" => Self::Pale,
            "puffy" | "bloated" | "swollen" | "puffed" | "puffing" => Self::Puffy,
            "spotty" | "pimply" | "acne" | "brokenout" => Self::Spotty,
            _ => return None,
        })
    }

    fn replies(self) -> &'static [&'static str] {
        match self {
            Self::Cold => &[
                "Wrap up in a dry layer and get somewhere warmer if you can. Is the heating on where you are?",
                "Another layer can help. Is it the whole place, or just the room you are in?",
                "If you're outdoors, get somewhere sheltered and warm. Are your clothes wet?",
            ],
            Self::Hot => &[
                "Try a drink of water and move somewhere cooler. Is there a cooler room or some shade nearby?",
                "Take it slowly and cool your skin with cool water. How long have you felt this hot?",
                "That kind of heat is draining. Have you got somewhere cooler to be?",
            ],
            Self::Hungry => &[
                "Then go and eat something. What have you got in?",
                "Eat first and sort the rest out after. What is quickest to hand?",
                "Even a small meal can help for now. Have you got something easy to eat?",
            ],
            Self::Thirsty => &[
                "Water first, then whatever you were doing. Is there a tap handy?",
                "Have a drink of water and keep the glass next to you afterwards.",
                "Try some water. Have you been in the heat or exercising?",
            ],
            Self::LightHead => &[
                "Sit or lie down somewhere safe until it passes, then get up slowly. Did it come on suddenly?",
                "If you feel like you might faint, lie down and raise your legs if you can. Has this happened before?",
                "Take it slowly and lie down somewhere cool if you've been in the heat. Is the lightheadedness easing?",
            ],
            Self::Queasy => &[
                "Go steady with small sips. Is it just queasiness, or have you been sick?",
                "Have some water in small sips. When did it start?",
            ],
            Self::Bunged => &[
                "Have a drink and rest your voice if it feels hoarse. Is it your nose, your throat, or both?",
                "Is it a blocked nose or a sore throat? Knowing which started first might help narrow it down.",
            ],
            Self::Clammy => &[
                "Sit somewhere safe while you see how you feel. Are you dizzy or feeling faint as well?",
                "Move somewhere cooler if you've been in the heat. Is it just your hands, or all over?",
            ],
            Self::Pale => &[
                "Sit down if you feel unsteady. Did the change come on suddenly, and are you dizzy as well?",
                "Has anyone said you look pale, or have you noticed it yourself? How long has it been like this?",
            ],
            Self::Puffy => &[
                "Which bit is puffy? If the swelling is new and stays, it is worth getting checked.",
                "Is it your stomach, or your ankles or hands? When did you first notice it?",
            ],
            Self::Spotty => &[
                "Try a gentle cleanser and avoid picking at the spots. How long have they been there?",
                "A GP can help if the spots are painful or leaving marks. Where are they coming up?",
            ],
        }
    }
}

fn state(line: &str) -> Option<State> {
    if line.contains('?') || line.contains(" but ") || line.contains(" not ") {
        return None;
    }
    let line = line
        .replace("light-headed", "lightheaded")
        .replace("light headed", "lightheaded")
        .replace("bunged-up", "bungedup")
        .replace("bunged up", "bungedup")
        .replace("blocked-up", "blockedup")
        .replace("blocked up", "blockedup");
    let (tail, part) = if let Some(found) = PERSONAL.captures(&line) {
        (found.name("state")?.as_str(), false)
    } else {
        (PART.captures(&line)?.name("state")?.as_str(), true)
    };
    let words: Vec<_> = tail
        .split_whitespace()
        .map(|word| word.trim_matches([',', '.', '!', ';', ':']))
        .filter(|word| !word.is_empty())
        .collect();
    let mut words = words.as_slice();
    while let Some((word, rest)) = words.split_first() {
        if let Some(state) = State::from_word(word) {
            if part && state == State::Hot {
                return None;
            }
            let plain = rest
                .iter()
                .all(|word| TAIL.split_whitespace().any(|known| known == *word));
            let same_part = part
                && rest.iter().all(|word| {
                    *word == "and"
                        || State::from_word(word) == Some(state)
                        || ["today", "now", "still", "again", "already", "outside"].contains(word)
                });
            let same_subject = !part
                && (SITUATIONS.contains(&rest.join(" ").as_str())
                    || matches!(rest, ["and", also] if ALSO.split_whitespace().any(|known| known == *also)));
            if plain || same_part || same_subject {
                return Some(state);
            }
            if State::from_word(rest.first()?) == Some(state) {
                words = rest;
                continue;
            }
            return None;
        }
        if !LEAD.split_whitespace().any(|known| known == *word) {
            return None;
        }
        words = rest;
    }
    None
}

pub(in crate::bot) fn reply(line: &str, memory: &Memory, seed: &str) -> Option<String> {
    Some(compose::pick(state(line)?.replies(), memory, seed).to_owned())
}
