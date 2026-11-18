//! Return-to-work, working time and statutory holiday calculations.

use std::{borrow::Cow, sync::LazyLock};

use fancy_regex::Regex as Lookaround;
use regex::Regex;

use crate::bot::{Exchange, Reading};

use super::{cleaned, said_text, uses};

#[path = "work_hours_data.rs"]
mod data;

macro_rules! pattern {
    ($name:ident, $source:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}
pattern!(IN_LAW, r"\bin\s*-?\s*laws?\b");
pattern!(
    BREAK_CONDITION,
    r"\b(?:when|if|every time)\b[^.?!]*\bbreak(?:s|ing)?\b"
);
pattern!(BREAK_DOWN, r"\bbreak(?:s|ing)?\s+down\b");
pattern!(
    DAYS_WORKED,
    r"\b(\d(?:\.5)?|one|two|three|four|five|six)\s*days?\s*(?:a|per|each)\s*week\b"
);
pattern!(QUOTED_DAYS, r"\b(\d{1,2})\s*days?\b");
pattern!(BALANCE, r"\b(?:left|remaining|balance)\b|\bstill\s+have\b");
pattern!(
    ELSES_WEEK,
    r"\b(?:my|his|her|their)\s+(?:wife|husband|partner|son|daughter|mum|dad|friend|colleague|sister|brother)\b"
);
pattern!(OWN_WEEK, r"\bi\b|\bim\b|\bi'm\b");

static CARRY_OVER: LazyLock<Lookaround> = LazyLock::new(|| {
    let weight = r"(?!\s+(?:weight|fat|kilos?|kilograms?|pounds?|lbs|stones?|inches|my weight|some weight|a stone|a few pounds))";
    Lookaround::new(&format!(
        r"\bcarry|carried|carrying|roll(?:ed)? over|use it or lose|\bsaved\b|\bsaved up\b|left over|leftover|from last year|from the year before|\blose\b{weight}|\blost\b{weight}|forfeit|unused|\binto (?:january|february|march|april|may|june|july|august|september|october|november|december|next year|the next year|next month)\b|\btake it into the next\b"
    )).expect("holiday carryover")
});

pub(super) fn going_back(text: &str) -> &'static str {
    let said = cleaned(text);
    if uses(
        &said,
        &[
            "hate",
            "hate it",
            "regret",
            "miserable",
            "cant face",
            "can't face",
        ],
    ) {
        return data::GOING_BACK_WRONG;
    }
    if uses(&said, &["my wife", "my husband", "my partner"]) {
        return data::GOING_BACK_THEIRS;
    }
    // Return rights depend on the type and duration of statutory leave.
    // https://www.legislation.gov.uk/uksi/1999/3312/body/2025-04-06
    if uses(
        &said,
        &[
            "maternity",
            "maternity leave",
            "paternity",
            "adoption leave",
            "parental leave",
            "shared parental",
            "my baby",
            "the baby",
            "got a baby",
            "new baby",
        ],
    ) {
        return "Ask your employer to confirm your role, hours and pay before you return. After up to 26 weeks of maternity leave in Great Britain, you usually have the right to return to the same job. After longer maternity leave, the same job remains the starting point. If that is not reasonably practicable, your employer must offer a suitable role on terms no less favourable. If you took another type of family leave or combined leaves, check the rules for that leave. You can ask about flexible hours or agree a keeping-in-touch day where your leave allows it.";
    }
    if uses(
        &said,
        &[
            "off sick",
            "sick leave",
            "sick note",
            "signed off",
            "fit note",
            "phased return",
            "stress",
            "burnout",
            "anxiety",
            "depression",
            "depressed",
            "hospital",
            "operation",
            "surgery",
            "miscarriage",
            "bereavement",
            "grief",
            "sickness",
            "illness",
            "unwell",
            "after being ill",
            "flu",
            "covid",
            "time off",
        ],
    ) {
        return "Ask your manager about a phased return, such as shorter days, fewer days or different duties, based on what you can manage now. A fit note can suggest changes. If your employer has occupational health, ask for its help in agreeing a plan and a review date.";
    }
    if uses(
        &said,
        &[
            "dont want to go back",
            "don't want to go back",
            "do not want to go back",
            "dont want to return",
            "don't want to return",
            "not looking forward",
            "dreading going back",
            "dreading it",
            "cant face going back",
            "can't face going back",
        ],
    ) {
        return data::GOING_BACK_DONT_WANT;
    }
    if uses(
        &said,
        &[
            "exhausted",
            "knackered",
            "shattered",
            "tired",
            "drained",
            "wrecked",
            "done in",
            "knackering",
        ],
    ) {
        return "Returning to a work routine can be tiring. Keep a few evenings clear, aim for regular sleep and take your lunch break away from work. If the tiredness is affecting daily life, or it lasts a few weeks without a clear cause, talk to your GP. They can decide whether you need tests.";
    }
    if uses(
        &said,
        &[
            "nervous",
            "anxious",
            "anxiety",
            "worried",
            "worry",
            "dread",
            "dreading",
            "scared",
            "guilty",
            "frightened",
            "terrified",
        ],
    ) {
        return data::GOING_BACK_NERVES;
    }
    data::GOING_BACK_PREP
}

// https://www.gov.uk/rest-breaks-work
// https://www.gov.uk/maximum-weekly-working-hours
// https://www.gov.uk/overtime-your-rights
pub(super) fn pay_and_hours(text: &str) -> Option<&'static str> {
    let said = cleaned(text);
    let names = |phrases: &[&str]| uses(&said, phrases);
    if names(&["an hour", "half an hour", "an hour or two", "to kill"])
        && names(&[
            "what can i do",
            "what should i do",
            "what to do",
            "shall i do",
            "anything to do",
        ])
    {
        return None;
    }
    if names(&[
        "zero hours",
        "zero hour contract",
        "casual contract",
        "no guaranteed",
    ]) {
        return Some(
            "A zero-hours contract gives you no guaranteed hours. Check how work is offered and what the contract says about declining it. In Great Britain, a zero-hours employer cannot stop you taking another job. Workers are entitled to the applicable minimum wage and paid holiday. If you work regular hours, you can ask for a contract that reflects them.",
        );
    }
    if names(&[
        "unpaid leave",
        "unpaid time off",
        "leave without pay",
        "time off without pay",
    ]) {
        return Some(
            "Ordinary unpaid leave depends on your employer's agreement. There are separate statutory rights to some types of leave, including emergency time off for dependants and unpaid parental leave for eligible employees. Check which applies. Ask in writing about dates and how the work will be covered.",
        );
    }
    let has_break_cue = names(data::BREAK_WORK_CUES);
    if has_break_cue
        && names(&[
            "lunch",
            "break",
            "breaks",
            "rest break",
            "rest breaks",
            "breather",
        ])
    {
        return Some(
            "In Great Britain most adult workers get an uninterrupted 20-minute break if they work more than six hours, and 11 hours' daily rest. The break can be unpaid, and there are different rules for some kinds of work and younger workers. If your lunch is being cut short, keep a record and raise the actual times in writing.",
        );
    }
    if names(&[
        "48",
        "forty eight",
        "long hours",
        "too many hours",
        "maximum hours",
        "how many hours",
    ]) {
        return Some(
            "For most adult workers in Great Britain the limit is an average of 48 hours a week, usually over 17 weeks. Some jobs have different rules. If your job allows an opt-out, it is voluntary and must be in writing. Check how long you have to give to end it and ask for your hours calculation in writing if it looks wrong.",
        );
    }
    if names(&[
        "change my hours",
        "changed my hours",
        "change my shift",
        "rota",
        "different hours",
        "new hours",
        "change my contract",
    ]) {
        return Some(
            "A change to your contracted hours usually needs agreement unless a relevant term allows it. An employer should consult you and use a flexibility clause reasonably, with reasonable notice. Ask for the change and the reason in writing and speak to your union or Acas if you cannot agree.",
        );
    }
    if names(&[
        "overtime",
        "extra hours",
        "work late",
        "stay late",
        "time and a half",
    ]) {
        if names(&[
            "make me",
            "make us",
            "can my boss",
            "can my employer",
            "can my work",
            "have to",
            "refuse",
            "stay late",
            "staying late",
            "work late",
            "working late",
            "without pay",
            "unpaid overtime",
        ]) {
            return Some(
                "Check whether your contract requires overtime. An employer can usually require it only where the contract says so, and the working-time rules still apply. Extra pay depends on the contract, but your average pay over the hours worked must still meet the applicable minimum wage. Ask for the expected hours and rate in writing.",
            );
        }
        return Some(
            "Overtime pay depends on your contract or the arrangement you agreed. Your average pay over all hours worked must still meet the applicable minimum wage. Check when overtime begins, what rate applies and whether you get time off instead.",
        );
    }
    if !names(data::WORK_SUBJECT)
        || BREAK_CONDITION.is_match(&said)
        || (BREAK_DOWN.is_match(&said)
            && !names(&["shift", "shifts", "rota", "work", "working", "job", "boss"]))
        || (names(data::INJURY_WORDS) && !has_break_cue)
        || (IN_LAW.is_match(&said) && !names(data::IN_LAW_JOB))
    {
        return None;
    }
    Some(
        "Check your contract for your hours, breaks and whether overtime is paid. For most adult workers in Great Britain, the usual statutory rules include a rest break after more than six hours, 11 hours' daily rest and a 48-hour average week. There are exceptions and some workers may opt out of the weekly limit. If your employer's records do not match the hours you work, put the details in writing and ask for a correction.",
    )
}

fn current_week(said: &str) -> Option<&str> {
    let found = DAYS_WORKED.captures(said)?.get(1)?;
    (!ELSES_WEEK.is_match(&said[..found.end()])).then_some(found.as_str())
}

fn earlier_week(said: &str) -> Option<&str> {
    let found = DAYS_WORKED.captures(said)?.get(1)?;
    let start = said[..found.start()]
        .char_indices()
        .rev()
        .nth(39)
        .map_or(0, |(at, _)| at);
    let nearby = &said[start..found.end()];
    (!ELSES_WEEK.is_match(nearby) && OWN_WEEK.is_match(nearby)).then_some(found.as_str())
}

fn part_time_days(text: &str) -> Option<f64> {
    let days = match text {
        "one" => 1.0,
        "two" => 2.0,
        "three" => 3.0,
        "four" => 4.0,
        "five" => 5.0,
        "six" => 6.0,
        _ => text.parse::<f64>().ok()?,
    };
    (days > 0.0).then_some(days)
}

fn yearly_days(days: f64) -> String {
    let shown = format!("{:.1}", (5.6 * days).min(28.0));
    shown.strip_suffix(".0").unwrap_or(&shown).to_owned()
}

// https://www.gov.uk/holiday-entitlement-rights/calculate-leave-entitlement
// https://www.gov.uk/government/publications/simplifying-holiday-entitlement-and-holiday-pay-calculations/holiday-pay-and-entitlement-reforms-from-1-january-2024
pub(super) fn holiday(reading: &Reading<'_>, turns: &[Exchange]) -> Cow<'static, str> {
    const ENTITLEMENT: &str = "Most workers in Great Britain get at least 5.6 weeks of paid holiday per leave year, capped at 28 days for a regular working week. Bank holidays can count towards it.";
    const IRREGULAR: &str = "Irregular-hours and part-year workers generally accrue statutory holiday based on hours worked. Check your contract for any extra leave.";
    const REMAINING: &str = "Your remaining balance depends on your start date, hours and leave used or booked. Check your employer's holiday system or ask payroll for the calculation.";
    let line = said_text(reading).to_lowercase();
    if CARRY_OVER.is_match(&line).unwrap_or(false) {
        return Cow::Borrowed(
            "Check your contract for any holiday you can carry into the next leave year. Statutory carry-over can also apply if you were unable to take leave because of sickness or statutory family leave, or if the employer did not give you a reasonable opportunity to take it. The amount and deadline depend on why the leave was untaken and your working pattern.",
        );
    }
    let typed = reading.raw.to_lowercase();
    let days = current_week(&typed).and_then(part_time_days).or_else(|| {
        turns
            .iter()
            .rev()
            .take(3)
            .find_map(|turn| earlier_week(&turn.user.to_lowercase()).and_then(part_time_days))
    });
    let balance = BALANCE.is_match(&line);
    if let Some(days) = days {
        let year = yearly_days(days);
        let individual = format!(
            "For a regular {days}-day week that is {year} paid days for a full leave year."
        );
        if balance {
            return Cow::Owned(format!("{ENTITLEMENT} {individual} {REMAINING}"));
        }
        return Cow::Owned(format!("{ENTITLEMENT} {individual} {IRREGULAR}"));
    }
    if balance {
        return Cow::Owned(format!("{ENTITLEMENT} {REMAINING}"));
    }
    if let Some(quoted) = QUOTED_DAYS
        .captures(&line)
        .and_then(|found| found[1].parse::<u8>().ok())
    {
        let comparison = if quoted < 28 { "below" } else { "at or above" };
        return Cow::Owned(format!(
            "{quoted} days is {comparison} the 28-day statutory minimum for a regular five-day week worked for the full leave year. Bank holidays can count, so check whether your figure includes them and whether you work a full leave year. {IRREGULAR}"
        ));
    }
    Cow::Owned(format!(
        "{ENTITLEMENT} For a regular four-day week that is 22.4 paid days. {IRREGULAR}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leave_counts_the_owners_regular_week_and_caps_statutory_days() {
        let reading = Reading::new("How much holiday do I get?");
        let turn = |user: &str| Exchange {
            user: user.into(),
            bot: String::new(),
            module: "",
        };
        assert!(holiday(&reading, &[turn("I work three days a week.")]).contains("16.8 paid days"));
        assert!(
            !holiday(&reading, &[turn("My partner works three days a week.")]).contains("16.8")
        );
        assert!(
            holiday(
                &Reading::new("I work six days a week. How much holiday do I get?"),
                &[]
            )
            .contains("28 paid days")
        );
        assert!(pay_and_hours("I have an hour before work what can I do?").is_none());
        assert!(
            going_back("I am going back to work after being signed off sick")
                .contains("phased return")
        );
    }
}
