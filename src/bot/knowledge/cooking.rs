//! Whole-chicken roasting times (PyBot cooking.py's _chicken_time).
use regex::Regex;
use std::sync::LazyLock;

static BIRD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:chicken|chickens|chiken)\b").unwrap());
static OVEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:roast(?:ing)?|cook(?:ing)?|oven|how long|minutes per|per (?:pound|lb))\b")
        .unwrap()
});
static AVOID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
    r"\b(?:pox|wings?|soup|stock|salad|nuggets?|thighs?|rest\w*|skin\w*|crisp\w*|crackling|golden|",
    r"pale|flabby|breasts?|legs?|drumsticks?|tenders?|strips?|goujons?|fridge|freezer|frozen|freeze|",
    r"leftovers?|left|spare|already cooked|keep|shelf.life|good for|off|defrost\w*|thaw\w*|unthaw|",
    r"curry|casserole|pie|stew|simmer|marinad\w*|price|cost\w*|expensive|cheap|worth|dear|",
    r"poach\w*|fry\w*|air.fry\w*|barbecue\w*|raw|undercooked|wash\w*)\b"
)).unwrap()
});
static WEIGHT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(\d+(?:[.,]\d+)?)\s*(kilos?|kilograms?|kg|grams?|g|pounds?|lbs?|ounces?|oz)\b")
        .unwrap()
});

pub(super) const POULTRY_DONENESS: &str = "Check the thickest part away from bone with a clean thermometer. Cook it to 70°C for 2 minutes or 75°C for 30 seconds.";

pub(crate) fn roast_chicken(clean: &str) -> Option<String> {
    if !BIRD.is_match(clean) || !OVEN.is_match(clean) || AVOID.is_match(clean) {
        return None;
    }
    let (kilos, said) = if let Some(m) = WEIGHT.captures(clean) {
        let quantity: f64 = m[1].replace(',', ".").parse().ok()?;
        let kilos = match &m[2] {
            "g" | "gram" | "grams" => quantity / 1000.0,
            "pound" | "pounds" | "lb" | "lbs" => quantity * 0.453_592_37,
            "ounce" | "ounces" | "oz" => quantity * 0.028_349_523,
            _ => quantity,
        };
        if !(0.4..10.0).contains(&kilos) {
            return None;
        }
        (kilos, m.get(0)?.as_str())
    } else {
        (2., "2kg")
    };
    Some(chicken_roast_reply(kilos, said))
}

pub(super) fn chicken_roast_reply(kilos: f64, said: &str) -> String {
    let time = cooking_minutes(40. * kilos + 20.);
    format!(
        "Allow about 20 minutes per 500g plus 20 minutes at 190°C (170°C fan), so a {said} bird takes roughly {time}. Follow the pack instructions if provided. {POULTRY_DONENESS} Without a probe, check that the meat is steaming hot all the way through, with no pink meat and clear juices. Rest it for 15 minutes before carving."
    )
}

pub(super) fn cooking_minutes(minutes: f64) -> String {
    let minutes = ((minutes / 5.).round_ties_even() as u32) * 5;
    let (hours, rest) = (minutes / 60, minutes % 60);
    match (hours, rest) {
        (0, m) => format!("{m} minutes"),
        (1, 0) => "1 hour".into(),
        (1, m) => format!("1 hour {m} minutes"),
        (h, 0) => format!("{h} hours"),
        (h, m) => format!("{h} hours {m} minutes"),
    }
}
