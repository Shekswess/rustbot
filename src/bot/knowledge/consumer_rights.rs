//! UK consumer purchase and return responses with the source subject checks.

use std::{borrow::Cow, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use regex::Regex;

use crate::bot::Reading;

use super::{kitchen, updated_guidance::REFUND_OR_REPAIR};

#[path = "consumer_rights_data.rs"]
mod data;
#[path = "consumer_followthrough.rs"]
mod followthrough;
#[path = "consumer_services.rs"]
mod services;
pub(crate) use services::pension;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    A_MARK_ON_ANYTHING,
    IT_ARRIVED,
    A_MARK_THAT_CAME_WITH_IT,
    A_HOW_TO_ASK,
    A_REPAIR_OR_REPLACE,
    GIVES_UP,
    RENTED_HOME,
    FITTED_TO_THE_FLAT,
    A_BICYCLE_CABLE,
    A_REPAIR_ASK,
    A_SHOP_CLAIM,
    A_STAKE_IN_A_COMPANY,
);
static A_BUYING_LINE: LazyLock<Lookaround> =
    LazyLock::new(|| Lookaround::new(data::A_BUYING_LINE).expect("authored purchase cue"));

pub(crate) fn passes(id: &str, reading: &Reading<'_>) -> bool {
    match id {
        "faulty-goods" => faulty_goods(reading),
        "delivery-promise" => !A_STAKE_IN_A_COMPANY.is_match(&actual_line(reading)),
        _ => followthrough::passes(id, reading),
    }
}

fn includes(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

fn actual_line(reading: &Reading<'_>) -> String {
    format!(
        "{} {} {}",
        reading.line.clean, reading.line.text, reading.raw
    )
    .to_lowercase()
}

fn full_line(reading: &Reading<'_>) -> String {
    format!("{} {}", actual_line(reading), joined_words(reading))
}

fn joined_words(reading: &Reading<'_>) -> String {
    if reading.line.words.is_empty() {
        if reading.line.clean.is_empty() {
            reading.line.text.to_lowercase()
        } else {
            reading.line.clean.to_lowercase()
        }
    } else {
        reading.line.words.join(" ").to_lowercase()
    }
}

fn device_gave_up(said: &str) -> bool {
    if said
        .split_whitespace()
        .any(|word| data::THINGS.contains(&word))
    {
        return true;
    }
    let Some(found) = GIVES_UP.find(said) else {
        return true;
    };
    let object = said[found.end()..]
        .split_whitespace()
        .find(|word| !data::BEFORE_THE_OBJECT.contains(word));
    object.is_none_or(|word| {
        word == "ghost"
            || !(word.len() > 4 && word.ends_with("ing") || data::WHAT_PEOPLE_QUIT.contains(&word))
    })
}

fn faulty_goods(reading: &Reading<'_>) -> bool {
    let said = joined_words(reading);
    if includes(&said, data::MACHINES)
        && !reading
            .line
            .words
            .iter()
            .any(|word| data::PURCHASE_IN_THE_LINE.contains(&word.as_str()))
    {
        return false;
    }
    let purchase = A_BUYING_LINE.is_match(&said).unwrap_or(false);
    if (!purchase && (A_MARK_ON_ANYTHING.is_match(&said) || A_HOW_TO_ASK.is_match(&said)))
        || (RENTED_HOME.is_match(&said) && FITTED_TO_THE_FLAT.is_match(&said) && !purchase)
        || (IT_ARRIVED.is_match(&said) && A_MARK_THAT_CAME_WITH_IT.is_match(&said))
        || A_REPAIR_OR_REPLACE.is_match(&said)
        || !device_gave_up(&said)
    {
        return false;
    }
    let actual = actual_line(reading);
    if A_BICYCLE_CABLE.is_match(&actual)
        && includes(&actual, &["bike", "bicycle", "cycle", "pushbike"])
        && A_REPAIR_ASK.is_match(&actual)
        && !A_SHOP_CLAIM.is_match(&actual)
    {
        return false;
    }
    includes(&said, data::KITCHEN_QUIET) || kitchen::without(&reading.line).is_none()
}

fn stated_age_months(reading: &Reading<'_>) -> Option<u64> {
    const NUMBERS: &[&str] = &[
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    reading.line.words.windows(2).find_map(|pair| {
        let multiplier = match pair[1].as_str() {
            "month" | "months" => 1,
            "year" | "years" | "yr" | "yrs" => 12,
            _ => return None,
        };
        let before = pair[0].as_str();
        let count = match before {
            "a" | "an" => Some(1),
            "couple" => Some(2),
            "few" => Some(3),
            _ => NUMBERS
                .iter()
                .position(|word| *word == before)
                .map(|index| index as u64 + 1)
                .or_else(|| {
                    before
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                        .parse()
                        .ok()
                }),
        }?;
        Some(count.saturating_mul(multiplier))
    })
}

// Consumer Rights Act 2015 sections 19–24 and 28.
// Consumer Contracts Regulations 2013 regulations 34–35.
pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    match id {
        "faulty-goods" => Some(faulty_reply(reading)),
        "delivery-promise" => Some(Cow::Borrowed(
            "If a UK retailer misses an agreed delivery date, ask it in writing to deliver within a further reasonable period. If there was no agreed date, it usually has to deliver without undue delay and within 30 days of the contract. You can usually end the contract and ask for a refund if the further deadline is missed. You may be able to cancel immediately if the retailer refuses to deliver or the original delivery date was essential. Keep the order confirmation and any messages showing the promise.",
        )),
        "return-costs" => Some(return_costs(&format!(
            "{} {}",
            actual_line(reading),
            joined_words(reading)
        ))),
        "shop-return" => Some(shop_return(&format!(
            "{} {}",
            actual_line(reading),
            joined_words(reading)
        ))),
        _ => followthrough::reply(id, reading).or_else(|| services::reply(id, reading)),
    }
}

fn faulty_reply(reading: &Reading<'_>) -> Cow<'static, str> {
    let proof = match stated_age_months(reading) {
        Some(months) if months > 6 => {
            " After six months, you may need to show the goods failed to meet the contract when supplied. Keep proof of purchase, correspondence and any inspection report."
        }
        _ => {
            " For a repair, replacement or final remedy within the first six months, the fault is generally presumed to have been present at delivery unless the retailer can show otherwise or that presumption does not fit the goods or fault. Keep proof of purchase and report the problem to the retailer."
        }
    };
    Cow::Owned(format!("{REFUND_OR_REPAIR}{proof}"))
}

fn return_costs(said: &str) -> Cow<'static, str> {
    const POSTAGE: &str = "If the goods are faulty, misdescribed or different from the order, ask the UK retailer for a prepaid label or collection. The retailer generally bears reasonable return costs. For a qualifying online change-of-mind cancellation, you may have to pay the direct return cost if you were told before buying and the trader did not agree to cover it.";
    if includes(said, data::FLAT_FEE_WORDS) {
        return Cow::Owned(format!(
            "For a qualifying statutory cancellation, the trader cannot charge a flat restocking or handling fee. It may deduct for loss of value if you handled the item more than necessary, subject to the cancellation rules. {POSTAGE}"
        ));
    }
    Cow::Borrowed(POSTAGE)
}

fn shop_return(said: &str) -> Cow<'static, str> {
    const REPLY: &str = "For a change of mind after an in-store purchase, check the shop's returns policy for deadlines, proof of purchase and whether it offers an exchange, credit or refund. For faulty or misdescribed goods from a UK retailer, you can usually reject them for a refund within 30 days. A shop policy cannot remove that statutory right.";
    if includes(said, data::SALE_TAG_WORDS) {
        return Cow::Owned(format!(
            "{REPLY} Sale items have the same statutory protections. A retailer can refuse a claim for a specific fault it pointed out before you bought the item."
        ));
    }
    Cow::Borrowed(REPLY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_follow_the_reason_and_faulty_goods_keep_the_source_stand_downs() {
        let reading = |line| Reading::new(line);
        assert!(!passes(
            "faulty-goods",
            &reading("My washing machine broke.")
        ));
        assert!(passes(
            "faulty-goods",
            &reading("My washing machine broke and I need a refund.")
        ));
        assert!(!passes(
            "faulty-goods",
            &reading("My new sofa arrived scratched.")
        ));
        assert!(!passes(
            "delivery-promise",
            &reading("How do I invest in stocks?")
        ));
        assert!(passes(
            "delivery-promise",
            &reading("Is this in stock for delivery?")
        ));
        assert!(
            faulty_reply(&reading("My laptop is two years old and stopped working."))
                .contains("After six months")
        );
        assert!(return_costs("restocking fee").contains("flat restocking"));
        assert!(shop_return("they said it was in the sale").contains("pointed out"));
    }
}
