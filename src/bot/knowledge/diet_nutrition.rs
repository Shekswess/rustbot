//! Nutrition comparisons and dietary reference cards.

use super::{cooking_nutrition, diet_nutrition_data as data, guards::Context};
use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

static CHILD_AGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::CHILD_AGE).expect("valid child age"));
static TEMPERATURE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::THERMOMETER).expect("valid thermometer cue"));
static COOKING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::COOKING_VERBS).expect("valid cooking cues"));

fn words(raw: &str) -> String {
    let text: String = raw
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '\'' {
                ch.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    format!(
        " {} ",
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    )
}

fn has(line: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| line.contains(term))
}

fn names(line: &str, terms: &[&str]) -> bool {
    terms
        .iter()
        .any(|term| line.contains(&format!(" {} ", term.trim())))
}

#[derive(Clone, Copy, PartialEq)]
enum Person {
    Speaker,
    Adult,
    Child,
}

fn person(text: &str) -> Option<Person> {
    if names(text, data::ME_ASKING) {
        return Some(Person::Speaker);
    }
    let child = names(text, data::A_CHILD)
        || CHILD_AGE
            .captures_iter(text)
            .filter_map(|age| age[1].parse::<u8>().ok())
            .any(|age| age < 18);
    if !child && names(text, data::A_GROWN_UP) {
        return Some(Person::Adult);
    }
    (child || names(text, data::FUSSY)).then_some(Person::Child)
}

pub(super) fn eat_veg(context: &Context<'_>) -> Option<Cow<'static, str>> {
    let line = words(context.reading.raw);
    let who = person(&line).or_else(|| {
        let someone_else = names(&line, data::SOMEONE_ELSE);
        context
            .earlier
            .iter()
            .rev()
            .map(|turn| words(&turn.user))
            .filter(|previous| previous != &line)
            .find_map(|previous| person(&previous))
            .filter(|who| !(someone_else && *who == Person::Speaker))
    });
    Some(Cow::Borrowed(match who {
        Some(Person::Speaker) => data::VEG_FIVE_A_DAY,
        Some(Person::Adult) => {
            "Serve the vegetables separately so each person can choose their own portion. Try a new vegetable alongside a dish they already enjoy, and ask which flavours or textures they prefer."
        }
        Some(Person::Child) => {
            "Offer a small amount alongside food your child already eats, with no pressure or reward for finishing it. Let them see you eating it and try again another day. Try different textures or add vegetables to familiar sauces and soups."
        }
        None => data::VEG_WHO_IS_IT,
    }))
}

pub(super) fn sugary_drink(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let raw = words(reading.raw);
    let drink = data::SWEET_DRINKS
        .iter()
        .find(|drink| names(&raw, &[drink]))
        .copied()
        .unwrap_or("cola");
    Some(Cow::Owned(format!(
        "If you drink regular {drink}, check the sugar per serving on the label because recipes and serving sizes vary. The UK adult guide is no more than 30g of free sugars a day. Water, unsweetened fizzy water and sugar-free versions can help reduce the amount you get from drinks."
    )))
}

pub(super) fn wholegrain(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = words(reading.raw);
    let pasta = names(
        &text,
        &[
            "pasta",
            "spaghetti",
            "macaroni",
            "noodles",
            "penne",
            "fusilli",
            "tagliatelle",
        ],
    );
    if pasta && !names(&text, &["rice"]) {
        return Some(Cow::Borrowed(
            "Wholewheat pasta keeps the bran and germ, so it usually has more fibre than white pasta. Both provide starchy carbohydrate. Follow the packet for timing, or try mixing wholewheat and white pasta if you prefer a gentler change.",
        ));
    }
    Some(Cow::Borrowed(
        "Brown rice keeps the bran and germ, so it usually has more fibre than white rice. Wholewheat pasta is a similar option. Both white and wholegrain versions provide starchy carbohydrate, and their calories depend on the product and portion size.",
    ))
}

pub(super) fn not_cooking(reading: &Reading<'_>) -> bool {
    let line = format!("{} {}", reading.line.text, reading.line.clean).to_lowercase();
    !COOKING.is_match(&line) && !TEMPERATURE.is_match(&line)
}

pub(super) fn red_meat(_: &Reading<'_>) -> Option<Cow<'static, str>> {
    Some(Cow::Borrowed(
        "UK guidance recommends cutting down to an average of 70g a day, cooked weight, if you currently eat more than 90g of red and processed meat a day. Red meat provides iron and zinc, while eating a lot of red or processed meat can raise bowel cancer risk. Try smaller portions or choose fish, beans or lentils for some meals.",
    ))
}

pub(super) fn nutrient(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = words(&reading.line.clean);
    if cooking_nutrition::names_nutrient_food(&text) {
        return None;
    }
    if id == "nutrient-a-day" {
        let (_, selected) = data::DAY_WORDS
            .iter()
            .find(|(alias, _)| names(&text, &[*alias]))?;
        let (_, adult, teen, woman) = data::DAILY.iter().find(|(name, ..)| name == selected)?;
        let reply = if let Some(teen) = teen.filter(|_| has(&text, data::DAY_TEEN)) {
            teen
        } else if let Some(woman) = woman.filter(|_| names(&text, data::DAY_WOMAN)) {
            woman
        } else {
            adult
        };
        return Some(Cow::Borrowed(reply));
    }
    let table = if id == "more-of-a-nutrient" {
        data::MORE_OF
    } else {
        data::ROLE
    };
    table
        .iter()
        .find(|(aliases, _)| names(&text, aliases))
        .map(|(_, reply)| Cow::Borrowed(*reply))
}

pub(super) fn cereal(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = words(reading.raw);
    let mut bowls = Vec::new();
    for (aliases, reply, short) in data::CEREAL_BOWLS {
        if let Some(position) = aliases
            .iter()
            .filter_map(|alias| text.find(&format!(" {alias} ")))
            .min()
        {
            bowls.push((position, aliases[0], *reply, *short));
        }
    }
    bowls.sort_unstable();
    if bowls.len() > 1 {
        let [(_, first, _, first_short), (_, second, _, second_short), ..] = &bowls[..] else {
            unreachable!();
        };
        if matches!(
            (*first, *second),
            ("porridge", "granola")
                | ("granola", "porridge")
                | ("porridge", "muesli")
                | ("muesli", "porridge")
        ) {
            return Some(Cow::Owned(format!(
                "Plain porridge gives you wholegrain oats with no added sugar. {first_short} {second_short} Compare products using the fibre and sugar figures per 100g."
            )));
        }
        return Some(Cow::Owned(format!("{first_short} {second_short}")));
    }
    Some(Cow::Borrowed(
        bowls.first().map_or(data::PORRIDGE, |entry| entry.2),
    ))
}

pub(super) fn milk(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = words(reading.raw);
    if names(&text, data::MILK_FOR_A_CHILD) {
        return None;
    }
    if names(&text, data::THE_PLANET) {
        return Some(Cow::Borrowed(
            "Plant drinks such as oat, soya and almond generally have lower greenhouse gas emissions and use less land than dairy milk. Almond drinks can require more irrigation water than oat or soya drinks. The comparison varies by how and where they were produced.",
        ));
    }
    let named: Vec<_> = data::PLANT_MILKS
        .iter()
        .filter(|(forms, _)| names(&text, forms))
        .map(|(_, reply)| *reply)
        .take(2)
        .collect();
    if let [first, second] = named.as_slice() {
        return Some(Cow::Owned(format!(
            "{first} {second} Compare the carton labels for protein, sugar and fortification."
        )));
    }
    if let [one] = named.as_slice() {
        return Some(Cow::Borrowed(one));
    }
    if names(&text, data::WHICH_MILK) {
        return Some(Cow::Borrowed(
            "For a dairy alternative, choose an unsweetened drink fortified with calcium and check for B12 and iodine if you rely on it. Soya and pea drinks often have more protein than oat, almond or rice drinks. Compare the cartons because nutrition varies by brand.",
        ));
    }
    let reply = "Cow's milk provides protein and calcium. Semi-skimmed milk retains both with less fat than whole milk. For a dairy alternative, check the label for calcium, protein and added vitamins.";
    if names(&text, data::BARE_OK) && !names(&text, data::NOURISHING) {
        return Some(Cow::Owned(format!(
            "{reply} For an opened bottle, check its use-by date and storage instructions. Discard it if it has gone sour, is past the use-by date, or was not kept chilled."
        )));
    }
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Exchange;

    #[test]
    fn daily_iron_and_sources_yield_to_amounts_in_food() {
        assert!(
            nutrient(
                "nutrient-a-day",
                &Reading::new("How much iron does an adult woman need a day?")
            )
            .unwrap()
            .contains("14.8mg")
        );
        assert!(
            nutrient(
                "more-of-a-nutrient",
                &Reading::new("How much protein in an egg?")
            )
            .is_none()
        );
        assert!(
            nutrient(
                "more-of-a-nutrient",
                &Reading::new("Where can I get more iron?")
            )
            .is_some()
        );
    }

    #[test]
    fn children_and_adults_receive_different_vegetable_advice() {
        let reading = Reading::new("He won't touch veg");
        let earlier = [Exchange {
            user: "My 40 year old husband is a fussy eater".into(),
            bot: String::new(),
            module: "chat",
        }];
        assert!(
            eat_veg(&Context::new(&reading, &earlier))
                .unwrap()
                .contains("separately")
        );
        let reading = Reading::new("My 4 year old won't eat vegetables");
        assert!(
            eat_veg(&Context::new(&reading, &[]))
                .unwrap()
                .contains("your child")
        );
    }

    #[test]
    fn cereals_and_milks_compare_the_named_products() {
        let both = cereal(&Reading::new("Which is healthier, granola or porridge?")).unwrap();
        assert!(both.contains("Plain porridge") && both.contains("Granola"));
        assert!(
            milk(&Reading::new("Which is better, oat milk or almond milk?"))
                .unwrap()
                .contains("carton labels")
        );
        assert!(milk(&Reading::new("Can my toddler drink oat milk?")).is_none());
        assert!(!not_cooking(&Reading::new(
            "What temperature should I cook pork at?"
        )));
    }
}
