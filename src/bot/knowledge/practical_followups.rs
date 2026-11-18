//! Phone use and damp-appliance follow-ups from earlier user turns.

use std::{borrow::Cow, sync::LazyLock};

use regex::{Regex, RegexSet};

use crate::bot::{Exchange, Reading};

macro_rules! phone_pattern {
    ($name:ident, $pattern:ident) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(super::$pattern).expect(stringify!($pattern)));
    };
}
phone_pattern!(PHOTOS, PHONE_USE_PHOTOS);
phone_pattern!(CALLS, PHONE_USE_CALLS);
phone_pattern!(PHONE, PHONE_USE_PHONE);
phone_pattern!(HEAVY, PHONE_USE_HEAVY);
phone_pattern!(OTHER_QUESTION, PHONE_USE_OTHER);
phone_pattern!(BUY, PHONE_USE_BUYING);
phone_pattern!(WITHOUT_MANNER, KIT_WITHOUT_A_MANNER);

static DOOR_FAULT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)cant|cannot|can not|won'?t|will not|does not|doesn'?t|hard to|stuck|swings?|",
        r"swinging|on its own|itself|hanging|off|lock|keyhole|slam|bang|squeak|squeal|",
        r"rub|paint|draught|draft|dragging|dropped|measure|door is not|door isnt|",
        r"door isn'?t|door is broken|door is stiff|door is loose|door is stuck|",
        r"door is jamming|door was broken|door was stiff|door was loose"
    ))
    .expect("machine door fault")
});

fn line<'a>(reading: &'a Reading<'_>) -> &'a str {
    if reading.line.text.is_empty() {
        &reading.line.clean
    } else {
        &reading.line.text
    }
}

pub(super) fn phone_use_ask(reading: &Reading<'_>) -> bool {
    let text = line(reading);
    !text.trim().is_empty() && !OTHER_QUESTION.is_match(text)
}

fn phone_use(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    let text = line(reading);
    let before = earlier.last().map_or("", |turn| turn.user.as_str());
    if !(PHONE.is_match(text) || PHONE.is_match(before))
        || !(BUY.is_match(text) || BUY.is_match(before))
    {
        return None;
    }
    let reply = match (
        PHOTOS.is_match(text),
        CALLS.is_match(text),
        HEAVY.is_match(text),
    ) {
        (true, _, false) => {
            "For photos and video, compare real camera samples and storage. Check how much space your pictures and videos already use, and leave room for new ones. Compare the cost of more built-in storage with any optional cloud backup."
        }
        (false, true, false) => {
            "For calls and messages, look for a phone with years of security support remaining, good battery life and a comfortable screen or keypad. Compare new and refurbished models, including warranty and remaining update support."
        }
        (false, _, true) => {
            "For games or work, check the requirements of the apps you use. Compare performance, display, battery life and storage on models that meet them. With a refurbished phone, check battery condition, remaining security support and the warranty."
        }
        (true, _, true) => {
            "For photos and demanding apps, check camera samples and compare performance with your actual apps. Allow enough room for both video and downloads. Compare the full cost of buying outright plus a plan with the total cost of a handset contract."
        }
        _ => return None,
    };
    Some(reply)
}

fn damp_machine(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<&'static str> {
    if DOOR_FAULT.is_match(line(reading)) {
        return None;
    }
    let mut machine = false;
    let mut smelly = false;
    let mut dishwasher = false;
    for turn in earlier.iter().rev().take(5) {
        let said = turn.user.to_lowercase();
        machine |= super::DAMP_MACHINE_NAMES
            .iter()
            .any(|word| said.contains(word));
        smelly |= super::DAMP_MACHINE_SMELLS
            .iter()
            .any(|word| said.contains(word));
        dishwasher |= said.contains("dishwasher");
    }
    if !machine || !smelly {
        return None;
    }
    Some(if dishwasher {
        "Closing the dishwasher immediately can trap moisture. Follow your dishwasher's instructions for ventilation after the cycle and for cleaning its seal and filter. Keep children and pets away from an open door. Use the recommended cleaning cycle and product if the smell persists."
    } else {
        "Moisture left inside a closed washer can cause a musty smell. Follow the manual for airing the drum and drawer between cycles, and keep children and pets away when the door is open. Wipe around the seal and run the model's recommended maintenance wash if needed."
    })
}

fn contains(text: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| text.contains(term))
}

pub(super) fn build_job_asked(reading: &Reading<'_>) -> bool {
    let text = if reading.line.words.is_empty() {
        line(reading).to_lowercase()
    } else {
        reading.line.words.join(" ").to_lowercase()
    };
    !contains(&text, super::BUILD_JOB_OTHER_ASK)
        && (contains(&text, super::BUILD_JOB_TIME_ASK)
            || contains(&text, super::BUILD_JOB_TOOLS_ASK))
        && super::BUILD_JOBS
            .iter()
            .any(|(names, ..)| contains(&text, names))
}

fn opener(text: &str) -> &str {
    text.split(',')
        .next()
        .unwrap_or(text)
        .split(" - ")
        .next()
        .unwrap_or(text)
        .trim()
}

fn build_job(reading: &Reading<'_>, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    let (_, time, tools, caveat) = super::BUILD_JOBS.iter().find(|(jobs, ..)| {
        jobs.iter()
            .any(|job| text.contains(job) || reading.line.words.iter().any(|word| word == job))
    })?;
    let wants_tools =
        !contains(text, super::BUILD_JOB_TIME_ASK) && contains(text, super::BUILD_JOB_TOOLS_ASK);
    let (mut first, spare) = if wants_tools {
        (tools, time)
    } else {
        (time, tools)
    };
    if let Some(last) = earlier.last().map(|turn| turn.bot.to_lowercase())
        && last.contains(&opener(first).to_lowercase())
    {
        if last.contains(&opener(spare).to_lowercase()) {
            return None;
        }
        first = spare;
    }
    Some(Cow::Owned(format!("{first} {caveat}")))
}

static WATER_SUBJECTS: LazyLock<RegexSet> = LazyLock::new(|| {
    super::material_care::word_patterns(
        &super::WATER_OUT_REPLIES
            .iter()
            .map(|(names, _)| *names)
            .collect::<Vec<_>>(),
    )
});

pub(super) fn water_out(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = &reading.line.clean;
    let text = if text.is_empty() { line(reading) } else { text };
    let reply = WATER_SUBJECTS.matches(text).iter().next().map_or(
        "Use the drain point specified by the maker. If there is no fixed drain and the contents are safe to handle, use a suitable pump or bail into a bucket. Use a drain that can take the volume and keep water away from electrical fittings.",
        |index| super::WATER_OUT_REPLIES[index].1,
    );
    Some(Cow::Borrowed(reply))
}

pub(super) fn tool_is_missing(reading: &Reading<'_>) -> bool {
    let mut manners = WITHOUT_MANNER.captures_iter(&reading.line.clean).peekable();
    manners.peek().is_none()
        || manners.any(|matched| super::KIT_MISSING_VERBS.contains(&&matched[1]))
}

pub(super) fn no_kit(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let mut best = None;
    for &(aliases, reply) in super::KIT_SUBSTITUTES {
        for &name in aliases {
            let named = if name.contains(' ') {
                reading.line.clean.contains(name)
            } else {
                reading.line.words.iter().any(|word| word == name)
            };
            if named && best.is_none_or(|(length, _)| name.len() > length) {
                best = Some((name.len(), reply));
            }
        }
    }
    best.map(|(_, reply)| Cow::Borrowed(reply))
}

pub(super) fn reply(
    domain: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let reply = match domain {
        "phone_use" => phone_use(reading, earlier),
        "machine_door" => damp_machine(reading, earlier),
        "build_jobs" => return build_job(reading, earlier),
        _ => None,
    }?;
    Some(Cow::Borrowed(reply))
}
