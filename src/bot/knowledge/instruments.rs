//! Learning an instrument, including named practice and history around the instrument.

use std::borrow::Cow;

use super::guards::Context;
use super::{
    INSTRUMENT_DRUMS, INSTRUMENT_GENERIC, INSTRUMENT_LOOKBACK, INSTRUMENT_OTHERS,
    INSTRUMENT_TABLES, INSTRUMENT_TOO_LATE,
};

pub(super) struct Table {
    pub(super) names: &'static [&'static str],
    pub(super) hint: &'static str,
    pub(super) start: &'static str,
    pub(super) practise: &'static str,
    pub(super) how_long: &'static str,
    pub(super) how_often: &'static str,
}

#[derive(Clone, Copy)]
pub(super) enum Ask {
    Start,
    Practise,
    HowLong,
    HowOften,
}

fn includes(text: &str, words: &[&str]) -> bool {
    words.iter().any(|word| text.contains(word))
}

fn drums(context: &Context<'_>, current: &str) -> bool {
    if includes(current, INSTRUMENT_DRUMS) {
        return true;
    }
    if includes(current, INSTRUMENT_OTHERS) {
        return false;
    }
    for turn in context.earlier.iter().rev().take(INSTRUMENT_LOOKBACK) {
        let user = turn.user.to_lowercase();
        let bot = turn.bot.to_lowercase();
        if includes(&bot, &["practice pad", "basic beat"]) || includes(&user, INSTRUMENT_DRUMS) {
            return true;
        }
        if includes(&user, INSTRUMENT_OTHERS) {
            return false;
        }
    }
    false
}

fn named(current: &str) -> Option<&'static Table> {
    INSTRUMENT_TABLES
        .iter()
        .find(|table| includes(current, table.names))
}

fn table(context: &Context<'_>, current: &str) -> &'static Table {
    if let Some(table) = named(current) {
        return table;
    }
    if includes(current, INSTRUMENT_OTHERS) {
        return &INSTRUMENT_GENERIC;
    }
    for turn in context.earlier.iter().rev().take(INSTRUMENT_LOOKBACK) {
        let bot = turn.bot.to_lowercase();
        if let Some(table) = INSTRUMENT_TABLES
            .iter()
            .find(|table| bot.contains(table.hint))
        {
            return table;
        }
        if let Some(table) = named(&turn.user.to_lowercase()) {
            return table;
        }
    }
    &INSTRUMENT_GENERIC
}

fn in_play(context: &Context<'_>, current: &str) -> bool {
    let mentions_instrument = |text: &str| {
        includes(text, INSTRUMENT_DRUMS)
            || includes(text, INSTRUMENT_OTHERS)
            || named(text).is_some()
    };
    mentions_instrument(current)
        || context
            .earlier
            .iter()
            .rev()
            .take(INSTRUMENT_LOOKBACK)
            .any(|turn| mentions_instrument(&format!("{} {}", turn.user, turn.bot).to_lowercase()))
}

fn already_talking_about_it(context: &Context<'_>) -> bool {
    context.earlier.last().is_some_and(|turn| {
        includes(
            &turn.bot.to_lowercase(),
            &["15–20 minutes", "learning an instrument", "four chords"],
        )
    })
}

fn drums_reply(ask: Ask, already: bool, doubt: bool) -> Cow<'static, str> {
    const START: &str = "Ten minutes a day on a practice pad gets you a steady basic beat in about six weeks: single strokes slow and even, then hi-hat on the eighths, snare on two and four, kick on one and three. Play along to songs you like before you buy a kit, and take one lesson at the start so your grip is right — a pad, or a cushion and a pair of sticks, is enough to begin.";
    match ask {
        Ask::Start if already => Cow::Borrowed(
            "Then start this week: pad and sticks, ten minutes a day, one lesson at the start, and one song you like to play along to. Speed comes last — slow and even beats fast and ragged every time.",
        ),
        Ask::Start if doubt => Cow::Owned(format!("Not too late at all. {START}")),
        Ask::Start => Cow::Borrowed(START),
        Ask::Practise => Cow::Borrowed(
            "Single strokes first, slow and even, ten minutes a day on a practice pad or a cushion: full strokes up to about a foot, then back down. Get that steady, then a basic beat — hi-hat on the eighths, snare on two and four, kick on one and three — and play along to a song you like. Speed comes last.",
        ),
        Ask::HowLong => Cow::Borrowed(
            "About six weeks of ten minutes a day for a steady basic beat, and three or four months before it holds together with a song at full speed. Play along to songs you like before you buy a kit.",
        ),
        Ask::HowOften => Cow::Borrowed(
            "Ten minutes a day, five or six days a week, on a practice pad: single strokes slow and even first, then the basic beat played along to a song. The hands want the day off more than they want the extra twenty minutes.",
        ),
    }
}

fn capitalize(reply: &'static str) -> Cow<'static, str> {
    let mut chars = reply.chars();
    match chars.next() {
        Some(first) if first.is_lowercase() => {
            Cow::Owned(format!("{}{}", first.to_uppercase(), chars.as_str()))
        }
        _ => Cow::Borrowed(reply),
    }
}

pub(super) fn answer(ask: Ask, context: &Context<'_>) -> Option<Cow<'static, str>> {
    let words = &context.reading.line.words;
    let say = if words.is_empty() {
        context.reading.raw.to_lowercase()
    } else {
        words.join(" ").to_lowercase()
    };
    let current = format!(" {say} ");
    let doubt = includes(&current, INSTRUMENT_TOO_LATE);
    let already = already_talking_about_it(context);
    if drums(context, &current) {
        return Some(drums_reply(ask, already, doubt));
    }
    let table = table(context, &current);
    match ask {
        Ask::Start if doubt && !already => Some(Cow::Owned(format!(
            "Not too late at all: adults pick up the theory faster than children, they just need shorter and more regular practice. {}",
            capitalize(table.start),
        ))),
        Ask::Start => Some(capitalize(table.start)),
        Ask::Practise
            if std::ptr::eq(table, &INSTRUMENT_GENERIC) && !in_play(context, &current) =>
        {
            None
        }
        Ask::Practise => Some(Cow::Borrowed(table.practise)),
        Ask::HowLong => Some(Cow::Borrowed(table.how_long)),
        Ask::HowOften => Some(Cow::Borrowed(table.how_often)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::{Exchange, Reading};

    #[test]
    fn practice_followup_uses_the_recent_instrument_and_yields_when_none_is_named() {
        let reading = Reading::new("What should I practise first?");
        let earlier = [Exchange {
            user: "I'm learning the drums.".to_owned(),
            bot: "Try a practice pad.".to_owned(),
            module: "answers",
        }];
        assert!(
            answer(Ask::Practise, &Context::new(&reading, &earlier))
                .is_some_and(|reply| reply.starts_with("Single strokes first")),
        );
        assert_eq!(answer(Ask::Practise, &Context::new(&reading, &[])), None);
    }
}
