//! Shape a trip around a mobility limit the person has just given.

use super::{mentions, split};
use crate::bot::{Memory, compose};

#[path = "chat_someone_travel_limit_data.rs"]
mod data;
use data::*;

fn options(clean: &str) -> Option<&'static [&'static str]> {
    if mentions(&IN_A_CHAIR, clean) {
        return Some(CHAIR_REPLIES);
    }
    if let Ok(Some(aid)) = AID.find(clean) {
        let aid = aid.as_str().to_lowercase();
        return Some(if aid.contains("wheelchair") || aid.contains("scooter") {
            CHAIR_REPLIES
        } else {
            WALK_REPLIES
        });
    }
    if let Ok(Some(limit)) = LIMIT_VERB.find(clean) {
        return Some(if limit.as_str().to_lowercase().contains("stairs") {
            STAIRS_REPLIES
        } else {
            WALK_REPLIES
        });
    }
    if mentions(&DISLIKES_THE_WALK, clean) || mentions(&FLAT, clean) {
        return Some(WALK_REPLIES);
    }
    let lead = RULE_LEAD
        .find(clean)
        .ok()
        .flatten()
        .map_or(0, |lead| lead.end());
    let rule = NO_WALKS.find_from_pos(clean, lead).ok().flatten()?;
    (rule.start() == lead && mentions(&RULE_TAIL, &clean[rule.end()..])).then_some(
        if mentions(&STAIRS_SAID, rule.as_str()) {
            STAIRS_REPLIES
        } else {
            WALK_REPLIES
        },
    )
}

pub(super) fn reply(text: &str, memory: &Memory) -> Option<String> {
    let line = text.replace('’', "'");
    let clean = split(&line).collect::<Vec<_>>().join(" ");
    if clean.is_empty()
        || clean.ends_with('?')
        || split(&clean).take(19).count() > 18
        || [&ASKS, &PET, &ILLNESS, &REFUSES_AID, &PUSHCHAIR]
            .iter()
            .any(|pattern| mentions(pattern, &clean))
    {
        return None;
    }
    let pool = options(&clean)?;
    if !(mentions(&A_TRIP, &clean)
        || mentions(&FLAT, &clean) && !mentions(&FLAT_IS_A_HOME, &clean)
        || memory
            .turns
            .iter()
            .rev()
            .take(4)
            .any(|turn| mentions(&A_TRIP, &turn.user)))
    {
        return None;
    }
    Some(compose::pick(pool, memory, text).to_owned())
}
