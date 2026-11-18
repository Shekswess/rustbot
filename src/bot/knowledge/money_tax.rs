//! UK pay estimates, gifts and inheritance tax, and suspicious prize messages.

use std::{borrow::Cow, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use regex::Regex;

use crate::bot::{Exchange, Reading};

use super::money_home::annual_income;
use super::money_savings::{comma, sums};

#[path = "money_tax_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    ABOUT_THE_WIN,
    ASKS_FOR_SOMETHING_BACK,
    A_WIN_NAMED,
    MONEY_LEFT_NAMED,
    NEVER_ENTERED,
    TAX_ASKED,
    BANK_DETAILS_ASKED,
    A_WIN_OR_AN_INHERITANCE,
    THE_ESTATE_BEING_WOUND_UP,
    MY_PAY
);

const WIN: &str =
    r"(?:won|win|winner|winnings|prize|prizes|jackpot|lottery|competition|sweepstake)";
const MEDIUM: &str = r"(?:text|texts|texted|message|messages|email|emails|letter|letters|call|phone call|voicemail|whatsapp|dm|notification)";
const MONEY_LEFT: &str = r"(?:inherit\w*|windfall|unclaimed|bequest|next of kin|beneficiary)";
static CLAIMS_WIN: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(&format!(
        r"(?is)\b{MEDIUM}\b(?:(?!{WIN}).){{0,60}}?\b{WIN}\b"
    ))
    .expect("message before win")
});
static CLAIMS_MONEY: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(&format!(
        r"(?is)\b{MEDIUM}\b(?:(?!{WIN}).){{0,60}}?\b{MONEY_LEFT}\b"
    ))
    .expect("message before inheritance")
});
static WIN_THEN_MESSAGE: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(&format!(
        r"(?is)\b{WIN}\b(?:(?!{MEDIUM}).){{0,60}}?\b{MEDIUM}\b"
    ))
    .expect("win before message")
});
static MONEY_OUT: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::MONEY_OUT).expect("prize payment request"));
static RELATIVE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?is)\b(?:(?:my|our)\s+)?(?:sister|brother|son|daughter|mum|mam|mother|dad|father|friend|mate|partner|husband|wife|colleague|girlfriend|boyfriend|cousin|nan|gran|granny|aunt|uncle|niece|nephew|neighbour|neighbor)\b.{0,40}?\b(?:texted|texts|messaged|messages|told|tells|called|calls|rang|rings|phoned|phones|wrote|whatsapped|emailed|emails)\b.{0,20}?\b(?:me|us)\b"
).expect("relative reported the prize")
});
pub(super) static SCOTLAND: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:scotland|scottish|s1257l|edinburgh|glasgow|aberdeen|dundee|inverness)\b")
        .expect("Scottish rates named")
});

const ALLOWANCE: f64 = 12_570.0;

pub(super) fn pay_breakdown(income: f64, scotland: bool) -> (f64, f64) {
    let allowance = (ALLOWANCE - (income - 100_000.0).max(0.0) / 2.0).max(0.0);
    let taxable = (income - allowance).max(0.0);
    // Band ceilings apply to income after allowances. The final band is open-ended.
    let bands: &[(f64, f64)] = if scotland {
        &[
            (3_967.0, 0.19),
            (16_956.0, 0.20),
            (31_092.0, 0.21),
            (62_430.0, 0.42),
            (125_140.0, 0.45),
            (f64::INFINITY, 0.48),
        ]
    } else {
        &[(37_700.0, 0.20), (125_140.0, 0.40), (f64::INFINITY, 0.45)]
    };
    let mut floor = 0.0;
    let mut tax = 0.0;
    for &(ceiling, rate) in bands {
        tax += (taxable.min(ceiling) - floor).max(0.0) * rate;
        floor = ceiling;
    }
    let ni =
        (income.min(50_270.0) - ALLOWANCE).max(0.0) * 0.08 + (income - 50_270.0).max(0.0) * 0.02;
    (tax, ni)
}

fn quoted_salary(text: &str) -> Option<u64> {
    annual_income(text).or_else(|| {
        sums(text)
            .map(|(value, _)| value as u64)
            .filter(|value| *value >= 3_000)
            .max()
    })
}

pub(crate) fn take_home(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let text = reading.raw.to_lowercase();
    let scotland = SCOTLAND.is_match(&text);
    let location = if scotland {
        "Scotland"
    } else {
        "England, Wales and Northern Ireland"
    };
    let income = quoted_salary(&text).or_else(|| {
        earlier
            .iter()
            .rev()
            .take(8)
            .filter_map(|turn| {
                let line = turn.user.to_lowercase();
                MY_PAY
                    .is_match(&line)
                    .then(|| annual_income(&line))
                    .flatten()
            })
            .next()
    });
    let Some(income) = income else {
        let bands = if scotland {
            "Scotland has six rates on employment income, from 19% to 48% after allowances."
        } else {
            "The standard Personal Allowance is £12,570, with employment income bands of 20%, 40% and 45% above it."
        };
        return Some(Cow::Owned(format!(
            "For 2026–27, {bands} Most employees pay National Insurance at 8% up to the upper earnings limit and 2% above. What's the gross annual salary?"
        )));
    };
    let (tax, ni) = pay_breakdown(income as f64, scotland);
    let net = (income as f64 - tax - ni) / 12.0;
    Some(Cow::Owned(format!(
        "On £{} gross a year, estimated monthly take-home is £{} in {location} for 2026–27. That uses about £{} in Income Tax and £{} in employee National Insurance a year. It assumes one job, the usual tax code and no pension, student loan or other payroll deductions. HMRC's take-home pay checker can use your own circumstances.",
        comma(income),
        comma(net.round() as u64),
        comma(tax.round() as u64),
        comma(ni.round() as u64)
    )))
}

fn suspicious_prize(text: &str) -> bool {
    if text.is_empty() {
        return true;
    }
    if ABOUT_THE_WIN.is_match(text) || RELATIVE.is_match(text) || TAX_ASKED.is_match(text) {
        return false;
    }
    if CLAIMS_WIN.is_match(text).unwrap_or(false)
        || CLAIMS_MONEY.is_match(text).unwrap_or(false)
        || NEVER_ENTERED.is_match(text)
    {
        return true;
    }
    if WIN_THEN_MESSAGE.is_match(text).unwrap_or(false) {
        return ASKS_FOR_SOMETHING_BACK.is_match(text);
    }
    (BANK_DETAILS_ASKED.is_match(text) && A_WIN_OR_AN_INHERITANCE.is_match(text))
        || ((A_WIN_NAMED.is_match(text) || MONEY_LEFT_NAMED.is_match(text))
            && MONEY_OUT.is_match(text).unwrap_or(false))
}

pub(crate) fn prize_claim(reading: &Reading<'_>) -> bool {
    suspicious_prize(&reading.clean)
}

fn prize_answer(text: &str) -> Cow<'static, str> {
    const STEPS: &str = "Send no money or personal details. Verify the organisation using contact details you find independently. Keep the message. If you've paid or disclosed banking details, contact your bank straight away. Report fraud at reportfraud.police.uk in England, Wales or Northern Ireland, or contact Police Scotland on 101. You can forward suspicious texts to 7726.";
    let lead = if NEVER_ENTERED.is_match(text) || text.contains("dont remember entering") {
        "An unexpected message about a competition you didn't enter is a strong sign of a scam."
    } else {
        "A message that asks you to pay a fee or hand over banking details before you get a prize is a strong sign of a scam."
    };
    Cow::Owned(format!("{lead} {STEPS}"))
}

const GIFT_RULES: &str = "Some gifts can affect Inheritance Tax if the giver dies within seven years. The annual exemption is £3,000 in total. The £250 small-gift allowance applies per person if no other allowance was used for them that tax year. Gifts from regular income may also qualify if they leave the giver enough for their usual living costs. Keep a record of gifts and check GOV.UK for your circumstances.";
const INHERITANCE: &str = "The standard UK Inheritance Tax threshold is £325,000. An estate can usually use any unused threshold from a deceased spouse or civil partner. A qualifying home left to direct descendants can add up to £175,000, with a taper for estates above £2 million. Inheritance Tax is generally 40% on the part of the estate above the available threshold. Gifts can affect the calculation, especially within seven years of death or if the giver still benefits from the gift. Check GOV.UK for exemptions and who must pay.";

fn gift(text: &str) -> Cow<'static, str> {
    let clean = format!(
        " {} ",
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    let lead = if data::GIFT_FROM_WORK
        .iter()
        .any(|phrase| clean.contains(phrase))
    {
        "An employer gift may be taxable. A trivial benefit can be exempt if it costs £50 or less, is neither cash nor a cash voucher, and is outside your contract and unrelated to work performance. Ask payroll how yours is treated."
    } else if data::GIFT_ARRIVES
        .iter()
        .any(|phrase| clean.contains(phrase))
    {
        "A genuine cash gift from another person is generally outside Income Tax when you receive it. Interest you earn on it later can be taxable."
    } else {
        "Giving someone cash generally does not trigger an immediate UK gift tax. Gifts of assets can have other tax consequences, including Capital Gains Tax."
    };
    Cow::Owned(format!("{lead} {GIFT_RULES}"))
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.clean;
    Some(match id {
        "prize-that-wants-a-fee" => prize_answer(text),
        "inheritance-tax" | "tax-on-a-gift" => {
            if !text.is_empty()
                && !THE_ESTATE_BEING_WOUND_UP.is_match(text)
                && suspicious_prize(text)
            {
                prize_answer(text)
            } else if id == "inheritance-tax" {
                Cow::Borrowed(INHERITANCE)
            } else {
                gift(text)
            }
        }
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tax_and_prize_checks_use_current_bands_and_the_request_shape() {
        let (tax, ni) = pay_breakdown(150_000.0, false);
        assert_eq!(tax.round(), 53_703.0);
        assert_eq!(ni.round(), 5_011.0);
        let (scottish_tax, _) = pay_breakdown(45_000.0, true);
        assert!(scottish_tax > pay_breakdown(45_000.0, false).0);
        assert!(suspicious_prize(
            "a letter says i won a prize and must pay a fee"
        ));
        assert!(!suspicious_prize(
            "how much inheritance tax on a £500,000 estate"
        ));
    }
}
