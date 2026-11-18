//! Time spans, temperature scales and oven settings from PyBot's unit tables.

#[path = "units_facts.rs"]
mod facts;
#[path = "units_ingredient.rs"]
mod ingredient;
#[path = "units_object.rs"]
mod object;
#[path = "units_portion.rs"]
mod portion;
pub(super) use portion::people_in_groups;
#[path = "units_timespan.rs"]
mod timespan;
#[path = "units_utensils.rs"]
mod utensils;

use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(GAS_MARK, r"gas\s*mark\s*(\d)");
pattern!(FAN, r"\bfan\b");
pattern!(GAS, r"\bgas\b");
pattern!(
    OVEN_DEGREES,
    r"(-?\d{2,3})\s*(?:degrees?\s*|°\s*)?(c|celsius|centigrade|fan|f|fahrenheit)\b"
);
pattern!(OVEN_DURATION, r"\b(how long|minutes?|hours?|mins)\b");
pattern!(
    OVEN_ASK,
    r"\b(what|which|how|convert|equivalent|set|change|turn|use)\b"
);
pattern!(BARE_OVEN_DEGREES, r"\d+(?:\.\d+)?|\.\d+");
pattern!(
    FAHRENHEIT_UNIT,
    r"\d\s*(?:degrees?\s*|°\s*)?(f|fahrenheit)\b"
);
pattern!(
    C_TO_F,
    r"(-?\d+(?:\.\d+)?)\s*(?:degrees?\s*|°\s*)?(?:c|celsius|centigrade)\b.*?\b(?:in|to|into|as)\s*(?:degrees?\s*)?(?:f|fahrenheit)\b"
);
pattern!(
    F_TO_C,
    r"(-?\d+(?:\.\d+)?)\s*(?:degrees?\s*|°\s*)?(?:f|fahrenheit)\b.*?\b(?:in|to|into|as)\s*(?:degrees?\s*)?(?:c|celsius|centigrade)\b"
);
pattern!(C_ASK, r"\bcelsius|\bcentigrade|\bdegrees? c\b|°c\b|\bc\b");
pattern!(F_ASK, r"\bfahrenheit|\bdegrees? f\b|°f\b|\bf\b");
pattern!(
    BARE_TEMPERATURE,
    r"^\s*(?:convert\s+)?(?:degrees?\s+)?(?:c|celsius|centigrade|f|fahrenheit)\s+(?:to|in|into)\s*(?:degrees?\s+)?(?:c|celsius|centigrade|f|fahrenheit)\s*$"
);
pattern!(
    HOW_TO,
    r"(?:how\s+do\s+i|how\s+to|how\s+can\s+i|how\s+do\s+you|convert|change|work\s+out|turn)\b"
);
pattern!(COUNT_WORD, r"\b(?:a|an|one)\b");
pattern!(
    COMPONENT,
    r"\bhow (?:much|many)\s+(?:of\s+)?(?:grams?\s+of\s+)?(?:sugar|salt|fat|protein|fibre|fiber|carbs?|carbohydrates?|calories|kcal|alcohol|caffeine|sodium|calcium|iron|vitamins?)\b"
);

#[derive(Clone, Copy)]
struct Oven {
    mark: u8,
    celsius: i32,
    fahrenheit: i32,
}

const OVEN_SETTINGS: [Oven; 9] = [
    Oven {
        mark: 1,
        celsius: 140,
        fahrenheit: 275,
    },
    Oven {
        mark: 2,
        celsius: 150,
        fahrenheit: 300,
    },
    Oven {
        mark: 3,
        celsius: 170,
        fahrenheit: 325,
    },
    Oven {
        mark: 4,
        celsius: 180,
        fahrenheit: 350,
    },
    Oven {
        mark: 5,
        celsius: 190,
        fahrenheit: 375,
    },
    Oven {
        mark: 6,
        celsius: 200,
        fahrenheit: 400,
    },
    Oven {
        mark: 7,
        celsius: 220,
        fahrenheit: 425,
    },
    Oven {
        mark: 8,
        celsius: 230,
        fahrenheit: 450,
    },
    Oven {
        mark: 9,
        celsius: 240,
        fahrenheit: 475,
    },
];
const FAN_OFFSET: i32 = 20;

fn nearest(value: f64, measure: impl Fn(Oven) -> i32) -> Oven {
    OVEN_SETTINGS
        .into_iter()
        .min_by(|a, b| {
            (f64::from(measure(*a)) - value)
                .abs()
                .total_cmp(&(f64::from(measure(*b)) - value).abs())
        })
        .expect("oven table has settings")
}

fn say(value: f64) -> String {
    let size = value.abs();
    if size >= 1000. {
        return super::maths::number(value.round_ties_even());
    }
    let precision = if size >= 100. {
        0
    } else if size >= 1. {
        1
    } else {
        2
    };
    let rendered = format!("{value:.precision$}");
    if precision == 0 {
        rendered
    } else {
        rendered
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_owned()
    }
}

fn degrees(text: &str) -> Option<f64> {
    let value: f64 = text.parse().ok()?;
    value.is_finite().then_some(value)
}

fn as_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.) * 5. / 9.
}

fn celsius_mark(celsius: f64) -> u8 {
    nearest(celsius, |oven| oven.celsius).mark
}

fn for_fan(celsius: f64) -> String {
    format!(
        "{}°C is {}°C in a fan oven, gas mark {}.",
        say(celsius),
        say(celsius - f64::from(FAN_OFFSET)),
        celsius_mark(celsius)
    )
}

fn oven(clean: &str) -> Option<String> {
    if let Some(found) = GAS_MARK.captures(clean) {
        let mark: u8 = found[1].parse().ok()?;
        let oven = OVEN_SETTINGS.iter().find(|oven| oven.mark == mark)?;
        return Some(format!(
            "Gas mark {mark} is {}°C ({}°F), or {}°C in a fan oven.",
            oven.celsius,
            oven.fahrenheit,
            oven.celsius - FAN_OFFSET,
        ));
    }
    let fan_asked = FAN.is_match(clean);
    let gas_asked = GAS.is_match(clean);
    let Some(found) = OVEN_DEGREES.captures(clean) else {
        if fan_asked && !OVEN_DURATION.is_match(clean) && OVEN_ASK.is_match(clean) {
            let bare = BARE_OVEN_DEGREES.find_iter(clean).find(|number| {
                let number = number.as_str();
                (2..=3).contains(&number.chars().count()) && !number.contains('.')
            })?;
            let celsius = degrees(bare.as_str())?;
            if (90.0..=280.0).contains(&celsius) {
                return Some(for_fan(celsius));
            }
        }
        return None;
    };
    let amount = degrees(&found[1])?;
    let unit = &found[2];
    if matches!(unit, "f" | "fahrenheit") && (fan_asked || gas_asked) {
        let celsius = as_celsius(amount);
        let mut reply = format!("{}°F is {}°C", &found[1], say(celsius));
        if fan_asked {
            reply.push_str(&format!(
                ", or {}°C in a fan oven",
                say(celsius - f64::from(FAN_OFFSET))
            ));
        }
        reply.push_str(&format!(", gas mark {}.", celsius_mark(celsius)));
        return Some(reply);
    }
    if unit == "fan" {
        let plain = amount + f64::from(FAN_OFFSET);
        return Some(format!(
            "{}°C on a fan setting is {}°C in a plain oven, gas mark {}.",
            say(amount),
            say(plain),
            celsius_mark(plain)
        ));
    }
    if gas_asked {
        if FAHRENHEIT_UNIT.is_match(clean) {
            let celsius = as_celsius(amount);
            return Some(format!(
                "{}°F is gas mark {}, and {}°C in a plain oven.",
                &found[1],
                celsius_mark(celsius),
                say(celsius)
            ));
        }
        return Some(format!(
            "{}°C is gas mark {}, and {}°C in a fan oven.",
            say(amount),
            celsius_mark(amount),
            say(amount - f64::from(FAN_OFFSET))
        ));
    }
    fan_asked.then(|| for_fan(amount))
}

fn scale(clean: &str) -> Option<String> {
    if let Some(matched) = C_TO_F.captures(clean) {
        let value = degrees(&matched[1])?;
        return Some(format!(
            "{}°C is {}°F.",
            say(value),
            say(value * 9. / 5. + 32.)
        ));
    }
    let matched = F_TO_C.captures(clean)?;
    let value = degrees(&matched[1])?;
    let result = as_celsius(value);
    let oven = nearest(value, |oven| oven.fahrenheit);
    if (f64::from(oven.fahrenheit) - value).abs() <= 12. {
        return Some(format!(
            "{}°F is about {}°C - gas mark {}, which most recipes call {}°C.",
            say(value),
            say(result),
            oven.mark,
            say(f64::from(oven.celsius))
        ));
    }
    Some(format!("{}°F is {}°C.", say(value), say(result)))
}

fn scale_rule(clean: &str) -> Option<String> {
    if clean.bytes().any(|b| b.is_ascii_digit())
        || COUNT_WORD.is_match(clean)
        || !(HOW_TO.is_match(clean) || BARE_TEMPERATURE.is_match(clean))
    {
        return None;
    }
    let celsius = C_ASK.find(clean)?;
    let fahrenheit = F_ASK.find(clean)?;
    Some(if celsius.start() < fahrenheit.start() {
        "Multiply the Celsius by 9, divide by 5 and add 32: 20°C is 68°F. The quick way in your head is to double it and add 30, which is close enough for the weather."
    } else {
        "Take away 32, then multiply by 5 and divide by 9: 68°F is 20°C. The quick way in your head is to take away 30 and halve it."
    }.to_owned())
}

/// Convert temperatures or oven settings when the line names the conversion.
fn temperature(clean: &str) -> Option<String> {
    oven(clean)
        .or_else(|| scale(clean))
        .or_else(|| scale_rule(clean))
}

pub(super) fn convert(clean: &str) -> Option<String> {
    if clean.chars().count() > 200 {
        return None;
    }
    if COMPONENT.is_match(clean) {
        return ingredient::answer(clean);
    }
    utensils::answer(clean, true)
        .or_else(|| timespan::answer(clean))
        .or_else(|| temperature(clean))
        .or_else(|| {
            portion::serving_question(clean, || {
                object::answer(clean)
                    .or_else(|| ingredient::liquid_weight(clean))
                    .or_else(|| ingredient::answer(clean))
            })
        })
        .or_else(|| facts::answer(clean).map(str::to_owned))
}

pub(super) fn carried(clean: &str, turns: &[crate::bot::Exchange]) -> Option<String> {
    utensils::carried(clean, turns)
}

#[cfg(test)]
mod tests {
    mod reference {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/temperature.rs"
        ));
    }

    #[test]
    fn emergency_thread_suppresses_calculation_until_reset() {
        let mut bot = crate::bot::Bot::new(Some("0")).unwrap();
        let hazard = bot.turn("I smell gas in my kitchen.");
        assert_eq!(hazard.module, Some("safety"));
        let question = "What is an hour and a half?";
        let expected = "An hour and a half is 90 minutes.";
        let during = bot.turn(question);
        assert!(
            during
                .candidates
                .iter()
                .all(|proposal| proposal.reply != expected)
        );
        bot.reset().unwrap();
        assert_eq!(bot.turn(question).reply, expected);
    }

    #[test]
    fn oven_and_temperature_match_the_reference() {
        let mismatches: Vec<_> = reference::CASES
            .iter()
            .filter_map(|&(prompt, expected)| {
                let actual = super::temperature(prompt);
                (actual.as_deref() != expected).then_some((prompt, expected, actual))
            })
            .collect();
        assert!(mismatches.is_empty(), "{mismatches:?}");
    }
}
