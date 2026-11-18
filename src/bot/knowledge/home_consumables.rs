//! Household supplies and chemical-exposure replies.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::Reading;

use super::history_cards::whole_words;

#[path = "home_consumables_data.rs"]
mod data;

type NamedChoices = Vec<(usize, Regex, &'static str)>;

static CHEMICALS: LazyLock<NamedChoices> = LazyLock::new(|| named_choices(data::CHEMICALS));
static SUPPLIES: LazyLock<NamedChoices> = LazyLock::new(|| named_choices(data::SUPPLIES));
static CHILD: LazyLock<RegexSet> = LazyLock::new(|| whole_words(data::CHEMICAL_CHILD));
static PET: LazyLock<RegexSet> = LazyLock::new(|| whole_words(data::CHEMICAL_PET));
static EXPECTING: LazyLock<RegexSet> = LazyLock::new(|| whole_words(data::CHEMICAL_EXPECTING));

fn named_choices(rows: &[(&[&str], &'static str)]) -> NamedChoices {
    let mut choices: NamedChoices = rows
        .iter()
        .flat_map(|&(names, reply)| {
            names.iter().map(move |name| {
                let pattern = Regex::new(&format!(r"\b{}\b", regex::escape(name)))
                    .expect("household product name");
                (name.len(), pattern, reply)
            })
        })
        .collect();
    choices.sort_by_key(|(len, _, _)| std::cmp::Reverse(*len));
    choices
}

fn product_advice(line: &str, choices: &NamedChoices) -> Option<&'static str> {
    choices
        .iter()
        .find_map(|(_, pattern, reply)| pattern.is_match(line).then_some(*reply))
}

pub(super) fn run_out(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = format!(
        "{} {} {}",
        reading.line.clean,
        reading.line.text,
        reading.line.words.join(" ")
    )
    .to_lowercase();
    product_advice(&line, &SUPPLIES).map(Cow::Borrowed)
}

const HUMAN_EXPOSURE: &str = "If a child or anyone else may have swallowed, touched or breathed in something harmful, call 999 now in the UK or your local emergency number. If you are unsure whether the substance was harmful, call NHS 111 in the UK. Keep the packaging nearby. Do not make them vomit or give them anything to eat or drink.";
const PET_EXPOSURE: &str = "If a pet may have been exposed, call a vet now and have the label to hand. Do not try to make the pet vomit.";
const PREGNANCY: &str = "If you are pregnant, follow the product's label precautions and ask someone else to apply it if possible.";

pub(super) fn chemical_safety(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let said = reading.line.words.join(" ").to_lowercase();
    let child = CHILD.is_match(&said);
    let pet = PET.is_match(&said);
    let expecting = EXPECTING.is_match(&said);
    if !child && !pet && !expecting {
        return None;
    }
    let mut parts = Vec::with_capacity(4);
    if child || expecting {
        parts.push(HUMAN_EXPOSURE);
    }
    if pet {
        parts.push(PET_EXPOSURE);
    }
    parts.push(product_advice(&said, &CHEMICALS).unwrap_or(data::CHEMICAL_FALLBACK));
    if expecting {
        parts.push(PREGNANCY);
    }
    Some(Cow::Owned(parts.join(" ")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn household_substitutions_keep_specific_longest_alias() {
        let reply = run_out(&Reading::new(
            "I ran out of dishwasher tablets and dish soap",
        ))
        .unwrap();
        assert!(reply.starts_with("Use only dishwasher detergent"));
        let reply = run_out(&Reading::new("we are out of laundry detergent")).unwrap();
        assert!(reply.starts_with("Use laundry detergent made for your machine"));
        assert_eq!(run_out(&Reading::new("I ran out of pencils")), None);
    }

    #[test]
    fn actual_poisoning_actions_precede_product_storage() {
        let reply = chemical_safety(&Reading::new("my toddler drank bleach")).unwrap();
        assert!(reply.starts_with(HUMAN_EXPOSURE));
        assert!(reply.contains("Never mix cleaning products"));
        let reply = chemical_safety(&Reading::new("my dog licked antifreeze")).unwrap();
        assert!(reply.starts_with(PET_EXPOSURE));
        assert!(reply.contains("Keep antifreeze"));
        assert_eq!(chemical_safety(&Reading::new("what is bleach?")), None);
    }
}
