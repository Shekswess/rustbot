//! Conversation history checks shared by the chooser and arbiter.
use crate::bot::Memory;
use regex::Regex;
use std::sync::LazyLock;
/// Look back through up to three replies that could still be part of an emergency.
pub(crate) fn in_a_safety_thread(memory: &crate::bot::Memory) -> bool {
    memory
        .turns
        .iter()
        .rev()
        .take(3)
        .find(|turn| {
            !matches!(
                turn.module,
                "greeting" | "carers_turn" | "safety_cards" | ""
            )
        })
        .is_some_and(|turn| turn.module == "safety")
}

static CLAIMS_A_PAST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:where\s+we\s+(?:were|left\s+off|got\s+to)|what\s+were\s+we\s+(?:saying|on\s+about|up\s+to)|what\s+were\s+you\s+saying|as\s+(?:i|we)\s+said|as\s+before|same\s+as\s+before|you\s+mentioned|earlier\s+(?:you|we)|last\s+time\s+(?:we|you|i)\b|what\s+did\s+i\s+say|remind\s+me\s+(?:what|where|which|how)|carry\s+on\s+where|before\s+we\s+(?:broke\s+off|stopped)|still\s+here\s*,?\s+(?:if|so)|what\s+(?:are|were)\s+we\s+(?:working\s+on|on|talking\s+about)|whatever\s+you(?:'ve|\s+have)\s+been\s+(?:doing|up\s+to)|(?:a\s+)?break\s+does\s+(?:a\s+lot\s+of\s+|you\s+)?good|keep\s+it\s+going\s+(?:for\s+)?another|since\s+we\s+last\s+spoke|last\s+we\s+spoke|(?:we|i)\s+(?:talked|spoke)\s+about\s+(?:it|that|this)|where\s+(?:were\s+we|did\s+we\s+get\s+to|did\s+we\s+leave|have\s+we\s+got\s+to))\b"#).unwrap()
});
static GREETS_A_RETURN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^\s*(?:(?:oh|ah|well|hello|hi|hey|right|ok|okay|and|so)\s*[,!.]?\s*)?(?:welcome\s+back|you(?:'re|\s+are)\s+back|back\s+again|(?:good|nice)\s+to\s+(?:see|have)\s+you\s+(?:again|back)|(?:hello|hi|hey)\s+again)\b"#).unwrap()
});
const ANSWERS_AN_EARLIER_TURN: &[&str] = &[
    "good to hear",
    "glad to hear",
    "sounds alright",
    "sounds good",
    "nice one",
    "thanks for telling me",
    "thanks for that",
    "say more",
    "tell me more about that",
    "how did the rest of it go",
    "anything good in it",
    "same as before",
    "same answer",
    "same again",
    "nothing new to add",
    "that is still the thing to do",
    "as i said",
    "as you said",
    "like i said",
    "you said",
    "you told me",
    "you mentioned",
    "back to what we",
    "where we were",
    "where were we",
    "the same goes for",
    "how did that go",
    "what was the best",
];
const NO_CONTENT_WORDS: &[&str] = &[
    "a",
    "about",
    "add",
    "after",
    "again",
    "again",
    "ah",
    "am",
    "an",
    "and",
    "anyway",
    "anyways",
    "apologies",
    "apologise",
    "apologize",
    "are",
    "as",
    "at",
    "away",
    "back",
    "be",
    "been",
    "before",
    "being",
    "bit",
    "but",
    "carry",
    "carrying",
    "continue",
    "cut",
    "did",
    "distracted",
    "do",
    "does",
    "done",
    "down",
    "earlier",
    "eh",
    "fine",
    "for",
    "get",
    "gets",
    "go",
    "goes",
    "going",
    "gone",
    "gone",
    "got",
    "had",
    "half",
    "has",
    "have",
    "he",
    "hello",
    "hey",
    "hi",
    "hmm",
    "how",
    "i",
    "if",
    "in",
    "interrupted",
    "is",
    "it",
    "just",
    "kind",
    "lot",
    "me",
    "message",
    "messages",
    "mid",
    "midway",
    "missed",
    "missed",
    "my",
    "never",
    "new",
    "no",
    "nope",
    "nothing",
    "now",
    "of",
    "off",
    "oh",
    "ok",
    "okay",
    "on",
    "one",
    "or",
    "our",
    "out",
    "pause",
    "paused",
    "please",
    "quiet",
    "really",
    "replied",
    "replies",
    "reply",
    "resume",
    "right",
    "said",
    "same",
    "say",
    "saying",
    "says",
    "she",
    "silence",
    "so",
    "so",
    "sorry",
    "sort",
    "still",
    "tell",
    "than",
    "thank",
    "thanks",
    "that",
    "the",
    "them",
    "then",
    "these",
    "they",
    "this",
    "those",
    "thought",
    "thread",
    "to",
    "told",
    "um",
    "unfinished",
    "up",
    "us",
    "very",
    "was",
    "we",
    "well",
    "went",
    "were",
    "what",
    "when",
    "where",
    "which",
    "who",
    "why",
    "with",
    "yeah",
    "yes",
    "you",
    "your",
];

pub(crate) fn implies_earlier_turn(reply: &str, memory: &Memory, said: Option<&str>) -> bool {
    if memory
        .turns
        .iter()
        .any(|turn| turn.user.chars().any(|c| !crate::text::is_space(c)))
    {
        return false;
    }
    let low = reply
        .to_lowercase()
        .split(crate::text::is_space)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if CLAIMS_A_PAST.is_match(&low) || GREETS_A_RETURN.is_match(&low) {
        return true;
    }
    let Some(said) = said else { return false };
    static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9']+").unwrap());
    WORD.find_iter(&said.to_lowercase())
        .all(|w| NO_CONTENT_WORDS.binary_search(&w.as_str()).is_ok())
        && ANSWERS_AN_EARLIER_TURN
            .iter()
            .any(|phrase| low.contains(phrase))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_return_greeting_needs_history_and_a_receipt_can_follow_an_actual_event() {
        let memory = Memory::default();
        assert!(implies_earlier_turn("Welcome back!", &memory, None));
        assert!(implies_earlier_turn(
            "Nice one. How did the rest of it go?",
            &memory,
            Some("carry on")
        ));
        assert!(!implies_earlier_turn(
            "Nice one. How did the rest of it go?",
            &memory,
            Some("I won a prize")
        ));
        assert!(!implies_earlier_turn(
            "Walk the dog again when you are back.",
            &memory,
            None
        ));
    }
}
