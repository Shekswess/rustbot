//! Named event dates and date corrections, from PyBot's bot/memory/when.py.
use super::again;
use crate::bot::{Memory, Reading};
use regex::{Captures, Regex};
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $regex:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($regex).unwrap());
    };
}

const WEEKDAYS: &str = "monday|tuesday|wednesday|thursday|friday|saturday|sunday";
const MONTHS: &str =
    "january|february|march|april|may|june|july|august|september|october|november|december";
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const DAY_NAMES: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];
const EVENTS: &str = "dentist(?:'s)?(?: appointment)?|doctor(?:'s)?(?: appointment)?|hospital appointment|appointment|job interview|interview|driving test|exam|test|meeting|check[- ]?up|wedding|birthday|operation|surgery|flight|trip|holiday|vacation|train|delivery|lesson|deadline|house move|new job";
const MOVED: &str =
    "moved|changed|rescheduled|put back|rebooked|pushed|shifted|switched|bumped|rearranged";

pattern!(
    STATEMENT,
    &format!(
        r"^(?:(?P<owner>my|our|the|i have an?|i've got an?|we have an?|we've got an?) )?(?P<event>{EVENTS})(?:\s+(?P<rest>.+))?$"
    )
);
pattern!(
    DATE_PREDICATE,
    r"^(?:(?:is|was|will be|has been|is now|is actually)? ?(?:due|set|booked|scheduled|expected) |(?:is|was|will be|is now|is actually) )"
);
pattern!(
    DATED_NOTICE,
    r"^(?P<notice>(?:reminder|paperwork|letter|text|email|notice) (?:came|arrived|was sent|was delivered)) (?P<timing>.+)$"
);
pattern!(
    MOVE_PRONOUN,
    &format!(
        r"^(?:i|we|they|he|she|the [a-z]+) (?:{MOVED}) (?:it|that|this|them) (?:back |forward |ahead |earlier |later )?(?:to|until|till|for) (?P<time>.*?)(?: (?:because|cos|cause|coz|since|so|as) .+)?$"
    )
);
pattern!(
    TRIP,
    r"^(?:(?:i|we)(?:'m|'re|'ve| am| are| have)?|im) (?:going|flying|travelling|traveling|travel|fly) (?:out )?to (?P<rest>.+)$"
);
pattern!(
    TRIP_ASK,
    r"^(?:when|what day) (?:(?:am i|are we|do i|do we|will i|will we) (?:go(?:ing)?|fl(?:y|ying)|travell?ing|leave))(?: to (?P<place>.+?))?(?: again| now)?$"
);
pattern!(
    PLAN,
    r"^(?:i|we)(?:'m|'re| am| are)? (?P<action>mov(?:e|ed|ing)|start(?:ed|ing)?|begin(?:ning)?|finish(?:ed|ing)?)(?: (?P<subject>house|work|(?:my|our) (?:new )?job))? (?P<timing>.+)$"
);
pattern!(
    PLAN_ASK,
    r"^(?:when|what day|what date) (?:am i|are we|do i|do we|did i|did we|will i|will we) (?P<action>mov(?:e|ing)|start(?:ing)?|begin(?:ning)?|finish(?:ing)?)(?: (?:house|work|(?:my|our) (?:new )?job))?(?: now| again| exactly)?$"
);
pattern!(
    RECENT_PLAN_ASK,
    r"^when (?:are|were|was|am) (?:we|i) (?:off|going|away|moving|flying|travelling|traveling)\b"
);
pattern!(OPENING, r"^(?:(?:and|so|but|oh|right|also|ok|okay)[ ,]+)*");
pattern!(
    ASK,
    &format!(
        r"^(?P<kind>when(?:'s|s| is| are| was)|what (?:day|date|time)(?: is| was| does)?|which day(?: is| was)?) (?:(?:my|our|the|a) )?(?P<event>{EVENTS}|it|that|this)(?: (?:on|at|again|now|then|exactly))?$"
    )
);
pattern!(
    QUESTION,
    r"^(?:what|where|when|who|why|how|which|do|does|did|is|are|was|were|has|have|can|could|should|would|will|am)\b"
);
pattern!(
    CORRECTION,
    &format!(
        r"^(?:(?:actually|sorry|no|wait|i meant|make that|scratch that)[ ,]+(?P<changed>.+)|(?P<moved>(?:it's|its|it is|that's|thats|that is) (?:(?:(?:already |just )?been )?(?:{MOVED})|now) .+))$"
    )
);
pattern!(
    CORRECTION_WORDS,
    &format!(
        r"^(?:(?:it's|its|it is|that's|thats|that is) )?(?:(?:(?:(?:already |just )?been )?(?:{MOVED})|now) )?"
    )
);
pattern!(
    CALENDAR,
    r"^what (?:day|date|time) is it(?: (?:again|now|then|exactly))?$"
);
pattern!(
    WITHDRAWN,
    r"\b(?:isn't|isnt|wasn't|wasnt|is not|was not|won't|wont|not on|cannot|no longer|maybe|might|whether|could be|may (?:be|not)|instead of|cancell?ed)\b"
);
pattern!(
    DATE,
    &format!(
        concat!(
            r"(?:(?P<join>on|in|by|for|to) )?\b",
            r"(?:(?P<relative>(?:this|next|last) (?:{weekdays}|{months}|week|month|year)|today|tomorrow|tonight|yesterday)|",
            r"(?P<ordinal>(?:(?:the )?(?:0?[1-9]|[12][0-9]|3[01])(?:st|nd|rd|th)? (?:of )?(?:{months})|",
            r"(?:{months}) (?:0?[1-9]|[12][0-9]|3[01])(?:st|nd|rd|th)?|",
            r"(?:the )?(?:0?[1-9]|[12][0-9]|3[01])(?:st|nd|rd|th)))|",
            r"(?P<day>{weekdays})|(?:(?P<month_modifier>early|mid|late) )?(?P<month>{months}))\b"
        ),
        weekdays = WEEKDAYS,
        months = MONTHS
    )
);
pattern!(
    DAY_MONTH,
    &format!(
        r"^(?:(?P<month_first>{MONTHS}) )?(?:the )?(?P<day>\d{{1,2}})(?P<suffix>st|nd|rd|th)?(?: (?:of )?(?P<month_last>{MONTHS}))?$"
    )
);
pattern!(
    CLOCK,
    r"\b(?:at|around|about) (?P<clock>(?:[01]?[0-9]|2[0-3])(?:[:.][0-5][0-9])?(?:\s*(?:a\.?m|p\.?m))?|(?:half past |quarter (?:past|to) )?(?:one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)|noon|midday|midnight)\b"
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EventType {
    Dentist,
    Doctor,
    Appointment,
    Interview,
    Exam,
    DrivingTest,
    Meeting,
    Checkup,
    Wedding,
    Birthday,
    Operation,
    Journey,
    Flight,
    Train,
    Delivery,
    Lesson,
    Deadline,
    Move,
    Start,
    Finish,
}

impl EventType {
    fn of(word: &str) -> Option<Self> {
        Some(match word {
            "dentist" | "dentist's" | "dentist appointment" | "dentist's appointment" => {
                Self::Dentist
            }
            "doctor"
            | "doctor's"
            | "doctor appointment"
            | "doctor's appointment"
            | "hospital appointment" => Self::Doctor,
            "appointment" => Self::Appointment,
            "job interview" | "interview" => Self::Interview,
            "driving test" => Self::DrivingTest,
            "exam" | "test" => Self::Exam,
            "meeting" => Self::Meeting,
            "checkup" | "check-up" | "check up" => Self::Checkup,
            "wedding" => Self::Wedding,
            "birthday" => Self::Birthday,
            "operation" | "surgery" => Self::Operation,
            "trip" | "holiday" | "vacation" => Self::Journey,
            "flight" => Self::Flight,
            "train" => Self::Train,
            "delivery" => Self::Delivery,
            "lesson" => Self::Lesson,
            "deadline" => Self::Deadline,
            "house move" | "move" | "moving" | "moved" => Self::Move,
            "new job" | "start" | "started" | "starting" | "begin" | "beginning" => Self::Start,
            "finish" | "finished" | "finishing" => Self::Finish,
            _ => return None,
        })
    }

    fn requested(self, other: Self) -> bool {
        self == other
            || self == Self::Appointment
                && matches!(other, Self::Dentist | Self::Doctor | Self::Checkup)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Date {
    Weekday {
        day: usize,
        by: bool,
        bare: bool,
    },
    Month {
        month: usize,
        qualifier: Option<String>,
        by: bool,
    },
    Ordinal {
        day: u8,
        month: Option<usize>,
        by: bool,
    },
    NumericDay {
        day: u8,
        month: usize,
        month_first: bool,
        by: bool,
    },
    Relative(String),
}

impl Date {
    fn phrase(&self) -> String {
        match self {
            Self::Weekday { day, by, .. } => {
                format!("{} {}", if *by { "by" } else { "on" }, DAY_NAMES[*day])
            }
            Self::Month {
                month,
                qualifier,
                by,
            } => {
                let join = if *by { "by" } else { "in" };
                let name = MONTH_NAMES[*month];
                qualifier.as_ref().map_or_else(
                    || format!("{join} {name}"),
                    |word| format!("{join} {word} {name}"),
                )
            }
            Self::Ordinal { day, month, by } => {
                let month =
                    month.map_or_else(String::new, |month| format!(" of {}", MONTH_NAMES[month]));
                format!("{}{month}", ordinal_phrase(*day, *by))
            }
            Self::NumericDay {
                day,
                month,
                month_first,
                by,
            } => {
                let join = if *by { "by" } else { "on" };
                let month = MONTH_NAMES[*month];
                if *month_first {
                    format!("{join} {month} {day}")
                } else {
                    format!("{join} {day} {month}")
                }
            }
            Self::Relative(text) => text
                .split_whitespace()
                .map(|word| {
                    find_label(&DAY_NAMES, word)
                        .map(|day| DAY_NAMES[day])
                        .or_else(|| find_label(&MONTH_NAMES, word).map(|month| MONTH_NAMES[month]))
                        .unwrap_or(word)
                })
                .collect::<Vec<_>>()
                .join(" "),
        }
    }

    fn month(&self) -> Option<usize> {
        match self {
            Self::Month { month, .. } | Self::NumericDay { month, .. } => Some(*month),
            Self::Ordinal { month, .. } => *month,
            _ => None,
        }
    }
}

fn ordinal_phrase(day: u8, by: bool) -> String {
    let suffix = match day {
        11..=13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };
    format!("{} the {day}{suffix}", if by { "by" } else { "on" })
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Timing {
    date: Option<Date>,
    clock: Option<String>,
}

impl Timing {
    fn merge(&mut self, update: Self) {
        if let Some(date) = update.date {
            self.date = Some(match (self.date.take(), date) {
                (
                    Some(Date::NumericDay {
                        month: old_month,
                        month_first,
                        by,
                        ..
                    }),
                    Date::Ordinal { day, month, .. },
                ) => Date::NumericDay {
                    day,
                    month: month.unwrap_or(old_month),
                    month_first,
                    by,
                },
                (
                    Some(Date::NumericDay {
                        day,
                        month_first,
                        by,
                        ..
                    }),
                    Date::Month { month, .. },
                ) => Date::NumericDay {
                    day,
                    month,
                    month_first,
                    by,
                },
                (Some(Date::Ordinal { day, by, .. }), Date::Month { month, .. }) => Date::Ordinal {
                    day,
                    month: Some(month),
                    by,
                },
                (
                    before,
                    Date::Ordinal {
                        day,
                        month: None,
                        by,
                    },
                ) => Date::Ordinal {
                    day,
                    month: before.as_ref().and_then(Date::month),
                    by,
                },
                (_, date) => date,
            });
        }
        if update.clock.is_some() {
            self.clock = update.clock;
        }
    }

    fn empty(&self) -> bool {
        self.date.is_none() && self.clock.is_none()
    }

    fn full(&self, clock_first: bool) -> Option<String> {
        match (self.date.as_ref(), self.clock.as_deref()) {
            (Some(date), Some(clock)) if clock_first => {
                Some(format!("at {clock} {}", date.phrase()))
            }
            (Some(date), Some(clock)) => Some(format!("{} at {clock}", date.phrase())),
            (Some(date), None) => Some(date.phrase()),
            (None, Some(clock)) => Some(format!("at {clock}")),
            (None, None) => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Wording {
    Your(String),
    YourNotice(String),
    Have { article: String, item: String },
    Named(String),
    Travel { place: String },
    Plan(String),
}

impl Wording {
    fn full(&self, timing: &str) -> String {
        match self {
            Self::Your(item) => format!("your {item} is {timing}"),
            Self::YourNotice(notice) => format!("your {notice} {timing}"),
            Self::Have { article, item } => format!("you have {article} {item} {timing}"),
            Self::Named(item) => format!("the {item} is {timing}"),
            Self::Travel { .. } => timing.to_owned(),
            Self::Plan(action)
                if action
                    .split_whitespace()
                    .next()
                    .is_some_and(|word| word.ends_with("ing")) =>
            {
                format!("you're {action} {timing}")
            }
            Self::Plan(action) => format!("you {action} {timing}"),
        }
    }

    fn matches_place(&self, target: Option<&str>) -> bool {
        match self {
            Self::Travel { place } => {
                target.is_none_or(|target| place.eq_ignore_ascii_case(target))
            }
            _ => target.is_none(),
        }
    }
}

struct Event {
    kind: EventType,
    wording: Wording,
    timing: Timing,
}

impl Event {
    fn sentence(&self, clock_first: bool) -> Option<String> {
        if let (
            None,
            Some(Date::Weekday {
                day, bare: true, ..
            }),
        ) = (&self.timing.clock, &self.timing.date)
            && matches!(self.wording, Wording::Your(_) | Wording::Named(_))
        {
            return Some(self.wording.full(DAY_NAMES[*day]));
        }
        Some(self.wording.full(&self.timing.full(clock_first)?))
    }

    fn clock_correction(&self) -> Option<String> {
        let timing = self.timing.full(true)?;
        match &self.wording {
            Wording::Your(item) | Wording::Named(item) => {
                Some(format!("{item} {}", timing.to_lowercase()))
            }
            _ => self.sentence(true),
        }
    }
}

#[derive(Default)]
pub(super) struct Schedule {
    entries: Vec<Event>,
    focus: Option<(usize, usize)>,
    corrected: Option<(usize, bool)>,
}

fn find_label(labels: &[&str], word: &str) -> Option<usize> {
    labels
        .iter()
        .position(|label| label.eq_ignore_ascii_case(word))
}

fn capture<'a>(groups: &Captures<'a>, name: &str) -> Option<&'a str> {
    groups.name(name).map(|value| value.as_str())
}

fn read_date(groups: &Captures<'_>) -> Option<Date> {
    let join = capture(groups, "join");
    let by = join == Some("by");
    if let Some(day) = capture(groups, "day").and_then(|day| find_label(&DAY_NAMES, day)) {
        return Some(Date::Weekday {
            day,
            by,
            bare: join.is_none(),
        });
    }
    if let Some(month) = capture(groups, "month").and_then(|month| find_label(&MONTH_NAMES, month))
    {
        return Some(Date::Month {
            month,
            qualifier: capture(groups, "month_modifier").map(str::to_owned),
            by,
        });
    }
    if let Some(spec) = capture(groups, "ordinal").and_then(|value| DAY_MONTH.captures(value)) {
        let day = capture(&spec, "day")?.parse().ok()?;
        let leading_month = capture(&spec, "month_first");
        let month = leading_month
            .or_else(|| capture(&spec, "month_last"))
            .and_then(|month| find_label(&MONTH_NAMES, month));
        if let Some(month) = month
            && capture(&spec, "suffix").is_none()
        {
            return Some(Date::NumericDay {
                day,
                month,
                month_first: leading_month.is_some(),
                by,
            });
        }
        return Some(Date::Ordinal { day, month, by });
    }
    capture(groups, "relative").map(|relative| Date::Relative(relative.to_owned()))
}

fn timing(line: &str) -> Timing {
    let clock = CLOCK
        .captures(line)
        .and_then(|found| capture(&found, "clock"))
        .map(|clock| clock.replace("a.m", "am").replace("p.m", "pm"));
    let date = DATE.captures(line).as_ref().and_then(read_date);
    Timing { date, clock }
}

fn only_timing(line: &str) -> Option<Timing> {
    let without_clock = CLOCK.replace(line, "");
    let remaining = DATE.replace(&without_clock, "");
    if !remaining.trim_matches([' ', ',', '.', ';']).is_empty() {
        return None;
    }
    let result = timing(line);
    (!result.empty()).then_some(result)
}

fn appointment_timing(line: &str) -> Option<Timing> {
    only_timing(&DATE_PREDICATE.replace(line, ""))
}

fn finish(text: &str) -> String {
    let mut chars = text.chars();
    let head: String = chars
        .next()
        .into_iter()
        .flat_map(char::to_uppercase)
        .collect();
    format!("{head}{}.", chars.as_str())
}

fn ask(line: &str) -> String {
    OPENING
        .replace(line.trim_end_matches(['.', '?', '!']), "")
        .trim()
        .to_owned()
}

impl Schedule {
    pub(super) fn observe(&mut self, reading: &Reading<'_>, turns: usize) {
        self.corrected = None;
        let line = ask(&reading.clean);
        if reading.clean.ends_with('?') || QUESTION.is_match(&line) {
            return;
        }
        if self.correct(&line, turns) {
            return;
        }
        let statement = CORRECTION
            .captures(&line)
            .as_ref()
            .and_then(|found| capture(found, "changed"))
            .unwrap_or(&line);
        for part in statement
            .split(';')
            .flat_map(|clause| clause.split(" and "))
        {
            self.observe_event(part.trim_matches([' ', ',']), turns);
        }
    }

    fn correct(&mut self, line: &str, turns: usize) -> bool {
        let ordinary = CORRECTION.captures(line);
        let move_ask = MOVE_PRONOUN.captures(line);
        let value = ordinary
            .as_ref()
            .and_then(|found| capture(found, "changed").or_else(|| capture(found, "moved")))
            .or_else(|| move_ask.as_ref().and_then(|found| capture(found, "time")));
        let Some(value) = value.filter(|value| !WITHDRAWN.is_match(value)) else {
            return false;
        };
        let value = CORRECTION_WORDS.replace(value, "");
        let value = value.trim_end_matches(" instead");
        let Some(mut update) = only_timing(value) else {
            return false;
        };
        let clock_only = update.clock.is_some() && update.date.is_none();
        // Clock corrections scan the stored plan list for a clock. The most recent plan can
        // be a separate one about the same event and contain no clock.
        let target = if clock_only {
            self.entries
                .iter()
                .rposition(|entry| entry.timing.clock.is_some())
        } else {
            self.focus.map(|(index, _)| index)
        };
        let Some(index) = target else {
            return false;
        };
        if let Some(Date::Weekday { bare, .. }) = &mut update.date {
            *bare = false;
        }
        let entry = &mut self.entries[index];
        let previous = entry.timing.clone();
        entry.timing.merge(update);
        if previous == entry.timing {
            return false;
        }
        self.focus = Some((index, turns));
        self.corrected = Some((index, clock_only));
        true
    }

    pub(super) fn move_trip_month(&mut self, place: &str, month: &str, turns: usize) {
        let update = timing(&format!("in {}", month.to_lowercase()));
        if update.empty() {
            return;
        }
        if let Some(index) = self.latest(EventType::Journey, Some(place)) {
            self.entries[index].timing.merge(update);
            self.focus = Some((index, turns));
        } else {
            self.save(
                EventType::Journey,
                Wording::Travel {
                    place: place.to_owned(),
                },
                update,
                turns,
            );
        }
    }

    fn observe_event(&mut self, line: &str, turns: usize) {
        if WITHDRAWN.is_match(line) || line.is_empty() {
            return;
        }
        if let Some(groups) = PLAN.captures(line)
            && let Some(kind) = capture(&groups, "action").and_then(EventType::of)
            && let Some(timing) = only_timing(&groups["timing"])
        {
            let mut action = groups["action"].to_owned();
            if let Some(subject) = capture(&groups, "subject") {
                action.push(' ');
                if let Some(item) = subject
                    .strip_prefix("my ")
                    .or_else(|| subject.strip_prefix("our "))
                {
                    action.push_str("your ");
                    action.push_str(item);
                } else {
                    action.push_str(subject);
                }
            }
            self.save(kind, Wording::Plan(action), timing, turns);
            return;
        }
        if let Some(groups) = TRIP.captures(line) {
            let rest = &groups["rest"];
            if let Some(found) = DATE.find(rest) {
                let place = rest[..found.start()].trim();
                if !place.is_empty() {
                    self.save(
                        EventType::Journey,
                        Wording::Travel {
                            place: place.into(),
                        },
                        timing(rest),
                        turns,
                    );
                }
            }
            return;
        }
        if let Some(groups) = STATEMENT.captures(line)
            && let Some(kind) = capture(&groups, "event").and_then(EventType::of)
            && let Some(rest) = capture(&groups, "rest")
        {
            let item = groups["event"].to_owned();
            if matches!(capture(&groups, "owner"), Some("my" | "our"))
                && let Some(notice) = DATED_NOTICE.captures(rest)
                && let Some(timing) = capture(&notice, "timing").and_then(only_timing)
            {
                self.save(
                    kind,
                    Wording::YourNotice(format!("{item} {}", &notice["notice"])),
                    timing,
                    turns,
                );
                return;
            }
            let Some(timing) = appointment_timing(rest) else {
                return;
            };
            let wording = match capture(&groups, "owner") {
                None | Some("my" | "our") => Wording::Your(item),
                Some("the") => Wording::Named(item),
                Some(owner) => Wording::Have {
                    article: if owner.ends_with(" an") { "an" } else { "a" }.into(),
                    item,
                },
            };
            self.save(kind, wording, timing, turns);
        }
    }

    fn save(&mut self, kind: EventType, wording: Wording, timing: Timing, turns: usize) {
        let index = if let Some(index) = self.entries.iter().rposition(|entry| {
            kind == entry.kind && wording == entry.wording && timing == entry.timing
        }) {
            index
        } else {
            self.entries.push(Event {
                kind,
                wording,
                timing,
            });
            self.entries.len() - 1
        };
        self.focus = Some((index, turns));
    }

    fn latest(&self, kind: EventType, place: Option<&str>) -> Option<usize> {
        self.entries
            .iter()
            .rposition(|entry| kind.requested(entry.kind) && entry.wording.matches_place(place))
    }

    fn named_target(&self, line: &str) -> Option<usize> {
        if let Some(journey) = TRIP_ASK.captures(line) {
            return self.latest(EventType::Journey, capture(&journey, "place"));
        }
        if let Some(plan) = PLAN_ASK.captures(line) {
            let kind = capture(&plan, "action").and_then(EventType::of)?;
            return self.latest(kind, None);
        }
        let asked = ASK.captures(line)?;
        let kind = EventType::of(&asked["event"])?;
        self.latest(kind, None)
    }

    pub(super) fn respond(&self, reading: &Reading<'_>, memory: &Memory) -> Option<String> {
        if let Some((index, clock_only)) = self.corrected {
            let event = self.entries.get(index)?;
            return Some(if clock_only {
                format!("Got it — {}", finish(&event.clock_correction()?))
            } else {
                format!("Got it — {}.", event.sentence(true)?)
            });
        }
        let line = ask(&reading.clean);
        if RECENT_PLAN_ASK.is_match(&line) {
            let event = self.entries.last()?;
            let reply = finish(&event.timing.date.as_ref()?.phrase());
            if memory
                .turns
                .last()
                .is_some_and(|turn| turn.bot.eq_ignore_ascii_case(&reply))
            {
                return Some(finish(&event.sentence(true)?));
            }
            return Some(reply);
        }
        if TRIP_ASK.is_match(&line) {
            let event = &self.entries[self.named_target(&line)?];
            return Some(finish(&event.timing.date.as_ref()?.phrase()));
        }
        if PLAN_ASK.is_match(&line) {
            let event = &self.entries[self.named_target(&line)?];
            let date = event.timing.date.as_ref()?.phrase();
            let reply = if event.kind == EventType::Start && line.starts_with("when") {
                finish(&event.wording.full(&date))
            } else {
                finish(&date)
            };
            return Some(again::frame_short(&reply, memory).unwrap_or(reply));
        }
        let read = ASK.captures(&line)?;
        let event = &read["event"];
        let pointing = matches!(event, "it" | "that" | "this");
        let kind = &read["kind"];
        if pointing && (CALENDAR.is_match(&line) || line.ends_with(" now")) {
            return None;
        }
        let index = if pointing {
            let (index, said) = self.focus?;
            if memory.turns.len().saturating_sub(said) > 3
                || memory.turns.last().is_some_and(|turn| {
                    matches!(turn.module, "safety" | "urgent" | "loss" | "grief")
                })
            {
                return None;
            }
            index
        } else {
            self.named_target(&line)?
        };
        let target = &self.entries[index];
        let reply = if kind.starts_with("what time") {
            if let Some(clock) = &target.timing.clock {
                finish(&format!("at {clock}"))
            } else if pointing {
                return None;
            } else {
                finish(&target.sentence(true)?)
            }
        } else if pointing
            || kind.starts_with("what day")
            || kind.starts_with("what date")
            || kind.starts_with("which day")
        {
            let date = target.timing.date.as_ref()?;
            match date {
                Date::NumericDay { .. } if !pointing => {
                    finish(&target.wording.full(&date.phrase()))
                }
                Date::Ordinal { day, by, .. } if !pointing => finish(&ordinal_phrase(*day, *by)),
                _ => finish(&date.phrase()),
            }
        } else {
            let clock_first = !event.contains("appointment") || event == "appointment";
            let sentence = target.sentence(clock_first)?;
            // The source's named-when branch completes the driving-test phrase from
            // the question, repeating its final noun.
            if event == "driving test" && target.kind == EventType::DrivingTest {
                finish(&sentence.replacen("driving test", "driving test test", 1))
            } else if matches!(event, "trip" | "holiday" | "vacation")
                && let Wording::Travel { place } = &target.wording
                && super::stated_plans::opens_on_a_place(place)
                && let Some(place) = super::personal::place(place)
            {
                finish(&format!("you're going to {place} {sentence}"))
            } else {
                finish(&sentence)
            }
        };
        Some(again::frame_short(&reply, memory).unwrap_or(reply))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn correcting_the_day_keeps_the_clock_and_does_not_move_another_event() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My dentist appointment is on Monday.");
        bot.turn("My interview is on Tuesday at 3 pm.");
        assert_eq!(
            bot.turn("Actually, it's on Thursday.").reply,
            "Got it — your interview is at 3 pm on Thursday."
        );
        assert_eq!(bot.turn("What time is my interview?").reply, "At 3 pm.");
        assert_eq!(
            bot.turn("What day is my dentist appointment?").reply,
            "On Monday."
        );
    }

    #[test]
    fn correcting_a_month_with_a_day_preserves_the_month_and_ignores_questions_as_records() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My exam is in June.");
        assert_eq!(
            bot.turn("Actually, it's the 12th.").reply,
            "Got it — your exam is on the 12th of June."
        );
        bot.turn("Is my exam on Saturday?");
        assert_eq!(
            bot.turn("When is my exam?").reply,
            "Your exam is on the 12th of June."
        );
    }

    #[test]
    fn keeps_compound_dates_separate_and_leaves_calendar_and_live_queries_to_other_modules() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My interview is on Tuesday and I have a dentist appointment on Friday.");
        assert_eq!(bot.turn("What day is my interview?").reply, "On Tuesday.");
        assert_eq!(
            bot.turn("What day is my dentist appointment?").reply,
            "On Friday."
        );
        assert_ne!(bot.turn("What day is it?").module, Some("memory_store"));
        assert_ne!(bot.turn("What day is it now?").module, Some("memory_store"));
        assert_ne!(
            bot.turn("When is the next train?").module,
            Some("memory_store")
        );
        bot.turn("My dentist appointment reminder came on Monday.");
        assert!(
            bot.turn("What day is my dentist appointment?")
                .reply
                .ends_with("On Monday.")
        );
    }

    #[test]
    fn bare_appointments_and_plans_keep_their_own_time_and_date() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("Dentist on Tuesday, at 3.");
        assert_eq!(bot.turn("What time is my dentist?").reply, "At 3.");
        bot.turn("I'm moving house in March.");
        assert_eq!(bot.turn("When am I moving?").reply, "In March.");
        bot.turn("I start my new job on Monday.");
        assert_eq!(
            bot.turn("When do I start?").reply,
            "You start your new job on Monday."
        );
    }

    #[test]
    fn separate_plan_records_and_clock_corrections_follow_upstream_order() {
        let mut bot = crate::Bot::new(Some("0")).unwrap();
        bot.turn("My interview is on Tuesday at 3 pm.");
        bot.turn("Actually, my interview is Thursday.");
        bot.turn("I moved my interview from Thursday to Friday.");
        let revised = bot.turn("Actually, it's at 4 pm.");
        assert!(revised.candidates.iter().any(|candidate| {
            candidate.module == "memory_store"
                && candidate.reply == "Got it — Interview at 4 pm on tuesday."
        }));
        assert_eq!(
            bot.turn("When is my interview?").reply,
            "Your interview is Thursday."
        );
    }
}
