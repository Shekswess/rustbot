//! Household members and whether a person mentioned in the conversation lives alone.

use crate::bot::{Exchange, Memory, Reading, compose};
use fancy_regex::{Captures, Regex};
use std::collections::BTreeMap;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "household_counts.rs"]
mod counts;
#[path = "household_data.rs"]
mod data;
use data::*;

#[derive(Default)]
pub(crate) struct Facts {
    own: Option<Home>,
    others: BTreeMap<String, bool>,
    last_ask: Option<Pending>,
    counts: counts::Counts,
}

enum Home {
    Alone,
    With(Vec<String>),
}

#[derive(Clone)]
struct Person {
    spoken: String,
    relation: Option<String>,
}

impl Person {
    fn relation(relation: &str) -> Self {
        Self {
            spoken: format!("your {relation}"),
            relation: Some(relation.into()),
        }
    }

    fn object(&self) -> &str {
        if self.relation.is_some() {
            return &self.spoken;
        }
        match self.spoken.as_str() {
            "she" => "her",
            "he" => "him",
            "they" => "them",
            _ => &self.spoken,
        }
    }
}

enum Pending {
    Alone(Person),
    Who,
}

enum Update {
    Ask(Pending),
    Answer(Option<String>, bool),
}

struct Proposal {
    reply: String,
    update: Option<Update>,
}

pub(super) fn flatten(text: &str) -> String {
    text.to_lowercase()
        .split(crate::text::is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn clean(text: &str) -> String {
    flatten(text).trim_matches([' ', '.', '!', ',']).to_owned()
}

pub(super) fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

pub(super) fn starts(pattern: &Regex, text: &str) -> bool {
    pattern
        .find(text)
        .ok()
        .flatten()
        .is_some_and(|found| found.start() == 0)
}

fn fields<'a>(pattern: &Regex, text: &'a str) -> Option<Captures<'a>> {
    pattern.captures(text).ok().flatten()
}

fn part<'a>(found: &Captures<'a>, name: &str) -> Option<&'a str> {
    Some(found.name(name)?.as_str())
}

fn who_said(text: &str) -> String {
    format!("your {}", flatten(text))
}

impl Facts {
    pub(crate) fn observe(&mut self, raw: &str, turns: &[Exchange]) {
        self.counts.observe(&flatten(raw), turns);
        let said = clean(raw);
        for found in THEIR_ALONE_SAID.captures_iter(&said).flatten() {
            if let Some(who) = part(&found, "who") {
                self.others.insert(who.to_owned(), true);
            }
        }
        if matches(&ON_MY_OWN, &said) {
            self.own = Some(Home::Alone);
        } else if let Some(found) = fields(&JUST_ME_AND, &said) {
            if let Some(who) = part(&found, "who") {
                let mut people = vec![who_said(who)];
                if let Some(second) = part(&found, "more").map(who_said)
                    && !people.contains(&second)
                {
                    people.push(second);
                }
                self.own = Some(Home::With(people));
            }
        } else if matches(&JUST_ME, &said) {
            self.own = Some(Home::Alone);
        }
    }
}

fn who_asked(pronoun: &str, memory: &Memory) -> Person {
    let spoken = flatten(pronoun);
    if let Some(relation) = spoken.strip_prefix("your ") {
        return Person::relation(relation);
    }
    if let Some((relation, _, _)) = memory.personal.family_people().last() {
        return Person::relation(relation);
    }
    if memory.household.others.len() == 1
        && let Some((relation, _)) = memory.household.others.first_key_value()
    {
        return Person::relation(relation);
    }
    Person {
        spoken,
        relation: None,
    }
}

fn asked_last(said: &str, memory: &Memory) -> Option<Person> {
    if !starts(&YES_AND_NO, said) {
        return None;
    }
    let last = memory.turns.last()?;
    if last.module == "household"
        && let Some(Pending::Alone(asked)) = &memory.household.last_ask
    {
        return Some(asked.clone());
    }
    let found = fields(&THE_ANSWER_ASKED, &last.bot)?;
    Some(who_asked(part(&found, "who")?, memory))
}

fn yes(said: &str) -> bool {
    !["no", "nope", "not"]
        .iter()
        .any(|word| said.starts_with(word))
        && ![
            " doesnt", " doesn't", " dont", " don't", " isnt", " isn't", " arent", " aren't",
        ]
        .iter()
        .any(|word| said.contains(word))
}

fn formatted(template: &str, who: &str) -> String {
    let mut letters = who.chars();
    let title: String = letters
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .chain(letters)
        .collect();
    template.replace("{who}", who).replace("{Who}", &title)
}

fn proposal(reading: &Reading<'_>, memory: &Memory) -> Option<Proposal> {
    let said = clean(reading.raw);
    if said.is_empty() {
        return None;
    }
    let pick = |pool| compose::pick(pool, memory, reading.raw);
    let plain = |reply| {
        Some(Proposal {
            reply,
            update: None,
        })
    };
    let update = |reply, update| {
        Some(Proposal {
            reply,
            update: Some(update),
        })
    };
    if let Some(asked) = asked_last(&said, memory) {
        let yes = yes(&said);
        let reply = formatted(
            pick(if yes { THEN_ALONE } else { THEN_NOT_ALONE }),
            asked.object(),
        );
        return update(reply, Update::Answer(asked.relation, yes));
    }
    if matches(&SHOULD_I_MORE, &said)
        && memory
            .turns
            .last()
            .is_some_and(|last| last.module == "household")
    {
        return plain(pick(SHOULD_MORE).to_owned());
    }
    if let Some(found) = fields(&THEIR_HOUSE_WHO, &said) {
        let person = part(&found, "who")
            .or_else(|| part(&found, "who2"))
            .map(|who| who_asked(who, memory));
        let who = person
            .as_ref()
            .map(Person::object)
            .or_else(|| part(&found, "who3"))?;
        return update(
            formatted(pick(THEIRS_NOT_SAID_WHO), who),
            Update::Ask(Pending::Who),
        );
    }
    if let Some(found) = fields(&THEIR_HOUSE_ASK, &said) {
        let person = who_asked(
            part(&found, "who").or_else(|| part(&found, "who2"))?,
            memory,
        );
        match person
            .relation
            .as_ref()
            .and_then(|role| memory.household.others.get(role))
        {
            Some(true) => return plain(formatted(pick(THEIRS_ALONE), &person.spoken)),
            Some(false) => return plain(formatted(pick(THEIRS_WITH), person.object())),
            None => {
                return update(
                    formatted(pick(THEIRS_NOT_SAID), &person.spoken),
                    Update::Ask(Pending::Alone(person)),
                );
            }
        }
    }
    if matches(&WHO_LIVES, &said) {
        return match &memory.household.own {
            Some(Home::Alone) => plain(pick(ALONE).to_owned()),
            Some(Home::With(people)) => {
                let (last, before) = people.split_last()?;
                let who = if before.is_empty() {
                    last.to_owned()
                } else {
                    format!("{} and {last}", before.join(", "))
                };
                plain(formatted(pick(WITH_SOMEONE), &who))
            }
            None if memory
                .household
                .counts
                .answer(&said, &memory.turns)
                .is_some() =>
            {
                None
            }
            None => plain(pick(NOT_SAID).to_owned()),
        };
    }
    let who = who_said(part(&fields(&JUST_ME_AND, &said)?, "who")?);
    plain(formatted(pick(THEIR_HOUSE), &who))
}

pub(crate) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    proposal(reading, memory).map(|proposal| proposal.reply)
}

pub(crate) fn count_reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    memory.household.counts.answer(reading.raw, &memory.turns)
}

pub(crate) fn claims(reading: &Reading<'_>, memory: &Memory) -> bool {
    let said = clean(reading.raw);
    if matches(&WHO_LIVES, &said) {
        return memory
            .household
            .counts
            .answer(&said, &memory.turns)
            .is_none();
    }
    matches(&THEIR_HOUSE_ASK, &said)
        || matches(&THEIR_HOUSE_WHO, &said)
        || asked_last(&said, memory).is_some()
        || (matches(&SHOULD_I_MORE, &said)
            && memory
                .turns
                .last()
                .is_some_and(|last| last.module == "household"))
        || matches(&JUST_ME_AND, &said)
}

pub(crate) fn record_selected(reading: &Reading<'_>, memory: &mut Memory) {
    match proposal(reading, memory).and_then(|proposal| proposal.update) {
        Some(Update::Ask(asked)) => memory.household.last_ask = Some(asked),
        Some(Update::Answer(relation, yes)) => {
            memory.household.last_ask = None;
            if let Some(relation) = relation {
                memory.household.others.insert(relation, yes);
            }
        }
        None => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn turn(memory: &mut Memory, line: &str) -> Option<String> {
        let reading = Reading::new(line);
        memory.household.observe(line, &memory.turns);
        let said = reply(&reading, memory);
        let module = if said.is_some() {
            "household"
        } else {
            "greeting"
        };
        if said.is_some() {
            assert!(claims(&reading, memory));
            record_selected(&reading, memory);
        }
        memory.turns.push(Exchange {
            user: line.into(),
            bot: said.clone().unwrap_or_else(|| "All right.".into()),
            module,
        });
        said
    }

    #[test]
    fn remembers_and_updates_the_household_and_leaves_places_to_other_readers() {
        let mut memory = Memory::default();
        for (line, expected) in [
            (
                "its just me and my daughter",
                Some("Your daughter and you, then — that's the household."),
            ),
            (
                "who lives with us",
                Some("Just you and your daughter, the way you told me."),
            ),
            (
                "me and my daughter and my son",
                Some("Your daughter and you, then — that's the household."),
            ),
            (
                "who lives with us",
                Some("You and your daughter and your son — that's who you've mentioned."),
            ),
            ("i live alone", None),
            ("who lives with us", Some("On your own, you told me.")),
            ("where do i live", None),
        ] {
            assert_eq!(turn(&mut memory, line).as_deref(), expected, "{line}");
        }
    }

    #[test]
    fn files_the_answers_to_recent_third_person_questions() {
        let mut memory = Memory::default();
        for (line, expected) in [
            (
                "does your mum live on her own",
                "I can't tell from here — but if your mum is living alone, a standing call or visit beats a one-off grand gesture.",
            ),
            (
                "yes she does",
                "Then a standing time to ring your mum is worth more than a long catch-up now and then — the same day each week, ten minutes, and it becomes a fixture rather than a good intention.",
            ),
            (
                "does your mum live on her own",
                "You told me — your mum lives alone.",
            ),
            (
                "should i ring more",
                "Little and often beats long and rare: a short call on the same day each week is easier to keep than a long one whenever there is time, and it is the one that gets counted on.",
            ),
            (
                "who does she live with",
                "I don't know — you haven't told me who lives with your mum.",
            ),
            (
                "is your dad on his own",
                "I don't know that one. If you're worried about your dad living alone, it's the small regular contact that helps most.",
            ),
            (
                "no he does not",
                "That is good — then it is the ordinary days that want the visits rather than the occasions, and a standing one is the easiest kind to keep.",
            ),
            (
                "is your dad on his own",
                "You said so yourself — someone lives with your dad.",
            ),
        ] {
            assert_eq!(turn(&mut memory, line).as_deref(), Some(expected), "{line}");
        }
        assert_eq!(memory.household.others.get("mum"), Some(&true));
        assert_eq!(memory.household.others.get("dad"), Some(&false));
    }

    #[test]
    fn a_group_count_is_corrected_and_left_to_the_store() {
        let mut memory = Memory::default();
        assert_eq!(turn(&mut memory, "theres 10 of us"), None);
        let ask = Reading::new("how many of us are there");
        assert!(!claims(&ask, &memory));
        assert_eq!(reply(&ask, &memory), None);
        assert_eq!(
            count_reply(&ask, &memory).as_deref(),
            Some("There are ten of you.")
        );
        turn(&mut memory, "actually 8 of us");
        let ask = Reading::new("how many of you are there");
        assert_eq!(reply(&ask, &memory), None);
        assert_eq!(
            count_reply(&ask, &memory).as_deref(),
            Some("There are eight of you.")
        );
    }

    #[test]
    fn the_headcount_keeps_the_turn_when_members_are_known_too() {
        let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
        bot.turn("there are three people here");
        bot.turn("me and my kids");
        let turn = bot.turn("how many of us are there");
        assert!(
            turn.candidates
                .iter()
                .any(|proposal| proposal.module == "household")
        );
        assert_eq!(turn.module, Some("memory_store"));
        assert_eq!(turn.reply, "There are three of you.");
    }
}
