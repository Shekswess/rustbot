//! Add up a household's stated income and regular costs.

use crate::bot::{Exchange, Memory, Reading};
use regex::{Match, Regex};
use std::collections::HashSet;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $value:literal) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($value).expect(stringify!($name)));
    };
}

pattern!(
    SUM,
    r"£\s*(?P<marked>\d[\d,]*(?:\.\d+)?)\s*(?P<marked_unit>k|grand|thousand)?|(?P<bare>\d[\d,]*(?:\.\d+)?)\s*(?P<bare_unit>k|grand|thousand|pounds?|quid)\b"
);
pattern!(BARE_NUMBER, r"\d[\d,]*(?:\.\d+)?");
pattern!(
    INCOME,
    r"\b(?:i|we)(?:'m|m|'re| are)?\s+(?:on|earn|earns|make|makes|bring\s+in|brings\s+in|get|gets)\b|\btake[\s-]?home\b|\b(?:my|our)\s+(?:pay|wage|wages|salary|pension|income|earnings)\b|\bafter\s+tax\b|\b(?:partner|husband|wife)\s+(?:earns?|makes?|brings?\s+in|gets?|takes?\s+home)\b"
);
pattern!(
    FIXED_COST,
    r"\b(?:rents?|mortgage|childcare|child\s*care|childminder|nursery|nursery\s+fees|school\s+(?:fees|dinners|clubs?)|breakfast\s+club|after[\s-]school\s+club|council\s+tax|bills?|utility|utilities|energy|electric(?:ity)?|water|gas|heating|broadband|internet|phone|mobile|tv\s+licen[cs]e|car\s+finance|car\s+payment|car\s+insurance|loan|loans|repayments?|insurance|commut\w*|travel|fares?|transport|petrol|diesel|fuel|parking|tolls?|train\s+fare|season\s+ticket|bus\s+pass|subscriptions?|gym|clothes|uniforms?|nappies|food|groceries|shopping|car|van|pets?|vets?|vet\s+bills?|medicines?|prescriptions?|dentist|child\s+maintenance|maintenance)\b"
);
pattern!(
    AFTER_FIGURE,
    r"^\s*(?:a\s+(?:month|week|year|fortnight)\s*)?(?:goes?|went|is\s+spent|are\s+spent)?\s*(?:on|for|towards|into)\s+(?P<noun>[a-z][a-z'\-]*)"
);
pattern!(
    NO_COST_NAME,
    r"^(?:a|an|the|it|them|that|this|me|us|him|her|his|my|our|their|its|and|or|so|but|if|when|top|all|everything|nothing|stuff|things?)$"
);
pattern!(
    MONTH,
    r"\b(?:a|per|each|every)\s+month\b|\bmonthly\b|\bpcm\b|\b(?:a|per)\s+4\s+weeks\b"
);
pattern!(WEEK, r"\b(?:a|per|each|every)\s+week\b|\bweekly\b");
pattern!(
    YEAR,
    r"\b(?:a|per|each|every)\s+year\b|\byearly\b|\bannually\b|\ba\s+year\b|\bp\.?a\.?\b"
);
pattern!(
    NOT_BUDGET,
    r"\b(?:arrears?|behind|overdue|evict\w*|notice|can'?t\s+pay|cant\s+pay|couldn'?t\s+pay|not\s+paid|unpaid|short\s+this\s+month|owing|owe|owed|debt|debts|default\w*|bailiff\w*|due|going\s+up|went\s+up|go\s+up|gone\s+up|goes\s+up|rising|increas\w*|raise[sd]?|cheaper|reduce|reducing|lower|negotiate|landlord|housing\s+benefit|universal\s+credit)\b"
);
pattern!(
    ASK,
    r"^\s*(?:should|could|can|do|does|is|are|was|were|what|how|when|where|which|why|would|will)\b|[,;]\s*(?:should|could|can|do|does|is|are|was|were|what|how|when|where|which|why|would|will)\s+\w|\?\s*$"
);
pattern!(
    MEASURE,
    r"^\s*(?:mins?|minutes?|hours?|hrs?|days?|weeks?|months?|years?|miles?|km|kg|bed(?:room)?s?|kids?|people|guests?|percent|%)"
);
pattern!(TAX_CODE, r"\b\d{3,}[a-z]{1,2}\b");
pattern!(
    SAVING_POT,
    r"\b(?:put|set|stick)\s+(?:£?\d[\d,]*(?:\.\d+)?\s+)?(?:by|aside|away)\b|\b(?:save[sd]?|saving|savings|pot|putting\s+by)\b"
);
pattern!(
    START,
    r"\b(?:what|where)\s+(?:should|do|shall)\s+i\s+(?:do|start|begin)\b[^.?!]*\b(?:first|next|now|start)\b|\bhow\s+do\s+i\s+(?:start|begin)\b|\bwhere\s+(?:do|should)\s+i\s+(?:even\s+)?(?:start|begin)\b|\bwhat'?s\s+(?:the\s+)?first\s+(?:step|thing)\b"
);
pattern!(
    LEFT_ASK,
    r"\bwhat'?s\s+left\b|\bwhat\s+is\s+left\b|\bwhat'?s\s+left\s+over\b|\bhow\s+much\s+(?:is\s+left|left|do\s+i\s+have\s+left|have\s+i\s+(?:got\s+)?left)\b"
);
pattern!(
    POINTS_BACK,
    r"\b(?:that|this|it|them|those|these|now|next|first|then|month|months|money|save|saves|saving|savings|budget|budgeting|pay|paid|payday|salary|wage|income|rent|bills?|pot|aside|afford)\b"
);
pattern!(
    SAVINGS,
    r"\b(?:save|saves|saving|savings|put\s+(?:by|aside|away)|budget|deposit|putting\s+by)\b"
);
pattern!(OTHER_CURRENCY, r"[$€]|\b(?:dollars?|euros?|bucks)\b");

const LEFT: &[&str] = &[
    "That leaves {left} after the {named}. Food, travel and the rest come out of that, and whatever is over at the end is what you can put by.",
    "{left} left after the {named}, then. A standing order for the amount you want to keep, on the day after payday, takes it out before you can spend it.",
    "So {in_amount} in and {out_amount} out on the {named} leaves {left}. Pick the figure you want to keep and move it the day you are paid.",
];
const YEAR_AND_MONTH: &[&str] = &[
    "{in_amount} a year is about {a_month} a month, so the {named} leaves {left}.",
    "{in_amount} a year works out at {a_month} a month, and after the {named} there is {left}.",
];
const FIRST_MOVE: &[&str] = &[
    "Set the standing order first: choose the figure you want to keep and move it the day after you are paid, so the month runs on what is left rather than the other way round.",
    "Start with the standing order on payday for the figure you want to keep. Everything else in the month can move around it.",
];
const LEFT_AGAIN: &[&str] = &[
    "{left} {unit}, on the figures you gave.",
    "Same sum, on the figures you gave: {left} {unit}.",
];
const A_SHARE: &[&str] = &[
    "A tenth to a fifth of {in_amount} is {low} to {high} {period}. Pick an amount that fits after paying essentials.",
    "A tenth to a fifth is {low} to {high} {period}, out of {in_amount}. If that would leave you short for essentials, start lower.",
];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Period {
    #[default]
    Unstated,
    Month,
    Week,
    Year,
}

impl Period {
    fn wording(self) -> &'static str {
        match self {
            Self::Unstated => "",
            Self::Month => "a month",
            Self::Week => "a week",
            Self::Year => "a year",
        }
    }
}

fn periods() -> [(&'static Regex, Period); 3] {
    [
        (&WEEK, Period::Week),
        (&YEAR, Period::Year),
        (&MONTH, Period::Month),
    ]
}

fn period(text: &str) -> Period {
    periods()
        .into_iter()
        .find(|(pattern, _)| pattern.is_match(text))
        .map_or(Period::Unstated, |(_, unit)| unit)
}

fn next_chars(text: &str, count: usize) -> &str {
    &text[..text
        .char_indices()
        .nth(count)
        .map_or(text.len(), |(i, _)| i)]
}

fn last_chars(text: &str, count: usize) -> &str {
    &text[text
        .char_indices()
        .rev()
        .nth(count - 1)
        .map_or(0, |(i, _)| i)..]
}

#[derive(Clone, Copy)]
struct Figure {
    value: f64,
    start: usize,
    end: usize,
}

fn sums(text: &str) -> Vec<Figure> {
    let mut result = Vec::new();
    let add = |matched: Match<'_>, number: &str, suffix: Option<&str>| {
        let rest = &text[matched.end()..];
        if rest.starts_with(char::is_alphabetic) || MEASURE.is_match(rest) {
            return None;
        }
        let mut value: f64 = number.replace(',', "").parse().ok()?;
        if matches!(suffix, Some("k" | "grand" | "thousand")) {
            value *= 1_000.0;
        }
        (20.0..1e12).contains(&value).then_some(Figure {
            value,
            start: matched.start(),
            end: matched.end(),
        })
    };
    result.extend(SUM.captures_iter(text).filter_map(|capture| {
        let amount = capture.name("marked").or_else(|| capture.name("bare"))?;
        let suffix = capture
            .name("marked_unit")
            .or_else(|| capture.name("bare_unit"));
        add(
            capture.get(0)?,
            amount.as_str(),
            suffix.as_ref().map(Match::as_str),
        )
    }));
    let marked = result.clone();
    result.extend(BARE_NUMBER.find_iter(text).filter_map(|matched| {
        if marked
            .iter()
            .any(|figure| (figure.start..figure.end).contains(&matched.start()))
        {
            return None;
        }
        add(matched, matched.as_str(), None)
    }));
    result.sort_by_key(|figure| figure.start);
    result
}

fn incomes(line: &str, figures: &[Figure]) -> Vec<Figure> {
    if TAX_CODE.is_match(line) {
        return Vec::new();
    }
    let mut used = HashSet::new();
    INCOME
        .find_iter(line)
        .filter_map(|cue| {
            figures
                .iter()
                .filter_map(|figure| {
                    let gap = figure
                        .start
                        .checked_sub(cue.end())
                        .filter(|gap| *gap <= 20)
                        .or_else(|| cue.start().checked_sub(figure.end).filter(|gap| *gap <= 8))?;
                    Some((gap, *figure))
                })
                .min_by_key(|(distance, _)| *distance)
                .map(|(_, figure)| figure)
        })
        .filter(|figure| used.insert(figure.start))
        .collect()
}

fn saving_pot(line: &str, figure: &Figure) -> bool {
    let before = last_chars(&line[..figure.start], 18).len();
    let after = next_chars(&line[figure.end..], 18).len();
    SAVING_POT.is_match(&line[(figure.start - before)..(figure.end + after)])
}

#[derive(Clone)]
struct Cost<'a> {
    name: &'a str,
    value: f64,
    unit: Period,
}

fn cost_name<'a>(line: &'a str, figure: &Figure) -> Option<&'a str> {
    let before = last_chars(&line[..figure.start], 26);
    let after = next_chars(&line[figure.end..], 26);
    FIXED_COST
        .find_iter(before)
        .map(|hit| (before.len() - hit.end(), hit.as_str()))
        .chain(
            FIXED_COST
                .find_iter(after)
                .map(|hit| (hit.start(), hit.as_str())),
        )
        .min_by_key(|(distance, _)| *distance)
        .map(|(_, name)| name)
        .or_else(|| {
            let noun = AFTER_FIGURE.captures(after)?.name("noun")?.as_str();
            (!NO_COST_NAME.is_match(noun)).then_some(noun)
        })
}

fn figure_period(line: &str, end: usize) -> Period {
    let tail = next_chars(&line[end..], 16);
    [
        (&*MONTH, Period::Month),
        (&*WEEK, Period::Week),
        (&*YEAR, Period::Year),
    ]
    .into_iter()
    .find_map(|(pattern, unit)| {
        let got = pattern.find(tail)?;
        tail[..got.start()]
            .trim_matches([' ', ',', '-'])
            .is_empty()
            .then_some(unit)
    })
    .unwrap_or_default()
}

struct BudgetLine<'a> {
    income: Option<f64>,
    costs: Vec<Cost<'a>>,
    pot: Option<f64>,
}

fn budget_line(line: &str) -> BudgetLine<'_> {
    let figures = sums(line);
    let income = incomes(line, &figures);
    let value = (!income.is_empty()).then(|| income.iter().map(|entry| entry.value).sum());
    let mut pot = None;
    let mut costs = Vec::new();
    for figure in figures {
        if income.iter().any(|entry| entry.start == figure.start) {
            continue;
        }
        if saving_pot(line, &figure) {
            pot.get_or_insert(figure.value);
            continue;
        }
        let Some(name) = cost_name(line, &figure) else {
            costs.clear();
            break;
        };
        costs.push(Cost {
            name,
            value: figure.value,
            unit: figure_period(line, figure.end),
        });
    }
    BudgetLine {
        income: value,
        costs,
        pot,
    }
}

fn choice(shapes: &'static [&str], line: &str, was: &str) -> &'static str {
    let start = line.chars().map(|letter| letter as usize).sum::<usize>() % shapes.len();
    shapes
        .iter()
        .cycle()
        .skip(start)
        .take(shapes.len())
        .copied()
        .find(|shape| {
            let opener = shape.split('{').next().unwrap_or("").trim();
            opener.is_empty() || !was.contains(next_chars(opener, 28))
        })
        .unwrap_or(shapes[start])
}

fn pounds(value: f64) -> String {
    let pennies = (value * 100.0).round_ties_even() as u64;
    let integer = (pennies / 100).to_string();
    let mut result = String::from("£");
    for (index, digit) in integer.chars().enumerate() {
        if index > 0 && (integer.len() - index).is_multiple_of(3) {
            result.push(',');
        }
        result.push(digit);
    }
    if !pennies.is_multiple_of(100) {
        result.push_str(&format!(".{:02}", pennies % 100));
    }
    result
}

fn named(costs: &[Cost<'_>]) -> String {
    let mut names = Vec::new();
    for cost in costs {
        let name = match cost.name {
            "rents" => "rent",
            "loans" => "loan payments",
            "repayments" => "repayment",
            name => name,
        };
        if !names.contains(&name) {
            names.push(name);
        }
    }
    match names.split_last() {
        Some((only, [])) => (*only).to_owned(),
        Some((last, rest)) => format!("{} and the {last}", rest.join(", ")),
        None => String::new(),
    }
}

struct ResultLine {
    reply: String,
    previous_month: Option<(f64, Period)>,
}

fn render(line: &str, earlier: &[Exchange]) -> Option<ResultLine> {
    let raw = line.trim();
    if raw.is_empty() || raw.chars().count() > 220 {
        return None;
    }
    let line = raw.replace('’', "'").to_lowercase();
    if NOT_BUDGET.is_match(&line)
        || ASK.is_match(&line)
        || OTHER_CURRENCY.is_match(&line)
        || periods()
            .iter()
            .filter(|(pattern, _)| pattern.is_match(&line))
            .count()
            > 1
    {
        return None;
    }
    let now = budget_line(&line);
    let previous: Vec<_> = earlier
        .iter()
        .rev()
        .filter(|turn| !turn.user.trim().is_empty())
        .take(2)
        .map(|turn| turn.user.to_lowercase())
        .collect();
    let was = earlier.last().map_or("", |turn| turn.bot.as_str());
    let unit_now = period(&line);
    let chat_period = previous
        .iter()
        .find_map(|line| {
            let unit = period(line);
            (unit != Period::Unstated).then_some(unit)
        })
        .unwrap_or_default();
    let (income, unit) = if let Some(income) = now.income {
        (
            income,
            if unit_now == Period::Unstated {
                chat_period
            } else {
                unit_now
            },
        )
    } else {
        if now.costs.is_empty() {
            return None;
        }
        let (income, inherited) = previous.iter().find_map(|prior| {
            budget_line(prior)
                .income
                .map(|income| (income, period(prior)))
        })?;
        (
            income,
            [inherited, unit_now, chat_period]
                .into_iter()
                .find(|unit| *unit != Period::Unstated)
                .unwrap_or_default(),
        )
    };
    if now.costs.is_empty() {
        if unit == Period::Unstated
            || !earlier.iter().rev().take(2).any(|turn| {
                SAVINGS.is_match(&turn.user.to_lowercase())
                    || SAVINGS.is_match(&turn.bot.to_lowercase())
            })
        {
            return None;
        }
        let reply = choice(A_SHARE, raw, was)
            .replace("{in_amount}", &pounds(income))
            .replace("{low}", &pounds(income * 0.10))
            .replace("{high}", &pounds(income * 0.20))
            .replace("{period}", unit.wording());
        return Some(ResultLine {
            reply,
            previous_month: None,
        });
    }
    let cost_unit = now.costs.last()?.unit;
    let mut costs = Vec::new();
    if now.income.is_none() {
        for prior in previous.iter().rev() {
            let budget = budget_line(prior);
            costs.extend(
                budget
                    .costs
                    .into_iter()
                    .filter(|cost| !now.costs.iter().any(|entry| entry.name == cost.name)),
            );
        }
    }
    costs.extend(now.costs);
    if costs
        .iter()
        .any(|cost| cost.unit != Period::Unstated && cost.unit != unit)
    {
        return None;
    }
    let convert = unit == Period::Year && cost_unit != Period::Year;
    let income_for_costs = if convert {
        (income / 12.0).round_ties_even()
    } else {
        income
    };
    let outgoing: f64 = costs.iter().map(|item| item.value).sum();
    let left = income_for_costs - outgoing;
    if left <= 0.0 || left < income_for_costs * 0.05 {
        return None;
    }
    let unit = if convert { Period::Month } else { unit };
    let left_said = if unit == Period::Unstated {
        pounds(left)
    } else {
        format!("{} {}", pounds(left), unit.wording())
    };
    let template = if convert { YEAR_AND_MONTH } else { LEFT };
    let mut reply = choice(template, raw, was)
        .replace("{in_amount}", &pounds(income))
        .replace("{a_month}", &pounds(income_for_costs))
        .replace("{out_amount}", &pounds(outgoing))
        .replace("{named}", &named(&costs))
        .replace("{left}", &left_said);
    if !convert && let Some(pot) = now.pot {
        reply.push_str(&format!(
            " The {} you put by comes out of that.",
            pounds(pot)
        ));
    }
    Some(ResultLine {
        reply,
        previous_month: Some((left, unit)),
    })
}

pub(in crate::bot) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let line = reading.raw.to_lowercase();
    if (LEFT_ASK.is_match(&line) || START.is_match(&line))
        && let Some((last, history)) = memory.turns.split_last()
        && last.module == "the_month_they_have"
        && let Some(previous) = render(&last.user, history)?.previous_month
    {
        let (amount, unit) = previous;
        if LEFT_ASK.is_match(&line) {
            let unit = if unit == Period::Unstated {
                Period::Month
            } else {
                unit
            };
            return Some(
                choice(LEFT_AGAIN, reading.raw, &last.bot)
                    .replace("{left}", &pounds(amount.round_ties_even()))
                    .replace("{unit}", unit.wording()),
            );
        }
        if POINTS_BACK.is_match(&line) || line.split_whitespace().count() <= 4 {
            return Some(choice(FIRST_MOVE, reading.raw, &last.bot).to_owned());
        }
        return None;
    }
    render(reading.raw, &memory.turns).map(|summary| summary.reply)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bot::Bot;

    #[test]
    fn stated_hours_other_currencies_and_mixed_periods_are_not_budget_sums() {
        for prompt in [
            "I earn 1800 a month and rent is 200 a week",
            "I earn 1800 and have a 90 minute commute",
            "I earn $1800 and rent is $900",
            "I am on 1257L and rent is 700",
            "I earn 1800 and my rent is 700, is that enough?",
            "My rent is in arrears and I earn 1800 and rent is 700",
        ] {
            assert!(render(prompt, &[]).is_none(), "{prompt}");
        }
    }

    #[test]
    fn combines_two_incomes_and_annualises_against_monthly_costs() {
        let answer = render("I make 1200 and my partner makes 800 and rent is 900", &[]).unwrap();
        assert_eq!(answer.previous_month, Some((1_100.0, Period::Unstated)));
        assert!(answer.reply.contains("£1,100"));
        let answer = render("I earn 30000 a year and my rent is 700", &[]).unwrap();
        assert_eq!(answer.previous_month, Some((1_800.0, Period::Month)));
    }

    #[test]
    fn first_turn_and_continuation_use_the_same_month() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let one = bot.turn("We bring in 2400 a month and the mortgage is 900");
        assert_eq!(one.module, Some("the_month_they_have"), "{}", one.reply);
        assert!(one.reply.contains("£1,500"));
        let two = bot.turn("The nursery is 600 on top");
        assert_eq!(two.module, Some("the_month_they_have"), "{}", two.reply);
        assert!(
            two.reply.contains("£900")
                && two.reply.contains("mortgage")
                && two.reply.contains("nursery")
        );
        let three = bot.turn("What's left?");
        assert_eq!(three.module, Some("the_month_they_have"), "{}", three.reply);
        assert!(three.reply.contains("£900 a month"));
    }
}
