//! Word and phrase primitives ported from PyBot's knowledge matcher.

use std::borrow::Cow;

use crate::text::is_space;

#[path = "normalize_data.rs"]
mod tables;

const ENDINGS: &[&str] = &[
    "iness", "iest", "ings", "ing", "ies", "ied", "ier", "ers", "est", "er", "es", "ed", "ly", "s",
    "y", "ness", "ful",
];
const MIN_FORM: usize = 3;

fn has_word(table: &[&str], word: &str) -> bool {
    table.binary_search(&word).is_ok()
}

fn spelling_for<T: Copy>(table: &[(&str, T)], word: &str) -> Option<T> {
    table
        .binary_search_by(|(key, _)| key.cmp(&word))
        .ok()
        .map(|at| table[at].1)
}

fn normalize_apostrophes(text: &str) -> Cow<'_, str> {
    const APOSTROPHES: [char; 3] = ['’', '‘', 'ʼ'];
    if text.contains(APOSTROPHES) {
        Cow::Owned(text.replace(APOSTROPHES, "'"))
    } else {
        Cow::Borrowed(text)
    }
}

/// Extract lowercase ASCII words and their apostrophe suffixes from user input.
pub(crate) fn tokens(text: &str) -> Vec<String> {
    let normalized = normalize_apostrophes(text).to_lowercase();
    let mut remaining = normalized.as_str();
    let mut words = Vec::new();

    // The lookup tokenizer is [a-z0-9]+(?:'[a-z]+)? over the lowercased input.
    while let Some(start) = remaining
        .bytes()
        .position(|byte| byte.is_ascii_alphanumeric())
    {
        remaining = &remaining[start..];
        let mut end = remaining
            .bytes()
            .take_while(u8::is_ascii_alphanumeric)
            .count();
        if let Some(following) = remaining[end..].strip_prefix('\'') {
            let contraction = following.bytes().take_while(u8::is_ascii_lowercase).count();
            if contraction > 0 {
                end += 1 + contraction;
            }
        }
        words.push(remaining[..end].to_owned());
        remaining = &remaining[end..];
    }
    words
}

/// Check whether two words differ by at most one substitution, insertion, or adjacent swap.
pub(crate) fn one_edit(first: &str, second: &str) -> bool {
    let first_len = first.chars().count();
    let second_len = second.chars().count();
    if first_len.abs_diff(second_len) > 1 {
        return false;
    }
    let (shorter, longer) = if first_len <= second_len {
        (first, second)
    } else {
        (second, first)
    };
    let mut shorter = shorter.chars();
    let mut longer = longer.chars();
    loop {
        match (shorter.next(), longer.next()) {
            (Some(left), Some(right)) if left == right => continue,
            (None, _) => return true,
            (Some(left), Some(right)) if first_len == second_len => {
                return shorter.as_str() == longer.as_str()
                    || (shorter.next() == Some(right)
                        && longer.next() == Some(left)
                        && shorter.as_str() == longer.as_str());
            }
            (Some(left), Some(_)) => {
                return longer.next() == Some(left) && shorter.as_str() == longer.as_str();
            }
            (Some(_), None) => return false,
        }
    }
}

/// Strip the first recognized ending whose stem has at least three characters.
pub(crate) fn stem(word: &str) -> &str {
    let length = word.chars().count();
    ENDINGS
        .iter()
        .find_map(|ending| {
            word.strip_suffix(ending)
                .filter(|_| length >= MIN_FORM + ending.len())
        })
        .unwrap_or(word)
}

/// Check the standard vocabulary before treating an input word as a typo.
pub(crate) fn engine_reads(word: &str) -> bool {
    has_word(tables::KNOWN_WORDS, word) || has_word(tables::KNOWN_STEMS, stem(word))
}

/// Return the loose inflectional forms of a word, including the original spelling.
pub(crate) fn forms(input: &str) -> Vec<String> {
    let normalized = normalize_apostrophes(input);
    let word = normalized.as_ref();
    let length = word.chars().count();
    let mut result = vec![word.to_owned()];

    if let Some(other) = spelling_for(tables::US_SPELLINGS, word) {
        add_form(&mut result, other.to_owned(), 0);
    }
    if let Some(stem) = word.strip_suffix("ies")
        && length > 5
    {
        add_form(&mut result, format!("{stem}y"), 0);
    }
    if let Some(stem) = word.strip_suffix('y')
        && length > 3
        && !is_vowel(stem.chars().next_back())
    {
        add_form(&mut result, format!("{stem}ies"), 0);
    }

    if let Some(singular) = spelling_for(tables::IRREGULAR_PLURALS, word) {
        add_form(&mut result, singular.to_owned(), 0);
        return result;
    }
    if length <= 3 {
        return result;
    }

    if let Some(stem) = word.strip_suffix("'s")
        && !has_word(tables::CONTRACTIONS, word)
    {
        add_form(&mut result, stem.to_owned(), 0);
    } else if let Some(stem) = word.strip_suffix("s'") {
        add_form(&mut result, stem.to_owned(), 0);
        add_form(&mut result, word[..word.len() - 1].to_owned(), 0);
    }

    if let Some(stem) = word.strip_suffix('s')
        && !word.ends_with("ss")
        && !has_word(tables::RETAIN_S, word)
    {
        add_form(&mut result, stem.to_owned(), MIN_FORM);
    }
    if let Some(stem) = word.strip_suffix("es")
        && length > 4
        && (stem.ends_with("ch")
            || stem.ends_with("sh")
            || stem.ends_with(['s', 'x', 'z', 'i', 'o']))
    {
        add_form(&mut result, stem.to_owned(), MIN_FORM);
    }
    if let Some(stem) = word.strip_suffix("ed")
        && length > 4
    {
        let with_e = &word[..word.len() - 1];
        let base = if engine_reads(stem) && engine_reads(with_e) {
            with_e
        } else {
            stem
        };
        add_form(&mut result, base.to_owned(), MIN_FORM);
        if length > 5 && doubles_for_an_ending(stem) {
            add_form(
                &mut result,
                without_final_character(stem).to_owned(),
                MIN_FORM,
            );
        }
    }
    if let Some(stem) = word.strip_suffix("ing")
        && length > 5
    {
        let with_e = format!("{stem}e");
        if !has_word(tables::NO_E_ING, word) {
            add_form(&mut result, with_e.clone(), MIN_FORM);
        }
        if !has_word(tables::NO_BARE_ING, word)
            && !has_word(tables::NO_BASE, stem)
            && (!cvc(stem, length - 3)
                || stem.ends_with('y')
                || (engine_reads(stem)
                    && !engine_reads(&with_e)
                    && !has_word(tables::FUNCTION_WORDS, stem)))
        {
            add_form(&mut result, stem.to_owned(), MIN_FORM);
        }
        if doubles_for_an_ending(stem) && !has_word(tables::NO_DOUBLED_ING, word) {
            add_form(
                &mut result,
                without_final_character(stem).to_owned(),
                MIN_FORM,
            );
        }
    }

    result
}

fn add_form(forms: &mut Vec<String>, form: String, minimum: usize) {
    if form.chars().count() >= minimum && !forms.contains(&form) {
        forms.push(form);
    }
}

fn without_final_character(word: &str) -> &str {
    let mut characters = word.chars();
    characters.next_back();
    characters.as_str()
}

fn is_vowel(character: Option<char>) -> bool {
    matches!(character, Some('a' | 'e' | 'i' | 'o' | 'u'))
}

fn cvc(stem: &str, length: usize) -> bool {
    let mut tail = stem.chars().rev();
    (3..=4).contains(&length)
        && !is_vowel(tail.next())
        && is_vowel(tail.next())
        && !is_vowel(tail.next())
}

fn doubles_for_an_ending(stem: &str) -> bool {
    let mut tail = stem.chars().rev();
    let last = tail.next();
    let previous = tail.next();
    let vowel = tail.next();
    let Some(lead) = tail.next() else {
        return false;
    };
    if last != previous || is_vowel(last) || last == Some('s') {
        return false;
    }
    if last == Some('l') {
        return true;
    }
    // A u after q behaves as a consonant for doubling (quitting -> quit).
    is_vowel(vowel) && (!is_vowel(Some(lead)) || (lead == 'u' && tail.next() == Some('q')))
}

/// Return variants with one contraction or spelling expanded or contracted.
pub fn phrase_alternatives(phrase: &str) -> Vec<String> {
    let lower = phrase.to_lowercase();
    let words: Vec<_> = lower
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect();
    let mut alternatives = Vec::new();
    for width in 1..=2 {
        for (index, group) in words.windows(width).enumerate() {
            let key = if let [word] = group {
                Cow::Borrowed(*word)
            } else {
                Cow::Owned(group.join(" "))
            };
            let Some(spellings) = spelling_for(tables::PHRASE_SPELLINGS, key.as_ref()) else {
                continue;
            };
            for spelling in spellings {
                if width == 2 && spelling.contains(' ') {
                    continue;
                }
                let mut variant = words.clone();
                variant.splice(index..index + width, [*spelling]);
                alternatives.push(variant.join(" "));
            }
        }
    }
    alternatives
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_preserve_lookup_apostrophes() {
        assert_eq!(
            tokens("SISTERS’ PUPPY'S a'1 off-site abʼCD‘ef İK"),
            [
                "sisters", "puppy's", "a", "1", "off", "site", "ab'cd", "ef", "i", "k"
            ]
        );
    }

    #[test]
    fn typo_distance_counts_unicode_characters() {
        for (left, right) in [
            ("", "a"),
            ("cat", "cat"),
            ("chiken", "chicken"),
            ("aé💡", "a💡é"),
            ("abc", "aec"),
        ] {
            assert!(one_edit(left, right));
            assert!(one_edit(right, left));
        }
        assert!(!one_edit("bakes", "bank"));
        assert!(!one_edit("bakes", "bay"));
    }

    #[test]
    fn forms_keep_distinct_words_apart() {
        for (input, expected) in [
            ("cities", &["cities", "city", "citie", "citi"][..]),
            ("leaves", &["leaves", "leaf"]),
            ("cares", &["cares", "care"]),
            ("quitting", &["quitting", "quitte", "quitt", "quit"]),
            ("housing", &["housing"]),
            ("bedding", &["bedding", "bedde", "bedd"]),
            ("sister’s", &["sister's", "sister", "sister'"]),
            ("hated", &["hated", "hate"]),
        ] {
            assert_eq!(forms(input), expected, "{input}");
        }
    }

    #[test]
    fn contractions_expand_and_contract_one_span_at_a_time() {
        assert_eq!(phrase_alternatives("WON'T stay"), ["will not stay"]);
        assert_eq!(
            phrase_alternatives("I WILL\u{1e}NOT stay"),
            ["i won't stay", "i wont stay"]
        );
    }
}
