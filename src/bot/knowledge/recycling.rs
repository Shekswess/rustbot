//! Item-specific branches of the household bin card, in source order.

use std::borrow::Cow;

use crate::bot::Reading;

pub(super) fn which_bin(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let source = &reading.line;
    let line = if source.text.is_empty() {
        &source.clean
    } else {
        &source.text
    };
    let line = line.to_lowercase().replace('\u{2019}', "'");
    let reply = super::RECYCLING_ITEM_ROUTES
        .iter()
        .find(|(items, _)| items.iter().any(|item| super::has_space_delimited_phrase(&line, item)))
        .map_or(
            "Which item is it? Electricals and batteries generally need separate recycling. Ask your council where it accepts household chemicals, and check its bulky waste collection for large items.",
            |(_, reply)| *reply,
        );
    Some(Cow::Borrowed(reply))
}
