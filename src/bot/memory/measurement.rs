//! Short measurement follow-ups from the immediately preceding knowledge answer.
//! Ported from the figure handling in PyBot's memory/narrow.py and memory/repeat.py.
use crate::bot::{Memory, Reading, understanding::normalize::tokens};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($source).unwrap());
    };
}

pattern!(
    DURATION_ASK,
    r"\b(?:how (?:long|soon|much longer)|how many (?:seconds?|minutes?|hours?|days?|weeks?|months?|years?|nights?))\b"
);
pattern!(
    TEMPERATURE_ASK,
    r"\b(?:how (?:hot|warm)|what (?:temperature|heat|gas mark|oven setting))\b"
);
pattern!(FREQUENCY_ASK, r"\b(?:how often|how many times)\b");
pattern!(CLOCK_ASK, r"\b(?:what time|which time)\b");
pattern!(DISTANCE_ASK, r"\b(?:how far|what distance)\b");
pattern!(PRESSURE_ASK, r"\b(?:what pressure|how many (?:bar|psi))\b");
pattern!(AMOUNT_ASK, r"\b(?:how (?:much|many)|what amount)\b");
pattern!(
    REPEAT_CUE,
    r"\b(?:did you (?:say|tell me)|was it you said|say again|remind me)\b"
);
pattern!(
    DURATION,
    concat!(
        r"(?i)\b(?:\d+\s*hours?\s*(?:and\s*)?\d+\s*(?:minutes?|mins?)|",
        r"(?:half an?|an?|one|two|three|four|five|six|seven|eight|nine|ten|twelve)\s+",
        r"(?:hours?|minutes?|days?|weeks?)\s+and\s+a\s+half|half an?\s+(?:hour|minute|second)|",
        r"(?:\d+(?:\.\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|fifteen|twenty|thirty|forty|fifty|sixty|an?)",
        r"(?:\s*(?:-|–|to|or)\s*(?:\d+(?:\.\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|twelve|twenty))?",
        r"(?:[ -]+)(?:(?:full|whole|solid|clear)\s+)?(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|months?|years?))\b"
    )
);
pattern!(
    TEMPERATURE,
    r"(?i)\b(?:\d{2,3}\s*(?:°\s*)?(?:celsius|fahrenheit|c|f)|\d{2,3}\s*degrees?|gas mark\s+\d)\b"
);
pattern!(
    FREQUENCY,
    r"(?i)\b(?:(?:once|twice|(?:\d+|three|four|five|six|seven)\s+times)\s+(?:a|per|every)\s+(?:day|week|month|year|night)|every\s+(?:day|week|month|year|morning|night))\b"
);
pattern!(
    CLOCK,
    r"(?i)\b(?:\d{1,2}[:.]\d{2}(?:\s*(?:am|pm))?|\d{1,2}\s*(?:am|pm)|noon|midday|midnight)\b"
);
pattern!(
    DISTANCE,
    r"(?i)\b\d+(?:\.\d+)?\s*(?:km|mi|m|miles?|kilometers?|kilometres?|meters?|metres?)\b"
);
pattern!(
    PRESSURE,
    r"(?i)\b\d+(?:\.\d+)?(?:\s*(?:-|–|to|and)\s*\d+(?:\.\d+)?)?\s*(?:bar|psi)\b"
);
pattern!(
    AMOUNT,
    r"(?i)(?:[£$€]\s*\d[\d,]*(?:\.\d+)?(?:\s*(?:-|–|to)\s*[£$€]?\d[\d,]*(?:\.\d+)?)?|\b\d+(?:\.\d+)?(?:\s*(?:-|–|to)\s*\d+(?:\.\d+)?)?\s*(?:kilograms?|kg|grams?|g|ounces?|oz|lb|lbs|pints?|ml|millilitres?|milliliters?|litres?|liters?|cups?|teaspoons?|tablespoons?|tsp|tbsp|l)\b)"
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Kind {
    Duration,
    Temperature,
    Frequency,
    Clock,
    Distance,
    Pressure,
    Amount,
}

impl Kind {
    fn measurement(self) -> &'static Regex {
        match self {
            Self::Duration => &DURATION,
            Self::Temperature => &TEMPERATURE,
            Self::Frequency => &FREQUENCY,
            Self::Clock => &CLOCK,
            Self::Distance => &DISTANCE,
            Self::Pressure => &PRESSURE,
            Self::Amount => &AMOUNT,
        }
    }
}

fn request(line: &str) -> Option<Kind> {
    const WORDS: &str = "and so okay ok please now again already then exactly actually roughly much many only like there what how did you say tell me was is are it that this they these those should would may can could do does will i we he she them to for on in of at take takes took last lasts keep keeps leave left wait hold rest cook bake roast boil simmer soak chill dry cool stay use put set give heat warm fan oven thigh thermometer chicken temperature cup cups gram grams tsp tbsp litres liters ml time first before after much longer finish";
    let (pattern, kind) = [
        (&DURATION_ASK, Kind::Duration),
        (&TEMPERATURE_ASK, Kind::Temperature),
        (&FREQUENCY_ASK, Kind::Frequency),
        (&CLOCK_ASK, Kind::Clock),
        (&DISTANCE_ASK, Kind::Distance),
        (&PRESSURE_ASK, Kind::Pressure),
        (&AMOUNT_ASK, Kind::Amount),
    ]
    .into_iter()
    .find(|(ask, _)| ask.is_match(line))?;
    let rest = pattern.replace(line, " ");
    let allowed = |word: &str| {
        WORDS
            .split_ascii_whitespace()
            .any(|allowed| word == allowed)
    };
    tokens(&rest)
        .iter()
        .all(|word| allowed(word))
        .then_some(kind)
}

pub(super) fn explicit_request(line: &str) -> bool {
    REPEAT_CUE.is_match(line) && request(line).is_some()
}

fn recent_answer(memory: &Memory) -> Option<&str> {
    memory
        .turns
        .iter()
        .rev()
        .take(4)
        .take_while(|turn| match turn.module {
            "safety" | "loss" | "grief" | "urgent" => false,
            "answers" => true,
            "chat" | "memory_store" | "greeting" => {
                request(&turn.user.to_lowercase()).is_some()
                    || matches!(
                        tokens(&turn.user).join(" ").as_str(),
                        "thanks" | "thank you" | "okay" | "ok"
                    )
            }
            _ => false,
        })
        .find(|turn| turn.module == "answers")
        .map(|turn| turn.bot.trim())
}

pub(super) fn sentences(line: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut quoted = false;
    for (at, ch) in line.char_indices() {
        match ch {
            '"' => quoted = !quoted,
            '“' => quoted = true,
            '”' => quoted = false,
            '.' | '!' | '?' if !quoted => {
                let end = at + ch.len_utf8();
                if line[end..].starts_with(char::is_whitespace) {
                    out.push(line[start..end].trim());
                    start = end;
                }
            }
            _ => {}
        }
    }
    if let Some(last) = line
        .get(start..)
        .map(str::trim)
        .filter(|last| !last.is_empty())
    {
        out.push(last);
    }
    out
}

fn trailing_relation(rest: &str) -> bool {
    rest.split_whitespace().next().is_some_and(|word| {
        ["less", "more", "earlier", "later", "longer", "shorter"]
            .contains(&word.trim_matches([',', '.', ';']))
    })
}

fn formatted(part: &str) -> String {
    let part = part.trim().trim_end_matches(['.', ',', ';']);
    let mut chars = part.chars();
    let head: String = chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .collect();
    format!("{head}{}.", chars.as_str())
}

fn select(reply: &str, kind: Kind, question: &str) -> Option<String> {
    let words = tokens(question);
    let requested_step = [
        "rest", "boil", "soak", "wait", "bake", "fan", "thigh", "oven",
    ]
    .into_iter()
    .find(|step| words.iter().any(|word| word == step));
    let mut best = None;
    let mut priority = i32::MIN;
    let mut tied = false;
    for sentence in sentences(reply) {
        let low = sentence.to_lowercase();
        if sentence.ends_with('?') || low.starts_with("i don't know") {
            continue;
        }
        for found in kind.measurement().find_iter(sentence) {
            let tail = sentence[found.end()..].trim_start();
            if trailing_relation(tail) {
                continue;
            }
            let mut rank = 0;
            let (before, after) = (&sentence[..found.start()], &sentence[found.end()..]);
            let context = before
                .rsplit([',', ';', ':'])
                .next()
                .unwrap_or_default()
                .to_lowercase();
            if kind == Kind::Duration {
                if context.contains("take")
                    || context.contains("start to finish")
                    || context.contains("total")
                    || context.contains("roughly")
                {
                    rank += 4;
                }
                if tail.starts_with("per ")
                    || context.ends_with("plus ")
                    || tail.starts_with("before carving")
                {
                    rank -= 4;
                }
                if context.contains("year old")
                    || context.contains("aged ")
                    || after.to_lowercase().starts_with(" old")
                {
                    continue;
                }
            }
            if let Some(step) = requested_step {
                let nearby = tokens(&format!(
                    "{context} {}",
                    tail.split_whitespace()
                        .take(3)
                        .collect::<Vec<_>>()
                        .join(" ")
                ));
                if nearby.iter().any(|word| word == step) {
                    rank += 12;
                }
            }
            if rank > priority {
                priority = rank;
                best = Some(found.as_str());
                tied = false;
            } else if rank == priority && best != Some(found.as_str()) {
                tied = true;
            }
        }
    }
    if tied { None } else { best.map(formatted) }
}

pub(in crate::bot) fn duration_sentence(answer: &str) -> Option<&str> {
    sentences(answer)
        .into_iter()
        .find(|sentence| !sentence.ends_with('?') && DURATION.is_match(sentence))
}

pub(crate) fn follow_up(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let kind = request(&reading.clean)?;
    let answer = recent_answer(memory)?;
    if let Some(selected) = select(answer, kind, &reading.clean) {
        return Some(selected);
    }
    if kind == Kind::Temperature && TEMPERATURE.find_iter(answer).count() > 1 {
        // Retain the full context when the answer names different temperatures.
        return Some(
            if answer.starts_with("As I said: ") || answer.starts_with("Same again: ") {
                answer.to_owned()
            } else {
                format!("As I said: {answer}")
            },
        );
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_selects_the_total_and_preserves_the_complete_compound_span() {
        let text = "About 20 minutes per 500g plus 20 minutes at 190C (170C fan), so a 2kg bird takes roughly 1 hour 40 minutes. Then rest it for 15 minutes before carving.";
        assert_eq!(
            select(text, Kind::Duration, "how long did you say?").as_deref(),
            Some("1 hour 40 minutes.")
        );
        assert_eq!(
            select(text, Kind::Duration, "how long should i rest it?").as_deref(),
            Some("15 minutes.")
        );
        assert!(select(text, Kind::Temperature, "what temperature?").is_none());
    }

    #[test]
    fn exact_follow_ups_preserve_units_and_decline_queries_that_name_a_new_thing() {
        assert_eq!(
            select(
                "Give it half an hour. Then rinse.",
                Kind::Duration,
                "how long?"
            )
            .as_deref(),
            Some("Half an hour.")
        );
        assert_eq!(
            select("At 8:30 pm. Arrive early.", Kind::Clock, "what time?").as_deref(),
            Some("8:30 pm.")
        );
        assert_eq!(request("what pressure do i use?"), Some(Kind::Pressure));
        assert_eq!(request("how much does a bath use in litres?"), None);
    }
}
