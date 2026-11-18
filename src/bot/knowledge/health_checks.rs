//! Measurement, results and specialist-device cards.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(TEMPERATURE_NOUN, super::HEALTH_TEMPERATURE_NOUN);
pattern!(ASK_LOW, super::HEALTH_TEMPERATURE_LOW);
pattern!(BLOOD_PRESSURE, super::HEALTH_BP_FIGURES);
pattern!(PROSTHESIS_GIVING_WAY, super::HEALTH_LIMB_GIVING_WAY);
pattern!(PROSTHESIS_HURTS, super::HEALTH_LIMB_HURTS);
pattern!(PROSTHESIS_CLEANING, super::HEALTH_LIMB_CLEANING);
pattern!(COUGH_AGE, super::HEALTH_COUGH_AGE);
pattern!(NAME_TEST, super::HEALTH_TEST_NAME);
pattern!(RULES_OUT, super::HEALTH_TEST_RULES_OUT);
pattern!(RESULT_CLEAR, super::HEALTH_TEST_CLEAR);
pattern!(RESULT_ABNORMAL, super::HEALTH_TEST_ABNORMAL);
pattern!(ASK_PERMISSION, super::HEALTH_TEST_PERMISSION);
pattern!(ASK_RETURN, super::HEALTH_TEST_RETURN);
pattern!(CHECK_RESULT, super::HEALTH_TEST_CHECK);
pattern!(MEDICINE, super::HEALTH_TEST_MEDICINE);
pattern!(DRIVING, super::HEALTH_TEST_DRIVING);
pattern!(SPORT, super::HEALTH_TEST_SPORT);
pattern!(WORK, super::HEALTH_TEST_WORK);
pattern!(PET, r"\b(?:vet|vets)\b");
pattern!(
    SAYS_CLEAR,
    r"\b(?:clear|all clear|normal|fine|nothing there|nothing found|no problems|nothing showed up|(?:she|he)'?s fine|(?:she|he) is fine)\b"
);
pattern!(SAYS_NEGATIVE, r"\b(?:negative|not detected|no trace)\b");
pattern!(
    SAYS_ABNORMAL,
    r"\b(?:abnormal|borderline|flagged|outside the range|out of range|high|low|showed something|shows something|showed up)\b"
);
pattern!(
    NEEDS_FOLLOWUP,
    r"\b(?:need(?:s)? (?:a )?follow up|want(?:s)? to see|asked me to come|come back in|another appointment|appointment)\b"
);
pattern!(
    CHASE,
    r"\b(?:follow up|follow-up|chase|chasing|ring|phone|call|ask|heard|hear)\b"
);
pattern!(
    RESULT_ASK,
    r"\b(?:result|results|follow up|follow-up|came back)\b"
);
pattern!(RATE_RANGE, r"(\d{2,3})\s*(?:to|-|–)\s*(\d{2,3})");

fn has(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn temperatures<'a>(reading: &'a Reading<'_>) -> impl Iterator<Item = f64> + 'a {
    reading
        .numbers
        .iter()
        .filter(|n| {
            matches!(
                n.unit,
                None | Some("fahrenheit" | "celsius" | "degree" | "degrees")
            )
        })
        .map(|n| {
            if n.unit == Some("fahrenheit") || (n.unit != Some("celsius") && n.value >= 45.0) {
                (n.value - 32.0) * 5.0 / 9.0
            } else {
                n.value
            }
        })
}

pub(super) fn body_temperature_number(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    TEMPERATURE_NOUN.is_match(text)
        || (!has(text, super::HEALTH_TEMPERATURE_NONBODY)
            && temperatures(reading).any(|value| (34.0..=42.5).contains(&value)))
}

const FEVER: &str = "Rest and drink enough fluids if you can. Get urgent advice if the fever is getting worse or will not settle. Call 999 for difficulty breathing, a stiff neck with fever, or a rash that does not fade when pressed. For a baby under three months, contact NHS 111 or your GP urgently if the temperature is 38°C or higher.";

fn temperature(reading: &Reading<'_>) -> Cow<'static, str> {
    let Some(value) = temperatures(reading).next() else {
        return Cow::Borrowed(if ASK_LOW.is_match(&reading.line.clean) {
            "Body temperature varies by person and how it was measured. Below 35°C can be hypothermia, which is an emergency. Call 999 if you suspect it."
        } else {
            "Normal body temperature varies by person and time of day. A temperature of 38°C or higher usually means a fever."
        });
    };
    let rounded = format!("{value:.1}");
    let display = rounded.trim_end_matches(".0");
    if value < 35.0 {
        Cow::Owned(format!(
            "{display}°C is below 35°C. If that is a body temperature, call 999 for possible hypothermia. Keep the person warm and wait for help."
        ))
    } else if value >= 38.0 {
        Cow::Owned(format!("{display}°C is in the fever range. {FEVER}"))
    } else if value < 36.0 {
        Cow::Owned(format!(
            "{display}°C is a low reading. Recheck using the thermometer instructions, and get urgent advice if the person seems unwell, especially a baby. Call 999 if you suspect hypothermia or the person is hard to wake."
        ))
    } else if value >= 37.5 {
        Cow::Owned(format!(
            "{display}°C is below the usual 38°C fever threshold. Someone can still be unwell or feel feverish below that number. Recheck if symptoms continue and seek advice if they are getting worse."
        ))
    } else {
        Cow::Owned(format!(
            "{display}°C is within the usual range. Check how the person feels as well as the number. Seek help if they have serious symptoms."
        ))
    }
}

fn bp(reading: &Reading<'_>) -> Cow<'static, str> {
    const REGULAR: &str = "High blood pressure is generally 135/85 or higher at home or 140/90 or higher in a clinic. Ask your GP how to measure it over several days and share the readings for review.";
    let Some(values) = BLOOD_PRESSURE.captures(&reading.line.clean) else {
        return Cow::Borrowed(REGULAR);
    };
    let Ok(top) = values[1].parse::<u16>() else {
        return Cow::Borrowed(REGULAR);
    };
    let Ok(bottom) = values[2].parse::<u16>() else {
        return Cow::Borrowed(REGULAR);
    };
    let answer = if top >= 180 || bottom >= 120 {
        "is very high. Call 999 if you have ongoing chest pain, trouble breathing, or possible stroke symptoms. Otherwise repeat it after resting and get urgent medical advice if it stays this high."
    } else if top < 90 || bottom < 60 {
        "is in the low range. Ask your GP if you keep feeling dizzy or faint, or if your readings are repeatedly low."
    } else if top >= 140 || bottom >= 90 {
        "is above the usual home and clinic thresholds. Ask your GP how to repeat and record it for assessment."
    } else if top >= 135 || bottom >= 85 {
        "is above the usual threshold for a home reading. Follow your GP's advice on repeat checks and share the results."
    } else {
        "is below the usual home high-blood-pressure threshold. Keep following any monitoring plan from your clinician, especially if you have symptoms."
    };
    Cow::Owned(format!("{top} over {bottom} {answer}"))
}

const RATE_TAIL: &str = "For most adults, the resting range is around 60 to 100 beats per minute. Check again when resting and contact your GP for repeated unusual readings or palpitations. Call 999 if palpitations are happening with chest pain, shortness of breath, feeling faint or fainting.";

fn pulse(reading: &Reading<'_>) -> Cow<'static, str> {
    let text = &reading.line.clean;
    if text.split_whitespace().any(|word| {
        [
            "baby", "child", "children", "kid", "kids", "boy", "girl", "son", "daughter", "teen",
            "teenager",
        ]
        .contains(&word)
    }) {
        return Cow::Borrowed(
            "A child's usual heart rate depends on their age and how active or unwell they are. If a reading is unusual and they are unwell, seek medical advice. Call 999 if they have difficulty breathing, turn blue or grey, or are hard to wake.",
        );
    }
    let asks_meaning = has(
        text,
        &[
            "mean",
            "stand for",
            "what does",
            "whats",
            "short for",
            "abbreviation",
        ],
    );
    let device = has(
        text,
        &["watch", "fitbit", "garmin", "tracker", "band", " app"],
    );
    let given = reading
        .numbers
        .iter()
        .find(|n| (25.0..=250.0).contains(&n.value))
        .map(|n| n.value.trunc());
    let lead = if asks_meaning && has(text, &["bpm", "beats per minute"]) {
        "BPM means beats per minute.".to_owned()
    } else if asks_meaning && (text.split_whitespace().any(|w| w == "hr") || device) {
        "HR means heart rate.".to_owned()
    } else if RATE_RANGE.is_match(text) {
        "A range gives more context than a single reading.".to_owned()
    } else if let Some(given) = given {
        let watch = if device { " on a watch" } else { "" };
        format!(
            "A heart rate of {given} beats per minute{watch} is one measurement. Activity, illness and other factors can change the rate."
        )
    } else {
        "Measure your heart rate after sitting quietly.".to_owned()
    };
    Cow::Owned(format!("{lead} {RATE_TAIL}"))
}

const CLEAR: &str = "That sounds reassuring for what the test checked. Ask the clinician whether any follow-up is planned, and mention symptoms that persist, worsen or return.";
const ABNORMAL: &str = "Ask the clinician what the abnormal result means in your situation, how soon it needs follow-up, and whether you need another test. A result outside the reference range can have different causes.";
const NEGATIVE: &str = "A negative result means the test did not detect what it was looking for. What it rules out depends on the test and when it was taken. Follow the clinician's advice if symptoms continue.";
const NEITHER: &str = "Ask the service that ordered the test when the result will be available and how they will tell you. Contact them if it is overdue or you need help understanding it.";
const PET_CLEAR: &str = "That sounds reassuring for what the vet tested. Ask whether they need a follow-up and contact the vet if the symptoms persist or worsen.";
const PET_ABNORMAL: &str = "Call the vet to ask what the result means for your pet, what follow-up they recommend and how soon. Tell them whether your pet's symptoms have changed.";

fn prior_reply(earlier: &[Exchange]) -> &str {
    earlier
        .iter()
        .rev()
        .take(2)
        .find_map(|turn| (!turn.bot.is_empty()).then_some(turn.bot.as_str()))
        .unwrap_or_default()
}

fn result(id: &str, reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let text = reading.line.text.to_lowercase();
    let previous = prior_reply(earlier).to_lowercase();
    let clear = previous.starts_with("that sounds reassuring") || RESULT_CLEAR.is_match(&previous);
    if id == "after-a-clear-result" {
        if !clear || !ASK_PERMISSION.is_match(&text) || !ASK_RETURN.is_match(&text) {
            return None;
        }
        return Some(if MEDICINE.is_match(&text) {
            "Check with your prescriber before stopping, restarting or changing your medicine, even after a clear result."
        } else if DRIVING.is_match(&text) {
            "Ask the clinician if you are fit to drive and check any DVLA restrictions for the condition. A clear result by itself does not settle when you can drive."
        } else if SPORT.is_match(&text) {
            "Ask the treating team when you can return to exercise, especially if you still have symptoms or were told to rest. Return gradually once they say it is safe."
        } else if WORK.is_match(&text) {
            "Ask your clinician when you can return to work. Tell them what your job involves and whether you need a fit note or adjusted duties."
        } else {
            "Check with the clinician who requested the test before changing the plan they gave you."
        });
    }
    if id == "thats-good-then" {
        return (clear
            && text.split_whitespace().count() <= 6
            && CHECK_RESULT.is_match(text.trim()))
        .then_some(CLEAR);
    }
    let pet = PET.is_match(&text);
    let cleared = if pet { PET_CLEAR } else { CLEAR };
    let abnormal = if pet { PET_ABNORMAL } else { ABNORMAL };
    if RULES_OUT.is_match(&text) || (NAME_TEST.is_match(&text) && SAYS_CLEAR.is_match(&text)) {
        Some(cleared)
    } else if NAME_TEST.is_match(&text) && SAYS_NEGATIVE.is_match(&text) {
        Some(NEGATIVE)
    } else if (NAME_TEST.is_match(&text) && SAYS_ABNORMAL.is_match(&text))
        || RESULT_ABNORMAL.is_match(&previous)
    {
        Some(abnormal)
    } else if clear {
        Some(cleared)
    } else if NEEDS_FOLLOWUP.is_match(&text) {
        Some(abnormal)
    } else if CHASE.is_match(&text) {
        Some(NEITHER)
    } else {
        RESULT_ASK.is_match(&text).then_some(abnormal)
    }
}

pub(super) fn reply(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    let reply = match id {
        "toothache-dentist" => {
            if super::pregnancy_cards::said_pregnant(reading) {
                "Contact your dentist for a toothache during pregnancy and tell them you are pregnant. Ask a pharmacist or clinician what pain relief is suitable. Call 999 or go to A&E if swelling makes it hard to breathe, swallow or speak."
            } else {
                "See a dentist if toothache lasts more than two days, does not settle with pain relief, or comes with a swollen cheek or jaw. Ask a pharmacist which pain relief is suitable. Call 999 or go to A&E if swelling makes it hard to breathe, swallow or speak."
            }
        }
        "glass-eye-sore" => {
            if has(text, super::HEALTH_EYE_URGENT) {
                "Contact your ocular prosthetist or GP if the socket is painful, swollen or red, or if discharge changes colour. Ask them how to care for the prosthesis while it is sore."
            } else {
                "Follow your ocular prosthetist's cleaning and removal instructions. Wash your hands before handling the artificial eye. Ask your prosthetist for help if removing it is painful or difficult."
            }
        }
        "prosthesis-sore" => {
            if PROSTHESIS_GIVING_WAY.is_match(text) {
                "Stop using the prosthesis if it gives way or is unsafe to stand on. Use a safe mobility aid if you have one and contact your prosthetic team for a check. Leave adjustments to the team."
            } else if PROSTHESIS_HURTS.is_match(text) || !PROSTHESIS_CLEANING.is_match(text) {
                "Take the prosthesis off if it is causing skin irritation or pain, check the skin and contact your prosthetic team. Ask for urgent medical advice if the area is increasingly red, hot or swollen, or you feel unwell."
            } else {
                "Follow your prosthetic unit's instructions for washing and drying the liner and socket. Check your skin daily and contact the unit if you notice sores or persistent redness."
            }
        }
        "body-temperature" => return Some(temperature(reading)),
        "bp-reading" => return Some(bp(reading)),
        "heart-rate-on-a-watch" => return Some(pulse(reading)),
        "a-grown-persons-cough"
            if has(text, &["cough", "coughing", "coughed"])
                && !has(text, super::HEALTH_COUGH_CHILD)
                && !COUGH_AGE
                    .captures_iter(text)
                    .any(|age| age[1].parse::<u8>().is_ok_and(|age| age < 18))
                && has(text, super::HEALTH_COUGH_RELATION) =>
        {
            "Most coughs get better within three to four weeks. Rest, drink fluids and ask a pharmacist about symptom relief. See a GP if it lasts more than three weeks. Get urgent help if they cough up blood, have chest pain, cannot stop coughing, or have difficulty breathing."
        }
        "result-came-back-clear" | "after-a-clear-result" | "thats-good-then" => {
            result(id, reading, earlier)?
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}
