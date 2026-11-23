// Changed in the cppalliance fork of RustBot: the time comes from the turn. See "Changes in this fork" in README.md.

//! Follow-up calculations for a savings goal stated over several turns.

use std::sync::LazyLock;

use jiff::civil::Date;
use regex::Regex;

use crate::bot::{Memory, Reading, knowledge::money_savings as savings};

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(
    SAVING,
    r"\b(?:sav(?:e[sd]?|ing|ings)|put(?:ting)? (?:by|away|aside)|put(?:ting)?\b[^.?]{0,18}?\b(?:by|away|aside)|set(?:ting)? aside|stash(?:ing)?|sock(?:ing)? away)\b"
);
pattern!(
    TARGET,
    r"\b(?:want|need|hope|aim|plan|trying|target|goal|afford|deposit)\b"
);
pattern!(
    HELD,
    r"\b(?:already|so far|in (?:my |our |the )?(?:savings|pot)|saved|in it)\b"
);
pattern!(
    POSITION,
    r"^(?:(?:and|so|but) +)?(?:i(?:'m|m| am)|we(?:'re|re| are)|(?:the|my|our) pot (?:is|stands)) (?:(?:only|already|currently) )?(?:on|at|up to|sitting at)\b"
);
pattern!(
    GAP,
    r"\b(?:how (?:far (?:off|away)|short) (?:am i|are we|is it)|how much (?:more|left|short) (?:do (?:i|we) (?:need|have)|have (?:i|we)|is there|to go)|what(?:'s| is) (?:left|the gap)|how much (?:is )?left(?: to (?:save|go|find))?\s*[?.!]*$)\b"
);
pattern!(
    RATE,
    r"(?:£\s*)?(\d[\d,]*(?:\.\d+)?)\s*(k|grand|thousand)?\s*(?:pounds?|quid)?\s*(?:a|per|each|every)\s+(month|week|fortnight|year)\b"
);
pattern!(
    RATE_ASK,
    r"\bhow (?:much|many)\b[^?.!]{0,24}\b(?:a|per|each) (month|week)\b|\bhow much (?:should|do|would) i (?:put|save|need)\b"
);
pattern!(
    EXTRA_ASK,
    r"^(?:(?:and|but|so|ok|okay|right|well)\b[ ,]*)*how much (?:more|extra) (?:(?:a|per|each) month|(?:would|do|will) (?:i|we) (?:need|have to))\b"
);
pattern!(
    ENOUGH,
    r"^(?:(?:and|but|so|ok|okay|right|well)\b[ ,]*)*(?:is|will|would|does|do|am i|are we) (?:that|this|it|they|we|i|(?:(?:putting by|saving|saved) )?£?\s*\d[\d,]*\s*(?:a|per|each) month) (?:be )?(?:enough|do(?: it)?|get (?:(?:me|us) )?there|cover(?: it)?)(?: (?:then|though|really|now|do you think|i wonder))?[.!?]*$"
);
pattern!(ADD, r"\b(?:another|extra|plus|more|on top)\b");
pattern!(
    RESTATED,
    r"^(?:(?:and|but|so|then|ok|okay|right)\b[ ,]*)*(?:(?:what|how) about |at |make it )?(?:£\s*)?\d[\d,]*(?:\.\d+)?\s*k?\s*(?:pounds?\s+)?(?:a|per|each) month(?: (?:then|now))?[.!]*$"
);
pattern!(
    BARE_RATE,
    r"^(?:(?:and|but|so|then|ok|okay|right)\b[ ,]*)*(?:(?:what|how) about |what if (?:i )?|if i )?(?:only )?(?:put|putting|set|setting|pay|paying) (?:(?:away|by|aside|in|up) )?(?:£\s*)?\d[\d,]*(?:\.\d+)?\s*k?\s*(?:pounds?|quid)?\s*(?:instead|then|now)?[.!?]*$"
);
pattern!(
    LENGTH,
    r"\b(?:how long|how soon|how many (?:months|years|weeks)|when (?:will|would|can|do|should) (?:i|we))\b"
);
pattern!(
    LENGTH_FOLLOWUP,
    r"^(?:(?:and|but|so|ok|okay|then|oh|right|yeah)\b[ ,]+)*(?:how long(?:(?: (?:will|would|does|did|is) (?:that|it|this)(?: (?:take|be))?)?(?: to (?:save|get)(?: (?:that|it|there|the money))?)?| (?:until|till|til|before) (?:i|we|it|that|this) (?:have|get|reach|hit|be|save).*)|how many (?:months|years|weeks)(?: is (?:that|it|this))?|how much time(?: (?:will|would|does) (?:that|it|this) take)?|when (?:will|would|can|could|should|might) (?:i|we|it|that|this) (?:have|get|be|reach|afford|save|hit|make|manage)\b.*)\s*[?.!]*$"
);
pattern!(
    BUY,
    r"\b(?:buy|get|afford|pay for) (?:it|that|one|(?:the|a|an|my) (\w+))\b"
);
pattern!(
    PRICE_REPLY,
    r"^(?:about |around |roughly |a |an |it(?:'s|s| is) )*(?:£\s*)?\d[\d,]*(?:\.\d+)?\s*(?:k|grand|thousand)?\s*(?:pounds?|quid)?(?: one)?[.!?]*$"
);
pattern!(
    CORRECTION,
    r"^(?:(?:actually|no|sorry|nope|rather|instead|oh|i mean|(?:make|change) (?:that|it)(?: to)?|it(?:'s|s| is)|that(?:'s|s| is))\b[ ,]*)+"
);
pattern!(
    EARNED,
    r"\b(?:earn(?:s|ed|ing)?|salary|wages?|income|pension|take home|paid|rent|bill|mortgage|spend(?:s|ing)?|costs?|pay(?:ing)? for)\b"
);
pattern!(
    INCOME,
    r"\b(?:earn(?:s|ed|ing)?|salary|wages?|income|take[- ]home|paid|payslip)\b"
);
pattern!(
    SPLIT_PURPOSE,
    r"\b(?:saving(?: up)? for|save(?: up)? for|deposit|pot)\b"
);
pattern!(
    SPLIT_PRICE,
    r"\b(?:need(?:s)?|aiming for|target is|costs?|(?:it|one|thing) (?:is|was)|it'?s a)\b|,\s*£?\d"
);
pattern!(
    ANOTHER_POT,
    r"^\s*(?:on|off|from) (?:my|the|our|a|an|each|every|it)\b"
);
pattern!(
    FURTHER,
    r"\b(?:another|(?:an|a) (?:extra|further))\s*£?\s*\d|\bon top of\b"
);
pattern!(
    OTHER_RATE,
    r"^\s*(?:(?:into|in|towards?|for) (?:a|an|my|our|the|his|her|their) |on (?:the |my |our )?|(?:rent|mortgage|bills?|loan|debt|insurance|childcare)\b)"
);
pattern!(
    DURATION,
    r"\b(?:for|over|in|within) (?:\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten)\s+(?:months?|years?|weeks?|fortnights?)\b"
);
pattern!(SUGGESTED_GOAL, r"(?:For £|the £|save £)([\d,]+)");
pattern!(WEEKLY_ANSWER, r"£([\d,]+) a week\b");
pattern!(CLAUSE_BREAK, r" and | but |, |\. ");

const MONTHS: [&str; 12] = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];
const OCCASIONS: &[(&str, i8)] = &[
    ("christmas", 12),
    ("xmas", 12),
    ("new year", 12),
    ("hogmanay", 12),
    ("halloween", 10),
    ("valentine", 2),
    ("easter", 4),
    ("black friday", 11),
    ("spring", 3),
    ("summer", 6),
    ("autumn", 9),
    ("fall", 10),
    ("winter", 12),
    ("the year", 12),
    ("next month", 0),
    ("this month", 0),
    ("next week", 0),
    ("this week", 0),
    ("next year", 0),
    ("this year", 0),
];
static DEADLINE: LazyLock<Regex> = LazyLock::new(|| {
    let when = MONTHS
        .into_iter()
        .chain(OCCASIONS.iter().map(|(name, _)| *name))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(
        r"(?P<lead>\b(?:by|before|until|till|for|in) )?(?P<end>(?:at )?(?:the )?end of )?(?:next |this |early |late )?(?P<label>{when})\b"
    )).expect("savings date")
});

#[derive(Clone, Copy)]
struct Contribution {
    amount: u64,
    period: &'static str,
}

impl Contribution {
    fn monthly(self) -> u64 {
        let annual = match self.period {
            "week" => 52,
            "fortnight" => 26,
            "year" => 1,
            _ => 12,
        };
        ((self.amount * annual) as f64 / 12.0).round_ties_even() as u64
    }

    fn reply(self, goal: u64) -> String {
        if self.period == "month" {
            return at_rate(goal, self.amount);
        }
        let count = goal.div_ceil(self.amount);
        format!(
            "At £{} a {}, £{} takes {}.",
            comma(self.amount),
            self.period,
            comma(goal),
            duration(count, self.period)
        )
    }
}

fn number(value: &str) -> Option<u64> {
    value.replace(',', "").parse().ok()
}

fn named_amount(value: &str) -> Option<u64> {
    savings::sums(value)
        .map(|(sum, _)| sum.round_ties_even() as u64)
        .find(|sum| *sum > 0)
}

fn contributions(line: &str) -> impl Iterator<Item = Contribution> + '_ {
    RATE.captures_iter(line).filter_map(|found| {
        let span = found.get(0)?;
        let before = CLAUSE_BREAK.split(&line[..span.start()]).last()?;
        let after = &line[span.end()..];
        if (EARNED.is_match(before) && !SAVING.is_match(before)) || OTHER_RATE.is_match(after) {
            return None;
        }
        let amount = named_amount(span.as_str())?;
        let period = match found.get(3)?.as_str() {
            "week" => "week",
            "fortnight" => "fortnight",
            "year" => "year",
            _ => "month",
        };
        (amount >= 10).then_some(Contribution { amount, period })
    })
}

fn held(line: &str) -> Option<u64> {
    savings::sums(line)
        .filter(|(_, span)| {
            let before = CLAUSE_BREAK
                .split(&line[..span.start()])
                .last()
                .unwrap_or("");
            let after = CLAUSE_BREAK.split(&line[span.end()..]).next().unwrap_or("");
            !RATE
                .find_at(line, span.start())
                .is_some_and(|rate| rate.start() < span.end())
                && !EARNED.is_match(before)
                && (HELD.is_match(before) || HELD.is_match(after))
        })
        .map(|(sum, _)| sum as u64)
        .find(|sum| *sum >= 20)
}

fn goal(line: &str) -> Option<u64> {
    if !SAVING.is_match(line) || (HELD.is_match(line) && !TARGET.is_match(line)) {
        return None;
    }
    let pot = held(line);
    savings::sums(line)
        .filter(|(_, span)| {
            !RATE
                .find_at(line, span.start())
                .is_some_and(|rate| rate.start() < span.end())
                && !DURATION
                    .find_at(line, span.start().saturating_sub(6))
                    .is_some_and(|duration| {
                        duration.start() <= span.start() && duration.end() >= span.end()
                    })
                && !ANOTHER_POT.is_match(&line[span.end()..])
                && !EARNED.is_match(
                    CLAUSE_BREAK
                        .split(&line[..span.start()])
                        .last()
                        .unwrap_or(""),
                )
        })
        .map(|(sum, _)| sum as u64)
        .filter(|sum| *sum >= 20 && Some(*sum) != pot)
        .max()
        .map(|amount| match pot.filter(|_| FURTHER.is_match(line)) {
            Some(held) => amount + held,
            None => amount,
        })
}

#[derive(Clone)]
struct Due {
    label: String,
    date: Date,
    months: u64,
}

fn due(line: &str, today: Date, bare: bool) -> Option<Due> {
    let line = CORRECTION.replace(line, "");
    let cleaned = line.trim().trim_end_matches(['.', '!', ',']).trim_end();
    let found = DEADLINE.captures_iter(cleaned).find(|found| {
        found.get(0).is_some_and(|span| {
            if bare {
                span.as_str() == cleaned
            } else {
                found.get(1).is_some() || found.get(2).is_some()
            }
        })
    })?;
    let name = found.name("label")?.as_str();
    let end = found.name("end").is_some() || name == "the year";
    let current = today.month();
    let (year, month, months) = match name {
        "next month" => {
            let month = current % 12 + 1;
            (today.year() + i16::from(month == 1), month, 1)
        }
        "this month" => (today.year(), current, 0),
        "next year" => (today.year() + 1, 12, (24 - current) as u64),
        "this year" | "the year" => (today.year(), 12, (12 - current) as u64),
        "this week" | "next week" => return None,
        _ => {
            let month = match (name, end) {
                ("spring", true) => 5,
                ("summer", true) => 8,
                ("autumn" | "fall", true) => 11,
                ("winter", true) => 2,
                _ => MONTHS
                    .iter()
                    .position(|month| *month == name)
                    .map(|index| index as i8 + 1)
                    .or_else(|| {
                        OCCASIONS
                            .iter()
                            .find_map(|(occasion, month)| (*occasion == name).then_some(*month))
                    })?,
            };
            let year = today.year() + i16::from(month < current || (month == current && !end));
            (
                year,
                month,
                ((year - today.year()) * 12 + i16::from(month - current)) as u64,
            )
        }
    };
    let first = Date::new(year, month, 1).ok()?;
    let date = if end { first.last_of_month() } else { first };
    let label = if end {
        format!("the end of {name}")
    } else {
        let mut letters = name.chars();
        format!("{}{}", letters.next()?.to_uppercase(), letters.as_str())
    };
    Some(Due {
        label,
        date,
        months,
    })
}

#[derive(Default)]
struct Plan {
    goal: u64,
    pot: u64,
    rate: Option<Contribution>,
    income: Option<u64>,
    has_saving_rate: bool,
    date: Option<Due>,
}

impl Plan {
    fn remaining(&self) -> u64 {
        self.goal.saturating_sub(self.pot)
    }
}

fn previous_plan(memory: &Memory, today: Date) -> Plan {
    let mut plan = Plan::default();
    let lines: Vec<String> = memory
        .turns
        .iter()
        .rev()
        .take(6)
        .rev()
        .map(|turn| turn.user.to_lowercase())
        .collect();
    let mentions_purpose = lines.iter().any(|line| SPLIT_PURPOSE.is_match(line));
    for line in &lines {
        if let Some(value) = goal(line) {
            plan.goal = value;
        } else if plan.goal > 0
            && CORRECTION.is_match(line)
            && PRICE_REPLY.is_match(CORRECTION.replace(line, "").trim())
            && let Some(value) = named_amount(line)
        {
            if plan.rate.is_some() && value < plan.goal && value < 1_000 {
                plan.rate = Some(Contribution {
                    amount: value,
                    period: "month",
                });
            } else {
                plan.goal = value;
            }
        } else if mentions_purpose
            && plan.goal == 0
            && SPLIT_PRICE.is_match(line)
            && !INCOME.is_match(line)
            && !POSITION.is_match(line)
            && !HELD.is_match(line)
            && !RATE.is_match(line)
            && let Some(value) = named_amount(line).filter(|value| *value >= 100)
        {
            plan.goal = value;
        }
        if let Some(earned) = INCOME.find(line) {
            plan.income = named_amount(&line[earned.end()..]).or(plan.income);
        }
        if let Some(value) = held(line).or_else(|| {
            POSITION
                .is_match(line)
                .then(|| named_amount(line))
                .flatten()
        }) {
            plan.pot = value;
        }
        if let Some(value) = contributions(line).last()
            && !line.contains('?')
            && !line
                .split_whitespace()
                .any(|word| matches!(word, "if" | "maybe"))
            && (SAVING.is_match(line) || plan.goal > 0 && RESTATED.is_match(line))
        {
            plan.rate = Some(value);
            plan.has_saving_rate |= SAVING.is_match(line);
        }
        if plan.goal > 0
            && let Some(date) = due(line, today, true).or_else(|| due(line, today, false))
        {
            plan.date = Some(date);
        }
    }
    plan
}

use savings::comma;

fn gap(goal: u64, pot: u64) -> String {
    let share = (100.0 * pot as f64 / goal as f64).round_ties_even() as u64;
    format!(
        "£{} to go of £{} - you're £{} in, {share}% of the way there.",
        comma(goal - pot),
        comma(goal),
        comma(pot)
    )
}

fn duration(count: u64, period: &str) -> String {
    let suffix = if count == 1 { "" } else { "s" };
    format!("{} {period}{suffix}", comma(count))
}

fn months(count: u64) -> String {
    let count_text = duration(count, "month");
    let years = count / 12;
    let extra = count % 12;
    if years == 0 {
        return count_text;
    }
    let years_text = duration(years, "year");
    if extra == 0 {
        return format!("{count_text}, about {years_text}");
    }
    format!(
        "{count_text}, about {years_text} and {}",
        duration(extra, "month")
    )
}

fn at_rate(goal: u64, rate: u64) -> String {
    format!(
        "£{} at £{} a month is {}.",
        comma(goal),
        comma(rate),
        months(goal.div_ceil(rate))
    )
}

fn rate_with_deadline(goal: u64, rate: u64, due: &Due) -> Option<String> {
    let left = due.months;
    if left == 0 || goal <= rate {
        return None;
    }
    let time = goal.div_ceil(rate);
    let lead = format!(
        "£{} at £{} a month takes {} and {} is {} away",
        comma(goal),
        comma(rate),
        months(time),
        due.label,
        months(left)
    );
    if time <= left {
        return Some(format!(
            "{lead}, so that rate gets you there. If the amount fits your budget, a standing order after payday can help."
        ));
    }
    let target = goal.div_ceil(left).div_ceil(10) * 10;
    Some(format!(
        "{lead}, so it does not get you there — that would take about £{} a month. At £{} a month you have £{} of it by then. A bonus, a bill that drops or a later date could close the gap.",
        comma(target),
        comma(rate),
        comma(rate * left)
    ))
}

fn by_date(goal: u64, due: &Due, today: Date) -> Option<String> {
    let days = u64::try_from(today.until(due.date).ok()?.get_days()).ok()?;
    if days <= 31 {
        return Some(format!(
            "£{} by {} is one pay packet at this point. Check what you can put by after your essential bills.",
            comma(goal),
            due.label
        ));
    }
    let weeks = days.div_ceil(7);
    Some(format!(
        "£{} by {} is about £{} a week over the {weeks} weeks left. Set a standing order after payday if that amount fits your budget.",
        comma(goal),
        due.label,
        comma(goal.div_ceil(weeks))
    ))
}

fn price_requested(memory: &Memory) -> bool {
    memory.turns.last().is_some_and(|turn| {
        turn.module == "saving_followup"
            && (turn.bot.contains(" going for?")
                || turn.bot.contains("the price on it?")
                || turn.bot.contains("Tell me the price"))
    })
}

fn preserve_fresh(reply: String, memory: &Memory) -> Option<String> {
    memory.turns.first_reply(&reply).is_none().then_some(reply)
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.raw.to_lowercase();
    let line = line.trim();
    let now = reading.now().date();
    let date = due(line, now, true);
    if date.is_none()
        && !line.bytes().any(|byte| byte.is_ascii_digit())
        && !LENGTH.is_match(line)
        && !GAP.is_match(line)
        && !RATE_ASK.is_match(line)
        && !ENOUGH.is_match(line)
        && !EXTRA_ASK.is_match(line)
    {
        return None;
    }
    let plan = previous_plan(memory, now);
    let remaining = plan.remaining();
    let local_goal = goal(line);
    let local_rate = contributions(line).last();
    let named_date = date.clone().or_else(|| due(line, now, false));
    let prior_rate = plan
        .rate
        .map(Contribution::monthly)
        .filter(|rate| *rate > 0);
    let prior_date = plan.date.as_ref();
    let mentioned = named_amount(line);

    if GAP.is_match(line) && mentioned.is_none() && plan.pot > 0 && remaining > 0 {
        let reply = gap(plan.goal, plan.pot);
        if memory.turns.last().is_some_and(|turn| turn.bot == reply) {
            return Some(if let Some(rate) = prior_rate {
                format!(
                    "At £{} a month, the £{} still to find takes {}.",
                    comma(rate),
                    comma(remaining),
                    duration(remaining.div_ceil(rate), "month")
                )
            } else {
                format!(
                    "Still £{} to go - you've £{} of the £{} in the pot.",
                    comma(remaining),
                    comma(plan.pot),
                    comma(plan.goal)
                )
            });
        }
        return preserve_fresh(reply, memory);
    }
    if POSITION.is_match(line)
        && held(line).is_none()
        && local_goal.is_none()
        && let Some(pot) = mentioned.filter(|pot| *pot > 0 && *pot < plan.goal)
    {
        return preserve_fresh(gap(plan.goal, pot), memory);
    }
    if EXTRA_ASK.is_match(line)
        && remaining > 0
        && let (Some(rate), Some(due)) = (prior_rate, prior_date.filter(|due| due.months > 0))
    {
        let left = due.months;
        let time = remaining.div_ceil(rate);
        let reply = if time <= left {
            format!(
                "Nothing more — £{} a month already gets you to £{} by {} with {} to spare.",
                comma(rate),
                comma(remaining),
                due.label,
                duration(left - time, "month")
            )
        } else {
            let need = remaining.div_ceil(left).div_ceil(10) * 10;
            format!(
                "About £{} a month more: £{} a month for the {} left, rather than the £{} you have. Or give it longer — at £{} a month the whole £{} takes {}.",
                comma(need - rate),
                comma(need),
                months(left),
                comma(rate),
                comma(rate),
                comma(remaining),
                months(time)
            )
        };
        return preserve_fresh(reply, memory);
    }
    let unit_asked = RATE_ASK.captures(line);
    if unit_asked
        .as_ref()
        .and_then(|captures| captures.get(1))
        .is_some_and(|unit| unit.as_str() == "month")
        && remaining > 0
        && mentioned.is_none()
        && let Some(weekly) = memory
            .turns
            .iter()
            .rev()
            .take(4)
            .find_map(|turn| {
                WEEKLY_ANSWER
                    .captures(&turn.bot)
                    .and_then(|found| number(&found[1]))
            })
            .filter(|rate| *rate >= 5)
    {
        let monthly = ((weekly * 52) as f64 / 120.0).round_ties_even() as u64 * 10;
        return preserve_fresh(
            format!(
                "£{} a week is about £{} a month. A standing order after payday can help if that amount fits your budget.",
                comma(weekly),
                comma(monthly)
            ),
            memory,
        );
    }
    if ENOUGH.is_match(line)
        && remaining > 0
        && let Some(rate) = prior_rate
    {
        let time = remaining.div_ceil(rate);
        let reply = match prior_date.filter(|date| date.months > 0) {
            Some(due) if time <= due.months => format!(
                "Yes — £{} a month gets you to £{} by {}, with {} to spare.",
                comma(rate),
                comma(remaining),
                due.label,
                duration(due.months - time, "month")
            ),
            Some(due) => format!(
                "No — at £{} a month you have £{} of £{} by {}. The whole £{} takes {} at that rate, so the rest wants more a month or a later date.",
                comma(rate),
                comma(rate * due.months),
                comma(remaining),
                due.label,
                comma(remaining),
                months(time)
            ),
            None => format!(
                "£{} at £{} a month takes {}. Name the month you want it by and I can work out whether that is soon enough.",
                comma(remaining),
                comma(rate),
                months(time)
            ),
        };
        return preserve_fresh(reply, memory);
    }
    if unit_asked.is_some()
        && mentioned.is_none()
        && named_date.is_none()
        && prior_date.is_none()
        && remaining > 0
    {
        return preserve_fresh(
            format!(
                "£{} is a different rate over six months than over a year, so name the month you want it by and I can work out what to put by each month.",
                comma(remaining)
            ),
            memory,
        );
    }
    if let Some(rate) = prior_rate.filter(|rate| *rate < remaining)
        && ADD.is_match(line)
        && local_goal.is_none()
        && !EARNED.is_match(line)
        && let Some(extra) =
            mentioned.filter(|extra| (5..=5_000).contains(extra) && rate + extra < remaining)
    {
        return preserve_fresh(
            prior_date
                .and_then(|due| rate_with_deadline(remaining, rate + extra, due))
                .unwrap_or_else(|| at_rate(remaining, rate + extra)),
            memory,
        );
    }
    if named_date.is_none()
        && (RESTATED.is_match(line) || BARE_RATE.is_match(line) && prior_rate.is_some())
        && let Some(rate) = local_rate
            .map(Contribution::monthly)
            .or(mentioned)
            .filter(|rate| *rate >= 20 && *rate < remaining && Some(*rate) != prior_rate)
    {
        let reply = prior_date
            .and_then(|due| rate_with_deadline(remaining, rate, due))
            .unwrap_or_else(|| at_rate(remaining, rate));
        if memory
            .turns
            .last()
            .is_some_and(|turn| turn.bot.starts_with(&reply))
        {
            return Some(format!(
                "That's £{} a month. If it fits your budget, you could set that amount aside after payday.",
                comma(rate)
            ));
        }
        return preserve_fresh(reply, memory);
    }
    if LENGTH.is_match(line)
        && local_rate.is_none()
        && prior_date.is_none()
        && let Some(rate) = prior_rate
    {
        if let Some(value) = mentioned.filter(|amount| *amount > rate)
            && (SAVING.is_match(line) || line.contains("afford") || BUY.is_match(line))
        {
            return preserve_fresh(plan.rate?.reply(value), memory);
        }
        if plan.has_saving_rate
            && plan.goal == 0
            && let Some(buy) = BUY.captures(line)
        {
            let lead = format!(
                "£{} a month is £{} every six months, £{} in a year.",
                comma(rate),
                comma(rate * 6),
                comma(rate * 12)
            );
            let ask = if price_requested(memory) {
                "Tell me the price and I'll do the weeks.".into()
            } else if let Some(thing) = buy
                .get(1)
                .filter(|word| !matches!(word.as_str(), "one" | "new" | "second" | "cheap"))
            {
                format!("What's the {} going for?", thing.as_str())
            } else {
                "What's the price on it?".into()
            };
            return preserve_fresh(format!("{lead} {ask}"), memory);
        }
    }
    if price_requested(memory)
        && PRICE_REPLY.is_match(line)
        && let Some(rate) = prior_rate
        && let Some(value) = mentioned.filter(|amount| *amount > rate)
    {
        return preserve_fresh(at_rate(value, rate), memory);
    }
    if SAVING.is_match(line)
        && !DURATION.is_match(line)
        && local_goal.is_none()
        && named_date.is_none()
        && let Some(rate) = local_rate.filter(|rate| rate.amount > 0 && rate.amount < remaining)
    {
        return preserve_fresh(
            prior_date
                .and_then(|due| rate_with_deadline(remaining, rate.monthly(), due))
                .unwrap_or_else(|| rate.reply(remaining)),
            memory,
        );
    }
    if LENGTH_FOLLOWUP.is_match(line)
        && mentioned.is_none()
        && remaining >= 100
        && !memory
            .turns
            .last()
            .is_some_and(|turn| LENGTH.is_match(&turn.user.to_lowercase()))
    {
        if let Some(rate) = plan.rate {
            return preserve_fresh(rate.reply(remaining), memory);
        }
        if prior_date.is_none() {
            let left = if plan.pot > 0 {
                format!("the £{} still to find", comma(remaining))
            } else {
                format!("£{}", comma(remaining))
            };
            let slow = months(remaining.div_ceil(100));
            let fast = months(remaining.div_ceil(250));
            let reply = if let Some(income) = plan.income {
                format!(
                    "How much of the £{} can you put by each month? At £100 a month, {left} takes {slow}.",
                    comma(income)
                )
            } else if slow == fast {
                format!("At £100 a month, {left} takes {slow}. Pick a rate your budget can carry.")
            } else {
                format!(
                    "At £100 a month, {left} takes {slow}, or {fast} at £250 a month. Pick a rate your budget can carry."
                )
            };
            return preserve_fresh(reply, memory);
        }
    }
    let local_pot = held(line);
    if local_goal.is_none()
        && let Some(pot) = local_pot.filter(|pot| *pot < plan.goal)
        && let Some(reply) = savings::sample_rates(plan.goal - pot, &[])
    {
        return preserve_fresh(
            format!(
                "£{} still to find: {}",
                comma(plan.goal - pot),
                reply.replacen("For ", "for ", 1)
            ),
            memory,
        );
    }
    if SAVING.is_match(line)
        && let Some(total) = local_goal
    {
        let goal = total.saturating_sub(local_pot.unwrap_or(0));
        let reply = if DURATION.is_match(line) {
            savings::time_to_save(line, &memory.turns)
        } else if let Some(rate) = local_rate {
            Some(
                named_date
                    .as_ref()
                    .and_then(|due| rate_with_deadline(goal, rate.monthly(), due))
                    .unwrap_or_else(|| rate.reply(goal)),
            )
        } else {
            named_date
                .as_ref()
                .and_then(|due| by_date(goal, due, now))
                .or_else(|| savings::sample_rates(goal, &[]))
        }?;
        return preserve_fresh(
            if let Some(pot) = local_pot {
                format!(
                    "£{} to find on top of the £{} you already have: {}",
                    comma(goal),
                    comma(pot),
                    reply.replacen("For ", "for ", 1)
                )
            } else {
                reply
            },
            memory,
        );
    }
    if unit_asked.is_some() && mentioned.is_none() && named_date.is_none() && remaining > 0 {
        return preserve_fresh(by_date(remaining, prior_date?, now)?, memory);
    }
    if let Some(due) = date.filter(|_| remaining > 0) {
        let reply = prior_rate
            .and_then(|rate| rate_with_deadline(remaining, rate, &due))
            .or_else(|| by_date(remaining, &due, now))?;
        return preserve_fresh(reply, memory);
    }
    if line.starts_with("what about ")
        || line.starts_with("how about ")
        || CORRECTION.is_match(line)
    {
        let later = line
            .strip_prefix("what about ")
            .or_else(|| line.strip_prefix("how about "));
        let corrected = CORRECTION.replace(line, "");
        if (plan.goal > 0
            || memory
                .turns
                .last()
                .is_some_and(|turn| SUGGESTED_GOAL.is_match(&turn.bot)))
            && PRICE_REPLY.is_match(later.unwrap_or(&corrected).trim())
            && let Some(value) = mentioned.filter(|amount| *amount >= 20)
        {
            return preserve_fresh(
                prior_date
                    .and_then(|due| by_date(value, due, now))
                    .or_else(|| savings::sample_rates(value, &[]))?,
                memory,
            );
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::bot::Bot;

    #[test]
    fn saving_followup_uses_the_goal_pot_and_committed_rate() {
        let mut bot = Bot::new(Some("0")).unwrap();
        assert_eq!(
            bot.turn("I want to save 3000 pounds").module,
            Some("saving_followup")
        );
        assert!(
            bot.turn("I have 1000 saved already")
                .reply
                .contains("£2,000 still to find")
        );
        let rate = bot.turn("I can put by 200 a month");
        assert_eq!(rate.module, Some("saving_followup"));
        assert!(
            rate.reply.contains("£2,000 at £200 a month is 10 months"),
            "{rate:?}"
        );
        let proposed = bot.turn("and if I cut the gym thats another 40");
        assert_eq!(proposed.module, Some("saving_followup"));
        assert!(proposed.reply.contains("£240 a month is 9 months"));
        let committed = bot.turn("is that enough");
        assert!(committed.reply.contains("£200 a month takes 10 months"));
    }

    #[test]
    fn saving_followup_does_not_treat_income_as_a_savings_rate() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("I want to save 2000 pounds");
        bot.turn("I earn 1500 a month");
        assert_ne!(bot.turn("is that enough").module, Some("saving_followup"));
        assert_ne!(
            bot.turn("is that enough salt").module,
            Some("saving_followup")
        );
    }

    #[test]
    fn saving_followup_uses_the_original_period_and_a_price_stated_separately() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("I'm saving up for a bike");
        bot.turn("The one I want costs £1,200");
        let turn = bot.turn("I put by £100 a week");
        assert_eq!(turn.module, Some("saving_followup"), "{turn:?}");
        assert!(turn.reply.contains("takes 12 weeks"), "{turn:?}");
        assert_ne!(
            bot.turn("How long to cook chicken").module,
            Some("saving_followup")
        );
    }

    #[test]
    fn saving_followup_prices_examples_for_a_goal_without_a_rate() {
        let mut bot = Bot::new(Some("0")).unwrap();
        bot.turn("I want to save £3,000 for a holiday");
        let turn = bot.turn("How long will that take");
        assert_eq!(turn.module, Some("saving_followup"), "{turn:?}");
        assert!(
            turn.reply.contains("30 months") && turn.reply.contains("12 months"),
            "{turn:?}"
        );
    }
}
