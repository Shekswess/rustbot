//! Native guards and temperature branches from `bot/knowledge/clothing.py`.

use regex::Regex;
use std::sync::LazyLock;

use crate::bot::{Exchange, Reading};

use super::{
    CLOTHING_BOILING_PATTERN, CLOTHING_CHOICE_FRAME, CLOTHING_COLD_PATTERN, CLOTHING_COLOUR_CHOICE,
    CLOTHING_DRESSING_PATTERN, CLOTHING_FIT_USE, CLOTHING_GLASSES_PATTERN, CLOTHING_HAIR_PATTERN,
    CLOTHING_INTERVIEW_WORDS, CLOTHING_MEAL_WORDS, CLOTHING_OTHER_TOPICS, CLOTHING_PAN_PATTERN,
    CLOTHING_TEMPERATURE_PATTERN, CLOTHING_TIME_PATTERN, CLOTHING_ZIP_FAULT, CLOTHING_ZIP_FITTING,
    CLOTHING_ZIP_NEEDLE, guards::put_on_is_clothes,
};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(ZIP_FAULT, CLOTHING_ZIP_FAULT);
pattern!(ZIP_NEEDLE, CLOTHING_ZIP_NEEDLE);
pattern!(ZIP_FITTING, CLOTHING_ZIP_FITTING);
pattern!(HAIR, CLOTHING_HAIR_PATTERN);
pattern!(COLOUR_CHOICE, CLOTHING_COLOUR_CHOICE);
pattern!(CHOICE_FRAME, CLOTHING_CHOICE_FRAME);
pattern!(GLASSES, CLOTHING_GLASSES_PATTERN);
pattern!(GLASS, r"\b(?:my|a|the|this|that|another)\s+glass\b");
pattern!(GLASS_OF, r"^\s*of\b");
pattern!(COLD_AS_ILLNESS, CLOTHING_COLD_PATTERN);
pattern!(BOILING_FOOD, CLOTHING_BOILING_PATTERN);
pattern!(COOKING_TIME, CLOTHING_TIME_PATTERN);
pattern!(PAN, CLOTHING_PAN_PATTERN);
pattern!(DRESSING, CLOTHING_DRESSING_PATTERN);
pattern!(TEMPERATURE, CLOTHING_TEMPERATURE_PATTERN);

pub(super) fn zip_fault(reading: &Reading<'_>) -> bool {
    let said = format!(
        "{} {} {}",
        reading.folded(),
        reading.line.clean,
        reading.line.norm_text,
    );
    ZIP_FAULT.is_match(&said) || !(ZIP_NEEDLE.is_match(&said) || ZIP_FITTING.is_match(&said))
}

pub(super) fn date_night_meal(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    let said = words.join(" ");
    let said = if words.is_empty() {
        reading.folded()
    } else {
        said.as_str()
    };
    words
        .iter()
        .any(|word| CLOTHING_MEAL_WORDS.contains(&word.as_str()))
        || [
            "dinner ideas",
            "dinner for",
            "dinner at home",
            "dinner tonight",
            "dinner for two",
            "dinner to cook",
            "meal ideas",
        ]
        .iter()
        .any(|phrase| said.contains(phrase))
}

pub(super) fn mentions_interview(reading: &Reading<'_>) -> bool {
    reading
        .line
        .clean
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|word| CLOTHING_INTERVIEW_WORDS.contains(&word))
}

fn normalized_or_typed<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.norm_text.is_empty() {
        reading.folded()
    } else {
        &reading.line.norm_text
    }
}

pub(super) fn hair_line(reading: &Reading<'_>) -> bool {
    HAIR.is_match(normalized_or_typed(reading))
}

pub(super) fn chooses_colour(reading: &Reading<'_>) -> bool {
    let said = normalized_or_typed(reading);
    COLOUR_CHOICE.is_match(said) && CHOICE_FRAME.is_match(said)
}

pub(super) fn spectacles(reading: &Reading<'_>) -> bool {
    let said = format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.norm_text,
        reading.folded(),
    );
    GLASSES.is_match(&said)
        || !GLASS
            .find_iter(&said)
            .any(|matched| !GLASS_OF.is_match(&said[matched.end()..]))
}

pub(super) fn wear_for_weather(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    if !put_on_is_clothes(words) {
        return false;
    }
    let said = words.join(" ");
    if COLD_AS_ILLNESS.is_match(&said)
        || BOILING_FOOD.is_match(&said)
        || (COOKING_TIME.is_match(&said) && PAN.is_match(&said))
    {
        return false;
    }
    reading
        .topic
        .is_none_or(|(topic, _)| !CLOTHING_OTHER_TOPICS.contains(&topic))
        || DRESSING.is_match(&said)
}

pub(super) fn temperature(reading: &Reading<'_>) -> Option<i32> {
    TEMPERATURE.captures(reading.folded())?[1].parse().ok()
}

pub(super) fn dress_for_temperature(reading: &Reading<'_>) -> Option<&'static str> {
    let degrees = temperature(reading)?;
    Some(if degrees >= 24 {
        "In real heat, loose light cotton in a pale colour is the coolest thing you can wear, because it lets air through and reflects the sun. A wide hat and sunglasses are worth more than another coat of sunscreen, and shoes you can walk in matter on hot tarmac. Dark, tight and synthetic is what makes it feel worse."
    } else if degrees <= 6 {
        "Three thin layers beat one thick one: a base layer, a jumper and a windproof coat, so you can take the middle off indoors. Cover the neck, ears, wrists and ankles, which is where the cold gets in, and a hat is worth more than it looks. Wool or a synthetic base layer stays warm where cotton goes cold and damp."
    } else {
        "Layers you can take off, and one of them waterproof: a thin base layer, a jumper and a shell over the top sees you through a day that starts cold and warms up. Shoes with a sole rather than canvas, and a bag with a zip, because the British version of this number comes with rain."
    })
}

fn mentioned_recently(turns: &[Exchange], recent: usize, words: &[&str]) -> bool {
    !words.is_empty()
        && turns.iter().rev().take(recent).any(|turn| {
            let line = turn.user.to_lowercase();
            words.iter().any(|word| line.contains(word))
        })
}

pub(super) fn choose_garment(fit: &'static str, turns: &[Exchange]) -> &'static str {
    CLOTHING_FIT_USE
        .iter()
        .find_map(|(words, reply)| mentioned_recently(turns, 4, words).then_some(*reply))
        .unwrap_or(fit)
}

pub(super) fn choose_branch(
    reading: &Reading<'_>,
    turns: &[Exchange],
    past_fence: &[&str],
    choices: &[(&[&str], &'static str)],
    fallback: &'static str,
) -> Option<&'static str> {
    if mentioned_recently(turns, 3, past_fence) {
        return None;
    }
    let said = format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.norm_text,
        reading.folded()
    );
    let reply = choices
        .iter()
        .find_map(|(words, reply)| {
            words
                .iter()
                .any(|word| said.contains(word))
                .then_some(*reply)
        })
        .unwrap_or(fallback);
    (!reply.is_empty()).then_some(reply)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garment_callback_uses_previous_activity_and_selects_text_branches() {
        let running = [Exchange {
            user: "i want to start running".to_owned(),
            bot: String::new(),
            module: "test",
        }];
        assert!(choose_garment("fit", &running).starts_with("For running"));
        assert_eq!(choose_garment("fit", &[]), "fit");
        let boot = super::super::lookup("what should i look for in walking boots?")
            .expect("walking boot branch available");
        assert_eq!(boot.id, "boots-to-buy");
        assert!(boot.reply.starts_with("For walking"));
    }

    #[test]
    fn weather_keeps_travel_illness_and_cook_times_separate() {
        assert!(wear_for_weather(&Reading::new(
            "what should i wear when its boiling outside"
        )));
        assert!(!wear_for_weather(&Reading::new(
            "how do i get rid of my cold by friday"
        )));
        assert!(!wear_for_weather(&Reading::new(
            "how long do potatoes take to boil"
        )));
        assert_eq!(
            temperature(&Reading::new("what should i wear when its -4 degrees")),
            Some(-4)
        );
        assert!(
            dress_for_temperature(&Reading::new("what should i wear when its 35 degrees"))
                .is_some_and(|reply| reply.starts_with("In real heat"))
        );
        assert!(
            dress_for_temperature(&Reading::new("what should i wear when its 17 degrees"))
                .is_some_and(|reply| reply.starts_with("Layers"))
        );
    }

    #[test]
    fn clothes_guards_use_the_requested_subject() {
        assert!(!zip_fault(&Reading::new("how do i sew a zip into a dress")));
        assert!(zip_fault(&Reading::new("my zip is stuck, can i sew it")));
        assert!(date_night_meal(&Reading::new(
            "what should i cook for a date night"
        )));
        assert!(!date_night_meal(&Reading::new(
            "what should i wear for a dinner date"
        )));
        assert!(mentions_interview(&Reading::new(
            "is this for the interview?"
        )));
        assert!(chooses_colour(&Reading::new(
            "do you want the white one or the grey one"
        )));
        assert!(hair_line(&Reading::new("my hair looks thin")));
        assert!(spectacles(&Reading::new("where can i fix my glasses")));
        assert!(!spectacles(&Reading::new(
            "how do i fix a glass i knocked off the table"
        )));
    }
}
