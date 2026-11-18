//! Native reading of calendar, queue, and ticket asks (bot/modules/live_source.py).

use crate::bot::{Memory, Reading, knowledge::airport_arrival};
use crate::text::is_space;
use regex::Regex;
use std::sync::LazyLock;

static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\p{L}\p{N}_]+").unwrap());
static LEAD_IN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^(?:ok|okay|oh|so|and|but|also|anyway|anyways|btw|right|well|hmm|um|er|",
        r"like|sorry|quick one|one more|then|now|hi|hey|yeah|yep|please)[ ,.!-]+"
    ))
    .unwrap()
});
static CALENDAR_PLAN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?:^|[^\p{L}\p{N}_])(?:what (?:can|should|could) (?:i|we|you) do|things to do|",
        r"something to do|ideas|activities|days out|day out|keep (?:the |them |my )?",
        r"[\p{L}\p{N}_]* ?busy|places to go)(?:$|[^\p{L}\p{N}_])"
    ))
    .unwrap()
});

const ASK_OPENERS: &[&str] = &[
    "when", "what", "which", "who", "whose", "where", "why", "how", "is", "are", "was", "were",
    "do", "does", "did", "can", "could", "will", "would", "should", "shall", "may", "might",
    "have", "has", "had", "any", "am",
];
const CALENDAR_OPENERS: &[&str] = &[
    "when", "what", "which", "is", "are", "do", "does", "will", "should", "how",
];
const DRAW_ASK_WORDS: &[&str] = &["did", "do", "does", "are", "is", "have", "has", "any"];
const DRAW_RESULT_WORDS: &[&str] = &[
    "win", "won", "winner", "winners", "winning", "result", "results", "numbers", "draw", "drawn",
    "check",
];

type ReplyPool = [&'static str; 2];

/// Direct questions about live source data to the organization that publishes it.
pub(crate) fn respond(reading: &Reading<'_>, _memory: &Memory) -> Option<String> {
    // This module normalizes case and whitespace while retaining the typed punctuation.
    let clean = reading.raw.to_lowercase();
    let clean = clean
        .split(is_space)
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let said = without_lead_in(&clean);
    let first_word = WORD
        .find(said)
        .filter(|word| word.start() == 0)
        .map(|word| word.as_str());
    let asks = first_word.is_some_and(|word| {
        ASK_OPENERS.contains(&word)
            || word
                .strip_suffix('s')
                .is_some_and(|base| ASK_OPENERS.contains(&base))
    });
    if clean.is_empty() || !(asks || reading.raw.contains('?')) {
        return None;
    }

    let replies = queue(&clean)
        .or_else(|| draw(&clean))
        .or_else(|| date(&clean, first_word))?;
    // The last bit of the sum chooses one of the two wordings.
    let wording = usize::from(clean.bytes().fold(0, u8::wrapping_add) & 1);
    Some(replies[wording].to_owned())
}

fn without_lead_in(mut said: &str) -> &str {
    while let Some(lead) = LEAD_IN.find(said) {
        let shorter = &said[lead.end()..];
        if shorter.is_empty() {
            break;
        }
        said = shorter;
    }
    said
}

fn has(text: &str, phrases: &[&str]) -> bool {
    let bytes = text.as_bytes();
    phrases.iter().any(|phrase| {
        text.match_indices(phrase).any(|(start, matched)| {
            let before = start.checked_sub(1).map(|previous| bytes[previous]);
            let after = bytes.get(start + matched.len());
            !before.is_some_and(|byte| byte.is_ascii_lowercase())
                && !after.is_some_and(u8::is_ascii_lowercase)
        })
    })
}

fn queue(clean: &str) -> Option<ReplyPool> {
    if !has(clean, WAIT) {
        return None;
    }
    if has(clean, A_E) {
        Some(A_E_LINES)
    } else if has(clean, AIRPORT) && airport_arrival::is_live_queue(clean) {
        Some(AIRPORT_LINES)
    } else {
        None
    }
}

fn draw(clean: &str) -> Option<ReplyPool> {
    let monthly = has(clean, MONTHLY_DRAW);
    let national = has(clean, NATIONAL_DRAW);
    let local = has(clean, LOCAL_DRAW);
    if !(monthly || national || local || has(clean, TICKET)) || !asks_for_draw_result(clean) {
        return None;
    }
    if monthly {
        Some(MONTHLY_LINES)
    } else if national {
        Some(NATIONAL_LINES)
    } else if local {
        Some(LOCAL_LINES)
    } else {
        Some(TICKET_LINES)
    }
}

fn asks_for_draw_result(clean: &str) -> bool {
    let mut words = WORD.find_iter(clean).peekable();
    let mut ask_end = None;
    while let Some(word) = words.next() {
        if DRAW_ASK_WORDS.contains(&word.as_str()) {
            ask_end = Some(word.end());
        } else if let Some(end) = ask_end {
            let gap = &clean[end..word.start()];
            if gap.chars().take(41).count() > 40 || gap.contains('?') {
                ask_end = None;
                continue;
            }
            if DRAW_RESULT_WORDS.contains(&word.as_str())
                || (matches!(word.as_str(), "come" | "came")
                    && words.peek().is_some_and(|next| {
                        next.as_str() == "up" && &clean[word.end()..next.start()] == " "
                    }))
            {
                return true;
            }
        }
    }
    false
}

fn date(clean: &str, first_word: Option<&str>) -> Option<ReplyPool> {
    if !first_word.is_some_and(|word| CALENDAR_OPENERS.contains(&word))
        || CALENDAR_PLAN.is_match(clean)
        || (has(clean, CALENDAR_NOT) && !has(clean, CALENDAR_SCHOOL))
    {
        return None;
    }
    if has(clean, TERM) {
        Some(TERM_LINES)
    } else if has(clean, BANK_HOLIDAY) {
        Some(BANK_LINES)
    } else {
        None
    }
}

// Phrases and reply pools from bot/modules/live_source.py at the pinned revision.
const BANK_HOLIDAY: &[&str] = &["bank holiday", "bank holidays", "public holiday"];
const TERM: &[&str] = &[
    "school holidays",
    "school holiday",
    "term dates",
    "term date",
    "school term",
    "half term",
    "half-term",
    "inset day",
    "when do the schools",
    "when do schools",
    "when does term",
    "when do the kids break up",
    "when do they break up",
    "when do we break up",
    "school go back",
    "schools go back",
    "go back to school",
    "school start",
    "schools start",
    "school finish",
    "schools finish",
    "break up for",
    "summer holidays start",
    "summer holiday start",
    "school break up",
    "schools off",
    "schools are off",
    "school off",
    "is school off",
    "schools closed",
    "are schools closed",
];
const CALENDAR_NOT: &[&str] = &[
    "ticketed",
    "ticket",
    "fine",
    "fine'd",
    "parking",
    "booked off",
    "my shift",
    "rota",
    "overtime",
    "day rate",
    "post",
    "postman",
    "deliveries",
    "delivery",
    "buses",
    "bus",
    "trains",
    "train",
    "timetable",
    "shops",
    "shop",
    "supermarket",
    "open",
    "closed",
    "close",
    "runs",
    "run on",
    "running",
    "collections",
    "collection",
    "bin",
    "bins",
    "rubbish",
    "tips",
    "the tip",
    "banks",
    "the bank",
    "pharmacy",
    "dentist",
    "doctor",
    "swimming",
    "pool",
];
const CALENDAR_SCHOOL: &[&str] = &[
    "school",
    "schools",
    "term",
    "half term",
    "half-term",
    "inset day",
    "term time",
    "break up",
    "go back",
];
const NATIONAL_DRAW: &[&str] = &[
    "lottery",
    "lotto",
    "euromillions",
    "euro millions",
    "thunderball",
    "scratchcard",
    "scratch card",
    "jackpot",
    "numbers come up",
    "my numbers",
    "the balls",
    "lotto numbers",
];
const TICKET: &[&str] = &[
    "ticket win",
    "tickets win",
    "ticket won",
    "tickets won",
    "ticket come up",
    "tickets come up",
    "ticket a winner",
    "tickets a winner",
    "my ticket",
    "our ticket",
];
const LOCAL_DRAW: &[&str] = &[
    "raffle",
    "sweepstake",
    "sweepstakes",
    "tombola",
    "bingo",
    "the draw",
    "prize draw",
    "draw ticket",
    "draw tickets",
    "the raffle",
    "the tombola",
    "meat raffle",
    "bonus ball",
];
const MONTHLY_DRAW: &[&str] = &[
    "postcode lottery",
    "premium bonds",
    "premium bond",
    "premium bond draw",
];
const WAIT: &[&str] = &[
    "queue",
    "queues",
    "wait",
    "waiting",
    "busy",
    "backed up",
    "how long",
];
const AIRPORT: &[&str] = &[
    "airport",
    "terminal",
    "security",
    "passport control",
    "check in",
    "check-in",
    "bag drop",
    "departures hall",
];
const A_E: &[&str] = &[
    "a and e",
    "a&e",
    "casualty",
    "accident and emergency",
    "emergency department",
    "minor injuries",
    "urgent care",
    "walk in centre",
    "walk-in centre",
];
const BANK_LINES: ReplyPool = [
    "In England and Wales the bank holidays are 1 January, Good Friday, Easter Monday, the first and last Mondays in May, the last Monday in August, then 25 and 26 December - so only the Easter ones and this year's May and August Mondays need looking up. Scotland adds 2 January and 30 November and has no Easter Monday, and Northern Ireland adds 17 March and 12 July; gov.uk's bank-holidays page has the exact date for each nation.",
    "The bank holidays that move are the Easter ones and the first and last Mondays in May and August; the rest are 1 January, 25 December and 26 December - in England and Wales. Scotland and Northern Ireland differ (2 January and St Andrew's Day, St Patrick's Day and the Twelfth), so I can't give one list for the UK: gov.uk's bank-holidays page has the date, nation by nation.",
];
const TERM_LINES: ReplyPool = [
    "English state schools run three terms - September to December, January to Easter, and Easter to July - with a week off in October, February and May and about six weeks in summer. The exact weeks are set locally, so the council's school-term-dates page or the school's own site has this year's.",
    "A school year in England is three terms with a week's half term in October, February and May, two weeks at Christmas and about six weeks in summer, with the summer break starting in the second half of July. Which week is which is the council's or the school's to publish, because it differs by area - and Scotland and private schools run to their own calendar.",
];
const AIRPORT_LINES: ReplyPool = [airport_arrival::LIVE_QUEUE; 2];
const A_E_LINES: ReplyPool = [
    "For anything that is not life-threatening, an urgent-care centre or a minor injuries unit usually sees you faster than A&E, and NHS 111 will say which one near you has room tonight as well as the A&E waits, which the hospital publishes itself.",
    "A&E is for the things that cannot wait, and everything else moves quicker at urgent care or a minor injuries unit. Ring 111 - it knows which of them near you is quietest tonight and what the A&E wait is - because that changes by the hour and I can't see it.",
];
const NATIONAL_LINES: ReplyPool = [
    "Lotto draws on Wednesday and Saturday evenings and EuroMillions on Tuesday and Friday, so a result that is not up yet usually is not in yet. Your numbers are on the ticket: the app or the terminal in the shop that sold it checks a ticket in a second.",
    "The draws are in the evening - Lotto on Wednesday and Saturday, EuroMillions on Tuesday and Friday - and the result goes up with the operator straight after. Checking is instant on the ticket's own app or at the shop terminal, because I can't see the draw from here.",
];
const LOCAL_LINES: ReplyPool = [
    "A raffle, a sweepstake or a tombola is the organiser's to confirm - they hold the numbers, and at a club the caller has them on the night or they go up on the board by the door. I can't see a draw I was not in, so whoever sold you the ticket is the one to ask.",
    "There is no page to look a local draw up on: the organiser keeps the numbers, and a work or club sweepstake is called out on the night or pinned up afterwards. The seller or the organiser is the person who can check a ticket, not me.",
];
const TICKET_LINES: ReplyPool = [
    "Whether a ticket won depends on what it is for: a Lotto or EuroMillions ticket checks itself in the Lotto app or at the shop terminal, and a raffle or sweepstake ticket is the organiser's to confirm. The numbers are on the ticket either way, and I can't see the draw.",
    "It depends on the kind of ticket - the national draws check in the Lotto app or at a shop terminal, and a raffle or a sweepstake is the organiser's to call. Your numbers are printed on it, which is the only part I could not see for you.",
];
const MONTHLY_LINES: ReplyPool = [
    "Premium Bonds are drawn at the start of each month and the Postcode Lottery through the month, and both check your numbers for you: NS&I's own site or app for the Bonds, the Postcode Lottery's site with your postcode. I can't see either draw from here.",
    "The monthly draws are the operator's - NS&I publishes each Premium Bonds prize and its holder on its own site, and the Postcode Lottery's site takes your postcode and shows whether it has come up. That is the only place a result is real.",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn reply(text: &str) -> Option<String> {
        respond(&Reading::new(text), &Memory::default())
    }

    #[test]
    fn calendar_responds_to_dates_and_ignores_plans_and_service_hours() {
        let bank_holiday = reply("Okay, so is Monday a bank holiday?").unwrap();
        let half_term = reply("Is half term over the bank holiday?").unwrap();
        assert!(BANK_LINES.contains(&bank_holiday.as_str()));
        assert!(TERM_LINES.contains(&half_term.as_str()));
        assert_eq!(reply("Does the post run on a bank holiday?"), None);
        assert_eq!(reply("What can I do with the kids in half term?"), None);
    }

    #[test]
    fn draw_results_require_an_ask_in_the_same_question() {
        let ticket = reply("Did my ticket win?").unwrap();
        let monthly = reply("Have my Premium Bonds numbers come up?").unwrap();
        assert!(TICKET_LINES.contains(&ticket.as_str()));
        assert!(MONTHLY_LINES.contains(&monthly.as_str()));
        assert_eq!(reply("Did you see my ticket? I won"), None);
        assert_eq!(reply("I won the lottery"), None);
    }

    #[test]
    fn live_airport_queues_refer_to_updated_sources() {
        let plain = reply("Whats the wait like at the airport?").unwrap();
        let canonical = reply(" \tWHATS the  wait like at the airport?\u{1e}").unwrap();
        assert_eq!(plain, canonical);
        assert_eq!(plain, airport_arrival::LIVE_QUEUE);
        assert_eq!(reply("The wait at the airport was rough"), None);
        assert_eq!(
            reply("How early should I get to the airport to avoid the queue?"),
            None
        );
        assert_eq!(
            reply("How long is the flight from the airport to Rome?"),
            None
        );
    }
}
