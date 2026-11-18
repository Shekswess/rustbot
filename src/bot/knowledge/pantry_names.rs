//! Ordered pantry alias matching for the pinned recipe tables.

use crate::bot::Reading;

use super::{
    PANTRY_CLAUSE_BREAK, PANTRY_EVENT_HEADS, PANTRY_INGREDIENT_ALIASES, PANTRY_NEGATIONS,
    PANTRY_NOT_FOOD, PANTRY_REFUSAL_JOINERS, PANTRY_RULING_OUT, PANTRY_SOFTENED, food_fold::fold,
};

pub(super) fn ingredients(reading: &Reading<'_>, own_words: &[String]) -> Vec<&'static str> {
    let repaired: Vec<_> = reading.line.plain.split_whitespace().map(fold).collect();
    let words = if repaired.len() == own_words.len() && repaired != own_words {
        &repaired
    } else {
        own_words
    };
    let mut used = vec![false; words.len()];
    let mut hits = Vec::new();
    for &(parts, name) in PANTRY_INGREDIENT_ALIASES {
        if !words.iter().any(|word| word == parts[0]) {
            continue;
        }
        let Some(start) = words
            .windows(parts.len())
            .enumerate()
            .find_map(|(start, words)| {
                let free = used[start..start + parts.len()].iter().all(|used| !used);
                (free && words.iter().map(String::as_str).eq(parts.iter().copied()))
                    .then_some(start)
            })
        else {
            continue;
        };
        let end = start + parts.len();
        used[start..end].fill(true);
        hits.push((start, name, end));
    }

    hits.sort_unstable();
    let mut named = Vec::new();
    let mut kept = Vec::new();
    let mut refused_ends = Vec::new();
    for (start, name, end) in hits {
        if named.contains(&name)
            || start > 0 && PANTRY_EVENT_HEADS.contains(&words[start - 1].as_str())
        {
            continue;
        }
        named.push(name);
        if ruled_out(words, start)
            || refused_ends.iter().any(|&end| {
                end < start
                    && words[end..start]
                        .iter()
                        .all(|word: &String| PANTRY_REFUSAL_JOINERS.contains(&word.as_str()))
            })
        {
            refused_ends.push(end);
        } else {
            kept.push(name);
        }
    }
    let clean = &reading.line.clean;
    kept.iter()
        .copied()
        .filter(|&name| {
            !kept
                .iter()
                .any(|&other| name != other && other.split_whitespace().any(|word| name == word))
                && !PANTRY_NOT_FOOD.iter().any(|(food, phrases)| {
                    *food == name && phrases.iter().any(|phrase| clean.contains(phrase))
                })
        })
        .collect()
}

fn ruled_out(words: &[String], start: usize) -> bool {
    let before = &words[start.saturating_sub(3)..start];
    if before
        .iter()
        .any(|word| PANTRY_NEGATIONS.contains(&word.as_str()))
        && !before
            .iter()
            .any(|word| PANTRY_SOFTENED.contains(&word.as_str()))
    {
        return true;
    }
    let earlier = &words[start.saturating_sub(6)..start];
    let offset = earlier
        .iter()
        .rposition(|word| PANTRY_CLAUSE_BREAK.contains(&word.as_str()))
        .map_or(0, |i| i + 1);
    let plain = format!(" {} ", earlier[offset..].join(" "));
    PANTRY_RULING_OUT
        .iter()
        .any(|cue| plain.contains(&format!(" {cue} ")))
}
