//! Household reply variants selected by the identified item and recent conversation.

use crate::bot::{Exchange, Reading};
use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

#[path = "home_reply_data.rs"]
mod data;

static TAP_TALK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(data::TAP_TALK).expect("tap conversation pattern"));
static WASHER_SOUND: LazyLock<RegexSet> = LazyLock::new(|| {
    RegexSet::new(
        data::WASHER_SOUND_PATTERNS
            .iter()
            .map(|&(pattern, _)| pattern),
    )
    .expect("washer sound patterns")
});

pub(crate) fn registered(
    id: &str,
    reading: &Reading<'_>,
    turns: &[Exchange],
) -> Option<Cow<'static, str>> {
    let reply = match id {
        "home/a-washer-two-things" => {
            if chat_about_tap(reading, turns) {
                data::THE_TAP_WASHER
            } else {
                data::WASHER_TWO_THINGS
            }
        }
        "home/washer-sound" => {
            let said = reading.line.clean.to_lowercase();
            let kind = WASHER_SOUND
                .matches(&said)
                .into_iter()
                .next()
                .unwrap_or_default();
            data::WASHER_SOUND_PATTERNS[kind].1
        }
        "home/a-fixing-and-its-drill-bit" => fixing(reading),
        "home/furniture-upstairs" => return Some(Cow::Owned(upstairs(reading))),
        "home/machine-consumables" => return Some(consumable(reading)),
        _ => return super::extra::reply(id, reading).map(Cow::Borrowed),
    };
    Some(Cow::Borrowed(reply))
}

fn chat_about_tap(reading: &Reading<'_>, turns: &[Exchange]) -> bool {
    let said = if reading.line.text.is_empty() {
        super::clean(reading)
    } else {
        &reading.line.text
    };
    let said = said.trim().to_lowercase();
    for prior in turns
        .iter()
        .rev()
        .map(|turn| turn.user.trim().to_lowercase())
        .filter(|text| !text.is_empty() && text != &said)
        .take(2)
    {
        if prior
            .split(crate::text::is_space)
            .any(|word| super::data::MACHINE_WORDS.contains(&word))
        {
            return false;
        }
        if TAP_TALK.is_match(&prior) {
            return true;
        }
    }
    false
}

fn fixing(reading: &Reading<'_>) -> &'static str {
    let said = reading.line.clean.to_lowercase();
    let shelf = super::words_include(reading, &["shelf", "shelves", "bookcase", "bookcase shelf"])
        || said.contains("a shelf");
    if super::contains_any(&said, &["plug", "drill bit", "masonry bit", "drill"]) {
        return data::PLUG_ANSWER;
    }
    if shelf {
        return data::SHELF_ANSWER;
    }
    if super::words_include(
        reading,
        &[
            "nut", "nuts", "bolt", "bolts", "thread", "threads", "spanner", "spanners",
        ],
    ) {
        return data::NUT_ANSWER;
    }
    data::PLUG_ANSWER
}

fn upstairs(reading: &Reading<'_>) -> String {
    let said = format!(" {} ", reading.line.clean.to_lowercase());
    let (kind, first) = data::UPSTAIRS_ITEMS
        .iter()
        .find_map(|&(words, first)| {
            words
                .iter()
                .copied()
                .filter(|word| begins_word(&said, word))
                .reduce(|best, word| if word.len() > best.len() { word } else { best })
                .map(|word| (word, first))
        })
        .unwrap_or(("furniture", data::UPSTAIRS_ITEMS.last().unwrap().1));
    format!(
        "Measure first: the stairwell's width and the tightest height under the turn, then \
          the narrowest side of the {kind}. {first}. Two people and a strap, not \
          one: a man with a van is 40 to 80 pounds and cheaper than a surgeon. If the numbers \
          say no, a two-piece or flat-pack version that comes up in boxes is the honest \
          answer."
    )
}

fn begins_word(said: &str, word: &str) -> bool {
    said.match_indices(word)
        .any(|(offset, _)| said[..offset].ends_with(' '))
}

fn consumable(reading: &Reading<'_>) -> Cow<'static, str> {
    let said = if reading.line.plain.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.plain
    };
    let dishwasher = super::DISHWASHER.is_match(said);
    let reply = if dishwasher
        || (!super::WASHER_MACHINE.is_match(said)
            && super::contains_any(said, &["rinse aid", "rinse-aid"]))
    {
        data::DISHWASHER_COMPARTMENTS
    } else {
        data::WASHER_DRAWER
    };
    let said = said.to_lowercase();
    let mut best_score = 0;
    let mut best = 0;
    for (index, sentence) in reply.split_inclusive('.').enumerate() {
        let here = sentence.to_lowercase();
        let score = data::CONSUMABLE_WORDS
            .iter()
            .filter(|word| said.contains(**word) && here.contains(**word))
            .count();
        if score > best_score {
            best = index;
            best_score = score;
        }
    }
    if best == 0 {
        return Cow::Borrowed(reply);
    }
    let sentences: Vec<_> = reply.split_inclusive('.').map(str::trim).collect();
    let reordered = std::iter::once(sentences[best])
        .chain(
            sentences
                .iter()
                .enumerate()
                .filter_map(|(index, &sentence)| (index != best).then_some(sentence)),
        )
        .collect::<Vec<_>>()
        .join(" ");
    Cow::Owned(reordered)
}

#[cfg(test)]
#[path = "../../../tests/fixtures/home_replies.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn household_reply_variants_match_pybot() {
        let mut mismatches = Vec::new();
        for &fixtures::Case(id, prompt, history, expected) in fixtures::CASES {
            let turns: Vec<_> = history
                .iter()
                .map(|&text| Exchange {
                    user: text.into(),
                    bot: String::new(),
                    module: "",
                })
                .collect();
            let actual = registered(id, &Reading::new(prompt), &turns);
            if actual.as_deref() != Some(expected) {
                mismatches.push((id, prompt, history, expected, actual));
            }
        }
        assert!(mismatches.is_empty(), "{mismatches:#?}");
    }
}
