//! NHS wording for source cataract, sight test, dental, and knee cards.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::Reading;

use super::super::lookup::{
    BREASTFEEDING_COULD_NOT, BREASTFEEDING_ILL, BREASTFEEDING_LENGTH, BREASTFEEDING_PAIN,
    BREASTFEEDING_PUBLIC,
};

const SUDDEN_SIGHT: &str = "If you suddenly cannot see from one or both eyes or have sudden severe eye pain, go to A&E or call 999. Do not drive yourself.";
const CATARACT_LENGTH: &str = "Cataract surgery usually takes 20 to 45 minutes. The cloudy lens is replaced with an artificial lens, usually under local anaesthetic. Most people can go home the same day. Arrange a lift home and follow the surgical team's advice about recovery and driving.";
const CATARACT: &str = "A cataract is clouding of the lens inside the eye. It can cause gradually blurred vision or glare. An optometrist can check what is causing it. Stronger glasses or brighter lighting can help for a while, and an eye specialist can discuss surgery if it affects daily life. Sudden loss of sight or sudden severe eye pain needs emergency care.";
const MILK_TOOTH: &str = "Milk teeth usually start falling out naturally at around six. Let a wobbly tooth come out on its own. If a tooth was knocked out by an injury, call for an emergency dentist appointment or NHS 111 straight away. Do not put a baby tooth back into its socket.";
const ADULT_TOOTH: &str = "A loose permanent tooth needs a dentist to check the cause. Arrange an appointment promptly and avoid wiggling it. If the tooth was knocked out in an injury, ask for an emergency dental appointment straight away.";
const KNOCKED_TOOTH: &str = "Call a dentist now for an emergency appointment, or call NHS 111 if you cannot get one. If a permanent tooth was knocked out, handle it by the crown, avoid touching the root and store it in milk if it cannot easily go back in. Never put a baby tooth back in.";
const SIGHT_COST: &str = "In England, NHS sight tests are free for eligible people, including those under 16 or aged 60 and over. Other eligibility rules apply, including for some benefits and eye conditions. Ask your optician about eligibility or the price of a private test. Most adults should have a sight test every two years or more often if advised.";
const SIGHT_INTERVAL: &str = "A sight test checks your eye health as well as whether you need glasses. Most adults should have one every two years, or sooner if advised by their optometrist. In England, ask your optician about a free NHS sight test if eligible, or the price of a private test.";
const KNEE_URGENT: &str = "Get advice from NHS 111 now if the knee clicks painfully, locks or gives way, is badly swollen, has changed shape, or you cannot put weight on it. A hot or red knee with fever also needs urgent advice.";
const KNEE_NOISY: &str = "Painless clicking on its own is usually normal. If your knee starts clicking painfully, gives way, locks or becomes badly swollen, get advice from NHS 111.";

macro_rules! pattern {
    ($name:ident, $pattern:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($pattern).expect(stringify!($name)));
    };
}

pattern!(
    EYE_COST,
    r"(?i)\b(?:free|cost|costs|pay|paid|price|how much|charge|charges|afford|nhs)\b"
);
pattern!(
    EYE_WHEN,
    r"(?i)\b(?:how often|how frequently|when|how soon)\b"
);
pattern!(EYE_YES, r"(?i)\b(?:should|shall|ought)\b");
pattern!(
    EYE_EMERGENCY,
    r"\b(?:(?:suddenly|sudden) (?:can'?t see|cannot see|couldn'?t see|lost (?:my |the )?(?:sight|vision)|(?:loss of )?(?:sight|vision) loss|loss of (?:sight|vision)|severe eye pain)|(?:lost (?:my |the )?(?:sight|vision)|can'?t see|cannot see) suddenly)\b"
);
pattern!(
    DENTAL_INJURY,
    r"\b(?:knock(?:ed|s)?(?: \w+){0,4} out|(?:hit|bang|blow|fall|accident|injury)(?: \w+){0,4} tooth|tooth(?: \w+){0,4} (?:knocked|bang|fall|injury))\b"
);
pattern!(
    KNEE_ALERT,
    r"\b(?:locks?|locking|giving way|gives way|gave way|badly swollen|changed shape|can(?:not|'?t) (?:walk|bear weight|put weight))\b"
);

fn contains_any(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

pub(super) fn cataract(reading: &Reading<'_>) -> &'static str {
    let said = &reading.line.clean;
    if EYE_EMERGENCY.is_match(said) {
        return SUDDEN_SIGHT;
    }
    if contains_any(said, &["how long", "how many minutes", "how soon"])
        && contains_any(
            said,
            &["operation", "surgery", "operate", "procedure", "the op"],
        )
        && !contains_any(
            said,
            &[
                "wait",
                "recover",
                "heal",
                "back to work",
                "back to normal",
                "off work",
                "drive again",
                "after the op",
            ],
        )
    {
        return CATARACT_LENGTH;
    }
    CATARACT
}

pub(super) fn sight_test(reading: &Reading<'_>) -> std::borrow::Cow<'static, str> {
    let said = if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    };
    if EYE_EMERGENCY.is_match(said) {
        return SUDDEN_SIGHT.into();
    }
    if EYE_COST.is_match(said) {
        return SIGHT_COST.into();
    }
    if !EYE_WHEN.is_match(said) && EYE_YES.is_match(said) {
        return format!("Yes. {SIGHT_INTERVAL}").into();
    }
    SIGHT_INTERVAL.into()
}

pub(super) fn milk_tooth(reading: &Reading<'_>) -> &'static str {
    let said = &reading.line.clean;
    if DENTAL_INJURY.is_match(said) {
        return KNOCKED_TOOTH;
    }
    if contains_any(
        said,
        &[
            "adult tooth",
            "adult teeth",
            "my tooth",
            "permanent tooth",
            "i am 30",
            "i'm 30",
            "i am 40",
            "i'm 40",
            "grown up tooth",
            "my own tooth",
            "gum disease",
        ],
    ) && !contains_any(
        said,
        &[
            "my son",
            "my daughter",
            "my child",
            "my kid",
            "toddler",
            "baby",
        ],
    ) {
        return ADULT_TOOTH;
    }
    MILK_TOOTH
}

pub(super) fn knee_noise(reading: &Reading<'_>) -> &'static str {
    let words = &reading.line.words;
    let mentions = |terms: &[&str]| words.iter().any(|word| terms.contains(&word.as_str()));
    if KNEE_ALERT.is_match(&reading.line.clean)
        || (mentions(&[
            "hurt", "hurts", "hurting", "pain", "painful", "ache", "aches", "aching", "sore",
            "swollen", "swelling", "sharp", "stiff",
        ]) && !mentions(&[
            "not", "no", "dont", "doesnt", "didnt", "isnt", "never", "without", "hardly",
        ]))
    {
        return KNEE_URGENT;
    }
    KNEE_NOISY
}

pub(super) fn breastfeeding(reading: &Reading<'_>) -> &'static str {
    let text = format!(" {} ", reading.line.clean);
    let asks_about = |terms: &[&str]| contains_any(&text, terms);
    if asks_about(&[
        "fewer wet nappies",
        "no wet nappy",
        "not wetting",
        "hard to wake",
        "won't wake",
        "wont wake",
        "won't feed",
        "wont feed",
        "stopped feeding",
        "refusing feeds",
        "feeding much less",
        "sleepy and not feeding",
    ]) {
        return "If your baby is feeding much less than usual or has fewer wet nappies than expected for their age, contact your midwife, infant feeding team or NHS 111 now. Call 999 if you cannot wake your baby.";
    }
    if asks_about(BREASTFEEDING_COULD_NOT) {
        return "How you feed your baby is your decision. First infant formula is suitable from birth, and mixed feeding is an option. A midwife or health visitor can help if you want to switch, combine feeds or get support with breastfeeding. Tell your midwife or GP if guilt or low mood is weighing on you.";
    }
    if asks_about(BREASTFEEDING_PUBLIC) {
        return "You can breastfeed in public. In Great Britain, the Equality Act protects you from unfavourable treatment for breastfeeding while receiving services. A quiet spot or a cover is a choice if it makes you more comfortable.";
    }
    if asks_about(BREASTFEEDING_PAIN) {
        return "Ask your midwife or health visitor for help if breastfeeding hurts, particularly if nipples are cracked or bleeding. They can watch a feed and check the latch. A swollen, hot or painful area with fever may be mastitis. Ask for medical advice if symptoms worsen or do not improve.";
    }
    if asks_about(BREASTFEEDING_ILL) {
        return "For many common illnesses, such as colds, you can keep breastfeeding. If you're seriously unwell, or need medicine, ask your midwife, GP or pharmacist about your illness and the exact product. If you're too tired to feed safely, ask someone to help and put your baby on their back in a separate clear sleep space after the feed.";
    }
    if asks_about(BREASTFEEDING_LENGTH) {
        return "Exclusive breastfeeding is recommended for around the first six months. After that, you can continue breastfeeding alongside solid foods for as long as it suits you and your baby.";
    }
    if asks_about(&[" formula", " bottle ", " bottles "]) {
        return "First infant formula can be used from birth. You can combine breast and bottle feeds if that works for you. Follow the preparation instructions for your exact formula and ask your midwife or health visitor if you want help making the change.";
    }
    "Offer the breast in response to feeding cues. From day five, at least six heavy wet nappies a day and steady weight gain are signs your baby is getting enough milk. Contact your midwife, health visitor or NHS 111 promptly if your baby feeds much less, is unusually sleepy or has fewer wet nappies than expected."
}

pub(super) fn painkiller_dose(reading: &Reading<'_>) -> Option<&'static str> {
    let words = &reading.line.words;
    let mentions = |names: &[&str]| words.iter().any(|word| names.contains(&word.as_str()));
    if mentions(&[
        "aspirin",
        "disprin",
        "anadin",
        "naproxen",
        "diclofenac",
        "voltarol",
        "tramadol",
        "solpadeine",
        "veganin",
        "feminax",
        "migraleve",
        "dydramol",
        "paracodol",
        "dihydrocodeine",
    ]) {
        return None;
    }
    let codeine = mentions(&[
        "codeine",
        "co-codamol",
        "co codamol",
        "cocodamol",
        "codamol",
    ]);
    let paracetamol = mentions(&["paracetamol", "acetaminophen", "calpol", "panadol"]);
    let ibuprofen = mentions(&["ibuprofen", "nurofen", "brufen"]);
    Some(if codeine {
        "Check the exact product, strength and prescribed directions with a pharmacist. Co-codamol already contains paracetamol, so avoid adding other medicines that contain it unless your doctor advises it. If you have taken more than the recommended dose of co-codamol, call NHS 111 now even if you feel well. Call 999 if you are having trouble breathing."
    } else if ibuprofen && !paracetamol {
        "Check the dose and maximum for the exact ibuprofen strength and form on the leaflet or prescription. Ask a pharmacist first if you take other medicines, have had a stomach ulcer or have kidney, heart or liver problems."
    } else if paracetamol && !ibuprofen {
        "Follow the dose and spacing for the exact paracetamol product. Check other medicines for paracetamol before combining them. Ask a pharmacist if you weigh under 50 kg, have liver or kidney problems or are unsure of the dose. Call NHS 111 now if you've taken more than the recommended dose, even if you feel well."
    } else {
        "Check each painkiller's leaflet or prescription for the dose, spacing and maximum. Ask a pharmacist whether the medicines can be taken together and check that you aren't doubling up on an ingredient."
    })
}

pub(super) fn painkiller_effects(reading: &Reading<'_>) -> &'static str {
    let line = &reading.line.clean;
    if contains_any(
        line,
        &[
            "vomiting blood",
            "vomit blood",
            "vomited blood",
            "coffee grounds",
            "black tarry",
            "tarry stool",
            "severe tummy pain",
            "severe stomach pain",
        ],
    ) {
        return "Get urgent medical help for vomiting blood, vomit that looks like coffee grounds, tarry black stools or severe stomach pain. Go to A&E or call 999 if you are unwell or have severe pain or black, sticky stools. If vomiting blood has stopped and you have no other symptoms, call NHS 111 or ask for an urgent GP appointment.";
    }
    let paracetamol = contains_any(
        line,
        &[
            "paracetamol",
            "acetaminophen",
            "panadol",
            "calpol",
            "co codamol",
            "co-codamol",
        ],
    );
    let nsaid = contains_any(
        line,
        &[
            "ibuprofen",
            "nurofen",
            "brufen",
            "naproxen",
            "aspirin",
            "diclofenac",
            "nsaid",
            "inflammatory",
            "voltarol",
            "indomethacin",
        ],
    );
    if paracetamol && !nsaid {
        return "Paracetamol is a different type of painkiller from ibuprofen. Follow your exact pack or prescription, check for other medicines containing paracetamol and ask a pharmacist if you take other medicines or have liver or kidney problems.";
    }
    if contains_any(
        line,
        &[
            "kidney",
            "kidneys",
            "renal",
            "water tablet",
            "diuretic",
            "blood pressure",
        ],
    ) {
        return "Anti-inflammatory painkillers such as ibuprofen can affect the kidneys. Ask a pharmacist or prescriber before taking them if you have kidney problems, are dehydrated or take a diuretic or blood pressure medicine. They can help you choose and dose an option safely.";
    }
    "Ibuprofen and other anti-inflammatory painkillers can irritate the stomach and sometimes cause bleeding. Follow the exact product instructions and ask a pharmacist if you have had an ulcer or take blood thinners. Get urgent help for vomiting blood, tarry black stools or severe stomach pain."
}
