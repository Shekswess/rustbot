//! The household and table-size portion of the pinned count reader.

use super::{flatten, matches, starts};
use crate::bot::Exchange;
use fancy_regex::Regex;

#[path = "household_counts_data.rs"]
mod data;
use data::*;

#[derive(Default)]
pub(super) struct Counts {
    household: Option<u16>,
    table: Option<u16>,
    booked: bool,
}

fn number(said: &str) -> Option<u16> {
    if let Some(found) = crate::bot::memory::groupages::decimal(said) {
        return (found > 0).then_some(found);
    }
    let said = said.to_lowercase();
    NUMBER_MAP
        .iter()
        .find_map(|(word, number)| (*word == said && *number > 0).then_some(*number))
}

fn number_in(text: &str, patterns: &[&Regex]) -> Option<u16> {
    patterns.iter().copied().find_map(|pattern| {
        pattern.captures_iter(text).flatten().find_map(|matched| {
            (1..matched.len())
                .filter_map(|i| matched.get(i))
                .find_map(|amount| number(amount.as_str()))
        })
    })
}

fn many(number: u16) -> String {
    NUMBER_MAP[..PET_NUMBERS]
        .iter()
        .find_map(|(word, value)| (*value == number && !word.contains(' ')).then_some(*word))
        .map_or_else(|| number.to_string(), str::to_owned)
}

fn group_reply(number: u16) -> String {
    match number {
        1 => "Just you, then.".into(),
        2 => "The two of you, then.".into(),
        _ => format!("There are {} of you.", many(number)),
    }
}

fn is_a_meal(line: &str) -> bool {
    matches(&A_MEAL_IN_THE_CHAT, line)
}

fn sets_a_table(line: &str) -> bool {
    is_a_meal(line) || matches(&A_TABLE_BOOKED, line) || matches(&A_TABLE_CUE, line)
}

impl Counts {
    pub(super) fn observe(&mut self, text: &str, turns: &[Exchange]) {
        if starts(&ASKING, text) {
            return;
        }
        let group = number_in(text, &[&GROUP_OF_US, &GROUP_OF_FOUR, &GROUP_CORRECTED]);
        if let Some(group) = group {
            self.household = Some(group);
        }
        let explicitly = number_in(text, &[&PARTY]);
        let adjusted = self
            .table
            .filter(|_| !matches(&ABOUT_THE_HOUSE, text))
            .and_then(|older| {
                if starts(&ONE_MORE, text) {
                    return older.checked_add(1);
                }
                if starts(&ONE_LESS, text) {
                    return (older > 1).then(|| older - 1);
                }
                number_in(text, &[&CHANGED_BARE, &NOW_THE_NUMBER])
            });
        let meal = turns
            .iter()
            .rev()
            .take(LOOK_BACK_FOR_A_MEAL)
            .any(|turn| is_a_meal(&turn.user));
        let bare = if meal {
            number_in(text, &[&BARE_PEOPLE])
        } else {
            None
        };
        let changed = group.filter(|_| {
            self.table.is_some() && matches(&CHANGED, text) && !matches(&ABOUT_THE_HOUSE, text)
        });
        if let Some(table) = explicitly.or(adjusted).or(bare).or(changed) {
            self.table = Some(table);
        }
        self.booked |= matches(&A_TABLE_BOOKED, text);
    }

    fn table_reply(&self, number: u16, low: &str, turns: &[Exchange], of_group: bool) -> String {
        let many = many(number);
        let noun = if of_group {
            "of you"
        } else if low.contains("guest") {
            if number == 1 { "guest" } else { "guests" }
        } else if low.contains("adult") {
            if number == 1 { "adult" } else { "adults" }
        } else if self.booked
            && ["book", "table", "reserv"]
                .iter()
                .any(|word| low.contains(word))
        {
            return format!("The table is for {many}.");
        } else if number == 1 {
            "person"
        } else {
            "people"
        };
        let first = format!("You said {many} {noun}.");
        if turns.iter().any(|turn| turn.bot == first) {
            format!("Still {many} {noun}.")
        } else {
            first
        }
    }

    pub(super) fn answer(&self, text: &str, turns: &[Exchange]) -> Option<String> {
        let low = flatten(text);
        if matches(&NOT_THE_COUNT, &low) {
            return None;
        }
        let of_you = matches(&GROUP_ASK_YOU, &low);
        if matches(&GROUP_ASK, &low) || matches(&BARE_GROUP_ASK, &low) || of_you {
            if let Some(number) = self.household {
                return Some(group_reply(number));
            }
            if of_you {
                return None;
            }
            if let Some(number) = self.table {
                return Some(self.table_reply(number, &low, turns, low.contains("of us")));
            }
        }
        if matches(&PARTY_ASK, &low) {
            if let Some(table) = self.table
                && turns
                    .iter()
                    .rev()
                    .take(LOOK_BACK_FOR_A_MEAL)
                    .any(|turn| sets_a_table(&turn.user))
            {
                return Some(self.table_reply(table, &low, turns, false));
            }
            return self
                .household
                .filter(|_| matches(&POINTS_AT_A_NUMBER, &low))
                .map(|number| self.table_reply(number, &low, turns, true));
        }
        if matches(&BARE_COUNT_ASK, &low) {
            return self
                .table
                .map(|number| self.table_reply(number, &low, turns, false))
                .or_else(|| {
                    self.household
                        .map(|number| self.table_reply(number, &low, turns, true))
                });
        }
        None
    }
}
