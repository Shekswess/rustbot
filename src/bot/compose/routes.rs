//! Identify the source of information for questions about a policy, venue, or local service.
#[path = "route_data.rs"]
mod data;
use crate::bot::{Memory, Reading};
use data::*;
use fancy_regex::Regex;
type Pool = &'static [&'static str];
fn matches(p: &Regex, s: &str) -> bool {
    p.is_match(s).unwrap_or(false)
}
fn options(s: &str) -> Option<Pool> {
    let hit = |p: &Regex| matches(p, s);
    if s.is_empty()
        || [
            &*COVER_IT,
            &*COVERED_IN,
            &*PERSON_ALLOWED,
            &*WORKING_OUT,
            &*BODY_STATE,
            &*PAST,
        ]
        .iter()
        .any(|p| hit(p))
        || hit(&VISITOR) && (hit(&WARD) || hit(&SOMEONES_PERSON))
    {
        return None;
    }
    let asking = hit(&QUESTION);
    if asking {
        if hit(&THE_OLD_ONE) && hit(&MOVING_THE_OLD) && !hit(&NOT_THE_OLD_ONE) {
            return Some(THE_OLD_ONE_POOL);
        }
        if (hit(&A_PLACE) || hit(&A_ROOM_FACILITY))
            && hit(&A_FACILITY)
            && hit(&A_HAS_ASK)
            && !hit(&NOT_A_HAS)
        {
            return Some(A_PLACE_HAS);
        }
        if hit(&CANCEL_TERMS) {
            return Some(TERMS);
        }
        if hit(&HARM_NOUN) && hit(&COVER) {
            return Some(INSURER);
        }
        if hit(&TERM_SHAPE) {
            if hit(&PRICE_ASK) && !hit(&TERM_WORD) {
                return None;
            }
            return Some(if hit(&INSURANCE_WORD) || hit(&HARM_NOUN) {
                INSURER
            } else if hit(&TICKET_WORD) {
                OPERATOR
            } else if hit(&CONTRACT_WORDS) {
                CONTRACT
            } else {
                TERMS
            });
        }
    }
    let terms = hit(&TERMS_THING) || hit(&COVERED_PHRASE);
    if asking && !terms && hit(&INCLUDED_ASK) && hit(&PRICE_THING) {
        return Some(INCLUDED);
    }
    if hit(&COURT_ARM) {
        return None;
    }
    if asking && terms && hit(&TERMS_ASK) {
        return Some(if hit(&INSURANCE_WORD) {
            INSURER
        } else if hit(&TICKET_WORD) {
            OPERATOR
        } else {
            TERMS
        });
    }
    if (hit(&LEAVING_ASK) || (hit(&EARLY) || hit(&GET_OUT_OF)) && hit(&AGREEMENT_WORD))
        && !(hit(&LEAVE_EARLY) && hit(&SHIFT_WORD) || hit(&NOT_AN_AGREEMENT))
    {
        return Some(CONTRACT);
    }
    if hit(&COUNT_AS) && hit(&ALLOWANCE_WORD) {
        return Some(VENUE);
    }
    if asking && hit(&NEEDED_THING) && hit(&NEEDED_ASK) {
        return Some(ISSUER);
    }
    let taking = !hit(&NAMED_OBJECT) && (hit(&TAKE_ON) || hit(&TAKE_TO) && hit(&VENUE_WORDS));
    if taking || hit(&ALLOWED_ASK) {
        if hit(&LOCAL_WORDS) {
            return Some(LOCAL);
        }
        if hit(&LAW_NAMED) && !(hit(&CARRYING) && hit(&TO_INSIDE)) {
            return None;
        }
        return Some(if hit(&VENUE_WORDS) {
            VENUE
        } else if hit(&CONTRACT_WORDS) {
            CONTRACT
        } else {
            ALLOWED
        });
    }
    (hit(&ASKING_SHAPE)
        && !hit(&NOT_A_SERVICE)
        && (hit(&SERVICE_WORDS) || hit(&NOISE) && hit(&COMPLAINT)))
    .then_some(LOCAL_SERVICE)
}
pub(super) fn has_route(clean: &str) -> bool {
    options(clean).is_some()
}

fn shares_subject(reply: &str, clean: &str, skip: &[&str]) -> bool {
    let words = |s: &str| {
        s.to_lowercase()
            .split(|c: char| !c.is_ascii_lowercase())
            .filter(|w| w.len() > 2 && PROPOSAL_QUESTION_FILLER.binary_search(w).is_err())
            .map(str::to_owned)
            .collect::<std::collections::HashSet<_>>()
    };
    let said = words(reply);
    words(clean)
        .into_iter()
        .any(|w| skip.binary_search(&w.as_str()).is_err() && said.contains(&w))
}
pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = reading
        .line
        .clean
        .to_lowercase()
        .replace(['—', '–'], "-")
        .replace(['\'', '’'], "");
    if matches(&BUSY, &clean) && matches(&BUSY_ASKS, &clean) {
        return None;
    }
    let pool = options(&clean)?;
    let reply = crate::bot::modules::greeting::respond(reading, memory)?;
    let hedge = hedge(&reply) || matches(&PROPOSAL_SEARCH_ONLY, &reply);
    let wrong_subject = if pool == A_PLACE_HAS && matches(&PROPOSAL_A_YES_OR_NO_ASK, &clean) {
        matches(&PROPOSAL_MONEY_WORDS, &reply)
            || matches(&PROPOSAL_LIVE_ONLY, &reply)
            || !shares_subject(&reply, &clean, &[])
    } else if pool == THE_OLD_ONE_POOL {
        !shares_subject(&reply, &clean, PROPOSAL_OLD_ONE_VERBS)
    } else {
        pool == INCLUDED
            && matches(&PROPOSAL_INCLUDED_ASK, &clean)
            && matches(&PROPOSAL_A_FIGURE_REPLY, &reply)
    };
    (hedge || wrong_subject).then(|| {
        super::choose_recent(pool, memory, &reading.folded().replace(['\'', '’'], "")).into()
    })
}

pub(in crate::bot) fn hedge(reply: &str) -> bool {
    let head = reply.chars().take(40).collect::<String>();
    HEDGE_HEADS.binary_search(&head.as_str()).is_ok()
        || HEDGE_PREFIXES.iter().any(|p| reply.starts_with(p))
}

#[cfg(test)]
#[path = "../../../tests/fixtures/routes.rs"]
mod tests;
