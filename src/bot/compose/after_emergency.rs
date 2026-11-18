//! Continue a selected gas warning while the conversation is still about it.

use crate::bot::{Exchange, Memory, Reading, modules::gas_followup, understanding::kinds::Kind};
use crate::text::is_space;
use fancy_regex::Regex;
use std::collections::BTreeSet;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        pub(super) static $name: std::sync::LazyLock<fancy_regex::Regex> =
            std::sync::LazyLock::new(|| fancy_regex::Regex::new($source).expect(stringify!($name)));
    };
}

#[path = "after_emergency_data.rs"]
mod data;
use data::*;

pattern!(WORDS, r"[a-z']+");
pattern!(
    CALL_ACT,
    r"\b(?:call|ring|ringing|dial|phone|text|press|try)\b"
);
pattern!(COME_OVER, r"\b(?:come|coming|let)\s+(?:in|round)\b");
pattern!(HOLD_SENTENCES, r"(?<=[.!?])\s+");
pattern!(
    INDOORS,
    r"\b(?:i(?:'m|m| am)|we(?:'re| are)|they(?:'re| are))\s+(?:still\s+)?(?:inside|indoors|in the (?:house|flat|building))\b"
);

fn matches(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).unwrap_or(false)
}

fn lower(text: &str) -> String {
    text.to_lowercase()
        .replace(['\u{2018}', '\u{2019}'], "'")
        .split(is_space)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    WORDS.find_iter(text).flatten().map(|found| found.as_str())
}

fn has(table: &[&str], text: &str) -> bool {
    table.binary_search(&text).is_ok()
}

fn includes(line: &[&str], table: &[&str]) -> bool {
    line.iter().any(|word| has(table, word))
}

fn not_a_subject(text: &str) -> bool {
    let clean = lower(text);
    [&NOTHING_SAID, &GREETING_WITH_AN_ADDRESS]
        .iter()
        .any(|pattern| matches(pattern, &clean))
        || [
            &RECALL_SHAPE,
            &RECALL_ABOUT,
            &CHAT_QUESTION,
            &FIRST_SAID_BY_BOT,
            &FIRST_THING,
            &TALKED_ABOUT,
        ]
        .iter()
        .any(|pattern| matches(pattern, &clean))
}

fn shared_words(line: &str, answer: &str) -> usize {
    let line = line.to_lowercase();
    let answer = answer.to_lowercase();
    let said: BTreeSet<_> = words(&answer).collect();
    words(&line)
        .filter(|word| word.len() >= 5 && said.contains(word))
        .collect::<BTreeSet<_>>()
        .len()
}

struct Thread<'a> {
    safety: &'a Exchange,
    since: &'a [Exchange],
    outside: bool,
}

fn current(memory: &Memory) -> Option<Thread<'_>> {
    let start = memory.turns.len().saturating_sub(THREAD_TURNS);
    let index = start
        + memory.turns[start..]
            .iter()
            .rposition(|turn| turn.module == "safety")?;
    let outside = gas_followup::gas_history(&memory.turns[..=index])?.outside;
    let (safety, since) = memory.turns[index..].split_first()?;
    if since.iter().any(|turn| {
        turn.module != "greeting"
            && !not_a_subject(&turn.user)
            && shared_words(&turn.bot, &safety.bot) < THE_THREAD_KEPT
    }) {
        return None;
    }
    Some(Thread {
        safety,
        since,
        outside,
    })
}

fn call_was_requested(answer: &str) -> bool {
    let answer = answer.to_lowercase();
    SENTENCE_END.split(&answer).flatten().any(|sentence| {
        let low = ALREADY_OR_NOT.replace_all(sentence, "already");
        let mut condition = false;
        for clause in CLAUSE_END.split(&low).flatten() {
            let conditional = matches(&A_CONDITION, clause);
            if matches(&CALL_THEM_NOW, clause) && !conditional && !condition {
                return true;
            }
            condition = conditional && !matches(&CALL_ACT, clause);
        }
        false
    })
}

fn fear(clean: &str, asking: bool) -> bool {
    if asking
        || matches(&CARE_ASKS, clean)
        || !matches(&ABOUT_THE_SPEAKER, clean)
        || matches(&NOT_FRIGHTENED, clean)
    {
        return false;
    }
    if matches(&FRIGHTENED, clean) {
        return true;
    }
    let terms: Vec<_> = words(clean).take(7).collect();
    !terms.is_empty()
        && terms.len() <= 6
        && !terms.contains(&"about")
        && includes(&terms, WORRY_WORDS)
        && terms
            .iter()
            .all(|word| has(WORRY_FILLER, word) || has(WORRY_WORDS, word))
}

fn arriving(clean: &str) -> bool {
    matches(&WITH_YOU, clean) || matches(&COME_OVER, clean) && matches(&ARRIVING, clean)
}

fn rang_recently(turns: &[Exchange]) -> bool {
    turns
        .iter()
        .rev()
        .take(3)
        .any(|turn| matches(&ALREADY_RANG, &lower(&turn.user)))
}

fn hold_pool(last: &str, called: bool, report: bool) -> &'static [&'static str] {
    let carrying: Vec<_> = HOLD_SENTENCES
        .split(last)
        .flatten()
        .filter(|sentence| CALL_WORDS.iter().any(|word| sentence.contains(word)))
        .collect();
    let known = called
        || !carrying.is_empty()
            && (!last.contains("111") || matches(&IMMEDIATE_CALL, last))
            && !carrying
                .iter()
                .all(|sentence| matches(&CONDITIONAL_OPENING, sentence));
    if !known {
        STILL_HERE_AFTER
    } else if report && !called {
        SORTED_AND_NO_CALL_YET
    } else {
        HOLD_AFTER
    }
}

fn feels_better(clean: &str) -> bool {
    matches(&FEELS_BETTER, clean)
        && !matches(&FEELS_NOT_BETTER, clean)
        && !clean.contains('?')
        && !matches(&MOVE_ON, clean)
        && words(clean).all(|word| {
            [EASING_FILLER, SYMPTOM_WORDS, BODY_PARTS, HEALTH_WORDS]
                .iter()
                .any(|table| has(table, word))
        })
}

fn did_not_work(terms: &[&str], clean: &str) -> bool {
    !clean.contains('?')
        && terms.len() <= 14
        && !includes(terms, THANKS_WORDS)
        && terms.iter().all(|term| {
            let bare = term.replace('\'', "");
            has(REPORT_FILLER, &bare) || has(REPORT_VERDICT_WORD, &bare)
        })
        && (matches(&TRIED_NO_CHANGE, clean) || matches(&TRIED_WORSE, clean))
}

fn thanks_reports_trouble(terms: &[&str], clean: &str) -> bool {
    let content = terms.iter().any(|word| {
        has(CARE_WORDS, word)
            && !has(THANKS_WORDS, word)
            && !has(CARE_PRONOUNS, word)
            && !has(CARE_POINTER, word)
    });
    if content && includes(terms, ONGOING_WORDS) {
        return true;
    }
    if includes(terms, CARE_RESOLVED) {
        return includes(terms, CARE_NOT_RESOLVED);
    }
    content
        || matches(&CARE_ASKS, clean)
        || CARE_PHRASES.iter().any(|phrase| clean.contains(phrase))
}

fn keeps(
    thread: &Thread<'_>,
    reading: &Reading<'_>,
    clean: &str,
    asking: bool,
    live: bool,
) -> bool {
    let norm: Vec<_> = reading.line.norm.iter().map(String::as_str).collect();
    let own = reading.own_topic.map(|(topic, _)| topic);
    if includes(&norm, CARE_HURTS) {
        return true;
    }
    if !live && !includes(&norm, CARE_INJURY) && matches(&HOUSE_FAULT, clean) {
        return false;
    }
    if norm
        .iter()
        .any(|word| has(CARE_WORDS, word) && !has(CARE_POINTER, word))
        || matches(&CARE_ASKS, clean)
        || includes(&norm, REACTION_WORDS)
        || includes(&norm, CARE_RESOLVED)
            && includes(&norm, CARE_NOT_RESOLVED)
            && matches!(own, None | Some("feelings" | "health"))
        || includes(&norm, CARE_POINTER) && own.is_none() && !matches(&POINTER_OWN_OBJECT, clean)
        || CARE_PHRASES.iter().any(|phrase| clean.contains(phrase))
        || arriving(clean)
        || matches(&ALREADY_RANG, clean)
        || matches(&CARE_SEEN_ASKS, clean)
        || reading.context_fields.presence
        || matches(&STILL_HERE, clean)
    {
        return true;
    }
    if reading.kind == Kind::Thanks {
        return norm.len() <= 5;
    }
    if live && includes(&norm, BODY_PARTS) {
        return true;
    }
    if asking {
        return norm
            .iter()
            .copied()
            .filter(|word| {
                !has(STOPWORDS, word)
                    && (word.chars().count() > 1 || word.chars().all(char::is_numeric))
            })
            .take(MAX_KEYWORDS)
            .all(|word| has(GENERIC_WORDS, word));
    }
    if norm.len() <= 2 && includes(&norm, AGREEMENTS) {
        return true;
    }
    let opener = lower(&thread.safety.user);
    let mut content = norm
        .iter()
        .copied()
        .filter(|word| word.len() > 3 && !has(ASKING_WORDS, word))
        .peekable();
    own.is_none()
        && (content.peek().is_none() || content.any(|term| words(&opener).any(|old| term == old)))
}

/// Answer a follow-up to the latest gas card while the thread is still on that incident.
pub(super) fn reply(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let thread = current(memory)?;
    let normalized = lower(&reading.line.clean);
    let clean = normalized.as_str();
    let words: Vec<_> = reading.line.words.iter().map(String::as_str).collect();
    let answer =
        |options| Some(super::choose_recent(options, memory, &reading.line.clean).to_owned());
    let asked = reading.kind == Kind::Question || clean.contains('?');
    let last = thread.safety.bot.to_lowercase();
    let live = call_was_requested(&last);
    let outside = thread.outside
        || gas_followup::reports_outside(clean)
        || thread
            .since
            .iter()
            .rev()
            .any(|turn| gas_followup::reports_outside(&lower(&turn.user)));
    let called_now = matches(&ALREADY_RANG, clean);
    let called_earlier = rang_recently(&memory.turns);
    if live && reports_a_gas_smell(clean) {
        return Some(PERSISTENT_GAS_SMELL.to_owned());
    }
    let hold = |pool| {
        let chosen = super::choose_recent(pool, memory, &reading.line.clean);
        // The otherwise generic hold can tell a caller to stay put. If they say they are
        // still inside during a gas leak, keep the answer on leaving and calling from outside.
        if matches(&INDOORS, clean) && (chosen == HOLD_AFTER[1] || chosen == HOLD_AFTER[2]) {
            Some(evacuate().to_owned())
        } else {
            Some(chosen.to_owned())
        }
    };
    if fear(clean, asked) {
        return answer(AFRAID_AFTER);
    }
    if reading.kind == Kind::Greeting && !includes(&words, CARE_WORDS) {
        return answer(if live {
            HELLO_STILL_THERE
        } else {
            GREETING_AFTER
        });
    }
    if reading.context_fields.asks_how_i_am {
        return answer(CARE_HERE);
    }
    if !live && matches(&A_BACK_AGAIN, clean) {
        return answer(GREETING_AFTER);
    }
    let short_ack = words.len() <= 2 && includes(&words, ACKS);
    if short_ack
        && !live
        && thread
            .since
            .iter()
            .rev()
            .find(|t| t.module == "greeting")
            .is_some_and(|t| {
                THANKS_AFTER
                    .iter()
                    .any(|line| t.bot.chars().take(24).eq(line.chars().take(24)))
            })
    {
        return None;
    }
    if matches(&MOVE_ON, clean) {
        return answer(MOVE_ON_AFTER);
    }
    if live && did_not_work(&words, clean) {
        return answer(EMERGENCY_NOT_WORKED);
    }
    if matches(&THING_SORTED, clean) {
        return hold(hold_pool(&last, called_now || called_earlier, true));
    }
    if matches(&THE_WAY_OUT, clean) || matches(&FETCH_FROM_INSIDE, clean) {
        // This source branch directs the user to switch the boiler off during a suspected
        // gas escape. Reuse the safety module's outside advice instead of operating a switch.
        return Some(gas_followup::waiting(outside).to_owned());
    }
    let better = feels_better(clean);
    if !better && !keeps(&thread, reading, clean, asked, live) {
        return None;
    }
    if (reading.kind == Kind::Thanks || includes(&words, THANKS_WORDS))
        && !thanks_reports_trouble(&words, clean)
    {
        return answer(THANKS_AFTER);
    }
    if short_ack {
        return hold(hold_pool(&last, called_now || called_earlier, false));
    }
    if includes(&words, REACTION_WORDS)
        && memory
            .turns
            .iter()
            .rev()
            .take(2)
            .any(|turn| RESOLVED_AFTER.contains(&turn.bot.as_str()))
    {
        return answer(AFTER_RESOLVED);
    }
    let arrived = arriving(clean);
    if !asked && matches(&RESOLVED, clean) && !matches(&NEEDS_SEEN, clean) && !arrived {
        return answer(RESOLVED_AFTER);
    }
    if called_now {
        if matches(&A_HELPLINE, clean) {
            return answer(if SAFETY_WORDS.iter().any(|word| last.contains(word)) {
                HELPLINE_NOT_AN_AMBULANCE
            } else {
                STILL_HERE_AFTER
            });
        }
        return hold(HOLD_AFTER);
    }
    if includes(&words, REACTION_WORDS) {
        return hold(hold_pool(&last, called_now || called_earlier, false));
    }
    if matches(&CARE_SEEN_ASKS, clean) {
        return answer(CARE_SEEN);
    }
    if better {
        return answer(CARE_BETTER);
    }
    if arrived {
        return answer(WITH_YOU_AFTER);
    }
    if called_earlier {
        return hold(HOLD_AFTER);
    }
    if live {
        return answer(if matches(&HAZARD_LINE, &last) {
            HAZARD_AFTER
        } else {
            EMERGENCY_AFTER
        });
    }
    answer(if matches(&STILL_GOING, clean) {
        CARE_AFTER_STILL
    } else {
        CARE_AFTER
    })
}

pub(super) fn evacuate() -> &'static str {
    HAZARD_AFTER[1]
}

const PERSISTENT_GAS_SMELL: &str = "If you can smell gas in the area now, get outside and call the gas emergency line from there. Stay outside until the emergency team says it is safe. If you cannot get out safely or anyone is unwell, call 999.";

fn reports_a_gas_smell(text: &str) -> bool {
    [
        "smell gas",
        "smells of gas",
        "smell of gas",
        "smells like gas",
        "smell like gas",
    ]
    .iter()
    .any(|phrase| text.contains(phrase))
        && ![
            "no smell of gas",
            "does not smell",
            "doesn't smell",
            "doesnt smell",
            "cannot smell",
            "can't smell",
            "cant smell",
            "no longer",
            "natural gas bill",
        ]
        .iter()
        .any(|phrase| text.contains(phrase))
}

pub(crate) fn needs_evacuation(reading: &Reading<'_>, candidate: &str) -> bool {
    let bare = crate::bot::memory::unframed_reply(candidate).unwrap_or(candidate);
    bare == PERSISTENT_GAS_SMELL
        || (bare == evacuate() && matches(&INDOORS, &lower(&reading.line.clean)))
}

/// A fresh composer proposal for the after-gas thread can keep the turn.
pub(crate) fn claims(reading: &Reading<'_>, memory: &Memory, candidate: &str) -> bool {
    let bare = crate::bot::memory::unframed_reply(candidate).unwrap_or(candidate);
    let said = super::said_again(candidate);
    !said.is_empty()
        && memory
            .turns
            .last()
            .is_none_or(|turn| super::said_again(&turn.bot) != said)
        && reply(reading, memory).is_some_and(|after| after == bare)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gas_history() -> Memory {
        let raw = "I smell gas in my kitchen. What should I do?";
        let empty = Memory::default();
        let bot = crate::bot::modules::safety::respond(&Reading::new(raw), &empty).unwrap();
        Memory {
            turns: vec![Exchange {
                user: raw.into(),
                bot,
                module: "safety",
            }]
            .into(),
            ..empty
        }
    }

    #[test]
    fn a_new_subject_leaves_the_gas_fallback() {
        let memory = gas_history();
        for line in [
            "How much is council tax?",
            "My boiler warranty has expired.",
            "Should I buy a gas boiler?",
        ] {
            assert_eq!(reply(&Reading::new(line), &memory), None, "{line}");
        }
    }

    #[test]
    fn a_follow_up_about_going_back_in_retains_the_outside_wording() {
        let memory = gas_history();
        // The pinned fallback's GAS_ANSWER starts with an instruction to switch off the boiler.
        const WAIT_OUTSIDE: &str = "Stay outside and ring the gas emergency line from there; don't use matches, lighters or doorbells until the gas engineer says it's safe.";
        assert_eq!(
            reply(&Reading::new("Can I go back in for my bag?"), &memory).as_deref(),
            Some(WAIT_OUTSIDE)
        );
        assert_eq!(
            reply(
                &Reading::new("I'm outside, can I go back in for my bag?"),
                &memory
            )
            .as_deref(),
            Some(
                "Stay outside until the gas emergency engineer says it's safe, and don't go back in before that."
            )
        );
        assert_eq!(
            reply(&Reading::new("Should I open the windows?"), &memory).as_deref(),
            Some(WAIT_OUTSIDE)
        );
    }

    #[test]
    fn a_caller_still_indoors_does_not_get_a_stay_put_reply() {
        let memory = gas_history();
        for (line, source) in [
            (
                "I called the gas emergency line, I'm still inside.",
                "I'm with you — stay put and let the operator talk you through it.",
            ),
            (
                "I rang them and I'm inside.",
                "Right with you. Stay where you are; the crew are on their way.",
            ),
        ] {
            let actual = reply(&Reading::new(line), &memory);
            assert_ne!(actual.as_deref(), Some(source));
            assert_eq!(
                actual.as_deref(),
                Some(
                    "Get everyone out first and call from outside; tell the operator exactly what is burning or leaking and where."
                ),
                "{line}"
            );
        }
        assert_eq!(
            reply(
                &Reading::new("I called the gas emergency line and I'm outside."),
                &memory
            )
            .as_deref(),
            Some("Still here. Keep the line open and follow what they tell you.")
        );
    }

    #[test]
    fn confirmation_that_help_is_coming_keeps_the_source_wording() {
        let memory = gas_history();
        for (line, expected) in [
            (
                "they are on their way",
                "I'm glad you're not on your own with it. Stay together and keep the line open.",
            ),
            (
                "the gas engineer is here",
                "That's good to hear. Two of you is better than one — say if anything changes.",
            ),
            (
                "the smell is gone",
                "That's the important bit done. Look after yourself now, and get seen if anything flares up.",
            ),
        ] {
            assert_eq!(
                reply(&Reading::new(line), &memory).as_deref(),
                Some(expected),
                "{line}"
            );
        }
    }

    #[test]
    fn an_unrelated_answer_closes_the_gas_fallback() {
        let mut memory = gas_history();
        memory.turns.push(Exchange {
            user: "What should I cook tonight?".into(),
            bot: "Pasta with tomatoes.".into(),
            module: "answers",
        });
        assert_eq!(reply(&Reading::new("thanks"), &memory), None);
    }
}
