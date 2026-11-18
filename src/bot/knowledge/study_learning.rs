//! Authored guidance for assessments, revision, and the workload of a course.

use crate::bot::{Exchange, Reading};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "study_assessments.rs"]
mod assessments;
#[path = "study_careers.rs"]
mod careers;
#[path = "study_learning_data.rs"]
mod data;
pub(crate) use assessments::{degree_mark, names_language};

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    TOP_GRADES,
    SCORE,
    GRADE_NAME,
    LETTER_GRADES,
    TEST_AS_A_VERB,
    TIME_AFTER_TEST,
    LATER_STUDY,
    CAREER_CHANGE,
    NOT_AGE_STUDY,
    AGE_ASK,
    AGE_NUMBER,
);
static AGE_PUNCTUATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-z0-9' ]+").unwrap());
static ABOUT_GRADES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:gcses?|grades?|marks|score|scores|pass|passed|mocks?|exams?)\b").unwrap()
});
static UNIVERSITY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\buni\b|\buniversit(y|ies)\b|\bcollege\b|sixth form").unwrap());

pub(crate) fn registered(
    id: &str,
    reading: &Reading<'_>,
    turns: &[Exchange],
) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    match id {
        "grade-meaning" => grade_result(&clean.to_lowercase()),
        "mock-grade" => Some(Cow::Owned(mock(&clean.to_lowercase()))),
        "revision-start" => revising(reading).then_some(Cow::Borrowed(data::REVISION)),
        "exam-nerves" => Some(nerves(clean_or_typed(reading))),
        "language-hours" => Some(assessments::language(reading, turns)),
        "university-pass-mark" => Some(assessments::university(reading)),
        "child-cant-write-his-name" => Some(Cow::Borrowed(assessments::handwriting(reading))),
        "job-grades" | "job-route" | "career-subjects" => Some(careers::reply(id, reading, turns)),
        "career-entry-points" => Some(careers::reply("career-subjects", reading, turns)),
        "study-age" => study_age(clean_or_typed(reading), turns),
        "course-hours" => {
            let said = clean_or_typed(reading).to_lowercase();
            let follows_course = !LATER_STUDY.is_match(&said)
                && turns.iter().rev().take(2).any(|turn| {
                    let before = turn.user.to_lowercase();
                    LATER_STUDY.is_match(&before) || CAREER_CHANGE.is_match(&before)
                });
            follows_course.then_some(Cow::Borrowed(data::COURSE_HOURS))
        }
        _ => None,
    }
}

fn clean_or_typed<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn study_age(text: &str, turns: &[Exchange]) -> Option<Cow<'static, str>> {
    let lowercase = text.to_lowercase();
    let said = AGE_PUNCTUATION.replace_all(&lowercase, " ");
    if NOT_AGE_STUDY.is_match(&said) {
        return None;
    }
    let career = CAREER_CHANGE.is_match(&said);
    if !career
        && (!AGE_ASK.is_match(&said)
            || !(LATER_STUDY.is_match(&said)
                || turns
                    .iter()
                    .rev()
                    .take(2)
                    .any(|turn| LATER_STUDY.is_match(&turn.user.to_lowercase()))))
    {
        return None;
    }
    let age = AGE_NUMBER
        .captures(&said)
        .and_then(|found| found[1].parse::<u8>().ok());
    match age {
        Some(age @ 61..=90) => Some(Cow::Owned(format!(
            "At {age} you would be older than most of the class, {}",
            data::STUDY_AGE_OLDER
        ))),
        Some(age @ 18..=60) => Some(Cow::Owned(format!(
            "At {age} you would not be late: {}",
            data::STUDY_AGE
        ))),
        _ => Some(Cow::Owned(format!(
            "You would not be late: {}",
            data::STUDY_AGE
        ))),
    }
}

fn grade(clean: &str) -> Option<u8> {
    let words: Vec<_> = clean
        .split(|c| c == ',' || crate::text::is_space(c))
        .filter(|word| !word.is_empty())
        .collect();
    for (at, &word) in words.iter().enumerate() {
        let [value @ b'1'..=b'9'] = word.as_bytes() else {
            continue;
        };
        let before = at
            .checked_sub(1)
            .and_then(|at| words.get(at))
            .copied()
            .unwrap_or("");
        let after = words.get(at + 1).copied().unwrap_or("");
        let next_two = words[at + 1..words.len().min(at + 3)].join(" ");
        if data::A_MARK_NOT_A_GRADE.contains(&before)
            || data::COUNTED_AFTER.contains(&after)
            || next_two.starts_with("a level")
            || GRADE_NAME.is_match(&next_two)
            || LETTER_GRADES.is_match(&next_two)
        {
            continue;
        }
        return Some(value - b'0');
    }
    None
}

fn decimal(text: &str) -> Option<u32> {
    if text.is_ascii() {
        return text.parse().ok();
    }
    text.chars().try_fold(0, |number, ch| {
        let point = u32::from(ch);
        let digit = if let Some(digit) = ch.to_digit(10) {
            digit
        } else {
            let start = data::DECIMAL_ZEROES
                .partition_point(|&zero| zero <= point)
                .checked_sub(1)?;
            let digit = point - data::DECIMAL_ZEROES[start];
            if digit >= 10 {
                return None;
            }
            digit
        };
        Some(number * 10 + digit)
    })
}

fn score_sentence(clean: &str) -> Option<String> {
    let found = SCORE.captures(clean)?;
    let got = decimal(&found[1])?;
    let total = decimal(&found[2])?;
    if got == 0 || total < 5 || got > total || total > 200 {
        return None;
    }
    let percentage = (100.0 * f64::from(got) / f64::from(total)).round_ties_even();
    let part = if (40.0..=60.0).contains(&percentage) {
        "about half"
    } else if percentage > 60.0 {
        "most"
    } else {
        "under half"
    };
    Some(format!("That is {got} out of {total}, {part} the marks."))
}

fn grade_result(clean: &str) -> Option<Cow<'static, str>> {
    if TOP_GRADES.is_match(clean) {
        return None;
    }
    if let Some(score) = score_sentence(clean) {
        return Some(Cow::Owned(format!(
            "{score} Treat that as a score rather than a grade, because a GCSE grade is where you stand across a whole subject and one piece of work cannot tell you that. Go through it and write down the questions that lost the marks, then ask your teacher for the mark scheme on the worst two, since the same kind of question is usually where the marks are next time."
        )));
    }
    let Some(grade) = grade(clean) else {
        return ABOUT_GRADES.is_match(clean).then_some(Cow::Borrowed(
            "GCSE grades run from 9 down to 1: 4 is a standard pass, 5 a strong one, and 7 to 9 are the top grades. Below 4 is not a pass yet, and English and maths can be retaken at college until you are 18. Tell me the grade and the subject and I will say what it opens up."
        ));
    };
    let sentence = data::GRADES[usize::from(grade - 1)];
    if grade >= 4 {
        Some(Cow::Owned(format!(
            "{sentence} English and maths are the two that colleges and employers look at first, whatever else you take."
        )))
    } else {
        Some(Cow::Borrowed(sentence))
    }
}

fn mock(clean: &str) -> String {
    let mut answer = String::new();
    if let Some(grade) = grade(clean) {
        answer.push_str(data::GRADES[usize::from(grade - 1)]);
        answer.push(' ');
    }
    if let Some(score) = score_sentence(clean) {
        answer.push_str(&score);
        answer.push(' ');
    }
    answer.push_str("A mock or a predicted grade is a signal, not a verdict: find out which topics cost you the marks — the mark scheme for a mock, your teacher for a predicted grade — and give the worst two a 25-minute slot each week. Moving a grade or two in a term is normal.");
    if UNIVERSITY.is_match(clean) {
        answer.push_str(" For the university question, what counts is what the course asks for: check two or three you like and their entry requirements, because a 6 at GCSE rarely closes a door on its own.");
    }
    answer
}

fn revising(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let said = if line.norm_text.is_empty() {
        &line.clean
    } else {
        &line.norm_text
    };
    !TEST_AS_A_VERB.is_match(said)
        || TIME_AFTER_TEST.is_match(said)
        || line.words.iter().any(|word| {
            data::LEARNING_OBJECTS
                .iter()
                .any(|one| word.eq_ignore_ascii_case(one))
        })
}

fn nerves(text: &str) -> Cow<'static, str> {
    let said = text.to_lowercase();
    if !data::NERVES_BODY.iter().any(|word| said.contains(word)) {
        return Cow::Borrowed(data::EXAM_NERVES);
    }
    let sign = data::SIGNS
        .iter()
        .find_map(|&(phrases, label)| {
            phrases
                .iter()
                .any(|phrase| said.contains(phrase))
                .then_some(label)
        })
        .unwrap_or("That feeling");
    let verb = if data::SIGN_PLURAL.contains(&sign) {
        "are"
    } else {
        "is"
    };
    Cow::Owned(format!(
        "{sign} before an exam {verb} adrenaline, not a bug: eat something plain, take water in with you, and breathe out slowly for a minute in the queue. {}",
        data::EXAM_NERVES
    ))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/study_learning.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn study_replies_use_the_source_score_and_course_context() {
        for &fixtures::Case(id, raw, history, expected) in fixtures::CASES {
            let turns: Vec<_> = history
                .iter()
                .map(|&user| Exchange {
                    user: user.into(),
                    bot: String::new(),
                    module: "answers",
                })
                .collect();
            let actual = registered(id, &Reading::new(raw), &turns);
            assert_eq!(actual.as_deref(), expected, "{id}: {raw} after {history:?}");
        }
    }
}
