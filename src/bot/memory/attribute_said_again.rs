//! Recall of attributes and values from the user's own lines.
use super::filler::without_filler;
use crate::bot::{Memory, Reading, understanding::kinds::Kind};
use regex::{Captures, Regex, RegexBuilder};
use std::{collections::BTreeSet, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $regex:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| {
            RegexBuilder::new($regex)
                .case_insensitive(true)
                .build()
                .expect(stringify!($name))
        });
    };
}

const ATTRIBUTES: &str = "shoe size|ring size|weight|height|colour|budget|number|color|brand|model|breed|size|name|make|plate|age";
const MONEY_WORDS: &str =
    "earnings|salary|income|budget|savings|deposit|wages|price|cost|rent|wage|pay";
const MONEY_QUALIFIER: &str = "current|currently|annual|annually|monthly|weekly|hourly|yearly|real|actual|actually|basic|gross|net|total|usual|normal|present|own";
const SALARY_WORDS: &[&str] = &["salary", "pay", "wage", "wages", "income", "earnings"];
const MONTHS_AND_DAYS: &str = "january february march april may june july august september october november december monday tuesday wednesday thursday friday saturday sunday";
const OTHER_TIMES: &str = "weekend tonight tomorrow today morning afternoon evening night midday noon christmas easter summer winter spring autumn";
const SLOT_STOP: &str = "i we you me us my our it its the a an to be been being was were are is am do did does say said about that this of for in on at and but so then just again please oh ok okay actually";
const QUESTION_WORDS: &str = "best better option options chance chances odds next plan plans point problem issue deal story time day week month year";
const PAY_ASK_WORDS: &str = "how what much many do did does i im my the a an is was are were for of to in on at it that this me you your get got getting pay paid pays paying payment payments wage wages salary salaries earn earns earned earning make makes made making take home takehome and or per plus now again current currently exact exactly actual actually latest today so far have has had been like think about guess roughly all just really total figure number amount sum much there then";

pattern!(WORD, r"[a-z']+");
pattern!(TIME_WORD, r"[a-z'’]+");
pattern!(
    ASK,
    &format!(r"\bwhat(?:\s+is|\s*'s|s)?\s+(?P<attr>{ATTRIBUTES})\s+am\s+i\b")
);
pattern!(
    OF_MINE,
    &format!(r"\bwhat(?:\s+is|\s*'s|s)?\s+my\s+(?P<attr>{ATTRIBUTES})\b")
);
pattern!(
    WHATS_MY,
    r"\bwhat(?:\s+is|\s*'s|s)?\s+my\s+(?P<attr>[a-z][a-z' -]{1,24}?)\b"
);
pattern!(
    A_FIGURE,
    r"(?:[£$€]\s*\d[\d,.]*|\b\d[\d,.]*\s*(?:a|per|each)\s+(?:month|week|year|fortnight|day)\b|\b\d[\d,.]*\s*(?:quid|pounds?|dollars?|euros?|pm)\b)"
);
pattern!(
    NOT_A_COST,
    r"\b(?:earn|earns|earned|earning|salary|wage|wages|take[- ]home|save|saves|saved|saving|work|works|worked|working|shift|shifts|eat|eats|ate|eating|drink|drinks|drank|drinking|run|runs|ran|running|walk|walks|walked|walking|swim|swims|swam|swimming|sleep|sleeps|slept|sleeping|study|studies|studied|studying|read|reads|reading)\b"
);
pattern!(
    PAY_ASK,
    r"\b(?:what|how\s+much)\b[^.!?]*?\b(?:am|was|are|were|do|did)\s+i\b[^.!?]*?\b(?:pay|paying|paid|cost|costs|costing)\b"
);
pattern!(
    COST_ASK,
    r"^\W*(?:and\s+|so\s+|ok\s+|okay\s+)?how\s+much\s+(?:is|was)\s+it\b"
);
pattern!(
    VALUE_FRAME,
    r"^(?:you(?:'re| are| were| have|'ve)|your .+? is|your|it(?:'s|s)|you (?:pay|paid)|(?:it )?costs?) (.+)$"
);
pattern!(
    WHEN_ASK,
    r"\b(?:did|do)\s+(?:i|we)\s+say\s+when\b(.*)|\bwhen\s+did\s+(?:i|we)\s+say\b(.*)|\bwhen\s+(?:was|is)\s+it\s+(?:that\s+)?(?:i|we)\s+said\b(.*)"
);
pattern!(
    WHERE_ASK,
    r"\b(?:did|do)\s+(?:i|we)\s+say\s+where\b(.*)|\bwhere\s+did\s+(?:i|we)\s+say\b(.*)"
);
pattern!(
    TIME_SPAN,
    r"\b(?:next|last|this)\s+(?:week|month|year|summer|winter|spring|autumn)\b|\bin\s+(?:about\s+)?\d+\s+(?:days?|weeks?|months?|years?)\b|\b\d{1,2}(?:[.:]\d\d)?\s*(?:am|pm)\b|\bat\s+\d{1,2}\b"
);
pattern!(
    PLACE,
    r"\b(?:near|next to|close to|round the corner from|opposite)\s+(?P<place>[a-z][a-z'’]*(?:\s+[a-z][a-z'’]*){0,3})"
);
pattern!(
    SIZE_ASK,
    r"\b(?:how\s+(?:big|large|wide|long|tall|high|deep)|what\s+(?:size|dimensions?|measurements?)|how\s+many\s+(?:metres|meters|feet|foot|ft|centimetres|centimeters|cm|square\s+(?:metres|meters|feet)))\b(?P<rest>[^?.!]*)"
);
pattern!(
    SIZE_ADVICE,
    r"\b(?:should|shall|would|could|can|might|will|need|needs|enough|recommend|best)\b"
);
pattern!(
    A_SIZE,
    concat!(
        r"(?P<measure>\b\d+(?:[.,]\d+)?\s*(?:by|x|×)\s*\d+(?:[.,]\d+)?\s*(?:m|metres?|meters?|ft|feet|foot|cm|centimetres?|centimeters?|inches|inch)\b",
        r"|\b\d+(?:[.,]\d+)?\s*(?:square|sq)\s*(?:metres?|meters?|feet|foot|ft|m)\b",
        r"|\b\d+(?:[.,]\d+)?\s*(?:m|metres?|meters?|cm|centimetres?|centimeters?|ft|feet|foot|inches|inch|litres?|liters?|ml)\b)(?P<after>\s+[a-z][a-z'-]*)?"
    )
);
pattern!(
    PRONOUN_LINE,
    r"^\s*(?:its|it is|it's|that's|thats|theres|there's|theyre|they are|hes|shes|were|we are)\b"
);
pattern!(
    MONEY_ASK,
    &format!(
        concat!(
            r"\bwhat(?:'s| is| was|s)?\s+(?:my|our|their|his|her)\s+(?:(?:{qualifier})\s+)?({money})\b",
            r"|\bhow much\s+(?:was|is|were|did\s+(?:i|we)\s+say)\s+(?:my|our|the|their|his|her)\s+(?:(?:{qualifier})\s+)?({money})\b"
        ),
        qualifier = MONEY_QUALIFIER,
        money = MONEY_WORDS
    )
);
pattern!(
    A_PER_YEAR,
    r"\b\d+(?:\.\d+)?\s*k\b(?:\s*(?:a|per)\s*(?:year|annum))?|£?\s?\d[\d,]{3,}\s*(?:a|per)\s*(?:year|annum)\b"
);
pattern!(
    A_MONEY,
    r"(?:£\s?\d[\d,]*(?:\.\d\d)?|\b\d[\d,]*(?:\.\d+)?\s*(?:pounds?|quid|dollars?|euros?|grand)\b|\b\d[\d,]{2,}\b)(?:\s*(?:a|per)\s*(?:month|week|year|fortnight))?"
);
pattern!(
    AGE_ASK,
    concat!(
        r"\b(?:what|how\s+old)\b[^.?!]{0,40}?\b(?:did|do)\s+i\s+(?:say|tell\s+you|told\s+you|think|thought|mention|mentioned)\b[^.?!]{0,30}?\b(?:age|old)\b",
        r"|\bwhat\s+(?:was|is)\s+(?:his|her|their|my|our|its)\s+age\b",
        r"|\bhow\s+old\s+(?:did|do)\s+i\s+(?:say|tell\s+you|told\s+you|think|mention)\b"
    )
);
pattern!(
    AGE_OWNER,
    r"\b(his|her|their|my|our|its)\b[^.?!]{0,12}?\b(?:age|old)\b"
);
pattern!(AGE_PRONOUN, r"\b(he|she|they)\b");
pattern!(AGE_SELF, r"\b(?:i|im|i'm|am)\b");
pattern!(
    AN_AGE_CLAUSE,
    r"(?:^|[,;]|\bis\b|\bwas\b|\bim\b|\bi'm\b|\bam\b|\b-\s*)\s*(?P<n>\d{1,3})\s*(?:$|[,.;!?]|\s+and\b|\s+but\b)"
);
pattern!(
    NAMED_AGE,
    r"\b(?:my|our)\s+(?P<who>[a-z]+)\s+(?P<was>is|was)\s+(?P<n>\d{1,3})\b"
);
pattern!(
    SOMEBODY,
    r"\b(?:i|im|i'm|me|myself|he|she|him|her|his|they|them|their|my|our|son|daughter|lad|girl|boy|kid|kids|baby|toddler|mum|mom|dad|husband|wife|partner|brother|sister|nan|gran|grandad|dog|cat|pup|puppy|labrador|retriever|terrier|shepherd|spaniel|collie|hound|mutt|rabbit|hamster|horse)\b"
);
pattern!(A_COUNT_OF_PEOPLE, r"\b\d+\s+(?:of\s+us|people|adults)\b");
pattern!(
    OFTEN_ASK,
    concat!(
        r"\bhow\s+often\b[^.?!]{0,30}?\b(?:did|do)\s+i\s+(?:say|tell\s+you|told\s+you|mention)\b",
        r"|\bhow\s+many\s+times\b[^.?!]{0,30}?\b(?:did|do)\s+i\s+(?:say|tell\s+you|mention)\b",
        r"|\bhow\s+often\s+(?:was|is)\s+it\b[^.?!]{0,12}?\bagain\b"
    )
);
pattern!(
    A_HABIT_SAID,
    concat!(
        r"\b(?:(?:once|twice|three|four|five|six|seven|eight|nine|ten|\d+)\s+times?\s+(?:a|per|each)\s+(?:day|week|month|year)",
        r"|(?:once|twice)\s+(?:a|per)\s+(?:day|week|month|year)|every\s+(?:day|week|weekend|morning|evening|night)|most\s+(?:days|weeks|mornings|evenings)|\bdaily\b)"
    )
);
pattern!(
    OFF_WITH_ASK,
    r"\bwhat\s+(?:was|is)\s+i\s+off\b|\bwhat\s+did\s+i\s+say\s+i\s+was\s+off\b"
);
pattern!(
    OFF_WITH_SAID,
    r"\b(?:off\s+(?:sick|work|ill)|been\s+off|signed\s+off)\b[^.?!]*?\bwith\s+(?P<thing>an?\s+)?(?P<what>[a-z][a-z' -]{3,30})\s*$"
);
pattern!(
    SAID_ASK,
    concat!(
        r"^(?:and\s+|so\s+|but\s+|also\s+|oh\s+|ok\s+|okay\s+)?",
        r"(?:(?:what|which)\s+(?P<attr>[a-z][a-z' -]{1,20}?)\s+(?:did|do|have)\s+(?:i|we)\s+(?:say|give|mention|tell\s+you)|how\s+(?P<how>[a-z]+)\s+did\s+(?:i|we)\s+say)\b[^?.!]*$"
    )
);
pattern!(
    SIZE_VALUE,
    r"\b(?:xs|s|m|l|xl|xxl|xxxl|small|medium|large|extra\s+large|size\s+\d{1,2}|(?:a\s+)?\d{1,2}\s*(?:-|to)\s*\d{1,2})\b"
);
pattern!(
    LENGTH_VALUE,
    r"\b(?:long|short|shoulder[- ]length|past\s+(?:my\s+)?shoulders|bob|crop|pixie|just\s+(?:above|below)\s+\w+|\d+\s*(?:cm|inches|inch))\b"
);
pattern!(
    WEIGHT_VALUE,
    r"\b\d+(?:[.,]\d+)?\s*(?:kg|kilo|kilos|kilograms|st|stone|stones|lbs?|pounds)\b"
);
pattern!(
    AGE_VALUE,
    r"\b(?:\d{1,2}\s*(?:years?\s*old|yr|yrs)?|aged\s+\d{1,2})\b"
);
pattern!(
    COLOUR_VALUE,
    r"\b(?:red|blue|green|black|white|grey|gray|brown|beige|cream|navy|pink|purple|orange|yellow|gold|silver|teal|turquoise|burgundy|maroon|khaki|olive|charcoal)\b"
);
pattern!(
    THEIR_QUESTION,
    r"^\W*(?:what|whats|what's|how|when|where|which|who|why|can|could|should|shall|would|will|does|do|did|is|are|was|were|have|has|any|anything|tell\s+me|remind\s+me)\b"
);
pattern!(LEAD, r"^(?:(?:and|so|but|oh|ok|okay|yeah|well)[ ,]+)+");
pattern!(
    PERSON,
    r"\b(?:i'm|im|i've|ive|i'll|ill|i'd|id|we're|were|we've|weve|we'll|we'd|i am|i was|we are|we were|i|we|my|our|me|us|mine|ours|myself|ourselves)\b"
);
pattern!(
    CONTRACTIONS,
    r"\b(?:you are|you have|it is|that is|i am)\b|^its\b"
);
pattern!(NUMBER_COMMA, r"(\d),(\d{3})\b");
pattern!(
    ATTRIBUTE_NAME,
    r"\b(?:ring size|model|brand|make|plate|name)\s+(?:is|was)\s+(?P<value>(?-i:[A-Z][A-Za-z0-9-]*(?: [A-Z][A-Za-z0-9-]*)*))\b"
);
pattern!(
    RELATION_NAME,
    r"\b(?:my|our)\s+(?:mum|mother|mom|dad|father|sister|brother|daughter|son|partner|friend|wife|husband)\s+(?:is|was|is called|was called)\s+(?P<value>(?-i:\p{Lu}[\p{L}'’-]+))\b"
);

type Tokens = BTreeSet<String>;

#[derive(Clone, Copy)]
enum Slot<'a> {
    When,
    Where,
    Size,
    Money(&'a str),
    Age,
    Often,
    OffWith,
}

#[derive(Clone, Copy)]
enum Value {
    Time,
    Size,
    Length,
    Weight,
    Age,
    Colour,
    Money,
}

fn capture<'a>(found: &Captures<'a>, name: &str) -> Option<&'a str> {
    found.name(name).map(|value| value.as_str())
}

fn first_group<'a>(found: &Captures<'a>) -> Option<&'a str> {
    found
        .iter()
        .skip(1)
        .flatten()
        .next()
        .map(|value| value.as_str())
}

fn space(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn has(words: &str, term: &str) -> bool {
    words.split_whitespace().any(|word| word == term)
}

fn stem(word: &str) -> String {
    let word = word.trim_matches('\'').to_ascii_lowercase();
    for suffix in ["ing", "ed", "es", "s"] {
        if word.len() > suffix.len() + 3
            && let Some(root) = word.strip_suffix(suffix)
        {
            return root.to_owned();
        }
    }
    word
}

fn tokens(line: &str) -> Tokens {
    WORD.find_iter(line)
        .map(|word| stem(word.as_str()))
        .collect()
}

fn subject(rest: &str) -> Tokens {
    WORD.find_iter(rest)
        .map(|word| word.as_str())
        .filter(|word| !has(SLOT_STOP, word))
        .map(stem)
        .collect()
}

pub(super) fn their_attribute(low: &str) -> Option<&str> {
    [&ASK, &OF_MINE].into_iter().find_map(|pattern| {
        pattern
            .captures(low)
            .as_ref()
            .and_then(|found| capture(found, "attr"))
    })
}

fn slot_ask(low: &str) -> Option<(Slot<'_>, Tokens)> {
    for (slot, pattern) in [(Slot::When, &WHEN_ASK), (Slot::Where, &WHERE_ASK)] {
        if let Some(found) = pattern.captures(low) {
            return Some((slot, subject(first_group(&found).unwrap_or(""))));
        }
    }
    for (slot, pattern) in [
        (Slot::Age, &AGE_ASK),
        (Slot::Often, &OFTEN_ASK),
        (Slot::OffWith, &OFF_WITH_ASK),
    ] {
        if pattern.is_match(low) {
            return Some((slot, Tokens::new()));
        }
    }
    if their_attribute(low).is_some() {
        return None;
    }
    if let Some(found) = SIZE_ASK.captures(low)
        && !SIZE_ADVICE.is_match(low)
    {
        return Some((Slot::Size, subject(&found["rest"])));
    }
    let found = MONEY_ASK.captures(low)?;
    Some((Slot::Money(first_group(&found)?), Tokens::new()))
}

fn a_time(line: &str) -> Option<String> {
    let low = line.to_lowercase();
    let words = || TIME_WORD.find_iter(&low).map(|word| word.as_str());
    if let Some(name) = words().find(|word| has(MONTHS_AND_DAYS, word)) {
        let mut name = name.to_owned();
        name[..1].make_ascii_uppercase();
        return Some(name);
    }
    if let Some(span) = TIME_SPAN.find(&low) {
        return Some(space(span.as_str()));
    }
    words()
        .find(|word| has(OTHER_TIMES, word))
        .map(str::to_owned)
}

fn a_place(line: &str) -> Option<String> {
    let found = PLACE.captures(line)?;
    let place = &found["place"];
    if place
        .split_whitespace()
        .all(|word| has(SLOT_STOP, &stem(word)))
    {
        return None;
    }
    Some(space(found.get(0)?.as_str()))
}

fn a_size(line: &str, subject: &Tokens) -> Option<String> {
    let found = A_SIZE.captures(line)?;
    let mut measure = space(&found["measure"]);
    if let Some(tail) = capture(&found, "after").map(|word| word.trim().to_lowercase())
        && subject.contains(&stem(&tail))
    {
        measure.push(' ');
        measure.push_str(&tail);
    }
    Some(measure)
}

fn age_given(lines: &[&str], ask: &str) -> Option<String> {
    let whose = if let Some(found) = AGE_OWNER.captures(ask) {
        if &found[1] == "my" {
            "your"
        } else {
            first_group(&found)?
        }
    } else {
        match AGE_PRONOUN.captures(ask).as_ref().and_then(first_group) {
            Some("he") => "his",
            Some("she") => "her",
            _ => "their",
        }
    };
    for line in lines.iter().rev() {
        if A_COUNT_OF_PEOPLE.is_match(line) || !SOMEBODY.is_match(line) {
            continue;
        }
        let Some(age) = AN_AGE_CLAUSE.captures(line) else {
            continue;
        };
        let number = &age["n"];
        if !number
            .parse::<u16>()
            .is_ok_and(|number| (1..=120).contains(&number))
        {
            continue;
        }
        if let Some(named) = NAMED_AGE
            .captures_iter(line)
            .find(|named| &named["n"] == number)
        {
            return Some(format!(
                "your {} {} {number}",
                named["who"].to_lowercase(),
                named["was"].to_lowercase()
            ));
        }
        let whose = if matches!(whose, "their" | "its") && AGE_SELF.is_match(line) {
            "your"
        } else {
            whose
        };
        return Some(format!("{whose} age was {number}"));
    }
    None
}

fn slot_value(slot: Slot<'_>, subject: &Tokens, ask: &str, lines: &[&str]) -> Option<String> {
    match slot {
        Slot::Age => age_given(lines, ask),
        Slot::Often => lines.iter().rev().find_map(|line| {
            A_HABIT_SAID
                .find(line)
                .map(|value| space(&value.as_str().to_lowercase()))
        }),
        Slot::OffWith => lines.iter().rev().find_map(|line| {
            let found = OFF_WITH_SAID.captures(line)?;
            let article = if capture(&found, "thing")
                .is_some_and(|article| article.trim().eq_ignore_ascii_case("an"))
            {
                "an"
            } else {
                "a"
            };
            Some(format!(
                "you were off with {article} {}",
                space(&found["what"].to_lowercase())
            ))
        }),
        Slot::Money(what) => lines.iter().rev().find_map(|line| {
            let found = if SALARY_WORDS.contains(&what) {
                A_PER_YEAR.find(line)
            } else if tokens(line).contains(&stem(what)) {
                A_MONEY.find(line)
            } else {
                None
            };
            found.map(|value| space(value.as_str()))
        }),
        Slot::Size => lines.iter().enumerate().rev().find_map(|(index, line)| {
            let value = a_size(line, subject)?;
            let matches = if subject.is_empty() {
                lines.len() - index <= 2
            } else {
                !tokens(line).is_disjoint(subject)
                    || PRONOUN_LINE.is_match(line)
                        && index > 0
                        && !tokens(lines[index - 1]).is_disjoint(subject)
            };
            matches.then_some(value)
        }),
        Slot::When | Slot::Where => lines.iter().rev().find_map(|line| {
            let value = if matches!(slot, Slot::When) {
                a_time(line)?
            } else {
                a_place(line)?
            };
            (subject.is_empty() || !tokens(line).is_disjoint(subject)).then_some(value)
        }),
    }
}

fn asked_value(low: &str) -> Option<Value> {
    let asked = SAID_ASK.captures(low)?;
    if let Some(attr) = capture(&asked, "attr") {
        return match attr {
            "size" | "shoe size" | "ring size" => Some(Value::Size),
            "colour" | "color" => Some(Value::Colour),
            "weight" => Some(Value::Weight),
            "age" => Some(Value::Age),
            "length" | "hair" => Some(Value::Length),
            "time" | "day" | "date" | "month" | "year" | "week" => Some(Value::Time),
            attr if MONEY_WORDS.split('|').any(|word| word == attr) => Some(Value::Money),
            _ => None,
        };
    }
    match capture(&asked, "how")? {
        "short" | "long" | "tall" => Some(Value::Length),
        "big" | "large" | "small" => Some(Value::Size),
        "heavy" => Some(Value::Weight),
        "old" => Some(Value::Age),
        _ => None,
    }
}

fn values(kind: Value, line: &str) -> Vec<String> {
    if matches!(kind, Value::Time) {
        return a_time(line).into_iter().collect();
    }
    if matches!(kind, Value::Money) {
        return A_MONEY
            .find(line)
            .map(|found| space(found.as_str()))
            .into_iter()
            .collect();
    }
    let regex = match kind {
        Value::Size => &*SIZE_VALUE,
        Value::Length => &*LENGTH_VALUE,
        Value::Weight => &*WEIGHT_VALUE,
        Value::Age => &*AGE_VALUE,
        Value::Colour => &*COLOUR_VALUE,
        Value::Time | Value::Money => unreachable!(),
    };
    let mut seen = Tokens::new();
    regex
        .find_iter(line)
        .filter_map(|found| {
            let value = space(found.as_str());
            if matches!(kind, Value::Size)
                && matches!(
                    value.to_ascii_lowercase().as_str(),
                    "xs" | "s" | "m" | "l" | "xl" | "xxl" | "xxxl"
                )
                && (line[..found.start()].ends_with('\'') || line[found.end()..].starts_with('\''))
            {
                return None;
            }
            seen.insert(stem(&value)).then_some(value)
        })
        .collect()
}

fn restated(line: &str) -> Option<String> {
    if THEIR_QUESTION.is_match(line) || line.contains('?') {
        return None;
    }
    let read = Reading::new(line);
    if matches!(
        read.kind,
        Kind::Greeting | Kind::Thanks | Kind::Question | Kind::Empty
    ) || read.wants == Some("comfort")
    {
        return None;
    }
    let low = read.clean;
    let low = without_filler(&low);
    let low = LEAD.replace(&low, "");
    let low = NUMBER_COMMA.replace_all(
        low.trim_matches([' ', '.', '!', ',', ';', ':']),
        |money: &Captures<'_>| format!("{}{}", &money[1], &money[2]),
    );
    let change = PERSON.replace_all(&low, |found: &Captures<'_>| match &found[0] {
        "i'm" | "im" | "we're" | "i am" | "we are" => "you are",
        "i've" | "ive" | "we've" | "weve" => "you have",
        "i'll" | "ill" | "we'll" => "you'll",
        "i'd" | "id" | "we'd" => "you'd",
        "i was" | "we were" => "you were",
        "my" | "our" => "your",
        "me" | "us" => "you",
        "mine" | "ours" => "yours",
        "myself" | "ourselves" => "yourself",
        "were" => "were",
        _ => "you",
    });
    let result = CONTRACTIONS.replace_all(&change, |found: &Captures<'_>| match &found[0] {
        "you are" => "you're",
        "you have" => "you've",
        "it is" | "its" => "it's",
        "i am" => "i'm",
        _ => "that's",
    });
    if !(1..=14).contains(&result.split_whitespace().count()) {
        return None;
    }
    let mut result = result.into_owned();
    for name in [&*ATTRIBUTE_NAME, &*RELATION_NAME]
        .into_iter()
        .flat_map(|matcher| matcher.captures_iter(line))
        .filter_map(|found| capture(&found, "value"))
    {
        let low_name = name.to_lowercase();
        if let Some((before, after)) = result.rsplit_once(&low_name)
            && before.ends_with(char::is_whitespace)
            && (after.is_empty()
                || after.starts_with(|letter: char| !letter.is_ascii_alphanumeric()))
        {
            result = format!("{before}{name}{after}");
        }
    }
    Some(result)
}

fn cost(line: &str) -> bool {
    A_FIGURE.is_match(line) && !NOT_A_COST.is_match(line)
}

fn phrase_of(lines: &[&str], attribute: &str, needs_figure: bool) -> Option<String> {
    let asked = tokens(attribute);
    for line in lines.iter().rev() {
        if attribute == "name" && super::asks_to_forget_name(line.trim()) {
            continue;
        }
        if (needs_figure && !cost(line)) || !asked.is_subset(&tokens(line)) {
            continue;
        }
        if let Some(phrase) = restated(line)
            && asked.is_subset(&tokens(&phrase))
        {
            return Some(phrase);
        }
    }
    None
}

fn unseen_pay_subject(low: &str, lines: &[&str]) -> bool {
    let frame = tokens(PAY_ASK_WORDS);
    let asked = tokens(low);
    let said: Tokens = lines.iter().flat_map(|line| tokens(line)).collect();
    asked.difference(&frame).any(|word| !said.contains(word))
}

fn recall(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let low = space(reading.raw).to_lowercase();
    let lines: Vec<_> = memory.turns.iter().map(|turn| turn.user.as_str()).collect();
    let last = memory.turns.last()?.bot.to_lowercase();
    if let Some(kind) = asked_value(&low) {
        let found = lines.iter().rev().find_map(|line| {
            let line = space(line);
            let lower = line.to_lowercase();
            if THEIR_QUESTION.is_match(&line)
                || SAID_ASK.is_match(&line)
                || their_attribute(&lower).is_some()
            {
                return None;
            }
            let found = values(kind, &line);
            (!found.is_empty()).then_some((line, found))
        });
        if let Some((line, values)) = found
            && super::respond(reading, memory).is_none()
        {
            let (said, reply) = if let [value] = values.as_slice() {
                (value.as_str(), format!("You said {value}."))
            } else {
                (line.as_str(), format!("You said: \"{line}\""))
            };
            return Some(
                if last.contains(reply.trim_end_matches('.').to_lowercase().as_str()) {
                    format!("Still the same: {said}")
                } else {
                    reply
                },
            );
        }
    }
    if let Some((slot, subject)) = slot_ask(&low)
        && let Some(value) = slot_value(slot, &subject, &low, &lines)
    {
        let said = format!("You said {value}.");
        return Some(if last.contains(&said.to_lowercase()) {
            format!("Still {value}.")
        } else {
            said
        });
    }
    let phrase = if let Some(attr) = their_attribute(&low) {
        phrase_of(&lines, attr, false)
    } else if PAY_ASK.is_match(&low) {
        if unseen_pay_subject(&low, &lines) {
            return None;
        }
        restated(lines.iter().rev().find(|line| cost(line))?)
    } else if COST_ASK.is_match(&low) {
        restated(lines.last().filter(|line| cost(line))?)
    } else {
        let found = WHATS_MY.captures(&low)?;
        let attr = &found["attr"];
        let mut words = attr.split_whitespace();
        if has(QUESTION_WORDS, words.next()?) || words.count() >= 3 {
            return None;
        }
        phrase_of(&lines, attr, true)
    }?;
    if let Some(value) = VALUE_FRAME.captures(&phrase).as_ref().and_then(first_group)
        && last.contains(&value.to_lowercase())
        && last.contains("you told me")
    {
        return Some(format!("Still {value}."));
    }
    Some(format!("You told me {phrase}."))
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    recall(reading, memory)
}

pub(crate) fn takes_as_it_comes(reading: &Reading<'_>, memory: &Memory) -> bool {
    let low = space(reading.raw).to_lowercase();
    !matches!(
        slot_ask(&low),
        Some((Slot::Age | Slot::Often | Slot::OffWith, _))
    ) && recall(reading, memory).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    fn prior(lines: &[&str]) -> Memory {
        Memory {
            turns: lines
                .iter()
                .map(|line| Exchange {
                    user: (*line).into(),
                    bot: String::new(),
                    module: "greeting",
                })
                .collect(),
            ..Memory::default()
        }
    }

    #[test]
    fn a_pronoun_supplies_dimensions_only_for_the_subject_just_named() {
        let memory = prior(&[
            "We're redecorating the kitchen",
            "It's small, about 3 by 2 metres",
        ]);
        let kitchen = Reading::new("How big is the kitchen?");
        assert_eq!(
            respond(&kitchen, &memory).as_deref(),
            Some("You said 3 by 2 metres.")
        );
        assert!(takes_as_it_comes(&kitchen, &memory));
        assert_eq!(
            respond(&Reading::new("How big is the garden?"), &memory),
            None
        );
    }

    #[test]
    fn cost_recall_skips_rates_for_work_and_declines_a_new_pay_subject() {
        let memory = prior(&["My rent is 900 a month", "I work 40 a week"]);
        assert_eq!(
            respond(&Reading::new("What was I paying?"), &memory).as_deref(),
            Some("You told me your rent is 900 a month.")
        );
        assert_eq!(
            respond(&Reading::new("How much do I pay for the gym?"), &memory),
            None
        );
    }

    #[test]
    fn the_age_candidate_does_not_claim_precedence_over_the_store() {
        let memory = prior(&["My son is 14", "There are 6 of us"]);
        let reading = Reading::new("How old did I say he was?");
        assert_eq!(
            respond(&reading, &memory).as_deref(),
            Some("You said your son is 14.")
        );
        assert!(!takes_as_it_comes(&reading, &memory));
    }
}
