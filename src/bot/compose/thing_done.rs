//! Respond to the small action a person just said they finished or are doing.

use crate::bot::{Memory, Reading};
use crate::text::is_space;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        pub(in crate::bot::compose) static $name: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($value).expect(stringify!($name)));
    };
}

#[path = "thing_done_data.rs"]
mod data;
use data::*;
pub(super) use data::{HARM_ANYWHERE, HARM_NAMED, NEGATIVE_LINE, QUESTION_OPENERS};

enum Action<'a> {
    Finished(&'a str),
    Going,
    AtIt,
    Ahead,
}

fn vocabulary(set: &[&str], word: &str) -> bool {
    set.binary_search(&word).is_ok()
}

fn words(line: &str) -> impl Iterator<Item = &str> + '_ {
    static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
    WORD.find_iter(line).map(|found| found.as_str())
}

fn spaced(line: &str) -> impl Iterator<Item = &str> + '_ {
    line.split(is_space).filter(|piece| !piece.is_empty())
}

pub(super) fn regret(line: &str) -> bool {
    let line = spaced(line).collect::<Vec<_>>().join(" ");
    if line.is_empty()
        || spaced(&line).count() > 20
        || !A_REGRET_DOING.is_match(&line)
        || NOT_A_REGRET.is_match(&line)
        || SNAPPED_A_PHOTO.is_match(&line)
    {
        return false;
    }
    if A_MISTAKE.is_match(&line)
        && THE_JOB_NAMED
            .captures(&line)
            .is_some_and(|found| !vocabulary(NOT_A_JOB_BUT_A_PERSON, &found[1]))
    {
        return false;
    }
    !HARM_NAMED.is_match(&line) && !HARM_ON_ME.is_match(&line)
}

fn classify<'a>(line: &'a str, clean: &str) -> Option<Action<'a>> {
    let terms: Vec<_> = words(line).collect();
    for (start, _) in terms
        .iter()
        .enumerate()
        .filter(|(_, w)| matches!(**w, "i" | "we"))
    {
        let mut rest = terms[start + 1..].iter().copied().skip_while(|word| {
            matches!(
                *word,
                "just" | "also" | "finally" | "only" | "actually" | "have" | "had" | "been"
            )
        });
        let Some(verb) = rest.next().filter(|word| vocabulary(DONE_VERB, word)) else {
            continue;
        };
        if rest
            .clone()
            .take(2)
            .any(|word| vocabulary(DONE_NOT_THIS, word))
            || vocabulary(DONE_ONLY_A_THING, verb)
                && rest.take(3).any(|word| vocabulary(DONE_NOT_A_PERSON, word))
        {
            return None;
        }
        return Some(Action::Finished(verb));
    }
    if let Some((&first, rest)) = terms.split_first()
        && vocabulary(DONE_VERB, first)
        && !HARM_ANYWHERE.is_match(clean)
        && !rest
            .iter()
            .take(2)
            .any(|word| vocabulary(DONE_NOT_THIS, word))
    {
        return Some(Action::Finished(first));
    }
    if WENT_AT_IT.is_match(line) {
        return Some(Action::Finished("went"));
    }
    if DID_AT_IT.is_match(line) {
        return Some(Action::Finished("did"));
    }
    if SPENT_TIME
        .captures(line)
        .is_some_and(|found| vocabulary(DONE_ING, &found[1]))
        || BEEN_AT_IT
            .captures(line)
            .is_some_and(|found| vocabulary(DONE_ING, &found[1]))
            && TIME_SPAN.is_match(line)
    {
        return Some(Action::Going);
    }
    let ongoing = AT_IT.captures(line)?;
    let verb = &ongoing[1];
    let rest = &line[ongoing.get(0)?.end()..];
    if !(vocabulary(DONE_ING, verb) || vocabulary(AT_IT_EXTRA, verb))
        || vocabulary(BECOMING, verb)
            && spaced(rest)
                .next()
                .is_some_and(|word| vocabulary(BECOMING_STATE, word))
        || !words(rest).any(|word| word.len() > 2 && !vocabulary(THING_AFTER, word))
        || HABIT_TIME.is_match(rest)
        || words(rest).any(|word| vocabulary(AT_IT_NOT, word))
    {
        return None;
    }
    Some(if FUTURE_TIME.is_match(rest) {
        Action::Ahead
    } else {
        Action::AtIt
    })
}

fn pool(clean: &str) -> Option<&'static [&'static str]> {
    if clean.contains('?')
        || clean.contains(" but ")
        || spaced(clean)
            .next()
            .is_some_and(|word| vocabulary(QUESTION_OPENERS, word))
        || [
            &*TAIL_ASK,
            &*TAIL_GRUMBLE,
            &*TAIL_TROUBLE,
            &*NEGATIVE_LINE,
            &*HARM_AFTER,
            &*HARM_NAMED,
            &*HARM_ON_ME,
            &*FEELING_OFF,
            &*A_TROUBLE_IN_THE_TAIL,
        ]
        .iter()
        .any(|pat| pat.is_match(clean))
        || TAIL_JOIN
            .find(clean)
            .is_some_and(|found| TAIL_COMPLAINT.is_match(&clean[found.end()..]))
        || regret(clean)
    {
        return None;
    }
    static I_HAVE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(?:ive|i've)\b").unwrap());
    static WE_HAVE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(?:weve|we've)\b").unwrap());
    static HAD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(i|we)'d\b").unwrap());
    let typed = I_HAVE.replace_all(clean, "i have");
    let typed = WE_HAVE.replace_all(&typed, "we have");
    let typed = HAD.replace_all(&typed, "${1} had");
    let action = classify(&typed, clean)?;
    let before: Vec<_> = words(clean).collect();
    if super::trouble::reads_as_trouble(clean, &before, clean).is_some() {
        return None;
    }
    match action {
        Action::Going => Some(A_THING_ON),
        Action::AtIt | Action::Ahead => Some(A_THING_AHEAD),
        Action::Finished(verb) if vocabulary(GONE_VERB, verb) => Some(A_THING_GONE),
        Action::Finished(verb) if vocabulary(BOUGHT_VERB, verb) => {
            if spaced(clean).any(|word| vocabulary(ALIVE, word)) {
                None
            } else if spaced(clean).any(|word| vocabulary(NOT_CHOSEN, word)) {
                Some(A_THING_SEEN_TO)
            } else {
                Some(A_THING_BOUGHT)
            }
        }
        Action::Finished(verb) if vocabulary(SEEN_TO_VERB, verb) => Some(A_THING_SEEN_TO),
        Action::Finished(_) => Some(A_THING_DONE),
    }
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = &reading.line.clean;
    let pool = pool(line)?;
    super::trouble::neutral_tone(reading).then(|| super::choose_recent(pool, memory, line).into())
}

const PLAN_REPLIES: &[&str] = &[
    "Good. What's the first step you're unsure about?",
    "Nice — worth doing properly. What would you want sorted out first?",
    "Worth deciding what the first step is, and what you'd count as done.",
    "Sounds like a good plan. What's the bit you're least sure about?",
];
const PLAN_WITH_LIMIT: &[&str] = &[PLAN_REPLIES[0], PLAN_REPLIES[3]];

pub(super) fn plan_remark(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = &reading.line.clean;
    // A plan names work to be done over time. A broad wish such as "get fitter"
    // and a search for a place do not call for an unsolicited planning question.
    static PROJECT: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(concat!(
            r"\b(?:planning(?: to)?|plan to|book a|sav(?:e|ing) up|saving for|applying|apply for)\b",
            r"|\b(?:going to|gonna|want(?:ing)? to|would like to|hoping to|thinking (?:of|about)) ",
            r"(?:start|move|save|buy|learn|book|travel|change|quit|sell|apply|build|train|study|",
            r"open|join|switch|retrain|volunteer|emigrate|adopt|redecorate|renovate|downsize|",
            r"sort|fix|paint|clear|declutter|budget|insulate|convert|rent|plant|grow|write|",
            r"work on|(?:going|getting) back)\b",
        )).unwrap()
    });
    if reading.raw.contains('?') || !PROJECT.is_match(clean) {
        return None;
    }
    static UNSURE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(?:not sure|unsure|don'?t know|whether|if i|should i|no idea|in two minds|don'?t want|do not want|leav(?:e|ing) (?:my (?:husband|wife|partner)|him|her)|never do|keep planning|today|tomorrow|yesterday|tonight)\b").unwrap()
    });
    static PROTECTED: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(concat!(
            r"\b(?:visit|visits|visiting|anniversary|birthdays?|remembrance|memorial|homework|",
            r"died|dying|dead|death|passed away|funeral|buried|miscarriage|cancer|tumou?r|",
            r"diagnosis|hospital|hospice|suicid\w*|overdose|abuse|assaulted|emergency|ambulance|",
            r"bleeding|chest|stroke|seizure|unconscious|grieving|custody|divorce|",
            r"abortion|termination|crematorium|stillbirth|ectopic|coffin|appointment|consultant|",
            r"surgery|gp|doctor|midwife|chemo|biopsy|operation|ward|petrol|gas|fumes|smoke|",
            r"burning|monoxide|sparks|wiring|fire|breath\w*|wheez\w*|cough\w*|dizzy|faint|",
            r"palpitations|pain|hurts?|aching|sore)\b",
        ))
        .unwrap()
    });
    if UNSURE.is_match(clean)
        || PROTECTED.is_match(clean)
        || words(clean).any(|word| {
            matches!(
                word,
                "how"
                    | "what"
                    | "when"
                    | "where"
                    | "which"
                    | "why"
                    | "who"
                    | "advice"
                    | "tips"
                    | "suggest"
                    | "recommend"
            )
        })
    {
        return None;
    }
    static LIMIT: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(?:but|except|only|never|don'?t have|haven'?t got|can'?t|no money|no time|too late|already tried)\b").unwrap()
    });
    let pool = if LIMIT.is_match(clean) {
        PLAN_WITH_LIMIT
    } else if memory
        .turns
        .last()
        .is_some_and(|turn| turn.bot.trim_end().ends_with('?'))
    {
        &PLAN_REPLIES[2..3]
    } else {
        PLAN_REPLIES
    };
    Some(super::pick(pool, memory, clean).to_owned())
}

pub(super) fn completed_plan(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    static COMPLETED: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(?:(?:i|we) )(?:(?:have|just|finally) )*(?:finished|completed) (?:the |my |our )?([a-z]+)(?: [a-z'-]+){0,6}[.!]*$").unwrap()
    });
    static UNSETTLED: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(?:but|wrong|broken|hurt|pain|sick|injured|again|failed|can't|cannot)\b")
            .unwrap()
    });
    let line = &reading.line.clean;
    let action = COMPLETED.captures(line)?.get(1)?.as_str();
    let plan = memory.stated_plans.latest()?;
    let planned_action = plan.split_whitespace().next()?;
    let forms = crate::bot::understanding::normalize::forms(action);
    let completed = crate::bot::understanding::normalize::forms(planned_action);
    if UNSETTLED.is_match(line) || !forms.iter().any(|form| completed.contains(form)) {
        return None;
    }
    const POSITIVE: &[&str] = &[
        "That's great news — nice one. What's next?",
        "Lovely to hear. Enjoy it!",
        "Good for you! That's really lovely news.",
        "Brilliant. Thanks for telling me.",
    ];
    Some(super::choose_recent(POSITIVE, memory, line).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_remark_reads_projects_with_a_first_step() {
        let memory = Memory::default();
        assert!(
            plan_remark(
                &Reading::new("I was going to paint the spare room but I haven't done it yet."),
                &memory
            )
            .is_some()
        );
        for line in [
            "I want to get fitter.",
            "I'm looking for a launderette.",
            "I want to start running but I get out of breath.",
        ] {
            assert!(
                plan_remark(&Reading::new(line), &memory).is_none(),
                "{line}"
            );
        }
    }

    #[test]
    fn chooses_the_question_that_fits_the_action() {
        let memory = Memory::default();
        for (line, wanted) in [
            (
                "im shopping for some laptops next month",
                "Good luck with it - what are you doing first?",
            ),
            (
                "weve been sorting the hall all morning",
                "Good on you. How much longer have you got?",
            ),
            ("i bought a prescription", "Good - was it straightforward?"),
            ("i went for a run", "Good on you. How long did that take?"),
        ] {
            assert_eq!(
                reply(&Reading::new(line), &memory).as_deref(),
                Some(wanted),
                "{line}"
            );
        }
    }

    #[test]
    fn passes_on_habits_regrets_and_complaints() {
        let memory = Memory::default();
        for line in [
            "im doing a yoga class every week",
            "i made a daft mistake at work",
            "im painting the hall and the fumes are making me dizzy",
            "sorted the boiler and now the radiators are cold",
        ] {
            assert_eq!(reply(&Reading::new(line), &memory), None, "{line}");
        }
    }
}
