//! Fruit preparation, growing, and pip questions.

use super::fruit_card_data as data;
use crate::bot::Reading;
use std::borrow::Cow;

fn line(reading: &Reading<'_>) -> String {
    let clean = &reading.line.clean;
    let original = if clean.is_empty() {
        &reading.line.text
    } else {
        clean
    };
    let plain: String = original
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
        plain.split_whitespace().collect::<Vec<_>>().join(" ")
    )
}

fn has(line: &str, term: &str) -> bool {
    line.contains(&format!(" {term} "))
}

fn names(line: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| has(line, term))
}

pub(super) fn prepare(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = line(reading);
    if names(&line, data::JUICE_WORDS) {
        return (names(&line, data::JUICE_ASKS) && names(&line, data::CITRUS))
            .then_some(Cow::Borrowed(data::JUICING));
    }
    if !names(&line, data::PREP_WORDS) {
        return None;
    }
    data::METHODS
        .iter()
        .find(|(aliases, _)| names(&line, aliases))
        .map(|(_, method)| Cow::Borrowed(*method))
}

pub(super) fn grow(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = line(reading);
    if names(&line, &["avocado", "avocados"]) {
        return Some(Cow::Borrowed(
            "An avocado stone makes a good foliage plant and rarely fruits on a windowsill. Rinse the stone and push three cocktail sticks into it to suspend the broad end in water, or sow it directly in moist, free-draining compost. Keep it warm and bright. When roots and a shoot form, pot it up if you started it in water.",
        ));
    }
    if names(&line, data::SHOP_SEED_VEG) {
        return names(&line, data::SEED_SOURCE).then(|| {
            let lead = if names(&line, &["can i", "could i", "is it possible", "will it", "would it", "does it"]) {
                "Yes, and it is worth doing:"
            } else {
                "Save the seed first:"
            };
            Cow::Owned(format!("{lead} scoop the seeds and gel into a glass with a little water. Stir twice a day and let them ferment until the seeds settle, usually within five days. Rinse and dry the seeds, then sow indoors when you can keep them warm and give the seedlings plenty of light. Seeds from a hybrid shop tomato may grow into plants with different fruit."))
        });
    }
    if names(&line, data::CITRUS) {
        let citrus = data::CITRUS_NAMES
            .iter()
            .find(|name| has(&line, name))
            .copied()
            .unwrap_or("citrus");
        let citrus = match citrus {
            "citrus" => citrus,
            "satsumas" => "satsuma",
            _ => citrus.strip_suffix('s').unwrap_or(citrus),
        };
        return Some(Cow::Owned(format!(
            "A {citrus} seed from the fruit bowl can grow. Sow it fresh in moist seed compost, keep it warm and pot it on as it grows. Seedlings can take years to fruit and the fruit may differ from the parent. Choose a grafted tree if you want a named variety that fruits sooner."
        )));
    }
    if let Some((_, reply)) = data::TENDER
        .iter()
        .find(|(aliases, _)| names(&line, aliases))
    {
        return Some(Cow::Borrowed(reply));
    }
    if names(
        &line,
        &[
            "peach",
            "peaches",
            "nectarine",
            "nectarines",
            "plum",
            "plums",
            "cherry",
            "cherries",
            "apricot",
            "apricots",
            "damson",
            "damsons",
            "olive",
            "olives",
        ],
    ) {
        return Some(Cow::Borrowed(
            "A stone can grow, but many stone fruits need a spell of winter cold before they germinate. Clean off the flesh, chill the stone in damp compost and then sow it. The time needed varies by fruit. A seedling can take years to fruit and may differ from the fruit you ate. Choose a named grafted tree for a reliable variety.",
        ));
    }
    names(
        &line,
        &["apple", "apples", "pear", "pears", "apple tree", "pear tree", "fruit tree", "fruit pip", "fruit pips"],
    )
    .then_some(Cow::Borrowed("An apple or pear pip can grow, but the fruit will differ from the apple or pear you ate. The seedling takes years to fruit. Try chilling the pips in damp compost in the fridge, then sow them and pot on any that germinate. For a known eating variety, buy a named grafted tree."))
}

pub(super) fn eat_pips(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = line(reading);
    if names(
        &line,
        &[
            "dog", "dogs", "puppy", "cat", "cats", "kitten", "pet", "pets", "bird", "birds",
            "parrot",
        ],
    ) {
        return Some(Cow::Borrowed(
            "Keep fruit stones and pips away from pets. Chewed stones and pips from fruit such as cherries, peaches and apricots can release cyanide. A whole stone can also cause choking or a blockage. If a pet has swallowed or chewed a stone or you are unsure what it ate, call a vet promptly.",
        ));
    }
    if names(&line, data::KERNEL_FRUIT) {
        return Some(Cow::Borrowed(
            "Avoid eating raw apricot kernels and bitter almonds. Their cyanide content varies and chewing them can release cyanide. If you think someone has swallowed a harmful kernel, call 999 or go to A&E. If you are unsure whether what they swallowed is harmful, call NHS 111 for advice. Keep any packaging and do not try to make them vomit.",
        ));
    }
    let count = has(&line, "how many")
        && names(
            &line,
            &[
                "pip", "pips", "seed", "seeds", "stone", "stones", "kernel", "kernels",
            ],
        )
        && names(&line, &["in", "inside", "does", "has", "have", "are there"])
        && !names(
            &line,
            &[
                "eat",
                "eaten",
                "swallow",
                "kill",
                "poison",
                "harm",
                "toxic",
                "die",
                "dangerous",
            ],
        );
    if count && names(&line, &["apple", "apples"]) {
        return Some(Cow::Borrowed(
            "An apple normally has five chambers in the core, with up to two seeds in each. Some chambers may contain fewer, so the number of pips varies.",
        ));
    }
    if count
        && names(
            &line,
            &["lemon", "lemons", "lime", "limes", "orange", "oranges"],
        )
    {
        return Some(Cow::Borrowed(
            "Citrus seeds sit in the segments. The number depends on the variety and how it was pollinated. Some lemons, limes and oranges have several seeds and some are seedless.",
        ));
    }
    Some(Cow::Borrowed(
        "Eat the flesh and leave the pips and stones in the core. Chewing or crushing some fruit pips and kernels can release cyanide, and the amount varies by fruit. Accidentally swallowing a whole apple pip is different from chewing a stone fruit kernel. If you are unsure whether what someone swallowed is harmful, call NHS 111. If you think they swallowed something harmful, call 999 or go to A&E.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prep_chooses_the_named_method_and_stands_down_for_non_citrus_juicing() {
        let answer = prepare(&Reading::new("How do I remove the stone from an avocado?")).unwrap();
        assert!(answer.contains("spoon"));
        assert!(!answer.contains("whack"));
        assert!(prepare(&Reading::new("How do I juice a pineapple?")).is_none());
        assert!(prepare(&Reading::new("How do I juice a lemon?")).is_some());
    }

    #[test]
    fn distinguishes_source_planting_and_seed_risk() {
        assert!(
            grow(&Reading::new("Can I grow seeds from a supermarket tomato?"))
                .unwrap()
                .contains("hybrid")
        );
        assert!(grow(&Reading::new("How do I sow tomato seeds?")).is_none());
        assert!(
            eat_pips(&Reading::new("How many pips are in an apple?"))
                .unwrap()
                .contains("five chambers")
        );
        assert!(
            eat_pips(&Reading::new("Are apricot kernels dangerous to eat?"))
                .unwrap()
                .contains("NHS 111")
        );
        assert!(
            eat_pips(&Reading::new("My dog swallowed a cherry stone"))
                .unwrap()
                .contains("a vet")
        );
    }
}
