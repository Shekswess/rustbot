//! Body registry guards and clinical replies with explicit escalation boundaries.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(PIERCING, super::BODY_TRIAGE_PIERCING);
pattern!(PIERCING_TROUBLE, super::BODY_TRIAGE_PIERCING_TROUBLE);
pattern!(
    PIERCING_NOT_TROUBLE,
    super::BODY_TRIAGE_PIERCING_NOT_TROUBLE
);
pattern!(PIERCING_BUMP, super::BODY_TRIAGE_PIERCING_BUMP);
pattern!(CARTILAGE, super::BODY_TRIAGE_CARTILAGE);
pattern!(DENTIST_CAREER, super::BODY_TRIAGE_DENTIST_CAREER);
pattern!(DENTIST_ERRAND, super::BODY_TRIAGE_DENTIST_ERRAND);
pattern!(DENTIST_QUEUE, super::BODY_TRIAGE_DENTIST_QUEUE);
pattern!(VISIT_LENGTH, super::BODY_TRIAGE_VISIT_LENGTH);
pattern!(WEEKS, super::BODY_TRIAGE_WEEKS);
pattern!(SWELLING, super::BODY_TRIAGE_SWELLING);
pattern!(OLD_HURT, super::BODY_TRIAGE_OLD_HURT);
pattern!(TAMPON, super::BODY_TRIAGE_TAMPON);
pattern!(TAMPON_DISPOSAL, super::BODY_TRIAGE_TAMPON_DISPOSAL);
pattern!(MOLE_CHECK_METHOD, super::BODY_TRIAGE_MOLE_CHECK);
pattern!(SEA_WATER_ALARM, super::BODY_TRIAGE_SEA_WATER_ALARM);
pattern!(WORD, r"[a-z']+");

fn has(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn said<'a>(reading: &'a Reading<'_>) -> &'a str {
    &reading.line.clean
}

fn mentions(reading: &Reading<'_>, options: &[&str]) -> bool {
    reading
        .line
        .words
        .iter()
        .any(|word| options.contains(&word.as_str()))
}

fn whole(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| {
        text.match_indices(term).any(|(start, hit)| {
            let end = start + hit.len();
            !text.as_bytes()[..start]
                .last()
                .is_some_and(u8::is_ascii_alphabetic)
                && !text
                    .as_bytes()
                    .get(end)
                    .is_some_and(u8::is_ascii_alphabetic)
        })
    })
}

pub(super) fn pregnancy_weeks(text: &str) -> bool {
    WEEKS.is_match(text) && SWELLING.is_match(text)
}

pub(super) fn gate(id: &str, reading: &Reading<'_>) -> bool {
    let text = said(reading);
    match id {
        "piercing-healing" => PIERCING_BUMP.is_match(text) || !PIERCING_TROUBLE.is_match(text),
        "piercing-infection" => {
            PIERCING.is_match(text)
                && PIERCING_TROUBLE.is_match(text)
                && !PIERCING_NOT_TROUBLE.is_match(text)
        }
        "register-dentist" => {
            !DENTIST_CAREER.is_match(text)
                && (DENTIST_ERRAND.is_match(text)
                    || DENTIST_QUEUE.is_match(text)
                    || VISIT_LENGTH.is_match(text))
        }
        "body-see-today" => super::body::person_part(reading),
        "pregnant-swollen-ankles-in-weeks" => pregnancy_weeks(text),
        "back-pain-pregnant" => super::body::back_is_a_body_part(reading),
        "hurt-limb-someone" => {
            if !super::body::person_part(reading) || OLD_HURT.is_match(text) {
                return false;
            }
            let tokens = &reading.line.words;
            if !tokens
                .iter()
                .any(|word| super::BODY_TRIAGE_SOMEONE_ELSE.contains(&word.trim_end_matches("'s")))
            {
                return false;
            }
            let Some(part) = tokens
                .iter()
                .position(|word| super::BODY_TRIAGE_SOFT_TISSUE.contains(&word.as_str()))
            else {
                return false;
            };
            mentions(reading, super::BODY_TRIAGE_HAPPENED)
                || text.contains("went over")
                || tokens[..part]
                    .iter()
                    .any(|word| matches!(word.as_str(), "hurt" | "hurting"))
        }
        "see-about-a-part" => {
            !mentions(reading, &["back"])
                || reading.line.words.iter().any(|word| {
                    word != "back" && super::BODY_TRIAGE_DECISION_PARTS.contains(&word.as_str())
                })
                || super::body::back_is_a_body_part(reading)
        }
        "see-about-it" => {
            let mut words = WORD.find_iter(text).peekable();
            words.peek().is_some()
                && words.all(|word| super::BODY_TRIAGE_DECISION_WORDS.contains(&word.as_str()))
        }
        "tampon-how-long" => TAMPON.is_match(text) && !TAMPON_DISPOSAL.is_match(text),
        _ => false,
    }
}

const BACK_EMERGENCY: &str = "Go to A&E now if you have back pain with new numbness around your genitals or anus, trouble peeing or controlling your bowel or bladder, or weakness, numbness or tingling in both legs. Do not drive yourself. Call 999 if you need an ambulance.";
const MOLE_CHANGED: &str = "Arrange a GP appointment for a mole that has changed size, shape or colour, or is painful, itchy, inflamed, bleeding or crusted. A clinician can examine it and arrange referral if needed.";
const MOLE_CHECK: &str = "Check for changes in size, shape and colour, an uneven border, or bleeding, crusting or itching. Use a mirror or ask someone to help with areas you cannot see. Contact your GP about an unusual new mark or a mole that has changed.";
const PIERCING_CARE: &str = "Wash your hands before touching it and follow the written aftercare instructions. Leave the jewellery in unless a clinician tells you to take it out. If the area becomes hot, increasingly painful or swollen, leaks pus, or you feel unwell, contact your GP urgently or use NHS 111.";
const PET_SEA_WATER: &str = "Call your vet now if your pet drank more than a few licks of seawater, or if they are vomiting, weak, wobbly, unusually sleepy or having tremors. Tell the vet how much they might have drunk and when, and follow their instructions about water and transport.";

fn red_flag(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let cleaned = said(reading).replace(['\'', '’'], "");
    let lower_numbness = whole(&cleaned, super::BODY_TRIAGE_BACK)
        && whole(&cleaned, super::BODY_TRIAGE_NUMB)
        && whole(&cleaned, super::BODY_TRIAGE_NUMB_LOWER);
    if whole(&cleaned, super::BODY_TRIAGE_SPELLED)
        || lower_numbness
        || whole(&cleaned, super::BODY_TRIAGE_LOWER_HALF)
    {
        return Some(Cow::Borrowed(BACK_EMERGENCY));
    }
    if whole(&cleaned, super::BODY_TRIAGE_SOMEWHERE_ELSE) {
        return None;
    }
    Some(Cow::Borrowed(BACK_EMERGENCY))
}

fn dentist(reading: &Reading<'_>) -> &'static str {
    let text = said(reading);
    let queue = mentions(
        reading,
        &["wait", "waiting", "soon", "queue", "see", "seen"],
    ) || text.contains("to get");
    let register = has(text, &["register", "sign up", "join", "new patients"]);
    if mentions(reading, &["crown", "crowns"])
        && mentions(reading, &["last", "lasts"])
        && !mentions(reading, &["take", "takes", "taking"])
        && !queue
        && !register
    {
        "A crown's lifespan depends on the tooth, the crown and how it is looked after. Ask your dentist what to expect and get it checked if it becomes loose or painful."
    } else if !queue && !register && VISIT_LENGTH.is_match(text) {
        if mentions(
            reading,
            &["crown", "crowns", "implant", "implants", "canal"],
        ) {
            "Ask the dental practice how long to allow and how many visits you will need. A crown, implant or root canal often takes more than one appointment, depending on the treatment plan."
        } else {
            "Ask the dental practice how long they have booked for the check-up, filling or hygienist visit. Tell them when booking if there is more than one problem to look at."
        }
    } else if queue && !register {
        "Ask the practice for its next available appointment and whether it has a cancellation list. If you need urgent dental care and cannot get an appointment, use NHS 111."
    } else {
        "Use the NHS Find a dentist service and contact practices directly to ask whether they are taking NHS patients. If you cannot find urgent dental care or need help out of hours, use NHS 111."
    }
}

fn see_today(reading: &Reading<'_>) -> Cow<'static, str> {
    let part = super::body::part_named_with_background(reading);
    if part.is_none_or(|part| super::BODY_TRIAGE_CLOT_CHEST.contains(&part)) {
        return Cow::Borrowed(
            "Contact NHS 111 or your GP urgently for unexplained one-sided leg swelling or pain, or pain when you breathe. Call 999 if you have sudden shortness of breath, chest pain, or cough up blood with breathing difficulty.",
        );
    }
    let part = part.unwrap_or_default();
    if mentions(reading, super::BODY_TRIAGE_HOT) {
        Cow::Owned(format!(
            "Contact NHS 111 or your GP urgently about a hot, swollen {part}, especially if it became painful suddenly or you feel unwell or feverish. A clinician can check whether you need treatment."
        ))
    } else {
        Cow::Owned(format!(
            "Ask your GP about a swollen {part} that is unexplained, getting worse or affecting normal movement. Get urgent advice from NHS 111 if the swelling is severe, comes on suddenly, or the area becomes hot and painful."
        ))
    }
}

pub(super) fn reply(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let text = said(reading);
    let reply = match id {
        "register-dentist" => dentist(reading),
        "body-see-today" => return Some(see_today(reading)),
        "body-urgent" => return red_flag(reading),
        "pregnant-swollen-ankles-in-weeks" | "pregnant-swollen-ankles" => {
            "Gradual swelling in the feet and ankles can happen in pregnancy. Rest with your feet raised and use comfortable shoes. Contact your midwife or maternity unit immediately if swelling suddenly increases, particularly in your face, hands or feet, or if you have a severe headache, vision changes or pain under your ribs."
        }
        "back-pain-pregnant" => {
            "Tell your midwife or GP if back pain is severe or persistent. In the second or third trimester, or if there is fever, bleeding, pain when you pee or pain under your ribs, contact your midwife or GP urgently. Loss of feeling around the genitals or bottom, or in the legs, needs emergency assessment."
        }
        "body-sea-water" => {
            let pet = |line: &str| whole(line, &["dog", "puppy", "cat", "kitten", "pet"]);
            if pet(text)
                || (whole(text, &["he", "she", "it"])
                    && earlier
                        .last()
                        .is_some_and(|turn| pet(&turn.user.to_lowercase())))
            {
                PET_SEA_WATER
            } else if SEA_WATER_ALARM.is_match(text) {
                "Contact NHS 111 now if someone swallowed a lot of seawater or has vomiting or dizziness. Call 999 for breathing trouble, severe confusion, a seizure or difficulty waking them. Avoid making them vomit."
            } else {
                "Rinse your mouth with fresh water after swallowing seawater. Contact NHS 111 if you swallowed a lot or feel unwell. Call 999 for breathing trouble, severe confusion, seizures or difficulty waking. Boiling seawater in a pan leaves the salt behind."
            }
        }
        "see-about-a-part" | "see-about-it" => {
            if mentions(reading, super::BODY_TRIAGE_SORENESS)
                && has(text, super::BODY_TRIAGE_EXERCISE_AFTER)
            {
                return None;
            }
            if let Some(part) = super::body::part_named_with_background(reading) {
                return Some(Cow::Owned(format!(
                    "Get the {part} checked sooner if the pain is severe or getting worse, keeps you awake or stops normal movement. Contact NHS 111 after a significant injury or if you have fever, new numbness or weakness. You can ask your GP or local musculoskeletal service about physiotherapy for persistent symptoms."
                )));
            }
            "If symptoms are severe, getting worse or affecting daily life, contact your GP. Use NHS 111 if you need urgent advice or are unsure where to go. Call 999 for a life-threatening emergency."
        }
        "hurt-limb-someone" => {
            "Get urgent advice from NHS 111 if the pain or swelling is severe, the person cannot use the limb normally, or you think a bone may be broken. Go to A&E if it looks out of shape, bone is visible, or the limb is numb, blue or cold. Keep the limb still while arranging help."
        }
        "broken-or-sprained" if super::body::person_part(reading) => {
            let lead = if mentions(reading, super::BODY_TRIAGE_HANDS) {
                "Difficulty gripping or moving a finger or wrist after an injury can need assessment."
            } else if mentions(reading, super::BODY_TRIAGE_SHOULDER) {
                "Difficulty raising or using the arm after a shoulder injury can need assessment."
            } else {
                "Severe pain or being unable to put weight on the injured leg or foot can need assessment."
            };
            return Some(Cow::Owned(format!(
                "{lead} Being able to move it does not rule out a fracture. Contact NHS 111 if you suspect a break. Go to A&E if the limb is out of shape, numb, blue or cold, or bone is visible."
            )));
        }
        "changing-mole" => {
            let changed = mentions(reading, super::BODY_TRIAGE_MOLE_HAPPENING);
            if !changed && has(text, super::BODY_TRIAGE_MOLE_REMOVE) {
                "Ask your GP about a mole that bothers you or is changing. If removal is appropriate, a clinician can explain how it is done. Leave removal to a qualified clinician."
            } else if MOLE_CHECK_METHOD.is_match(text)
                || (mentions(reading, &["hair", "hairy"])
                    && !mentions(reading, super::BODY_TRIAGE_MOLE_CLEAR))
            {
                MOLE_CHECK
            } else if changed
                || mentions(reading, &["check", "checked", "checking"])
                || (reading.line.words.len() <= 5
                    && earlier.last().is_some_and(|turn| {
                        turn.bot.starts_with("Arrange a GP appointment for a mole")
                    }))
            {
                MOLE_CHANGED
            } else {
                MOLE_CHECK
            }
        }
        "hair-removal" if mentions(reading, super::BODY_TRIAGE_WAX) => super::BODY_TRIAGE_WAX_REPLY,
        "hair-removal" => super::BODY_TRIAGE_SHAVE_REPLY,
        "muscle-healing" => {
            let previous = earlier
                .last()
                .map_or_else(String::new, |turn| turn.user.to_lowercase());
            let named = |terms| mentions(reading, terms) || whole(&previous, terms);
            let lead = if named(&["groin", "adductor"]) {
                "For a groin strain, return to kicking and changes of direction gradually when it feels strong and moves comfortably."
            } else if named(&["hamstring", "calf", "calves", "tear", "torn"]) {
                "A hamstring or calf strain can take weeks or longer, depending on severity. Build back up gradually after pain and swelling settle."
            } else {
                "For a thigh strain, ease back into movement as pain allows and build up gradually before returning to sport."
            };
            return Some(Cow::Owned(format!(
                "{lead} Get urgent advice if there is severe pain, marked bruising, or you cannot use the limb or bear weight. Seek an assessment if it is not improving."
            )));
        }
        "piercing-healing" => {
            let lead = if CARTILAGE.is_match(text) {
                "Cartilage piercings often take longer to heal than earlobe piercings."
            } else {
                "Healing time depends on where the piercing is and how it is healing."
            };
            return Some(Cow::Owned(format!("{lead} {PIERCING_CARE}")));
        }
        "piercing-infection" => {
            "Heat, increasing pain or swelling, pus, or feeling feverish can mean a piercing is infected. Ask for an urgent GP appointment or contact NHS 111 now. Leave the jewellery in unless a doctor tells you to take it out."
        }
        "tampon-how-long" => {
            "Follow the pack's changing instructions and never leave a tampon in for more than eight hours. Use the lowest absorbency you need and wash your hands before and after. If you develop fever, vomiting, a rash or dizziness while using one or soon afterwards, remove it and contact NHS 111 urgently. Call 999 for severe symptoms such as confusion or trouble breathing. After birth, wait until your six-week postnatal check before using tampons."
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}
