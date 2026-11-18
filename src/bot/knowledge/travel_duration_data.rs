//! Pinned stay-length guidance and the source ambiguity fences.

pub(super) const A_JOURNEY_OR_A_BAG: &str = r#"\b(?:flight|flights|fly|flying|train|trains|bus|coach|drive|driving|journey|walk|walking|ferry|transfer|queue|wait|pack|packing|clothes|suitcase|luggage|case)\b"#;
pub(super) const ASKS_A_LENGTH: &str = r#"(?i)\b(?:how long|how many days|how many nights|how much time|enough time|too long|too short|days in|nights in)\b"#;
pub(super) const COUNTS_AS_A_THING: &str = r#"(?i)\b(?:a|an|some|any|another|one|no|every|this|that|these|those|my|your|our|their|his|her|its|roast|roasted|frozen|whole|raw|stuffed|cooked|leftover|defrosted|fresh|smoked)\s+$"#;
pub(super) const COUNTS_AS_A_TRIP: &str = r#"(?i)\s+(?:holiday|holidays|vacation|vacations|trip|trips|break|breaks|getaway|getaways|week|weeks|weekend|weekends|fortnight|resort|resorts|flight|flights|hotel|hotels|hostel|hostels|tour|tours|guide|guides|map|maps|visa|visas|beach|beaches|city|coast|island|islands|airport|airports|transfer|transfers|excursion|excursions|adventure|adventures)\b"#;
pub(super) const ALSO_A_THING: &[&str] = &["chile", "china", "guinea", "jersey", "turkey"];
pub(super) const CITIES: &[(&str, &str)] = &[
    (
        "rio de janeiro",
        "Three full days in Rio De Janeiro, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "san francisco",
        "Two full days in San Francisco, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "san sebastian",
        "Two full days in San Sebastian, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "buenos aires",
        "Three full days in Buenos Aires, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "los angeles",
        "Three full days in Los Angeles, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "mexico city",
        "Three full days in Mexico City, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "copenhagen",
        "Two full days in Copenhagen, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "amsterdam",
        "Two full days in Amsterdam, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "barcelona",
        "Three full days in Barcelona, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "cape town",
        "Four full days in Cape Town, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "dubrovnik",
        "Two full days in Dubrovnik, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "edinburgh",
        "Two full days in Edinburgh, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "ljubljana",
        "One full day in Ljubljana, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "marrakech",
        "Two full days in Marrakech, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "reykjavik",
        "Three full days in Reykjavik, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "santorini",
        "Two full days in Santorini, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "singapore",
        "Three full days in Singapore, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "stockholm",
        "Two full days in Stockholm, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "brussels",
        "Two full days in Brussels, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "budapest",
        "Two full days in Budapest, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "florence",
        "Two full days in Florence, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "helsinki",
        "Two full days in Helsinki, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "istanbul",
        "Three full days in Istanbul, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "new york",
        "Four full days in New York, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "valencia",
        "Two full days in Valencia, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "bangkok",
        "Three full days in Bangkok, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "chicago",
        "Two full days in Chicago, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "granada",
        "Two full days in Granada, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "seville",
        "Two full days in Seville, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "tallinn",
        "One full day in Tallinn, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "toronto",
        "Two full days in Toronto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "athens",
        "Two full days in Athens, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "berlin",
        "Three full days in Berlin, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "boston",
        "Two full days in Boston, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "dublin",
        "Two full days in Dublin, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "havana",
        "Two full days in Havana, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "krakow",
        "Two full days in Krakow, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "lisbon",
        "Three full days in Lisbon, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "london",
        "Four full days in London, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "madrid",
        "Three full days in Madrid, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "munich",
        "Two full days in Munich, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "naples",
        "Two full days in Naples, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "prague",
        "Three full days in Prague, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "sydney",
        "Three full days in Sydney, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "venice",
        "Two full days in Venice, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "vienna",
        "Three full days in Vienna, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "zurich",
        "Two full days in Zurich, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "dubai",
        "Three full days in Dubai, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "hanoi",
        "Three full days in Hanoi, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "kyoto",
        "Three full days in Kyoto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "milan",
        "Two full days in Milan, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "paris",
        "Four full days in Paris, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "porto",
        "Two full days in Porto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "tokyo",
        "Four full days in Tokyo, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "bath",
        "One full day in Bath, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "oslo",
        "Two full days in Oslo, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
    (
        "rome",
        "Three full days in Rome, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
    ),
];
pub(super) struct Country(pub &'static str, pub Option<&'static str>);
pub(super) const COUNTRIES: &[Country] = &[
    Country(
        "united states",
        Some(
            "The States: New York four full days, San Francisco two, so six days in all with the journey between them, and a day or two more for Chicago.",
        ),
    ),
    Country(
        "netherlands",
        Some(
            "Two full days in Amsterdam, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "switzerland",
        Some(
            "Two full days in Zurich, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country("new zealand", None),
    Country(
        "australia",
        Some(
            "Three full days in Sydney, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country("singapore", None),
    Country("cape town", None),
    Country(
        "argentina",
        Some(
            "Three full days in Buenos Aires, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "portugal",
        Some(
            "Portugal: Lisbon three full days, Porto two, so five days in all with the journey between them.",
        ),
    ),
    Country(
        "scotland",
        Some(
            "Two full days in Edinburgh, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "thailand",
        Some(
            "Three full days in Bangkok, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "germany",
        Some(
            "Germany: Berlin three full days, Munich two, so five days in all with the journey between them.",
        ),
    ),
    Country(
        "ireland",
        Some(
            "Two full days in Dublin, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "england",
        Some(
            "England: London four full days, Bath one, so five days in all with the journey between them.",
        ),
    ),
    Country(
        "morocco",
        Some(
            "Two full days in Marrakech, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "croatia",
        Some(
            "Two full days in Dubrovnik, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "iceland",
        Some(
            "Three full days in Reykjavik, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "czechia",
        Some(
            "Three full days in Prague, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "hungary",
        Some(
            "Two full days in Budapest, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "austria",
        Some(
            "Three full days in Vienna, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "belgium",
        Some(
            "Two full days in Brussels, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "denmark",
        Some(
            "Two full days in Copenhagen, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "finland",
        Some(
            "Two full days in Helsinki, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "vietnam",
        Some(
            "Three full days in Hanoi, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "france",
        Some(
            "Four full days in Paris, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "greece",
        Some(
            "Greece: Athens two full days, Santorini two, so four days in all with the journey between them.",
        ),
    ),
    Country(
        "turkey",
        Some(
            "Three full days in Istanbul, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "norway",
        Some(
            "Two full days in Oslo, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "mexico",
        Some(
            "Three full days in Mexico City, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country(
        "canada",
        Some(
            "Two full days in Toronto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country("poland", None),
    Country(
        "sweden",
        Some(
            "Two full days in Stockholm, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
    Country("brazil", None),
    Country(
        "spain",
        Some(
            "Spain: Barcelona three full days, Seville two, so five days in all with the journey between them, and a day or two more for Madrid.",
        ),
    ),
    Country(
        "italy",
        Some(
            "Italy: Rome three full days, Florence two, so five days in all with the journey between them, and a day or two more for Naples.",
        ),
    ),
    Country(
        "japan",
        Some(
            "Japan: Tokyo four full days, Kyoto three, so seven days in all with the journey between them.",
        ),
    ),
    Country("india", None),
    Country("egypt", None),
    Country("china", None),
    Country("dubai", None),
    Country(
        "cuba",
        Some(
            "Two full days in Havana, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
    ),
];
