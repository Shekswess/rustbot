//! Low-priority urgent assessment for current reports about another person.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Memory, Reading, compose};

#[path = "urgent_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $regex:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($regex).expect(stringify!($name)));
    };
}

pattern!(WORD, r"[a-z0-9]+(?:'[a-z]+)?");
pattern!(
    PRO_ANSWERING,
    concat!(
        r"\b(?:my|his|her|their|our|the)\s+(?:carers?|care\s+workers?|support\s+workers?|",
        r"social\s+workers?|personal\s+assistants?|nurses?|midwives?|midwife|doctors?|gps?|",
        r"physios?|therapists?)\b[^.?!]{0,30}\b(?:isn'?t|is\s+not|not|hasn'?t|has\s+not|never|",
        r"no)\s+respond(?:ing|ed|e)?\b",
        r"|\bno\s+response\s+from\s+(?:my|the|our)\s+(?:carer|care\s+worker|support\s+worker|",
        r"social\s+worker|personal\s+assistant|nurse|doctor|gp|physio|therapist)s?\b"
    )
);
pattern!(
    BODY_IN_ROOM,
    concat!(
        r"\b(?:floor|ground|bed|sofa|settee|couch|bath|pavement|street|doorstep)\b",
        r"|\b(?:collapsed|collapsing|fell|fallen|fainted|pass(?:ed)?\s+out|seizure|fitting)\b",
        r"|\b(?:breathing|breath|breathe|pulse|blue|lips|cold|clammy|sweating|eyes|pale|grey|gray)\b",
        r"|\b(?:unconscious|out\s+cold|unresponsive|in\s+front\s+of\s+me|with\s+me|next\s+to\s+me)\b",
        r"|\b(?:to\s+(?:pain|touch|voices?|noise|light|stimulus)|(?:his|her|their)\s+name)\b"
    )
);
pattern!(
    NOSE_BREATH,
    r"\b(?:can'?t|cannot|couldn'?t|could\s+not|struggling|hard|unable|not|trouble)\s+(?:to\s+)?breath\w*[^.?!]{0,18}?\bthrough\b[^.?!]{0,12}?\bnose\b"
);
pattern!(
    FIT_OF,
    r"\bfits?\s+of\s+(?:cough\w*|sneez\w*|giggl\w*|laughter|laughing|screaming|crying|the\s+giggles|rage|temper|anger|pique|panic|inspiration)\b"
);
pattern!(
    FIT_ROUTINE,
    r"\b(?:every\s+(?:few|couple\s+of|so\s+many)\s+(?:months|weeks|years)|twice\s+a\s+year|once\s+a\s+year|a\s+(?:few|couple)\s+of\s+times\s+a\s+year|now\s+and\s+(?:then|again)|occasionally|from\s+time\s+to\s+time|keppra|epilep\w*)\b"
);
pattern!(
    FIT_CONCERN,
    r"\b(?:more than (?:5|five) minutes|over (?:5|five) minutes|first (?:fit|seizure)|seizure after seizure|another fit (?:right away|straight away)|still (?:shaking|fitting|unresponsive))\b"
);
pattern!(
    LAUGH_IDIOM,
    r"\b(?:collaps\w*|went down|doubled over|fell about|fell over)\s+(?:laughing|giggling|in (?:a )?(?:fit|stitches|hysterics)|with laugh\w*|helpless)\b"
);
pattern!(
    PAST,
    concat!(
        r"\b(?:(?:weeks?|months?|years?|wks?|mos?|yrs?) ago|last (?:year|month|summer|winter|autumn|spring|week)|in (?:19|20)\d\d|",
        r"years back|months back|when (?:i|he|she|they|it|we) (?:was|were) (?:a |an )?",
        r"(?:bab(?:y|ies)|toddler|tot|child|kid|teenager|born|little|small|young|tiny|",
        r"at school|\d+|one|two|three|four|five|six|seven|eight|nine|ten)|",
        r"as a (?:baby|child|kid|toddler|teenager)|used to)\b"
    )
);
pattern!(
    NOW_WORDS,
    r"\b(?:now|right now|still|since|today|tonight|this (?:morning|evening|afternoon)|at the moment|any ?more)\b"
);
pattern!(
    ALL_BETTER,
    concat!(
        r"\b(?:now|since|and)\b[^.?!]{0,16}\b(?:he|she|they|it|i|we)\s*(?:is|are|was|were|'s)?\s*",
        r"(?:fine|ok|okay|better|recovered|alright|back to normal|himself again|herself again)\b",
        r"|\b(?:now|since)\s*(?:hes|shes|theyre|its|im)\s*(?:fine|ok|okay|better|recovered|alright)\b"
    )
);
pattern!(
    NORMAL_ASK,
    r"\b(?:is that|is it|is this|should i|do i need to)\b[^.?!]{0,40}\b(?:normal|ok|okay|fine|worry|worried|worrying|serious|dangerous)\b"
);
pattern!(
    OBJECTISH,
    concat!(
        r"^\s+(?:the|a|an|my|your|his|her|their|our|its|it|me|him|them|us|you|new|some|these|",
        r"those|this|that|another|two|three|four|five|one|carpet|door|floor|tiles|kitchen|bathroom|",
        r"wardrobe|blind|window|shelves|shelf|units|sink|sofa)\b"
    )
);
pattern!(
    SETTLING_PLACE,
    r"^(?:at|in|into|with)\s+(?:the\s+|his\s+|her\s+|their\s+|our\s+|a\s+)?(?:new\s+)?(?:nursery|preschool|playgroup|school|class|clubs?|teams?|groups?|other\s+kids|the\s+others)\b"
);
pattern!(
    SECOND_CLAUSE,
    r"^(?:but|and|then|so|because|yet|though|when)\b"
);
pattern!(
    CONFUSED,
    r"\b(?:suddenly|very|extremely|really|getting more|more)\b"
);
pattern!(CONFUSED_AFTER, r"^\s*(?:and\b|or\b|,)");
pattern!(
    ACUTE_CONFUSION,
    r"\b(?:suddenly confused|sudden confusion|confused all of a sudden|confusion came on suddenly)\b"
);
pattern!(
    STUCK,
    concat!(
        r"\bstuck\b[^.?!]{0,30}\bthroat\b|\bthroat\b[^.?!]{0,30}\bstuck\b",
        r"|\b(?:can'?t|unable to|couldn'?t) (?:talk|speak|breathe)\b[^.?!]{0,25}\bthroat\b",
        r"|\bthroat\b[^.?!]{0,25}\b(?:can'?t|couldn'?t) (?:talk|speak|breathe)\b",
        r"|\bcan'?t get it out\b|\b(?:turning|going|gone) purple\b"
    )
);
pattern!(
    INGEST_HYPOTHETICAL,
    r"\b(?:what if|could|might|would happen if)\b[^.?!]{0,35}\b(?:swallow|eat|drink|lick|chew)\b"
);
pattern!(
    INGEST_NEGATED,
    r"\b(?:didn'?t|hasn'?t|hadn'?t|not|never)\s+(?:actually\s+)?(?:swallow\w*|eat\w*|drink\w*|lick\w*|chew\w*|ate|drank|eaten)\b"
);
pattern!(
    HEAVY_BLEEDING,
    r"\b(?:bleeding heavily|heavy bleeding|spurting blood|blood spurting|bleeding (?:and )?won'?t stop|won'?t stop bleeding)\b"
);
pattern!(
    SERIOUS_OBJECT,
    r"\b(?:battery|batteries|magnet|magnets|drawing pin|sharp object|screw)\b"
);
pattern!(
    HIGH_RISK_MEDICINE,
    r"\b(?:methadone|insulin|codeine|morphine|opioids?|fentanyl)\b"
);

const PARTS: &str = r"lips?|mouth|tongue|face|cheeks?|skin|fingers?|fingertips?|hands?|nails?|feet|foot|toes?|arms?|legs?|body";
const COLOUR_WORDS: &str = r"a\s+bit|a\s+little|very|really|quite|all|pale|and";
const STATE_WORDS: &str = r"has\s+gone|have\s+gone|had\s+gone|has\s+turned|have\s+turned|had\s+turned|gone|goes|went|is|are|was|were|looks?|looked|turned";
const CHANGING: &str =
    r"getting|looking|turning|going|is\s+going|are\s+going|was\s+going|were\s+going";
static COLOUR_BODY: LazyLock<Regex> = LazyLock::new(|| {
    let people = data::FAMILY.join("|");
    let qualifiers = format!(r"(?:\s+(?:{COLOUR_WORDS}|{STATE_WORDS}))*");
    let body = format!(r"(?:\s+(?:{COLOUR_WORDS}|{STATE_WORDS}|{CHANGING}|grey|gray))*");
    let subject = format!(r"\b(?:{people}|he|she|they|hes|shes|he's|she's|everyone)\b");
    Regex::new(&format!(
        r"(?x)\b(?:{PARTS})\b{body}\s+(?:blue|grey|gray)\b|{subject}{body}\s+blue\b|{subject}{qualifiers}\s+(?:grey|gray)\b"
    ))
    .expect("acute changes of body colour")
});

const DETERMINERS: &[&str] = &["a", "an", "the", "my", "his", "her", "their", "our"];
const SETTLING: &[&str] = &[
    "well",
    "nicely",
    "easily",
    "fine",
    "perfectly",
    "beautifully",
    "smoothly",
    "brilliantly",
];
const MANNER: &[&str] = &[
    "well",
    "nicely",
    "easily",
    "fine",
    "ok",
    "okay",
    "perfectly",
    "together",
    "up",
    "out",
    "round",
    "around",
];
const CALENDAR: &[&str] = &["today", "yesterday", "lately", "recently", "nowadays"];
const DURATION: &[&str] = &[
    "minute", "minutes", "min", "hour", "hours", "day", "days", "week", "weeks", "month", "months",
    "year", "years",
];
const ASKING: &[&str] = &[
    "about", "what", "why", "how", "should i", "can i", "is that", "do i",
];

fn words(line: &str) -> impl Iterator<Item = &str> {
    WORD.find_iter(line).map(|item| item.as_str())
}

fn in_list(word: &str, list: &str) -> bool {
    list.split_whitespace().any(|value| value == word)
}

fn has(line: &str, choices: &[&str]) -> bool {
    choices.iter().any(|choice| line.contains(choice))
}

fn word_or_phrase(line: &str, choices: &[&str]) -> bool {
    choices.iter().any(|phrase| {
        line.match_indices(phrase)
            .any(|(at, _)| boundary(line, at, phrase.len()))
    })
}

fn boundary(line: &str, at: usize, length: usize) -> bool {
    !line[..at].ends_with(char::is_alphanumeric)
        && !line[at + length..].starts_with(char::is_alphanumeric)
}

fn fed_and_unresponsive(before: &str, after: &str) -> bool {
    before.split_whitespace().rev().take(3).any(|word| {
        in_list(
            word.trim_matches(|ch: char| !ch.is_ascii_alphabetic()),
            data::SMALL_PERSON,
        )
    }) && words(after).take(4).any(|word| in_list(word, data::A_FEED))
}

fn not_a_body(after: &str, before: &str) -> bool {
    let mut next = words(after);
    let Some(first) = next.next() else {
        return false;
    };
    let second = next.next().unwrap_or("");
    let third = next.next().unwrap_or("");
    match first {
        "to" => {
            let noun = if ["the", "any", "some", "his", "her", "their"].contains(&second) {
                third
            } else {
                second
            };
            !(data::TO_PAIN.contains(&noun) || fed_and_unresponsive(before, after))
        }
        "by" | "about" | "from" | "into" | "of" => true,
        _ if SETTLING.contains(&first) => {
            let tail = after
                .trim_start()
                .trim_start_matches(first)
                .trim_matches(|ch: char| ch.is_whitespace() || ".,!?;:".contains(ch));
            tail.is_empty()
                || CALENDAR.contains(&tail)
                || SECOND_CLAUSE.is_match(tail)
                || SETTLING_PLACE.is_match(tail)
        }
        "together" => {
            DETERMINERS.contains(&second) || ["some", "these", "those", "this"].contains(&second)
        }
        "when" | "whenever" => words(after)
            .take(6)
            .any(|word| in_list(word, data::ORDINARY_STATE)),
        "in" => {
            matches!(second, "with" | "at")
                || (MANNER.contains(&second) && !in_list(second, data::PLACE))
                || (DETERMINERS.contains(&second) && !in_list(third, data::PLACE))
        }
        "with" => {
            if matches!(second, "a" | "an" | "the") {
                !in_list(third, data::SYMPTOM)
            } else {
                in_list(second, data::OBJECT_WORD)
            }
        }
        "on" => {
            let noun = if DETERMINERS.contains(&second) || matches!(second, "this" | "that") {
                third
            } else {
                second
            };
            in_list(noun, data::ONLINE)
        }
        "for" => {
            DURATION.contains(&second)
                || (["a", "an", "one", "two", "half"].contains(&second)
                    && DURATION.contains(&third))
        }
        _ => false,
    }
}

fn actual_cue(line: &str, cue: &str, start: usize) -> bool {
    let (before, after) = (&line[..start], &line[start + cue.len()..]);
    if !boundary(line, start, cue.len()) {
        return false;
    }
    if cue == "cardiac" && !after.trim_start().starts_with("arrest") {
        return false;
    }
    if cue == "unconscious" && after.trim_start().starts_with("bias") {
        return false;
    }
    if cue == "choking" && after.trim_start().starts_with("up") {
        return false;
    }
    if cue == "limp"
        && words(before).last().is_some_and(|word| {
            DETERMINERS.contains(&word) || matches!(word, "this" | "that" | "your")
        })
    {
        return false;
    }
    let wake = [
        "wont wake",
        "won't wake",
        "cant wake",
        "can't wake",
        "not waking",
    ]
    .contains(&cue);
    if wake {
        let mut after = words(after).peekable();
        if after.peek() == Some(&"up") {
            after.next();
        }
        if let Some(rest) = after.peek()
            && (["for", "to", "at", "on", "before", "until", "by"].contains(rest)
                || !after.take(5).any(|word| {
                    data::FAMILY.contains(&word)
                        || [
                            "him", "her", "them", "it", "my", "your", "his", "our", "their", "the",
                            "this", "that",
                        ]
                        .contains(&word)
                }))
        {
            return false;
        }
    }
    if cue == "burning up"
        && [
            "at the idea",
            "at the thought",
            "at the prospect",
            "at the mention",
        ]
        .iter()
        .any(|phrase| after.trim_start().starts_with(phrase))
    {
        return false;
    }
    if cue == "confused" && !(CONFUSED.is_match(line) || CONFUSED_AFTER.is_match(after)) {
        return false;
    }
    if matches!(cue, "fitting" | "burning up") && OBJECTISH.is_match(after) {
        return false;
    }
    let ambiguous = wake
        || data::AMBIGUOUS.contains(&cue)
        || [
            "isnt responding",
            "isn't responding",
            "isnt responsive",
            "isn't responsive",
            "not responsive",
        ]
        .contains(&cue);
    !ambiguous || !not_a_body(after, before)
}

fn first(line: &str, cues: &'static [&'static str]) -> Option<&'static str> {
    cues.iter().copied().find(|cue| {
        line.match_indices(cue)
            .any(|(at, _)| actual_cue(line, cue, at))
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Signal {
    Emergency,
    Seizure,
    Airway,
    Chemical,
    ForeignObject,
    Swallowed,
    Bleeding,
    Today,
}

fn swallowed(line: &str) -> Option<Signal> {
    if INGEST_HYPOTHETICAL.is_match(line) || INGEST_NEGATED.is_match(line) {
        return None;
    }
    for verb in WORD
        .find_iter(line)
        .filter(|verb| in_list(verb.as_str(), data::INGEST))
    {
        let tail = &line[verb.end()..];
        let tail = tail.split(['.', '?', '!', ';', ',']).next().unwrap_or("");
        let end = WORD
            .find_iter(tail)
            .enumerate()
            .find(|(index, word)| {
                *index == 8
                    || ["while", "when", "after", "before", "because", "then", "and"]
                        .contains(&word.as_str())
            })
            .map_or(tail.len(), |(_, word)| word.start());
        let tail = &tail[..end];
        let chemicals = tail.replace("paint ball", " ").replace("paintball", " ");
        if word_or_phrase(&chemicals, data::CHEMICAL) || HIGH_RISK_MEDICINE.is_match(tail) {
            return Some(Signal::Chemical);
        }
        if SERIOUS_OBJECT.is_match(tail) {
            return Some(Signal::ForeignObject);
        }
        if word_or_phrase(tail, data::SMALL_OBJECT)
            || words(tail).any(|word| in_list(word, data::MEDICINE))
        {
            return Some(Signal::Swallowed);
        }
    }
    None
}

fn classify(clean: &str) -> Option<Signal> {
    let first_subject = words(clean).find_map(|word| {
        if data::FAMILY.contains(&word.strip_suffix("'s").unwrap_or(word)) {
            Some(true)
        } else if in_list(word, data::PETS) {
            Some(false)
        } else {
            None
        }
    });
    if first_subject == Some(false)
        || (first_subject.is_none()
            && !words(clean)
                .any(|word| matches!(word, "he" | "she" | "hes" | "shes" | "he's" | "she's")))
        || super::the_detail_the_thread_adds::heavy::only_a_dream(clean)
        || (PRO_ANSWERING.is_match(clean) && !BODY_IN_ROOM.is_match(clean))
    {
        return None;
    }
    let colour = has(clean, &["blue", "grey", "gray"]) && COLOUR_BODY.is_match(clean);
    let acute_confusion = clean.contains("confus") && ACUTE_CONFUSION.is_match(clean);
    if first_subject.is_none()
        && !(colour || acute_confusion || first(clean, data::PRONOUN_SAFE).is_some())
    {
        return None;
    }
    let current_nose = NOSE_BREATH.replace_all(clean, " ");
    let current = current_nose.as_ref();
    let nasal_only = current != clean;
    let past = PAST.is_match(clean);
    if past && (!NOW_WORDS.is_match(clean) || ALL_BETTER.is_match(clean)) {
        return None;
    }
    if word_or_phrase(clean, data::DEATH)
        && !has(
            current,
            &[
                "not breathing now",
                "stopped breathing just now",
                "is dying now",
            ],
        )
    {
        return None;
    }
    let another_sign = has(current, data::FIT_STILL_RINGS) || FIT_CONCERN.is_match(current);
    let routine_fit = FIT_OF.is_match(clean) || FIT_ROUTINE.is_match(clean);
    let without_idiom = LAUGH_IDIOM.replace_all(current, " ");
    let current = without_idiom.as_ref();
    let near = word_or_phrase(clean, &["almost", "nearly", "like to have"]);
    let now = first(current, data::NOW);
    if HEAVY_BLEEDING.is_match(current) {
        return Some(Signal::Bleeding);
    }
    if colour || acute_confusion {
        return Some(Signal::Emergency);
    }
    if let Some(cue) = now {
        let fit = has(cue, &["fit", "seizure", "convuls"]);
        let cleared = data::SOFT_STATES.contains(&cue)
            && NORMAL_ASK.is_match(clean)
            && !word_or_phrase(clean, &["suddenly", "right now"]);
        if !cleared && !(routine_fit && fit && !another_sign) {
            if near && !another_sign {
                return Some(Signal::Today);
            }
            if fit {
                return Some(Signal::Seizure);
            }
            return Some(if cue == "choking" {
                Signal::Airway
            } else {
                Signal::Emergency
            });
        }
    }
    if STUCK.is_match(current) {
        return Some(Signal::Airway);
    }
    if let Some(signal) = swallowed(clean) {
        return Some(signal);
    }
    if routine_fit && !another_sign {
        return None;
    }
    let report = !clean.contains('?') && !word_or_phrase(clean, ASKING);
    if !past && report && first(current, data::TODAY).is_some() {
        return Some(Signal::Today);
    }
    if nasal_only && super::safety::breathing_or_colour_emergency(current) {
        return Some(Signal::Emergency);
    }
    None
}

fn clean(reading: &Reading<'_>) -> String {
    reading
        .raw
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
        .replace('’', "'")
}

pub(super) fn overrides_answer(reading: &Reading<'_>, health_answer: bool) -> bool {
    let line = clean(reading);
    let signal = classify(&line);
    if signal == Some(Signal::Swallowed) {
        return health_answer;
    }
    // A named hazardous exposure must win even if the matcher chose device care for "tablet".
    signal.is_some()
        && matches!(
            swallowed(&line),
            Some(Signal::Chemical | Signal::ForeignObject)
        )
}

// https://www.nhs.uk/nhs-services/urgent-and-emergency-care-services/when-to-call-999/
// https://www.nhs.uk/symptoms/what-to-do-if-someone-has-a-seizure-fit/
// https://www.nhs.uk/conditions/poisoning/
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = clean(reading);
    let signal = classify(&line)?;
    if super::poison::explicit_dose(reading) || super::poison::is_child_dose(reading) {
        return None;
    }
    let replies: &[&str] = match signal {
        Signal::Emergency => &[
            "Call 999 now in the UK, or the local emergency number. Stay with them and tell the call handler if they are unresponsive or breathing abnormally. If they are unresponsive and not breathing normally, start CPR as the handler directs.",
            "Get emergency help now. Call 999 in the UK, or your local emergency number. Check whether they respond and are breathing normally. Stay with them and follow the call handler's instructions.",
        ],
        Signal::Seizure => &[
            "Call 999 now in the UK if this is their first seizure, it has lasted five minutes, it is repeating without recovery, or breathing is difficult. If you are unsure whether it is ongoing, call now. Time the seizure, move nearby hazards, protect their head and follow the call handler. Keep their mouth clear of objects.",
        ],
        Signal::Airway => &[
            "If they cannot cough effectively, speak or breathe, call 999 now in the UK, or the local emergency number. Start age-appropriate choking first aid as the call handler directs. If they can cough forcefully, encourage coughing. Seek immediate advice if something still feels stuck.",
        ],
        Signal::Chemical => &[
            "Call 999 now in the UK, or the local emergency number, for possible poisoning. Keep the container or packet for the call handler and tell them what happened and when. Stay with the person. Avoid giving food or drink or trying to make them vomit.",
        ],
        Signal::ForeignObject => &[
            "Get emergency assessment now for a swallowed battery, magnet or sharp object. Go to A&E, or call 999 in the UK if they are choking, struggling to breathe or hard to wake. Take any packaging or a matching object with you and follow the clinician's advice.",
        ],
        Signal::Swallowed => &[
            "Call NHS 111 now for advice about what they swallowed if you are unsure whether it is harmful. Keep any packet or another example of the object handy. Call 999 instead if they may have swallowed a harmful substance, are hard to wake, have a seizure or have trouble breathing. Avoid trying to make them vomit.",
        ],
        Signal::Bleeding => &[
            "Call 999 now in the UK for heavy bleeding that will not stop. Keep firm pressure on the wound with a clean pad while you wait. If something is embedded, press around it and follow the call handler's instructions.",
        ],
        Signal::Today => &[
            "Get medical advice today. Call NHS 111 or their GP and describe what happened and when. If they become hard to wake, turn blue or grey, or struggle to breathe, call 999 in the UK.",
            "Call NHS 111 or their GP today and describe the symptoms. If they become unresponsive or are breathing abnormally, call 999 in the UK immediately.",
        ],
    };
    let current: Vec<_> = replies
        .iter()
        .copied()
        .filter(|reply| {
            !memory
                .turns
                .iter()
                .rev()
                .take(5)
                .any(|turn| turn.bot == *reply)
        })
        .collect();
    let choices = if current.is_empty() {
        replies
    } else {
        &current
    };
    Some(compose::seeded_choice(choices, memory, &line).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Bot, Candidate, modules::arbiter};

    #[test]
    fn urgent_controls_distinguish_a_body_from_other_meanings() {
        for text in [
            "my son is fitting the new window",
            "my daughter is fitting in nicely at nursery",
            "my mum is unresponsive to my texts",
            "my carer is not responding",
            "my mum collapsed laughing",
            "my son has a limp",
            "my son is limping",
            "my kid has a wobbly tooth thats gone grey",
            "my husband is going grey",
            "my daughter keeps asking why the sky is blue",
            "my baby can't breathe through his nose",
            "i dreamt my baby stopped breathing",
            "my mum was unconscious years ago",
            "my cat is fitting in with the new baby",
            "he had a fit of coughing",
            "my son has a seizure every few months",
            "my child ate some toast while i was spraying bleach",
        ] {
            assert_eq!(classify(text), None, "{text}");
        }

        for (text, expected) in [
            ("my mum is unresponsive", Signal::Emergency),
            ("my carer is not responding on the floor", Signal::Emergency),
            ("my baby is unresponsive to her milk", Signal::Emergency),
            ("my son wont wake up", Signal::Emergency),
            ("he's gone grey", Signal::Emergency),
            (
                "my baby can't breathe through his nose and his lips are blue",
                Signal::Emergency,
            ),
            ("my wife is fitting", Signal::Seizure),
            ("my son is choking and cannot speak", Signal::Airway),
            ("my dad is suddenly confused", Signal::Emergency),
            ("my baby has a temperature of 40", Signal::Today),
            (
                "my daughter swallowed a button battery 5 minutes ago",
                Signal::ForeignObject,
            ),
            ("my daughter ate a washing tablet", Signal::Chemical),
            ("my son swallowed a pill", Signal::Swallowed),
        ] {
            assert_eq!(classify(text), Some(expected), "{text}");
        }
    }

    #[test]
    fn urgent_is_a_fallback_except_for_health_ingestion() {
        let reading = Reading::new("My daughter ate a washing tablet");
        let memory = Memory::default();
        let reply = respond(&reading, &memory).unwrap();
        assert!(reply.contains("Call 999"));
        let proposals = [
            Candidate {
                module: "answers",
                priority: 0.0,
                reply: "Ordinary advice".to_owned(),
            },
            Candidate {
                module: "urgent",
                priority: -900.0,
                reply,
            },
        ];
        let selected = arbiter::choose(
            &reading,
            &memory,
            &proposals,
            false,
            Some(("health", "test")),
        );
        assert_eq!(selected.unwrap().module, "urgent");

        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("My mum is unresponsive");
        assert!(turn.reply.contains("999"), "{turn:?}");
        assert!(
            turn.candidates
                .iter()
                .any(|proposal| proposal.module == "urgent" && proposal.priority == -900.0)
        );

        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("My daughter ate a washing tablet");
        assert!(turn.reply.contains("999"), "{turn:?}");
        assert_eq!(turn.module, Some("urgent"));
        let quantified = Reading::new("My child swallowed a whole packet of paracetamol");
        assert!(respond(&quantified, &Memory::default()).is_none());
        let lower_risk = Reading::new("My son swallowed a coin");
        assert!(!overrides_answer(&lower_risk, false));
        assert!(overrides_answer(&lower_risk, true));
    }
}
