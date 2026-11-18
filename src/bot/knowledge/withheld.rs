//! The explicitly omitted breastfeeding medicine reply can otherwise fall through
//! to an unrelated headache or study card. An emergency responder may still act.
use crate::bot::Memory;
use regex::Regex;
use std::sync::LazyLock;

pub(crate) const REPLY: &str =
    "I don't have an answer about using that medicine while breastfeeding.";

static FEEDING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?i)\b(?:(?:breast|chest)[\s-]*feed(?:ing)?|(?:breast|chest)[\s-]*fed|nursing|(?:expressing|pumping) (?:breast ?milk|milk))\b"
).unwrap()
});
static UNRELATED_NURSING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?i)^nursing\s+(?:homes?|school|course|shift|career|jobs?|staff|placement|degrees?|a (?:cold|hangover|headache|drink|grudge|wound|sprain))\b"
).unwrap()
});
static CODEINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\b(?:codeine|co[\s-]?codamol)\b").unwrap());
static MEDICINES_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?ix)\b(?: what \s+ (?:can|should|could) \s+ i \s+ (?:safely\s+)? take \s* (?:[?.!,]|$|(?:while|whilst|when|if|now\s+that|for\s+pain)\b) | (?:which|what)\s+(?:medicines?|medication|pain\s*killers?)\b )"
).unwrap()
});
static CONTINUES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?i)^\s*(?:and\b|but\b|what about\b|how about\b|can i (?:still )?(?:use|take|have)\b|is (?:that|it|codeine|co[ -]?codamol)\b|while\b|when\b|even if\b|i(?:'m| am) (?:also |still )?(?:nursing|breast|chest))"
).unwrap()
});

fn feeding(text: &str) -> bool {
    FEEDING
        .find_iter(text)
        .any(|m| !UNRELATED_NURSING.is_match(&text[m.start()..]))
}

pub(crate) fn applies(text: &str, memory: Option<&Memory>) -> bool {
    let has_feeding = feeding(text);
    let has_codeine = CODEINE.is_match(text);
    let asks_medication = MEDICINES_ASK.is_match(text);
    if has_feeding && (has_codeine || asks_medication) {
        return true;
    }
    if !(has_feeding || has_codeine || asks_medication) || !CONTINUES.is_match(text) {
        return false;
    }
    memory.and_then(|m| m.turns.last()).is_some_and(|prior| {
        (has_feeding && CODEINE.is_match(&prior.user))
            || ((has_codeine || asks_medication) && feeding(&prior.user))
    })
}

#[cfg(test)]
mod tests {
    use super::REPLY;
    use crate::{Bot, lookup_knowledge};

    #[test]
    fn withholds_direct_and_contextual_requests() {
        for line in [
            "Can I take codeine while breastfeeding?",
            "I am nursing. Can I have codeine for a headache?",
            "Is codeine safe when I'm breast feeding?",
            "What can I take while breastfeeding?",
            "How about co-codamol while chestfeeding?",
        ] {
            let turn = Bot::new(Some("0")).unwrap().turn(line);
            assert_eq!(turn.reply, REPLY, "{line}");
            assert!(turn.module.is_none() && turn.candidates.is_empty());
            assert!(lookup_knowledge(line).is_none());
        }
        for (one, two) in [
            (
                "I'm breastfeeding my newborn",
                "Can I take codeine for a headache?",
            ),
            (
                "I was prescribed co-codamol",
                "What about while I'm nursing?",
            ),
        ] {
            let mut bot = Bot::new(Some("0")).unwrap();
            bot.turn(one);
            assert_eq!(bot.turn(two).reply, REPLY, "{one} / {two}");
            bot.reset().unwrap();
            assert_ne!(bot.turn(two).reply, REPLY, "{two} without context");
        }
    }

    #[test]
    fn preserves_other_cards_and_emergency_precedence() {
        for line in [
            "Why does co-codamol make me drowsy?",
            "Can I take paracetamol while pregnant?",
            "My nursing home stores codeine for patients",
        ] {
            assert!(!super::applies(line, None), "{line}");
        }
        let mut bot = Bot::new(Some("0")).unwrap();
        let turn = bot.turn("I smell gas in my kitchen. Can I take codeine while breastfeeding?");
        assert_eq!(turn.module, Some("safety"));
    }
}
