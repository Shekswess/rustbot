//! Definition branches for gross and net pay.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::{bot::Reading, text::is_space};

use super::has_space_delimited_phrase;

static NUMBER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"[\p{Nd}\u{b2}\u{b3}\u{b9}\u{1369}-\u{1371}\u{19da}\u{2070}\u{2074}-\u{2079}\u{2080}-\u{2089}\u{2460}-\u{2468}\u{2474}-\u{247c}\u{2488}-\u{2490}\u{24ea}\u{24f5}-\u{24fd}\u{24ff}\u{2776}-\u{277e}\u{2780}-\u{2788}\u{278a}-\u{2792}\u{10a40}-\u{10a43}\u{10e60}-\u{10e68}\u{11052}-\u{1105a}\u{1f100}-\u{1f10a}]",
).expect("Unicode decimal digits and compatibility digits")
});

fn line_text<'a>(reading: &'a Reading<'_>) -> Cow<'a, str> {
    let line = &reading.line;
    let text = if line.clean.is_empty() {
        Cow::Owned(line.text.to_lowercase())
    } else {
        Cow::Borrowed(line.clean.as_str())
    };
    if text.bytes().all(|byte| {
        byte.is_ascii()
            && (!byte.is_ascii_whitespace() || byte == b' ')
            && !(0x1c..=0x1f).contains(&byte)
    }) && !text.contains("  ")
    {
        return text;
    }
    Cow::Owned(
        text.split(is_space)
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>()
            .join(" "),
    )
}

fn names(line: &str, terms: &[&str]) -> bool {
    terms
        .iter()
        .any(|term| has_space_delimited_phrase(line, term))
}

pub(super) fn about_pay(reading: &Reading<'_>) -> bool {
    let line = line_text(reading);
    let asks = |terms: &[&str]| names(&line, terms);
    if NUMBER.is_match(&line)
        && !asks(&[
            "include",
            "includes",
            "included",
            "include tax",
            "includes tax",
            "tax included",
            "before or after",
            "after or before",
        ])
    {
        return false;
    }
    let gross = asks(super::PAY_GROSS_WORDS);
    let net = asks(super::PAY_NET_WORDS);
    if gross && net {
        return true;
    }
    let tax = asks(&["tax", "taxed"]);
    if gross || net {
        return tax || asks(super::PAY_SUBJECT_WORDS);
    }
    tax && asks(super::PAY_QUOTED_WORDS)
        && asks(&[
            "include",
            "includes",
            "included",
            "taxed",
            "before",
            "after",
            "quoted",
            "advertised",
        ])
}

pub(super) fn gross_or_net(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = line_text(reading);
    let asks = |terms: &[&str]| names(&line, terms);
    let gross = asks(&[
        "gross",
        "gross pay",
        "gross salary",
        "gross income",
        "before tax",
        "pre tax",
        "pre-tax",
        "pretax",
    ]);
    let net = asks(&[
        "net",
        "net pay",
        "net income",
        "after tax",
        "take home",
        "take-home",
        "takehome",
        "take home pay",
        "net of tax",
    ]);
    const BOTH: &str = "Gross is what you earn before deductions. Net, or take-home pay, is what you receive after deductions such as Income Tax, National Insurance, pension contributions or student loan repayments. Check the gross and net figures on your payslip and use HMRC's current tax checker if the tax looks wrong.";
    let reply = if (gross && (net || asks(super::PAY_COMPARISON_WORDS)))
        || asks(&[
            "before or after",
            "after or before",
            "gross or net",
            "net or gross",
        ]) {
        BOTH
    } else if asks(super::PAY_INCLUDED_WORDS) {
        "An advertised salary is usually the gross figure, before deductions. Ask the employer to confirm it in writing if the advert is unclear. The take-home amount depends on your tax code, where you pay Income Tax and any other deductions."
    } else if net {
        "Net, or take-home pay, is what you receive after deductions. Gross is the amount before deductions. If your net pay looks wrong, check the tax code and the deductions listed on your payslip."
    } else if gross {
        "Gross pay is your pay before deductions such as Income Tax, National Insurance or pension contributions. Net pay is the amount you receive after those deductions."
    } else {
        BOTH
    };
    Some(Cow::Borrowed(reply))
}
