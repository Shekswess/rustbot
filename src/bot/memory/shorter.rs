//! Requests for a shorter answer, after PyBot's memory/repeat.shorter_for.
use super::measurement::sentences;
use crate::bot::{
    Memory, Reading, compose,
    understanding::normalize::{stem, tokens},
};
use regex::Regex;
use std::sync::LazyLock;

static BREVITY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(?:short(?:er)?|brief(?:ly|er)?|concisely|concise|tldr|tl;dr|summari[sz]e|sum (?:it )?up|the gist|less detail|skip the detail|one (?:line|sentence)|no jargon|layman's terms|simpl(?:e|er|y|ify|ified)|plain (?:english|terms|words)|in english|like (?:i'm|im|i am) (?:5|five)|cut (?:it|that) down|get to the point)\b"
).unwrap()
});
static KEEP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"(?:[ ,]+but)?[ ,]+(?:keep(?: in)?|leave in|don't (?:leave out|drop|lose)|without (?:losing|dropping)) (?P<what>.+?)[.?!]*$"
).unwrap()
});
static FIGURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(?:timings?|times?|numbers?|figures?|ratios?|amounts?|quantit(?:y|ies)|temperatures?|degrees?|weights?|measures?|proportions?)\b"
).unwrap()
});
static QUOTE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"“[^”]+”|"[^"]+""#).unwrap());

const LESS_LEADS: &[&str] = &[
    "{short}",
    "Shorter: {short}",
    "In short: {short}",
    "Briefly: {short}",
];
const JUST_THIS: &[&str] = &["Just this: {short}", "The one thing in it: {short}"];
const ALREADY_SHORT: &[&str] = &[
    "That's about as short as it gets: {short}",
    "That is the short version: {short}",
];
const NOTHING_TO_CUT: &[&str] = &[
    "Nothing to cut down yet - ask me something and I'll keep it short.",
    "Nothing in hand to shorten - ask away and I'll keep it brief.",
];
const NO_SHORTER: &[&str] = &[
    "That is the short version - there is nothing left in it to take out.",
    "Nothing left to cut: every part of that answer you have now had on its own.",
];

pub(super) fn reads_it(line: &str) -> bool {
    let head = KEEP
        .find(line)
        .map_or(line, |clause| &line[..clause.start()]);
    if !BREVITY.is_match(head) || head.starts_with("how ") {
        return false;
    }
    const WORDS: &str = "can could would you please now then just make made it that keep be say put give me a bit lot really much nice short shorter brief briefer briefly quick quicker snappy concise concisely tldr tl dr summarise summarize sum up gist answer version cut down chase get to the point skip detail details less in sentence line one and so okay ok right again too long word words more fewer condensed simple simpler simply simplify simplified plain english terms jargon free no layman layman's explain plainly like im i'm i am five 5 dumb dumbed easy easier wordy";
    tokens(head)
        .iter()
        .all(|word| WORDS.split_ascii_whitespace().any(|known| known == word))
}

fn already_shortened(short: &str, memory: &Memory) -> bool {
    let body = tokens(short).join(" ");
    memory.turns.iter().rev().take(5).any(|turn| {
        reads_it(&turn.user.to_lowercase()) && tokens(&turn.bot).join(" ").contains(&body)
    })
}

fn condensed(said: &str, memory: &Memory) -> Option<String> {
    sentences(said)
        .into_iter()
        .filter_map(|part| {
            let part = part.trim().trim_matches([',', ';', ':']);
            let words = part.split_whitespace().count();
            ((4..=45).contains(&words) && !already_shortened(part, memory)).then_some((words, part))
        })
        .min_by_key(|(words, _)| *words)
        .map(|(_, part)| part.to_owned())
}

fn say(template: &[&str], short: &str, reading: &Reading<'_>, memory: &Memory) -> String {
    compose::pick(template, memory, reading.raw).replace("{short}", short)
}

fn labelled(line: &str) -> bool {
    line.split_once(':').is_some_and(|(label, _)| {
        label.chars().count() <= 45
            && label.split_whitespace().count() <= 5
            && !label.contains(['.', '!', '?'])
    })
}

fn one_sentence(said: &str) -> Option<&str> {
    let parts = sentences(said);
    if parts.len() > 1 {
        let first = parts[0];
        return Some(if first.split_whitespace().count() < 8 {
            parts[1]
        } else {
            first
        });
    }
    let (head, _) = said.split_once(':')?;
    (head.split_whitespace().count() >= 8 && !labelled(said)).then_some(head)
}

fn keep_clause(said: &str, keep: &str) -> Option<String> {
    let numeric = FIGURE.is_match(keep);
    let key: Vec<_> = tokens(keep)
        .iter()
        .filter(|word| word.len() >= 4)
        .map(|word| stem(word).to_owned())
        .collect();
    sentences(said)
        .into_iter()
        .find(|line| {
            if numeric && line.chars().any(|ch| ch.is_ascii_digit()) {
                return true;
            }
            let words = tokens(line);
            !key.is_empty()
                && key
                    .iter()
                    .any(|key| words.iter().any(|word| stem(word) == key))
        })
        .map(str::to_owned)
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if !reads_it(&reading.clean) {
        return None;
    }
    let keep = KEEP
        .captures(&reading.clean)
        .and_then(|found| found.name("what"))
        .map(|part| part.as_str());
    let mut first = None;
    for turn in memory
        .turns
        .iter()
        .rev()
        .take(12)
        .take_while(|turn| !matches!(turn.module, "safety" | "urgent" | "grief" | "loss"))
        .filter(|turn| {
            let said = turn.bot.trim();
            (!matches!(turn.module, "greeting" | "first_line_open") || QUOTE.is_match(said))
                && said.split_whitespace().count() >= 6
                && !said.ends_with('?')
                && !["For you, that means:", "In your case:"]
                    .iter()
                    .any(|lead| said.starts_with(lead))
                && !reads_it(&turn.user.to_lowercase())
        })
        .take(2)
    {
        let said = turn.bot.trim();
        first.get_or_insert(said);
        if let Some(keep) = keep
            && let Some(short) = keep_clause(said, keep)
        {
            let templates = [
                format!("Keeping {keep}: {short}"),
                format!("With {keep} in: {short}"),
            ];
            return Some(
                compose::seeded_choice(
                    &templates.each_ref().map(String::as_str),
                    memory,
                    reading.raw,
                )
                .into(),
            );
        }
        if let Some(quote) = QUOTE.find(said)
            && quote.as_str() != said
            && !already_shortened(quote.as_str(), memory)
        {
            return Some(say(JUST_THIS, quote.as_str(), reading, memory));
        }
        if let Some(lead) = one_sentence(said)
            && !already_shortened(lead, memory)
        {
            return Some(if labelled(lead) {
                lead.into()
            } else {
                say(LESS_LEADS, lead, reading, memory)
            });
        }
        if let Some(short) = condensed(said, memory)
            && short != said
        {
            return Some(say(JUST_THIS, &short, reading, memory));
        }
    }
    let Some(said) = first else {
        return Some(compose::pick(NOTHING_TO_CUT, memory, reading.raw).into());
    };
    if let Some(short) = condensed(said, memory) {
        return Some(say(ALREADY_SHORT, &short, reading, memory));
    }
    Some(compose::pick(NO_SHORTER, memory, reading.raw).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fewer_words_requests_do_not_intercept_questions_on_their_own_subject() {
        assert!(reads_it("could you put that in simple terms?"));
        assert!(reads_it("shorter but keep the timings"));
        assert!(!reads_it("how do i make my essay shorter?"));
    }
}
