//! Caffeine amounts and sleep timing, duration, and supplements.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Captures, Regex};

use crate::bot::Reading;

use super::sleep_timing::clock_label;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    SLEEP_SAID,
    r"\b(\d{1,2}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\s*(?:-\s*\d{1,2}\s*)?(?:hours?|hrs?|h)\b"
);
pattern!(
    TEEN,
    r"\b(?:teens?|teenage|teenagers?)\b|\b(?:1[3-7]|thirteen|fourteen|fifteen|sixteen|seventeen)\s*[- ](?:year|years|yr)\s*[- ]old\b|\b(?:im|i'm|i am|aged)\s+(?:1[3-7]|thirteen|fourteen|fifteen|sixteen|seventeen)\b"
);
pattern!(
    OLDER,
    r"\b(?:over[- ]65|older|elderly|pensioners?)\b|\b(?:im|i'm|i am|aged)\s+(?:6[5-9]|[789]\d)\b|\b(?:6[5-9]|[789]\d)\s*[- ](?:year|years|yr)\s*[- ]old\b"
);
pattern!(CONFIRMS_ENOUGH, r"\b(?:enough|ok|okay|fine|normal)\b");
pattern!(
    WAKE_CLOCK,
    r"\b(?:up|awake|wake|wakes|alarm|get up|getting up)\b[^0-9]{0,16}?\b(\d{1,2})(?:[:.](\d{2}))?\s*(am|pm)?\b"
);
pattern!(
    BED_CLOCK,
    r"\b(?:bed|bedtime|sleep|lights out)\b[^0-9]{0,14}?\b(\d{1,2})(?:[:.](\d{2}))?\s*(am|pm)?\b"
);
pattern!(
    SLEEP_CHILD,
    r"\b(?:children|child|kids?|my son|my daughter|babies|baby|toddlers?|teenagers?|my boy|my girl|(?:8|9|10|11|12)[- ]years?(?:[- ]old)?)\b"
);

const SLEEP_ROUTINE: &str = "Keep a regular wake time and give yourself time to wind down. If sleep is affecting your daily life or changes are not helping, speak to your GP.";
const SLEEP_AIDS_HELP: &str = "Check with a pharmacist before taking anything for sleep, especially if you take other medicines. A regular wake time and less late caffeine may help. If insomnia persists or affects daily life, a GP can help you look for causes and treatments such as cognitive behavioural therapy for insomnia.";
const MAGNESIUM: &str = "Evidence that magnesium supplements improve sleep is limited and mixed. They can cause side effects or interact with medicines.";
const HERBAL: &str = "There's little reliable evidence that valerian or herbal teas treat insomnia. Herbal products can interact with medicines.";
const GUMMIES: &str = "Check the ingredients instead of assuming all sleep gummies contain the same thing. In the UK, melatonin is prescription-only. Some pharmacy sleep aids contain sedating antihistamines and are intended for short-term use.";

const LIMIT_ASKS: &[&str] = &[
    "much",
    "many",
    "too",
    "limit",
    "limits",
    "safe",
    "safely",
    "day",
    "days",
    "daily",
    "week",
    "weekly",
    "cups",
    "often",
    "amount",
    "maximum",
    "max",
    "bad",
    "good",
    "harmful",
    "healthy",
    "unhealthy",
    "ok",
    "okay",
    "fine",
    "worse",
    "risky",
    "risk",
];
const CAFFEINE: &str = "For most healthy adults, EFSA cites up to 400mg of caffeine spread across a day and up to 200mg at once. NHS guidance is no more than 200mg per day during pregnancy. Count caffeine across all drinks, foods and supplements. Reduce it earlier in the day if it affects your sleep.";

fn has(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| text.contains(phrase))
}

fn caffeine_limit(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    if !reading
        .line
        .words
        .iter()
        .any(|word| LIMIT_ASKS.contains(&word.as_str()))
    {
        return None;
    }
    let text = &reading.clean;
    if has(
        text,
        &[
            "herbal tea",
            "herbal teas",
            "camomile",
            "chamomile",
            "peppermint tea",
            "fruit tea",
            "rooibos",
        ],
    ) {
        return Some(Cow::Borrowed(
            "Plain chamomile, mint and rooibos are generally caffeine-free. Blends can contain black or green tea, so check the ingredients and caffeine information if you want to avoid it.",
        ));
    }
    let tea = has(
        text,
        &["tea", "teas", "green tea", "builder's tea", "builders tea"],
    );
    let coffee = has(
        text,
        &[
            "coffee",
            "coffees",
            "espresso",
            "latte",
            "cappuccino",
            "americano",
        ],
    );
    let energy = has(text, &["energy drink", "monster", "red bull"]);
    let cola = has(text, &["cola", "coke", "pepsi"]);
    let lead = match (tea, coffee, energy, cola) {
        (true, false, false, false) => {
            "The NHS estimates around 75mg in a cup of black or green tea. The actual amount depends on how much tea you use and how long you brew it."
        }
        (false, false, true, false) => {
            "Many energy drinks have around 80mg in a 250ml can. Check the label and size, because some contain much more per can."
        }
        (false, false, false, true) => {
            "A can of cola may contain around 40mg of caffeine. Check the label, as amounts vary and some varieties are caffeine-free."
        }
        _ => {
            "The NHS estimates around 100mg of caffeine in a cup of instant coffee. Coffee-shop or filter coffee can contain more, depending on serving size and preparation."
        }
    };
    Some(Cow::Owned(format!("{lead} {CAFFEINE}")))
}

fn sleep_hours(text: &str) -> Cow<'static, str> {
    if TEEN.is_match(text) {
        return Cow::Borrowed(
            "Teenagers generally need 8 to 10 hours of sleep a night. A regular wake time is a good starting point. Count backwards to give yourself enough time to sleep, and ask for advice if you still struggle to stay awake during the day.",
        );
    }
    if OLDER.is_match(text) {
        return Cow::Borrowed(
            "Adults aged 65 and over generally need about 7 to 8 hours of sleep. A regular schedule and how rested you feel are useful clues. Speak to your GP if you regularly wake unrefreshed or feel sleepy during the day.",
        );
    }
    let said = SLEEP_SAID
        .captures(text)
        .and_then(|found| {
            [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
                "eleven", "twelve",
            ]
            .iter()
            .position(|word| *word == &found[1])
            .map(|index| index as u16 + 1)
            .or_else(|| found[1].parse::<u16>().ok())
        })
        .filter(|hours| (1..=14).contains(hours));
    let guide = match said {
        Some(hours @ 1..=6) => format!("{hours} hours is on the short side for most adults, who usually need around 7 to 9. If your wake time is fixed, try allowing a little more time in bed."),
        Some(hours @ 7..=9) => {
            let yes = if CONFIRMS_ENOUGH.is_match(text) { "Yes. " } else { "" };
            format!("{yes}{hours} hours is within the usual 7 to 9 hours for adults. How refreshed you feel and whether you can stay alert during the day matter too.")
        }
        Some(hours) => format!("{hours} hours is longer than the usual 7 to 9 for adults. Sleep needs can vary. If you're regularly sleeping that long and still feel unrefreshed or very sleepy, speak to your GP."),
        None => "Most adults need around 7 to 9 hours of sleep a night. How rested and alert you feel during the day matters too.".to_owned(),
    };
    Cow::Owned(format!("{guide} {SLEEP_ROUTINE}"))
}

fn parsed_clock(found: &Captures<'_>, bedtime: bool) -> Option<i32> {
    let mut hour = found[1].parse::<i32>().ok()?;
    let minute = found
        .get(2)
        .map_or(Some(0), |found| found.as_str().parse::<i32>().ok())?;
    if hour > 23 || minute > 59 {
        return None;
    }
    match found.get(3).map(|found| found.as_str()) {
        Some("am") if (1..=12).contains(&hour) => hour %= 12,
        Some("pm") if (1..=12).contains(&hour) => hour = hour % 12 + 12,
        Some(_) => return None,
        None if bedtime && (6..=11).contains(&hour) => hour += 12,
        None => (),
    }
    Some(hour * 60 + minute)
}

fn shifted(clock: i32, hours: i32, offset: i32) -> String {
    clock_label((clock + hours * 60 + offset).rem_euclid(24 * 60) as u16)
}

fn sleep_clock(text: &str) -> Cow<'static, str> {
    let wake = WAKE_CLOCK
        .captures(text)
        .and_then(|found| parsed_clock(&found, false));
    let in_bed = BED_CLOCK
        .captures(text)
        .and_then(|found| parsed_clock(&found, true));
    if let Some(wake) = wake.filter(|wake| (4 * 60..12 * 60).contains(wake)) {
        return Cow::Owned(format!(
            "For a {} wake-up, lights out about {} gives you 8 hours of sleep if you take 20 minutes to fall asleep. For 9 hours, try about {}. Adjust for how long falling asleep usually takes you.",
            clock_label(wake as u16),
            shifted(wake, -8, -20),
            shifted(wake, -9, -20)
        ));
    }
    if wake.is_none()
        && let Some(bed) = in_bed.filter(|bed| *bed >= 20 * 60 || *bed < 4 * 60)
    {
        return Cow::Owned(format!(
            "From a {} bedtime, allowing 20 minutes to fall asleep, 8 hours takes you to {} and 9 hours to {}. Adjust the estimate for how long it usually takes you to drop off.",
            clock_label(bed as u16),
            shifted(bed, 8, 20),
            shifted(bed, 9, 20)
        ));
    }
    Cow::Borrowed(
        "Start with when you need to get up. Count back the sleep you need and add however long you usually take to fall asleep. For example, if you want 8 hours, take 20 minutes to fall asleep and need to get up at 7am, try lights out about 10:40pm. Keep your wake time reasonably consistent.",
    )
}

fn sleep_supplements(text: &str) -> Cow<'static, str> {
    if SLEEP_CHILD.is_match(text) {
        return Cow::Borrowed(
            "Ask your child's GP or a pharmacist before giving them any sleep aid, supplement or gummy. Some products contain medicines or ingredients unsuitable for children. A clinician can help you work out what's affecting their sleep and what to try.",
        );
    }
    let sections = if has(text, &["valerian", "herbal", "tea"]) {
        [HERBAL, MAGNESIUM, GUMMIES]
    } else if has(text, &["gumm", "melatonin", "antihistamine"]) {
        [GUMMIES, MAGNESIUM, HERBAL]
    } else {
        [MAGNESIUM, HERBAL, GUMMIES]
    };
    Cow::Owned(format!("{} {SLEEP_AIDS_HELP}", sections.join(" ")))
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.raw.to_lowercase();
    match id {
        "caffeine-limit" => caffeine_limit(reading),
        "sleep-hours" => Some(sleep_hours(&text)),
        "sleep-clock" => Some(sleep_clock(&text)),
        "sleep-supplements" => Some(sleep_supplements(&text)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sleep_clock_counts_across_midnight_and_keeps_advice_age_specific() {
        assert!(
            sleep_clock("what time to go to bed if i need to be up at 6:30am").contains("10:10pm")
        );
        assert!(sleep_hours("im 16 and im only getting six hours of sleep").contains("8 to 10"));
        assert!(sleep_supplements("are sleep gummies ok for my child").contains("child's GP"));
        assert!(
            caffeine_limit(&Reading::new("how much caffeine in a cup of tea"))
                .unwrap()
                .contains("75mg")
        );
    }
}
