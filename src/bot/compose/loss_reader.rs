//! Loss and end-of-life cues from the original conversation reader.

use crate::text::is_space;
use fancy_regex::{Captures, Match, Regex};

macro_rules! loss_pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "loss_reader_data.rs"]
mod data;
use data::*;
include!("loss_ownership_data.rs");

fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn has(table: &[&str], word: &str) -> bool {
    table.binary_search(&word).is_ok()
}

fn includes(words: &[&str], table: &[&str]) -> bool {
    words.iter().any(|word| has(table, word))
}

fn mentions(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| text.contains(phrase))
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(is_space)
        .filter(|word| !word.is_empty())
        .map(|word| word.trim_matches(['.', ',', '!', '?', ';', ':', '"', '\'', '(', ')']))
}

/// The person or pet whose loss the speaker reports as their own.
pub(in crate::bot) fn own_subject(text: &str) -> Option<(&str, bool)> {
    let stranger = matches(&A_STRANGERS_ANIMAL, text);
    if stranger
        || !matches(&CHILDS_OWN_LOSS, text)
            && [&*SOMEONE_ELSES, &*HIS_HERS, &*SOMEONE_ELSES_OWN]
                .iter()
                .any(|pattern| matches(pattern, text))
    {
        return None;
    }
    let line: Vec<_> = words(text).collect();
    if includes(&line, PET_WORDS) {
        for second in ["pig", "pigs"] {
            if line.contains(&"guinea") && line.contains(&second) {
                return Some((
                    if second == "pig" {
                        "guinea pig"
                    } else {
                        "guinea pigs"
                    },
                    true,
                ));
            }
        }
        let animal_before_death = DEATH_WORD.find(text).ok().flatten().and_then(|hit| {
            words(&text[..hit.start()])
                .filter(|word| has(PET_WORDS, word))
                .last()
        });
        if let Some(animal) = animal_before_death
            .or_else(|| line.iter().rev().copied().find(|word| has(PET_WORDS, word)))
        {
            return Some((animal, true));
        }
    }
    line.into_iter().find_map(|word| {
        let word = word
            .strip_suffix('s')
            .filter(|singular| has(PET_WORDS, singular))
            .unwrap_or(word);
        let pet = has(PET_WORDS, word);
        (pet || has(PEOPLE, word)).then_some((word, pet))
    })
}

fn capture<'a>(pattern: &Regex, text: &'a str, group: usize) -> Option<(Match<'a>, &'a str)> {
    let found = pattern.captures(text).ok().flatten()?;
    Some((found.get(0)?, found.get(group)?.as_str()))
}

fn matches_from(pattern: &Regex, text: &str, start: usize) -> bool {
    pattern
        .find_from_pos(text, start)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == start)
}

fn loose_person(words: &[&str], clean: &str) -> bool {
    includes(words, UNNAMED) && !includes(words, MEDIA) && matches(&ASK_WORDS, clean)
}

fn person(words: &[&str]) -> bool {
    words.iter().any(|word| {
        has(PEOPLE, word)
            || word
                .strip_suffix("'s")
                .is_some_and(|stem| has(PEOPLE, stem))
            || word.strip_suffix('s').is_some_and(|stem| has(PEOPLE, stem))
    }) || words
        .windows(2)
        .any(|pair| PEOPLE_PHRASES.contains(&(pair[0], pair[1])))
        || matches(&ROLE_PHRASE, &words.join(" "))
}

fn personal_object(word: &str) -> bool {
    has(OBJECTS, word) || has(THEIRS, word)
}

fn thing_of_theirs(words: &[&str]) -> bool {
    words.windows(2).enumerate().any(|(at, window)| {
        let word = window[0];
        let stem = word
            .strip_suffix("'s")
            .or_else(|| word.strip_suffix('s'))
            .unwrap_or(word);
        personal_object(window[1])
            && (has(PEOPLE, word) || has(PEOPLE, stem))
            && !words.iter().enumerate().any(|(other, word)| {
                at != other
                    && (has(PEOPLE, word)
                        || word.len() > 2
                            && word.strip_suffix('s').is_some_and(|stem| has(PEOPLE, stem)))
            })
    })
}

fn object_died(words: &[&str]) -> bool {
    words.iter().enumerate().any(|(at, &word)| {
        at > 0
            && death_word(word)
            && (has(OBJECTS, words[at - 1])
                || at > 1
                    && has(OBJECTS, words[at - 2])
                    && matches!(words[at - 1], "is" | "was" | "has" | "had"))
    })
}

fn death_word(word: &str) -> bool {
    matches!(word, "died" | "dies" | "dead" | "dying")
}

fn dead_is_a_death(following: Option<&&str>) -> bool {
    following.is_none_or(|word| !has(DEAD_IDIOM, word) && has(DEAD_PREDICATE, word))
}

fn says_question(line: &str) -> bool {
    let tight = words(line)
        .filter(|word| !has(FILLERS, word))
        .collect::<Vec<_>>()
        .join(" ");
    line.contains('?')
        || matches(&ASK_FORM, &tight)
        || [
            "what", "how", "should", "can", "do", "does", "is", "why", "when", "where",
        ]
        .iter()
        .any(|lead| line.starts_with(lead))
}

fn only_proceedings(clean: &str, named: bool, unnamed: bool) -> bool {
    !named
        && !unnamed
        && !matches(&A_DEATH_HAPPENED, clean)
        && !matches(&THIS_ONE, clean)
        && !matches(&ASK_WORDS, clean)
        && !matches(&CARD_WORD, clean)
        && (matches(&A_PRICE, clean) || matches(&PROCEEDINGS_ASK, clean))
}

fn missed_day(line: &[&str], named: bool) -> bool {
    let tight = line
        .iter()
        .copied()
        .filter(|word| !has(FILLERS, word))
        .collect::<Vec<_>>()
        .join(" ");
    if matches(&WOULD_HAVE, &tight) {
        return named || includes(line, PRONOUN);
    }
    matches(&OCCASION, &tight) && matches(&WITHOUT, &tight)
}

fn lost_forms(clean: &str) -> impl Iterator<Item = (Match<'_>, &str)> {
    [(&*LOST_FORM_1, 2), (&*LOST_FORM_2, 1), (&*LOST_FORM_3, 1)]
        .into_iter()
        .filter_map(move |(pattern, group)| capture(pattern, clean, group))
}

fn lost_in_place_after(clean: &str, start: usize) -> bool {
    LOST_IN_A_PLACE
        .captures_from_pos(clean, start)
        .ok()
        .flatten()
        .and_then(|capture| capture.get(1))
        .is_some_and(|place| has(PLACES, place.as_str()))
}

fn feared_not_done(clean: &str, start: usize) -> bool {
    FEARED_NOT_YET.find_iter(clean).flatten().any(|frame| {
        frame.start() <= start
            && frame
                .end()
                .checked_sub(start)
                .is_some_and(|tail| (4..=6).contains(&tail))
    })
}

pub(super) fn names_a_mouse_device(clean: &str) -> bool {
    let said = clean.replace('\'', "");
    let words: Vec<_> = said
        .split(|letter: char| !letter.is_ascii_lowercase())
        .collect();
    (words.contains(&"mouse") || words.contains(&"mousepad"))
        && !words.contains(&"mice")
        && !mentions(&said, MACHINE_WORD_PHRASES)
        && !includes(&words, MOUSE_ANIMAL_WORDS)
}

fn group_lost(clean: &str, device: bool) -> bool {
    let deathable = |word| has(DEATHABLE, word) && !(device && has(MACHINE_ANIMALS, word));
    if capture(&LOST_THE, clean, 1).is_some_and(|(_, who)| {
        deathable(who) && !has(PET_WORDS, who) && !matches(&LOST_THE_FOUND, clean)
    }) {
        return true;
    }
    if lost_forms(clean).any(|(found, who)| {
        deathable(who)
            && !clean[found.end()..]
                .split_whitespace()
                .next()
                .is_some_and(personal_object)
            && !lost_in_place_after(clean, found.end())
            && !matches_from(&SHED, clean, found.end())
            && !feared_not_done(clean, found.start())
    }) {
        return true;
    }
    if capture(&LOSS_OF, clean, 1)
        .is_some_and(|(found, who)| deathable(who) && !lost_in_place_after(clean, found.end()))
        || capture(&NEWS_OF, clean, 1).is_some_and(|(_, who)| deathable(who))
    {
        return true;
    }
    capture(&LOSING, clean, 1).is_some_and(|(found, who)| {
        deathable(who)
            && (has(CLOSE, who) || matches(&LOSS_GRIEF_CUE, clean))
            && !matches_from(&SHED, clean, found.end())
            && !feared_not_done(clean, found.start())
            && !lost_in_place_after(clean, found.end())
    })
}

fn spelling(clean: &str) -> std::borrow::Cow<'_, str> {
    CONTRACTED.replace_all(clean, |capture: &Captures<'_>| {
        let base = &capture[1];
        let lowered = base.to_lowercase();
        if has(CAN_BE_DYING, &lowered) && !has(NOT_A_CONTRACTION, &format!("{lowered}s")) {
            format!("{base} is ")
        } else {
            capture[0].to_owned()
        }
    })
}

fn killed_in_story(clean: &str) -> bool {
    if matches(&GAME_TITLES, clean) {
        return true;
    }
    KILLED_IN.find(clean).ok().flatten().is_some_and(|found| {
        clean[found.end()..]
            .split_whitespace()
            .take(4)
            .find(|word| has(STORY_INCIDENT, word) || has(STORY_MEDIUM, word))
            .is_some_and(|word| !has(STORY_INCIDENT, word))
    })
}

fn buried_person(clean: &str, line: &[&str], named: bool) -> bool {
    if !matches(&BURIED, clean) {
        return false;
    }
    let Some(at) = line
        .iter()
        .position(|word| matches!(*word, "buried" | "bury"))
    else {
        return false;
    };
    if at > 0 && has(PASSIVE_AUX, line[at - 1]) {
        return named
            || includes(line, PET_WORDS)
            || includes(line, REMAINS)
            || !includes(line, BURIABLE);
    }
    let mut rest = line
        .iter()
        .skip(at + 1)
        .take(4)
        .copied()
        .skip_while(|word| has(BURIED_ADJECTIVE, word));
    if rest
        .next()
        .is_some_and(|head| has(BURIED_OBJECT, head) && !matches!(head, "him" | "her" | "them"))
    {
        return rest
            .find(|word| !has(BURIED_ADJECTIVE, word))
            .is_some_and(|who| has(PEOPLE, who));
    }
    true
}

fn put_down(clean: &str, line: &[&str]) -> bool {
    let loss_context = mentions(clean, LOSS_CONTEXT);
    matches(&PUT_DOWN, clean)
        && (!mentions(clean, BEDTIME) || loss_context)
        && (includes(line, PET_WORDS) || loss_context)
}

fn accident_death(clean: &str) -> bool {
    matches(&MISADVENTURE, clean)
        && !matches(&SURVIVED, clean)
        && !matches(&NOT_A_DEATH, clean)
        && !matches(&VERB_OBJECT, clean)
        && OWNED
            .captures_iter(clean)
            .flatten()
            .filter_map(|found| found.get(1))
            .flat_map(|phrase| phrase.as_str().split_whitespace())
            .find(|word| has(DEATHABLE, word))
            .is_some_and(|victim| {
                has(PET_WORDS, victim)
                    || !matches(&KNOCKED_BY, clean)
                    || matches(&DEATH_SAID, clean)
            })
}

fn starts_with_digit(word: &str) -> bool {
    word.chars().next().is_some_and(char::is_numeric)
}

fn passed_on(clean: &str, named: bool) -> bool {
    let Some((before, after)) = clean.split_once("passed on") else {
        return false;
    };
    if !named
        && !before
            .split_whitespace()
            .rev()
            .take(3)
            .any(|word| has(PASSED_ON_SUBJECT, word))
    {
        return false;
    }
    let mut rest = after.split_whitespace();
    rest.next().is_none_or(|head| {
        !has(PASSED_ON_OBJECT, head)
            || head == "the"
                && rest.next().is_some_and(|date| {
                    has(MONTHS, date)
                        || starts_with_digit(date)
                        || matches!(date, "1st" | "2nd" | "3rd")
                })
    })
}

fn passed_alone(clean: &str, named: bool) -> bool {
    if !clean.contains("passed") {
        return false;
    }
    let line: Vec<_> = clean
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect();
    line.iter()
        .enumerate()
        .filter(|(_, word)| **word == "passed")
        .any(|(at, _)| {
            if !named && !includes(&line[at.saturating_sub(4)..at], PASSED_ON_SUBJECT)
                || includes(&line[..at], PASSED_A_RESULT)
            {
                return false;
            }
            let rest = &line[at + 1..];
            let Some(&head) = rest.first() else {
                return true;
            };
            let next = rest.get(1).copied().unwrap_or("");
            if has(PASSED_ALONE_TAIL, head) {
                return false;
            }
            if head == "at" {
                return has(PASSED_TIME, next);
            }
            if matches!(head, "in" | "on" | "last" | "this" | "that" | "back") {
                return has(PASSED_DATE, next) || has(PASSED_TIME, next) || starts_with_digit(next);
            }
            head == "it" && matches!(next, "was" | "is" | "has" | "had")
                || !has(PASSED_ON_OBJECT, head)
                    && !has(PASSED_ALONE_HANDED, head)
                    && !has(PASSED_ALONE_NOUN, head)
        })
}

fn death_noun(clean: &str, named: bool) -> bool {
    if !matches(&DEATH_NOUN, clean)
        || matches(&NOT_A_DEATH, clean)
        || matches(&THING_DEATH, clean)
        || matches(&PET_DEATH, clean)
        || matches(&DEATH_COMPOUND, clean)
        || matches(&FICTION, clean) && !named
        || matches(&OWN_WORRY, clean)
    {
        return false;
    }
    named
        || OWN_DEATH.find(clean).ok().flatten().is_some_and(|found| {
            found
                .as_str()
                .split_whitespace()
                .next()
                .is_some_and(|owner| {
                    matches!(owner, "my" | "his" | "her" | "their" | "your" | "our")
                        || has(PEOPLE, owner)
                        || has(PEOPLE, owner.trim_end_matches('s').trim_end_matches('\''))
                })
        })
        || matches(&DEATH_CUE, clean)
}

fn death_as_a_thing(clean: &str, named: bool) -> bool {
    !matches(&OWN_WORRY, clean)
        && !matches(&THING_DEATH, clean)
        && !matches(&PET_DEATH, clean)
        && !matches(&DEATH_COMPOUND, clean)
        && (named
            || !matches(&FICTION, clean)
                && !capture(&DEATH_OF_OWNER, clean, 1).is_some_and(|(_, phrase)| {
                    phrase
                        .split_whitespace()
                        .next_back()
                        .is_some_and(|last| has(THINGS, last))
                }))
}

fn giving_up_on_somebody(clean: &str, third_person: bool) -> bool {
    let Some((_, subject)) = capture(&THE_GIVE_UP_NAMES, clean, 1) else {
        return !third_person;
    };
    if matches(&SERIOUS_CARE, subject) {
        return true;
    }
    let Some(last) = words(subject).last() else {
        return !third_person;
    };
    (!third_person || !matches!(last, "me" | "us" | "i" | "mine" | "ours"))
        && (has(PEOPLE_AFTER_A_GIVE_UP, last) || person(&[last]))
}

fn palliative_statement(clean: &str) -> bool {
    matches(&PALLIATIVE_ITSELF, clean)
        && !says_question(clean)
        && !matches(&WORKING_WITH_THE_DYING, clean)
}

fn family_is_told(clean: &str, serious: bool) -> bool {
    if serious && (matches(&NOT_WORKING, clean) || matches(&PREPARE, clean)) {
        return !matches(&PREPARE_OTHERWISE, clean);
    }
    if palliative_statement(clean) {
        return true;
    }
    if matches(&COULD_BE_DAYS, clean) && matches(&TOLD_US, clean) {
        return !matches(&DAYS_WAIT_FOR, clean);
    }
    matches(&OFF_LIFE_SUPPORT, clean) && !matches(&OFF_SUPPORT_WELL, clean)
        || matches(&HOSPICE_CARE, clean) && !matches(&HOSPICE_JOB, clean)
}

fn end_is_near(clean: &str, line: &[&str], named: bool) -> bool {
    let care_worker = matches(&CARE_WHO, clean);
    let no_more = matches(&NOTHING_MORE_TO_DO, clean);
    let serious = matches(&SERIOUS_CARE, clean);
    if care_worker
        && (no_more
            || matches(&TREATMENT_STOPPED, clean)
            || serious && matches(&NOT_WORKING, clean))
        && (!no_more || giving_up_on_somebody(clean, false))
        || !care_worker
            && no_more
            && matches(&SAID_IT, clean)
            && !matches(&NOT_A_CARE_GIVER, clean)
            && giving_up_on_somebody(clean, true)
        || serious && matches(&MEDICATION_STOPPED, clean)
        || matches(&ON_THE_LAST_DRUG, clean) && !matches(&SELF_ON_THE_DRUG, clean)
        || matches(&FINAL_DAYS, clean)
    {
        return true;
    }
    let pet = includes(line, PET_WORDS);
    let somebody = named
        || pet
        || line.contains(&"she")
        || line.contains(&"he")
        || includes(line, HUMAN_WORDS);
    if !somebody && !care_worker {
        return family_is_told(clean, serious);
    }
    if !matches(&NOT_THE_END, clean) {
        if matches(&DAYS_LEFT, clean) {
            return true;
        }
        if matches(&COULD_BE_DAYS, clean) {
            return (care_worker || named || pet) && !matches(&DAYS_WAIT_FOR, clean);
        }
    }
    matches(&LAST_STAGES, clean)
        || matches(&NOT_GETTING_BETTER, clean)
        || care_worker
            && (matches(&GIVEN_UP_ON, clean)
                || matches(&PREPARE, clean) && !matches(&PREPARE_OTHERWISE, clean))
        || matches(&OFF_LIFE_SUPPORT, clean) && !matches(&OFF_SUPPORT_WELL, clean)
        || matches(&HOSPICE_CARE, clean) && !matches(&HOSPICE_JOB, clean)
        || palliative_statement(clean)
}

/// Whether the turn concerns a death or loss of a person or animal named in the text.
pub(crate) fn reads_as_loss(text: &str) -> bool {
    let expanded = text.to_lowercase().replace('’', "'");
    let line: Vec<_> = words(&expanded).collect();
    if line.is_empty() {
        return false;
    }
    let clean = line.join(" ");
    let clean = clean.as_str();
    if mentions(clean, LAUGHING)
        || lost_forms(clean)
            .find(|(_, who)| has(DEATHABLE, who))
            .is_some_and(|(found, _)| lost_in_place_after(clean, found.end()))
    {
        return false;
    }
    let named_person = person(&line);
    let unnamed = loose_person(&line, clean);
    let mut named = named_person || unnamed;
    if named && matches(&WOULD_HAVE_BEEN, clean) {
        return true;
    }
    let prey = includes(&line, PET_WORDS)
        && matches(&PREY, clean)
        && matches(&HUNTING, clean)
        && !matches(&PET_DIED, &PREY.replace_all(clean, " "));
    if prey || thing_of_theirs(&line) || matches(&IT_DIED, clean) && includes(&line, THINGS) {
        return false;
    }
    if end_is_near(clean, &line, named_person) {
        return true;
    }
    if only_proceedings(clean, named_person, unnamed) {
        return false;
    }
    if named
        && matches(&THING_ROLE, clean)
        && !line
            .iter()
            .any(|word| has(HUMAN_WORDS, word) && !has(TWO_WAY_ROLES, word))
        && !matches(&ROLE_PHRASE, clean)
    {
        named = false;
    }
    if matches(&PLAIN, clean)
        || passed_alone(clean, named)
        || passed_on(clean, named)
        || death_noun(clean, named)
        || matches(&WAKE, clean)
        || buried_person(clean, &line, named)
        || matches(&SYMPATHY, clean) && matches(&CARD_WORD, clean)
        || matches(&TOOK, clean)
        || matches(&KILLED, clean)
            && (named || includes(&line, PASSED_ON_SUBJECT))
            && !killed_in_story(clean)
        || put_down(clean, &line)
        || named
            && matches(&STILL_CRY, clean)
            && !matches(&OTHER_WEEPS, clean)
            && !matches(&THEIR_TEARS_IN_A_CLAUSE, clean)
    {
        return true;
    }
    if matches(&DEATH_AS_A_THING, clean) && !matches(&NOT_A_DEATH, clean) {
        return death_as_a_thing(clean, named);
    }
    if group_lost(clean, names_a_mouse_device(clean)) || missed_day(&line, named_person) {
        return true;
    }
    if matches(&CONDITIONAL_TAIL, clean) && !includes(&line, HUMAN_WORDS) {
        return false;
    }
    let spoken = spelling(clean);
    let pronoun = words(&spoken).any(|word| has(PEOPLE_PRONOUNS, word));
    let object = object_died(&line);
    !object
        && ((named || pronoun)
            && matches(&ANTICIPATED, &spoken)
            && (!matches(&WORKING_WITH_THE_DYING, clean) || matches(&SAID_DYING, clean))
            || !named && pronoun && matches(&DYING_NOW, &spoken) && !matches(&DYING_TO, clean)
            || (named || pronoun) && matches(&GOING_TO_DIE, clean))
        || accident_death(clean)
        || named
            && !object
            && (matches(&DYING, clean) || matches(&DIED, clean))
            && line.iter().enumerate().any(|(at, &word)| {
                death_word(word) && (word != "dead" || dead_is_a_death(line.get(at + 1)))
            })
}

/// Whether a turn reports a person or animal near death, with the death still ahead.
pub(crate) fn is_dying_now(text: &str) -> bool {
    let text = text.replace(['\u{001c}', '\u{001d}', '\u{001e}', '\u{001f}'], " ");
    let spoken = spelling(&text);
    let spoken = spoken.to_lowercase().replace('’', "'");
    let line: Vec<_> = words(&spoken).collect();
    let clean = line
        .iter()
        .copied()
        .filter(|word| !has(FILLERS, word))
        .collect::<Vec<_>>()
        .join(" ");
    let dying = matches(&DYING_NOW, &clean);
    let end_near = !dying && end_is_near(&clean, &line, person(&line));
    if !(dying || end_near || matches(&AT_THE_END, &clean))
        || matches(&DYING_ALREADY, &clean)
        || matches(&WORKING_WITH_THE_DYING, &clean) && !matches(&SAID_DYING, &clean)
    {
        return false;
    }
    end_near
        || family_is_told(&clean, matches(&SERIOUS_CARE, &clean))
        || includes(&line, HUMAN_WORDS)
        || includes(&line, PET_WORDS)
        || includes(&line, PEOPLE_PRONOUNS)
        || matches(&ROLE_PHRASE, &clean)
}

#[cfg(test)]
mod tests {
    use super::{is_dying_now, reads_as_loss};

    #[test]
    fn distinguishes_a_loss_from_ongoing_care_and_a_job() {
        assert!(reads_as_loss("my dad passed last year"));
        assert!(!is_dying_now("my dad passed last year"));
        assert!(reads_as_loss("my dad is dying"));
        assert!(is_dying_now("my dad is dying"));
        assert!(!is_dying_now("my mum is a palliative care nurse"));
        assert!(!reads_as_loss("my mum is a palliative care nurse"));
        assert!(is_dying_now("shes \u{001c}  dying"));
    }
}
