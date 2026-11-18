//! Phone controls, payment-app settings and charger care from the technology domain.
#[cfg(test)]
#[path = "technology_phone_tests.rs"]
mod tests;

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(source::$name).expect(stringify!($name)));)+
    };
}
patterns!(A_PHONE_IN_THE_LINE, CABLE_PREVENT_ASK, CABLE_DAMAGE_NOW);

fn text_views(reading: &Reading<'_>) -> String {
    format!(
        "{} {} {}",
        reading.line.text, reading.line.norm_text, reading.line.clean
    )
}

pub(super) fn phone_block(reading: &Reading<'_>) -> bool {
    let text = format!("{} {}", text_views(reading), reading.raw).to_lowercase();
    A_PHONE_IN_THE_LINE.is_match(&text)
}

pub(super) fn cable_damage(reading: &Reading<'_>) -> bool {
    let text = format!("{} {}", text_views(reading), reading.raw);
    !CABLE_PREVENT_ASK.is_match(&text) || CABLE_DAMAGE_NOW.is_match(&text)
}

pub(super) fn wallet_context(reading: &Reading<'_>) -> bool {
    let text = format!(" {} {} ", text_views(reading), reading.raw).to_lowercase();
    let begins_with = |phrase: &str| text.contains(&format!(" {phrase}"));
    source::WALLET_NAMED.iter().any(|word| begins_with(word))
        || !source::DEBT_SHAPE.iter().any(|phrase| begins_with(phrase))
}

pub(super) fn card_on_phone(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = text_views(reading).to_lowercase();
    let reply = if source::TAP_PROBLEM
        .iter()
        .any(|phrase| text.contains(phrase))
    {
        source::TAP_REPLY
    } else {
        source::ADD_TO_WALLET
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn free_app(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = format!(
        "{} {} {} {}",
        reading.line.clean, reading.line.norm_text, reading.line.text, reading.raw
    )
    .to_lowercase();
    let app = source::FREE_PLATFORM_NAMES
        .iter()
        .find_map(|(word, name)| text.contains(word).then_some(*name))
        .unwrap_or("That app");
    Some(Cow::Owned(source::FREE_APP_TEMPLATE.replace("{app}", app)))
}

mod source {
    //! Authored data from PyBot 7d63c7f, bot/knowledge/technology.py.
    pub(super) const A_PHONE_IN_THE_LINE: &str = r#"\b(?:phone|phones|phoned|phoning|mobile|mobiles|call|calls|called|calling|ring|rings|ringing|rang|text|texts|texted|texting|message|messages|messaging|number|numbers|caller|callers|sim|sims|landline|voicemail|facetime|contact|contacts|block|blocked|blocking|unblock|unblocked|unblocking|blocklist|nuisance|withheld|telemarketer|telemarketers|scam|spam)\b"#;

    pub(super) const CABLE_PREVENT_ASK: &str = r#"(?i)\b(?:stop|stops|prevent|prevents|avoid|avoids)\b[^.?!]{0,40}?\b(?:fraying|splitting|wearing)\b"#;

    pub(super) const CABLE_DAMAGE_NOW: &str = r#"(?i)\b(?:frayed|fraying?\s+wire|split|splits|splitting|exposed|bare|showing|shows|nicked|worn|wearing\s+through|tape|taped|still\s+use|still\s+safe|shock|spark|burnt|burned|smell|hot)\b"#;

    pub(super) const WALLET_NAMED: &[&str] = &[
        "phone",
        "mobile",
        "wallet",
        "apple pay",
        "google pay",
        "contactless",
        "tap",
        "shop",
        "till",
        "card machine",
        "terminal",
        "reader",
        "checkout",
        "self service",
        "nfc",
        "screen",
        "app",
    ];

    pub(super) const DEBT_SHAPE: &[&str] = &[
        "cant pay",
        "can't pay",
        "cannot pay",
        "can not pay",
        "unable to pay",
        "not paid",
        "wont pay",
        "won't pay",
        "will not pay",
        "stopped paying",
        "has not paid",
        "hasnt paid",
        "hasn't paid",
        "mortgage",
        "rent",
        "rents",
        "tenant",
        "tenants",
        "landlord",
        "arrears",
        "cant afford",
        "can't afford",
        "couldnt afford",
        "couldn't afford",
        "owe",
        "owes",
        "owed",
        "overdraft",
        "bill",
        "bills",
        "due",
        "payment",
        "payments",
        "balance",
        "arrears",
        "behind on",
        "missed a payment",
        "miss a payment",
        "minimum",
        "forgot",
        "forgotten",
        "forget",
        "forgetting",
        "forgot to pay",
        "forget to pay",
        "keep forgetting",
        "late payment",
        "paid late",
        "pay late",
        "paying late",
        "payment is late",
        "missing a payment",
        "missed payment",
        "late",
        "didnt pay",
        "didn't pay",
        "not paid",
        "unpaid",
        "overdue",
        "due date",
    ];

    pub(super) const TAP_PROBLEM: &[&str] = &[
        "wont tap",
        "won't tap",
        "doesnt tap",
        "doesn't tap",
        "not tapping",
        "no tap",
        "will not tap",
        "does not tap",
        "will not work",
        "does not work",
        "not working",
        "fails",
        "failed",
        "declined",
        "declines",
        "rejected",
        "refused",
        "wont pay",
        "won't pay",
        "cant pay",
        "can't pay",
        "cannot pay",
        "can not pay",
        "not accepted",
        "is not accepted",
        "keeps failing",
        "no longer taps",
        "wont accept",
        "won't accept",
        "doesnt accept",
        "doesn't accept",
        "will not accept",
        "said no",
        "says no",
    ];

    pub(super) const TAP_REPLY: &str = "Nine times in ten it is the phone, not the card: the screen has to be on and unlocked, NFC (contactless) has to be switched on, and the card has to be the one set as default in Wallet - check all three and try the back of the phone flat on the reader, not the edge. If it still refuses, take the phone out of its case, and try a second till or a shop you have used before, because one terminal can be the fault. When the phone taps nowhere at all, the card itself still works: use the plastic, then remove the card in the bank app and add it again, which reissues the token - and if the app says the card is blocked, ring the bank, because a block is on the card and no amount of tapping clears it.";

    pub(super) const ADD_TO_WALLET: &str = "In the bank's own app there is usually an 'Add to Apple Pay' or 'Add to Google Pay' button on the card, and it is the quickest way; otherwise open Wallet on the phone and add the card number by hand or with the camera. The bank then sends a code, and some cards need one contactless payment with the PIN before the phone version works.";

    pub(super) const FREE_PLATFORM_NAMES: &[(&str, &str)] = &[
        ("facebook", "Facebook"),
        ("instagram", "Instagram"),
        ("snapchat", "Snapchat"),
        ("tiktok", "TikTok"),
        ("twitter", "X"),
        ("whatsapp", "WhatsApp"),
        ("messenger", "Messenger"),
        ("reddit", "Reddit"),
        ("linkedin", "LinkedIn"),
        ("pinterest", "Pinterest"),
        ("discord", "Discord"),
        ("youtube", "YouTube"),
        ("google maps", "Google Maps"),
        ("gmail", "Gmail"),
    ];

    pub(super) const FREE_APP_TEMPLATE: &str = "{app} is free: there is no subscription to cancel, so a payment going out is an in-app purchase or a subscription bought through the app store. Cancel that in Settings > your name > Subscriptions on an iPhone, or Play Store > Payments and subscriptions on Android. To leave the account itself, deactivate it in the app's settings - deactivation hides it until you sign in again, and deletion is permanent.";
}
