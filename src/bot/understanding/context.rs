//! Prior user lines available to the understanding reader.
use super::kinds::Kind;
use super::{
    Reading,
    entities::{self, Entity},
    line::Line,
    references::{self, Prior, References},
};
use regex::Regex;
use std::{collections::VecDeque, sync::LazyLock};
#[derive(Default)]
pub(crate) struct History {
    lines: VecDeque<Previous>,
    animal: Option<String>,
    pub(super) cues: super::topics::PriorCues,
    pub(super) last_time: Option<super::kinds::TimeDependence>,
}
struct Previous {
    entities: Vec<Entity>,
    subject: Option<&'static str>,
    topic: Option<(&'static str, i32)>,
    constraint: bool,
    short_opening: bool,
}

/// The asking clause and its competing setting refer to different named things.
#[derive(Clone, Copy, Debug)]
pub(crate) struct QuestionScene {
    asked: usize,
    setting: usize,
}

impl QuestionScene {
    pub(crate) fn names(self, entities: &[Entity]) -> Option<(&str, &str)> {
        Some((
            &entities.get(self.asked)?.text,
            &entities.get(self.setting)?.text,
        ))
    }
}

pub(super) fn question_scene(line: &Line, entities: &[Entity]) -> Option<QuestionScene> {
    if line.clauses.len() < 2 || entities.len() < 2 {
        return None;
    }
    // The source uses the last clause equal to focus. Entity positions are
    // measured against the whole normalized line, like these clause offsets.
    let focus = line
        .clauses
        .iter()
        .rposition(|clause| clause == &line.focus)?;
    let start = line.clauses[..focus].iter().map(Vec::len).sum::<usize>();
    let end = start + line.clauses[focus].len();
    let mut asked = None;
    let mut setting = None;
    for (index, entity) in entities.iter().enumerate() {
        if !matches!(entity.kind, "title" | "name" | "person" | "place" | "thing") {
            continue;
        }
        let slot = if (start..end).contains(&entity.at) {
            &mut asked
        } else {
            &mut setting
        };
        slot.get_or_insert(index);
    }
    let (asked, setting) = asked.zip(setting)?;
    let (asked_entity, setting_entity) = (&entities[asked], &entities[setting]);
    if asked_entity.text.eq_ignore_ascii_case(&setting_entity.text)
        || (asked_entity.kind == "person") != (setting_entity.kind == "person")
        || line
            .clauses
            .iter()
            .any(|clause| clause != &line.focus && says_what_is_missing(clause))
    {
        return None;
    }
    Some(QuestionScene { asked, setting })
}

fn says_what_is_missing(clause: &[String]) -> bool {
    clause.iter().any(|word| {
        matches!(
            word.as_str(),
            "no" | "not"
                | "without"
                | "havent"
                | "hasnt"
                | "hadnt"
                | "dont"
                | "doesnt"
                | "didnt"
                | "cant"
                | "cannot"
                | "isnt"
                | "arent"
                | "wasnt"
                | "wont"
                | "lack"
                | "lacks"
                | "lacking"
                | "empty"
                | "none"
                | "nothing"
        )
    }) || clause.windows(2).any(|pair| {
        let (first, second) = (pair[0].as_str(), pair[1].as_str());
        matches!((first, second), ("ran" | "run", "out") | ("out", "of"))
    })
}

static CONSTRAINT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(?:my|our|the)\s+(?:budget|price|cost|age|salary|wage|wages|income|savings|time|spending|tariff|allowance)\s+(?:is|was|'s)\b").unwrap()
});
static ACTUALLY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:^|\|)\s*actually\b|\b(?:no|oh|well|sorry|and|but),? actually\b").unwrap()
});
pub(crate) fn shifts(line: &Line) -> bool {
    const SHIFTS: &[&str] = &[
        "never mind",
        "nevermind",
        "forget that",
        "forget it",
        "different question",
        "new question",
        "new topic",
        "changing the subject",
        "change of subject",
        "on another note",
        "moving on",
        "back to",
        "anyway",
        "anyways",
        "instead",
        "another thing",
        "unrelated",
        "off topic",
    ];
    SHIFTS.iter().any(|s| line.norm_text.contains(s)) || ACTUALLY.is_match(&line.norm_text)
}
impl History {
    pub(super) fn opens_conversation(&self) -> bool {
        self.lines
            .back()
            .is_none_or(|p| p.short_opening && p.entities.is_empty() && p.topic.is_none())
    }
    pub(crate) fn retain(&mut self, read: &mut Reading<'_>) {
        self.animal = read.animal_in_hand.take();
        self.last_time = Some(read.time.0);
        if read.raw.trim().is_empty() {
            return;
        }
        if self.lines.len() == 4 {
            self.lines.pop_front();
        }
        self.cues = super::topics::PriorCues::from_line(&read.line.norm);
        self.lines.push_back(Previous {
            entities: std::mem::take(&mut read.entities),
            subject: read.subject_topic,
            short_opening: read
                .line
                .norm
                .iter()
                .filter(|s| !s.is_empty() && *s != "|")
                .count()
                <= 3,
            topic: read.own_topic,
            constraint: CONSTRAINT.is_match(&read.line.norm.join(" ")),
        });
    }
    pub(super) fn resolve(
        &self,
        line: &Line,
        kind: Kind,
        current: &[Entity],
        own: Option<&str>,
        subject: Option<&str>,
    ) -> (References, bool, Option<String>, Option<&'static str>) {
        let previous = self.lines.back();
        let mut picked = previous;
        if let Some(last) = previous
            && (last.entities.is_empty() || last.constraint)
        {
            let carry = self
                .lines
                .iter()
                .rev()
                .skip(1)
                .find(|p| p.entities.iter().any(entities::pointable));
            if last.constraint
                || last.subject.is_none()
                || carry.is_some_and(|p| p.subject.is_none() || p.subject == last.subject)
            {
                picked = carry.or(previous)
            }
        }
        let older = picked
            .zip(previous)
            .is_some_and(|(a, b)| !std::ptr::eq(a, b));
        let filtered;
        let prior_entities = if let Some(p) = picked {
            if older {
                filtered = p
                    .entities
                    .iter()
                    .filter(|e| entities::pointable(e))
                    .cloned()
                    .collect::<Vec<_>>();
                filtered.as_slice()
            } else {
                &p.entities
            }
        } else {
            &[]
        };
        let prior_topic = previous
            .and_then(|p| p.topic)
            .or_else(|| picked.and_then(|p| p.topic))
            .map(|t| t.0);
        let refs = references::resolve(
            line,
            kind,
            current,
            own,
            subject,
            previous.map(|_| Prior {
                entities: prior_entities,
                topic: prior_topic,
            }),
        );
        let new_person = references::person_is_new(line, current, prior_entities);
        let animal = references::animal_in_hand(&refs, &line.words, self.animal.as_deref());
        let inherited = if (refs.followup || subject.is_none()) && !shifts(line) {
            // Iteration from most recent to earliest makes equal scores prefer the earliest line.
            self.lines
                .iter()
                .rev()
                .filter_map(|p| p.topic)
                .filter(|t| t.1 >= 2)
                .max_by_key(|t| t.1)
                .map(|t| t.0)
                .or(prior_topic)
        } else {
            None
        };
        (refs, new_person, animal, inherited)
    }
}
