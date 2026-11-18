//! Country guides and average city climate from the pinned travel tables.

pub(super) const TRIP_WORDS: &str = r#"(?i)\b(?:trip|trips|holiday|holidays|travel\w*|visit\w*|flying|flights?|honeymoon|abroad|somewhere|city break|weekend (?:away|in)|a week in|fortnight|beach\w*|going away|get away|day out|get(?:ting)? to|where (?:should|shall|do|can) i go)\b"#;
pub(super) const A_FACT_OF_THE_COUNTRY: &str = r#"(?i)\bcapital\b"#;
pub(super) const US_AS_A_PLACE: &str = r#"(?i)\bthe\s+us\b"#;
pub(super) const COUNTRIES: &[(&str, &str)] = &[
    (
        "united states",
        "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
    ),
    (
        "netherlands",
        "The Netherlands: Amsterdam for the canals and the museums. Haarlem or Delft as a quieter base. The country is small, so everywhere is easy to reach.",
    ),
    (
        "switzerland",
        "Switzerland: Zurich or Lucerne as a base. The Bernese Oberland for the mountain trains. A rail pass pays off fast.",
    ),
    (
        "new zealand",
        "New Zealand: Queenstown and the southern lakes for the mountains. The North Island for beaches and volcanoes. Pick one island rather than trying both.",
    ),
    (
        "australia",
        "Australia: Sydney for the harbour and the beaches. Melbourne for food, with the Great Ocean Road from there. Flying between the cities is normal.",
    ),
    (
        "singapore",
        "Singapore: The hawker centres first. Gardens by the Bay after dark. A walk round Tiong Bahru for the old flats and the coffee.",
    ),
    (
        "cape town",
        "Cape Town: Table Mountain on a clear morning. The Cape peninsula drive. The winelands for the wine farms, and a car to do any of it.",
    ),
    (
        "argentina",
        "Argentina: Buenos Aires for the food and the nightlife. Mendoza for wine. Patagonia if you have the time.",
    ),
    (
        "portugal",
        "Portugal: Lisbon for trams, tarts and the river. Porto for the port cellars and the bookshop. The Algarve for beaches and cliffs.",
    ),
    (
        "scotland",
        "Scotland: Edinburgh for the castle and the Old Town. The Highlands for lochs and mountains. A car helps north of the cities.",
    ),
    (
        "thailand",
        "Thailand: Bangkok for temples and street food. Chiang Mai for the north. A flight south for the islands.",
    ),
    (
        "germany",
        "Germany: Berlin for the history and the nightlife. Munich for beer gardens and the Alps. The Rhine and Mosel valleys for small wine towns.",
    ),
    (
        "ireland",
        "Ireland: Dublin for a city break. Galway and the Burren if you fly into Shannon. The west is the trip.",
    ),
    (
        "england",
        "England: London for the museums and the markets. Bath for the Roman baths. The Lake District for hills and walking.",
    ),
    (
        "morocco",
        "Morocco: Marrakech for the medina and the food. Fes for the crafts and the old city. Essaouira for the coast and the wind.",
    ),
    (
        "croatia",
        "Croatia: Split for a Roman palace and ferries to the islands. Dubrovnik for the walls, early before the crowds. Zadar for quieter evenings.",
    ),
    (
        "iceland",
        "Iceland: Reykjavik for the city. The south coast to Vik for waterfalls and black-sand beaches. The ring road takes longer than most people think.",
    ),
    (
        "czechia",
        "Czechia: Prague for the old town and the castle. Český Krumlov for a smaller, quieter base. Brno for wine and fewer tourists.",
    ),
    (
        "hungary",
        "Hungary: Budapest for the baths, the ruin bars and the parliament. The Danube bend if you want a break from the city. Lake Balaton in summer.",
    ),
    (
        "austria",
        "Austria: Vienna for the palaces and the coffee houses. Salzburg for the old town and the hills. Innsbruck for the Alps.",
    ),
    (
        "belgium",
        "Belgium: Brussels briefly, for the Grand Place and the museums. Bruges for the canals, and Ghent for the same with fewer people. Both are short train rides from Brussels.",
    ),
    (
        "denmark",
        "Denmark: Copenhagen for the city and the food. Aarhus for a second city. Malmö is a train ride over the bridge.",
    ),
    (
        "finland",
        "Finland: Helsinki for the design and the saunas. Lapland in winter for the snow and the northern lights. A summer cabin by a lake in the other season.",
    ),
    (
        "vietnam",
        "Vietnam: Hanoi for the old quarter and the food. Hoi An and the coast by train or a short flight south. Ha Long Bay for limestone islands and a boat.",
    ),
    (
        "france",
        "France: Paris for the museums and the food. Lyon for the best eating in the country. Provence for small towns and markets, and don't rush it.",
    ),
    (
        "greece",
        "Greece: Athens for the Acropolis and street food. Santorini for the view. Naxos for beaches and cheaper rooms.",
    ),
    (
        "turkey",
        "Turkey: Istanbul for mosques, bazaars and the ferries up the Bosphorus. Cappadocia for the balloon flights at sunrise. The coast if you want to swim.",
    ),
    (
        "norway",
        "Norway: Oslo for the museums and the opera house. The train to Bergen for the fjords, because the journey is half the point. Tromsø in winter for the northern lights.",
    ),
    (
        "mexico",
        "Mexico: Mexico City for the food and the museums. Oaxaca for markets and mezcal. Tulum if you mainly want the beach.",
    ),
    (
        "canada",
        "Canada: Toronto and Montreal for cities. Banff and Jasper for the Rockies. Vancouver if you want sea and mountains together.",
    ),
    (
        "poland",
        "Poland: Kraków for the old town, with Auschwitz as a sobering trip out. Warsaw for the rebuilt city and the museums. Gdańsk for the coast.",
    ),
    (
        "sweden",
        "Sweden: Stockholm for the islands and the museums. The archipelago in summer. Gothenburg for a second city.",
    ),
    (
        "brazil",
        "Brazil: Rio for the beaches and the views. The Pantanal for wildlife. Salvador for music and the old town.",
    ),
    (
        "spain",
        "Spain: Barcelona for a city and a beach in one trip. Seville for the Alcázar and the tapas. Madrid for the museums, with Toledo a short train ride away.",
    ),
    (
        "italy",
        "Italy: Rome for the ancient city and the Vatican. Florence for the art you can walk between. Naples and the Amalfi coast for pizza and sea.",
    ),
    (
        "japan",
        "Japan: Tokyo for the city and the food. Kyoto for temples and old streets. Osaka for eating, with a rail pass once you visit more than one.",
    ),
    (
        "india",
        "India: Delhi for the history. Jaipur and Jodhpur for Rajasthan. Kerala for the backwaters, and one region rather than criss-crossing the country.",
    ),
    (
        "egypt",
        "Egypt: Cairo for the pyramids and the museum. Luxor for the temples and the Valley of the Kings. Aswan for a slower stretch by the river.",
    ),
    (
        "china",
        "China: Beijing for the Wall and the Forbidden City. Xi'an for the terracotta army. Shanghai for a city you can walk.",
    ),
    (
        "dubai",
        "Dubai: A short trip: the old creek and the souks in the morning. The desert in the evening. One big mall, and the marina at night.",
    ),
    (
        "cuba",
        "Cuba: Havana for the old town and the cars. Viñales for tobacco country. Take cash, because cards barely work.",
    ),
];
pub(super) const ALIASES: &[(&str, &str)] = &[
    (
        "czech republic",
        "Czechia: Prague for the old town and the castle. Český Krumlov for a smaller, quieter base. Brno for wine and fewer tourists.",
    ),
    (
        "great britain",
        "England: London for the museums and the markets. Bath for the Roman baths. The Lake District for hills and walking.",
    ),
    (
        "the states",
        "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
    ),
    (
        "america",
        "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
    ),
    (
        "britain",
        "England: London for the museums and the markets. Bath for the Roman baths. The Lake District for hills and walking.",
    ),
    (
        "holland",
        "The Netherlands: Amsterdam for the canals and the museums. Haarlem or Delft as a quieter base. The country is small, so everywhere is easy to reach.",
    ),
    (
        "usa",
        "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
    ),
    (
        "uae",
        "Dubai: A short trip: the old creek and the souks in the morning. The desert in the evening. One big mall, and the marina at night.",
    ),
    (
        "us",
        "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
    ),
    (
        "uk",
        "England: London for the museums and the markets. Bath for the Roman baths. The Lake District for hills and walking.",
    ),
];
pub(super) struct City(pub &'static str, pub &'static str, pub usize);
pub(super) const CITIES: &[City] = &[
    City("copenhagen", "Copenhagen", 1),
    City("barcelona", "Barcelona", 0),
    City("amsterdam", "Amsterdam", 1),
    City("edinburgh", "Edinburgh", 1),
    City("stockholm", "Stockholm", 1),
    City("reykjavik", "Reykjavik", 1),
    City("florence", "Florence", 0),
    City("istanbul", "Istanbul", 0),
    City("new york", "New York", 1),
    City("budapest", "Budapest", 2),
    City("seville", "Seville", 0),
    City("lisbon", "Lisbon", 0),
    City("madrid", "Madrid", 0),
    City("athens", "Athens", 0),
    City("venice", "Venice", 0),
    City("london", "London", 1),
    City("dublin", "Dublin", 1),
    City("berlin", "Berlin", 1),
    City("boston", "Boston", 1),
    City("prague", "Prague", 2),
    City("vienna", "Vienna", 2),
    City("porto", "Porto", 0),
    City("paris", "Paris", 1),
    City("rome", "Rome", 0),
];
pub(super) const CLIMATES: &[[(i16, i16); 12]] = &[
    [
        (8, 15),
        (9, 16),
        (11, 18),
        (12, 20),
        (15, 24),
        (19, 29),
        (22, 32),
        (22, 32),
        (19, 28),
        (15, 23),
        (11, 18),
        (9, 15),
    ],
    [
        (1, 7),
        (1, 8),
        (3, 11),
        (5, 15),
        (8, 19),
        (11, 22),
        (13, 24),
        (13, 24),
        (11, 20),
        (8, 15),
        (4, 10),
        (2, 7),
    ],
    [
        (0, 4),
        (-1, 5),
        (3, 10),
        (8, 17),
        (13, 23),
        (17, 27),
        (19, 29),
        (18, 28),
        (14, 23),
        (8, 16),
        (3, 9),
        (1, 5),
    ],
];
pub(super) const MONTHS: &[(&str, usize)] = &[
    ("september", 8),
    ("february", 1),
    ("november", 10),
    ("december", 11),
    ("january", 0),
    ("october", 9),
    ("august", 7),
    ("march", 2),
    ("april", 3),
    ("june", 5),
    ("july", 6),
    ("sept", 8),
    ("may", 4),
    ("jan", 0),
    ("feb", 1),
    ("mar", 2),
    ("apr", 3),
    ("jun", 5),
    ("jul", 6),
    ("aug", 7),
    ("sep", 8),
    ("oct", 9),
    ("nov", 10),
    ("dec", 11),
];
pub(super) const LONG_MONTHS: &[&str] = &[
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
