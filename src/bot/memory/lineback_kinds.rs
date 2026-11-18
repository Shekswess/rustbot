//! Detail shapes used by PyBot's memory/kinds.py, in their original order.
use fancy_regex::Regex as ContextRegex;
use regex::Regex;
use std::{collections::BTreeSet, sync::LazyLock};

const FRAME: &str = "a an the and but so or if my our your their his her its it this that these those of to in on at for with from about again was were is are am be been being do does did done have has had what which who whom whose when where why how much many long old big tall wide heavy far time hour day date make brand model manufacturer price cost size age number place town city airport name called colour color count weight height length say said tell told mention mentioned pay paid paying cost costs costed charge charged spend spent buy bought i we you me us them they he she just only even ever before already first last next other another else more most still yet now please pls can could would will shall may might should must cant cannot dont doesnt didnt remember recall forget forgot know knew knews remind mind think thought reckon guess go goes going went gone come comes coming came been one ones thing things stuff kind sort like";

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($source).unwrap());
    };
}

pattern!(WORD, r"[a-z][a-z'-]*");
pattern!(
    BACK_REF,
    r"(?i)^\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright)\W+)*(?:it'?s|its|it\s+is|they'?re|they\s+are|that'?s|there'?s|there\s+are|it\s+was|was|is|are)\b"
);
pattern!(
    LEAD,
    r"^\W*(?:(?:and|but|so|oh|also|well|yeah|yep|ok|okay|right|alright|sorry|hmm|hi|hiya|hey|hello|now|then|again|quick\s+one|mate)[ ,.!-]+)*"
);
pattern!(
    RECALL,
    r"\b(?:again|before|earlier|was\s+(?:it|that)|did\s+(?:i|we|you)|have\s+i|you\s+know|tell\s+me|remind(?:ed)?\s+me|the\s+(?:time|hour|day|date|make|model|brand|price|cost|size|name|place|colour|color|number))\b"
);
pattern!(
    QUESTION,
    r"(?i)\?\s*$|^\W*(?:what|which|who|whom|whose|when|where|why|how|is|are|was|were|do|does|did|can|could|should|would|will|shall|have|has|am|any|remind|tell\s+me)(?:'?s)?\b"
);
pattern!(
    ADVICE,
    r"\b(?:do|does|should|can|could|would|shall|will|might)\s+(?:i|we|you)\b"
);
pattern!(
    END,
    r"(?i)\b(?:end|ends|ending|finish|finishes|finishing|last|lasts|leaves|leaving|close|closes|closing)\b"
);
pattern!(
    START,
    r"(?i)\b(?:start|starts|starting|started|begin|begins|beginning|began|open|opens|opening)\b"
);

struct Detail {
    ask: Regex,
    gives: Option<ContextRegex>,
    unnamed: bool,
}

static DETAILS: LazyLock<Vec<Detail>> = LazyLock::new(|| {
    [
    (r"\b(?:what|which)\s+(?:time|hour)\b|\bthe\s+(?:time|hour)\b|\bo'?clock\b",
     Some(r"(?i)\b(?:at|by|around|about|from|until|till|before|after)\s+\d{1,2}(?::\d\d)?(?:\s*(?:am|pm|a\.m\.|p\.m\.))?\b|\b\d{1,2}(?::\d\d)?\s*(?:am|pm)\b|\b(?:noon|midday|midnight)\b|\b(?:half|quarter)\s+(?:past|to)\s+\w+|\bo'?clock\b"), false),
    (r"\b(?:what|which)\s+(?:day|date)\b|\bthe\s+(?:day|date)\b",
     Some(r"(?i)\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tues|tue|weds|wed|thurs|thu|fri|sat|sun|tomorrow|today|tonight|weekend|next\s+(?:week|month|year)|this\s+week|the\s+\d{1,2}(?:st|nd|rd|th)|\d{1,2}(?:st|nd|rd|th)\s+(?:of\s+)?[a-z]+)\b"), false),
    (r"^\W*(?:and\s+|also\s+)?when\b|\bwhen\s+(?:again|was\s+it|is\s+it)\b",
     Some(r"(?i)\b(?:at|by|around)\s+\d{1,2}(?::\d\d)?(?!\d)|\b\d{1,2}\s*(?:am|pm)\b|\b(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday|tomorrow|today|tonight|next\s+week|this\s+week)\b"), false),
    (r"\b(?:what|which)\s+(?:make|brand|model|manufacturer)\b|\b(?:make|brand|model)\s+(?:again|is\s+it|was\s+it)\b",
     Some(r"(?i)\b(?:it'?s|its|it\s+is|i\s+got|we\s+got|i\s+bought|we\s+bought|they\s+installed)\s+(?:a|an)\s+[a-z][\w-]*(?:\s+[a-z][\w-]*)?"), true),
    (r"\bhow\s+much\b|\b(?:what|which)\s+(?:price|cost)\b|\bthe\s+(?:price|cost)\b|\b(?:what|how\s+much)\s+did\s+(?:i|we|you)\s+pay\b",
     Some(r"(?i)[£$€]\s*\d|\b\d[\d,.]*\s*(?:pounds?|quid|euros?|dollars?)\b|\b(?:costs?|cost\s+me|paid|paying|charges?|quote[ds]?)\b|\b\d[\d,.]*\s*(?:pounds?|quid|euros?|dollars?)\s+a\s+(?:month|week|year)\b"), true),
    (r"\bhow\s+long\b", None, false),
    (r"\b(?:what|which)\s+size\b|\bhow\s+(?:big|tall|wide|heavy|far)\b",
     Some(r"(?i)\b\d+(?:\.\d+)?\s*(?:mm|cm|m|metres?|meters?|feet|foot|ft|inches|inch|in|kg|kilos?|grams?|g|stone|lb|lbs|litres?|liters?|miles?|ml)\b|\bsize\s+\w"), true),
    (r"\bhow\s+old\b|\b(?:what|which)\s+age\b",
     Some(r"(?i)\b(?:\d{1,3}|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|sixteen|seventeen|eighteen|nineteen|twenty)\s*(?:years?|months?|weeks?)\s*(?:old)?\s*(?=$|[.,;!?]|\b(?:and|or|in|now|so)\b)|\baged\s+\d"), true),
    (r"\b(?:what|which)\s+colou?r\b|\bcolou?r\s+(?:again|was\s+it|is\s+it)\b",
     Some(r"(?i)\b(?:red|blue|green|yellow|black|white|grey|gray|brown|pink|purple|orange|beige|cream|navy|silver|gold|teal|turquoise|maroon|olive|charcoal)\b"), true),
    (r"\bwhere\b|\b(?:what|which)\s+(?:place|town|city|airport)\b",
     Some(r"(?i)\b(?:in|at|from|to|near|outside|inside|via)\s+(?!night|morning|afternoon|evening|all|least|once|times?|moment|end|same|other|top|bottom|front|back|side|way|case|fact|past|future|total|order|general|particular|short|full|half|the\s+(?:end|moment|same|other|top|bottom|front|back|side|way|case|fact|total|order))[a-z][\w'-]+"), false),
    (r"\b(?:what|which)\s+(?:name|make\s+is\s+it\s+called)\b|\bwho\s+(?:made|makes|built)\b|\bwhat'?s\s+it\s+called\b|\bwhat\s+was\s+it\s+called\b",
     Some(r"\b(?:called|named)\s+[a-z]|\bname'?s?\s+[a-z]|\b(?:it'?s|its)\s+[A-Z]"), false),
].into_iter().map(|(ask, gives, unnamed)| Detail {
    ask: Regex::new(ask).unwrap(),
    gives: gives.map(|pattern| ContextRegex::new(pattern).unwrap()),
    unnamed,
}).collect()
});

fn words(line: &str, with_plural: bool) -> BTreeSet<&str> {
    WORD.find_iter(line)
        .flat_map(|found| {
            let word = found.as_str().trim_matches(['\'', '-']);
            if word.len() < 3 || FRAME.split_whitespace().any(|stop| stop == word) {
                return [None, None].into_iter().flatten();
            }
            let singular = if with_plural && word.len() > 4 {
                word.strip_suffix('s')
            } else {
                None
            };
            [Some(word), singular].into_iter().flatten()
        })
        .collect()
}

fn mentions(line: &str, wanted: &BTreeSet<&str>) -> bool {
    let low = line.to_ascii_lowercase();
    let have = words(&low, true);
    wanted.iter().any(|word| {
        have.contains(word)
            || low.contains(word)
            || (word.len() > 4 && have.contains(&word[..word.len() - 1]))
    })
}

fn other_end(ask: &str, prior: &str) -> bool {
    match (END.is_match(ask), START.is_match(ask)) {
        (true, false) => START.is_match(prior),
        (false, true) => END.is_match(prior),
        _ => false,
    }
}

pub(super) fn find<'a>(ask: &str, lines: &[&'a str]) -> Option<&'a str> {
    let low = ask.to_ascii_lowercase();
    let low = LEAD.replace(&low, "");
    if !QUESTION.is_match(&low) || ADVICE.is_match(&low) {
        return None;
    }
    let named = words(&low, false);
    let again = RECALL.is_match(&low);
    let lines: Vec<_> = lines
        .iter()
        .copied()
        .filter(|line| !QUESTION.is_match(line))
        .collect();
    for detail in DETAILS.iter().filter(|detail| detail.ask.is_match(&low)) {
        let Some(gives) = &detail.gives else {
            continue;
        };
        if named.is_empty() && !again && !detail.unnamed {
            continue;
        }
        for (index, &line) in lines.iter().enumerate().rev() {
            if !gives.is_match(line).unwrap_or(false) || other_end(&low, line) {
                continue;
            }
            if named.is_empty()
                || mentions(line, &named)
                || index > 0 && BACK_REF.is_match(line) && mentions(lines[index - 1], &named)
            {
                return Some(line);
            }
        }
    }
    None
}
