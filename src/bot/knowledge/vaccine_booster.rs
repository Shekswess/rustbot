//! Booster question scope and current NHS/UKHSA vaccine guidance.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{Reading, understanding::kinds::Kind};

use super::{Context, has, word};

static APPOINTMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|tonight|",
        r"next week|next month|this week|this afternoon|this morning|booked|booking|appointments?|",
        r"rescheduled|rebooked|cancelled|canceled|yesterday|already had|already been|just had)\b"
    ))
    .expect("vaccine appointment language")
});
static JAB: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:covid|flu|jab|jabs|jabbed|jabbing|vaccin\w*|immunis\w*|immuniz\w*|",
        r"inoculation|tetanus|lockjaw|booster shot|booster jab|pregnan\w*|expecting|",
        r"whooping cough|shingles|pneumonia|pneumococcal|measles|mumps|rubella|mmr|mmrv)\b"
    ))
    .expect("vaccine terms")
});
static SAFETY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(?:safe|any (?:risks?|danger)|side effects?|dangerous|harmful|bad for (?:me|you))\b",
    )
    .expect("vaccine safety question")
});

pub(in super::super) fn asks(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    (reading.kind == Kind::Question || !APPOINTMENT.is_match(clean))
        && (!word(clean, &["flu"]) || has(clean, &["pregnan", "expecting", "booster", "jabbed"]))
}

// https://www.nhs.uk/vaccinations/
// https://www.nhs.uk/pregnancy/keeping-well/vaccinations/
// https://www.gov.uk/government/publications/tetanus-advice-for-health-professionals/guidance-on-the-management-of-suspected-tetanus-cases-and-the-assessment-and-management-of-tetanus-prone-wounds
pub(super) fn booster(context: &Context<'_>) -> Option<&'static str> {
    let clean = &context.reading.line.clean;
    if !JAB.is_match(clean)
        && context
            .earlier
            .last()
            .is_some_and(|turn| !JAB.is_match(&turn.user.to_lowercase()))
    {
        return None;
    }
    if word(clean, &["tetanus", "lockjaw"]) {
        return Some(
            "If you have a dirty or deep wound, bite, puncture or burn, get it assessed promptly. A clinician will check the wound and your vaccine record to see whether you need tetanus vaccination or other treatment. Get emergency help for heavy bleeding that will not stop. For routine protection, ask your GP practice to check whether you have completed the schedule.",
        );
    }
    if has(clean, &["pregnan", "expecting", "due in", "weeks gone"]) {
        return Some(
            "Ask your midwife or GP which vaccines are due and when. The NHS recommends flu, whooping cough and RSV vaccination in pregnancy at the appropriate times. Check current COVID-19 eligibility separately. If you are asking about measles exposure or a live vaccine during pregnancy, contact your maternity team promptly.",
        );
    }
    if SAFETY.is_match(clean) {
        return Some(
            "Temporary soreness and tiredness can follow some vaccines. Tell your vaccinator about previous serious allergic reactions, pregnancy or a weakened immune system so they can check the exact vaccine. Sudden trouble breathing or swelling of the face, tongue or throat after a vaccine needs emergency help.",
        );
    }
    if word(clean, &["measles", "mumps", "rubella", "mmr", "mmrv"]) {
        return Some(
            "Ask your GP practice to check whether you have two recorded doses of a measles-containing vaccine and whether you need a catch-up dose. The vaccine offered and the timing depend on age and the current NHS schedule. If you think you were exposed to measles or have symptoms, call for advice before visiting a clinic.",
        );
    }
    if word(clean, &["shingles"]) {
        return Some(
            "NHS shingles vaccine eligibility depends on age, risk and the programme in your nation. Ask your GP practice to check the current schedule, including whether you qualify through a weakened immune system and how many doses you need. Tell them if you have had shingles before.",
        );
    }
    if word(clean, &["pneumonia", "pneumococcal"]) {
        return Some(
            "Pneumococcal vaccine eligibility and repeat doses depend on age, medical conditions and previous vaccinations. Ask your GP practice to check your record and the current schedule. People with some medical conditions may need additional doses.",
        );
    }
    Some(
        "COVID-19 and flu vaccination programmes have different eligibility rules, and they can change by season and UK nation. Check the current NHS guidance where you live or ask your GP practice or pharmacy to look up which vaccine you are due.",
    )
}
