//! Family cards whose opening follows the age, relationship, or question asked.

use crate::bot::{Exchange, Reading, understanding::kinds::Kind};
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[path = "family_card_data.rs"]
mod data;
use data::*;

static YEAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(YEAR_PATTERN).unwrap());
static HOSPITAL_OR_WARD: LazyLock<Regex> = LazyLock::new(|| Regex::new(HOSPITAL_PATTERN).unwrap());
static BIRTH_IN_CHAT: LazyLock<Regex> = LazyLock::new(|| Regex::new(BIRTH_PATTERN).unwrap());
static NOT_A_BITE: LazyLock<Regex> = LazyLock::new(|| Regex::new(NOT_A_BITE_PATTERN).unwrap());
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
static NO_CAFFEINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(NO_CAFFEINE_PATTERN).unwrap());
static CAFFEINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(CAFFEINE_PATTERN).unwrap());
static SAY_TO_SOMEONE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(SAY_TO_SOMEONE_PATTERN).unwrap());
static NAMES_A_LIKING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(NAMES_A_LIKING_PATTERN).unwrap());
static NOT_A_LIKING: LazyLock<Regex> = LazyLock::new(|| Regex::new(NOT_A_LIKING_PATTERN).unwrap());
static MONTHS_OLD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(\d{1,2})\s*(?:month|months|mth|mo)\b").unwrap());
static KNOWN_FEEDING_ISSUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\b(?:premature|prematurity|preterm|preemie|tube fed|tube feeding|milk protein allergy|",
        r"(?:milk|dairy) allerg(?:y|ies)|not gaining weight|losing weight|poor weight gain|",
        r"refus(?:e|es|ing) (?:feeds|milk))\b"
    ))
    .expect("individual infant feeding needs")
});
static BREASTFEEDING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:breast[- ]?(?:feed(?:ing|s)?|fed|milk)|chest[- ]?feeding|nursing)\b").unwrap()
});
static FORMULA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bformula\b").unwrap());
static STOPPED_FEEDING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:stopped (?:feeding|taking milk)|dehydrat(?:ed|ion))\b").unwrap()
});

pub(super) fn hurt_feelings(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(
        if reading.kind == Kind::Statement && reading.features.asks.is_empty() {
            Cow::Owned(format!(
                "That's not on, and it is fair to feel stung. {HURT_FEELINGS}"
            ))
        } else {
            Cow::Borrowed(HURT_FEELINGS)
        },
    )
}

pub(super) fn asks_for_money(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let family = reading
        .line
        .words
        .iter()
        .any(|word| MONEY_ASKERS.contains(&word.as_str()));
    Some(Cow::Borrowed(if family {
        FAMILY_ASKING_MONEY
    } else {
        FRIEND_ASKING_MONEY
    }))
}

pub(super) fn school_year(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.line.text.to_lowercase();
    let age = YEAR
        .captures(&text)
        .and_then(|found| found[1].parse::<u8>().ok())
        .or_else(|| (text.contains("secondary") || text.contains("high school")).then_some(7));
    let Some(year) = age else {
        return Some(Cow::Borrowed(YEAR_GENERIC));
    };
    let note = match year {
        1..=3 => YEAR_READING,
        4..=6 => YEAR_WORK,
        7..=9 => YEAR_BUILDING,
        _ => YEAR_EXAMS,
    };
    Some(Cow::Owned(format!(
        "Year {year} is ages {} and {}. {note} {YEAR_TAIL}",
        year + 4,
        year + 5
    )))
}

pub(super) fn change_nappy(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = format!(
        "{} {} {}",
        reading.line.clean, reading.line.norm_text, reading.line.plain
    )
    .to_lowercase();
    Some(Cow::Borrowed(
        if HOW_OFTEN.iter().any(|word| text.contains(word)) {
            NAPPY_OFTEN
        } else {
            NAPPY_METHOD
        },
    ))
}

pub(super) fn nappies_a_day(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = format!(" {} ", reading.line.clean.to_lowercase());
    let reply = NAPPY_AGES
        .iter()
        .find_map(|(ages, reply)| {
            ages.iter()
                .any(|age| said.contains(&format!(" {age} ")))
                .then_some(*reply)
        })
        .unwrap_or(NAPPY_NEWBORN);
    Some(Cow::Borrowed(reply))
}

pub(super) fn hospital_visits(
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let text = if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    };
    let newborn = !HOSPITAL_OR_WARD.is_match(text) && {
        let before = earlier
            .iter()
            .flat_map(|turn| [&turn.user, &turn.bot])
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" ");
        BIRTH_IN_CHAT.is_match(&before)
    };
    Some(Cow::Borrowed(if newborn {
        NEWBORN_VISIT
    } else {
        WARD_VISIT
    }))
}

pub(super) fn toddler_hurts(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let mut text = String::new();
    for view in [
        &reading.line.clean,
        &reading.line.text,
        &reading.line.norm_text,
    ] {
        let low = view.to_lowercase();
        text.push_str(&NOT_A_BITE.replace_all(&low, " "));
        text.push(' ');
    }
    let words: std::collections::HashSet<_> =
        WORD.find_iter(&text).map(|word| word.as_str()).collect();
    let (behaviour, say) = HURTING_WORDS
        .iter()
        .find_map(|(variants, name, say)| {
            variants
                .iter()
                .any(|word| words.contains(word))
                .then_some((*name, *say))
        })
        .unwrap_or(("hitting", "'No - hitting hurts'"));
    Some(Cow::Owned(format!(
        "Stop it in the moment, every time, and name it: {say}, then take them away from the other child for a minute and help them say what they wanted ('you wanted the truck'). The {behaviour} is a shortcut for a word they don't have yet, so give them the sentence and praise it the first time. Never do it back: it teaches that the biggest person wins."
    )))
}

pub(super) fn not_real_coffee(reading: &Reading<'_>) -> bool {
    let text = if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    };
    NO_CAFFEINE.is_match(text) || !CAFFEINE.is_match(text)
}

pub(super) fn drink_while_feeding(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let words: Vec<_> = reading
        .line
        .words
        .iter()
        .map(|word| word.trim_matches(|c: char| c.is_ascii_punctuation()))
        .collect();
    let said = format!(" {} ", words.join(" ")).to_lowercase();
    let names = |drinks: &[&str]| {
        drinks.iter().any(|drink| {
            said.contains(&format!(" {drink} ")) || said.contains(&format!(" {drink}s "))
        })
    };
    let reply = if !names(OTHER_DRINKS) && names(WATER_WORDS) {
        WATER_REPLY
    } else {
        DRINKS_REPLY
    };
    Some(Cow::Borrowed(reply))
}

pub(super) fn asking_out(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let said = if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    };
    if !SAY_TO_SOMEONE.is_match(&said.to_lowercase()) {
        return Some(Cow::Borrowed(ASKING_OUT));
    }
    let reply = earlier.iter().rev().take(3).find_map(|turn| {
        let said = turn.user.to_lowercase();
        if NOT_A_LIKING.is_match(&said) {
            Some(NOT_WANTING)
        } else if NAMES_A_LIKING.is_match(&said) {
            Some(ASKING_OUT)
        } else {
            None
        }
    })?;
    Some(Cow::Borrowed(reply))
}

fn named_months(said: &str) -> Option<u8> {
    MONTHS_OLD
        .captures(said)
        .and_then(|age| age[1].parse::<u8>().ok())
        .or_else(|| {
            MONTH_WORDS.iter().find_map(|(word, count)| {
                said.contains(&format!(" {word} month")).then_some(*count)
            })
        })
}

fn single_year_or_more(said: &str) -> bool {
    YEAR_OR_MORE.iter().any(|phrase| said.contains(phrase))
}

pub(super) fn older_baby_milk(reading: &Reading<'_>) -> bool {
    let said = format!(" {} ", reading.line.clean.to_lowercase());
    single_year_or_more(&said)
        || AT_LEAST_SIX.iter().any(|phrase| said.contains(phrase))
        || named_months(&said).is_some_and(|count| count >= 6)
}

// Preserve the card's authored admission guard. The output follows NHS first solids,
// age-specific weaning guides, and NHS advice on drinks for young children.
pub(super) fn milk_for_age(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = format!(" {} ", reading.line.clean.to_lowercase());
    let months = if single_year_or_more(&said) {
        Some(12)
    } else {
        named_months(&said).map(u32::from)
    };
    Some(Cow::Borrowed(infant_milk_advice(reading, months)))
}

pub(super) fn infant_milk_advice(reading: &Reading<'_>, months: Option<u32>) -> &'static str {
    let said = reading.line.clean.as_str();
    let breast = BREASTFEEDING.is_match(said);
    let formula = FORMULA.is_match(said);
    if STOPPED_FEEDING.is_match(said) {
        "If your baby has stopped taking milk or you think they may be dehydrated, contact NHS 111 or ask for an urgent GP appointment. For ongoing concerns about milk intake or weaning, speak to your health visitor."
    } else if KNOWN_FEEDING_ISSUE.is_match(said) {
        "If your baby was born early, has an allergy, has a feeding plan or isn't feeding or gaining weight as expected, ask your health visitor or their care team before changing their milk. They can tailor the amount and type to your baby."
    } else if months.is_some_and(|count| count >= 12) {
        "From 12 months, pasteurised whole or semi-skimmed cows' milk can be a main drink. You can keep breastfeeding for as long as you both want. Up to their first birthday, keep breast milk or first infant formula as the main drink. Ask your health visitor if you're unsure how to make the change."
    } else if breast && formula || said.contains("mixed feeding") {
        "When you combine breastfeeds and first infant formula, keep both alongside solids. How much formula your baby needs depends on how much they're breastfeeding, so follow their cues and ask your health visitor for help with the amount. Breast milk or first infant formula remains their main drink until their first birthday."
    } else if breast {
        "Keep breastfeeding in response to your baby's cues as you introduce solids from around six months. Breast milk stays their main drink through the first year, and their feeds can change as they eat more food. Ask your health visitor if you're worried about how much they're taking."
    } else {
        match months {
            Some(7..=9) => {
                "Breast milk or first infant formula stays the main drink as you introduce solids. At 7 to 9 months, about 600 ml a day is an NHS guide for formula-fed babies. Follow their hunger and fullness cues and let breastfeeds adjust as they eat more. Ask your health visitor if you're unsure how much your baby needs."
            }
            Some(10..=11) => {
                "Breast milk or first infant formula stays the main drink until their first birthday. At 10 to 12 months, about 400 ml a day is an NHS guide for formula-fed babies who are also eating meals. Follow their hunger and fullness cues and let breastfeeds adjust as they eat more. Ask your health visitor if you're unsure how much your baby needs."
            }
            _ => {
                "Start offering small amounts of solids around six months when your baby is ready, alongside their usual breastfeeds or first infant formula. Keep breast milk or formula as the main drink until their first birthday and follow their feeding cues. Ask your health visitor if you're unsure how much milk your baby needs."
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dating_followups_use_the_latest_clear_feeling() {
        let current = Reading::new("what do i say to her");
        assert_eq!(asking_out(&current, &[]), None);
        let turn = |user: &str| Exchange {
            user: user.to_owned(),
            bot: String::new(),
            module: "",
        };
        let liked = turn("i like a girl at work");
        assert_eq!(
            asking_out(&current, &[liked]),
            Some(Cow::Borrowed(ASKING_OUT))
        );
        let corrected = [
            turn("i like a girl at work"),
            turn("i dont like her anymore"),
        ];
        assert_eq!(
            asking_out(&current, &corrected),
            Some(Cow::Borrowed(NOT_WANTING))
        );
        let other = Reading::new("how do i ask a girl out");
        assert_eq!(asking_out(&other, &[]), Some(Cow::Borrowed(ASKING_OUT)));
    }

    #[test]
    fn older_baby_milk_keeps_source_guard_and_distinguishes_feed_type_and_age() {
        assert!(!older_baby_milk(&Reading::new(
            "how much milk for a five month old"
        )));
        assert!(older_baby_milk(&Reading::new(
            "how much milk for a baby of half a year"
        )));
        for (line, wanted) in [
            (
                "how much formula for a six month old",
                "alongside their usual breastfeeds or first infant formula",
            ),
            (
                "how much formula for an eight month old",
                "about 600 ml a day",
            ),
            (
                "how much formula for an eleven month old",
                "about 400 ml a day",
            ),
            ("how much milk does my 12 month old need", "From 12 months"),
            (
                "breastfeeding my 8 month old, how much milk",
                "breastfeeding in response to your baby's cues",
            ),
            (
                "breastfeeding my 8 month old and giving formula, how much milk",
                "When you combine breastfeeds and first infant formula",
            ),
            (
                "my 8 month old was born premature, how much milk",
                "ask your health visitor or their care team before changing their milk",
            ),
            (
                "my 8 month old stopped taking milk",
                "contact NHS 111 or ask for an urgent GP appointment",
            ),
        ] {
            let reading = Reading::new(line);
            assert!(older_baby_milk(&reading), "{line}");
            let reply = milk_for_age(&reading).unwrap();
            assert!(reply.contains(wanted), "{line}: {reply}");
            assert!(!reply.contains("500 ml"), "{line}");
        }
    }

    #[test]
    fn uses_the_literal_school_year_and_selects_its_band() {
        let reply = school_year(&Reading::new("my son starts year 4 in september")).unwrap();
        assert!(reply.starts_with("Year 4 is ages 8 and 9."));
        assert!(reply.contains(YEAR_WORK));
        let secondary = school_year(&Reading::new("my daughter starts secondary school")).unwrap();
        assert!(secondary.starts_with("Year 7 is ages 11 and 12."));
        assert_eq!(
            school_year(&Reading::new("moving up a year")),
            Some(Cow::Borrowed(YEAR_GENERIC))
        );
    }

    #[test]
    fn distinguishes_nappy_frequency_from_steps_and_age_bands() {
        assert_eq!(
            change_nappy(&Reading::new("how often should i change a nappy")),
            Some(Cow::Borrowed(NAPPY_OFTEN))
        );
        assert_eq!(
            change_nappy(&Reading::new("how do i change a nappy")),
            Some(Cow::Borrowed(NAPPY_METHOD))
        );
        assert_eq!(
            nappies_a_day(&Reading::new("how many nappies does a 6 month old need")),
            Some(Cow::Borrowed(NAPPY_AGES[1].1))
        );
    }
}
