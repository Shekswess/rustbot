//! Practical sources for live or local questions and short travel follow-ups.
#[path = "findout_data.rs"]
mod data;
use super::super::Memory;
use data::*;
use fancy_regex::Regex;
use std::sync::LazyLock;
type Pool = &'static [&'static str];

static UNNAMED_NEAREST: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"^where(?:'s| is| are)? (?:the )?nearest (?:one|ones|place|spot)(?: (?:near me|near here|near us|nearby|please))?$",
    )
    .expect("unnamed nearest place question")
});

// The unnamed-place branch of PyBot's find_a_place reader.
const UNNAMED_PLACE: Pool = &[
    "No map on this side, so I can't point you at anywhere near you. A map app with the name of the thing and your town is the quickest way, and a phone call settles whether they keep it.",
    "I can't see what's near you, so any shop I named would be a guess. Search it in a map app with your town added, and ring first: that saves the trip if they haven't got it.",
];

fn hit(p: &Regex, s: &str) -> bool {
    p.is_match(s).unwrap_or(false)
}
fn report(s: &str) -> bool {
    !s.ends_with('?') && hit(&A_REPORT, s)
}
fn pool(s: &str, memory: Option<&Memory>) -> Option<Pool> {
    static WORDS: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new("[a-z0-9'-]+").unwrap());
    if s.is_empty() || report(s) {
        return None;
    }
    let words = WORDS.find_iter(s).map(|m| m.as_str()).collect::<Vec<_>>();
    let contains = |set: &[&str]| words.iter().any(|w| set.binary_search(w).is_ok());
    let found = |p: &Regex| hit(p, s);
    if found(&BUSY) && found(&BUSY_ASKS) {
        if found(&A_PERSON_SUBJECT) {
            return None;
        }
        let named = contains(A_PLACE)
            || found(&A_PLACE_IN_A_TURN)
            || found(&A_AND_E)
            || memory.is_some_and(|m| {
                m.turns.iter().rev().take(3).any(|t| {
                    [&t.user, &t.bot].iter().any(|v| {
                        hit(
                            &A_PLACE_IN_A_TURN,
                            &v.to_lowercase()
                                .split_whitespace()
                                .collect::<Vec<_>>()
                                .join(" "),
                        )
                    })
                })
            });
        if named {
            return Some(BUSY_LINES);
        }
        if !found(&A_WAIT_NOUN) {
            return None;
        }
        return Some(if memory.is_some_and(|m| !m.turns.is_empty()) {
            BUSY_NO_PLACE_AND_CONTEXT
        } else {
            BUSY_NO_PLACE
        });
    }
    if (found(&HOURS_ASK) && (contains(A_PLACE) || words.contains(&"it"))
        || found(&LATE_ASK) && contains(A_PLACE))
        && !found(&HOW_TO_OPEN)
        && !contains(NOT_A_VENUE)
        && !contains(THE_PEOPLE)
    {
        return Some(HOURS);
    }
    if found(&KICK_OFF) {
        return Some(KICK_OFF_LINES);
    }
    if (contains(ON_WORDS) || contains(ON_EVENT)) && found(&ON_ASK)
        || contains(ON_WORDS) && found(&WHAT_IS_GOOD)
    {
        return Some(WHATS_ON_LINES);
    }
    if (contains(TRAVEL) || found(&ROAD))
        && !found(&HOW_TO)
        && !contains(TRAVEL_CLAIM)
        && !found(&THEIR_OWN_ONLINE)
        && found(&ASKED)
        && (!found(&STORY) || found(&OPENS_ASKING))
        && (contains(STATUS) || STATUS_PHRASES.iter().any(|p| s.contains(p)))
    {
        return Some(TRAVEL_LINES);
    }
    if found(&PRICE_ASK) && contains(PRICE_THING) || found(&FREE_ASK) && !contains(FREE_COMPOUND) {
        return Some(PRICE_LINES);
    }
    if found(&AVAIL) {
        return Some(AVAIL_LINES);
    }
    if found(&LOCAL_AREA) && found(&LOCAL_ASK) {
        return Some(LOCAL_LINES);
    }
    if found(&DELIVER) {
        return Some(DELIVER_LINES);
    }
    if found(&TERM_DATES) {
        return Some(TERM_LINES);
    }
    if found(&NEARBY_AREA) && found(&ASKS_LOCAL) || found(&NEAREST) && contains(NEARBY_SERVICE) {
        return Some(NEARBY_LINES);
    }
    if found(&OFFICIAL_ASK) && !(contains(PET) && contains(DIED)) {
        if contains(OFFICIAL_THING) {
            return Some(OFFICIAL_LINES);
        }
        if found(&RULES_ASK)
            && !contains(GAME)
            && words.iter().any(|w| ASK_STOP.binary_search(w).is_err())
        {
            return Some(RULES_LINES);
        }
    }
    None
}
fn more_about_journey(s: &str, memory: &Memory) -> Option<Pool> {
    if !hit(&A_FOLLOW_UP, s) || !hit(&A_BACK_REFERENCE, s) {
        return None;
    }
    let last = memory
        .turns
        .iter()
        .rev()
        .find(|t| !t.bot.is_empty())?
        .bot
        .as_str();
    if TRAVEL_LINES.contains(&last) {
        return Some(TRAVEL_LINES);
    }
    LIVE_POOLS.iter().copied().find(|pool| pool.contains(&last))
}
fn over_receipt(s: &str) -> Option<Pool> {
    if !hit(&A_STATUS_SPENT_ON, s) || report(s) {
        return None;
    }
    let bare = s.strip_prefix("the ").unwrap_or(s).trim();
    pool(bare, None).or_else(|| (!hit(&NOT_THEIR_STATUS, bare)).then_some(TRAVEL_LINES))
}
pub(crate) fn replace(s: &str, reply: &str, memory: &Memory) -> Option<String> {
    let receipt = super::fallback::is_neutral(reply);
    let find = super::routes::hedge(reply)
        && (hit(&BUSY, s) && hit(&BUSY_ASKS, s)
            || !hit(&A_PLACE_NAMED, &reply.to_lowercase())
                && !hit(&HOLDS_IT, &reply.to_lowercase()));
    if !(receipt || find) {
        return None;
    }
    let bare = s.trim_matches([' ', '?', '.', '!', ',']);
    if UNNAMED_NEAREST.is_match(bare) {
        return Some(super::pick(UNNAMED_PLACE, memory, bare).into());
    }
    let choices = pool(s, Some(memory))
        .or_else(|| more_about_journey(s, memory))
        .or_else(|| if receipt { over_receipt(s) } else { None })?;
    Some(super::choose_recent(choices, memory, s).into())
}

#[cfg(test)]
#[path = "../../../tests/fixtures/findout.rs"]
mod reference;
#[test]
fn classifies_reference_queries() {
    for &(line, expected) in reference::CASES {
        assert_eq!(pool(line, None), expected, "{line}");
    }
}

#[test]
fn nearest_place_followup_keeps_the_local_candidates_and_cash_answer() {
    use crate::bot::Bot;

    let mut bot = Bot::new(Some("0")).unwrap();
    bot.turn("im looking for a launderette");
    assert_eq!(bot.turn("is there one close by").module, Some("near_me"));

    let cash = bot.turn("does it take cash");
    assert_eq!(cash.module, Some("answers"));
    assert_eq!(
        cash.candidates.iter().map(|c| c.module).collect::<Vec<_>>(),
        ["answers", "greeting"]
    );
    assert!(
        cash.reply
            .contains("Check foreign transaction and ATM fees with your provider.")
    );

    let nearby = bot.turn("where is the nearest one");
    assert_eq!(nearby.module, Some("near_me"));
    assert!(nearby.reply.contains("launderette"));
    assert_eq!(
        nearby
            .candidates
            .iter()
            .map(|c| c.module)
            .collect::<Vec<_>>(),
        ["near_me", "greeting"]
    );
    assert_eq!(nearby.candidates[1].reply, UNNAMED_PLACE[1]);

    let hedge = super::fallback::unknown_replies()[0];
    let memory = Memory::default();
    assert!(
        replace("where's the nearest one?", hedge, &memory)
            .is_some_and(|reply| UNNAMED_PLACE.contains(&reply.as_str()))
    );
    assert!(
        replace("where is the nearest café", hedge, &memory)
            .is_none_or(|reply| !UNNAMED_PLACE.contains(&reply.as_str()))
    );
}
