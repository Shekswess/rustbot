//! Borders, continents and EU membership, ported from PyBot's geographic readers.
#[path = "geography_data.rs"]
mod data;
pub(crate) use super::geography_records::registered;
use data::*;

pub(super) struct Country {
    name: &'static str,
    shown: &'static str,
    subject: &'static str,
    continent: &'static str,
    neighbours: &'static [&'static str],
    neighbour_names: &'static str,
    note: &'static str,
    continent_note: &'static str,
    eu_member: bool,
    eu_note: &'static str,
}

fn occurrences<'a>(text: &'a str, phrase: &'a str) -> impl Iterator<Item = (usize, usize)> + 'a {
    text.match_indices(phrase).filter_map(move |(at, matched)| {
        let end = at + matched.len();
        let bytes = text.as_bytes();
        (!bytes[..at].last().is_some_and(u8::is_ascii_lowercase)
            && !bytes.get(end).is_some_and(u8::is_ascii_lowercase))
        .then_some((at, end))
    })
}
fn any(text: &str, phrases: &[&str]) -> bool {
    phrases
        .iter()
        .any(|p| occurrences(text, p).next().is_some())
}

/// Use the original geography registry and require a literal space on both sides.
pub(super) fn source_country_named(text: &str) -> bool {
    let bytes = text.as_bytes();
    COUNTRIES.iter().any(|country| {
        // These two display forms were added for the native geographical answers.
        !matches!(
            country.name,
            "democratic republic of the congo" | "republic of the congo"
        ) && text.match_indices(country.name).any(|(start, matched)| {
            start > 0 && bytes[start - 1] == b' ' && bytes.get(start + matched.len()) == Some(&b' ')
        })
    })
}
pub(super) fn countries(text: &str) -> Vec<&'static Country> {
    let mut spans = Vec::new();
    for &(phrase, id) in PHRASES {
        for (at, end) in occurrences(text, phrase) {
            if !spans
                .iter()
                .any(|&(start, stop, _)| start < end && at < stop)
            {
                spans.push((at, end, id))
            }
        }
    }
    spans.sort_unstable();
    let mut found: Vec<&Country> = Vec::new();
    for (_, _, id) in spans {
        let country = &COUNTRIES[id];
        if !found.iter().any(|c| c.name == country.name) {
            found.push(country);
        }
    }
    found
}
pub(super) fn region(text: &str) -> Option<&'static str> {
    CONTINENTS
        .iter()
        .find_map(|&(name, region)| occurrences(text, name).next().map(|_| region))
}
fn sentence(mut answer: String, note: &str) -> String {
    if !note.is_empty() {
        answer.push(' ');
        answer.push_str(note)
    }
    if let Some(first) = answer.get_mut(..1) {
        first.make_ascii_uppercase()
    }
    answer
}
pub(super) fn borders(clean: &str, places: &[&Country]) -> Option<String> {
    let &first = places.first()?;
    if first.continent.is_empty() {
        return None;
    }
    let subject = first.subject;
    if let Some(second) = places.get(1) {
        let other = second.shown;
        return Some(
            if first.neighbours.contains(&second.name) || second.neighbours.contains(&first.name) {
                format!("Yes - {subject} and {other} share a border.")
            } else {
                format!("No - {subject} and {other} do not share a land border.")
            },
        );
    }
    if first.neighbours.is_empty() {
        return Some(sentence(
            format!("{subject} has no land neighbours."),
            first.note,
        ));
    }
    if any(clean, &["how many"]) {
        return Some(sentence(
            format!(
                "{subject} has {} land neighbours: {}.",
                first.neighbours.len(),
                first.neighbour_names
            ),
            "",
        ));
    }
    Some(sentence(
        format!("{subject} borders {}.", first.neighbour_names),
        first.note,
    ))
}
pub(super) fn continent(clean: &str, country: &Country) -> Option<String> {
    if country.continent.is_empty() {
        return None;
    }
    let prefix = match region(clean) {
        Some(region)
            if region != country.continent
                && country
                    .continent
                    .to_lowercase()
                    .contains(&region.to_lowercase()) =>
        {
            "Yes - "
        }
        Some(region) if region != country.continent => "No - ",
        _ => "",
    };
    Some(sentence(
        format!("{prefix}{} is in {}.", country.subject, country.continent),
        country.continent_note,
    ))
}
pub(super) fn eu(clean: &str, country: Option<&&Country>) -> String {
    let Some(country) = country else {
        return "Twenty-seven countries are in the EU: Austria, Belgium, Bulgaria, Croatia, Cyprus, Czech Republic, Denmark, Estonia, Finland, France, Germany, Greece, Hungary, Ireland, Italy, Latvia, Lithuania, Luxembourg, Malta, the Netherlands, Poland, Portugal, Romania, Slovakia, Slovenia, Spain and Sweden.".into();
    };
    let subject = country.subject;
    if any(
        clean,
        &[
            "leave", "leaves", "leaving", "left", "quit", "quitting", "brexit",
        ],
    ) {
        if country.name == "united kingdom" {
            return "Yes - the UK left the EU on 31 January 2020, after the Brexit vote in 2016. It has been outside the single market and customs union since then.".into();
        }
        if country.eu_member {
            return format!("No - {subject} is still in the EU; it is one of the 27 members.");
        }
        return format!("No - {subject} was never in the EU, so there was nothing to leave.");
    }
    if country.eu_member {
        format!("Yes - {subject} is one of the 27 countries in the EU.")
    } else if country.eu_note.is_empty() {
        format!("No - {subject} is not in the EU.")
    } else {
        format!("No - {subject} is not in the EU: {}", country.eu_note)
    }
}

pub(super) fn region_countries(place: &str) -> Option<&'static str> {
    REGION_LISTS
        .iter()
        .find_map(|&(name, reply)| (name == place).then_some(reply))
}

pub(crate) fn answer(clean: &str) -> Option<String> {
    let first = clean.split_whitespace().next()?;
    if !clean.contains('?')
        && !matches!(
            first,
            "what"
                | "what's"
                | "whats"
                | "which"
                | "who"
                | "where"
                | "how"
                | "are"
                | "is"
                | "does"
                | "do"
                | "did"
                | "can"
                | "has"
                | "have"
        )
    {
        return None;
    }
    if any(clean, EU_ASK) && !any(clean, EU_AVOID) {
        return Some(eu(clean, countries(clean).first()));
    }
    if any(clean, BORDERS_ASK) && !any(clean, BORDERS_AVOID) {
        return borders(clean, &countries(clean));
    }
    if any(clean, REGION_ASK)
        && !any(clean, REGION_AVOID)
        && let Some(region) = region(clean)
    {
        return region_countries(region).map(str::to_owned);
    }
    if any(clean, CONTINENT_ASK) && !any(clean, CONTINENT_AVOID) {
        return continent(clean, countries(clean).first()?);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::source_country_named;

    #[test]
    fn family_country_reader_keeps_source_names_and_literal_boundaries() {
        for text in [
            " she moved to mongolia ",
            " moving to united states ",
            " going to bosnia and herzegovina ",
        ] {
            assert!(source_country_named(text), "{text}");
        }
        for text in [
            " she moved to madeira ",
            " she moved to united states! ",
            " went to xmongolia ",
            " going to the democratic republic of the congo ",
            " going to the republic of the congo ",
        ] {
            assert!(!source_country_named(text), "{text}");
        }
    }
}
