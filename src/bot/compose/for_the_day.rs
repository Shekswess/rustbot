//! Weather questions weighing the day against a particular activity.
use regex::Regex;
use std::sync::LazyLock;
static A_DAY_WEIGHED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:too|warm enough|mild enough|dry enough|cold enough|good enough)\s+(?:hot|cold|wet|windy|warm|cool|dry|muggy|humid)?\s*(?:to|for)\s+(?:go\s+)?(?:run|running|walk|walking|cycle|cycling|jog|jogging|hike|hiking|play|playing|gardening|garden|barbecue|bbq|swim|swimming|paint|painting|the\s+dog|a\s+run|a\s+walk|the\s+wash)\b"#).unwrap()
});
static A_DAY_ASKED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:is|will|would|should)\s+it\s+(?:be\s+)?(?:too\s+|warm\s+enough|dry\s+enough|cold\s+enough|nice\s+enough\s+)?(?:hot|cold|wet|windy|warm|muggy|cool|dry)\b(?:\s+(?:out|outside|today|tomorrow|later|this\s+\w+))?\s*\??\s*$"#).unwrap()
});
static A_COLD_DAY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?i)\b(?:cold|freezing|chilly|frost\w*|ice|icy|snow\w*|wind\w*|gale\w*|blowing)\b"#,
    )
    .unwrap()
});
const TOO_HOT: &[&str] = &[
    "What is too hot is personal, but the thing that catches people out is the muggy air rather than the number: above the mid twenties with no breeze, go early or go shorter, and take water even if you do not feel like it. A weather app has today's figure and the humidity - what time were you thinking of going?",
    "Heat is judged on the air as well as the temperature: still, humid and in the sun is a lot harder than the same number with a breeze. If it is close or over about 25 the answer is early, shorter, and water. The app will have the figure for today - morning or evening for you?",
];
const TOO_COLD: &[&str] = &[
    "Cold is the easier one to get wrong: it is wind and wet that turn a cold day into a bad one, and a dry still frost is fine walking weather once you are moving. Two thin layers and something for your ears beat one thick coat. The app will have today's wind chill - how long are you out for?",
    "What decides a cold day is the wind, not the number: the same 3 degrees is fine in a still street and bitter on a hill. Layers you can open when you warm up, and gloves even when it does not look like it. A weather app has the wind chill for today.",
];

pub(super) fn for_the_ask(clean: &str, repaired: &str) -> Option<&'static [&'static str]> {
    [clean, repaired].into_iter().find_map(|line| {
        (A_DAY_WEIGHED.is_match(line) || A_DAY_ASKED.is_match(line.trim())).then(|| {
            if A_COLD_DAY.is_match(line) {
                TOO_COLD
            } else {
                TOO_HOT
            }
        })
    })
}
