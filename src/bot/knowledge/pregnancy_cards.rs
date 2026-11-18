//! Narrow source guards for pregnancy questions and dated cycles.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

static PREGNANT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PREGNANCY_SAID).expect("pregnancy pattern"));
static PREGNANCY_DENIED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(super::PREGNANCY_DENIED).expect("pregnancy denial pattern"));
static PERIOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:periods?|cycles?|pregnan\w*|tests?)\b").unwrap());

pub(super) fn said_pregnant(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let text = format!("{} {} {}", line.clean, line.text, line.norm_text).to_lowercase();
    !PREGNANCY_DENIED.is_match(&text) && PREGNANT.is_match(&text)
}

pub(super) fn a_late_that_is_not_a_period(reading: &Reading<'_>) -> bool {
    reading.sense.get("late") != Some(&"time")
        && (reading.context_fields.shift.is_none() || PERIOD.is_match(&reading.line.clean))
}

pub(super) fn pregnancy_milk(_: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(
        "You can drink pasteurised milk during pregnancy. Avoid unpasteurised cow's, goat's and sheep's milk. Follow NHS pregnancy guidance for cheese, since what is safe depends on the type and how it is prepared. If you choose a plant drink, check the label for added calcium and iodine.",
    ))
}

pub(super) fn pregnancy_headache(_: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(
        "Contact your midwife, maternity unit or NHS 111 immediately if you have a severe headache, blurred vision or flashing lights, pain below your ribs, vomiting, or sudden swelling of your face, hands or feet. These can be signs of pre-eclampsia. Paracetamol is usually the first choice for pain in pregnancy. Check with your midwife, doctor or pharmacist before taking anything else.",
    ))
}

use jiff::{Span, Zoned, civil::Date};

macro_rules! pattern {
    ($name:ident, $source:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(super::$source).expect(stringify!($source)));
    };
}

pattern!(DUE_SHAPE, DUE_SHAPE);
pattern!(DUE_ASK, DUE_ASK);
pattern!(GIVES_DAY, DUE_GIVES_DAY);
pattern!(MONTH, DUE_MONTH);
pattern!(SHORT_ASK, DUE_SHORT_ASK);
pattern!(ELSEWHERE, DUE_ELSEWHERE);
pattern!(MILESTONE, DUE_MILESTONE);
pattern!(PERIOD_ONLY, DUE_PERIOD_ONLY);
pattern!(PERIOD_PREGNANCY, DUE_PERIOD_PREGNANCY);
pattern!(PREG_WORD, DUE_PREG_WORD);
pattern!(PREG_SOFT, DUE_PREG_SOFT);
pattern!(OTHER_SUBJECT, DUE_OTHER_SUBJECT);
pattern!(WEEKS_NOW, DUE_WEEKS_NOW);
pattern!(DAY_THEN_MONTH, DUE_DAY_THEN_MONTH);
pattern!(MONTH_THEN_DAY, DUE_MONTH_THEN_DAY);
pattern!(NOT_BIRTH, NOT_BIRTH_LINE);
pattern!(BIRTH_LINE, BIRTH_LINE);

pub(super) fn gives_month(text: &str) -> bool {
    MONTH.is_match(text)
}

pub(super) fn asks_when_due(reading: &Reading<'_>) -> bool {
    let text = &reading.line.clean;
    DUE_SHAPE.is_match(text)
        && (DUE_ASK.is_match(text)
            || (GIVES_DAY.is_match(text) && gives_month(text))
            || (SHORT_ASK.is_match(text) && text.split_whitespace().count() <= 4))
        && !ELSEWHERE.is_match(text)
        && !MILESTONE.is_match(text)
        && (!PERIOD_ONLY.is_match(text) || PERIOD_PREGNANCY.is_match(text))
        && (PREG_WORD.is_match(text) || (PREG_SOFT.is_match(text) && !OTHER_SUBJECT.is_match(text)))
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

fn render_date(date: Date, today: Date) -> String {
    let month = MONTH_NAMES[(date.month() - 1) as usize];
    if date.year() == today.year() {
        format!("{} {month}", date.day())
    } else {
        format!("{} {month} {}", date.day(), date.year())
    }
}

fn lmp_date(text: &str, today: Date) -> Option<Date> {
    let date = |name: &str, day: &str| {
        let month = super::DUE_MONTH_NUMBERS
            .iter()
            .find(|&&(key, _)| name.eq_ignore_ascii_case(key))?
            .1;
        let day = day.parse().ok()?;
        let date = Date::new(today.year(), month, day).ok()?;
        if date <= today.checked_add(Span::new().days(3)).ok()? {
            Some(date)
        } else {
            Date::new(today.year() - 1, month, day).ok()
        }
    };
    DAY_THEN_MONTH
        .captures_iter(text)
        .find_map(|m| date(&m[2], &m[1]))
        .or_else(|| {
            MONTH_THEN_DAY
                .captures_iter(text)
                .find_map(|m| date(&m[1], &m[2]))
        })
}

fn due_date_at(text: &str, today: Date) -> Option<Cow<'static, str>> {
    const CONFIRM: &str = "Use the estimated due date recorded by your maternity team if a dating scan has given you one.";
    let weeks: Option<i32> = WEEKS_NOW.captures_iter(text).find_map(|m| {
        let week = m.get(1).or_else(|| m.get(2))?.as_str().parse().ok()?;
        (1..=45).contains(&week).then_some(week)
    });
    if let Some(week) = weeks {
        if week > 40 {
            return Some(Cow::Owned(format!(
                "At {week} weeks, you are past the estimated 40-week date. Contact your maternity team about their recommended next steps and refer to the date in your maternity notes."
            )));
        }
        let left = 40 - week;
        let due = today.checked_add(Span::new().days(left * 7)).ok()?;
        return Some(Cow::Owned(format!(
            "At {week} weeks, there are about {left} weeks to the estimated due date, {}. {CONFIRM}",
            render_date(due, today)
        )));
    }
    if let Some(start) = lmp_date(text, today) {
        let due = start.checked_add(Span::new().days(280)).ok()?;
        let days = today.duration_since(start).as_secs() / 86_400;
        let mut answer = format!(
            "Counting 280 days from the first day of your last period, {}, gives an estimated due date of {}.",
            render_date(start, today),
            render_date(due, today)
        );
        if (0..=300).contains(&days) {
            answer.push_str(&format!(
                " That is about {} weeks pregnant today.",
                days / 7
            ));
        }
        answer.push(' ');
        answer.push_str(CONFIRM);
        return Some(Cow::Owned(answer));
    }
    Some(Cow::Borrowed(
        "An estimated due date is usually around 40 weeks from the first day of your last period. A dating scan may give a different estimate. Check the date in your maternity notes, or tell me the date your last period started.",
    ))
}

pub(super) fn due_date(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    due_date_at(&reading.line.clean, Zoned::now().date())
}

const FOOD_GUIDANCE: &[&str] = &[
    "Pineapple is generally fine in pregnancy. Wash fresh fruit and choose pasteurised juice. Contact your maternity team if you think labour has started.",
    "Choose pasteurised juice, and follow the label for storage and use-by dates. Whole fruit provides more fibre.",
    "Wash fruit well under running water before eating it, including fruit you will peel or cut. Follow any use-by date on ready-cut fruit.",
    "Wash and prepare fruit hygienically and check NHS pregnancy food guidance. Ask your midwife if you are worried about a specific food or supplement.",
    "The advice to avoid honey applies to babies under one year. If you eat it yourself, check any other ingredients against pregnancy food guidance.",
    "For hummus, follow the use-by date and the pack's instructions for refrigeration and how long to keep it after opening.",
    "You can eat spicy food during pregnancy if you tolerate it. If it makes heartburn worse, smaller meals or reducing the foods that trigger it may help.",
    "Choose ice cream made with pasteurised milk and follow the seller's storage and hygiene advice. Check the label of homemade or speciality products for raw eggs or unpasteurised milk.",
    "Keep chilled salads refrigerated, follow the use-by date, and avoid food that has been left out or whose storage you are unsure of. Check the ingredients for pregnancy-specific restrictions.",
    "Cook sprouts thoroughly unless they are labelled ready to eat, and follow the pack instructions. Washing alone may leave harmful bacteria behind.",
    "Follow storage and use-by instructions for olives or pickles after opening. Ask your maternity team if they have advised you to limit salt.",
    "Dates can be part of a varied diet in pregnancy. Follow your maternity team's advice on food if you have gestational diabetes.",
];

pub(super) fn preg_food(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words = &reading.line.words;
    let joined = words.join(" ");
    let lead = super::PREGNANCY_FOOD_GROUPS
        .iter()
        .zip(FOOD_GUIDANCE)
        .find_map(|(aliases, &reply)| {
            aliases
                .iter()
                .any(|name| {
                    if name.contains(' ') {
                        joined.contains(name)
                    } else {
                        words.iter().any(|word| {
                            word == name || name.strip_suffix('s').is_some_and(|base| word == base)
                        })
                    }
                })
                .then_some(reply)
        });
    Some(Cow::Borrowed(lead.unwrap_or("Whether a food is suitable in pregnancy depends on its ingredients and how it was prepared and stored. Check the NHS foods-to-avoid list for the specific food. Wash fruit and vegetables well, cook meat and shellfish thoroughly, and follow use-by dates.")))
}

const BIRTH_REPLIES: &[&str] = &[
    "A caesarean section is an operation to deliver a baby through cuts in the abdomen and womb. It can be planned or needed during labour. The maternity team can explain the reason, anaesthetic and recovery.",
    "A doula offers practical and emotional support during pregnancy, birth or afterwards. They do not provide clinical care. Ask about the support they offer, cost and whether they can attend your birth setting.",
    "A midwife is a registered professional who provides care during pregnancy, birth and after the baby is born. They work with doctors and other clinicians if you or your baby need specialist care.",
    "An epidural is a form of pain relief in which an anaesthetist puts a small tube in your back to give medicine. Ask your maternity team how it works, when it can be given and what effects to expect.",
    "A birth plan tells your maternity team your preferences for labour, birth and the first hours afterwards. Include what matters most to you and discuss how the plan might change if you or the baby need extra care.",
    "Induction means starting labour with help from the maternity team. They can explain why it is offered, the methods available in your situation and what to expect if you choose to wait.",
    "A birth pool can help you relax and ease pain during labour. Whether you can use one during labour or give birth in it depends on your care needs and local facilities. Discuss it with your midwife.",
    "Colostrum is the first milk your breasts make. Ask your midwife about feeding or expressing and whether collecting colostrum before birth is suitable for you.",
    "A membrane sweep is an examination during which a midwife or doctor moves a finger around the cervix to help separate the membranes. It can make labour more likely to start and is optional. Ask your midwife about timing and what to expect.",
    "A ventouse is a suction cup used by a clinician to help guide a baby out during an assisted vaginal birth. The clinician can explain why it is being offered and the alternatives.",
    "An episiotomy is a cut made in the area between the vagina and anus during birth when the maternity team thinks more room is needed. It is usually repaired with stitches. Ask how to look after it and when to seek help.",
    "NICU means neonatal intensive care unit. SCBU means special care baby unit. Neonatal teams look after babies who need extra support. Ask the unit about visiting and how you can take part in your baby's care.",
    "Labour is when contractions open the cervix and help the baby be born. Contact your maternity unit if you think labour has started or are unsure. Call it immediately if your waters break, you are bleeding, the baby moves less than usual, or you think you are in labour before 37 weeks.",
    "The placenta supplies oxygen and nutrients to the baby through the umbilical cord. It is delivered after the baby. Ask your midwife about the options for managing that stage.",
    "A birth centre is a unit led by midwives. Ask which birth settings are suitable for you, what facilities are available and what happens if you need to transfer to an obstetric unit.",
    "A home birth is supported by a maternity team in your home, with transfer to hospital if extra care is needed. Discuss benefits, risks and local arrangements with your midwife.",
    "A perineal tear is a tear in the area between the vagina and anus during birth. The degree describes the tissues involved and helps determine what repair and follow-up are needed. Tell your midwife if pain increases or the wound is causing problems.",
    "Lochia is bleeding and discharge after birth. It usually gets lighter over time. Get medical help if it becomes heavier, smells unusual, or you feel feverish. Call 999 for sudden heavy bleeding with faintness or a fast heartbeat.",
    "The newborn blood spot test uses drops of blood from the baby's heel to screen for rare conditions. It is usually offered at around five days old. Your midwife can explain the local screening programme and when results are expected.",
    "A show is mucus, sometimes streaked with blood, that comes away from the cervix near the start of labour. Contact your maternity unit straight away if you have vaginal bleeding, your waters break or your baby is moving less than usual.",
    "Ask your midwife which scan is being offered. The dating scan estimates how far along the pregnancy is, and the 20-week screening scan checks for certain physical conditions. Nuchal translucency measurement can be part of the earlier combined screening test.",
    "Skin-to-skin contact means holding the baby against a bare chest. Keep the baby warm with their face visible and airway clear. Ask the midwife to show you safe positioning.",
    "Forceps are instruments a clinician can use to guide the baby out during assisted vaginal birth. A Kiwi cup is a type of ventouse. The clinician can explain which option they recommend and why.",
    "Tongue-tie means a band of tissue under the tongue limits its movement. If feeding is difficult, ask your midwife, health visitor or feeding specialist for an assessment.",
    "Gestational diabetes is high blood sugar that develops during pregnancy. Your maternity team can explain screening and whether you need changes to food, blood sugar checks or treatment.",
];

static BIRTH_NAMES: LazyLock<Vec<(Regex, usize, bool)>> = LazyLock::new(|| {
    super::BIRTH_WORD_ALIASES
        .iter()
        .map(|&(name, group, ambiguous)| {
            let pattern = format!(r"\b{}s?\b", regex::escape(name));
            (
                Regex::new(&pattern).expect("valid birth glossary pattern"),
                group,
                ambiguous,
            )
        })
        .collect()
});

pub(super) fn birth_word(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = if reading.line.plain.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.plain
    };
    if NOT_BIRTH.is_match(text) {
        return None;
    }
    let birth = BIRTH_LINE.is_match(text);
    BIRTH_NAMES.iter().find_map(|(pattern, group, ambiguous)| {
        ((!ambiguous || birth) && pattern.is_match(text))
            .then_some(Cow::Borrowed(BIRTH_REPLIES[*group]))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dated_pregnancy_routes_and_counts_across_years() {
        let today = Date::new(2026, 1, 8).unwrap();
        let question = Reading::new("My last period was 3 December");
        assert!(asks_when_due(&question));
        assert_eq!(
            lmp_date(&question.line.clean, today),
            Date::new(2025, 12, 3).ok()
        );
        assert!(
            due_date_at(&question.line.clean, today)
                .unwrap()
                .contains("9 September")
        );
        assert!(
            birth_word(&Reading::new("What is a c-section?"))
                .unwrap()
                .starts_with("A caesarean")
        );
        assert!(birth_word(&Reading::new("What is Labour Day?")).is_none());
    }
}
