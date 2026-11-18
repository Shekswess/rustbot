//! Packed lunches, breakfast, snacks, and leftover ideas from the pinned registry.

use super::super::meal_shortlist::catalogue as meal_catalogue;
use super::{
    cooking_crowds, cooking_data::FOOD_WORDBANK, food_fold::fold, guards::Context,
    meal_window_data as data, pantry_names,
};
use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, collections::HashSet, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $value:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

pattern!(OTHER_PEOPLE, data::OTHER_PEOPLE_AT_LUNCH);
pattern!(NAMED_TABLE, data::NAMED_TABLE);
pattern!(SHORT_OF, data::SHORT_OF_ASK);
pattern!(
    EXCLUSION,
    r"\b(?:without|no|avoid|avoids|avoiding|allergic to|allergy to|intolerant to|intolerant of|cant eat|can't eat|cannot eat|dont eat|don't eat|doesnt eat|doesn't eat|wont eat|won't eat)\s+(?P<foods>[^,.!?;]+)"
);
pattern!(FOOD_ALLERGY, r"\b([a-z]+) allerg(?:y|ies)\b");
pattern!(WORD, r"[a-z]+");

const NO_SAFE_OPTIONS: &str = "Build this around food you already know fits the dietary requirements. Check the ingredients and any cross-contact warnings on the products you use.";
const KEEP_COLD: &str = "Keep perishable lunchbox food cold until you eat it.";

fn contains(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

fn table_count(line: &str) -> u32 {
    cooking_crowds::counts_for(&line.replace('-', " ")).unwrap_or(0)
}

fn names_table(line: &str) -> bool {
    let count = table_count(line);
    if count >= 6 {
        return true;
    }
    if count == 0 {
        return NAMED_TABLE.is_match(line);
    }
    if count >= 3 && (line.contains("of us") || contains(line, data::CROWD_PHRASES)) {
        return true;
    }
    contains(line, data::TABLE_PHRASES)
        && (contains(line, &["cook", "cooking", "make", "feed", "serve"])
            || (count >= 3 && contains(line, &["lunch", "lunches", "meal", "meals", "eat"])))
}

pub(super) fn individual_lunch(reading: &Reading<'_>) -> bool {
    for line in [&reading.line.text, &reading.line.clean] {
        let said = line.to_lowercase();
        if contains(&said, data::PACKED_LUNCH_NOUNS) {
            return true;
        }
        if OTHER_PEOPLE.is_match(&said) {
            return false;
        }
        let count = table_count(&said);
        if count >= 6 || (count >= 3 && said.contains("of us")) {
            return false;
        }
    }
    true
}

pub(super) fn individual_meal(reading: &Reading<'_>) -> bool {
    [&reading.line.text, &reading.line.clean]
        .into_iter()
        .all(|line| !names_table(&line.to_lowercase()))
}

fn tags(name: &str) -> &[&str] {
    match name {
        "nut" | "peanut" | "almond" | "cashew" | "walnut" | "hazelnut" | "pistachio" | "pecan" => {
            &["nut"]
        }
        "dairy" | "milk" | "cheese" | "yoghurt" | "yogurt" | "butter" | "cream" | "lactose" => {
            &["dairy"]
        }
        "gluten" | "wheat" | "coeliac" | "celiac" | "flour" | "bread" | "pasta" | "cracker" => {
            &["gluten"]
        }
        "soy" | "soya" => &["soy"],
        "sesame" | "tahini" | "hummus" => &["sesame"],
        "egg" | "mayonnaise" => &["egg"],
        "meat" | "chicken" | "ham" | "beef" | "bacon" | "pork" | "lamb" | "turkey" => &["meat"],
        "fish" | "tuna" | "salmon" | "cod" | "seafood" | "shellfish" | "prawn" => &["fish"],
        _ => &[],
    }
}

pub(super) struct Limits<'context, 'reading> {
    context: &'context Context<'reading>,
    banned: HashSet<String>,
}

impl<'context, 'reading> Limits<'context, 'reading> {
    pub(super) fn new(context: &'context Context<'reading>, include_shortage: bool) -> Self {
        let mut limits = Self {
            context,
            banned: HashSet::new(),
        };
        if let Some(memory) = context.memory {
            for allergen in memory.personal.reported_allergens() {
                limits.add_names(allergen);
            }
        }
        for line in std::iter::once(context.reading.raw).chain(
            context
                .earlier
                .iter()
                .rev()
                .take(4)
                .map(|turn| turn.user.as_str()),
        ) {
            let line = line.to_lowercase();
            for hit in EXCLUSION.captures_iter(&line) {
                limits.add_names(&hit["foods"]);
            }
            for hit in FOOD_ALLERGY.captures_iter(&line) {
                limits.add_names(&hit[1]);
            }
            for (phrase, tag) in [
                ("gluten free", "gluten"),
                ("gluten-free", "gluten"),
                ("dairy free", "dairy"),
                ("dairy-free", "dairy"),
                ("nut free", "nut"),
                ("nut-free", "nut"),
                ("egg free", "egg"),
                ("egg-free", "egg"),
                ("vegetarian", "vegetarian"),
                ("vegan", "vegan"),
                ("coeliac", "gluten"),
                ("celiac", "gluten"),
            ] {
                if line.contains(phrase) && !line.contains(&format!("not {phrase}")) {
                    match tag {
                        "vegetarian" => limits.add_names("meat fish"),
                        "vegan" => limits.add_names("meat fish dairy egg"),
                        _ => limits.add_names(tag),
                    }
                }
            }
            if include_shortage {
                for hit in SHORT_OF.find_iter(&line) {
                    let rest = line[hit.end()..]
                        .split(['.', ',', ';', '?', '!'])
                        .next()
                        .unwrap_or("");
                    limits.add_names(rest);
                }
            }
        }
        limits
    }

    fn add_names(&mut self, text: &str) {
        for word in WORD.find_iter(&text.to_lowercase()) {
            let name = fold(word.as_str());
            let families = tags(&name);
            if !families.is_empty()
                || FOOD_WORDBANK.binary_search(&name.as_str()).is_ok()
                || matches!(
                    name.as_str(),
                    "nut" | "oat" | "jam" | "fruit" | "pea" | "rice"
                )
            {
                self.banned
                    .extend(families.iter().map(|tag| (*tag).to_owned()));
                self.banned.insert(name);
            }
        }
    }

    fn allows(&self, food: &str, ingredients: &str) -> bool {
        let named_safe = ingredients
            .split_whitespace()
            .all(|name| !self.banned.contains(&fold(name)));
        let text_safe = WORD
            .find_iter(&food.to_lowercase())
            .all(|word| !self.banned.contains(&fold(word.as_str())));
        named_safe
            && text_safe
            && self
                .context
                .memory
                .is_none_or(|memory| !memory.pantry.reply_uses_blocked_food(food, memory))
    }

    fn has(&self, tag: &str) -> bool {
        self.banned.contains(tag)
    }

    pub(super) fn restricted(&self) -> bool {
        !self.banned.is_empty()
            || self
                .context
                .memory
                .is_some_and(|memory| memory.pantry.blocks_typed_offer(memory))
    }
}

fn list(foods: &[&str], comma: bool) -> String {
    match foods.split_last() {
        None => String::new(),
        Some((last, [])) => (*last).to_owned(),
        Some((last, first)) => {
            let joiner = if comma && first.len() > 1 {
                ", or "
            } else {
                " or "
            };
            format!("{}{joiner}{last}", first.join(", "))
        }
    }
}

fn breakfast(limits: &Limits<'_, '_>) -> Option<Cow<'static, str>> {
    if !limits.restricted()
        && limits.allows(data::ANY_BREAKFAST, "nut egg dairy gluten banana oats")
    {
        return Some(Cow::Borrowed(data::ANY_BREAKFAST));
    }
    let porridge = match (limits.has("gluten"), limits.has("dairy")) {
        (true, true) => "Porridge made with labelled gluten-free oats and water",
        (true, false) => "Porridge made with labelled gluten-free oats and milk",
        (false, true) => "Porridge made with oat milk and a spoon of jam",
        (false, false) => "Porridge with a banana and a spoon of jam",
    };
    let porridge_ingredients = match (limits.has("gluten"), limits.has("dairy")) {
        (true, true) => "oats",
        (true, false) => "oats dairy",
        (false, true) => "oats jam",
        (false, false) => "oats banana jam",
    };
    let choices = [
        (porridge, porridge_ingredients),
        ("eggs on toast", "egg gluten"),
        ("toast with peanut butter", "gluten nut"),
        ("beans on toast", "bean gluten"),
        ("toast with hummus", "gluten sesame chickpea"),
        ("yoghurt with fruit and oats", "dairy gluten oats fruit"),
        ("rice cakes with cheese", "rice dairy"),
        ("a boiled egg with fruit", "egg fruit"),
        ("a banana with a handful of nuts", "banana nut"),
    ];
    let choices: Vec<_> = choices
        .into_iter()
        .filter_map(|(food, ingredients)| {
            if food == porridge && limits.has("gluten") {
                // The oats are explicitly gluten-free. A declaration against oats themselves still wins.
                (!limits.has("oat")
                    && !limits.has("porridge")
                    && porridge_ingredients
                        .split_whitespace()
                        .all(|name| !limits.has(&fold(name))))
                .then_some(food)
            } else {
                limits.allows(food, ingredients).then_some(food)
            }
        })
        .take(3)
        .collect();
    if choices.is_empty() {
        return Some(Cow::Borrowed(NO_SAFE_OPTIONS));
    }
    let mut answer = list(&choices, true);
    if choices.len() == 3 {
        answer.push_str(data::BREAKFAST_TAIL);
    } else {
        answer.push_str(". They take under ten minutes.");
    }
    if limits.allows("a boiled egg with fruit", "egg fruit") {
        answer.push_str(" If mornings are rushed, boil an egg the night before and refrigerate it, then take a piece of fruit.");
    } else {
        answer.push_str(" If mornings are rushed, set out what you need the night before so breakfast is ready to assemble.");
    }
    Some(Cow::Owned(answer))
}

fn lunch_is_for_child(context: &Context<'_>) -> bool {
    std::iter::once(context.reading.raw)
        .chain(
            context
                .earlier
                .iter()
                .rev()
                .take(3)
                .map(|turn| turn.user.as_str()),
        )
        .any(|line| {
            let line = line.to_lowercase();
            WORD.find_iter(&line)
                .any(|word| data::CHILD_NAMES.contains(&word.as_str()))
                || contains(&line, data::CHILDS_LUNCH_WORDS)
                || contains(&line, &["year old", "nursery", "school", "fussy", "picky"])
        })
}

fn asks_for_no_sandwich(context: &Context<'_>) -> bool {
    std::iter::once(context.reading.raw)
        .chain(
            context
                .earlier
                .iter()
                .rev()
                .take(4)
                .map(|turn| turn.user.as_str()),
        )
        .any(|line| contains(&line.to_lowercase(), data::NO_SANDWICH))
}

fn no_sandwich(limits: &Limits<'_, '_>) -> Cow<'static, str> {
    let child = lunch_is_for_child(limits.context);
    let (opening, options) = if child {
        ("For a lunchbox with no sandwich", data::LUNCHBOX_KID)
    } else {
        ("For a packed lunch with no sandwich", data::LUNCHBOX_ADULT)
    };
    if !limits.restricted() {
        return Cow::Borrowed(if child {
            "For a lunchbox with no sandwich: cooked pasta or rice from last night's dinner in a tub, cheese cubes or a boiled egg with crackers or oatcakes, veg sticks with hummus, a yoghurt and some fruit - any three of those and a drink is a lunch. A flask of soup or leftovers does the same job and is often the easiest with a fussy eater, because it is food they already eat at home. Keep cold food chilled. For rice, cool it within one hour and use it within 24 hours of refrigerating."
        } else {
            "For a packed lunch with no sandwich: last night's dinner in a tub is the easiest, and pasta or rice salad, cold chicken or cheese with oatcakes, a boiled egg, veg sticks and hummus, soup in a flask, yoghurt and fruit all travel well. Any three of those plus a drink is a lunch, and the tub of dinner is the one that keeps you full until tea. Keep cold food chilled. For rice, cool it within one hour and use it within 24 hours of refrigerating."
        });
    }
    let choices: Vec<_> = options
        .iter()
        .filter_map(|&(food, metadata)| {
            let hidden = if food.contains("hummus") {
                "chickpea"
            } else if food.contains("oatcake") {
                "oat"
            } else if food.contains("rice") {
                "rice"
            } else {
                ""
            };
            (!metadata.is_empty() || food.contains("fruit"))
                .then_some(food)
                .filter(|_| {
                    limits.allows(food, metadata) && (hidden.is_empty() || !limits.has(hidden))
                })
        })
        .collect();
    if choices.is_empty() {
        return Cow::Borrowed(NO_SAFE_OPTIONS);
    }
    let count = match choices.len() {
        1 => "Put that with a drink.",
        2 => "Put both with a drink.",
        _ => "Choose any three and add a drink.",
    };
    Cow::Owned(format!(
        "{opening}: {}. {count} {KEEP_COLD}",
        list(&choices, false)
    ))
}

fn lunch(limits: &Limits<'_, '_>) -> Cow<'static, str> {
    if asks_for_no_sandwich(limits.context) && !limits.has("gluten") {
        return no_sandwich(limits);
    }
    if !limits.restricted()
        && limits.allows(
            data::ANY_LUNCH,
            "gluten fish tuna dairy meat sesame chickpea egg",
        )
    {
        return Cow::Borrowed(data::ANY_LUNCH);
    }
    let choices = [
        ("cheese", "dairy"),
        ("tuna", "fish"),
        ("ham", "meat"),
        ("egg", "egg"),
        ("hummus", "sesame chickpea"),
        ("avocado", "avocado"),
        ("roasted vegetables", "vegetable"),
    ];
    let choices: Vec<_> = choices
        .into_iter()
        .filter_map(|(food, ingredients)| limits.allows(food, ingredients).then_some(food))
        .take(3)
        .collect();
    if choices.is_empty() {
        return Cow::Borrowed(NO_SAFE_OPTIONS);
    }
    if limits.has("gluten") {
        let salad = match (limits.has("rice"), limits.has("potato")) {
            (true, true) => "a salad",
            (true, false) => "a potato salad",
            (false, true) => "a rice salad",
            (false, false) => "a rice or potato salad",
        };
        return Cow::Owned(format!(
            "For a lunch that is not a sandwich: {salad} with {}, fruit and a drink. Keep it in a cool bag until lunch. If using rice, cool it within one hour and use it within 24 hours of refrigerating.",
            choices.join(", ")
        ));
    }
    if !limits.allows("a sandwich or wrap", "gluten wheat") {
        return no_sandwich(limits);
    }
    Cow::Owned(format!(
        "A sandwich or wrap with {}, fruit, something crunchy and a drink is a packed lunch in five minutes. {KEEP_COLD}",
        choices.join(", ")
    ))
}

fn snack(limits: &Limits<'_, '_>) -> Cow<'static, str> {
    let choices: Vec<_> = data::SNACKS
        .iter()
        .filter_map(|&(food, ingredients)| {
            let oats_ok =
                !food.contains("oatcakes") || (!limits.has("oat") && !limits.has("gluten"));
            (oats_ok && limits.allows(food, ingredients)).then_some(food)
        })
        .take(3)
        .collect();
    let choice = if choices.is_empty() {
        "whatever protein and fibre you can eat safely".to_owned()
    } else if choices.len() == 2 {
        choices.join(", ")
    } else {
        list(&choices, false)
    };
    Cow::Owned(format!(
        "Go for protein plus fibre at about four in the afternoon, 150 to 250 calories: {choice}. It holds you a couple of hours where a biscuit holds you twenty minutes, so you are not starving at dinner."
    ))
}

fn meal_slot(limits: &Limits<'_, '_>) -> Option<Cow<'static, str>> {
    let said = format!(" {} ", limits.context.reading.line.clean);
    let (slot, reply) = data::SLOTS.iter().find(|(slot, _)| {
        said.contains(&format!(" {slot}")) || said.contains(&format!("{slot}s "))
    })?;
    if !limits.restricted() && limits.allows(reply, "") {
        return Some(Cow::Borrowed(reply));
    }
    match *slot {
        "brunch" => breakfast(limits),
        "lunch" => Some(lunch(limits)),
        "snack" => Some(snack(limits)),
        _ => None,
    }
}

fn leftovers(limits: &Limits<'_, '_>) -> Option<Cow<'static, str>> {
    let context = limits.context;
    let words: Vec<_> = context
        .reading
        .line
        .words
        .iter()
        .map(|word| fold(word))
        .collect();
    let named = pantry_names::ingredients(context.reading, &words);
    if let [food] = named.as_slice() {
        let food = if *food == "leftover chicken" {
            "chicken"
        } else {
            *food
        };
        if let Some((_, reply)) = data::LEFTOVER_IDEAS.iter().find(|(name, _)| *name == food)
            && limits.allows(reply, food)
        {
            let storage = " Use properly chilled leftovers within 48 hours and reheat them only once until steaming hot throughout. Use cooled refrigerated rice within 24 hours.";
            return Some(Cow::Owned(format!("{reply}{storage}")));
        }
        if let Some(recipe) =
            meal_catalogue::indexed("meal-use-up", context.reading, context.memory)
            && limits.allows(&recipe, food)
        {
            return Some(Cow::Owned(recipe));
        }
    }
    if limits.restricted() {
        return Some(Cow::Borrowed(NO_SAFE_OPTIONS));
    }
    Some(Cow::Borrowed(data::LEFTOVER_SOUP))
}

pub(super) fn registered(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    match id {
        "breakfast-ideas" => breakfast(&Limits::new(context, true)),
        "lunch-ideas" | "lunch-exclusion" => Some(lunch(&Limits::new(context, false))),
        "lunch-no-sandwich" => Some(no_sandwich(&Limits::new(context, false))),
        "snack-ideas" => Some(snack(&Limits::new(context, false))),
        "meal-idea-for-a-slot" => meal_slot(&Limits::new(context, false)),
        "leftover-meal" => leftovers(&Limits::new(context, false)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    fn turn(user: &str) -> Exchange {
        Exchange {
            user: user.to_owned(),
            bot: String::new(),
            module: "chat",
        }
    }

    #[test]
    fn counted_tables_keep_their_answer_but_a_lunchbox_can_be_for_anyone() {
        assert!(!individual_lunch(&Reading::new(
            "What's a good lunch for four of us?"
        )));
        assert!(!individual_meal(&Reading::new(
            "What's a good lunch for four of us?"
        )));
        assert!(individual_lunch(&Reading::new(
            "Packed lunch ideas for the whole family"
        )));
        assert!(individual_lunch(&Reading::new(
            "What's a good lunch for two of us?"
        )));
    }

    #[test]
    fn sandwich_refusals_and_named_allergens_stay_in_the_packed_lunch() {
        let earlier = [
            turn("My daughter is allergic to milk and sesame"),
            turn("She won't eat sandwiches"),
        ];
        let reading = Reading::new("What can I put in her lunchbox?");
        let context = Context::new(&reading, &earlier);
        let reply = registered("lunch-ideas", &context).unwrap();
        assert!(reply.starts_with("For a lunchbox with no sandwich"));
        assert!(!reply.contains("cheese"));
        assert!(!reply.contains("yoghurt"));
        assert!(!reply.contains("hummus"));
        assert!(reply.contains("boiled egg"));
    }

    #[test]
    fn meal_choices_leave_out_shortages_and_respect_recent_dietary_constraints() {
        let prompt = Reading::new("I'm out of eggs, what can I have for breakfast?");
        let reply = registered("breakfast-ideas", &Context::new(&prompt, &[])).unwrap();
        assert!(!reply.contains("egg"));
        let earlier = [turn("I'm allergic to peanuts and eggs")];
        let prompt = Reading::new("Could you suggest a snack?");
        let reply = registered("snack-ideas", &Context::new(&prompt, &earlier)).unwrap();
        assert!(!reply.contains("peanut"));
        assert!(!reply.contains("egg"));
        assert!(reply.contains("Greek yoghurt with berries"));
        let prompt = Reading::new("What can I make with leftover chicken?");
        let reply = registered("leftover-meal", &Context::new(&prompt, &[])).unwrap();
        assert!(reply.starts_with("Cooked chicken:"));
        assert!(reply.contains("reheat them only once"));
    }
}
