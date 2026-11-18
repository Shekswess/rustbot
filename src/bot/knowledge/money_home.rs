//! First-home steps, mortgage illustrations and residential purchase tax.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::Reading;

use super::money_savings::{comma, sums};

#[path = "money_property_tax.rs"]
mod property_tax;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    INCOME_BEFORE,
    r"(?i)(?:\bearn\w*|\bsalar\w*|\bwage\w*|\bincomes?\b|\btake home\b|(?:(?:am|is|are|was|were|been|get|gets|got|getting)\s+paid)|\bbring(?:ing)? in\b|\bmake\b|\bgot a job\b|\b(?:i|we|he|she|they)(?:'?m|'?re|'?s)?\s+(?:am|is|are)?\s*on)(?:\s+(?:is|was|of|at|around|about|roughly|approx(?:imately)?|now|still))?\s*[£$€]?\s*$"
);
pattern!(
    INCOME_AFTER,
    r"(?i)^\s*(?:(?:a|per|each|every)\s*(?:year|annum)|pa\b|p\.a\.|gross|before tax|after tax)"
);
pattern!(
    INCOME_ON_K,
    r"(?i)(?:\band|\bthen|\balso|,|;)\s+on\s*[£$€]?\s*$"
);
pattern!(
    PRICE_NEAR,
    r"(?i)house|home|flat|property|price|deposit|cost|worth|asking|valued|selling|market|rent|mortgage|\bpaid\b"
);
pattern!(CLAUSE, r"(?i)[\n.;,]|\band\b|\bbut\b");
pattern!(
    PER_MONTH,
    r"(?i)^\s*(?:(?:a|per|each|every|/)\s*month|monthly|pcm)\b"
);
pattern!(
    OTHER_RATE,
    r"(?i)^\s*(?:(?:a|per|each|every|/)\s*(?:day|week|fortnight|year)|weekly|annually)\b"
);
pattern!(
    HOUSE_JOB,
    r"\b(?:clean(?:s|ing|ed)?|tidy|tidies|tidying|tidied|hoover(?:s|ing)?|vacuum(?:s|ing)?|dust(?:ing)?|scrub(?:bing)?|mop(?:ping)?|bleach(?:ing)?|declutter(?:ing)?|sort(?:ing)? out|fix(?:ing)?|repair(?:s|ing)?|unpack(?:ing)?|move|moving in|moved in|heat(?:ing)?|insulate|insulation|smell(?:s|y)?|damp|mould|mold|garden|lawn|fences?|bins?|painting|repaint(?:ing)?)\b"
);
pattern!(
    BUYING_WORD,
    r"\b(?:deposits?|sav(?:e[sd]?|ing|ings)|afford(?:s|able)?|buy(?:s|ing)?|bought|purchase|purchasing|prices?|costs?|mortgages?|worth|lenders?|loans?|how much|stamp duty|offer|sold|sell(?:ing)?|estate agent|viewings?)\b"
);

static STILL_SAVING: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        "how do i start",
        "how to start",
        "where do i start",
        "start buying",
        "want to buy",
        "want to get on",
        "save up",
        "saving up",
        "can i afford",
        "afford a house",
        "afford a flat",
        "what do i need",
        "get on the ladder",
        "thinking about buying",
        "planning to buy",
        "hoping to buy",
        "in a few years",
        "eventually",
        "one day",
    ])
    .expect("preparing to buy")
});
static ALREADY_BUYING: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        "buying a house",
        "buying a flat",
        "buying a home",
        "buying my first",
        "buying our first",
        "in the process of buying",
        "had an offer accepted",
        "offer accepted",
        "offer has been accepted",
        "we've had an offer accepted",
        "mortgage application",
        "applying for a mortgage",
        "just had my offer",
        "just had our offer",
        "got a mortgage",
    ])
    .expect("already buying")
});

const LISA: &str = "If you're eligible, a Lifetime ISA adds 25% to up to £4,000 of savings each tax year. Check GOV.UK before opening one. A charge-free first-home withdrawal has conditions, including a price of £450,000 or less and at least 12 months since your first contribution.";
const AFFORDABILITY: &str = "A rough borrowing ceiling some lenders use is 4.5 times gross annual income. On £30,000 that's £135,000 of borrowing, with a deposit on top. What you're offered depends on income, spending, debts, credit history and the lender. Use MoneyHelper's mortgage affordability calculator before viewing.";
const OFFER: &str = "A decision in principle is an estimate of what a lender may offer. Line up a solicitor or conveyancer and ask for a written quote, then budget separately for a survey, any purchase tax, mortgage fees and moving. In Scotland, take advice from your solicitor before offering, as the process differs from the rest of the UK.";

fn as_income(text: &str, span: regex::Match<'_>) -> bool {
    let before = &text[..span.start()];
    let after = &text[span.end()..];
    if INCOME_BEFORE.is_match(before) || INCOME_AFTER.is_match(after) {
        return true;
    }
    span.as_str().trim_end().ends_with(['k', 'K'])
        && INCOME_ON_K.is_match(before)
        && !PRICE_NEAR.is_match(CLAUSE.split(before).last().unwrap_or(before))
}

pub(super) fn annual_income(text: &str) -> Option<u64> {
    sums(text)
        .filter(|(_, span)| as_income(text, *span))
        .map(|(value, span)| {
            let is_monthly = PER_MONTH.is_match(&text[span.end()..]);
            (value * if is_monthly { 12.0 } else { 1.0 }) as u64
        })
        .filter(|&value| value > 0)
        .max()
}

pub(super) fn price(text: &str) -> u64 {
    sums(text)
        .filter(|(_, span)| {
            !as_income(text, *span)
                && !PER_MONTH.is_match(&text[span.end()..])
                && !OTHER_RATE.is_match(&text[span.end()..])
        })
        .map(|(value, _)| value as u64)
        .max()
        .unwrap_or(0)
}

fn first_steps(text: &str) -> Cow<'static, str> {
    if !STILL_SAVING.is_match(text) && ALREADY_BUYING.is_match(text) {
        return Cow::Borrowed(OFFER);
    }
    Cow::Owned(format!(
        "Estimate the price range you can afford and set a deposit goal. A 10% deposit on a £200,000 home is £20,000, about £334 a month for five years before interest or bonuses. Budget separately for fees and moving costs. {LISA}"
    ))
}

pub(crate) fn buying_a_home(reading: &Reading<'_>) -> bool {
    let text = &reading.clean;
    !HOUSE_JOB.is_match(text) || BUYING_WORD.is_match(text)
}

fn deposit(text: &str) -> Option<Cow<'static, str>> {
    let price = price(text);
    if price >= 20_000 {
        return Some(Cow::Owned(format!(
            "On a £{} home, a 10% deposit is £{} and 5% is £{}. A larger deposit can improve the rates available. Check which mortgages you're eligible for and keep purchase costs separate. {LISA}",
            comma(price),
            comma(price / 10),
            comma(price / 20)
        )));
    }
    if price >= 1_000 {
        return None;
    }
    if let Some(pay) = annual_income(text) {
        let high = pay * 9 / 2;
        return Some(Cow::Owned(format!(
            "On a gross salary of £{}, 4 to 4.5 times pay is about £{} to £{} of borrowing. That's an illustration. The deposit is usually 5% to 10% or more of the property price, so work out the price you can afford after other costs. {LISA}",
            comma(pay),
            comma(pay * 4),
            comma(high)
        )));
    }
    Some(Cow::Owned(format!(
        "Most mortgages ask for 5% to 10% or more of the property price. On a £200,000 home that's £10,000 to £20,000. A larger deposit can improve the rates available. {LISA}"
    )))
}

fn supported_loan(payment: u64, annual_rate: f64, years: u16) -> u64 {
    let rate = annual_rate / 12.0;
    (payment as f64 * (1.0 - (1.0 + rate).powi(-12 * i32::from(years))) / rate).round() as u64
}

fn required_payment(loan: u64, annual_rate: f64, years: u16) -> u64 {
    let rate = annual_rate / 12.0;
    (loan as f64 * rate / (1.0 - (1.0 + rate).powi(-12 * i32::from(years)))).round() as u64
}

fn mortgage_afford(text: &str) -> Option<Cow<'static, str>> {
    let income = annual_income(text);
    let payment = sums(text)
        .find(|(_, span)| PER_MONTH.is_match(&text[span.end()..]) && !as_income(text, *span))
        .map(|(value, _)| value as u64)
        .filter(|&value| value > 0);
    let answer = match (income, payment) {
        (None, None) if text.contains("afford") => return Some(Cow::Borrowed(AFFORDABILITY)),
        (None, None) => return None,
        (Some(income), None) => {
            let cap = income * 9 / 2;
            let wanted = price(text);
            if wanted < 20_000 {
                return Some(Cow::Owned(format!(
                    "On £{} gross a year, 4.5 times income is £{} of borrowing. Your deposit, outgoings, debts and credit record affect what the lender offers. Use MoneyHelper's mortgage affordability calculator to test monthly payments and costs.",
                    comma(income),
                    comma(cap)
                )));
            }
            let position = if wanted > cap { "above" } else { "within" };
            format!(
                "A £{} mortgage is {position} the rough £{} borrowing figure from 4.5 times a £{} annual income. Actual offers depend on the lender, deposit, credit record and spending. If £{} is the property price, subtract your deposit to find the amount you need to borrow.",
                comma(wanted),
                comma(cap),
                comma(income),
                comma(wanted)
            )
        }
        (None, Some(payment)) => {
            format!(
                "£{} a month would repay roughly £{} over 25 years at an illustrative 5% interest rate. Check the monthly payment against your take-home pay, bills, debts and emergency costs, and see how the payment changes if rates go up.",
                comma(payment),
                comma(supported_loan(payment, 0.05, 25))
            )
        }
        (Some(income), Some(payment)) => {
            let cap = income * 9 / 2;
            let support = supported_loan(payment, 0.05, 30);
            let loan = cap.min(support);
            format!(
                "At £{} a month, an illustrative 5% rate over 30 years supports about £{} of borrowing. A rough 4.5-times-income ceiling on £{} gross is £{}. Using the lower figure, £{} over 30 years at 4.5% would cost about £{} a month. Allow room for bills, fees and changes in interest rates.",
                comma(payment),
                comma(support),
                comma(income),
                comma(cap),
                comma(loan),
                comma(required_payment(loan, 0.045, 30))
            )
        }
    };
    Some(Cow::Owned(answer))
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let raw = reading.raw.to_lowercase();
    match id {
        "first-home-steps" => Some(first_steps(&reading.clean)),
        "house-deposit" => deposit(&raw),
        "mortgage-afford" => mortgage_afford(&raw),
        "stamp-duty" => Some(property_tax::reply(&raw)),
        _ => None,
    }
}
