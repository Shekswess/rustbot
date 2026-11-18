//! Named food and ingestion cards from the pinned pets registry.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::Reading;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(BIN, super::PET_FOOD_BIN);
pattern!(DOG_OR_CAT, super::PET_FOOD_DOG_OR_CAT);
pattern!(HAMSTER, super::PET_FOOD_HAMSTER);
pattern!(GUINEA_PIG, super::PET_FOOD_GUINEA);
pattern!(GERBIL, r"\bgerbils?\b");
pattern!(
    BIRD,
    r"\b(?:budgie|budgies|budgerigar|parakeet|cockatiel|canary|parrot|parrots|bird|birds)\b"
);
pattern!(CHINCHILLA, r"\bchinchillas?\b");
pattern!(TORTOISE, r"\b(?:tortoises?|turtles?)\b");
pattern!(
    HORSE,
    r"\b(?:horses?|pon(?:y|ies)|foals?|mares?|geldings?)\b"
);
pattern!(RABBIT, r"\b(?:rabbits?|bunn(?:y|ies))\b");
pattern!(CAT, r"\b(?:cats?|kittens?)\b");
pattern!(NON_WORDS, r"[^a-z' ]+");
pattern!(WORDS, r"[a-z']+");
pattern!(ALLIUM, r"\b(?:onions?|garlic|leeks?|chives?|shallots?)\b");
pattern!(GRAPES, r"\b(?:grapes?|raisins?|sultanas?|currants?)\b");

type FoodRows = &'static [(&'static str, &'static str)];
pub(super) type SpeciesFoods = &'static [(&'static str, FoodRows)];

fn get(table: FoodRows, name: &str) -> Option<&'static str> {
    table
        .iter()
        .find_map(|&(key, value)| (key == name).then_some(value))
}

fn small_kind(clean: &str) -> Option<&'static str> {
    [
        ("gerbil", &*GERBIL),
        ("hamster", &*HAMSTER),
        ("guinea pig", &*GUINEA_PIG),
        ("bird", &*BIRD),
        ("chinchilla", &*CHINCHILLA),
        ("tortoise", &*TORTOISE),
        ("horse", &*HORSE),
    ]
    .into_iter()
    .find_map(|(name, pattern)| pattern.is_match(clean).then_some(name))
}

pub(super) fn leftovers_not_in_the_bin(reading: &Reading<'_>) -> bool {
    let clean = reading.line.clean.to_lowercase();
    !(BIN.is_match(&clean) && DOG_OR_CAT.is_match(&clean))
}

fn cat(reading: &Reading<'_>) -> bool {
    CAT.is_match(
        reading
            .animal_in_hand
            .as_deref()
            .unwrap_or(&reading.line.clean),
    )
}

const BONE: &str = "Call your vet now for advice if your pet swallowed a bone. Tell them the kind of bone, how big it was and when it happened. Cooked and raw bones can lodge or splinter. Struggling to breathe or choking is an emergency. Avoid trying to make your pet vomit or reaching into their mouth.";
const CHOCOLATE: &str = "Chocolate can poison pets, including dogs and cats. If your pet ate it, call your vet now, even if they seem well. Have the packaging, the type and amount of chocolate, the time they ate it and your pet's weight ready.";
const GRAPE_DOG: &str = "If your dog ate grapes, raisins, currants or sultanas, call your vet now, even if they seem well. These fruits can cause kidney failure, and there is no reliable safe amount. Tell the vet how much your dog ate and when.";
const GRAPE_CAT: &str = "If your cat ate grapes or a product containing raisins, currants or sultanas, call your vet for advice. Tell them what your cat ate, how much and when.";
const RAW_MEAT: &str = "Raw meat, fish and eggs can carry germs that can make pets and people ill. Wash hands, bowls and any surfaces the raw food touched. If your pet is vomiting repeatedly, has bloody diarrhoea, is unusually quiet or cannot keep water down, call a vet for urgent advice. Call sooner if the food also contained bones or seasoning.";

fn grape_named(clean: &str) -> bool {
    GRAPES.is_match(clean)
        || super::PET_RAISIN_FOODS
            .iter()
            .any(|food| clean.contains(food))
}

fn ate(clean: &str) -> bool {
    WORDS
        .find_iter(clean)
        .any(|word| super::PET_LEFTOVERS_EATEN.contains(&word.as_str()))
        || super::PET_LEFTOVERS_EATEN_PHRASES
            .iter()
            .any(|phrase| clean.contains(phrase))
}

fn leftovers(clean: &str, is_cat: bool) -> &'static str {
    let kind = if RABBIT.is_match(clean) {
        Some("rabbit")
    } else {
        small_kind(clean)
    };
    if ate(clean) {
        match kind {
            Some("rabbit" | "guinea pig") => {
                "Remove the leftovers and keep hay and fresh water available. If your pet stops eating or passing droppings, call a vet immediately. Call your vet for advice if you suspect anything harmful was in the leftovers, and tell them what your pet ate and when."
            }
            Some("bird") => {
                "Remove the leftovers and offer your bird its usual food and water. Call an avian vet immediately if the food contained avocado, chocolate or caffeine, or if your bird seems unwell. Tell the vet what it ate and when."
            }
            Some(_) => {
                "Remove the leftovers and offer your pet its usual food and water. Call your vet if the scraps may have contained something harmful or if your pet develops diarrhoea, stops eating or seems unwell. Tell the vet what it ate and when."
            }
            None => {
                "Check what was in the leftovers. Call your vet now if they contained chocolate, onion, garlic, a bone or a sweetener such as xylitol, or if a dog ate grapes, raisins, currants or sultanas. Tell the vet how much your pet ate and when. If they only ate plain, boneless cooked food and seem fine, keep water available and watch for vomiting, diarrhoea or a sore belly."
            }
        }
    } else if kind.is_some() {
        "Keep table scraps out of the bowl. Give your pet food suited to its species and ask your vet before offering a new food you are unsure about."
    } else if is_cat {
        "A small piece of plain cooked, boneless meat can be a treat for a cat. Keep their regular complete cat food as the main diet, and skip seasoned food, gravy and bones."
    } else {
        "A small piece of plain cooked, boneless meat or a suitable vegetable can be a treat for a dog. Leave out gravy, salt, fat and seasoning. Keep onion, garlic, grapes, raisins, chocolate, xylitol and bones out of reach."
    }
}

fn named_food(line: &str, words: &[&str], food: &str) -> bool {
    words
        .iter()
        .any(|word| *word == food || (food.len() > 3 && word.starts_with(food)))
        || (food.contains(' ') && line.contains(food))
}

fn can_eat(clean: &str, is_cat: bool) -> Cow<'static, str> {
    if grape_named(clean) {
        return Cow::Borrowed(if is_cat { GRAPE_CAT } else { GRAPE_DOG });
    }
    if (clean.contains("xylitol") || clean.contains("birch sugar"))
        && !clean.contains("xylitol free")
        && !clean.contains("xylitol-free")
    {
        return Cow::Borrowed(
            "Keep products containing xylitol away from pets. It can poison dogs. Call your vet now if your dog ate some, even if they seem well, and have the packaging to hand.",
        );
    }
    if ALLIUM.is_match(clean) {
        return Cow::Borrowed(
            "Onions, garlic, leeks, chives and shallots can harm dogs and cats. If your pet ate food containing them, call your vet for advice and tell them how much and when.",
        );
    }
    let text = NON_WORDS.replace_all(clean, " ");
    let words: Vec<_> = WORDS.find_iter(&text).map(|word| word.as_str()).collect();
    if let Some(&(name, advice)) = super::PET_CAUTION_FOODS
        .iter()
        .find(|&&(name, _)| named_food(&text, &words, name))
    {
        return match name {
            "bone" => Cow::Borrowed(BONE),
            "avocado" => Cow::Borrowed(
                "Keep the skin and stone away from dogs and cats. The stone can block their gut. Call your vet for advice if your pet swallowed the stone, is choking or seems unwell.",
            ),
            "pizza" => Cow::Borrowed(
                "Pizza may contain onion or garlic, which can harm dogs and cats. If your pet ate pizza containing either ingredient, call your vet for advice and say how much they ate.",
            ),
            "ice cream" => Cow::Borrowed(
                "Skip ice cream as a pet treat. Dairy may upset their stomach, and some flavours and sugar-free products contain chocolate or xylitol. Call your vet if your pet ate an ice cream containing a harmful ingredient.",
            ),
            "coffee" => Cow::Borrowed(
                "Caffeine can poison dogs and cats. If your pet drank coffee or ate grounds or caffeine tablets, call your vet now and tell them how much and when.",
            ),
            _ => Cow::Owned(format!("Careful: {}", advice.replace("{one}", "your pet"))),
        };
    }
    if let Some(&(name, advice)) = super::PET_SAFE_FOODS
        .iter()
        .find(|&&(name, _)| named_food(&text, &words, name))
    {
        let kind = if is_cat { "cat" } else { "dog" };
        let advice = if name == "rice" {
            "plain cooked rice is fine in a small amount"
        } else {
            advice
        };
        return Cow::Owned(format!(
            "Yes: {}. Keep it to a small amount and off the dinner plate, because a pet that begs gets fat and a fat {kind} gets joint trouble. Anything new is worth a small piece of first, and a look an hour later.",
            advice.trim_end_matches('.'),
        ));
    }
    Cow::Borrowed(
        "Keep grapes and raisins away from dogs, and keep chocolate, onions, garlic, xylitol and raw dough out of the pet bowl. Call your vet now if your pet may have eaten something harmful. For an unfamiliar food, check that it is suitable for your pet before offering it.",
    )
}

fn small_food(clean: &str) -> Option<&'static str> {
    let kind = small_kind(clean)?;
    let words: Vec<_> = WORDS.find_iter(clean).map(|word| word.as_str()).collect();
    let known = super::PET_SMALL_FOODS
        .iter()
        .find_map(|&(animal, foods)| (animal == kind).then_some(foods))
        .unwrap_or_default()
        .iter()
        .find(|&&(food, _)| {
            words.contains(&food)
                || (food.len() > 3 && words.contains(&format!("{food}s").as_str()))
        });
    if words.contains(&"chocolate") {
        return Some(CHOCOLATE);
    }
    match (kind, known) {
        ("tortoise", Some(_)) => Some(
            "A tortoise's diet depends on its species. Many commonly kept tortoises need a varied diet of suitable leafy plants and weeds, with little or no fruit. Ask a reptile vet what is safe for your tortoise before offering unfamiliar foods.",
        ),
        ("horse", Some(&("hay", _))) => Some(
            "Hay or other suitable forage is the basis of most horses' diets. The amount depends on body weight, workload, health and grazing. Ask your vet or an equine nutritionist for a feeding plan if you are unsure.",
        ),
        ("horse", Some(&("bread", _))) => Some(
            "Choose a suitable horse treat, such as a small piece of carrot if your horse's diet allows it. If your horse needs a low-sugar or low-starch diet, ask your vet which treats are suitable.",
        ),
        ("bird", Some(&("avocado", _))) => Some(
            "Keep avocado away from pet birds. Call an avian vet immediately if your bird ate any avocado, including the flesh, skin or stone.",
        ),
        (_, Some(&(_, advice))) => Some(advice),
        ("tortoise" | "horse", None) => None,
        (_, None) => get(super::PET_SMALL_DIET, kind),
    }
}

// https://www.pdsa.org.uk/pet-help-and-advice/looking-after-your-pet/all-pets/poisons-and-hazards-for-your-pets
// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/conditions/grape-raisin-currant-and-sultana-poisoning-in-dogs
// https://www.pdsa.org.uk/pet-help-and-advice/looking-after-your-pet/all-pets/raw-diets
// https://www.pdsa.org.uk/pet-help-and-advice/pet-health-hub/symptoms/my-dog-has-eaten-something-harmful
// https://prod.bhs.org.uk/horse-care-and-welfare/health-care-management/horse-health/feeding-horses/
pub(super) fn registered(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = &reading.line;
    let clean = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    let clean = clean.to_lowercase();
    let text = match id {
        "dog-chocolate" => CHOCOLATE,
        "dog-ate-bone" => BONE,
        "dog-grapes" if cat(reading) => GRAPE_CAT,
        "dog-grapes" => GRAPE_DOG,
        "dog-raw-meat" => RAW_MEAT,
        "dog-leftovers" => leftovers(&clean, cat(reading)),
        "pet-can-eat" => return Some(can_eat(&clean, cat(reading))),
        "small-pet-food" => small_food(&clean)?,
        _ => return None,
    };
    Some(Cow::Borrowed(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingestions_prioritize_vet_and_leftovers_keep_patient() {
        for (id, prompt) in [
            ("dog-chocolate", "My dog ate an easter egg"),
            ("dog-ate-bone", "My cat swallowed a chicken bone"),
            ("dog-grapes", "My dog ate raisins"),
            ("dog-leftovers", "My dog ate my dinner"),
        ] {
            let reply = registered(id, &Reading::new(prompt)).unwrap();
            assert!(reply.to_lowercase().contains("vet now"), "{reply}");
        }
        assert!(!leftovers_not_in_the_bin(&Reading::new(
            "My dog ate from the bin"
        )));
        assert!(leftovers_not_in_the_bin(&Reading::new(
            "My rabbit ate from the bin"
        )));
        assert!(leftovers("can my dog eat scraps", false).contains("plain cooked, boneless meat"));
    }

    #[test]
    fn food_tables_keep_affirmations_and_species_specific_cautions() {
        assert!(can_eat("can my dog eat chicken", false).starts_with("Yes: plain cooked chicken"));
        assert!(can_eat("my dog ate pizza with garlic", false).contains("call your vet"));
        assert!(
            small_food("can budgies eat avocado")
                .unwrap()
                .contains("avian vet immediately")
        );
        assert!(small_food("what do horses eat").is_none());
        assert!(
            small_food("can horses eat hay")
                .unwrap()
                .contains("depends on body weight")
        );
        assert!(
            small_food("can hamsters eat apples")
                .unwrap()
                .contains("without the pips")
        );
    }
}
