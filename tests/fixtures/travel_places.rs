//! Direct source fixtures for country suggestions and average city climate.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "country-trip",
        "we are going to greece in july",
        &[],
        Some(
            "Greece: Athens for the Acropolis and street food. Santorini for the view. Naxos for beaches and cheaper rooms.",
        ),
    ),
    Case(
        "country-trip",
        "we're off to canada in december",
        &[],
        Some(
            "Canada: Toronto and Montreal for cities. Banff and Jasper for the Rockies. Vancouver if you want sea and mountains together.",
        ),
    ),
    Case(
        "country-trip",
        "we are going to the us in october",
        &[],
        Some(
            "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
        ),
    ),
    Case(
        "country-trip",
        "we are going to usa",
        &[],
        Some(
            "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
        ),
    ),
    Case(
        "country-trip",
        "going to new zealand",
        &[],
        Some(
            "New Zealand: Queenstown and the southern lakes for the mountains. The North Island for beaches and volcanoes. Pick one island rather than trying both.",
        ),
    ),
    Case(
        "country-trip",
        "we are off to holland",
        &[],
        Some(
            "The Netherlands: Amsterdam for the canals and the museums. Haarlem or Delft as a quieter base. The country is small, so everywhere is easy to reach.",
        ),
    ),
    Case(
        "country-trip",
        "we are going to the uk",
        &[],
        Some(
            "England: London for the museums and the markets. Bath for the Roman baths. The Lake District for hills and walking.",
        ),
    ),
    Case(
        "country-trip",
        "lets go to portugal",
        &[],
        Some(
            "Portugal: Lisbon for trams, tarts and the river. Porto for the port cellars and the bookshop. The Algarve for beaches and cliffs.",
        ),
    ),
    Case(
        "country-trip",
        "were going to brazil",
        &[],
        Some(
            "Brazil: Rio for the beaches and the views. The Pantanal for wildlife. Salvador for music and the old town.",
        ),
    ),
    Case(
        "country-trip",
        "are we going to mexico or spain",
        &[],
        Some(
            "Mexico: Mexico City for the food and the museums. Oaxaca for markets and mezcal. Tulum if you mainly want the beach.",
        ),
    ),
    Case(
        "country-trip",
        "we are travelling to turkey",
        &[],
        Some(
            "Turkey: Istanbul for mosques, bazaars and the ferries up the Bosphorus. Cappadocia for the balloon flights at sunrise. The coast if you want to swim.",
        ),
    ),
    Case("country-trip", "what about us", &[], None),
    Case(
        "country-where-to-go",
        "where should i go in spain",
        &[],
        Some(
            "Spain: Barcelona for a city and a beach in one trip. Seville for the Alcázar and the tapas. Madrid for the museums, with Toledo a short train ride away.",
        ),
    ),
    Case(
        "country-where-to-go",
        "which places should i go in japan",
        &[],
        Some(
            "Japan: Tokyo for the city and the food. Kyoto for temples and old streets. Osaka for eating, with a rail pass once you visit more than one.",
        ),
    ),
    Case(
        "country-where-to-go",
        "what about canada",
        &[],
        Some(
            "Canada: Toronto and Montreal for cities. Banff and Jasper for the Rockies. Vancouver if you want sea and mountains together.",
        ),
    ),
    Case(
        "country-where-to-go",
        "what about austrian towns",
        &[],
        None,
    ),
    Case(
        "country-where-to-go",
        "where should i go in the united states",
        &[],
        Some(
            "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
        ),
    ),
    Case(
        "country-where-to-go",
        "want to see china",
        &[],
        Some(
            "China: Beijing for the Wall and the Forbidden City. Xi'an for the terracotta army. Shanghai for a city you can walk.",
        ),
    ),
    Case(
        "country-where-to-go",
        "i want to buy a china bowl",
        &[],
        None,
    ),
    Case(
        "country-where-to-go",
        "what are best places to see in turkey",
        &[],
        Some(
            "Turkey: Istanbul for mosques, bazaars and the ferries up the Bosphorus. Cappadocia for the balloon flights at sunrise. The coast if you want to swim.",
        ),
    ),
    Case("country-where-to-go", "for a turkey", &[], None),
    Case("country-where-to-go", "my turkey weighs 5 kg", &[], None),
    Case(
        "country-where-to-go",
        "how long in a turkey holiday",
        &[],
        Some(
            "Turkey: Istanbul for mosques, bazaars and the ferries up the Bosphorus. Cappadocia for the balloon flights at sunrise. The coast if you want to swim.",
        ),
    ),
    Case("country-where-to-go", "my guinea pig is quiet", &[], None),
    Case(
        "country-where-to-go",
        "is a greece a good place to go",
        &[],
        Some(
            "Greece: Athens for the Acropolis and street food. Santorini for the view. Naxos for beaches and cheaper rooms.",
        ),
    ),
    Case(
        "country-where-to-go",
        "what about belgium or croatia",
        &[],
        Some(
            "Croatia: Split for a Roman palace and ferries to the islands. Dubrovnik for the walls, early before the crowds. Zadar for quieter evenings.",
        ),
    ),
    Case(
        "country-where-to-go",
        "what about the usa",
        &[],
        Some(
            "The States: New York for a first trip. San Francisco and the coast for California. Chicago for food and the lake in summer.",
        ),
    ),
    Case(
        "country-where-to-go",
        "best places to see in iberia",
        &[],
        None,
    ),
    Case(
        "country-where-to-go",
        "what to visit in deutschland",
        &[],
        None,
    ),
    Case(
        "city-weather",
        "what is the weather like in lisbon in may",
        &[],
        Some(
            "May in Lisbon is usually about 15 to 24C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "can you see the weather today in rome",
        &[],
        None,
    ),
    Case(
        "city-weather",
        "is it warm in athens in dec",
        &[],
        Some(
            "December in Athens is usually about 9 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case("city-weather", "is it cold in nyc in november", &[], None),
    Case(
        "city-weather",
        "how cold is it in tokyo in february",
        &[],
        None,
    ),
    Case("city-weather", "is it warm in cork this january", &[], None),
    Case("city-weather", "weather in bath in may", &[], None),
    Case(
        "city-weather",
        "how is the weather in split in feb",
        &[],
        None,
    ),
    Case(
        "city-weather",
        "is it hot in marrakech in summer",
        &[],
        None,
    ),
    Case(
        "city-weather",
        "temperature in rome in may or july",
        &[],
        Some(
            "July in Rome is usually about 22 to 32C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case("city-weather", "where is cool near madrid", &[], None),
    Case(
        "city-weather",
        "whats the weather in las vegas in july",
        &[],
        None,
    ),
    Case(
        "city-weather",
        "temperature in jakarta in september",
        &[],
        None,
    ),
    Case(
        "city-weather",
        "florence in october weather",
        &[],
        Some(
            "October in Florence is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "how cool is it in tokyo in mayfair",
        &[],
        None,
    ),
    Case("city-weather", "weather in york april", &[], None),
    Case(
        "city-weather",
        "what weather in new york may i ask",
        &[],
        Some(
            "May in New York is usually about 8 to 19C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case("city-weather", "is it too hot there in april", &[], None),
    Case(
        "country-where-to-go",
        "and what about canada",
        &["what is the capital of australia"],
        None,
    ),
    Case(
        "country-where-to-go",
        "and what about canada",
        &["what is the capital of Australia"],
        Some(
            "Canada: Toronto and Montreal for cities. Banff and Jasper for the Rockies. Vancouver if you want sea and mountains together.",
        ),
    ),
    Case(
        "country-where-to-go",
        "and what about australia",
        &["what is the capital of australia"],
        Some(
            "Australia: Sydney for the harbour and the beaches. Melbourne for food, with the Great Ocean Road from there. Flying between the cities is normal.",
        ),
    ),
    Case(
        "country-where-to-go",
        "what can i see in canada",
        &["what is the capital of australia"],
        None,
    ),
    Case(
        "country-where-to-go",
        "is it worth a trip to canada",
        &["what is the capital of australia"],
        Some(
            "Canada: Toronto and Montreal for cities. Banff and Jasper for the Rockies. Vancouver if you want sea and mountains together.",
        ),
    ),
    Case(
        "country-where-to-go",
        "and what about canada",
        &["the capital of australia is where our trip ends"],
        Some(
            "Canada: Toronto and Montreal for cities. Banff and Jasper for the Rockies. Vancouver if you want sea and mountains together.",
        ),
    ),
    Case(
        "country-where-to-go",
        "and what about canada",
        &["my sister lives in australia"],
        Some(
            "Canada: Toronto and Montreal for cities. Banff and Jasper for the Rockies. Vancouver if you want sea and mountains together.",
        ),
    ),
    Case(
        "city-weather",
        "is it too cold there in april",
        &["i am planning a trip to edinburgh"],
        Some(
            "April in Edinburgh is usually about 5 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "what is the weather like",
        &["im going to lisbon in march", "thanks"],
        Some(
            "March in Lisbon is usually about 11 to 18C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "is it too hot there",
        &[
            "im visiting tokyo in december",
            "what should i eat",
            "thanks",
        ],
        None,
    ),
    Case(
        "city-weather",
        "does april get cold",
        &[
            "i'm planning to visit rome in june",
            "10 days",
            "thanks",
            "all of us",
        ],
        Some(
            "April in Rome is usually about 12 to 20C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "is it too hot there in may",
        &[
            "i am going to madrid in august",
            "thanks",
            "how long",
            "do i need an adaptor",
        ],
        Some(
            "May in Madrid is usually about 15 to 24C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "is it too hot there",
        &[
            "i am going to madrid in august",
            "thanks",
            "how long",
            "do i need an adaptor",
            "it all sounds good",
        ],
        None,
    ),
    Case(
        "city-weather",
        "what weather when we go",
        &[
            "i'm taking my wife to paris in august",
            "sorry may",
            "great",
        ],
        Some(
            "May in Paris is usually about 8 to 19C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in lisbon in october",
        &[],
        Some(
            "October in Lisbon is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in rome in october",
        &[],
        Some(
            "October in Rome is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in florence in october",
        &[],
        Some(
            "October in Florence is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in barcelona in october",
        &[],
        Some(
            "October in Barcelona is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in madrid in october",
        &[],
        Some(
            "October in Madrid is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in athens in october",
        &[],
        Some(
            "October in Athens is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in seville in october",
        &[],
        Some(
            "October in Seville is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in istanbul in october",
        &[],
        Some(
            "October in Istanbul is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in venice in october",
        &[],
        Some(
            "October in Venice is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in porto in october",
        &[],
        Some(
            "October in Porto is usually about 15 to 23C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in paris in october",
        &[],
        Some(
            "October in Paris is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in amsterdam in october",
        &[],
        Some(
            "October in Amsterdam is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in london in october",
        &[],
        Some(
            "October in London is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in edinburgh in october",
        &[],
        Some(
            "October in Edinburgh is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in dublin in october",
        &[],
        Some(
            "October in Dublin is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in berlin in october",
        &[],
        Some(
            "October in Berlin is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in copenhagen in october",
        &[],
        Some(
            "October in Copenhagen is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in stockholm in october",
        &[],
        Some(
            "October in Stockholm is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in reykjavik in october",
        &[],
        Some(
            "October in Reykjavik is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in new york in october",
        &[],
        Some(
            "October in New York is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in boston in october",
        &[],
        Some(
            "October in Boston is usually about 8 to 15C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in prague in october",
        &[],
        Some(
            "October in Prague is usually about 8 to 16C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in vienna in october",
        &[],
        Some(
            "October in Vienna is usually about 8 to 16C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
    Case(
        "city-weather",
        "whats the weather like in budapest in october",
        &[],
        Some(
            "October in Budapest is usually about 8 to 16C, and I can't see a live forecast from here. Take a layer either way: evenings are cooler than the afternoon, and a light rain jacket covers most days.",
        ),
    ),
];
