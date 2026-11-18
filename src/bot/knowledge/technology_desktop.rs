//! Desktop settings, connection and file recovery cards from the pinned technology domain.
#[path = "technology_desktop_data.rs"]
mod data;
#[cfg(test)]
#[path = "technology_desktop_tests.rs"]
mod tests;

use crate::bot::Reading;
use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

#[derive(Clone, Copy, Debug)]
pub(super) enum Reply {
    EmptyTrash,
    ForceQuit,
    DefaultBrowser,
    BatteryHealth,
    Uninstall,
    ToTv,
    Symbol,
    Recover,
    Definition,
}

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}
patterns!(
    PLATFORM_MAC,
    SCREEN_NAMED_RE,
    NOWHERE_TO_KEEP_STILL,
    IN_A_FOLDER,
    A_CARD,
    A_DRIVE
);

fn clean_or_text<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.clean.is_empty() {
        &reading.line.text
    } else {
        &reading.line.clean
    }
}

fn machine_steps(steps: data::PlatformSteps, reading: &Reading<'_>) -> Cow<'static, str> {
    let text = if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    };
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let (first, second) = if PLATFORM_MAC.is_match(&text) {
        (steps.mac, steps.windows)
    } else {
        (steps.windows, steps.mac)
    };
    Cow::Owned(format!("{first} {second}{}", steps.tail))
}

pub(super) fn computer_bin(reading: &Reading<'_>) -> bool {
    let words = &reading.line.words;
    !words
        .iter()
        .any(|word| matches!(word.as_str(), "bin" | "bins"))
        || words.iter().any(|word| {
            data::MACHINE_BIN_WORDS
                .binary_search(&word.as_str())
                .is_ok()
        })
        || reading.topics.iter().any(|(name, _)| *name == "technology")
}

pub(super) fn screen_named(reading: &Reading<'_>) -> bool {
    [
        reading.line.text.as_str(),
        reading.line.norm_text.as_str(),
        reading.line.clean.as_str(),
        reading.raw,
    ]
    .iter()
    .any(|line| SCREEN_NAMED_RE.is_match(line))
}

fn to_tv(reading: &Reading<'_>) -> &'static str {
    let clean = reading.line.clean.to_lowercase();
    if ["phone", "iphone", "android", "tablet", "ipad"]
        .iter()
        .any(|word| clean.contains(word))
    {
        data::TV_CAST
    } else {
        data::TV_CABLE
    }
}

fn typed_symbol(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let clean = clean_or_text(reading).to_lowercase();
    let (_, spelling) = data::SYMBOLS
        .iter()
        .find(|(names, _)| names.iter().any(|name| clean.contains(name)))?;
    Some(Cow::Owned(format!(
        "On a UK keyboard {spelling}. If a symbol comes out as a different one, the keyboard is set to the US layout: Settings > Time & language > Language on Windows, System Settings > Keyboard > Input Sources on a Mac."
    )))
}

fn recover(reading: &Reading<'_>) -> Cow<'static, str> {
    let clean = clean_or_text(reading).to_lowercase();
    if !NOWHERE_TO_KEEP_STILL.is_match(&clean) || IN_A_FOLDER.is_match(&clean) {
        return Cow::Borrowed(data::RECOVER_IN_A_BIN);
    }
    let item = if A_CARD.is_match(&clean) {
        "card"
    } else if A_DRIVE.is_match(&clean) {
        "drive"
    } else {
        "stick"
    };
    Cow::Owned(data::RECOVER_ON_A_DRIVE.replace("{thing}", item))
}

fn definition(reading: &Reading<'_>) -> Option<&'static str> {
    let has = |word: &str| {
        reading.line.clean.contains(word) || reading.line.words.iter().any(|seen| seen == word)
    };
    data::TECH_WORDS
        .iter()
        .find(|item| item.keys.iter().any(|key| has(key)) && !item.avoid.iter().any(|cue| has(cue)))
        .map(|item| item.reply)
}

pub(super) fn reply(form: Reply, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let answer = match form {
        Reply::EmptyTrash => machine_steps(data::EMPTY_TRASH, reading),
        Reply::ForceQuit => machine_steps(data::FORCE_QUIT, reading),
        Reply::DefaultBrowser => machine_steps(data::DEFAULT_BROWSER, reading),
        Reply::BatteryHealth => machine_steps(data::BATTERY_HEALTH, reading),
        Reply::Uninstall => machine_steps(data::UNINSTALL_PROGRAM, reading),
        Reply::ToTv => Cow::Borrowed(to_tv(reading)),
        Reply::Symbol => return typed_symbol(reading),
        Reply::Recover => recover(reading),
        Reply::Definition => Cow::Borrowed(definition(reading)?),
    };
    Some(answer)
}
