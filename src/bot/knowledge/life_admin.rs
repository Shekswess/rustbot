//! Complaint, cancellation, lending and other life administration readings.

use regex::Regex;
use std::{
    borrow::Cow,
    sync::{LazyLock, OnceLock},
};

use crate::bot::Reading;

use super::life_admin_data as data;

macro_rules! pattern {
    ($name:ident, $value:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

pattern!(STREET, data::STREET);
pattern!(DRAIN, data::DRAIN);
pattern!(WHO, data::WHO);
pattern!(ONLY_ASK, data::ONLY_ASK);
pattern!(COMPLAINT, r"\bcomplain");
pattern!(MOOD_FINE, data::MOOD_FINE);
pattern!(FINE, r"\bfine\b(?:\s+([a-z0-9']+))?");
pattern!(FINED, r"\b(?:fined|fines)\b");
pattern!(SCHOOL_FEELING, data::SCHOOL_FEELING);
pattern!(SCHOOL_PLACE, data::SCHOOL_PLACE);
pattern!(EXCESS, data::EXCESS);
pattern!(EXCESS_PRICE, data::EXCESS_PRICE);
pattern!(CLAIM_LOSS, data::CLAIM_LOSS);
pattern!(EXCESS_DICT, data::EXCESS_DICT);
pattern!(CON, r"\bcons?\b");
pattern!(CONNED, r"\bconned\b");
pattern!(CON_PHRASE, data::CON_PHRASE);
pattern!(BORROWING, data::BORROWING);
pattern!(OFF_PERSON, data::OFF_PERSON);
pattern!(LIBRARY_RENEW, data::LIBRARY_RENEW);
pattern!(LIBRARY_COUNT, data::LIBRARY_COUNT);
pattern!(LIBRARY_RESERVE, data::LIBRARY_RESERVE);
pattern!(LIBRARY_LOST, data::LIBRARY_LOST);
pattern!(LIBRARY_LATE, data::LIBRARY_LATE);
pattern!(LIBRARY_RETURN, data::LIBRARY_RETURN);
pattern!(SCAM_ELSE, data::SCAM_ELSE);
pattern!(PHOTO_PRICE, data::PHOTO_PRICE);
pattern!(PHOTO_WHERE, data::PHOTO_WHERE);
pattern!(PHOTO_AT_HOME, data::PHOTO_AT_HOME);
pattern!(PHOTO_CHEAP, data::PHOTO_CHEAP);
pattern!(TIP_SUM, data::TIP_SUM);
pattern!(
    TIP_NON_BILL,
    r"^(?:percent|%|people|of us|pints|drinks|hours?|oclock|o.clock)\b"
);

#[derive(Clone, Copy, Debug)]
pub(super) enum Guard {
    CouncilDrain,
    BareComplaint,
    CourtFine,
    SchoolPlace,
    ExcessClaim,
    ExcessDefinition,
    Scam,
    Discount,
    PostedThing,
}

fn has(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

impl Guard {
    pub(super) fn passes(self, reading: &Reading<'_>) -> bool {
        let said = &reading.line.clean;
        match self {
            Self::CouncilDrain => {
                !DRAIN.is_match(said) || STREET.is_match(said) || !WHO.is_match(said)
            }
            Self::BareComplaint => {
                COMPLAINT.is_match(said) && ONLY_ASK.replace_all(said, " ").trim().is_empty()
            }
            Self::CourtFine => {
                let line = said.replace(
                    |c: char| !(c.is_ascii_alphanumeric() || matches!(c, ' ' | '\'')),
                    " ",
                );
                if FINED.is_match(&line) {
                    return true;
                }
                if has(&line, data::FINE_NOUN) {
                    return true;
                }
                !MOOD_FINE.is_match(&line)
                    && FINE.captures_iter(&line).any(|found| {
                        found
                            .get(1)
                            .is_none_or(|word| !data::FINE_ADJECTIVE.contains(&word.as_str()))
                    })
            }
            Self::SchoolPlace => !SCHOOL_FEELING.is_match(said) && SCHOOL_PLACE.is_match(said),
            Self::ExcessClaim => {
                !EXCESS.is_match(said) || !EXCESS_PRICE.is_match(said) || CLAIM_LOSS.is_match(said)
            }
            Self::ExcessDefinition => {
                let mut words = said
                    .split(|c: char| !(c.is_ascii_alphanumeric() || c == '\''))
                    .filter(|word| !word.is_empty())
                    .peekable();
                !(EXCESS_DICT.is_match(said)
                    || EXCESS_PRICE.is_match(said)
                    || CLAIM_LOSS.is_match(said))
                    && words.peek().is_some()
                    && words.all(|word| data::EXCESS_WORDS.contains(&word))
            }
            Self::Scam => {
                !CON.is_match(said) || CONNED.is_match(said) || !CON_PHRASE.is_match(said)
            }
            Self::PostedThing => !has(&all_views(reading), data::NOT_POSTED),
            Self::Discount => {
                !BORROWING.is_match(reading.folded()) && !OFF_PERSON.is_match(reading.folded())
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum Decision {
    Library,
    Scammed,
    ChangeName,
}

impl Decision {
    pub(super) fn answer(
        self,
        reading: &Reading<'_>,
        replies: &'static [&'static str],
    ) -> Option<&'static str> {
        let said = &reading.line.clean;
        let index = match self {
            Self::Library => [
                &*LIBRARY_RENEW,
                &*LIBRARY_COUNT,
                &*LIBRARY_RESERVE,
                &*LIBRARY_LOST,
                &*LIBRARY_LATE,
                &*LIBRARY_RETURN,
            ]
            .iter()
            .position(|pattern| pattern.is_match(said))
            .unwrap_or(6),
            Self::Scammed => usize::from(!SCAM_ELSE.is_match(said)),
            Self::ChangeName => {
                let said = if reading.line.norm_text.is_empty() {
                    reading.folded()
                } else {
                    &reading.line.norm_text
                };
                if has(said, data::NAME_CHILD) {
                    0
                } else if has(said, data::NAME_DIVORCE) {
                    1
                } else {
                    2
                }
            }
        };
        replies.get(index).copied()
    }
}

pub(super) fn cancel_order(reading: &Reading<'_>) -> Cow<'static, str> {
    let said = if reading.line.norm_text.is_empty() {
        reading.folded()
    } else {
        &reading.line.norm_text
    };
    let text = if has(said, data::CANCEL_SENT) {
        format!(
            "{} {} {}",
            data::CANCEL_WINDOW,
            data::CANCEL_REFUSE,
            data::CANCEL_REFUND
        )
    } else if has(said, data::CANCEL_RETURN) {
        format!("{} {}", data::CANCEL_WINDOW, data::CANCEL_REFUND)
    } else {
        format!(
            "{} {} {}",
            data::CANCEL_AT_WAREHOUSE,
            data::CANCEL_REFUSE,
            data::CANCEL_REFUND
        )
    };
    Cow::Owned(text)
}

pub(super) fn stolen_bag(reading: &Reading<'_>) -> &'static str {
    if has(&reading.line.clean, data::AWAY) {
        data::STOLEN_BAG_AWAY
    } else {
        data::STOLEN_BAG
    }
}

pub(super) fn stolen_thing(reading: &Reading<'_>) -> &'static str {
    let said = format!(" {} ", reading.line.clean);
    data::STOLEN_THINGS
        .iter()
        .find_map(|(names, reply)| {
            names
                .iter()
                .any(|name| {
                    said.contains(&format!(" {name} ")) || said.contains(&format!(" {name}s "))
                })
                .then_some(*reply)
        })
        .unwrap_or(data::STOLEN_THING_GEAR)
}

fn all_views(reading: &Reading<'_>) -> String {
    format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.norm_text,
        reading.folded()
    )
}

pub(super) fn missing_post(reading: &Reading<'_>) -> &'static str {
    if has(
        &all_views(reading),
        &["passport", "passport office", "travel document"],
    ) {
        data::PASSPORT_CLAIM
    } else {
        data::PARCEL_CLAIM
    }
}

pub(super) fn legal_word(reading: &Reading<'_>) -> Option<&'static str> {
    let words = &reading.line.words;
    let text = words.join(" ");
    data::LEGAL_WORDS.iter().find_map(|(aliases, reply)| {
        aliases
            .iter()
            .any(|key| {
                (key.contains(' ') && text.contains(key)) || words.iter().any(|word| word == key)
            })
            .then_some(*reply)
    })
}

pub(super) fn passport(
    reading: &Reading<'_>,
    sections: &'static [(&'static str, Option<&'static str>)],
    default: &'static str,
) -> &'static str {
    static PATTERNS: OnceLock<Vec<fancy_regex::Regex>> = OnceLock::new();
    let patterns = PATTERNS.get_or_init(|| {
        sections
            .iter()
            .map(|(pattern, _)| fancy_regex::Regex::new(pattern).expect("valid passport rule"))
            .collect()
    });
    let text = &reading.line.clean;
    for (pattern, (_, answer)) in patterns.iter().zip(sections) {
        if pattern.is_match(text).is_ok_and(|matched| matched) {
            return answer.unwrap_or_else(|| passport_photo(text));
        }
    }
    default
}

fn passport_photo(text: &str) -> &'static str {
    if PHOTO_PRICE.is_match(text) {
        data::PASSPORT_PHOTO_PRICE
    } else if PHOTO_WHERE.is_match(text) {
        data::PASSPORT_PHOTO_PLACES
    } else if PHOTO_AT_HOME.is_match(text) {
        data::PASSPORT_PHOTO_AT_HOME
    } else if PHOTO_CHEAP.is_match(text) {
        data::PASSPORT_PHOTO_PRICE
    } else {
        data::PASSPORT_PHOTO_RULES
    }
}

fn tip_amount(text: &str) -> Option<f64> {
    TIP_SUM.captures_iter(text).find_map(|found| {
        let amount = found[1].parse::<f64>().ok()?;
        let after: String = text[found.get(0)?.end()..].chars().take(12).collect();
        (amount.is_finite() && amount >= 5.0 && !TIP_NON_BILL.is_match(after.trim()))
            .then_some(amount)
    })
}

fn pounds(amount: f64) -> String {
    let whole = amount.round_ties_even();
    if amount >= 100.0 || (amount - whole).abs() < 0.005 {
        format!("£{whole:.0}")
    } else {
        format!("£{amount:.2}")
    }
}

fn whole_pounds(low: f64, high: f64) -> String {
    let first = low.ceil();
    let last = high.floor();
    if first > last {
        format!("about {}", pounds(((low + high) / 2.0).round_ties_even()))
    } else if first == last {
        pounds(first)
    } else {
        format!("{} or {}", pounds(first), pounds(last))
    }
}

pub(super) fn tipping(
    reading: &Reading<'_>,
    replies: &'static [&'static str],
) -> Option<Cow<'static, str>> {
    let text = if reading.line.norm_text.is_empty() {
        reading.folded()
    } else {
        &reading.line.norm_text
    };
    let words = text.split(|c: char| !(c.is_ascii_lowercase() || c == '\''));
    let mentions = |terms: &[&str]| {
        terms.iter().any(|term| {
            if term.contains(' ') {
                text.contains(term)
            } else {
                words.clone().any(|word| word == *term)
            }
        })
    };
    if mentions(data::TIP_LEASEHOLD) || mentions(data::TIP_ABROAD) {
        return None;
    }
    if text.contains("service charge")
        && !data::TIP_GROUPS
            .iter()
            .any(|(index, group)| matches!(*index, 3 | 4 | 6) && mentions(group))
    {
        return replies.get(2).copied().map(Cow::Borrowed);
    }
    let group = data::TIP_GROUPS
        .iter()
        .find_map(|(index, group)| mentions(group).then_some(*index));
    if group.is_none_or(|index| index > 11)
        && has(text, &["wedding", "reception", "hen do", "stag do"])
    {
        return replies.get(11).copied().map(Cow::Borrowed);
    }
    if let Some(index) = group {
        return replies.get(index).copied().map(Cow::Borrowed);
    }
    if let Some(bill) = tip_amount(text) {
        let low = bill * 0.10;
        let high = bill * 0.125;
        let amount = pounds(bill);
        let low_said = pounds(low);
        let high_said = pounds(high);
        let mut result = format!(
            "On {amount}, ten percent is {low_said} and twelve and a half is {high_said}, so {low_said} to {high_said}"
        );
        if high - low >= 2.0 {
            result.push_str(&format!(
                ", and {} is what people leave",
                whole_pounds(low, high)
            ));
        }
        result.push_str(". Check the bill for a service charge first, because that is the tip and you add nothing on top.");
        return Some(Cow::Owned(result));
    }
    replies.last().copied().map(Cow::Borrowed)
}

pub(super) fn tenancy_copy(reading: &Reading<'_>, default: &'static str) -> &'static str {
    let said = format!(
        " {} {} {} ",
        reading.line.clean,
        reading.folded(),
        reading.line.norm_text
    );
    if has(
        &said,
        &[
            " what is ",
            " whats ",
            " what's ",
            " explain ",
            " what does ",
            " mean ",
            " means ",
        ],
    ) {
        data::TENANCY_DEFINITION
    } else {
        default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancellation_distinguishes_returns_and_dispatch() {
        let returned = cancel_order(&Reading::new("can i change my mind about an online order"));
        assert!(returned.starts_with("Anything bought online"));
        assert!(!returned.contains("refuse the delivery"));
        let shipped = cancel_order(&Reading::new(
            "how do i cancel an order that has already been dispatched",
        ));
        assert!(shipped.contains("refuse the delivery"));
    }

    #[test]
    fn administration_guards_distinguish_a_named_subject() {
        assert!(Guard::BareComplaint.passes(&Reading::new("who do i complain to")));
        assert!(!Guard::BareComplaint.passes(&Reading::new("who do i complain to about noise")));
        assert!(!Guard::Discount.passes(&Reading::new("can i get a tenner off my mum")));
        assert!(Guard::Discount.passes(&Reading::new("can i get some money off my order")));
        assert!(!Guard::CourtFine.passes(&Reading::new("yes thats fine")));
        assert!(Guard::CourtFine.passes(&Reading::new("got a fine in the post")));
    }

    #[test]
    fn administration_replies_read_the_requested_branch() {
        let passport = passport_photo("where do i get a passport photo");
        assert!(passport.starts_with("A photo booth"));
        let reply = tipping(&Reading::new("how do i work out a tip on 84 pounds"), &[])
            .expect("tip for the bill");
        assert!(reply.starts_with("On £84, ten percent is £8.40 and twelve and a half is £10.50"));
        assert!(tipping(&Reading::new("should i tip in japan"), &["UK tip"]).is_none());
        assert!(
            legal_word(&Reading::new("what is a notary"))
                .is_some_and(|reply| reply.starts_with("A notary public"))
        );
    }
}
