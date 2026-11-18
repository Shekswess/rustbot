//! PyBot bot/modules/talkto.py and bot/compose/talkto.py at 7d63c7f.

use super::{greeting, urgent};
use crate::bot::{Memory, Reading, compose, knowledge::lookup};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! lead {
    () => {
        r"^(?:(?:ok|okay|so|right|hi|hey|well)\s+)?"
    };
}
macro_rules! verb {
    () => {
        r"(?:want|need|needed|would\s+like|'?d\s+like|have\s+to|got\s+to|gotta|have|got)"
    };
}
macro_rules! when {
    () => {r"(?:again|now|today|tonight|tomorrow|later|sometime|soon|this\s+(?:evening|afternoon|morning|weekend)|next\s+week|when\s+(?:you\s+)?(?:have|get|'ve\s+got)\s+a\s+(?:minute|mo|moment))"};
}
macro_rules! rule {
    ($name:ident, $pattern:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}

static ASK: LazyLock<[Regex; 6]> = LazyLock::new(|| {
    [
        concat!(lead!(), r"(?:can|could|would|will)\s+you\s+(?:(?:please|just)\s+)?(?:talk|chat|speak)\s+(?:to|with)\s+me\s+(?:about|through)\s+(.+)$"),
        concat!(lead!(), r"(?:can|could|may)\s+(?:i|we)\s+(?:please\s+)?(?:talk|chat|speak)(?:\s+(?:to|with)\s+you)?\s+(?:about|through)\s+(.+)$"),
        concat!(lead!(), r"(?:i|we)\s+(?:(?:just|really|honestly)\s+)?", verb!(), r"\s+(?:to\s+)?(?:talk|chat|speak)(?:\s+(?:to|with)\s+you)?\s+(?:about|through)\s+(.+)$"),
        concat!(lead!(), r"(?:(?:please|just)\s+)?(?:talk|chat|speak)\s+to\s+me\s+(?:about|through)\s+(.+)$"),
        concat!(lead!(), r"(?:can|could|would|will)\s+you\s+(?:(?:please|just)\s+)?help\s+me\s+(?:think|talk|work)\s+(?:about|through|out)\s+(.+)$"),
        concat!(lead!(), r"(?:i|we)\s+(?:(?:just|really)\s+)?", verb!(), r"\s+(?:to\s+)?(?:think|work)\s+(?:about|through|out)\s+(.+)$"),
    ]
    .map(|pattern| Regex::new(pattern).expect("talk-to request"))
});
rule!(
    TIME_BEFORE_SUBJECT,
    concat!(
        r"\b(talk|chat|speak|think|work)\s+(?:to\s+(?:you|me)\s+)?(?:",
        when!(),
        r"\s+)+(about|through|out)\b"
    )
);
rule!(
    BARE_INVITATION,
    concat!(
        lead!(),
        r"(?:(?:can|could|may|shall|should)\s+(?:i|we)\s+(?:(?:please|just)\s+)?|(?:i|we)\s+",
        verb!(),
        r"\s+(?:to\s+)?|(?:(?:please|just)\s+)?)(?:talk|chat|speak)(?:\s+(?:to|with)\s+(?:you|me))?(?:\s+",
        when!(),
        r")+\s*[?.!]*$"
    )
);
rule!(
    INVITATION_LEAD,
    concat!(
        lead!(),
        r"(?:(?:can|could|may|shall|should)\s+(?:i|we)\s+(?:(?:please|just)\s+)?|(?:i|we)\s+",
        verb!(),
        r"\s+(?:to\s+)?)(?:talk|chat|speak)"
    )
);
rule!(
    TAIL,
    r"\s+(?:with|to)\s+(?:you|me)\s*$|\s+(?:again|please|now|today|tonight|later|tomorrow|sometime|soon|this\s+(?:evening|afternoon|morning|weekend)|next\s+week)\s*$"
);
rule!(HEAD, r"^(?:the|that|this)\s+thing\s+(?:about|with)\s+");
rule!(
    SOMETHING_ELSE,
    r"^(?:something|anything|somebody|anybody|a\s+different\s+thing|something\s+different|another\s+thing|anything\s+else\s+at\s+all)\s+else\s*[,;:\-–—]?\s*"
);
rule!(
    CLAUSE,
    r"^(?:how|what|why|whether|if|when|where|which|who|whoever|whatever)\b"
);
rule!(
    SUBJECT_END,
    r"\s+(?:its|it's|it is|it was|and|but|because|cos|cuz|coz|so|which|that|who|i|ive|i've|we|were|we're|he|she|they|my|our)\b.*$"
);
rule!(
    CLAUSE_END,
    r"\s+(?:its|it's|it is|it was|and|but|because|cos|cuz|coz|which|that)\b.*$"
);
rule!(
    HEDGE,
    r"\s*,?\s*(?:maybe|perhaps|i\s+think|i\s+reckon|if\s+that\s+is\s+ok)\s*$"
);
rule!(
    CRISIS,
    r"(?i)suicid|kill(?:ing)?\s+myself|self[\s-]?harm|hurt(?:ing)?\s+myself|end(?:ing)?\s+my\s+life|end(?:ing)?\s+it\s+all|take\s+my\s+own\s+life|want(?:ing|ed)?\s+to\s+die|(?:not\s+want(?:ing)?|do(?:n'?t|\s+not)\s+want(?:ing)?)\s+to\s+(?:be\s+here|live|go\s+on|carry\s+on)|better\s+off\s+dead|no\s+reason\s+to\s+live"
);

const INVITED: &[&str] = &[
    "Yes - I'm here whenever you are. What's on your mind?",
    "Any time. What is it you want to talk about?",
    "Of course. What's going on, and where do you want to start?",
];
const POOL: &[&str] = &[
    "Of course. Tell me about {thing} - what's the part that's bothering you most?",
    "{Thing} it is. What's on your mind about it?",
    "Happy to. Where do you want to start with {thing}?",
    "Right, {thing} - go on. What's the bit you want to think out loud about?",
];
const WEIGHING: &[&str] = &[
    "Yes. What are the options you're weighing, as they stand now?",
    "Let's lay them out. What's on each side of it?",
    "Of course. What would you lose with each one?",
];
const GENERIC_OPENER: &str = "conversation-openers";

fn as_typed(line: &str) -> String {
    let mut last = "";
    let mut typed = Vec::new();
    for word in line.split_whitespace() {
        if last == word && word.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        last = word;
        typed.push(match word {
            "u" | "ya" => "you",
            "ur" | "yr" => "your",
            "wana" | "wanna" | "wanta" => "want",
            _ => word,
        });
    }
    typed.join(" ")
}

fn subject(line: &str) -> Option<String> {
    let found = ASK.iter().find_map(|pattern| pattern.captures(line))?;
    let name = found
        .get(1)?
        .as_str()
        .trim_end_matches(['.', '!', '?', ',']);
    let name = TAIL.replace(name, "");
    let name = HEAD.replace(name.trim(), "");
    let name = SOMETHING_ELSE.replace(name.trim(), "");
    let clause = CLAUSE.is_match(name.trim());
    let name = if clause { &CLAUSE_END } else { &SUBJECT_END }.replace(name.trim(), "");
    let name = HEDGE.replace(name.trim(), "");
    let name = name.trim();
    if matches!(
        name,
        "it" | "this"
            | "that"
            | "them"
            | "something"
            | "somethin"
            | "anything"
            | "everything"
            | "stuff"
            | "a thing"
            | "one thing"
            | "it all"
            | "something else"
    ) {
        return None;
    }
    let mut words: Vec<_> = name.split_whitespace().collect();
    if words.is_empty() || words.len() > if clause { 9 } else { 5 } {
        return None;
    }
    if matches!(words[0], "my" | "our") {
        words[0] = "your";
    }
    Some(words.join(" "))
}

enum Request {
    Named(String),
    Scheduled,
}

fn request(typed: &str, unscheduled: &str, asked: bool) -> Option<Request> {
    subject(unscheduled).map(Request::Named).or_else(|| {
        (BARE_INVITATION.is_match(typed) && (INVITATION_LEAD.is_match(typed) || asked))
            .then_some(Request::Scheduled)
    })
}

impl Request {
    fn reply(self, reading: &Reading<'_>, memory: &Memory) -> String {
        let Self::Named(thing) = self else {
            return compose::pick(INVITED, memory, reading.raw).into();
        };
        if thing.split_whitespace().any(|word| {
            matches!(
                word,
                "options"
                    | "option"
                    | "choice"
                    | "choices"
                    | "decision"
                    | "decisions"
                    | "pros"
                    | "cons"
            )
        }) {
            return compose::pick(WEIGHING, memory, reading.raw).into();
        }

        let clauses = [POOL[0], POOL[3]];
        let pool = if CLAUSE.is_match(&thing) {
            &clauses
        } else {
            POOL
        };
        let mut letters = thing.chars();
        let upper: String = letters
            .next()
            .into_iter()
            .flat_map(char::to_uppercase)
            .chain(letters)
            .collect();
        let options: Vec<_> = pool
            .iter()
            .map(|pattern| {
                pattern
                    .replace("{thing}", &thing)
                    .replace("{Thing}", &upper)
            })
            .collect();
        let lines: Vec<_> = options.iter().map(String::as_str).collect();
        compose::pick(&lines, memory, reading.raw).into()
    }
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let typed = as_typed(&reading.line.clean);
    let unscheduled = TIME_BEFORE_SUBJECT.replace_all(&typed, "$1 $2");
    let asked = reading.raw.trim_end().ends_with('?');
    let request = request(&typed, &unscheduled, asked)?;
    if CRISIS.is_match(reading.raw) || urgent::respond(reading, memory).is_some() {
        return None;
    }
    let entry = lookup::matched_id(reading);
    if entry.is_some_and(|entry| entry != GENERIC_OPENER)
        || (entry.is_none()
            && unscheduled != typed
            && lookup::matched_id(&Reading::new(&unscheduled))
                .is_some_and(|entry| entry != GENERIC_OPENER))
        || (entry != Some(GENERIC_OPENER)
            && greeting::respond(reading, memory)
                .is_some_and(|reply| compose::trouble::advisor::names_a_route(&reply)))
    {
        return None;
    }
    Some(request.reply(reading, memory))
}

pub(super) fn specialist_route(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let typed = as_typed(&reading.line.clean);
    let named = TIME_BEFORE_SUBJECT.replace_all(&typed, "$1 $2");
    subject(&named)?;
    compose::trouble::advisor::conversation_referral(reading, memory)
}

pub(super) fn yields_small_talk(card: Option<(&str, &str)>) -> bool {
    card.is_some_and(|(_, id)| id == GENERIC_OPENER)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn asks_for_the_named_subject_or_a_time() {
        let memory = Memory::default();
        let text = Reading::new("I wana talk about my job");
        let answer = respond(&text, &memory).unwrap();
        assert!(answer.to_lowercase().contains("your job"), "{answer}");
        for line in ["Can we talk tomorrow?", "Can I talk to you later?"] {
            assert!(INVITED.contains(&respond(&Reading::new(line), &memory).unwrap().as_str()));
        }
        for line in [
            "Talk later",
            "He won't talk to me about it",
            "I want to talk about that",
        ] {
            assert!(respond(&Reading::new(line), &memory).is_none(), "{line}");
        }
    }

    #[test]
    fn urgent_subjects_and_specific_cards_keep_their_turn() {
        let mut bot = Bot::new(Some("0")).unwrap();
        for line in [
            "Can I talk to you about my suicidal thoughts?",
            "Can I talk to you about my son swallowing a battery?",
            "Can you help me think about my tax?",
            "Can we talk tomorrow about my mum?",
        ] {
            let answer = bot.turn(line);
            assert_ne!(answer.module, Some("talkto"), "{line}: {answer:?}");
            bot.reset().unwrap();
        }
        let answer = bot.turn("Can you help me think about my options?");
        assert_eq!(answer.module, Some("talkto"), "{answer:?}");
        assert!(WEIGHING.contains(&answer.reply.as_str()), "{answer:?}");
    }
}
