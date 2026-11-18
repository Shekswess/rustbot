//! Eligibility checks for the pinned cleaning advice cards.

use std::sync::LazyLock;

use regex::Regex;

use crate::{bot::Reading, text::is_space};

#[path = "cleaning_guard_data.rs"]
mod data;

use data::*;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(DUST, DUST_NOUN);
pattern!(ARRIVING, DUST_ARRIVING);
pattern!(ON_THING, DUST_THING);
pattern!(IN_PLACE, DUST_PLACE);
pattern!(NAMES_DRYER, DRYER);
pattern!(NAMES_WASHER, WASHER);
pattern!(GONE, HAS_GONE);

fn any_substring(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|word| text.contains(word))
}

fn the_line(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    format!("{} {} {}", line.clean, line.norm_text, line.plain).to_lowercase()
}

pub(super) fn windows_streak(reading: &Reading<'_>) -> bool {
    let said = reading.raw.to_lowercase();
    let wants_wiping = !any_substring(&said, WET_GLASS) || any_substring(&said, WIPE_GLASS);
    let mentions_window = any_substring(&said, WINDOWS);
    let describes_other_glass = [GLASSWARE, DRINKS, VESSELS]
        .iter()
        .any(|family| any_substring(&said, family));
    wants_wiping && (mentions_window || !describes_other_glass)
}

pub(super) fn two_steps_in_order(reading: &Reading<'_>) -> bool {
    let typed = if reading.raw.is_empty() {
        &reading.line.clean
    } else {
        reading.raw
    };
    let typed = typed.to_lowercase();
    !any_substring(&typed, OBJECTS_WITH_OWN_METHOD)
        && STEPS
            .iter()
            .filter(|step| typed.contains(*step))
            .nth(1)
            .is_some()
}

pub(super) fn colour_can_have_run(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let said = if !line.words.is_empty() {
        line.words.join(" ")
    } else if !line.clean.is_empty() {
        line.clean.clone()
    } else {
        reading.raw.to_owned()
    };
    let said = format!(" {} ", said.to_lowercase());
    !GONE.captures_iter(&said).any(|found| {
        found[1]
            .split(is_space)
            .map(|word| word.trim_matches(['\'', '.', ',', '!', '?']))
            .find(|word| !word.is_empty() && GONE_INTENSIFIER.binary_search(word).is_err())
            .is_none_or(|word| GONE_STATE.binary_search(&word).is_err())
    })
}

pub(super) fn not_the_dryer_alone(reading: &Reading<'_>) -> bool {
    let said = the_line(reading);
    !NAMES_DRYER.is_match(&said) || NAMES_WASHER.is_match(&said)
}

pub(super) fn no_room_specific_card(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    if line.words.is_empty() {
        return true;
    }
    let plain = if line.plain.is_empty() {
        reading.raw
    } else {
        &line.plain
    };
    let said = format!(" {} {plain} ", line.words.join(" ")).to_lowercase();
    !ROOMS_WITH_OWN_CARD.iter().any(|word| {
        if word.contains(' ') {
            said.contains(word)
        } else {
            line.words.iter().any(|typed| typed == word)
        }
    })
}

pub(super) fn house_dust(reading: &Reading<'_>) -> bool {
    let said = reading.line.clean.to_lowercase();
    !DUST.is_match(&said)
        || ARRIVING.is_match(&said)
        || ON_THING
            .captures_iter(&said)
            .all(|found| IN_PLACE.is_match(found[1].trim_matches(is_space)))
}

pub(super) fn not_the_back_of_a_screen(reading: &Reading<'_>) -> bool {
    if reading.line.words.is_empty() {
        return true;
    }
    let text = format!(" {} ", reading.line.words.join(" "));
    !text.contains(" back of ")
        || !reading
            .line
            .words
            .iter()
            .any(|typed| SCREENS.contains(&typed.as_str()))
}

pub(super) fn asks_for_dusting_method(reading: &Reading<'_>) -> bool {
    any_substring(&the_line(reading), DUST_METHOD_ASK)
}

pub(super) fn room_smell(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let said = if !line.plain.is_empty() {
        &line.plain
    } else if !line.clean.is_empty() {
        &line.clean
    } else {
        reading.raw
    };
    !super::guards::food_has_gone_off(&said.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn washing_a_window_requires_a_pane_or_glass_and_a_cleaning_question() {
        for (line, expected) in [
            ("my windows have condensation inside", false),
            ("how do i clean condensation off my windows", true),
            ("my glasses are streaky after cleaning", false),
            ("how do i remove streaks from a beer glass", false),
            ("my window glass is streaky", true),
            ("the glass door is fogging up how do i wipe it", true),
        ] {
            assert_eq!(windows_streak(&Reading::new(line)), expected, "{line}");
        }
    }

    #[test]
    fn household_dust_distinguishes_buildup_a_surface_and_an_asked_method() {
        assert!(house_dust(&Reading::new(
            "dust keeps coming back on my monitor"
        )));
        assert!(!house_dust(&Reading::new(
            "how do i get dust off my monitor"
        )));
        assert!(house_dust(&Reading::new("why is there dust under my bed")));
        assert!(!not_the_back_of_a_screen(&Reading::new(
            "dust the back of my tv"
        )));
        assert!(not_the_back_of_a_screen(&Reading::new("dust behind my tv")));
        assert!(asks_for_dusting_method(&Reading::new(
            "whats the best way to dust"
        )));
        assert!(!asks_for_dusting_method(&Reading::new("do i need to dust")));
    }

    #[test]
    fn laundry_and_housework_leave_specific_faults_to_their_own_cards() {
        assert!(!colour_can_have_run(&Reading::new(
            "my washing machine has gone"
        )));
        assert!(colour_can_have_run(&Reading::new(
            "my whites have gone all pink"
        )));
        assert!(!not_the_dryer_alone(&Reading::new(
            "my dryer lint filter is full of hair"
        )));
        assert!(not_the_dryer_alone(&Reading::new(
            "hair in my washer and dryer"
        )));
        assert!(!no_room_specific_card(&Reading::new(
            "how often should i clean windows"
        )));
        assert!(no_room_specific_card(&Reading::new(
            "how do i clean a whole house"
        )));
        assert!(!two_steps_in_order(&Reading::new(
            "should i wipe skirting before hoovering"
        )));
        assert!(two_steps_in_order(&Reading::new(
            "should i dust first or mop"
        )));
        assert!(!room_smell(&Reading::new(
            "my chicken smells bad can i cook it"
        )));
        assert!(room_smell(&Reading::new("my kitchen smells of cooking")));
    }
}
