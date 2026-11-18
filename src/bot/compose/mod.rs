//! Wording selection based on bot/compose/__init__.py and fallback pools.
pub(crate) mod a_first_time;
pub(crate) mod a_thing_being_weighed;
pub(crate) mod a_thing_wants_doing;
pub(in crate::bot) mod body_state;
pub(crate) mod carrier;
pub(crate) mod check_with_you;
mod day_ahead;
mod dither;
pub(crate) mod fallback;
pub(crate) mod findout;
mod for_the_day;
pub(crate) mod gap;
pub(in crate::bot) mod habit;
pub(crate) mod instruction;
pub(crate) mod lead_in_probe;
mod leaving;
pub(crate) mod loss;
pub(crate) mod loss_reader;
mod medical_news;
mod morning_after;
pub(crate) mod nearby_one;
mod news_ask;
pub(crate) mod not_knowing;
mod occasion;
pub(super) mod only_they_know;
pub(crate) mod routes;
pub(super) mod ruledout;
mod small_day;
pub(super) mod still;
pub(crate) mod the_shape_without_the_fact;
pub(crate) mod their_detail;
pub(crate) mod their_life;
mod their_people;
pub(crate) mod theirperson;
mod thing_done;
pub(crate) mod trouble;
pub(crate) mod ward;
pub(crate) mod what_decides_it;
pub(crate) mod what_i_do_know;
pub(super) mod who;
use super::Memory;
use mt19937::MT19937;
use rand_core::Rng;
use regex::Regex;
use std::sync::LazyLock;

fn unheard_fragment(option: &str, last: Option<&str>, said: &str) -> bool {
    static FRAGMENT: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\s*([A-Z][\w'’]*(?:\s+[\w'’]+){0,3})\?\s").unwrap());
    static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-z']+").unwrap());
    if last.is_some_and(|line| line.trim_end().ends_with('?')) {
        return false;
    }
    let Some(fragment) = FRAGMENT.captures(option) else {
        return false;
    };
    let phrase = fragment[1].to_lowercase();
    let words: Vec<_> = WORD
        .find_iter(&phrase)
        .map(|w| w.as_str())
        .filter(|w| w.len() > 2)
        .collect();
    if words.is_empty()
        || words.iter().any(|w| {
            matches!(
                *w,
                "what"
                    | "who"
                    | "why"
                    | "when"
                    | "where"
                    | "which"
                    | "how"
                    | "whatever"
                    | "honestly"
                    | "really"
                    | "sorry"
                    | "right"
                    | "ok"
                    | "okay"
                    | "so"
                    | "then"
                    | "and"
                    | "but"
                    | "well"
                    | "anyway"
            )
        })
    {
        return false;
    }
    ![last.unwrap_or(""), said].iter().any(|line| {
        WORD.find_iter(&line.to_lowercase())
            .any(|w| words.contains(&w.as_str()))
    })
}

pub(crate) fn pick<'a>(options: &[&'a str], memory: &Memory, seed_text: &str) -> &'a str {
    let last = memory.turns.last().map(|turn| turn.bot.as_str());
    let standing: Vec<_> = options
        .iter()
        .copied()
        .filter(|w| !unheard_fragment(w, last, seed_text))
        .collect();
    let options = if standing.is_empty() {
        options
    } else {
        &standing
    };
    let backed: Vec<_> = if memory
        .turns
        .last()
        .is_none_or(|turn| turn.user.trim().is_empty())
    {
        options
            .iter()
            .copied()
            .filter(|w| !super::memory::history::implies_earlier_turn(w, memory, None))
            .collect()
    } else {
        Vec::new()
    };
    let options = if backed.is_empty() { options } else { &backed };
    let mut fresh: Vec<_> = options
        .iter()
        .copied()
        .filter(|option| Some(*option) != last)
        .collect();
    if fresh.is_empty() {
        fresh.extend_from_slice(options);
    }
    if !memory.turns.is_empty() {
        // The first occurrence breaks ties when every option has already been said.
        let first: Vec<_> = fresh
            .iter()
            .map(|option| memory.turns.first_reply(option))
            .collect();
        let oldest = first.iter().min().copied().flatten();
        let mut positions = first.into_iter();
        fresh.retain(|_| positions.next().flatten() == oldest);
    }
    seeded_choice(&fresh, memory, seed_text)
}

pub(crate) fn seeded_choice<'a>(options: &[&'a str], memory: &Memory, seed_text: &str) -> &'a str {
    seeded_choice_at(options, memory.turns.len(), seed_text)
}

pub(crate) fn seeded_choice_at<'a>(options: &[&'a str], turns: usize, seed_text: &str) -> &'a str {
    let turns = turns as u64;
    let seed = if seed_text.is_empty() {
        turns
    } else {
        turns
            .wrapping_mul(7919)
            .wrapping_add(seed_text.chars().map(|ch| ch as u64).sum::<u64>())
    };
    options[index(seed, options.len())]
}

fn index(seed: u64, count: usize) -> usize {
    debug_assert!(count > 0 && count <= u32::MAX as usize);
    // Python's integer seed uses little-endian 32-bit words, including one
    // zero word for seed 0. choice() draws bit_length(n) bits with rejection.
    let words = [seed as u32, (seed >> 32) as u32];
    let mut rng = MT19937::new_with_slice_seed(&words[..if words[1] == 0 { 1 } else { 2 }]);
    draw(&mut rng, count)
}

/// The fallback chooser first excludes five recent replies, then just the latest.
pub(crate) fn choose_recent<'a>(options: &[&'a str], memory: &Memory, clean: &str) -> &'a str {
    recent_with_seed(options, memory, clean.chars().map(|c| c as u64).sum())
}

pub(crate) fn choose_mixed<'a>(options: &[&'a str], memory: &Memory, text: &str) -> &'a str {
    let seed = text
        .chars()
        .fold(7u64, |value, c| (value * 31 + c as u64) % 1000003);
    recent_with_seed(options, memory, seed)
}

fn recent_with_seed<'a>(options: &[&'a str], memory: &Memory, line_seed: u64) -> &'a str {
    let mut fresh: Vec<_> = options
        .iter()
        .copied()
        .filter(|option| {
            !memory
                .turns
                .iter()
                .rev()
                .take(5)
                .any(|turn| turn.bot == **option)
        })
        .collect();
    if fresh.is_empty() {
        let last = memory
            .turns
            .iter()
            .rev()
            .take(5)
            .find(|turn| !turn.bot.is_empty());
        fresh.extend(
            options
                .iter()
                .copied()
                .filter(|option| last.is_none_or(|turn| turn.bot != **option)),
        );
    }
    let pool = if fresh.is_empty() { options } else { &fresh };
    let seed = (memory.turns.len() as u64)
        .wrapping_mul(7919)
        .wrapping_add(line_seed);
    pool[index(seed, pool.len())]
}

pub(crate) fn draw(rng: &mut MT19937, count: usize) -> usize {
    debug_assert!(count > 0 && count <= u32::MAX as usize);
    let bits = u32::BITS - (count as u32).leading_zeros();
    loop {
        let choice = (rng.next_u32() >> (u32::BITS - bits)) as usize;
        if choice < count {
            return choice;
        }
    }
}

pub(crate) fn said_again(text: &str) -> String {
    static PERSON: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"\b(?:he|she|they|him|her|them|his|hers|their|himself|herself|themselves)\b")
            .unwrap()
    });
    let flat = text.to_lowercase();
    PERSON
        .replace_all(&flat, "@")
        .split(crate::text::is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_upstream_integer_seed_and_rejection() {
        assert_eq!(
            [index(0, 5), index(1, 5), index(2, 5), index(532, 5)],
            [3, 1, 0, 1]
        );
        assert_eq!(index(u32::MAX as u64 + 1, 10), 1);
    }

    #[test]
    fn pick_keeps_seeded_ties_and_exact_last_reply_filtering() {
        let mut memory = Memory::default();
        for reply in ["HE answered.", "They wrote.", "They answered."] {
            memory.turns.record(crate::bot::Exchange {
                user: "A test input".into(),
                bot: reply.into(),
                module: "answers",
            });
        }
        let options = [
            "They wrote.",
            "She answered.",
            "They answered.",
            "He answered.",
        ];
        let tie = &["She answered.", "He answered."];
        for seed in ["one", "two", "three"] {
            assert_eq!(
                pick(&options, &memory, seed),
                seeded_choice(tie, &memory, seed)
            );
            assert_eq!(
                pick(&["She answered.", "Brand new."], &memory, seed),
                "Brand new."
            );
        }
    }
}
pub(crate) mod after_emergency;
