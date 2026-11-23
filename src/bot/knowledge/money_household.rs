// Changed in the cppalliance fork of RustBot: the time comes from the turn. See "Changes in this fork" in README.md.

//! Household cash flow and immediate food budgeting.

use std::{borrow::Cow, sync::LazyLock};

use jiff::civil::Date;
use regex::Regex;

use crate::bot::Reading;

use super::money_savings::{count, sums};

#[path = "money_household_data.rs"]
mod data;

macro_rules! patterns {
    ($($name:ident),+ $(,)?) => {
        $(static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(data::$name).expect(stringify!($name)));)+
    };
}

patterns!(
    A_MONTH_NAMED,
    A_SUM_NAMED,
    A_THING_TO_COVER,
    A_SPEND_ON_A_THING,
    A_FOOD_WORD,
    AN_OCCASION_OR_A_TREAT,
    A_BIRTHDAY,
    A_THIN_WEEK,
    ASKS_FOR_SOMETHING_TO_DO,
    ASKS_WHAT_TO_COOK,
    SAYS_NOTHING_AT_ALL,
    A_TRIP,
    A_STATED_BUDGET,
    ASKS_THE_PRICE,
    ASKS_HOW_LONG,
    NO_MONEY,
    THE_FIGURE,
    A_MEAL,
    A_HEADCOUNT,
    A_THE_HOUSEHOLD,
    DEBT_DUE,
    A_DEBT,
    SERVICING_A_DEBT,
    DAY_NAMED,
    A_WEEK_DAY_NAMED,
    TWO_OF_US,
    A_HOUSEHOLD,
    A_STATED_SIZE,
);

const STAPLES: &str = "Buy the staples first if you need them, such as rice, pasta, tinned tomatoes, lentils, eggs, oats, potatoes and frozen veg. Meals you can build around them include tomato pasta, lentil dhal, egg fried rice, jacket potato with beans and chickpea curry.";
const DINNERS: &str =
    "tomato pasta, lentil dhal, egg fried rice, jacket potato with beans or chickpea curry";
const FOOD_HELP: &str = "If you have no money for food, look up your local food bank at trussell.org.uk/emergency-food. Its team can tell you where to get a referral. You can also ask your council about emergency support.";
const WEEKDAYS: &[&str] = &[
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
];
const SPANS: &[(&str, i32)] = &[
    ("a fortnight", 14),
    ("two weeks", 14),
    ("a month", 30),
    ("for the month", 30),
    ("this month", 30),
    ("a week", 7),
    ("this week", 7),
    ("ten days", 10),
    ("a few days", 4),
    ("the week", 7),
    ("today", 1),
    ("tonight", 1),
];

pub(super) fn pounds(amount: f64) -> String {
    let pennies = (amount * 100.0).round_ties_even() as u64;
    if pennies.is_multiple_of(100) {
        format!("£{}", super::money_savings::comma(pennies / 100))
    } else {
        format!(
            "£{}.{:02}",
            super::money_savings::comma(pennies / 100),
            pennies % 100
        )
    }
}

fn named_amount(text: &str) -> Option<f64> {
    if let Some(captures) = THE_FIGURE.captures(text) {
        let amount = captures
            .get(1)
            .or_else(|| captures.get(2))?
            .as_str()
            .replace(',', "");
        return amount
            .parse::<f64>()
            .ok()
            .filter(|amount| *amount > 0.0 && *amount < 1e15);
    }
    [("tenner", 10.0), ("fiver", 5.0), ("a score", 20.0)]
        .into_iter()
        .find_map(|(word, amount)| text.contains(word).then_some(amount))
}

fn days_to_weekday(today: Date, weekday: i32) -> i32 {
    let days = (weekday - i32::from(today.weekday().to_monday_zero_offset())).rem_euclid(7);
    if days == 0 { 7 } else { days }
}

fn plain_days(text: &str, today: Date) -> Option<(i32, bool)> {
    if let Some(&(_, days)) = SPANS.iter().find(|(phrase, _)| text.contains(phrase)) {
        return Some((days, false));
    }
    if let Some(weekday) = WEEKDAYS.iter().position(|weekday| text.contains(weekday)) {
        return Some((days_to_weekday(today, weekday as i32), false));
    }
    if text.contains("payday") {
        return Some((14, true));
    }
    text.contains("weekend").then_some((5, false))
}

fn day_label(text: &str, today: Date) -> Option<(String, i32)> {
    let captures = DAY_NAMED.captures(text)?;
    let said = &captures[1];
    if let Some(day) = WEEKDAYS.iter().position(|day| *day == said) {
        let name = format!("{}{}", said[..1].to_uppercase(), &said[1..]);
        return Some((format!("until {name}"), days_to_weekday(today, day as i32)));
    }
    let (label, days) = match said {
        "today" => ("until today", 1),
        "tonight" => ("until tonight", 1),
        "tomorrow" => ("until tomorrow", 1),
        "the weekend"
        | "the week"
        | "this week"
        | "the end of the week"
        | "end of the week"
        | "next week" => ("until the end of the week", days_to_weekday(today, 6)),
        "next friday" => ("until Friday", days_to_weekday(today, 4)),
        "a week" => ("for a week", 7),
        "a fortnight" | "the fortnight" | "two weeks" => ("for a fortnight", 14),
        _ => {
            let last = today.last_of_month();
            let days = today.until(last).ok()?.get_days().max(1);
            let label = if said.contains("month") {
                "until the end of the month"
            } else {
                "until payday"
            };
            (label, days)
        }
    };
    Some((label.to_owned(), days))
}

fn headcount(captures: &regex::Captures<'_>) -> Option<u64> {
    let said = captures.iter().skip(1).flatten().next()?.as_str();
    count(said)
        .or(match said {
            "eleven" => Some(11),
            "twelve" => Some(12),
            _ => None,
        })
        .filter(|heads| *heads > 0)
}

pub(crate) fn enough_for_food(reading: &Reading<'_>) -> bool {
    let text = &reading.clean;
    !A_MONTH_NAMED.is_match(text) || A_SUM_NAMED.is_match(text) || A_THING_TO_COVER.is_match(text)
}

fn money_to_last(text: &str, today: Date) -> Option<Cow<'static, str>> {
    let food = A_FOOD_WORD.is_match(text);
    let asks_how_long = ASKS_HOW_LONG.is_match(text);
    let no_money = NO_MONEY.is_match(text);
    let amount = named_amount(text);
    if (ASKS_THE_PRICE.is_match(text) && food && !asks_how_long && !no_money && amount.is_none())
        || (A_STATED_BUDGET.is_match(text) && !asks_how_long)
        || (!food && (A_SPEND_ON_A_THING.is_match(text) || ASKS_FOR_SOMETHING_TO_DO.is_match(text)))
    {
        return None;
    }
    if !food && A_BIRTHDAY.is_match(text) && A_THIN_WEEK.is_match(text) {
        return Some(Cow::Borrowed(
            "Mark the birthday with something they enjoy that fits what's available. Try a favourite meal from the cupboard, a homemade card or a free outing to a park. How old are they turning?",
        ));
    }
    if !food
        && (AN_OCCASION_OR_A_TREAT.is_match(text) || (amount.is_none() && A_TRIP.is_match(text)))
    {
        return None;
    }
    let days = plain_days(text, today);
    if let Some(amount) = amount {
        let shown = pounds(amount);
        let line = match days {
            Some((1, _)) => format!(
                "{shown} for the day. Check what's already in and plan around one meal you can cook in double, such as {DINNERS}."
            ),
            Some((days, true)) => format!(
                "If payday is a fortnight off, {shown} is {} a day. {STAPLES}",
                pounds(amount / f64::from(days))
            ),
            Some((days, false)) => format!(
                "{shown} over {days} days is {} a day. {STAPLES}",
                pounds(amount / f64::from(days))
            ),
            None => {
                if let Some(heads) = A_HEADCOUNT
                    .captures(text)
                    .and_then(|captures| headcount(&captures))
                {
                    format!(
                        "{shown} for {heads} is {} a head. Check what you have in for {DINNERS}.",
                        pounds(amount / heads as f64)
                    )
                } else if A_MEAL.is_match(text) {
                    format!(
                        "For a meal on {shown}, check what you've already got for {DINNERS}. How many people does it need to feed?"
                    )
                } else {
                    format!(
                        "If that has to last a week, {shown} is {} a day. {STAPLES}",
                        pounds(amount / 7.0)
                    )
                }
            }
        };
        return Some(Cow::Owned(line));
    }
    if no_money || (days.is_none() && SAYS_NOTHING_AT_ALL.is_match(text)) {
        return if !no_money && ASKS_WHAT_TO_COOK.is_match(text) {
            Some(Cow::Owned(format!("{STAPLES} {FOOD_HELP}")))
        } else {
            Some(Cow::Owned(format!(
                "{FOOD_HELP} If there are staples in the cupboard, try {DINNERS}."
            )))
        };
    }
    Some(Cow::Borrowed(STAPLES))
}

fn make_it_last(text: &str, today: Date) -> Option<String> {
    let amount = sums(text).map(|(sum, _)| sum).max_by(f64::total_cmp)?;
    let (label, days) = day_label(text, today)?;
    if amount <= 0.0 || A_WEEK_DAY_NAMED.is_match(text) {
        return None;
    }
    let per_day = amount / f64::from(days);
    let shown_amount = pounds(amount);
    let shown_day = pounds(per_day);
    if A_FOOD_WORD.is_match(text) {
        let heads = A_STATED_SIZE
            .captures(text)
            .and_then(|captures| headcount(&captures))
            .or(if TWO_OF_US.is_match(text) {
                Some(2)
            } else if A_HOUSEHOLD.is_match(text) {
                None
            } else {
                Some(1)
            });
        let share = heads
            .filter(|heads| *heads > 1)
            .map_or_else(String::new, |heads| {
                format!(
                    " That's {} per person per day for {heads}.",
                    pounds(per_day / heads as f64)
                )
            });
        let tight = if per_day / (heads.unwrap_or(1) as f64) < 3.0 {
            " If you're likely to run short of food, contact your council or a food bank now."
        } else {
            ""
        };
        return Some(format!(
            "{shown_amount} {label} is {shown_day} a day on food.{share}{tight} {STAPLES}"
        ));
    }
    let plan = if per_day < 3.0 {
        "Set aside what has to go on transport, prescriptions or other essentials and check what food you already have. If you can't cover basics, contact the council or Citizens Advice now about emergency support."
    } else if per_day < 15.0 {
        "Set aside what has to go on travel, phone and other essentials. Plan one shop around staples and what you've already got."
    } else {
        "Set aside the bills and travel costs due in that time. Plan one shop and carry any money left over into the next stretch."
    };
    Some(format!("That is {shown_day} a day {label}. {plan}"))
}

fn debt_due(text: &str) -> bool {
    A_DEBT.is_match(text) && DEBT_DUE.is_match(text)
}

pub(crate) fn cash_flow_trouble(reading: &Reading<'_>) -> bool {
    let text = &reading.clean;
    !data::ASKING_FOR_AN_OVERDRAFT
        .iter()
        .any(|phrase| text.contains(phrase))
        && (!A_DEBT.is_match(text) || debt_due(text) || !SERVICING_A_DEBT.is_match(text))
}

fn money_trouble(text: &str) -> &'static str {
    if debt_due(text) {
        "Contact the lender before the payment is due if you can. Explain what you can afford after essentials and ask what help they can offer, such as a payment plan or a freeze on interest. Ask how an arrangement would affect your credit file and ask for it in writing. MoneyHelper can put you in touch with free debt advice. Which payment is due, and when?"
    } else if data::SPENT_UP.iter().any(|phrase| text.contains(phrase)) {
        "Write down which essentials need paying before the next money comes in. Contact the firms you may struggle to pay and ask what they can offer. Check what meals you can make with food already in. What is left, and how long till you are paid?"
    } else {
        "Cover food, housing, energy and other essentials first. Look at non-essential subscriptions and any bills you can lower without an exit fee. If Council Tax is the issue in England, ask the council about paying over 12 months and whether you qualify for a reduction. Which part of the budget is tightest?"
    }
}

// Defra Family Food FYE 2025 estimates per person per week, rounded for budgeting.
fn food_bill(text: &str) -> &'static str {
    let monthly = text.contains("a month")
        || text.contains("per month")
        || text.contains("monthly")
        || text.contains("each month")
        || text.contains("every month");
    let daily = text.contains("a day")
        || text.contains("per day")
        || text.contains("daily")
        || text.contains("each day")
        || text.contains("every day");
    let weekly = text.contains("a week")
        || text.contains("per week")
        || text.contains("weekly")
        || text.contains("each week")
        || text.contains("every week")
        || text.contains("week for food");
    let household = A_THE_HOUSEHOLD.is_match(text);
    if ASKS_THE_PRICE.is_match(text) && monthly {
        if household {
            "Defra's UK survey for April 2024 to March 2025 puts groceries and non-alcoholic drinks at roughly £146 per person per month, or £292 for two. Use your recent receipts to adjust for your household."
        } else {
            "Defra's UK survey for April 2024 to March 2025 puts groceries and non-alcoholic drinks at roughly £146 per person per month. All food and drink, including eating out and alcohol, came to about £217."
        }
    } else if daily {
        "Defra's UK survey for April 2024 to March 2025 works out at about £4.80 per person per day on groceries and non-alcoholic drinks, or £7.15 including eating out and alcohol. Use local shop prices and what you plan to cook to set your own budget."
    } else if ASKS_THE_PRICE.is_match(text)
        || (weekly
            && !["how do i", "how can i", "how to"]
                .iter()
                .any(|ask| text.contains(ask)))
    {
        if household {
            "Defra's UK survey for April 2024 to March 2025 puts groceries and non-alcoholic drinks at about £34 per person per week, roughly £67 for two or £135 for four. That's a survey average. Your own shop can vary with ages, diets and prices."
        } else {
            "Defra's UK survey for April 2024 to March 2025 puts groceries and non-alcoholic drinks at about £34 per person per week. Including food out and alcohol, the average was about £50. Start with your recent receipts to set your own budget."
        }
    } else {
        "Plan the week around what's already in the cupboard, shop with a list, and compare prices per kilo. Own-brand staples, frozen veg, beans and lentils are worth checking for meals you cook often."
    }
}

pub(crate) fn reply(id: &str, reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let text = reading.raw.to_lowercase();
    match id {
        "make-it-last" => make_it_last(&text, reading.now().date()).map(Cow::Owned),
        "money-to-last" => money_to_last(&text, reading.now().date()),
        "money-trouble" => Some(Cow::Borrowed(money_trouble(&reading.clean))),
        "food-bill" => Some(Cow::Borrowed(food_bill(&text))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn household_budget_keeps_food_bills_and_deadlines_distinct() {
        let today = Date::new(2026, 11, 18).unwrap();
        assert!(money_to_last("how much should i budget for food a week", today).is_none());
        assert!(
            money_to_last("i have 20 quid until friday", today)
                .unwrap()
                .contains("£10 a day")
        );
        assert!(make_it_last("i have 20 pounds until friday", today).is_none());
        assert!(
            make_it_last("i have £24 until the end of the month", today)
                .unwrap()
                .contains("£2 a day")
        );
        assert!(
            money_to_last("i have no money for food", today)
                .unwrap()
                .contains("referral")
        );
    }
}
