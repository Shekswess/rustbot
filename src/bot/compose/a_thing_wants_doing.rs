//! A household task described by its current state, from the pinned PyBot composer.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;
static THE_THING_IS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^(?:the|my|our|your|his|her|their)\s+(?P<thing>[a-z]+(?:\s+[a-z]+)?)\s+(?:is|are|looks|seems|gets|got)\s+(?:a bit |a little |really |very |so |quite |proper |right |pretty |well )?(?P<state>worn out|piled up|[a-z]+)\s*$"#).unwrap()
});
const A_THING_WANTS_DOING: &[&str] = &[
    "How bad is it — something you can do in an hour, or has it got away from you?",
    "What's the job on it? I'll work the next step out with you.",
    "Tell me how bad it is and I'll work out where to start.",
    "Right. How long has it been like that?",
    "Where would you start with it?",
];
const A_STATE: &[&str] = &[
    "cluttered",
    "dirty",
    "dusty",
    "faded",
    "filthy",
    "grim",
    "grimy",
    "grubby",
    "long",
    "manky",
    "messy",
    "minging",
    "mucky",
    "overgrown",
    "piled up",
    "rusty",
    "scruffy",
    "shabby",
    "tatty",
    "untidy",
    "worn",
    "worn out",
];
const A_PERSON: &[&str] = &[
    "aunt",
    "baby",
    "bird",
    "birds",
    "boss",
    "boy",
    "boyfriend",
    "brother",
    "cat",
    "chickens",
    "child",
    "children",
    "client",
    "colleague",
    "cousin",
    "dad",
    "daughter",
    "dog",
    "father",
    "fish",
    "friend",
    "friends",
    "girl",
    "girlfriend",
    "grandad",
    "grandma",
    "grandpa",
    "granny",
    "hamster",
    "hens",
    "horse",
    "husband",
    "kid",
    "kids",
    "kitten",
    "mam",
    "mate",
    "mates",
    "mom",
    "mother",
    "mum",
    "nan",
    "neighbor",
    "neighbour",
    "partner",
    "puppies",
    "puppy",
    "rabbit",
    "sister",
    "son",
    "toddler",
    "uncle",
    "wife",
];
const A_FEELING: &[&str] = &[
    "angry",
    "annoyed",
    "ashamed",
    "cross",
    "depressed",
    "done",
    "embarrassed",
    "enough",
    "exhausted",
    "fed",
    "guilty",
    "had",
    "lonely",
    "sad",
    "shattered",
    "sick",
    "stressed",
    "tired",
    "up",
    "upset",
    "worried",
];
const BODY_PARTS: &[&str] = &[
    "ankle",
    "ankles",
    "arm",
    "arms",
    "back",
    "belly",
    "chest",
    "ear",
    "ears",
    "eye",
    "eyes",
    "feet",
    "finger",
    "fingers",
    "foot",
    "gum",
    "gums",
    "hand",
    "hands",
    "head",
    "hip",
    "hips",
    "jaw",
    "knee",
    "knees",
    "leg",
    "legs",
    "neck",
    "shoulder",
    "shoulders",
    "sinus",
    "sinuses",
    "skin",
    "stomach",
    "teeth",
    "throat",
    "toe",
    "toes",
    "tooth",
    "wrist",
    "wrists",
];

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let flat = &reading.line.clean;
    let mut count = 0;
    for word in flat.split_whitespace() {
        count += 1;
        if count > 8 || A_FEELING.binary_search(&word).is_ok() {
            return None;
        }
    }
    let found = THE_THING_IS.captures(flat)?;
    let state = found.name("state")?.as_str();
    let thing = found.name("thing")?.as_str();
    if A_STATE.binary_search(&state).is_err()
        || thing
            .split_whitespace()
            .any(|w| A_PERSON.binary_search(&w).is_ok() || BODY_PARTS.binary_search(&w).is_ok())
    {
        return None;
    }
    let answer = crate::bot::modules::greeting::respond(reading, memory)?;
    super::fallback::is_neutral(&answer)
        .then(|| super::choose_recent(A_THING_WANTS_DOING, memory, flat).to_owned())
}
