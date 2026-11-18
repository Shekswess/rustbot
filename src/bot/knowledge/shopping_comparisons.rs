//! Supermarket comparisons and value questions for appliances and larger purchases.

use std::{borrow::Cow, cmp::Reverse, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::{Exchange, Reading, knowledge::shopping_replies::said_words};

use super::{
    SHOPPING_ACTION_NAME, SHOPPING_APPLIANCE_ASK, SHOPPING_APPLIANCE_VALUE, SHOPPING_CHOICE_VALUE,
    SHOPPING_DECISION_WORDS, SHOPPING_DRYER_WORDS, SUPERMARKET_CHEAPEST_TAIL,
    SUPERMARKET_DEAR_TAIL, SUPERMARKET_MIDDLE_TAIL, SUPERMARKET_QUALITY_WORDS,
    SUPERMARKET_RANGE_WORDS, SUPERMARKET_SERVICE_WORDS, SUPERMARKET_SHOP_ASK, SUPERMARKETS,
};

pub(super) struct Store {
    pub(super) name: &'static str,
    pub(super) aliases: &'static [&'static str],
    pub(super) rank: u8,
    pub(super) description: &'static str,
    pub(super) reason: &'static str,
    pub(super) contrast: &'static str,
}

type ValueRows = &'static [(&'static [&'static str], &'static str)];

struct ValueIndex {
    patterns: RegexSet,
    rows: Vec<(&'static str, &'static str)>,
}

impl ValueIndex {
    fn new(table: ValueRows) -> Self {
        let mut rows: Vec<_> = table
            .iter()
            .flat_map(|&(names, reply)| names.iter().map(move |name| (name.trim(), reply)))
            .collect();
        rows.sort_by_key(|(name, _)| Reverse(name.chars().count()));
        let patterns = RegexSet::new(rows.iter().map(|(name, _)| whole_phrase(name)))
            .expect("shopping comparison names");
        Self { patterns, rows }
    }

    fn longest_match(&self, said: &str) -> Option<(&'static str, &'static str)> {
        let index = self.patterns.matches(said).iter().next()?;
        Some(self.rows[index])
    }
}

static APPLIANCES: LazyLock<ValueIndex> =
    LazyLock::new(|| ValueIndex::new(SHOPPING_APPLIANCE_VALUE));
static CHOICES: LazyLock<ValueIndex> = LazyLock::new(|| ValueIndex::new(SHOPPING_CHOICE_VALUE));
static ACTION_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(SHOPPING_ACTION_NAME).expect("shopping action name"));
static DECISION_ASKS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    SHOPPING_APPLIANCE_ASK
        .iter()
        .map(|ask| Regex::new(&whole_phrase(ask.trim())).expect("shopping decision ask"))
        .collect()
});
static DECISION_WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[a-z0-9']+").expect("shopping decision words"));

fn whole_phrase(phrase: &str) -> String {
    format!(r"\b{}\b", regex::escape(phrase))
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    match id {
        "supermarket-cheapest" => supermarket(reading, earlier),
        "appliance-worth" => {
            let said = said_words(reading);
            APPLIANCES
                .longest_match(&said)
                .map(|(_, reply)| Cow::Borrowed(reply))
        }
        "choice-worth" => choice_value(reading).map(Cow::Borrowed),
        _ => None,
    }
}

fn choice_value(reading: &Reading<'_>) -> Option<&'static str> {
    let said = said_words(reading);
    let (name, reply) = CHOICES.longest_match(&said)?;
    if ACTION_NAME.is_match(name) && !bare_decision(&said, name) {
        return None;
    }
    if mentions(&said, SHOPPING_DRYER_WORDS) {
        return SHOPPING_CHOICE_VALUE
            .iter()
            .find(|(names, _)| names.iter().any(|name| mentions(name, &["dryer", "drier"])))
            .map(|&(_, reply)| reply);
    }
    Some(reply)
}

fn bare_decision(said: &str, name: &str) -> bool {
    let name = Regex::new(&whole_phrase(name)).expect("shopping action phrase");
    let mut rest = name.replace_all(said, " ").into_owned();
    for ask in DECISION_ASKS.iter() {
        if let Cow::Owned(replaced) = ask.replace_all(&rest, " ") {
            rest = replaced;
        }
    }
    DECISION_WORD
        .find_iter(&rest)
        .all(|word| SHOPPING_DECISION_WORDS.contains(&word.as_str()))
}

fn mentions(said: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| said.contains(phrase))
}

pub(super) fn place_is_shop(reading: &Reading<'_>) -> bool {
    const GOING: &[&str] = &[
        " pack",
        " packing",
        " trip",
        " travel",
        " travelling",
        " traveling",
        " flight",
        " fly",
        " holiday",
        " holidays",
        " visit",
        " visiting",
        " hotel",
        " weather",
        " things to do",
        " airport",
        " tourist",
        " sightseeing",
        " northern lights",
        " what to wear",
        " what to take",
        " what to see",
        " in october",
        " in november",
        " in december",
        " in january",
        " in february",
        " in march",
    ];
    const PLACE: &[&str] = &[
        " with kids",
        " with children",
        " with a family",
        " with the family",
        " with a baby",
        " how many days",
        " how long",
        " best time to",
        " when to go",
        " worth visiting",
        " things to do",
        " it worth going",
        " reykjavik",
        " blue lagoon",
        " golden circle",
        " whale watching",
        " northern lights",
        " hire car",
        " car hire",
        " road trip",
        " visa",
        " what to pack",
        " what to wear",
        " is it safe",
        " safe to",
    ];
    const PRICE: &[&str] = &[
        " cheap",
        " cheaper",
        " price",
        " prices",
        " quality",
        " fresh",
        " value",
        " compare",
        " comparison",
        " expensive",
        " basket",
        " groceries",
        " supermarket",
        " shop",
    ];
    let line = &reading.line;
    let said = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    let said = format!(" {} ", said.to_lowercase());
    !mentions(&said, PLACE) && (!mentions(&said, GOING) || mentions(&said, PRICE))
}

const SUPERMARKET_RANKING: &str = "Aldi and Lidl are the cheapest of the big names for own-brand staples, around a fifth less than Tesco or Sainsbury's. Asda is usually the cheapest of the big four, Morrisons, Tesco and Sainsbury's come next and price-match Aldi on the basics, and Waitrose and M&S are the dearest. A convenience shop - the Co-op, a corner shop - costs more again, so it is for a top-up, not the weekly shop.";

fn supermarket(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    // The pinned callback reads the last saved user input before checking the active reading.
    if earlier
        .last()
        .is_some_and(|turn| mentions(&turn.user.to_lowercase(), SUPERMARKET_RANGE_WORDS))
    {
        return None;
    }
    let line = &reading.line;
    let said = if line.text.is_empty() {
        &line.clean
    } else {
        &line.text
    };
    let said = said.to_lowercase().replace('’', "'");
    if mentions(&said, SUPERMARKET_RANGE_WORDS)
        || mentions(&said, SUPERMARKET_SERVICE_WORDS)
        || (mentions(&said, &["i shop at", "i shop in", "i get my shopping"])
            && !mentions(&said, SUPERMARKET_SHOP_ASK))
    {
        return None;
    }
    let mut named: Vec<_> = SUPERMARKETS
        .iter()
        .filter(|store| mentions(&said, store.aliases))
        .collect();
    named.sort_by_key(|store| (store.rank, store.name));
    let Some(first) = named.first() else {
        return Some(Cow::Borrowed(SUPERMARKET_RANKING));
    };
    let reply = if mentions(&said, SUPERMARKET_QUALITY_WORDS) {
        supermarket_quality(first, named.last().filter(|_| named.len() > 1).copied())
    } else if named.len() > 1 {
        supermarket_price(first, named[named.len() - 1])
    } else {
        let tail = match first.rank {
            0..=1 => SUPERMARKET_CHEAPEST_TAIL,
            2 => SUPERMARKET_MIDDLE_TAIL,
            _ => SUPERMARKET_DEAR_TAIL,
        };
        format!(
            "{} is {}: {}.{tail}",
            first.name, first.description, first.reason
        )
    };
    Some(Cow::Owned(reply))
}

fn supermarket_price(cheap: &Store, dear: &Store) -> String {
    let (cheap_name, dear_name) = (cheap.name, dear.name);
    if cheap.rank == dear.rank {
        return format!(
            "{cheap_name} and {dear_name} are level on price, so pick on what you cook: {cheap_name} {}, and {dear_name} {}. Go to whichever is nearer.",
            cheap.contrast, dear.contrast
        );
    }
    let tail = if cheap.rank >= 2 {
        " Check the price per 100g on the shelf label rather than the pack size, because the price match only covers the basics."
    } else {
        ""
    };
    format!(
        "{cheap_name} is the cheaper of the two: {}. {dear_name} {}.{tail}",
        cheap.reason, dear.contrast
    )
}

fn supermarket_quality(cheap: &Store, dear: Option<&Store>) -> String {
    let Some(dear) = dear else {
        let tail = match cheap.rank {
            0..=1 => {
                "None of that makes the staples worse: most own-brand basics come from the same factories as the named ones, so the saving is on the same food, and a basket built on ingredients rather than ready-made versions of them misses the weakest part."
            }
            2 => {
                "Much the same as the rest on quality: the basics are the same items at a price-matched price, and what it adds is range and fresh food."
            }
            3 => {
                "Much the same as the rest on quality: the basics are the same items, and what the extra buys is convenience."
            }
            _ => {
                "The extra money goes on the parts of the shop the staples do not cover, not on the basics, which are the same items under a different label, so a full shop there costs more for much the same basket."
            }
        };
        return format!(
            "{} is {}: {}. {tail}",
            cheap.name, cheap.description, cheap.reason
        );
    };
    if cheap.rank < dear.rank {
        return format!(
            "On quality they are close on the staples: most own-brand basics are made in the same factories as the named ones, so the store cupboard is the same food under a different label. It shows in the fresh food and the range - {} {}, where {} {}. For a full shop the cheaper one is fine; for a specific ingredient, or for a weekend meal, the dearer one earns it.",
            cheap.name, cheap.contrast, dear.name, dear.contrast
        );
    }
    let mut contrast = dear.contrast;
    if cheap.contrast.starts_with("costs more")
        && let Some(rest) = [
            "costs more, and ",
            "costs more and ",
            "costs more, ",
            "costs more ",
        ]
        .iter()
        .find_map(|lead| contrast.strip_prefix(lead))
    {
        contrast = rest.trim();
    }
    format!(
        "On quality they are closer than on price: the basics are the same food under a different label, so what you pay for is the fresh food and the range - {} {}, and {} {contrast}.",
        cheap.name, cheap.contrast, dear.name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supermarket_price_quality_and_place_checks() {
        let cases = [
            (
                "is aldi cheaper than lidl",
                "Aldi and Lidl are level on price, so pick on what you cook: Aldi has the cheaper staples and a smaller range, and Lidl is level on price and stronger on the bakery. Go to whichever is nearer.",
            ),
            (
                "is waitrose as good as marks and spencer",
                "On quality they are closer than on price: the basics are the same food under a different label, so what you pay for is the fresh food and the range - M&S costs more, and the ready meals are why people go, and Waitrose is better on fresh food.",
            ),
        ];
        for (prompt, expected) in cases {
            assert_eq!(
                supermarket(&Reading::new(prompt), &[]).as_deref(),
                Some(expected)
            );
        }
        for prompt in [
            "is tesco mobile any good",
            "is aldi good for gluten free",
            "i shop at tesco",
        ] {
            assert!(
                supermarket(&Reading::new(prompt), &[]).is_none(),
                "{prompt}"
            );
        }
        assert!(!place_is_shop(&Reading::new(
            "what should i pack for iceland"
        )));
        assert!(place_is_shop(&Reading::new("is iceland cheaper than asda")));
    }

    #[test]
    fn longer_appliance_names_and_bare_decisions() {
        let appliance = Reading::new("is a steam cleaner worth it");
        assert!(
            registered("appliance-worth", &appliance, &[])
                .unwrap()
                .starts_with("A steam cleaner")
        );
        let extra = Reading::new("is a vacuum sealer worth it");
        assert!(
            registered("appliance-worth", &extra, &[])
                .unwrap()
                .starts_with("A vacuum sealer")
        );

        assert!(choice_value(&Reading::new("should i go to the gym")).is_some());
        assert!(choice_value(&Reading::new("should i go to the gym with a fever")).is_none());
        let dryer = choice_value(&Reading::new(
            "heat pump or condenser dryer which should i buy",
        ));
        assert!(dryer.unwrap().starts_with("A heat pump dryer"));
        let heat_pump = choice_value(&Reading::new("should i get a heat pump")).unwrap();
        assert!(heat_pump.contains("Ofgem Boiler Upgrade Scheme eligibility"));
        let water_meter = choice_value(&Reading::new("is a water meter worth it")).unwrap();
        assert!(water_meter.contains("requested the meter voluntarily in England or Wales"));
        assert!(water_meter.contains("Ask your water company what applies to you"));
    }
}
