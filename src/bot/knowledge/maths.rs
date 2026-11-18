//! Native stand-alone arithmetic readers from bot/knowledge/maths.py.
use regex::{Captures, Regex};
use std::{borrow::Cow, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $pattern:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
    };
}

pattern!(
    ASK,
    r"\b(?:what's|whats|what is|calculate|work out|how much is|how much does|please|tell me|the answer to|how do i|how do you|how can i|how would i|how to|can you|could you)\b"
);
pattern!(DIVIDE, r"\bdivide\s+(\d[\d.]*)\s+by\s+(\d[\d.]*)");
pattern!(MULTIPLY, r"\b(?:multiplied by|times|x)\b");
pattern!(OVER, r"\b(?:divided by|over)\b");
pattern!(PLUS, r"\b(?:plus|and)\b");
pattern!(MINUS, r"\b(?:minus|take away|subtract)\b");
pattern!(SQUARED, r"\bsquared\b");
pattern!(CUBED, r"\bcubed\b");
pattern!(
    POWER,
    r"\b(?:to the power of|to the power|raised to the power of|raised to the|raised to)\b"
);
pattern!(NUMBER, r"^[0-9]*\.?[0-9]+(?:[eE][+-]?[0-9]+)?");
pattern!(
    PERCENT,
    r"([0-9]+(?:\.[0-9]+)?)\s*(?:%|percent|per cent)\s*(?:service(?:\s+charge)?|tax|charge|vat)?\s*(of|off|on(?:\s+top\s+of)?)\s*(?:(?:my|our|the|a|an)\s+)?(?:(?:bill|total|check|tab)\s+of\s+)?([0-9,.]+)"
);
pattern!(
    PERCENT_REVERSE,
    r"what(?:'s| is)\s*([0-9]+(?:\.[0-9]+)?)\s*(of|off)\s*([0-9,.]+)\s*%"
);
pattern!(PERCENT_TIP, r"(?:%|percent|per cent)\s*(?:tip|gratuity)\b");
pattern!(
    PERCENT_AFTER,
    r"(?:costs?|priced at|is|are|was)\s+(?:£|\$)?([0-9,.]+)\D.*?([0-9]+(?:\.[0-9]+)?)\s*(?:%|percent|per cent)\s*off"
);
pattern!(
    ROOT,
    r"\b(square root|cube root|sqrt|root)\s*(?:of|is)?\s*(-?[0-9][0-9,.]*)"
);
pattern!(PRIME, r"\bis\s+([0-9]+)\s+(?:a\s+)?prime\b");
pattern!(
    REVERSE_CONVERSION,
    r"\bhow (?:many|much) (\w+) (?:are(?: there)? in|in|to|into|is|as) (?:a |an )?(\d+(?:\.\d+)? )?(\w+)\b"
);
pattern!(
    FORWARD_CONVERSION,
    r"\b(\d+(?:\.\d+)?)\s*(\w+?)\s+(?:to|into|in)\s+(\w+)\b"
);

fn finite(val: f64) -> Option<f64> {
    val.is_finite().then_some(val)
}

fn read_number(text: &str) -> Option<f64> {
    finite(text.trim_end_matches('.').replace(',', "").parse().ok()?)
}

pub(super) fn number(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        let integer = format!("{:.0}", if value == 0.0 { 0.0 } else { value });
        let (sign, digits) = integer
            .strip_prefix('-')
            .map_or(("", integer.as_str()), |v| ("-", v));
        let mut result = String::with_capacity(integer.len() * 2);
        result.push_str(sign);
        for (i, ch) in digits.chars().enumerate() {
            if i > 0 && (digits.len() - i) % 3 == 0 {
                result.push(',');
            }
            result.push(ch);
        }
        return result;
    }
    let mut rendered = format!("{value:.2}");
    for places in 2..=4 {
        let exact = format!("{value:.places$}");
        if exact.parse::<f64>().is_ok_and(|n| (value - n).abs() < 1e-9) {
            rendered = exact;
            break;
        }
    }
    rendered
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}

fn percent(clean: &str) -> Option<String> {
    if PERCENT_TIP.is_match(clean) {
        return None;
    }
    let (part, kind, total) = if let Some(m) = PERCENT_AFTER.captures(clean) {
        (read_number(&m[2])?, "off".to_owned(), read_number(&m[1])?)
    } else {
        let m = PERCENT
            .captures(clean)
            .or_else(|| PERCENT_REVERSE.captures(clean))?;
        (read_number(&m[1])?, m[2].to_owned(), read_number(&m[3])?)
    };
    let amount = finite(total * part / 100.0)?;
    Some(if kind == "off" {
        format!(
            "{}% off {} leaves {} (you save {}).",
            number(part),
            number(total),
            number(total - amount),
            number(amount)
        )
    } else {
        format!(
            "{}% of {} is {}.",
            number(part),
            number(total),
            number(amount)
        )
    })
}

fn prime(clean: &str) -> Option<String> {
    let n: u64 = PRIME.captures(clean)?[1].parse().ok()?;
    if n < 2 {
        return Some(format!("{n} is not prime: primes start at 2."));
    }
    // Bound work on arbitrary input; the full large-integer reader is not ported.
    if n > 1_000_000_000_000 {
        return None;
    }
    let divisor = if n > 2 && n.is_multiple_of(2) {
        Some(2)
    } else {
        (3..=n.isqrt()).step_by(2).find(|&d| n.is_multiple_of(d))
    };
    Some(if let Some(d) = divisor {
        format!("No: {n} is {d} × {}, so it is not prime.", n / d)
    } else {
        format!("Yes: nothing but 1 and {n} divides {n}, so it is prime.")
    })
}

fn root(clean: &str) -> Option<String> {
    if clean.contains("fourth root") {
        return None;
    }
    let m = ROOT.captures(clean)?;
    let val = read_number(&m[2])?;
    let cube = &m[1] == "cube root";
    if val < 0.0 && !cube {
        return Some("A negative number has no real square root.".into());
    }
    let result = if cube { val.cbrt() } else { val.sqrt() };
    let name = if cube { "cube root" } else { "square root" };
    let exact = (result - result.round()).abs() < 1e-9;
    Some(format!(
        "The {name} of {} is {}{}.",
        number(val),
        if exact { "" } else { "about " },
        number(result)
    ))
}

fn unit(unit: &str) -> Option<(u8, f64, &str, &str)> {
    Some(match unit {
        "ml" | "millilitre" | "millilitres" | "milliliter" | "milliliters" => (0, 1., "ml", "ml"),
        "l" | "litre" | "litres" | "liter" | "liters" => (0, 1000., "litre", "litres"),
        "teaspoon" | "teaspoons" | "tsp" => (0, 5., "teaspoon", "teaspoons"),
        "tablespoon" | "tablespoons" | "tbsp" => (0, 15., "tablespoon", "tablespoons"),
        "g" | "gram" | "grams" => (1, 1., "g", "g"),
        "kg" | "kilogram" | "kilograms" => (1, 1000., "kg", "kg"),
        "oz" | "ounce" | "ounces" => (1, 28.349523125, "ounce", "ounces"),
        "lb" | "pound" | "pounds" => (1, 453.59237, "pound", "pounds"),
        "mm" | "millimetre" | "millimetres" => (2, 1., "mm", "mm"),
        "cm" | "centimetre" | "centimetres" => (2, 10., "cm", "cm"),
        "m" | "metre" | "metres" => (2, 1000., "metre", "metres"),
        "km" | "kilometre" | "kilometres" => (2, 1_000_000., "km", "km"),
        "inch" | "inches" | "in" => (2, 25.4, "inch", "inches"),
        "foot" | "feet" | "ft" => (2, 304.8, "foot", "feet"),
        "mile" | "miles" | "mi" => (2, 1_609_344., "mile", "miles"),
        _ => return None,
    })
}

fn conversion(clean: &str) -> Option<String> {
    let (n, from, to) = if let Some(m) = REVERSE_CONVERSION.captures(clean) {
        (
            m.get(2)
                .map_or(Some(1.), |m| read_number(m.as_str().trim()))?,
            m.get(3)?.as_str(),
            m.get(1)?.as_str(),
        )
    } else {
        let m = FORWARD_CONVERSION.captures(clean)?;
        (read_number(&m[1])?, m.get(2)?.as_str(), m.get(3)?.as_str())
    };
    let from = unit(from)?;
    let to = unit(to)?;
    if n == 1. && from.2 == "ounce" && to.2 == "g" {
        return Some("There are about 28.35 grams in an ounce, so 100g is about 3.5oz.".into());
    }
    if from.0 != to.0 || from.1 == to.1 {
        return None;
    }
    let value = finite(n * from.1 / to.1)?;
    let approximate = if (value * 100.).round() / 100. == value {
        ""
    } else {
        "about "
    };
    let lhs = if n.abs() == 1. { from.2 } else { from.3 };
    let rhs = if value.abs() == 1. { to.2 } else { to.3 };
    Some(format!(
        "{} {lhs} is {approximate}{} {rhs}.",
        number(n),
        number(value)
    ))
}

fn arithmetic(clean: &str) -> Option<String> {
    if !clean.bytes().any(|c| c.is_ascii_digit()) || clean.len() > 256 {
        return None;
    }
    let removed = ASK.replace_all(clean, " ").replace([',', '?'], "");
    let divided = DIVIDE.replace_all(&removed, |m: &Captures<'_>| format!("{}/{}", &m[1], &m[2]));
    let mut expr = divided.into_owned();
    for (pattern, op) in [
        (&*MULTIPLY, "*"),
        (&*OVER, "/"),
        (&*PLUS, "+"),
        (&*MINUS, "-"),
        (&*SQUARED, "**2"),
        (&*CUBED, "**3"),
        (&*POWER, "**"),
    ] {
        if let Cow::Owned(replaced) = pattern.replace_all(&expr, op) {
            expr = replaced;
        }
    }
    let expr = expr.replace('^', "**");
    let expr = expr.trim();
    if expr.len() > 80
        || !expr.bytes().any(|b| b"+-*/".contains(&b))
        || !expr
            .bytes()
            .all(|b| b.is_ascii_digit() || b.is_ascii_whitespace() || b"+-*/().".contains(&b))
    {
        return None;
    }
    let mut parser = Expression { remaining: expr };
    let value = parser.expr(0, 0)?;
    if !parser.remaining.trim().is_empty() {
        return None;
    }
    Some(format!("That's {}.", number(value)))
}

struct Expression<'a> {
    remaining: &'a str,
}
impl Expression<'_> {
    fn expr(&mut self, min: u8, depth: u8) -> Option<f64> {
        if depth > 64 {
            return None;
        }
        self.remaining = self.remaining.trim_start();
        let mut lhs = if let Some(rest) = self.remaining.strip_prefix('(') {
            self.remaining = rest;
            let val = self.expr(0, depth + 1)?;
            self.remaining = self.remaining.trim_start().strip_prefix(')')?;
            val
        } else if let Some(rest) = self.remaining.strip_prefix('+') {
            self.remaining = rest;
            self.expr(5, depth + 1)?
        } else if let Some(rest) = self.remaining.strip_prefix('-') {
            self.remaining = rest;
            -self.expr(5, depth + 1)?
        } else {
            let found = NUMBER.find(self.remaining)?.as_str();
            self.remaining = &self.remaining[found.len()..];
            finite(found.parse().ok()?)?
        };
        loop {
            let remaining = self.remaining.trim_start();
            let (op, left, right) = if remaining.starts_with("**") {
                ("**", 6, 5)
            } else if let Some(c) = remaining.as_bytes().first() {
                match c {
                    b'+' => ("+", 1, 2),
                    b'-' => ("-", 1, 2),
                    b'*' => ("*", 3, 4),
                    b'/' => ("/", 3, 4),
                    _ => return Some(lhs),
                }
            } else {
                return Some(lhs);
            };
            if left < min {
                return Some(lhs);
            }
            self.remaining = &remaining[op.len()..];
            let rhs = self.expr(right, depth + 1)?;
            lhs = finite(match op {
                "+" => lhs + rhs,
                "-" => lhs - rhs,
                "*" => lhs * rhs,
                "/" => lhs / rhs,
                "**" if rhs.abs() <= 64.0 => lhs.powf(rhs),
                _ => return None,
            })?;
        }
    }
}

pub(crate) fn solve(clean: &str) -> Option<String> {
    if !clean.bytes().any(|b| b.is_ascii_digit()) {
        return super::units::convert(clean);
    }
    root(clean)
        .or_else(|| prime(clean))
        .or_else(|| percent(clean))
        .or_else(|| super::units::convert(clean))
        .or_else(|| conversion(clean))
        .or_else(|| arithmetic(clean))
}

pub(crate) fn carried_measure(clean: &str, memory: &crate::bot::Memory) -> Option<String> {
    super::units::carried(clean, &memory.turns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respects_precedence_and_bounds() {
        assert_eq!(
            solve("what is -2 ** 3 ** 2 + 4 / 2?").as_deref(),
            Some("That's -510.")
        );
        assert_eq!(solve("3 divided by 8").as_deref(), Some("That's 0.375."));
        assert_eq!(solve("1 / 0"), None);
        assert_eq!(solve("2 ^ 99"), None);
        assert_eq!(solve("my 3 kids play at 8"), None);
    }

    #[test]
    fn solves_supported_percentages_and_primes() {
        assert_eq!(
            solve("what is 17% of 240?").as_deref(),
            Some("17% of 240 is 40.8.")
        );
        assert_eq!(
            solve("is 91 a prime number?").as_deref(),
            Some("No: 91 is 7 × 13, so it is not prime.")
        );
    }

    #[test]
    fn converts_when_the_target_unit_precedes_the_quantity() {
        assert_eq!(
            solve("how many tablespoons is 30 ml?").as_deref(),
            Some("30 ml is 2 tablespoons.")
        );
    }
}
