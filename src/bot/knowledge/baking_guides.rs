//! Native reply selection for baking techniques and simple cooking guides.

use crate::bot::{Reading, understanding::line::Line};

#[path = "baking_guides_data.rs"]
mod data;

pub(crate) fn registered(id: &str, reading: &Reading<'_>) -> Option<&'static str> {
    let line = &reading.line;
    let reply = match id {
        "loaf-tin" => tin(line),
        "cake-fault" => data::CAKE_FAULTS
            .iter()
            .find(|(cues, _)| in_reading(line, cues))
            .map_or(data::CAKE_DEFAULT, |(_, reply)| *reply),
        "pastry-fault" => {
            if in_reading(line, data::PASTRY_SHRINK) {
                data::PASTRY_SHRUNK
            } else {
                data::PASTRY_DEFAULT
            }
        }
        "bread-rise" => {
            let joined;
            let said = if line.clean.is_empty() {
                joined = line.words.join(" ");
                &joined
            } else {
                &line.clean
            };
            if contains_any(said, data::NO_YEAST) {
                return None;
            }
            data::PROVING
                .iter()
                .find(|(cues, _)| contains_any(said, cues))
                .map_or(data::BREAD_DEFAULT, |(_, reply)| *reply)
        }
        "flatbreads" => flatbread(&line.clean)?,
        "pasta-cook" => {
            if line
                .words
                .iter()
                .any(|word| data::CLUMP.contains(&word.as_str()))
            {
                data::PASTA_CLUMP
            } else {
                data::PASTA_DEFAULT
            }
        }
        "soup-flavour" => {
            const FLAVOUR: &[&str] = &[
                "better",
                "flavour",
                "improve",
                "boring",
                "lifeless",
                "interesting",
            ];
            if line
                .words
                .iter()
                .any(|word| FLAVOUR.iter().any(|cue| word.starts_with(cue)))
            {
                data::SOUP_DULL
            } else {
                data::SOUP_DEFAULT
            }
        }
        _ => return None,
    };
    Some(reply)
}

fn contains_any(text: &str, cues: &[&str]) -> bool {
    cues.iter().any(|cue| text.contains(cue))
}

fn in_reading(line: &Line, cues: &[&str]) -> bool {
    contains_any(&line.clean, cues)
        || contains_any(&line.norm_text, cues)
        || line.norm.iter().any(|word| contains_any(word, cues))
}

fn tin(line: &Line) -> &'static str {
    if in_reading(line, data::TIN_FRUIT_WORDS) {
        data::TIN_FRUIT
    } else if in_reading(line, data::TIN_LINE_WORDS) {
        data::TIN_LINE
    } else if in_reading(line, data::TIN_GREASE_WORDS) {
        data::TIN_GREASE
    } else {
        data::TIN_DEFAULT
    }
}

fn flatbread(said: &str) -> Option<&'static str> {
    if contains_any(said, &["spanish", "espanola", "espanol", "omelette"]) {
        return None;
    }
    let trimmed = said.trim_matches(crate::text::is_space);
    let bytes = said.as_bytes();
    let names = |name: &str| {
        said.match_indices(name).any(|(start, found)| {
            let before = start == 0 || bytes[start - 1] == b' ';
            let after = bytes
                .get(start + found.len())
                .is_none_or(|&byte| byte == b' ');
            before && after
        }) || trimmed
            .strip_suffix(name)
            .is_some_and(|before| before.ends_with(' '))
    };
    Some(
        data::FLATBREADS
            .iter()
            .find(|(words, _)| words.iter().any(|word| names(word)))
            .map_or(data::FLATBREAD_DEFAULT, |(_, reply)| *reply),
    )
}

#[cfg(test)]
#[path = "../../../tests/fixtures/baking_guides.rs"]
mod fixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_baking_and_cooking_branches_are_exact() {
        for &(id, raw, expected) in fixtures::CASES {
            assert_eq!(registered(id, &Reading::new(raw)), expected, "{id}: {raw}");
        }
    }
}
