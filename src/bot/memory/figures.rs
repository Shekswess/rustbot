//! Short amount recalls from PyBot's repeat.the_figure_they_gave.
use crate::bot::{Memory, Reading, compose};
use fancy_regex::Regex as ContextRegex;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! rule {
    ($name:ident, $regex:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($regex).unwrap());
    };
}

rule!(
    BARE_ASK,
    concat!(
        r"\bhow\s+much\s+(?:was|is|were|are)\s+(?:it|that|this|they|those|these)\s*(?:again|now|then)?[.?!\s]*$",
        r"|\bhow\s+much\s+(?:did|do|does)\s+(?:i|we)\s+(?:say|need|want|have|owe|spend|agree)\s*",
        r"(?:again|now|then|already|so\s+far|in\s+total|altogether|for\s+it)?[.?!\s]*$"
    )
);
rule!(
    PAY_ASK,
    r"^(?:(?:and|so|but|oh|ok|okay)[, ]+)*how much (?:did|do|does) (?:i|we) (?:pay|earn|make)(?: (?:again|now|then|currently))?[.?!]*$"
);
rule!(
    NAMED_ASK,
    r"^(?:(?:and|so|but|oh|ok|okay)[, ]+)*what(?:'s|s| is| was) (?:my|our) (rent|bill|phone bill|gas bill|electricity bill|water bill|price|cost|excess|mortgage|deposit|fee)(?: (?:again|now|then))?[.?!]*$"
);
rule!(
    PAY,
    r"(?i)\b(?:earn|earns|earning|earnings|pay|pays|paid|wage|wages|salary|salaries|income|make|makes|making|made|take home)\b"
);
rule!(
    QUESTION,
    concat!(
        r"(?i)^(?:\W*(?:and|but|so|ok|okay|also|then|well|now|hmm|oh|yeah|yes|right|actually|sorry|plus|just)\b[\s,]*)*",
        r"\W*(?:how|what|which|when|where|who|why|do|did|does|can|could|should|would|will|shall|is|are|was|were|have|has)\b"
    )
);
rule!(
    OTHER,
    r"(?i)^\s*(?:(?:my|our) (?:mum|mom|mother|dad|father|sister|brother|son|daughter|wife|husband|partner|friend|boss|neighbour)|he|she|they|you)\b"
);
rule!(
    CORRECTION,
    concat!(
        r"(?i)^[\s,.\-]*(?:(?:oh|ok|okay|and|but|so|well|sorry|no|nope|actually|wait|oops|correction|my mistake|it'?s)\b[\s,;:.-]*){1,3}",
        r"|^[\s,.\-]*(?:make\s+(?:it|that)|i\s+mean|we\s+mean|scratch\s+that|i\s+meant|we\s+meant)\b"
    )
);
rule!(
    A_RATE,
    r"(?i)(?:\d|£|\$|€)\s*$|\b(?:once|twice|thrice|times|per|each|every|a|an)\s*$"
);
rule!(AFTER_TIME, r"(?i)^\s*(?:before|after|past|prior\s+to)\b");
rule!(
    HAS_TIME,
    r"(?i)\b(?:have|has|had|got|there's|there are|there is)\b"
);
rule!(MORE, r"(?i)\b(?:more|extra|other|others|fewer|less)\b");
rule!(
    NUMBER,
    r"(?i)\b(?:\d[\d,]*(?:\.\d+)?|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety|hundred|thousand)\b"
);
const JOINERS: &str = "a an the and or but so of to in on at for with from by as then that this is are was were it them us you per because cos coz cuz cause since";

static FIGURE: LazyLock<ContextRegex> = LazyLock::new(|| {
    ContextRegex::new(concat!(
        r"(?i)(?P<money>£\s?\d[\d,]*(?:\.\d+)?(?:\s?(?:k|grand|thousand|m)\b)?|\b(?<![\d.])\d[\d,]*(?:\.\d+)?\s?(?:grand|k|quid|pounds?|dollars?|euros?|bucks)\b)",
        r"|(?P<time>\b(?<![\d.])\d[\d,]*(?:\.\d+)?\s*(?:seconds?|secs?|minutes?|mins?|hours?|hrs?|days?|weeks?|wks?|months?|years?|nights?)\b",
        r"|\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\s+(?:seconds?|minutes?|hours?|days?|weeks?|months?|years?|nights?)\b",
        r"|\b(?:about\s+|around\s+|nearly\s+|over\s+|under\s+)?(?:a|an)\s+(?:second|minute|hour|day|week|month|year|night)\b",
        r"|\b(?:a\s+)?(?:couple\s+of|few|couple)\s+(?:seconds?|minutes?|hours?|days?|weeks?|months?|years?|nights?)\b",
        r"|\bhalf\s+an?\s+(?:hour|day|week|month)\b)",
        r"|(?P<share>\b(?<![\d.])\d[\d,]*(?:\.\d+)?\s?(?:%|per\s?cent\b|percent\b))",
        r"|(?P<count>\b(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|(?<![\d.])\d[\d,]*(?:\.\d+)?)\s+[a-z][a-z-]{2,}\b)",
        r"|(?P<amount>\b(?<![\d.])\d[\d,]{2,}\b)"
    )).unwrap()
});

#[derive(PartialEq, Eq)]
enum Kind {
    Amount,
    Time,
}

struct Figure {
    kind: Kind,
    text: String,
}

fn figures(line: &str) -> Vec<Figure> {
    if line.contains('?') || QUESTION.is_match(line) || OTHER.is_match(line) {
        return Vec::new();
    }
    FIGURE
        .captures_iter(line)
        .filter_map(|found| {
            let found = found.ok()?;
            let whole = found.get(0)?;
            let before = &line[..whole.start()];
            let after = &line[whole.end()..];
            let kind = if found.name("time").is_some() {
                if A_RATE.is_match(before)
                    || (AFTER_TIME.is_match(after) && !HAS_TIME.is_match(before))
                {
                    return None;
                }
                Kind::Time
            } else {
                Kind::Amount
            };
            if found.name("share").is_some() {
                let number = NUMBER.find(whole.as_str())?.as_str();
                return Some(Figure {
                    kind,
                    text: format!("{number} per cent"),
                });
            }
            let mut words = whole.as_str().split_whitespace().collect::<Vec<_>>();
            if found.name("count").is_some() && words.get(1).is_some_and(|word| MORE.is_match(word))
            {
                return None;
            }
            while words.len() > 1
                && words.last().is_some_and(|word| {
                    JOINERS
                        .split_whitespace()
                        .any(|one| word.eq_ignore_ascii_case(one))
                })
            {
                words.pop();
            }
            let text = words.join(" ");
            let text = text.trim_end_matches([' ', ',', '.', ';', ':']);
            (!text.is_empty()).then(|| Figure {
                kind,
                text: text.to_owned(),
            })
        })
        .collect()
}

fn named(line: &str, name: &str, pay: bool) -> bool {
    if pay {
        return PAY.is_match(line);
    }
    let words = line.to_ascii_lowercase();
    name.split_whitespace().all(|term| {
        words
            .split(|letter: char| !letter.is_ascii_alphabetic())
            .any(|word| word == term)
    })
}

fn one(line: &str, pay: bool) -> Option<String> {
    let figures = figures(line);
    if pay {
        return figures
            .into_iter()
            .find(|one| one.kind == Kind::Amount)
            .map(|one| one.text);
    }
    if figures.iter().any(|one| one.kind != Kind::Amount) {
        return None;
    }
    figures.into_iter().next().map(|one| one.text)
}

fn named_amount(lines: &[&str], name: &str, pay: bool) -> Option<String> {
    let (index, mut value) = lines.iter().enumerate().rev().find_map(|(i, line)| {
        if !named(line, name, pay) {
            return None;
        }
        Some((i, one(line, pay)?))
    })?;
    for &line in &lines[index + 1..] {
        if CORRECTION.is_match(line) && !QUESTION.is_match(line) && !line.contains('?') {
            if let Some(new) = one(line, pay) {
                value = new;
            } else if NUMBER.is_match(line) {
                return None;
            }
        }
    }
    Some(value)
}

fn bare_amount(lines: &[&str]) -> Option<String> {
    let found: Vec<_> = lines
        .iter()
        .flat_map(|line| {
            figures(line)
                .into_iter()
                .filter(|one| one.kind == Kind::Amount)
                .map(|one| (one.text, CORRECTION.is_match(line)))
        })
        .collect();
    let (last, _) = found.last()?;
    let last_correction = found.iter().rfind(|(_, corrected)| *corrected);
    if found
        .iter()
        .any(|(value, _)| !last.eq_ignore_ascii_case(value))
        && !last_correction.is_some_and(|(value, _)| value == last)
    {
        return None;
    }
    Some(last.clone())
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = &reading.clean;
    let name = NAMED_ASK.captures(line);
    let pay = PAY_ASK.is_match(line);
    let bare = BARE_ASK.is_match(line);
    if name.is_none() && !pay && !bare {
        return None;
    }
    let lines: Vec<_> = memory
        .turns
        .iter()
        .rev()
        .take(8)
        .rev()
        .map(|turn| turn.user.as_str())
        .collect();
    let amount = if let Some(subject) = name {
        named_amount(&lines, &subject[1], false)?
    } else if pay {
        named_amount(&lines, "", true)?
    } else {
        bare_amount(&lines)?
    };
    let plain = format!("You said {amount}.");
    let alt = if !bare
        && NUMBER
            .find(&amount)
            .is_some_and(|number| number.as_str() == amount)
    {
        format!("{amount}, as you said.")
    } else {
        let mut letters = amount.chars();
        let first: String = letters.next()?.to_uppercase().collect();
        format!("{first}{} — that's what you told me.", letters.as_str())
    };
    Some(compose::pick(&[&plain, &alt], memory, "").into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bare_amount_uses_the_latest_correction_only_when_it_is_the_latest_figure() {
        assert_eq!(
            bare_amount(&["My rent is 900 a month", "It's 32 a month for 20 gig"]).as_deref(),
            Some("20 gig")
        );
        assert_eq!(
            bare_amount(&["My rent is 900 a month", "My phone has 20 gig"]),
            None
        );
    }
}
