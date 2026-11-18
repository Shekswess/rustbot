//! Follow-ups on contact with an organization and on a personally unanswered message.
#[path = "chasing_data.rs"]
mod data;
use crate::bot::{Memory, Reading, compose};
use data::*;
use fancy_regex::Regex;
use std::sync::LazyLock;
type Body = (&'static str, &'static str);
fn hit(re: &Regex, text: &str) -> bool {
    re.is_match(text).unwrap_or(false)
}
fn body(line: &str) -> Option<Body> {
    if hit(&A_PERSON_WHO_WORKS_THERE, line) {
        return None;
    }
    static WORDS: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new("[a-z']+").unwrap());
    static AFTER: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"^\s+(?:(\w+)\s+)?(\w+)").unwrap());
    for &(triggers, org, reference) in ORGANISATIONS {
        for trigger in triggers {
            if trigger.contains(' ') {
                if line.contains(trigger) {
                    return Some((org, reference));
                }
            } else if WORDS.find_iter(line).any(|m| m.as_str() == *trigger) {
                let person = line
                    .match_indices(trigger)
                    .filter(|(i, _)| {
                        line[..*i]
                            .chars()
                            .next_back()
                            .is_none_or(|c| !c.is_alphanumeric() && c != '_')
                    })
                    .find_map(|(i, w)| AFTER.captures(&line[i + w.len()..]))
                    .is_some_and(|m| {
                        m.iter()
                            .skip(1)
                            .flatten()
                            .any(|w| A_PERSON_AFTER_THE_NAME.binary_search(&w.as_str()).is_ok())
                    });
                if !person {
                    return Some((org, reference));
                }
            }
        }
    }
    None
}
fn earlier<'a>(m: &'a Memory, now: &'a str) -> impl Iterator<Item = String> + 'a {
    m.turns.iter().rev().filter_map(move |t| {
        let user = t.user.trim().to_lowercase();
        (!user.is_empty() && user != now)
            .then(|| user.split_whitespace().collect::<Vec<_>>().join(" "))
    })
}
fn filled(template: &str, body: Body) -> String {
    let text = template
        .replace("{reference}", body.1)
        .replace("{body}", body.0);
    let mut output = String::with_capacity(text.len());
    let mut prev = [' ', ' '];
    for (at, c) in text.chars().enumerate() {
        if at == 0 || prev[1] == ' ' && matches!(prev[0], '.' | '!' | '?') {
            output.push(c.to_ascii_uppercase())
        } else {
            output.push(c)
        }
        prev = [prev[1], c];
    }
    output
}
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let s = reading.folded();
    if s.is_empty() {
        return None;
    }
    let choose = |pool: &'static [&'static str]| compose::choose_recent(pool, memory, s);
    let finds = |r: &Regex| hit(r, s);
    let previous = || earlier(memory, s).find_map(|l| body(&l));
    if let Some(org) = body(s) {
        return (finds(&NOT_DONE) || finds(&HOW_TO_CHASE)).then(|| filled(choose(THE_SHAPE), org));
    }
    let continuation = BARE_CONTINUATION
        .find(s)
        .ok()
        .flatten()
        .is_some_and(|m| m.start() == 0);
    if !continuation
        && finds(&THE_TIMESCALE_ASK)
        && let Some(org) = previous()
    {
        return Some(filled(choose(THE_TIMESCALE), org));
    }
    if continuation {
        if let Some(org) = previous() {
            return Some(filled(choose(THE_NEXT_STEP), org));
        }
        return earlier(memory, s)
            .next()
            .filter(|l| {
                body(l).is_none()
                    && !hit(&A_PERSON_NOT_A_BODY, l)
                    && hit(&RAISED, l)
                    && hit(&NOT_DONE, l)
            })
            .map(|_| filled(choose(THE_NEXT_STEP_UNNAMED), ("", "")));
    }
    if finds(&IVE_TRIED)
        && let Some(org) = previous()
    {
        return Some(format!(
            "{} {}",
            choose(IVE_TRIED_LEAD),
            filled(choose(THE_SHAPE), org)
        ));
    }
    let wait = finds(&A_PERSON_OWES_A_REPLY)
        || finds(&A_PERSON_DID_THE_SILENT_THING)
        || (finds(&A_PRONOUN_DID_THE_SILENT_THING) || finds(&A_PRONOUN_OWES_A_REPLY))
            && memory
                .turns
                .iter()
                .rev()
                .take(3)
                .any(|t| hit(&OWN_PERSON, &t.user));
    if wait && !finds(&ANSWERING_SOMETHING_ELSE) {
        return Some(choose(THE_WAIT_FOR_A_PERSON).into());
    }
    (finds(&RAISED) && finds(&NOT_DONE) && !finds(&A_PERSON_NOT_A_BODY))
        .then(|| choose(THE_SHAPE_UNNAMED).into())
}
