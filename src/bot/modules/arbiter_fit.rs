//! Topic evidence used to resolve competing proposals.
use crate::bot::Reading;
use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

#[path = "arbiter_topics.rs"]
mod data;

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z0-9]+(?:'[a-z]+)?").unwrap());
static LETTERS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z]+").unwrap());
static POINTED: LazyLock<Regex> = LazyLock::new(|| Regex::new(data::POINTS_AT_THREAD).unwrap());

// The pinned arbiter excludes sentence glue when deciding whether a reply
// names the subject the person actually brought up.
const NOT_A_SUBJECT_WORD: &str = concat!(
    "the and but for with you your yours they them their his her hers she him our ours us its it ",
    "that this these those was were been are am is be does did done have has had can cant cannot ",
    "will would should could may might must not nor off out down too very really much many some ",
    "any all more most just only then there here now how what when where why who which about than ",
    "into onto over under again still even ever never get got gets one two put say says said think ",
    "know like want need make made take took going went gone come came keep keeps kept let lets"
);

fn subject_word(word: &str) -> Option<Cow<'_, str>> {
    if word.len() < 3
        || NOT_A_SUBJECT_WORD
            .split_ascii_whitespace()
            .any(|skip| skip == word)
    {
        return None;
    }
    for ending in ["ies", "es", "s"] {
        if let Some(stem) = word.strip_suffix(ending).filter(|stem| stem.len() >= 3) {
            return Some(if ending == "ies" {
                Cow::Owned(format!("{stem}y"))
            } else {
                Cow::Borrowed(stem)
            });
        }
    }
    Some(Cow::Borrowed(word))
}

fn shared_opening(first: &str, second: &str) -> bool {
    first.starts_with(second) || second.starts_with(first)
}

fn names_subject(reply: &str, words: &[String]) -> bool {
    let plain = reply.to_ascii_lowercase();
    let said: Vec<_> = WORD.find_iter(&plain).map(|word| word.as_str()).collect();
    let subjects: Vec<_> = said.iter().filter_map(|word| subject_word(word)).collect();
    if subjects.is_empty() {
        return false;
    }
    words.iter().any(|word| match subject_word(word) {
        Some(stem) => subjects.iter().any(|one| {
            one == &stem || (one.len() >= 4 && stem.len() >= 4 && shared_opening(one, &stem))
        }),
        None => word.len() >= 4 && said.iter().any(|one| shared_opening(one, word)),
    })
}

fn only_repeats_line(reply: &str, words: &[String]) -> bool {
    let plain = reply.to_ascii_lowercase();
    let said = LETTERS
        .find_iter(&plain)
        .map(|word| word.as_str())
        .filter(|word| word.len() >= 4);
    let mut count = 0;
    let mut fresh = 0;
    for word in said {
        count += 1;
        if count > 8 {
            return false;
        }
        if !words
            .iter()
            .any(|one| one.len() >= 4 && shared_opening(word, one))
        {
            fresh += 1;
        }
    }
    count != 0 && fresh * 2 <= count
}

fn label(text: &str) -> String {
    WORD.find_iter(&text.to_lowercase())
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn points_at_thread(text: &str) -> bool {
    let clean = text
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    clean.split_whitespace().count() <= 12 && POINTED.is_match(&clean)
}

pub(super) struct Fit {
    own: Vec<String>,
    clean: String,
    words: Vec<String>,
}

impl Fit {
    pub(super) fn new(reading: &Reading<'_>) -> Self {
        let own = reading
            .topics
            .iter()
            .chain(reading.topic.iter())
            .map(|(t, _)| label(t))
            .collect();
        let clean = format!(" {} ", label(reading.raw));
        let words = reading
            .line
            .words
            .iter()
            .flat_map(|w| {
                label(w)
                    .split_whitespace()
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .collect();
        Self { own, clean, words }
    }

    pub(super) fn carried<'a>(values: impl Iterator<Item = &'a str>) -> Vec<String> {
        values.map(label).collect()
    }

    pub(super) fn owns(&self, module: &str) -> bool {
        data::answered(module)
            .iter()
            .any(|t| self.own.iter().any(|own| own == t))
    }

    pub(super) fn preserves_subject(&self, first: &str, next: &str) -> bool {
        names_subject(first, &self.words)
            && !only_repeats_line(first, &self.words)
            && !names_subject(next, &self.words)
    }

    pub(super) fn score(&self, module: &str, carried: &[String]) -> usize {
        data::answered(module)
            .iter()
            .map(|label| {
                if self.own.iter().any(|v| v == label) {
                    2
                } else {
                    usize::from(
                        carried.iter().any(|v| v == label)
                            || self.clean.contains(&format!(" {label} "))
                            || self.words.iter().any(|v| v == label),
                    )
                }
            })
            .sum()
    }
}
