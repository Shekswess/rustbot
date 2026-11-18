//! Vegetable texture, chilli, wine pairings, and kitchen equipment.

use super::{cooking_extra_data as data, cooking_storage, guards::Context};
use crate::bot::{Reading, knowledge::dishes, understanding::body::Part};
use std::borrow::Cow;

fn contains(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

fn line(text: &str) -> String {
    let plain: String = text
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '\'' {
                ch.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    format!(
        " {} ",
        plain.split_whitespace().collect::<Vec<_>>().join(" ")
    )
}

fn names(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| {
        [" ", "s ", "es "]
            .iter()
            .any(|suffix| text.contains(&format!(" {word}{suffix}")))
    })
}

pub(super) fn soft_veg(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let mut text = line(context.reading.raw);
    if !names(&text, data::VEG_SOFT_NAMES)
        && let Some(previous) = context.earlier.last()
    {
        let earlier = line(&previous.user);
        if names(&earlier, data::VEG_SOFT_NAMES) {
            text = format!("{text} {earlier}");
        }
    }
    if names(&text, &["slimy", "smelly", "sour", "gone off"]) {
        return Some(Cow::Borrowed(
            "Discard vegetables that smell off, are slimy from spoilage, or are past a use-by date. Cooking does not make spoiled food safe. If freshly cooked vegetables are simply watery, drain them and season them, or add them to soup.",
        ));
    }
    if names(&text, data::VEG_RAW_FAULTS) && !names(&text, data::VEG_COOKED) {
        return Some(Cow::Borrowed(
            "A raw vegetable can go limp as it loses moisture. If it has only lost crispness and shows no sign of spoilage, try soaking it in cold water for 20 to 30 minutes or use it in a soup or stir-fry.",
        ));
    }
    if names(&text, data::VEG_PREVENTION) && !names(&text, data::VEG_GENERIC) {
        return None;
    }
    for (category, reply) in [
        (
            data::VEG_PODS,
            "Peas and beans soften quickly. Follow the packet times for frozen veg and drain it as soon as it is cooked. Serve soft peas with butter and lemon or add them to soup.",
        ),
        (
            data::VEG_LEAVES,
            "Wilt spinach or kale in a wide pan with a little water and drain it as soon as it softens. Press extra water out of freshly cooked leaves and dress them with butter or vinegar.",
        ),
        (
            data::VEG_BRASSICAS,
            "Broccoli, cauliflower and cabbage soften and lose colour if boiled for too long. Cook florets or shreds briefly and drain them as soon as they are tender. Use already soft vegetables in soup or a gratin.",
        ),
        (
            data::VEG_WET,
            "Courgettes, aubergines and tomatoes release water. Pat the slices dry and fry or roast them in one uncrowded layer so steam can escape. Use already soft vegetables in a sauce or soup.",
        ),
        (
            data::VEG_ROASTED,
            "Dry the vegetables well, toss them in a little oil and spread them out on the tray. Roast them at around 200°C (180°C fan) so they can brown. Give a crowded tray extra space and finish it uncovered.",
        ),
    ] {
        if names(&text, category) {
            return Some(Cow::Borrowed(reply));
        }
    }
    Some(Cow::Borrowed(
        "Cook vegetables in a shallow layer with room for steam to escape. Check a minute before the recipe time and drain as soon as they are tender. If already soft, drive off a little water in a hot pan or add them to soup or a sauce.",
    ))
}

pub(super) fn chilli_keep(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    let reading = context.reading;
    let explicit = contains(&reading.raw.to_lowercase(), data::CHILLI_IN_THE_LINE);
    if !explicit && dishes::plan_label_in_chat(reading.raw, context.earlier).is_some() {
        let response = if id == "chilli-freeze" {
            cooking_storage::registered("freeze-cooked", reading, context.earlier)
        } else {
            dishes::followup::reply(reading, context.earlier, context.memory)
        };
        if response.is_some() {
            return response;
        }
    }
    Some(Cow::Borrowed(if id == "chilli-freeze" {
        "Freeze fresh chillies whole in a sealed freezer bag and slice them while still frozen. They soften on thawing, so use them in cooked dishes. There is no need to blanch them first."
    } else {
        "Keep fresh chillies dry in the fridge drawer. They usually last a week or two, depending on how fresh they are. If you have a glut, freeze them whole or dry them using a tested method."
    }))
}

pub(super) fn curry_heat(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = line(&reading.line.clean);
    if contains(&text, data::CURRY_FAULT) {
        return None;
    }
    let verdict = data::CURRY_HEAT
        .iter()
        .find_map(|(aliases, description)| {
            aliases
                .iter()
                .find(|name| text.contains(**name))
                .map(|name| format!("A {name} is {description}. "))
        })
        .unwrap_or_default();
    Some(Cow::Owned(format!(
        "{verdict} From mild to hot, the usual takeaway order is {}. Recipes vary by kitchen, so ask how hot a dish is and whether they can make it milder.",
        data::CURRY_ORDER,
    ).trim_start().to_owned()))
}

pub(super) fn wine(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = line(&reading.line.clean);
    data::WINE_PAIRS
        .iter()
        .find(|(aliases, _)| names(&text, aliases))
        .map(|(_, reply)| Cow::Borrowed(*reply))
        .or_else(|| names(&text, data::WINE_MEALS).then_some(Cow::Borrowed(data::WINE_WEIGHT)))
}

pub(super) fn kitchen_fault(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = line(&reading.line.clean);
    if names(&text, &["jam", "marmalade", "chutney"]) && names(&text, &["cloudy"]) {
        return Some(Cow::Borrowed(
            "Cloudiness while cooking jam or marmalade can come from trapped air or fruit pulp. Skim foam during cooking if you want a clearer finish, and follow a tested recipe for jarring it. Discard a stored jar if you see mould, a bulging lid or signs of spoilage.",
        ));
    }
    if names(
        &text,
        &[
            "pickle",
            "chutney",
            "white stuff",
            "white bits",
            "white film",
        ],
    ) {
        if names(
            &text,
            &[
                "mould", "mouldy", "mold", "moldy", "furry", "fuzz", "hairy", "slimy",
            ],
        ) {
            return Some(Cow::Borrowed(
                "Discard the jar if you find mould, slime or a disagreeable smell. Do not taste it to check. Use a tested pickling recipe and keep fermenting vegetables below the brine.",
            ));
        }
        if names(&text, &["fizzy", "bubbling", "bubble"]) {
            return Some(Cow::Borrowed(
                "Bubbles can be normal in an actively fermenting brine pickle made with a tested recipe. Unexpected fizz in a stored vinegar pickle or a sealed jar, especially with a bulging lid, is a reason to discard it. Do not taste it to check.",
            ));
        }
        if names(
            &text,
            &[
                "white stuff",
                "white bits",
                "white speck",
                "white film",
                "white bloom",
                "cloudy brine",
            ],
        ) {
            return Some(Cow::Borrowed(
                "A white film can be yeast, but appearance alone cannot confirm what it is. If the jar is of uncertain origin or you cannot distinguish it from mould, discard it. Follow a tested fermenting recipe and keep the vegetables submerged in brine.",
            ));
        }
    }
    if names(
        &text,
        &[
            "lentil",
            "chickpea",
            "bean",
            "haricot",
            "split pea",
            "cannellini",
        ],
    ) && names(
        &text,
        &[
            "still hard",
            "went hard",
            "stay hard",
            "stayed hard",
            "wont soften",
            "won't soften",
            "didnt soften",
            "didn't soften",
            "not softening",
            "rock hard",
            "too hard",
        ],
    ) {
        return Some(Cow::Borrowed(
            "Older dried pulses can take longer to soften. Follow the packet for soaking and cooking, and keep cooking until tender. Add acidic ingredients such as tomatoes after the pulses have softened. Dried kidney beans need a vigorous boil after soaking, and a slow cooker alone is unsuitable.",
        ));
    }
    None
}

pub(super) fn probe_guard(reading: &Reading<'_>) -> bool {
    let text = line(&reading.line.clean);
    if dishes::named_dish_has_heat(&text) {
        return false;
    }
    if names(
        &text,
        &[
            "temperature probe",
            "temperature sensor",
            "temperature gauge",
            "temperature check",
        ],
    ) {
        return true;
    }
    reading.body != Some(Part::Temperature)
        && !names(
            &text,
            &[
                "my temperature",
                "our temperature",
                "your temperature",
                "their temperature",
                "his temperature",
                "her temperature",
                "has a temperature",
                "have a temperature",
            ],
        )
}

pub(super) fn probe(_: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(
        "Insert the probe into the thickest part of the food, away from bone and the pan. Wait for the reading to settle, and check more than one spot in a large piece. For poultry, burgers, sausages and other foods that need to be cooked through, the centre should reach 70°C for at least 2 minutes or 75°C for at least 30 seconds. Follow the thermometer instructions to check its calibration and clean the probe between uses.",
    ))
}

pub(super) fn air_basket(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = line(&reading.line.clean);
    for (aliases, answer) in [
        (
            &["parchment", "baking paper", "greaseproof"][..],
            "Check your model's manual before using baking paper. Some air fryers advise against it. If your model allows it, never put it in loose or during preheating, since it can lift into the element and burn.",
        ),
        (
            &[
                "cling film",
                "plastic",
                "freezer bag",
                "tub",
                "tupperware",
                "container",
            ][..],
            "Take off ordinary plastic, cling film and freezer bags before cooking. Use only cookware that your air fryer manual approves, and set a hot basket on a heat-resistant surface.",
        ),
        (
            &["foil"][..],
            "Check your model's manual before using foil. Some manufacturers advise against it. Where allowed, secure it under food, keep it away from the element and leave the air vents clear.",
        ),
        (
            &["whole chicken"][..],
            "Use a whole chicken only if the manual allows it and there is space around the bird. Keep it away from the element and follow the appliance's timings. Check the thickest parts reach 70°C for 2 minutes or 75°C for 30 seconds.",
        ),
        (
            &[
                "glass", "pyrex", "ceramic", "ramekin", "dish", "bowl", "plate", "mug",
            ][..],
            "Use a glass or ceramic dish only if it is oven-safe and your air fryer manual allows it. Leave room for airflow and avoid sudden temperature changes that can crack the dish.",
        ),
        (
            &["silicone", "sillicone"][..],
            "Check both the air fryer manual and the silicone item's temperature rating. Keep the mould or liner clear of the element and leave room for airflow.",
        ),
        (
            &["metal", "tin", "mould", "mold", "skewer", "loaf"][..],
            "Use an oven-safe metal tin or skewer only if your model allows it. Keep it clear of the element and leave space for air to circulate.",
        ),
    ] {
        if names(&text, aliases) {
            return Some(Cow::Borrowed(answer));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_the_table_order_and_leaves_curry_faults_to_the_rescue_card() {
        assert!(
            wine(&Reading::new("What wine with prawns and steak?"))
                .unwrap()
                .contains("Sauvignon")
        );
        assert!(
            curry_heat(&Reading::new("How hot is a madras?"))
                .unwrap()
                .starts_with("A madras")
        );
        assert!(curry_heat(&Reading::new("My madras is too spicy")).is_none());
    }

    #[test]
    fn kitchen_cards_choose_safe_materials_and_stand_down_for_fever() {
        assert!(
            air_basket(&Reading::new("Can I put foil in the air fryer?"))
                .unwrap()
                .contains("manual")
        );
        assert!(!probe_guard(&Reading::new(
            "My temperature is 39, what should I do?"
        )));
        assert!(probe_guard(&Reading::new(
            "How do I use a meat thermometer?"
        )));
        assert!(
            kitchen_fault(&Reading::new("Pickles have fuzzy mould"))
                .unwrap()
                .starts_with("Discard")
        );
    }
}
