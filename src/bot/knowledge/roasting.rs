//! Native selection for the authored roasting, poultry and thawing cards.

use std::{borrow::Cow, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use regex::Regex;

use crate::{bot::Reading, text::is_space};

use super::super::cooking::{POULTRY_DONENESS, chicken_roast_reply, cooking_minutes};
use super::food_fold::fold;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(PRICE_JUDGMENT, super::ROAST_PRICE_JUDGMENT);
pattern!(
    WEIGHT_WORD,
    r"\d|\b(?:kilo\w*|kg|g|lb|lbs|pound|pounds|oz|ounce|ounces)\b"
);
pattern!(FISH, r"\b(haddock|cod|trout|salmon)\b");
pattern!(
    JOINT_WORD,
    r"\b(?:chicken|turkey|beef|lamb|pork|ham|gammon|goose|duck|joint|bird|brisket)\b"
);
pattern!(JOINT_TIMING, super::ROAST_JOINT_TIMING);
pattern!(FAT_WORD, super::ROAST_FAT_WORD);

const FROZEN_PACKAGE: &str = "Cook it from frozen only if the label gives instructions for doing so. Otherwise, thaw it fully in the fridge before cooking. ";

fn has(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn weight<'a>(reading: &'a Reading<'_>) -> Option<(f64, Cow<'a, str>)> {
    reading.numbers.iter().find_map(|number| {
        let kilos = number.value
            * match number.unit? {
                "kg" => 1.,
                "g" => 0.001,
                "lb" => 0.4536,
                "oz" => 0.02835,
                _ => return None,
            };
        if !(0.4..=12.).contains(&kilos) {
            return None;
        }
        let spoken = &number.text;
        let label = if WEIGHT_WORD.is_match(spoken) {
            Cow::Borrowed(
                spoken
                    .strip_prefix("a ")
                    .or_else(|| spoken.strip_prefix("an "))
                    .unwrap_or(spoken),
            )
        } else if kilos < 1. {
            Cow::Owned(format!("{}g", (kilos * 1000.).round_ties_even()))
        } else {
            Cow::Owned(format!("{kilos}kg"))
        };
        Some((kilos, label))
    })
}

fn line<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

pub(super) fn chicken_time(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    if PRICE_JUDGMENT.is_match(line(reading)) {
        return None;
    }
    let (kilos, named) = weight(reading).unwrap_or((2., Cow::Borrowed("2kg")));
    Some(Cow::Owned(chicken_roast_reply(kilos, &named)))
}

pub(super) fn tenders_are_a_piece(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    if words
        .iter()
        .any(|word| matches!(word.as_str(), "tenders" | "strips" | "goujons"))
    {
        return true;
    }
    words
        .windows(2)
        .enumerate()
        .find_map(|(at, pair)| {
            (pair[0] == "chicken" && matches!(pair[1].as_str(), "tender" | "strip" | "goujon"))
                .then(|| {
                    at.checked_sub(1).is_none_or(|before| {
                        !super::ROAST_TENDER_ADJECTIVE.contains(&words[before].as_str())
                    })
                })
        })
        .unwrap_or(false)
}

pub(super) fn tenders_method(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words = reading.line.words.join(" ");
    let method = if has(&words, &["air", "airfryer", "airfryers"]) {
        "In an air fryer, chicken tenders take about 9 to 10 minutes at 190°C in a single layer. Shake the basket halfway through."
    } else if words.contains("grill") {
        "Under a hot grill, chicken tenders take about 4 to 5 minutes per side."
    } else if has(&words, &["fry", "fried", "pan", "shallow"]) {
        "In a frying pan over medium heat, chicken tenders take about 3 to 4 minutes per side."
    } else {
        "Chicken tenders take about 10 to 12 minutes at 200°C (180°C fan). Spread them out on an oiled tray and turn them halfway through."
    };
    Some(Cow::Owned(format!(
        "{method} {POULTRY_DONENESS} Without a thermometer, cut the thickest one open and check that it is steaming hot with no pink meat."
    )))
}

pub(super) fn legs(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words = reading.line.words.join(" ");
    let small = words.contains("drumstick");
    let (pieces, minutes) = if small {
        ("drumsticks", "20 to 25")
    } else {
        ("legs", "25 to 30")
    };
    let method = if has(&words, &["boil", "poach", "simmer"]) {
        format!("Simmer the {pieces} in salted water for about {minutes} minutes.")
    } else if has(&words, &["grill", "barbecue", "barbeque", "bbq"]) {
        let per_side = if small { "10" } else { "8 to 10" };
        format!(
            "Cook the {pieces} over medium heat for about {per_side} minutes a side. Move them away from the hottest spot if they colour too quickly."
        )
    } else if has(&words, &["slow", "crock"]) {
        "For slow-cooked legs or drumsticks, use fully thawed poultry and follow the slow cooker's instructions and a recipe for the cut. Add stock or tomatoes if the recipe uses them.".to_owned()
    } else if words.contains("air") {
        format!(
            "In an air fryer, allow about {minutes} minutes at 190°C for {pieces}, turning them once."
        )
    } else if small {
        "Roast drumsticks at 200°C (180°C fan) for about 25 to 30 minutes. Space them out, skin side up.".to_owned()
    } else {
        "A whole chicken leg takes about 35 to 40 minutes at 200°C (180°C fan), skin side up. A drumstick alone may take 25 to 30 minutes.".to_owned()
    };
    let frozen = if has(&words, &["frozen", "freezer"]) {
        FROZEN_PACKAGE
    } else {
        ""
    };
    Some(Cow::Owned(format!(
        "{frozen}{method} {POULTRY_DONENESS} The time is an estimate for thawed meat."
    )))
}

pub(super) fn turkey(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let named = format!(
        "{} {}",
        reading.line.words.join(" "),
        reading.line.text.to_lowercase()
    );
    if named.contains("crown") {
        return Some(Cow::Owned(format!(
            "Follow the pack instructions for the crown's weight. Many crowns roast at 180°C (160°C fan), and the time depends on the size and whether it is stuffed. {POULTRY_DONENESS} Rest it for about 30 minutes before carving."
        )));
    }
    let (kilos, spoken) = weight(reading).unwrap_or((5., Cow::Borrowed("5kg")));
    // Food Standards Agency guidance for an unstuffed whole bird without pack instructions.
    let duration = if kilos < 4.5 {
        45. * kilos + 20.
    } else if kilos <= 6.5 {
        40. * kilos
    } else {
        35. * kilos
    };
    let duration = cooking_minutes(duration);
    Some(Cow::Owned(format!(
        "Follow the turkey's pack instructions. If there are none, a fully thawed, unstuffed {spoken} turkey takes about {duration} at 180°C (160°C fan). Cook stuffing in a separate tray. Check the thickest parts of the breast and thigh, clear of bone, with a clean probe. They should reach 70°C for 2 minutes or 75°C for 30 seconds. Rest the bird before carving."
    )))
}

pub(super) fn turkey_cut(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let named = format!(
        "{} {}",
        reading.line.words.join(" "),
        reading.line.text.to_lowercase()
    );
    let method = if named.contains("spatchcock") {
        "A spatchcocked turkey lies flat and cooks more evenly. Roast at 200°C (180°C fan) and follow the pack or recipe timing for the weight. Check both the breast and thigh."
    } else if named.contains("wing") {
        "Turkey wings roast at 200°C (180°C fan). Start checking after about 25 to 30 minutes, turning them once."
    } else if named.contains("drumstick") {
        "Turkey drumsticks roast at 200°C (180°C fan). Start checking after about 35 to 40 minutes."
    } else if named.contains("leg") {
        "Turkey legs roast at 190°C (170°C fan), skin side up. Start checking after about 45 to 50 minutes."
    } else if named.contains("thigh") {
        "Boneless turkey thighs roast at 190°C (170°C fan). Start checking after about 40 to 45 minutes. Bone-in thighs may take longer."
    } else if has(&named, &["fillet", "steak", "scallop"]) {
        "For thin turkey fillets, steaks or escalopes, start checking after about 20 to 25 minutes at 190°C (170°C fan). The time in a frying pan depends on thickness."
    } else {
        let boneless = has(&named, &["boneless", "rolled", "butterflied", "flat"]);
        let (kilos, spoken) = weight(reading).unwrap_or(if boneless {
            (1.5, Cow::Borrowed("1.5kg"))
        } else {
            (2., Cow::Borrowed("2kg"))
        });
        let duration = cooking_minutes(if boneless {
            kilos * 60.
        } else {
            kilos * 40. + 20.
        });
        let cut = if boneless {
            "boneless, rolled"
        } else {
            "bone-in"
        };
        return Some(Cow::Owned(format!(
            "A {spoken} {cut} turkey breast takes roughly {duration} at 190°C (170°C fan). Follow the pack instructions if provided. {POULTRY_DONENESS} Rest it for about 10 minutes."
        )));
    };
    Some(Cow::Owned(format!(
        "{method} Follow the pack instructions if provided. {POULTRY_DONENESS}"
    )))
}

pub(super) fn poach_fish(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let fish = FISH
        .find_iter(&reading.line.clean)
        .map(|found| found.as_str())
        .min_by_key(|name| match *name {
            "haddock" => 0,
            "cod" => 1,
            "trout" => 2,
            _ => 3,
        })
        .unwrap_or("salmon");
    Some(Cow::Owned(format!(
        "Poach {fish} in water or milk held just below a simmer. Start checking a fillet at 6 to 8 minutes. A whole side in a 120°C oven may take 15 to 20 minutes. The flesh should be opaque at the thickest point and flake easily with a fork. A bay leaf and peppercorns give the liquid more flavour."
    )))
}

pub(super) fn roast_joint(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.line.text.to_lowercase();
    let text = text
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if has(&text, super::ROAST_GOING_OFF)
        || (has(&text, super::ROAST_KEEPING) && !has(&text, super::ROAST_OVEN))
    {
        return None;
    }
    let words = &reading.line.words;
    let matches = |named: &[&str]| {
        words
            .iter()
            .any(|word| named.contains(&fold(word).as_str()))
    };
    const PORK: &str = "For pork, follow the pack instructions. A rough guide is 30 minutes per 500g at 180°C (160°C fan). Cook it through, checking the thickest part registers 70°C for 2 minutes or 75°C for 30 seconds. Rest it before carving.";
    const BEEF: &str = "For an intact whole joint of beef or lamb, brown all surfaces fully. A timing guide is 20 minutes at 220°C (200°C fan), then 190°C (170°C fan) for roughly 15, 20 or 25 minutes per 500g depending on doneness. Cook rolled or minced meat all the way through. Check the pack instructions and rest the joint before carving.";
    Some(if matches(&["pork", "gammon", "ham"]) {
        Cow::Borrowed(PORK)
    } else if matches(&["beef", "lamb", "mutton"]) {
        Cow::Borrowed(BEEF)
    } else {
        Cow::Owned(format!("{BEEF} {PORK}"))
    })
}

pub(super) fn joint_time_asked(reading: &Reading<'_>) -> bool {
    let text = reading.line.words.join(" ");
    !text.contains("gammon") || has(&text, super::JOINT_SLOW_WORDS)
}

pub(super) fn joint_time(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.line.words.join(" ");
    if has(&text, super::JOINT_SLOW_WORDS) {
        return Some(Cow::Borrowed(
            "Use the slow cooker's instructions and a recipe for the cut and weight. Thaw the joint fully first, use the liquid the recipe calls for, and keep the lid on during cooking. Check that the centre reaches 70°C for 2 minutes or 75°C for 30 seconds. A joint meant for pulling may need longer to become tender.",
        ));
    }
    roast_joint(reading)
}

pub(super) fn meat_defrost(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    if has(text, super::ROAST_FROZEN_MEAL) && has(text, &["frozen", "freezer"]) {
        return None;
    }
    let big = has(text, &["turkey", "joint", "whole bird", "roasting"]);
    if !big
        && has(text, super::ROAST_FROM_FROZEN)
        && !has(text, super::ROAST_FROZEN_TIME)
        && (has(text, super::ROAST_FROZEN_ASK)
            || has(
                text,
                &[
                    "from frozen",
                    "cook it frozen",
                    "without defrosting",
                    "without thawing",
                    "no need to defrost",
                    "skip the defrosting",
                ],
            ))
    {
        return Some(Cow::Owned(format!(
            "{FROZEN_PACKAGE}Check the thickest part is cooked through before serving."
        )));
    }
    const THAW: &str = "Thaw it in a covered dish on the bottom shelf of the fridge. Follow the label and use it within 24 hours of fully defrosting unless the label gives a different time.";
    let small = has(
        text,
        &[
            "mince", "sausage", "chop", "steak", "bacon", "liver", "kidney",
        ],
    );
    if !small && has(text, &["turkey", "bird", "joint"]) {
        let estimate = if text.contains("crown") {
            "Use the turkey crown's defrosting instructions and make sure it has fully thawed before cooking.".to_owned()
        } else if has(text, &["turkey", "bird"]) {
            weight(reading).map_or_else(|| "For a whole turkey without defrosting instructions, allow about 10 to 12 hours per kg in the fridge.".to_owned(), |(kg, spoken)| {
                format!("A whole {spoken} turkey without defrosting instructions needs about {} to {} hours in the fridge.", (kg * 10.).round(), (kg * 12.).round())
            })
        } else {
            "A large joint may need a day or more in the fridge. Check the pack for the cut and size.".to_owned()
        };
        return Some(Cow::Owned(format!("{estimate} {THAW}")));
    }
    let detail = if small {
        "Mince, sausages and chops often thaw overnight."
    } else {
        "A turkey or a big joint can take several days. Smaller cuts often thaw overnight."
    };
    Some(Cow::Owned(format!(
        "{THAW} {detail} Use a microwave's defrost setting only if you can cook it straight afterwards."
    )))
}

pub(super) fn not_the_joint_timing(reading: &Reading<'_>) -> bool {
    let line = format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.norm_text,
        reading.line.norm.join(" ")
    );
    !JOINT_WORD.is_match(&line) || FAT_WORD.is_match(&line) || !JOINT_TIMING.is_match(&line)
}

pub(super) fn roast_potatoes(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    let reply = if has(text, super::ROAST_POTATO_STUCK) {
        "Dry the parboiled potatoes well and spread them out in one layer in a lightly oiled tray. Turn them gently with a fish slice once the undersides have browned enough to release."
    } else if has(text, super::ROAST_POTATO_HARD) {
        "Cut potatoes into even pieces and parboil them for about 8 minutes before roasting. If they are already in the oven, give them more time in one layer at 200°C (180°C fan). A knife should slide easily into the biggest piece."
    } else if has(text, super::ROAST_POTATO_BURNT) {
        "Lower the oven to 190°C (170°C fan), space the potatoes apart and turn them during roasting. Aim for golden yellow. Start checking sooner if the outside darkens before the middle softens."
    } else {
        "Parboil peeled potatoes for 8 minutes, drain them and shake the pan to rough up the edges. Roast them in a lightly oiled tray at 200°C (180°C fan) for about 40 to 45 minutes until golden. For frozen roast potatoes, follow the instructions on the bag."
    };
    Some(Cow::Borrowed(reply))
}

static PEOPLE: LazyLock<Vec<Lookaround>> = LazyLock::new(|| {
    let after = r"(?:people|persons?|of us|of you|guests|adults|heads|folk|for dinner|for lunch)";
    let when = r"(?:on\s+\w+day|tonight|tomorrow|this weekend|next week|at the weekend|this christmas|for christmas)";
    let written = r"one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve";
    [
        (r"\bfor\s+", after),
        (r"\b", after),
        (r"\bfor\s+", when),
        (r"\bfor\s+", r"(?=[,.!?]|$)"),
        (r"\bwe(?:'re| are)\s+", r"\b"),
    ]
    .into_iter()
    .flat_map(|(start, end)| {
        [r"\d{1,2}", written].map(move |count| {
            let gap = if end == r"\b" { "" } else { r"\s*" };
            Lookaround::new(&format!("{start}({count}){gap}{end}")).expect("roast headcount")
        })
    })
    .collect()
});

fn headcount(text: &str) -> Option<u32> {
    let names = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ];
    PEOPLE.iter().find_map(|shape| {
        let capture = shape.captures(text).ok()??;
        let raw = capture.get(1)?.as_str();
        let number = raw.parse::<u32>().ok().or_else(|| {
            names
                .iter()
                .position(|name| *name == raw)
                .map(|index| index as u32 + 1)
        })?;
        (2..=20).contains(&number).then_some(number)
    })
}

pub(super) fn roast_shopping(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = line(reading);
    let people = headcount(text);
    let meat = if has(text, &["turkey", "christmas dinner", "xmas dinner"]) {
        "turkey"
    } else if text.contains("chicken") {
        "chicken"
    } else {
        "raw joint"
    };
    let grams = if meat == "raw joint" { 250 } else { 500 };
    let amount = people.map_or_else(
        || format!("Allow about {grams}g of {meat} per person"),
        |people| {
            let kilos = f64::from(people * grams) / 1000.;
            format!("Allow about {kilos}kg of {meat} for {people} people, at {grams}g per person")
        },
    );
    let sides = if meat == "raw joint" {
        " Add ingredients for Yorkshire puddings if you want them, and mint or apple sauce for lamb or pork."
    } else {
        ""
    };
    Some(Cow::Owned(format!(
        "{amount}. Buy potatoes, carrots, parsnips or swede, and something green such as cabbage or sprouts. Add oil, stock and flour for gravy.{sides} Use the pack instructions for the cut and weight when planning the oven time."
    )))
}

pub(super) fn rest_the_meat(context: &super::guards::Context<'_>) -> Option<Cow<'static, str>> {
    let reading = context.reading;
    let line = reading.line.words.join(" ");
    let phrase = |term| super::has_space_delimited_phrase(&line, term);
    if phrase("the rest") || phrase("rest of") {
        return None;
    }
    let body = super::ROAST_REST_BODY.iter().copied().any(phrase);
    if body
        && (!has(&line, super::ROAST_REST_CUT_PHRASE)
            || !super::ROAST_REST_CUT.iter().copied().any(phrase))
    {
        return None;
    }
    if super::super::dishes::followup::has_rest_reply(reading, context.earlier)
        && super::DATABASE
            .find_excluding(reading, context.memory, true, Some("rest-meat"))
            .is_some_and(|(entry, _)| entry.row.id == "plan-followup")
    {
        return None;
    }
    let reply = if reading.features.asks.contains(&"why") {
        "Rest it because standing lets the heat that has built up round the outside move back into the middle, so the juice stays in the meat when you carve it instead of running out onto the board - loosely under foil and off the heat, not back in the oven: 15 to 20 minutes for a chicken or a turkey, 10 to 20 for a joint of beef, lamb or pork, 5 to 10 for a steak or chops, and 20 to 30 for a brisket. It stays hot that long under the foil, so nothing is lost by waiting."
    } else {
        "Loosely under foil and off the heat, not back in the oven: 15 to 20 minutes for a chicken or a turkey, 10 to 20 for a joint of beef, lamb or pork, 5 to 10 for a steak or chops, and 20 to 30 for a brisket. Standing lets the heat that has built up round the outside move back into the middle, so the juice stays in the meat when you carve it instead of running out onto the board. It stays hot that long under the foil, so nothing is lost by waiting."
    };
    Some(Cow::Borrowed(reply))
}
