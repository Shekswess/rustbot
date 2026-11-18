//! Fixed-weight definitions and common unit-pair facts.

use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
include!("units_facts_data.rs");

static TAIL_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    TAILS
        .iter()
        .map(|source| Regex::new(source).expect("unit fact tail"))
        .collect()
});

fn alias(word: &str) -> Option<&'static str> {
    UNIT_ALIASES
        .binary_search_by_key(&word, |&(name, _)| name)
        .ok()
        .map(|i| UNIT_ALIASES[i].1)
}

fn pair(a: &str, b: &str) -> Option<&'static str> {
    let (a, b) = (alias(a)?, alias(b)?);
    FACTS.iter().find_map(|&(from, to, reply)| {
        ((a == from && b == to) || (a == to && b == from)).then_some(reply)
    })
}

pub(super) fn answer(clean: &str) -> Option<&'static str> {
    let plain = clean
        .trim()
        .trim_matches(['?', '.', '!'])
        .replace([',', ';'], " ");
    let plain = POLITE.replace(&plain, "");
    let plain = FILLER.replace_all(&plain, " ");
    let text = plain
        .split(crate::text::is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if let Some(heavy) = HEAVY.captures(&text) {
        return WEIGHT
            .binary_search_by_key(&&heavy[1], |&(name, _)| name)
            .ok()
            .map(|i| WEIGHT[i].1);
    }
    if let Some(much) = HOW_MUCH.captures(&text) {
        return pair(&much[1], &much[2]);
    }
    let amount = AMOUNT.captures(&text)?;
    for pattern in TAIL_PATTERNS.iter() {
        if let Some(tail) = pattern.captures(&amount[2])
            && alias(&tail[1]).is_some()
        {
            return pair(&amount[1], &tail[1]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    mod reference {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/unit_facts.rs"
        ));
    }

    #[test]
    fn facts_match_the_reference() {
        let differences: Vec<_> = reference::CASES
            .iter()
            .filter_map(|&(text, expected)| {
                let actual = super::answer(text);
                (actual != expected).then_some((text, expected, actual))
            })
            .collect();
        assert!(differences.is_empty(), "{differences:?}");
    }
}
