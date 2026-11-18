//! Choose the opening sentence for a fruit from the pinned keeping table.

use super::{
    RIPEN_CHAT_REPLIES, RIPEN_KEEP_ASKS, RIPEN_TELL_ASKS, RIPEN_TRICK_ASKS, guards::Context,
};

pub(super) fn answer(
    context: &Context<'_>,
    test_first: &'static str,
    trick_first: &'static str,
    keep: &'static str,
) -> &'static str {
    let text = &context.reading.line.clean;
    let has = |words: &[&str]| words.iter().any(|word| text.contains(word));
    if has(RIPEN_TELL_ASKS) {
        test_first
    } else if has(RIPEN_TRICK_ASKS) {
        trick_first
    } else if has(RIPEN_KEEP_ASKS) {
        keep
    } else {
        test_first
    }
}

pub(super) fn from_chat(context: &Context<'_>) -> &'static str {
    let earlier = &context.earlier[context.earlier.len().saturating_sub(3)..];
    let text = std::iter::once(context.reading.line.clean.as_str())
        .chain(earlier.iter().map(|turn| turn.user.as_str()))
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase();
    for (aliases, reply) in RIPEN_CHAT_REPLIES {
        if aliases.iter().any(|alias| names(&text, alias)) {
            return reply;
        }
    }
    "Most fruit ripens on the counter in 2 to 4 days, and a paper bag with a banana in it halves that; the fridge is what stops it, so leave them out until they give. Once they give, they keep a few days in the fridge."
}

fn names(line: &str, name: &str) -> bool {
    let in_word = |c: char| c.is_alphanumeric() || c == '_';
    line.match_indices(name).any(|(start, matched)| {
        !line[..start].ends_with(in_word) && !line[start + matched.len()..].starts_with(in_word)
    })
}
