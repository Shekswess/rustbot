//! Read the bill in a turn that describes a shock at opening it.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Memory, Reading};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    ENDING,
    r"\b(?:nearly|almost|bloody near|like to have) (?:died|fell over|fainted|passed out|choked|keeled over|collapsed)\b|\b(?:had|having) a heart attack\b|\bmy heart (?:stopped|sank)\b|\bcould(?:n'?t| not) (?:breathe|get my breath|catch my breath)\b|\bmy (?:jaw|eyes|heart) (?:dropped|fell|nearly dropped)\b|\b(?:was|were) (?:gasping|choking)\b|\b(?:choked|gasped)\b|\bnearly (?:cried|wept)\b|\bwent (?:white|pale)\b|\bmy knees (?:went|gave)\b"
);
pattern!(
    PAGE_SHOCK,
    r"\btook my breath away\b|\bmy (?:jaw|heart) (?:dropped|fell|sank)\b|\bwent (?:white|pale)\b|\bmy knees (?:went|gave)\b"
);
pattern!(DOCUMENT, r"\b(?:bills?|statements?|invoices?|invoiced)\b");
pattern!(
    NOT_YET,
    r"\b(?:hasn'?t|haven'?t|hadn'?t|didn'?t|never|no)\b[^.!?]{0,30}\b(?:arrived|come|came|turned up|showed up)\b|\bno (?:bill|statement|invoice) (?:yet|at all)\b|\b(?:not|never) (?:had|received|got) (?:it|a (?:bill|statement|invoice)) yet\b|\bstill waiting\b"
);
pattern!(
    PAGE_VERB,
    r"\b(?:saw|seen|opening|opened|read|reading|looked at|looking at|looked through|checked|checking|got|get|received|receiving|landed|arrived|through the door)\b"
);
pattern!(
    ONLY_PAGE,
    r"^(?:i|we)?\s*(?:have |had |just |only just |finally |today |this morning |yesterday )*(?:saw|seen|opened|opening|read|reading|got|get|received|checked|looked at) (?:the|my|this|our|that|a) (?:bills?|statements?|invoices?)\b(?: (?:today|yesterday|this morning|last night|again|now))?[.!]*$"
);
pattern!(
    SYMPTOM,
    r"\b(?:ache|aches|aching|bled|bleed|bleeding|breath|breathe|breathing|bruise|bruised|confused|cough|coughing|cramps|dizziness|dizzy|exhausted|fever|forgetful|forgetting|headaches?|hurt|hurting|hurts|infection|lumps?|memory|migraines?|nausea|nauseous|pains?|painful|palpitations|pregnant|rash|sick|sore|swelling|swollen|temperature|tired|vomit|vomiting|wheezing|numb|numbness|weak|weakness|shaky|shaking|lightheaded|sweating|faint|fainting|breathless|breathlessness|blacked out|blackout|blacked|passed out|out cold|unconscious|keeled over|collapsed|fainted|fainting fits?)\b"
);
pattern!(
    BODY_PART,
    r"\b(?:ankles?|arms?|back|belly|chest|ears?|eyes?|feet|fingers?|foot|gums?|hands?|head|hips?|jaw|knees?|legs?|neck|shoulders?|sinus(?:es)?|skin|stomach|teeth|throat|toes?|tooth|wrists?)\b"
);
pattern!(
    BODY_STATE,
    r"\b(?:hurt|hurts|hurting|sore|soreness|tight|tightness|numb|numbness|ache|aches|aching|throb|throbbing|swollen|swelling|bleeding|stiff|stiffness|weak|weakness|shaky|shaking|still|worse|pounding|squeezing|burning|itchy|itching|watering|gone|goes|went)\b"
);
pattern!(
    SOMEBODY_ELSE,
    r"\b(?:my|his|her|their|our) (?:mum|mom|mammy|dad|father|mother|son|daughter|wife|husband|partner|boyfriend|girlfriend|nan|nana|gran|granny|grandma|granddad|grandad|grandpa|grandmother|grandfather|brother|sister|uncle|aunt|cousin|friend|kid|kids|child|baby|boy|girl|dog|cat|neighbour|neighbor)\b|\b(?:he|she|they|him|her|them)\b"
);
pattern!(
    FIGURE,
    r"(?:£\s*\d|\b\d+(?:[.,]\d+)?\s*(?:pounds?|quid|p|per ?cent)\b|\b\d+(?:[.,]\d+)?\s*%|\b(?:for|of|cost(?:s|ing)?|paid|owe[sd]?|totall?ing) £?\d+)"
);
pattern!(
    MONEY_PROBLEM,
    r"\b(?:charged (?:me )?twice|double charged|charged me double|duplicate|over ?charged|can'?t pay|can'?t afford|cannot pay|cannot afford|owe|owes|owed|arrears|payment plan|instal+ments?|wrong|too much|discount|rebate|refund|benefit|hardship|meter read|reading|estimated)\b"
);
pattern!(
    MONEY_ANSWER,
    r"(?i)\b(?:pay|pays|paid|payment|payments|overdraft|refund|refunds|duplicate|charge|charges|charged|owe|owes|arrears|debit|instal+ments?|bill|bills|council|band|discount|discounts|rebate|tariff|standing charge|meter|account|bank|money|rent)\b"
);
pattern!(WORD, r"[a-z']+");

const ACCOUNT: &str = "account annual quarterly monthly bank bill card charge council credit debit electricity electric energy gas insurance internet loan metered metering meter mobile mortgage phone rent service sewerage student supply tax tv utility water";
const DESCRIPTORS: &str = "a an the my our your their his her its this that these those some any no big small huge massive enormous giant tiny little new old latest last first second third final whole same other another next unexpected surprise shocking ridiculous stupid bloody damn awful terrible monthly quarterly annual weekly daily";
const PRECEDING: &str = "i me my we us our you your he him his she her it its they them their saw see seen seeing read reading reads open opened opening opens got get gets getting receive received receiving arriving arrived arrive came come comes coming paid pay paying sent send sends sending took take taken in on at for from with about of to and but or when after before then now today the first second last final";

const PAGE: &[&str] = &[
    "A bill is worth checking before you pay it: the dates it covers, what it says you are being charged for, and the reference. Ask for the breakdown in writing, and if it is wrong say so in writing before the date on it, keeping a copy. What is it for, and how much?",
    "Before paying a bill you did not expect, check the dates it covers, what it is charging for and the reference. Ask for the breakdown in writing, and answer in writing before the date on it, keeping a copy. What is it charging you for?",
];
const WITH_AMOUNT: &[&str] = &[
    "That is worth checking before you pay it: the dates it covers, what it says you are being charged for, and the reference. Ask for the breakdown in writing, and if it is wrong say so in writing before the date on it, keeping a copy. What is the reference on it?",
    "Before you pay it, check the dates it covers, what it says you are charged for and the reference. Ask for the breakdown in writing and keep a copy of everything. Does it say what period it covers?",
];
const BEFORE_ARRIVAL: &[&str] = &[
    "When it comes, the three things to find are the date they want an answer by, what it says you are charged for and the reference. Ask them to confirm when it was sent and to which address, and note who you spoke to and when.",
    "While you wait, it is worth confirming they have your right address, and noting the date and the name of whoever you speak to. When it lands, the reply-by date, what it charges for and the reference are the things to check first.",
];

fn another_bill_kind(line: &str) -> bool {
    if line.contains("council tax") {
        return true;
    }
    for document in DOCUMENT.find_iter(line) {
        let (before, _) = line.split_at(document.start());
        let last = before
            .char_indices()
            .rev()
            .nth(39)
            .map_or(0, |(start, _)| start);
        if let Some(kind) = WORD
            .find_iter(&before[last..])
            .filter(|word| {
                !DESCRIPTORS
                    .split_whitespace()
                    .any(|known| known == word.as_str())
            })
            .last()
        {
            return !PRECEDING
                .split_whitespace()
                .chain(ACCOUNT.split_whitespace())
                .any(|known| known == kind.as_str());
        }
    }
    false
}

fn refers_to_the_page(line: &str, ending: Option<regex::Match<'_>>) -> bool {
    let Some(document) = DOCUMENT.find(line) else {
        return false;
    };
    if (ONLY_PAGE.is_match(line) && !NOT_YET.is_match(line)) || PAGE_SHOCK.is_match(line) {
        return true;
    }
    let Some(ending) = ending else {
        return false;
    };
    if NOT_YET.is_match(line) || DOCUMENT.find_at(line, ending.end()).is_some() {
        return true;
    }
    let before = &line[..document.start()];
    let last = before
        .char_indices()
        .rev()
        .nth(23)
        .map_or(0, |(start, _)| start);
    PAGE_VERB.is_match(&before[last..])
}

pub(in crate::bot) fn yields_to_money_answer(reading: &Reading<'_>, answer: &str) -> bool {
    MONEY_PROBLEM.is_match(&reading.clean) && MONEY_ANSWER.is_match(answer)
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading
        .raw
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let ending = ENDING.find(&line);
    if !refers_to_the_page(&line, ending) || another_bill_kind(&line) {
        return None;
    }
    if let Some(ending) = ending {
        let tail = &line[ending.end()..];
        if SOMEBODY_ELSE.is_match(&line[..ending.start()])
            || SYMPTOM.is_match(tail)
            || (BODY_PART.is_match(tail) && BODY_STATE.is_match(tail))
        {
            return None;
        }
    }
    let lines = if NOT_YET.is_match(&line) {
        BEFORE_ARRIVAL
    } else if FIGURE.is_match(&line) {
        WITH_AMOUNT
    } else {
        PAGE
    };
    let recent = memory.turns.last().map(|turn| turn.bot.as_str());
    Some(
        lines
            .iter()
            .find(|reply| Some(**reply) != recent)
            .unwrap_or(&lines[0])
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn shock_at_bill_keeps_the_page_and_yields_to_real_or_specialist_detail() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("I nearly died when I saw the bill");
        assert_eq!(turn.module, Some("a_shock_at_a_bill"));
        assert!(turn.reply.contains("dates it covers"));
        let turn = bot.turn("I nearly died when I saw the bill for £600");
        assert!(turn.reply.contains("What is the reference"));
        for line in [
            "I nearly died when I saw the council tax bill",
            "I nearly died when I saw the vet bill",
            "I nearly fainted when I saw the bill and now I feel sick",
            "My mum nearly died and they sent the bill",
        ] {
            assert!(
                respond(&Reading::new(line), &Memory::default()).is_none(),
                "{line}"
            );
        }
        assert!(yields_to_money_answer(
            &Reading::new("I nearly died at the bill and they charged me twice"),
            "Ask for the duplicate payment to be reversed."
        ));
    }
}
