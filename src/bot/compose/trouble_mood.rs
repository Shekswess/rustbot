//! Tone of a report in the composer, used to decide which trouble question fits.

use super::{has, includes, matches};
use fancy_regex::Captures;
use std::borrow::Cow;
use std::sync::LazyLock;

#[path = "trouble_mood_data.rs"]
mod data;
use data::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Mood {
    Positive,
    Negative,
    Neutral,
}

fn no_pleasure(line: &str) -> bool {
    !line.is_empty()
        && ![
            &A_COMPARISON,
            &NOTHING_THAT_LIMITS,
            &PLEASURE_ABOUT_ONE_THING,
        ]
        .iter()
        .any(|pattern| matches(pattern, line))
        && [
            &FEELING_NOTHING,
            &PLEASURE_GONE_OUT,
            &NOTHING_AND_A_PLEASURE,
            &NOTHING_NEGATED,
        ]
        .iter()
        .any(|pattern| matches(pattern, line))
}

fn harm(line: &str) -> bool {
    !line.is_empty()
        && !matches(&A_HARM_IDIOM, line)
        && (matches(&A_PART_OUT, line)
            || matches(&A_PROGNOSIS, line) && !matches(&A_PROGNOSIS_THAT_IS_NOT, line)
            || matches(&A_HURT_DONE, line) && matches(&A_BODY_WORD, line)
            || matches(&A_HURT_BY_SOMEONE, line) && !matches(&A_HURT_BY_SOMEONE_NOT, line)
            || matches(&A_HARM_ILL, line) && !matches(&A_HARM_ILL_NOT, line)
            || [
                &A_HARM_HAD,
                &A_PART_WENT,
                &A_HARM_KNOCK,
                &A_OUT_OF_SOCKET,
                &A_IN_AGONY,
            ]
            .iter()
            .any(|pattern| matches(pattern, line)))
}

fn illness(line: &str) -> bool {
    let words: Vec<_> = super::split(line)
        .map(|word| word.trim_matches(['.', ',', '!', '?', ';', ':', '\'', '"']))
        .collect();
    words.iter().enumerate().any(|(i, word)| {
        let before = i.checked_sub(1).map_or("", |j| words[j]);
        let next = words.get(i + 1).copied();
        *word == "ill"
            && (has(ILL_SAID_OF_A_BODY, before)
                || !next.is_some_and(|next| has(ILL_WILL_VERBS, next))
                    && (has(ILL_BEFORE, before) || next.is_none_or(|next| has(ILL_AFTER, next))))
    })
}

fn bare_ailment(line: &str) -> bool {
    if line.trim().is_empty() || line.contains('?') || matches(&BARE_WORD_ONLY, line) {
        return false;
    }
    let words: Vec<_> = super::words(line).take(6).collect();
    !words.is_empty()
        && words.len() <= 5
        && (matches(&BARE_ACHE, line)
            || matches(&BARE_ILL, line)
            || matches(&BARE_BODY, line) && matches(&BARE_HURTS, line))
        && words.iter().all(|word| {
            has(BARE_FILLER, word)
                || [
                    &BARE_ACHE_ONLY,
                    &BARE_ILL_ONLY,
                    &BARE_BODY_ONLY,
                    &BARE_HURTS_ONLY,
                ]
                .iter()
                .any(|pattern| matches(pattern, word))
        })
}

pub(crate) fn without_sick_figure(line: &str) -> Cow<'_, str> {
    let original = FEELING_BEFORE_SICK.replace_all(line, |parts: &Captures<'_>| {
        super::split(&parts[0])
            .next()
            .unwrap_or_default()
            .to_owned()
    });
    let changed = SICK_AS_FEELING.replace_all(&original, |parts: &Captures<'_>| {
        let word = super::flat(&parts[1]);
        FEELING_WORD
            .binary_search_by_key(&word.as_str(), |(key, _)| key)
            .map_or("upset", |index| FEELING_WORD[index].1)
    });
    match changed {
        Cow::Owned(line) => Cow::Owned(line),
        Cow::Borrowed(_) => original,
    }
}

pub(crate) fn reads_like_symptom(line: &str) -> bool {
    let line = without_sick_figure(line);
    SYMPTOM.iter().any(|pattern| matches(pattern, &line)) || bare_ailment(&line)
}

fn an_aim(line: &str, word: &str) -> bool {
    if [" but ", " though ", " although "]
        .iter()
        .any(|contrast| line.contains(contrast))
    {
        return false;
    }
    static WORD_CHAR: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"\w").unwrap());
    let is_word = |c: char| WORD_CHAR.is_match(c.encode_utf8(&mut [0; 4]));
    let mut found = line
        .match_indices(word)
        .filter(|(start, cue)| {
            !line[..*start].chars().next_back().is_some_and(&is_word)
                && !line[start + cue.len()..]
                    .chars()
                    .next()
                    .is_some_and(&is_word)
        })
        .peekable();
    fn trim(word: &str) -> &str {
        word.trim_matches(['.', ',', '!', '?'])
    }
    found.peek().is_some()
        && found.all(|(start, cue)| {
            let before = &line[..start];
            matches(&AIM_BEFORE, before)
                && !super::split(before)
                    .rev()
                    .take(3)
                    .any(|word| has(AIM_NEGATORS, trim(word)))
                && !super::split(&line[start + cue.len()..])
                    .any(|word| has(AIM_FAILING, trim(word)))
        })
}

fn describes(words: &[&str], cue: &str) -> bool {
    words.iter().enumerate().any(|(i, word)| {
        if *word != cue {
            return false;
        }
        let prev = i.checked_sub(1).map_or("", |j| words[j]);
        let next = words.get(i + 1).copied().unwrap_or_default();
        if !has(COPULA, prev) {
            return has(MOOD_BEFORE, prev) || has(MOOD_AFTER, next);
        }
        let subject = &words[..i.saturating_sub(1)];
        let last = subject.last().copied().unwrap_or_default();
        subject.is_empty()
            || has(ABOUT_THEM_BEFORE_COPULA, last)
            || includes(subject, STRETCH_OF_TIME)
            || subject.len() == 1 && matches!(last, "it" | "this" | "today")
    })
}

pub(in crate::bot::compose) fn turns_round(before: &[&str]) -> bool {
    let letters: Vec<_> = before
        .iter()
        .map(|word| word.replace(['\'', '’'], ""))
        .collect();
    let turned = letters.iter().enumerate().any(|(i, word)| {
        has(TURN_NEGATORS, word)
            || word
                .strip_suffix("nt")
                .is_some_and(|base| word.len() > 3 && has(CONTRACTED_AUX, base))
            || before[i] == "t" && i > 0 && has(AUX, before[i - 1])
    });
    turned
        && !(letters.iter().any(|word| has(DOUBT, word))
            && letters.iter().any(|word| has(CANNOT, word)))
}

fn cue_negated(words: &[&str], cue: &str) -> bool {
    words.iter().enumerate().any(|(i, word)| {
        let before = &words[i.saturating_sub(3)..i];
        *word == cue
            && (includes(&words[i.saturating_sub(2)..i], NEGATORS)
                || includes(before, DOUBTED)
                || includes(before, DOUBT) && turns_round(before))
    })
}

fn stops(after: &[&str]) -> bool {
    let mut after = after
        .iter()
        .take(4)
        .skip_while(|word| has(THE_WORDS_BETWEEN, word));
    let Some(&first) = after.next() else {
        return false;
    };
    has(THINGS_THAT_STOP, first)
        || has(THINGS_THAT_STOP_AFTER_UP, first)
            && after
                .next()
                .is_some_and(|next| matches!(*next, "up" | "in" | "out"))
            && after.next().is_none_or(|next| {
                matches!(*next, "on" | "it" | "them" | "me" | "him" | "her" | "us")
            })
}

fn no_such_thing(words: &[&str], index: usize) -> bool {
    words[..index]
        .iter()
        .enumerate()
        .rev()
        .take(8)
        .take_while(|(_, word)| !has(CONTRAST, word))
        .any(|(i, word)| {
            has(NOTHING_AT_ALL, word) || *word == "one" && i > 0 && words[i - 1] == "no"
        })
}

fn crowd_win(words: &[&str], i: usize) -> bool {
    if words.get(i + 1).is_some_and(|word| has(ABOUT_WORDS, word))
        && words.get(i + 2).is_some_and(|word| has(PERSON_WORDS, word))
    {
        return false;
    }
    for (j, word) in words[..i].iter().enumerate().rev().take(6) {
        if has(YOU_SUBJECTS, word) {
            return false;
        }
        if has(CROWD, word)
            || has(PEOPLE, word)
                && word.ends_with('s')
                && words[j.saturating_sub(3)..=j].contains(&"all")
        {
            return true;
        }
    }
    false
}

pub(super) fn read(words: &[&str], clean: &str) -> Mood {
    let plain = super::flat(clean);
    if super::taste::only_dislike(&plain)
        || matches(&NOTHING_THAT_LIMITS, &format!(" {plain} "))
        || matches(&NO_LAUGH_OR_SCARES, &format!(" {plain} "))
    {
        return Mood::Neutral;
    }
    if no_pleasure(&plain)
        || words.contains(&"ill") && illness(clean)
        || harm(&plain) && !reads_like_symptom(clean)
    {
        return Mood::Negative;
    }
    let mut good = false;
    for (i, &word) in words.iter().enumerate() {
        let positive = has(POSITIVE_CUES, word);
        let junk = has(JUNK_CUES, word);
        let break_word = has(BREAK_WORDS, word);
        if !(positive || has(NEGATIVE_CUES, word) || junk || break_word) || an_aim(clean, word) {
            continue;
        }
        if junk && describes(words, word) {
            return Mood::Negative;
        }
        if break_word && matches(&A_BREAK_THAT_IS_GOOD, clean) {
            good = true;
            continue;
        }
        if !positive && !has(NEGATIVE_CUES, word) {
            continue;
        }
        let before = &words[i.saturating_sub(3)..i];
        let after = &words[i + 1..];
        if let Ok(index) = MOOD_WITH_A_CATCH.binary_search_by_key(&word, |(cue, _)| *cue) {
            if after.starts_with(&["up"]) {
                good = true;
                continue;
            }
            if !includes(before, MOOD_WITH_A_CATCH[index].1)
                && !after
                    .first()
                    .is_some_and(|word| matches!(*word, "me" | "him" | "her" | "them" | "us"))
            {
                continue;
            }
        }
        if includes(before, NEARLY) || has(PAST_STATE_CUES, word) && includes(before, WAS) {
            continue;
        }
        let negated = cue_negated(words, word);
        if positive {
            if stops(after) {
                continue;
            }
            if negated || no_such_thing(words, i) {
                if has(STATE_CUES, word) {
                    return Mood::Negative;
                }
            } else if !(has(A_WAIT_ENDING, word) && matches(&SOMETHING_CAME_APART, clean))
                && !crowd_win(words, i)
            {
                good = true;
            }
        } else if !negated || includes(before, AIM_HEADS) {
            return Mood::Negative;
        }
    }
    if good { Mood::Positive } else { Mood::Neutral }
}
