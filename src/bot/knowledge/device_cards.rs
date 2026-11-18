//! Device-specific storage and location-sharing replies from the pinned registry.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::{
    DEVICE_BUYING_CARD, DEVICE_LOCATION_HOW, DEVICE_LOCATION_KNOW, DEVICE_LOCATION_STEPS,
    DEVICE_LOCATION_STOP, DEVICE_LOCATION_WHETHER, DEVICE_LOCATION_WHO, DEVICE_NO_CARD_SLOT,
    DEVICE_UNREADABLE_CARD, LOCATION_ACCESS,
};

macro_rules! pattern {
    ($name:ident, $source:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($source)));
    };
}

pattern!(ASKS_STOP, DEVICE_LOCATION_STOP);
pattern!(ASKS_HOW_GPS, DEVICE_LOCATION_HOW);
pattern!(ASKS_ALREADY_KNOW, DEVICE_LOCATION_KNOW);
pattern!(ASKS_WHETHER, DEVICE_LOCATION_WHETHER);
pattern!(ASKS_WHO, DEVICE_LOCATION_WHO);
pattern!(ASKS_STEPS, DEVICE_LOCATION_STEPS);

fn typed<'a>(reading: &'a Reading<'_>) -> Cow<'a, str> {
    let line = &reading.line;
    [&line.text, &line.clean, &line.norm_text, &line.plain]
        .into_iter()
        .find(|view| !view.is_empty())
        .map(|view| Cow::Borrowed(view.as_str()))
        .unwrap_or_else(|| Cow::Owned(line.words.join(" ")))
}

fn who_can_see(text: &str) -> bool {
    ASKS_WHO.is_match(text) && !ASKS_STEPS.is_match(text)
}

pub(super) fn sharing_subject(reading: &Reading<'_>) -> bool {
    let text = typed(reading);
    who_can_see(&text)
        || !(ASKS_STOP.is_match(&text)
            || ASKS_ALREADY_KNOW.is_match(&text)
            || ASKS_HOW_GPS.is_match(&text))
}

pub(super) fn share_location(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = typed(reading);
    let reply = if who_can_see(&text) {
        LOCATION_ACCESS
    } else if ASKS_WHETHER.is_match(&text) {
        "Share your location only with someone you trust, and check who is in the chat or can open the link. A one-off pin shows where you were when you sent it. Live sharing updates your location for as long as you allow it, and the recipient may still have a record of the last location they saw."
    } else {
        "In WhatsApp, open a chat and choose Attach > Location. Send your current location for a one-off pin, or choose Share live location and a duration for updates. On an iPhone, use Find My > People. In Google Maps, tap your profile picture > Location sharing. Check the recipients and duration before sending."
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn sd_card(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = &reading.line;
    let text = format!("{} {}", line.text, line.clean).to_lowercase();
    let mentions = |names: &[&str]| names.iter().any(|name| text.contains(name));
    let phone = mentions(&["phone", "android", "samsung", "tablet", "galaxy", "pixel"]);
    let full = mentions(&["full", "no room", "no space", "filled"]);
    let reply = if mentions(DEVICE_NO_CARD_SLOT) {
        "Without a card slot, use the device's internal storage, a verified backup or a compatible external reader. Check that saved copies open before deleting anything from the device or card."
    } else if mentions(DEVICE_UNREADABLE_CARD) {
        "Try reseating the card as the device manual describes, or try a compatible card reader. If it becomes readable, copy the files off and check that the copies open. Avoid formatting or writing to an unreadable card if it holds irreplaceable files you want to recover."
    } else if mentions(&["switch 2", "switch two"]) {
        "Nintendo Switch 2 requires a microSD Express card for expandable game storage. A standard microSD card from an original Switch cannot store games on a Switch 2. Check Nintendo's instructions for your exact console before buying."
    } else if mentions(&["switch", "nintendo"]) {
        "The original Nintendo Switch uses a microSD card rather than a full-size SD card. Check the console model for the slot location and supported capacity. Back up photos and screenshots before formatting."
    } else if mentions(&["steam deck", "console"]) {
        "Check which card format and capacity your console supports. If the console needs to format the card for games, back up any existing photos or files and verify those copies first."
    } else if phone && full {
        "Copy the card's photos and videos to another device or backup before deleting anything. Open a few files from the copy and make sure the transfer finished. Check your phone's supported card capacity if you want to replace it."
    } else if text.contains("iphone") {
        "An iPhone has no built-in SD card slot. Use a compatible external card reader to import photos, or review Settings > General > iPhone Storage to see what is taking room. Verify a backup before deleting original files."
    } else if mentions(&["laptop", "macbook", "notebook"]) {
        "Use the laptop's SD slot if it has one, or a compatible USB card reader. Copy the photos and videos to the laptop and check that they open before deleting or formatting the card."
    } else if mentions(&["ipad", "tablet"]) {
        "An iPad can import photos with a compatible external reader. For an Android tablet, check the exact model for a microSD slot and supported capacity. Some models can format a card as internal storage, which erases it and can tie it to that device."
    } else if phone || text.contains("handset") {
        "Check the phone's exact model for a microSD slot and supported capacity. If it offers mobile or internal storage modes, read the device instructions before choosing. Formatting erases the card, so verify a backup first."
    } else if mentions(DEVICE_BUYING_CARD)
        && !mentions(&["full", "no room", "no space", "running out"])
    {
        "Check the device's supported SD or microSD format, capacity and speed class. Video modes can have different write-speed requirements, so use the camera's manual for the mode you need. Buy from a reputable retailer."
    } else {
        "Copy the files off the card to another device and check that the copies open before deleting or formatting anything. If the camera requires its own format, follow the camera manual once the backup is verified."
    };
    Some(Cow::Borrowed(reply))
}
