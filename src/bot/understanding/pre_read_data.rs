//! Static patterns and wording from the pinned understanding pre-reader.
use regex::Regex;
use std::sync::LazyLock;
pub(super) static FEELING_BEFORE_SICK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:worried|scared|frightened|terrified|bored|stressed|anxious)\s+sick\b"##)
        .expect("FEELING_BEFORE_SICK")
});
pub(super) static SICK_AS_FEELING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(?:sick|ill|unwell|nauseous|nauseated|queasy|shaking|shaky|trembling|dizzy)\s+(?:(?:with|from)\s+)?(worry|worrying|worried|stress|stress(?:ed)?\s+out|fear|fright|frightened|panic\w*|nerves|nervous|anxious|anxiety|guilt\w*|grief|shame|ashamed|dread\w*|anger|rage|jealousy|it\s+all)\b"##).expect("SICK_AS_FEELING")
});
pub(super) static PART_THAT_IS_BAD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(my|his|her|their|our|your)\s+(shoulders|forehead|shoulder|temples|stomach|fingers|muscles|muscle|temple|cheeks|calves|finger|throat|tongue|thighs|armpit|waist|hands|belly|cheek|knees|spine|mouth|lungs|wrist|elbow|shins|palms|chest|ankle|teeth|groin|thigh|tooth|thumb|tummy|shin|lung|hips|ears|ribs|nose|toes|gums|legs|hand|calf|eyes|back|neck|heel|head|foot|arms|skin|lips|chin|palm|knee|feet|jaw|toe|gut|hip|ear|leg|rib|bum|gum|lip|arm|eye)\s+(?P<link>is|are|was|were|has\s+been|have\s+been|feels?|feels?\s+like)\s+(?:(?:an?\s+|such\s+an?\s+|one\s+big\s+|an?\s+(?:complete|right|total|proper|real|absolute|utter|bit\s+of\s+a|bloody)\s+)?(?:joke|nightmare|mess|disaster|wreck|state|shambles|agony|torture|car\s+crash|pain\s+in\s+the\s+(?:arse|bum|butt|neck|back))|(?:terrible|awful|rubbish|crap|horrible|dreadful|brutal|shocking|knackered|shot|agony|unbearable|wrecked|ruined|buggered|done\s+in))\b"##).expect("PART_THAT_IS_BAD")
});
pub(super) static SUGAR_PRICE_SHAPE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)^(?:[£$€]|\d+(?:\.\d+)?p(?:ence)?$)"##).expect("SUGAR_PRICE_SHAPE")
});
pub(super) static SUGAR_FIRST_PERSON: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?i)\b(my|his|her|their|your)\s+sugar\b((?:\s+[a-z0-9'£$€]+){0,5})"##)
        .expect("SUGAR_FIRST_PERSON")
});
pub(super) const SUGAR_LEVEL: &[&str] = &[
    "crash", "crashed", "crashes", "crashing", "dip", "dipped", "dips", "down", "drop", "dropped",
    "dropping", "drops", "falling", "falls", "fell", "high", "level", "levels", "low", "reading",
    "readings", "rises", "rising", "rose", "spike", "spiked", "spiking", "up",
];
pub(super) const SUGAR_AFTER_ROSE: &[&str] = &[
    "6",
    "7",
    "8",
    "9",
    "a",
    "above",
    "after",
    "again",
    "all",
    "around",
    "before",
    "bit",
    "during",
    "from",
    "high",
    "low",
    "overnight",
    "past",
    "quickly",
    "sharply",
    "since",
    "so",
    "steadily",
    "suddenly",
    "to",
    "very",
];
pub(super) const SUGAR_PRICE: &[&str] = &[
    "bag",
    "bags",
    "cost",
    "costs",
    "gram",
    "grams",
    "kilo",
    "kilos",
    "lb",
    "lbs",
    "packet",
    "packets",
    "pound",
    "pounds",
    "price",
    "prices",
    "spoon",
    "spoons",
    "teaspoon",
    "teaspoons",
    "tin",
    "tins",
];
pub(super) const THE_ODD_PLURALS: &[&str] = &["calves", "feet", "teeth"];
pub(super) fn feeling(word: &str) -> &'static str {
    match word {
        "worry" => "worried",
        "worrying" => "worried",
        "worried" => "worried",
        "stress" => "stressed",
        "stressed out" => "stressed",
        "stressed" => "stressed",
        "fear" => "frightened",
        "fright" => "frightened",
        "frightened" => "frightened",
        "panic" => "panicky",
        "panicking" => "panicky",
        "panicky" => "panicky",
        "nerve" => "nervous",
        "nerves" => "nervous",
        "nervous" => "nervous",
        "anxious" => "anxious",
        "anxiety" => "anxious",
        "guilt" => "guilty",
        "guilty" => "guilty",
        "grief" => "grieving",
        "shame" => "ashamed",
        "ashamed" => "ashamed",
        "dread" => "dreading",
        "dreading" => "dreading",
        "anger" => "angry",
        "rage" => "angry",
        "jealousy" => "jealous",
        "it all" => "overwhelmed",
        _ => "upset",
    }
}
