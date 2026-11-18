//! Question, greeting and courtesy classification for a normalized turn.
#[path = "kinds_data.rs"]
mod data;
use data::*;

#[path = "kinds_features.rs"]
mod feature;
pub(in crate::bot) use feature::claim;
pub(crate) use feature::{
    AboutBotKind, Intent, KindFeatures, StateOwner, a_stop_ask, broken_heart, cue_about_person,
};
pub(super) use feature::{gratitude_span, remove_background_labels};

#[path = "kinds_extra.rs"]
mod extra;
pub(crate) use extra::{
    CareHome, CareShape, Interest, Shift, a_care_home, a_home_carried_forward, asks_how_i_am,
    asks_with_nothing_named, is_a_routine, opening_line, setup, shift_worked, taking_up,
    talk_frame,
};

#[path = "kinds_time.rs"]
mod time;
pub(crate) use time::{TimeDependence, over_time, over_time_followup};

#[path = "kinds_names.rs"]
mod names;

use super::{intent::Asks, line::Line};
use fancy_regex::Regex as ContextRegex;
use regex::Regex;
use std::sync::LazyLock;
static HELP_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:anything|something)\s+(?:else\s+)?(?:i|we|you)\s+(?:can|could|should|might|would)\s+(?:do|try|use|take|add|change)\b|\b(?:anything|something)\s+(?:else\s+)?(?:that|which)\s+(?:helps|would help|might help|will help|works)\s*[?.!]*\s*$|\bany\s+(?:way|ways)\s+(?:of|to|round)\b|\bis there any\s+(?:way|ways|thing)\s+(?:of|to|i|we|you|else)\b"#).unwrap()
});
static HELP_ASK_SUBJECT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:is|was|are|were|seems|seemed|feels|felt|sounds|sounded|helps|helped|works|worked|suits|suited|fits|fitted|does|did|its|thats|im|youre|(?:would|will|might|must|could|can|may) be)\b"#).unwrap()
});
static HELP_ASK_TAIL_CLAUSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\|\s*\w+\s+\w+"#).unwrap());
static HELP_ASK_NOT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\b(?:no|not|isnt|isn't|wasnt|wasn't|arent|aren't|cant|can't|couldnt|couldn't|dont|don't|doesnt|doesn't|didnt|didn't|never|nothing|without|hardly)\b"#).unwrap()
});
static HELP_ASK_OWN_SUBJECT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\b(?:my|our|his|her|their|your|the|a|an)\s+[a-z]"#).unwrap());

static STILL_WRONG: LazyLock<ContextRegex> =
    LazyLock::new(|| ContextRegex::new(STILL_WRONG_PATTERN).expect("thanks fault shape"));
static PRAISE_SAID: LazyLock<ContextRegex> =
    LazyLock::new(|| ContextRegex::new(PRAISE_SAID_PATTERN).expect("thanks praise shape"));
static SIGNING_OFF: LazyLock<ContextRegex> =
    LazyLock::new(|| ContextRegex::new(SIGNING_OFF_PATTERN).expect("farewell shape"));

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Kind {
    Empty,
    Greeting,
    Thanks,
    Question,
    Statement,
}

fn has(set: &[&str], word: &str) -> bool {
    set.binary_search(&word).is_ok()
}
fn starts(words: &[impl AsRef<str>], prefix: &str) -> bool {
    let n = prefix.split_whitespace().count();
    words.len() >= n
        && words
            .iter()
            .take(n)
            .map(AsRef::as_ref)
            .eq(prefix.split_whitespace())
}
fn after<'a, T: AsRef<str>>(words: &'a [T], phrases: &[&str]) -> Option<&'a [T]> {
    let longest = phrases
        .iter()
        .filter(|p| starts(words, p))
        .max_by_key(|p| p.len())?;
    Some(&words[longest.split_whitespace().count()..])
}

fn without_courtesy_filler<T: AsRef<str>>(words: &[T]) -> &[T] {
    match words {
        [first, rest @ ..] if !rest.is_empty() && has(BEFORE_COURTESY, first.as_ref()) => rest,
        _ => words,
    }
}

/// Words following an initial thank-you, including an empty tail for a bare courtesy.
pub(crate) fn courtesy_tail(norm: &[String]) -> Option<&[String]> {
    let rest = after(without_courtesy_filler(norm), THANKS)?;
    (!rest.first().is_some_and(|word| has(COURTESY_THING, word))).then_some(rest)
}
fn asks_help(text: &str) -> bool {
    !HELP_ASK_OWN_SUBJECT.is_match(text)
        && HELP_ASK.find_iter(text).any(|m| {
            !HELP_ASK_NOT.is_match(&text[..m.start()])
                && !HELP_ASK_SUBJECT.is_match(&text[m.end()..])
                && !HELP_ASK_TAIL_CLAUSE.is_match(&text[m.end()..])
        })
}

fn without_subject(words: &[&str]) -> bool {
    let Some((&aux, tail)) = words.split_first() else {
        return false;
    };
    if !has(AUX_OPENS, aux) {
        return false;
    }
    let rest: Vec<_> = tail.iter().copied().filter(|w| *w != "|").collect();
    let Some(&second) = rest.first() else {
        return false;
    };
    if has(SUBJECTS, second) {
        return false;
    }
    if aux == "am" || has(PREDICATE_ONLY, second) {
        return true;
    }
    if has(SUBJECTLESS_AUX, aux) {
        let mut states = tail
            .iter()
            .copied()
            .take_while(|w| *w != "|")
            .skip_while(|w| has(STATE_FILLER, w));
        if states.next().is_some_and(|w| has(A_STATE_OF_THE_PERSON, w))
            && states
                .next()
                .is_none_or(|w| has(STATE_TAIL, w) || has(STATE_FILLER, w))
        {
            return true;
        }
    }
    if aux == "have" {
        if has(HAVE_NEGATIONS, second) {
            return true;
        }
        if has(HAVE_PARTICIPLES, second) && rest.len() > 2 {
            return has(HAVE_THING, rest[1])
                && !has(HAVE_ASKING, rest[2])
                && !rest[1..].iter().any(|w| has(TIME_WORDS, w));
        }
    }
    let participle = |w: &str| w.ends_with("ing") || w.ends_with("ed");
    if second.len() > 4 && participle(second) {
        if rest.len() < 2 {
            return true;
        }
        return has(AFTER_PARTICIPLE, rest[1])
            && (has(SUBJECTLESS_ONLY, aux) || rest.get(2).is_some_and(|w| participle(w)));
    }
    false
}
fn opens_question(mut words: &[&str]) -> bool {
    while words.len() > 1 && has(LEADERS, words[0]) {
        words = &words[1..]
    }
    let Some((&head, rest)) = words.split_first() else {
        return false;
    };
    if let [verb, tail @ ..] = rest
        && has(ASK_THE_ROOM, head)
        && has(ASK_THE_ROOM_VERB, verb)
        && (has(ASK_THE_ROOM_ANY, head)
            || has(ASK_FOR_A_NAME, verb)
                && tail.first().is_some_and(|w| has(ASK_THE_ROOM_WANTS, w)))
    {
        return true;
    }
    if !has(QUESTION_STARTS, head) {
        return false;
    }
    if let [second, third, ..] = rest
        && has(NEGATIONS, second)
        && !has(SUBJECT_NEGATIONS, second)
        && !has(PRONOUNS, third)
    {
        return false;
    }
    if let Some(&second) = rest.first() {
        if head == "do" && has(QUESTION_WORDS, second) {
            return false;
        }
        if has(YESNO_STARTS, head)
            && (!second.ends_with("ing") || has(PAST_AUX, head))
            && has(VERB_FORMS, second)
        {
            return false;
        }
    }
    !without_subject(words)
}
fn subject(clause: &[&str]) -> bool {
    let Some(first) = clause.first() else {
        return false;
    };
    !clause
        .iter()
        .any(|w| has(VERBISH, w) || w.len() > 3 && (w.ends_with("ed") || w.ends_with("ing")))
        && (has(DETERMINERS, first) || clause.len() == 1 && !has(NOT_A_SUBJECT, first))
}
fn continues(clauses: &[Vec<&str>], i: usize) -> bool {
    if i == 0 || !subject(&clauses[i - 1]) {
        return false;
    }
    let words = &clauses[i];
    let Some(&head) = words.first() else {
        return false;
    };
    if words
        .iter()
        .skip(1)
        .take(2)
        .any(|w| has(SUBJECT_OF_ITS_OWN, w))
    {
        return false;
    }
    has(WH_CONTINUES, head) || has(YESNO_STARTS, head) && i >= 2 && subject(&clauses[i - 2])
}
fn agrees(rest: &[&str]) -> bool {
    if let Some(at) = rest.iter().position(|w| *w == "not") {
        let mut tail = rest[at + 1..]
            .iter()
            .copied()
            .skip_while(|w| has(NOT_FILLER, w))
            .peekable();
        return tail.peek().is_some_and(|w| has(AGREE_AFTER_NOT, w))
            && tail.all(|w| has(AGREE_TAIL, w) || has(AGREE_AFTER_NOT, w));
    }
    rest.iter().all(|w| has(AGREE_TAIL, w))
}
fn ends_with_thanks(words: &[&str]) -> bool {
    let end = words
        .iter()
        .rposition(|w| !has(ADDRESSED, w))
        .map_or(0, |i| i + 1);
    let line = &words[..end];
    THANKS.iter().any(|phrase| {
        let pat = phrase.split_whitespace().collect::<Vec<_>>();
        if line.len() <= pat.len() || !line.ends_with(&pat) {
            return false;
        }
        let rest = line[..line.len() - pat.len()]
            .iter()
            .copied()
            .filter(|w| !has(THANKS_NOISE, w))
            .collect::<Vec<_>>();
        rest.is_empty()
            || rest.len() <= 6
                && !(end == words.len() && rest.iter().all(|w| has(AFFIRM, w) || has(ACK, w)))
                && !matches!(rest[0], "no" | "nope" | "nah" | "never" | "naw")
                && agrees(&rest)
    })
}
fn helped(mut words: &[&str]) -> bool {
    while words.len() > 2 && has(BEFORE_HELP_WAS_GOOD, words[0]) {
        words = &words[1..]
    }
    while words.len() > 2 {
        let cut = AFTER_HELP_WAS_GOOD_SAID_OUT
            .iter()
            .find_map(|p| {
                let tail = p.split_whitespace().collect::<Vec<_>>();
                words.ends_with(&tail).then_some(tail.len())
            })
            .or_else(|| has(AFTER_HELP_WAS_GOOD, words.last()?).then_some(1));
        let Some(n) = cut else { break };
        if n + 2 > words.len() {
            break;
        }
        words = &words[..words.len() - n];
    }
    has(HELP_WAS_GOOD, &words.join(" "))
}
fn greeting_only(rest: &[&str]) -> bool {
    if rest.iter().all(|w| has(GREETING_TAIL, w) || has(POLITE, w)) {
        return true;
    }
    if rest.len() <= 2 && !rest.iter().any(|w| has(NEEDS_ANSWER, w)) {
        return true;
    }
    after(rest, INTRODUCES).is_some_and(|tail| {
        let mut names = tail.iter().filter(|w| !has(NAMING_WORDS, w));
        (1..=2).contains(&names.clone().count()) && names.all(|word| names::reads_as_a_name(word))
    })
}
/// Subject following the link in a turn shaped like "any word on the job".
pub(crate) fn after_any_link<T: AsRef<str>>(words: &[T]) -> &[T] {
    let [head, noun, tail @ ..] = words else {
        return &[];
    };
    if !matches!(head.as_ref(), "any" | "anything") || !has(ANY_ANSWER_WANTED, noun.as_ref()) {
        return &[];
    }
    tail.iter()
        .position(|word| has(ANY_ANSWER_LINKS, word.as_ref()))
        .map_or(&[], |at| &tail[at + 1..])
}

fn asks_after_any(words: &[&str]) -> bool {
    if words.get(1).is_none_or(|word| has(ANY_ALREADY_ASKS, word)) {
        return false;
    }
    let about = after_any_link(words);
    !about.is_empty()
        && !about
            .windows(2)
            .any(|w| has(OWNS_A_PERSON, w[0]) && has(A_PERSON_NAMED, w[1]))
}

/// Whether the line only names an option and a place, with no finite verb.
pub(crate) fn noun_phrase_request(line: &Line) -> bool {
    const PLACE_WORD: &[&str] = &["around", "at", "from", "in", "near", "to"];
    const SELF: &[&str] = &[
        "he", "here", "i", "it", "its", "me", "mine", "my", "myself", "our", "she", "that",
        "there", "they", "this", "us", "we", "you", "your",
    ];
    let words = &line.norm;
    (2..=8).contains(&words.len())
        && !line.signals.question_mark
        && !words.iter().enumerate().any(|(at, word)| {
            has(SELF, word) || has(VERBISH, word) && (at == 0 || words[at - 1] != "to")
        })
        && words.iter().any(|word| has(PLACE_WORD, word))
}
fn asks_after_thanks(words: &[&str]) -> bool {
    words.iter().enumerate().any(|(at, w)| {
        let before = at.checked_sub(1).map(|i| words[i]).unwrap_or("");
        !has(GIVEN_ADVICE, before)
            && (has(THANKS_ASK_WORDS, w)
                || has(THANKS_ASK_NOUNS, w)
                    && words.get(at + 1).is_some_and(|w| has(THANKS_ASK_PREPS, w))
                    && words
                        .get(at + 2)
                        .is_some_and(|w| !w.is_empty() && w.chars().all(char::is_alphabetic)))
    })
}

fn indicates_a_fall(words: &[&str]) -> bool {
    words.iter().enumerate().any(|(index, &word)| {
        if has(FALL, word) || word.starts_with("collaps") {
            return true;
        }
        let after = &words[index + 1..];
        if word == "fell" {
            return !after.first().is_some_and(|word| {
                matches!(*word, "asleep" | "for" | "into" | "in" | "behind" | "out")
            });
        }
        matches!(
            word,
            "tripped" | "slipped" | "slipping" | "stumbled" | "stumbling"
        ) && !after.first().is_some_and(|word| has(SLIPPED_FROM, word))
            && !after.iter().take(6).any(|word| has(FOUND, word))
    })
}

fn step_reported_done(rest: &[&str]) -> bool {
    let ["i" | "we", verb, after @ ..] = rest else {
        return false;
    };
    if after.is_empty()
        || !(has(STEP_VERBS, verb)
            || *verb == "gave" && after.iter().take(2).any(|word| *word == "go"))
    {
        return false;
    }
    if matches!(*verb, "tried" | "swapped") {
        return after.starts_with(&["it"]);
    }
    (!has(STEP_ANY_OBJECT, verb) || matches!(after.first(), Some(&("it" | "the"))))
        && (after.starts_with(&["it"]) || after.contains(&"my") || after.contains(&"the"))
}

fn step_failed(rest: &[&str]) -> bool {
    rest.iter().skip(2).any(|word| has(STEP_WRONG, word))
}

fn praises_with_verb(rest: &[&str], said: &str) -> bool {
    rest.iter().any(|word| has(PRAISING_VERB, word)) || PRAISE_SAID.is_match(said).unwrap_or(false)
}

fn trouble(rest: &[&str], said: &str) -> bool {
    rest.iter().any(|word| has(NEEDS_ANSWER, word)) || STILL_WRONG.is_match(said).unwrap_or(false)
}

/// A fault or symptom left after a leading courtesy that still needs an answer.
pub(crate) fn courtesy_trouble(rest: &[String]) -> bool {
    let words: Vec<_> = rest.iter().map(String::as_str).collect();
    trouble(&words, &rest.join(" "))
}

/// A completed step, compliment, routine expression, or sign-off following a courtesy.
pub(crate) fn courtesy_signoff(rest: &[String]) -> bool {
    let words: Vec<_> = rest.iter().map(String::as_str).collect();
    let said = rest.join(" ");
    step_reported_done(&words) && !step_failed(&words)
        || praises_with_verb(&words, &said)
        || words.iter().any(|word| has(FELL_WORDS, word)) && !indicates_a_fall(&words)
        || words.contains(&"temperature")
        || SIGNING_OFF.is_match(&said).unwrap_or(false)
}

fn thanks_only(rest: &[&str], clauses: &[Vec<&str>]) -> bool {
    let said = rest.join(" ");
    if rest.first().is_some_and(|word| has(COURTESY_THING, word))
        || trouble(rest, &said)
        || indicates_a_fall(rest)
    {
        return false;
    }
    if rest.len() < 4
        || rest.first().is_some_and(|word| has(THANKS_ABOUT, word))
        || after(rest, THANKS).is_some()
    {
        return true;
    }
    if step_reported_done(rest) {
        return !step_failed(rest);
    }
    let praised = (rest.iter().any(|word| has(PRAISING, word)) || praises_with_verb(rest, &said))
        && !rest.iter().any(|word| has(TURNED_ROUND, word));
    agrees(rest)
        || rest
            .iter()
            .any(|word| has(BETTER, word) || has(SAYS_ABOUT_YOU, word) || has(FELL_WORDS, word))
        || praised
        || rest.contains(&"temperature")
        || SIGNING_OFF.is_match(&said).unwrap_or(false)
        || !clauses
            .iter()
            .skip(1)
            .any(|clause| clause.len() >= 4 && !has(THANKS_ABOUT, clause[0]))
}

fn asks_the_bot(words: &[&str]) -> bool {
    matches!(words, [who, verb, following, ..]
        if has(YOU_WORDS, who) && has(ASK_VERBS, verb) && has(ASK_FOLLOW, following))
}

fn confirmation(words: &[&str]) -> bool {
    let Some((last, body)) = words.split_last() else {
        return false;
    };
    if body.len() < 2
        || !has(TAG_ENDINGS, last)
        || opens_question(words)
        || asks_the_bot(words)
        || body.iter().any(|w| has(TAG_PLAN, w))
        || body.windows(2).any(|p| {
            p[0] == "at" && has(TAG_TIME_NUMBER, p[1]) || has(TAG_CHECKABLE, p[0]) && p[1] == "be"
        })
        || !body.iter().any(|w| has(TAG_CHECKABLE, w))
    {
        return false;
    }
    if *last != "right" {
        return body.len() >= 3;
    }
    let ending = body[body.len() - 1];
    !ending.ends_with("ing") && !ending.ends_with("ed") && !has(TAG_RIGHT_NOT, ending)
}

fn recommendation_frame(words: &[&str], text: &str, asks: &Asks) -> bool {
    static TO_VERB: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\bto\s+[a-z]+").unwrap());
    let words: Vec<_> = words.iter().copied().filter(|w| !has(POLITE, w)).collect();
    words.len() <= 8
        && words.first().is_some_and(|w| has(OPENS_A_FRAME, w))
        && !words
            .iter()
            .any(|w| has(SPEAKER, w) || has(SAYS_WHAT_IT_IS, w))
        && words.iter().any(|w| has(PICKING, w))
        && TO_VERB.is_match(text)
        && (!asks.is_empty() || words.iter().any(|w| has(PICKING_QUALITY, w)))
}

pub(super) fn yesno_clause(line: &Line) -> bool {
    line.clauses.iter().any(|clause| {
        super::line::strip_leaders(clause)
            .first()
            .is_some_and(|word| has(YESNO_STARTS, word))
    })
}

fn classify(words: &[&str], line: &Line, asks: &mut Asks, tag: bool) -> Kind {
    let text = line.norm_text.as_str();
    if words.is_empty() {
        return Kind::Empty;
    }
    let clauses: Vec<Vec<_>> = line
        .clauses
        .iter()
        .map(|clause| clause.iter().map(String::as_str).collect())
        .collect();
    let courtesy = without_courtesy_filler(words);
    let thanks = after(courtesy, THANKS);
    if line.signals.question_mark
        || opens_question(words)
        || clauses
            .iter()
            .enumerate()
            .any(|(i, c)| opens_question(c) && !continues(&clauses, i))
        || asks_the_bot(words)
        || tag
        || asks_help(text)
        || asks_after_any(words)
        || !asks.is_empty()
            && after(words, THANKS).is_some()
            && (words.iter().any(|word| has(QUESTION_WORDS, word)) || asks_after_thanks(words))
    {
        return Kind::Question;
    }
    if recommendation_frame(words, text, asks) {
        asks.insert("recommend");
        return Kind::Question;
    }
    if after(words, GREETINGS).is_some_and(greeting_only) {
        return Kind::Greeting;
    }
    if thanks.is_some_and(|rest| thanks_only(rest, &clauses))
        || helped(words)
        || ends_with_thanks(courtesy)
    {
        return Kind::Thanks;
    }
    Kind::Statement
}

/// Read the broad dialogue kind and its details with the same underlying ask labels.
pub(crate) fn read(line: &Line) -> (Kind, KindFeatures) {
    let words: Vec<_> = line.norm.iter().map(String::as_str).collect();
    let functional: Vec<_> = words
        .iter()
        .copied()
        .filter(|word| !matches!(*word, "" | "|"))
        .collect();
    let (mut asks, quiz) = feature::initial_asks(line, &words, &functional);
    let tag = confirmation(&words);
    let kind = classify(&words, line, &mut asks, tag);
    (
        kind,
        feature::features(line, &words, &functional, kind, asks, tag, quiz),
    )
}

#[cfg(test)]
#[path = "kinds_tests.rs"]
mod tests;
