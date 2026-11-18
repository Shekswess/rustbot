//! Food safety and nutrition replies that retain the source card branches.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

use super::super::lookup::{BROWN_SAFETY_ASK, FRUIT_PEEL_ASK, FRUIT_SKIN_IN_DISH};

static SAFETY_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(BROWN_SAFETY_ASK).expect("valid browned-produce safety query"));
static PEEL_ASK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(FRUIT_PEEL_ASK).expect("valid fruit-peeling query"));
static SKIN_IN_DISH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(FRUIT_SKIN_IN_DISH).expect("valid fruit-skin dish query"));

fn contains_any(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

pub(super) fn browned_produce(reading: &Reading<'_>) -> &'static str {
    let text = &reading.line.clean;
    if reading
        .line
        .words
        .iter()
        .any(|word| matches!(word.as_str(), "mushroom" | "mushrooms"))
    {
        return "Mushrooms can darken as they age. Colour alone can't tell you whether they are safe. Follow the use-by date and storage instructions, and discard slimy mushrooms or any that look spoiled. Refrigerate cooked mushrooms within two hours and use them within 48 hours or freeze them.";
    }
    if SAFETY_ASK.is_match(text) {
        return "Cut fruit and vegetables can turn brown when exposed to air. That colour alone can't tell you whether the food is safe. Follow any use-by date and storage instructions and discard mouldy or spoiled produce. A little lemon juice can slow browning after cutting.";
    }
    let words = &reading.line.words;
    let names = |names: &[&str]| words.iter().any(|word| names.contains(&word.as_str()));
    if names(&["apple", "apples", "pear", "pears", "avocado", "avocados"]) {
        return "Toss the cut pieces with a little lemon or lime juice, or cover the cut surface to keep air off it. Refrigerate cut fruit if you're keeping it for later.";
    }
    if names(&[
        "potato",
        "potatoes",
        "aubergine",
        "aubergines",
        "eggplant",
        "parsnip",
        "parsnips",
        "artichoke",
        "artichokes",
        "salsify",
    ]) {
        return "Put the pieces in a bowl of cold water as you cut them to slow browning. A squeeze of lemon can help. Drain them before cooking.";
    }
    if names(&["celery", "celeriac"]) {
        return "Wrap the cut celery and keep it in the fridge. Browning at the edges can come from air exposure or water loss. Follow any use-by date and discard spoiled pieces.";
    }
    "The cut surface can brown when enzymes in the flesh meet air. Lemon juice or a cover pressed against the cut surface can slow the browning. Follow the storage instructions for the produce."
}

pub(super) fn fruit_skin(reading: &Reading<'_>) -> Cow<'static, str> {
    let text = &reading.line.clean;
    if contains_any(
        text,
        &[
            "mouth itch",
            "itchy mouth",
            "mouth tingl",
            "lips tingl",
            "allerg",
            "swollen lips",
            "throat tight",
        ],
    ) {
        return Cow::Borrowed(
            "An itchy or tingly mouth after raw fruit can be an allergic reaction. Avoid the food that caused it and ask a GP for advice before trying it again. Call 999 for trouble breathing, throat tightness or sudden swelling of the lips, mouth, tongue or throat.",
        );
    }
    let lead = if PEEL_ASK.is_match(text) {
        "No need to peel. "
    } else {
        "Yes. "
    };
    let body = if contains_any(text, &["kiwi", "kiwifruit"]) {
        "Kiwi skin is edible. Rinse the fruit under running water and rub off the fuzz if you prefer."
    } else if text.contains("grape") {
        "Grape skins are edible. Rinse the grapes. For babies and young children, cut grapes lengthways into quarters and remove any pips to reduce choking risk."
    } else if contains_any(text, &["peach", "plum"]) {
        "Peach and plum skin is edible. Rinse the fruit and remove the stone. Peel it if you prefer a smoother texture."
    } else {
        "Apple and pear skins are edible. Rinse the fruit, and slice, grate or soften firm fruit for young children."
    };
    if SKIN_IN_DISH.is_match(text) && !contains_any(text, &["kiwi", "grape"]) {
        return Cow::Owned(format!(
            "{lead}{body} For a crumble, pie or cake, the skin softens as the fruit cooks. Peel it for a smoother filling."
        ));
    }
    Cow::Owned(format!("{lead}{body}"))
}

pub(super) fn frozen_or_tinned(reading: &Reading<'_>) -> &'static str {
    let text = &reading.line.clean;
    if contains_any(text, &["smoothie", "blender", "shake"]) {
        return "Frozen fruit can thicken a smoothie without ice. Check that the pack says the fruit is suitable to eat without cooking. If the pack calls for cooking, follow its instructions before using the fruit. Choose unsweetened fruit when you can.";
    }
    if contains_any(
        text,
        &["cheap", "cost", "price", "budget", "dearer", "value"],
    ) {
        return "Frozen or tinned vegetables are often cheaper than fresh. Compare the price per unit weight and how much you'll use. With frozen vegetables, you can take only the portion you need and keep the rest frozen.";
    }
    if contains_any(
        text,
        &["fruit", "peach", "mango", "pineapple", "pears", "cherries"],
    ) && contains_any(
        text,
        &["tinned", "tin of", "canned", "can of", "from a tin"],
    ) {
        return "Tinned fruit counts as fruit. Choose fruit in its own juice or water without added sugar if you can.";
    }
    "Fresh, frozen and tinned fruit and vegetables can all be part of a healthy diet. For tinned produce, choose options without added sugar or salt when available. Follow the pack's cooking and defrosting instructions."
}

pub(super) fn tea_or_coffee(reading: &Reading<'_>) -> &'static str {
    let text = &reading.line.clean;
    if contains_any(text, &["pregnan", "expecting a baby"]) {
        return "The NHS recommends limiting caffeine to 200 mg a day during pregnancy, counting coffee, black and green tea, and all other sources. Amounts vary by drink, brand and size.";
    }
    if contains_any(
        text,
        &[
            "iron tablets",
            "iron supplement",
            "ferrous sulfate",
            "anaemi",
            "anemi",
        ],
    ) {
        return "Tea and coffee can reduce iron absorption. If you take ferrous sulfate, NHS guidance says to leave a two-hour gap before drinking them. For another iron product, check its leaflet or ask a pharmacist.";
    }
    let tea = text.starts_with("tea") || text.contains(" tea");
    if tea && !contains_any(text, &["coffee", "espresso", "latte"]) {
        return "Unsweetened tea counts towards your fluid intake. Black and green tea contain caffeine, so check the label for herbal or decaffeinated blends if you're trying to limit it. If it disrupts your sleep, try having it earlier in the day.";
    }
    "Coffee can be part of a balanced diet. Caffeine varies with the preparation and cup size. If it disrupts your sleep, try a smaller cup, decaf or drinking it earlier. Syrup, sugar and cream add to what is in the cup."
}
