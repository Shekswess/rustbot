//! Practical questions about a hospital stay, from bot/compose/ward.py.
use crate::bot::{Memory, Reading};
use fancy_regex::Regex;
use std::sync::LazyLock;
static PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:he|she|they|him|her|them|his|their)(?![a-z])|(?<![a-z])(?:my|our|his|her|their)\s+(?:mum|mom|mother|dad|father|son|daughter|child|kid|wife|husband|partner|brother|sister|gran|granny|grandma|grandad|grandpa|nana|nan|aunt|uncle|cousin|friend|mate|neighbour|neighbor|baby|boy|girl|lad|boss|colleague)(?![a-z])"#).unwrap()
});
static SPEAKER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)(?<![a-z])(?:i|me|my|mine|myself)(?![a-z])"#).unwrap());
static PLACE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:hospital|ward|hospice|care home|nursing home|residential home|clinic|infirmary|a&e|theatre|theatres)(?![a-z])"#).unwrap()
});
static STAFF: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:consultant|doctor|nurse|midwife|anaesthetist|anesthetist|surgeon|registrar|duty nurse)(?![a-z])"#).unwrap()
});
static DETAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:operation|surgery|op|procedure|scan|x-ray|xray|mri|ct scan|ultrasound|results|test results|notes|chart|discharge|discharged|visitors|visiting|visiting hours)(?![a-z])"#).unwrap()
});
static ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:how long|how soon|how many days|when|what time|what ward|what room|what unit|what bed|what hospital|which|where|has (?:she|he|they)|have they|is (?:she|he|they)|are they|will (?:she|he|they)|can i|can we|could i|am i allowed|are we allowed|did they)(?![a-z])"#).unwrap()
});
static SOMETHING_WRONG: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:pain|painful|hurts?|hurting|aching|aches?|sore|swollen|swelling|bleeding|bled|fever|temperature|rash|infection|infected|vomit|vomiting|dizzy|breathless|bruise|wound|symptom|diagnos\w*)(?![a-z])"#).unwrap()
});
static A_PET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:vet|vets|veterinary|puppy|kitten|rabbit|hamster|guinea|budgie|parrot|dog|cat|horse|pony)(?![a-z])"#).unwrap()
});
static HOME_AGAIN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:is (?:now )?home|are (?:now )?home|was home|were home|got home|back home|came home|has come home|had come home|left (?:the )?hospital|came out of (?:the )?hospital|got out of (?:the )?hospital|out of (?:the )?hospital now|(?<!come )(?<!coming )(?<!comes )(?<!get )(?<!getting )(?<!go )(?<!going )(?<!going to )home from (?:the )?hospital|(?:has|have|had) been discharged|was discharged|is discharged)(?![a-z])"#).unwrap()
});
static A_STAY_IN_THE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?<![a-z])(?:in for|in there|in here|still in|be in|is (?:she|he|they) in|are they in|staying|stay overnight|staying overnight|can i stay|out of (?:there|hospital)|discharg\w*|visitors?|visiting|room|bed|bay|ward|(?:coming|came|come) home|(?:going|went) home|get home|getting home|back on (?:her|his|their) feet)(?![a-z])"#).unwrap()
});
static THE_BOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)(?<![a-z])(?:you|your|yours|yourself)(?![a-z])"#).unwrap());
const THE_WARD_KNOWS: &[&str] = &[
    "Nobody here can see {whose} notes, so the ward is the way to find out: ring and ask for the nurse looking after {whom}, and put the question as you put it to me.",
    "That sits in {whose} notes, and nothing here can read them. Ring the ward and ask for the nurse on duty - they will tell you what is happening and what happens next.",
    "Only the people looking after {whom} can say. Ring the ward, say who you are and who you are asking about, and ask them straight.",
    "I can't see {whose} chart from here. Ring the ward and ask for the nurse looking after {whom}: they have the notes in front of them and they answer this all day.",
    "The answer is in {whose} notes, so it has to come from the ward. Ring them, ask for the nurse on duty, and say what you need to know - they can look it up while you are on the phone.",
];
const STAY_ESTIMATE: &[&str] = &[
    "How long is the ward's to say, and they will tell you: ask for the plan in days rather than \"a while\", and ask who to ring for updates. What have they said so far?",
    "That depends on what they are treating, so it is their team's answer rather than mine. Ask what has to happen before they can come home - that is the number that matters. Have they given you a plan?",
    "Nobody here can say, and the team looking after them will: they give a rough plan and revise it as things change. Ask what to expect over the next few days. How are they in themselves?",
];

static HOW_LONG_IN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:and |so )?how (?:long|many days) (?:will|is|are|might|could) (?:he|she|they) (?:(?:likely to|going to|still) )?(?:be )?(?:in|in for|in there|in hospital|on the ward|staying)[?.!]*$").unwrap()
});

fn has(pattern: &Regex, line: &str) -> bool {
    // PyBot's ward module abstains on a failed reading, including a matcher error.
    pattern.is_match(line).unwrap_or(false)
}

fn care(line: &str) -> bool {
    has(&PLACE, line) || has(&STAFF, line) || has(&DETAIL, line)
}

pub(in crate::bot) fn duration_followup(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = reading.folded();
    if !has(&HOW_LONG_IN, said) {
        return None;
    }
    let current = memory.turns.iter().rev().take(4).find_map(|turn| {
        let before = turn.user.to_lowercase();
        if has(&HOME_AGAIN, &before) || has(&A_PET, &before) {
            Some(false)
        } else {
            (has(&PLACE, &before) && has(&PERSON, &before)).then_some(true)
        }
    });
    current
        .filter(|active| *active)
        .map(|_| super::choose_recent(STAY_ESTIMATE, memory, &reading.line.clean).to_owned())
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = reading.folded();
    if said.split_whitespace().take(3).count() < 3 || has(&THE_BOT, said) || !has(&ASK, said) {
        return None;
    }
    let mut room = String::from(said);
    for turn in &memory.turns[memory.turns.len().saturating_sub(3)..] {
        room.push(' ');
        room.push_str(&turn.user.to_lowercase());
    }
    if has(&A_PET, &room)
        || has(&SOMETHING_WRONG, &room)
        || has(&HOME_AGAIN, &room)
        || !care(&room)
        || !care(said) && !has(&A_STAY_IN_THE_LINE, said)
    {
        return None;
    }
    static SHE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)(?<![a-z])she(?![a-z])").unwrap());
    static HE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)(?<![a-z])he(?![a-z])").unwrap());
    let (whom, whose) = if has(&PERSON, said) {
        if has(&SHE, said) {
            ("her", "her")
        } else if has(&HE, said) {
            ("him", "his")
        } else {
            ("them", "their")
        }
    } else if has(&SPEAKER, said) && has(&PLACE, &room) {
        ("you", "your")
    } else {
        return None;
    };
    let index = (memory.turns.len().wrapping_mul(31) + said.chars().count()) % THE_WARD_KNOWS.len();
    Some(
        THE_WARD_KNOWS[index]
            .replace("{whose}", whose)
            .replace("{whom}", whom),
    )
}
