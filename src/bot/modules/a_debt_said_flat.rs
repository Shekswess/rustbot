//! Fallback for a borrowing stated without a direct question.

use std::sync::LazyLock;

use fancy_regex::Regex as Lookaround;
use regex::Regex;

use crate::bot::{Memory, Reading, compose};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(RATE, r"\b(\d{1,2}(?:\.\d{1,2})?)\s*(?:%|per ?cent|percent)");
pattern!(
    BORROWING,
    r"\b(?:credit cards?|store ?cards?|payday loan|overdraft|debts?|catalogue|klarna|bnpl|buy now pay later|loans?|cards?)\b|\b(?:my|the|on the|our|me) (?:visa|mastercard|amex|american express)\b|\b(?:visa|mastercard) card\b"
);
pattern!(
    JOINED,
    r"^(?:and|but|so|also|then|plus|or|what about|how about|same|still|anyway)\b"
);
pattern!(
    BALANCE,
    r"£\s?\d[\d,]*|\b\d[\d,]{2,7}\s?(?:pounds|quid|grand|k\b)?|\b\d+\s?(?:grand|k\b)"
);
pattern!(
    DONE,
    r"\b(?:paid off|paid up|paying off|cleared|clearing|settled|rid of|getting rid|sorted out|over with|done with)\b|\b(?:paid|pay|paying|settled)\b[^.?!]{0,20}\b(?:off|up)\b"
);
pattern!(
    EXCLUDED,
    r"\bstudent (?:loans?|debts?|finance|loan company)\b|\bplan [1245]\b|\bmaintenance loan\b|\bmortgages?\b"
);
pattern!(
    PAYMENT,
    r"\b\d[\d,]{1,7}\s?(?:pounds|quid|grand|k)?\s*(?:a|per|each) (?:month|week|year|fortnight)\b"
);
pattern!(
    DEBT_NAMED,
    r"\b(?:credit cards?|store ?cards?|overdraft|catalogue|klarna|bnpl|pay ?day loans?|buy now pay later|(?:a|my|the) loan|loans|debts?)\b|\b(?:my|the|our|on the|me) (?:visa|mastercard|amex|american express)\b|\b(?:visa|mastercard) card\b"
);
pattern!(
    HAVE,
    r"\b(?:got|have|has|owe|owes|i'?ve|we'?ve|they'?ve|there'?s)\b"
);
pattern!(
    THINKING,
    r"\b(?:spend|spending|spent|overspend|stop|again|keep|shouldn'?t|worried|scared|hate|stuck|struggling)\b"
);
pattern!(
    ASK,
    r"^(?:should|shall|can|could|would|do|does|did|is|are|was|were|what|how|which|when|where|who|why|worth|better|best)\b"
);
pattern!(STORE, r"\b(?:store ?cards?|catalogue)\b");
pattern!(PAY_LATER, r"\b(?:bnpl|klarna|buy now pay later)\b");
static GREETING_CARD: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(r"\b(?:birthday|christmas|xmas|thank[- ]?you|sympathy|get[- ]well|anniversary|leaving|wedding|new baby|congratulations|farewell) cards?\b|\bcards? (?:for|from|to) (?!my card|the card|pay)\w+|\b(?:bought|buy|buying|sent|send|sending|gave|give|giving|made|making|got|getting|picked up|picking up)\b(?: (?!on\b|off\b)\w+){0,3} (?:a|an|the|some) cards?\b").expect("card given as a gift")
});

const RATE_HIGH: &[&str] = &[
    "At {rate}%, check how much the borrowing costs against what your savings earn. After essentials and minimum payments, paying down the more expensive balance can save interest if you can keep an emergency buffer and avoid repayment fees. Roughly what is on it?",
    "If the {rate}% you quoted is higher than your savings rate, extra repayments could save you money. Keep enough for essentials and emergencies, and check for repayment fees. How much is left on it?",
    "The {rate}% matters alongside what you owe and any fees. After essential bills and minimum payments, extra money usually saves most interest on your highest-cost borrowing. How much is on it?",
];
const RATE_LOW: &[&str] = &[
    "At {rate}%, check whether the rate is fixed or promotional and what it becomes afterwards. Keep up with the minimum payments. Do you know when that rate ends?",
    "The question with {rate}% is how long that rate lasts and whether fees apply. Check the agreement for the rate after that. Is it fixed for a set time?",
];
const DEBT_BALANCE: &[&str] = &[
    "Check the rate on the balance and the minimum payment. After essentials, focusing spare money on the highest-cost debt can save interest if you have room in your budget and no early repayment fee. What rate are they charging?",
    "The balance tells you how much is left. The interest rate, any fees and what else you owe tell you where extra money would save most. What is the rate, and is this your only balance?",
    "Check the rate and whether it is higher than the rate on any other balance. Keep up with minimum payments and essential bills first. Is it just the one balance?",
];
const STORE_CARD: &[&str] = &[
    "Check the APR on the statement for the store card or catalogue. It can be expensive compared with other credit. How much is on it?",
    "The statement should give the interest rate and minimum payment. What is the rate, and how much is left on it?",
];
const BNPL: &[&str] = &[
    "Check when the buy now, pay later payment is due and what the agreement says about interest or fees if you miss it. When is the next payment?",
    "For buy now, pay later, check the instalment dates and when any interest-free period ends. Which agreement is it, and when is the next payment due?",
];
const NO_FIGURE: &[&str] = &[
    "Check the statement for the rate and the amount left on the borrowing. Do you know either?",
    "The balance and the interest and fees are the figures to look for. They should be on the statement or agreement. Which have you got?",
];

/// The source observes these lines without recording a fact.
pub(crate) fn observe(_reading: &Reading<'_>, _memory: &mut Memory) {}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading
        .raw
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if line.contains('?')
        || !BORROWING.is_match(&line)
        || DONE.is_match(&line)
        || JOINED.is_match(&line)
        || EXCLUDED.is_match(&line)
        || GREETING_CARD.is_match(&line).unwrap_or(false)
    {
        return None;
    }
    if let Some(rate) = RATE.captures(&line) {
        let percent = &rate[1];
        let lines = if percent.parse::<f64>().ok()? >= 8.0 {
            RATE_HIGH
        } else {
            RATE_LOW
        };
        return Some(compose::pick(lines, memory, &line).replace("{rate}", percent));
    }
    if ASK.is_match(&line) {
        return None;
    }
    if BALANCE.is_match(&line) {
        return (!PAYMENT.is_match(&line))
            .then(|| compose::pick(DEBT_BALANCE, memory, &line).to_owned());
    }
    if !DEBT_NAMED.is_match(&line) || !HAVE.is_match(&line) || THINKING.is_match(&line) {
        return None;
    }
    let lines = if PAY_LATER.is_match(&line) {
        BNPL
    } else if STORE.is_match(&line) {
        STORE_CARD
    } else {
        NO_FIGURE
    };
    Some(compose::pick(lines, memory, &line).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn flat_debt_handles_the_stated_rate_and_abstains_from_other_readings() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("I have £900 on the Visa at 24 percent");
        assert!(
            turn.candidates
                .iter()
                .any(|offer| { offer.module == "a_debt_said_flat" && offer.reply.contains("24%") })
        );
        let memory = Memory::default();
        let low = respond(&Reading::new("my Visa is at 4 percent"), &memory).unwrap();
        assert!(low.contains("4%"));
        for line in [
            "And what about the other card",
            "I paid off my card",
            "My student loan is at 9 percent",
            "I bought my mum a card and spent 200 pounds",
            "My loan payment is 240 a month",
        ] {
            assert!(respond(&Reading::new(line), &memory).is_none(), "{line}");
        }
    }
}
