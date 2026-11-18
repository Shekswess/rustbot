//! Device-specific handling for technology/phone-water from the pinned PyBot.

use crate::bot::{Memory, Reading, compose};

macro_rules! survival_conditions {
    () => {
        "Whether the phone survives depends on the model, its condition and what it was exposed to."
    };
}

// Apple documents controlled test conditions, wear, and liquid handling:
// https://support.apple.com/en-us/108039
// Apple's connector guidance advises air drying and avoiding heat and rice:
// https://support.apple.com/en-us/102643
// Google advises switching a submerged Pixel off and drying it at room temperature:
// https://support.google.com/pixelphone/answer/7533279
pub(super) const EXPOSURE: &str = concat!(
    "Switch the phone off and disconnect its cables. Wipe it with a soft cloth and leave it to dry at room temperature with airflow. Keep it off and unplugged while it dries, and avoid rice or external heat. ",
    survival_conditions!(),
    " Follow the manufacturer's instructions for your model before charging it."
);
const PROGNOSIS: &str = concat!(
    survival_conditions!(),
    " IP67 and IP68 describe controlled tests, and wear or damage can reduce water resistance. Follow the manufacturer's drying instructions and contact their repair service if the phone still malfunctions."
);
pub(super) const OUTCOMES: super::outcomes::Outcomes = &[("survives", PROGNOSIS)];

pub(crate) fn greeting(reading: &Reading<'_>, memory: &Memory, fallback: &str) -> Option<String> {
    let ask = reading.line.clean.trim_end_matches(['?', '.', '!']).trim();
    if !matches!(ask, "will it survive" | "will it be ok" | "will it be okay")
        || !compose::routes::hedge(fallback)
    {
        return None;
    }

    let last = memory.turns.last()?;
    if last.module == "answers"
        && memory.last_answer_anchor.is_some_and(|anchor| {
            anchor.turn + 1 == memory.turns.len()
                && anchor.card == Some(("technology", "phone-water"))
        })
    {
        return Some(survival_conditions!().into());
    }

    // The reference's asked_about reader can retrieve the earlier selected answer
    // after an aside. Require the same just-answered question, the actual exposure
    // and prognosis replies, and one independent factual question in between.
    let [.., exposed, answered, aside] = memory.turns.as_slice() else {
        return None;
    };
    let prior = answered
        .user
        .trim()
        .trim_end_matches(['?', '.', '!'])
        .trim();
    if [exposed, answered, aside]
        .iter()
        .any(|turn| turn.module != "answers")
        || !prior.eq_ignore_ascii_case(ask)
        || !exposed.bot.contains(EXPOSURE)
        || !answered.bot.contains(PROGNOSIS)
    {
        return None;
    }
    let other = Reading::new(&aside.user);
    if super::geography::answer(&other.clean).is_none()
        && super::travel::capital(&other.clean).is_none()
    {
        return None;
    }
    Some("For the phone we discussed earlier, recovery depends on the model, its condition and what it was exposed to. Wear or damage can reduce water resistance. Follow the manufacturer's drying instructions for that model.".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn resumes_the_selected_prognosis_after_one_independent_aside() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let first = bot.turn("i dropped my phone in the sink");
        assert_eq!(first.module, Some("answers"), "{first:?}");
        assert_eq!(first.reply, EXPOSURE);

        let question = bot.turn("will it survive");
        assert_eq!(question.module, Some("answers"), "{question:?}");
        assert_eq!(question.reply, PROGNOSIS);
        assert!(
            question.candidates.iter().any(|proposal| {
                proposal.module == "greeting" && proposal.reply == survival_conditions!()
            }),
            "{question:?}"
        );

        assert_eq!(bot.turn("what is the capital of Italy").reply, "Rome.");
        let repeated = bot.turn("Will it survive?");
        assert_eq!(repeated.module, Some("greeting"), "{repeated:?}");
        assert!(
            repeated
                .reply
                .starts_with("For the phone we discussed earlier"),
            "{repeated:?}"
        );
    }

    #[test]
    fn does_not_attach_a_new_devices_question_to_the_old_phone() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let first = bot.turn("will it survive");
        assert!(!first.reply.contains("phone"), "{first:?}");
        bot.turn("i dropped my phone in the sink");
        bot.turn("will it survive");
        bot.turn("I spilled coffee on my laptop");
        let response = bot.turn("will it survive");
        assert!(!response.reply.contains("phone"), "{response:?}");
    }
}
