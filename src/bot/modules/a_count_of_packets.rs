//! Recognise medicine taken in counted containers.

use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;

const VERB: &str = r"(?:swallowed|swallowing|drank|drunk|eaten|ate|took|taken|downed|inhaled|popped|necked|finished|used|washed\s+down|swilled|gulped|gave|given|handed)";
pub(super) const MEDICINE: &str = r"(?:pills|tablets|paracetamol|ibuprofen|painkillers|analgesics|meds|medicines?|sleeping\s+pills|antidepressants|tranquill?i[sz]ers?|insulin|codeine|co[\s-]?codamol|codamol|tramadol|morphine|diazepam|cough\s+medicine)";
pub(super) const ANIMAL: &str = r"(?:dogs?|pupp(?:y|ies)|cats?|kittens?|rabbits?|bunn(?:y|ies)|hamsters?|budgies?|birds?|parrots?|horses?|pon(?:y|ies)|guinea\s+pigs?|gerbils?|ferrets?|tortoises?|animals?|pets?)";
const COUNT: &str = r"(?:two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,3}|a\s+couple\s+of|a\s+few|several|a\s+handful\s+of)";
const CONTAINER: &str =
    r"(?:packets?|packs?|boxes|box|bottles?|blisters?|strips?|sleeves?|sachets?|handfuls?)";

fn packets() -> String {
    format!(r"{COUNT}\s+{CONTAINER}\s+of\s+[^.?!]{{0,15}}\b{MEDICINE}\b")
}

fn regex(pattern: &str) -> Regex {
    Regex::new(pattern).expect("valid medicine packet pattern")
}

static TAKEN: LazyLock<Regex> =
    LazyLock::new(|| regex(&format!(r"\b{VERB}\b[^.?!]{{0,25}}\b{}", packets())));
static NO_VERB: LazyLock<Regex> = LazyLock::new(|| {
    regex(&format!(
        r"\b{}[^.?!]{{0,25}}\b(?:feel|felt|feeling|sick|ill|dizzy|drowsy|sleepy|vomit\w*|throwing\s+up|shak\w*|blurry|off\s+my\s+face|can'?t\s+stay\s+awake|sleeping\s+all\s+day|swallow\w*|and\s+then\s+i|and\s+now\s+i)\b",
        packets()
    ))
});
static ANIMAL_TOOK: LazyLock<Regex> =
    LazyLock::new(|| regex(&format!(r"\b{ANIMAL}\b[^.?!]{{0,25}}\b{VERB}\b")));
static STOCK_BEFORE: LazyLock<Regex> = LazyLock::new(|| {
    regex(&format!(
        r"\b(?:buy|buys|buying|bought|shop|shopping|order|ordered|prescription|prescribed|pharmacy|chemist|doctor|gp|nurse|getting|pick\s+up|picked\s+up|stock|stocked\s+up|spare|first\s+aid)\b[^.?!]{{0,30}}{}",
        packets()
    ))
});
static STOCK_ON: LazyLock<Regex> = LazyLock::new(|| {
    regex(&format!(
        r"\b{}[^.?!]{{0,25}}\b(?:for\s+(?:the|a|my)\s+(?:trip|holiday|weekend|flight|journey|kids)|on\s+holiday|on\s+business|to\s+take\s+away|away\s+with\s+me|in\s+my\s+(?:bag|pocket|suitcase|handbag|purse)|in\s+the\s+(?:cupboard|drawer|cabinet|glovebox)|at\s+home|left\s+over|spare|abroad)\b",
        packets()
    ))
});
static STOCK_PLACE: LazyLock<Regex> = LazyLock::new(|| {
    regex(
        r"\b(?:in|inside|kept\s+in|keep\s+in|keeps\s+in|have|has|got|carry|carrying|stash)\b[^.?!]{0,20}\b(?:cupboard|drawer|cabinet|shelf|shelves|bag|pocket|purse|glovebox)\b",
    )
});
static NEGATED: LazyLock<Regex> = LazyLock::new(|| {
    regex(&format!(
        r"\b(?:didn'?t|did not|hasn'?t|has not|hadn'?t|had not|never)\b[^.?!]{{0,9}}\b{VERB}\b"
    ))
});
static NO_INGESTION: LazyLock<Regex> = LazyLock::new(|| {
    regex(r"\b(?:no pills were swallowed|none were swallowed|only packed them)\b")
});

pub(super) fn is_dose(text: &str) -> bool {
    let text = text.replace('’', "'").to_lowercase();
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if !(line.contains("packet")
        || line.contains("pack")
        || line.contains("box")
        || line.contains("bottle")
        || line.contains("blister")
        || line.contains("strip")
        || line.contains("sleeve")
        || line.contains("sachet")
        || line.contains("handful"))
    {
        return false;
    }
    !ANIMAL_TOOK.is_match(&line)
        && !STOCK_BEFORE.is_match(&line)
        && !STOCK_ON.is_match(&line)
        && !STOCK_PLACE.is_match(&line)
        && !NEGATED.is_match(&line)
        && !NO_INGESTION.is_match(&line)
        && (TAKEN.is_match(&line) || NO_VERB.is_match(&line))
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if super::the_detail_the_thread_adds::heavy::only_a_dream(&reading.clean)
        || !is_dose(reading.raw)
        || super::safety::respond(reading, memory).is_some()
    {
        return None;
    }
    Some(super::poison::emergency(reading, memory))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn direct_taken_and_symptoms_count_but_stock_and_pets_do_not() {
        for line in [
            "I swallowed two packets of pills",
            "Two packets of pills and I feel sick",
            "We are away this week and I took two packets of pills",
            "I took two packets of pills and left the house",
        ] {
            assert!(is_dose(line), "{line}");
        }
        for line in [
            "I bought two packets of paracetamol",
            "I took two packets of pills for the trip",
            "My bag has two packets of pills in it",
            "My dog swallowed two packets of pills",
            "I did not take two packets of pills",
        ] {
            assert!(!is_dose(line), "{line}");
        }
    }

    #[test]
    fn packet_safety_precedes_generic_tablets_and_carries_to_a_follow_up() {
        let mut bot = Bot::new(Some("1")).unwrap();
        let first = bot.turn("I swallowed two packets of pills");
        assert_eq!(first.module, Some("a_count_of_packets"));
        assert!(first.reply.contains("999") && first.reply.contains("vomit"));
        assert_eq!(
            bot.turn("Is he going to be alright?").module,
            Some("safety")
        );
        let mut bot = Bot::new(Some("1")).unwrap();
        let child = bot.turn("My toddler swallowed two packets of tablets");
        assert_eq!(child.module, Some("safety"));
        assert!(child.reply.contains("999"));
    }
}
