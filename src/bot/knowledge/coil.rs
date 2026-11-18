//! NHS-aligned guidance for questions about copper and hormonal contraceptive coils.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

#[path = "coil_data.rs"]
mod data;

static PREGNANCY_CONCERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:i(?: think| might| may| could) (?:i )?(?:be|am) pregnant|positive (?:pregnancy )?test|(?:my|a) (?:pregnancy )?test (?:is|was|came back) positive|missed (?:my|a) period|late period)\b")
        .expect("a current pregnancy concern")
});
static WARNING_SIGNS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:fever|high temperature|smelly discharge|unusual discharge|very heavy bleeding|(?:severe|worsening|sudden) (?:pain|cramps?)|pain (?:is |gets )?worse)\b")
        .expect("coil warning signs")
});

const URGENT: &str = "If you have a coil and develop severe or worsening lower abdominal pain, fever, unusual or smelly discharge, or very heavy bleeding, get urgent advice from a sexual health clinic, your GP or NHS 111.";
const PREGNANCY: &str = "Pregnancy with a coil is uncommon. If you think you might be pregnant, get urgent advice from your GP, sexual health clinic or NHS 111. A clinician should check for an ectopic pregnancy. Leave the coil in place until they advise you. For sudden severe abdominal pain with dizziness or fainting, call 999 or go to A&E.";
const LOST: &str = "If the threads feel different, you can't feel them, or the coil has come out, contact a sexual health clinic, your GP or NHS 111 straight away. Use another method, such as condoms, until a trained clinician checks it. If you've had sex without another method, ask promptly whether you need emergency contraception. The hormonal coil can affect which emergency option is suitable.";
const SAFE: &str = "A doctor or nurse can check which coil is suitable for you. Rare risks include infection after fitting, the coil moving or coming out, and injury to the womb. The copper coil can make periods heavier or more painful. A hormonal coil can cause spotting at first and often makes periods lighter over time. Neither type protects against sexually transmitted infections.";
const EFFECT: &str = "Both types of coil are over 99% effective at preventing pregnancy. A copper coil usually lasts 5 to 10 years and a hormonal coil 3 to 8 years, depending on the device. Ask the clinician when yours needs replacing. If you think it has moved, get it checked and use another method until then.";
const ONSET: &str = "A copper coil protects against pregnancy straight away. A hormonal coil protects straight away when fitted during the first five days of your cycle, counting the first day of your period as day one. Otherwise, use another method, such as condoms, for seven days after fitting unless your clinician gives you different advice for your circumstances.";
const REMOVAL: &str = "A trained nurse or doctor can remove a coil at any time. Book with a GP or sexual health clinic. You can get pregnant immediately after removal. If you want to avoid pregnancy, use another method, such as condoms, from seven days before removal, and ask the clinic how to arrange a switch.";
const UNDER_16: &str = "NHS sexual health services offer free, confidential contraception advice, including for people under 16. A clinician will check you understand the decision and which method suits you. They may need to share information if there's a serious risk to your safety or welfare, and would usually discuss that with you first.";
const AFTER_BIRTH: &str = "A copper or hormonal coil can sometimes be fitted within 48 hours of birth, including if you're breastfeeding. If it isn't fitted in that window, the NHS usually advises waiting until four weeks after birth. Ask your maternity or sexual health team which contraception to use while you wait.";
const PAIN: &str = "Many people feel pain during fitting, and the amount varies. Tell the nurse or doctor about any concerns beforehand. Ask what pain relief they can offer, including local anaesthetic, and tell them if you want them to pause or stop.";
const PERIODS: &str = "The copper coil can make periods heavier, longer or more painful, especially at first. A hormonal coil may cause irregular bleeding for the first few months and can later make periods lighter, shorter or stop them. Get very heavy bleeding assessed urgently and discuss persistent or unexpected bleeding with a clinician.";
const DURATION: &str = "A copper coil generally lasts 5 to 10 years and a hormonal coil 3 to 8 years, depending on which type you have. Ask the clinic when your specific device needs replacing. You can ask a trained nurse or doctor to remove it earlier.";

fn has(text: &str, cues: &[&str]) -> bool {
    cues.iter().any(|cue| text.contains(cue))
}

fn answer(text: &str) -> Cow<'static, str> {
    if PREGNANCY_CONCERN.is_match(text) {
        return Cow::Borrowed(PREGNANCY);
    }
    if has(text, data::COIL_LOST) && !has(text, data::COIL_REMOVAL_ASK) {
        return Cow::Borrowed(LOST);
    }
    if WARNING_SIGNS.is_match(text) {
        return Cow::Borrowed(URGENT);
    }
    if has(text, data::COIL_SAFE) {
        return Cow::Owned(format!("{SAFE} {URGENT}"));
    }
    if has(text, data::COIL_EFFECT) {
        return Cow::Borrowed(EFFECT);
    }
    for (cues, reply) in [
        (data::COIL_PREGNANT, PREGNANCY),
        (data::COIL_SOON, ONSET),
        (data::COIL_OUT, REMOVAL),
        (data::COIL_YOUNG, UNDER_16),
        (data::COIL_BABY, AFTER_BIRTH),
    ] {
        if has(text, cues) {
            return Cow::Borrowed(reply);
        }
    }
    if has(text, data::COIL_AFTER) {
        return Cow::Owned(format!(
            "Period-like cramps and light spotting can happen for a few days after fitting. {URGENT} If you can't feel the threads or they feel different, contact the clinic, GP or 111 straight away and use another method until checked."
        ));
    }
    for (cues, reply) in [
        (data::COIL_HURT, PAIN),
        (
            data::COIL_FREE,
            "Contraception from NHS services is free, including coil fitting at a sexual health clinic or a GP practice that provides it. Call to check availability and ask about the appointment.",
        ),
        (data::COIL_BLEED, PERIODS),
        (data::COIL_LIFE, DURATION),
    ] {
        if has(text, cues) {
            return Cow::Borrowed(reply);
        }
    }
    Cow::Borrowed(
        "Book with a sexual health clinic or a GP practice that fits coils. A trained doctor or nurse will talk you through which type suits you and may offer pregnancy or STI testing. The fitting itself usually takes 5 to 10 minutes. Allow at least 30 minutes for the appointment and ask in advance about pain relief. You can ask the clinician to pause or stop at any time.",
    )
}

pub(crate) fn reply(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = format!("{} {}", reading.raw, reading.clean).to_lowercase();
    Some(answer(&text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contraception_cues_keep_timing_and_urgent_questions_distinct() {
        assert!(answer("how soon does a hormonal coil work").contains("first five days"));
        assert!(answer("is it safe if my coil has come out").contains("straight away"));
        assert!(answer("my pregnancy test came back positive with the coil").contains("ectopic"));
    }
}
