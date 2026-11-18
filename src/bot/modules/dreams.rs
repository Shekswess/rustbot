//! Keep dream scenes distinct from waking events and active emergencies.

use std::sync::LazyLock;

use regex::{Regex, RegexSet};

use crate::bot::{Memory, Reading};

#[path = "dreams_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));
    };
}

pattern!(DREAM);
pattern!(FIRST_PERSON);
pattern!(NAMED_A_NIGHTMARE);
pattern!(A_SCENE_OF_NOTHING);
pattern!(SETTLED_IT);
pattern!(DREAM_IS_HAPPENING_NOW);
pattern!(SOMEONE_DIED);
pattern!(A_FIRE);

static A_WISH: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(data::A_WISH).expect("dream wish pattern"));
static SCENE: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new([
        data::DIED_MYSELF,
        data::ATTACKED,
        data::ILL_IN_THE_DREAM,
        data::SOMEONE_DIED,
        data::TEETH,
        data::A_FIRE,
        data::A_BREAK_IN,
        data::A_BABY,
        data::AN_EX,
        data::A_VISIT,
        data::PEOPLE_HURT,
        data::CHASED_OR_FALLING,
        data::A_GOOD_ONE,
    ])
    .expect("ordered dream scenes")
});

fn normalize(text: &str) -> String {
    text.to_lowercase()
        .replace(['‘', '’'], "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn scene(text: &str) -> Option<&str> {
    if text.len() > 400 || NAMED_A_NIGHTMARE.is_match(text) {
        return None;
    }
    let verb = DREAM.find(text)?;
    let before = &text[..verb.start()];
    if !FIRST_PERSON.is_match(before) || SOMEONE_DIED.is_match(before) || A_FIRE.is_match(before) {
        return None;
    }
    let scene = text[verb.end()..].trim_matches([' ', ',', '.', ':', ';', '!', '-']);
    if !super::the_detail_the_thread_adds::heavy::only_a_dream(text)
        || A_WISH.is_match(text).unwrap_or(false)
        || scene.contains("one day")
        || A_SCENE_OF_NOTHING.is_match(scene)
    {
        return None;
    }
    Some(scene)
}

fn pick(line: &str, options: &[&str]) -> String {
    let total = line.chars().map(|c| c as usize).sum::<usize>();
    options[total % options.len()].to_owned()
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = normalize(reading.raw);
    let Some(scene) = scene(&line) else {
        if DREAM_IS_HAPPENING_NOW.is_match(&line) || !SETTLED_IT.is_match(&line) {
            return None;
        }
        let previous = memory.turns.last()?;
        let dream =
            previous.module == "dreams" || self::scene(&normalize(&previous.user)).is_some();
        return dream.then(|| pick(&line, data::SETTLED_LINES));
    };
    let matches = SCENE.matches(scene);
    let options = matches
        .iter()
        .find(|index| *index != 2 || !matches.matched(10))
        .map_or(data::PLAIN_LINES, |index| {
            [
                data::OWN_DEATH_LINES,
                data::ATTACK_LINES,
                data::ILL_LINES,
                data::DEATH_LINES,
                data::TEETH_LINES,
                data::FIRE_LINES,
                data::BREAK_IN_LINES,
                data::BABY_LINES,
                data::EX_LINES,
                data::VISIT_LINES,
                data::PEOPLE_HURT_LINES,
                data::CHASE_LINES,
                data::GOOD_LINES,
            ][index]
        });
    Some(pick(&line, options))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dreams_are_separate_from_waking_events_and_ordinary_definitions() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        assert_eq!(bot.turn("I dreamt my dad died").module, Some("dreams"));
        assert_eq!(
            bot.turn("He's fine, it was only a dream").module,
            Some("dreams")
        );
        for line in [
            "My dad died and I dreamt about him.",
            "I dreamt there was a fire and I can smell smoke.",
            "Why do we dream?",
            "My dream job is nursing",
        ] {
            assert!(
                respond(&Reading::new(line), &Memory::default()).is_none(),
                "{line}"
            );
        }
    }
}
