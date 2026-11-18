//! Original stay-duration callbacks with enough context to determine the place.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub Option<&'static str>,
    pub Option<bool>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "city-stay",
        "how long should i stay in rome",
        &[],
        Some(
            "Three full days in Rome, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days in lisbon",
        &[],
        Some(
            "Three full days in Lisbon, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "is five days in lisbon too long",
        &[],
        Some(
            "Three full days in Lisbon, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "i have four days in lisbon",
        &[],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "is four days enough in lisbon",
        &[],
        Some(
            "Three full days in Lisbon, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how long should i stay at home",
        &[],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how much time is enough for rome",
        &[],
        Some(
            "Three full days in Rome, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how long is the flight to rome",
        &[],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days should i stay in bath",
        &[],
        Some(
            "One full day in Bath, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "what is there to see in bath",
        &[],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how long do i need to pack for venice",
        &[],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how much time should i allow for florence",
        &[],
        Some(
            "Two full days in Florence, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how long do i need to spend in copenhagen",
        &[],
        Some(
            "Two full days in Copenhagen, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case("city-stay", "how long for a turkey", &[], None, Some(true)),
    Case(
        "city-stay",
        "how long for a 5 kg turkey",
        &[],
        Some(
            "Three full days in Istanbul, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(false),
    ),
    Case(
        "city-stay",
        "how long do i need at the museum in paris",
        &[],
        Some(
            "Four full days in Paris, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "would nine days be too long in berlin",
        &[],
        Some(
            "Three full days in Berlin, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "is the bus to rome worth it",
        &[],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how long should i spend seeing athens",
        &[],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how much time for washington dc",
        &[],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how long do i need in japan",
        &[],
        Some(
            "Japan: Tokyo four full days, Kyoto three, so seven days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how much time should we spend in australia",
        &[],
        Some(
            "Three full days in Sydney, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days do i need in portugal",
        &[],
        Some(
            "Portugal: Lisbon three full days, Porto two, so five days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long to stay in spain",
        &[],
        Some(
            "Spain: Barcelona three full days, Seville two, so five days in all with the journey between them, and a day or two more for Madrid.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days do i need in a spain",
        &[],
        Some(
            "Spain: Barcelona three full days, Seville two, so five days in all with the journey between them, and a day or two more for Madrid.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how much time should i spend in italy",
        &[],
        Some(
            "Italy: Rome three full days, Florence two, so five days in all with the journey between them, and a day or two more for Naples.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many nights do i need in japan",
        &[],
        Some(
            "Japan: Tokyo four full days, Kyoto three, so seven days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long should i stay in portugal to see lisbon",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "do you need several days in scotland",
        &[],
        Some(
            "Two full days in Edinburgh, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how much time for italy without driving",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days in the usa",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long in a turkey",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long in turkey",
        &[],
        Some(
            "Three full days in Istanbul, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long for a turkey and a trip in turkey",
        &[],
        Some(
            "Three full days in Istanbul, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long for a china bowl",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how much time should i spend in greece",
        &[],
        Some(
            "Greece: Athens two full days, Santorini two, so four days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days do i need in france",
        &[],
        Some(
            "Four full days in Paris, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "what is the time in belgium",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "we live in belgium and how long should we stay",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should you visit a greece",
        &[],
        Some(
            "Greece: Athens two full days, Santorini two, so four days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how much time in germany",
        &[],
        Some(
            "Germany: Berlin three full days, Munich two, so five days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days in norway",
        &[],
        Some(
            "Two full days in Oslo, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long to see the city in england",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many weeks should we go to vietnam",
        &[],
        Some(
            "Three full days in Hanoi, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how much time is enough in thailand",
        &[],
        Some(
            "Three full days in Bangkok, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days in the car for france",
        &[],
        Some(
            "Four full days in Paris, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay-followup",
        "how long should i stay there",
        &[],
        None,
        None,
    ),
    Case(
        "city-stay-followup",
        "how many days in rome",
        &[],
        None,
        None,
    ),
    Case(
        "city-stay-followup",
        "how long would i need there",
        &[],
        None,
        None,
    ),
    Case(
        "city-stay-followup",
        "should we stay there for six days",
        &[],
        None,
        None,
    ),
    Case("city-stay-followup", "is it enough time", &[], None, None),
    Case(
        "city-stay-followup",
        "how long does it take there",
        &[],
        None,
        None,
    ),
    Case(
        "city-stay-followup",
        "how many nights do we need there",
        &[],
        None,
        None,
    ),
    Case(
        "city-stay",
        "how long do i need in rome",
        &["i live in rome"],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how long should i stay in bath",
        &["i live near leeds"],
        Some(
            "One full day in Bath, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how long should i stay in edinburgh",
        &["i live in edinburgh"],
        None,
        Some(true),
    ),
    Case(
        "city-stay",
        "how long do i need there",
        &["i visited edinburgh last year"],
        Some(
            "Two full days in Edinburgh, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days are enough there",
        &["we're going to lisbon in may"],
        Some(
            "Three full days in Lisbon, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how much time is enough there",
        &[
            "we're going to lisbon in may",
            "we thought of spending a day in porto",
        ],
        Some(
            "Two full days in Porto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should we spend in spain",
        &["we went to madrid last year"],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long should we spend there",
        &["we're going to spain"],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how long should we spend in turkey",
        &["i live in turkey"],
        Some(
            "Three full days in Istanbul, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay-followup",
        "how long should we stay there",
        &["we are going to edinburgh"],
        Some(
            "Two full days in Edinburgh, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        None,
    ),
    Case(
        "city-stay-followup",
        "how long should we stay there",
        &["i live in edinburgh"],
        None,
        None,
    ),
    Case(
        "city-stay-followup",
        "what about a night there",
        &["we're planning a trip to paris"],
        Some(
            "Four full days in Paris, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        None,
    ),
    Case(
        "city-stay-followup",
        "how long do we need there",
        &["should we visit athens"],
        Some(
            "Two full days in Athens, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        None,
    ),
    Case(
        "city-stay-followup",
        "how long do we need there",
        &["we might book athens", "who is going"],
        Some(
            "Two full days in Athens, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        None,
    ),
    Case(
        "city-stay-followup",
        "how many days",
        &["i work in lisbon"],
        None,
        None,
    ),
    Case(
        "city-stay-followup",
        "how long there",
        &["we are visiting cork next week", "what is a bottle cork"],
        None,
        None,
    ),
    Case(
        "city-stay-followup",
        "how long there",
        &["we are visiting cork next week", "yes please", "thanks"],
        None,
        None,
    ),
    Case(
        "city-stay",
        "how many days do i need in amsterdam",
        &[],
        Some(
            "Two full days in Amsterdam, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in athens",
        &[],
        Some(
            "Two full days in Athens, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in bangkok",
        &[],
        Some(
            "Three full days in Bangkok, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in barcelona",
        &[],
        Some(
            "Three full days in Barcelona, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in bath",
        &[],
        Some(
            "One full day in Bath, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in berlin",
        &[],
        Some(
            "Three full days in Berlin, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in boston",
        &[],
        Some(
            "Two full days in Boston, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in brussels",
        &[],
        Some(
            "Two full days in Brussels, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in budapest",
        &[],
        Some(
            "Two full days in Budapest, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in buenos aires",
        &[],
        Some(
            "Three full days in Buenos Aires, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in cape town",
        &[],
        Some(
            "Four full days in Cape Town, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in chicago",
        &[],
        Some(
            "Two full days in Chicago, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in copenhagen",
        &[],
        Some(
            "Two full days in Copenhagen, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in dubai",
        &[],
        Some(
            "Three full days in Dubai, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in dublin",
        &[],
        Some(
            "Two full days in Dublin, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in dubrovnik",
        &[],
        Some(
            "Two full days in Dubrovnik, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in edinburgh",
        &[],
        Some(
            "Two full days in Edinburgh, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in florence",
        &[],
        Some(
            "Two full days in Florence, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in granada",
        &[],
        Some(
            "Two full days in Granada, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in hanoi",
        &[],
        Some(
            "Three full days in Hanoi, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in havana",
        &[],
        Some(
            "Two full days in Havana, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in helsinki",
        &[],
        Some(
            "Two full days in Helsinki, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in istanbul",
        &[],
        Some(
            "Three full days in Istanbul, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in krakow",
        &[],
        Some(
            "Two full days in Krakow, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in kyoto",
        &[],
        Some(
            "Three full days in Kyoto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in lisbon",
        &[],
        Some(
            "Three full days in Lisbon, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in ljubljana",
        &[],
        Some(
            "One full day in Ljubljana, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in london",
        &[],
        Some(
            "Four full days in London, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in los angeles",
        &[],
        Some(
            "Three full days in Los Angeles, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in madrid",
        &[],
        Some(
            "Three full days in Madrid, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in marrakech",
        &[],
        Some(
            "Two full days in Marrakech, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in mexico city",
        &[],
        Some(
            "Three full days in Mexico City, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in milan",
        &[],
        Some(
            "Two full days in Milan, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in munich",
        &[],
        Some(
            "Two full days in Munich, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in naples",
        &[],
        Some(
            "Two full days in Naples, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in new york",
        &[],
        Some(
            "Four full days in New York, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in oslo",
        &[],
        Some(
            "Two full days in Oslo, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in paris",
        &[],
        Some(
            "Four full days in Paris, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in porto",
        &[],
        Some(
            "Two full days in Porto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in prague",
        &[],
        Some(
            "Three full days in Prague, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in reykjavik",
        &[],
        Some(
            "Three full days in Reykjavik, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in rio de janeiro",
        &[],
        Some(
            "Three full days in Rio De Janeiro, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in rome",
        &[],
        Some(
            "Three full days in Rome, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in san francisco",
        &[],
        Some(
            "Two full days in San Francisco, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in san sebastian",
        &[],
        Some(
            "Two full days in San Sebastian, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in santorini",
        &[],
        Some(
            "Two full days in Santorini, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in seville",
        &[],
        Some(
            "Two full days in Seville, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in singapore",
        &[],
        Some(
            "Three full days in Singapore, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in stockholm",
        &[],
        Some(
            "Two full days in Stockholm, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in sydney",
        &[],
        Some(
            "Three full days in Sydney, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in tallinn",
        &[],
        Some(
            "One full day in Tallinn, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in tokyo",
        &[],
        Some(
            "Four full days in Tokyo, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in toronto",
        &[],
        Some(
            "Two full days in Toronto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in valencia",
        &[],
        Some(
            "Two full days in Valencia, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in venice",
        &[],
        Some(
            "Two full days in Venice, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in vienna",
        &[],
        Some(
            "Three full days in Vienna, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "city-stay",
        "how many days do i need in zurich",
        &[],
        Some(
            "Two full days in Zurich, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in spain",
        &[],
        Some(
            "Spain: Barcelona three full days, Seville two, so five days in all with the journey between them, and a day or two more for Madrid.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in italy",
        &[],
        Some(
            "Italy: Rome three full days, Florence two, so five days in all with the journey between them, and a day or two more for Naples.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in france",
        &[],
        Some(
            "Four full days in Paris, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in portugal",
        &[],
        Some(
            "Portugal: Lisbon three full days, Porto two, so five days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in greece",
        &[],
        Some(
            "Greece: Athens two full days, Santorini two, so four days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in japan",
        &[],
        Some(
            "Japan: Tokyo four full days, Kyoto three, so seven days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in netherlands",
        &[],
        Some(
            "Two full days in Amsterdam, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in germany",
        &[],
        Some(
            "Germany: Berlin three full days, Munich two, so five days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in ireland",
        &[],
        Some(
            "Two full days in Dublin, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in scotland",
        &[],
        Some(
            "Two full days in Edinburgh, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in england",
        &[],
        Some(
            "England: London four full days, Bath one, so five days in all with the journey between them.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in turkey",
        &[],
        Some(
            "Three full days in Istanbul, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in morocco",
        &[],
        Some(
            "Two full days in Marrakech, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in croatia",
        &[],
        Some(
            "Two full days in Dubrovnik, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in iceland",
        &[],
        Some(
            "Three full days in Reykjavik, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in norway",
        &[],
        Some(
            "Two full days in Oslo, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in thailand",
        &[],
        Some(
            "Three full days in Bangkok, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in mexico",
        &[],
        Some(
            "Three full days in Mexico City, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in united states",
        &[],
        Some(
            "The States: New York four full days, San Francisco two, so six days in all with the journey between them, and a day or two more for Chicago.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in india",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in australia",
        &[],
        Some(
            "Three full days in Sydney, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in egypt",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in canada",
        &[],
        Some(
            "Two full days in Toronto, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in czechia",
        &[],
        Some(
            "Three full days in Prague, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in hungary",
        &[],
        Some(
            "Two full days in Budapest, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in poland",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in austria",
        &[],
        Some(
            "Three full days in Vienna, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in switzerland",
        &[],
        Some(
            "Two full days in Zurich, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in belgium",
        &[],
        Some(
            "Two full days in Brussels, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in sweden",
        &[],
        Some(
            "Two full days in Stockholm, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in denmark",
        &[],
        Some(
            "Two full days in Copenhagen, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in finland",
        &[],
        Some(
            "Two full days in Helsinki, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in new zealand",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in china",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in vietnam",
        &[],
        Some(
            "Three full days in Hanoi, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in dubai",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in singapore",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in cape town",
        &[],
        None,
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in cuba",
        &[],
        Some(
            "Two full days in Havana, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in argentina",
        &[],
        Some(
            "Three full days in Buenos Aires, not counting the day you arrive or the day you fly home: that covers the main sights at a normal pace, with time to sit down for lunch, and a day more if you want one of the day trips.",
        ),
        Some(true),
    ),
    Case(
        "country-stay-length",
        "how many days should i spend in brazil",
        &[],
        None,
        Some(true),
    ),
];
