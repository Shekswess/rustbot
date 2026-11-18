//! Store, clothing, and gift replies that depend on wording or earlier turns.

use crate::bot::{Exchange, Reading, memory::groupages::decimal};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

#[path = "shopping_replies_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $expression:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($expression).expect(stringify!($name)));
    };
}
pattern!(INSIDE, data::INSIDE);
pattern!(MINE, data::MINE);
pattern!(HEIGHT, data::HEIGHT);
pattern!(WATCH_VERB_SHAPE, data::WATCH_VERB_SHAPE);
pattern!(HAGGLE_SHY, r"\brude\b|\bawkward\b|\bembarrass");

fn cue_patterns(phrases: &[&str], suffix: &str) -> String {
    let alternatives = phrases
        .iter()
        .map(|phrase| regex::escape(phrase.trim()))
        .collect::<Vec<_>>()
        .join("|");
    format!(r"\b(?:{alternatives}){suffix}")
}

static COAT_KINDS: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        data::COAT_KINDS
            .iter()
            .map(|(phrases, _)| cue_patterns(phrases, "")),
    )
    .expect("coat types in source order")
});
static COAT_CHECK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&cue_patterns(data::COAT_CHECK_WORDS, "")).expect("coat fitting cues")
});
static EVERYDAY: LazyLock<(RegexSet, Vec<&'static str>)> = LazyLock::new(|| {
    let aliases: Vec<_> = data::EVERYDAY_WHERE
        .iter()
        .flat_map(|&(aliases, reply)| aliases.iter().map(move |alias| (alias, reply)))
        .collect();
    let patterns = aliases
        .iter()
        .map(|(alias, _)| cue_patterns(&[alias], r"\b"));
    let mut preferred: Vec<_> = (0..aliases.len()).collect();
    preferred.sort_by_key(|&index| std::cmp::Reverse(aliases[index].0.chars().count()));
    let patterns = patterns.collect::<Vec<_>>();
    let replies = preferred.iter().map(|&index| aliases[index].1).collect();
    let set = RegexSet::new(preferred.iter().map(|&index| &patterns[index]))
        .expect("everyday purchase names in descending length and source order");
    (set, replies)
});

pub(crate) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let reply = match id {
        "gift-dad" => gift_condition(Cow::Borrowed(data::GIFT_DAD), reading, earlier),
        "gift-mum" => gift_condition(Cow::Borrowed(data::GIFT_MUM), reading, earlier),
        "coat-worth" => coat(&said_words(reading), earlier),
        "watch-buy" => Cow::Borrowed(watch(reading)?),
        "where-to-buy-everyday" => Cow::Borrowed(everyday(&said_words(reading))?),
        "haggle-price" => {
            let said = said_words(reading);
            Cow::Borrowed(if HAGGLE_SHY.is_match(&said) {
                data::HAGGLE_SHY
            } else if data::TRADER_WORDS.iter().any(|word| said.contains(word)) {
                data::HAGGLE_TRADER
            } else {
                data::HAGGLE_SHOP
            })
        }
        _ => return None,
    };
    Some(reply)
}

pub(super) fn said_words(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    [&line.clean, &line.text]
        .into_iter()
        .chain(&line.words)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn condition(said: &str) -> Option<&'static data::Condition> {
    let said = format!(" {} ", said.to_lowercase());
    data::CONDITIONS
        .iter()
        .find(|entry| entry.0.iter().any(|phrase| said.contains(phrase)))
}

fn recipient_condition<T>(
    reading: &Reading<'_>,
    earlier: &[Exchange],
    find: impl Fn(&str) -> Option<T>,
) -> Option<T> {
    let line = &reading.line;
    let typed = if line.text.is_empty() {
        &line.clean
    } else {
        &line.text
    };
    find(typed).or_else(|| {
        let said = typed.to_lowercase();
        if data::PRESENT_PEOPLE.iter().any(|word| {
            let match_word = |word: &str| {
                said.match_indices(word).any(|(start, _)| {
                    let end = start + word.len();
                    (start == 0 || said.as_bytes()[start - 1] == b' ')
                        && (end == said.len() || said.as_bytes()[end] == b' ')
                })
            };
            match_word(word) || match_word(&format!("{word}s"))
        }) {
            return None;
        }
        let previous = earlier[earlier.len().saturating_sub(2)..]
            .iter()
            .map(|turn| turn.user.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        find(&previous)
    })
}

pub(crate) fn gift_avoids_alcohol(reading: &Reading<'_>, earlier: &[Exchange]) -> bool {
    // The first source condition is for recipients who do not drink alcohol.
    recipient_condition(reading, earlier, |said| {
        let said = format!(" {} ", said.to_lowercase().replace('’', "'"));
        data::CONDITIONS[0]
            .0
            .iter()
            .any(|phrase| said.contains(phrase))
            .then_some(())
    })
    .is_some()
}

pub(crate) fn gift_condition(
    base: Cow<'static, str>,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Cow<'static, str> {
    let Some(data::Condition(_, note, drop)) = recipient_condition(reading, earlier, condition)
    else {
        return base;
    };
    if base.to_lowercase().contains(&note.to_lowercase()) {
        return base;
    }
    let mut reply = base.into_owned();
    for phrase in *drop {
        reply = reply.replace(phrase, "");
    }
    reply.truncate(reply.trim_end_matches(crate::text::is_space).len());
    reply.push(' ');
    reply.push_str(note);
    Cow::Owned(reply)
}

fn coat(said: &str, earlier: &[Exchange]) -> Cow<'static, str> {
    if let Some(height) = HEIGHT.captures(said)
        && let (Some(feet), Some(inches)) = (decimal(&height[1]), decimal(&height[2]))
    {
        let total = f64::from(feet * 12 + inches);
        let low = (total * 0.42) as u16;
        let high = (total * 0.50) as u16;
        return Cow::Owned(format!(
            "At {} foot {}, look for a coat about {low} to {high} inches from the shoulder - mid-thigh on you - so it keeps your legs warm without swamping you. Check the sleeve ends at your wrist bone with your arms down, and try it on over the thickest jumper you own.",
            &height[1], &height[2]
        ));
    }
    if let Some(first) = COAT_KINDS.matches(said).iter().next() {
        return Cow::Borrowed(data::COAT_KINDS[first].1);
    }
    if !COAT_CHECK.is_match(said) {
        return Cow::Borrowed(data::COAT_GENERIC);
    }
    if earlier.last().is_some_and(|recent| {
        let reply = recent.bot.to_lowercase();
        [
            "buys a padded coat",
            "pick the coat by the weather",
            "waterproof shell with taped seams",
        ]
        .iter()
        .any(|phrase| reply.contains(phrase))
    }) {
        Cow::Borrowed(data::COAT_CHECK)
    } else {
        Cow::Owned(format!("{} {}", data::COAT_GENERIC, data::COAT_CHECK))
    }
}

fn watch(reading: &Reading<'_>) -> Option<&'static str> {
    let line = &reading.line;
    // Python's third field is the token list representation. It keeps each token quoted
    // and separated by commas, unlike the other two textual views.
    let norm = format!("{:?}", line.norm);
    let forms = [
        line.clean.to_lowercase(),
        line.norm_text.to_lowercase(),
        norm.to_lowercase(),
    ];
    let said = forms.join(" ");
    if data::WATCH_SMART.iter().any(|name| said.contains(name)) {
        Some(data::WATCH_SMART_SAY)
    } else if data::WATCH_VERBS.iter().any(|word| said.contains(word))
        || forms.iter().any(|part| WATCH_VERB_SHAPE.is_match(part))
    {
        None
    } else {
        Some(data::WATCH_PLAIN)
    }
}

fn everyday(said: &str) -> Option<&'static str> {
    if INSIDE
        .captures(said)
        .is_some_and(|capture| !data::SHOP_PLACES.contains(&&capture[1]))
        || (MINE.is_match(said) && said.split(crate::text::is_space).any(|word| word == "find"))
    {
        return None;
    }
    let (names, replies) = &*EVERYDAY;
    names
        .matches(said)
        .iter()
        .next()
        .map(|index| replies[index])
}

#[cfg(test)]
#[path = "../../../tests/fixtures/shopping_replies.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shop_and_gift_replies_match_the_reference() {
        for fixtures::Case(id, prompt, history, reply) in fixtures::CASES {
            let reading = Reading::new(prompt);
            let earlier: Vec<_> = history
                .iter()
                .map(|&(user, bot)| Exchange {
                    user: user.to_owned(),
                    bot: bot.to_owned(),
                    module: "answers",
                })
                .collect();
            assert_eq!(
                registered(id, &reading, &earlier).as_deref(),
                *reply,
                "{id}: {prompt}"
            );
        }
    }
}
