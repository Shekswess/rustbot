//! Native reading of live departure and status asks (bot/modules/live_times.py).
use crate::bot::{Memory, Reading, knowledge::airport_arrival};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! pattern {
    ($name:ident, $pattern:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pattern).unwrap());
    };
}

pattern!(
    VEHICLE,
    r"\b(?:bus|buses|train|trains|tube|tram|trams|coach|coaches|ferry|ferries|flight|flights)\b"
);
pattern!(
    TIME,
    r"\b(?:when|what time|how soon|next|on time|delayed|cancelled|canceled|running|due|leaves?|departs?|comes?|arrives?|platform|replacement|engineering works?)\b"
);
pattern!(
    STATUS,
    r"\b(?:on time|delayed|cancelled|canceled|running|due|replacement|engineering works?)\b"
);
pattern!(
    PLATFORM,
    r"\b(?:platform|station|stop|interchange|departure board)\b"
);
pattern!(
    PLACE_OPENING,
    r"\b(?:train|bus|coach|tram|tube) (?:station|stop)\b[^?.]{0,25}\b(?:open|opens|opening|close|closes|closed|shut|shuts|hours)\b"
);
pattern!(
    MONEY,
    r"\b(?:worth|cost|costs|price|prices|fare|fares|cheap|cheaper|expensive|refund|refunds|compensation|money|claim|claims|insurance|voucher|rebook|rebooking|delay repay|pass|rights?|entitled|entitlement|owed)\b"
);
pattern!(
    ADVICE,
    r"\b(?:what if|should (?:i|we)|what time should|how (?:do|can|should) (?:i|we)|how early|how long before|what happens if|do (?:i|we) need)\b"
);
pattern!(
    DELAYED,
    r"\b(?:delayed|delays?|running|on time|disruption|disrupted)\b"
);
pattern!(
    BOARD_ASK,
    r"\b(?:next|last|first|when|what time|how soon|platform|departure board|station|stop|leaves?|departs?)\b"
);
pattern!(
    FLIGHT_LIVE,
    r"\b(?:next|last|first|running|on time|delayed|cancelled|canceled|platform|leaves?|departs?|replacement|engineering works?)\b"
);
pattern!(
    ASK,
    r"^(?:when|what|how|is|are|was|were|has|have|did|does|do|can|could|will|would|any|next|which)\b"
);
pattern!(
    WHEN_ASK,
    r"^when (?:is|are|was|were|does|do|did|will|would|should|can|could)\b"
);
pattern!(
    NEWS,
    r"\b(?:strike|strikes|striking|walkout|walk out|industrial action|union|ballot|announcement|news)\b"
);
pattern!(
    SERVICE,
    r"\b(?:(?P<clock>[0-9]{1,2}(?::[0-9]{2})? *(?:o'?clock)?) +)?(?:(?P<qual>next|night|last|first|early|late|morning|evening|midnight) +)?(?P<vehicle>bus|train|tube|tram|coach|ferry|flight)\b(?: +(?P<via>to|into|from) +(?P<dest>[a-z][a-z'-]*(?: +[a-z][a-z'-]*){0,4}))?"
);
pattern!(
    BREAK_DEST,
    r"\b(?:cancelled|canceled|delayed|running|tomorrow|today|tonight|please|leave|leaves|go|goes|departs?|arrives?)\b"
);
pattern!(
    DEST_TAIL,
    r"(?:\s*\b(?:of|the|and|a|an|to|from|in|on|at|for|by)\b)+$"
);
pattern!(
    CLOCK,
    r"\b\d{1,2}[:.]\d{2} *(?:am|pm)?\b|\b\d{1,2} *(?:am|pm|o'?clock)\b|\b(?:at|by|around|about|before|after|from|until|till) \d{1,2}\b|\b\d{1,2} (?:tomorrow|today|tonight)\b|\b(?:this )?(?:morning|afternoon|evening|noon|midnight)\b"
);
pattern!(
    NOT_OWN,
    r"\b(?:next|last|first|night|early|late|platform|station|stop|running|on time|delayed|cancelled|canceled|replacement)\b"
);
pattern!(FLIGHT_NAMES, r"\b(?:flight|flights|plane)\b");
pattern!(REPLACEMENT, r"\b(?:replacement|engineering works?)\b");
pattern!(NEXT, r"\bnext\b");
pattern!(WHAT_TIME, r"\bwhat time\b");

fn stem(vehicle: &str) -> &str {
    match vehicle {
        "buses" => "bus",
        "trains" => "train",
        "trams" => "tram",
        "coaches" => "coach",
        "ferries" => "ferry",
        "flights" => "flight",
        _ => vehicle,
    }
}

fn their_own_time(clean: &str, vehicle: &str, memory: &Memory) -> bool {
    !NOT_OWN.is_match(clean)
        && memory.turns.iter().rev().take(3).any(|turn| {
            let said = turn.user.to_lowercase();
            VEHICLE
                .find_iter(&said)
                .any(|m| stem(m.as_str()) == stem(vehicle))
                && CLOCK.is_match(&said)
        })
}

pub(crate) fn respond(reading: &Reading<'_>, memory: &Memory) -> Option<String> {
    let clean = &reading.clean;
    let vehicle = VEHICLE.find(clean)?.as_str();
    if !TIME.is_match(clean)
        || matches!(vehicle, "flight" | "flights")
            && airport_arrival::arrival_calculation(reading).is_some()
        || MONEY.is_match(clean)
        || NEWS.is_match(clean)
        || PLACE_OPENING.is_match(clean)
        || ADVICE.is_match(clean)
        || their_own_time(clean, vehicle, memory)
        || DELAYED.is_match(clean) && !BOARD_ASK.is_match(clean)
        || FLIGHT_NAMES.is_match(clean) && !FLIGHT_LIVE.is_match(clean)
        || !ASK.is_match(clean) && !reading.raw.trim().ends_with('?')
        || !(NEXT.is_match(clean)
            || STATUS.is_match(clean)
            || PLATFORM.is_match(clean)
            || WHAT_TIME.is_match(clean)
            || WHEN_ASK.is_match(clean))
    {
        return None;
    }
    if REPLACEMENT.is_match(clean) {
        return Some("A replacement is the operator's own announcement: their app or site says where it leaves from and when, and the station's board carries it on the day.".into());
    }

    let service = SERVICE.captures(clean);
    let mut lead = String::new();
    if let Some(m) = &service {
        lead.push_str("For the ");
        for group in ["clock", "qual"] {
            if let Some(name) = m.name(group) {
                lead.push_str(name.as_str());
                lead.push(' ');
            }
        }
        lead.push_str(&m["vehicle"]);
        if let Some(dest) = m.name("dest") {
            let dest = BREAK_DEST.split(dest.as_str()).next().unwrap_or("");
            let dest = DEST_TAIL.replace(dest.trim(), "");
            if !dest.trim().is_empty() {
                lead.push(' ');
                lead.push_str(&m["via"]);
                lead.push(' ');
                lead.push_str(dest.trim());
            }
        }
        lead.push_str(": ");
    }

    let vehicle = service.as_ref().map_or(stem(vehicle), |m| &m["vehicle"]);
    let alternate = memory.turns.len() % 2;
    match vehicle {
        "flight" => lead.push_str(FLIGHT[alternate]),
        "ferry" => lead.push_str(FERRY[alternate]),
        "train" | "tram" | "tube" | "coach" => {
            let board = match vehicle {
                "tram" => "the board at the stop",
                "tube" => "the board on the platform",
                "coach" => "the board at the coach station",
                _ => "the board at the station",
            };
            lead.push_str(
                &RAIL[alternate]
                    .replace("{vehicle}", vehicle)
                    .replace("{where}", board),
            );
        }
        _ => lead.push_str(BUS[alternate]),
    }
    Some(lead)
}

const BUS: [&str; 2] = [
    "I can't see live bus times from here. The operator's app, or the timetable at the stop, has the next one; where the stop has a code, a text to it brings the times back. A journey planner puts the whole trip together.",
    "The next one is live, so nothing here can see it. The bus operator's app, or the timetable printed at the stop, has the time - text the stop's code where it has one - and a journey planner joins the trip up.",
];
const RAIL: [&str; 2] = [
    "I can't see live {vehicle} times from here. The operator's app, or {where}, has the next one and any delay or cancellation; a journey planner puts the whole trip together.",
    "That one is a live departure, so the operator's app, or {where}, is where the next one and any delay or cancellation show up. A journey planner puts the whole trip together.",
];
const FLIGHT: [&str; 2] = [
    "I can't see live flights from here. The airline's app, or the airport's own departures board, has the gate and any delay; the booking reference brings it up.",
    "The gate and any delay are live, and I can't see them from here. The airline's app, or the airport's own departures board, has them, and the booking reference brings the booking up.",
];
const FERRY: [&str; 2] = [
    "I can't see live ferry times from here. The operator's app, or the port's own departure board, has the next sailing and any delay; a journey planner puts the whole crossing together.",
    "That sailing is a live one, and I can't see it from here. The ferry operator's app, or the board at the port, has the next one and any delay; a journey planner joins the crossing up.",
];
