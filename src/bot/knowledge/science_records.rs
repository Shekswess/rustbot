//! Moon, building, bridge and tunnel facts selected by the source science cards.

use std::borrow::Cow;

use super::has_space_delimited_phrase as has_term;
use crate::bot::Reading;

const PLANET_MOONS: &[(&str, &str)] = &[
    (
        "jupiter",
        "Jupiter had 115 confirmed moons as of August 2026.",
    ),
    (
        "saturn",
        "Saturn had 293 confirmed moons as of August 2026.",
    ),
    ("uranus", "Uranus had 29 confirmed moons as of August 2026."),
    ("neptune", "Neptune has 16 confirmed moons."),
    ("mars", "Mars has 2 confirmed moons."),
    ("earth", "Earth has one moon."),
    ("venus", "Venus has no moons."),
    ("mercury", "Mercury has no moons."),
];

const MOST_MOONS: &str =
    "Saturn had the most confirmed moons in NASA's August 2026 count, with 293. Jupiter had 115.";

pub(super) fn moons(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = &reading.line.clean;
    if contains_any(line, &["name", "called", "which moon", "list"]) {
        if line.contains("earth") {
            return Some(Cow::Borrowed(
                "Just the Moon, the fifth largest in the solar system and the only other world people have stood on.",
            ));
        }
        if let Some((_, reply)) = super::SCIENCE_NAMED_MOONS
            .iter()
            .find(|(planet, _)| line.contains(planet))
        {
            return Some(Cow::Borrowed(reply));
        }
        return Some(Cow::Borrowed(
            "Earth's is the Moon, Mars has Phobos and Deimos, and the four big Jovian moons are Io, Europa, Ganymede and Callisto. Most of the hundreds the counts refer to are unnamed rocks a few kilometres across.",
        ));
    }
    if line.contains("most") && !contains_any(line, &["earth", "mars", "jupiter"]) {
        return Some(Cow::Borrowed(MOST_MOONS));
    }
    if let Some((_, reply)) = PLANET_MOONS.iter().find(|(name, _)| line.contains(name)) {
        return Some(Cow::Borrowed(reply));
    }
    Some(Cow::Borrowed(
        "Mercury and Venus have none, Earth has one and Mars has two. Saturn topped NASA's August 2026 count with 293 confirmed moons.",
    ))
}

const TALLEST_PLACES: &[&str] = &[
    "the world",
    "world",
    "london",
    "uk",
    "the uk",
    "britain",
    "great britain",
    "the united kingdom",
    "united kingdom",
    "england",
    "scotland",
    "wales",
    "british",
];
const LONGEST_PLACES: &[&str] = &[
    "the world",
    "world",
    "europe",
    "uk",
    "the uk",
    "britain",
    "great britain",
    "the united kingdom",
    "united kingdom",
    "england",
    "scotland",
    "wales",
    "british",
];
const BRITAIN: &[&str] = &["uk", "britain", "england", "scotland", "wales", "british"];

fn line_with_spaces(reading: &Reading<'_>) -> String {
    let line = &reading.line;
    if line.clean.is_empty() {
        format!(" {} ", line.text.to_lowercase())
    } else {
        format!(" {} ", line.clean)
    }
}

fn contains_any(line: &str, words: &[&str]) -> bool {
    words.iter().any(|word| line.contains(word))
}

fn has_any_term(line: &str, terms: &[&str]) -> bool {
    terms.iter().any(|term| has_term(line, term))
}

fn names_unknown_place(line: &str, known: &[&str]) -> bool {
    let Some((_, place)) = line.rsplit_once(" in ") else {
        return false;
    };
    let words = place.split_whitespace();
    words.clone().next().is_some()
        && !known.iter().any(|name| {
            let mut words = words.clone();
            name.split(' ')
                .all(|expected| words.next() == Some(expected))
        })
}

pub(super) fn the_tallest_building(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = line_with_spaces(reading);
    if let Some((_, reply)) = super::SCIENCE_TALL_BUILDINGS
        .iter()
        .find(|(names, _)| has_any_term(&line, names))
    {
        return Some(Cow::Borrowed(reply));
    }
    if names_unknown_place(&line, TALLEST_PLACES) {
        return None;
    }
    let reply = if has_any_term(&line, &["skyscraper", "skyscrapers"])
        && !has_any_term(
            &line,
            &[
                "tallest", "highest", "biggest", "world", "london", "uk", "britain",
            ],
        ) {
        "A skyscraper is a very tall building. The Council on Tall Buildings and Urban Habitat defines supertall as at least 300 m. The steel frame and the safety lift made it possible to build much higher. The ten-storey Home Insurance Building in Chicago is an early example."
    } else if has_term(&line, "london") && !has_term(&line, "world") {
        "The tallest building in London is The Shard at 310 m, finished in 2012, with 95 storeys; One Canada Square at Canary Wharf, 235 m, was the tallest before it. The tallest in the United Kingdom is still The Shard."
    } else if has_any_term(&line, BRITAIN) {
        "The tallest building in the United Kingdom is The Shard in London at 310 m, finished in 2012."
    } else {
        "The tallest building in the world is the Burj Khalifa in Dubai at 828 m, 163 floors, finished in 2010; Merdeka 118 in Kuala Lumpur at 679 m is second, and the Shanghai Tower at 632 m third. The Burj was built that tall because the client wanted it to be, and the limits above 500 m are the lifts - a single lift run gets too long - and the wind, which is why the tower steps in as it rises."
    };
    Some(Cow::Borrowed(reply))
}

fn tunnel_record(line: &str) -> Option<&'static str> {
    if !line.contains(" tunnel") {
        return None;
    }
    if line.contains("channel tunnel") || (has_term(line, "channel") && line.contains("long")) {
        return Some(
            "The Channel Tunnel is 50.5 km from end to end, including about 37 km under the sea. Three tubes run side by side: two for trains and a smaller service tunnel between them, bored through the chalk beneath the seabed.",
        );
    }
    contains_any(line, &["longest", "how long"]).then_some(
        "The Gotthard Base Tunnel in Switzerland is the longest railway tunnel, at 57 km. Norway's Lærdal Tunnel is 24.5 km long. The Channel Tunnel has roughly 37 km beneath the sea.",
    )
}

fn regional_bridge_record(line: &str) -> Option<&'static str> {
    if !line.contains(" bridge")
        || !contains_any(line, &["longest", "biggest", "largest"])
        || names_unknown_place(line, LONGEST_PLACES)
    {
        return None;
    }
    if has_term(line, "europe") && !has_term(line, "world") {
        return Some(
            "The Crimean Bridge is about 19 km long. Inside the European Union, the Vasco da Gama Bridge crossing in Lisbon is about 17 km long. Comparisons depend on whether they count approach viaducts and tunnels.",
        );
    }
    has_any_term(line, BRITAIN).then_some(
        "The Prince of Wales Bridge carries the M4 over the Severn on a crossing about 5 km long. The Humber Bridge is about 2.2 km long. Total crossing length and longest single span are different records.",
    )
}

pub(super) fn a_kind_of_bridge(reading: &Reading<'_>) -> Option<Cow<'static, str>> {
    let line = line_with_spaces(reading);
    if let Some(reply) = regional_bridge_record(&line).or_else(|| tunnel_record(&line)) {
        return Some(Cow::Borrowed(reply));
    }
    if contains_any(&line, &["longest", "biggest", "largest"])
        && names_unknown_place(&line, LONGEST_PLACES)
    {
        return None;
    }
    if has_any_term(
        &line,
        &[
            "build",
            "built",
            "builds",
            "building",
            "construct",
            "constructed",
        ],
    ) && has_any_term(&line, &["bridge", "bridges", "viaduct", "flyover"])
        && has_any_term(
            &line,
            &[
                "motorway",
                "motorways",
                "road",
                "roads",
                "railway",
                "railways",
                "rail",
            ],
        )
    {
        return Some(Cow::Borrowed(
            "A bridge over a motorway is built a piece at a time so the road can stay open: the beams or the whole deck are put together beside it and lifted or slid into place in overnight closures, or the deck is pushed out from one side with a launching girder. The piers go in first, the beams land on them, and the road below is only shut for the hours it takes to place each piece.",
        ));
    }
    if has_any_term(
        &line,
        &[
            "how long does it take",
            "how long to",
            "cost of",
            "how much does",
        ],
    ) || (has_any_term(&line, &["what does", "what do", "meaning", "in english"])
        && !has_any_term(
            &line,
            &[
                "bridge", "bridges", "tunnel", "tunnels", "viaduct", "aqueduct", "span", "spans",
            ],
        ))
    {
        return None;
    }
    let reply = super::SCIENCE_STRUCTURES
        .iter()
        .find(|(names, _)| has_any_term(&line, names))
        .map_or("A bridge carries a load across a gap, and it does it in one of four ways: a beam resting on a support at each end, an arch pushing the weight outwards into solid ground, a truss or box girder making a beam stiffer, or cables hanging the deck from towers - suspension or cable-stayed. Stone and brick work in an arch, steel works in all four.", |(_, reply)| *reply);
    Some(Cow::Borrowed(reply))
}
