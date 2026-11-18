//! The explicit per-relation age lists used by the memory store.

use super::{AGES, WORDS, decimal};
use crate::bot::{Memory, Reading};
use fancy_regex::{Captures, Regex};

#[path = "groupages_store_data.rs"]
mod data;
use data::*;

pattern!(
    NOT_A_RECALL,
    r"\b(?:normal|typical|usual|common|expected)\b"
);
pattern!(IN_LAWS, r"\bin laws$");

#[derive(Default)]
pub(super) struct Stored {
    lists: Vec<(String, Vec<u16>)>,
}

fn flat(text: &str) -> String {
    text.split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn matches(regex: &Regex, text: &str) -> bool {
    regex.is_match(text).unwrap_or(false)
}

fn starts(regex: &Regex, text: &str) -> bool {
    regex
        .find(text)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == 0)
}

fn part<'a>(found: &Captures<'a>, name: &str) -> Option<&'a str> {
    Some(found.name(name)?.as_str())
}

fn maybe_key<'a>(words: &[(&'a str, &'a str)], label: &str) -> Option<&'a str> {
    words
        .iter()
        .find_map(|(key, value)| (*key == label).then_some(*value))
}

fn ages(raw: &str) -> Option<Vec<u16>> {
    let ages: Vec<_> = AGES
        .find_iter(raw)
        .flatten()
        .map(|age| decimal(age.as_str()))
        .collect::<Option<_>>()?;
    ages.iter().all(|age| *age <= 120).then_some(ages)
}

fn keep_same_line(found: &mut Vec<(String, Vec<u16>)>, who: String, ages: Vec<u16>) {
    if let Some((_, prior)) = found.iter_mut().find(|(record, _)| *record == who) {
        if prior.len() >= ages.len() && ages.iter().all(|age| prior.contains(age)) {
            return;
        }
        if ages.len() > 1 {
            *prior = ages;
        } else if let Some(age) = ages.first()
            && !prior.contains(age)
        {
            prior.push(*age);
        }
    } else {
        found.push((who, ages));
    }
}

fn finds_lists(raw: &str) -> Vec<(String, Vec<u16>)> {
    let digits = super::super::kidcount::figures_as_digits(raw);
    let said = flat(&digits);
    if said.is_empty() {
        return Vec::new();
    }
    let said = if starts(&QUESTION_OPEN, &said) {
        CLAUSE_JOIN
            .split(&said)
            .flatten()
            .map(|part| part.trim_matches([' ', ',', '.', ';', ':', '!', '?']))
            .filter(|clause| !clause.is_empty() && !starts(&QUESTION_OPEN, clause))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        said
    };
    let mut found = Vec::new();
    for pattern in [&KID_GROUP_AGES_ARE, &KID_COUNT_AGES, &KID_ADULTS_AGES] {
        for groups in pattern.captures_iter(&said).flatten() {
            let (Some(full), Some(run)) = (groups.get(0), groups.name("ages")) else {
                continue;
            };
            if matches(&KID_AGES_NOT, &said[..run.start()])
                || !starts(&AGE_TAIL, &said[full.end()..])
            {
                continue;
            }
            let Some(ages) = ages(run.as_str()) else {
                continue;
            };
            let who = part(&groups, "who")
                .map(str::to_lowercase)
                .unwrap_or_else(|| {
                    found
                        .iter()
                        .find(|(who, _): &&(String, Vec<u16>)| KID_GROUP.contains(&who.as_str()))
                        .map_or_else(|| "kids".into(), |(who, _)| who.clone())
                });
            keep_same_line(&mut found, who, ages);
        }
    }
    for groups in RELATION_AGE.captures_iter(&said).flatten() {
        let (Some(full), Some(age)) = (groups.get(0), groups.name("age")) else {
            continue;
        };
        if let Some(word) = part(&groups, "many")
            && let Ok(Some(list)) = A_LIST_OF_AGES.captures(&said[age.start()..])
            && let Some(list_span) = list.get(0)
            && list_span.start() == 0
            && starts(&AGE_TAIL, &said[age.start() + list_span.end()..])
            && let Some(ages) = part(&list, "ages")
                .and_then(ages)
                .filter(|ages| ages.len() > 1)
        {
            keep_same_line(&mut found, flat(&word.to_lowercase()), ages);
            continue;
        }
        if !starts(&AGE_TAIL, &said[full.end()..]) {
            continue;
        }
        let (Some(who), Some(age)) = (
            part(&groups, "many").or_else(|| part(&groups, "one")),
            decimal(age.as_str()).filter(|age| *age <= 120),
        ) else {
            continue;
        };
        let who = flat(&who.to_lowercase());
        let who = IN_LAWS.replace(&who, "in law");
        let principal = who
            .strip_suffix('s')
            .filter(|one| KNOWN_RELATIONS.contains(one))
            .unwrap_or(&who);
        if found.iter().all(|(word, _)| word != principal) {
            found.push((principal.into(), vec![age]));
        }
    }
    found
}

impl Stored {
    pub(super) fn observe(&mut self, raw: &str) {
        for (group, ages) in finds_lists(raw) {
            if let Some(old) = self.lists.iter_mut().find(|(word, _)| *word == group) {
                // A scalar for the same stored relation retires its previous list. Keep
                // its position for a subsequent list of that relation.
                old.1 = if ages.len() > 1 { ages } else { Vec::new() };
            } else {
                self.lists
                    .push((group, if ages.len() > 1 { ages } else { Vec::new() }));
            }
        }
    }

    fn live_lists(&self) -> impl Iterator<Item = (&str, &[u16])> {
        self.lists
            .iter()
            .filter(|(_, ages)| !ages.is_empty())
            .map(|(group, ages)| (group.as_str(), ages.as_slice()))
    }

    fn has_any(&self) -> bool {
        self.live_lists().next().is_some()
    }
}

fn plural(who: &str) -> String {
    if let Some(word) = maybe_key(MANY_OF_THEM, who) {
        return word.to_owned();
    }
    if who.ends_with('s') || MANY_OF_THEM.iter().any(|(_, plural)| *plural == who) {
        return who.to_owned();
    }
    if let Some(before) = who.strip_suffix('y')
        && before
            .chars()
            .next_back()
            .is_none_or(|letter| !"aeiou".contains(letter))
    {
        return format!("{before}ies");
    }
    format!("{who}s")
}

fn sentence<'a>(lists: impl Iterator<Item = (&'a str, &'a [u16])>) -> Option<String> {
    let mut clauses = Vec::new();
    let mut remaining = 4;
    for (who, ages) in lists {
        let used = ages.len().min(remaining);
        let Some((last, earlier)) = ages[..used].split_last() else {
            break;
        };
        let clause = if earlier.is_empty() {
            let verb = if who.ends_with('s') { "are" } else { "is" };
            format!("your {who} {verb} {last}")
        } else {
            let list = earlier
                .iter()
                .map(u16::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("your {} are {list} and {last}", plural(who))
        };
        clauses.push(clause);
        remaining -= used;
        if remaining == 0 {
            break;
        }
    }
    let (last, before) = clauses.split_last()?;
    let wording = if before.is_empty() {
        last.clone()
    } else {
        format!("{} and {last}", before.join(", "))
    };
    Some(format!("Y{}.", &wording[1..]))
}

fn child(group: &str) -> bool {
    CHILD.contains(&maybe_key(CHILD_OF, group).unwrap_or_else(|| group.trim_end_matches('s')))
        || CHILD.contains(&group)
}

fn end_child(low: &str, memory: &Memory, stored: &Stored) -> Option<String> {
    let which = A_PLACE_ASK.captures(low).ok().flatten()?;
    let tail = low[which.get(0)?.end()..]
        .split(['.', ',', ';', ':', '!', '?'])
        .next()?
        .trim();
    if let Some(word) = tail.split_whitespace().next()
        && !PLACE_STANDS_ALONE.contains(&word)
    {
        return None;
    }
    let word = flat(part(&which, "who")?);
    let mut ages: Vec<_> = stored
        .live_lists()
        .filter(|(role, _)| child(role))
        .flat_map(|(_, ages)| ages.iter().copied())
        .collect();
    if ages.is_empty() {
        return None;
    }
    ages.extend(
        memory
            .personal
            .family_people()
            .filter(|(role, _, _)| child(role))
            .filter(|(role, _, _)| stored.live_lists().all(|(group, _)| group != *role))
            .filter_map(|(_, _, age)| age.map(u16::from)),
    );
    ages.sort_unstable();
    let value = if YOUNGER_END.contains(&word.as_str()) {
        ages.first()?
    } else {
        let (old, others) = ages.split_last()?;
        if others.last() == Some(old) {
            return None;
        }
        old
    };
    Some(format!("Your {word} is {value}."))
}

fn age_ask_allowed(low: &str, memory: &Memory, stored: &Stored) -> bool {
    let Some(found) = AGE_ASK.find(low).ok().flatten() else {
        return true;
    };
    let mut rest = low[found.end()..].trim_start();
    while let Ok(Some(adverb)) = AGE_ADVERB.find(rest) {
        rest = rest[adverb.end()..].trim_start();
    }
    if let Ok(Some(word)) = AGE_RULE_WORD.find(rest) {
        rest = rest[word.end()..].trim_start();
        let Some(say) = AGE_YOU_SAY.find(rest).ok().flatten() else {
            return false;
        };
        rest = rest[say.end()..].trim_start();
    }
    if let Ok(Some(word)) = AGE_COPULA.find(rest) {
        rest = rest[word.end()..].trim_start();
    }
    if let Ok(Some(word)) = AGE_DETERMINER.find(rest) {
        if matches!(word.as_str(), "a" | "an") {
            return false;
        }
        rest = rest[word.end()..].trim_start();
    }
    let Ok(Some(subject)) = WORDS.find(rest) else {
        return false;
    };
    subject.start() == 0
        && (matches(&AGE_PERSON_WORD, subject.as_str())
            || KNOWN_RELATIONS.contains(&subject.as_str())
            || stored
                .live_lists()
                .any(|(name, _)| name == subject.as_str())
            || memory.personal.family_people().any(|(role, name, _)| {
                role == subject.as_str()
                    || name.is_some_and(|name| name.eq_ignore_ascii_case(subject.as_str()))
            }))
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory, stored: &Stored) -> Option<String> {
    if !stored.has_any() {
        return None;
    }
    let low = flat(&reading.clean);
    let age = matches(&AGE_ASK, &low);
    if !age
        && (!matches(&AGE_WORD, &low)
            || matches(&AGE_AS_A_CAUSE, &low)
            || matches(&NOT_A_RECALL, &low))
    {
        return None;
    }
    if !age_ask_allowed(&low, memory, stored) {
        return None;
    }
    if let Some(position) = end_child(&low, memory, stored) {
        return Some(position);
    }
    if let Some(found) = GROUP_ASK.captures(&low).ok().flatten() {
        let asked = found.get(1)?.as_str();
        let (_, family) = FAMILY_GROUP.iter().find(|(word, _)| *word == asked)?;
        let belongs = |role: &str| {
            role == asked
                || family.contains(&role)
                || maybe_key(CHILD_OF, role).is_some_and(|one| family.contains(&one))
        };
        let held = stored.live_lists().filter(|(word, _)| belongs(word));
        return sentence(held);
    }
    if let Some(who) = DIRECT_RELATION.captures(&low).ok().flatten() {
        let role = part(&who, "who")?;
        if memory
            .personal
            .family_people()
            .any(|(person, _, age)| person == role && age.is_some())
        {
            return None;
        }
        let equivalent = plural(role);
        let held = stored
            .live_lists()
            .filter(|(who, _)| *who == role || *who == equivalent);
        return sentence(held);
    }
    if !matches(&THE_KIDS, &low) {
        return None;
    }
    let children = stored.live_lists().any(|(group, _)| child(group));
    let children_only = children || matches(&CHILD_WORD, &low);
    let held = stored
        .live_lists()
        .filter(|(group, _)| !children_only || child(group));
    sentence(held)
}
