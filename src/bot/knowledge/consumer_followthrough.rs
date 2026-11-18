//! Description, return conditions, deposits and business complaints.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::{REFUND_OR_REPAIR, data, full_line, includes, joined_words};

static HUNG_UP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:put|puts|putting)\s+the\s+phone\s+down\b|\bhung up on (?:me|him)\b").unwrap()
});
static SELLING_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:shop|store|seller|retailer|company|order|orders|ordered|refund|refunded|delivery|delivered|bought|buy|purchase|purchased|item|product|warranty|guarantee|receipt|invoice|customer service|call centre|call center|supplier|marketplace|app)\b").unwrap()
});

fn padded(said: &str) -> String {
    format!(
        " {} ",
        said.split_whitespace().collect::<Vec<_>>().join(" ")
    )
}

fn phrases(said: &str, words: &[&str]) -> bool {
    words.iter().any(|word| said.contains(&format!(" {word} ")))
}

pub(super) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    match id {
        "not-as-described" => {
            reading
                .line
                .words
                .iter()
                .any(|word| data::DESCRIBED_WORDS.contains(&word.as_str()))
                || includes(&full_line(reading), data::DESCRIBED_PHRASES)
        }
        "baggage-lost" => baggage_named(&joined_words(reading)),
        _ => false,
    }
}

fn baggage_named(said: &str) -> bool {
    let said = padded(&said.replace([',', '.'], " "));
    phrases(&said, data::TRAVEL_SAID)
        || phrases(&said, data::CAME_OFF_THE_BELT)
        || includes(&said, &["baggage", "luggage"])
        || phrases(&said, data::GONE_MISSING)
}

pub(super) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "not-as-described" => Some(described(&full_line(reading))),
        "worn-return" => {
            let said = if reading.line.clean.is_empty() {
                &reading.line.text
            } else {
                &reading.line.clean
            };
            Some(worn_return(&padded(&said.to_lowercase())))
        }
        "holding-deposit" => Some(holding_deposit(reading)),
        "seller-ignoring" => seller_ignoring(&full_line(reading)),
        // https://www.caa.co.uk/air-passengers/about-your-trip/baggage/lost-delayed-or-damaged-baggage/
        "baggage-lost" => Some(Cow::Borrowed(
            "Tell the airline's baggage desk as soon as you can and keep the Property Irregularity Report, luggage tags and photos. Submit a separate written claim to the airline. Report damage within 7 days of receiving the bag and delay within 21 days of getting it back. For a missing bag, start the claim promptly. Keep receipts for reasonable essentials and evidence of the bag and contents' value. What you can recover depends on your loss and the applicable liability rules.",
        )),
        _ => None,
    }
}

fn described(said: &str) -> Cow<'static, str> {
    let subject = if includes(said, data::WAS_USED_WORDS) {
        "An item sold as new should match that description. If a used, refurbished or display model arrived instead, keep the listing and take photos of its condition, the packaging and serial number."
    } else {
        "Goods should match the seller's description. Keep the listing or label and take photos showing how the actual material, colour or item differs."
    };
    Cow::Owned(format!(
        "{subject} Report the mismatch to the retailer in writing and say which remedy you want. {REFUND_OR_REPAIR}"
    ))
}

fn worn_return(said: &str) -> Cow<'static, str> {
    let has = |words: &[&str]| phrases(said, words);
    let opening = if (has(&["tag", "tags", "label", "labels", "tagged"])
        && has(&[
            "without", "no", "missing", "lost", "off", "ripped", "removed", "cut",
        ]))
        || (has(&["tag", "tags"]) && !has(&["worn", "wore", "slept"]))
    {
        "Missing tags do not by themselves cancel your statutory rights for faulty goods. The shop may require tags for a voluntary in-store return."
    } else if has(&[
        "opened",
        "opening",
        "unopened",
        "box",
        "packaging",
        "packet",
        "seal",
        "sealed",
        "unwrapped",
        "wrapper",
        "shrink wrap",
        "shrinkwrap",
    ]) {
        "Opening the packaging to inspect a product usually does not take away your rights. Some sealed goods have specific cancellation exceptions once opened, including qualifying hygiene products."
    } else if has(&["slept", "mattress", "bed", "duvet"]) {
        "For a mattress or bedding, check the retailer's trial and returns terms. Sleeping on it can go beyond the inspection permitted for an online cancellation. If the item is faulty, the statutory remedies still apply."
    } else {
        "Wearing something does not by itself remove your rights if it was faulty. For a change of mind, taking it out for the evening can go beyond simply trying it on."
    };
    Cow::Owned(format!(
        "{opening} For an eligible UK distance purchase, you normally have 14 days after delivery to tell the retailer you are cancelling and another 14 days to send it back. The retailer may deduct for value lost through handling beyond what is necessary to check it. For an in-store change of mind, check the shop's policy."
    ))
}

// https://www.gov.uk/guidance/tenant-fees-act-2019-guidance-for-tenants
fn holding_deposit(reading: &Reading<'_>) -> Cow<'static, str> {
    const RULE: &str = "In England, a holding deposit reserves a rental while checks are made and is capped at one week's rent. The default agreement deadline is 15 days unless you agree a different period in writing. If repayment is due, it is generally due within seven days of the agreement, the landlord deciding not to proceed or the deadline passing, as applicable.";
    let said = padded(
        &format!(
            "{} {} {}",
            reading.line.clean, reading.line.text, reading.line.norm_text
        )
        .to_lowercase(),
    );
    if phrases(
        &said,
        &[
            "keep",
            "keeps",
            "kept",
            "keeping",
            "withholding",
            "refuse",
            "not back",
            "not return",
            "won't return",
        ],
    ) {
        return Cow::Owned(format!(
            "{RULE} Retention is allowed only on specified grounds, such as withdrawal, a failed Right to Rent check, materially misleading information or not taking reasonable steps to progress the tenancy. Ask for the reason in writing. You can challenge it with the local council or the First-tier Tribunal."
        ));
    }
    Cow::Owned(format!(
        "{RULE} If the tenancy goes ahead, you can agree to put the holding deposit towards the first rent payment or the tenancy deposit."
    ))
}

// https://www.financial-ombudsman.org.uk/consumers/complaints-can-help/credit-borrowing-money/goods-services-bought-credit
fn seller_ignoring(said: &str) -> Option<Cow<'static, str>> {
    if HUNG_UP.is_match(said) && !SELLING_WORDS.is_match(said) {
        return None;
    }
    let base = "Send a written complaint to the retailer with your order details, evidence, what went wrong and the remedy you want. Give a reasonable response deadline. If it still ignores you, ask your card provider about chargeback or, for a qualifying credit purchase costing over £100 and up to £30,000, Section 75. Ask about deadlines and keep your correspondence.";
    let said = padded(&said.replace([',', '.', '!', '?'], " "));
    if phrases(&said, data::PHONE_WORDS) {
        return Some(Cow::Owned(format!(
            "{base} Keep a call log with dates, contact names and what was said."
        )));
    }
    Some(Cow::Borrowed(base))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complaints_follow_the_description_deposit_or_airline_path() {
        assert!(!passes(
            "not-as-described",
            &Reading::new("He said his phone died.")
        ));
        let phone = Reading::new("They sent me a refurbished phone sold as new.");
        assert!(passes("not-as-described", &phone));
        assert!(
            reply("not-as-described", &phone)
                .unwrap()
                .contains("refurbished")
        );
        assert!(worn_return(" i opened the box ").contains("qualifying hygiene"));
        assert!(
            holding_deposit(&Reading::new("Can the agent keep my holding deposit?"))
                .contains("First-tier Tribunal")
        );
        assert!(seller_ignoring("She hung up on me.").is_none());
        assert!(
            seller_ignoring("The retailer hung up on me.")
                .unwrap()
                .contains("call log")
        );
        assert!(!baggage_named("the zip broke on my bag"));
        assert!(baggage_named("the bag came out torn at the airport"));
    }
}
