//! Shared line normalization for conversational and knowledge rules.

use crate::text::is_space;
use fancy_regex::Regex as ContextRegex;
use regex::{Captures, Regex};
use std::{collections::BTreeMap, sync::LazyLock};
use unicode_normalization::{UnicodeNormalization, char::canonical_combining_class};

#[path = "line_tables.rs"]
mod tables;
use tables::*;
#[path = "line_spelling.rs"]
mod spelling;

/// The distinct text views used by the understanding and matching layers.
#[derive(Debug, Default, serde::Serialize)]
pub(crate) struct Line {
    pub(crate) text: String,
    pub(crate) clean: String,
    pub(crate) words: Vec<String>,
    pub(crate) plain: String,
    pub(crate) norm: Vec<String>,
    pub(crate) norm_text: String,
    pub(crate) clauses: Vec<Vec<String>>,
    pub(crate) focus: Vec<String>,
    pub(crate) asking: Vec<String>,
    pub(crate) fixes: BTreeMap<String, String>,
    pub(crate) intensifier: Vec<String>,
    pub(crate) intensifier_left: Vec<String>,
    pub(crate) signals: Signals,
    pub(crate) dropped_subject: bool,
    pub(crate) change_of_subject: bool,
    pub(crate) kept: Option<Kept>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct Kept {
    pub(crate) place: Option<String>,
    pub(crate) time: String,
}

#[derive(Debug, Default, serde::Serialize)]
pub(crate) struct Signals {
    pub(crate) question_mark: bool,
    pub(crate) shout: bool,
    pub(crate) laughter: bool,
    pub(crate) ellipsis: bool,
    pub(crate) exclamation: bool,
    pub(crate) empty: bool,
}

macro_rules! pattern {
    ($label:ident, $regex:expr) => {
        static $label: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($regex).expect(stringify!($label)));
    };
}

const MEASURE: &str = r"[0-9]+(?:[.,][0-9]+)*[a-z%]*|[a-z]{1,3}[0-9][a-z0-9]*|[a-z]+(?:'[a-z]+)*";
pattern!(RAW_WORD, MEASURE);
pattern!(TOKEN, &format!(r"{MEASURE}|\|"));
pattern!(
    CURRENCY_SIGN,
    r"([£$€])\s*([0-9]+(?:[.,][0-9]+)*)(?:\s*([km])\b)?"
);
pattern!(PUNCT_BREAK, r"\s*[,;:!?]\s*");
pattern!(SENTENCE_END, r"[a-z0-9]{2}\.");
pattern!(THOUSANDS, r"([0-9]),([0-9]{3})\b");
pattern!(THOUSANDS_SPACE, r"[0-9]{1,3}(?: [0-9]{3})+");
pattern!(MRS_AS_WIFE, r"\b(the|my|our|his|her)\s+[Mm]rs\b\.?");
pattern!(DAY_NAMED, r"\b(birthday|anniversary|wedding)s?\b");
pattern!(
    OWNER_WITHOUT_DAY,
    r"\b(?:it|its|it's|that|thats|that's)(?:\s+is)?\s+(?:not\s+)?(?:my|our)\s+([a-z]+'?s)\b"
);
pattern!(
    NUMBER_OF_DAY,
    r"\b(?:my|our)\s+([a-z]+\'?s?)\s+([0-9]+(?:st|nd|rd|th))\b"
);
pattern!(
    NOT_MY_DAY,
    r"\s*(?:,?\s*\b(?:and\s+)?not\s+(?:my|mine)\s+(birthday|bday|anniversary|wedding)\b)"
);
pattern!(
    GAME_FORM,
    concat!(
        r"\bhow\s+many\s+(?:players?|people|a\s+side)\b|\bhow\s+(?:long|many\s+halves)\b[^.]{0,40}\b(?:match|game|half|halves)\b",
        r"|\b(?:the\s+)?rules?\b|\boffside\b|\bhow\s+big\b|\bpitch\s+(?:size|dimensions)\b|\bfull\s+(?:match|game)\b"
    )
);
pattern!(
    TEAM_SEARCH,
    concat!(
        r"\b(?:join|joining|find|finding|looking\s+for|sign\s+up\s+(?:for|to)|get\s+into)\b",
        r"|\bteams?\b[^.]{0,24}\b(?:in|near|around|local)\b|\b(?:clubs?|leagues?)\b"
    )
);
pattern!(CAN_NOT, r"\bcan\s+not\b");
pattern!(
    SPLIT_CONTRACTION,
    r"\b(i|u|you|he|she|it|we|they|that|there|what|who|where|when|how|this|these|those|let)\s+(s|m|re|ve|ll|d)\b"
);
pattern!(SPLIT_NOT, r"\b([a-z]{2,}n)\s+t\b");
pattern!(
    EXCLAIMED_AFTER_QUESTION,
    r"\b(what|how|why|where|who|which|when|whatever|however|whoever|whenever)\s+(?:on\s+earth|in\s+the\s+world|the\s+dickens|the\s+blazes|on\s+god'?s\s+earth)\b"
);
pattern!(
    EXCLAMATION_PHRASE,
    r"^(?:oh\s+my\s+god|my\s+god|good\s+grief|for\s+goodness'?\s*sake|for\s+god'?s?\s*sake)\s+"
);
pattern!(
    UP_TOO_ASKS,
    r"(\b(?:what|whats|watcha|whatcha|wotcha)\b[^.!?]{0,24}?\b)up too\b"
);
pattern!(LAUGH_RUN, r"^(?:ha|he|hi|ho|hu|l(?:ol)+)+$|^h{2,}$");
pattern!(LAUGH_PHRASE, r"\b(?:ha\s*){2,}|\b(?:heh\s*){2,}");
pattern!(
    CHANGE_TOPIC,
    concat!(
        r"^(?:(?:and|so|but|oh|ok|okay|right|now|anyway|anyways|also|plus|well)\s*,?\s*){0,3}",
        r"(?:(?:changing|change\s+of|changing\s+the)\s+(?:the\s+)?(?:subject|topic|tack)",
        r"|(?:moving\s+on\s+to|on\s+to|onto|on|to)\s+(?:a\s+)?(?:new|different|another)\s+(?:subject|topic|tack)",
        r"|(?:moving\s+on\s+to|on\s+to|onto)\s+something\s+else",
        r"|moving\s+on|(?:a\s+)?(?:new|different|another)\s+(?:subject|topic|tack))\s*[,;:.–—-]+\s*",
    )
);
pattern!(
    TYPED_SURPRISE,
    concat!(
        r"\b(?:what|why|when|where|who|which|whose|how|whatever|however|whoever|whenever)",
        r"\s+(on\s+earth|in\s+the\s+world|the\s+(?:hell|heck|devil|blazes|dickens|fuck))",
    )
);
pattern!(SURPRISE_SHORT, r"\b(?:wtf|wth|wtaf)\b");
pattern!(SURPRISE_GAPS, r"[^a-z']+");
pattern!(KEPT_NUMBER, r"^[0-9]{1,3}(?:st|nd|rd|th)?$");
pattern!(
    NAMED_LETTER,
    concat!(
        r"\b(?:with\s+(?:an?|one|two|three|double)\s+[a-z]\b",
        r"|(?:one|two|three|a\s+double)\s+[a-z]\s+or\s+(?:one|two|three|an?)\b",
        r"|double\s+[a-z]\b|[a-z]\s+or\s+(?:two|three)\b)",
    )
);

/// Normalize raw input into its surface, repaired, and expanded forms.
pub(crate) fn normalise(raw: &str) -> Line {
    let (clean, tags, change_of_subject) = clean_line(raw);
    let clean = docs_as_doctor(&meal_name(&clean));
    let said = spell_currency(&clean);
    let mut words: Vec<_> = RAW_WORD
        .find_iter(&said)
        .map(|matched| matched.as_str().to_owned())
        .collect();
    let phrase_text = phrase_pass(&said);
    let punctuated = split_punctuation(&phrase_text);
    let tokens: Vec<_> = TOKEN
        .find_iter(&punctuated)
        .map(|matched| matched.as_str())
        .collect();
    let mut tokens = said_apart(&join_chat_digits(&tokens));
    possessive_me(&mut tokens);
    tokens = pretoken_context(&tokens);
    let mut tokens = one_relation_says(&tokens);
    let mut fixes = contextual_slips(&mut tokens);
    let mut fixed = fixes.clone();
    let mut plain = Vec::new();
    let mut norm: Vec<String> = Vec::new();
    let mut dropped_subject = false;
    for (index, word) in tokens.iter().enumerate() {
        let word = word.as_str();
        let before = index
            .checked_sub(1)
            .and_then(|index| tokens.get(index))
            .map(String::as_str);
        let before_before = index
            .checked_sub(2)
            .and_then(|index| tokens.get(index))
            .map(String::as_str);
        let end = (index + 4).min(tokens.len());
        let next = &tokens[index + 1..end];
        let after = next.first().map(String::as_str);
        if opens_without_subject(&tokens[..index], word, &tokens[index + 1..]) {
            norm.extend(["i".into(), word.to_owned()]);
            plain.push(word.to_owned());
            dropped_subject = true;
            continue;
        }
        if a_name_slot(word, before, before_before) {
            norm.push(word.to_owned());
            plain.push(word.to_owned());
            continue;
        }
        let spelled = if let Some(verb) = spelling::core_verb(word, before, after) {
            repair(&mut fixes, word, verb);
            repair(&mut fixed, word, verb);
            vec![verb]
        } else if matches!(after, None | Some("|"))
            && let Some(state) =
                spelling::terminal_state(word, &tokens[index.saturating_sub(5)..index])
        {
            vec![state]
        } else if (present(FIRST_PERSON_BE, before)
            || present(FIRST_PERSON_BE, before_before) && present(AMOUNT_BEFORE_A_STATE, before))
            && let Some(state) = spelling::state_ending(word)
        {
            vec![state]
        } else {
            positional(word, before, before_before, next)
        };
        for spelled in spelled {
            let mut expanded = token(spelled, before, next, &mut fixes);
            if expanded.len() == 1
                && expanded[0] == spelled
                && !has(LEXICON, spelled)
                && let Some(state) =
                    spelling::near_state(spelled, &tokens[index.saturating_sub(5)..index], next)
            {
                repair(&mut fixes, spelled, &state);
                expanded[0] = state;
            }
            if expanded.first() == norm.last()
                && expanded
                    .first()
                    .is_some_and(|first| has(REPEAT_ONCE, first))
            {
                expanded.remove(0);
            } else if let [first, second, ..] = expanded.as_slice()
                && first == "i"
                && second == "do"
                && norm
                    .last()
                    .is_some_and(|last| has(REPEAT_ONCE_SUBJECTS, last))
            {
                expanded.remove(0);
            }
            if expanded.is_empty() {
                continue;
            }
            let plain_word = if expanded.len() > 1 && has(PLAIN_EXPANDS, spelled) {
                expanded.join(" ")
            } else if expanded.len() == 1
                && get(DIGIT_WORDS, spelled).is_none()
                && get(US_SPELLINGS, spelled).is_none()
            {
                let first = &expanded[0];
                if (first != spelled
                    && (get(MISSPELLINGS, spelled).is_some()
                        || get(URGENT_MISSPELLINGS, spelled).is_some()
                        || has(SAID_SHORT, spelled) && !short_word_named(spelled, before, after)))
                    || (word == "bin" && first == "been")
                {
                    fixed.insert(word.to_owned(), first.clone());
                } else if word != spelled && has(STATES_WITHOUT_G, word) {
                    fixed.insert(word.to_owned(), spelled.to_owned());
                }
                first.clone()
            } else {
                get(TYPED_CONTRACTIONS, spelled)
                    .unwrap_or(spelled)
                    .to_owned()
            };
            plain.push(plain_word);
            norm.extend(expanded);
        }
    }

    fix_auxiliaries(&mut norm);
    let intensifier_left = remove_intensifiers(&mut norm);
    let mut intensifier = typed_intensifiers(raw);
    if intensifier.is_empty() {
        intensifier.clone_from(&intensifier_left);
    }
    game_name(&mut norm, &mut words);
    body_words(&mut norm, &mut plain, &mut fixes);
    if fixes.get("mon").is_some_and(|word| word == "moon")
        && !tokens.iter().any(|word| has(MOON_CUES, word))
    {
        fixes.remove("mon");
        for word in plain.iter_mut().chain(&mut norm) {
            if word == "moon" {
                "mon".clone_into(word);
            }
        }
    }
    let (clauses, focus) = split_and_focus(&norm, &mut fixes);
    let asking = focus.map_or_else(Vec::new, |index| {
        let mut ask = Vec::new();
        if index > 0 {
            let prior = strip_leaders(&clauses[index - 1]);
            if prior.first().is_some_and(|word| word == "how")
                && prior.get(1).is_some_and(|word| has(HOW_DEGREE, word))
            {
                ask.extend_from_slice(&clauses[index - 1]);
            }
        }
        ask.extend_from_slice(&clauses[index]);
        ask
    });
    let signals = Signals {
        question_mark: raw.contains('?'),
        shout: raw.split(is_space).any(|word| {
            word.chars().count() > 2
                && word.chars().any(char::is_uppercase)
                && !word.chars().any(char::is_lowercase)
        }),
        laughter: norm.iter().chain(&tags).any(|word| is_laugh(word)) || LAUGH_PHRASE.is_match(raw),
        ellipsis: raw.contains(".."),
        exclamation: raw.contains('!'),
        empty: words.is_empty(),
    };
    let keep_spelling = keeps_spelling(&words, &fixed);
    for (typed, repaired) in &fixes {
        if typed != repaired && !repaired.contains(' ') && !only_doubles_a_letter(typed, repaired) {
            fixed
                .entry(typed.clone())
                .or_insert_with(|| repaired.clone());
        }
    }
    if !keep_spelling {
        for word in &mut words {
            if let Some(repaired) = fixed.get(word) {
                word.clone_from(repaired);
            }
        }
    }
    let plain = plain.join(" ").replace(" |", ",").replace('|', ",");
    let focus = focus.map_or_else(Vec::new, |index| clauses[index].clone());
    let norm_text = clauses
        .iter()
        .map(|clause| clause.join(" "))
        .collect::<Vec<_>>()
        .join(" | ");
    let norm: Vec<_> = clauses.iter().flatten().cloned().collect();
    let kept = kept_for(&norm);
    Line {
        text: raw.to_owned(),
        clean,
        words,
        plain: plain.trim_matches([' ', ',']).to_owned(),
        norm,
        norm_text,
        clauses,
        focus,
        asking,
        fixes,
        intensifier,
        intensifier_left,
        signals,
        dropped_subject,
        change_of_subject,
        kept,
    }
}

pub(super) fn common_name(word: &str) -> bool {
    has(COMMON_NAMES, word)
}
pub(crate) fn body_word(word: &str) -> bool {
    has(BODY_PARTS, word)
}
pub(super) fn count_unit(word: &str) -> bool {
    has(COUNT_UNITS, word)
}

fn has(table: &[&str], word: &str) -> bool {
    table.binary_search(&word).is_ok()
}

pub(crate) fn lexical_word(word: &str) -> bool {
    has(LEXICON, word)
}

pub(super) fn curated_spelling(word: &str) -> Option<&'static str> {
    [
        CHAT,
        EXPAND,
        US_SPELLINGS,
        MISSPELLINGS,
        URGENT_MISSPELLINGS,
    ]
    .into_iter()
    .find_map(|table| get(table, word))
}

pub(super) fn repaired_word(word: &str) -> Vec<String> {
    token(word, None, &[], &mut BTreeMap::new())
}

pub(super) fn spells_a_state(word: &str) -> bool {
    spelling::state_ending(word).is_some()
        || spelling::terminal_state_word(word).is_some()
        || spelling::two_letters_gone(word).is_some()
        || spelling::state_without_vowel(word).is_some()
}

fn get<'a>(table: &[(&str, &'a str)], word: &str) -> Option<&'a str> {
    table
        .binary_search_by(|(key, _)| key.cmp(&word))
        .ok()
        .map(|index| table[index].1)
}

fn present(table: &[&str], value: Option<&str>) -> bool {
    value.is_some_and(|word| has(table, word))
}

fn repair(fixes: &mut BTreeMap<String, String>, original: &str, replacement: &str) {
    fixes.insert(original.to_owned(), replacement.to_owned());
}

fn clean_line(raw: &str) -> (String, Vec<String>, bool) {
    let unaccented = strip_accents(raw);
    let glued = glued_said_apart(&unaccented);
    let split = typed_without_spaces(&glued);
    let joined = join_spaced_abbreviation(&split);
    let joined = MRS_AS_WIFE.replace_all(&joined, |found: &Captures<'_>| {
        let stop = found.get(0).unwrap().end();
        if stop < joined.len()
            && joined[stop..].starts_with(' ')
            && joined
                .as_bytes()
                .get(stop + 1)
                .is_some_and(u8::is_ascii_uppercase)
        {
            found[0].to_owned()
        } else {
            format!("{} wife", &found[1])
        }
    });
    let lowered = joined.to_lowercase();
    let mut clean = CAN_NOT.replace_all(&lowered, "cant").into_owned();
    clean = name_day(&clean);
    while let std::borrow::Cow::Owned(joined) = THOUSANDS.replace_all(&clean, "${1}${2}") {
        clean = joined;
    }
    clean = THOUSANDS_SPACE
        .replace_all(&clean, |found: &Captures<'_>| {
            let matched = found.get(0).unwrap();
            let start = matched.start();
            let end = matched.end();
            let bytes = clean.as_bytes();
            if start > 0 && bytes[start - 1].is_ascii_digit()
                || start >= 5
                    && bytes[start - 1] == b' '
                    && bytes[start - 5..start - 1].iter().all(u8::is_ascii_digit)
                || bytes.get(end).is_some_and(u8::is_ascii_digit)
            {
                matched.as_str().to_owned()
            } else {
                matched.as_str().replace(' ', "")
            }
        })
        .into_owned();
    for (source, replacement) in [
        ("chick pea", "chickpea"),
        ("chick peas", "chickpeas"),
        ("sweet corn", "sweetcorn"),
    ] {
        clean = replace_bounded(&clean, source, replacement, false);
    }
    if let Some((first, rest)) = clean.split_once(' ')
        && first.trim_matches([',', '.', '!', '?']) == "like"
        && rest.split(' ').next().is_some_and(|word| {
            has(
                LIKE_STARTS_A_CLAUSE,
                word.trim_matches([',', '.', '!', '?']),
            )
        })
    {
        clean = rest.to_owned();
    }
    clean = rewrite_joined_words(&clean);
    clean = splice_contraction(&clean);
    clean = remove_exclamation(&clean).to_owned();
    clean = EXCLAIMED_AFTER_QUESTION
        .replace_all(&clean, "$1")
        .into_owned();
    clean = normalise_short_hello(&clean);
    clean = spell_chat_words(&clean);
    clean = spell_ill(&clean);
    let (mut clean, tags) = remove_tags(&clean);
    let mut change_of_subject = false;
    if let Some(lead) = CHANGE_TOPIC.find(&clean) {
        let rest = &clean[lead.end()..];
        if RAW_WORD.find_iter(rest).take(2).count() == 2 {
            clean = rest.to_owned();
            change_of_subject = true;
        }
    }
    (clean, tags, change_of_subject)
}

fn name_day(text: &str) -> String {
    let mut with_day = String::new();
    let mut end = 0;
    if let Some(named) = DAY_NAMED.captures(text) {
        let named_end = named.get(0).unwrap().end();
        for owner in OWNER_WITHOUT_DAY.find_iter(text) {
            if owner.start() < named_end
                || text[owner.end()..]
                    .split([',', '.', ';', ':', '!', '?'])
                    .next()
                    .is_some_and(|tail| {
                        tail.split_whitespace()
                            .any(|word| !has(TAIL_OF_A_DAY, word))
                    })
            {
                continue;
            }
            with_day.push_str(&text[end..owner.end()]);
            with_day.push(' ');
            with_day.push_str(&named[1]);
            end = owner.end();
        }
    }
    with_day.push_str(&text[end..]);
    if let Some(denied) = NOT_MY_DAY.captures(&with_day) {
        let denied_line = denied.get(0).unwrap();
        if let Some(owner) = NUMBER_OF_DAY
            .captures_iter(&with_day[..denied_line.start()])
            .filter(|owner| has(KIN, owner[1].trim_end_matches('s')))
            .last()
        {
            let owner_end = owner.get(0).unwrap().end();
            return format!(
                "{} {} {}{}",
                &with_day[..owner_end],
                &denied[1],
                &with_day[owner_end..denied_line.start()],
                &with_day[denied_line.end()..]
            )
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        }
    }
    with_day
}

fn spell_ill(text: &str) -> String {
    let mut words: Vec<_> = text.split_whitespace().collect();
    for index in 0..words.len().saturating_sub(1) {
        let before = index.checked_sub(1).map(|index| words[index]);
        if reads_as_will(words[index], before, Some(words[index + 1])) {
            words[index] = "i will";
        }
    }
    words.join(" ")
}

fn reads_as_will(token: &str, before: Option<&str>, after: Option<&str>) -> bool {
    token == "ill"
        && matches!(after, Some("go" | "take" | "catch"))
        && (matches!(before, None | Some("|")) || present(LEADERS, before))
}

fn meal_name(text: &str) -> String {
    let words: Vec<_> = RAW_WORD.find_iter(text).collect();
    let mut output = String::new();
    let mut end = 0;
    for (index, word) in words.iter().enumerate().skip(1) {
        let typed = word.as_str();
        if typed.len() < 3
            || typed == "tee"
            || has(LEXICON, typed)
            || !has(FOOD_SLOT_LEAD, plain_chat(words[index - 1].as_str()))
            || words
                .get(index + 1)
                .is_some_and(|next| !has(TIME_AFTER_MEAL, plain_chat(next.as_str())))
        {
            continue;
        }
        let prefix = MEAL_WORDS
            .iter()
            .copied()
            .find(|meal| typed.len() >= 4 && meal.starts_with(typed));
        let mut choices = MEAL_WORDS
            .iter()
            .copied()
            .filter(|meal| spelling::feeling_edit(typed, meal));
        if let Some(meal) = prefix.or_else(|| choices.next().filter(|_| choices.next().is_none())) {
            output.push_str(&text[end..word.start()]);
            output.push_str(meal);
            end = word.end();
        }
    }
    output.push_str(&text[end..]);
    output
}

fn docs_as_doctor(text: &str) -> String {
    let words: Vec<_> = RAW_WORD.find_iter(text).collect();
    if !words
        .iter()
        .any(|word| matches!(word.as_str(), "doc" | "docs"))
        || words.iter().enumerate().any(|(index, word)| {
            let word = word.as_str();
            has(DOCS_PAPER, word)
                || has(DOCS_SCREEN, word)
                || matches!(
                    word,
                    "phone" | "tablet" | "laptop" | "pc" | "screen" | "file"
                ) && words[index.saturating_sub(2)..index]
                    .iter()
                    .any(|word| has(DOCS_ON, word.as_str()))
        })
    {
        return text.into();
    }
    let mut output = String::new();
    let mut end = 0;
    for (index, word) in words.iter().enumerate() {
        if matches!(word.as_str(), "doc" | "docs")
            && words[index.saturating_sub(4)..(index + 3).min(words.len())]
                .iter()
                .any(|word| has(DOCS_VISIT, word.as_str()))
        {
            output.push_str(&text[end..word.start()]);
            output.push_str("doctor");
            end = word.end();
        }
    }
    output.push_str(&text[end..]);
    output
}

fn typed_intensifiers(text: &str) -> Vec<String> {
    let lower = text.to_lowercase().replace('’', "'");
    let letters = SURPRISE_GAPS.replace_all(&lower, " ");
    let expanded = SURPRISE_SHORT.replace_all(&letters, "what on earth");
    let mut found = Vec::new();
    for part in TYPED_SURPRISE.captures_iter(&expanded) {
        if part
            .get(0)
            .is_some_and(|m| expanded[..m.start()].ends_with('\''))
        {
            continue;
        }
        for word in part[1].split_whitespace() {
            if !found.iter().any(|known| known == word) {
                found.push(word.to_owned());
            }
        }
    }
    found
}

fn remove_intensifiers(words: &mut Vec<String>) -> Vec<String> {
    let mut index = 0;
    let mut kept = Vec::with_capacity(words.len());
    let mut removed = Vec::new();
    while index < words.len() {
        let word = &words[index];
        kept.push(word.clone());
        index += 1;
        if !has(WH_OPENERS, word) {
            continue;
        }
        let phrase = words
            .get(index..index + 3)
            .is_some_and(|words| words == ["in", "the", "world"]);
        let pair = words.get(index..index + 2).is_some_and(|words| {
            words == ["on", "earth"]
                || words[0] == "the"
                    && matches!(
                        words[1].as_str(),
                        "hell" | "heck" | "devil" | "blazes" | "dickens" | "fuck"
                    )
        });
        let length = if phrase {
            3
        } else if pair {
            2
        } else {
            0
        };
        removed.extend(words[index..index + length].iter().cloned());
        index += length;
    }
    *words = kept;
    removed
}

fn game_name(norm: &mut Vec<String>, words: &mut Vec<String>) {
    if !norm
        .iter()
        .any(|word| has(A_SIDE, word) || matches!(word.as_str(), "kickabout" | "kick"))
        || words.iter().any(|word| has(BODY_PARTS, word))
            && words.iter().any(|word| has(PAIN_WORDS, word))
    {
        return;
    }
    let text = norm.join(" ");
    if GAME_FORM.is_match(&text) || TEAM_SEARCH.is_match(&text) {
        return;
    }
    let mut index = 0;
    let mut named = false;
    let mut read = Vec::with_capacity(norm.len());
    while index < norm.len() {
        let word = &norm[index];
        let span = if has(A_SIDE, word)
            && norm
                .get(index + 1..index + 3)
                .is_some_and(|tail| tail == ["a", "side"])
        {
            3
        } else if word == "kickabout" {
            1
        } else if word == "kick" && norm.get(index + 1).is_some_and(|next| next == "about") {
            2
        } else {
            read.push(word.clone());
            index += 1;
            continue;
        };
        read.push("football".to_owned());
        named = true;
        index += span;
    }
    if named {
        *norm = read;
        if !words.iter().any(|word| word == "football") {
            words.push("football".into());
        }
    }
}

fn body_words(norm: &mut [String], plain: &mut [String], fixes: &mut BTreeMap<String, String>) {
    let pain: Vec<_> = norm
        .iter()
        .enumerate()
        .filter_map(|(index, word)| has(PAIN_WORDS, word).then_some(index))
        .collect();
    if pain.is_empty() {
        return;
    }
    for index in 0..norm.len() {
        let owned = index
            .checked_sub(1)
            .and_then(|prior| norm.get(prior))
            .is_some_and(|word| matches!(word.as_str(), "my" | "our"));
        if !pain
            .iter()
            .any(|&pain| index.abs_diff(pain) == 1 || owned && index.abs_diff(pain) <= 3)
        {
            continue;
        }
        let word = &mut norm[index];
        if word.len() < 3
            || !word.bytes().all(|byte| byte.is_ascii_lowercase())
            || has(LEXICON, word)
            || has(BODY_PARTS, word)
        {
            continue;
        }
        let mut parts = BODY_PARTS.iter().copied().filter(|part| {
            part.len() >= 4
                && part.len().abs_diff(word.len()) == 1
                && super::normalize::one_edit(part, word)
        });
        if let Some(part) = parts.next().filter(|_| parts.next().is_none()) {
            repair(fixes, word, part);
            part.clone_into(word);
        }
    }
    if norm.iter().any(|word| has(PAIN_WORDS, word)) {
        for word in plain {
            if let Some(repaired) = fixes.get(word) {
                word.clone_from(repaired);
            }
        }
    }
}

fn kept_for(words: &[String]) -> Option<Kept> {
    let (index, time) = words.iter().enumerate().find_map(|(index, word)| {
        let next = words.get(index + 1);
        if (has(SMALL_NUMBERS, word) || KEPT_NUMBER.is_match(word))
            && let Some(day) = next.filter(|word| has(DAY_WORDS, word))
        {
            return Some((index, format!("{word} {day}")));
        }
        has(KEPT_ALREADY, word).then(|| {
            let time = if word == "since"
                && let Some(next) = next
            {
                format!("{word} {next}")
            } else {
                word.to_owned()
            };
            (index, time)
        })
    })?;
    if let Some((position, place)) = words
        .iter()
        .enumerate()
        .find(|(_, word)| has(KEEPING_PLACES, word))
    {
        return (index.abs_diff(position) <= 10).then(|| Kept {
            place: Some(place.clone()),
            time,
        });
    }
    words
        .iter()
        .any(|word| has(KEPT_THINGS, word))
        .then_some(Kept { place: None, time })
}

fn glued_said_apart(text: &str) -> String {
    let mut words = text
        .split(is_space)
        .filter(|word| !word.is_empty())
        .peekable();
    let mut out = Vec::new();
    while let Some(word) = words.next() {
        match glued_words(word, words.peek().copied()) {
            Some(parts) => {
                let first = get(CHAT, parts[0])
                    .filter(|value| value.contains(' '))
                    .unwrap_or(parts[0]);
                out.push(first);
                out.extend(&parts[1..]);
            }
            None => out.push(word),
        }
    }
    out.join(" ")
}

fn glued_words<'a>(word: &'a str, following: Option<&str>) -> Option<Vec<&'a str>> {
    if word.chars().count() < 4
        || has(LEXICON, word)
        || word.chars().any(char::is_uppercase)
        || !word.chars().any(char::is_lowercase)
    {
        return None;
    }
    let bare = word.replace(['\'', '’'], "");
    if !has(GLUED_COUNTRY_NAMES, &bare) && !has(COMMON_NAMES, &bare) {
        for (who, states) in [("im", IM_GLUED_STATES), ("ive", IVE_GLUED_STATES)] {
            if bare.strip_prefix(who).is_some_and(|rest| has(states, rest)) {
                let original = if who == "im" && word.starts_with("i'm") {
                    word.get(3..)
                } else if who == "ive" && word.starts_with("i've") {
                    word.get(4..)
                } else {
                    word.strip_prefix(who)
                };
                if let Some(rest) = original {
                    return Some(vec![who, rest]);
                }
            }
        }
        for who in ["u", "i", "we", "you", "they", "he", "she"] {
            if bare
                .strip_prefix(who)
                .is_some_and(|rest| has(I_GLUED, rest))
                && let Some(rest) = word.strip_prefix(who)
            {
                return Some(vec![who, rest]);
            }
        }
    }
    if let Some(rest) = word.strip_prefix("im")
        && has(STATES_SAID_ONTO_IM, rest)
    {
        return Some(vec!["im", rest]);
    }
    if word.get(..word.len() / 2) == word.get(word.len() / 2..) {
        return None;
    }
    for cut in (2..=6).rev() {
        let Some((first, rest)) = word.split_at_checked(cut) else {
            continue;
        };
        if !has(GLUED_LEFT, first) {
            continue;
        }
        let valid = |second| {
            first != second && !(has(GLUED_DO, first) && present(GLUED_AFTER_A_DO, following))
        };
        if has(GLUED_RIGHT, rest) {
            return valid(rest).then(|| vec![first, rest]);
        }
        if rest == "itthere" {
            return Some(vec![first, "there"]);
        }
        for second in (2..=5).rev() {
            if let Some((second, third)) = rest.split_at_checked(second)
                && has(GLUED_RIGHT, second)
                && has(GLUED_RIGHT, third)
            {
                return valid(second).then(|| vec![first, second, third]);
            }
        }
    }
    None
}

fn strip_accents(raw: &str) -> String {
    let mut mapped = String::with_capacity(raw.len());
    for letter in raw.chars() {
        match letter {
            'æ' => mapped.push_str("ae"),
            'Æ' => mapped.push_str("AE"),
            'œ' => mapped.push_str("oe"),
            'Œ' => mapped.push_str("OE"),
            'ø' => mapped.push('o'),
            'Ø' => mapped.push('O'),
            'ł' => mapped.push('l'),
            'Ł' => mapped.push('L'),
            'đ' | 'ð' => mapped.push('d'),
            'Đ' | 'Ð' => mapped.push('D'),
            'þ' => mapped.push_str("th"),
            'Þ' => mapped.push_str("TH"),
            'ß' => mapped.push_str("ss"),
            'ı' => mapped.push('i'),
            _ => mapped.push(letter),
        }
    }
    mapped
        .nfkd()
        .filter(|letter| canonical_combining_class(*letter) == 0)
        .collect()
}

fn typed_without_spaces(raw: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    for word in raw.split(is_space).filter(|word| !word.is_empty()) {
        let start = word.find(char::is_alphabetic).unwrap_or(word.len());
        let end = word.rfind(char::is_alphabetic).map_or(start, |end| {
            end + word[end..].chars().next().unwrap().len_utf8()
        });
        if let Some(pieces) = split_word_run(&word[start..end]) {
            let mut phrase = pieces.join(" ");
            phrase.push_str(&word[end..]);
            if start > 0 && !words.is_empty() {
                words[0].insert_str(0, &word[..start]);
            } else {
                phrase.insert_str(0, &word[..start]);
            }
            words.push(phrase);
        } else {
            words.push(word.to_owned());
        }
    }
    words.join(" ")
}

fn split_word_run(word: &str) -> Option<Vec<String>> {
    if !word.bytes().all(|byte| byte.is_ascii_alphabetic()) || word.len() < 8 {
        return None;
    }
    let word = word.to_ascii_lowercase();
    if has(LEXICON, &word) {
        return None;
    }
    static LONGEST_WORD: LazyLock<usize> = LazyLock::new(|| {
        LEXICON
            .iter()
            .chain(RUN_ON_WORD_EXTRAS)
            .map(|word| word.len())
            .max()
            .unwrap_or(0)
    });
    let mut best = vec![None; word.len() + 1];
    best[0] = Some((0, 0));
    for start in 0..word.len() {
        let Some((_, length)) = best[start] else {
            continue;
        };
        for end in (start + 1..=word.len().min(start + *LONGEST_WORD)).rev() {
            let piece = &word[start..end];
            if (has(LEXICON, piece) || has(RUN_ON_WORD_EXTRAS, piece))
                && best[end].is_none_or(|(_, pieces)| length + 1 < pieces)
            {
                best[end] = Some((start, length + 1));
            }
        }
    }
    let (_, piece_count) = best[word.len()]?;
    let mut pieces = Vec::with_capacity(piece_count);
    let mut end = word.len();
    while end > 0 {
        let (start, _) = best[end]?;
        pieces.push(&word[start..end]);
        end = start;
    }
    pieces.reverse();
    if pieces.len() < 2
        || has(RUN_ON_LAST_PIECES, pieces.last()?)
        || pieces.iter().filter(|word| word.len() == 1).count() > 1
    {
        return None;
    }
    let first = pieces[0];
    let a_self = has(RUN_ON_SELF, first) || has(RUN_ON_MINE, first);
    if (a_self && (word.len() < 9 || pieces.len() < 3))
        || (!a_self
            && (!has(RUN_ON_OPENERS, first) || !pieces.iter().any(|word| has(RUN_ON_PEOPLE, word))))
    {
        return None;
    }
    let first = get(CHAT, first)
        .filter(|value| value.contains(' '))
        .unwrap_or(first);
    Some(
        std::iter::once(first)
            .chain(pieces.into_iter().skip(1))
            .map(str::to_owned)
            .collect(),
    )
}

fn join_spaced_abbreviation(text: &str) -> String {
    let words: Vec<_> = text.split(' ').collect();
    let mut output = Vec::new();
    let mut index = 0;
    while index < words.len() {
        if let Some((word, length)) = spaced_abbreviation(&words[index..]) {
            output.push(word);
            index += length;
        } else {
            output.push(words[index].to_owned());
            index += 1;
        }
    }
    output.join(" ")
}

fn spaced_abbreviation(words: &[&str]) -> Option<(String, usize)> {
    for length in (3..=4).rev() {
        let Some(part) = words.get(..length) else {
            continue;
        };
        if !part
            .iter()
            .all(|word| word.len() == 1 && word.as_bytes()[0].is_ascii_alphanumeric())
        {
            continue;
        }
        let joined = part.concat().to_ascii_lowercase();
        if get(CHAT, &joined).is_some() {
            return Some((joined, length));
        }
    }
    None
}

fn replace_bounded(text: &str, needle: &str, replacement: &str, digits: bool) -> String {
    let mut out = String::with_capacity(text.len());
    let mut end = 0;
    let word = |byte: &u8| byte.is_ascii_lowercase() || digits && byte.is_ascii_digit();
    let bytes = text.as_bytes();
    for (start, found) in text.match_indices(needle) {
        let stop = start + found.len();
        if !bytes.get(start.wrapping_sub(1)).is_some_and(word) && !bytes.get(stop).is_some_and(word)
        {
            out.push_str(&text[end..start]);
            out.push_str(replacement);
            end = stop;
        }
    }
    out.push_str(&text[end..]);
    out
}

fn rewrite_joined_words(clean: &str) -> String {
    RAW_WORD
        .replace_all(clean, |part: &Captures<'_>| {
            get(JOINED_WORDS, &part[0]).unwrap_or(&part[0]).to_owned()
        })
        .into_owned()
}

fn splice_contraction(clean: &str) -> String {
    let joined = SPLIT_CONTRACTION.replace_all(clean, "${1}'${2}");
    SPLIT_NOT
        .replace_all(&joined, |part: &Captures<'_>| {
            if has(NOT_WORDS, &format!("{}t", &part[1])) {
                format!("{}'t", &part[1])
            } else {
                part[0].to_owned()
            }
        })
        .into_owned()
}

fn remove_exclamation(mut text: &str) -> &str {
    for _ in 0..3 {
        if let Some(lead) = EXCLAMATION_PHRASE.find(text) {
            let rest = &text[lead.end()..];
            if rest.split(' ').next().is_some_and(|word| {
                has(
                    EXCLAMATION_SUBJECTS,
                    word.trim_matches([',', '.', '!', '?', ';', ':', '-', '|']),
                )
            }) {
                text = rest;
                continue;
            }
            break;
        }
        let Some((head, rest)) = text.split_once(' ') else {
            break;
        };
        let next = rest
            .split(' ')
            .next()
            .unwrap_or("")
            .trim_matches([',', '.', '!', '?', ';', ':', '-', '|']);
        if !has(
            EXCLAMATION_OPENS,
            head.trim_matches([',', '.', '!', '?', ';', ':', '-', '|']),
        ) || !(has(EXCLAMATION_SUBJECTS, next) || has(EXCLAMATION_OPENS, next))
        {
            break;
        }
        text = rest;
    }
    text
}

fn normalise_short_hello(text: &str) -> String {
    for &(typed, meant) in HELLO_SAID_SHORT {
        let Some(rest) = text.strip_prefix(typed) else {
            continue;
        };
        if rest.is_empty() {
            return meant.into();
        }
        if rest.starts_with([' ', ',', '!']) {
            let rest = rest
                .trim_matches([' ', ',', '!', '.'])
                .split(' ')
                .skip_while(|word| has(HELLO_TAIL, word.trim_matches([',', '.', '!'])))
                .collect::<Vec<_>>()
                .join(" ");
            let rest = rest.trim_matches([' ', ',', '!', '.']);
            return if rest.is_empty() {
                meant.to_owned()
            } else {
                rest.to_owned()
            };
        }
    }
    text.to_owned()
}

fn spell_chat_words(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    for word in text.split(' ').filter(|word| !word.is_empty()) {
        let said = get(SAID_OUT, word).unwrap_or(word);
        let said = if output
            .split(' ')
            .next_back()
            .is_some_and(|last| has(SUBJECT_SAID, last))
        {
            said.strip_prefix("i ")
                .or_else(|| said.strip_prefix("I "))
                .unwrap_or(said)
        } else {
            said
        };
        if !output.is_empty() {
            output.push(' ');
        }
        output.push_str(said);
    }
    output
}

fn remove_tags(clean: &str) -> (String, Vec<String>) {
    let mut words: Vec<_> = clean.split(' ').filter(|word| !word.is_empty()).collect();
    let trim = |word: &str| word.trim_matches(['.', ',', '!', '?']).to_owned();
    let asks = clean.contains('?')
        || words
            .first()
            .is_some_and(|word| has(TAG_QUESTION_LEADS, &trim(word)));
    let mut removed = Vec::new();
    while words.len() > 1 && has(TAG_WORDS, &trim(words[words.len() - 1])) {
        if has(TAG_GUARD_BEFORE, &trim(words[words.len() - 2])) {
            break;
        }
        removed.push(words.pop().unwrap().to_owned());
    }
    if asks {
        for (table, minimum) in [(TAG_PLEASE, 1), (LAUGHTER, 1), (TAG_THANKS, 3)] {
            while words.len() > minimum && has(table, &trim(words[words.len() - 1])) {
                removed.push(words.pop().unwrap().to_owned());
            }
        }
    }
    removed.reverse();
    (words.join(" "), removed)
}

fn spell_currency(text: &str) -> String {
    CURRENCY_SIGN
        .replace_all(text, |found: &Captures<'_>| {
            let matched = found.get(0).unwrap();
            if text[matched.end()..].starts_with(|ch: char| ch.is_alphanumeric() || ch == '_') {
                return matched.as_str().to_owned();
            }
            let amount = &found[2];
            let amount = if let (Some(comma), Some(period)) = (amount.rfind(','), amount.rfind('.'))
            {
                if comma > period {
                    amount.replace('.', "").replace(',', ".")
                } else {
                    amount.replace(',', "")
                }
            } else if let Some((_, tail)) = amount.rsplit_once(',') {
                if tail.len() == 3 {
                    amount.replace(',', "")
                } else {
                    amount.replace(',', ".")
                }
            } else {
                amount.to_owned()
            };
            let Ok(value) = amount.parse::<f64>() else {
                return matched.as_str().to_owned();
            };
            if !value.is_finite() {
                return matched.as_str().to_owned();
            }
            let value = if value.fract() == 0.0 {
                value
            } else {
                (value * 100.0).round() / 100.0
            };
            let factor = match found.get(3).map(|scale| scale.as_str()) {
                Some("k") => 1_000.0,
                Some("m") => 1_000_000.0,
                _ => 1.0,
            };
            let numeric = value * factor;
            let currency = match &found[1] {
                "£" => "quid",
                "$" => "dollars",
                _ => "euros",
            };
            if numeric.fract() == 0.0 {
                format!("{numeric:.0} {currency}")
            } else {
                format!("{numeric:.2} {currency}")
            }
        })
        .into_owned()
}

fn phrase_pass(said: &str) -> String {
    static FEELING: LazyLock<Vec<(ContextRegex, &str)>> = LazyLock::new(|| {
        [
            (A_HEAD_IN_THE_IDIOM_PATTERN, " my head"),
            (A_CLIPPED_HEAD_PATTERN, " my head"),
        ]
        .into_iter()
        .chain(FEELING_PHRASES.iter().copied())
        .map(|(pattern, replacement)| {
            (
                ContextRegex::new(pattern).expect("source phrase"),
                replacement,
            )
        })
        .collect()
    });
    pattern!(SAID_IT, SAID_IT_PATTERN);
    let mut text = said.to_owned();
    for (pattern, replacement) in &*FEELING {
        if let Ok(std::borrow::Cow::Owned(rewritten)) = pattern.try_replacen(&text, 0, *replacement)
        {
            text = rewritten;
        }
    }
    for (phrase, replacement) in PHRASES {
        if text.contains(phrase) {
            text = replace_bounded(&text, phrase, &format!(" {replacement} "), true);
        }
    }
    let text = UP_TOO_ASKS.replace_all(&text, "${1}up to");
    SAID_IT
        .replace_all(&text, |part: &Captures<'_>| {
            if let Some(who) = part.name("who") {
                return format!("what did {} say", who.as_str());
            }
            if part.name("saying").is_some_and(|who| who.as_str() == "i") {
                "what was i saying".into()
            } else {
                "what were we saying".into()
            }
        })
        .into_owned()
}

fn split_punctuation(text: &str) -> String {
    let marked = SENTENCE_END.replace_all(text, |part: &Captures<'_>| {
        let found = part.get(0).unwrap();
        if found.end() == text.len() || text[found.end()..].starts_with(is_space) {
            format!("{} | ", &found.as_str()[..2])
        } else {
            found.as_str().to_owned()
        }
    });
    PUNCT_BREAK.replace_all(&marked, " | ").into_owned()
}

fn join_chat_digits(tokens: &[&str]) -> Vec<String> {
    let mut joined = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let token = tokens[index];
        if let Some(next) = tokens.get(index + 1)
            && (next.starts_with(|letter: char| letter.is_ascii_digit()) || is_digits(token))
        {
            let candidate = format!("{token}{next}");
            if get(CHAT, &candidate).is_some() || get(MISSPELLINGS, &candidate).is_some() {
                joined.push(candidate);
                index += 2;
                continue;
            }
        }
        if let Some((word, length)) = spaced_abbreviation(&tokens[index..]) {
            joined.push(word);
            index += length;
        } else {
            joined.push(token.into());
            index += 1;
        }
    }
    joined
}

fn is_digits(word: &str) -> bool {
    !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_digit())
}

fn a_name_slot(word: &str, before: Option<&str>, before_before: Option<&str>) -> bool {
    if word.len() < 2 || !word.bytes().all(|byte| byte.is_ascii_alphabetic()) || has(LEXICON, word)
    {
        return false;
    }
    present(NAME_FRAMES, before)
        || present(BE_WORDS, before) && before_before == Some("name")
        || has(CHAT_NAMES, word)
            && (present(NAME_ROLL, before)
                || matches!(before, Some("me" | "you" | "u")) && present(NAME_CALLS, before_before)
                || before == Some("not")
                    && (present(BE_WORDS, before_before)
                        || matches!(before_before, Some("im" | "i"))))
        || has(COMMON_NAMES, word) && present(KIN, before) && present(POSSESSIVES, before_before)
}

fn said_apart(tokens: &[String]) -> Vec<String> {
    let mut combined = Vec::with_capacity(tokens.len());
    let mut index = 0;
    while index < tokens.len() {
        let pair = tokens.get(index..index + 2);
        let triple = tokens.get(index..index + 3);
        let matched = triple
            .and_then(|tokens| get(SAID_APART_FORMS, &tokens.join(" ")).map(|word| (word, 3)))
            .or_else(|| {
                pair.and_then(|tokens| {
                    get(SAID_APART_FORMS, &tokens.join(" ")).map(|word| (word, 2))
                })
            });
        if let Some((word, length)) = matched {
            if index > 0 && has(A_LETTER_AFTER, &tokens[index - 1]) {
                combined.extend_from_slice(&tokens[index..index + length]);
            } else {
                combined.push(word.to_owned());
            }
            index += length;
        } else {
            combined.push(tokens[index].clone());
            index += 1;
        }
    }
    combined
}

pub(crate) fn possessive_me(tokens: &mut [String]) {
    for index in 0..tokens.len().saturating_sub(1) {
        if tokens[index] != "me" {
            continue;
        }
        let thing = &tokens[index + 1];
        let before = index.checked_sub(1).map(|index| tokens[index].as_str());
        let previous = index.checked_sub(2).map(|index| tokens[index].as_str());
        let after = tokens.get(index + 2).map(String::as_str);
        let opens_phrase = before.is_none() || present(BEFORE_A_THING, before);
        let asks_whose = present(ASKS_WHOSE, before)
            || (present(BE_WORDS, before) && present(ASKS_WHOSE, previous));
        let owns_person = has(KIN, thing)
            && (after.is_none()
                || present(AFTER_A_THING, after)
                || present(BEFORE_A_THING, after)
                || present(TIME_WORDS, after)
                || present(PLACE_WORDS, after));
        let doing_something = has(DOING_NOT_THING, thing)
            || (thing.ends_with("ed") && thing.len() >= 5
                || thing.ends_with("ing") && thing.len() >= 6)
                && !has(ING_THINGS, thing);
        let owns_thing = present(AFTER_A_THING, after)
            || present(OWNING_VERBS, before)
            || owns_person
            || (opens_phrase && present(PAIN_WORDS, after))
            || (asks_whose
                && !has(NOT_A_THING_AFTER, thing)
                && !["ly", "ing", "ed"]
                    .iter()
                    .any(|suffix| thing.ends_with(suffix)))
            || (opens_phrase
                && !has(NOT_A_THING_AFTER, thing)
                && !thing.ends_with("ly")
                && !(present(BEFORE_A_PREPOSITION, before) && doing_something));
        let how_or_when = has(TIME_WORDS, thing)
            || has(PLACE_WORDS, thing)
            || present(TIME_WORDS, after)
            || after.is_some_and(|word| has(NOT_A_THING_PAIR, &format!("{thing} {word}")));
        if owns_thing
            && thing.len() > 2
            && thing.bytes().all(|byte| byte.is_ascii_alphabetic())
            && !has(NOT_A_THING, thing)
            && (owns_person || !how_or_when)
        {
            tokens[index] = "my".into();
        }
    }
}

fn positional<'a>(
    word: &'a str,
    before: Option<&str>,
    before_before: Option<&str>,
    next: &[String],
) -> Vec<&'a str> {
    let after = next.first().map(String::as_str);
    let further = next.get(1).map(String::as_str);
    let plain_before = before.map(plain_chat);
    let plain_after = after.map(plain_chat);
    match word {
        word if has(UP_TO_SPELLINGS, word)
            && (matches!(before, None | Some("|")) || present(UP_TO_LEADS, before))
            && (matches!(after, None | Some("|")) || present(UP_TO_TAGS, after))
            && matches!(further, None | Some("|")) =>
        {
            vec!["what", "you", "up", "to"]
        }
        word if has(WHAT_ARE_YOU_SPELLINGS, word) && present(WHAT_ARE_YOU_AFTER, after) => {
            vec!["what", "you"]
        }
        "w" => vec!["with"],
        "c" if present(LETTER_C_IS_SEE, after) => vec!["see"],
        "wers"
            if !matches!(before, None | Some("|"))
                && !present(LEADERS, before)
                && (present(WEARS_AFTER, plain_before) || is_person(before)) =>
        {
            vec!["wears"]
        }
        "wers" => vec!["where", "is"],
        "sum"
            if !present(SUM_THE_NOUN, plain_before)
                && !present(SUM_STAYS_AFTER, plain_after)
                && !matches!(plain_before, Some("in" | "to")) =>
        {
            vec!["some"]
        }
        "were" => read_were(before, after, further).unwrap_or_else(|| {
            if present(CONTRACTION_NEXT, after)
                && (matches!(before, None | Some("|"))
                    || present(LEADERS, before)
                        && (matches!(before_before, None | Some("|"))
                            || present(LEADERS, before_before)))
            {
                vec!["we", "are"]
            } else {
                vec![word]
            }
        }),
        "ill" if reads_as_will(word, before, after) => vec!["i", "will"],
        "rad" | "rads" if clipped_thing(plain_before, after, further) => {
            vec![if word == "rad" {
                "radiator"
            } else {
                "radiators"
            }]
        }
        "yr" | "hr" | "min" | "sec"
            if plain_before.is_some_and(|word| {
                is_digits(word) || has(A_COUNT, word) || has(A_PARTS_OF_A_PERIOD, word)
            }) =>
        {
            let unit = clipped_unit(word, plain_before, plain_after);
            vec![unit.unwrap_or(word)]
        }
        "wdyt" if !matches!(after, None | Some("|")) => vec!["what", "do", "you", "think"],
        "ow" if present(HOW_FOLLOWS, after) => vec!["how"],
        "bout" if !present(DETERMINERS, before) => vec!["about"],
        "no" if (present(KNOW_SUBJECT, before) || present(NOT_BEFORE_KNOW, before))
            && (matches!(after, None | Some("|"))
                || present(A_VERB_CAN_FOLLOW, after)
                || present(A_VERB_CAN_FOLLOW, after.and_then(|word| get(CHAT, word)))) =>
        {
            vec!["know"]
        }
        "hv" if !present(DETERMINERS, plain_before)
            && (present(HV_SUBJECTS, plain_before)
                || present(HV_HELPERS, plain_before)
                || present(HV_SUBJECTS, plain_after)
                || plain_after == Some("to")) =>
        {
            vec!["have"]
        }
        "bin" if bin_is_been(before, before_before, after, further) => vec!["been"],
        "of" if present(MODAL_FORMS, plain_before)
            && is_participle(plain_after)
            && !present(DETERMINERS, before_before) =>
        {
            vec!["have"]
        }
        "of" if after == Some("to")
            && (present(BE_FORMS, before) || present(BE_FORMS, before.map(last_chat))) =>
        {
            vec!["off"]
        }
        "b" if before.is_some_and(|word| {
            let tail = last_chat(word);
            has(MODAL_FORMS, tail)
                || has(BE_FORMS, tail)
                || has(HAVE_FORMS, tail)
                || matches!(tail, "to" | "not")
        }) =>
        {
            vec!["be"]
        }
        "mi" => vec![read_mi(before, after)],
        "veggies" if present(VEGGIES_AFTER, before) => vec!["vegetables"],
        "ur" | "your" if present(YOU_ARE_AFTER, after) => vec!["you're"],
        spelling if has(AINT_FORMS, spelling) => {
            let auxiliary = if is_participle(plain_after) || present(PAST_AS_PARTICIPLE, after) {
                if present(HAS_SUBJECT, before) {
                    "has"
                } else {
                    "have"
                }
            } else if plain_before == Some("i") {
                "am"
            } else if present(AINT_PLURAL, plain_before) {
                "are"
            } else {
                "is"
            };
            vec![auxiliary, "not"]
        }
        "yr" | "yrs" if before.is_some_and(is_digits) => {
            vec![if word == "yrs" || after == Some("old") {
                "years"
            } else {
                "year"
            }]
        }
        "2" | "4" => vec![digit_word(word, before, next)],
        _ => sounds_like_a_feeling(word, before, before_before, after)
            .map(|word| vec![word])
            .or_else(|| one_part(word, before, after).map(|word| vec![word]))
            .or_else(|| kin_helper(word, before, before_before, after, further))
            .or_else(|| family_short(word, before, after, further))
            .unwrap_or_else(|| vec![word]),
    }
}

fn sounds_like_a_feeling(
    word: &str,
    before: Option<&str>,
    previous: Option<&str>,
    after: Option<&str>,
) -> Option<&'static str> {
    let feeling = get(SOUNDS_LIKE_A_FEELING, word)?;
    let before = if present(BEFORE_A_FEELING, before) {
        previous
    } else {
        before
    };
    (present(FIRST_PERSON_BE, before)
        && (after.is_none() || present(A_FEELING_CAN_BE_FOLLOWED, after)))
    .then_some(feeling)
}

fn one_part<'a>(word: &'a str, before: Option<&str>, after: Option<&str>) -> Option<&'a str> {
    let part = word
        .strip_suffix('s')
        .filter(|part| part.len() >= 3 && has(ONE_OF_A_KIND, part))?;
    (before.is_some_and(|word| has(POSSESSIVES, &word.replace(['\'', '’'], "")))
        && (present(ONE_PART_AFTER, after)
            || after.is_some_and(|word| word.ends_with("ing") || word.ends_with("in"))))
    .then_some(part)
}

fn family_short(
    word: &str,
    before: Option<&str>,
    after: Option<&str>,
    further: Option<&str>,
) -> Option<Vec<&'static str>> {
    let family = get(FAMILY_SHRT, word)?;
    if !before.is_some_and(|word| has(POSSESSIVES, &word.replace(['\'', '’'], ""))) {
        return None;
    }
    let fence = match word {
        "ds" => DS_THINGS,
        "dd" => DD_THINGS,
        _ => &[],
    };
    if present(fence, after) || present(fence, further) {
        return None;
    }
    Some(family.split_whitespace().collect())
}

fn plain_chat(word: &str) -> &str {
    get(CHAT, word)
        .or_else(|| get(EXPAND, word))
        .unwrap_or(word)
        .split_whitespace()
        .next()
        .unwrap_or(word)
}

fn last_chat(word: &str) -> &str {
    get(CHAT, word)
        .or_else(|| get(EXPAND, word))
        .unwrap_or(word)
        .split_whitespace()
        .next_back()
        .unwrap_or(word)
}

fn is_doing(word: &str) -> bool {
    let word = word.replace('\'', "");
    if let Some(base) = word.strip_suffix("ing") {
        return has(LEXICON, &word) || has(LEXICON, base);
    }
    word.len() >= 6
        && word
            .strip_suffix("in")
            .is_some_and(|base| has(LEXICON, &word) || has(LEXICON, base))
}

fn read_were(
    before: Option<&str>,
    after: Option<&str>,
    further: Option<&str>,
) -> Option<Vec<&'static str>> {
    let after = after.filter(|word| *word != "|")?;
    if !matches!(before, None | Some("|")) {
        return None;
    }
    if has(A_QUESTION_VERB, after) {
        return Some(vec!["where"]);
    }
    let state = |word: &str| has(WERE_STATE, word) || has(WERE_DOING, word);
    if state(after)
        || has(WERE_ARE, after)
        || has(WERE_LEAD_IN, after) && further.is_some_and(|word| state(word) || is_doing(word))
        || !has(WERE_LEAD_IN, after)
            && (is_doing(after) && (further.is_none() || present(AFTER_A_DOING, further))
                || has(WERE_PLACE, after) && !present(WERE_IN_THE_PAST, further))
    {
        Some(vec!["we", "are"])
    } else if has(WERE_LEAD_IN, after)
        && matches!(
            further,
            Some(
                "i" | "you"
                    | "we"
                    | "they"
                    | "he"
                    | "she"
                    | "it"
                    | "them"
                    | "us"
                    | "him"
                    | "her"
                    | "me"
                    | "of"
            )
        )
    {
        Some(vec!["were"])
    } else {
        None
    }
}

fn read_mi(before: Option<&str>, after: Option<&str>) -> &'static str {
    if present(MI_IS_ME_AFTER, after) {
        return "me";
    }
    if present(MI_IS_A_NAME_AFTER, before) {
        return "mi";
    }
    if present(MI_IS_ME_BEFORE, after)
        || after == Some("back") && present(MI_BACK_VERBS, before)
        || matches!(after, None | Some("|"))
    {
        return "me";
    }
    if present(SOLFEGE, before) && present(SOLFEGE, after)
        || before.is_some_and(|word| word != "|" && !has(LEXICON, word))
    {
        "mi"
    } else {
        "my"
    }
}

fn clipped_thing(before: Option<&str>, after: Option<&str>, further: Option<&str>) -> bool {
    present(THING_AFTER, after)
        || (matches!(after, None | Some("|")) || present(THING_JOIN, after))
            && present(THING_SUBJECT, before)
        || present(THING_PLACE_AFTER, after)
            && present(THING_DETERMINER, before)
            && present(THING_PLACE_WORDS, further)
}

fn clipped_unit(word: &str, before: Option<&str>, after: Option<&str>) -> Option<&'static str> {
    if word == "hr" && present(HR_IS_THE_DEPARTMENT_AFTER, after) {
        return None;
    }
    let one = matches!(before, Some("1" | "a" | "an" | "one"))
        || present(A_PARTS_OF_A_PERIOD, before)
        || !matches!(after, None | Some("|")) && !present(NOT_BEFORE_A_NOUN, after);
    match (word, one) {
        ("yr", true) => Some("year"),
        ("yr", false) => Some("years"),
        ("hr", true) => Some("hour"),
        ("hr", false) => Some("hours"),
        ("min", true) => Some("minute"),
        ("min", false) => Some("minutes"),
        ("sec", true) => Some("second"),
        ("sec", false) => Some("seconds"),
        _ => None,
    }
}

fn kin_helper(
    word: &str,
    before: Option<&str>,
    previous: Option<&str>,
    after: Option<&str>,
    further: Option<&str>,
) -> Option<Vec<&'static str>> {
    let kin = get(KIN_SINGULAR, word)?;
    if before != Some("my") || !matches!(previous, None | Some("|")) && !present(LEADERS, previous)
    {
        return None;
    }
    if present(KIN_HAS, after) {
        return Some(vec![kin, "has"]);
    }
    if let Some(after) = after
        && (has(KIN_IS, after) || after == "not" || is_digits(after))
    {
        if after.len() >= 5
            && after.ends_with("ing")
            && !matches!(further, None | Some("" | "|"))
            && !present(CAN_FOLLOW_A_VERB, further)
        {
            return None;
        }
        return Some(vec![kin, "is"]);
    }
    if present(KIN_FILLERS, after)
        && (present(KIN_IS, further) || present(KIN_HAS, further) || further == Some("not"))
    {
        return Some(vec![
            kin,
            if present(KIN_HAS, further) {
                "has"
            } else {
                "is"
            },
        ]);
    }
    None
}

fn is_person(word: Option<&str>) -> bool {
    word.is_some_and(|word| {
        let bare = word.replace(['\'', '’'], "");
        has(PERSON_WORDS, &bare)
            || has(PERSON_WORDS, bare.trim_end_matches('s'))
            || has(PERSON_WORDS, plain_chat(word))
    })
}

fn is_participle(word: Option<&str>) -> bool {
    word.is_some_and(|word| word.ends_with("ed") || has(IRREGULAR_PARTICIPLES, word))
}

fn bin_is_been(
    before: Option<&str>,
    previous: Option<&str>,
    after: Option<&str>,
    further: Option<&str>,
) -> bool {
    if present(HAVE_FORMS, before) || before.is_some_and(|word| word.ends_with("ve")) {
        return true;
    }
    let plain = after.map(plain_chat);
    let state = plain.is_some_and(|word| {
        !present(PERSON_PREPS, after)
            && (word.len() > 3
                && ["ing", "ed", "in"]
                    .iter()
                    .any(|ending| word.ends_with(ending))
                || has(IRREGULAR_PARTICIPLES, word)
                || has(BEEN_WORDS, word))
    });
    if is_person(before) && (present(PERSON_PREPS, after) || is_participle(plain) || state) {
        return true;
    }
    if present(THING_POSSESSIVES, previous)
        && !present(BIN_CONTAINERS, before.map(plain_chat))
        && plain.is_some_and(|word| {
            has(BEEN_WORDS, word)
                || word.ends_with("ing")
                || word.ends_with("ed")
                || word != "in" && word.ends_with("in") && !has(IN_NOUNS, word)
        })
    {
        return true;
    }
    (present(BEEN_SUBJECT, before)
        || present(BE_FORMS, before)
        || matches!(before, None | Some("|")))
        && (state
            || after.is_some_and(is_digits)
            || present(PERIOD_COUNTS, plain)
            || present(PERIOD_COUNTS, further.map(plain_chat))
            || present(BEEN_SUBJECT, before) && matches!(after, Some("a" | "an")))
}
fn digit_word(digit: &str, before: Option<&str>, next: &[String]) -> &'static str {
    let after = next.first().map_or("|", String::as_str);
    let number = if digit == "2" { "two" } else { "four" };
    if matches!(after, "yr" | "yrs") {
        return number;
    }
    let before = before.unwrap_or("|");
    let spoken_before = get(CHAT, before)
        .or_else(|| get(EXPAND, before))
        .unwrap_or(before)
        .split(' ')
        .next_back()
        .unwrap_or("|");
    let after = get(CHAT, after)
        .unwrap_or(after)
        .split(' ')
        .next()
        .unwrap_or("|");
    let tail = next.get(1).map_or("|", String::as_str);
    let tail = get(CHAT, tail)
        .unwrap_or(tail)
        .split(' ')
        .next()
        .unwrap_or("|");
    if has(OPERATORS, after) || has(OPERATORS, before) {
        return number;
    }
    if has(COUNTS, after) && (has(BEFORE_VERB, before) || has(TOO_AFTER, before) || before == "|") {
        return if has(A_PERIOD_FOLLOWS, tail) || tail == "|" {
            number
        } else {
            "for"
        };
    }
    if has(A_LABEL_BEFORE, spoken_before) && !is_digits(after) {
        return number;
    }
    if digit == "2" && has(MOTION_BEFORE, before) && !has(A_PERIOD_FOLLOWS, after) {
        return "to";
    }
    if has(DEMONSTRATIVE, after) && has(A_PERIOD_FOLLOWS, tail) {
        return number;
    }
    if digit == "2" {
        if has(TO_AFTER, before) && (after == "|" || has(UP_TO_TAGS, after)) {
            return "to";
        }
        if after == "|" {
            return if has(TOO_AFTER, before) {
                "too"
            } else {
                number
            };
        }
        if has(TOO_A, after) {
            return "too";
        }
        if has(BEFORE_WORD, after) || has(BEFORE_VERB, after) {
            return "to";
        }
        if has(TOO_A_GRADED, after) && has(A_NEGATION_BEFORE, spoken_before) {
            return "too";
        }
        return number;
    }
    if has(BE_VERBS, before) {
        return number;
    }
    if has(FOR_A, after)
        || has(FOR_OBJECT, after) && (before != "|" || has(FOR_MEAL, after))
        || is_digits(after) && !is_digits(before)
    {
        return "for";
    }
    let counts = after == "|"
        || is_digits(after)
        || has(COUNT_UNITS, after)
        || has(NOT_COUNTED_AFTER, after)
        || after.len() > 2
            && after.ends_with('s')
            && !["ss", "us", "is"].iter().any(|tail| after.ends_with(tail));
    if has(COMPOUND_HEADS, after) && has(COMPOUND_TAILS, tail)
        || has(NUMBERS_BEFORE, before)
        || counts
    {
        number
    } else {
        "for"
    }
}

fn token(
    word: &str,
    before: Option<&str>,
    following: &[String],
    fixes: &mut BTreeMap<String, String>,
) -> Vec<String> {
    let next = following.first().map(String::as_str);
    if word == "cause" && cause_is_literal(before, next, following) {
        return vec![word.to_owned()];
    }
    if (present(POSSESSIVES, before) || present(A_THING_GOES_AFTER, before))
        && let Some(thing) = get(A_THING_TYPED_AS_A_LITTLE_WORD, word)
        && has(LEXICON, thing)
    {
        repair(fixes, word, thing);
        return vec![thing.to_owned()];
    }
    for (table, record) in [
        (DIGIT_WORDS, false),
        (CHAT, false),
        (EXPAND, false),
        (US_SPELLINGS, false),
        (MISSPELLINGS, false),
        (URGENT_MISSPELLINGS, true),
    ] {
        if let Some(expansion) = get(table, word) {
            if record {
                repair(fixes, word, expansion);
            }
            return expansion.split_whitespace().map(str::to_owned).collect();
        }
    }
    if let Some(expanded) = apostrophe(word) {
        return expanded;
    }
    if let Some(rest) = word.strip_prefix('4')
        && word.len() >= 4
        && rest.starts_with(['a', 'e', 'i', 'o', 'u', 'g'])
        && (has(LEXICON, rest) || get(CHAT, rest).is_some() || get(MISSPELLINGS, rest).is_some())
    {
        let replacement = format!("for{rest}");
        repair(fixes, word, &replacement);
        return vec![replacement];
    }
    if let Some(short) = collapse(word) {
        repair(fixes, word, &short);
        return vec![short];
    }
    if has(LEXICON, word) || word.len() < 2 || !word.bytes().all(|byte| byte.is_ascii_alphabetic())
    {
        return vec![word.to_owned()];
    }
    if word.len() >= 4 && word.ends_with(['s', 'z']) {
        let root = &word[..word.len() - 1];
        let opener = get(CHAT, root)
            .or_else(|| get(EXPAND, root))
            .or_else(|| get(MISSPELLINGS, root))
            .unwrap_or(root);
        if has(WH_OPENERS, opener) {
            repair(fixes, word, opener);
            return vec![opener.to_owned(), "is".into()];
        }
    }
    if let Some(repaired) = regular_typo(word) {
        repair(fixes, word, &repaired);
        return vec![repaired];
    }
    vec![word.to_owned()]
}

fn cause_is_literal(before: Option<&str>, after: Option<&str>, following: &[String]) -> bool {
    if present(CAUSE_BEFORE_A_VERB, before) || present(CAUSE_BEFORE_A_NOUN, before) {
        return true;
    }
    if present(CAUSE_A_DEMONSTRATIVE, after) {
        return following
            .get(1)
            .is_none_or(|word| !has(CAUSE_A_SUBJECT_FOLLOWS, word));
    }
    if present(CAUSE_A_SUBJECT_FOLLOWS, after) || after.is_none() {
        return false;
    }
    !(after == Some("the")
        && following
            .iter()
            .skip(1)
            .take(3)
            .any(|word| has(CAUSE_A_CLAUSE_VERB, word)))
}

fn apostrophe(word: &str) -> Option<Vec<String>> {
    let (base, suffix) = word.split_once('\'')?;
    if suffix == "t" && word.ends_with("n't") {
        let stem = match &word[..word.len() - 3] {
            "ca" => "can",
            "wo" => "will",
            "sha" => "shall",
            "ai" => "is",
            stem => stem,
        };
        return (!stem.is_empty()).then(|| vec![stem.to_owned(), "not".into()]);
    }
    let words = match suffix {
        "m" => vec![base, "am"],
        "re" => vec![base, "are"],
        "ve" => vec![base, "have"],
        "ll" => vec![base, "will"],
        "d" if has(PRONOUN_BASES, base) => vec![base, "would"],
        "d" => vec![base],
        "s" if has(PRONOUN_BASES, base) => vec![base, "is"],
        "s" => match get(CHAT, base)
            .or_else(|| get(MISSPELLINGS, base))
            .or_else(|| get(URGENT_MISSPELLINGS, base))
        {
            Some(spelled) if !spelled.contains(' ') => vec![spelled, "is"],
            _ => vec![base],
        },
        _ => return None,
    };
    Some(words.into_iter().map(str::to_owned).collect())
}

fn collapse(word: &str) -> Option<String> {
    let bytes = word.as_bytes();
    if !bytes
        .windows(3)
        .any(|run| run[0] == run[1] && run[1] == run[2])
    {
        return None;
    }
    let mut runs = Vec::new();
    let mut cursor = 0;
    while cursor + 1 < bytes.len() {
        if bytes[cursor] != bytes[cursor + 1] {
            cursor += 1;
            continue;
        }
        let start = cursor;
        cursor += bytes[cursor..]
            .iter()
            .take_while(|&&byte| byte == bytes[start])
            .count();
        runs.push((start, cursor));
    }
    if runs.len() > 3 {
        return None;
    }
    fn settle(
        word: &str,
        runs: &[(usize, usize)],
        start: usize,
        out: &mut String,
    ) -> Option<String> {
        let Some((&(from, to), rest)) = runs.split_first() else {
            out.push_str(&word[start..]);
            return has(LEXICON, out).then(|| out.clone());
        };
        out.push_str(&word[start..from]);
        let before_run = out.len();
        for length in 1..=(to - from).min(3) {
            out.truncate(before_run);
            out.push_str(&word[from..from + length]);
            if let Some(candidate) = settle(word, rest, to, out)
                && candidate != word
            {
                return Some(candidate);
            }
        }
        None
    }
    settle(word, &runs, 0, &mut String::new())
}

fn regular_typo(word: &str) -> Option<String> {
    if has(TYPED_WORDS, word) || has(COMMON_NAMES, word) || has(NOT_A_FEELING, word) {
        return None;
    }
    if let Some(prefix) = word.strip_suffix('z')
        && word.len() >= 4
    {
        let plural = format!("{prefix}s");
        if has(LEXICON, &plural) {
            return Some(plural);
        }
    }
    for tail in ["aike", "ayke", "eake"] {
        if let Some(prefix) = word.strip_suffix(tail)
            && !prefix.is_empty()
        {
            let candidate = format!("{prefix}ache");
            if has(LEXICON, &candidate) {
                return Some(candidate);
            }
        }
    }
    spelling::repair_typo(word).or_else(|| spelling::said_with_f(word))
}

fn opens_without_subject(before: &[String], word: &str, next: &[String]) -> bool {
    if !has(DROPPED_SUBJECT_AUX, word)
        || !before
            .iter()
            .rev()
            .take_while(|word| word.as_str() != "|")
            .all(|word| has(LEADERS, word))
    {
        return false;
    }
    let clause: Vec<_> = next
        .iter()
        .take(3)
        .take_while(|word| word.as_str() != "|")
        .map(|word| {
            token(word, None, &[], &mut BTreeMap::new())
                .into_iter()
                .next()
                .unwrap_or_else(|| word.clone())
        })
        .collect();
    let Some(after) = clause.first() else {
        return false;
    };
    if has(SUBJECTS, after) {
        return false;
    }
    if word == "am" || has(PREDICATE_ONLY, after) {
        return true;
    }
    if has(SUBJECTLESS_ONLY, word)
        && let Some(index) = clause.iter().position(|word| !has(STATE_FILLER, word))
        && has(A_STATE_OF_THE_PERSON, &clause[index])
        && clause
            .get(index + 1)
            .is_none_or(|word| has(STATE_TAIL, word) || has(STATE_FILLER, word))
    {
        return true;
    }
    if word == "have" {
        if has(HAVE_NEGATIONS, after) {
            return true;
        }
        if has(HAVE_PARTICIPLES, after)
            && let [_, object, description, ..] = clause.as_slice()
        {
            return has(HAVE_THING, object)
                && !has(HAVE_ASKING, description)
                && !clause[1..].iter().any(|word| has(TIME_WORDS, word));
        }
    }
    if after.len() <= 4 || !(after.ends_with("ing") || after.ends_with("ed")) {
        return false;
    }
    let Some(further) = clause.get(1) else {
        return true;
    };
    has(AFTER_PARTICIPLE, further)
        && (has(SUBJECTLESS_ONLY, word)
            || clause
                .get(2)
                .is_some_and(|word| word.ends_with("ing") || word.ends_with("ed")))
}

fn one_relation_says(tokens: &[String]) -> Vec<String> {
    let mut out = Vec::with_capacity(tokens.len());
    for (index, word) in tokens.iter().enumerate() {
        if let Some(relation) = word
            .strip_suffix('s')
            .filter(|word| has(ONE_RELATION, word))
            && let Some(before) = index.checked_sub(1).map(|index| &tokens[index])
            && has(POSSESSIVES, &before.replace(['\'', '’'], ""))
            && let Some(next) = tokens.get(index + 1).map(|word| plain_chat(word))
            && (has(ONE_RELATION_VERBS, next)
                || has(HAVE_FORMS, next)
                || has(BE_FORMS, next)
                || has(MODAL_FORMS, next))
        {
            out.push(relation.to_owned());
            if matches!(next, "got" | "bin" | "had") {
                out.push("has".into());
            }
        } else {
            out.push(word.clone());
        }
    }
    out
}

fn pretoken_context(tokens: &[String]) -> Vec<String> {
    tokens
        .iter()
        .enumerate()
        .map(|(index, word)| {
            let before = index.checked_sub(1).map(|index| tokens[index].as_str());
            let earlier = index.checked_sub(2).map(|index| tokens[index].as_str());
            let after = tokens.get(index + 1).map(String::as_str);
            if let Some(fixed) = get(LOOSE_FOR_LOSE, word) {
                for token in tokens.iter().skip(index + 1).take(4) {
                    if has(THE_LOSING, token) {
                        return fixed.to_owned();
                    }
                    if !has(BEFORE_THE_LOSING, token) && !is_digits(token) {
                        break;
                    }
                }
            }
            if word == "sor"
                && (present(SORE_IN_FRONT, before)
                    || present(SORE_FILLERS, before) && present(SORE_IN_FRONT, earlier)
                    || present(SORE_PARTS, after))
            {
                return "sore".into();
            }
            if matches!(word.as_str(), "sun" | "suns")
                && present(OWNING_A_PERSON, before)
                && !present(SUN_WORDS, after)
                && present(SON_VERBS, after)
            {
                return if word == "sun" { "son" } else { "sons" }.into();
            }
            word.clone()
        })
        .collect()
}

fn contextual_slips(tokens: &mut Vec<String>) -> BTreeMap<String, String> {
    let mut fixes = BTreeMap::new();
    let mut output = tokens.clone();
    let any = |table| tokens.iter().any(|word| has(table, word));
    for (index, word) in tokens.iter().enumerate() {
        let before = index.checked_sub(1).map(|index| tokens[index].as_str());
        let earlier = index.checked_sub(2).map(|index| tokens[index].as_str());
        let after = tokens.get(index + 1).map(String::as_str);
        let further = tokens.get(index + 2).map(String::as_str);
        let fix = match word.as_str() {
            spelling if get(A_PAIN_WORD, spelling).is_some() => {
                let storage = present(DETERMINERS, before) && present(A_KEEPING_A_THING, earlier);
                let body = present(A_PART_THE_PAIN_IS_IN, before)
                    || present(A_PART_A_PERSON_OWNS, before) && present(OWNING_A_PERSON, earlier);
                let feels = present(A_STATE_YOU_CAN_FEEL, after)
                    || present(A_COPULA, after)
                    || matches!(
                        after,
                        Some(
                            "hurt"
                                | "hurts"
                                | "ache"
                                | "aches"
                                | "aching"
                                | "kill"
                                | "killing"
                                | "throb"
                                | "throbbing"
                        )
                    );
                let located = after == Some("in")
                    && (index + 2..(index + 5).min(tokens.len())).any(|position| {
                        has(A_PART_THE_PAIN_IS_IN, &tokens[position])
                            || has(A_PART_A_PERSON_OWNS, &tokens[position])
                                && has(OWNING_A_PERSON, &tokens[position - 1])
                    });
                if !storage && !any(A_GLASS_OR_A_PAN) && (body && feels || located) {
                    get(A_PAIN_WORD, spelling)
                } else {
                    None
                }
            }
            "hart" if any(THE_HEART_SAID_THE_OTHER_WAY) => Some("heart"),
            "chocked" | "chockes"
                if !present(DETERMINERS, after) && any(A_CHOKE_SAID_THE_OTHER_WAY) =>
            {
                Some(if word == "chocked" {
                    "choked"
                } else {
                    "chokes"
                })
            }
            "ware" | "wared" if any(A_THING_WORN) => {
                Some(if word == "ware" { "wear" } else { "wore" })
            }
            "desert" if before == Some("for") => Some("dessert"),
            "live"
                if present(BEFORE_A_JOB, after)
                    && present(A_JOB, further)
                    && (matches!(tokens.get(index + 3).map(String::as_str), None | Some("|"))
                        || present(A_TIME_TO_LEAVE, tokens.get(index + 3).map(String::as_str))) =>
            {
                Some("leave")
            }
            "saw" if present(A_COPULA, before) && any(A_PART_THE_PAIN_IS_IN) => Some("sore"),
            "felling" if present(A_STATE_YOU_CAN_FEEL, after) => Some("feeling"),
            _ => None,
        };
        if let Some(fix) = fix {
            output[index] = fix.to_owned();
            repair(&mut fixes, word, fix);
        }
    }
    *tokens = output;
    fixes
}

fn fix_auxiliaries(words: &mut Vec<String>) {
    for index in 1..words.len().saturating_sub(1) {
        if words[index] == "is"
            && has(HAS_SUBJECTS, &words[index - 1])
            && (has(HAS_AFTER, &words[index + 1])
                || words[index + 1] == "gone" && index + 2 < words.len())
        {
            words[index] = "has".into();
        }
    }
    let mut at = 1;
    while at < words.len() {
        if words[at] != "been" {
            at += 1;
            continue;
        }
        let last = &words[at - 1];
        let earlier = at.checked_sub(2).map(|index| words[index].as_str());
        if has(BE_FORMS, last) {
            words[at - 1] = if present(PLURAL_SUBJECTS, earlier) {
                "have"
            } else {
                "has"
            }
            .into();
        } else if (has(PLURAL_SUBJECTS, last) || has(THIRD_PERSON, last))
            && !present(HAVE_FORMS, earlier)
            && !present(BE_FORMS, earlier)
            && !present(MODAL_FORMS, earlier)
            && earlier != Some("not")
        {
            let auxiliary = if has(PLURAL_SUBJECTS, last) {
                "have"
            } else {
                "has"
            };
            words.insert(at, auxiliary.into());
            at += 1;
        }
        at += 1;
    }
}

fn split_and_focus(
    norm: &[String],
    fixes: &mut BTreeMap<String, String>,
) -> (Vec<Vec<String>>, Option<usize>) {
    let mut clauses = Vec::new();
    let mut current = Vec::new();
    for (index, word) in norm.iter().enumerate() {
        if word == "|"
            || has(BREAKS, word)
                && current.len() >= 2
                && norm.get(index + 1).is_some_and(|word| {
                    [CLAUSE_OPENERS, SUBJECT_WORDS, WH_WORDS, BREAKS, LEADERS]
                        .into_iter()
                        .any(|table| has(table, word))
                })
        {
            if !current.is_empty() {
                clauses.push(std::mem::take(&mut current));
            }
        } else {
            current.push(word.clone());
        }
    }
    if !current.is_empty() {
        clauses.push(current);
    }

    let mut split = Vec::new();
    for mut clause in clauses {
        for start in [0, clause.len() - strip_leaders(&clause).len()] {
            if let Some(word) = clause.get(start)
                && !has(BASE, word)
                && [CHAT, MISSPELLINGS, EXPAND]
                    .into_iter()
                    .all(|table| get(table, word).is_none())
                && let Some(opener) = repair_opener(word)
            {
                repair(fixes, word, opener);
                clause.splice(start..=start, token(opener, None, &[], fixes));
                break;
            }
        }
        if let Some(at) = ask_start(&clause).filter(|&at| at < clause.len() - 1) {
            let after = clause.split_off(at);
            split.push(clause);
            split.push(after);
        } else {
            split.push(clause);
        }
    }
    let focus = split
        .iter()
        .rposition(|clause| {
            strip_leaders(clause)
                .first()
                .is_some_and(|word| has(ASK_WORDS, word))
        })
        .or_else(|| {
            split
                .iter()
                .rposition(|clause| clause.iter().any(|word| has(WANT_WORDS, word)))
        })
        .or_else(|| split.len().checked_sub(1));
    (split, focus)
}

pub(crate) fn strip_leaders(clause: &[String]) -> &[String] {
    let mut start = 0;
    while start + 1 < clause.len() && has(LEADERS, &clause[start]) {
        start += 1;
    }
    &clause[start..]
}

/// Remove conversational openers while preserving the text that follows.
pub(crate) fn without_leaders(mut text: &str) -> &str {
    static LEADER: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^\s*([A-Za-z']+)\s*[,.;:!\-—]*\s*").unwrap());
    while let Some(capture) = LEADER.captures(text) {
        if !has(LEADERS, &capture[1].to_ascii_lowercase()) {
            break;
        }
        let next = &text[capture.get(0).unwrap().end()..];
        if next.chars().all(is_space) {
            break;
        }
        text = next;
    }
    text
}

fn repair_opener(word: &str) -> Option<&'static str> {
    if word.len() < 2 || !word.bytes().all(|byte| byte.is_ascii_alphabetic()) {
        return None;
    }
    let doubled = word
        .as_bytes()
        .windows(2)
        .last()
        .is_some_and(|last| last[0] == last[1]);
    let candidates: Vec<_> = OPENERS
        .iter()
        .copied()
        .filter(|opener| {
            (word.len() != 2 || has(WH_OPENERS, opener))
                && (word.len() < opener.len() || doubled)
                && word.len().abs_diff(opener.len()) == 1
                && word[..1] == opener[..1]
                && super::normalize::one_edit(word, opener)
        })
        .collect();
    if let [one] = candidates.as_slice() {
        return Some(one);
    }
    for end in [
        word.as_bytes()[word.len() - 1],
        word.as_bytes()[word.len() - 2],
    ] {
        let mut matching = candidates
            .iter()
            .copied()
            .filter(|candidate| candidate.as_bytes().last() == Some(&end));
        if let Some(candidate) = matching.next()
            && matching.next().is_none()
        {
            return Some(candidate);
        }
        if !doubled {
            break;
        }
    }
    None
}

fn ask_start(clause: &[String]) -> Option<usize> {
    if clause
        .first()
        .is_none_or(|word| has(WH_WORDS, word) || has(AUX_WORDS, word))
    {
        return None;
    }
    for index in 1..clause.len().saturating_sub(1) {
        let word = &clause[index];
        let next = &clause[index + 1];
        if has(WH_WORDS, word) {
            let across = clause[index + 1..]
                .iter()
                .take(5)
                .position(|piece| has(AUX_WORDS, piece))
                .is_some_and(|offset| {
                    let crossing = clause[index + 1..index + 1 + offset].iter().filter(|word| {
                        !has(WH_QUANTIFIERS, word) && !matches!(word.as_str(), "of" | "for")
                    });
                    crossing.clone().count() <= 2
                        && !crossing
                            .clone()
                            .any(|word| has(NOT_IN_A_QUESTION_PHRASE, word))
                        && clause
                            .get(index + 2 + offset)
                            .is_some_and(|word| has(AFTER_AN_INVERTED_AUX, word))
                });
            let asking = has(AUX_WORDS, next)
                || across
                || has(HOW_DEGREE, next)
                || has(ASK_NOUNS, next)
                    && clause
                        .get(index + 2)
                        .is_some_and(|word| has(AUX_WORDS, word));
            if asking && !reported(clause, index) {
                return Some(index);
            }
        } else if has(AUX_WORDS, word)
            && has(SUBJECT_WORDS, next)
            && !has(NO_ASK_AFTER, &clause[index - 1])
            && !has(SUBJECT_WORDS, &clause[index - 1])
            && !has(BE_WORDS, word)
        {
            return Some(index);
        }
    }
    None
}

fn reported(clause: &[String], index: usize) -> bool {
    let before = &clause[index.saturating_sub(2)..index];
    let latest = &clause[index - 1];
    if before.iter().any(|word| has(REPORTING, word)) {
        return true;
    }
    let inverted = clause
        .get(index + 1)
        .is_some_and(|word| has(RELATIVE_ASKS, word))
        && clause
            .get(index + 2)
            .is_some_and(|word| has(SUBJECT_WORDS, word));
    if has(RELATIVE, &clause[index])
        && !inverted
        && !has(AUX_WORDS, latest)
        && !has(WH_WORDS, latest)
        && (has(DETERMINERS, latest)
            || before.len() > 1
                && (has(DETERMINERS, &before[0]) || has(NOMINAL_BEFORE_COPULA, &before[0])))
    {
        return true;
    }
    has(COPULAS, latest) && (before.len() < 2 || has(NOMINAL_BEFORE_COPULA, &before[0]))
}

pub(super) fn is_laugh(word: &str) -> bool {
    !has(NOT_A_LAUGH, word)
        && (has(LAUGHTER, word) || (word.len() > 2 || word == "hh") && LAUGH_RUN.is_match(word))
}

fn only_doubles_a_letter(typed: &str, repaired: &str) -> bool {
    repaired.len() == typed.len() + 1
        && repaired
            .as_bytes()
            .windows(2)
            .enumerate()
            .any(|(at, pair)| {
                pair[0] == pair[1]
                    && typed.get(..at) == repaired.get(..at)
                    && typed.get(at..) == repaired.get(at + 1..)
            })
}

fn keeps_spelling(words: &[String], fixed: &BTreeMap<String, String>) -> bool {
    if fixed.is_empty() {
        return false;
    }
    if words.iter().enumerate().any(|(index, word)| {
        has(SPELLING_ASK, word)
            || fixed.contains_key(word)
                && NAMED_LETTER.is_match(
                    &words[index.saturating_sub(4)..(index + 5).min(words.len())].join(" "),
                )
    }) {
        return true;
    }
    words.windows(3).any(|span| {
        if span[1] != "or" {
            return false;
        }
        [(&span[0], &span[2]), (&span[2], &span[0])]
            .into_iter()
            .any(|(typed, other)| {
                fixed.get(typed).is_some_and(|repaired| {
                    repaired == other || typed == other || {
                        let (short, long) = if repaired.len() <= other.len() {
                            (repaired, other)
                        } else {
                            (other, repaired)
                        };
                        short.len() >= 4
                            && long
                                .strip_prefix(short.as_str())
                                .is_some_and(|ending| has(SPELLING_ENDINGS, ending))
                    }
                })
            })
    })
}

fn short_word_named(short: &str, before: Option<&str>, after: Option<&str>) -> bool {
    present(A_SHORT_WORD_IS_NAMED_BEFORE, before)
        || present(A_SHORT_WORD_IS_NAMED_AFTER, after)
        || (short == "u" && after.is_some_and(|word| word.ends_with("'s")))
}

#[cfg(test)]
mod tests {
    use super::normalise;

    #[test]
    fn keeps_surface_and_expanded_text_separate() {
        let line = normalise("can’t you see it?");
        assert_eq!(line.clean, "can’t you see it?");
        assert_eq!(line.words, ["can", "t", "you", "see", "it"]);
        assert_eq!(line.plain, "cant you see it");
        assert_eq!(line.norm_text, "can not you see it");
        assert!(line.signals.question_mark);
    }

    #[test]
    fn contextual_typo_reaches_readers_but_a_spelling_question_keeps_the_original() {
        let repaired = normalise("my mum collpsd");
        assert_eq!(repaired.words, ["my", "mum", "collapsed"]);
        assert_eq!(
            repaired.fixes.get("collpsd").map(String::as_str),
            Some("collapsed")
        );

        let question = normalise("is neccessary one c or two");
        assert_eq!(question.words[1], "neccessary");
        assert_eq!(question.norm[1], "necessary");
    }

    #[test]
    fn recognises_a_day_belonging_to_the_named_relation() {
        let line = normalise("its my sons 5th not my birthday");
        assert_eq!(line.clean, "its my sons 5th birthday");
        assert_eq!(line.words, ["its", "my", "sons", "5th", "birthday"]);
    }

    #[test]
    fn retains_the_original_intensifier_and_reads_storage_duration() {
        let line = normalise("What the hell is this?");
        assert_eq!(line.intensifier, ["the", "hell"]);
        assert_eq!(line.intensifier_left, ["the", "hell"]);
        assert_eq!(line.norm_text, "what is this");

        let line = normalise("Is it ok to eat rice that's been in the fridge 3 days?");
        let kept = line.kept.expect("storage duration");
        assert_eq!(kept.place.as_deref(), Some("fridge"));
        assert_eq!(kept.time, "three days");
    }
}
