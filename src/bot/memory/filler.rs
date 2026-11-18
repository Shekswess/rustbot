//! Trailing conversational filler, from bot/memory/filler.py.
const TAIL_WORDS: &[&str] = &[
    "tho", "though", "lol", "lmao", "haha", "ha", "hehe", "tbh", "ngl", "imo", "idk", "innit", "x",
    "xx", "xxx", "actually", "btw",
];
const TAIL_PHRASES: &[&str] = &[
    "by the way",
    "out of interest",
    "while im at it",
    "whilst im at it",
    "as it happens",
    "if thats ok",
    "if that's ok",
    "if you dont mind",
    "if you don't mind",
    "come to think of it",
    "now i think of it",
    "now that i think of it",
];

pub(super) fn without_filler(said: &str) -> String {
    let mut words: Vec<_> = said
        .split(crate::text::is_space)
        .filter(|word| !word.is_empty())
        .collect();
    while words.len() > 1
        && words.last().is_some_and(|word| {
            TAIL_WORDS.contains(
                &word
                    .trim_matches(['.', ',', ';', ':', '!', '?'])
                    .to_ascii_lowercase()
                    .as_str(),
            )
        })
    {
        words.pop();
    }
    let text = words.join(" ");
    let mut rest = text.as_str();
    loop {
        let stripped = rest.trim_end_matches([' ', '.', ',', ';', ':', '!', '?']);
        let lower = stripped.to_ascii_lowercase();
        let Some(cut) = TAIL_PHRASES.iter().find_map(|phrase| {
            let prefix = lower.strip_suffix(phrase)?;
            (prefix.ends_with(' ')).then_some(prefix.len())
        }) else {
            break;
        };
        let next = stripped[..cut].trim_end_matches([' ', ',', ';', ':', '-']);
        if next.is_empty() {
            break;
        }
        rest = next;
    }
    rest.to_owned()
}
