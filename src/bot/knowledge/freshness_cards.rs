//! Cooling, room-temperature storage, and produce freshness.

use super::{freshness_card_data as data, guards::Context};
use crate::bot::Reading;
use fancy_regex::Regex as FancyRegex;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

macro_rules! pattern {
    ($name:ident, $kind:ident, $value:expr) => {
        static $name: LazyLock<$kind> =
            LazyLock::new(|| $kind::new($value).expect(stringify!($name)));
    };
}

pattern!(FREEZER, Regex, data::FREEZER_WORD);
pattern!(OUT_OF_FREEZER, Regex, data::OUT_OF_THE_FREEZER);
pattern!(LEAVING, FancyRegex, data::LEAVING_OUT);
pattern!(CHICKEN_OUT, Regex, data::CHICKEN_OUT);
pattern!(LEFT_OUT, Regex, data::LEFT_OUT_WORD);
pattern!(LEFTOVER, Regex, data::LEFTOVER_WORD);
pattern!(OUT_OF_PLACE, Regex, data::OUT_OF_PLACE_WORD);
pattern!(SAID_OUT, Regex, data::SAID_OUT);
pattern!(TAKING_OUT, Regex, data::TAKING_OUT);
pattern!(SPENT_ON_THE_SIDE, Regex, data::SPENT_ON_THE_SIDE);
pattern!(A_BIRD, Regex, data::A_BIRD);
pattern!(SPAN, Regex, data::SPAN);
pattern!(A_PERIOD, FancyRegex, data::A_PERIOD_OF_TIME);
pattern!(COLD_MACHINE, Regex, data::COLD_MACHINE);
pattern!(IN_DISH, Regex, data::IN_THE_DISH);
pattern!(FAULT, Regex, data::A_FAULT_IN_THE_FOOD);
pattern!(NOT_EATING, Regex, data::NOT_WANTING_TO_EAT);
pattern!(MAKE_STATE, Regex, data::HOW_TO_MAKE_IT_SO);
pattern!(
    OVERNIGHT,
    Regex,
    r"\b(?:overnight|all night|the night|night|next morning|in the morning)\b"
);
pattern!(
    PAST_A_WHILE,
    Regex,
    r"\b(?:all afternoon|all day|hours and hours)\b"
);
pattern!(
    HOT,
    Regex,
    r"\b(?:hot day|heatwave|in the sun|sunny|summer|hot car|warm car)\b"
);
pattern!(
    TIMING,
    Regex,
    r"\b(?:what time|when should i|when do i|what time do i)\b"
);
pattern!(
    CHILL,
    Regex,
    r"\b(?:fridge|freez\w*|refrigerat\w*|cool\w*|cold|ice|chill\w*|worktop|defrost\w*)\b"
);

const RICE: &str = "Cool cooked rice quickly, ideally within an hour. Divide it into shallow portions, cover and refrigerate it. Use it within 24 hours and reheat it only once until steaming hot throughout.";
const LEFTOVERS: &str = "Cool cooked food promptly in shallow portions and put it in the fridge within two hours. Use leftovers within 48 hours or freeze them. If reheating, heat only once until steaming hot throughout.";
const FORMULA: &str = "Use freshly made infant formula where possible. If you need to store a made-up feed, cool it quickly in cold water and refrigerate it for up to 24 hours. Use it within two hours at room temperature and throw away any formula left after a feed.";
const BREAST_MILK: &str = "Follow NHS storage times for expressed breast milk. Once a baby has drunk from a bottle, use the rest within one hour and discard anything left. Use previously frozen breast milk once defrosted, and discard any unused milk.";

fn clean(reading: &Reading<'_>) -> String {
    if reading.line.clean.is_empty() {
        reading.raw.to_lowercase()
    } else {
        reading.line.clean.to_lowercase()
    }
}

fn has(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

fn words(text: &str) -> String {
    let spaced: String = text
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '\'' {
                ch
            } else {
                ' '
            }
        })
        .collect();
    format!(
        " {} ",
        spaced.split_whitespace().collect::<Vec<_>>().join(" ")
    )
}

fn named(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|word| text.contains(&format!(" {word} ")))
}

fn hours_out(text: &str) -> Option<f32> {
    if let Some(found) = SPAN.captures(text) {
        let count = data::COUNT_WORDS
            .iter()
            .find(|(phrase, _)| *phrase == &found[1])
            .map(|(_, count)| *count)
            .or_else(|| found[1].parse().ok())?;
        return Some(if found[2].starts_with("min") {
            count / 60.0
        } else {
            count
        });
    }
    if OVERNIGHT.is_match(text) {
        return Some(8.0);
    }
    PAST_A_WHILE.is_match(text).then_some(6.0)
}

pub(super) fn left_out_guard(reading: &Reading<'_>) -> bool {
    let line = clean(reading);
    !(FREEZER.is_match(&line) && !OUT_OF_FREEZER.is_match(&line)
        || named(&words(&line), &["soak", "soaking", "soaked", "soaks"])
            && !has(&line, data::OUT_OF_THE_FRIDGE))
}

fn says_left_out(line: &str) -> bool {
    has(line, data::LEFT_OUT_PHRASE)
        || has(line, data::OUT_OF_THE_FRIDGE)
        || LEAVING.is_match(line).unwrap_or(false)
}

fn inherits_food(line: &str, previous: &str) -> bool {
    let line = words(line);
    let previous = words(previous);
    let current_foods: Vec<_> = data::LEFT_OUT_FOODS
        .iter()
        .copied()
        .filter(|food| named(&line, &[*food]))
        .collect();
    named(&previous, data::LEFT_OUT_FOODS)
        && (current_foods.is_empty() || named(&previous, &current_foods))
}

fn room_temperature(text: &str, duration: Option<f32>, hot: bool) -> Option<Cow<'static, str>> {
    let line = words(text);
    let asks_for_limit = text.contains("how long");
    let breast_milk = named(&line, &["breast milk", "expressed milk"]);
    if breast_milk {
        return Some(Cow::Borrowed(BREAST_MILK));
    }
    if named(&line, data::LEFT_OUT_BOTTLE) {
        let discard = duration.is_some_and(|hours| hours > 2.0) || hot;
        return Some(if discard && !asks_for_limit {
            Cow::Owned(format!("Discard the made-up formula. {FORMULA}"))
        } else {
            Cow::Borrowed(FORMULA)
        });
    }
    if named(&line, data::LEFT_OUT_RICE) {
        return Some(
            if duration.is_some_and(|hours| hours > 1.0) && !asks_for_limit {
                Cow::Owned(format!(
                    "Discard the rice. Reheating does not remove toxins that can form if cooked rice sits out too long. {RICE}"
                ))
            } else {
                Cow::Borrowed(RICE)
            },
        );
    }
    let egg = named(&line, data::LEFT_OUT_EGG);
    let cooked_egg = egg && named(&line, data::EGG_COOKED);
    if egg && !cooked_egg && !named(&line, data::LEFT_OUT_MEAT) {
        return Some(Cow::Borrowed(
            "Store whole eggs as directed on the box, ideally in the fridge at a consistent temperature. Check their best-before date and discard eggs with cracked shells. Cooked or cracked eggs and dishes containing them need prompt refrigeration.",
        ));
    }
    let perishable = cooked_egg
        || named(&line, data::LEFT_OUT_SOFT_CHEESE)
        || named(&line, data::LEFT_OUT_DAIRY)
        || named(&line, data::LEFT_OUT_MEAT)
        || named(&line, &["leftovers", "leftover", "takeaway"]);
    if !perishable && named(&line, data::LEFT_OUT_KEEP) {
        return Some(Cow::Borrowed(
            "Follow the storage instructions for that food. Plain bread, uncut fruit and unopened shelf-stable foods generally keep at room temperature. Foods with perishable fillings and products labelled 'keep refrigerated' need to stay cold.",
        ));
    }
    if asks_for_limit || duration.is_none() {
        return Some(Cow::Borrowed(
            "Refrigerate food that needs chilling promptly. Cool cooked leftovers and put them in the fridge within two hours, and chill cooked rice within an hour. Use a cool bag with ice packs if you are travelling. If perishable food spent the night out, discard it.",
        ));
    }
    if duration.is_some_and(|hours| hours > 2.0) {
        return Some(Cow::Borrowed(
            "Discard perishable food that has been left warm that long. Harmful bacteria may grow without changing the smell or appearance. Keep food that needs refrigeration cold, and get cooked leftovers chilled promptly.",
        ));
    }
    Some(Cow::Borrowed(
        "Put food that needs chilling into the fridge now, provided it has stayed within the safe storage time and its use-by date. Keep it at 5°C or below and follow its storage label. Use properly cooled leftovers within 48 hours, or sooner for foods such as rice.",
    ))
}

fn left_out(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let mut line = clean(context.reading);
    if !says_left_out(&line)
        && has(&line, data::STILL_OK_ASK)
        && let Some(turn) = context.earlier.last()
    {
        let previous = turn.user.to_lowercase();
        if says_left_out(&previous) && inherits_food(&line, &previous) {
            line = format!("{previous} {line}");
        }
    }
    let duration = hours_out(&line);
    if !says_left_out(&line)
        || CHICKEN_OUT.is_match(&line) && !LEFT_OUT.is_match(&line)
        || TAKING_OUT.is_match(&line)
            && A_BIRD.is_match(&line)
            && !SPENT_ON_THE_SIDE.is_match(&line)
        || has(&line, data::COOKING_RESULT) && duration.is_none() && !SAID_OUT.is_match(&line)
        || LEFTOVER.is_match(&line) && !OUT_OF_PLACE.is_match(&line)
        || has(&line, data::BURNT_WORK) && has(&line, data::BURNT_VESSEL)
    {
        return None;
    }
    room_temperature(&line, duration, HOT.is_match(&line))
}

pub(super) fn car_guard(reading: &Reading<'_>) -> bool {
    !named(
        &words(&clean(reading)),
        &[
            "dog", "dogs", "puppy", "pup", "puppies", "cat", "cats", "kitten", "kittens", "pet",
            "pets",
        ],
    )
}

pub(super) fn in_car(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = clean(reading);
    let duration = hours_out(&line);
    let hot = HOT.is_match(&line);
    let food = words(&line);
    if named(&food, data::LEFT_OUT_BOTTLE)
        || named(&food, data::LEFT_OUT_RICE)
        || duration.is_some()
    {
        return room_temperature(&line, duration, hot);
    }
    if hot {
        return Some(Cow::Borrowed(
            "A parked car can heat up quickly. Carry food that needs chilling in a cool bag with ice packs, then refrigerate it as soon as you get home. Discard perishable food that was left warm in the car all afternoon, even if it smells fine.",
        ));
    }
    Some(Cow::Borrowed(
        "Take chilled groceries straight home and put them into the fridge as soon as you arrive. Use a cool bag with ice packs for a longer trip. If food that needs chilling spent the afternoon warm, discard it.",
    ))
}

pub(super) fn marinate(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = words(&clean(reading));
    let timing = if named(
        &line,
        &["fish", "salmon", "prawn", "prawns", "shrimp", "seafood"],
    ) {
        "Fish and prawns usually need only 30 minutes to an hour, especially in an acidic marinade."
    } else if named(&line, &["chicken", "turkey"]) {
        "Chicken or turkey can marinate for a few hours or overnight in the fridge if it is within its use-by date."
    } else {
        "Tougher cuts can marinate overnight in the fridge. Steaks and chops usually need only a few hours."
    };
    Some(Cow::Owned(format!(
        "{timing} Marinate raw meat or fish in a covered container in the fridge. For a simple marinade, combine three parts oil with one part lemon juice or vinegar and add herbs or garlic. Discard the used marinade, or bring it to a full boil before reusing it as a sauce."
    )))
}

pub(super) fn cooling_guard(reading: &Reading<'_>) -> bool {
    let line = reading.line.words.join(" ");
    let line = if line.is_empty() {
        clean(reading)
    } else {
        line
    };
    !A_PERIOD.is_match(&line).unwrap_or(false)
        && (!COLD_MACHINE.is_match(&line) || IN_DISH.is_match(&line))
}

fn cool(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let line = clean(context.reading);
    let words = words(&line);
    if !named(&words, data::A_COOLING_SUBJECT)
        || FAULT.is_match(&line)
        || TIMING.is_match(&line) && !CHILL.is_match(&line)
        || has(&line, data::GADGETS)
        || context
            .earlier
            .iter()
            .rev()
            .take(3)
            .any(|turn| has(&turn.user.to_lowercase(), data::GADGETS))
    {
        return None;
    }
    if named(&words, &["rice"]) {
        return Some(Cow::Borrowed(RICE));
    }
    if named(&words, &["breast milk", "expressed milk"]) {
        return Some(Cow::Borrowed(BREAST_MILK));
    }
    if named(&words, &["formula", "baby milk", "baby bottle"]) {
        return Some(Cow::Borrowed(FORMULA));
    }
    if named(
        &words,
        &[
            "hot milk",
            "warm milk",
            "bottle of milk",
            "milk",
            "bottle",
            "bottles",
        ],
    ) {
        return Some(Cow::Borrowed(
            "Stand the closed bottle in cold water to cool it, then put it in the fridge promptly. If this is a baby's bottle, follow the storage guidance for the particular formula or expressed breast milk, and discard milk left after feeding as directed.",
        ));
    }
    if named(
        &words,
        &[
            "cake", "cakes", "sponge", "brownie", "brownies", "muffin", "muffins", "cupcake",
            "cupcakes", "loaf", "loaves", "traybake", "bread", "buns",
        ],
    ) {
        return Some(Cow::Borrowed(
            "Leave the cake in the tin briefly, then turn it onto a cooling rack. Let it cool before adding icing. Refrigerate cakes with fresh cream or any filling that needs chilling.",
        ));
    }
    if named(
        &words,
        &[
            "quickly", "quick", "fast", "faster", "fastest", "quickest", "rapidly", "speed",
            "sooner", "hurry",
        ],
    ) {
        return Some(Cow::Owned(format!(
            "To cool a pot quickly, divide the food into shallow containers or stand the pot in a sink of cold water and stir. {LEFTOVERS}"
        )));
    }
    Some(Cow::Borrowed(LEFTOVERS))
}

pub(super) fn past_best_guard(reading: &Reading<'_>) -> bool {
    let line = clean(reading);
    !NOT_EATING.is_match(&line)
        && !MAKE_STATE.is_match(&line)
        && named(&words(&line), data::PAST_IT_STATE)
}

pub(super) fn past_best(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = clean(reading);
    let words = words(&line);
    if !named(&words, data::PAST_IT_PRODUCE) {
        return None;
    }
    if named(&words, data::PAST_IT_BIN) {
        return Some(Cow::Borrowed(
            "Discard mouldy, slimy or rotten produce. Cooking does not make spoiled food safe. Clean the container or drawer where it was stored.",
        ));
    }
    if named(&words, &["use by", "use-by"]) {
        return Some(Cow::Borrowed(
            "Follow the use-by date on the pack. Discard food past that date even if it looks and smells fine.",
        ));
    }
    data::PAST_IT_KINDS
        .iter()
        .find(|(names, _)| named(&words, names))
        .map(|(_, reply)| Cow::Borrowed(*reply))
        .or(Some(Cow::Borrowed("A little wilting or shrivelling can be a loss of moisture. Check for spoilage and follow any use-by date. If it has only lost crispness, it can still work in a cooked dish.")))
}

pub(super) fn registered(id: &str, context: &Context<'_>) -> Option<Cow<'static, str>> {
    match id {
        "food-left-out" => left_out(context),
        "cool-before-fridge" => cool(context),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn room_temperature_thresholds_follow_the_food() {
        let rice = Reading::new("I left cooked rice on the worktop for 90 minutes.");
        assert!(
            left_out(&Context::new(&rice, &[]))
                .unwrap()
                .starts_with("Discard the rice")
        );
        let overnight = Reading::new("I left the roast chicken out overnight.");
        assert!(
            left_out(&Context::new(&overnight, &[]))
                .unwrap()
                .starts_with("Discard perishable")
        );
        assert!(!left_out_guard(&Reading::new(
            "How long can I keep chicken in the freezer?"
        )));
        assert!(left_out_guard(&Reading::new(
            "How long can chicken stay out of the freezer?"
        )));
    }

    #[test]
    fn food_safety_followups_use_only_the_same_food() {
        let previous = [Exchange {
            user: "I left chicken on the counter overnight".to_owned(),
            bot: String::new(),
            module: "chat",
        }];
        let follow = Reading::new("Can I still cook it?");
        assert!(
            left_out(&Context::new(&follow, &previous))
                .unwrap()
                .starts_with("Discard perishable")
        );
        let unrelated = Reading::new("Can I still cook the beef?");
        assert!(left_out(&Context::new(&unrelated, &previous)).is_none());
    }

    #[test]
    fn cooling_marination_and_produce_have_separate_answers() {
        let rice = Reading::new("Can I put hot rice in the fridge?");
        assert!(cooling_guard(&rice));
        assert!(
            cool(&Context::new(&rice, &[]))
                .unwrap()
                .contains("within 24 hours")
        );
        assert!(!cooling_guard(&Reading::new(
            "My fridge is taking ages to cool down"
        )));
        assert!(
            marinate(&Reading::new("Can I marinate chicken overnight?"))
                .unwrap()
                .contains("in the fridge")
        );
        assert!(
            past_best(&Reading::new("Can I eat slimy carrots?"))
                .unwrap()
                .starts_with("Discard")
        );
        assert!(!past_best_guard(&Reading::new("How do I dry chillies?")));
    }
}
