//! Native port of bot/modules/a_day_somebody_else_keeps.py.
use crate::bot::{Memory, Reading};
use regex::Regex;
use std::sync::LazyLock;
static MY_CARE_ASK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?:^|[^a-z])(?:what time|when|which time|what hours)\b[^?]{0,40}?\b(?:my|our)\s+(?:son|daughter|kid|kids|child|children|boy|girl|little one|baby|niece|nephew|grandson|granddaughter|grandchild|grandchildren|childminder|babysitter|class|classes|lesson|lessons|course|club|swimming|training|nursery|school)\b[^?]{0,30}?\b(?:finish|finishes|start|starts|get out|gets out|break up|breaks up|come out|comes out|be done|is done|end|ends|go in|goes in|be picked up|gets picked up|get home|gets home|be home|is home|get there|gets there)\b"#).unwrap()
});
static MY_CARE_NOT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:bus|buses|train|trains|tram|tube|flight|flights|coach|ferry|traffic|motorway|road|roads|strike|delayed|cancelled|appointment|appointments|doctor|dentist|hospital|physio|clinic|surgery|nurse|pharmacy|chemist|tablets?|medicine|medication|exam|test results?|results?)\b"#).unwrap()
});
static THEIR_CLASS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:class|classes|lesson|lessons|course|club|swimming|training)\b"#).unwrap()
});
static A_CHILD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)\b(?:son|daughter|kid|kids|child|children|boy|girl|little one|baby|niece|nephew|grandson|granddaughter|grandchild|grandchildren|childminder|babysitter)\b"#).unwrap()
});
const SCHOOL_LINES: &[&str] = &[
    "A school day is usually the same length every day of the week, so the finish time they gave you in September still holds unless it is the last day of term, an inset day or a trip. The school office, its app or the newsletter has this term's version; give me the times and how long the journey takes and I'll work out when you need to set off.",
    "The times are set by the school or club, not by anything I can look up, but they rarely move inside a term - it is the last day of term, inset days and trips that shift them. Their office will tell you in one call and the app you sign them in on has the same; hand me the times once you have them and I'll plan the pick-up around the rest of your day.",
    "Those hours belong to the school or club, so I can't know this week's, but a school day holds steady through a term apart from the last day of it and any inset day - and those are on the school's own calendar. Tell me the finish time and the trip and the rest is arithmetic I can do with you.",
];
const CLASS_LINES: &[&str] = &[
    "A class time is set by whoever runs the course and it can move week to week - a new tutor, a session off, a room change - so the timetable you were given is the starting point rather than the answer. The provider's app, email or the notice on the door carries this week's; tell me the times and what you're fitting around them and I'll plan it with you.",
    "I can't see your timetable from here. What I can say is that classes move more often than people expect - a session cancelled, a tutor changed - and it is the provider's own message that carries the change. Bring me the times and I'll do the planning around them.",
    "The provider keeps that timetable, not me, and it changes: a bank holiday, a cancelled session, a new room. Their app or the email they send is where this week's version sits; give me the times and I'll work out the rest of your day around them.",
];

pub(in crate::bot) fn asks_about_personal_schedule(said: &str) -> bool {
    MY_CARE_ASK.is_match(said) && !MY_CARE_NOT.is_match(said)
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let said = reading.folded();
    if !asks_about_personal_schedule(said) {
        return None;
    }
    let pool = if THEIR_CLASS.is_match(said) && !A_CHILD.is_match(said) {
        CLASS_LINES
    } else {
        SCHOOL_LINES
    };
    let last = memory.turns.last().map(|turn| turn.bot.trim());
    let fresh: Vec<_> = pool
        .iter()
        .copied()
        .filter(|line| Some(*line) != last)
        .collect();
    Some(
        crate::bot::compose::seeded_choice(
            if fresh.is_empty() { pool } else { &fresh },
            memory,
            said,
        )
        .into(),
    )
}
