//! Acknowledge a dated event in the person's own plans.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(concat!("(?i)", $value)).expect(stringify!($name)));
    };
}

pattern!(
    DATE_AHEAD,
    r####"\b(?:tomorrow|tonight|next (?:week|month|year|weekend)|this (?:weekend|afternoon|evening)|(?:on )?(?:mon|tues|wednes|thurs|fri|satur|sun)day|next (?:mon|tues|wednes|thurs|fri|satur|sun)day|in (?:\d+|a few|a couple of|two|three|four) (?:days?|weeks?|months?)|in the morning|the week after|not long now)\b"####
);
pattern!(
    DATE_AHEAD_OWN,
    r####"\b(?:my|our|im|i'm|i am|we are|we're|ive|i've|i have|i start|i go|i get|im off|i'm off)\b|\bthe (?:wedding|holiday|trip|exam|interview|test|party|scan|operation|appointment|viewing|hearing|move)\b"####
);
pattern!(
    AHEAD_NICE,
    r####"\b(?:holiday|holidays|trip|break|week off|visit|visiting|visitor|party|wedding|christening|concert|gig|festival|night out|day out|meal|dinner|lunch|housewarming|camping|abroad|flight|date|seeing|meeting up|reunion|graduation|barbecue|bbq|staying|going away|getaway|off to|away for)\b"####
);
pattern!(
    AHEAD_TENSE,
    r####"\b(?:exam|exams|test|interview|scan|operation|surgery|appointment|hospital|dentist|doctor|assessment|presentation|results|deadline|inspection|audit|court|hearing|tribunal|check-?up|treatment|procedure|settlement|viewing|inspection|review|appraisal|one-to-one|back to work|back at work|back to school|term starts|jab|jabs|vaccine|vaccination|injection|booster|blood test)\b"####
);
pattern!(
    NOT_A_PLAN,
    r####"\b(?:died|dying|death|funeral|buried|cremation|hospice|terminal|cancer|chemotherapy|hit me|hurt me|abuse[sd]?|miscarriage|stillborn|passed away|suicide|kill myself)\b"####
);

const NICE: &[&str] = &[
    "Something to look forward to, then - what's the plan?",
    "Nice. How are the plans going?",
    "Good. Is there much to sort out first?",
];
const TENSE: &[&str] = &[
    "How are you feeling about it?",
    "How ready do you feel for it?",
    "Is there much to get ready before it?",
];

fn read(line: &str) -> Option<&'static [&'static str]> {
    if line.contains('?')
        || !DATE_AHEAD.is_match(line)
        || !DATE_AHEAD_OWN.is_match(line)
        || NOT_A_PLAN.is_match(line)
    {
        return None;
    }
    if AHEAD_TENSE.is_match(line) {
        return Some(TENSE);
    }
    AHEAD_NICE.is_match(line).then_some(NICE)
}

pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if reading.raw.contains('?') {
        return None;
    }
    let typed = &reading.line.clean;
    let repaired = &reading.line.plain;
    let pool = read(typed).or_else(|| (repaired != typed).then(|| read(repaired)).flatten())?;
    (!super::trouble::negative_tone(reading))
        .then(|| super::choose_recent(pool, memory, typed).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dated_plans_match_the_pinned_composer_pool() {
        let cases = [
            (
                "The appointment is on Thursday.",
                Some("How ready do you feel for it?"),
            ),
            (
                "My exam is on Monday.",
                Some("How ready do you feel for it?"),
            ),
            (
                "I have a hospital appointment tomorrow.",
                Some("How ready do you feel for it?"),
            ),
            (
                "I am due at the dentist on Friday.",
                Some("How ready do you feel for it?"),
            ),
            (
                "My holiday is next week.",
                Some("Something to look forward to, then - what's the plan?"),
            ),
            (
                "Our wedding is next month.",
                Some("Nice. How are the plans going?"),
            ),
            (
                "My sister is visiting next week.",
                Some("Nice. How are the plans going?"),
            ),
            ("We are starting a new project next week.", None),
            ("Is my appointment on Thursday?", None),
            ("Sundays are lovely.", None),
            (
                "The hearing is on Monday.",
                Some("How are you feeling about it?"),
            ),
            ("My birthday is on Tuesday.", None),
            ("Their holiday is next week.", None),
        ];
        for (line, expected) in cases {
            assert_eq!(
                reply(&Reading::new(line), &Memory::default()).as_deref(),
                expected,
                "{line}"
            );
        }
    }
}
