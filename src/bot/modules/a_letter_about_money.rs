//! Fallback for a bill, letter, or notice about money.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Memory, Reading};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

const DOCUMENT: &str = r"\b(?:a|an|this|the|my|our) +(?:[a-z'-]+ +){0,2}(letter|notice|bill)\b";
pattern!(
    ARRIVED,
    &format!(
        r"\b(?:got|received|receive|had|have)\b[^.!?]{{0,30}}{DOCUMENT}|\b(?:(?:was|were|been|be|got) sent|sent (?:me|us))\b[^.!?]{{0,20}}{DOCUMENT}|{DOCUMENT}[^.!?]{{0,25}}\b(?:says?|said|about|from|saying|came|arrived|turned up)\b"
    )
);
pattern!(NOUN, r"\b(?:letter|notice|bill)\b");
pattern!(
    NOT_YET,
    r"\b(?:hasn'?t|haven'?t|hadn'?t|didn'?t|never|not|still no|no)\b[^.!?]{0,30}\b(?:arrived|arrive|come|came|turned up|show(?:ed|n)? up|been sent|receive|received|got|get|had)\b|\b(?:still )?waiting (?:for|on)\b|\bno (?:[a-z'-]+ ){0,3}(?:letter|notice|bill)\b[^.!?]{0,20}\byet\b"
);
pattern!(
    OUTGOING,
    r"\b(?:i|we) (?:sent|send|sending|posted|post|posting|wrote|write|writing|need|needs|want|wants|asked|asking)\b|\b(?:have|had|got|need|needs|want|wants) to (?:write|send|post|get|receive)\b|\b(?:can|could|may|might|should|would|will) (?:i|we) (?:get|have|receive|request|ask)\b|\b(?:could|can|would|will) you (?:send|post|write|get)\b|\b(?:writing|sending|posting) (?:a|an|the|my|our) (?:letter|notice|bill)\b"
);
pattern!(
    MONEY,
    r"\b(?:overpay(?:ment|ments)?|overpaid|benefits?|universal credit|tax credits?|housing benefit|child benefit|dwp|hmrc|tax|taxes|council tax|arrears?|debts?|owe|owed|owes|repay|repayment|money|amount|charges?|bills?|rents?|mortgage|insurance|loans?|credit card|pension|pip|dla|esa|jsa|income support|student loan|fines?)\b"
);
pattern!(
    AMOUNT,
    r"£\s*\d|\b\d+(?:[.,]\d+)?\s*(?:pounds?|quid|p|pc|per ?cent)\b|\b\d+(?:[.,]\d+)?\s*%|\b(?:for|of|cost(?:s|ing)?|paid|owe[sd]?) £?\d+\b"
);
pattern!(
    DATE,
    r"\b(?:today|yesterday|tomorrow|(?:this|last|next) (?:week|month|year)|january|february|march|april|may|june|july|august|september|october|november|december|monday|tuesday|wednesday|thursday|friday|saturday|sunday|\d{1,2}[/-]\d{1,2}(?:[/-]\d{2,4})?)\b"
);
pattern!(WORD, r"[a-z]+");

const LETTER: &[&str] = &[
    "Check the date they want a reply by, the amount and the period it covers. Use the sender's official contact details to ask for a written breakdown. If you want to challenge a decision, check which procedure and deadline apply, and keep a copy of anything you send.",
    "Find the reply deadline, the amount and the period it covers. Ask the sender for a written breakdown and follow the instructions for disputing the decision or charge. Keep a copy of your reply.",
];
const BILL: &[&str] = &[
    "Check the dates it covers, what you're being charged for and the reference. Ask for the breakdown in writing. If something looks wrong, contact the supplier promptly and keep a copy of what you send.",
    "Check what the bill is charging for, which dates it covers and the reference. Ask them for a breakdown and use their dispute process if you think it's wrong. Keep a copy of any reply.",
];
const NOT_ARRIVED: &[&str] = &[
    "If it should have arrived, use the sender's official account page or phone line to confirm when it was sent and where. Note who you spoke to and when. When it comes, look for the reply deadline and the reference.",
    "Check the sender has your current address through their official account page or phone line, and ask when it went out. Keep a note of the call. When it comes, check the reply deadline and the reference.",
];

fn pick(lines: &'static [&'static str], memory: &Memory) -> &'static str {
    if memory
        .turns
        .last()
        .is_some_and(|turn| turn.bot.starts_with(lines[0]))
    {
        return lines[1];
    }
    lines[0]
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading
        .raw
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let line = WORD.replace_all(&line, |word: &regex::Captures<'_>| match &word[0] {
        "recieved" | "recived" | "receved" => "received".to_owned(),
        "benifit" | "benifits" | "benfit" | "benfits" => "benefits".to_owned(),
        "lettre" | "lettr" | "lettar" | "lette" => "letter".to_owned(),
        "univeral" => "universal".to_owned(),
        "overpaymant" => "overpayment".to_owned(),
        "housng" => "housing".to_owned(),
        word => word.to_owned(),
    });
    if !MONEY.is_match(&line) {
        return None;
    }
    let not_yet = NOT_YET.is_match(&line);
    let Some(found) = ARRIVED.captures(&line) else {
        return (not_yet && NOUN.is_match(&line) && !OUTGOING.is_match(&line))
            .then(|| pick(NOT_ARRIVED, memory).to_owned());
    };
    if OUTGOING.is_match(&line[..found.get(0)?.start()])
        || OUTGOING.is_match(&line[found.get(0)?.range()])
    {
        return None;
    }
    if not_yet {
        return Some(pick(NOT_ARRIVED, memory).to_owned());
    }
    let document = found.iter().skip(1).flatten().next()?.as_str();
    let opening = pick(if document == "bill" { BILL } else { LETTER }, memory);
    let missing = match (AMOUNT.is_match(&line), DATE.is_match(&line)) {
        (false, false) => "What amount and dates does it give?",
        (false, true) => "What amount does it give?",
        (true, false) => "What dates does it cover?",
        (true, true) => "Does it say which account or decision the charge relates to?",
    };
    Some(format!("{opening} {missing}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn letter_about_money_separates_arrival_direction_and_missing_details() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("I received a notice about a benefit overpayment of £150");
        assert!(turn.candidates.iter().any(|offer| {
            offer.module == "a_letter_about_money"
                && offer.reply.ends_with("What dates does it cover?")
        }));
        let memory = Memory::default();
        assert!(
            respond(&Reading::new("my benifit lettar never arrived"), &memory)
                .unwrap()
                .contains("official account page")
        );
        assert!(
            respond(
                &Reading::new("I have to send a notice about my rent"),
                &memory
            )
            .is_none()
        );
        assert!(
            respond(
                &Reading::new("I got a letter about my appointment"),
                &memory
            )
            .is_none()
        );
    }
}
