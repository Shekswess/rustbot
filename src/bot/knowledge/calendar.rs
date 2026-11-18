//! The timing gate shared by the pinned shopping calendar cards.

use std::{borrow::Cow, sync::LazyLock};

use crate::bot::Reading;

use super::{
    CALENDAR_COMPARING, CALENDAR_FAILED, CALENDAR_NEXT, CALENDAR_NOT_THE_ASK,
    CALENDAR_PERIOD_NOUNS, CALENDAR_PREPOSITIONS, CALENDAR_TIME_ASK, CALENDAR_WHEN_ASKING,
    SHOP_COMPARED_THINGS, SHOP_PART_WORDS, SHOP_PLACE_PHRASES, SHOP_RECEIPT_LENGTH, one_edit,
};

static RECEIPT_LENGTH: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(SHOP_RECEIPT_LENGTH).expect("record keeping and tax"));

pub(super) fn return_without_length(reading: &Reading<'_>) -> bool {
    [&reading.line.text, &reading.line.clean]
        .into_iter()
        .all(|text| !RECEIPT_LENGTH.is_match(&text.to_lowercase()))
}

pub(super) fn price_comparison(reading: &Reading<'_>) -> bool {
    let words = reading.line.words.join(" ");
    let where_asked = reading.features.asks.contains(&"where")
        || SHOP_PLACE_PHRASES
            .iter()
            .any(|phrase| words.contains(phrase));
    !where_asked || asks_shop_place(reading)
}

pub(super) fn asks_shop_place(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    let text = words.join(" ");
    let where_label = reading.line.norm.iter().any(|word| word == "where");
    (where_label
        || SHOP_PLACE_PHRASES
            .iter()
            .any(|phrase| text.contains(phrase)))
        && !words
            .iter()
            .any(|word| SHOP_PART_WORDS.contains(&word.as_str()))
        && SHOP_COMPARED_THINGS
            .iter()
            .any(|thing| text.contains(thing))
}

fn says_it(text: &str, name: &str) -> bool {
    let names = std::iter::once(name.to_owned())
        .chain([format!("{name}s"), format!("{name}es")])
        .chain(name.strip_suffix('s').map(str::to_owned));
    if names.into_iter().any(|name| {
        text.match_indices(&name).any(|(start, matched)| {
            !text[..start].ends_with(|c: char| c.is_ascii_lowercase())
                && !text[start + matched.len()..].starts_with(|c: char| c.is_ascii_lowercase())
        })
    }) {
        return true;
    }
    !name.contains(' ')
        && name.chars().count() >= 6
        && text.split_whitespace().any(|word| {
            let word = word.trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"', '(', ')']);
            word.chars().count() >= 5 && one_edit(word, name)
        })
}

fn compared_period(line: &str) -> bool {
    let words = line.split_whitespace().collect::<Vec<_>>();
    (words.iter().any(|word| CALENDAR_COMPARING.contains(word))
        || ["half price", "price drop", "prices drop"]
            .into_iter()
            .any(|phrase| line.contains(phrase)))
        && words.iter().enumerate().any(|(at, word)| {
            CALENDAR_PERIOD_NOUNS.contains(word)
                && words[at.saturating_sub(3)..at]
                    .iter()
                    .any(|word| CALENDAR_PREPOSITIONS.contains(word))
        })
}

pub(super) fn asks_when(reading: &Reading<'_>, subjects: &[&str]) -> bool {
    let mut text = calendar_line(reading);
    if !subjects.is_empty() && !subjects.iter().any(|name| says_it(&text, name)) {
        return false;
    }
    for name in subjects {
        text = text
            .replace(&format!(" {name} "), " ")
            .replace(&format!(" {name}s "), " ");
    }
    let timed_ask = CALENDAR_TIME_ASK.iter().any(|phrase| {
        text.contains(&format!(" {phrase} ")) || text.contains(&format!(" {phrase}s "))
    });
    if !timed_ask && !compared_period(&text) {
        return false;
    }
    let mentions = |choices: &[&str]| choices.iter().any(|phrase| text.contains(phrase));
    let asks_when = !text.contains("when")
        || CALENDAR_TIME_ASK
            .iter()
            .filter(|&&ask| ask != "when")
            .any(|ask| text.contains(ask))
        || mentions(CALENDAR_WHEN_ASKING);
    asks_when
        && (!mentions(CALENDAR_FAILED) || mentions(CALENDAR_NEXT))
        && !mentions(CALENDAR_NOT_THE_ASK)
}

fn calendar_line(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    if !line.clean.is_empty() {
        format!(" {} ", line.clean)
    } else if !line.norm_text.is_empty() {
        format!(" {} ", line.norm_text)
    } else {
        format!(" {} ", line.words.join(" "))
    }
}

pub(super) fn general_sales(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    asks_when(reading, &[]).then_some(Cow::Borrowed(super::SALES_CALENDAR))
}

pub(super) fn christmas(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    if !asks_when(reading, &[]) {
        return None;
    }
    let text = calendar_line(reading);
    let contains = |phrases: &[&str]| phrases.iter().any(|phrase| text.contains(phrase));
    if contains(&[
        " take ",
        " taken ",
        " taking ",
        " put ",
        " putting ",
        " pack ",
        " packing ",
        " throw ",
        " go up ",
        " goes up ",
        " going up ",
        " gone up ",
        " come down ",
        " comes down ",
        " coming down ",
    ]) && contains(&[" down ", " up ", " away ", " out "])
    {
        return None;
    }
    let tree = contains(&["tree", "trees", "real tree", "fir"]);
    let lights = contains(&[
        "light",
        "lights",
        "fairy",
        "decorations",
        "baubles",
        "tinsel",
        "ornaments",
    ]);
    let reply = match (tree, lights) {
        (true, false) => {
            "Buy a real tree in mid-December, a week or two before Christmas: a tree cut in November drops its needles by the big day, so ask the grower when it was cut and keep it in water. Artificial trees are cheapest in the January sales, so buy the one you want now and store it."
        }
        (false, true) => {
            "Christmas lights and decorations are cheapest in the first week of January, at 70 to 90 per cent off, so buy for next year on the 2nd or 3rd rather than in November, when the same set is full price."
        }
        _ => {
            "Buy a real tree in mid-December, a week or two before Christmas, from a grower who will tell you when it was cut; lights and decorations are cheapest in the first week of January, at 70 to 90 per cent off, for next year."
        }
    };
    Some(Cow::Borrowed(reply))
}

#[cfg(test)]
#[test]
fn shopping_questions_keep_their_authored_guards_and_reviewed_guidance() {
    use super::{guards::Context, rows};
    for (id, prompt, prefix, detail) in [
        (
            "bag-charge",
            "Why do supermarkets charge for plastic carrier bags?",
            Some("Shops in England have to charge at least 10p"),
            "retailers choose whether",
        ),
        (
            "bag-charge",
            "Why does bread go mouldy in a plastic bag?",
            None,
            "",
        ),
        (
            "where-to-buy-cheapest",
            "Where can I buy a cheap tumble dryer?",
            Some("Compare the exact model"),
            "over £100 and up to £30,000",
        ),
        (
            "where-to-buy-cheapest",
            "Where can I buy a Hoover filter?",
            None,
            "",
        ),
        (
            "return-without-receipt",
            "Can I return a kettle with no receipt?",
            Some("You do not need the paper slip."),
            "a bank or card statement",
        ),
        (
            "return-without-receipt",
            "How long do I need to keep receipts for tax?",
            None,
            "",
        ),
    ] {
        let reading = Reading::new(prompt);
        let context = Context::new(&reading, &[]);
        let row = rows()
            .find(|row| row.domain == "shopping" && row.id == id)
            .unwrap();
        let allowed = row
            .guard
            .passes(&context, id, row.key.first().copied().unwrap_or_default());
        let actual = allowed.then(|| row.answer(&context)).flatten();
        if let Some(prefix) = prefix {
            let actual = actual.unwrap_or_else(|| panic!("declined {id}: {prompt}"));
            assert!(
                actual.starts_with(prefix) && actual.contains(detail),
                "{prompt}: {actual}"
            );
        } else {
            assert!(actual.is_none(), "{prompt}: {actual:?}");
        }
    }
}
