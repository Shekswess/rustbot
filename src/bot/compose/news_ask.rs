//! A request for current headlines, from bot/compose/news_ask.py.
use regex::Regex;
use std::sync::LazyLock;
static A_FRAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?:so|and|ok|okay|well|but)?\s*(?:any|anything|got|get|is\s+there|are\s+there|whats?|what\s+(?:is|are)|tell\s+me|you\s+got|have\s+you\s+got|more|the|some)?\s*(?:in\s+the\s+|from\s+the\s+|on\s+the\s+|the\s+)?(?:news|headlines?)(?:\s+(?:about|on|of|regarding)\s+.*|\s+(?:today|tonight|this\s+(?:morning|afternoon|evening)|at\s+the\s+moment|right\s+now|please|then)\b.*)?$"#).unwrap()
});
static A_NEWS_DOING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:news|headlines?)\s+(?:is|are|was|were|has|have|had|gets?|got|makes?|made|keeps?|puts?|bothered|upsets?|worries)\b"#).unwrap()
});
static A_VERDICT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:no|some|good|bad|great|awful|sad|terrible|nice|exciting|brilliant|lovely|fantastic|amazing|grim|big)\s+(?:news|headlines?)\b"#).unwrap()
});
static THEIR_OWN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:i|we|my|our|his|her|their)\b.*\b(?:news|headlines?)\b|\b(?:news|headlines?)\b.*\b(?:my|our|his|her|their)\b"#).unwrap()
});
static A_REPORT_VERB: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:watched|watching|read|reading|heard|hear|told|saw|seen|made|missed|followed|caught)\b"#).unwrap()
});
static A_YEAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\b(?:19|20)\d\d\b"#).unwrap());

pub(super) fn reply(clean: &str) -> Option<&'static str> {
    let said = clean.trim().trim_matches([' ', '.', '!']);
    if matches!(
        said.trim_matches([' ', '?', '!', '.']),
        "news" | "the news" | "and the news"
    ) {
        return Some(NEWS);
    }
    if !A_FRAME.is_match(said)
        || [
            &*A_NEWS_DOING,
            &*A_VERDICT,
            &*THEIR_OWN,
            &*A_REPORT_VERB,
            &*A_YEAR,
        ]
        .iter()
        .any(|p| p.is_match(said))
    {
        return None;
    }
    Some(NEWS)
}

const NEWS: &str = "I can't see the news from here, sorry. A news site or app will have the latest; tell me the story and I'll help you make sense of it.";
