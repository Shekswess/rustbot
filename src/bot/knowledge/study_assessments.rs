//! Language-study estimates, university marks and early handwriting.

use std::{borrow::Cow, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use regex::{Regex, RegexSet};

use crate::bot::{Exchange, Reading};

use super::clean_or_typed;

const LANGUAGE_HOURS: &[(&str, u16)] = &[
    ("spanish", 600),
    ("french", 750),
    ("italian", 600),
    ("portuguese", 600),
    ("dutch", 600),
    ("afrikaans", 600),
    ("swedish", 600),
    ("norwegian", 600),
    ("danish", 600),
    ("romanian", 600),
    ("german", 900),
    ("welsh", 900),
    ("irish", 900),
    ("greek", 1100),
    ("polish", 1100),
    ("russian", 1100),
    ("turkish", 1100),
    ("hebrew", 1100),
    ("hindi", 1100),
    ("urdu", 1100),
    ("punjabi", 1100),
    ("vietnamese", 1100),
    ("thai", 1100),
    ("persian", 1100),
    ("farsi", 1100),
    ("japanese", 2200),
    ("chinese", 2200),
    ("mandarin", 2200),
    ("cantonese", 2200),
    ("korean", 2200),
    ("arabic", 2200),
];
static LANGUAGES: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        LANGUAGE_HOURS
            .iter()
            .map(|(language, _)| format!(r"\b{language}\b")),
    )
    .expect("language names")
});
static LANGUAGE_CUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:languages?|fluent|fluency|fluently|bilingual|conversations?|speak|speaking|spoken)\b").unwrap()
});

fn language_named(text: &str) -> Option<(&'static str, u16)> {
    LANGUAGES.matches(text).iter().fold(None, |best, at| {
        let name = LANGUAGE_HOURS[at];
        if best.is_none_or(|(old, _): (&str, u16)| name.0.len() > old.len()) {
            Some(name)
        } else {
            best
        }
    })
}

pub(crate) fn names_language(reading: &Reading<'_>) -> bool {
    let text = format!(
        "{} {}",
        clean_or_typed(reading),
        reading.line.words.join(" ")
    )
    .to_lowercase();
    LANGUAGES.is_match(&text) || LANGUAGE_CUE.is_match(&text)
}

// Class-hour estimates are benchmarks from intensive instruction.
// https://fam.state.gov/FAM/03FAM/03FAM3910.html
pub(super) fn language(reading: &Reading<'_>, turns: &[Exchange]) -> Cow<'static, str> {
    let text = clean_or_typed(reading).to_lowercase();
    let (language, hours) = language_named(&text)
        .or_else(|| {
            turns
                .iter()
                .rev()
                .take(6)
                .find_map(|turn| language_named(&turn.user.to_lowercase()))
        })
        .unwrap_or(("Spanish, French or Italian", 600));
    let (initial, rest) = language.split_at(1);
    let language = format!("{}{rest}", initial.to_ascii_uppercase());
    let years = f64::from(hours) / 365.0;
    Cow::Owned(format!(
        "About {hours} hours is a rough study benchmark for {language}. At an hour a day that is about {years:.1} years of study time, and at twenty minutes a day it is about {:.1} years. These are planning figures based on intensive language training. How soon you can hold a conversation depends on what you already know and how often you practise speaking and listening.",
        3.0 * years
    ))
}

static MEASUREMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b\d+(?:[.,]\d+)?\s*(?:kg|kgs|kilo|kilos|g|gram|grams|gm|gms|lb|lbs|pound|pounds|st|stone|stones|l|litre|litres|ml|cl|cm|mm|metre|metres|km)\b").unwrap()
});
static TEMPERATURE: LazyLock<Lookaround> = LazyLock::new(|| {
    Lookaround::new(r"(?<![\w:.\d])(?<!\d\s)(?:[1-9]|[1-3][0-9]|40)\s*(?:degrees?\b|deg\b|°)")
        .unwrap()
});
static NAMED_QUALIFICATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:my|the|a|his|her|their|your)\s+degree\b|\bdegree\s+(?:in|course|exam)\b")
        .unwrap()
});
const MEASUREMENT_UNITS: &[&str] = &[
    "kg", "kgs", "kilo", "kilos", "g", "gram", "grams", "gm", "gms", "lb", "lbs", "pound",
    "pounds", "st", "stone", "stones", "l", "litre", "litres", "liter", "liters", "ml", "cl", "cm",
    "mm", "m", "metre", "metres", "meter", "meters", "km",
];
const NUMBER_WORDS: &[&str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
    "hundred",
];
const EXAM_WORDS: &[&str] = &[
    "exam",
    "exams",
    "examination",
    "gcse",
    "gcses",
    "a-level",
    "a-levels",
    "alevel",
    "alevels",
    "as-level",
    "as-levels",
    "mock",
    "mocks",
    "coursework",
    "resit",
    "resits",
    "retake",
    "retakes",
    "remark",
    "remarks",
    "module",
    "modules",
    "semester",
    "dissertation",
    "grade",
    "grades",
    "uni",
    "university",
    "college",
    "school",
    "sixth",
    "year",
    "paper",
    "papers",
    "results",
    "sats",
    "btec",
    "ib",
];

pub(crate) fn degree_mark(reading: &Reading<'_>) -> bool {
    let text = clean_or_typed(reading).to_lowercase();
    if reading.numbers.iter().any(|number| {
        number
            .unit
            .is_some_and(|unit| MEASUREMENT_UNITS.contains(&unit))
    }) || MEASUREMENT.is_match(&text)
    {
        return false;
    }
    let words = &reading.line.words;
    if !words
        .iter()
        .any(|word| matches!(word.as_str(), "degree" | "degrees"))
    {
        return true;
    }
    let number_words = words.iter().any(|word| {
        word.split(['-', ' '])
            .any(|part| NUMBER_WORDS.contains(&part))
    });
    !(number_words || TEMPERATURE.is_match(&text).unwrap_or(false))
        || words.iter().any(|word| EXAM_WORDS.contains(&word.as_str()))
        || NAMED_QUALIFICATION.is_match(&text)
}

const CLASS: &[(&str, u8, &str, &str)] = &[
    (
        "2:1",
        60,
        r"\b(?:2\s*:\s*1|2\.1|2\s1|upper second|upper-second)\b",
        "A 2:1 is upper second-class honours, commonly 60% to under 70%.",
    ),
    (
        "2:2",
        50,
        r"\b(?:2\s*:\s*2|2\.2|2\s2|lower second|lower-second)\b",
        "A 2:2 is lower second-class honours, commonly 50% to under 60%.",
    ),
    (
        "first",
        70,
        r"\b(?:first class|first-class|1st class)\b",
        "A first is first-class honours, commonly 70% or higher.",
    ),
    (
        "third",
        40,
        r"\b(?:third class|third-class|3rd class)\b",
        "A third is third-class honours, commonly 40% to under 50%.",
    ),
    (
        "distinction",
        0,
        r"\bdistinction\b",
        "A distinction is often the highest award on a taught master's, commonly 70% or higher.",
    ),
    (
        "merit",
        0,
        r"\bmerit\b",
        "A merit on a taught master's commonly means 60% to under 70%, above a pass and below a distinction.",
    ),
];
static CLASSES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    CLASS
        .iter()
        .map(|(_, _, pattern, _)| Regex::new(pattern).unwrap())
        .collect()
});
static BARE_CLASS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:a|the|my|get|got|need|want)\s+(first|third)\b").unwrap());
static NOT_BARE_CLASS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:first (?:year|term|attempt|time|day|week|draft|choice|job|child|baby|home|house|car)|third (?:year|term|attempt|time|week|child|baby|party))\b").unwrap()
});
static HALF_CLASS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(low|high)\s+(2\s*:\s*[12]|2\.[12]|upper second|lower second)\b").unwrap()
});
static DEFINE_CLASS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:mean|means|stands for|stand for|difference between|percentage|percent|what grade|what is|whats|what's|define)\b").unwrap()
});

const PASS_MARK: &str = "Many UK universities use 40% as the undergraduate pass mark and 50% for taught postgraduate modules. Common honours thresholds are 70% for a first, 60% for a 2:1, 50% for a 2:2 and 40% for a third. Check your course regulations for credit weighting, progression, any must-pass modules and resit caps.";
const CHECK_MARKS: &str = "Check your university's course regulations for how it weights credits, handles borderline results and awards the degree.";

pub(super) fn university(reading: &Reading<'_>) -> Cow<'static, str> {
    let line = clean_or_typed(reading).to_lowercase();
    if !DEFINE_CLASS.is_match(&line) || line.trim_start().starts_with("how ") {
        return Cow::Borrowed(PASS_MARK);
    }
    let mut classes: Vec<_> = CLASSES
        .iter()
        .enumerate()
        .filter_map(|(at, re)| re.find(&line).map(|span| (span.start(), at)))
        .collect();
    if classes.is_empty() && !NOT_BARE_CLASS.is_match(&line) {
        classes.extend(BARE_CLASS.captures_iter(&line).map(|found| {
            let at = if &found[1] == "first" { 2 } else { 3 };
            (found.get(0).unwrap().start(), at)
        }));
    }
    classes.sort_unstable();
    let Some(&(_, at)) = classes.first() else {
        return Cow::Borrowed(PASS_MARK);
    };
    let (name, mark, _, description) = CLASS[at];
    if let Some(half) = HALF_CLASS.captures(&line)
        && matches!(name, "2:1" | "2:2")
    {
        let bottom = if &half[1] == "low" { mark } else { mark + 5 };
        return Cow::Owned(format!(
            "A {} {name} informally means around {bottom}% to {}%. {description} {CHECK_MARKS}",
            &half[1],
            bottom + 4,
        ));
    }
    if let Some(&(_, next)) = classes.iter().find(|(_, other)| *other != at) {
        let (second, second_mark, _, second_description) = CLASS[next];
        let gap = if mark != 0 && second_mark != 0 {
            format!(
                " The typical thresholds for {name} and {second} are {} percentage points apart.",
                mark.abs_diff(second_mark)
            )
        } else {
            String::new()
        };
        return Cow::Owned(format!(
            "{description} {second_description}{gap} {CHECK_MARKS}"
        ));
    }
    Cow::Owned(format!("{description} {CHECK_MARKS}"))
}

static CHILD_AGE: LazyLock<Vec<Regex>> =
    LazyLock::new(|| {
        [
        r"\b(\d{1,2})\s*(?:year|yr)s?[\s-]*(?:old|young)\b",
        r"\b(?:is|at|aged|age|turned|nearly)\s+(\d{1,2})\b",
        r"\b(\d{1,2})\s*(?:and|,)\s*(?:cant|can't|cannot|doesnt|doesn't|still|only|hasnt|hasn't)\b",
    ].into_iter().map(|pattern| Regex::new(pattern).unwrap()).collect()
    });

// https://www.somersetft.nhs.uk/children-and-young-peoples-therapy-service/handwriting/
pub(super) fn handwriting(reading: &Reading<'_>) -> &'static str {
    let line = clean_or_typed(reading).to_lowercase();
    let age = CHILD_AGE.iter().find_map(|pattern| {
        let age = pattern.captures(&line)?[1].parse::<u8>().ok()?;
        (1..=18).contains(&age).then_some(age)
    });
    match age {
        Some(1..=4) => {
            "At that age, keep it playful. Try playdough, threading and drawing with chunky chalk, and let your child copy or trace their name if they want to. Keep it short. If you are concerned about how they use their hands in everyday activities, ask their nursery or health visitor."
        }
        Some(7..) => {
            "Ask the teacher what your child can write in class and whether they find other tasks with a pencil or their hands difficult. Ask the school about handwriting support or a conversation with the SENCO. A GP or school can help decide if hearing, vision or an occupational therapy assessment is appropriate. Keep practice brief and pressure-free."
        }
        Some(_) => {
            "Give your child short, relaxed practice with a pencil that is comfortable to hold, or use chalk and playdough to build confidence. Ask the teacher what they see in class. If writing remains difficult or causes frustration, agree with the teacher on what to try and when to review progress."
        }
        None => {
            "Try drawing with chunky chalk, playing with playdough or briefly tracing their name when they are interested. Ask their nursery or teacher what they can do independently and how you can help. If it is persistently hard or affects everyday tasks, ask the teacher, health visitor or GP about support."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn study_assessments_understand_units_classes_and_named_languages() {
        assert!(!degree_mark(&Reading::new("Is 2.2 kilos good?")));
        assert!(!degree_mark(&Reading::new(
            "It is 35 degrees here, what now?"
        )));
        assert!(degree_mark(&Reading::new("What does a 2:1 degree mean?")));
        assert!(university(&Reading::new("What does a low 2:1 mean?")).contains("60% to 64%"));
        assert!(!names_language(&Reading::new(
            "How long to learn the guitar?"
        )));
        assert!(language(&Reading::new("How long to learn Japanese?"), &[]).contains("2200 hours"));
        assert!(
            handwriting(&Reading::new("My son is 7 and can't write his name.")).contains("SENCO")
        );
    }
}
