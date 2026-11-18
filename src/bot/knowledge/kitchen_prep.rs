//! Kitchen steps before cooking, including the source's freezer follow-up.

use std::sync::LazyLock;

use regex::Regex;

use super::{
    PREP_BLANCHABLE, PREP_FOOD, PREP_FREEZING, PREP_NOT_FOOD, PREP_STEP_VERBS, guards::Context,
};

#[derive(Clone, Copy, Debug)]
pub(super) enum Gate {
    Needed,
    HowOrWhether,
    Freezing,
    PatDry,
}

#[derive(Clone, Copy)]
pub(super) struct Reply {
    pub(super) cases: &'static [(&'static [&'static str], &'static str)],
    pub(super) default: &'static str,
    pub(super) freezing: Option<&'static str>,
}

const NEED: &[&str] = &[
    "need",
    "have to",
    "has to",
    "should",
    "must",
    "necessary",
    "worth",
    "point",
    "bother",
    "essential",
    "important",
];
const HOW: &[&str] = &["how do i", "how to ", "how should i", "how do you"];
const POTATOES: &[&str] = &["potato", "potatoes", "chips", "chip", "fries"];

static FOOD: LazyLock<Regex> = LazyLock::new(|| Regex::new(PREP_FOOD).unwrap());
static OTHER: LazyLock<Regex> = LazyLock::new(|| Regex::new(PREP_NOT_FOOD).unwrap());
static BLANCHABLE: LazyLock<Regex> = LazyLock::new(|| Regex::new(PREP_BLANCHABLE).unwrap());

fn said(context: &Context<'_>) -> String {
    let line = &context.reading.line;
    let text = if !line.clean.is_empty() {
        &line.clean
    } else if !line.text.is_empty() {
        &line.text
    } else {
        return format!(" {} ", line.words.join(" ").to_lowercase());
    };
    format!(" {} ", text.to_lowercase())
}

fn any_phrase(line: &str, aliases: &[&str]) -> bool {
    let word_char = |character: char| character.is_alphanumeric() || character == '_';
    aliases.iter().any(|alias| {
        line.match_indices(alias).any(|(start, matched)| {
            !line[..start].ends_with(word_char)
                && !line[start + matched.len()..].starts_with(word_char)
        })
    })
}

impl Gate {
    pub(super) fn passes(self, context: &Context<'_>) -> bool {
        let text = said(context);
        if OTHER.is_match(&text) {
            return false;
        }
        let food = FOOD.is_match(&text);
        let needed = food && NEED.iter().any(|word| text.contains(word));
        match self {
            Self::Needed => needed,
            Self::HowOrWhether => needed || food && HOW.iter().any(|ask| text.contains(ask)),
            Self::PatDry => needed && text.contains(" dry"),
            Self::Freezing => {
                needed
                    || any_phrase(&text, PREP_FREEZING)
                        && !any_phrase(&text, POTATOES)
                        && (BLANCHABLE.is_match(&text)
                            || !food
                                && [" if ", " and ", "what about", "then ", "when "]
                                    .iter()
                                    .any(|phrase| text.contains(phrase)))
            }
        }
    }
}

impl Reply {
    pub(super) fn answer(self, context: &Context<'_>) -> Option<&'static str> {
        let line = said(context);
        if let Some(reply) = self.freezing
            && any_phrase(&line, PREP_FREEZING)
        {
            return (!any_phrase(&line, POTATOES)
                && (BLANCHABLE.is_match(&line) || previously_prepared_food(context)))
            .then_some(reply);
        }
        Some(
            self.cases
                .iter()
                .find_map(|(names, reply)| any_phrase(&line, names).then_some(*reply))
                .unwrap_or(self.default),
        )
    }
}

fn previously_prepared_food(context: &Context<'_>) -> bool {
    context.earlier.last().is_some_and(|previous| {
        let line = previous
            .user
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        !OTHER.is_match(&line)
            && FOOD.is_match(&line)
            && PREP_STEP_VERBS.iter().any(|words| any_phrase(&line, words))
    })
}
