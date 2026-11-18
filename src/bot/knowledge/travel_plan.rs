//! Monthly trip suggestions and per-destination planning and budgets.

use std::{borrow::Cow, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading, memory::stated_plans::Facts};

use super::city_guides::named_place;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(MONTH_OTHER, super::TRAVEL_PLAN_MONTH_OTHER);
pattern!(MONTH_IN_TRIP, super::TRAVEL_PLAN_MONTH_IN_TRIP);
pattern!(NAMED_TRIP, super::TRAVEL_PLAN_NAMED_TRIP);
pattern!(TRIP_WORD, super::TRAVEL_PLAN_TRIP_WORD);
pattern!(REFUSAL, super::TRAVEL_PLAN_REFUSAL);
pattern!(SOMEONE_WANTS, super::TRAVEL_PLAN_SOMEONE_WANTS);
pattern!(REFUSES_WANT, super::TRAVEL_PLAN_REFUSES_WANT);
pattern!(TRIP_GO, super::TRAVEL_PLAN_TRIP_GO);
pattern!(ABOUT_TRIP, super::TRAVEL_PLAN_ABOUT_TRIP);
pattern!(JOURNEY, super::TRAVEL_PLAN_JOURNEY);
pattern!(TREATMENT, super::TRAVEL_PLAN_TREATMENT);
pattern!(WORDS, r"[a-z']+");
pattern!(PLACE_WORD, r"[a-z][a-z-]*");
pattern!(STAG, r"\bstag\b|\bhen (?:do|weekend|night)\b");
pattern!(
    RETURN_HOME,
    r"\b(?:go|going|get|getting) (?:home|back|in|out)\b"
);
pattern!(COMPARE, r"\bor\b|\bvs\b|versus|/");
pattern!(
    TRIP_REPORT,
    r"\b(?:(?:i|we) (?:am|are|have|got|booked|plan|planning|fancy)|i'm|we're)\b"
);
pattern!(THREE_WEEKS, r"\b(?:three|3) weeks?\b|\b(?:20|21) days\b");
pattern!(
    TWO_WEEKS,
    r"\b(?:two|2) weeks?\b|\b(?:14|13|12|11|10|ten) days\b"
);
pattern!(
    ONE_WEEK,
    r"\b(?:one|1) week\b|\ba week\b|\b(?:seven|7|five|5|six|6) days\b"
);
pattern!(WEEKEND, r"\bweekend\b|\b(?:two|2|three|3) nights\b");
pattern!(
    COST_WORD,
    r"\b(?:how much|costs?|costing|expensive|pricey|afford|budget|spend(?:ing)?|will that|does that|go far|saved|money|cheap|inexpensive|affordable)\b"
);
pattern!(
    COST_ITEM,
    r"\b(?:flights?|fly|flying|hotels?|accommodation|apartment|villa|hostel|tickets?|deals?|package|insurance|car hire)\b"
);
pattern!(
    COST_LENGTH,
    r"\b(?:days|nights|how long|too long|too short)\b"
);
pattern!(COST_AMOUNT, r"\d[\d,]*");
pattern!(
    COST_NOT_MONEY,
    r"\b(?:warm|cold|hot|good|bad|big|small|deep|strong|quick|fast|early|late|cheap|safe|close|near|light|heavy|bright|tall|old|fit|well|soon|long|clear|loud|quiet|tidy) enough\b"
);

const MONTHS: &[&str] = &[
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

fn phrases(line: &str, words: &[&str]) -> bool {
    words.iter().any(|word| line.contains(word))
}

fn word_in(line: &str, words: &[&str]) -> bool {
    WORDS
        .find_iter(line)
        .any(|word| words.contains(&word.as_str()))
}

fn whole_phrase(line: &str, words: &[&str]) -> bool {
    words.iter().any(|word| named_place(line, word))
}

fn label(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .into_iter()
                .flat_map(char::to_uppercase)
                .chain(chars)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn month_matches(line: &str) -> Vec<(usize, usize, usize)> {
    let mut found = Vec::new();
    for &(name, number) in super::TRAVEL_PLAN_MONTHS {
        for (start, _) in line.match_indices(name) {
            let end = start + name.len();
            let before = &line[..start];
            let after = &line[end..];
            let word = |character: char| character.is_ascii_alphanumeric() || character == '_';
            if before.ends_with(word) || after.starts_with(word) {
                continue;
            }
            if name == "may"
                && ![
                    "in ", "for ", "this ", "next ", "early ", "late ", "during ", "by ", "until ",
                ]
                .iter()
                .any(|prefix| before.ends_with(prefix))
                && !after.trim_start_matches([' ', ',', '.', '?']).is_empty()
                && !after.starts_with(" or ")
                && !after.starts_with(" is ")
            {
                continue;
            }
            if !found.iter().any(|&(at, _, _)| at == start) {
                found.push((start, end, usize::from(number - 1)));
            }
        }
    }
    found.sort_by_key(|&(start, _, _)| start);
    found
}

fn lookup<'a, T: Copy>(line: &str, table: &'a [(&'a str, T)]) -> Option<(&'a str, T)> {
    table
        .iter()
        .copied()
        .find(|(name, _)| named_place(line, name))
}

fn value<'a>(table: &'a [(&str, &'a str)], key: &str) -> Option<&'a str> {
    table
        .iter()
        .find_map(|&(name, reply)| (name == key).then_some(reply))
}

fn topic_place<'a, T: Copy>(
    clean: &str,
    earlier: &[Exchange],
    plans: Option<&Facts>,
    table: &'a [(&'a str, T)],
) -> Option<(&'a str, T)> {
    lookup(clean, table)
        .or_else(|| super::travel_stays::in_plan(plans, |plan| lookup(plan, table)))
        .or_else(|| {
            earlier
                .iter()
                .rev()
                .take(2)
                .find_map(|turn| lookup(&turn.user.to_lowercase(), table))
        })
}

fn monthly_destination(
    reading: &Reading<'_>,
    clean: &str,
    earlier: &[Exchange],
    plans: Option<&Facts>,
) -> Option<String> {
    let found = month_matches(clean);
    let &(_, end, month) = found.first()?;
    if MONTH_OTHER.is_match(clean)
        || found
            .get(1)
            .is_some_and(|&(next, _, _)| COMPARE.is_match(&clean[end..next]))
    {
        return None;
    }
    let month = MONTHS[month];
    let open = word_in(clean, super::TRAVEL_PLAN_OPEN);
    if !open
        && reading
            .topics
            .iter()
            .any(|&(topic, _)| topic != "small-talk")
        && !reading
            .topics
            .iter()
            .any(|&(topic, _)| super::TRAVEL_PLAN_TOPICS.contains(&topic))
        && !NAMED_TRIP.is_match(clean)
        && month_matches(&reading.line.focus.join(" ")).is_empty()
    {
        return None;
    }
    month_for_place(month, clean, earlier, plans, !open)
        .or_else(|| {
            if whole_phrase(clean, super::TRAVEL_PLAN_KNOWN) {
                return None;
            }
            if TRIP_REPORT.is_match(clean) && !phrases(clean, super::TRAVEL_PLAN_WHERE) {
                return ABOUT_TRIP.is_match(clean).then(|| {
                    format!("Where are you going in {}? The destination makes a difference to the weather and cost.", label(month))
                });
            }
            if clean.contains("ski") || clean.contains("snow") {
                return Some(format!("For a ski trip in {}, check which resorts are open and compare current snow reports. Higher resorts often have better snow coverage.", label(month)));
            }
            let table = if word_in(clean, super::TRAVEL_PLAN_SUN_ASK) {
                super::TRAVEL_PLAN_MONTH_SUN
            } else {
                super::TRAVEL_PLAN_MONTH_PICKS
            };
            let reply = value(table, month)?;
            let reply = reply.split(", and the first week of January").next()?;
            Some(format!("{}: {reply}.", label(month)))
        })
}

fn month_for_place(
    month: &str,
    clean: &str,
    earlier: &[Exchange],
    plans: Option<&Facts>,
    context: bool,
) -> Option<String> {
    let wanted = |table| {
        lookup(clean, table).or_else(|| {
            context
                .then(|| topic_place("", earlier, plans, table))
                .flatten()
        })
    };
    if let Some((name, window)) = lookup(clean, super::TRAVEL_PLAN_WINDOWS)
        .or_else(|| wanted(super::TRAVEL_PLAN_BEST))
        .or_else(|| wanted(super::TRAVEL_PLAN_WINDOWS))
    {
        let first = window.split(['.', ';']).next()?.to_lowercase();
        if named_place(&first, month) || named_place(&first, &month[..3]) {
            return Some(format!(
                "{} is in the usual travel window for {}. {window}",
                label(month),
                label(name)
            ));
        }
        return Some(format!("{} in {}. {window}", label(name), label(month)));
    }
    let (name, kind) = wanted(super::TRAVEL_PLAN_HOME)?;
    let season = value(super::TRAVEL_PLAN_MONTH_SEASON, month)?;
    let advice = value(super::TRAVEL_PLAN_WEEKS_AT_HOME, season)?;
    let terrain = value(super::TRAVEL_PLAN_HOME_KIND, kind)?;
    Some(format!(
        "{} in {}: {advice} {terrain}",
        label(name),
        label(month)
    ))
}

fn month_followup(clean: &str, earlier: &[Exchange], plans: Option<&Facts>) -> Option<String> {
    if !month_matches(clean).is_empty()
        || clean.split_whitespace().any(|word| {
            let word = word.trim_matches(['.', ',', '!', '?', '\'']);
            word.len() > 2 && !super::TRAVEL_PLAN_FILLER.contains(&word)
        })
    {
        return None;
    }
    let month = earlier.iter().rev().take(4).find_map(|turn| {
        let said = turn.user.to_lowercase();
        MONTH_IN_TRIP
            .is_match(&said)
            .then(|| month_matches(&said).first().map(|row| MONTHS[row.2]))
            .flatten()
    })?;
    if let Some(reply) = month_for_place(month, clean, earlier, plans, true) {
        return Some(reply);
    }
    if word_in(clean, super::TRAVEL_PLAN_SUN_ASK) {
        return Some(format!(
            "{}: {}.",
            label(month),
            value(super::TRAVEL_PLAN_MONTH_SUN, month)?
        ));
    }
    let lead = if phrases(clean, super::TRAVEL_PLAN_MONTH_FAULT) {
        ""
    } else {
        "Yes, for the place it suits. "
    };
    Some(format!(
        "{lead}{} is {}.",
        label(month),
        value(super::TRAVEL_PLAN_MONTH_VERDICTS, month)?
    ))
}

fn trip_plan(clean: &str) -> Option<String> {
    let length = if THREE_WEEKS.is_match(clean) {
        Some("Three weeks gives you time to pick a region and split it between two or three bases.")
    } else if TWO_WEEKS.is_match(clean) {
        Some("For two weeks, pick two or three bases and leave time to travel between them.")
    } else if ONE_WEEK.is_match(clean) {
        Some("For a week, try one base and two or three day trips.")
    } else if WEEKEND.is_match(clean) {
        Some("For a weekend, choose one place and build the trip around it.")
    } else {
        None
    };
    let city = lookup(clean, super::TRAVEL_PLAN_CITY_DAYS);
    let place = lookup(clean, super::TRAVEL_PLAN_BEST);
    if length.is_some() && place.is_none() && city.is_none() && !TRIP_WORD.is_match(clean)
        || length.is_none() && place.is_none() && city.is_none() && clean.contains("holiday")
    {
        return None;
    }
    let rest = "Book the transport and stays that constrain your dates. Leave one day free and allow time to reach the departure airport or station.";
    if let Some((name, days)) = city {
        return Some(format!(
            "Allow about {days} full days for {}. {rest}",
            label(name)
        ));
    }
    let shape = length.unwrap_or("Count the nights first, then choose a base for each part of the trip and allow time to get between them.");
    Some(if let Some((place, advice)) = place {
        let window = advice.split(". ").next()?.trim_end_matches('.');
        format!("{shape} For {}: {window}. {rest}", label(place))
    } else {
        format!("{shape} {rest}")
    })
}

pub(super) fn shortlist_shape(reading: &Reading<'_>) -> bool {
    let line = &reading.line.clean;
    whole_phrase(line, super::TRAVEL_PLAN_PRICE)
        || !whole_phrase(line, super::TRAVEL_PLAN_VAGUE_ASK)
        || !whole_phrase(line, super::TRAVEL_PLAN_LENGTH_ONLY)
}

fn shortlist(line: &str) -> &'static str {
    super::TRAVEL_PLAN_SHORTLIST.iter().find_map(|&(cues, reply)| whole_phrase(line, cues).then_some(reply))
        .unwrap_or("For a week in the sun, consider the Canaries, Greece or southern Spain. For a long weekend, consider Lisbon, Porto, Prague or Edinburgh. If you are travelling with children, the Algarve and Menorca are worth comparing. What month are you planning to go?")
}

fn wanted_place(line: &str) -> Option<String> {
    let tail = TRIP_GO.captures(line)?.get(1)?.as_str();
    let mut words = PLACE_WORD
        .find_iter(tail)
        .map(|word| word.as_str())
        .filter(|word| !super::TRAVEL_PLAN_PLACE_FILLER.contains(word));
    let first = words.next()?;
    if super::TRAVEL_PLAN_NOT_PLACE.contains(&first)
        || super::TRAVEL_PLAN_TRIP_SHAPE.contains(&first)
    {
        return None;
    }
    let rest = words.take(2).take_while(|word| {
        !super::TRAVEL_PLAN_PLACE_END.contains(word) && !super::TRAVEL_PLAN_NOT_PLACE.contains(word)
    });
    Some(
        std::iter::once(first)
            .chain(rest)
            .collect::<Vec<_>>()
            .join(" "),
    )
}

pub(super) fn wants_trip(reading: &Reading<'_>) -> bool {
    let line = &reading.line.clean;
    if REFUSAL.is_match(line) {
        return false;
    }
    if STAG.is_match(line) {
        return true;
    }
    if word_in(line, super::TRAVEL_PLAN_SOMEONE_ELSE)
        || SOMEONE_WANTS.is_match(line)
        || REFUSES_WANT.is_match(line)
        || word_in(line, super::TRAVEL_PLAN_NOT_PLACE)
        || word_in(line, super::TRAVEL_PLAN_NOT_EVENT)
    {
        return false;
    }
    wanted_place(line).is_some_and(|_| !whole_phrase(line, super::TRAVEL_PLAN_KNOWN))
        || (word_in(line, super::TRAVEL_PLAN_TRIP_SHAPE) && !RETURN_HOME.is_match(line))
}

fn wanted_trip(line: &str) -> Option<Cow<'static, str>> {
    if REFUSAL.is_match(line) {
        return None;
    }
    if STAG.is_match(line) {
        return Some(Cow::Borrowed(
            "Pick one city, agree a budget and plan one activity everyone wants. If people are paying separately, collect their share before you book. Compare destinations such as Budapest, Kraków or Prague using prices for your dates.",
        ));
    }
    if whole_phrase(line, super::TRAVEL_PLAN_KNOWN) {
        return None;
    }
    Some(if let Some(name) = wanted_place(line) {
        Cow::Owned(format!(
            "For a few days in {}, agree the dates and a budget, then pick one base. Book your transport and rooms once everyone is committed. Leave room for meals and time to explore.",
            label(&name)
        ))
    } else {
        Cow::Borrowed(shortlist(line))
    })
}

pub(super) fn trip_budget(clean: &str, earlier: &[Exchange]) -> Option<String> {
    let amount = COST_AMOUNT
        .find_iter(clean)
        .filter_map(|digits| digits.as_str().replace(',', "").parse::<u32>().ok())
        .find(|amount| (50..=20_000).contains(amount));
    let cost = COST_WORD.is_match(clean);
    if COST_ITEM.is_match(clean)
        || (!cost
            && amount.is_none()
            && (!clean.contains("enough")
                || COST_NOT_MONEY.is_match(clean)
                || COST_LENGTH.is_match(clean)))
    {
        return None;
    }
    let find = |line: &str| {
        super::TRAVEL_PLAN_BUDGETS
            .iter()
            .copied()
            .find(|&(name, _, _)| named_place(line, name))
    };
    let (place, low, high) = find(clean).or_else(|| {
        earlier
            .iter()
            .rev()
            .take(2)
            .find_map(|turn| find(&turn.user.to_lowercase()))
    })?;
    let label = label(place);
    let days = super::TRAVEL_PLAN_UNITS
        .iter()
        .find_map(|&(unit, days)| named_place(clean, unit).then_some(days));
    let range = if let Some(days) = days {
        format!(
            "For {days} days in {label}, a rough starting budget is £{} to £{} per person for meals, local transport and activities.",
            low * days,
            high * days
        )
    } else {
        format!(
            "For {label}, a rough starting budget is £{low} to £{high} per person per day for meals, local transport and activities."
        )
    };
    let comparison = if let (Some(days), Some(amount)) = (days, amount) {
        if clean.contains(['$', '€']) {
            ""
        } else if amount < low * days {
            " Your stated amount is below that range."
        } else if amount < high * days {
            " Your stated amount is within that range."
        } else {
            " Your stated amount covers that range."
        }
    } else {
        ""
    };
    Some(format!(
        "{range}{comparison} Add accommodation and the journey there separately, and check prices for your dates."
    ))
}

pub(super) fn journey_not_treatment(reading: &Reading<'_>) -> bool {
    JOURNEY.is_match(&reading.line.clean) || !TREATMENT.is_match(&reading.line.clean)
}

// https://www.gov.uk/guidance/foreign-travel-checklist
const ESSENTIALS: &str = "Save your transport and accommodation details where you can access them offline. For travel abroad, check your destination's passport-validity and visa requirements and arrange travel insurance. Keep a secure backup of essential documents and emergency contacts. Check the destination's plug type, drinking-water advice and any rules for medicines you plan to carry.";

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
    plans: Option<&Facts>,
) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    match id {
        "where-in-month" => monthly_destination(reading, clean, earlier, plans).map(Cow::Owned),
        "month-verdict-followup" => month_followup(clean, earlier, plans).map(Cow::Owned),
        "plan-a-trip" => trip_plan(clean).map(Cow::Owned),
        "holiday-costs" if whole_phrase(clean, super::TRAVEL_PLAN_COST_SUBJECT) => {
            Some(Cow::Borrowed(
                "Compare total costs for a few nearby dates, including bags, airport transfers and accommodation. Check public transport, set a daily budget for meals and activities, and compare package and separate prices before booking.",
            ))
        }
        "where-no-place" => Some(Cow::Borrowed(shortlist(clean))),
        "want-a-trip" => wanted_trip(clean),
        "trip-budget" => trip_budget(clean, earlier).map(Cow::Owned),
        "trip-essentials" => Some(Cow::Borrowed(ESSENTIALS)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_replies_distinguish_open_and_pointed_questions() {
        let open = Reading::new("Where should I go in September?");
        let out = monthly_destination(&open, &open.line.clean, &[], None).unwrap();
        assert!(out.starts_with("September:"));
        let comparison = Reading::new("Should I go away in August or September?");
        assert!(monthly_destination(&comparison, &comparison.line.clean, &[], None).is_none());
        let prior = [Exchange {
            user: "We are planning a trip in August".into(),
            bot: String::new(),
            module: "answers",
        }];
        assert!(
            month_followup("is that a good time to go", &prior, None)
                .unwrap()
                .contains("August")
        );
        assert!(month_followup("is the eden project worth it", &prior, None).is_none());
    }

    #[test]
    fn travel_plan_scope_and_budget_arithmetic() {
        assert!(!wants_trip(&Reading::new(
            "I want to go to the gym three times a week"
        )));
        assert!(
            wanted_trip("my mates want to go to benidorm")
                .unwrap()
                .contains("Benidorm")
        );
        assert!(trip_plan("my new job starts in two weeks").is_none());
        let cost = trip_budget("is 500 pounds enough for a long weekend in lisbon", &[]).unwrap();
        assert!(cost.contains("3 days in Lisbon"));
        assert!(cost.contains("£135 to £240"));
        assert!(trip_budget("is it warm enough in lisbon", &[]).is_none());
    }
}
