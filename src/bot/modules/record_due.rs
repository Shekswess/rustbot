//! Native port of bot/modules/record_due.py.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;
static RECORD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:my|our)\s+(?:[a-z']+\s+){0,2}(?P<thing>parcels?|orders?|deliver(?:y|ies)|tracking|couriers?|dispatch|library\s+cards?|library\s+books?|library\s+loans?)\b"#).unwrap()
});
static QUALIFIED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)\s+(?:of|that|which)\b"#).unwrap());
static ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^\s*(?:so|and|but|ok|okay|also|hi|hello|morning|quick one)?\s*,?\s*(?:will|is|are|was|were|has|have|did|does|do|can|could|when|where|what|how|any)\b"#).unwrap()
});
static STATE_AFTER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:(?:still|never|now|already)\s+)?(?:is|are|was|were|has|have|had|did|does|do)(?:n't|nt)?\s+(?:not\s+|yet\s+)?(?:[a-z]+\s+){0,2}(?:late|overdue|due|back|out|renewed|expired|dispatched|delivered|arrived|arriving|coming|come|came|delayed|lost|missing|stuck|held|waiting|ready|moved|on\s+its\s+way|turned\s+up|shown\s+up|meant\s+to\s+(?:arrive|come)|supposed\s+to\s+(?:arrive|come)|expected)\b"#).unwrap()
});
static NOT_THE_STATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:how\s+(?:do|can|could|should)\s+i|how\s+to|should\s+i|do\s+i\s+need|return|refund|cancel|change|complain|worth|warranty|guarantee|recycle)\b"#).unwrap()
});
static DELIVERY_STATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:arrive|arrives|arrived|arriving|come|comes|coming|came|get\s+here|be\s+here|turn\s+up|turned\s+up|show\s+up|shown\s+up|delivered|delivery|dispatch|dispatched|tracking|track|check|overdue|late|delayed|lost|missing|stuck|held|waiting|ready|moved|on\s+its\s+way|expired|where|when|before|by|due|today|tomorrow|tonight|this\s+week|next\s+week|mon(?:day)?|tues(?:day)?|wed(?:nesday)?|thurs(?:day)?|fri(?:day)?|sat(?:urday)?|sun(?:day)?)\b"#).unwrap()
});
static LATE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?i)\b(?:late|overdue)\b"#).unwrap());
static BARE_STATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:never|not)\s+(?:arrived|came|come|turned\s+up|shown\s+up|moved)\b"#)
        .unwrap()
});
static LOST: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?i)\b(?:lost|stolen)\b"#).unwrap());
static TROUBLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:late|delayed|lost|missing|stuck|held|still|never|overdue)\b|\bhas(?:n't|nt)\b|\bnot\s+moved\b|\b(?:meant|supposed)\s+to\b"#).unwrap()
});
static AGAIN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(?i)\bagain\b"#).unwrap());
const LIBRARY_CARD_LINES: &[&str] = &[
    "Your library account has the expiry date: sign in with the card number and PIN, or ring the branch and they can renew the card in a minute - usually free if nothing is owed on it.",
    "That lives with the library rather than anywhere I can see. The card number gets you into your account, which shows when it runs out, and the desk can renew it on the spot.",
];
const LIBRARY_LOAN_LINES: &[&str] = &[
    "Your library account has the due date: sign in with the card number and PIN to see the loans and renew them, and the desk or the number on the card can do it in a minute.",
    "That one lives in your library account rather than here. The card number shows what is out, when it is due and how many renewals are left; the branch can renew it by phone.",
];
const LIBRARY_LATE_LINES: &[&str] = &[
    "Ugh, the late ones are a pain. Renew it in your library account first: the card number and PIN do it if nothing is reserved, and that stops the clock. If it is already overdue, take it back and the desk will tell you what is owed - I can't see the fine from here.",
    "Late books happen. The account with the card number and PIN is where the renewal is, if the book is not reserved. Once it is overdue the desk has the amount and can take it back; I can't see either from here.",
];
const LIBRARY_LOST_LINES: &[&str] = &[
    "Tell the library as soon as you can: the desk or the number on their site can cancel the card or mark the book lost, and tell you what a replacement costs. I can't see the charge from here.",
    "That is the library's to sort, not something I can see: ring them or call at the desk with your card number, and they will cancel it or mark the book lost and say what is owed.",
];
const DELIVERY_LINES: &[&str] = &[
    "The tracking link in the confirmation email or the shop's app has the last scan, and if it has not moved for a day or two it is the shop to chase - they open the case with the courier, not you. I can't see the tracking from here.",
    "The tracking link in the order email shows where a parcel is and the day it is due, and if that day has gone the shop chases the courier for you; no window on it from this end.",
];
const DELIVERY_TROUBLE_LINES: &[&str] = &[
    "That's a pain. The tracking link in the order email has the last scan and the day it was due; if that day has gone or it has not moved for a day or two, the shop chases the courier - they open the case, not you.",
    "Sorry - that is annoying. The order email's tracking shows where it got to; once the promised day has gone, the shop can chase the courier or refund and replace it, so ask them rather than the courier.",
];
const DELIVERY_AGAIN_LINES: &[&str] = &[
    "Again? That's a pain. The tracking in the order email has the last scan; if it has not moved for a day or two, the shop is the one to chase the courier, and they can refund or replace it.",
    "Not again - sorry. Check the tracking in the order email first, then ask the shop to chase the courier: they open the case and can refund or replace it, and the courier cannot do that for you.",
];

fn record_in_the_question(said: &str) -> Option<&str> {
    if !(ASK.is_match(said) || STATE_AFTER.is_match(said) || BARE_STATE.is_match(said))
        || NOT_THE_STATE.is_match(said)
    {
        return None;
    }
    RECORD.captures_iter(said).find_map(|m| {
        let tail = &said[m.get(0)?.end()..];
        if QUALIFIED.find(tail).is_some_and(|part| part.start() == 0) {
            return None;
        }
        Some(m.name("thing")?.as_str())
    })
}

pub(in crate::bot) fn asks_about_delivery(said: &str) -> bool {
    ASK.is_match(said)
        && record_in_the_question(said).is_some_and(|thing| !thing.starts_with("library"))
        && DELIVERY_STATE.is_match(said)
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = reading.folded();
    let record = record_in_the_question(said)?;
    let pool = match record {
        "library card" | "library cards" if LOST.is_match(said) => LIBRARY_LOST_LINES,
        "library card" | "library cards" => LIBRARY_CARD_LINES,
        "library book" | "library books" | "library loan" | "library loans"
            if LOST.is_match(said) =>
        {
            LIBRARY_LOST_LINES
        }
        "library book" | "library books" | "library loan" | "library loans"
            if LATE.is_match(said) =>
        {
            LIBRARY_LATE_LINES
        }
        "library book" | "library books" | "library loan" | "library loans" => LIBRARY_LOAN_LINES,
        _ if !DELIVERY_STATE.is_match(said) => return None,
        _ if TROUBLE.is_match(said) && AGAIN.is_match(said) => DELIVERY_AGAIN_LINES,
        _ if TROUBLE.is_match(said) => DELIVERY_TROUBLE_LINES,
        _ => DELIVERY_LINES,
    };
    let last = memory.turns.last().map(|turn| turn.bot.as_str());
    Some(
        pool.iter()
            .copied()
            .find(|line| Some(*line) != last)
            .unwrap_or(pool[0])
            .into(),
    )
}
