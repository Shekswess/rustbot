//! Unattended appliance advice from UK fire-service guidance.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

static LEAVES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bleaves\s+(\w+)").expect("appliance action"));
static FIRE_OR_HEAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:ovens?|hobs?|cookers?|grills?|stoves?|fryers?|chip pan|candles?|irons?|heaters?|",
        r"toasters?|microwaves?|wood ?burners?|fireplaces?|open fires?|gas fires?|log fires?|coal fires?)\b"
    ))
    .expect("items that need supervision")
});

fn has(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

pub(in super::super) fn applies(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    if has(
        clean,
        &[
            "in the oven",
            "in the fridge",
            "in the freezer",
            "in the microwave",
            "in the dishwasher",
            "in the washer",
            "in the tumble dryer",
            "in the air fryer",
        ],
    ) {
        return false;
    }
    if let Some(found) = LEAVES.captures(clean) {
        let words = &clean[found.get(1).expect("first word after leaves").start()..];
        let first = &found[1];
        let switching = matches!(first, "it" | "them" | "on" | "all")
            || words.starts_with("everything on")
            || matches!(first, "the" | "a") && words.split_whitespace().nth(2) == Some("on");
        if !switching {
            return false;
        }
    }
    !has(clean, &["fridge", "freezer"])
        || has(
            clean,
            &[
                "while im away",
                "while i am away",
                "while we are away",
                "on holiday",
                "fortnight",
                "two weeks",
                "a week away",
                "for a week",
                "for the weekend",
                "turn it off",
                "turning it off",
                "switch it off",
                "switched it off",
                "switching it off",
                "switched off",
                "switch off",
                "unplug",
                "empty it",
                "emptied",
                "left running",
                "left on",
                "leave it on",
                "leave the fridge on",
                "leave the freezer on",
                "leave both on",
                "running while",
                "keep it on",
                "keeping it on",
                "propped open",
            ],
        )
}

// https://www.london-fire.gov.uk/safety/the-home/electrical-items/whitegoods/
// https://www.london-fire.gov.uk/incidents/2022/february/flat-fire-pinner/
pub(in super::super) fn unattended(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    let reply = if FIRE_OR_HEAT.is_match(clean) {
        "Switch off cooking appliances, irons and portable heaters, and put out candles or open fires safely before leaving or going to sleep. If there is a fire or smoke, get everyone out, stay out and call 999 in the UK. Never use water on burning cooking oil."
    } else if has(
        clean,
        &[
            "dishwasher",
            "washing machine",
            "washer",
            "tumble dryer",
            "tumble drier",
            "dryer",
        ],
    ) {
        if has(clean, &["dryer", "drier"]) {
            "Run a tumble dryer only while you are home and awake. Clean the lint filter after each load and keep any vent clear according to the manual. Check the model for safety recalls."
        } else {
            "Run the washing machine or dishwasher while you are home and awake. Check the model for recalls and follow the manufacturer's maintenance instructions. A delay timer still leaves the machine unattended if it runs while you are asleep or out."
        }
    } else if has(clean, &["fridge", "freezer"]) {
        "A fridge or freezer storing food normally needs to keep running. If you switch an empty unit off for a longer absence, clean it and follow its manual for shutdown and restart. Check the appliance for recalls and make sure its ventilation openings are clear."
    } else if has(
        clean,
        &[
            "charger",
            "on charge",
            "extension lead",
            "electric blanket",
            "straighteners",
        ],
    ) {
        "Unplug chargers and switch off straighteners when you finish. Follow the model's instructions for an electric blanket and leave it on while sleeping only if it is designed for all-night use. Stop using any item with a damaged lead, burning smell or overheating plug."
    } else if has(clean, &["fairy lights", "christmas lights", "tree lights"]) {
        "Switch decorative lights off before bed or leaving home. Use a set intended for its location and stop using it if the cable, plug or lamps are damaged."
    } else {
        return None;
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn left_on_unattended_distinguishes_a_dishwasher_from_stored_food() {
        assert!(!applies(&Reading::new(
            "Can I leave rice in the fridge overnight?"
        )));
        assert!(!applies(&Reading::new("My dishwasher leaves white marks")));
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("Is it safe to leave the dishwasher running overnight?");
        assert!(turn.reply.contains("home and awake"), "{}", turn.reply);
    }
}
