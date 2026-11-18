//! Phone and cable fixtures captured from PyBot 7d63c7f.
use super::*;

#[test]
fn native_phone_controls_require_phone_language() {
    for (line, expected) in [
        ("How do I block a number on my phone?", true),
        ("How do I stop someone ringing me?", true),
        ("How do I stop someone snoring?", false),
        ("How do I stop someone smoking?", false),
        ("How do I view my blocked list?", true),
        ("My mum keeps ringing me at work", true),
        ("How do I see contact details?", true),
        ("How do I stop someone contacting me?", false),
        ("How do I stop noise from upstairs?", false),
        ("Where is my blocklist?", true),
    ] {
        assert_eq!(phone_block(&Reading::new(line)), expected, "{line:?}");
    }
}

#[test]
fn a_frayed_cable_answer_stands_aside_for_prevention() {
    for (line, expected) in [
        ("My phone charger is frayed", true),
        ("How do I stop my cable fraying?", false),
        ("How can I prevent my charging cable from fraying?", false),
        ("The cable is frayed. How do I stop it fraying?", true),
        ("Can I tape a split charger cable?", true),
        ("How do I stop my cable from splitting?", true),
        ("How can I stop my cable wearing out?", false),
        ("Can I still use my cable even though it's fraying?", true),
        ("My bike brake cable is frayed", true),
        ("How do I stop the wires exposing themselves?", true),
        ("How can I avoid a cable wearing through?", true),
    ] {
        assert_eq!(cable_damage(&Reading::new(line)), expected, "{line:?}");
    }
}

#[test]
fn billing_replies_keep_original_app_names_and_default() {
    for (line, expected_app) in [
        ("Is Facebook free?", "Facebook"),
        ("How do I stop paying for Instagram?", "Instagram"),
        ("Is Snapchat a subscription?", "Snapchat"),
        ("How do I cancel TikTok?", "TikTok"),
        ("How much does Twitter cost?", "X"),
        ("How much is WhatsApp monthly?", "WhatsApp"),
        ("Does Messenger cost anything?", "Messenger"),
        ("How much does Reddit cost?", "Reddit"),
        ("Is a LinkedIn account free?", "LinkedIn"),
        ("Do I have to pay for Pinterest?", "Pinterest"),
        ("Is Discord free to sign up?", "Discord"),
        ("Does Youtube charge?", "YouTube"),
        ("Do I have to pay for Google Maps?", "Google Maps"),
        ("Is Gmail free?", "Gmail"),
        ("Is this app free?", "That app"),
        ("Is WhatsApp or Facebook free?", "Facebook"),
        ("Is X free?", "That app"),
    ] {
        let expected = format!("{expected_app}{FREE_APP_BODY}");
        assert_eq!(
            free_app(&Reading::new(line)).as_deref(),
            Some(expected.as_str()),
            "{line:?}"
        );
    }
}

#[test]
fn wallet_reply_and_debt_exclusion_match_the_reference() {
    for (line, expected, allowed) in [
        ("How do I add my card to my phone?", SETUP, true),
        ("How do I set up Apple Pay?", SETUP, true),
        ("My phone won't tap in the shop", TAP_TROUBLE, true),
        ("My card doesn't tap", TAP_TROUBLE, true),
        ("Google Pay doesn't work", TAP_TROUBLE, true),
        ("Apple Pay is declined", TAP_TROUBLE, true),
        ("I can't pay my credit card bill", TAP_TROUBLE, false),
        ("I've missed a card payment", SETUP, false),
        ("My card is maxed out", SETUP, true),
        (
            "I can't pay my phone bill with my credit card",
            TAP_TROUBLE,
            true,
        ),
        ("Can I tap a debit card?", SETUP, true),
        ("I want to pay with a card", SETUP, true),
        ("How do I use contactless on my mobile?", SETUP, true),
        ("My NFC isn't working", TAP_TROUBLE, true),
        ("My card won't tap to pay", TAP_TROUBLE, true),
        ("I can't tap the card on my phone", SETUP, true),
        (
            "I can't pay my card. The APR is too high",
            TAP_TROUBLE,
            false,
        ),
    ] {
        let reading = Reading::new(line);
        assert_eq!(
            card_on_phone(&reading).as_deref(),
            Some(expected),
            "{line:?}"
        );
        assert_eq!(wallet_context(&reading), allowed, "{line:?}");
    }
}

const FREE_APP_BODY: &str = " is free: there is no subscription to cancel, so a payment going out is an in-app purchase or a subscription bought through the app store. Cancel that in Settings > your name > Subscriptions on an iPhone, or Play Store > Payments and subscriptions on Android. To leave the account itself, deactivate it in the app's settings - deactivation hides it until you sign in again, and deletion is permanent.";

const SETUP: &str = "In the bank's own app there is usually an 'Add to Apple Pay' or 'Add to Google Pay' button on the card, and it is the quickest way; otherwise open Wallet on the phone and add the card number by hand or with the camera. The bank then sends a code, and some cards need one contactless payment with the PIN before the phone version works.";

const TAP_TROUBLE: &str = "Nine times in ten it is the phone, not the card: the screen has to be on and unlocked, NFC (contactless) has to be switched on, and the card has to be the one set as default in Wallet - check all three and try the back of the phone flat on the reader, not the edge. If it still refuses, take the phone out of its case, and try a second till or a shop you have used before, because one terminal can be the fault. When the phone taps nowhere at all, the card itself still works: use the plastic, then remove the card in the bank app and add it again, which reissues the token - and if the app says the card is blocked, ring the bank, because a block is on the card and no amount of tapping clears it.";
