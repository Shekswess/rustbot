//! Source reply replacements for clinical and regulatory guidance.

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "updated_guidance_data.rs"]
mod data;

#[path = "financial_guidance.rs"]
mod financial;
#[path = "food_guidance.rs"]
mod food;
#[path = "health_guidance.rs"]
mod health;

const MEDICINE_FOOD: &str = "Check the leaflet for your exact medicine and ask a pharmacist about the food or drink. The advice depends on the product. Take your medicine as prescribed while you check.";
const WARFARIN: &str = "If you take warfarin, avoid cranberry juice and grapefruit juice, and keep your intake of vitamin K-rich foods such as leafy greens consistent. Ask your anticoagulant clinic or pharmacist about your exact medicine and any changes you want to make.";
const SIDE_EFFECTS: &str = "Check the leaflet for your exact medicine and ask a pharmacist or prescriber about the symptoms. Tell them when you took it and when the symptoms started. If you vomited after taking a contraceptive pill, follow the vomiting or missed-pill instructions for your exact pill. Speak to a pharmacist or sexual health clinic promptly if you had sex without a condom and are unsure you were protected.";
const ACUTE_JOINT: &str = "A joint that becomes suddenly very painful, hot, swollen, or discoloured needs prompt assessment. Ask for an urgent GP appointment or contact NHS 111. A joint infection can produce these symptoms, even without a fever.";
const ARTHRITIS: &str = "Regular gentle activity and exercises to strengthen the muscles around the joint can help with osteoarthritis. A GP or physiotherapist can help confirm the cause and suggest suitable exercises. Ask a pharmacist or GP which pain relief is suitable for you. A sudden very painful, hot, swollen or discoloured joint needs urgent assessment.";
const HEAVY_PERIOD: &str = "If heavy periods affect your daily life, speak to a GP about the bleeding and your treatment options. Tell them how often you change products, whether you pass large clots and whether you feel tired or short of breath. Get urgent medical advice if you are bleeding very heavily now or feel faint.";
const PERIOD_CHANGE: &str = "If your periods have become heavier, more painful or irregular, speak to a GP. Tracking dates, bleeding and pain can help explain what's changed. Ask for urgent care if the pain is severe or worse than usual and pain relief has not helped.";
const PERIOD_PAIN: &str = "A warm bath, a wrapped hot-water bottle or gentle exercise may ease period pain. Ask a pharmacist whether a painkiller such as paracetamol or ibuprofen is suitable for you and follow the leaflet. Speak to a GP if the pain regularly stops you doing everyday activities. Ask for an urgent GP appointment or contact NHS 111 if pain is severe or worse than usual and pain relief has not helped.";
const DEPOSIT_CAP: &str = "For tenancies covered by the deposit cap in England, the maximum is five weeks' rent below £50,000 in annual rent and six weeks' rent at £50,000 or more. ";
const DEPOSIT: &str = "For tenancies covered by England's deposit protection rules, the landlord must protect the deposit in an approved scheme and give you the details within 30 days of receiving it. Ask for it back in writing. If you disagree about deductions, use the scheme's dispute resolution service.";
// Registration after the 2024 certification reforms in England and Wales.
// https://www.gov.uk/government/publications/changes-to-the-death-certification-process/an-overview-of-the-death-certification-reforms
// https://www.gov.uk/after-a-death
// https://www.gov.uk/after-a-death/when-a-death-is-reported-to-a-coroner
// https://www.gov.uk/register-a-death
pub(super) const REGISTER_DEATH: &str = concat!(
    "In England and Wales, the medical examiner's office sends the medical certificate of cause of death to the registrar and tells you when you can register. Register within five days of that confirmation. ",
    "If a coroner is handling the death, follow the instructions from the coroner's office or the registrar. The register office can tell you what details to bring. ",
    "GOV.UK's Register a death service explains which process to use in Scotland, Northern Ireland or if the death happened abroad. ",
    "If the person lived in England, Scotland or Wales, ask the registrar about Tell Us Once. Contact their bank and insurers separately."
);
const SERVICE: &str = "Use a Gas Safe registered engineer and follow the appliance manufacturer's servicing instructions. HSE recommends an annual service when those instructions are unavailable, unless a Gas Safe engineer advises otherwise. Landlords must arrange an annual gas safety check and keep the record. Ask the engineer to quote separately for servicing and the safety check.";
const SERVICE_PRICE: &str = "Ask a Gas Safe registered engineer for an itemised quote for the boiler service and, if you need it, the landlord gas safety check. The annual gas safety check is a legal duty for landlords. Servicing is part of maintaining the appliance and follows the manufacturer's instructions.";
const EGG_STORAGE: &str = "Store eggs in their original carton in the fridge and follow the storage instructions on the pack.";
const EGG_FRESHNESS: &str = "Check the best-before date and storage instructions on the carton. A float test estimates an egg's age. Use the carton guidance to decide whether to eat it.";
const BREAD_STORAGE: &str = "Keep bread at room temperature in a paper bag or a bread bin, not the fridge: the cold makes it go stale faster. Freeze what you won't eat in two days, sliced, and bring a stale loaf back with a splash of water and 5 minutes in a hot oven.";
const BREAD_LIFESPAN: &str = "A loaf keeps 3 to 4 days in a paper bag or a bread bin at room temperature, and 3 months sliced in the freezer. The fridge stales it faster than the cupboard, so leave it out and cut slices as you need them, freeze what you won't eat in two days, and a stale loaf comes back with a splash of water and 5 minutes in a hot oven.";
const BREAD_DATE: &str = "Check whether the bread's label says use-by or best before. A use-by date is about safety. If it has passed and the bread wasn't frozen in time, don't eat it even if it looks and smells fine. A best-before date is about quality. Follow the storage instructions, and throw out the whole loaf if you see mould.";
const FRIDGE_STORAGE: &str = "Keep your fridge at 5°C or below. Refrigerate cooked leftovers within two hours and eat them within 48 hours or freeze them. Cool cooked rice quickly, ideally within one hour, and use it within 24 hours if refrigerated. Follow the pack's storage and opening instructions for meat, fish and dairy. If a use-by date has passed, discard the food unless it was cooked or frozen in time. Appearance and smell cannot tell you whether food past its use-by date is safe. A best-before date is about quality.";
const PARKING_DEFINITION: &str =
    "A Penalty Charge Notice (PCN) is a charge for a civil parking or traffic contravention.";
const PARKING_APPEAL: &str = "For a council parking PCN in England or Wales, you have 28 days to challenge it. Check who issued it, follow the instructions on the notice and send any photos or other evidence. If the council rejects a challenge made within 14 days, it may still offer the 50% discount.";
const DIARRHOEA: &str = "Call NHS 111 now if there is blood in your diarrhoea, you keep vomiting and cannot keep fluids down, or diarrhoea has lasted more than seven days. Call 999 or go to A&E for sudden severe tummy pain or vomiting blood. For mild symptoms, take small, frequent sips of water if you feel sick and ask a pharmacist about oral rehydration sachets. Eat when you feel able and stay off school or work until 48 hours after the last episode of diarrhoea or vomiting.";
const DRY_EYE: &str = "Ask for an urgent GP appointment or contact NHS 111 if an eye is painful and red, or if you have a red eye and wear contact lenses. Call 999 or go to A&E if your sight changes, light hurts your eyes, or you have an eye injury. For mild dryness, take out contact lenses and wear glasses, take breaks from screens and ask a pharmacist about lubricating drops. See an optician or GP if symptoms persist.";
const NUT_ALLERGY: &str = "For a peanut or tree nut allergy, check the ingredients and any 'may contain' warnings for each allergen you need to avoid, every time you buy a product. Ask staff about ingredients and cross-contact when eating out. At home, wash hands, worktops and utensils before preparing the food, and keep it separate from anything containing the allergen.";
const CAT_WOUND: &str = "Go to A&E if the wound is large or deep, or the bleeding won't stop. For a small scratch or bite, wash it with soap and warm water, pat it dry and cover it with a clean dressing. Contact NHS 111 for a bite to the hand, foot, face or head, signs of infection such as heat, swelling or leaking fluid, a fever, or if you may need a tetanus booster. Get urgent advice if the eye itself was injured.";
const INFECTED_FINGER: &str = "A clinician can check what is causing the finger symptoms. Ask for an urgent GP appointment or contact NHS 111 if the skin is painful, hot and swollen, the redness is spreading, you have pus or you see a red streak. If you also feel faint or confused, breathe unusually fast or have a very high temperature, call 999 or go to A&E. Keep the finger clean and covered and avoid squeezing or draining it. If there are blisters, contact a GP promptly, as some treatments work best early.";
const VULVAL_ITCH: &str = "Itching or soreness around the vulva can have several causes. See a GP or sexual health clinic if this is the first time, the symptoms are different from usual, there's unusual discharge or it is getting worse. Rinse the outside gently with water, pat dry and avoid scented products. Ask for medical advice promptly if you also feel feverish or have pelvic pain.";
const VULVAL_PATCHES: &str = "See a GP if the skin around the vulva has a white patch, keeps splitting or bleeding, or has a new lump. Lichen sclerosus is one possible cause and a clinician can check what it is and whether you need treatment. Avoid scented products and rubbing the skin. Ask for medical advice promptly if you also feel feverish or have pelvic pain.";
const HERNIA: &str = "A hernia is when tissue pushes through a weak spot in the surrounding muscle or tissue. It can cause a lump that changes when you stand, cough or lie down. See a GP if you think you have one. Call NHS 111 promptly if a known hernia becomes painful or you feel sick, bloated, constipated or feverish. Go to A&E straight away if you have sudden severe pain, the lump becomes firm or cannot go back in, or you cannot pass stool or wind.";
const HIATUS_HERNIA: &str = "A hiatus hernia is when part of the stomach moves up into the chest. It can cause heartburn or reflux. Smaller meals and leaving three to four hours between eating and going to bed may help. Ask for an urgent GP appointment or contact NHS 111 if food sticks when you swallow, you are frequently sick or you lose weight without trying.";
pub(super) const REFUND_OR_REPAIR: &str = "If goods from a UK retailer are faulty or not as described, you can usually reject them for a refund within 30 days. You can also ask for a repair or replacement. After that, the retailer usually gets a chance to repair or replace the item. If that fails, takes too long or causes significant inconvenience, you can usually ask for a price reduction or return it for a refund. Time limits and deductions can vary, including for vehicles.";
const PASTRY_REHEAT: &str = "An oven or air fryer can crisp the pastry. Follow the pack instructions and check that the filling is steaming hot throughout before serving. Reheat leftovers only once.";
const PASTRY_THAW: &str = "Defrost a pie or quiche fully in the fridge unless the pack says it can be cooked from frozen. Follow the pack instructions and use it within 24 hours of fully defrosting.";
const BATTERY_FIRE: &str =
    "If the battery is smoking or on fire, warn people nearby, leave, stay outside and call 999.";
const BATTERY_FAULT: &str = "If the pack is damaged, unusually hot, swollen, leaking, hissing, smelling unusual, or failing to charge properly, stop using it and switch off the charger if you can do so safely. Ask the manufacturer or retailer about checking or replacing it. If it starts smoking or catches fire, leave and call 999 from outside.";
const BATTERY_REPLACEMENT: &str = "Ask the bike manufacturer or a reputable retailer for a compatible replacement and a price for your model. Use a charger approved by the manufacturer. Do not charge or use a battery you suspect is damaged.";
const BATTERY_RANGE: &str = "Check the manufacturer's range and charging-time figures for your model. Terrain and assistance level affect how far a charge will take you. If the battery starts taking much longer to charge or fails to charge properly, stop using it and ask the manufacturer or retailer for advice.";
const BATTERY_LIFE: &str = "Battery life depends on the model, how it is used and how it is stored. Follow the manufacturer's storage and maintenance instructions. If charging performance deteriorates, ask the manufacturer or retailer to check the battery.";
const BATTERY_CHARGING: &str = "Use the supplied or manufacturer-approved charger and follow the instructions for storage and charging. Charge while awake and present, away from escape routes, and unplug when the pack is full. Stop charging and get advice if the battery is damaged or shows signs of a fault.";
const BIKE_BRAKES: &str = "If braking is weak, the lever feels soft or pulls to the handlebar, or the wheel is loose, don't ride until a qualified bike mechanic has checked the bike. Moisture can make disc brakes squeal briefly even when braking is normal. If the noise persists once dry or the brake rubs as the wheel turns, follow the manufacturer's manual for your brake and wheel or have a mechanic inspect them.";

static BATTERY_FIRE_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:smok(?:e|es|ing)|flames?|on fire|caught fire)\b")
        .expect("battery fire wording")
});
static BATTERY_FAULT_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:damag\w*|bulg\w*|swoll\w*|swell\w*|leak\w*|hiss\w*|unusually hot|too hot|strange smell|",
        r"not charg\w*|won'?t charg\w*|will not charg\w*|doesn'?t charg\w*|keeps going flat|",
        r"takes? longer to charge|stopped charging|charger cuts out)\b"
    ))
    .expect("battery fault wording")
});

static WARFARIN_WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bwarfarin\b").expect("warfarin wording"));
static UNFOLLOW: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bunfollow").expect("unfollow wording"));
static BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bblock(?:ed|ing)?\b").expect("block wording"));
static UNFRIEND: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:unfriend|defriend|remove (?:a )?friend)").expect("unfriend wording")
});
static FUEL_OR_SOLVENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:petrol|gasoline|diesel|white spirit|paint thinner|solvents?)\b")
        .expect("fuel and solvent wording")
});
static BREAD_DATE_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:past |after )(?:the|its|their) date|\b(?:use[- ]by|sell[- ]by|best[- ]before|out of date|date on it|date on the|date says|date said|the date)\b")
        .expect("bread date wording")
});
static BREAD_DURATION_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:how long|how many days|how many weeks|shelf[- ]?life|lasts? for)\b")
        .expect("bread duration wording")
});
static VULVAL_PATCH_WORDS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:white patch(?:es)?|white skin|skin is white|white and thin|thin and white|thin skin|",
        r"tears|tearing|splits|splitting|split skin|shiny skin|tight skin|shrunken)\b"
    ))
    .expect("vulval skin changes")
});
static SUDDEN_JOINT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:swoll\w*|swell\w*|fever|feverish|shiver\w*|hot and red|hot to (?:the )?touch|",
        r"cannot (?:move|bear weight)|can'?t (?:move|bear weight)|sudden(?:ly)? (?:severe|very bad) pain)\b"
    )).expect("joint symptoms wording")
});

pub(crate) fn registered(
    domain: &str,
    id: &str,
    reading: &Reading<'_>,
) -> Option<Cow<'static, str>> {
    let reply = match (domain, id) {
        ("health", "hernia") if reading.line.clean.contains("hiatus") => HIATUS_HERNIA,
        ("health", "hernia") => HERNIA,
        ("health", "cataract") => health::cataract(reading),
        ("family", "milk-tooth") => health::milk_tooth(reading),
        ("family", "breastfeeding-basics") => health::breastfeeding(reading),
        ("science", "eyes-test") => return Some(health::sight_test(reading)),
        ("sport", "knee-noise") => health::knee_noise(reading),
        ("sport", "stitch") => {
            "A side stitch is a short-lived pain that can come on during exercise. Its exact cause is uncertain. Slow down or stop and see if it eases. Get medical advice if it keeps coming back or persists after stopping. Get urgent help for sudden severe pain, or chest pain with trouble breathing or faintness."
        }
        ("warm_food", "reheat-pastry") => return Some(reheat_pastry(reading)),
        ("cooking", "produce-brown") => food::browned_produce(reading),
        ("fruit", "fruit-skin-edible") => return Some(food::fruit_skin(reading)),
        ("nutrition", "frozen-and-tinned") => food::frozen_or_tinned(reading),
        ("nutrition", "coffee-good") => food::tea_or_coffee(reading),
        ("body", "painkillers-dose") => health::painkiller_dose(reading)?,
        ("body", "painkiller-stomach") => health::painkiller_effects(reading),
        ("devices", "unfriend-or-block-on-social") => social_account(reading),
        ("odours", "smell-in-a-garment" | "smell-in-a-garment-why")
            if FUEL_OR_SOLVENT.is_match(&reading.line.clean) =>
        {
            "Keep clothing that may be contaminated with fuel or a flammable solvent away from heat, flames and sparks. Check the product's safety instructions. Keep it out of the washing machine and tumble dryer."
        }
        ("odours", "smell-in-a-garment") => {
            "Air the garment first, then follow its care label for washing and drying. If it is dry-clean only, tell a dry cleaner what caused the smell. If the smell comes back after clean clothes go into storage, check the drawer or wardrobe for damp and let it dry before putting them back."
        }
        ("odours", "smell-in-a-garment-why") => {
            "Odour can linger after washing if body oils, detergent residue or damp remain in the fabric. Synthetic fibres can retain body oils. Use the dose on the detergent pack, wash at the temperature on the care label and let the garment dry fully before storing it."
        }
        ("money", "pension") => financial::pension(reading),
        ("money", "missed-payment") => financial::missed_payment(reading),
        ("money", "money-back-from-a-scam") => financial::scam_recovery(reading),
        ("consumer", "refund-or-repair") => REFUND_OR_REPAIR,
        ("travel", "airport-arrival") => return super::airport_arrival::reply(reading),
        ("health", "diarrhoea") => DIARRHOEA,
        ("health", "dry-eye") => DRY_EYE,
        ("cooking", "nut-allergy") => NUT_ALLERGY,
        ("pets", "cat-scratch-wound") => CAT_WOUND,
        ("health", "infected-finger") => INFECTED_FINGER,
        ("complaints", "vulval-itch") => {
            if VULVAL_PATCH_WORDS.is_match(&reading.line.clean) {
                VULVAL_PATCHES
            } else {
                VULVAL_ITCH
            }
        }
        ("bikes", "bike-battery" | "bike-battery-follow-up") => battery(reading),
        ("bikes", "bike-brakes-rub") => BIKE_BRAKES,
        ("health", "medicine-and-a-food" | "medicine-with-food") => {
            if WARFARIN_WORD.is_match(&reading.line.clean) {
                WARFARIN
            } else {
                MEDICINE_FOOD
            }
        }
        ("health", "tablet-side-effects") => {
            let device = reading
                .line
                .clean
                .split(|ch: char| !ch.is_ascii_lowercase() && ch != '\'')
                .any(|word| data::TABLET_DEVICE.contains(&word));
            if device {
                return None;
            }
            SIDE_EFFECTS
        }
        ("body", "arthritis") => {
            if SUDDEN_JOINT.is_match(&reading.line.clean) {
                ACUTE_JOINT
            } else {
                ARTHRITIS
            }
        }
        ("science", "period-pain") => period(reading)?,
        ("money", "rent-deposit") => {
            if reading.features.asks.contains(&"how_much")
                || reading.line.clean.contains("how many")
            {
                return Some(Cow::Owned([DEPOSIT_CAP, DEPOSIT].concat()));
            }
            DEPOSIT
        }
        ("costs", "cost-boiler-service") => {
            let asks = &reading.features.asks;
            if !asks.contains(&"how_often")
                && (asks.contains(&"yesno") || asks.contains(&"recommend"))
            {
                SERVICE
            } else {
                SERVICE_PRICE
            }
        }
        ("cooking", "bread-keep") => {
            let line = &reading.line;
            let matches = |pattern: &Regex| {
                [&line.clean, &line.norm_text]
                    .into_iter()
                    .chain(&line.norm)
                    .any(|view| pattern.is_match(view))
            };
            if matches(&BREAD_DATE_WORDS) {
                BREAD_DATE
            } else if matches(&BREAD_DURATION_WORDS) {
                BREAD_LIFESPAN
            } else {
                BREAD_STORAGE
            }
        }
        ("cooking", "fridge-life" | "fridge-life-cooked") => FRIDGE_STORAGE,
        ("cooking", "egg-fresh") => {
            let words = &reading.line.words;
            let names = |cues: &[&str]| {
                words
                    .iter()
                    .any(|word| cues.iter().any(|cue| word.eq_ignore_ascii_case(cue)))
            };
            let storage_first = names(data::EGG_STORE) || !names(data::EGG_FRESH);
            let parts = if storage_first {
                [EGG_STORAGE, EGG_FRESHNESS]
            } else {
                [EGG_FRESHNESS, EGG_STORAGE]
            };
            return Some(Cow::Owned(parts.join(" ")));
        }
        ("cars", "parking-ticket-appeal") => {
            if reading.features.asks.contains(&"definition")
                && !reading
                    .line
                    .clean
                    .split(crate::text::is_space)
                    .any(|word| matches!(word, "my" | "our"))
            {
                return Some(Cow::Owned([PARKING_DEFINITION, PARKING_APPEAL].join(" ")));
            }
            PARKING_APPEAL
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}

fn social_account(reading: &Reading<'_>) -> &'static str {
    let text = if reading.line.text.is_empty() {
        reading.line.clean.as_str()
    } else {
        reading.line.text.as_str()
    }
    .to_lowercase();
    let unfriend = UNFRIEND.is_match(&text);
    if UNFOLLOW.is_match(&text) && !unfriend {
        return "On Facebook or Instagram, open the person's profile and look for Unfollow under Following. Their posts are removed from your feed and the connection stays in place. You can block the account from its profile if you also want to limit contact.";
    }
    if BLOCK.is_match(&text) && !unfriend {
        return "Open the profile, select the options menu and choose Block. Blocking limits contact and access from that account. Shared spaces and other accounts can still expose some activity, so check your post audience and privacy settings too.";
    }
    "On Facebook, open the person's profile, select Friends and choose Unfriend. That removes the connection. On Instagram, use Following to unfollow. Use Block on the profile to limit contact further, and check which posts are public in your privacy settings."
}

fn reheat_pastry(reading: &Reading<'_>) -> Cow<'static, str> {
    let has = |terms: &[&str]| {
        reading
            .line
            .words
            .iter()
            .any(|word| terms.contains(&word.as_str()))
    };
    if has(&["defrost", "defrosting", "thaw", "thawing"])
        && !has(&[
            "reheat",
            "reheating",
            "heat up",
            "heating up",
            "warm up",
            "warm",
            "warming",
            "hot again",
        ])
    {
        return Cow::Borrowed(PASTRY_THAW);
    }
    if reading.line.clean.contains("frozen")
        || has(&[
            "freezer",
            "defrost",
            "defrosting",
            "defrosted",
            "thaw",
            "thawing",
            "thawed",
        ])
    {
        return Cow::Owned(format!("{PASTRY_REHEAT} {PASTRY_THAW}"));
    }
    Cow::Borrowed(PASTRY_REHEAT)
}

fn battery(reading: &Reading<'_>) -> &'static str {
    let said = reading.line.clean.as_str();
    let has = |phrases: &[&str]| phrases.iter().any(|phrase| said.contains(phrase));
    if BATTERY_FIRE_WORDS.is_match(said) {
        BATTERY_FIRE
    } else if BATTERY_FAULT_WORDS.is_match(said) {
        BATTERY_FAULT
    } else if has(&["cost", "price", "how much", "new battery", "replace"]) {
        BATTERY_REPLACEMENT
    } else if has(&["how far", "range", "miles", "distance"]) {
        BATTERY_RANGE
    } else if has(&["how long", "life", "last", "years", "how many charges"]) {
        BATTERY_LIFE
    } else {
        BATTERY_CHARGING
    }
}

pub(super) fn bike_battery_context(reading: &Reading<'_>, earlier: &[Exchange]) -> bool {
    // Bicycle words from the pinned follow-up, with word boundaries for words like "chain".
    static BIKE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(concat!(
            r"\b(?:bike|bicycle|cycle|cyclist|pushbike|push bike|e-?bike|e bike|two wheels|",
            r"cycling|biking|handlebar|crank|pedal|saddle|pannier|drop bars|flat bars|riser bars|",
            r"bar ends|aerobars|bullhorn bars|top tube|chainring|derailleur|inner tube|tubeless|",
            r"quick release|chain)s?\b"
        ))
        .expect("bicycle subjects")
    });
    static OTHER_BATTERY: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(concat!(
            r"\b(?:phones?|mobiles?|smartphone|iphone|ipad|tablet|laptop|macbook|",
            r"power bank|remote|watch|hearing aid|smoke alarm)\b"
        ))
        .expect("other battery subjects")
    });
    let subject = |line: &str| {
        if BIKE.is_match(line) {
            Some(true)
        } else if OTHER_BATTERY.is_match(line) {
            Some(false)
        } else {
            None
        }
    };
    subject(&reading.line.clean)
        .or_else(|| {
            earlier
                .iter()
                .rev()
                .find_map(|turn| subject(&turn.user.to_lowercase()))
        })
        .unwrap_or(false)
}

fn period(reading: &Reading<'_>) -> Option<&'static str> {
    let said = format!(" {} ", reading.line.clean);
    let names = |pool: &[&str]| {
        pool.iter().any(|word| {
            reading.line.words.iter().any(|token| token == word)
                || said.match_indices(word).any(|(offset, found)| {
                    said[..offset].ends_with(' ') && said[offset + found.len()..].starts_with(' ')
                })
        })
    };
    if names(data::NOT_THIS_PERSONS_PERIOD) {
        None
    } else if names(data::HEAVY_PERIOD_WORDS) {
        Some(HEAVY_PERIOD)
    } else if names(data::PERIOD_CHANGE_WORDS) {
        Some(PERIOD_CHANGE)
    } else {
        Some(PERIOD_PAIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn bike_battery_followup_reuses_guidance_in_bike_context() {
        let reading = Reading::new("And how much is a new battery?");
        assert!(!bike_battery_context(&reading, &[]));
        let earlier = [Exchange {
            user: "How far can an e-bike go on one charge?".into(),
            bot: String::new(),
            module: "answers",
        }];
        assert!(bike_battery_context(&reading, &earlier));
        assert!(!bike_battery_context(
            &Reading::new("How much is a new battery for my phone?"),
            &earlier,
        ));
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn(&earlier[0].user);
        let turn = bot.turn(reading.raw);
        assert_eq!(turn.reply, BATTERY_REPLACEMENT);
    }

    #[test]
    fn storage_dates_keep_the_original_cards_and_distinguish_the_labels() {
        for (prompt, expected) in [
            ("Is bread OK to eat past the date?", BREAD_DATE),
            ("Can I eat a loaf past the use by date?", BREAD_DATE),
            ("How do I keep my bread in the fridge?", BREAD_STORAGE),
            ("How long does a loaf keep?", BREAD_LIFESPAN),
            ("How long is food safe in the fridge?", FRIDGE_STORAGE),
        ] {
            let mut bot = Bot::new(Some("0")).unwrap();
            let turn = bot.turn(prompt);
            assert_eq!(turn.module, Some("answers"), "{prompt}: {turn:?}");
            assert_eq!(turn.reply, expected, "{prompt}");
        }
        for card in ["fridge-life", "fridge-life-cooked"] {
            assert_eq!(
                registered("cooking", card, &Reading::new("fridge")).as_deref(),
                Some(FRIDGE_STORAGE),
            );
        }
    }

    #[test]
    fn approved_clinical_triggers_reach_the_native_answer_path() {
        for (prompt, domain, id, expected) in [
            (
                "How do I treat a deep cat scratch on my hand?",
                "pets",
                "cat-scratch-wound",
                CAT_WOUND,
            ),
            (
                "What do I do about an infected fingernail?",
                "health",
                "infected-finger",
                INFECTED_FINGER,
            ),
            (
                "My vulva is itchy.",
                "complaints",
                "vulval-itch",
                VULVAL_ITCH,
            ),
            (
                "The skin around my vulva has itchy white patches.",
                "complaints",
                "vulval-itch",
                VULVAL_PATCHES,
            ),
        ] {
            let card = crate::lookup_knowledge(prompt).expect("source clinical card");
            assert_eq!((card.domain, card.id), (domain, id), "{prompt}");
            assert_eq!(card.reply, expected, "{prompt}");
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            assert!(
                turn.candidates
                    .iter()
                    .any(|candidate| candidate.module == "answers" && candidate.reply == expected),
                "{prompt}: {turn:?}"
            );
        }
        let unrelated = Bot::new(Some("0"))
            .unwrap()
            .turn("I scratched my own arm and it's bleeding.");
        assert!(
            unrelated
                .candidates
                .iter()
                .all(|candidate| candidate.reply != CAT_WOUND)
        );
    }

    #[test]
    fn bicycle_brake_guidance_reaches_fault_reports() {
        for prompt in [
            "My bicycle brakes squeak when wet.",
            "My bike brakes rub.",
            "Bike brakes are soft and do not stop when wet.",
        ] {
            let card = crate::lookup_knowledge(prompt).unwrap();
            assert_eq!((card.domain, card.id), ("bikes", "bike-brakes-rub"));
            assert_eq!(card.reply, BIKE_BRAKES);
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            assert_eq!(turn.module, Some("answers"), "{prompt:?}");
            assert_eq!(turn.reply, BIKE_BRAKES, "{prompt:?}");
        }
        for prompt in ["My bicycle brakes squeak.", "My motorbike brakes rub."] {
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            assert!(turn.candidates.iter().all(|part| part.reply != BIKE_BRAKES));
        }
        let downhill = crate::lookup_knowledge("My bicycle brakes when I go downhill.").unwrap();
        assert_eq!(downhill.id, "whenbuy-bike");
    }

    #[test]
    fn nut_allergy_reaches_the_original_card_and_respects_gift_and_pet_guards() {
        for prompt in [
            "I have a nut allergy.",
            "My favourite fruit is a mango, but I can't eat peanuts.",
        ] {
            let card = crate::lookup_knowledge(prompt).expect("source nut allergy card");
            assert_eq!((card.domain, card.id), ("cooking", "nut-allergy"));
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            assert_eq!(turn.module, Some("answers"), "{prompt}");
            assert_eq!(turn.reply, NUT_ALLERGY, "{prompt}");
        }
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("My favourite fruit is a mango, but I can't eat peanuts.");
        let recall = bot.turn("What is my favourite fruit?");
        assert_eq!(recall.module, Some("favourites"));
        assert!(recall.reply.contains("mango"), "{}", recall.reply);
        for prompt in [
            "What present can I buy for someone with a nut allergy?",
            "Can my dog eat peanuts?",
        ] {
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            assert!(
                turn.candidates
                    .iter()
                    .all(|candidate| candidate.reply != NUT_ALLERGY),
                "{prompt}"
            );
        }
    }

    #[test]
    fn approved_replies_track_the_cue_and_supported_scope() {
        let scenarios = [
            (
                "health",
                "medicine-and-a-food",
                "can i drink cranberry juice on warfarin",
                "avoid cranberry juice and grapefruit juice",
            ),
            (
                "health",
                "medicine-and-a-food",
                "can i take my tablet with milk",
                "exact medicine and ask a pharmacist",
            ),
            (
                "body",
                "arthritis",
                "my knee is swollen and hot is it arthritis",
                "urgent GP appointment",
            ),
            (
                "body",
                "arthritis",
                "what helps arthritis in my knee",
                "Regular gentle activity",
            ),
            (
                "science",
                "period-pain",
                "my periods are very heavy",
                "If heavy periods affect your daily life",
            ),
            (
                "science",
                "period-pain",
                "my period cramps really hurt",
                "whether a painkiller",
            ),
            (
                "money",
                "rent-deposit",
                "how many weeks can they ask for my rent deposit",
                "six weeks' rent at £50,000",
            ),
            (
                "costs",
                "cost-boiler-service",
                "is it worth getting the boiler serviced",
                "Landlords must arrange an annual gas safety check",
            ),
            (
                "costs",
                "cost-boiler-service",
                "how much does a boiler service cost",
                "itemised quote",
            ),
            (
                "health",
                "medicine-with-food",
                "do i need to eat before my medication",
                "Check the leaflet for your exact medicine",
            ),
            (
                "health",
                "tablet-side-effects",
                "this prescription makes me nauseous",
                "vomiting or missed-pill instructions for your exact pill",
            ),
            (
                "cooking",
                "egg-fresh",
                "how do i tell if my eggs are still fresh",
                "A float test estimates an egg's age",
            ),
            (
                "cars",
                "parking-ticket-appeal",
                "can I dispute my parking ticket",
                "you have 28 days to challenge it",
            ),
        ];
        for (domain, id, prompt, expected) in scenarios {
            let reply = registered(domain, id, &Reading::new(prompt)).unwrap();
            assert!(reply.contains(expected), "{prompt}: {reply:?}");
        }
        assert!(
            registered(
                "science",
                "period-pain",
                &Reading::new("my sister has period pain")
            )
            .is_none()
        );
        assert!(
            registered(
                "health",
                "tablet-side-effects",
                &Reading::new("my ipad tablet is giving me headaches")
            )
            .is_none()
        );
    }

    #[test]
    fn approved_replacements_reach_the_answer_channel() {
        for (prompt, cue) in [
            (
                "can i drink cranberry juice on warfarin",
                "avoid cranberry juice and grapefruit juice",
            ),
            (
                "my knee is hot and swollen and i think its arthritis",
                "urgent GP appointment",
            ),
            ("what can i do for period pain", "whether a painkiller"),
            (
                "how many weeks can they ask for my rent deposit",
                "six weeks' rent at £50,000",
            ),
            (
                "is it worth getting the boiler serviced",
                "Landlords must arrange an annual gas safety check",
            ),
            (
                "do i need to eat before my medication",
                "exact medicine and ask a pharmacist",
            ),
            (
                "how do i tell if my eggs are still fresh",
                "A float test estimates an egg's age",
            ),
            (
                "what is a penalty charge notice",
                "you have 28 days to challenge it",
            ),
            (
                "I have bloody diarrhoea, what should I do?",
                "Call NHS 111 now",
            ),
            (
                "How can I help my dry eyes?",
                "you have a red eye and wear contact lenses",
            ),
            ("Why will my e-bike battery not charge?", "stop using it"),
            (
                "What range does an e-bike battery get on one charge?",
                "range and charging-time figures for your model",
            ),
        ] {
            let turn = Bot::new(Some("0")).unwrap().turn(prompt);
            assert_eq!(turn.module, Some("answers"), "{prompt}");
            assert!(turn.reply.contains(cue), "{prompt}: {}", turn.reply);
            assert!(
                turn.candidates
                    .iter()
                    .any(|candidate| candidate.module == "answers" && candidate.reply == turn.reply),
                "{prompt}"
            );
        }
    }

    #[test]
    fn battery_guidance_prioritizes_damage_and_smoke_over_a_price_or_range_request() {
        for (prompt, expected) in [
            (
                "What range should I expect if my e-bike battery is leaking?",
                BATTERY_FAULT,
            ),
            (
                "My e-bike battery is smoking. How much is a replacement?",
                BATTERY_FIRE,
            ),
            (
                "How much is a replacement e-bike battery?",
                BATTERY_REPLACEMENT,
            ),
            ("How should I charge my e-bike battery?", BATTERY_CHARGING),
        ] {
            let reading = Reading::new(prompt);
            assert_eq!(
                registered("bikes", "bike-battery", &reading).as_deref(),
                Some(expected)
            );
        }
    }

    #[test]
    fn registering_a_death_uses_the_current_medical_examiner_process() {
        use crate::bot::knowledge::lookup;

        let ask = "How do I register a death?";
        let card = lookup::lookup(ask).unwrap();
        assert_eq!((card.domain, card.id), ("lifeadmin", "register-death"));
        assert_eq!(card.reply, REGISTER_DEATH);
        assert_eq!(
            lookup::by_id("lifeadmin", "register-death").unwrap().reply,
            REGISTER_DEATH
        );

        let turn = Bot::new(Some("0")).unwrap().turn(ask);
        assert_eq!(turn.module, Some("answers"), "{turn:?}");
        assert_eq!(turn.reply, REGISTER_DEATH);
        assert!(
            lookup::lookup("How do I register to vote?")
                .is_none_or(|card| card.id != "register-death")
        );
    }
}
