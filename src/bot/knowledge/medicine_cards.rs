//! Medicine-specific routing restored from the pinned health cards.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::{Exchange, Reading};

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());

fn hits(text: &str, choices: &[&str]) -> bool {
    choices.iter().any(|choice| text.contains(choice))
}

fn token_in(text: &str, choices: &[&str]) -> bool {
    WORD.find_iter(text).any(|m| choices.contains(&m.as_str()))
}

fn clean<'a>(reading: &'a Reading<'_>) -> &'a str {
    let line = &reading.line;
    if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    }
}

fn named_and_unhelpful(text: &str) -> (bool, bool) {
    let words: Vec<_> = WORD.find_iter(text).map(|m| m.as_str()).collect();
    let names: Vec<_> = words
        .iter()
        .enumerate()
        .filter_map(|(index, word)| super::MEDICINE_NAMED.contains(word).then_some(index))
        .collect();
    if names.is_empty() {
        return (false, false);
    }
    let nearby = super::MEDICINE_NOT_HELPED.iter().any(|phrase| {
        let size = phrase.split_whitespace().count();
        words.windows(size).enumerate().any(|(start, words)| {
            words.iter().copied().eq(phrase.split_whitespace())
                && names.iter().any(|name| name.abs_diff(start) <= 5)
        })
    });
    (true, nearby)
}

pub(super) fn named_medicine_guard(reading: &Reading<'_>) -> bool {
    let (named, unhelpful) = named_and_unhelpful(clean(reading));
    !named || unhelpful
}

pub(super) fn tablet_not_helping(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = clean(reading);
    if token_in(text, super::MEDICINE_TABLET_DEVICE) {
        return None;
    }
    let (named, unhelpful) = named_and_unhelpful(text);
    let medicinal = if named {
        unhelpful
    } else {
        token_in(text, super::MEDICINE_TABLET_TERMS)
            || token_in(text, &["tablets", "pills", "painkillers", "capsules"])
            || hits(
                text,
                &[
                    "on tablets",
                    "my tablets",
                    "the tablets",
                    "a day",
                    "per day",
                    "daily",
                ],
            )
    };
    medicinal.then_some(Cow::Borrowed(super::MEDICINE_TABLET_TEXT))
}

static AGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::MEDICINE_CHILD_AGE).expect("valid melatonin age pattern"));

pub(super) fn melatonin(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = clean(reading);
    let reply = if token_in(text, super::MEDICINE_MELATONIN_PREGNANT)
        || text.contains("breast feeding")
    {
        "Melatonin is not usually used in pregnancy or while breastfeeding. Tell your doctor or midwife if you take it or are planning to take it so they can advise you on sleep and check which options are suitable."
    } else if AGE
        .captures(text)
        .and_then(|age| age[1].parse::<u8>().ok())
        .is_some_and(|age| age < 18)
        || text.contains("months old")
        || (!token_in(text, super::MEDICINE_MELATONIN_ADULT)
            && (token_in(text, super::MEDICINE_MELATONIN_CHILD)
                || hits(text, super::MEDICINE_MELATONIN_GIVE)))
    {
        "Melatonin is prescription-only in the UK and is sometimes prescribed for children. Ask a GP or health visitor about the sleep problem and bedtime routines. Give it only if it was prescribed for that child, following their own instructions."
    } else {
        "Melatonin is a prescription medicine in the UK. The dose, timing and duration depend on the treatment and formulation. Follow your prescription and check the leaflet. Tell the prescriber or pharmacist about your other medicines, including antidepressants, epilepsy medicines and blood thinners."
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn drink_and_the_drug(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words = &reading.line.words;
    let contains = |names: &[&str]| words.iter().any(|word| names.contains(&word.as_str()));
    let reply = if contains(&["metronidazole", "tinidazole"]) {
        "Avoid alcohol with metronidazole tablets or liquid and with suppositories, and continue for 48 hours after stopping. Avoid it during treatment with tinidazole and for 72 hours afterwards. Check your leaflet or ask your pharmacist about the exact formulation."
    } else if contains(&[
        "antibiotic",
        "antibiotics",
        "penicillin",
        "amoxicillin",
        "trimethoprim",
        "nitrofurantoin",
    ]) {
        "Alcohol advice depends on the antibiotic. It can worsen nausea or dizziness and interacts with some antibiotics. Check the leaflet or ask a pharmacist about the exact medicine before drinking."
    } else if contains(&[
        "codeine",
        "co codamol",
        "co-codamol",
        "cocodamol",
        "codamol",
    ]) {
        "Avoid alcohol with codeine or co-codamol. It can increase sleepiness and the risk of serious side effects. Check the leaflet and ask a pharmacist about other medicines that may also make you sleepy."
    } else if contains(&[
        "blood pressure",
        "bp",
        "statin",
        "statins",
        "ramipril",
        "amlodipine",
        "lisinopril",
        "atorvastatin",
        "simvastatin",
        "rosuvastatin",
        "water tablet",
        "water tablets",
        "diuretic",
    ]) {
        "Some blood pressure medicines can make you feel dizzy when you drink. Alcohol advice also varies between medicines for cholesterol and with your health. Check your exact medicine with a pharmacist or in the leaflet before drinking."
    } else if contains(&[
        "antihistamine",
        "antihistamines",
        "hayfever",
        "hay fever",
        "piriton",
        "cetirizine",
        "loratadine",
        "chlorphenamine",
        "fexofenadine",
        "allergy",
    ]) {
        "Alcohol can make you sleepier on antihistamines, especially sedating types. Check the leaflet or ask your pharmacist about the exact antihistamine and avoid driving if you feel sleepy."
    } else if contains(&[
        "antidepressant",
        "antidepressants",
        "sertraline",
        "fluoxetine",
        "citalopram",
        "escitalopram",
        "paroxetine",
        "venlafaxine",
        "mirtazapine",
        "ssri",
    ]) {
        "Alcohol can make antidepressant side effects such as drowsiness worse, and the advice differs between types. Check the leaflet or speak to your pharmacist or prescriber before drinking."
    } else if contains(&["paracetamol", "acetaminophen", "calpol", "panadol"]) {
        "Check the paracetamol leaflet and ask a pharmacist if you have liver problems, drink heavily or take other medicines. Check whether any cold or pain remedy you take also contains paracetamol."
    } else if contains(&["ibuprofen", "nurofen", "brufen"]) {
        "Drinking a lot of alcohol with ibuprofen can increase the risk of side effects, including stomach problems. Check the leaflet or ask a pharmacist about your circumstances."
    } else {
        "Check the exact medicine in its patient leaflet or ask a pharmacist before drinking alcohol. Alcohol can increase side effects or interact with some medicines."
    };
    Some(Cow::Borrowed(reply))
}

static STATIN_INTERACTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(super::MEDICINE_STATIN_INTERACTION).expect("valid statin interaction pattern")
});
static STATIN_WHETHER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(super::MEDICINE_STATIN_WHETHER).expect("valid statin decision pattern")
});

pub(super) fn last_line_named_a_statin(earlier: &[Exchange]) -> bool {
    earlier
        .last()
        .is_some_and(|turn| hits(&turn.user.to_lowercase(), super::MEDICINE_STATIN_NAMES))
}

pub(super) fn a_statin_and_the_day(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = if reading.line.words.is_empty() {
        Cow::Borrowed(clean(reading))
    } else {
        Cow::Owned(reading.line.words.join(" "))
    };
    let medicines = super::MEDICINE_STATIN_MEDICINES
        .iter()
        .filter(|word| text.contains(**word))
        .count();
    let reply = if STATIN_INTERACTION.is_match(&text)
        || medicines >= 2
        || (medicines == 1 && hits(&text, super::MEDICINE_STATIN_MORE_THAN_ONE))
    {
        "Give your pharmacist or prescriber the name of your statin and a list of everything else you take, including supplements. Interactions vary by statin. Check before starting a new medicine or stopping a prescribed one."
    } else if hits(
        &text,
        &[
            "night",
            "evening",
            "morning",
            "what time",
            "when",
            "same time",
            "best time",
            "time of day",
            "together",
        ],
    ) {
        "The best time depends on which statin you take. Simvastatin is usually taken in the evening. You can usually take atorvastatin at any time, at the same time each day. Follow your prescription or check with your pharmacist if you are unsure."
    } else if STATIN_WHETHER.is_match(&text) && !hits(&text, super::MEDICINE_STATIN_STARTED) {
        return None;
    } else {
        "Follow the dose and timing your prescriber gave you and ask when they want to check your cholesterol and liver function. Contact the prescriber if you get unexplained muscle pain, tenderness or weakness. Ask before changing or stopping the statin."
    };
    Some(Cow::Borrowed(reply))
}

static EMERGENCY_ASKS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        super::MEDICINE_EC_AGE,
        super::MEDICINE_EC_SICK,
        super::MEDICINE_EC_OVULATION,
        super::MEDICINE_EC_HURT,
        super::MEDICINE_EC_FITTED,
        super::MEDICINE_EC_LEVO,
        super::MEDICINE_EC_ULI,
        super::MEDICINE_EC_COIL,
        super::MEDICINE_EC_EFFECTIVE,
        super::MEDICINE_EC_FREE,
        super::MEDICINE_EC_WHERE,
        super::MEDICINE_EC_LATE,
        r"\b(?:coil|iud)\b",
    ])
    .expect("valid emergency contraception patterns")
});

const CLOCK: &str = "Get emergency contraception as soon as you can. Levonorgestrel is taken within 72 hours and ulipristal acetate within 120 hours of unprotected sex.";
const FREE: &str = "Ask a pharmacy, sexual health clinic or GP surgery as soon as possible. NHS emergency contraception is free from services that provide it.";
const COIL: &str = "A copper IUD is the most effective emergency contraception. Contact a sexual health clinic or GP urgently to see if it can be fitted within the time window.";
const TEST: &str = "Ask the pharmacist or clinic when to take a pregnancy test and what contraception to use afterwards.";
const FITTING: &str = "A trained clinician can fit the copper IUD through the cervix at a sexual health clinic or some GP surgeries. Make an appointment urgently.";
const PAIN: &str = "Copper IUD fitting can be uncomfortable or painful. Ask about pain relief beforehand and tell the clinician if you need them to pause.";

pub(super) fn morning_after(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = clean(reading).to_lowercase();
    let matches = EMERGENCY_ASKS.matches(&text);
    let has = |index| matches.matched(index);
    let lead = if has(0) {
        "NHS contraception services are free and confidential, including for people under 16. Staff may share information if they believe someone is at risk of harm."
    } else if has(1) {
        "Emergency pills can cause nausea. If you are sick within three hours, contact a pharmacist or sexual health clinic promptly. You may need another dose or a copper IUD."
    } else if has(2) {
        return Some(Cow::Owned([
            "The pills work by delaying ovulation. Tell the clinician if you may have ovulated already and ask urgently about a copper IUD.",
            COIL, CLOCK,
        ].join(" ")));
    } else if has(3) && has(12) {
        return Some(Cow::Owned([PAIN, FITTING, CLOCK].join(" ")));
    } else if has(4) && has(12) {
        return Some(Cow::Owned([FITTING, PAIN, CLOCK].join(" ")));
    } else if (has(5) && has(6)) || (!has(7) && has(8)) {
        return Some(Cow::Owned([COIL, CLOCK, FREE, TEST].join(" ")));
    } else if has(7) {
        COIL
    } else if has(9) || has(10) {
        FREE
    } else if has(11) {
        CLOCK
    } else {
        return Some(Cow::Owned([CLOCK, FREE, COIL, TEST].join(" ")));
    };
    Some(Cow::Owned(
        std::iter::once(lead)
            .chain(
                [CLOCK, FREE, COIL]
                    .into_iter()
                    .filter(|sentence| *sentence != lead),
            )
            .take(3)
            .collect::<Vec<_>>()
            .join(" "),
    ))
}

pub(super) const BABY_URGENT: &str = "Contact NHS 111 or your GP urgently if a baby under three months has a temperature of 38°C or higher or you think they have a fever. Call 999 if they are hard to wake, struggle to breathe, or have a rash that does not fade when pressed.";

pub(super) fn medicine_for_a_young_baby(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let first = if hits(clean(reading), super::MEDICINE_BABY_TWO_MONTHS)
        || hits(&reading.line.norm_text, super::MEDICINE_BABY_TWO_MONTHS)
    {
        "Speak to a pharmacist or clinician before giving paracetamol to a two-month-old. Check the product, age, weight and whether they were born at term. Give ibuprofen before three months only if prescribed."
    } else {
        "For a baby under two months, get medical advice before giving paracetamol. Give ibuprofen before three months only if prescribed. Tell NHS 111 or your GP the baby's age, weight and temperature."
    };
    Some(Cow::Owned([first, BABY_URGENT].join(" ")))
}

static MISSED_PATTERNS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        super::MISSED_CONFUSED,
        super::MISSED_TABLET_MUDDLE,
        super::MISSED_CONFUSED_NEW,
        super::MISSED_THIRD_PERSON,
        super::MISSED_RAN_OUT,
        super::MISSED_CRITICAL,
        super::MISSED_REPEAT,
        super::MISSED_MISSED,
        super::MISSED_SUGAR,
        super::MISSED_ANTIBIOTIC,
        super::MISSED_TOO_MUCH,
        super::MISSED_DOUBLE,
    ])
    .expect("valid missed medicine patterns")
});

static MISSED_THIRD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!("\\A(?:{})", super::MISSED_THIRD))
        .expect("repeated third-person missed dose")
});

const MISSED_DOSE: &str = "Check the leaflet's missed-dose instructions for this exact medicine or call a pharmacist. Only take an extra dose if the instructions or a clinician tell you to.";
const ROUTINE: &str = "Set an alarm and make a list of which doses were taken. Ask a pharmacist whether a pill organiser is suitable and request a medicines review if doses keep getting missed or taken twice.";
const MEMORY: &str = "If they are also forgetting other things, note the changes and arrange a GP appointment. Call 999 for sudden confusion.";

pub(super) fn missed_medication(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = clean(reading);
    let matches = MISSED_PATTERNS.matches(text);
    let has = |index| matches.matched(index);
    let reply = if has(0) && has(1) && !has(2) {
        if has(3) {
            return Some(Cow::Owned([ROUTINE, MEMORY].join(" ")));
        }
        ROUTINE
    } else if has(0) {
        "Call 999 straight away if the confusion came on suddenly. For slower changes, contact the GP or NHS 111 promptly and tell them what medicines were taken and when."
    } else if has(4) {
        "Contact your usual pharmacy or GP as soon as possible for an emergency supply. NHS 111 can advise if they are unavailable and you need a repeat medicine urgently. Ask before stopping your medicine."
    } else if has(5) {
        "Contact your pharmacist or prescriber promptly for instructions on this medicine. Tell them the name, dose and when it was due. Follow the medicine-specific leaflet and take an extra dose only if instructed."
    } else if has(6) && has(7) {
        ROUTINE
    } else if has(9) {
        "Check the antibiotic's leaflet or ask a pharmacist what to do about the missed dose. Follow the prescribed course and take an extra dose only if instructed."
    } else if has(7) && has(8) && !has(6) {
        "Check your blood glucose and follow your own diabetes team's missed-insulin instructions. Contact your diabetes team or NHS 111 promptly if unsure, and tell them which insulin and dose you take."
    } else if has(10) && !has(7) {
        "If you may have taken too much or the wrong medicine, get advice now and have the packet and times ready. Contact NHS 111 if you are unsure about the risk. Call 999 for severe symptoms or if the person is unconscious."
    } else if has(7) && has(11) {
        return Some(Cow::Owned([MISSED_DOSE, ROUTINE].join(" ")));
    } else if has(7) && !has(6) {
        MISSED_DOSE
    } else if MISSED_THIRD.is_match(text) {
        return Some(Cow::Owned([ROUTINE, MEMORY].join(" ")));
    } else if has(6) {
        ROUTINE
    } else {
        return None;
    };
    Some(Cow::Borrowed(reply))
}

fn prior_text(earlier: &[Exchange]) -> String {
    earlier
        .last()
        .map_or_else(String::new, |turn| turn.user.to_lowercase())
}

pub(super) fn a_price_was_asked(reading: &Reading<'_>) -> bool {
    let text = clean(reading);
    hits(text, super::PHARMACY_MEDICINE_THINGS)
        && !hits(text, super::PHARMACY_PRESCRIBED)
        && !hits(text, super::PHARMACY_NOT_A_SHELF_PRICE)
        && !hits(text, super::PHARMACY_PETS)
}

const PHARMACY_KINDS: &[(&[&str], &str)] = &[
    (
        super::PHARMACY_HEADACHE,
        "Ask a pharmacist to help you choose a suitable painkiller for the headache and check your other medicines. Call 999 for a sudden, extremely painful headache or a headache with new weakness, confusion or difficulty speaking.",
    ),
    (
        super::PHARMACY_BACK,
        "A pharmacist can check which pain relief is suitable for your back pain. Go to A&E straight away if you lose feeling around your genitals or bottom, have new trouble peeing or controlling your bladder or bowels, or develop weakness or numbness in both legs.",
    ),
    (
        super::PHARMACY_PERIOD,
        "Ask a pharmacist which pain relief is suitable for your period pain. You can also try a warm pad wrapped in a towel. Seek urgent advice if the pain is severe or worse than usual and pain relief has not helped.",
    ),
    (
        super::PHARMACY_THROAT,
        "Ask the pharmacist about suitable pain relief and whether they can assess your sore throat. Call 999 if you cannot swallow, have difficulty breathing, or are drooling because you cannot swallow.",
    ),
    (
        super::PHARMACY_COUGH,
        "A pharmacist can advise on cough remedies and check which ones are suitable for your age, symptoms and other medicines. Get urgent help if the cough is severe or quickly gets worse, you feel very unwell, have chest pain, have breathing trouble or cough up blood.",
    ),
    (
        super::PHARMACY_COLD,
        "Ask a pharmacist about treatments for the symptoms that are bothering you, and check cold remedies for ingredients that overlap with medicines you already take.",
    ),
    (
        super::PHARMACY_HAY_FEVER,
        "Ask a pharmacist about antihistamines, nasal sprays or eye drops for hay fever. Some antihistamines cause drowsiness, so check which option fits your medicines and what you need to do.",
    ),
    (
        super::PHARMACY_HEARTBURN,
        "A pharmacist can advise on antacids or alginates for heartburn and check whether they interact with your medicines. See a GP if heartburn happens most days, food sticks when you swallow, or you are losing weight without trying.",
    ),
    (
        super::PHARMACY_DIARRHOEA,
        "Ask the pharmacist about oral rehydration salts and whether anything else is suitable. Get urgent advice from NHS 111 for bloody diarrhoea or if you keep vomiting and cannot keep fluids down.",
    ),
    (
        super::PHARMACY_CONSTIPATION,
        "Ask a pharmacist which laxative is suitable and how long to use it. Tell them about any medicines you take and contact a GP if constipation persists or keeps coming back.",
    ),
    (
        super::PHARMACY_THRUSH,
        "Ask a pharmacist about thrush treatment if you have had it diagnosed before and recognise the symptoms. Speak to a GP or sexual health clinic if this is the first time, you are pregnant or breastfeeding, or the symptoms keep returning.",
    ),
    (
        super::PHARMACY_VERTIGO,
        "Ask a pharmacist or clinician about vertigo so they can check the cause and advise on treatment. Call 999 if it comes with new weakness, difficulty speaking, double vision or loss of vision.",
    ),
    (
        super::PHARMACY_SLEEP,
        "A pharmacist can check whether a short-term sleep aid is suitable and whether it would interact with your medicines. See a GP if sleep problems last for months or affect daily life.",
    ),
    (
        super::PHARMACY_CYSTITIS,
        "A pharmacist may be able to assess a suspected urinary infection or direct you to the right service. Seek urgent help if you are pregnant, have fever or chills, pain in your back below your ribs, or blood in your urine.",
    ),
    (
        super::PHARMACY_EARACHE,
        "Ask a pharmacist what you can use for earache and whether you need an assessment. Seek urgent help if you feel very unwell, have swelling around the ear, or have fluid coming from it.",
    ),
];

pub(super) fn pharmacy(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let current = clean(reading);
    let previous = prior_text(earlier);
    let either = |words| hits(current, words) || hits(&previous, words);
    let reply = match id {
        "the-own-brand" => {
            "Ask a pharmacist to compare the active ingredient, strength and formulation and check that switching brands would be suitable. Some medicines have brand-specific prescribing advice."
        }
        "a-pet-at-the-counter" if either(super::PHARMACY_PETS) => {
            "Check with your vet before giving your pet any human medicine. If your pet may already have swallowed some, call the vet straight away and have the packet and likely amount ready."
        }
        "a-prescribed-course"
            if either(super::PHARMACY_PRESCRIBED)
                && !either(super::PHARMACY_NOT_A_PRESCRIPTION) =>
        {
            "Ask your pharmacy whether repeat dispensing is set up. Otherwise request a repeat through your GP or the NHS App and allow time to collect it. Contact the prescriber if the course did not work or the symptoms returned."
        }
        "at-the-counter" if !current.is_empty() && !either(super::PHARMACY_NOT_A_SYMPTOM) => {
            if either(super::PHARMACY_PREGNANT) {
                "Tell the pharmacist if you are pregnant or breastfeeding before using any medicine, including a cold remedy or herbal product. During pregnancy, contact your maternity team immediately for a severe headache, vision changes or pain under the ribs."
            } else if either(super::PHARMACY_CHILD) {
                return Some(Cow::Owned([
                    "Give the pharmacist the child's age, weight and symptoms so they can check whether a medicine is suitable. Follow the age-specific label and use the measuring device that came with it.",
                    BABY_URGENT,
                ].join(" ")));
            } else {
                let for_symptom = |text| {
                    PHARMACY_KINDS
                        .iter()
                        .find_map(|(words, reply)| hits(text, words).then_some(*reply))
                };
                for_symptom(current).or_else(|| for_symptom(&previous))?
            }
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn medicine_cards_keep_source_question_branches() {
        type Card = fn(&Reading<'_>) -> Option<Cow<'static, str>>;
        let cases: &[(Card, &str, &str)] = &[
            (
                melatonin,
                "Is melatonin safe for my 10 year old?",
                "prescribed for children",
            ),
            (
                melatonin,
                "Can I use melatonin while pregnant?",
                "not usually used in pregnancy",
            ),
            (
                morning_after,
                "Where can I get the morning after pill?",
                "Ask a pharmacy",
            ),
            (
                morning_after,
                "Does an emergency copper IUD fitting hurt?",
                "can be uncomfortable",
            ),
            (
                a_statin_and_the_day,
                "What time should I take simvastatin?",
                "evening",
            ),
            (
                a_statin_and_the_day,
                "Can I take atorvastatin with my antibiotics?",
                "the name of your statin",
            ),
            (
                missed_medication,
                "I missed a dose of my antibiotics",
                "antibiotic's leaflet",
            ),
            (
                missed_medication,
                "My dad ran out of his tablets",
                "emergency supply",
            ),
            (
                medicine_for_a_young_baby,
                "Can my two month old have paracetamol?",
                "38°C",
            ),
        ];
        for &(answer, question, expected) in cases {
            let actual = answer(&Reading::new(question)).expect(question);
            assert!(actual.contains(expected), "{question}: {actual}");
        }
        assert!(!named_medicine_guard(&Reading::new(
            "How much ibuprofen can I take?"
        )));
        assert!(named_medicine_guard(&Reading::new(
            "My ibuprofen isnt working"
        )));
    }

    #[test]
    fn pharmacy_prior_turn_carries_a_symptom_and_preserves_the_pet_route() {
        let reading = Reading::new("Can I buy anything from the chemist?");
        let mut earlier = [Exchange {
            user: "I have heartburn".into(),
            bot: String::new(),
            module: "answers",
        }];
        let reply = pharmacy("at-the-counter", &reading, &earlier).unwrap();
        assert!(reply.contains("antacids"));
        earlier[0].user = "My cat has a runny eye".into();
        assert!(pharmacy("at-the-counter", &reading, &earlier).is_none());
        let reply = pharmacy("a-pet-at-the-counter", &reading, &earlier).unwrap();
        assert!(reply.contains("vet"));
    }
}
