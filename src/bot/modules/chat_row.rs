//! Remember an argument and answer the next step while it is still the subject.
use crate::bot::{
    Memory, Reading, compose,
    knowledge::lookup::{self, KnowledgeMatch},
    memory::history::in_a_safety_thread,
};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

include!("chat_row_data.rs");
pattern!(PRONOUN, r"\b(?:her|him|them|she|he|they)\b");

#[derive(Clone, Copy)]
enum Person {
    Her,
    Him,
    Them,
}

impl Person {
    fn read(text: &str) -> Self {
        if FEMININE.is_match(text) {
            Self::Her
        } else if MASCULINE.is_match(text) {
            Self::Him
        } else {
            Self::Them
        }
    }

    fn pronouns(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Her => ("her", "she", "her"),
            Self::Him => ("him", "he", "his"),
            Self::Them => ("them", "they", "their"),
        }
    }
}

pub(in crate::bot) struct Row {
    person: Person,
    turn: usize,
    text: String,
    handed_over: bool,
}

impl Row {
    fn reported(text: &str, turn: usize) -> Option<Self> {
        ARGUMENT.find(text)?;
        if !CLOSE_PERSON.is_match(text) && DISTANT_PERSON.is_match(text)
            || ASK.find(text).is_some_and(|found| found.start() == 0)
            || text.trim_end().ends_with('?')
            || text.split_whitespace().count() > 18
            || LOSS.is_match(text)
        {
            return None;
        }
        Some(Self {
            person: Person::read(text),
            turn,
            text: text.to_owned(),
            handed_over: FALLING_OUT.is_match(text),
        })
    }

    fn still_current(&self, memory: &Memory) -> bool {
        memory.turns.len().saturating_sub(self.turn) <= 3
    }

    fn render(&self, template: &str) -> String {
        let (them, they, their) = self.person.pronouns();
        template
            .replace("{them}", them)
            .replace("{they}", they)
            .replace("{their}", their)
    }

    fn choose(&self, options: &[&str], memory: &Memory, text: &str) -> String {
        let lines: Vec<_> = options.iter().map(|line| self.render(line)).collect();
        let choices: Vec<_> = lines.iter().map(String::as_str).collect();
        compose::pick(&choices, memory, text).to_owned()
    }
}

pub(super) fn observe(reading: &Reading<'_>, memory: &mut Memory) {
    if let Some(row) = Row::reported(reading.raw, memory.turns.len())
        && !in_a_safety_thread(memory)
        && !crate::bot::modules::safety::has_medical_urgency(reading, memory)
        && !crate::bot::modules::safety::current_gas(&reading.line.clean)
    {
        memory.row = Some(row);
    }
}

fn clean(text: &str) -> String {
    text.to_lowercase()
        .replace('’', "'")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn reported_without_an_ask(text: &str, memory: &Memory) -> Option<Row> {
    let end = ARGUMENT.find(text)?.end();
    if ASK_AFTER.is_match(&text[end..]) {
        return None;
    }
    Row::reported(text, memory.turns.len())
}

fn card(text: &str) -> Option<KnowledgeMatch> {
    lookup::matched_card(&Reading::new(text))
}

fn card_answers_the_argument(text: &str, entry: &KnowledgeMatch) -> bool {
    EVENT_ID.is_match(entry.id)
        || entry.id.contains("fell-out")
        || SUBJECT
            .captures_iter(text)
            .any(|found| SUBJECT_PERSON.binary_search(&&found[1]).is_err())
}

fn close_people(text: &str) -> impl Iterator<Item = &str> {
    CLOSE_PERSON
        .find_iter(text)
        .map(|word| &word.as_str()[..word.len().min(4)])
}

fn names_the_person(text: &str, row: &Row) -> bool {
    if PRONOUN.is_match(text) {
        return true;
    }
    let lower = row.text.to_lowercase();
    let known: Vec<_> = close_people(&lower).collect();
    let mut named = close_people(text).peekable();
    named.peek().is_some() && (known.is_empty() || named.all(|person| known.contains(&person)))
}

fn ask_in_line(text: &str) -> &str {
    if let Some(hit) = ARGUMENT.find(text) {
        let tail = text[hit.end()..].trim();
        if !tail.is_empty() && ASK_AFTER.is_match(tail) {
            return tail;
        }
    }
    text
}

fn teenage_silence(text: &str) -> bool {
    QUIET.is_match(text) && !QUARREL.is_match(text) && CHILD.is_match(text) && TEEN.is_match(text)
}

fn only_teenage_silence(row: &Row, memory: &Memory) -> bool {
    teenage_silence(&row.text.to_lowercase())
        && !memory
            .turns
            .iter()
            .skip(row.turn)
            .any(|turn| QUARREL.is_match(&turn.user.to_lowercase()))
}

fn next(text: &str, row: &Row, memory: &Memory) -> Option<String> {
    let text = ask_in_line(text);
    if !row.still_current(memory) || text.split_whitespace().count() > 14 || LOSS.is_match(text) {
        return None;
    }
    let how = HOW.is_match(text);
    let space = SAID_SPACE.is_match(text);
    if how && row.handed_over
        || BARE_MOVE.is_match(text) && only_teenage_silence(row, memory)
        || !how
            && !space
            && !(CONTACT.is_match(text)
                && CONTACT_ASK.is_match(text)
                && names_the_person(text, row))
    {
        return None;
    }
    if let Some(entry) = card(text)
        && (card_answers_the_argument(text, &entry)
            || how && (WANTS_AN_APOLOGY.is_match(text) || ASKS_FOR_WORDS.is_match(text)))
    {
        return None;
    }
    let pool = if how {
        HOW_TO
    } else if space || SPACE.is_match(text) {
        PAUSE
    } else {
        NEXT
    };
    Some(row.choose(pool, memory, text))
}

fn ending(text: &str, row: &Row, memory: &Memory) -> Option<String> {
    let text = ask_in_line(text);
    if !row.still_current(memory)
        || !ENDING.is_match(text)
        || text.split_whitespace().count() > 14
        || LOSS.is_match(text)
    {
        return None;
    }
    let mut pool = END.to_vec();
    if HUNG_UP.is_match(text) {
        pool.extend(END_HUNG_UP);
    }
    Some(row.choose(&pool, memory, text))
}

pub(super) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    if in_a_safety_thread(memory) {
        return None;
    }
    let text = clean(reading.raw);
    let reported = reported_without_an_ask(&text, memory);
    if let Some(row) = &reported
        && card(&text).is_none_or(|entry| !card_answers_the_argument(&text, &entry))
    {
        return Some(row.choose(OPENING, memory, &text));
    }
    let row = reported.as_ref().or(memory.row.as_ref())?;
    next(&text, row, memory).or_else(|| ending(&text, row, memory))
}

pub(super) fn record_selected(reading: &Reading<'_>, reply: &str, memory: &mut Memory) {
    let text = clean(reading.raw);
    if let Some(reported) = opening_for_reply(&text, reply, memory) {
        memory.row = Some(reported);
    }
}

fn opening_for_reply(text: &str, reply: &str, memory: &Memory) -> Option<Row> {
    reported_without_an_ask(text, memory)
        .filter(|row| OPENING.iter().any(|line| row.render(line) == reply))
}

pub(super) fn yields_to_teenager_card(reading: &Reading<'_>, reply: &str, memory: &Memory) -> bool {
    let text = clean(reading.raw);
    teenage_silence(&text) && opening_for_reply(&text, reply, memory).is_some()
}

#[cfg(test)]
mod tests {
    use crate::Bot;

    #[test]
    fn remembers_the_next_argument_when_answers_won_both_openers() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(bot.turn("i argued with my mum").module, Some("answers"));
        assert_eq!(bot.memory.row.as_ref().unwrap().person.pronouns().0, "her");
        assert_eq!(
            bot.turn("i got into a huge argument with my husband")
                .module,
            Some("answers")
        );
        let row = bot.memory.row.as_ref().unwrap();
        assert_eq!(row.person.pronouns().0, "him");
        assert_eq!(row.turn, 1);
        assert!(
            !bot.turn("should i ring my mum")
                .candidates
                .iter()
                .any(|c| c.module == "chat")
        );
        let followup = bot.turn("should i call him now");
        assert_eq!(followup.module, Some("chat"));
        assert_eq!(
            followup.reply,
            "Say the thing you would be glad you said in a year, not the thing you have been rehearsing. And not after a drink, however much easier that feels."
        );
        bot.turn("ok");
        assert!(
            super::next(
                "should i call him now",
                bot.memory.row.as_ref().unwrap(),
                &bot.memory,
            )
            .is_none()
        );
    }

    #[test]
    fn a_late_ask_without_a_question_mark_still_records_the_report() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let reported = "i had a row with my mum what should i say";
        bot.turn(reported);
        assert_eq!(bot.memory.row.as_ref().unwrap().text, reported);
        bot.turn("what should i say about the row with my boss?");
        assert_eq!(bot.memory.row.as_ref().unwrap().text, reported);
    }

    #[test]
    fn the_falling_out_card_keeps_the_apology_but_leaves_its_ending_in_context() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(
            bot.turn("i fell out with my best mate").module,
            Some("answers")
        );
        assert!(bot.memory.row.as_ref().unwrap().handed_over);
        assert!(
            !bot.turn("how do i put it right")
                .candidates
                .iter()
                .any(|c| c.module == "chat")
        );
        let ending = bot.turn("she hung up on me");
        assert_eq!(ending.module, Some("chat"));
        assert_eq!(
            ending.reply,
            "That is the worst bit of it - being shut out leaves you with nothing to do with yourself. It usually means the night is needed, not that it is finished."
        );
    }

    #[test]
    fn the_teenage_silence_card_retains_its_turn() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let silence = bot.turn("my daughter is 14 and wont talk to me");
        assert_eq!(silence.module, Some("answers"));
        assert!(
            silence
                .reply
                .starts_with("Talk side by side rather than face to face")
        );
        assert!(
            silence
                .candidates
                .iter()
                .any(|candidate| candidate.module == "chat")
        );
        bot.turn("she spends all evening in her room");
        assert!(
            !bot.turn("what do i do")
                .candidates
                .iter()
                .any(|candidate| candidate.module == "chat")
        );
    }

    #[test]
    fn a_report_during_a_medical_emergency_does_not_begin_an_argument_thread() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let urgent = bot.turn("i had a fight with my dad and i cannot breathe right now");
        assert_eq!(urgent.module, Some("safety"));
        assert!(bot.memory.row.is_none());
        let followup = bot.turn("its every night");
        assert_eq!(followup.module, Some("safety"));
        assert!(!followup.candidates.iter().any(|c| c.module == "chat"));
    }
}
