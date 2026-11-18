//! Source clinical card triggers with reviewed NHS and UKHSA response wording.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Reading, understanding::topics};

macro_rules! source_pattern {
    ($reader:ident, $constant:ident) => {
        static $reader: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(super::$constant).expect(stringify!($constant)));
    };
}

source_pattern!(KNOWN_ILLNESS, CLINICAL_KNOWN_ILLNESS);
source_pattern!(NAMED_ILLNESS, CLINICAL_NAMED_ILLNESS);
source_pattern!(CANNOT_WARM, CLINICAL_CANNOT_WARM);
source_pattern!(WARM_UP, CLINICAL_WARM_UP);
source_pattern!(HEAP, CLINICAL_HEAP);
source_pattern!(CHILDS_AGE, CLINICAL_CHILDS_AGE);
source_pattern!(WHEN_ASK, CLINICAL_WHEN_ASK);
source_pattern!(RATE_MEANING, CLINICAL_RATE_MEANING);
source_pattern!(SAYS_COLD, CLINICAL_SAYS_COLD);
source_pattern!(SAYS_HOT, CLINICAL_SAYS_HOT);
source_pattern!(WATCHING_EYE, CLINICAL_WATCHING_EYE);

fn words<'a>(reading: &'a Reading<'_>) -> Cow<'a, str> {
    if reading.line.words.is_empty() {
        Cow::Borrowed(&reading.line.clean)
    } else {
        Cow::Owned(reading.line.words.join(" "))
    }
}

fn has(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

pub(super) fn an_illness_the_card_covers(reading: &Reading<'_>) -> bool {
    let text = words(reading);
    KNOWN_ILLNESS.is_match(&text) || !NAMED_ILLNESS.is_match(&text)
}

pub(super) fn not_on_the_warm_up(reading: &Reading<'_>) -> bool {
    let text = words(reading);
    CANNOT_WARM.is_match(&text) || !WARM_UP.is_match(&text)
}

pub(super) fn a_wound_not_a_cramp(reading: &Reading<'_>) -> bool {
    let text = words(reading);
    let tokens = text.split_whitespace();
    let names = |choices: &[&str]| tokens.clone().any(|word| choices.contains(&word));
    !names(super::CLINICAL_RUNNING)
        && (names(&[
            "stitches", "sutures", "suture", "glued", "glue", "staple", "staples",
        ]) || (names(&["stitch"])
            && names(&[
                "cut", "cuts", "wound", "wounds", "gash", "slashed", "bleeding",
            ])))
}

pub(super) fn a_heap_not_the_condition(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    !HEAP.is_match(&format!("{} {} {}", line.clean, line.norm_text, line.text))
}

pub(super) fn not_a_childs_age(reading: &Reading<'_>) -> bool {
    !CHILDS_AGE.is_match(&words(reading))
}

pub(super) fn not_a_when_ask(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    !WHEN_ASK.is_match(&format!(
        "{} {} {} {}",
        line.clean,
        line.text,
        reading.raw,
        line.words.join(" ")
    ))
}

pub(super) fn not_the_meaning_of_high_or_low(reading: &Reading<'_>) -> bool {
    !RATE_MEANING.is_match(&reading.line.clean)
        || !reading
            .line
            .words
            .iter()
            .any(|word| super::CLINICAL_RATE_QUALIFIERS.contains(&word.as_str()))
}

pub(super) fn not_keeping_an_eye_on_it(reading: &Reading<'_>) -> bool {
    if reading.line.text.is_empty() {
        !WATCHING_EYE.is_match(&words(reading))
    } else {
        !WATCHING_EYE.is_match(&reading.line.text)
    }
}

pub(super) fn says_cold(reading: &Reading<'_>) -> bool {
    SAYS_COLD.is_match(&words(reading))
}

pub(super) fn says_hot(reading: &Reading<'_>) -> bool {
    SAYS_HOT.is_match(&words(reading))
}

pub(super) fn feels_cold(reading: &Reading<'_>) -> bool {
    topics::warmth(&reading.line.norm).is_some_and(|term| super::CLINICAL_COLD.contains(&term))
}

pub(super) fn feels_hot(reading: &Reading<'_>) -> bool {
    topics::warmth(&reading.line.norm).is_some_and(|term| super::CLINICAL_HOT.contains(&term))
}

fn fear_subject(reading: &Reading<'_>) -> Option<String> {
    let text = reading
        .line
        .clean
        .split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let (_, _, rest) = super::FEAR_MARKERS
        .iter()
        .filter_map(|marker| {
            text.match_indices(marker).find_map(|(at, phrase)| {
                let following = at + phrase.len();
                (at == 0 || text.as_bytes()[at - 1] == b' ').then_some(())?;
                (text.as_bytes().get(following) == Some(&b' ')).then_some((
                    at,
                    std::cmp::Reverse(phrase.len()),
                    &text[following..],
                ))
            })
        })
        .min_by_key(|&(at, length, _)| (at, length))?;
    let mut subject = Vec::with_capacity(5);
    let mut doing = false;
    for word in rest.split(' ').filter(|word| !word.is_empty()) {
        let word = word.trim_matches([',', '.', '!', '?', ';', ':', '"', '\'']);
        if word.is_empty() || subject.len() == if doing { 5 } else { 3 } {
            break;
        }
        if super::FEAR_STOP_WORDS.contains(&word)
            && !(doing
                && (super::FEAR_LINK_WORDS.contains(&word)
                    || super::FEAR_DETERMINERS.contains(&word)))
        {
            break;
        }
        subject.push(word);
        if subject.len() == 1 {
            doing = word.len() > 4 && word.ends_with("ing");
        }
    }
    while subject
        .last()
        .is_some_and(|word| super::FEAR_FILLER_WORDS.contains(word))
    {
        subject.pop();
    }
    while subject.len() > 1
        && subject
            .last()
            .is_some_and(|word| super::FEAR_LINK_WORDS.contains(word))
    {
        subject.pop();
    }
    let first = *subject.first()?;
    if (doing && !super::FEAR_ING_NOUNS.contains(&first) && !super::FEAR_GERUNDS.contains(&first))
        || (subject.len() == 1
            && matches!(
                first,
                "the" | "a" | "an" | "my" | "it" | "them" | "that" | "this"
            ))
    {
        return None;
    }
    Some(subject.join(" "))
}

pub(super) fn fear_has_subject(reading: &Reading<'_>) -> bool {
    fear_subject(reading).is_some()
}

pub(super) fn fear_reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let subject = fear_subject(reading)?;
    Some(Cow::Owned(format!(
        "Fear of {subject} can be difficult to live with. If it is getting in the way of daily life, speak to a GP or contact an NHS talking therapies service. CBT can help, and a therapist can help you choose gradual steps that feel manageable and are safe to practise."
    )))
}

const ALCOHOL: &str = "UK guidance for adults who drink regularly advises up to 14 units a week, spread over at least 3 days if you drink that much. Less alcohol means less risk. Work out units by multiplying the drink's volume in millilitres by its ABV percentage and dividing by 1,000. A 175 ml glass of 12% wine is about 2.1 units. If you get withdrawal symptoms, get medical advice before cutting down or stopping.";

fn eye_infection(reading: &Reading<'_>) -> Cow<'static, str> {
    const ESCALATE: &str = "Get urgent advice for a very red, painful eye, a red eye with contact lenses, or a newborn with red, sticky eyes. Go to A&E or call 999 for a red eye with changed vision, pain in bright light, an eye injury or a severe headache with nausea.";
    let text = &reading.line.clean;
    let lids = reading.line.words.iter().any(|word| {
        matches!(
            word.as_str(),
            "stye" | "sty" | "styes" | "eyelid" | "eyelids" | "lash" | "lashes" | "lid"
        )
    });
    let opener = if has(
        text,
        &["itchy", "itch", "itches", "itching", "watery", "streaming"],
    ) && !has(
        text,
        &[
            "sticky",
            "crusty",
            "gunge",
            "gritty",
            "pus",
            "discharge",
            "glued",
            "infection",
        ],
    ) && !lids
    {
        "An itchy, watery eye can come from an allergy. Try a cool compress, avoid rubbing it, and ask a pharmacist about suitable drops."
    } else if lids {
        "For a small lump on the eyelid that looks like a stye, hold a clean warm flannel on it for 5 to 10 minutes a few times a day. Leave it alone instead of squeezing it and avoid eye make-up or contact lenses while it heals."
    } else {
        "Sticky or gritty red eyes can have several causes, including conjunctivitis. Keep contact lenses out and gently wipe away discharge with clean water and a fresh pad for each eye. Ask a pharmacist, GP or optician for advice if you are unsure."
    };
    Cow::Owned(format!("{opener} {ESCALATE}"))
}

pub(super) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = match id {
        "contagious-window" => {
            "How long you can pass an infection on depends on the illness. For diarrhoea or vomiting, stay away from work or school for at least 48 hours after symptoms stop. With flu or another respiratory illness, stay home if you have a high temperature or feel too unwell for normal activities. Follow the current public health guidance for the specific illness, especially around people at higher risk."
        }
        "someone-is-cold" => {
            "If they are confused, unusually sleepy, slurring their words or breathing slowly after getting cold, suspect hypothermia and call 999 now. Move them somewhere sheltered, remove wet clothing and wrap them in dry blankets. Offer a warm non-alcoholic drink only if they are fully awake. For ordinary chilliness, dry layers and a warm room can help."
        }
        "someone-is-hot" => {
            "Take their temperature if you can and offer fluids if they can drink. Call 999 if they are hard to wake, are struggling to breathe, or have a stiff neck or a rash that stays visible when pressed. A baby under 3 months with a temperature of 38°C or more needs urgent advice from NHS 111. Seek medical advice if they are getting worse or you are worried."
        }
        "units-a-week" | "alcohol-units" => ALCOHOL,
        "stitches-out" => {
            "Follow the removal date and wound-care instructions given by the team that closed the cut. Dissolvable stitches usually come away on their own. Non-dissolving stitches and staples need to be removed by a clinician. Call the clinic or your GP surgery if you are unsure which you have or when to book. Get urgent advice if the wound opens or becomes increasingly red, swollen or painful."
        }
        "cut-needs-stitches" => {
            "Put firm pressure on the cut with a clean dressing or cloth. Go to A&E or call 999 if bleeding will not stop, the wound is very deep or large, there is something stuck in it, or feeling or movement is affected. Get prompt advice from NHS 111 if the edges gape, it has reopened or it is contaminated. A clinician can decide whether to use stitches, strips or glue and whether you need tetanus protection."
        }
        "always-cold" => {
            "Feeling cold persistently can have several causes, including an underactive thyroid. If this is new, becoming worse, or comes with tiredness or other changes, book a GP appointment. They can assess your symptoms and decide what tests, if any, are appropriate."
        }
        "piles" => {
            "Fibre, fluids and avoiding straining can help with piles. A pharmacist can suggest symptom relief. See a GP if it is getting worse or keeps returning. Seek urgent advice for painless bleeding, pus or feeling feverish and unwell. Go to A&E or call 999 for heavy or continuous bleeding, large clots or severe pain."
        }
        "fertility" => {
            "Speak to your GP if you have been trying to conceive for a year without success. Get advice sooner if the person trying to become pregnant is 36 or older, or if either partner has a known fertility concern. The GP can explain the next steps for your circumstances."
        }
        "need-glasses" => {
            "Book a sight test with an optometrist if your vision has become blurry, you struggle to see at particular distances, or you are getting headaches with visual tasks. The test can check whether glasses would help and look for other eye problems. Sudden loss of vision or sudden severe eye pain needs emergency care."
        }
        "resting-heart-rate" => {
            "For most adults, a typical resting pulse is 60 to 100 beats per minute. It can be lower in people who train regularly. A one-off reading needs context. Ask a clinician about a new, persistent change or repeated palpitations. Call 999 for a racing heartbeat with chest pain, shortness of breath or feeling faint."
        }
        "eye-infection" => return Some(eye_infection(reading)),
        _ => return None,
    };
    Some(Cow::Borrowed(text))
}

static CONDITION_DENIALS: LazyLock<[Regex; 4]> = LazyLock::new(|| {
    [
        r"blood\s+pressure|bp|hypertension|hypertensive",
        r"diabetes|diabetic",
        r"epilepsy|epileptic",
        r"asthma|asthmatic",
    ]
    .map(|terms| {
        Regex::new(&super::CLINICAL_DENIAL_TEMPLATE.replace("%s", terms))
            .expect("valid condition denial pattern")
    })
});

pub(super) fn has_digits(text: &str) -> bool {
    text.chars().any(|ch| ch.is_ascii_digit())
}

pub(super) fn blood_sugar_reading(reading: &Reading<'_>) -> bool {
    !super::guards::high_glucose_reading(&reading.line.clean)
}

pub(super) fn condition_mentioned(id: &str, reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let index = match id {
        "blood-pressure" => 0,
        "diabetes" => {
            if super::guards::high_glucose_reading(&line.clean)
                || super::guards::high_glucose_reading(&line.text)
            {
                return false;
            }
            1
        }
        "epilepsy" => 2,
        "asthma-condition" => 3,
        _ => return false,
    };
    let text = if !line.text.is_empty() {
        Cow::Borrowed(line.text.as_str())
    } else {
        words(reading)
    };
    !CONDITION_DENIALS[index].is_match(&text.to_lowercase())
}

source_pattern!(PET_AS_SUBJECT, CLINICAL_PET_AS_SUBJECT);
source_pattern!(PET_SYMPTOM, CLINICAL_PET_SYMPTOM);
source_pattern!(PERSON_REACTING, CLINICAL_PERSON_REACTING);

pub(super) fn human_pet_allergy(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    let Some(subject) = PET_AS_SUBJECT.find(text) else {
        return true;
    };
    let rest = &text[subject.end()..];
    PET_SYMPTOM
        .find(rest)
        .is_none_or(|symptom| PERSON_REACTING.is_match(&rest[..symptom.start()]))
}

pub(super) fn returning_to_exercise(reading: &Reading<'_>) -> bool {
    has(
        &words(reading),
        &[
            "again",
            "back into",
            "back to",
            "get back",
            "getting back",
            "got back",
            "restart",
            "re-start",
            "havent",
            "haven't",
            "not exercised in",
            "not done",
            "after a break",
            "after years",
            "after a long",
            "years off",
            "years since",
            "since school",
            "used to",
            "out of shape",
            "stopped exercising",
        ],
    )
}

pub(super) fn condition_reply(id: &str, _: &Reading<'_>) -> Option<Cow<'static, str>> {
    let reply = match id {
        "blood-pressure" => {
            "Blood pressure is generally high at 135/85 or more when checked at home, or 140/90 or more at a GP surgery or pharmacy. Ask your GP how to check it over several days and bring the readings for review. Call 999 for possible stroke symptoms or ongoing chest pain."
        }
        "diabetes" => {
            "In type 1 diabetes the body makes very little or no insulin and insulin treatment is needed. In type 2 the body does not respond properly to insulin or makes too little. If you think you or your child may have type 1 diabetes, get an urgent GP appointment or contact NHS 111. Ask your diabetes team about your HbA1c and the regular checks and treatments suitable for you."
        }
        "epilepsy" => {
            "Epilepsy causes a tendency to have repeated seizures. Seizures can look like a brief loss of awareness, unusual sensations or jerking. A specialist can advise on the diagnosis and treatment. Follow DVLA rules if you have had a seizure and get advice before driving. Call 999 if this is the person's first seizure, it lasts over five minutes when you do not know their usual duration, they have another without recovering, or they have breathing difficulty."
        }
        "asthma-condition" => {
            "Asthma can cause wheezing, cough, shortness of breath and chest tightness. Ask your asthma team for a written action plan and advice on which inhaler to use, because treatment differs between inhaler types. Seek a review if symptoms are getting worse or waking you. Call 999 during an asthma attack if you are getting worse, your inhaler has not helped after the maximum dose in your plan, or you have no inhaler."
        }
        "blood-sugar-a-reading" => {
            "A blood glucose reading depends on the units, when you last ate and your individual targets. Record the number, time, symptoms and whether you had eaten. Follow your diabetes team's advice if you have one. Contact your GP if you have diabetes symptoms or concerning readings without a diagnosis. Seek urgent help for vomiting, drowsiness, confusion or fast breathing with high blood glucose."
        }
        "pet-allergy" => {
            "Pet dander and other animal proteins can trigger allergy symptoms. Try keeping pets out of the bedroom, washing your hands after contact and cleaning with a damp cloth. Ask a pharmacist which allergy treatment suits you, or contact your GP if symptoms are persistent or make your asthma worse."
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}
