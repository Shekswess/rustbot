//! Cash and card choices, purchase returns, and day-to-day spending.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::Reading;

use super::{
    money_household::pounds,
    money_savings::{count, sums},
    prices_catalogue::price_is_judged,
    updated_guidance::REFUND_OR_REPAIR,
};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(
    RECEIPT,
    r"(?i)\b(?:receipt|receipts|proof of purchase|paper slip|bank statement|card statement)\b"
);
pattern!(
    CASH_ON_HAND,
    r"\bhow much cash\b|\bcash (?:at home|in the house|on me|in my purse|in my wallet)\b|\bkeep (?:some )?cash\b|\bemergency cash\b"
);
pattern!(
    CASH_REFUSED,
    r"\brefuse[sd]? cash\b|\bcash only\b|\bno cash\b|\bnot tak(?:e|es|ing) cash\b|\bwont take cash\b|\bwon't take cash\b|\blegal tender\b|\bmust (?:a shop|they|shops) take cash\b"
);
pattern!(
    HABIT_THING,
    r"\b(?:coffee\w*|lunch(?:es)?|takeaway\w*|eating out|deliveroo|just eat|fast food)\b"
);
pattern!(
    PACKED_LUNCH,
    r"\b(?:coffee\w*|lunch(?:es)?|brew|sandwich\w*)\b"
);
pattern!(SMOKING, r"\b(?:smok\w*|cigarette\w*|fags?)\b");
pattern!(
    PERIOD,
    r"\b(?:a|per|each|every)\s+(day|week|month)\b|\b(daily|weekly|monthly)\b"
);
pattern!(
    TIMES,
    r"\b(?:(\d+|one|two|three|four|five|six|seven|eight|nine|ten)\s+(?:days|times|mornings|lunches|coffees|takeaways|pints)|(once|twice))\s+(?:a|per|each)\s+week\b"
);
pattern!(
    EACH_PRICE,
    r"(?i)£|\b(?:pounds?|quid|each|a cup|a coffee|a pint|a pack|a bottle|per pack)\b"
);
pattern!(
    CROWD,
    r"\b(?:for|feeds?|serves?|family of|party of)\s+(\d+|two|three|four|five|six|seven|eight|nine|ten)\b|\b(\d+|two|three|four|five|six|seven|eight|nine|ten)\s+(?:people|adults|guests|portions)\b"
);

struct Dish {
    names: &'static [&'static str],
    cents: u32,
    makes: Option<&'static str>,
    main_cost: &'static str,
}

impl Dish {
    const fn portion(names: &'static [&'static str], cents: u32, main_cost: &'static str) -> Self {
        Self {
            names,
            cents,
            makes: None,
            main_cost,
        }
    }

    const fn whole(
        names: &'static [&'static str],
        cents: u32,
        makes: &'static str,
        main_cost: &'static str,
    ) -> Self {
        Self {
            names,
            cents,
            makes: Some(makes),
            main_cost,
        }
    }
}

const DISHES: &[Dish] = &[
    Dish::portion(
        &["chilli con carne", "chili con carne", "chilli", "chili"],
        160,
        "mince is usually the largest ingredient cost. Try lentils or beans if they work for you",
    ),
    Dish::portion(
        &["spaghetti bolognese", "spag bol", "bolognese"],
        140,
        "check the price of mince first",
    ),
    Dish::portion(
        &["lasagne", "lasagna"],
        170,
        "mince and cheese are the main costs",
    ),
    Dish::portion(
        &["cottage pie", "shepherds pie", "shepherd's pie"],
        150,
        "check the mince and potatoes first",
    ),
    Dish::portion(&["fish pie"], 200, "compare fresh and frozen fish"),
    Dish::portion(
        &["roast chicken", "roast dinner", "sunday roast", "roast"],
        230,
        "the joint is the biggest item",
    ),
    Dish::portion(
        &["curry"],
        150,
        "meat often costs the most. Check what a lentil or chickpea version would cost",
    ),
    Dish::portion(
        &["dhal", "dal", "lentil"],
        60,
        "lentils and tomatoes make up most of the ingredients",
    ),
    Dish::portion(&["soup"], 50, "price the vegetables and stock"),
    Dish::portion(&["pizza"], 90, "price the flour and cheese first"),
    Dish::portion(
        &["stew", "casserole"],
        150,
        "check the meat and compare suitable cuts",
    ),
    Dish::portion(
        &["risotto"],
        100,
        "compare the price of the cheese and wine",
    ),
    Dish::portion(
        &["stir fry"],
        125,
        "check the protein and compare frozen veg",
    ),
    Dish::portion(
        &["macaroni cheese", "mac and cheese"],
        85,
        "cheese is usually the main cost",
    ),
    Dish::portion(&["sausage and mash"], 110, "check the cost of the sausages"),
    Dish::portion(&["toad in the hole"], 110, "check the cost of the sausages"),
    Dish::portion(&["pancakes"], 30, "work out the flour, egg and milk used"),
    Dish::portion(
        &["porridge"],
        20,
        "check the oats and any milk or fruit you add",
    ),
    Dish::whole(
        &["victoria sponge", "sponge cake", "cake"],
        350,
        "an 8-slice sponge",
        "check butter and eggs",
    ),
    Dish::whole(
        &["scones"],
        120,
        "a dozen scones",
        "work out the flour, butter and milk used",
    ),
    Dish::whole(
        &["brownies"],
        280,
        "a tray of 16 brownies",
        "chocolate and butter are the largest costs",
    ),
    Dish::whole(
        &["bread", "loaf"],
        55,
        "a large loaf",
        "check the flour and yeast, and allow for oven use separately",
    ),
];
static DISH_NAMES: LazyLock<Vec<RegexSet>> = LazyLock::new(|| {
    DISHES
        .iter()
        .map(|dish| {
            RegexSet::new(
                dish.names
                    .iter()
                    .map(|name| format!(r"\b{}\b", regex::escape(name))),
            )
            .expect("ingredient-cost dish names")
        })
        .collect()
});

const EXAMPLE_PRICES: &[(&str, f64)] = &[
    ("coffee", 3.5),
    ("latte", 3.5),
    ("cappuccino", 3.5),
    ("flat white", 3.5),
    ("meal deal", 4.0),
    ("sandwich", 5.0),
    ("lunch", 7.0),
    ("takeaway", 11.0),
    ("deliveroo", 11.0),
    ("just eat", 11.0),
    ("snack", 1.5),
    ("vape", 6.0),
    ("pint", 5.5),
    ("beer", 5.5),
    ("bottle of wine", 7.0),
];

fn dish_cost(text: &str) -> Option<Cow<'static, str>> {
    if price_is_judged(text) {
        return None;
    }
    let (_, dish) = DISH_NAMES
        .iter()
        .zip(DISHES)
        .filter_map(|(patterns, dish)| {
            let length = patterns
                .matches(text)
                .into_iter()
                .map(|index| dish.names[index].len())
                .max()?;
            Some((length, dish))
        })
        .max_by_key(|(length, _)| *length)?;
    let per = f64::from(dish.cents) / 100.0;
    let result = if let Some(makes) = dish.makes {
        format!(
            "An example ingredient budget for {makes} is {}.",
            pounds(per)
        )
    } else {
        let heads = CROWD
            .captures(text)
            .and_then(|found| count(found.get(1).or_else(|| found.get(2))?.as_str()))
            .filter(|heads| *heads >= 2)
            .unwrap_or(4);
        format!(
            "At about {} a portion, an example ingredient budget for {heads} is {}.",
            pounds(per),
            pounds(per * heads as f64)
        )
    };
    let mut note = dish.main_cost.to_owned();
    note[..1].make_ascii_uppercase();
    Some(Cow::Owned(format!(
        "{result} {note}. Use current shop prices and the amounts in your recipe for a shopping total."
    )))
}

fn habit_plan(text: &str) -> &'static str {
    if PACKED_LUNCH.is_match(text) {
        "Track what you spend on coffee or bought lunches for a normal week. Pack a flask or lunch the night before on the days you want to skip buying. Set a weekly allowance for the ones you still want. If it's an app habit, remove the saved card so ordering takes a deliberate step."
    } else {
        "Track what you spend on takeaways for a normal week, including delivery charges. Decide which orders you want to keep and set that money aside. Keep a quick meal in the freezer for the other nights and remove the saved card from delivery apps if that makes it easier to pause."
    }
}

fn habit_cost(text: &str) -> Option<Cow<'static, str>> {
    let frequency = TIMES.captures(text).and_then(|found| {
        if let Some(number) = found.get(1) {
            count(number.as_str())
        } else {
            Some(if &found[2] == "once" { 1 } else { 2 })
        }
    });
    let named = sums(text).find_map(|(sum, span)| {
        let after = &text[span.end()..];
        if sum <= 0.0 || after.trim_start().starts_with('%') {
            return None;
        }
        let period = PERIOD.find(after).filter(|found| {
            matches!(
                after[..found.start()].trim(),
                "" | "spent" | "pound" | "pounds" | "quid"
            )
        });
        let marked = span.as_str().contains('£')
            || EACH_PRICE
                .find(after)
                .is_some_and(|found| found.start() <= 4);
        (period.is_some() || marked).then_some((sum, period.map(|found| found.as_str())))
    });
    if named.is_none() && SMOKING.is_match(text) {
        return Some(Cow::Borrowed(
            "At an example price of £15 for 20 cigarettes, ten a day costs roughly £2,738 a year and twenty a day costs £5,475. Multiply your own pack price by how many packs you buy a week and then by 52. NHS quit smoking services offer free support if you want to stop.",
        ));
    }
    let (price, period) = if let Some((price, interval)) = named {
        (price, interval)
    } else {
        let &(_, price) = EXAMPLE_PRICES
            .iter()
            .find(|(word, _)| text.contains(word))?;
        let period = PERIOD
            .find(text)
            .filter(|found| !found.as_str().contains("month"))
            .map(|found| found.as_str());
        (price, period)
    };
    let (annual, reduction, rate) = match period {
        Some(period) if period.contains("day") || period == "daily" => {
            (price * 365.0, price * 104.0, "a day".to_owned())
        }
        Some(period) if period.contains("month") => {
            (price * 12.0, price * 6.0, "a month".to_owned())
        }
        Some(period) if named.is_some() && period.contains("week") => {
            (price * 52.0, price * 26.0, "a week".to_owned())
        }
        _ if frequency.is_some() => {
            let count = frequency?;
            let reduction = if count <= 1 { 26.0 } else { 52.0 };
            (
                price * count as f64 * 52.0,
                price * reduction,
                format!("each, {count} times a week"),
            )
        }
        _ => (price * 52.0, price * 26.0, "a week".to_owned()),
    };
    let lead = if named.is_none() {
        "As an example, "
    } else {
        ""
    };
    Some(Cow::Owned(format!(
        "{lead}{} {rate} adds up to about {} a year. Cutting back at that price could free up around {} over a year.",
        pounds(price),
        pounds(annual),
        pounds(reduction)
    )))
}

pub(crate) fn refund_without_receipt_topic(reading: &Reading<'_>) -> bool {
    !RECEIPT.is_match(reading.raw) && !RECEIPT.is_match(&reading.clean)
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.raw.to_lowercase();
    Some(match id {
        "cash-or-card" if CASH_REFUSED.is_match(&text) => Cow::Borrowed(
            "A UK shop can generally choose which payment methods it accepts. Legal tender refers to settling a debt in specific circumstances. It does not require a shop to accept cash for a new purchase, and the rules on notes and coins vary within the UK. If you've already agreed to buy something, check the payment terms.",
        ),
        "cash-or-card" if CASH_ON_HAND.is_match(&text) => Cow::Borrowed(
            "For a payment outage, work out what you might need for essentials such as a day's travel or food. If £50 to £100 fits your budget, keep it in small notes and coins somewhere secure. Choose a sum you could afford to lose.",
        ),
        "cash-or-card" => Cow::Borrowed(
            "Cash can make a weekly spending limit easier to see. Cards are convenient and can give you a way to claim if a purchase goes wrong. Section 75 can cover eligible credit purchases with a cash price over £100 and up to £30,000. A card issuer may be able to try chargeback on a debit or credit payment. Check the terms and pay off credit in full if you can.",
        ),
        "refund" => Cow::Owned(format!(
            "{REFUND_OR_REPAIR} For most online purchases, tell the retailer you are cancelling within 14 days of receiving the goods, then return them within another 14 days. Exceptions apply, and handling beyond what is needed to inspect the goods may reduce the refund. For a change of mind after buying in a shop, check its returns policy."
        )),
        "takeaway-habit" => Cow::Borrowed(habit_plan(&text)),
        "spend-too-much" if HABIT_THING.is_match(&text) => Cow::Borrowed(habit_plan(&text)),
        "spend-too-much" => Cow::Borrowed(
            "Record a month's spending or look through your banking app and sort it into categories. Check which costs surprise you. After essentials and debt payments, decide what you can spend on extras and set that amount aside on payday.",
        ),
        "dish-cost" => return dish_cost(&text),
        "habit-cost" => return habit_cost(&text),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn habit_cost_uses_an_explicit_monthly_amount_and_dish_yields_to_price_judgement() {
        assert!(
            habit_cost("i spend £20 a month on coffee")
                .unwrap()
                .contains("£240 a year")
        );
        assert!(dish_cost("is £5 expensive for a loaf of bread").is_none());
        assert!(
            dish_cost("what does lasagne cost to make for four")
                .unwrap()
                .contains("£6.80")
        );
    }
}
