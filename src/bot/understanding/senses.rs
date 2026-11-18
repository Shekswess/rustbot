//! Resolve word senses using co-occurring words in the whole current line.
#[path = "senses_data.rs"]
mod data;
use super::line::Line;
use std::collections::{BTreeMap, HashSet};
struct Sense {
    name: &'static str,
    forms: &'static [&'static str],
}
struct Ambiguity {
    word: &'static str,
    forms: &'static [&'static str],
    senses: &'static [Sense],
}

pub(crate) fn read(line: &Line) -> BTreeMap<&'static str, &'static str> {
    let mut words = HashSet::new();
    for word in line
        .norm
        .iter()
        .map(String::as_str)
        .chain(line.clean.split_whitespace())
    {
        words.insert(word);
        let len = word.chars().count();
        if len > 3
            && !word.ends_with("ss")
            && let Some(stem) = word.strip_suffix('s')
        {
            words.insert(stem);
        }
        if len > 5
            && let Some(stem) = word.strip_suffix("es")
        {
            words.insert(stem);
        }
    }
    let mut result = BTreeMap::new();
    for row in data::CUES {
        if !row.forms.iter().any(|w| words.contains(w)) {
            continue;
        }
        let mut count = 0;
        let mut winner = None;
        for sense in row.senses {
            let score = sense.forms.iter().filter(|w| words.contains(**w)).count();
            if score > count {
                count = score;
                winner = Some(sense.name);
            } else if score == count {
                winner = None;
            }
        }
        if count == 0
            && row.word == "degrees"
            && line.norm.windows(2).any(|p| {
                let before = p[0].trim_matches(['.', ',', '!', '?', ';', ':']);
                row.forms.contains(&p[1].as_str())
                    && (data::FIGURES.binary_search(&before).is_ok()
                        || !before.is_empty() && before.chars().all(|c| c.is_ascii_digit()))
            })
        {
            winner = Some("temperature")
        }
        if let Some(sense) = winner {
            result.insert(row.word, sense);
        }
    }
    result
}
