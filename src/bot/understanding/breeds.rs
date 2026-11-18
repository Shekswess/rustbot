//! Species names supplied by the breed reader in bot/understanding/breeds.py.
const BREEDS: &[(&str, &str)] = &[
    ("afghan hound", "dog"),
    ("akita", "dog"),
    ("alsatian", "dog"),
    ("basset hound", "dog"),
    ("beagle", "dog"),
    ("border collie", "dog"),
    ("british shorthair", "cat"),
    ("bull terrier", "dog"),
    ("bulldog", "dog"),
    ("cavapoo", "dog"),
    ("chihuahua", "dog"),
    ("cockapoo", "dog"),
    ("cocker spaniel", "dog"),
    ("collie", "dog"),
    ("corgi", "dog"),
    ("dachshund", "dog"),
    ("dalmatian", "dog"),
    ("doberman", "dog"),
    ("dobermann", "dog"),
    ("german shepherd", "dog"),
    ("golden retriever", "dog"),
    ("greyhound", "dog"),
    ("husky", "dog"),
    ("jack russell", "dog"),
    ("labradoodle", "dog"),
    ("labrador", "dog"),
    ("labrador retriever", "dog"),
    ("lhasa apso", "dog"),
    ("lurcher", "dog"),
    ("maine coon", "cat"),
    ("mastiff", "dog"),
    ("moggy", "cat"),
    ("mongrel", "dog"),
    ("papillon", "dog"),
    ("pomeranian", "dog"),
    ("poodle", "dog"),
    ("pug", "dog"),
    ("ragdoll", "cat"),
    ("retriever", "dog"),
    ("rottweiler", "dog"),
    ("samoyed", "dog"),
    ("schnauzer", "dog"),
    ("scottish fold", "cat"),
    ("shar pei", "dog"),
    ("shih tzu", "dog"),
    ("siamese", "cat"),
    ("spaniel", "dog"),
    ("sphynx", "cat"),
    ("staffie", "dog"),
    ("staffordshire bull terrier", "dog"),
    ("tabby", "cat"),
    ("terrier", "dog"),
    ("tortoiseshell", "cat"),
    ("vizsla", "dog"),
    ("weimaraner", "dog"),
    ("west highland terrier", "dog"),
    ("westie", "dog"),
    ("whippet", "dog"),
    ("yorkshire terrier", "dog"),
];
const MONEY: &[&str] = &[
    "bet", "bets", "betting", "grand", "money", "odds", "punter", "quid", "stake", "stakes",
    "tenner", "tenners", "winnings",
];
const LOOSE_FRAME: &[&str] = &["a", "an", "that", "the", "these", "this", "those"];
const NOT_A_CREATURE_NEXT: &[&str] = &[
    "breeder",
    "breeders",
    "bus",
    "buses",
    "club",
    "coach",
    "coaches",
    "curtains",
    "depot",
    "depots",
    "derby",
    "frames",
    "glasses",
    "kennel",
    "kennels",
    "line",
    "lines",
    "people",
    "person",
    "race",
    "races",
    "racing",
    "rescue",
    "rug",
    "rugs",
    "service",
    "services",
    "sofa",
    "sofas",
    "stadium",
    "station",
    "stations",
    "sunglasses",
    "terminal",
    "terminals",
    "ticket",
    "tickets",
    "timetable",
    "timetables",
    "track",
    "twin",
    "twins",
    "upholstery",
];
const FRAME_BEFORE: &[&str] = &[
    "a", "adopt", "adopted", "an", "another", "bought", "buy", "buys", "get", "gets", "got", "had",
    "has", "have", "having", "her", "his", "keep", "keeps", "kept", "kept", "my", "our", "rehome",
    "rehomed", "rescue", "rescued", "take", "takes", "that", "the", "their", "these", "this",
    "those", "took",
];

pub(super) fn exact_kind(name: &str) -> Option<&'static str> {
    BREEDS
        .binary_search_by_key(&name, |entry| entry.0)
        .ok()
        .map(|i| BREEDS[i].1)
}
fn kind(name: &str) -> Option<&'static str> {
    use exact_kind as lookup;
    lookup(name)
        .or_else(|| {
            name.strip_suffix("ies")
                .and_then(|stem| lookup(&format!("{stem}y")))
        })
        .or_else(|| name.strip_suffix("es").and_then(lookup))
        .or_else(|| name.strip_suffix('s').and_then(lookup))
}

fn span(words: &[String]) -> Option<(usize, &'static str)> {
    // Try complete names before their component breeds, as in “labrador retriever”.
    for width in (2..=4.min(words.len())).rev() {
        if let Some(animal) = kind(&words[..width].join(" ")) {
            return Some((width, animal));
        }
    }
    kind(&words[0]).map(|animal| (1, animal))
}

/// Add the species immediately after a framed breed. Keep racing, bus and betting senses intact.
pub(crate) fn expand(mut words: Vec<String>) -> Vec<String> {
    let mut inserts = Vec::new();
    let mut at: usize = 0;
    while at < words.len() {
        let before = &words[at.saturating_sub(4)..at];
        let mut frames = before
            .iter()
            .map(String::as_str)
            .filter(|w| FRAME_BEFORE.binary_search(w).is_ok())
            .peekable();
        if frames.peek().is_some()
            && let Some((width, kind)) = span(&words[at..])
        {
            let other = words
                .get(at + width)
                .is_some_and(|word| NOT_A_CREATURE_NEXT.binary_search(&word.as_str()).is_ok());
            let bet = frames.all(|frame| LOOSE_FRAME.binary_search(&frame).is_ok())
                && words
                    .iter()
                    .any(|w| MONEY.binary_search(&w.as_str()).is_ok());
            if !(other || bet) {
                at += width;
                inserts.push((at, kind));
                continue;
            }
        }
        at += 1;
    }
    for (at, kind) in inserts.into_iter().rev() {
        words.insert(at, kind.into());
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frame_owns_the_breed_and_the_next_word_can_override_it() {
        for (line, want) in [
            ("my greyhounds are hungry", "my greyhounds dog are hungry"),
            (
                "we got a jack russell and a maine coon",
                "we got a jack russell dog and a maine coon cat",
            ),
            ("im husky", "im husky"),
            ("my greyhound bus is late", "my greyhound bus is late"),
            (
                "i lost money on the greyhounds",
                "i lost money on the greyhounds",
            ),
        ] {
            assert_eq!(
                expand(line.split_whitespace().map(str::to_owned).collect()).join(" "),
                want
            );
        }
    }
}
