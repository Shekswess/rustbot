//! Approximate weights of common household and travel objects.

use fancy_regex::Regex as Fancy;
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
include!("units_object_data.rs");

pattern!(WEIGHT, r"\b(?:weigh|weighs|weight|heavy|heavier)\b");
pattern!(TOO_HEAVY, r"\btoo\s+(?:heavy|much)\b");
pattern!(
    WEIGHT_ASK,
    r"\bhow\s+(?:much|heavy|many|big)\b|\bwhat(?:'s| is)?\s+(?:the\s+)?weight\b"
);
pattern!(
    SHOP_OR_ALLOWANCE,
    r"\b(?:cost|costs|price|prices|worth|sell|selling|buy|buying|allowance|allowed|limit|limits|maximum|max|can)\b"
);

static COUNTED: LazyLock<Fancy> = LazyLock::new(|| {
    Fancy::new(
        r"(?<![\w.])(?:\d+|a|an|one|two|three|four|five|six|my|your|our|his|her|their|the)\s+$",
    )
    .expect("counted object")
});
static NAMES: LazyLock<Vec<[Fancy; 2]>> = LazyLock::new(|| {
    OBJECTS
        .iter()
        .map(|&(name, plural, _)| {
            [name, plural].map(|words| {
                Fancy::new(&format!(
                    r"(?<![\w.]){}(?![\w])(?=\s*(?:$|[,?]|weigh|weighs|heavy|about|is\b))",
                    regex::escape(words)
                ))
                .expect("object mention")
            })
        })
        .collect()
});

pub(super) fn answer(clean: &str) -> Option<String> {
    if !WEIGHT.is_match(clean)
        || TOO_HEAVY.is_match(clean)
        || !WEIGHT_ASK.is_match(clean)
        || SHOP_OR_ALLOWANCE.is_match(clean)
    {
        return None;
    }
    let mut best = None::<(&str, u32)>;
    for (&(name, _, grams), forms) in OBJECTS.iter().zip(NAMES.iter()) {
        if best.is_some_and(|(earlier, _)| earlier.len() >= name.len()) {
            continue;
        }
        if forms.iter().any(|form| {
            form.find(clean)
                .ok()
                .flatten()
                .is_some_and(|hit| COUNTED.is_match(&clean[..hit.start()]).unwrap_or(false))
        }) {
            best = Some((name, grams));
        }
    }
    let (name, grams) = best?;
    let article = if name.starts_with(['a', 'e', 'i', 'o', 'u']) {
        "An"
    } else {
        "A"
    };
    let weight = if grams >= 1000 {
        format!("{} kg", super::say(f64::from(grams) / 1000.))
    } else {
        format!("{} g", super::say(f64::from(grams)))
    };
    Some(format!("{article} {name} is about {weight}."))
}

#[cfg(test)]
mod tests {
    mod reference {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/unit_objects.rs"
        ));
    }

    #[test]
    fn object_weights_match_the_reference() {
        let differences: Vec<_> = reference::CASES
            .iter()
            .filter_map(|&(text, expected)| {
                let actual = super::answer(text);
                (actual.as_deref() != expected).then_some((text, expected, actual))
            })
            .collect();
        assert!(
            differences.is_empty(),
            "{} differences: {:?}",
            differences.len(),
            &differences[..differences.len().min(12)]
        );
    }
}
