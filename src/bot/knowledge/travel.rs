//! Country, regional and state capitals from the pinned PyBot travel table.
use regex::Regex;
use std::sync::LazyLock;

const CAPITALS: &[(&str, &str)] = &[
    ("democratic republic of the congo", "Kinshasa."),
    ("saint vincent and the grenadines", "Kingstown."),
    ("central african republic", "Bangui."),
    ("bosnia and herzegovina", "Sarajevo."),
    ("saint kitts and nevis", "Basseterre."),
    ("sao tome and principe", "Sao Tome."),
    ("united arab emirates", "Abu Dhabi, not Dubai."),
    ("antigua and barbuda", "Saint John's."),
    ("trinidad and tobago", "Port of Spain."),
    ("dominican republic", "Santo Domingo."),
    (
        "northern territory",
        "Darwin, not Canberra - Canberra is the capital of the country.",
    ),
    ("equatorial guinea", "Malabo."),
    (
        "western australia",
        "Perth, not Canberra - Canberra is the capital of the country.",
    ),
    ("marshall islands", "Majuro."),
    ("papua new guinea", "Port Moresby."),
    (
        "northern ireland",
        "Belfast, not Dublin - Dublin is the capital of the Republic of Ireland.",
    ),
    (
        "washington state",
        "Olympia, not Seattle - and not Washington DC, which is a different city.",
    ),
    ("north macedonia", "Skopje."),
    ("solomon islands", "Honiara."),
    (
        "new south wales",
        "Sydney, not Canberra - Canberra is the capital of the country.",
    ),
    (
        "south australia",
        "Adelaide, not Canberra - Canberra is the capital of the country.",
    ),
    ("czech republic", "Prague."),
    (
        "american samoa",
        "Pago Pago, on Tutuila - a US territory of its own, not Samoa, whose capital is Apia.",
    ),
    ("united kingdom", "London."),
    (
        "new york state",
        "Albany, not New York City - the city is not the capital of the state.",
    ),
    ("north carolina", "Raleigh."),
    ("south carolina", "Columbia."),
    ("united states", "Washington DC, not New York."),
    ("cote d'ivoire", "Yamoussoukro, not Abidjan."),
    ("guinea-bissau", "Bissau."),
    ("liechtenstein", "Vaduz."),
    (
        "great britain",
        "London, though Great Britain is the island, not the country: the UK is the four nations together.",
    ),
    ("massachusetts", "Boston."),
    ("new hampshire", "Concord."),
    ("west virginia", "Charleston."),
    (
        "south africa",
        "Pretoria, though the parliament sits in Cape Town and the courts in Johannesburg.",
    ),
    ("burkina faso", "Ouagadougou."),
    ("saudi arabia", "Riyadh."),
    ("sierra leone", "Freetown."),
    ("turkmenistan", "Ashgabat."),
    ("vatican city", "Vatican City, which is the whole country."),
    ("pennsylvania", "Harrisburg, not Philadelphia."),
    ("north dakota", "Bismarck."),
    ("rhode island", "Providence."),
    ("south dakota", "Pierre."),
    (
        "switzerland",
        "Bern, the de facto capital; Switzerland has no official one.",
    ),
    (
        "netherlands",
        "Amsterdam, though the government sits in The Hague.",
    ),
    ("new zealand", "Wellington, not Auckland."),
    ("south korea", "Seoul."),
    ("afghanistan", "Kabul."),
    ("el salvador", "San Salvador."),
    (
        "ivory coast",
        "Yamoussoukro, not Abidjan, which is the biggest city.",
    ),
    ("north korea", "Pyongyang."),
    (
        "philippines",
        "Manila, not Quezon City, which is the bigger city.",
    ),
    ("saint lucia", "Castries."),
    ("puerto rico", "San Juan, a US territory, not a state."),
    ("south sudan", "Juba."),
    ("timor-leste", "Dili."),
    ("connecticut", "Hartford."),
    ("mississippi", "Jackson."),
    ("azerbaijan", "Baku."),
    ("bangladesh", "Dhaka."),
    ("cape verde", "Praia."),
    ("costa rica", "San Jose."),
    (
        "kazakhstan",
        "Astana, which was called Nur-Sultan from 2019 to 2022.",
    ),
    ("kyrgyzstan", "Bishkek."),
    ("luxembourg", "Luxembourg City."),
    ("madagascar", "Antananarivo."),
    ("mauritania", "Nouakchott."),
    ("micronesia", "Palikir."),
    ("montenegro", "Podgorica."),
    ("mozambique", "Maputo."),
    ("san marino", "San Marino."),
    ("seychelles", "Victoria."),
    ("tajikistan", "Dushanbe."),
    ("uzbekistan", "Tashkent."),
    (
        "washington",
        "Olympia if you mean the state, Washington DC if you mean the city - the two are different, and neither is New York.",
    ),
    (
        "queensland",
        "Brisbane, not Canberra - Canberra is the capital of the country.",
    ),
    (
        "california",
        "Sacramento, not Los Angeles or San Francisco.",
    ),
    ("new mexico", "Santa Fe, not Albuquerque."),
    ("new jersey", "Trenton."),
    (
        "australia",
        "Canberra, not Sydney: it was purpose-built as the compromise between Sydney and Melbourne.",
    ),
    ("argentina", "Buenos Aires."),
    ("guatemala", "Guatemala City."),
    (
        "indonesia",
        "Jakarta, though the government is moving to a new capital, Nusantara, in Borneo.",
    ),
    ("lithuania", "Vilnius."),
    ("mauritius", "Port Louis."),
    ("nicaragua", "Managua."),
    ("singapore", "Singapore itself - the country is one city."),
    (
        "sri lanka",
        "Sri Jayawardenepura Kotte, though Colombo is the one everyone names.",
    ),
    (
        "swaziland",
        "Mbabane for the administration and Lobamba for the parliament; the country is called Eswatini now.",
    ),
    ("venezuela", "Caracas."),
    ("tennessee", "Nashville."),
    ("minnesota", "St Paul, not Minneapolis."),
    ("wisconsin", "Madison, not Milwaukee."),
    ("louisiana", "Baton Rouge, not New Orleans."),
    ("portugal", "Lisbon."),
    ("barbados", "Bridgetown."),
    ("botswana", "Gaborone."),
    ("bulgaria", "Sofia."),
    ("cambodia", "Phnom Penh."),
    ("cameroon", "Yaounde."),
    ("colombia", "Bogota."),
    ("djibouti", "Djibouti."),
    (
        "dominica",
        "Roseau - not Santo Domingo, which is the Dominican Republic's capital.",
    ),
    ("dr congo", "Kinshasa."),
    (
        "eswatini",
        "Mbabane for the administration, Lobamba for the parliament; the country was called Swaziland until 2018.",
    ),
    ("ethiopia", "Addis Ababa."),
    ("honduras", "Tegucigalpa."),
    ("kiribati", "South Tarawa."),
    (
        "malaysia",
        "Kuala Lumpur, though the government sits in Putrajaya.",
    ),
    ("maldives", "Male."),
    ("mongolia", "Ulaanbaatar."),
    ("pakistan", "Islamabad, not Karachi."),
    ("paraguay", "Asuncion."),
    ("slovakia", "Bratislava."),
    ("slovenia", "Ljubljana."),
    ("suriname", "Paramaribo."),
    (
        "tanzania",
        "Dodoma, not Dar es Salaam, which is the biggest city.",
    ),
    ("thailand", "Bangkok."),
    ("zimbabwe", "Harare."),
    (
        "tasmania",
        "Hobart, not Canberra - Canberra is the capital of the country.",
    ),
    (
        "victoria",
        "Melbourne, not Canberra - Canberra is the capital of the country.",
    ),
    (
        "scotland",
        "Edinburgh, not London - London is the capital of the UK.",
    ),
    (
        "new york",
        "Albany, not New York City - the city is not the capital of the state.",
    ),
    ("illinois", "Springfield, not Chicago."),
    ("michigan", "Lansing, not Detroit."),
    ("missouri", "Jefferson City, not St Louis or Kansas City."),
    ("maryland", "Annapolis, not Baltimore."),
    ("colorado", "Denver."),
    ("virginia", "Richmond."),
    ("kentucky", "Frankfort, not Louisville."),
    ("oklahoma", "Oklahoma City."),
    ("nebraska", "Lincoln, not Omaha."),
    ("arkansas", "Little Rock."),
    ("delaware", "Dover."),
    ("america", "Washington DC, not New York."),
    ("germany", "Berlin, since reunification in 1990."),
    ("denmark", "Copenhagen."),
    ("finland", "Helsinki."),
    ("ireland", "Dublin."),
    ("austria", "Vienna."),
    ("belgium", "Brussels."),
    ("hungary", "Budapest."),
    ("czechia", "Prague."),
    ("albania", "Tirana."),
    ("algeria", "Algiers."),
    ("andorra", "Andorra la Vella."),
    ("armenia", "Yerevan."),
    ("bahamas", "Nassau."),
    ("bahrain", "Manama."),
    ("belarus", "Minsk."),
    (
        "bolivia",
        "Sucre is the constitutional capital; La Paz is where the government sits.",
    ),
    (
        "burundi",
        "Gitega, since 2019; Bujumbura was the capital before that.",
    ),
    ("comoros", "Moroni."),
    ("croatia", "Zagreb."),
    ("ecuador", "Quito."),
    ("eritrea", "Asmara."),
    ("estonia", "Tallinn."),
    (
        "georgia",
        "Tbilisi - the country's capital. The US state of Georgia has Atlanta.",
    ),
    ("grenada", "St George's."),
    (
        "holland",
        "Amsterdam, though the government sits in The Hague.",
    ),
    ("jamaica", "Kingston."),
    ("lebanon", "Beirut."),
    ("lesotho", "Maseru."),
    ("liberia", "Monrovia."),
    ("moldova", "Chisinau."),
    (
        "myanmar",
        "Naypyidaw, not Yangon, which was the capital until 2006.",
    ),
    ("namibia", "Windhoek."),
    ("romania", "Bucharest."),
    (
        "bermuda",
        "Hamilton, a British territory - not Hamilton, Ontario.",
    ),
    ("senegal", "Dakar."),
    ("somalia", "Mogadishu."),
    ("tunisia", "Tunis."),
    ("ukraine", "Kyiv."),
    ("britain", "London."),
    ("uruguay", "Montevideo."),
    ("vanuatu", "Port Vila."),
    ("vietnam", "Hanoi, not Ho Chi Minh City."),
    ("florida", "Tallahassee, not Miami."),
    ("arizona", "Phoenix."),
    ("indiana", "Indianapolis."),
    ("alabama", "Montgomery."),
    ("montana", "Helena."),
    ("vermont", "Montpelier."),
    ("wyoming", "Cheyenne."),
    ("england", "London."),
    ("morocco", "Rabat, not Casablanca."),
    ("nigeria", "Abuja, not Lagos."),
    ("iceland", "Reykjavík."),
    ("canada", "Ottawa, not Toronto."),
    (
        "brazil",
        "Brasília, not Rio, and a planned city built in four years.",
    ),
    ("turkey", "Ankara, not Istanbul."),
    ("france", "Paris."),
    ("russia", "Moscow."),
    ("norway", "Oslo."),
    ("sweden", "Stockholm."),
    ("greece", "Athens."),
    ("poland", "Warsaw."),
    ("mexico", "Mexico City."),
    ("angola", "Luanda."),
    ("belize", "Belmopan, not Belize City."),
    ("bhutan", "Thimphu."),
    ("bosnia", "Sarajevo."),
    ("brunei", "Bandar Seri Begawan."),
    ("cyprus", "Nicosia."),
    ("gambia", "Banjul."),
    ("guinea", "Conakry."),
    ("guyana", "Georgetown."),
    (
        "israel",
        "Jerusalem, which Israel calls its capital; most countries keep their embassies in Tel Aviv because the status is disputed.",
    ),
    ("jordan", "Amman."),
    ("kuwait", "Kuwait City."),
    ("latvia", "Riga."),
    ("malawi", "Lilongwe."),
    ("monaco", "Monaco itself - the country is one city."),
    ("panama", "Panama City."),
    ("rwanda", "Kigali."),
    ("serbia", "Belgrade."),
    ("taiwan", "Taipei."),
    ("tuvalu", "Funafuti."),
    ("uganda", "Kampala."),
    ("zambia", "Lusaka."),
    ("alaska", "Juneau, not Anchorage."),
    ("nevada", "Carson City, not Las Vegas."),
    ("oregon", "Salem, not Portland."),
    ("kansas", "Topeka, not Wichita."),
    ("hawaii", "Honolulu."),
    ("spain", "Madrid."),
    ("italy", "Rome."),
    ("japan", "Tokyo."),
    ("china", "Beijing."),
    ("india", "New Delhi, not Mumbai."),
    (
        "benin",
        "Porto-Novo, though the government sits in Cotonou.",
    ),
    (
        "burma",
        "Naypyidaw, not Yangon, which was the capital until 2006.",
    ),
    ("chile", "Santiago."),
    (
        "congo",
        "Two countries share the name: Brazzaville for the Republic of the Congo, Kinshasa for the Democratic Republic of the Congo.",
    ),
    ("gabon", "Libreville."),
    ("ghana", "Accra."),
    ("haiti", "Port-au-Prince."),
    ("libya", "Tripoli."),
    ("malta", "Valletta."),
    (
        "nauru",
        "Yaren, where the government sits; Nauru has no official capital.",
    ),
    ("nepal", "Kathmandu."),
    ("niger", "Niamey."),
    (
        "palau",
        "Ngerulmud, a purpose-built capital; the biggest town is Koror.",
    ),
    ("qatar", "Doha."),
    ("samoa", "Apia."),
    ("sudan", "Khartoum."),
    ("syria", "Damascus."),
    ("tonga", "Nuku'alofa."),
    ("yemen", "Sanaa."),
    (
        "wales",
        "Cardiff, not London - London is the capital of the UK.",
    ),
    ("texas", "Austin, not Houston or Dallas."),
    ("idaho", "Boise."),
    ("maine", "Augusta, not Portland."),
    ("egypt", "Cairo."),
    ("kenya", "Nairobi."),
    ("chad", "N'Djamena."),
    ("cuba", "Havana."),
    ("fiji", "Suva."),
    ("iran", "Tehran."),
    ("iraq", "Baghdad."),
    ("laos", "Vientiane."),
    ("mali", "Bamako."),
    ("oman", "Muscat."),
    ("peru", "Lima."),
    ("guam", "Hagåtña, a US territory in the Pacific."),
    ("togo", "Lome."),
    ("ohio", "Columbus."),
    ("utah", "Salt Lake City."),
    ("iowa", "Des Moines."),
    ("usa", "Washington DC, not New York."),
    ("uae", "Abu Dhabi, not Dubai."),
    ("uk", "London."),
];

static CAPITAL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bcapitals?\b").unwrap());
static OTHER_USE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r"\b(?:gains|tax|taxes|punishment|death penalty|letters?|capitalise|capitalize|capitalism|investment|invest|investor|business|businesses|start up|startup|fund|funding|pension|mortgage|loan|venture|social capital|human capital|how much)\b"
).unwrap()
});

pub(crate) fn capital(clean: &str) -> Option<&'static str> {
    let reply = capital_reply(clean)?;
    (!OTHER_USE.is_match(clean)).then_some(reply)
}

/// Look up the place named in the capital card's own input.
pub(crate) fn capital_reply(clean: &str) -> Option<&'static str> {
    if !CAPITAL.is_match(clean) {
        return None;
    }
    // The reference tests ASCII letters at either end; punctuation and digits delimit a name.
    CAPITALS.iter().find_map(|&(name, answer)| {
        clean
            .match_indices(name)
            .any(|(at, _)| {
                !clean.as_bytes()[..at]
                    .last()
                    .is_some_and(u8::is_ascii_lowercase)
                    && !clean
                        .as_bytes()
                        .get(at + name.len())
                        .is_some_and(u8::is_ascii_lowercase)
            })
            .then_some(answer)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn specific_names_and_other_uses_of_capital() {
        assert_eq!(
            capital("what is the capital of Chile?".to_lowercase().as_str()),
            Some("Santiago.")
        );
        assert_eq!(
            capital("what is the capital of new south wales?"),
            Some("Sydney, not Canberra - Canberra is the capital of the country.")
        );
        assert_eq!(capital("what is the capital of the roman empire?"), None);
        assert_eq!(capital("do i pay capital gains tax in spain?"), None);
    }
}
