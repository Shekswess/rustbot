//! Ground transfers, cabin screening, and UK customs questions.

use regex::{Regex, RegexSet};
use std::{borrow::Cow, sync::LazyLock};

use crate::bot::{Exchange, Reading, understanding::kinds::CareShape};

use super::city_guides::named_place;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(WAGE, super::TRAVEL_DUTY_WAGE);
pattern!(GOODS, super::TRAVEL_DUTY_GOODS);
pattern!(CUSTOMS, super::TRAVEL_DUTY_CUSTOMS);
pattern!(ARTICLE, super::TRAVEL_BORDER_ARTICLE);
pattern!(PACK, r"\b(?:pack|packs|packing|packed)\b");
pattern!(
    LIQUID_AMOUNT,
    r"\bhow (?:much|many)\b.{0,20}\b(?:liquid|liquids|water|drink|drinks|ml)\b"
);
pattern!(
    BAG_SIZE,
    r"\bhow (?:much|many|big|large|heavy)\b.{0,30}\b(?:luggage|baggage|bags?|cabin bag|kilos?|kg|weight)\b"
);
pattern!(
    TRAY,
    r"\b(?:take (?:it |them |my \w+ )?out|out at security|out of my bag|out of the bag)\b"
);
pattern!(
    HOLD,
    r"\b(?:suitcase|hold|checked bag|checked in luggage|checked luggage|holdall)\b"
);
pattern!(
    BAG,
    r"\b(?:cabin bags?|hand luggage|hand baggage|carry ?ons?|luggage|baggage|backpack|suitcase|bags?)\b"
);
pattern!(
    BANNED,
    r"\b(?:not allow|banned|prohibit|forbidden|cannot take|can.t take|not take|what can.t i|what can i not|what am i not)"
);
pattern!(
    MEDICINE,
    r"\b(?:medicines?|tablets?|baby (?:milk|food)|breast milk|infant formula)\b"
);

static CROSSING: LazyLock<fancy_regex::Regex> =
    LazyLock::new(|| fancy_regex::Regex::new(super::TRAVEL_BORDER_WORD).expect("crossing verbs"));
static DIRECTION: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(super::TRAVEL_BORDER_DIRECTION).expect("border directions")
});
static DESTINATION: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(super::TRAVEL_BORDER_DESTINATION).expect("border destinations")
});
static CABIN_ITEMS: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(super::TRAVEL_CABIN_ITEMS).expect("screening item patterns"));

fn contains(line: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| line.contains(phrase))
}

fn airport_name(line: &str) -> Option<&'static str> {
    super::TRAVEL_AIRPORT_NAMES
        .iter()
        .copied()
        .find(|name| named_place(line, name))
}

fn first_airport(clean: &str, earlier: &[Exchange]) -> Option<&'static str> {
    airport_name(clean)
        .or_else(|| {
            if !clean.contains("airport") {
                return None;
            }
            super::TRAVEL_AIRPORT_NAMES
                .iter()
                .copied()
                .find(|name| named_place(clean, name.strip_suffix(" airport").unwrap_or(name)))
        })
        .or_else(|| {
            earlier
                .iter()
                .rev()
                .take(2)
                .find_map(|turn| airport_name(&turn.user.to_lowercase()))
        })
}

// Transfer routes are from the pinned AIRPORT_TRANSFER table, with current verified
// route changes and a lookup for current fares and schedules.
// https://www.trenitalia.com/en/services/connections-to-and-from-rome-fiumicino-airport.html
// https://www.iledefrance-mobilites.fr/en/acceder-aux-aeroports
// https://www.jfkairport.com/transportation/airtrain
fn airport_route(name: &str) -> &'static str {
    match name {
        "fiumicino" | "rome" => {
            "From Fiumicino, the Leonardo Express goes to Roma Termini. Regional trains serve stations such as Trastevere."
        }
        "lisbon airport" | "lisbon" => {
            "The Red Line metro links Lisbon Airport to the city network."
        }
        "charles de gaulle" => "RER B links Charles de Gaulle Airport to central Paris.",
        "paris" => {
            "From Charles de Gaulle, use RER B. From Orly, check Metro line 14 or the Orlyval connection to RER B."
        }
        "barcelona airport" => {
            "Compare the Aerobus to Plaça de Catalunya and the airport train to Sants. Check which terminal each serves."
        }
        "amsterdam airport" => "There are direct trains from Schiphol to Amsterdam Centraal.",
        "dublin airport" => {
            "Check airport coach services to central Dublin and choose the stop nearest your destination."
        }
        "edinburgh airport" => "Compare the tram and Airlink bus to central Edinburgh.",
        "jfk" => {
            "Take the AirTrain to Jamaica for the subway or Long Island Rail Road, or to Howard Beach for the subway."
        }
        "berlin airport" => {
            "Check the Airport Express and S-Bahn routes for trains between BER and the city."
        }
        "porto airport" => "Metro line E connects Porto Airport to the city network.",
        "dubrovnik airport" => {
            "Check the airport shuttle stops for the old town and main bus station."
        }
        "krakow airport" => "Check the airport train to Kraków Główny.",
        "budapest airport" => "Check route 100E to Deák Ferenc tér.",
        "venice airport" => {
            "From Marco Polo, compare buses to Piazzale Roma and Alilaguna boats to your part of the city."
        }
        "copenhagen airport" => {
            "Compare the metro to Kongens Nytorv and the train to Copenhagen Central."
        }
        "athens airport" => "Compare Metro line 3 toward Syntagma with the X95 bus.",
        "seville airport" => "Check the EA airport bus for the stop nearest your destination.",
        "florence airport" => "Check tram line T2 toward the city centre.",
        "naples airport" => "Check Alibus stops for the central railway station and port.",
        "madrid airport" => "Check Metro line 8 and Cercanías rail, which serves Terminal 4.",
        "prague airport" => {
            "Compare the Airport Express to the main station with the public transport connection to the metro."
        }
        "istanbul airport" => {
            "Check the metro and Havaist buses from your terminal, and compare the connections to where you are staying."
        }
        "vienna airport" => "Compare S-Bahn and City Airport Train services to Wien Mitte.",
        "zurich airport" => "Check trains to Zürich Hauptbahnhof.",
        "munich airport" => "Check S-Bahn lines S1 and S8.",
        "frankfurt airport" => "Check regional rail and S-Bahn services to the main station.",
        "stockholm airport" => {
            "From Arlanda, compare the express with commuter rail and airport coaches."
        }
        "oslo airport" => "Compare Flytoget and regional trains to Oslo Central.",
        "helsinki airport" => "Check trains I and P to Helsinki Central.",
        "tokyo airport" => {
            "From Haneda, compare the monorail and Keikyu Line. From Narita, check the Skyliner and other airport rail options."
        }
        "kyoto" => "From Kansai Airport, check the Haruka train to Kyoto.",
        "brussels airport" => {
            "Check trains from Brussels Airport to the city. If you land at Charleroi, check the direct coach."
        }
        "bangkok airport" => {
            "From Suvarnabhumi, check the Airport Rail Link and where you need to change for the local network."
        }
        "singapore airport" => {
            "Check the MRT route from Changi and any interchange for your destination."
        }
        "hanoi airport" => "Check the airport bus or an official taxi pickup for the Old Quarter.",
        "cape town airport" => {
            "Check the airport's designated taxi and ride-hailing pickup points before you land."
        }
        "mexico city airport" => {
            "Compare the official taxi service and public transport from your terminal. Check whether you are arriving at AICM or Felipe Ángeles."
        }
        "havana airport" => {
            "Check the airport's official taxi or transfer arrangements and confirm the currency and fare before travel."
        }
        "buenos aires airport" => {
            "Confirm whether you land at Ezeiza or Aeroparque. Check official taxis and bus routes from your airport."
        }
        "rio airport" => {
            "Confirm whether you land at Galeão or Santos Dumont. Check the airport's public transport and official taxi options."
        }
        "sydney airport" => "Check the Airport Link train to Central.",
        "toronto airport" => "From Pearson, check the UP Express to Union Station.",
        "chicago airport" => "From O'Hare, check the CTA Blue Line to central Chicago.",
        "boston airport" => {
            "Compare the Silver Line SL1 and other airport links to your part of Boston."
        }
        "san francisco airport" => "Check BART for a connection to your part of San Francisco.",
        "los angeles airport" => {
            "Check the FlyAway bus to Union Station and the official pickup locations if you prefer a taxi or rideshare."
        }
        _ => {
            "Check the transport page for your airport and compare the train, bus and licensed taxi options to your destination."
        }
    }
}

fn from_airport(
    reading: &Reading<'_>,
    earlier: &[Exchange],
    taxi: bool,
) -> Option<Cow<'static, str>> {
    let line = reading.line.words.join(" ");
    let clean = &reading.line.clean;
    const TAXI: &str = "Use the official taxi rank or a licensed transfer provider. Check the published fare, likely extras and payment method before you get in. Confirm a fixed price where available or check that the meter is running.";
    if taxi && contains(clean, super::TRAVEL_TAXI_TROUBLE) {
        return Some(Cow::Borrowed(TAXI));
    }
    if let Some(name) = first_airport(clean, earlier) {
        let route = airport_route(name);
        return Some(if taxi {
            Cow::Owned(format!(
                "{TAXI} {route} Check the airport or operator site for current fares and service times."
            ))
        } else {
            Cow::Owned(format!(
                "{route} Check the airport or operator site for current fares and service times."
            ))
        });
    }
    if taxi {
        return Some(Cow::Borrowed(TAXI));
    }
    if contains(&line, &["to the airport", "to the terminal"]) && !line.contains("from the airport")
    {
        return None;
    }
    Some(Cow::Borrowed(
        if contains(&line, super::TRAVEL_AIRPORT_PORT) {
            "Confirm the cruise terminal first. Compare the cruise line's transfer with a licensed taxi or public transport. Check the operator's delay policy and leave time for traffic and the ship's final boarding deadline."
        } else if contains(&line, super::TRAVEL_AIRPORT_HOTEL) {
            "Check the hotel's directions for the nearest stop and any shuttle. Compare a train or bus followed by a short walk with a taxi from the official airport rank."
        } else if contains(&line, super::TRAVEL_AIRPORT_CHEAPEST) {
            "Compare ordinary trains, metro and local buses on the airport's transport page. An express service or taxi may cost more. Include any onward fare in the comparison."
        } else if contains(&line, super::TRAVEL_AIRPORT_TRAIN) {
            "Check the airport's transport page for trains or metro, then check the station, fare and last departure for your arrival date. If the airport has no rail station, check the shuttle to the nearest one."
        } else if contains(&line, super::TRAVEL_AIRPORT_FASTEST) {
            "Compare the train or metro with a taxi for your actual destination and arrival time. Check for planned transport changes and allow time for luggage and the trip from the final stop."
        } else {
            "Use the airport's transport page to compare trains, buses and licensed taxis for your arrival time. Check the stop closest to where you are staying. If you take a taxi, use the official rank."
        },
    ))
}

fn to_airport(reading: &Reading<'_>) -> &'static str {
    let line = reading.line.words.join(" ");
    if contains(&line, super::TRAVEL_AIRPORT_EARLY) {
        "For an early flight, check the first train or bus for your departure day. If it arrives too late, arrange a licensed taxi, a lift or an airport hotel. Work back from the airport's arrival guidance and your airline's check-in, bag-drop and gate deadlines."
    } else {
        "Check the airport's transport page for trains, buses and the designated drop-off point. Plan a route using the airport's arrival guidance and your airline's check-in, bag-drop and gate deadlines. Check any drop-off charge ahead of time."
    }
}

fn not_wage(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    !WAGE.is_match(clean) || GOODS.is_match(clean) || CUSTOMS.is_match(clean)
}

pub(super) fn duty_shape(reading: &Reading<'_>) -> bool {
    not_wage(reading)
        && !reading
            .context_fields
            .care_home
            .as_ref()
            .is_some_and(|home| home.shape == CareShape::Bring)
}

fn place_after<'a>(regex: &fancy_regex::Regex, clean: &'a str) -> Option<&'a str> {
    regex
        .captures_iter(clean)
        .filter_map(Result::ok)
        .find_map(|matched| {
            let place = matched
                .get(1)?
                .as_str()
                .trim()
                .trim_matches(['.', ',', '!', '?', ';', ':']);
            let place = place.trim_start_matches(|c: char| c.is_ascii_whitespace());
            let mut bare = place;
            while let Some(article) = ARTICLE.find(bare) {
                bare = &bare[article.end()..];
            }
            (!bare.is_empty() && !super::TRAVEL_BORDER_VERBS.contains(&bare)).then_some(bare)
        })
}

fn everyday(place: &str) -> bool {
    super::TRAVEL_BORDER_EVERYDAY.iter().any(|name| {
        place == *name
            || place
                .strip_prefix(name)
                .is_some_and(|rest| rest.starts_with(' '))
            || place
                .strip_suffix(name)
                .is_some_and(|rest| rest.ends_with(' '))
    })
}

pub(super) fn food_border(reading: &Reading<'_>) -> bool {
    let line = &reading.line;
    let clean = if line.clean.is_empty() {
        &line.text
    } else {
        &line.clean
    };
    let clean = clean.to_lowercase();
    let crossing = CROSSING.is_match(&clean).unwrap_or(false)
        || place_after(&DIRECTION, &clean).is_some_and(|place| !everyday(place));
    crossing
        && place_after(&DESTINATION, &clean)
            .is_none_or(|place| everyday(place) || super::TRAVEL_BORDER_BRITAIN.contains(&place))
}

// https://www.gov.uk/bringing-goods-into-uk-personal-use
// https://www.gov.uk/bringing-goods-into-uk-personal-use/arriving-in-Great-Britain
fn duty(clean: &str) -> &'static str {
    if clean.contains("northern ireland") {
        "For Northern Ireland, check GOV.UK's personal allowances for your departure country. The rules differ depending on where you travel from. Check the allowance before shopping and declare goods that exceed it."
    } else {
        "For eligible travellers aged 17 or over arriving in Great Britain, the personal allowance includes up to 42 litres of beer and cider and 18 litres of wine. The separate allowance is 4 litres of spirits over 22% ABV or 9 litres of other drinks up to 22% ABV, or a proportional mix. The tobacco allowance includes an option of 200 cigarettes. Other personal goods generally have a £390 allowance, reduced to £270 on a private plane or boat. Check GOV.UK for your route, declare goods over the allowance, and check the separate Northern Ireland rules."
    }
}

// https://www.gov.uk/bringing-food-into-great-britain/meat-dairy-fish-animal-products
fn food_customs(clean: &str) -> &'static str {
    if clean.contains("northern ireland") {
        "Check the current Northern Ireland food import rules for your route with DAERA or GOV.UK before packing. The Great Britain food rules apply to arrivals in England, Scotland and Wales."
    } else {
        "Check GOV.UK's current rules for bringing food into Great Britain before you pack. Many meats such as pork and beef, and dairy such as cheese and butter, are restricted even in personal luggage. Rules depend on the origin and what the food contains. Fish, poultry and some other foods have separate rules. Ask customs if you are unsure."
    }
}

pub(super) fn cabin_shape(reading: &Reading<'_>) -> bool {
    let clean = &reading.line.clean;
    !PACK.is_match(clean) || CABIN_ITEMS.is_match(clean) || MEDICINE.is_match(clean)
}

// https://www.gov.uk/hand-luggage-restrictions/personal-items
// https://www.iata.org/contentassets/90f8038b0eea42069554b2f4530f49ea/guidance-to-operators---power-banks.pdf
fn cabin(clean: &str) -> Option<&'static str> {
    if LIQUID_AMOUNT.is_match(clean) {
        return Some(super::TRAVEL_FLIGHT_LIQUIDS);
    }
    if BAG_SIZE.is_match(clean) || TRAY.is_match(clean) {
        return None;
    }
    if MEDICINE.is_match(clean) {
        return Some(
            "Check the airport's rules for medicines, baby food and milk, and any documentation you need. Keep essentials accessible for security screening and confirm what your airline allows.",
        );
    }
    let item = CABIN_ITEMS.matches(clean).iter().next();
    Some(match item {
        Some(0) => {
            "Carry power banks, spare lithium batteries and vapes in hand luggage. Remove them from any bag that goes into the hold at the gate. Protect spare batteries against short circuits and check your airline's capacity, quantity and use rules. Vape liquids follow the airport's liquid rules."
        }
        Some(1) => {
            "Check your airline's device and battery rules and follow the airport's instructions for taking electronics out at screening. Some devices can travel in checked baggage if they are fully switched off and protected from damage."
        }
        Some(2) => {
            "Check the airport's current liquid rules for alcohol in hand luggage and the airline's restrictions on alcohol in checked baggage. If you buy duty-free liquids, keep the sealed security bag and receipt for connections. Check customs allowances at your destination."
        }
        Some(3) => {
            "UK airports usually allow disposable and fixed-cartridge razors in hand luggage. Check the departure and connection airports for restrictions on loose blades and other razor types."
        }
        Some(4) => {
            "UK airports usually allow small scissors with blades up to 6 cm in hand luggage. Longer blades go in checked luggage if your airline allows them. Check rules at any transfer airport."
        }
        Some(5) => {
            "UK airports usually allow tweezers and nail clippers in hand luggage. Check screening rules at any connecting or return airport."
        }
        Some(6..=8) => {
            "Check your airport's restrictions before travelling with a knife, corkscrew or tool. Many sharp items and tools have to go in checked baggage, subject to your airline's rules."
        }
        Some(9) => {
            "Check the airport and airline rules for the exact lighter or matches you have. UK rules usually allow one lighter carried on your person in a resealable bag. Lighter fuel and non-safety matches are restricted."
        }
        Some(10 | 12 | 13) => super::TRAVEL_FLIGHT_LIQUIDS,
        Some(11) => super::TRAVEL_FLIGHT_FOOD,
        None if HOLD.is_match(clean) => {
            "Check your airline's checked-baggage rules. Leave power banks, spare lithium batteries and vapes in your hand luggage. Check any liquids, aerosols, tools and sharp items against the carrier's restricted-item list."
        }
        None if BANNED.is_match(clean) => {
            "Check the airport and airline lists for the items you plan to pack. Sharp items and some tools are restricted in the cabin. Power banks, spare lithium batteries and vapes go in hand luggage. Liquids depend on the airport."
        }
        None if BAG.is_match(clean) => return None,
        _ => {
            "Check your airline and airport's restricted-items lists for what you want to carry. Security staff decide what can pass screening."
        }
    })
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    match id {
        "airport-to-city" => from_airport(reading, earlier, false),
        "taxi-abroad" => from_airport(reading, earlier, true),
        "to-the-airport" => Some(Cow::Borrowed(to_airport(reading))),
        "duty-free" => Some(Cow::Borrowed(duty(clean))),
        "customs-food" => Some(Cow::Borrowed(food_customs(clean))),
        "cabin-bag-rules" => cabin(clean).map(Cow::Borrowed),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_direction_and_current_operator_guidance() {
        let jfk = Reading::new("How do I get from JFK airport to the city centre?");
        assert!(
            from_airport(&jfk, &[], false)
                .unwrap()
                .contains("Howard Beach")
        );
        let outbound = Reading::new("How do I get to the airport for an early flight?");
        assert!(from_airport(&outbound, &[], false).is_none());
        assert!(to_airport(&outbound).contains("first train or bus"));
        let known = Reading::new("How do I avoid a taxi scam at Lisbon airport?");
        assert!(
            from_airport(&known, &[], true)
                .unwrap()
                .starts_with("Use the official taxi rank")
        );
    }

    #[test]
    fn customs_only_when_a_border_is_crossed_and_cabin_items_are_specific() {
        assert!(food_border(&Reading::new(
            "Can I bring cheese back from France?"
        )));
        assert!(!food_border(&Reading::new(
            "Can I bring cheese into Spain?"
        )));
        assert!(!food_border(&Reading::new(
            "Can I bring a sandwich to work?"
        )));
        assert!(!not_wage(&Reading::new("What does take home mean?")));
        assert!(
            cabin("can i put my power bank in my hold bag")
                .unwrap()
                .contains("hand luggage")
        );
        assert!(
            cabin("can i bring wine in my cabin bag")
                .unwrap()
                .contains("customs allowances")
        );
        assert!(!cabin_shape(&Reading::new(
            "How do I pack for a week in hand luggage?"
        )));
    }
}
