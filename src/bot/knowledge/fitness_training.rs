//! Short exercise sessions, training schedules, and injury prevention.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    MINUTES,
    r"\b(?:(\d{1,3})|(five|ten|fifteen|twenty(?:[\s-]five)?|thirty|forty(?:[\s-]five)?|sixty))\s*(?:-|\s)\s*(?:mins?|minutes?)\b"
);
pattern!(HALF_HOUR, r"\bhalf an hour\b");
pattern!(ONE_HOUR, r"\b(?:an|one|1)\s+hour\b");
pattern!(
    FARE,
    r"\b(?:pounds?|quid|pence|euros?|dollars?|cheap|cheaper|cheapest|expensive|price|prices|cost|costs|costing|fare|fares|ticket|tickets|refund|paid|pay|paying)\b|£|\$|€"
);
pattern!(
    ACHE_DAY,
    r"\b(?:(?:a|one|two|three|four|five|six|seven|eight|nine|ten|few|couple\s+of)\s+days?|(?:the\s+)?day\s+after|next\s+day|days?\s+(?:after|later|since)|48\s+hours|second\s+day|third\s+day)\b"
);
pattern!(ANOTHER_OWNER, r"\b(?:his|her|their)\b");
pattern!(
    EVERY_DAY_ASK,
    r"(?i)\b(?:should|shall|can|could|is it|ok|okay|bad|wrong|fine|alright|all right|better|worse|safe|harm|hurts?|need|rest|how often|how many|how much|how long|what|why|when|do i|does it|too much|too often|overdo\w*|overtrain\w*)\b"
);
pattern!(
    ALREADY_HURT,
    r"\b(?:i|ive|i've|i have|he|she|we|they)\s+(?:have\s+|has\s+|had\s+|got\s+|just\s+)?(?:hurt|injured)\b"
);

const MINUTE_WORDS: &[(&str, u16)] = &[
    ("five", 5),
    ("ten", 10),
    ("fifteen", 15),
    ("twenty", 20),
    ("twenty-five", 25),
    ("thirty", 30),
    ("forty", 40),
    ("forty-five", 45),
    ("sixty", 60),
];

const MANUAL_HANDLING: &str = "Plan the move first. Clear the route, make the load smaller, and use a trolley or get help for heavy or awkward items. For a manageable lift, keep the load close, place your feet apart, and bend slightly at the hips and knees. Lift smoothly and turn by moving your feet. Put the item down before adjusting its position.";
const SORENESS: &str = "A general muscle ache a day or two after unfamiliar or harder exercise can be delayed-onset muscle soreness. Ease off hard training while it settles and keep moving gently if that feels comfortable. Seek advice for sharp, localised or worsening pain, swelling or bruising. Get immediate medical assessment for cola-coloured urine or unusually severe muscle pain or weakness after exercise.";
const TRAINING_INJURIES: &str = "Build up gradually. Start with a few minutes of easy movement and a light set before lifting heavier. Increase the load, distance or pace in small steps and allow time to recover. Stop if a movement causes sharp pain. If pain persists or a joint gives way, get advice from a healthcare professional before returning to hard training.";
const MUSCLE_REST: &str = "Leave about a day between hard sessions for the same muscle group to start with. Two full-body sessions on non-consecutive days work well for many people. If you prefer consecutive days, alternate which muscles you train hard and adjust the load if soreness or fatigue is affecting your technique.";

fn minutes(text: &str) -> Option<u16> {
    let mut found = MINUTES
        .captures_iter(text)
        .filter_map(|captures| {
            if let Some(digits) = captures.get(1) {
                return digits.as_str().parse::<u16>().ok();
            }
            let word = captures.get(2)?.as_str().replace(' ', "-");
            MINUTE_WORDS
                .iter()
                .find_map(|&(name, count)| (name == word).then_some(count))
        })
        .max();
    let hour = if HALF_HOUR.is_match(text) {
        Some(30)
    } else if ONE_HOUR.is_match(text) {
        Some(60)
    } else {
        None
    };
    found = found.max(hour);
    found.filter(|&value| value > 0)
}

fn short_workout(text: &str) -> Cow<'static, str> {
    let time = minutes(text);
    let lead = time.map_or_else(
        || "In the time you have".to_owned(),
        |minutes| {
            let unit = if minutes == 1 { "minute" } else { "minutes" };
            format!("With {minutes} {unit}")
        },
    );
    let work = match time {
        Some(1..=5) => {
            "start with easy movement and try a few chair squats, wall press-ups or step-ups at a comfortable pace. A short bout still adds to your weekly activity."
        }
        Some(30..) => {
            "warm up, then try three rounds of squats, press-ups, a plank and lunges. Take about 40 seconds per exercise and rest between rounds. Finish with a brisk walk if you have time."
        }
        _ => {
            "warm up and try a round of squats, press-ups, a plank and lunges. Work for up to 40 seconds per exercise, rest as needed, and repeat if you have time. Wall press-ups are an easier option."
        }
    };
    Cow::Owned(format!("{lead}, {work}"))
}

fn cardio(text: &str) -> Cow<'static, str> {
    let time = minutes(text).unwrap_or(20);
    let unit = if time == 1 { "minute" } else { "minutes" };
    let next = if time < 20 {
        "Start with the time you have and add a little when it feels manageable."
    } else {
        "Use a pace where you can talk but would struggle to sing, and build up gradually."
    };
    Cow::Owned(format!(
        "{time} {unit} of brisk walking, cycling or another activity that raises your breathing counts. {next} The NHS target for most adults is at least 150 minutes of moderate activity a week, or 75 minutes of vigorous activity."
    ))
}

fn stretching(text: &str) -> Cow<'static, str> {
    let before = text.split_whitespace().any(|word| word == "before");
    let after = text
        .split_whitespace()
        .any(|word| word.trim_end_matches(['?', '.', ',']) == "after");
    let intro = match (before, after) {
        (true, false) => {
            "Before exercising, warm up with gentle movement such as walking, marching or controlled leg swings."
        }
        (false, true) => "After exercising, gentle stretches can be part of your cool-down.",
        _ => {
            "Warm up with easy movement before exercise and save gentle held stretches for the cool-down."
        }
    };
    Cow::Owned(format!(
        "{intro} If you stretch afterwards, hold each stretch for around 15 to 20 seconds without bouncing or pushing into pain."
    ))
}

pub(crate) fn not_a_fare(reading: &Reading<'_>) -> bool {
    !FARE.is_match(&reading.clean)
}

pub(crate) fn delayed_soreness(reading: &Reading<'_>) -> bool {
    let text = reading.raw.to_lowercase();
    !ANOTHER_OWNER.is_match(&text) && ACHE_DAY.is_match(&text)
}

pub(crate) fn asks_exercise_frequency(reading: &Reading<'_>) -> bool {
    EVERY_DAY_ASK.is_match(&reading.clean) || reading.line.signals.question_mark
}

pub(crate) fn preventing_injury(reading: &Reading<'_>) -> bool {
    !ALREADY_HURT.is_match(&reading.clean)
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(match id {
        "short-workout" => short_workout(&reading.raw.to_lowercase()),
        "20-min-cardio" => cardio(&reading.raw.to_lowercase()),
        "stretch-when" => stretching(&reading.raw.to_lowercase()),
        "back-lifting" => Cow::Borrowed(MANUAL_HANDLING),
        "doms-two-days" => Cow::Borrowed(SORENESS),
        "same-muscle-two-days" => Cow::Borrowed(MUSCLE_REST),
        "training-injuries" => Cow::Borrowed(TRAINING_INJURIES),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sessions_use_the_named_time_and_yield_for_costs_or_existing_injury() {
        assert!(short_workout("i have five minutes for a workout").contains("With 5 minutes"));
        assert!(cardio("can ten minutes of cardio count").contains("10 minutes"));
        assert!(!not_a_fare(&Reading::new("is 20 pounds cheap for a train")));
        assert!(!preventing_injury(&Reading::new(
            "i hurt myself at the gym"
        )));
        assert!(preventing_injury(&Reading::new(
            "how do i avoid hurting myself in the gym"
        )));
    }
}
