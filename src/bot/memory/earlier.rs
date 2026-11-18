//! Source-line recall from PyBot's earlier.py and store_replies.py.
use super::{again, measurement};
use crate::bot::{
    Memory, Reading, compose,
    understanding::normalize::{stem, tokens},
};
use fancy_regex::Regex as ContextRegex;
use regex::{Captures, Regex};
use std::{collections::BTreeSet, sync::LazyLock};

macro_rules! patterns {
    ($($name:ident = $source:expr;)*) => {
        $(static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($source).unwrap());)*
    };
}

patterns! {
    QUESTION = r"(?i)^(?:(?:and|so|but|or|oh|also|then)\s+)?(?:how|what|when|where|which|who|whose|why|do|does|did|is|are|was|were|am|can|could|should|would|will|shall|have|has|had|any)\b";
    RECALL = r"(?i)\b(?:did i (?:say|tell|mention)|what did i say|what was it i said|remind me|what did we say|how did i say|again)\b";
    CLAIM = r"(?i)\b(?:you (?:haven'?t|have not) (?:told|said|mentioned|given)|you'?ve not (?:said|told|mentioned)|you never (?:said|told|mentioned)|i don'?t have (?:your|the|their|his|her)\b[^.]*\bfrom you|i (?:do not|don'?t) know (?:what|how|when|where|who)\b[^.]*\bsaid)";
    THEIR_OWN_WORDS = r"(?i)\b(?:(?:did|do|does|have|had|was|were) i (?:say|tell|mention|said|go|use|have|had|get|got|buy|bought|pay|paid|book|pick|choose)|remind me|what was my|what did i|where did (?:i|we) say|again)\b";
    NOT_A_RECALL = r"(?i)\b(?:should|could|would|will|shall|might|can|may|roughly|maybe|probably|need|needs|got to|have to|havent got|is it worth|better off|do you think|any (?:idea|chance)|recommend)\b";
    QUOTE_HEAD = r"(?i)^(?:you said|that one you did say|you told me)\b";
    STRAIGHT_QUOTE = r#""([^"]{2,300})""#;
    FIGURE_RATE = r"(?i)\b(?:\d[\d,.]*|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\s*(?:times?\s*)?(?:a|an|per|/|each|every)\s*(?:month|week|year|day|hour)\b|£?\s*\d[\d,.]*\s*(?:pounds?|quid|grand|k)?\s*(?:a|an|per|/|each|every)\s*(?:month|week|year|day|hour)\b";
    OWN_WORDS_ASK = r"\b(?:did|have) i (?:ever )?(?:say|tell you|mention|said|told you|mentioned)\b";
    SAID_BACK = r"\b(?:(?:where|what|when|why|who|how\s+often|how\s+many\s+times)\s+)?did\s+(?:i|we)\s+(?:ever\s+)?(?:say|tell\s+you|mention)\b";
    SAID_PROPERTY = r"\bhow\s+(?:old|big|heavy|long|far|tall|late|early)\b|\bwhat\s+colou?r\b";
    OWN_RATE = r"(?i)\b\d+\s*(?:times?|x)\b|\b(?:once|twice|daily|weekly|monthly)\b|\bevery\s+(?:day|week|month|year|morning|night)\b";
    SAID_WORD = r"(?i)[a-z']+";
    CHAT_TAIL = r"(?i)[ ,]+(?:of|in|from|for|to) (?:this|our|the) (?:chat|conversation|talk|session|thread)(?: (?:so far|today|here|with you|we have had))?(?:[ ,]+(?:please|again))?[?.!,]*$";
    FIRST_USER = concat!(
        r"(?i)^(?:(?:and|so|okay|ok|please|before i go)[ ,]+)*",
        r"(?:(?:(?:(?:do|can|could|did|would) you remember |remind me )?",
        r"what (?:did (?:i|we) (?:say|ask|tell you|mention)|i (?:said|asked|told you|mentioned)))",
        r" (?:first|(?:at |in )?the (?:very )?(?:start|beginning)|when we (?:started|began))",
        r"|what (?:was|were|is) (?:my|the) (?:very )?first (?:thing|question|message|line|words?)",
        r"(?: (?:i|we) (?:asked|said))?|my first (?:question|message|line|words?)",
        r"|(?:how|where) did (?:we|(?:this|our|the) (?:chat|conversation|talk)) (?:start|begin|get started)",
        r"|(?:remind me )?where (?:we|this chat|our conversation) (?:all )?(?:started|began)",
        r"|(?:remind me )?how (?:we|this chat|our conversation) (?:started|began)",
        r"|(?:which (?:question|thing) did i (?:ask|say) first|which (?:question|thing) did i (?:start|begin) with)",
        r"|what did (?:i|we) (?:open|start) with)(?:[ ,]+please)?[?.!]*$"
    );
    FIRST_BOT = concat!(
        r"(?i)^(?:(?:and|so|okay|ok|please|before i go)[ ,]+)*",
        r"(?:what (?:was|were|is) the (?:very )?first (?:thing|words?) you (?:said|told me)",
        r"|what did you (?:say|tell me|answer|ask) (?:first|at the (?:very )?(?:start|beginning)|in the beginning|early on)",
        r"|what (?:was|is) your first (?:answer|reply|line|message))(?:[ ,]+please)?[?.!]*$"
    );
    LAST_USER = r"^(?:(?:and|so|okay|ok) )*(?:what did i just (?:say|tell you|mention|ask)|what did i (?:say|tell you|ask) (?:just now|last)|what was (?:my (?:last|latest) (?:line|message|question)|the last thing i (?:said|asked|told you)))[?.!]*$";
    HELLO_ONLY = r"(?i)^(?:hi|hiya|hello|hey|yo|morning|evening|good (?:morning|afternoon|evening))(?:[ ,]+(?:there|all|everyone|everybody|both|you|folks|guys|again|friend|friends|mate|team|y'all))*[.!,]*$";
    RECALL_ABOUT = r"^(?:(?:and|so|okay|ok|also)\s+)?what (?:did|have) i (?:say|tell you|mention|said|told you|mentioned) about (?P<topic>.+?)\s*(?: (?:again|before|earlier|please|so far))*[?.!]*$";
    TASK_ASK = r"^(?:(?:and|so|okay|ok|also)\s+)?what (?:did|have) i (?:say|tell you|mention|said|told you) i (?:had to do|needed to do|was going to do|should do|wanted to do)(?: (?:again|earlier))?[?.!]*$";
    TASK_SAID = r"(?i)^\s*(?:i|we)\s+(?:(?:was|were) going to|(?:have|had|need|needed|want|wanted) to|plan(?:ned)? to|(?:should|must|will))\s+\p{L}+";
    TELLING = r"(?i)^(?:i|we)\s+(?:(?:already|just)\s+)?(?:said|told you|mentioned)\b";
    TRAVEL_NAME = r"(?i)\b(?P<lead>(?:going to|off to|flying to|fly to|travell?ing to|travel to|trip to|holiday in|moving to|heading to|visit(?:ing)?)\s+)(?P<name>(?:(?:new|saint|st|san|santa|los|las|la|le|mount|lake|fort|cape|port|sri)\s+[a-z]{3,}|[a-z]{3,}))\b";
    AGE_ASK = r"\b(?:how old|age|how many years)\b";
    NAME_ASK = r"\bwhat(?:'s|s| is| was)?\s+(?:it |they |he |she )?called\b|\b(?:name|called)\b";
    NAME_SAID = r"(?i)\b(?:called|named|calls|name(?:'s| is| was)?)\s+[a-z]";
    WHEN_ASK = r"\b(?:when|what day|what date|what time|which day|what month)\b";
    WHEN_SAID = r"(?i)\b(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|sept|oct|nov|dec|monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|today|tonight|\d{1,2}(?:st|nd|rd|th)\b|\d{1,2}(?:[:.]\d{2})?\s*(?:am|pm)\b|\d{1,2}[:.]\d{2})\b";
    OFTEN_ASK = r"\b(?:how often|how many times)\b";
    OFTEN_SAID = r"(?i)\b(?:\d+\s+times|once|twice|three times|every\s+(?:day|week|month|morning|night|year)|a\s+(?:day|week|month)|most\s+(?:days|nights|weeks)|alternate)\b";
    SIZE_ASK = r"\b(?:size|how big|how tall|how heavy|how long is|how wide|how deep|how far)\b";
    SIZE_SAID = r"(?i)\b(?:\d+(?:\.\d+)?\s*(?:kg|g|l|ml|kilo|kilos|kilogram|kilograms|gram|grams|lb|lbs|pound|pounds|ounce|ounces|cm|mm|metre|metres|meter|meters|foot|feet|inch|inches|litre|litres|pint|pints)|size\s+\d+|\d+\s*(?:by|x)\s*\d+)";
    MONEY_ASK = r"\b(?:how much|budget|what did i say i (?:paid|spent))\b";
    MONEY_SAID = r"(?i)£\s*\d|\b\d+(?:,\d{3})*\s*(?:pounds?|quid|k\b|grand)|\bbudget\b|\b\d{3,}\b";
    DURATION_ASK = r"\b(?:how long|how many (?:days|weeks|months|years|hours|minutes))\b";
    DURATION_SAID = r"(?i)\b(?:\d+(?:\.\d+)?|an?|one|two|three|four|five|six|seven|eight|nine|ten|twelve|fifteen|twenty|half an)\s*(?:hours?|hrs?|minutes?|mins?|days?|weeks?|months?|years?)\b";
    COUNT_ASK = r"\bhow many\b";
    COUNT_SAID = r"(?i)\b(?:\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\s+(?:(?:of|in)\s+)?(?:[a-z]{2,}s|people|children|men|women|us|them)\b";
    PLACE_ASK = r"\bwhere\b";
    PLACE_SAID = r"(?i)\b(?:in|at|to|from|live|living|moved)\s+[a-z][a-z]{2,}";
}

static AGE_SAID: LazyLock<ContextRegex> = LazyLock::new(|| {
    ContextRegex::new(concat!(
    r"(?i)(?<!year )(?<!size )\b\d{1,3}\b|\b(?:\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|",
    r"sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)\s*(?:years? old|years?|yrs?|yo)\b|\baged?\s+\d"
)).unwrap()
});
const CLAIM_STOP: &str = "the a an and or but it its is was were are am be to of for in on at by with about again said say saying told tell you i me my have has had do did does not what when where which who how many much old size long often times time that this your yours called name names remember remind know think get got there their his her they them we our us one ones haven havent isnt wasnt didnt dont doesnt never mentioned given should would could will shall can any some just so if then than as from out up down he him she";
const VALUE_WORDS: &str = "heavy weigh weighs weight big tall height wide deep far long length size old age aged name called date day month time often times cost costs price paid spend spent much many number count amount budget how what";

const ASKED_LEADS: &[&str] = &[
    "You asked:",
    "Your question was:",
    "What you said was a question:",
];
const STATED_LEADS: &[&str] = &[
    "You told me:",
    "That was you, not me:",
    "From your own words:",
];
const HAND_BACK: &[&str] = &["You said:", "That one you did say:", "You told me:"];
const NO_RECORD: &[&str] = &[
    "You've not said that to me yet, so there's nothing to hand back - tell me and I'll keep it.",
    "I've nothing on that from you: it's yours to tell me, and then it's kept.",
];

fn flattened(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn excerpt(line: &str, limit: usize) -> String {
    let mut text = flattened(line);
    if let Some((at, _)) = text.char_indices().nth(limit) {
        text.truncate(at);
        text.pop();
        return format!("{}…", text.trim_end());
    }
    text
}

fn sentence_is_a_question(line: &str) -> bool {
    line.trim_end().ends_with('?') || QUESTION.is_match(line.trim_start())
}

fn previous_lines(memory: &Memory) -> impl DoubleEndedIterator<Item = &str> {
    memory.turns.iter().filter_map(|turn| {
        let line = turn.user.trim();
        (!line.is_empty() && !matches!(turn.module, "safety" | "urgent")).then_some(line)
    })
}

fn useful_words(line: &str) -> Vec<String> {
    tokens(line)
        .iter()
        .filter_map(|word| {
            let word = word.trim_end_matches("'s");
            if word.len() < 3
                || matches!(
                    word,
                    "the"
                        | "and"
                        | "but"
                        | "was"
                        | "were"
                        | "are"
                        | "you"
                        | "your"
                        | "our"
                        | "their"
                        | "who"
                        | "how"
                        | "what"
                        | "when"
                        | "where"
                        | "which"
                        | "does"
                        | "did"
                        | "had"
                        | "say"
                        | "said"
                        | "tell"
                        | "told"
                        | "mention"
                        | "mentioned"
                        | "remember"
                        | "remind"
                        | "about"
                        | "from"
                        | "often"
                        | "long"
                        | "size"
                        | "old"
                        | "years"
                        | "months"
                        | "many"
                        | "times"
                        | "time"
                        | "name"
                        | "called"
                        | "named"
                        | "age"
                        | "much"
                        | "budget"
                        | "date"
                        | "day"
                        | "month"
                        | "far"
                        | "tall"
                        | "now"
                        | "again"
                        | "then"
                        | "please"
                        | "before"
                        | "earlier"
                )
            {
                return None;
            }
            let word = match word {
                "mum" | "mom" | "mam" => "mother",
                "dad" => "father",
                "pet" | "puppy" => "dog",
                "fly" | "flew" | "flying" => "fly",
                _ => word,
            };
            Some(stem(word).to_owned())
        })
        .collect()
}

fn matching_topic(anchors: &[String], line: &str) -> bool {
    let actual = useful_words(line);
    !anchors.is_empty() && anchors.iter().all(|anchor| actual.contains(anchor))
}

fn own_quote(line: &str, reading: &Reading<'_>, memory: &Memory) -> String {
    let lead = compose::seeded_choice(STATED_LEADS, memory, reading.raw);
    let quoted = excerpt(line, 120);
    // The upstream store writes place-like words from travel frames as names in its replies.
    // Keep its stop list separate from the grammar of the task that was quoted.
    const NOT_A_PLACE: &str = "bed sleep work school town the see get have try start stop ask tell say make take give look feel stay live move eat drink buy sell pay book plan meet call come home there here that this them him her you your our their about back out over used today tonight tomorrow monday tuesday wednesday thursday friday saturday sunday january february march april may june july august september october november december new old saint san santa los las port cape fort great little south north east west upper lower mount lake northern southern eastern western united";
    let quoted = TRAVEL_NAME.replace_all(&quoted, |found: &Captures<'_>| {
        let location = &found["name"];
        let lower = location.to_ascii_lowercase();
        if !location.contains(' ')
            && NOT_A_PLACE
                .split_ascii_whitespace()
                .any(|word| word == lower)
        {
            return found[0].to_owned();
        }
        let proper = location
            .split_whitespace()
            .map(|word| {
                let mut letters = word.chars();
                letters
                    .next()
                    .into_iter()
                    .flat_map(char::to_uppercase)
                    .chain(letters)
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ");
        format!("{}{proper}", &found["lead"])
    });
    format!("{lead} “{quoted}”")
}

fn nearest_quote(line: &str, reading: &Reading<'_>, memory: &Memory) -> String {
    let nearest = excerpt(line, 120);
    let options = [
        format!("You've not said that to me, but the nearest you've given me is: “{nearest}”"),
        format!(
            "Nothing of yours on that yet - the closest is “{nearest}”, and I'll keep the rest when you say it."
        ),
    ];
    compose::seeded_choice(&options.each_ref().map(String::as_str), memory, reading.raw).into()
}

enum Chronology {
    FirstUser,
    FirstBot,
    LastUser,
}

fn chronology(line: &str) -> Option<Chronology> {
    let line = line.trim();
    let ask = CHAT_TAIL
        .find(line)
        .map_or(line, |tail| &line[..tail.start()]);
    if FIRST_USER.is_match(ask) {
        Some(Chronology::FirstUser)
    } else if FIRST_BOT.is_match(ask) {
        Some(Chronology::FirstBot)
    } else if LAST_USER.is_match(ask) {
        Some(Chronology::LastUser)
    } else {
        None
    }
}

pub(crate) fn chronology_ask(line: &str) -> bool {
    chronology(line).is_some()
}

fn rotate(options: &[String], memory: &Memory) -> String {
    again::vary(
        &options.iter().map(String::as_str).collect::<Vec<_>>(),
        memory,
    )
}

fn first_user(memory: &Memory) -> String {
    if memory.turns.first().is_none_or(|turn| turn.user.is_empty()) {
        return again::vary(
            &[
                "You haven't said anything yet, so this is the start.",
                "We're just beginning — you go first.",
            ],
            memory,
        );
    }
    let mut replies = previous_lines(memory).filter(|line| {
        let low = line.to_lowercase();
        // The bare first-slot form is answered by memory_store's later reader, but the
        // narrower upstream history reader does not classify it as a chat-only line.
        let bare_first = low.starts_with("my first ") && FIRST_USER.is_match(&low);
        !again::contentless(line)
            && !HELLO_ONLY.is_match(line)
            && (bare_first || !again::about_the_chat(line))
    });
    let Some(first) = replies.next() else {
        return again::vary(
            &[
                "We've only just started — nothing of yours to look back on yet.",
                "Only hellos so far: your first proper line is still to come.",
                "Nothing of yours to report back yet — you've only said hello.",
            ],
            memory,
        );
    };
    let first = excerpt(first, 90);
    let mut options = vec![
        format!("You started with: \"{first}\""),
        format!("Your first line was: \"{first}\""),
        format!("That's how we opened: \"{first}\""),
    ];
    if replies.next().is_none() {
        options.push(format!(
            "This chat opened with: \"{first}\" — not much else yet."
        ));
    }
    rotate(&options, memory)
}

fn first_bot(memory: &Memory) -> String {
    let first = memory
        .turns
        .iter()
        .find(|turn| !turn.bot.trim().is_empty())
        .and_then(|turn| measurement::sentences(&turn.bot).first().copied());
    let Some(first) = first else {
        return "I haven't said anything yet — you go first.".to_owned();
    };
    let first = if first.chars().count() > 70 {
        let cut = first
            .char_indices()
            .nth(69)
            .expect("more than 70 characters")
            .0;
        let short = &first[..cut];
        let short = short.rsplit_once(' ').map_or(short, |(before, _)| before);
        format!("{}…", short.trim_end_matches([' ', ',', ';', ':', '-']))
    } else {
        first.trim_end_matches([' ', '.', '!', '?']).to_owned()
    };
    let options = [
        format!("I opened with: \"{first}\""),
        format!("My first line was: \"{first}\""),
        format!("I started with: \"{first}\""),
    ];
    rotate(&options, memory)
}

fn last_user(memory: &Memory) -> Option<String> {
    let Some(last) = memory.turns.last() else {
        return Some("That was your first line — there's nothing before it.".to_owned());
    };
    if let Some(line) = previous_lines(memory).rev().take(8).find(|line| {
        !sentence_is_a_question(line) && !again::contentless(line) && !again::about_the_chat(line)
    }) {
        let line = excerpt(line, 90);
        let options = [
            format!("You said: \"{line}\""),
            format!("Your last line was: \"{line}\""),
            format!("That was: \"{line}\""),
        ];
        return Some(rotate(&options, memory));
    }
    let previous = memory
        .turns
        .iter()
        .rev()
        .take(8)
        .find(|turn| !again::contentless(&turn.user) && !again::about_the_chat(&turn.user))?;
    if !again::about_the_chat(&last.user) && previous.bot.split_whitespace().count() <= 14 {
        return None;
    }
    let first = measurement::sentences(&previous.bot).first().copied()?;
    let asked = excerpt(&previous.user, 90);
    let asked = asked.trim_end_matches(['.', '!', '?', ' ']);
    let options = [
        format!("You asked: \"{asked}\" — I said: {first}"),
        format!("That one was your question: \"{asked}\". My answer: {first}"),
        format!("You asked \"{asked}\", and what I said was: {first}"),
    ];
    Some(rotate(&options, memory))
}

fn said_words(line: &str) -> impl Iterator<Item = String> + '_ {
    SAID_WORD.find_iter(line).map(|word| {
        let word = word.as_str().to_ascii_lowercase();
        for suffix in ["ing", "ed", "es", "s"] {
            if word.len() > 4
                && let Some(root) = word.strip_suffix(suffix)
            {
                return root.into();
            }
        }
        word
    })
}

fn said_back(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    const STOP: &str = "about the a an and or but that this it its was were is are be been did do does say said tell told mention mentioned you your me my i we us our again";
    const GENERIC: &str = "go goes going went do does doing get gets got have has had make makes made take takes took put see saw look start stop carry";
    let found = SAID_BACK.find(&reading.clean)?;
    let ask = said_words(&reading.clean).collect::<Vec<_>>();
    let mut lines = Vec::new();
    for line in previous_lines(memory).rev() {
        if !lines
            .iter()
            .copied()
            .any(|seen: &str| seen.eq_ignore_ascii_case(line))
            && !said_words(line).eq(ask.iter().cloned())
        {
            lines.push(line);
            if lines.len() == 8 {
                break;
            }
        }
    }
    let Some(nearest) = lines.first() else {
        return Some(compose::seeded_choice(NO_RECORD, memory, reading.raw).into());
    };
    if OFTEN_ASK.is_match(&reading.clean)
        && let Some(line) = lines.iter().find(|line| OWN_RATE.is_match(line))
    {
        return Some(own_quote(line, reading, memory));
    }
    let asked = said_words(&reading.clean[found.end()..])
        .filter(|word| {
            !STOP
                .split_whitespace()
                .chain(GENERIC.split_whitespace())
                .any(|stop| stop == word)
        })
        .collect::<Vec<_>>();
    let mut best = *nearest;
    let mut best_score = 0;
    for &line in &lines {
        let words = said_words(line).collect::<Vec<_>>();
        let score = asked.iter().filter(|asked| words.contains(asked)).count();
        if score > best_score {
            best = line;
            best_score = score;
        }
    }
    if !asked.is_empty() && best_score == 0 {
        return Some(nearest_quote(nearest, reading, memory));
    }
    if SAID_PROPERTY.is_match(&reading.clean) && !best.chars().any(|letter| letter.is_ascii_digit())
    {
        return Some(nearest_quote(best, reading, memory));
    }
    Some(own_quote(best, reading, memory))
}

pub(super) fn recall(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if let Some(ask) = chronology(&reading.clean) {
        // The explicit position tells us which end of the transcript to read.
        // A general own-words search would otherwise take "at the start" as
        // a topic and mistake the latest line for the earliest one.
        return match ask {
            Chronology::FirstUser => Some(first_user(memory)),
            Chronology::FirstBot => Some(first_bot(memory)),
            Chronology::LastUser => last_user(memory),
        };
    }
    if TASK_ASK.is_match(&reading.clean) {
        let line = previous_lines(memory)
            .rev()
            .find(|line| !sentence_is_a_question(line) && TASK_SAID.is_match(line))?;
        return Some(own_quote(line, reading, memory));
    }
    let Some(ask) = RECALL_ABOUT.captures(&reading.clean) else {
        if !OWN_WORDS_ASK.is_match(&reading.clean) {
            return None;
        }
        if OFTEN_ASK.is_match(&reading.clean) {
            return said_back(reading, memory);
        }
        if let Some(kind) = Kind::infer(&reading.clean)
            && let Some(line) = earlier_value(kind, &reading.clean, memory)
        {
            if kind == Kind::Age && line.to_lowercase().contains("going on") {
                return Some(nearest_quote(line, reading, memory));
            }
            return Some(own_quote(line, reading, memory));
        }
        return said_back(reading, memory);
    };
    let topic = ask.name("topic")?.as_str().trim();
    if matches!(
        topic,
        "myself" | "ourselves" | "it" | "that" | "this" | "them"
    ) {
        return None;
    }
    let keywords = useful_words(topic);
    if keywords.is_empty() {
        return None;
    }
    let mut question = None;
    for line in previous_lines(memory).rev() {
        if RECALL.is_match(&line.to_lowercase())
            || TELLING.is_match(line)
            || !matching_topic(&keywords, line)
        {
            continue;
        }
        if sentence_is_a_question(line) {
            question = question.or(Some(line));
        } else {
            return Some(own_quote(line, reading, memory));
        }
    }
    if let Some(line) = question {
        let lead = ASKED_LEADS[memory.turns.len().min(8) % ASKED_LEADS.len()];
        return Some(format!("{lead} \"{}\"", excerpt(line, 120)));
    }
    if let Some(nearest) = previous_lines(memory).next_back() {
        return Some(nearest_quote(nearest, reading, memory));
    }
    let shown = topic
        .strip_prefix("my ")
        .map_or_else(|| topic.to_owned(), |topic| format!("your {topic}"));
    let options = [
        format!("You haven't said anything to me about {shown} yet."),
        format!("You haven't mentioned {shown} — say it and I'll keep it."),
    ];
    Some(compose::pick(&options.each_ref().map(String::as_str), memory, "").into())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Age,
    Name,
    Schedule,
    Frequency,
    Size,
    Money,
    Duration,
    Count,
    Place,
}

impl Kind {
    fn infer(ask: &str) -> Option<Self> {
        [
            (&AGE_ASK, Self::Age),
            (&NAME_ASK, Self::Name),
            (&WHEN_ASK, Self::Schedule),
            (&OFTEN_ASK, Self::Frequency),
            (&SIZE_ASK, Self::Size),
            (&MONEY_ASK, Self::Money),
            (&DURATION_ASK, Self::Duration),
            (&COUNT_ASK, Self::Count),
            (&PLACE_ASK, Self::Place),
        ]
        .into_iter()
        .find_map(|(pattern, kind)| pattern.is_match(ask).then_some(kind))
    }

    fn pattern(self) -> Option<&'static Regex> {
        Some(match self {
            Self::Age => return None,
            Self::Name => &NAME_SAID,
            Self::Schedule => &WHEN_SAID,
            Self::Frequency => &OFTEN_SAID,
            Self::Size => &SIZE_SAID,
            Self::Money => &MONEY_SAID,
            Self::Duration => &DURATION_SAID,
            Self::Count => &COUNT_SAID,
            Self::Place => &PLACE_SAID,
        })
    }

    fn carried_by(self, line: &str) -> bool {
        if matches!(self, Self::Duration | Self::Count) {
            let line = FIGURE_RATE.replace_all(line, " ");
            return self.pattern().is_some_and(|value| value.is_match(&line));
        }
        self.pattern().map_or_else(
            || AGE_SAID.is_match(line).unwrap_or(false),
            |value| value.is_match(line),
        )
    }

    fn values(self, line: &str) -> BTreeSet<String> {
        let line = line.to_ascii_lowercase();
        let plain = |value: &str| {
            flattened(&value.replace(
                |letter: char| !letter.is_ascii_alphanumeric() && letter != '.' && letter != ' ',
                " ",
            ))
        };
        if let Some(pattern) = self.pattern() {
            return pattern
                .find_iter(&line)
                .map(|value| plain(value.as_str()))
                .filter(|value| !value.is_empty())
                .collect();
        }
        AGE_SAID
            .find_iter(&line)
            .filter_map(Result::ok)
            .map(|value| plain(value.as_str()))
            .collect()
    }
}

fn points_to_person(ask: &str, line: &str) -> bool {
    let who = |text: &str, wanted: &[&str]| {
        tokens(text).iter().any(|word| {
            wanted.iter().any(|name| {
                word == name || word.split_once('\'').is_some_and(|(root, _)| root == *name)
            })
        })
    };
    const SHE: &[&str] = &[
        "she", "her", "mother", "mum", "mom", "daughter", "sister", "wife",
    ];
    const HE: &[&str] = &[
        "he", "him", "his", "father", "dad", "son", "brother", "husband",
    ];
    (!who(ask, &["she", "her"]) || who(line, SHE))
        && (!who(ask, &["he", "him", "his"]) || who(line, HE))
}

pub(crate) fn recall_cue(ask: &str) -> bool {
    RECALL.is_match(ask)
}

pub(crate) fn their_own_words(ask: &str) -> bool {
    THEIR_OWN_WORDS.is_match(ask)
}

fn hand_back(line: &str, seed_text: &str, memory: &Memory) -> String {
    let line = flattened(line);
    let seed = format!("{line} {seed_text}");
    let lead = compose::seeded_choice(HAND_BACK, memory, &seed);
    format!("{lead} \"{line}\".")
}

fn claim(reply: &str) -> Option<&str> {
    CLAIM.find(reply).map(|found| &reply[found.start()..])
}

fn subjects(line: &str, minimum: usize) -> Vec<String> {
    let line = line.to_ascii_lowercase();
    let mut seen = Vec::new();
    for word in SAID_WORD.find_iter(&line).map(|word| word.as_str()) {
        if word.len() >= minimum
            && !word.contains('\'')
            && !CLAIM_STOP
                .split_whitespace()
                .chain(VALUE_WORDS.split_whitespace())
                .any(|stop| stop == word)
            && (minimum > 2 || !seen.iter().any(|seen| seen == word))
        {
            seen.push(word.to_owned());
        }
    }
    seen
}

fn mentions(line: &str, words: &[String]) -> bool {
    let low = line.to_ascii_lowercase();
    SAID_WORD.find_iter(&low).any(|hit| {
        words.iter().any(|word| {
            hit.as_str().strip_prefix(word).is_some_and(|tail| {
                tail.starts_with('\'') || matches!(tail, "" | "s" | "es" | "ing" | "ed")
            })
        })
    })
}

fn history(memory: &Memory) -> impl DoubleEndedIterator<Item = (usize, &str)> {
    memory
        .turns
        .iter()
        .enumerate()
        .filter_map(|(number, turn)| {
            let line = turn.user.trim();
            (!line.is_empty() && !sentence_is_a_question(line)).then_some((number + 1, line))
        })
}

fn carried<'a>(
    memory: &'a Memory,
    kind: Kind,
    about: &[String],
    window: usize,
    prefer: &[String],
    after: usize,
) -> Option<(usize, &'a str)> {
    let hits: Vec<_> = history(memory)
        .rev()
        .take(window)
        .filter(|&(number, line)| number > after && kind.carried_by(line))
        .collect();
    if !about.is_empty()
        && let Some(found) = hits.iter().find(|(_, line)| mentions(line, about))
    {
        return Some(*found);
    }
    if !prefer.is_empty() {
        let best = hits
            .iter()
            .rev()
            .map(|&(number, line)| {
                let count = prefer
                    .iter()
                    .filter(|word| mentions(line, std::slice::from_ref(word)))
                    .count();
                (count, number, line)
            })
            .max_by_key(|&(score, number, _)| (score, number));
        if let Some((score, number, line)) = best
            && score > 0
        {
            return Some((number, line));
        }
    }
    if about.is_empty() {
        hits.first().copied()
    } else {
        None
    }
}

fn one_line(memory: &Memory, kind: Kind) -> Option<&str> {
    let mut hits = history(memory).filter(|&(_, line)| kind.carried_by(line));
    let (_, line) = hits.next()?;
    hits.next().is_none().then_some(line)
}

fn line_named<'a>(ask: &str, memory: &'a Memory) -> Option<(Kind, Vec<String>, &'a str)> {
    if !recall_cue(ask) && !their_own_words(ask) {
        return None;
    }
    let kind = Kind::infer(ask)?;
    let words = subjects(ask, 2);
    if words.is_empty() {
        return None;
    }
    let (_, line) = carried(memory, kind, &[], usize::MAX, &words, 0)?;
    Some((kind, words, line))
}

pub(crate) fn gives_named_value(ask: &str, reply: &str, memory: &Memory) -> bool {
    line_named(ask, memory)
        .is_some_and(|(kind, _, line)| !kind.values(reply).is_disjoint(&kind.values(line)))
}

pub(crate) fn another_line_was_asked_for(ask: &str, reply: &str, memory: &Memory) -> bool {
    line_named(ask, memory).is_some_and(|(kind, words, line)| {
        !mentions(reply, &words) && kind.values(reply).is_disjoint(&kind.values(line))
    })
}

pub(crate) fn repair(reply: &str, ask: &str, memory: &Memory) -> Option<String> {
    if let Some(answer) = put_right(reply, memory) {
        return Some(answer);
    }
    if !QUOTE_HEAD.is_match(reply) {
        return None;
    }
    let quote = STRAIGHT_QUOTE.captures(reply)?.get(1)?.as_str();
    let kind = Kind::infer(ask)?;
    if !kind.values(quote).is_empty() {
        return None;
    }
    let (_, line) = carried(memory, kind, &[], usize::MAX, &[], 0)?;
    if flattened(line).eq_ignore_ascii_case(&flattened(quote)) {
        return None;
    }
    Some(hand_back(line, claim(reply).unwrap_or(""), memory))
}

fn put_right(reply: &str, memory: &Memory) -> Option<String> {
    let claim = claim(reply)?;
    let asked = Kind::infer(&claim.to_ascii_lowercase());
    let kinds = [
        Kind::Age,
        Kind::Name,
        Kind::Schedule,
        Kind::Size,
        Kind::Money,
        Kind::Frequency,
        Kind::Duration,
    ];
    let subjects = subjects(claim, 3);
    let window = if subjects.is_empty() { 3 } else { usize::MAX };
    let pronouns = ["she".to_owned(), "he".to_owned(), "they".to_owned()];
    for kind in asked
        .as_ref()
        .map_or(kinds.as_slice(), std::slice::from_ref)
    {
        if let Some((_, line)) = carried(memory, *kind, &subjects, window, &[], 0) {
            return Some(hand_back(line, claim, memory));
        }
        // The caller supplies the store's claim without declaring a subject of its own.
        // A stated subject in the reply can therefore fall back to a pronoun anywhere in history.
        if !subjects.is_empty()
            && let Some((_, line)) = carried(memory, *kind, &pronouns, usize::MAX, &[], 0)
        {
            return Some(hand_back(line, claim, memory));
        }
    }
    if !subjects.is_empty() && asked.is_none() {
        let (_, line) = history(memory)
            .rev()
            .find(|&(_, line)| mentions(line, &subjects))?;
        return Some(hand_back(line, claim, memory));
    }
    None
}

pub(crate) fn value_from_history(ask: &str, memory: &Memory) -> Option<String> {
    let kind = Kind::infer(ask)?;
    let recall = recall_cue(ask);
    if !recall
        && (!QUESTION.is_match(ask)
            || matches!(kind, Kind::Schedule | Kind::Duration)
            || NOT_A_RECALL.is_match(ask))
    {
        return None;
    }
    let about = subjects(ask, 3);
    let named = subjects(ask, 2);
    let kinds = [kind, Kind::Size];
    let count = if !recall && kind == Kind::Money && ask.contains("how much") {
        2
    } else {
        1
    };
    for &kind in &kinds[..count] {
        let found = if about.is_empty() {
            carried(memory, kind, &[], usize::MAX, &named, 0).or_else(|| {
                named
                    .is_empty()
                    .then(|| carried(memory, kind, &[], 3, &[], 0))
                    .flatten()
            })
        } else {
            carried(memory, kind, &about, usize::MAX, &[], 0)
        };
        if let Some((_, line)) = found {
            return Some(hand_back(line, ask, memory));
        }
        if (named.is_empty() || history(memory).any(|(_, line)| mentions(line, &about)))
            && let Some(line) = one_line(memory, kind)
        {
            return Some(hand_back(line, ask, memory));
        }
    }
    None
}

fn earlier_value<'a>(kind: Kind, ask: &str, memory: &'a Memory) -> Option<&'a str> {
    let anchors: Vec<_> = useful_words(ask)
        .into_iter()
        .filter(|word| {
            !matches!(
                word.as_str(),
                "she"
                    | "they"
                    | "him"
                    | "his"
                    | "her"
                    | "them"
                    | "it's"
                    | "live"
                    | "lived"
                    | "living"
                    | "ago"
                    | "this"
                    | "that"
                    | "there"
                    | "same"
            )
        })
        .collect();
    let mut candidates = previous_lines(memory).rev().filter(|line| {
        !sentence_is_a_question(line) && kind.carried_by(line) && points_to_person(ask, line)
    });
    if anchors.is_empty() {
        candidates.next()
    } else {
        candidates.find(|line| matching_topic(&anchors, line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    fn history(lines: &[&str]) -> Memory {
        let turns = lines
            .iter()
            .map(|line| Exchange {
                user: (*line).to_owned(),
                bot: "Go on.".to_owned(),
                module: "greeting",
            })
            .collect();
        Memory {
            turns,
            ..Memory::default()
        }
    }

    #[test]
    fn an_explicit_topic_query_prefers_a_statement_over_a_newer_question() {
        let memory = history(&[
            "My bike chain keeps slipping.",
            "How do I fix the bike chain?",
        ]);
        let response = recall(
            &Reading::new("What did I say about my bike chain?"),
            &memory,
        )
        .unwrap();
        assert!(response.ends_with("“My bike chain keeps slipping.”"));
    }

    #[test]
    fn the_quote_lane_and_the_denial_repair_read_frequency_differently() {
        let memory = history(&[
            "I go to the gym three times a week.",
            "I walk the dog every day.",
        ]);
        let response = recall(
            &Reading::new("How often did I say I go to the gym?"),
            &memory,
        )
        .unwrap();
        assert!(response.contains("I walk the dog every day."), "{response}");
        assert!(another_line_was_asked_for(
            "how often did i say i go to the gym?",
            &response,
            &memory
        ));
        assert!(
            value_from_history("how often did i say i go to the gym?", &memory)
                .unwrap()
                .contains("I go to the gym three times a week.")
        );
        assert!(value_from_history("how often should i go to the gym?", &memory).is_none());
    }

    #[test]
    fn chronological_recall_skips_greetings_and_previous_recall_questions() {
        let memory = history(&[
            "Hi there!",
            "I live in Leeds.",
            "What did I just tell you?",
            "I finished painting yesterday.",
            "What did I say first?",
        ]);
        assert!(
            recall(&Reading::new("My first message?"), &memory)
                .unwrap()
                .contains("I live in Leeds.")
        );
        assert!(
            recall(&Reading::new("What did I just say?"), &memory)
                .unwrap()
                .contains("I finished painting yesterday.")
        );
    }

    #[test]
    fn start_recall_beats_the_nearest_line_in_a_long_conversation() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My name is Sam.");
        bot.turn("I told the garage about the noise and they said bring it in Tuesday.");
        for index in 1..=64 {
            bot.turn(&format!("What is 17 percent of {}?", 200 + index));
        }
        bot.turn("What is my favourite fruit?");

        for ask in [
            "What did I say at the start of this chat?",
            "What was my first message in our conversation?",
            "Remind me what I said at the beginning of this thread",
            "Which thing did I say first?",
        ] {
            let turn = bot.turn(ask);
            assert_eq!(turn.module, Some("memory_store"), "{ask}: {}", turn.reply);
            assert!(
                turn.reply.contains("\"My name is Sam.\""),
                "{ask}: {}",
                turn.reply
            );
        }
        let quote = bot.turn("Before I go, what did the garage say?");
        assert_eq!(quote.module, Some("what_they_said"));
        assert_eq!(quote.reply, "You said bring it in Tuesday.");

        bot.reset().unwrap();
        let turn = bot.turn("What did I say at the start of this chat?");
        assert_eq!(turn.module, Some("memory_store"));
        assert!(!turn.reply.contains("Sam"), "{}", turn.reply);
    }

    #[test]
    fn chronology_keeps_the_bot_and_the_persons_first_lines_distinct() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("What is 17% of 240?");
        bot.turn("What is 5% of 100?");
        let turn = bot.turn("What did you say at the start of our conversation?");
        assert_eq!(turn.module, Some("memory_store"));
        assert!(turn.reply.contains("17% of 240 is 40"), "{}", turn.reply);
        assert!(!chronology_ask(
            "What did I say at the start of the meeting?"
        ));
        assert!(!chronology_ask(
            "The argument started and I don't know how it started"
        ));
    }
}
