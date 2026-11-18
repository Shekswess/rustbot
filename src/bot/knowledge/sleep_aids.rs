//! Distinguish personal sleep aids from noise coming from outside the room.

use regex::Regex;
use std::sync::LazyLock;

use super::sleep_aid_data as data;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(SCREEN, data::SCREEN);
pattern!(SOUND, data::SOUND);
pattern!(LIGHT, data::LIGHT);
pattern!(TELLY, data::TELLY);
pattern!(SLEEP, data::SLEEP);
pattern!(DEPENDS, data::DEPENDS);
pattern!(COMPLAINT, data::COMPLAINT);
pattern!(NOISE, data::NOISE);
pattern!(OUTSIDE, data::OUTSIDE);
pattern!(SOMEONE_ELSES, data::SOMEONE_ELSES);
pattern!(WALL, data::WALL);
pattern!(THROUGH_WALL, data::THROUGH_WALL);

#[derive(Clone, Copy, Debug)]
pub(super) enum Shape {
    NeedsAid,
    NeedsTelly,
    OutsideNoise,
    ThroughWall,
}

fn their_own_aid(text: &str) -> bool {
    !SOMEONE_ELSES.is_match(text)
        && SLEEP.is_match(text)
        && DEPENDS.is_match(text)
        && COMPLAINT.is_match(text)
}

fn which(text: &str) -> Option<&'static str> {
    if !their_own_aid(text) {
        return None;
    }
    [
        (&*TELLY, data::TELLY_REPLY),
        (&*SCREEN, data::SCREEN_REPLY),
        (&*SOUND, data::SOUND_REPLY),
        (&*LIGHT, data::LIGHT_REPLY),
    ]
    .into_iter()
    .find_map(|(aid, reply)| aid.is_match(text).then_some(reply))
}

fn outside_noise(text: &str) -> bool {
    SLEEP.is_match(text)
        && COMPLAINT.is_match(text)
        && NOISE.is_match(text)
        && OUTSIDE.is_match(text)
}

fn noise_through_wall(text: &str) -> bool {
    WALL.is_match(text)
        && SLEEP.is_match(text)
        && COMPLAINT.is_match(text)
        && (TELLY.is_match(text)
            || SCREEN.is_match(text)
            || SOUND.is_match(text)
            || THROUGH_WALL.is_match(text))
}

impl Shape {
    pub(super) fn passes(self, text: &str) -> bool {
        match self {
            Self::NeedsAid => which(text).is_some(),
            Self::NeedsTelly => TELLY.is_match(text) && their_own_aid(text),
            Self::OutsideNoise => outside_noise(text) && which(text).is_none(),
            Self::ThroughWall => noise_through_wall(text),
        }
    }
}

pub(super) fn answer(text: &str) -> Option<&'static str> {
    which(text)
        .or_else(|| noise_through_wall(text).then_some(data::WALL_REPLY))
        .or_else(|| outside_noise(text).then_some(data::NOISE_REPLY))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_telly_in_the_next_room_is_a_noise_complaint() {
        let neighbour = "I can't sleep because the telly is on in the next room";
        let owned = "I can't sleep unless the telly is on";
        assert!(Shape::ThroughWall.passes(neighbour));
        assert!(!Shape::NeedsTelly.passes(neighbour));
        assert_eq!(answer(neighbour), Some(data::WALL_REPLY));
        assert_eq!(answer(owned), Some(data::TELLY_REPLY));
    }
}
