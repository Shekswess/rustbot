//! Booking, preparation and wait-time advice for appointments.
use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static DECISION_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:should|shall|do|would|could)\s+i\b[^.?!]*\b(?:see|go|get|be seen|have it|",
        r"have this|bother|worry)\b|\bis it worth\b|\bworth (?:seeing|going|getting|a trip)\b|",
        r"\bdo i need to\b|\bneed i\b",
    ))
    .unwrap()
});

const BOOKING_WORDS: &[&str] = &[
    "book",
    "books",
    "booking",
    "booked",
    "appointment",
    "appointments",
    "register",
    "registering",
    "registration",
    "online",
    "app",
    "phone",
    "ring",
    "call",
    "surgery",
    "practice",
    "clinic",
    "cancellation",
    "cancellations",
    "slot",
    "slots",
    "reception",
    "receptionist",
    "walk in",
    "111",
];
const RETURN_TO: &[&str] = &[
    "back to",
    "back at",
    "going back",
    "go back",
    "went back",
    "come back",
    "coming back",
    "got back",
];
const DENTAL_PLACES: &[&str] = &["dentist", "dentists", "dental", "orthodontist"];
const DOCTOR_PLACES: &[&str] = &[
    "doctor", "doctors", "gp", "gps", "surgery", "practice", "clinic", "nurse",
];
const TOOTH_WORDS: &[&str] = &[
    "tooth",
    "teeth",
    "toothache",
    "filling",
    "fillings",
    "crown",
    "crowns",
    "extraction",
    "extracted",
    "gum",
    "gums",
    "denture",
    "dentures",
    "wisdom",
    "brace",
    "braces",
    "root canal",
    "enamel",
    "abscess",
    "chipped",
    "chipped tooth",
];
const SEEN_ABOUT_IT: &[&str] = &[
    "book",
    "booking",
    "booked",
    "appointment",
    "appointments",
    "ring",
    "ringing",
    "call",
    "phone",
    "see",
    "seen",
    "register",
    "registering",
    "get in",
    "joined",
    "joining",
    "new",
    "urgent",
    "emergency",
    "pain",
    "hurts",
    "hurting",
];
const WAIT_WORDS: &[&str] = &[
    "long", "soon", "quickly", "wait", "waits", "waiting", "weeks", "days",
];

const GP: &str = "Ring at the moment the lines open - 8am for most GP surgeries - and ask for the first appointment of the day: same-day slots go in the first half hour. Say in one sentence what it is for, because \"I need it looked at today\" books faster than \"I was wondering\", and ask to go on the cancellation list if the wait is weeks. Dentists are the same: say it hurts and ask what they have sooner, and 111 can find an urgent NHS dentist if yours cannot see you within a few days. If there is nothing at all, ask which day the next slots are released and whether the duty doctor can ring you back, and remember a pharmacist can treat a lot of everyday things the same day.";
const DENTAL: &str = "Ring the dental practice as soon as it opens and ask for an urgent slot: most keep a few back each morning, and saying it hurts, or that a tooth has broken or a filling has come out, is what gets you one of them. Ask what they have sooner if the wait is weeks, and ring 111 for the out-of-hours dental service if your practice is shut or cannot see you within a few days - they can also name an urgent NHS dentist. Until then paracetamol or ibuprofen at the dose on the pack, and a pharmacist can help you hold on.";
const WAIT_GP: &str = "Ask what the wait is when you ring: a routine GP appointment is usually a few days to a couple of weeks, and anything urgent is seen the same day because surgeries keep slots back each morning for it. Ring at the moment the lines open for one of those, and 111 can find an urgent dentist or a walk-in centre if your surgery has nothing.";
const WAIT_DENTAL: &str = "Ask what the wait is when you ring: a routine dental check-up is usually a few days to a couple of weeks, depending on the practice, and a tooth that hurts is seen the same day or the next because most practices keep a few slots back each morning. Ring at opening time for one of those, and 111 can name an urgent NHS dentist if yours has nothing for days.";

pub(super) fn can_book(reading: &Reading<'_>) -> bool {
    let clean = reading.line.clean.to_lowercase();
    !DECISION_ASK.is_match(&clean)
        || RETURN_TO.iter().any(|place| clean.contains(place))
        || WORD
            .find_iter(&clean)
            .any(|word| BOOKING_WORDS.contains(&word.as_str()))
}

fn has(words: &[String], choices: &[&str]) -> bool {
    words.iter().any(|word| choices.contains(&word.as_str()))
}

fn dental_in_prior(earlier: &[Exchange]) -> bool {
    let Some(previous) = earlier.last() else {
        return false;
    };
    let line = previous.user.to_lowercase();
    let words: Vec<_> = WORD
        .find_iter(&line)
        .map(|found| found.as_str())
        .filter(|word| word.len() > 2)
        .collect();
    let says = |choices: &[&str]| words.iter().any(|word| choices.contains(word));
    says(TOOTH_WORDS) || (says(DENTAL_PLACES) && says(SEEN_ABOUT_IT))
}

pub(super) fn answer(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let words = &reading.line.words;
    let dental =
        has(words, DENTAL_PLACES) || (!has(words, DOCTOR_PLACES) && dental_in_prior(earlier));
    let answer = match (dental, has(words, WAIT_WORDS)) {
        (false, false) => GP,
        (true, false) => DENTAL,
        (false, true) => WAIT_GP,
        (true, true) => WAIT_DENTAL,
    };
    Some(Cow::Borrowed(answer))
}

static FAST_LENGTH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::APPOINTMENT_FAST_LENGTH).expect("fasting question"));
static TEST_DURATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::APPOINTMENT_TEST_DURATION).expect("duration of the test"));

const BLOOD_PREPARATION: &str = "Check the instructions from the clinician who requested the test. Only fast if they told you to, and follow their advice on the timing and whether you can drink plain water. If you take regular medicines, ask before changing anything. Wear a loose sleeve if you can.";

fn contains(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn nonmedical(text: &str) -> bool {
    contains(text, super::APPOINTMENT_NONMEDICAL)
        && !contains(text, super::APPOINTMENT_MEDICAL_CONTEXT)
}

pub(super) fn follows_appointment(earlier: &[Exchange]) -> bool {
    earlier.last().is_some_and(|turn| {
        let text = turn.user.to_lowercase();
        contains(&text, super::APPOINTMENT_KINDS) && !nonmedical(&text)
    })
}

fn asks_fasting_length(text: &str) -> bool {
    FAST_LENGTH.is_match(text) && !TEST_DURATION.is_match(text)
}

pub(super) fn blood_test(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = if asks_fasting_length(&reading.line.clean) {
        BLOOD_PREPARATION
    } else {
        "A clinician normally takes a small blood sample through a needle in your arm. The appointment often takes a few minutes. Which tests are run depends on why it was booked. Ask which results to expect and when. Follow any preparation instructions you were given."
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn preparation(
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let current = reading.line.text.to_lowercase();
    if nonmedical(&current) {
        return None;
    }
    let own_appointment = contains(&current, super::APPOINTMENT_KINDS)
        || contains(&current, super::APPOINTMENT_OPERATION);
    let subject = if own_appointment {
        current
    } else if let Some(previous) = earlier.last() {
        let previous = previous.user.to_lowercase();
        if nonmedical(&previous) {
            return None;
        }
        format!("{current} {previous}")
    } else {
        current
    };
    let text = &subject;
    let operation = contains(text, super::APPOINTMENT_OPERATION);
    if operation
        && contains(text, super::APPOINTMENT_MEDICINES)
        && !contains(text, super::APPOINTMENT_MEDICINE_LIST)
    {
        return None;
    }
    let reply = if operation
        && !contains(text, super::APPOINTMENT_NOT_FASTING)
        && !contains(
            text,
            &[
                "gp surgery",
                "the surgery",
                "at the surgery",
                "surgery hours",
                "surgery number",
            ],
        ) {
        "Follow your surgical team's instructions for when to stop eating and drinking and which medicines to take. Call the pre-op team if anything is unclear, especially if you take insulin or anticoagulants. Ask whether you need someone to collect you and stay with you afterwards."
    } else if contains(text, super::APPOINTMENT_BLOOD_TEST) {
        if reading.wants == Some("time") && asks_fasting_length(text) {
            return None;
        }
        BLOOD_PREPARATION
    } else if contains(text, super::APPOINTMENT_SCAN) {
        "Check the appointment letter. Preparation differs for X-rays, ultrasound, MRI and procedures such as colonoscopy. Before an MRI, tell the department about a pacemaker, an implant, metal in or near your eyes, or possible pregnancy. Call ahead if you are unsure about eating, drinking or medication."
    } else if contains(text, super::APPOINTMENT_DENTAL) {
        "For a routine dental check-up, brush as usual and bring a list of your medicines. Tell the practice about any medical conditions and if you are nervous. Follow the practice's instructions if the visit involves treatment or sedation."
    } else if contains(text, super::APPOINTMENT_OPTICIAN) {
        "Bring your current glasses or prescription, any eye drops you use and a note of changes in your vision. Ask beforehand if drops will be used to dilate your pupils. If they are, plan a way home that does not involve you driving until your eyesight returns to normal."
    } else {
        "Check the appointment letter or text for preparation instructions. Bring a list of your medicines and a note of the main concerns you want to discuss. Contact the clinic in advance if anything is unclear or you need to change the booking."
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn booking_and_wait_time_callbacks_match_pinned_pybot() {
        let cases = [
            ("How do I book an appointment?", "", true, GP),
            ("How do I book a dentist appointment?", "", true, DENTAL),
            (
                "How long will I wait for a GP appointment?",
                "",
                true,
                WAIT_GP,
            ),
            (
                "How long will I wait for a dentist appointment?",
                "",
                true,
                WAIT_DENTAL,
            ),
            (
                "How do I book an appointment?",
                "My tooth hurts",
                true,
                DENTAL,
            ),
            (
                "How do I book an appointment?",
                "I went to the dentist last week",
                true,
                GP,
            ),
            (
                "How do I book an appointment?",
                "I need to book the dentist",
                true,
                DENTAL,
            ),
            (
                "How do I book an appointment with my GP?",
                "My tooth hurts",
                true,
                GP,
            ),
            ("Should I see a doctor?", "", false, GP),
            ("Should I get an appointment?", "", true, GP),
            ("Back to my knee, should I see a doctor?", "", true, GP),
            (
                "What should I get appointment",
                "The appointment is on Thursday.",
                true,
                GP,
            ),
        ];
        for (ask, prior, guard, expected) in cases {
            let memory = (!prior.is_empty()).then(|| Exchange {
                user: prior.into(),
                bot: String::new(),
                module: "greeting",
            });
            let reading = Reading::new(ask);
            assert_eq!(can_book(&reading), guard, "{ask}");
            assert_eq!(
                answer(&reading, memory.as_slice()).as_deref(),
                Some(expected),
                "{ask} after {prior}"
            );
        }
    }

    #[test]
    fn a_short_booking_followup_uses_the_dated_appointment() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        let dated = bot.turn("The appointment is on Thursday.");
        assert_eq!(dated.module, Some("greeting"));
        assert_eq!(dated.reply, "How ready do you feel for it?");

        let booking = bot.turn("What should I get?");
        assert_eq!(booking.module, Some("answers"));
        assert_eq!(booking.reply, GP);
    }
}
