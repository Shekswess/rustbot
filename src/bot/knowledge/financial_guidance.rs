//! UK guidance on the source pension and missed-payment reply branches.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::Reading;

const PENSION_SAFE: &[&str] = &[
    "guaranteed",
    "guarantee",
    "safe",
    "protected",
    "secure",
    "at risk",
    "risky",
    "gone",
    "lost",
    "goes bust",
    "went bust",
    "go bust",
    "company fails",
    "collapse",
    "collapsed",
    "scam",
    "lose",
    "losing",
];
const PAYMENT_ALREADY_MISSED: &[&str] = &[
    "missed",
    "i missed",
    "ive missed",
    "i've missed",
    "paid late",
    "payed late",
    "paid it late",
    "was late",
    "is late",
    "went past",
    "overdue",
    "forgot",
    "forgotten",
    "didnt pay",
    "didn't pay",
    "havent paid",
    "haven't paid",
    "not paid",
    "bounced",
    "bounced back",
    "came back",
    "behind on",
    "in arrears",
    "defaulted",
    "default",
    "already missed",
    "went unpaid",
    "unpaid",
];
static PENSION_ACCESS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:take|taken|taking|access|cash in|withdraw|withdrawn|draw|early|lump sum|25%|quarter|before i retire|when can i|how early)\b")
        .expect("the pinned pension access pattern is valid")
});

fn contains_any(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

pub(super) fn pension(reading: &Reading<'_>) -> &'static str {
    let line = &reading.line;
    let text = if !line.clean.is_empty() {
        line.clean.as_str()
    } else {
        &line.text
    };
    if contains_any(text, PENSION_SAFE) {
        return "If your employer becomes insolvent, the Pension Protection Fund can assess an eligible defined-benefit scheme for protection. A defined-contribution pot is usually held separately from the employer and can rise or fall with its investments. Ask your provider or scheme trustees which protections apply. Use contact details from your statement if someone approaches you about moving your pension.";
    }
    if PENSION_ACCESS.is_match(text) {
        return "In the UK, the normal minimum age for taking a private pension is usually 55 and rises to 57 on 6 April 2028. Your scheme may set a later age and exceptions can apply. You can usually take up to 25% tax-free, subject to your lump sum allowance. Ask your provider about your options and the tax implications before taking money.";
    }
    "For most UK automatic enrolment schemes the minimum is 8% of qualifying earnings, with your employer paying at least 3%. The rest usually includes your contribution and any tax relief. Your scheme may use a different basis or higher contributions. Check your scheme rules, what your employer pays and what you would give up before deciding whether to opt out."
}

pub(super) fn missed_payment(reading: &Reading<'_>) -> &'static str {
    let said = reading.line.words.join(" ");
    if contains_any(&said, &["insurance", "policy"]) {
        return "Contact the insurer today to check whether the policy is still active and ask how to bring payments up to date. Check any notice from the insurer and ask about changing the payment date or arranging support. If it's car insurance, confirm you have valid cover before driving.";
    }
    if said.contains("mortgage") {
        if contains_any(
            &said,
            &[
                "application",
                "apply",
                "applying",
                "get a mortgage",
                "getting a mortgage",
                "lender",
                "afford",
            ],
        ) {
            return "How a missed payment affects a UK mortgage application depends on the lender and your credit history. Check your reports for errors and ask a regulated mortgage adviser what the missed payment is likely to mean for your options.";
        }
        return "Contact your mortgage lender now to discuss the missed payment and any charges. Explain what you can afford and ask about repayment options and how an arrangement may affect your credit file. If you are already in arrears, MoneyHelper can help you find free debt advice.";
    }
    if contains_any(&said, &["refund", "take it back", "get it back"]) {
        return "Ask the provider to explain the charge and whether it can refund it. Tell them if it was charged in error or if you're having money problems, and keep their answer in writing. If you think the fee is unfair and the provider won't resolve it, ask for its complaints process.";
    }
    if contains_any(&said, PAYMENT_ALREADY_MISSED) {
        return "Contact the lender as soon as you can. Ask what is overdue, whether a late charge or interest applies, and what they report to credit reference agencies. If you can't catch up, explain what you can afford to repay and get any agreement in writing.";
    }
    "Contact the lender before the due date if you may miss a payment. Ask about changing the date or arranging an affordable payment. Fees, interest and credit reporting depend on your agreement and the option you take. MoneyHelper can help you find free debt advice."
}

pub(super) fn scam_recovery(reading: &Reading<'_>) -> &'static str {
    let text = &reading.line.clean;
    let lost_money = contains_any(
        text,
        &[
            "money back",
            "get it back",
            "refund",
            "recover",
            "reclaim",
            "lost",
            "stolen",
            "took",
            "taken",
        ],
    );
    if !lost_money
        && contains_any(
            text,
            &[
                "report",
                "reporting",
                "action fraud",
                "police",
                "tell someone",
                "where do i",
            ],
        )
    {
        return "Contact Report Fraud at reportfraud.police.uk in England, Wales or Northern Ireland. In Scotland, contact Police Scotland. Keep the messages and payment references. If money was taken or someone may have access to your account, contact your bank or payment provider immediately.";
    }
    "Contact your bank or payment provider immediately and ask if it can stop the payment or reimburse you. Eligibility depends on how you paid and the circumstances. Keep the messages and payment references, and report it to Report Fraud at reportfraud.police.uk, or to Police Scotland if you're in Scotland. Avoid anyone asking you to pay a fee to recover the money."
}
