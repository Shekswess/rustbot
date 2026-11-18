//! Recognize when the person wants to be heard or asks for an opinion.
use crate::bot::{Memory, Reading, compose};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $pattern:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
    };
}

pattern!(
    DECLINE,
    concat!(
        r"\b(?:(?:i|we|he|she|they) (?:dont|don't|do not|doesnt|doesn't|didnt|didn't|wouldnt|wouldn't) ",
        r"(?:really |actually |even )?(?:want|need|wanted|needed|asked for|ask for) ",
        r"(?:(?:any|no|your|more|further|the|medical|financial|legal|diet|parenting|career) ){0,2}",
        r"(?:advice|advise|solutions?|tips?|suggestions?|a lecture|lectures?|to be told what to do|telling me what to do)|",
        r"(?:i|we|im|i'm) (?:am )?(?:not|n't) (?:looking|asking|after|here) for (?:a |any |the )?(?:advice|solutions?|fix|fixing|tips?|suggestions?)|",
        r"(?:not|n't) looking for (?:a |any )?(?:solution|advice|fix)|",
        r"no (?:advice|solutions?|tips?|lectures?)(?: (?:please|thanks|thank you))?|",
        r"without (?:the )?(?:advice|lecture|tips)|",
        r"(?:dont|don't|do not|no) (?:give|giving) me (?:any )?(?:advice|tips)|",
        r"not (?:asking|looking) for (?:advice|a solution)|",
        r"(?:dont|don't|do not|stop) tell(?:ing)? me what to do|",
        r"(?:i|i've|ive) (?:already )?know what to do, (?:i )?(?:just|only) (?:want|wanted|need|needed))\b"
    )
);
pattern!(
    WANT_HEARD,
    concat!(
        r"\b(?:(?:just|only|really) (?:want|wanted|need|needed) (?:to )?",
        r"(?:talk|vent|be heard|get (?:it|this|that|something) off (?:my|our) chest)|",
        r"(?:just|only) (?:wanted|want|needed) to (?:say (?:it|this|that|something)|tell (?:you|u|someone|somebody|them|him|her) something)|",
        r"(?:just|only) (?:wanted|want) (?:to )?(?:say|tell) (?:it )?out loud|wanted to say it out loud|",
        r"i (?:just )?(?:want|wanted) to be heard|",
        r"(?:just |only )?(?:want|wanted|need|needed) (?:someone|somebody|a person) (?:who )?(?:to )?listen|",
        r"just (?:talk|listen) to me|(?:let|allow) me (?:to )?(?:talk|vent|rant)|",
        r"(?:just |only )?(?:want|need|wanted|needed) to (?:vent|rant)|",
        r"can i (?:just )?(?:vent|rant)|i need (?:a )?(?:vent|rant)|can i just talk)\b"
    )
);
pattern!(
    JUST_LISTEN,
    r"\b(?:just|only) listen(?: to me)?(?: please)?\s*[.!]*$"
);
pattern!(
    VIEW,
    concat!(
        r"\b(?:what (?:do )?(?:you|u) (?:think|reckon|make of it)|",
        r"whats your (?:honest )?(?:opinion|view|take)|what is your (?:opinion|view|take)|",
        r"your honest opinion|your view on it|(?:must|would you) think of me|",
        r"does that (?:sound|seem) ?(?:mad|silly|wrong|bad)|am i (?:being )?(?:mad|silly|wrong|unreasonable))\b"
    )
);
pattern!(
    NOT_THIS,
    concat!(
        r"\b(?:what|some|good|the best|better) advice\b|",
        r"should i take (?:her|his|their|my|any|some) advice|",
        r"(?:give|giving|offer|offering) (?:her|him|them|my|his|your) advice|",
        r"(?:dont|don't|do not) want to give|asked (?:for|about) (?:her|his|their|my) advice"
    )
);
pattern!(
    ASK,
    concat!(
        r"(?:\b(?:what|how|which|when|where|who|why|should|could|would|can|is|are|does|do)\b[^?.!]{0,60}\?|",
        r"\b(?:what|how|which|where|who) (?:do|should|can|could|would)\b|",
        r"\bwhats\b|\bwhat's\b|\bshould i\b|\bcan i\b|\bcould i\b)"
    )
);
pattern!(
    ASK_LATER,
    concat!(
        r"\b(?:what|whats|what's|how|which|when|where|who|why|should|could|would|can|is|are|was|",
        r"were|do|does|did|will|want to know|wondering|wonder if|tell me (?:if|whether|what|how)|",
        r"let me know (?:if|whether|what|how))\b"
    )
);
pattern!(
    NOT_ASK,
    concat!(
        r"\b(?:is that (?:ok|okay|alright|fine)|if thats? (?:ok|okay|alright|fine)|",
        r"does that make sense|is that alright with you)\b"
    )
);
pattern!(
    DEATH,
    concat!(
        r"\b(?:died|dying|dead|death|killed|suicide|passed away|passed on|funeral|burial|cremation|",
        r"took (?:his|her|their) own life|buried|burying|bereav\w*|grief|grieving|widow|widowed|mourning)\b|",
        r"\b(?:lost|bury|lose|losing) (?:(?:my|our|her|his|their|the) )?(?:mum|mom|mother|dad|father|wife|",
        r"husband|baby|child|son|daughter|brother|sister|partner|gran|granny|nan|grandad|grandpa|friend|mate)\b"
    )
);

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = &reading.clean;
    let first = [&*DECLINE, &*WANT_HEARD, &*JUST_LISTEN]
        .iter()
        .filter_map(|pattern| pattern.find(line))
        .min_by_key(|hit| hit.start())?;
    if DEATH.is_match(line) {
        return None;
    }
    let view = VIEW.is_match(line);
    if !view
        && (NOT_THIS.is_match(line)
            || ASK.is_match(&JUST_LISTEN.replace_all(&WANT_HEARD.replace_all(line, " "), " "))
            || ASK_LATER.is_match(&NOT_ASK.replace_all(&line[first.end()..], " ")))
    {
        return None;
    }
    Some(compose::pick(if view { A_VIEW } else { HEARD }, memory, reading.raw).into())
}

const HEARD: &[&str] = &[
    "Fair enough - I'll keep the advice to myself. Tell me the rest of it.",
    "Understood. I'm listening - how are you doing with it?",
    "Right, no fixing from me then. What's on your mind?",
    "That's fair. Say it all and I'll just listen.",
    "No advice, then. Go on - I'm here.",
    "Okay - no advice from me. Where are you with it all?",
];
const A_VIEW: &[&str] = &[
    "Then tell me the whole of it and I'll say what I think - no fixing, just my honest view.",
    "Okay - the situation as it is, and I'll give you my straight view of it.",
    "I'll tell you what I think, then. What's the situation?",
    "Fair enough. Lay it out and I'll tell you honestly what I make of it.",
];
