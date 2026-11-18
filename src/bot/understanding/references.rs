//! Resolve the previous turn's entities and topic for conversational pronouns.

use super::{breeds, entities, entities::Entity, kinds::Kind, line::Line};
use regex::Regex;
use std::{collections::BTreeSet, sync::LazyLock};

#[path = "references_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));
    };
}

pattern!(PLACEHOLDER_IT);
pattern!(NAMES_A_THING);
pattern!(ANIMALS_IN_HAND);

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());

/// The prior turn's usable entities and topic. Presence also means this is a follow-up candidate.
pub(crate) struct Prior<'a> {
    pub(crate) entities: &'a [Entity],
    pub(crate) topic: Option<&'a str>,
}

/// The referents supplied to reply selection and lookup.
#[derive(Clone, Debug, Default, Eq, PartialEq, serde::Serialize)]
pub(crate) struct References {
    pub(crate) it: Option<String>,
    pub(crate) that: Option<String>,
    pub(crate) they: Option<String>,
    pub(crate) there: Option<String>,
    pub(crate) he: Option<String>,
    pub(crate) she: Option<String>,
    pub(crate) kind: Option<&'static str>,
    pub(crate) followup: bool,
}

const PRONOUNS: [&[&str]; 6] = [
    data::IT_WORDS,
    data::THAT_WORDS,
    data::THEY_WORDS,
    data::THERE_WORDS,
    data::HE_WORDS,
    data::SHE_WORDS,
];

fn has(words: &[&str], word: &str) -> bool {
    words.binary_search(&word).is_ok()
}

pub(super) fn target(prior: &Prior<'_>) -> (Option<String>, Option<&'static str>) {
    let mut things = prior
        .entities
        .iter()
        .filter(|entity| entity.kind == "thing");
    if let Some(first) = things.next() {
        let mut text = first.text.clone();
        let mut at = first.at;
        for later in things {
            if later.at == at + 1 {
                text.push(' ');
                text.push_str(&later.text);
                at = later.at;
            }
        }
        return (Some(text.to_lowercase()), Some("thing"));
    }
    if let Some(entity) = first_kind(prior.entities, &["title", "name", "place", "person"]) {
        return (Some(entity.text.to_lowercase()), Some(entity.kind));
    }
    if let Some(topic) = prior.topic.filter(|topic| !topic.is_empty()) {
        return (Some(topic.replace('-', " ")), Some("topic"));
    }
    (None, None)
}

fn first_kind<'a>(entities: &'a [Entity], order: &[&str]) -> Option<&'a Entity> {
    order
        .iter()
        .find_map(|kind| entities.iter().find(|entity| &entity.kind == kind))
}

fn last_noun_phrase(norm: &[String], at: usize) -> Option<usize> {
    let end = at.checked_sub(2)?;
    for index in (at.saturating_sub(11)..=end).rev() {
        let word = &norm[index];
        if has(data::DETERMINERS, word) {
            return Some(index);
        }
        if has(data::PRONOUN_WORDS, word) {
            return None;
        }
    }
    None
}

fn names_its_own_thing(norm: &[String], at: usize) -> bool {
    last_noun_phrase(norm, at).is_some_and(|opened| {
        has(data::POSSESSIVE_PRONOUNS, &norm[at])
            || norm[opened..at]
                .iter()
                .any(|word| has(data::COORDINATORS, word))
    })
}

fn asks_about(line: &Line, aliases: &[&str]) -> bool {
    line.clauses.iter().any(|clause| {
        clause
            .first()
            .is_some_and(|word| has(data::ASK_OPENERS, word))
            && clause.iter().take(3).any(|word| has(aliases, word))
    })
}

fn named_before(line: &Line, entities: &[Entity], aliases: &[&str], moved: bool) -> bool {
    let Some(at) = line.norm.iter().position(|word| has(aliases, word)) else {
        return false;
    };
    if at > 0 && has(data::POINTING_BACK, &line.norm[at - 1]) || asks_about(line, aliases) {
        return false;
    }
    entities
        .iter()
        .any(|entity| has(data::OWN_PERSONS, entity.kind) && entity.at < at)
        || moved && names_its_own_thing(&line.norm, at)
}

/// A newly introduced person should keep a question from inheriting an unrelated subject.
pub(crate) fn person_is_new(line: &Line, entities_here: &[Entity], prior: &[Entity]) -> bool {
    let people_here = entities_here.iter().any(|entity| entity.kind == "person")
        || !entities::person_names(&line.norm).is_empty();
    if !people_here && !line.norm.iter().any(|word| has(data::ONE_PERSON, word)) {
        return false;
    }
    !prior.iter().any(|entity| entity.kind == "person")
}

fn only_purpose(norm: &[String]) -> bool {
    norm.len() >= 2
        && norm[0] == "to"
        && norm[1..]
            .iter()
            .all(|word| !has(data::CLAUSE_SUBJECTS, word))
}

/// Whether an introductory "it" belongs to an impersonal question such as "is it ok to run".
pub(crate) fn placeholder_it(line: &Line) -> bool {
    let said = line.norm.join(" ");
    let Some(frame) = PLACEHOLDER_IT.find(&said) else {
        return false;
    };
    let rest = &said[frame.end()..];
    if NAMES_A_THING.is_match(rest)
        || rest
            .split_whitespace()
            .any(|word| has(data::STANDS_FOR_SOMETHING, word))
    {
        return false;
    }
    !rest
        .split_whitespace()
        .next_back()
        .is_some_and(|word| has(data::LEAVES_SOMETHING_OUT, word))
}

/// Resolve pronouns using the current turn's independent topic and the retained prior reading.
pub(crate) fn resolve(
    line: &Line,
    kind: Kind,
    entities: &[Entity],
    own_topic: Option<&str>,
    subject_topic: Option<&str>,
    previous: Option<Prior<'_>>,
) -> References {
    let Some(prior) = previous else {
        return References::default();
    };

    let used = PRONOUNS.map(|aliases| line.norm.iter().any(|word| has(aliases, word)));
    let impersonal = used[1..].iter().all(|used| !used) && placeholder_it(line);
    let reference = !impersonal && used.iter().any(|used| *used);
    let opening = line.norm[..line.norm.len().min(4)].join(" ");
    let open_followup = !impersonal
        && (data::FOLLOWUP_STARTS.iter().any(|phrase| {
            &opening == phrase
                || opening
                    .strip_prefix(phrase)
                    .is_some_and(|rest| rest.starts_with(' '))
        }) || only_purpose(&line.norm));
    let short = line.norm.len() <= 3 && !matches!(kind, Kind::Greeting | Kind::Thanks);
    let asks_without_topic = kind == Kind::Question && subject_topic.is_none();
    let implicit = open_followup || short || asks_without_topic;
    let (target, target_kind) = target(&prior);
    let person = if reference {
        first_kind(prior.entities, &["name", "title", "person"])
            .map(|entity| entity.text.to_lowercase())
            .or_else(|| target.clone())
    } else {
        None
    };
    let target = target.filter(|_| reference);
    let there = first_kind(prior.entities, &["place"])
        .map(|entity| entity.text.to_lowercase())
        .or_else(|| {
            prior
                .topic
                .filter(|topic| *topic == "geography-and-travel")
                .map(|topic| topic.replace('-', " "))
        });
    let mut resolved = References {
        it: target.clone(),
        that: target.clone(),
        they: target,
        there,
        he: person.clone(),
        she: person,
        kind: target_kind,
        followup: reference || implicit,
    };

    let moved = matches!((own_topic, prior.topic), (Some(own), Some(last)) if !own.is_empty() && !last.is_empty() && own != last);
    let mut named = [false; 4];
    for (index, aliases) in PRONOUNS[..4].iter().enumerate() {
        named[index] = named_before(line, entities, aliases, moved);
        if !named[index] {
            continue;
        }
        match index {
            0 => resolved.it = None,
            1 => resolved.that = None,
            2 => resolved.they = None,
            _ => resolved.there = None,
        }
        if matches!(index, 1 | 2) {
            resolved.he = None;
            resolved.she = None;
        }
    }
    if used[..4].iter().any(|used| *used) && used[..4] == named && !(used[4] || used[5]) {
        resolved.followup = implicit;
    }
    resolved
}

fn animal_from_breed(text: &str) -> Option<String> {
    let text = text.to_lowercase();
    if let Some(animal) = breeds::exact_kind(text.trim()) {
        return Some(animal.into());
    }
    let words: Vec<_> = WORD
        .find_iter(&text)
        .map(|word| word.as_str().to_owned())
        .collect();
    let expanded = breeds::expand(words.clone());
    if expanded.len() <= words.len() {
        return None;
    }
    let mut original = words.iter().peekable();
    for piece in expanded {
        if original.peek().is_some_and(|word| **word == piece) {
            original.next();
        } else {
            return Some(piece);
        }
    }
    None
}

fn bare_words(words: &[String]) -> BTreeSet<String> {
    words
        .iter()
        .map(|word| {
            word.to_lowercase()
                .trim_matches(|ch: char| ch.is_ascii_punctuation() || ch == '’')
                .split('\'')
                .next()
                .unwrap_or("")
                .to_owned()
        })
        .collect()
}

fn read_animal(reference: &References, bare: &BTreeSet<String>) -> Option<String> {
    if !reference.followup || data::OWN_SUBJECT.iter().any(|word| bare.contains(*word)) {
        return None;
    }
    let links = [
        (data::IT_WORDS, &[][..], reference.it.as_deref()),
        (data::THAT_WORDS, &[][..], reference.that.as_deref()),
        (
            data::THEY_WORDS,
            data::THEY_APOSTROPHES,
            reference.they.as_deref(),
        ),
        (data::THERE_WORDS, &[][..], reference.there.as_deref()),
        (
            data::HE_WORDS,
            data::HE_APOSTROPHES,
            reference.he.as_deref(),
        ),
        (
            data::SHE_WORDS,
            data::SHE_APOSTROPHES,
            reference.she.as_deref(),
        ),
    ];
    for (pronouns, contractions, read) in links {
        if let Some(read) = read
            && pronouns
                .iter()
                .chain(contractions)
                .any(|word| bare.contains(*word))
        {
            if let Some(animal) = ANIMALS_IN_HAND
                .captures(read)
                .and_then(|found| found.get(0))
            {
                return Some(animal.as_str().to_lowercase());
            }
            if let Some(animal) = animal_from_breed(read) {
                return Some(animal);
            }
        }
    }
    None
}

/// Return an animal referred to by this turn, using the retained animal for a bare pronoun.
pub(crate) fn animal_in_hand(
    reference: &References,
    words: &[String],
    prior_animal: Option<&str>,
) -> Option<String> {
    if !reference.followup && prior_animal.is_none() {
        return None;
    }
    let expanded = breeds::expand(words.to_vec());
    let bare = bare_words(&expanded);
    if data::ANIMAL_NAMES.iter().any(|word| bare.contains(*word)) {
        return None;
    }
    read_animal(reference, &bare).or_else(|| {
        let animal = prior_animal.filter(|animal| !animal.is_empty())?;
        if data::MINE.iter().any(|word| bare.contains(*word))
            || !data::THIRD_PERSON.iter().any(|word| bare.contains(*word))
        {
            None
        } else {
            Some(animal.to_lowercase())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{Kind, Prior, References, animal_in_hand, person_is_new, placeholder_it, resolve};
    use crate::bot::understanding::{entities::Entity, line::normalise};

    fn entity(text: &str, kind: &'static str, at: usize) -> Entity {
        Entity {
            text: text.into(),
            kind,
            at,
        }
    }

    #[test]
    fn retains_adjacent_things_and_person_separately() {
        let earlier = [
            entity("Birthday", "thing", 2),
            entity("cake", "thing", 3),
            entity("Son", "person", 6),
        ];
        let current = normalise("can i freeze it?");
        let result = resolve(
            &current,
            Kind::Question,
            &[],
            Some("food-and-cooking"),
            Some("food-and-cooking"),
            Some(Prior {
                entities: &earlier,
                topic: Some("food-and-cooking"),
            }),
        );
        assert_eq!(result.it.as_deref(), Some("birthday cake"));
        assert_eq!(result.he.as_deref(), Some("son"));
        assert_eq!(result.kind, Some("thing"));
        assert!(result.followup);
        assert_eq!(
            resolve(&current, Kind::Question, &[], None, None, None),
            References::default()
        );
    }

    #[test]
    fn a_new_clause_can_own_its_pronoun() {
        let earlier = [entity("Sofa", "thing", 0)];
        let prior = || Prior {
            entities: &earlier,
            topic: Some("pets-and-animals"),
        };
        let current = normalise("i started a new job and i hate it");
        let result = resolve(
            &current,
            Kind::Statement,
            &[],
            Some("work-and-careers"),
            Some("work-and-careers"),
            Some(prior()),
        );
        assert_eq!(result.it, None);
        assert!(!result.followup);

        let current = normalise("i spilled red wine on it");
        let result = resolve(
            &current,
            Kind::Statement,
            &[],
            Some("house-and-home"),
            Some("house-and-home"),
            Some(prior()),
        );
        assert_eq!(result.it.as_deref(), Some("sofa"));
        assert!(result.followup);
    }

    #[test]
    fn an_impersonal_question_does_not_reuse_the_prior_target() {
        let line = normalise("is it ok to run every day");
        assert!(placeholder_it(&line));
        assert!(!placeholder_it(&normalise(
            "is it safe to drink the tap water"
        )));
        let result = resolve(
            &line,
            Kind::Question,
            &[],
            Some("health-and-fitness"),
            Some("health-and-fitness"),
            Some(Prior {
                entities: &[],
                topic: Some("food-and-cooking"),
            }),
        );
        assert_eq!(result.it, None);
        assert!(!result.followup);
    }

    #[test]
    fn an_animal_can_follow_the_pronoun_until_a_new_subject_arrives() {
        let earlier = [entity("greyhound", "thing", 1)];
        let current = normalise("she has been drinking more");
        let result = resolve(
            &current,
            Kind::Statement,
            &[],
            Some("pets-and-animals"),
            Some("pets-and-animals"),
            Some(Prior {
                entities: &earlier,
                topic: Some("pets-and-animals"),
            }),
        );
        assert_eq!(
            animal_in_hand(&result, &current.words, None).as_deref(),
            Some("dog")
        );
        let current = normalise("my cat is happy today");
        assert_eq!(animal_in_hand(&result, &current.words, Some("dog")), None);
        assert!(person_is_new(
            &normalise("should i text him"),
            &[],
            &earlier
        ));
    }
}
