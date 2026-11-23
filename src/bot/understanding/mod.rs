// Changed in the cppalliance fork of RustBot: a reading gives the time of its turn. See "Changes in this fork" in README.md.

mod asked_again;
pub(crate) mod attempts;
pub(crate) mod body;
pub(crate) mod breeds;
pub(crate) mod car_glass;
pub(crate) mod cars;
pub(crate) mod confusion;
pub(crate) mod context;
pub(crate) mod details;
pub(crate) mod entities;
pub(crate) mod intent;
pub(crate) mod kinds;
pub(crate) mod line;
pub(crate) mod normalize;
pub(crate) mod numbers;
mod ownership;
mod pre_read;
mod presence;
mod reconsider;
pub(crate) mod references;
pub(crate) mod senses;
pub(crate) mod topics;

/// The raw line is retained for wording and fact extraction. Semantic readers use the folded clauses.
#[derive(Debug)]
pub(crate) struct Reading<'a> {
    stand_in_noun: Option<String>,
    observed_words: bool,
    pub raw: &'a str,
    pub clean: String,
    original_apostrophes: Option<String>,
    pub confusion: Option<confusion::Confusion>,
    pub kind: kinds::Kind,
    pub features: kinds::KindFeatures,
    pub context_fields: ContextFields,
    pub time: TimeFields,
    pub body: Option<body::Part>,
    pub wants: Option<&'static str>,
    pub line: line::Line,
    pub attempts: attempts::Attempts,
    pub car_fault: bool,
    pub sense: std::collections::BTreeMap<&'static str, &'static str>,
    pub numbers: Vec<numbers::Quantity>,
    pub entities: Vec<entities::Entity>,
    pub topics: Vec<(&'static str, i32)>,
    pub subject_topic: Option<&'static str>,
    pub reference: references::References,
    pub someone_new: bool,
    pub animal_in_hand: Option<String>,
    pub topic: Option<(&'static str, i32)>,
    pub own_topic: Option<(&'static str, i32)>,
}

#[derive(Debug)]
pub(crate) struct TimeFields(pub(super) kinds::TimeDependence);
impl serde::Serialize for TimeFields {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut out = ser.serialize_struct("TimeFields", 2)?;
        out.serialize_field("over_time", &self.0.value())?;
        out.serialize_field("over_time_what", &self.0.what())?;
        out.end()
    }
}
#[derive(Debug, serde::Serialize)]
pub(crate) struct ContextFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) asked_again: Option<String>,
    pub(crate) errand: Option<String>,
    pub(crate) topic_from: Vec<(String, i32)>,
    pub(crate) topic_is_a_brush: bool,
    pub(crate) subject_unclear: Option<&'static str>,
    pub(crate) subject: Option<String>,
    #[serde(skip)]
    pub(crate) question_scene: Option<context::QuestionScene>,
    pub(crate) presence: bool,
    pub(crate) opening: bool,
    pub(crate) asks_how_i_am: bool,
    pub(crate) taking_up: Option<kinds::Interest>,
    pub(crate) shift: Option<kinds::Shift>,
    pub(crate) care_home: Option<kinds::CareHome>,
    pub(crate) setup: Option<String>,
    pub(crate) routine: bool,
}
#[derive(Clone, Copy, PartialEq)]
enum Stage {
    Lexical,
    Conversational,
}
impl<'a> Reading<'a> {
    pub(crate) fn new(raw: &'a str) -> Self {
        Self::in_context(raw, std::iter::empty(), None, &context::History::default())
    }

    /// The time of the turn that this reading belongs to: the time the bot was given with
    /// `Bot::set_now`, or the system clock. Every reading of one turn gives the same time.
    pub(crate) fn now(&self) -> jiff::Zoned {
        crate::bot::clock::now()
    }
    pub(crate) fn for_inspection(raw: &'a str) -> Self {
        Self::with_line(
            raw,
            line::normalise(raw),
            cars::is_subject(raw, std::iter::empty()),
            None,
            &context::History::default(),
            Stage::Lexical,
        )
    }

    pub(crate) fn in_context<'b>(
        raw: &'a str,
        earlier: impl DoubleEndedIterator<Item = &'b str> + Clone,
        previous_wants: Option<&str>,
        history: &context::History,
    ) -> Self {
        Self::read_line(raw, raw, earlier, previous_wants, history)
    }

    pub(crate) fn understand<'b>(
        raw: &'a str,
        earlier: impl DoubleEndedIterator<Item = &'b str> + Clone,
        previous_wants: Option<&str>,
        history: &context::History,
    ) -> Self {
        let reread =
            |meaning: &str| Self::read_line(raw, meaning, earlier.clone(), previous_wants, history);
        if let Some(state) = reconsider::intro(raw) {
            return reread(&state);
        }
        let mut read = Self::in_context(raw, earlier.clone(), previous_wants, history);
        if let Some((errand, at_head)) = reconsider::errand(raw) {
            let mut alternative = reread(errand);
            if alternative.reaches()
                && (!read.reaches() || at_head && reconsider::changes(&alternative.line.clean))
            {
                alternative.line.fixes = read.line.fixes;
                alternative.context_fields.errand = Some(errand.to_owned());
                return alternative;
            }
            if at_head && !read.reaches() {
                read.context_fields.errand = Some(errand.to_owned());
                return read;
            }
        }
        if let Some(completed) = reconsider::fault(raw, earlier.clone()) {
            let alternative = reread(&completed);
            if alternative.reaches() {
                return alternative;
            }
        }
        if let Some((completed, noun)) = reconsider::borrow_noun(raw, &read, earlier.clone()) {
            let mut alternative = reread(&completed);
            if alternative.reaches() {
                alternative.stand_in_noun = Some(noun);
                return alternative;
            }
        }
        if let Some(said) = asked_again::replacement(&read, earlier.clone()) {
            let mut alternative = reread(&said);
            alternative.line.fixes = read.line.fixes;
            alternative.reference = read.reference;
            alternative.line.text = read.line.text;
            alternative.context_fields.asked_again = Some(said);
            return alternative;
        }
        if !read.reaches()
            && let Some((corrected, changes)) = reconsider::confused(raw)
        {
            let mut alternative = reread(&corrected);
            if alternative.reaches() {
                alternative.line.fixes.extend(changes);
                return alternative;
            }
        }
        read
    }

    fn reaches(&self) -> bool {
        self.topic.is_some()
            || self.features.emotion.is_some()
            || self.body.is_some()
            || !self.numbers.is_empty()
            || self.features.summons.is_some()
            || self.context_fields.presence
    }

    fn read_line<'b>(
        raw: &'a str,
        meaning: &str,
        earlier: impl DoubleEndedIterator<Item = &'b str> + Clone,
        previous_wants: Option<&str>,
        history: &context::History,
    ) -> Self {
        let car_fault = cars::is_subject(meaning, earlier.clone());
        let line = line::normalise(&pre_read::prepare(meaning));
        let mut reading = Self::with_line(
            raw,
            line,
            car_fault,
            previous_wants,
            history,
            Stage::Conversational,
        );
        let previous = earlier.clone().next_back().zip(history.last_time);
        if let Some(time) = kinds::over_time_followup(
            &reading.line,
            reading.kind,
            &reading.features,
            meaning,
            reading.reference.followup,
            previous,
        ) {
            reading.time = TimeFields(time);
        }
        if reading.context_fields.care_home.is_none() {
            reading.context_fields.care_home = kinds::a_home_carried_forward(
                &reading.line.clean,
                earlier.rev().filter(|s| !s.trim().is_empty()).take(8),
            );
        }
        reading
    }

    fn with_line(
        raw: &'a str,
        mut line: line::Line,
        car_fault: bool,
        previous_wants: Option<&str>,
        history: &context::History,
        stage: Stage,
    ) -> Self {
        let (mut kind, mut features) = kinds::read(&line);
        if stage == Stage::Conversational
            && kind == kinds::Kind::Thanks
            && let Some(rest) = details::after_courtesy(&line)
        {
            line = rest;
            (kind, features) = kinds::read(&line);
        }
        let folded = raw
            .split(crate::text::is_space)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();
        let (clean, original_apostrophes) = if folded.contains(['’', '‘', 'ʼ']) {
            (folded.replace(['’', '‘', 'ʼ'], "'"), Some(folded))
        } else {
            (folded, None)
        };
        let body = body::read(&line.norm_text);
        let numbers = numbers::detect(&line);
        let entities = entities::detect(&line);

        let sense = senses::read(&line);
        let attempts = attempts::read(&line.norm);
        let confusion = confusion::read(&line.norm_text);
        let linked = if stage == Stage::Conversational {
            let tail = kinds::after_any_link(&line.norm);
            (!tail.is_empty())
                .then(|| {
                    let narrow = line::normalise(&tail.join(" "));
                    let subjects = entities::detect(&narrow);
                    let numbers = numbers::detect(&narrow);
                    (narrow, subjects, numbers)
                })
                .filter(|(n, e, q)| topics::rank(n, e, q, None).1.is_some())
        } else {
            None
        };
        let (topic_line, entities, topic_numbers) = match linked.as_ref() {
            Some((n, e, q)) => (n, e.clone(), q.as_slice()),
            None => (&line, entities, numbers.as_slice()),
        };
        if stage == Stage::Conversational
            && kind == kinds::Kind::Statement
            && features.asks.is_empty()
            && entities.iter().any(|e| e.kind == "place")
            && kinds::noun_phrase_request(&line)
        {
            features.intent = kinds::Intent::Request;
            features.asks.push("recommend");
            features.replies_to_bot = false;
        }
        let asks = (stage == Stage::Conversational)
            .then_some(kind == kinds::Kind::Question || features.intent == kinds::Intent::Request);
        let (mut topics, subject_topic) =
            topics::rank_for_ask(topic_line, &entities, topic_numbers, None, asks);
        let wants = intent::wants(
            &line,
            kind,
            previous_wants.and_then(intent::Wants::from_str),
        )
        .map(intent::Wants::as_str);
        let own = topics.first().copied();
        let (reference, someone_new, animal_in_hand, inherited) =
            history.resolve(&line, kind, &entities, own.map(|t| t.0), subject_topic);
        let mut topic = if let Some(previous) = inherited {
            topics::rank_for_ask(topic_line, &entities, topic_numbers, Some(previous), asks)
                .0
                .first()
                .copied()
                .or(Some((previous, 0)))
        } else {
            own
        };
        let question_scene = if kind == kinds::Kind::Question || !features.asks.is_empty() {
            context::question_scene(topic_line, &entities)
        } else {
            None
        };
        if stage == Stage::Conversational {
            if let Some(plain) = ownership::plain(&line.plain, &entities) {
                line.plain = plain;
            }
            let derived = if topic.is_none()
                && !kinds::opening_line(&line.norm)
                && ownership::about_bot(&line, features.intent, kind, &entities)
            {
                Some("about-the-bot")
            } else if topic.is_none()
                && matches!(
                    features.intent,
                    kinds::Intent::Greeting | kinds::Intent::Smalltalk
                )
            {
                topic = Some(("small-talk", 0));
                None
            } else if topic.is_none()
                && features.state_owner == Some(kinds::StateOwner::Speaker)
                && features.emotion.is_some()
                && entities.is_empty()
                && !matches!(
                    features.intent,
                    kinds::Intent::Question | kinds::Intent::Request
                )
            {
                Some("feelings")
            } else if kind == kinds::Kind::Question
                && topic.is_some_and(|t| t.0 == "small-talk")
                && topics.iter().any(|t| t.0 == "nature-and-weather")
            {
                Some("nature-and-weather")
            } else {
                None
            };
            if let Some(derived) = derived {
                let score = topic.map_or(0, |t| t.1);
                let ranked_score = topics.iter().find(|t| t.0 == derived).map_or(0, |t| t.1);
                topic = Some((derived, score));
                topics.retain(|t| t.0 != derived);
                topics.insert(0, (derived, ranked_score));
            }
        }
        let time = TimeFields(kinds::over_time(&line, kind, &features));
        let presence = presence::reads(&line.norm, features.intent);
        let opening = !presence
            && matches!(
                features.intent,
                kinds::Intent::Question
                    | kinds::Intent::Request
                    | kinds::Intent::News
                    | kinds::Intent::Statement
                    | kinds::Intent::Ack
            )
            && ((topics.is_empty() && kinds::opening_line(&line.norm))
                || (entities.is_empty()
                    && history.opens_conversation()
                    && matches!(
                        features.intent,
                        kinds::Intent::Question | kinds::Intent::Request
                    )
                    && kinds::asks_with_nothing_named(&line.norm))
                || kinds::talk_frame(&line.norm));
        let topic_from = topics::evidence(&line, topic.map(|t| t.0));
        let subject = entities
            .iter()
            .find(|e| matches!(e.kind, "title" | "name" | "person" | "place" | "thing"))
            .map(|e| e.text.to_lowercase());
        let context_fields = ContextFields {
            asked_again: None,
            errand: None,
            subject,
            question_scene,
            subject_unclear: topics::unclear_subject(&line.norm, inherited, history.cues),
            topic_is_a_brush: topic_from.iter().all(|t| t.1 < 2),
            topic_from,
            presence,
            opening,
            asks_how_i_am: !presence
                && !opening
                && kinds::asks_how_i_am(&line.norm, features.intent),
            taking_up: kinds::taking_up(&line),
            shift: kinds::shift_worked(&line.clean),
            care_home: kinds::a_care_home(&line.clean),
            setup: kinds::setup(&line),
            routine: kinds::is_a_routine(&line, kind, &features.asks),
        };
        if presence && stage == Stage::Conversational {
            kind = kinds::Kind::Question;
        }
        Self {
            stand_in_noun: None,
            observed_words: false,
            raw,
            clean,
            confusion,
            features,
            context_fields,
            time,
            kind,
            body,
            wants,
            attempts,
            car_fault,
            sense,
            numbers,
            entities,
            topics,
            subject_topic,
            reference,
            someone_new,
            animal_in_hand,
            topic,
            own_topic: own,
            line,
            original_apostrophes,
        }
    }
    /// Replace the lexical view read by later responders after an observer fills a subject in.
    pub(crate) fn observe_words(&mut self, text: String, words: Vec<String>) {
        self.line.clean = text.clone();
        self.line.norm_text = text.clone();
        self.clean = text;
        self.line.words = words;
        self.original_apostrophes = None;
        self.observed_words = true;
    }

    pub(crate) fn lexical_words(&self) -> Option<&[String]> {
        self.observed_words.then_some(&self.line.words)
    }

    pub(crate) fn folded(&self) -> &str {
        self.original_apostrophes.as_deref().unwrap_or(&self.clean)
    }
}

#[cfg(test)]
#[path = "../../../tests/fixtures/conversation_reading.rs"]
mod reference;
#[test]
fn matches_pinned_conversational_readings() {
    use serde_json::{Value, json};
    let expected = reference::EXPECTED
        .iter()
        .map(|r| serde_json::from_str::<Value>(r).unwrap())
        .collect::<Vec<_>>();
    let mut mismatches = Vec::new();
    for &(text, at) in reference::CASES {
        let r = Reading::new(text);
        let actual = json!({"kind":r.kind,"presence":r.context_fields.presence,"norm":r.line.norm,"asks":r.features.asks,"topic_own":r.own_topic.map(|p|p.0),"topic_own_score":r.own_topic.map_or(0,|p|p.1)});
        for (field, want) in expected[at].as_object().unwrap() {
            if &actual[field] != want {
                mismatches.push(format!(
                    "{text:?} {field} expected {want}, got {}",
                    actual[field]
                ));
            }
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} mismatches:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}
