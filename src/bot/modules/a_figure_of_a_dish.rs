//! Native port of bot/modules/a_figure_of_a_dish.py.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;
static A_FIGURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\bhow (?:many|much)\s+(?:grams?|g|kg|calories?|cals?|carbs?|carbohydrate|sugar|salt|protein|fat|fibre|fiber|ml|millilitres?|litres?|ounces?|oz|slices?|portions?|servings?|spoons?|tablespoons?|teaspoons?)\b[^?!]{0,40}?\b(?:in|of|per|for)\b"#).unwrap()
});
static DISH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:stew|casserole|pie|quiche|tart|cake|flapjack|brownie|biscuit|scone|pudding|crumble|lasagne|lasagna|curry|chilli|soup|broth|risotto|paella|pizza|sandwich|wrap|salad|roast|stir ?fry|noodle|dumpling|loaf|bake|smoothie|porridge|granola|muffin|pastry|gravy|sauce|dressing|hummus|dip|pancake|omelette|frittata)\b"#).unwrap()
});
static THEIRS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:my|the one i|i made|i baked|i cooked|homemade|home made|left ?overs?)\b|\b(?:portion|serving|slice|bowl|plate|helping)\b"#).unwrap()
});
const POOL: &[&str] = &[
    "The recipe and the portion are what set that, not a general figure: the weights on your own recipe card or the label on the pack, divided by how many it served. Tell me what went in and how many it made, and I will work the sum out with you.",
    "Nothing here can know that one - it depends what went into it and how much of it you had. Your recipe or the pack has the figures for the whole thing, so halve it or quarter it; give me the amounts and I will do the division.",
];

pub(crate) fn respond(reading: &Reading<'_>, _: &Memory) -> Option<String> {
    let said = reading.folded();
    (A_FIGURE.is_match(said) && (DISH.is_match(said) || THEIRS.is_match(said)))
        .then(|| POOL[0].into())
}
