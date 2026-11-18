//! The user's prospective plans, separately from advice already delivered.

use std::{collections::VecDeque, sync::LazyLock};

use fancy_regex::{Captures, Match, Regex};

use crate::text::is_space;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "stated_plan_data.rs"]
mod data;
use data::*;

pattern!(CLAUSE_END, r"(?i)(?<=[.!?])\s|\s+(?:but|and then|so)\s+");
pattern!(
    COMMA_ASK,
    r"(?i),\s+(?:how|what|which|where|when|who|why|should|can|could|would|do|does|is|are|will|any|and)\b"
);
pattern!(
    CATCHING_A_GOODBYE,
    r"(?i)^to\s+(?:go|leave|head off|set off|get going|dash)\b"
);
pattern!(INFINITIVE, r"(?i)^to\b");
pattern!(
    DATED_CLAUSE,
    r"(?i)(?<=[.!?])\s+|,|;|\s+(?:but|and|so|then)\s+"
);
pattern!(
    LEAD,
    r"(?i)^(?:ok(?:ay)?[, ]+|and |so |but |also |well |oh )+"
);
pattern!(MY_PLANS, r"(?i)\b(?:my|our|i|we|i'm|im|me)\b");
pattern!(REPEATED_OWN, r"(?i)\b(?:i|my|we|our)\b");
pattern!(WORDS, r"[a-z']+");
pattern!(SEPARATORS, r"[\s,]+");
pattern!(NON_WORD, r"[^a-z' ]");
pattern!(FIRST_ARTICLE, r"^(?:the|my|our|a|an)\s+");
pattern!(FIRST_OWNER, r"^(?:the|my|our)\s+");
pattern!(FIRST_PREPOSITION, r"^(?:on|in|at)\s+");
pattern!(TRAILING_PREPOSITION, r"\s+(?:is|at|on|for)$");
pattern!(AT_LEAST_A_NAME, r"[a-z]{3}");
pattern!(
    HAS_ITS_PREPOSITION,
    r"^(?:on|this|next|today|tomorrow|tonight)\b"
);
pattern!(ORDINAL, r"^(?:the\s+)?\d{1,2}(?:st|nd|rd|th)$");
pattern!(
    MONTH,
    r"(?i)\b(?:january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|sept|oct|nov|dec)\b"
);
pattern!(
    WEEKDAY,
    r"(?i)\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|weekend)\b"
);
pattern!(NUMBERED_DAY, r"(?i)\b\d{1,2}(?:st|nd|rd|th)?\b");
pattern!(
    RELATIVE_MONTH,
    r"(?i)\b(?:next|this|that|the\s+following|the\s+coming|the\s+next)\s+month\b"
);
pattern!(
    KEPT_OTHER_PET,
    r"(?i)\b(?:my|our|(?:i|we) (?:have|have got|have just got|got)|(?:i|we)'?ve (?:got|just got))\s+(?:(?:[a-z]+|\d+)\s+){0,4}(?:dogs?|pupp(?:y|ies)|cats?|kittens?|rabbits?|hamsters?|guinea pigs?|parrots?|goldfish|fish|horses?)\b"
);
pattern!(
    BY_DATE,
    r"(?i)\d|\b(?:january|february|march|april|may|june|july|august|september|october|november|december)\b"
);

static AFTER_CLOCK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"(?i)^(?:on\s+)?{}$", APPOINTMENT_DAY.as_str()))
        .expect("appointment day after a clock")
});

#[derive(Clone)]
struct FiledPlan {
    text: String,
    origin: String,
}

#[derive(Default)]
pub(crate) struct Facts {
    plans: VecDeque<FiledPlan>,
    latest: Option<FiledPlan>,
}

fn matched<'a>(pattern: &Regex, text: &'a str) -> Option<Match<'a>> {
    pattern.find(text).ok().flatten()
}

fn captures<'a>(pattern: &Regex, text: &'a str) -> Option<Captures<'a>> {
    pattern.captures(text).ok().flatten()
}

fn test(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or_default()
}

fn starts(pattern: &Regex, text: &str) -> bool {
    matched(pattern, text).is_some_and(|found| found.start() == 0)
}

fn flat(text: &str) -> String {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn upper_head(text: &str) -> String {
    let mut chars = text.chars();
    let mut result = String::with_capacity(text.len());
    if let Some(first) = chars.next() {
        result.extend(first.to_uppercase());
        result.push_str(chars.as_str());
    }
    result
}

fn fact_value(plan: &str) -> &str {
    let source = plan.trim_matches(is_space);
    let mut begin = 0;
    let mut prev = "";
    for fence in SEPARATORS.find_iter(source).flatten() {
        let word = &source[begin..fence.start()];
        let low = word.to_lowercase();
        if low != "then"
            && ASK_WORDS.contains(&low.as_str())
            && !(prev == "then" && AFTER_THEN.contains(&low.as_str()))
        {
            return trim_fact(&source[..begin]);
        }
        prev = if low == "then" { "then" } else { "" };
        begin = fence.end();
    }
    let tail = source[begin..].to_lowercase();
    let kept = if ASK_WORDS.contains(&tail.as_str())
        && !(prev == "then" && AFTER_THEN.contains(&tail.as_str()))
    {
        &source[..begin]
    } else {
        source
    };
    trim_fact(kept)
}

fn trim_fact(value: &str) -> &str {
    let trimmed = value.trim_matches([' ', ',', ';', ':']);
    if trimmed.to_lowercase().ends_with(" then") {
        trimmed[..trimmed.len() - 5].trim_end_matches([' ', ',', ';', ':'])
    } else {
        trimmed
    }
}

fn clipped_plan(text: &str) -> String {
    let mut text = text.trim_matches([' ', ',', ';', ':', '-']);
    for boundary in [&*CLAUSE_END, &*COMMA_ASK] {
        if let Some(end) = matched(boundary, text) {
            text = text[..end.start()].trim_matches([' ', ',', ';', ':', '-']);
        }
    }
    upper_head(text)
}

fn asks_about_plan(before: &str) -> bool {
    if test(&ASKED_BEFORE, before) {
        return true;
    }
    let start = PLAN_CLAUSE_BREAK
        .find_iter(before)
        .flatten()
        .last()
        .map_or(0, |found| found.end());
    let clause = before[start..].trim_matches(is_space);
    starts(&ASKING, &PLAN_CLAUSE_OPENER.replace(clause, ""))
}

pub(super) fn opens_on_a_place(phrase: &str) -> bool {
    phrase
        .split_whitespace()
        .next()
        .is_some_and(|word| !PLACE_VERBS.contains(&word) && !NOT_PLACES.contains(&word))
}

fn explicit(text: &str) -> Option<String> {
    let opening = matched(&PLAN, text)?;
    if asks_about_plan(&text[..opening.start()])
        || test(&GOT_A, opening.as_str()) && !test(&FUTURE, text) && !test(&GOT_TO, text)
    {
        return None;
    }
    let plan = clipped_plan(&text[opening.end()..]);
    if plan.is_empty()
        || test(&CATCHING_A_GOODBYE, &plan) && test(&FAREWELL, text)
        || plan
            .chars()
            .next()
            .is_some_and(|ch| matches!(ch, '£' | '$' | '€') || ch.is_numeric())
    {
        return None;
    }

    let low = plan.to_lowercase();
    let as_opened = INFINITIVE.replace(&low, "");
    let first = as_opened
        .split_whitespace()
        .find(|word| !matches!(*word, "a" | "an" | "the" | "my" | "our"))
        .unwrap_or_default();
    let object_prep = low
        .split_whitespace()
        .next()
        .map(|word| word.replace(|ch: char| !ch.is_ascii_lowercase(), ""))
        .is_some_and(|first| PREPOSITIONS.contains(&first.as_str()));
    let has_object = !object_prep
        || WORDS
            .find_iter(&low)
            .flatten()
            .find(|word| {
                !ARTICLES.contains(&word.as_str()) && !PREPOSITIONS.contains(&word.as_str())
            })
            .is_some_and(|word| !POINTING.contains(&word.as_str()));
    let infinitive = test(&INFINITIVE, &low) || test(&INFINITIVE_FRAME, opening.as_str());
    if NOT_PLACES.contains(&first) || (!infinitive && (PLACE_VERBS.contains(&first) || !has_object))
    {
        return None;
    }
    let verb = PERSON_OPEN.replace(opening.as_str(), "");
    let verb = verb.trim_matches([' ', '.']);
    let mut words = verb.split_whitespace();
    let first = words.next().unwrap_or_default().to_lowercase();
    let last = verb
        .split_whitespace()
        .next_back()
        .unwrap_or_default()
        .to_lowercase();
    let carries_verb =
        first.ends_with("ing") && (object_prep || PARTICLES.contains(&last.as_str()));
    let plan = if carries_verb {
        let mut chars = plan.chars();
        let first = chars.next()?;
        format!(
            "{} {}{}",
            upper_head(verb),
            first.to_lowercase(),
            chars.as_str()
        )
    } else {
        plan
    };
    if test(&PETS, &plan) && test(&KEPT_OTHER_PET, text) {
        return None;
    }
    let plan = fact_value(&plan);
    (!plan.is_empty()).then(|| plan.to_owned())
}

fn coming_up(clause: &str) -> bool {
    !test(&RECURRENCE, clause)
        && (test(&FUTURE, clause)
            || test(&OBLIGATION, clause)
                && (test(&APPOINTMENT_CLOCK, clause) || test(&APPOINTMENT_DAY, clause))
            || test(&A_CLOCKED_EVENT, clause) && test(&APPOINTMENT_CLOCK, clause))
}

fn states_only(clause: &str) -> bool {
    if let Some(parts) = captures(&COPULA, clause)
        && !(test(&OFF_ABSENCE, parts.name("rest").unwrap().as_str()) && test(&OFF_AHEAD, clause))
    {
        let rest = parts.name("rest").unwrap().as_str().to_lowercase();
        let without_date = FUTURE.replace_all(&rest, " ");
        let words = NON_WORD.replace_all(&without_date, " ");
        let states: Vec<_> = words
            .split_whitespace()
            .filter(|word| !STATE_FILLER.contains(word) && !STATE_PARTICLE.contains(word))
            .collect();
        if !states.is_empty() && states.iter().all(|word| STATE_WORDS.contains(word)) {
            return true;
        }
    }
    let Some(parts) = captures(&THING_COPULA, clause) else {
        return false;
    };
    let rest = parts.name("rest").unwrap().as_str();
    if test(&AHEAD_OF_THE_THING, rest) {
        return false;
    }
    let rest = rest.to_lowercase();
    let undated = FUTURE.replace_all(&rest, " ");
    let clean = NON_WORD.replace_all(&undated, " ");
    let states: Vec<_> = clean
        .split_whitespace()
        .filter(|word| !STATE_FILLER.contains(word) && !STATE_PARTICLE.contains(word))
        .collect();
    !states.is_empty()
        && states
            .iter()
            .all(|word| STATE_WORDS.contains(word) || THING_TODAY.contains(word))
}

fn deed(clause: &str) -> bool {
    let Some(found) = captures(&PAST_DEED, clause) else {
        return false;
    };
    let verb = found[1].to_lowercase();
    !PAST_ARRANGED.contains(&verb.as_str())
        && (PAST_VERBS.contains(&verb.as_str())
            || verb.ends_with("ed")
                && verb.len() >= 5
                && !PAST_STATES.contains(&verb.as_str())
                && found.get(2).is_none_or(|word| {
                    !INTENTION_AFTER.contains(&word.as_str().to_lowercase().as_str())
                }))
}

fn after_subject(clause: &str) -> &str {
    let rest = matched(&CLAUSE_SUBJECT, clause).map_or(clause, |subject| &clause[subject.end()..]);
    matched(&WHEN_OR_WHERE, rest)
        .map_or(rest, |found| &rest[..found.start()])
        .trim_matches([' ', ','])
}

fn lend_verb(clause: String, earlier: &str) -> String {
    if earlier.is_empty() || !after_subject(&clause).is_empty() {
        return clause;
    }
    let body = after_subject(earlier);
    if !test(&A_SHARED_VERB, body) {
        return clause;
    }
    let end = matched(&CLAUSE_SUBJECT, &clause).map_or(0, |found| found.end());
    let subject = &clause[..end];
    let person = subject.trim().to_lowercase();
    let first = body.split_whitespace().next().unwrap_or_default();
    let low = first.to_lowercase();
    let rest = clause[end..].trim_start_matches(is_space);
    if rest.is_empty() {
        return clause;
    }
    let plural = matches!(person.as_str(), "we" | "they" | "you") || person.contains(" and ");
    let verb = if !plural {
        match low.as_str() {
            "am" | "are" => "is",
            "were" => "was",
            "is" if person == "i" => "am",
            _ => first,
        }
    } else {
        first
    };
    format!(
        "{} {verb}{} {rest}",
        subject.trim_end(),
        &body[first.len()..]
    )
    .trim()
    .to_owned()
}

fn dated_plans(text: &str) -> Vec<String> {
    if !test(&FUTURE, text) && !coming_up(text) {
        return Vec::new();
    }
    let mut plans = Vec::<String>::new();
    let mut begin = 0;
    let parts = DATED_CLAUSE
        .find_iter(text)
        .flatten()
        .map(|found| (found.start(), found.end()))
        .chain(std::iter::once((text.len(), text.len())));
    for (end, next) in parts {
        let clause_at = begin;
        let clause = flat(&text[begin..end]);
        let clause = clause.trim_matches([' ', ',', '.', ';', ':', '-', '!']);
        begin = next;
        if clause.is_empty() {
            continue;
        }
        let without_correction = CORRECTION_LEAD_RE.replace(clause, "");
        let opened = without_correction.trim_matches(is_space);
        let clause = if !opened.is_empty() && test(&OWN, opened) {
            opened
        } else {
            clause
        };
        let previous_piece = flat(&text[..clause_at]);
        let joined = if test(&SUBJECT_JOIN, &previous_piece) {
            format!("{previous_piece} {clause}")
        } else {
            clause.to_owned()
        };
        if !test(&OWN, &joined) && !test(&OWN, clause) && !test(&THE_THING_HAS_A_DAY, clause)
            || joined.contains('?')
        {
            continue;
        }
        let clause = lend_verb(joined, plans.last().map_or("", String::as_str));
        if !coming_up(&clause)
            || test(&PAST, &clause)
            || starts(&ASKING, &LEAD.replace(&clause, ""))
            || test(&SECOND_PERSON, &clause)
            || test(&NEGATED, &clause)
            || states_only(&clause)
            || deed(&clause)
            || !(3..=18).contains(&clause.split_whitespace().count())
            || !test(&MY_PLANS, &clause) && !test(&THE_THING_HAS_A_DAY, &clause)
            || REPEATED_OWN.find_iter(&clause).flatten().count() > 3
        {
            continue;
        }
        plans.push(clause);
    }
    plans
}

fn after_day(said: &str, day: Match<'_>) -> String {
    if day.start() > 4 {
        return String::new();
    }
    let rest = said[day.end()..].trim_matches([' ', ',', ';', ':', '-']);
    let rest = FIRST_PREPOSITION.replace(rest, "");
    let mut words = Vec::new();
    for word in rest.split_whitespace() {
        let bare = word.trim_matches([',', '.', '!', ';', ':']);
        if APPOINTMENT_A_VERB.contains(&bare)
            || APPOINTMENT_A_DATE_VERB.contains(&bare)
            || bare.len() > 3 && bare.ends_with("ed") && !APPOINTMENT_NOT_A_VERB.contains(&bare)
        {
            break;
        }
        words.push(word);
    }
    let thing = words.join(" ");
    FIRST_ARTICLE
        .replace(thing.trim_matches([' ', ',', ';', ':', '-']), "")
        .into_owned()
}

fn appointment(text: &str) -> Option<String> {
    let text = flat(text);
    let text = text.trim_matches([' ', '.', ',', '!', ';', ':', '-']);
    if text.is_empty() || text.contains('?') || text.split_whitespace().count() > 12 {
        return None;
    }
    let low = text.to_lowercase();
    if test(&APPOINTMENT_OWN_SUBJECT, &low)
        || starts(&ASKING, &low)
        || test(&APPOINTMENT_PAST, &low)
        || test(&NEGATED, &low)
        || test(&SECOND_PERSON, &low)
    {
        return None;
    }
    let clock = captures(&APPOINTMENT_CLOCK, &low).filter(|clock| {
        let found = clock.get(0).unwrap();
        !(found.as_str().starts_with("for ") && test(&A_COUNT_NOT_A_CLOCK, &low[..found.start()]))
    });
    let day = matched(&APPOINTMENT_DAY, &low);
    if let Some(parts) = &clock {
        let rest = low[parts.get(0).unwrap().end()..].trim_matches([' ', ',', '.', ';', ':', '-']);
        if !rest.is_empty() && !test(&AFTER_CLOCK, rest) {
            return None;
        }
    }
    let cut = clock
        .as_ref()
        .and_then(|value| value.get(0))
        .into_iter()
        .chain(day)
        .map(|found| found.start())
        .min()?;
    let prefix = FIRST_OWNER.replace(low[..cut].trim_matches([' ', ',', ';', ':', '-']), "");
    let prefix = TRAILING_PREPOSITION.replace(&prefix, "");
    let thing = if prefix.trim().is_empty() {
        day.map_or_else(String::new, |day| after_day(&low, day))
    } else {
        prefix.trim().to_owned()
    };
    let words: Vec<_> = thing.split_whitespace().collect();
    let first = *words.first()?;
    if words.len() > 4
        || test(&TAKE_BACK_WORDS, &thing)
        || words.iter().all(|word| {
            let word = word.trim_matches([',', '.', '!', ';', ':', '\'', '’']);
            word.is_empty() || DAY_REMARK_WORDS.contains(&word) || DAY_REMARK_GLUE.contains(&word)
        })
        || APPOINTMENT_NOT_A_THING.contains(&first)
        || APPOINTMENT_CORRECTION.contains(&first.trim_matches([',', '.', '!']))
        || !test(&AT_LEAST_A_NAME, &thing)
        || words.iter().any(|word| {
            APPOINTMENT_PAST_VERB.contains(word)
                || APPOINTMENT_A_VERB.contains(word)
                || word.len() > 3 && word.ends_with("ed") && !APPOINTMENT_NOT_A_VERB.contains(word)
        })
        || day.is_some_and(|day| test(&BY_DATE, day.as_str()))
            && APPOINTMENT_A_DATE_VERB.contains(&words.last()?.trim_matches([',', '.', '!']))
    {
        return None;
    }
    let mut plan = thing;
    if let Some(clock) = clock {
        plan.push_str(" at ");
        plan.push_str(&flat(&clock[1]));
    }
    if let Some(day) = day {
        let day = flat(day.as_str());
        plan.push_str(if test(&HAS_ITS_PREPOSITION, &day) {
            " "
        } else {
            " on "
        });
        plan.push_str(&day);
    }
    Some(plan)
}

fn without_filler(text: &str) -> String {
    let mut words: Vec<_> = text
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect();
    while words.len() > 1
        && TAIL_OFF.contains(
            &words
                .last()
                .unwrap()
                .to_lowercase()
                .trim_matches(['.', ',', ';', ':', '!', '?']),
        )
    {
        words.pop();
    }
    let mut text = words.join(" ");
    loop {
        let stripped = text.trim_end_matches([' ', '.', ',', ';', ':', '!', '?']);
        let low = stripped.to_lowercase();
        let cut = TAIL_PHRASES.iter().find_map(|phrase| {
            let head = low.strip_suffix(phrase)?;
            head.ends_with(' ').then(|| {
                stripped[..stripped.len() - phrase.len()]
                    .trim_end_matches([' ', ',', ';', ':', '-'])
            })
        });
        let Some(nonempty) = cut.filter(|value| !value.is_empty()) else {
            return text;
        };
        text = flat(nonempty);
    }
}

fn explicit_origin(record: &FiledPlan) -> Option<String> {
    explicit(&record.origin).map(|plan| without_filler(&plan))
}

fn topic(plan: &str) -> String {
    let low = plan.to_lowercase();
    let words: Vec<_> = WORDS
        .find_iter(&low)
        .flatten()
        .map(|word| word.as_str())
        .filter(|word| !WHEN_WORDS.contains(word))
        .collect();
    if words.len() > 1 {
        words.join(" ")
    } else {
        String::new()
    }
}

fn date_change(text: &str) -> Option<&str> {
    if text.contains('?') {
        return None;
    }
    let is_date = |date: &str| {
        let mut named = false;
        let mut number = false;
        for word in date.to_lowercase().split_whitespace() {
            if matches!(word, "of" | "the") {
                continue;
            }
            if test(&ORDINAL, word) || test(&MONTH, word) || test(&WEEKDAY, word) {
                named = true;
            } else if !number && word.chars().count() <= 2 && word.chars().all(char::is_numeric) {
                number = true;
            } else {
                return false;
            }
        }
        named
    };
    [
        &*CHANGED_TO,
        &*CHANGED_BARE,
        &*CHANGED_NOW,
        &*CORRECTION_A_DATE,
        &*A_TIME_INSTEAD,
        &*MOVED_BY_ITS_OWN_VERB,
    ]
    .into_iter()
    .filter_map(|pattern| captures(pattern, text))
    .find_map(|found| {
        found
            .iter()
            .skip(1)
            .flatten()
            .find(|word| is_date(word.as_str()))
            .map(|word| word.as_str())
    })
}

fn replace_in(text: &str, from: Match<'_>, to: &str) -> String {
    flat(&format!(
        "{}{to}{}",
        &text[..from.start()],
        &text[from.end()..]
    ))
}

fn change_month(plan: &str, date: &str) -> Option<String> {
    let month = matched(&MONTH, date)?;
    let month = upper_head(&month.as_str().to_lowercase());
    let day = matched(&NUMBERED_DAY, date).map(|found| found.as_str());
    if let Some(old) = matched(&MONTH, plan) {
        if day.is_none() {
            return Some(replace_in(plan, old, &month));
        }
        return None;
    }
    let old = matched(&RELATIVE_MONTH, plan)?;
    let new = day.map_or_else(
        || format!("in {month}"),
        |day| format!("on the {day} of {month}"),
    );
    Some(replace_in(plan, old, &new))
}

impl Facts {
    pub(super) fn correct_subject(&mut self, swap: impl Fn(&str) -> Option<String>) {
        for record in self.latest.iter_mut().chain(&mut self.plans) {
            if let Some(corrected) = swap(&record.text) {
                record.text = corrected;
            }
        }
    }

    pub(crate) fn observe(&mut self, raw: &str) {
        let text = flat(raw);
        if text.is_empty() {
            return;
        }
        let asking = test(&QUESTION_OPEN, &text);
        if asking {
            // The same spelling opens a question and “we're” statements.
            // The source keeps the explicit statement as its current plan.
            if text
                .split_whitespace()
                .next()
                .is_some_and(|word| word.eq_ignore_ascii_case("were"))
                && let Some(plan) = explicit(&text)
            {
                self.latest = Some(FiledPlan {
                    text: without_filler(&plan),
                    origin: text.clone(),
                });
            }
            return;
        }
        if let Some(date) = date_change(&text) {
            if let Some(previous) = &self.latest
                && let Some(current) = change_month(&previous.text, date)
            {
                let origin =
                    change_month(&previous.origin, date).unwrap_or_else(|| previous.origin.clone());
                let record = FiledPlan {
                    text: current,
                    origin,
                };
                for plan in &mut self.plans {
                    if plan.text.eq_ignore_ascii_case(&previous.text) {
                        plan.clone_from(&record);
                    }
                }
                self.latest = Some(record);
            }
            return;
        }
        let (first, items) = if let Some(plan) = explicit(&text) {
            (Some(plan.clone()), vec![plan])
        } else {
            let plans = dated_plans(&text);
            let first = appointment(&text).or_else(|| plans.first().cloned());
            let items = if plans.len() > 1 {
                plans
            } else {
                first.iter().cloned().collect()
            };
            (first, items)
        };
        if let Some(first) = first {
            self.latest = Some(FiledPlan {
                text: without_filler(&first),
                origin: text.clone(),
            });
        }
        if text.ends_with('?') {
            return;
        }
        for plan in items {
            let plan = without_filler(&plan);
            self.file(plan, &text);
        }
    }

    fn file(&mut self, plan: String, said: &str) {
        if plan.split_whitespace().count() < 2
            || test(&COUNT_HEAD, &plan)
            || self.plans.iter().any(|record| record.text == plan)
        {
            return;
        }
        let about = topic(&plan);
        if !about.is_empty() {
            self.plans.retain(|old| topic(&old.text) != about);
        }
        self.plans.push_back(FiledPlan {
            text: plan,
            origin: said.to_owned(),
        });
        if self.plans.len() > 6 {
            self.plans.pop_front();
        }
    }

    pub(crate) fn plans(&self) -> impl ExactSizeIterator<Item = &str> + DoubleEndedIterator {
        self.plans.iter().map(|record| record.text.as_str())
    }

    pub(super) fn last_filed_origin(&self) -> Option<&str> {
        let record = self.plans.back()?;
        (explicit_origin(record)? == record.text).then_some(record.origin.as_str())
    }

    pub(super) fn superseded_origin(&self, said: &str) -> bool {
        let said = flat(said);
        self.plans.iter().any(|record| {
            record.origin == said && explicit_origin(record).is_some_and(|plan| plan != record.text)
        })
    }

    pub(super) fn current_from(&self, said: &str) -> bool {
        self.latest.as_ref().is_some_and(|plan| plan.origin == said)
    }

    pub(crate) fn latest(&self) -> Option<&str> {
        self.latest.as_ref().map(|record| record.text.as_str())
    }

    pub(super) fn trip_origins(&self) -> impl DoubleEndedIterator<Item = (&str, &str)> {
        self.plans
            .iter()
            .chain(self.latest.iter())
            .map(|record| (record.text.as_str(), record.origin.as_str()))
    }

    pub(in crate::bot) fn file_booked_trip(&mut self, plan: String, said: &str) {
        let said = flat(said);
        if self
            .latest()
            .is_some_and(|latest| plan.eq_ignore_ascii_case(latest))
        {
            return;
        }
        self.latest = Some(FiledPlan {
            text: plan.clone(),
            origin: said.clone(),
        });
        if !said.ends_with('?') && !test(&QUESTION_OPEN, &said) {
            self.file(plan, &said);
        }
    }

    pub(in crate::bot) fn move_trip_month(
        &mut self,
        plan: &str,
        said: &str,
        old: &str,
        new: &str,
        line: &str,
    ) -> Option<String> {
        let old = Regex::new(&format!(r"(?i)\b{}\b", regex::escape(old))).ok()?;
        let value = old.replace(plan, new);
        if value == plan {
            return None;
        }
        let corrected = old.replace(said, new).into_owned();
        let saved = FiledPlan {
            text: value.into_owned(),
            origin: corrected.clone(),
        };
        let raw = flat(line).to_lowercase();
        let body = flat(&CORRECTION_LEAD_RE.replace(&raw, ""));
        self.plans.retain_mut(|one| {
            if one.text.eq_ignore_ascii_case(plan) {
                one.clone_from(&saved);
                return true;
            }
            let text = flat(&one.text).to_lowercase();
            text != raw && text != body
        });
        self.latest = Some(saved);
        Some(corrected)
    }

    pub(in crate::bot) fn values(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.plans().chain(
            self.latest()
                .filter(|latest| !latest.is_empty() && !self.plans().any(|plan| plan == *latest)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shared_verb_and_a_revised_date_keep_each_plan_in_order() {
        let mut facts = Facts::default();
        facts.observe("my sister is coming in may and my brother in june");
        facts.observe("actually my brother is coming in july");
        assert_eq!(
            facts.values().collect::<Vec<_>>(),
            ["my sister is coming in may", "my brother is coming in july"]
        );
    }

    #[test]
    fn the_plan_part_of_a_question_and_a_month_correction_are_filed() {
        let mut facts = Facts::default();
        facts.observe("i need to save 480 pounds in 6 months, how much each month?");
        assert_eq!(
            facts.values().collect::<Vec<_>>(),
            ["Save 480 pounds in 6 months"]
        );

        let mut facts = Facts::default();
        facts.observe("im going to Rome next month");
        facts.observe("no in july");
        assert_eq!(facts.values().collect::<Vec<_>>(), ["Rome in July"]);
        facts.observe("what are my plans again?");
        assert_eq!(facts.values().collect::<Vec<_>>(), ["Rome in July"]);
    }

    #[test]
    fn unpunctuated_we_are_keeps_the_singular_plan_across_trip_details() {
        let mut facts = Facts::default();
        facts.observe("were going to edinburgh in august");
        facts.observe("two adults and a baby");
        facts.observe("for four days");
        facts.observe("is the castle ok with a pram");
        assert_eq!(facts.latest(), Some("Edinburgh in august"));
        assert_eq!(facts.plans().len(), 0);

        let mut facts = Facts::default();
        facts.observe("were we going to edinburgh in august");
        assert_eq!(facts.latest(), None);
    }
}
