//! The native RustBot conversation engine.

mod bot;
mod text;

pub use bot::knowledge::lookup::{
    KnowledgeMatch, by_id as knowledge_by_id, knowledge_ids, lookup as lookup_knowledge,
};
pub use bot::understanding::normalize::phrase_alternatives;
pub use bot::{Bot, Candidate, Inventory, ModuleInfo, Turn};
pub use text::one_line;

/// The upstream repository commit used as the migration reference.
pub const PYBOT_REVISION: &str = "7d63c7fbebb44c7e466e5c8b5c1f341ab767929e";
/// The swarm commit identified by PyBot's original CLI.
pub const SOURCE_COMMIT: &str = "24cbc0439e07fd015c0747ab3be6e782a42b5c5b";

/// Return the native normalization fields and primary dialogue classification for one line.
pub fn inspect_line(text: &str) -> impl serde::Serialize {
    #[derive(serde::Serialize)]
    struct Inspection {
        #[serde(flatten)]
        line: bot::understanding::line::Line,
        kind: bot::understanding::kinds::Kind,
        #[serde(flatten)]
        features: bot::understanding::kinds::KindFeatures,
        #[serde(flatten)]
        details: bot::understanding::details::Details,
        #[serde(flatten)]
        context_fields: bot::understanding::ContextFields,
        #[serde(flatten)]
        time: bot::understanding::TimeFields,
        body: Option<bot::understanding::body::Part>,
        wants: Option<&'static str>,
        #[serde(flatten)]
        attempts: bot::understanding::attempts::Attempts,
        car_fault: bool,
        sense: std::collections::BTreeMap<&'static str, &'static str>,
        numbers: Vec<bot::understanding::numbers::Quantity>,
        entities: Vec<bot::understanding::entities::Entity>,
        person_names: Vec<String>,
        topics: Vec<(&'static str, i32)>,
        subject_topic: Option<&'static str>,
        #[serde(rename = "ref")]
        reference: bot::understanding::references::References,
        someone_new: bool,
        animal_in_hand: Option<String>,
        topic: Option<&'static str>,
        topic_score: i32,
        unsure_words: Vec<String>,
    }
    let reading = bot::understanding::Reading::for_inspection(text);
    let unsure_words =
        bot::understanding::topics::unsure(&reading.line.norm, reading.topics.first().map(|t| t.0))
            .into_iter()
            .map(str::to_owned)
            .collect();
    let person_names = bot::understanding::entities::person_names(&reading.line.norm)
        .into_iter()
        .map(str::to_owned)
        .collect();
    Inspection {
        details: bot::understanding::details::inspect(&reading),
        line: reading.line,
        kind: reading.kind,
        features: reading.features,
        context_fields: reading.context_fields,
        time: reading.time,
        body: reading.body,
        wants: reading.wants,
        attempts: reading.attempts,
        car_fault: reading.car_fault,
        sense: reading.sense,
        numbers: reading.numbers,
        entities: reading.entities,
        person_names,
        topics: reading.topics,
        subject_topic: reading.subject_topic,
        reference: reading.reference,
        someone_new: reading.someone_new,
        animal_in_hand: reading.animal_in_hand,
        topic: reading.topic.map(|(topic, _)| topic),
        topic_score: reading.topic.map_or(0, |(_, score)| score),
        unsure_words,
    }
}
