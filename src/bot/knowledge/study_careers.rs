//! Job entry requirements, training lengths and return-to-practice routes.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

use super::clean_or_typed;

#[path = "study_career_data.rs"]
mod data;

static ASKS_LENGTH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:how many years|how many months|how long|what length|years of study|length of the course|length of the training)\b").unwrap()
});
static DONE_BEFORE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:retrain|retraining|retrained|go back to|going back to|went back to|return to|returning to|returned to|years ago|used to be|i was a|i used to work|lapsed|let it lapse|on a break|career break|years out|years off|years away)\b").unwrap()
});

fn names(text: &str, name: &str) -> bool {
    let bytes = text.as_bytes();
    text.match_indices(name).any(|(at, alias)| {
        !at.checked_sub(1)
            .and_then(|at| bytes.get(at))
            .is_some_and(u8::is_ascii_lowercase)
            && !bytes
                .get(at + alias.len())
                .is_some_and(u8::is_ascii_lowercase)
    })
}

fn path_for(line: &str) -> Option<(&'static str, &'static str)> {
    data::JOB_LENGTHS
        .iter()
        .find_map(|&(aliases, length, returning)| {
            aliases
                .iter()
                .any(|alias| names(line, alias))
                .then_some((length, returning))
        })
}

fn length(line: &str, turns: &[Exchange]) -> Option<&'static str> {
    if !ASKS_LENGTH.is_match(line) {
        return None;
    }
    let prior = || {
        turns.iter().rev().take(6).find_map(|turn| {
            let user = turn.user.to_lowercase();
            let path = path_for(&user)?;
            Some((path, DONE_BEFORE.is_match(&user)))
        })
    };
    let ((length, returning), earlier_return) =
        path_for(line).map(|path| (path, false)).or_else(prior)?;
    if !returning.is_empty() && (earlier_return || DONE_BEFORE.is_match(line)) {
        Some(returning)
    } else {
        Some(length)
    }
}

pub(super) fn reply(id: &str, reading: &Reading<'_>, turns: &[Exchange]) -> Cow<'static, str> {
    let line = clean_or_typed(reading).to_lowercase();
    if let Some(reply) = length(&line, turns) {
        return Cow::Borrowed(reply);
    }
    if id == "career-subjects" {
        let reply = data::CAREER_SUBJECTS.iter().find_map(|&(aliases, reply)| {
            aliases
                .iter()
                .any(|alias| names(&line, alias))
                .then_some(reply)
        });
        return Cow::Borrowed(reply.unwrap_or(data::THE_GENERAL_CAREER));
    }
    let words = format!(" {} ", reading.line.words.join(" "));
    let leading = data::GRADES_ROUTE.iter().find_map(|&(aliases, reply)| {
        aliases.iter().any(|alias| words.contains(&format!(" {alias} "))).then_some(reply)
    }).unwrap_or("The route sets the grades, so there is no single number: the job names the course, and the course names the subjects.");
    Cow::Owned(format!(
        "{leading} Compare the current entry requirements for two or three relevant courses or employers before applying. Check for subjects, grade equivalents and alternative routes."
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn career_cards_keep_duration_and_entry_questions_distinct() {
        assert!(
            reply(
                "job-grades",
                &Reading::new("What grades do I need to be a doctor?"),
                &[]
            )
            .contains("chemistry")
        );
        assert!(
            reply(
                "job-route",
                &Reading::new("How long does it take to become a doctor?"),
                &[]
            )
            .contains("five years")
        );
        assert!(
            reply(
                "career-subjects",
                &Reading::new("What should I study to become a nurse?"),
                &[]
            )
            .contains("NMC-approved")
        );
        let earlier = [Exchange {
            user: "I worked as a physiotherapist years ago".into(),
            bot: String::new(),
            module: "",
        }];
        assert!(
            reply(
                "career-subjects",
                &Reading::new("How long does it take?"),
                &earlier
            )
            .contains("HCPC readmission")
        );
    }
}
