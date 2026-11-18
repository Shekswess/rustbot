//! Destination details, driving conditions, and geographic distances.

use std::{borrow::Cow, cmp::Reverse, sync::LazyLock};

use regex::Regex;

use crate::bot::{Exchange, Reading};

use super::city_guides::named_place;

#[path = "travel_coordinates.rs"]
mod coordinates;

macro_rules! pattern {
    ($name:ident, $source:expr) => {
        static $name: LazyLock<Regex> =
            LazyLock::new(|| Regex::new($source).expect(stringify!($name)));
    };
}

pattern!(US, super::TRAVEL_TIPPING_US);
pattern!(EUROPE, super::TRAVEL_TIPPING_EUROPE);
pattern!(THE_US, r"\bin (?:the )?us\b");
pattern!(TAXI, r"\b(?:taxi|cab|uber|driver|minicab)\b");
pattern!(JAPAN, r"\b(?:japan|japanese|tokyo|kyoto)\b");
pattern!(
    HOTEL,
    r"\b(?:hotel|housekeeping|porter|concierge|bellboy|maid|room service)\b"
);
pattern!(TOUR, r"\b(?:tour guide|tour|excursion|walking tour)\b");
pattern!(
    CASH_SAFETY,
    r"\b(?:safe|safely|safety|stolen|theft|pickpockets?|money belt|robbed|lose|lost|keep (?:my )?(?:money|cash|card))\b"
);
pattern!(
    HILL,
    r"\b(?:hike|hiking|hills?|mountains?|fell|munro|ridge|trek(?:king)?|coast path|rambl\w*|summit|peak|dales|moors?)\b"
);
pattern!(
    DAILY,
    r"\b(?:every day|everyday|daily|most days|each day)\b"
);
pattern!(QUANTITY, r"\b(?:how much|how many|how long)\b");
pattern!(
    KIT,
    r"\b(?:pack(?:ing|ed)?|bring|take|carry|wear|kit|bag|rucksack|backpack)\b"
);
pattern!(KIT_ASK, r"\b(?:what|which|anything|need|should)\b");
pattern!(
    FOOD,
    r"\b(?:eat|eating|ate|food|snacks?|lunch|fuel|energy|hungry|carbs?|carbohydrate)\b"
);
pattern!(WATER, r"\b(?:water|drinking?|liquid|litres?)\b");
pattern!(
    WATER_OTHER,
    r"\b(?:liquids|bottles?|hand luggage|cabin bag|security|plane|flight|fountain|how much|more water|enough water|glasses|dehydrat\w*|hot water|boil|watering|water (?:the|my|it|them)|bill|leak|pressure|heater|plants?|houseplants?|leaves|leaf)\b"
);
pattern!(
    WATER_ASK,
    r"\b(?:tap|safe|drink|drinking|ice|okay|ok|fine|stomach|ill)\b"
);
pattern!(
    WATER_HOME,
    r"\b(?:taps?|mains|bottled|safe|water company|water supply)\b"
);
pattern!(
    WATER_CHANGED,
    r"\b(?:brown|yellow|orange|rust(?:y)?|gritty|cloudy|smell\w*|taste\w*|chlorine|metallic|lead|discolou?red)\b"
);
pattern!(ABROAD, r"\b(?:abroad|holidays?|trip|travell?ing|travel)\b");

fn contains(clean: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| clean.contains(phrase))
}

fn title(line: &str) -> String {
    line.split_whitespace()
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

fn named<'a>(clean: &str, places: &'a [&str]) -> Option<&'a str> {
    places
        .iter()
        .copied()
        .find(|place| named_place(clean, place))
}

fn near_chat<T>(clean: &str, earlier: &[Exchange], find: impl Fn(&str) -> Option<T>) -> Option<T> {
    find(clean).or_else(|| {
        earlier
            .iter()
            .rev()
            .take(2)
            .find_map(|turn| find(&turn.user.to_lowercase()))
    })
}

// https://www.iec.ch/world-plugs
fn adapter(clean: &str, earlier: &[Exchange]) -> Option<String> {
    let (place, kind) = near_chat(clean, earlier, |line| {
        super::TRAVEL_PLUG_TYPES
            .iter()
            .copied()
            .find(|(name, _)| named_place(line, name))
    })?;
    let label = if place == "uae" {
        "UAE".to_owned()
    } else {
        title(place)
    };
    let pins = match (place, kind) {
        (_, "G") => format!("{label} commonly uses type G, the same three-pin plug as the UK."),
        ("china", _) => "China uses more than one socket type, including type I. Check your accommodation's sockets.".to_owned(),
        (_, "C and E") => format!("{label} commonly uses round-pin sockets. Check the exact type at your accommodation and take an adapter for a UK plug."),
        _ => format!("Common socket types in {label} include {kind}. Check which socket your accommodation has before choosing an adapter for a UK plug."),
    };
    Some(format!(
        "{pins} Check the voltage and frequency on the device label. A plug adapter changes the fit only. For equipment that requires an earth connection, use an adapter that maintains it."
    ))
}

// https://www.gov.uk/guidance/the-highway-code/general-rules-techniques-and-advice-for-all-drivers-and-riders-103-to-158
// https://www.gov.uk/guidance/the-highway-code/breakdowns-and-incidents-274-to-287
fn driving(reading: &Reading<'_>) -> &'static str {
    let said = format!("{} {}", reading.line.clean, reading.line.words.join(" "));
    let cue = super::TRAVEL_TRAFFIC_CUES
        .iter()
        .position(|words| contains(&said, words));
    match cue {
        Some(0) => {
            "If you aquaplane, ease off the accelerator and keep the steering steady. Avoid sudden braking or steering changes. Slow down and leave more room when the tyres regain grip."
        }
        Some(1) => {
            "Increase the space in front so you can slow down gradually. Keep a steady speed that suits the road and let the other driver pass when it is safe. Avoid sudden braking."
        }
        Some(2) => {
            "Use dipped headlights and slow enough to stop within the distance you can see. In the UK, use fog lights when visibility is seriously reduced, generally below 100 metres, and switch them off when it improves."
        }
        Some(3) => {
            "Use dipped headlights when visibility is reduced. Slow down before bends and leave extra stopping distance. On wet roads, the UK Highway Code advises at least twice the usual stopping distance."
        }
        Some(4) => {
            "Ease off the accelerator and steer gently to regain control. Avoid sudden changes to steering or speed. Reduce speed further when grip returns."
        }
        Some(5) => {
            "A red X closes the lane. Move out as soon as it is safe. If you break down, try to reach a place of relative safety and put on your hazard lights. If you stop in a live lane or cannot leave safely, stay belted in and call 999 in the UK. If you reach a safe place and can leave safely, exit on the side away from traffic and get behind the barrier where there is one."
        }
        Some(6) => {
            "Overtake only when it is safe and legal, with a clear view and space to return to your lane. Leave enough space between you and the vehicle you pass. Wait if you cannot complete the manoeuvre safely."
        }
        Some(7) => {
            "Use the slip road to adjust your speed to the traffic. Give priority to vehicles already on the motorway. Signal and merge only when there is a safe gap."
        }
        _ => {
            "Look well ahead, keep enough space to stop safely and brake gently as traffic slows. Allow more space in poor weather or visibility. Use your lane appropriately and leave extra time before setting off."
        }
    }
}

// https://www.japan.travel/en/plan/tipping-in-japan/
fn tipping(clean: &str) -> &'static str {
    let us = US.is_match(clean) || THE_US.is_match(clean);
    if JAPAN.is_match(clean) {
        "Tipping is generally not expected in Japan, including at restaurants, in taxis and at hotels. Private guides used to international visitors may accept a gratuity."
    } else if TAXI.is_match(clean) {
        if us {
            "In the US, check whether the taxi app or card terminal already includes a tip. You can usually add a gratuity for good service. Ask the driver or check the local visitor guidance if you are unsure."
        } else {
            "Taxi tipping varies by country and city. Check the fare or app for any service charge. Rounding up is a practical option where a small tip is customary."
        }
    } else if us {
        "In the US, tipping is customary at restaurants with table service. Around 18 to 20 percent is a common starting point for good service. Check the bill for a gratuity or service charge before adding more."
    } else if EUROPE.is_match(clean) {
        "Check whether the restaurant has included a service charge. Tipping customs vary across Europe. Where a small tip is customary, rounding up or adding a modest amount for good service is usually enough."
    } else if HOTEL.is_match(clean) || TOUR.is_match(clean) {
        "Ask the hotel or tour operator whether service is included and what guests usually do. A gratuity depends on the country, the service and whether a charge has already been added."
    } else {
        "Tipping customs vary by country and by service. Check whether the bill already includes a gratuity and follow local visitor guidance for restaurants, taxis and hotels."
    }
}

// https://www.moneyhelper.org.uk/en/everyday-money/banking/debit-cards
fn money(clean: &str, earlier: &[Exchange]) -> Cow<'static, str> {
    if CASH_SAFETY.is_match(clean) {
        return Cow::Borrowed(
            "Keep a backup card and some cash separate from your everyday wallet. Save your bank's lost-card contact details and check how to freeze your card if you lose it. Keep cash and cards in a secure pocket or zipped bag, and use a trusted cash machine.",
        );
    }
    const CARD: &str = "Take a backup way to pay and enough local cash for places that do not take cards. Check foreign transaction and ATM fees with your provider. If the terminal offers to convert to pounds, compare its rate with your card's terms and consider paying in the local currency.";
    if clean.contains("how much")
        && let Some(budget) = super::travel_plan::trip_budget(clean, earlier)
    {
        return Cow::Owned(format!("{budget} {CARD}"));
    }
    Cow::Borrowed(CARD)
}

fn country(clean: &str, earlier: &[Exchange]) -> Option<&'static str> {
    near_chat(clean, earlier, |line| {
        named(line, super::TRAVEL_WATER_OK).or_else(|| named(line, super::TRAVEL_WATER_NO))
    })
}

// https://www.gov.uk/foreign-travel-advice
fn travel_advice(clean: &str, earlier: &[Exchange]) -> Option<String> {
    let place = country(clean, earlier)?;
    Some(format!(
        "Read the Foreign, Commonwealth and Development Office travel advice for {} on GOV.UK before booking and again before you travel. Check regional warnings, entry requirements and local laws. Ask your insurer how current advice affects cover and save the relevant emergency contacts.",
        title(place)
    ))
}

fn city_safety(clean: &str, earlier: &[Exchange]) -> Option<String> {
    let city = named(clean, super::TRAVEL_SAFETY_CITIES).or_else(|| {
        earlier.iter().rev().take(3).find_map(|turn| {
            named(&turn.user.to_lowercase(), super::TRAVEL_SAFETY_CITIES)
                .or_else(|| named(&turn.bot.to_lowercase(), super::TRAVEL_SAFETY_CITIES))
        })
    })?;
    Some(format!(
        "For {}, check current travel advice and ask your accommodation or local transport staff about the specific area and time. Plan well-used routes, use licensed transport if you need it, keep valuables out of sight and save the local emergency number.",
        title(city)
    ))
}

// https://www.cdc.gov/yellow-book/hcp/preparing-international-travelers/food-and-water-precautions-for-travelers.html
// https://www.dwi.gov.uk/consumers/learn-more-about-your-water/brown-black-or-orange-water/
fn tap_water(clean: &str, earlier: &[Exchange]) -> Option<Cow<'static, str>> {
    if WATER_OTHER.is_match(clean) || !WATER_ASK.is_match(clean) {
        return None;
    }
    let named_place = country(clean, earlier).or_else(|| {
        near_chat(clean, earlier, |line| {
            named(line, super::TRAVEL_SAFETY_CITIES)
        })
    });
    if (WATER_CHANGED.is_match(clean) && clean.contains("my tap"))
        || (named_place.is_none() && !ABROAD.is_match(clean))
    {
        if !WATER_HOME.is_match(clean) {
            return None;
        }
        return Some(Cow::Borrowed(if WATER_CHANGED.is_match(clean) {
            "If your tap water has changed colour, smell or taste, contact your water supplier and follow their advice before drinking it. Check for notices affecting your address."
        } else {
            "Check your water supplier's current advisories for your address, especially if the water has changed colour, taste or smell."
        }));
    }
    let advice = "Check local drinking-water advice and any alerts for the building or area. If you are unsure that tap water is safe, use unopened bottled water or water treated according to local public-health guidance. Avoid ice and use safe water for brushing your teeth too.";
    Some(match named_place {
        Some(place) => Cow::Owned(format!("For {}, {advice}", title(place))),
        None => Cow::Borrowed(advice),
    })
}

// https://www.mountain.rescue.org.uk/2026/07/summer-in-the-mountains-prepare-well-enjoy-more/
fn hike(clean: &str) -> Option<&'static str> {
    if DAILY.is_match(clean) && !HILL.is_match(clean) {
        return None;
    }
    if !QUANTITY.is_match(clean) && KIT.is_match(clean) && KIT_ASK.is_match(clean) {
        Some(
            "Pack layers, waterproofs, suitable footwear, enough food and water, a map and compass, a charged phone, a torch, a first-aid kit and an emergency layer. Check the mountain forecast, daylight and route before leaving. Tell someone your route and return time.",
        )
    } else if FOOD.is_match(clean) && !WATER.is_match(clean) {
        Some(
            "Take food you know you can eat while walking, such as sandwiches, oatcakes or dried fruit. Bring extra in case the route takes longer. Drink regularly and plan safe refill points before you leave.",
        )
    } else {
        Some(
            "Carry enough water for the route, heat and effort, with extra if there are no reliable refill points. Drink regularly and use known safe water sources. Pack food you can eat on the move and bring extra in case the walk takes longer.",
        )
    }
}

type Place = (&'static str, f64, f64, &'static str);
static DISTANCE_PLACES: LazyLock<Vec<Place>> = LazyLock::new(|| {
    let mut places = coordinates::PLACES.to_vec();
    places.sort_by_key(|(name, _, _, _)| Reverse(name.len()));
    places
});

// https://www.irishferries.com/ie-en/routes-and-times/dublin-holyhead/
// https://www.eurostar.com/uk-en/travel-info/eurostar-experience/the-channel-tunnel
fn crossing(first: Place, second: Place) -> &'static str {
    let region = |(name, _, _, group): Place| match name {
        "belfast" | "derry" => "ie",
        _ => group,
    };
    match (region(first), region(second)) {
        ("uk", "ie") | ("ie", "uk") => {
            "The Irish Sea is between them. Check ferry or flight timetables."
        }
        ("uk", "eu") | ("eu", "uk") => {
            "For a route through France, check ferries or the Channel Tunnel. Eurostar carries foot passengers and LeShuttle carries cars."
        }
        _ => "Use a route planner for travel distance and time by your chosen transport.",
    }
}

fn distance(clean: &str) -> Option<String> {
    if !contains(clean, super::TRAVEL_DISTANCE_ASK) {
        return None;
    }
    let mut found: Vec<(usize, usize, Place)> = Vec::new();
    for &place @ (name, _, _, _) in &*DISTANCE_PLACES {
        for (start, _) in clean.match_indices(name) {
            let end = start + name.len();
            if clean[..start].ends_with(char::is_alphanumeric)
                || clean[end..].starts_with(char::is_alphanumeric)
                || found.iter().any(|&(lo, hi, _)| lo < end && start < hi)
            {
                continue;
            }
            found.push((start, end, place));
        }
    }
    found.sort_by_key(|&(at, _, _)| at);
    // Match the pinned source's exclusion of the ambiguous Belfast City airport name.
    found.retain(|&(_, _, (name, _, _, _))| name != "belfast city");
    let [(_, _, first_place), (_, _, second_place), ..] = found.as_slice() else {
        return None;
    };
    let (first, lat1, lon1, _) = *first_place;
    let (second, lat2, lon2, _) = *second_place;
    if first == second {
        return None;
    }
    let lat_delta = (lat2 - lat1).to_radians() / 2.0;
    let lon_delta = (lon2 - lon1).to_radians() / 2.0;
    let h = lat_delta.sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * lon_delta.sin().powi(2);
    let km = 2.0 * 6371.0 * h.clamp(0.0, 1.0).sqrt().asin();
    let round = |length: f64| {
        if length < 20.0 {
            length.round_ties_even()
        } else {
            (length / 5.0).round_ties_even() * 5.0
        }
    };
    Some(format!(
        "{} to {} is about {:.0} km ({:.0} miles) in a straight line. {}",
        title(first),
        title(second),
        round(km),
        round(km * 0.62137),
        crossing(*first_place, *second_place)
    ))
}

pub(super) fn registered(
    id: &str,
    reading: &Reading<'_>,
    earlier: &[Exchange],
) -> Option<Cow<'static, str>> {
    let clean = &reading.line.clean;
    match id {
        "plug-country" => adapter(clean, earlier).map(Cow::Owned),
        "driving-in-traffic" => Some(Cow::Borrowed(driving(reading))),
        "tipping-abroad" => Some(Cow::Borrowed(tipping(clean))),
        "money-abroad" => Some(money(clean, earlier)),
        "travel-advice-abroad" => travel_advice(clean, earlier).map(Cow::Owned),
        "tap-water-abroad" => tap_water(clean, earlier),
        "hike-day" => hike(clean).map(Cow::Borrowed),
        "city-safe-in-the-day" => city_safety(clean, earlier).map(Cow::Owned),
        "far-between-places" => distance(clean).map(Cow::Owned),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_prefers_complete_names_and_checks_absolute_rounding() {
        let result = distance("how far is new york from boston").unwrap();
        assert!(result.starts_with("New York to Boston"));
        assert!(result.contains("305 km (190 miles)"));
        assert!(
            distance("how far is belfast from manchester")
                .unwrap()
                .contains("Irish Sea")
        );
        assert!(
            distance("how far is paris from london")
                .unwrap()
                .contains("LeShuttle carries cars")
        );
        assert!(distance("how far is belfast city from london").is_none());
        assert!(distance("how far is london from london").is_none());
        assert!(distance("how far can i go on 20 pounds of petrol").is_none());
    }

    #[test]
    fn practical_cards_scope_safety_to_reported_destination_or_condition() {
        assert!(
            adapter("do i need an adapter in ireland", &[])
                .unwrap()
                .contains("type G")
        );
        assert!(
            tipping("how much should i tip my taxi driver in tokyo")
                .contains("not expected in Japan")
        );
        let motorway =
            Reading::new("What should I do if my car breaks down in a smart motorway lane?");
        assert!(driving(&motorway).contains("stay belted in and call 999"));
        assert!(
            tap_water("can i drink the tap water in mexico", &[])
                .unwrap()
                .contains("unopened bottled water")
        );
        assert!(tap_water("can i take a water bottle through security", &[]).is_none());
        assert!(hike("i walk daily and don't eat much").is_none());
        assert!(
            hike("what should i pack for a day hike")
                .unwrap()
                .contains("mountain forecast")
        );
    }
}
