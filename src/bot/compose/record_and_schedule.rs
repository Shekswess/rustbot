//! Refer record and timetable questions to the people who keep the information.

use std::sync::LazyLock;

use regex::Regex;

use crate::bot::{
    Memory,
    compose::choose_recent,
    modules::{a_day_somebody_else_keeps, record_due, the_kind_one_line_up},
};

static LIVE_DEPARTURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?:(?:when|what time) (?:is|are|does|do|will) (?:the |my |our )?)?",
        r"(?:next (?:bus|buses|train|trains)|(?:bus|train) times)\b",
    ))
    .expect("live departure question")
});

// The travel pool in PyBot's live source reader.
const LIVE_TRANSIT: &str = "I can't check live times or delays, sorry. The operator's own app or site has the next departure and any disruption, and a journey planner puts the whole trip together.";

pub(super) fn reply(line: &str, memory: &Memory) -> Option<String> {
    if LIVE_DEPARTURE.is_match(line) {
        return Some(LIVE_TRANSIT.to_owned());
    }
    if record_due::asks_about_delivery(line)
        || a_day_somebody_else_keeps::asks_about_personal_schedule(line)
    {
        // These pools are the same ones used for a nearby reference to their
        // account or timetable. The specialized candidates have the personal details.
        let pool = the_kind_one_line_up::place_pool(line)?;
        return Some(choose_recent(pool, memory, line).to_owned());
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::bot::{Bot, Exchange, Memory, Reading, modules::greeting};

    #[test]
    fn record_and_schedule_keeps_personal_sources_and_live_transit() {
        let mut bot = Bot::new(Some("0")).unwrap();
        let parcel = bot.turn("Will my parcel come before Friday?");
        assert_eq!(parcel.module, Some("record_due"));
        assert!(parcel.reply.starts_with("The tracking link"));
        assert_eq!(
            parcel
                .candidates
                .iter()
                .find(|offer| offer.module == "greeting")
                .unwrap()
                .reply,
            "Your own account has that, sorry - it is the only place with your numbers in it. What it says is worth reading against the last one, and I'll help you spot what moved.",
        );
        let school = bot.turn("What time does my son finish school?");
        assert_eq!(school.module, Some("a_day_somebody_else_keeps"));
        assert!(school.reply.contains("The school office"));
        assert_eq!(
            school
                .candidates
                .iter()
                .find(|offer| offer.module == "greeting")
                .unwrap()
                .reply,
            "That is live, sorry - timetables and listings change by the hour. The operator's or the venue's own app has the current one, today's included, and I'll help you plan around whatever it says.",
        );

        let memory = Memory {
            turns: (0..5)
                .map(|_| Exchange {
                    user: "a different question".to_owned(),
                    bot: "A different subject.".to_owned(),
                    module: "greeting",
                })
                .collect(),
            ..Memory::default()
        };
        assert_eq!(
            greeting::respond(&Reading::new("what time does my class finish?"), &memory).as_deref(),
            Some(
                "I can't check live times, sorry. The operator's app (or the board at the stop) is the one that knows, and tell me the time and I'll work out the rest of the day with you."
            ),
        );
        let bus = bot.turn("When is the next bus into town?");
        assert_eq!(bus.module, Some("live_times"));
        assert_eq!(
            bus.candidates
                .iter()
                .find(|offer| offer.module == "greeting")
                .unwrap()
                .reply,
            super::LIVE_TRANSIT,
        );
        assert!(super::reply("what time does the sun rise?", &Memory::default()).is_none());
        assert!(super::reply("can i cancel my order?", &Memory::default()).is_none());
    }
}
