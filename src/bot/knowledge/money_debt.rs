//! Repaying cards and personal debts, debt arrangements, and borrowing choices.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::money_savings::{comma, sums};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    HOW_LONG,
    r"\b(?:how long|how many years|when will|how quickly|how fast)\b"
);
pattern!(
    APR,
    r"\b(?:apr\s*(?:of\s+)?(\d+(?:\.\d+)?)\s*%?|(\d+(?:\.\d+)?)\s*%\s*(?:apr|interest))\b"
);
pattern!(
    PAYMENT,
    r"^\s*(?:pounds?\s+|quid\s+)?(?:a|per|each|every|/)\s*(month|week|fortnight|year)\b"
);
pattern!(
    PAYMENT_WORD,
    r"\b(?:pay(?:ing)?|repay(?:ing)?|payment|put aside|put by|pay off|pay down|overpay(?:ing)?)\b[^.?!]{0,35}$"
);
pattern!(
    REGULAR_INCOME,
    r"\b(?:income|earn|earned|salary|wage|paid|take home|make)\b[^.?!]{0,24}$"
);
pattern!(
    BORROW_MONEY,
    r"£|\d{2,}|\b(?:money|pounds|cash|loans?|bank|credit|grand|overdraft|apr|interest|euro|dollars|payday)\b"
);
pattern!(DRO, r"\b(?:debt relief orders?|debt relief|dros?)\b");
pattern!(TRUST_DEED, r"\b(?:protected trust deeds?|trust deeds?)\b");
pattern!(IVA, r"\b(?:ivas?|individual voluntary arrangements?)\b");
pattern!(DAS, r"\bdebt arrangement schemes?\b");
pattern!(DMP, r"\b(?:dmps?|debt management plans?|debt management)\b");
pattern!(ARRANGEMENT, r"\bdebt arrangements?\b");
pattern!(
    JUDGEMENT,
    r"\b(?:rude|ruder|politest|polite|impolite|cheeky|stingy|tight|mean|weird|strange|normal|ok|okay|acceptable|unacceptable|awkward|etiquette|manners|bad\s+form|selfish|generous|expected|allowed|supposed|better|worse|fine|wrong|right|fair|off|uncool|childish|petty|stingey)\b"
);
pattern!(
    TABLE,
    r"\b(?:bill|bills|tab|tabs|check|cheque|cost|costs|halves|half|dinner|lunch|meal|meals|restaurant|drinks|round|takeaway|takeout|coffee)\b"
);
pattern!(
    HOME,
    r"\b(?:rent|flats?|houses?|housemates?|flatmates?|roommates?|household|landlord|tenancy|mortgage|deposit|council\s+tax|service\s+charge|utilit(?:y|ies)|water|gas|electric|electricity|energy|heating|broadband|wifi|internet|tv\s+licen[cs]e|standing\s+order|moving\s+in|bills?\s+between\s+us)\b"
);

const FIRST_PERSON: &[&str] = &["i", "im", "ive", "i've", "we", "weve", "we're", "were"];
const THIRD_PERSON: &[&str] = &[
    "he",
    "she",
    "they",
    "him",
    "her",
    "them",
    "his",
    "their",
    "theirs",
    "hes",
    "shes",
    "theyll",
    "he's",
    "she's",
    "they'll",
    "it",
    "its",
    "it's",
    "mate",
    "mates",
    "friend",
    "friends",
    "brother",
    "brothers",
    "sister",
    "sisters",
    "mum",
    "mam",
    "dad",
    "mother",
    "father",
    "parents",
    "son",
    "daughter",
    "family",
    "colleague",
    "colleagues",
    "neighbour",
    "neighbours",
    "neighbor",
    "landlord",
    "ex",
    "partner",
    "husband",
    "wife",
    "cousin",
    "uncle",
    "aunt",
    "boss",
    "flatmate",
    "roommate",
    "housemate",
    "lad",
    "bloke",
];
const OWING: &[&str] = &[
    "owe",
    "owes",
    "owed",
    "owing",
    "borrow",
    "borrows",
    "borrowed",
    "borrowing",
];
const OWED: &[&str] = &["lend", "lends", "lent", "lending"];

const CARD_PLAN: &str = "Cover essential and priority bills first. Keep up minimum payments on each credit card where you can. Put any extra you can afford towards the highest interest rate. A balance transfer may reduce interest. Compare transfer fees and the rate after the offer, and check whether you could clear it by then. If you can't afford the payments, contact the lender and use MoneyHelper's free debt advice locator.";

fn targets(text: &str) -> (Option<f64>, Option<f64>) {
    let mut balance: Option<f64> = None;
    let mut payment = None;
    for (sum, span) in sums(text) {
        let next = &text[span.end()..];
        if next.trim_start().starts_with('%') {
            continue;
        }
        if let Some(period) = PAYMENT.captures(next) {
            let before = &text[..span.start()];
            if !REGULAR_INCOME.is_match(before) || PAYMENT_WORD.is_match(before) {
                let multiplier = match &period[1] {
                    "week" => 52.0 / 12.0,
                    "fortnight" => 26.0 / 12.0,
                    "year" => 1.0 / 12.0,
                    _ => 1.0,
                };
                payment.get_or_insert(sum * multiplier);
            }
        } else if sum > 0.0 && balance.is_none_or(|last| sum > last) {
            balance = Some(sum);
        }
    }
    (balance, payment)
}

fn payoff(balance: f64, payment: f64, apr: f64) -> Option<(u64, f64)> {
    let rate = apr / 1_200.0;
    if payment <= balance * rate || payment <= 0.0 {
        return None;
    }
    let periods = if rate == 0.0 {
        balance / payment
    } else {
        -(1.0 - rate * balance / payment).ln() / rate.ln_1p()
    };
    let months = periods.ceil().max(1.0) as u64;
    let regular = (months - 1) as f64;
    let last = if rate == 0.0 {
        balance - regular * payment
    } else {
        let growth = (1.0 + rate).powf(regular);
        (balance * growth - payment * (growth - 1.0) / rate) * (1.0 + rate)
    }
    .clamp(0.0, payment);
    Some((months, regular * payment + last))
}

fn card_payoff(text: &str) -> Cow<'static, str> {
    if !HOW_LONG.is_match(text) {
        return Cow::Borrowed(CARD_PLAN);
    }
    let apr = APR
        .captures(text)
        .and_then(|found| found.get(1).or_else(|| found.get(2)))
        .and_then(|found| found.as_str().parse::<f64>().ok())
        .filter(|apr| apr.is_finite() && *apr <= 1_000.0);
    let rate = apr.unwrap_or(20.0);
    let label = if apr.is_some() {
        "stated"
    } else {
        "illustrative"
    };
    let (balance, payment) = targets(text);
    let format = super::money_household::pounds;
    if let (Some(balance), Some(payment)) = (balance, payment) {
        if let Some((months, total)) = payoff(balance, payment, rate) {
            return Cow::Owned(format!(
                "A balance of {} at a {label} {rate}% APR with {} paid each month would take about {} months and cost about {} in total. This uses APR divided by 12 and assumes a fixed rate, no fees and no new spending. Check the rate and minimum on your statement.",
                format(balance),
                format(payment),
                comma(months),
                format(total)
            ));
        }
        return Cow::Owned(format!(
            "At a {label} {rate}% APR, {} a month would not cover the initial monthly interest on {}. Check the actual rate and minimum due. If you can't afford that payment, contact the lender or a free debt adviser before changing it.",
            format(payment),
            format(balance)
        ));
    }
    let balance = balance.unwrap_or(1_000.0);
    let example = (balance * 0.025).max(25.0);
    if let (Some((slow, _)), Some((fast, _))) = (
        payoff(balance, example, rate),
        payoff(balance, example * 2.0, rate),
    ) {
        return Cow::Owned(format!(
            "On {}, a {label} {rate}% APR gives about {} months at {} a month and {} months at {} a month. These examples use fixed payments and exclude fees or new spending. What balance, APR and monthly payment are on your statement?",
            format(balance),
            comma(slow),
            format(example),
            comma(fast),
            format(example * 2.0)
        ));
    }
    Cow::Borrowed(
        "The payoff date depends on the balance, APR and monthly payment. Check your statement and tell me those three figures. If the payment barely covers interest, ask the card provider what help is available.",
    )
}

fn debt_plans(text: &str) -> &'static str {
    let das = DAS.is_match(text);
    let plans = [
        (
            DRO.is_match(text),
            "In England and Wales a Debt Relief Order can clear eligible debts after 12 months if you qualify. It has no application fee and an approved debt adviser must apply for you. The usual limits include less than £50,000 debt and less than £75 a month spare income, with separate limits on assets and vehicles. It stays on your credit record for six years. Ask a free adviser about the rules in your UK nation.",
        ),
        (
            TRUST_DEED.is_match(text),
            "A protected trust deed in Scotland is a formal arrangement with a trustee. You usually pay what you can afford for around four years and included debts can be discharged if you complete the arrangement. It can affect your credit, assets and housing. Get free debt advice before signing.",
        ),
        (
            IVA.is_match(text),
            "An Individual Voluntary Arrangement is a formal agreement with creditors, usually arranged by an insolvency practitioner in England, Wales or Northern Ireland. If you complete the terms, remaining included debts can be written off. Fees, assets and employment can be affected, and details usually stay on your credit file for six years. Speak to a free debt adviser about the alternatives first.",
        ),
        (
            das,
            "Scotland's Debt Arrangement Scheme can let you repay debts over a longer period through an approved debt payment programme. It protects against certain enforcement action. Interest and charges are frozen and usually written off when the programme is completed. A money adviser can check eligibility and help you apply.",
        ),
        (
            DMP.is_match(text) || (!das && ARRANGEMENT.is_match(text)),
            "A Debt Management Plan is an informal agreement for paying non-priority debts at a rate you can afford after essentials. You may make one monthly payment shared between creditors. Creditors don't have to freeze interest or charges, and your credit file may be affected. Some free debt advice services arrange plans without fees.",
        ),
    ];
    let mut named = plans
        .iter()
        .filter_map(|(matched, reply)| matched.then_some(*reply));
    let first = named.next();
    if let Some(reply) = first
        && named.next().is_none()
    {
        return reply;
    }
    "A Debt Management Plan is informal. An IVA or protected trust deed is formal and can write off some debt if the terms are completed. Debt Relief Orders have eligibility limits and can clear certain debts. Scotland's Debt Arrangement Scheme generally provides more time to repay debts in full. Eligibility, protections and consequences depend on where you live. MoneyHelper can help you find free debt advice."
}

fn direction(reading: &Reading<'_>) -> Option<bool> {
    for clause in &reading.line.clauses {
        for (index, word) in clause.iter().map(String::as_str).enumerate() {
            if !OWED.contains(&word) && !OWING.contains(&word) {
                continue;
            }
            let subject = clause[index.saturating_sub(6)..index]
                .iter()
                .rev()
                .map(String::as_str)
                .find(|word| FIRST_PERSON.contains(word) || THIRD_PERSON.contains(word));
            if let Some(subject) = subject {
                return Some(FIRST_PERSON.contains(&subject) == OWED.contains(&word));
            }
        }
    }
    None
}

pub(crate) fn household_split(reading: &Reading<'_>) -> bool {
    let text = &reading.clean;
    HOME.is_match(text) || !(JUDGEMENT.is_match(text) && TABLE.is_match(text))
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.raw.to_lowercase();
    Some(match id {
        "credit-card-debt" => card_payoff(&text),
        "debt-plans" => Cow::Borrowed(debt_plans(&text)),
        "owe-family" if direction(reading) == Some(true) => Cow::Borrowed(
            "Ask plainly in writing and put a date on it. For example, “You said £200 by the end of the month. Can you pay on the 30th?” If money is tight for them, see if there's a smaller regular payment you can both agree to.",
        ),
        "owe-family" => Cow::Borrowed(
            "Tell them the amount and the date in one message. For example, “I owe you £600 and can pay £50 a week from Friday.” Check that the payment fits around essentials, and tell them before the date if you can't keep to it.",
        ),
        "borrow-how" if BORROW_MONEY.is_match(&text) => Cow::Borrowed(
            "Before borrowing, check what you can repay after essentials. Compare the APR, fees, total repayable and early repayment terms across a bank loan or credit union. Confirm that any eligibility check is a soft search before giving permission. If you're borrowing to pay bills or you've been refused credit, check grants or benefits and speak to a free debt adviser before taking on more.",
        ),
        "windfall-small" => {
            let (sum, rate) = targets(&text);
            if sum.is_some_and(|value| value < 300.0) || (sum.is_none() && rate.is_some()) {
                return None;
            }
            Cow::Borrowed(
                "Set aside essentials and deal with overdue priority bills first. Compare the cost of expensive debt with any early repayment charges, and keep some emergency money easy to access. If you get means-tested benefits, check how a lump sum affects them before you spend it. For longer-term money, think about when you'll need it and how much risk you can take. MoneyHelper explains the choices.",
            )
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debt_illustrations_use_the_last_partial_payment() {
        assert_eq!(payoff(1_050.0, 100.0, 0.0), Some((11, 1_050.0)));
        assert!(payoff(10_000.0, 100.0, 20.0).is_none());
        assert!(debt_plans("what is a debt relief order").contains("no application fee"));
    }
}
