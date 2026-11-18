//! Family cards that use a child's age or the preceding conversation.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(GRUNT, super::FAMILY_GRUNT);
pattern!(GRUNT_WITH_BREATH, super::FAMILY_GRUNT_BREATH);
pattern!(GRUNT_AS_ANSWER, super::FAMILY_GRUNT_ANSWER);
pattern!(AGE, super::FAMILY_AGE_IN_LINE);
pattern!(VISIT, super::FAMILY_HOME_VISIT);
pattern!(ASK_AGE, super::FAMILY_MILESTONE_ASK_AGE);
pattern!(PHONE_AGE, super::FAMILY_PHONE_ASK_AGE);
pattern!(CHILD_AGE, super::FAMILY_MILESTONE_AGE);
pattern!(GROWTH_SPURT, super::FAMILY_GROWTH_SPURT);
pattern!(WEIGHT_ANNOUNCEMENT, super::FAMILY_WEIGHT_ANNOUNCEMENT);
pattern!(WEIGHT_QUESTION, super::FAMILY_WEIGHT_QUESTION);
pattern!(WEIGHT_CONCERNS, super::FAMILY_WEIGHT_CONCERNS);
pattern!(WEIGHT_NOT_GAINING, super::FAMILY_WEIGHT_NOT_GAINING);
pattern!(SPEECH, super::FAMILY_TALK_WORD);
pattern!(SPEECH_NOUN, super::FAMILY_TALK_NOUN);
pattern!(AIMED_AT_PERSON, super::FAMILY_TALK_AIM);
pattern!(TALK_MONTHS, super::FAMILY_TALK_MONTHS);
pattern!(TALK_AGE_WORD, super::FAMILY_TALK_AGE_WORD);
pattern!(TALK_HALF, super::FAMILY_TALK_HALF);
pattern!(TALK_YEARS, super::FAMILY_TALK_YEARS);
pattern!(TALK_CHILD, super::FAMILY_TALK_CHILD);
pattern!(ASK_HOW_TALK, super::FAMILY_TALK_HOW);
pattern!(SILENT_WITH_PERSON, super::FAMILY_SILENCE);
pattern!(WORD, r"[a-z']+");
pattern!(ADULT_TATTOO, r"\b(?:18|18th)\b");
pattern!(
    SCALES,
    r"\bscales?\b|\b(?:what|which|how do i) (?:should|do|can) i weigh\b|\bweigh (?:her|him|them|the baby|my baby) on\b"
);

fn has(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

fn token_in(text: &str, terms: &[&str]) -> bool {
    WORD.find_iter(text)
        .any(|hit| terms.contains(&hit.as_str()))
}

fn age_word(word: &str) -> Option<u32> {
    [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve",
    ]
    .iter()
    .position(|name| *name == word)
    .map(|age| age as u32)
}

fn age_in_line(text: &str) -> Option<u32> {
    let m = AGE.captures(text)?;
    m.get(1)
        .or_else(|| m.get(2))
        .and_then(|m| m.as_str().parse().ok())
        .or_else(|| m.get(3).and_then(|m| age_word(m.as_str())))
}

fn child_named(text: &str) -> bool {
    CHILD_AGE.is_match(text) || token_in(text, super::FAMILY_MILESTONE_CHILD_WORDS)
}

pub(super) fn guard(id: &str, reading: &Reading<'_>, earlier: &[Exchange]) -> bool {
    let current = &reading.line.clean;
    let previous = earlier
        .last()
        .map_or_else(String::new, |turn| turn.user.to_lowercase());
    match id {
        "child-breathing" => {
            !GRUNT.is_match(current)
                || GRUNT_WITH_BREATH.is_match(current)
                || (!GRUNT_AS_ANSWER.is_match(current)
                    && (has(current, super::FAMILY_GRUNT_YOUNG)
                        || age_in_line(current)
                            .or_else(|| age_in_line(&previous).filter(|_| child_named(&previous)))
                            .is_none_or(|age| age <= 9)))
        }
        "what-to-bring-when-visiting-a-care-home" => {
            (current.is_empty() && previous.is_empty())
                || VISIT.is_match(current)
                || VISIT.is_match(&previous)
        }
        "child-wont-talk" => super::family_repair::not_their_deed(reading),
        "late-talker" => {
            super::family_repair::not_their_deed(reading)
                && SPEECH.is_match(current)
                && (SPEECH_NOUN.is_match(current)
                    || talk_age(current, true).is_some()
                    || !AIMED_AT_PERSON.is_match(current))
        }
        _ => false,
    }
}

fn talk_age(text: &str, strict: bool) -> Option<f64> {
    if strict && !TALK_CHILD.is_match(text) {
        return None;
    }
    if let Some(months) = TALK_MONTHS.captures(text) {
        return months[1].parse::<f64>().ok().map(|months| months / 12.0);
    }
    if let Some(years) = TALK_HALF.captures(text) {
        return age_word(&years[1]).map(|age| f64::from(age) + 0.5);
    }
    if let Some(years) = TALK_AGE_WORD.captures(text) {
        return age_word(&years[1]).map(f64::from);
    }
    let years = TALK_YEARS.captures(text)?;
    let age = years
        .iter()
        .skip(1)
        .flatten()
        .next()?
        .as_str()
        .parse::<f64>()
        .ok()?;
    Some(if text.contains("half") {
        age + 0.5
    } else {
        age
    })
}

fn milestone(text: &str, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    if !child_named(text)
        && !ASK_AGE.is_match(text)
        && !earlier
            .iter()
            .rev()
            .take(4)
            .any(|turn| child_named(&turn.user.to_lowercase()))
    {
        return None;
    }
    let lead = super::FAMILY_MILESTONE_SKILLS
        .iter()
        .find_map(|(clues, line)| has(text, clues).then_some(*line));
    let advice = "Children develop at different rates. If you are concerned about a skill, or your child loses skills they previously had, speak to your health visitor or GP. Bring your child's red book if you have it.";
    Some(Cow::Owned(match lead {
        Some(lead) => format!("{lead}. {advice}"),
        None => format!(
            "Ask your health visitor about the next developmental review and which skills to look for at your child's age. {advice}"
        ),
    }))
}

const BIRTH_WEIGHT: &str = "Birth weight varies with factors including how early a baby was born. Ask the midwife or health visitor to plot your baby's measurements on the chart in their red book and explain the trend. Get advice promptly if your baby is feeding poorly or you are concerned about their weight.";
const QUIET_CHILD: &str = "Give them a chance to talk while you are doing something together. Say that you have noticed they are quieter and that they can tell you when they are ready. If they have stopped speaking or you are worried about their wellbeing, talk to their GP or school for support.";

fn child_weight(text: &str) -> Option<Cow<'static, str>> {
    let reply = if GROWTH_SPURT.is_match(text) {
        return None;
    } else if WEIGHT_ANNOUNCEMENT.is_match(text)
        && !WEIGHT_CONCERNS.is_match(text)
        && !has(
            text,
            &[
                "too thin",
                "too skinny",
                "off their food",
                "not eating",
                "wont eat",
                "small for",
                "skinny",
            ],
        )
    {
        if !WEIGHT_QUESTION.is_match(text) {
            return None;
        }
        BIRTH_WEIGHT
    } else if SCALES.is_match(text) {
        "Ask your health visitor where to get your baby weighed and how often to check. Use properly calibrated baby scales and record the measurement and date in their red book."
    } else if has(
        text,
        &[
            "eating disorder",
            "anorexia",
            "bulimia",
            "skipping meals",
            "making herself sick",
            "making himself sick",
            "body image",
            "is fat",
            "too fat",
            "feels fat",
            "looks fat",
            "about her weight",
            "about his weight",
        ],
    ) {
        "Thank them for telling you how they feel and keep the conversation supportive. Speak to their GP promptly if they are losing weight, skipping meals, making themselves sick or restricting food. A GP can assess them and refer them for specialist help if needed."
    } else if WEIGHT_NOT_GAINING.is_match(text)
        || (token_in(
            text,
            &["baby", "babies", "newborn", "newborns", "infant", "infants"],
        ) && has(
            text,
            &[
                "putting on weight",
                "put on weight",
                "gaining weight",
                "gained weight",
                "is gaining",
                "has gained",
            ],
        ))
    {
        "Ask your health visitor or GP to check feeding and plot the weight on your baby's growth chart. Seek help promptly if your baby is feeding poorly, has fewer wet nappies than usual or is unusually sleepy."
    } else {
        "A child's healthy weight is assessed in relation to their age, height and growth over time. Ask their GP, health visitor or school nurse to review the growth chart. Try shared changes to meals and activity if they recommend them, and avoid putting the child on a restrictive diet on your own."
    };
    Some(Cow::Borrowed(reply))
}

const PIERCING_RULE: &str = "Check the rules with your local council and the studio. Age and consent requirements depend on where you live and the type of piercing, and studios may have their own limits. Use a registered or licensed studio where required and ask about sterilisation and written aftercare.";
const TATTOO_RULE: &str = "In Great Britain, decorative tattooing is for people aged 18 and over, including when a parent would consent. Ask a registered or licensed studio where required about hygiene and aftercare.";

pub(super) fn reply(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let text = reading.line.clean.as_str();
    let previous = earlier
        .last()
        .map_or_else(String::new, |turn| turn.user.to_lowercase());
    let age =
        || age_in_line(text).or_else(|| age_in_line(&previous).filter(|_| child_named(&previous)));
    let reply = match id {
        "child-breathing" => {
            "Call 999 if your child is grunting as they breathe, sucking in under the ribs, has pauses in breathing, turns blue or grey, or is hard to wake. If their breathing is faster than usual or there is a new wheeze and none of those signs are present, contact NHS 111 or your GP urgently."
        }
        "baby-milestones" => return milestone(text, earlier),
        "child-weight" => return child_weight(text),
        "child-dummy" => match age() {
            None | Some(0..=1) => {
                "If you use a dummy, offer it only at sleep times and aim to stop by 12 months. Keep it away from sugar and honey, and never tie it around a baby's neck. Ask your health visitor if you need help phasing it out."
            }
            Some(2..=4) => {
                "Choose a quiet time to stop the dummy. Start with a clear rule, such as only at bedtime, and agree a date to stop altogether. Let your child help with the plan and use another comforting bedtime routine."
            }
            Some(_) => {
                "Talk with your child about phasing out the dummy and find another way for them to settle. Ask a dentist or speech and language therapist if you are concerned about their teeth or speech."
            }
        },
        "relative-had-a-stroke"
            if token_in(text, super::FAMILY_STROKE_EATING)
                || reading
                    .line
                    .words
                    .iter()
                    .any(|word| super::FAMILY_STROKE_EATING.contains(&word.as_str())) =>
        {
            "Ask the stroke team what food and drink are safe after their swallowing assessment. Follow the texture and fluid instructions they gave. Tell the team promptly about coughing, choking or a wet-sounding voice after eating or drinking."
        }
        "relative-had-a-stroke" => {
            "Ask the stroke unit about the scan, swallowing assessment and rehabilitation plan. The team can explain what to expect and how you can help. Recovery varies. Call 999 if they have new stroke signs such as facial droop, arm weakness or difficulty speaking."
        }
        "child-wont-talk" => {
            let both = format!("{text} {previous}");
            if token_in(&both, super::FAMILY_QUIET_CHILD) {
                QUIET_CHILD
            } else if token_in(&both, super::FAMILY_QUIET_FRIEND)
                && !token_in(&both, super::FAMILY_QUIET_HOUSEHOLD)
                && has(&both, super::FAMILY_QUIET_DRIFT)
                && !has(&both, super::FAMILY_QUIET_WORRY)
            {
                let reason = if has(&both, super::FAMILY_QUIET_MOVED) {
                    super::FAMILY_QUIET_MOVED_REASON
                } else {
                    super::FAMILY_QUIET_REASON
                };
                return Some(Cow::Owned(
                    super::FAMILY_QUIET_FRIEND_REPLY.replace("{reason}", reason),
                ));
            } else if token_in(&both, super::FAMILY_QUIET_GROWN) {
                "Let them know you are around and offer something specific, like a walk or a cup of tea. Ask gently how they are and give them time to answer. If you are worried about their safety or they are withdrawing for a long time, offer to help them contact a GP or get urgent help."
            } else {
                QUIET_CHILD
            }
        }
        "late-talker" => {
            let age = talk_age(text, false).or_else(|| {
                earlier
                    .iter()
                    .rev()
                    .take(4)
                    .find_map(|turn| talk_age(&turn.user.to_lowercase(), true))
            });
            if age.is_some_and(|age| age >= 10.0)
                && (SILENT_WITH_PERSON.is_match(text) || SILENT_WITH_PERSON.is_match(&previous))
            {
                return None;
            }
            let lead = match age {
                Some(age) if age < 2.0 => {
                    "Ask your health visitor if your baby is not babbling, responding to sounds, or beginning to communicate as expected."
                }
                Some(age) if age < 3.0 => {
                    "Ask your health visitor about a hearing check and speech and language support if your two-year-old is using few words."
                }
                Some(age) if age < 4.0 => {
                    "Talk to your health visitor or nursery about your three-year-old's speech, and ask about a hearing check and speech and language assessment."
                }
                Some(_) => {
                    "Speak to your child's school and GP about a hearing check and an assessment by a speech and language therapist."
                }
                None => {
                    "Ask the health visitor or GP about a hearing check and speech and language assessment if you are worried about their speech or they have lost words they used before."
                }
            };
            if ASK_HOW_TALK.is_match(text) {
                return Some(Cow::Owned(format!(
                    "Talk and play together, follow their interests, pause to give them time to respond, and share books or songs. {lead}"
                )));
            }
            lead
        }
        "a-child-and-a-phone" if PHONE_AGE.is_match(text) => {
            "Choose a first-phone age based on what your child needs it for and how ready they are to use it safely. Agree the rules together, set suitable privacy and downtime controls, and keep phones out of bedrooms overnight if that helps with sleep."
        }
        "a-child-and-a-phone" => {
            "Agree phone rules together before the next argument. Decide on phone-free times that work for your household, where the phone charges at night and what happens when a limit is reached. Check that privacy and downtime settings match the rules."
        }
        "a-teenager-wants-a-tattoo"
            if age().is_some_and(|age| age >= 18)
                || (age().is_none() && ADULT_TATTOO.is_match(text)) =>
        {
            "At age 18 they can decide whether to get a tattoo. Take time to choose a design and ask the studio about its local registration or licensing, sterile equipment and written aftercare."
        }
        "a-teenager-wants-a-tattoo" => {
            return Some(Cow::Owned(format!(
                "{TATTOO_RULE} If they are still deciding on a design, let them draw it and revisit it later."
            )));
        }
        "a-teenager-wants-a-piercing" => {
            return Some(Cow::Owned(match age() {
                Some(age) => format!(
                    "Tell the studio the person is {age} and which piercing they want. {PIERCING_RULE}"
                ),
                None => PIERCING_RULE.to_owned(),
            }));
        }
        _ => return None,
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_followups_keep_the_breathing_and_speech_routes_separate() {
        let prior = [Exchange {
            user: "My daughter is 14 and won't talk to me".into(),
            bot: String::new(),
            module: "answers",
        }];
        let breathing = Reading::new("My son is grunting when he breathes");
        assert!(guard("child-breathing", &breathing, &[]));
        assert!(
            reply("child-breathing", &breathing, &[])
                .unwrap()
                .starts_with("Call 999")
        );
        assert!(!guard(
            "child-breathing",
            &Reading::new("She just grunts"),
            &prior
        ));
        let speech = Reading::new("My toddler is 18 months and is not saying words");
        assert!(guard("late-talker", &speech, &[]));
        assert!(
            reply("late-talker", &speech, &[])
                .unwrap()
                .contains("health visitor")
        );
        let milestone = Reading::new("When should my baby crawl?");
        assert!(
            reply("baby-milestones", &milestone, &[])
                .unwrap()
                .starts_with("Crawling")
        );
        let scales = Reading::new("Which scales should I use to weigh my baby?");
        assert!(
            reply("child-weight", &scales, &[])
                .unwrap()
                .contains("calibrated baby scales")
        );
    }
}
