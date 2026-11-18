//! Renewals, charges, pensions and substandard services.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

use super::{full_line, includes};

const CANCEL_WORDS: &[&str] = &[
    "cancel",
    "cancelled",
    "canceled",
    "cancelling",
    "canceling",
    "quit",
    "left it",
    "left the",
];
const TRIAL_WORDS: &[&str] = &[
    "free trial",
    "trial period",
    "a trial",
    "the trial",
    "signed up for a trial",
    "signed up to a trial",
    "free for a month",
    "free for 30 days",
];
const DELIVERY_FEE_WORDS: &[&str] = &[
    "delivery charge",
    "delivery fee",
    "charged for delivery",
    "free delivery",
    "postage charge",
    "shipping fee",
    "postage was free",
    "delivery was free",
];
const NEVER_HAPPENED_WORDS: &[&str] = &[
    "never booked",
    "didnt book",
    "didn't book",
    "never made",
    "didnt make",
    "didn't make",
    "never happened",
    "no show",
    "did turn up",
    "i was there",
    "never signed",
    "never asked",
    "didnt ask",
    "didn't ask",
    "never went",
    "didnt go",
    "didn't go",
    "cancelled in time",
    "canceled in time",
    "in advance",
    "with notice",
    "gave notice",
    "i cancelled",
    "i canceled",
    "i did cancel",
];
const OTHER_FEE_WORDS: &[&str] = &[
    "administration fee",
    "admin fee",
    "service fee",
    "processing fee",
    "convenience fee",
    "handling fee",
    "card fee",
    "transaction fee",
    "surcharge",
];
const CANCEL_FEE_WORDS: &[&str] = &[
    "cancellation fee",
    "cancellation charge",
    "cancelation fee",
    "cancelation charge",
    "no show fee",
    "no-show fee",
    "failure to appear",
];
const LATE_FEE_WORDS: &[&str] = &[
    "late fee",
    "late payment fee",
    "late charge",
    "late payment charge",
    "charged for being late",
    "fee for paying late",
];
const SERVICES_KIND: &[(&[&str], &[&str], &str)] = &[
    (
        &[
            "groomer", "groomers", "grooming", "dog", "dogs", "cat", "cats", "puppy", "pet",
            "pets", "vet",
        ],
        &[],
        "If an animal was injured, contact a vet promptly. Keep the vet's notes and receipts as evidence of what happened and what it cost.",
    ),
    (
        &[
            "garment", "garments", "suit", "suits", "dress", "dresses", "coat", "coats", "jacket",
            "jackets", "trousers", "tailor", "tailors",
        ],
        &[
            "dry cleaner",
            "dry cleaners",
            "dry cleaning",
            "dry-clean",
            "laundrette",
            "alterations",
        ],
        "Keep the garment, its care label and proof of its condition and value. Ask for the trader's proposal in writing if the damage cannot be put right.",
    ),
    (
        &[
            "repair",
            "repaired",
            "repairs",
            "screen",
            "watch",
            "laptop",
            "washer",
            "hoover",
            "lawnmower",
            "mower",
        ],
        &["repair shop", "phone repair", "the shop repaired"],
        "For a repair that made the fault worse, keep the repair invoice and photographs. An independent written assessment can help show what the repairer damaged.",
    ),
];
const SERVICES_HAIR_WORDS: &[&str] = &[
    "hair",
    "haircut",
    "haircutting",
    "barber",
    "salon",
    "nails",
    "nail",
    "manicure",
    "pedicure",
    "tattoo",
    "eyebrow",
    "eyebrows",
    "waxing",
    "waxed",
    "lashes",
    "cut",
    "trim",
    "colour",
    "dye",
];
const PAYMENT_MISSING: &[&str] = &[
    "not paid",
    "hasnt been paid",
    "hasn't been paid",
    "not been paid",
    "havent been paid",
    "haven't been paid",
    "havent had",
    "haven't had",
    "didnt get",
    "didn't get",
    "stopped",
    "short",
    "less than",
    "not receiving",
    "missing payment",
    "underpaid",
];
const OLD_PENSIONS: &[&str] = &[
    "old ones",
    "old one",
    "three old",
    "two old",
    "four old",
    "five old",
    "several old",
    "a few old",
    "several pensions",
    "multiple pensions",
    "a few pensions",
    "three pensions",
    "two pensions",
    "four pensions",
    "five pensions",
    "several pots",
    "several schemes",
    "a few schemes",
    "transfer my pensions",
    "move my pension",
];
const COMBINE_PENSIONS: &[&str] = &[
    "combine",
    "combining",
    "combined",
    "consolidate",
    "consolidating",
    "consolidated",
    "merge",
    "merging",
    "merged",
    "amalgamate",
    "amalgamating",
];

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static PENSION_COUNT_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:is|was) that (?:a lot|many|normal|usual|ok|okay|bad|too many|a problem)\b|\bhow many (?:is|are) (?:normal|usual|typical)\b|\bis (?:that|three) (?:a lot|normal|many)\b").unwrap()
});

fn has_word(said: &str, terms: &[&str]) -> bool {
    WORD.find_iter(said)
        .any(|word| terms.contains(&word.as_str()))
}

pub(super) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = full_line(reading);
    match id {
        "auto-renewal" => Some(Cow::Borrowed(renewed(reading))),
        "unexpected-fee" => Some(Cow::Borrowed(unexpected_fee(&said))),
        "booking-fee" => Some(fee_kind(&said)),
        "services-ruined" => Some(bad_service(&said)),
        _ => None,
    }
}

// https://www.gov.uk/guidance/writing-a-fair-contract-for-customers
fn renewed(reading: &Reading<'_>) -> &'static str {
    let said = format!("{} {}", reading.line.clean, reading.line.words.join(" ")).to_lowercase();
    if includes(&said, TRIAL_WORDS) {
        return "Check what the free-trial sign-up said about the first charge, the price and how to cancel. If the paid subscription or charge was not clearly disclosed, challenge it in writing and ask for a refund. Cancel through the provider and keep the confirmation. Some distance contracts have a 14-day cancellation period from the start, with exceptions. Ask your card provider about disputing any charge you did not authorise.";
    }
    if said
        .split_whitespace()
        .any(|word| CANCEL_WORDS.contains(&word))
    {
        return "If the payment covers a period after your cancellation took effect or was otherwise unauthorised, ask the company to refund it. Send the cancellation confirmation and ask which date it says your contract ended. If the charge continues, ask your bank or card provider about stopping the payment and disputing it. Check separately whether any contractual amount remains due.";
    }
    "Check what your contract said about auto-renewal, the notice period and how to cancel. Ask the company for the renewal terms and any reminder it sent. If the renewal was unclear or the term seems unfair, challenge the charge and ask to cancel in writing. Refund and cooling-off rights depend on the type of agreement and when it was made. If a card payment was unauthorised or the company will not resolve the dispute, ask the card provider about the options."
}

const UPFRONT: &str = "A trader must include mandatory fees in a UK headline price, or explain how to calculate charges that cannot reasonably be known in advance.";
const CANCEL_CHARGE: &str = "A cancellation or no-show fee should come from clear, fair terms and reflect the trader's reasonable loss after taking account of what it can save or recover.";

fn unexpected_fee(said: &str) -> &'static str {
    if includes(said, DELIVERY_FEE_WORDS) {
        return "If the seller promised free delivery, check the conditions and keep a copy of the offer and order confirmation. Ask it in writing to explain or refund the charge. Mandatory delivery costs generally have to be included in the total price where they can be calculated upfront. If you cannot resolve it, ask your card provider about disputing the payment or contact a consumer advice service.";
    }
    if includes(said, NEVER_HAPPENED_WORDS) {
        return "If you never booked or cancelled in line with the terms, dispute the fee with the trader. Give the cancellation date, any confirmation and the amount charged. A late cancellation or no-show charge can apply under fair terms, so ask the trader what term it relies on and how it calculated any loss. If the payment was unauthorised, contact your card provider promptly.";
    }
    "A UK trader generally has to show the total price including mandatory charges, or explain how to calculate any it cannot reasonably know. Save screenshots of the price you saw and the later fee. Ask the trader to explain or refund the difference. If it does not resolve it, contact your card provider or a consumer advice service about your options."
}

// https://www.gov.uk/government/publications/price-transparency-cma209/providing-clear-and-accurate-information-about-prices-summary
fn fee_kind(said: &str) -> Cow<'static, str> {
    if includes(said, LATE_FEE_WORDS) {
        return Cow::Borrowed(
            "A late fee is charged after the due date. Check the contract and any rules for the type of bill or credit. The fee should be clear and fair. Ask the company which term or rule allowed the charge and how it was calculated, then use its complaint process if you disagree.",
        );
    }
    if includes(said, CANCEL_FEE_WORDS) {
        return Cow::Owned(format!(
            "{CANCEL_CHARGE} Ask which term applies and how the trader calculated the charge. Challenge it if it was undisclosed or excessive."
        ));
    }
    if let Some(fee) = OTHER_FEE_WORDS.iter().find(|fee| said.contains(**fee)) {
        let article = if fee.starts_with(['a', 'e', 'i', 'o', 'u']) {
            "An"
        } else {
            "A"
        };
        return Cow::Owned(format!(
            "{article} {fee} is a charge for part of completing or handling the purchase. {UPFRONT} If it was only revealed later, keep the quote and ask the trader to explain or refund it."
        ));
    }
    if said.contains("cleaning fee") {
        return Cow::Owned(format!(
            "A cleaning fee can cover the cost of preparing accommodation for the next guest. {UPFRONT} If a charge was added after your stay, ask what it covers and what term or evidence supports it."
        ));
    }
    Cow::Owned(format!(
        "A booking fee is a charge for arranging the booking. {UPFRONT} Whether it is refundable depends on the booking and any statutory cancellation or refund rights. Ask to see the terms if a fee was hidden or charged for a booking you never made."
    ))
}

// https://www.legislation.gov.uk/ukpga/2015/15/part/1/chapter/4
fn bad_service(said: &str) -> Cow<'static, str> {
    const BASE: &str = "A UK trader must perform a service with reasonable care and skill. Tell it in writing what went wrong and what you want it to do. You can often ask it to put the work right within a reasonable time without significant inconvenience. If that is impossible, or the trader does not put it right in time, you may be entitled to a price reduction.";
    let detail = SERVICES_KIND.iter().find_map(|&(tokens, phrases, sentence)| {
        (has_word(said, tokens) || includes(said, phrases)).then_some(sentence)
    }).unwrap_or_else(|| {
        if has_word(said, SERVICES_HAIR_WORDS) {
            "Take photos and ask a second salon or professional for a written assessment and, if appropriate, a quote to put it right."
        } else {
            "Keep the receipt, photographs and any independent written assessment of the work."
        }
    });
    Cow::Owned(format!("{BASE} {detail}"))
}

// https://www.gov.uk/find-pension-contact-details
// https://www.gov.uk/check-benefits-state-pension-payments
pub(crate) fn pension(reading: &Reading<'_>, turns: &[Exchange]) -> Cow<'static, str> {
    let said = full_line(reading);
    if includes(&said, PAYMENT_MISSING) {
        if said.contains("state pension") {
            return Cow::Borrowed(
                "If your UK State Pension payment is missing or short, check your payment details and contact the Pension Service. Keep the dates, expected amount and any reference so they can investigate.",
            );
        }
        return Cow::Borrowed(
            "Contact the pension scheme or provider about a missing or short pension payment. For workplace contributions, check your payslip and ask your employer or payroll first. Keep the amounts, dates and written responses. If the scheme or employer does not resolve it, check the Pensions Ombudsman's complaint process and, for missing workplace contributions, the Pensions Regulator's reporting rules.",
        );
    }
    let asked = format!("{} {} {}", said, reading.line.norm_text, reading.line.plain);
    let count_followup = PENSION_COUNT_ASK.is_match(&asked)
        && turns
            .last()
            .is_some_and(|turn| includes(&turn.bot.to_lowercase(), &["old pots", "old pensions"]));
    if count_followup || includes(&said, OLD_PENSIONS) || has_word(&said, COMBINE_PENSIONS) {
        return Cow::Borrowed(
            "It is common to have old pension pots from different jobs. Use the free Pension Tracing Service to find scheme contact details if you need them. Ask each provider for a statement showing the value, charges and any guarantees. Before combining pots, check what you would give up and whether regulated financial advice is required for a transfer.",
        );
    }
    Cow::Borrowed(
        "Use the free GOV.UK Pension Tracing Service to find the contact details for a lost workplace or personal pension. Give the employer or provider name, then ask the scheme directly whether it holds a pension for you and request a statement. For the State Pension, use the GOV.UK forecast service or contact the Pension Service about an existing payment.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fees_renewals_and_pensions_use_the_relevant_branch() {
        assert!(unexpected_fee("delivery was free").contains("free delivery"));
        assert!(fee_kind("what is a late payment charge").starts_with("A late fee"));
        assert!(fee_kind("what is an admin fee").starts_with("An admin fee"));
        assert!(bad_service("the dog groomer cut my dog").contains("contact a vet"));
        assert!(
            renewed(&Reading::new("They charged me after a free trial")).contains("free-trial")
        );
        assert!(
            pension(&Reading::new("My State Pension has not been paid"), &[])
                .contains("Pension Service")
        );
        let previous = [Exchange {
            user: String::new(),
            bot: "You have several old pots.".into(),
            module: "",
        }];
        assert!(pension(&Reading::new("Is that normal?"), &previous).contains("Before combining"));
    }
}
