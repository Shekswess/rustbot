//! Direct reference cases for plans that carry a city or month forward.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "city-best-time",
        "best month to visit lisbon",
        &[],
        Some(
            "For Lisbon: May, June and September: warm and dry with fewer people. July and August are hot and busy with the tourists, and November to February is wet, though milder than the north.",
        ),
    ),
    Case("city-best-time", "is lisbon worth a visit", &[], None),
    Case(
        "city-best-time",
        "is lisbon worth visiting in may",
        &[],
        Some(
            "For Lisbon: May, June and September: warm and dry with fewer people. July and August are hot and busy with the tourists, and November to February is wet, though milder than the north.",
        ),
    ),
    Case(
        "city-best-time",
        "is lisbon worth visiting may i ask",
        &[],
        None,
    ),
    Case("city-best-time", "best time to go to bali", &[], None),
    Case(
        "city-best-time",
        "when should i visit scotland",
        &[],
        Some(
            "For Scotland: May to September for the Highlands, the islands and the long evenings. Midges are worst in July and August, and November to February has very short days.",
        ),
    ),
    Case(
        "city-best-time",
        "is scotland good in winter",
        &[],
        Some(
            "For Scotland: May to September for the Highlands, the islands and the long evenings. Midges are worst in July and August, and November to February has very short days.",
        ),
    ),
    Case(
        "city-best-time",
        "best time of year for the lakes",
        &[],
        None,
    ),
    Case(
        "city-best-time",
        "i want to go to the lake district or the highlands when",
        &[],
        Some(
            "For Lake District: May, June and September for long clear days at 15-20C; September is the quietest dry-ish month and the fells are at their best. July and August bring the crowds to Windermere and the passes, and November to February is wet, dark and often icy on the tops.",
        ),
    ),
    Case(
        "city-best-time",
        "when is the best time to book a long haul trip",
        &[],
        Some(
            "Book six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
        ),
    ),
    Case(
        "city-best-time",
        "when should i book the restaurant in rome",
        &[],
        Some(
            "For Rome: April, May, late September and October: 20-26C and the ruins walkable. July and August are 30C+ with the fountains as the only relief, and August means half the restaurants shut.",
        ),
    ),
    Case(
        "city-best-time",
        "when should i book a hire car in portugal",
        &[],
        Some(
            "For Portugal: May, June and September: warm and dry with fewer people. July and August are hot and busy on the Algarve, and the north is wet from November to February.",
        ),
    ),
    Case(
        "city-best-time",
        "how early do i book flights",
        &[],
        Some(
            "Book six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
        ),
    ),
    Case(
        "city-best-time",
        "whats the best time of year for paris",
        &[],
        Some(
            "For Paris: April to June and September to early October: 18-24C and the parks at their best. July and August are hot and many small restaurants close, and November to February is grey and cool.",
        ),
    ),
    Case("city-best-time", "when is it best for a bath", &[], None),
    Case(
        "city-best-time",
        "when to visit bath",
        &[],
        Some(
            "For Bath: May to September for the baths, the crescent and the canal; April and October are quieter and still fine. December is the Christmas market, which is the busiest weekend of the year.",
        ),
    ),
    Case("city-best-time", "when to go to cork", &[], None),
    Case(
        "city-best-time",
        "is split worth visiting in november",
        &[],
        Some(
            "For Split: May, June, September and early October: 22-27C, with the ferries to Hvar and Brac running. July and August are the peak and the old town is packed by day, and many islands shut from October.",
        ),
    ),
    Case(
        "city-best-time",
        "is it worth visiting athens in high summer",
        &[],
        Some(
            "For Athens: April, May, October and early November: 20-26C and the sites walkable. July and August are 35C+ and the Acropolis is a scramble by 10am, and the islands are at their best in June and September.",
        ),
    ),
    Case(
        "city-best-time",
        "is it worth visiting athens may i ask",
        &[],
        None,
    ),
    Case(
        "city-best-time",
        "march to june is lisbon worth it",
        &[],
        Some(
            "For Lisbon: May, June and September: warm and dry with fewer people. July and August are hot and busy with the tourists, and November to February is wet, though milder than the north.",
        ),
    ),
    Case(
        "city-transit",
        "how do i get around paris",
        &[],
        Some(
            "In Paris: Get a Navigo Easy card and load ten tickets: a single is €2.15 and ten is cheaper, and it covers the Metro inside the city. Keep the ticket until you are out of the gate, because inspectors fine on the spot.",
        ),
    ),
    Case(
        "city-transit",
        "how do you pay for the london tube",
        &[],
        Some(
            "In London: Tap a contactless card or phone on the bus and the Tube and it charges the cheapest fare of the day, with a daily cap, so a paper travelcard rarely beats it. Buses are a flat £1.75 and take no cash.",
        ),
    ),
    Case(
        "city-transit",
        "how do you get around in lisbon",
        &[],
        Some(
            "In Lisbon: Get a Viva Viagem card from the machine and load it: a single is €1.85 and a 24-hour pass €6.80. Tram 28 is beautiful and pickpocketed, so ride it early or take the metro instead.",
        ),
    ),
    Case(
        "city-transit",
        "is an oyster card worth it",
        &[],
        Some(
            "In London: Tap a contactless card or phone on the bus and the Tube and it charges the cheapest fare of the day, with a daily cap, so a paper travelcard rarely beats it. Buses are a flat £1.75 and take no cash.",
        ),
    ),
    Case(
        "city-transit",
        "where do i buy a suica card",
        &[],
        Some(
            "In Tokyo: Get a Suica or Pasmo card, or add one to your phone: a metro ride is about 180 yen and the machines take cash. Trains stop around midnight, so plan the last one.",
        ),
    ),
    Case("city-transit", "how do i use a navegante card", &[], None),
    Case(
        "city-transit",
        "would you recommend a day pass",
        &[],
        Some(
            "The rule is the same in most cities: tap a contactless bank card or your phone if the system takes one, because it charges the cheapest fare and usually caps the day, and if it does not, buy the stored-value card at a machine in the station - Oyster in London, Navigo in Paris, Suica in Tokyo. Compare a day or three-day pass with single tickets before you buy one: it only wins if you make several journeys a day, and the machines take a card, not always cash.",
        ),
    ),
    Case(
        "city-transit",
        "is contactless good",
        &[],
        Some(
            "The rule is the same in most cities: tap a contactless bank card or your phone if the system takes one, because it charges the cheapest fare and usually caps the day, and if it does not, buy the stored-value card at a machine in the station - Oyster in London, Navigo in Paris, Suica in Tokyo. Compare a day or three-day pass with single tickets before you buy one: it only wins if you make several journeys a day, and the machines take a card, not always cash.",
        ),
    ),
    Case(
        "city-transit",
        "how do i get a travelcard",
        &[],
        Some(
            "The rule is the same in most cities: tap a contactless bank card or your phone if the system takes one, because it charges the cheapest fare and usually caps the day, and if it does not, buy the stored-value card at a machine in the station - Oyster in London, Navigo in Paris, Suica in Tokyo. Compare a day or three-day pass with single tickets before you buy one: it only wins if you make several journeys a day, and the machines take a card, not always cash.",
        ),
    ),
    Case(
        "city-transit",
        "where can i get a metro card",
        &[],
        Some(
            "The rule is the same in most cities: tap a contactless bank card or your phone if the system takes one, because it charges the cheapest fare and usually caps the day, and if it does not, buy the stored-value card at a machine in the station - Oyster in London, Navigo in Paris, Suica in Tokyo. Compare a day or three-day pass with single tickets before you buy one: it only wins if you make several journeys a day, and the machines take a card, not always cash.",
        ),
    ),
    Case(
        "city-transit",
        "can i buy a bus ticket from the airport",
        &[],
        None,
    ),
    Case(
        "city-transit",
        "whats the easiest way to the airport in dublin",
        &[],
        None,
    ),
    Case("city-transit", "how do i get around", &[], None),
    Case("city-transit", "day trip to brussels", &[], None),
    Case("city-transit", "weekly pass in seoul", &[], None),
    Case(
        "city-transit",
        "is a metro card good for amsterdam",
        &[],
        Some(
            "In Amsterdam: Tap in and out with a contactless card or an OV-chipkaart: a trip inside the city is about €3.40 and forgetting to tap out costs you. The trams go everywhere and the centre is small.",
        ),
    ),
    Case(
        "city-transit",
        "is a taxi from the airport good in amsterdam",
        &[],
        None,
    ),
    Case(
        "city-transit",
        "tram for getting around edinburgh",
        &[],
        Some(
            "In Edinburgh: Lothian buses are a flat £2 and you tap a card, with no change given. The old town and the new town are 15 minutes apart on foot, and the tram is only worth it for the airport.",
        ),
    ),
    Case("city-transit", "what about getting about", &[], None),
    Case(
        "day-trip-place",
        "is sintra worth it",
        &[],
        Some(
            "Day trips from Lisbon: Sintra for the palaces (40 minutes by train). Buy the train ticket at the machine and go before 9am.",
        ),
    ),
    Case(
        "day-trip-place",
        "is obidos worth a trip",
        &[],
        Some(
            "Day trips from Lisbon: Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there).",
        ),
    ),
    Case(
        "day-trip-place",
        "is bruges worth it in november",
        &[],
        Some("Day trips from Brussels: Bruges (an hour by train) for the canals."),
    ),
    Case(
        "day-trip-place",
        "day trip to montserrat",
        &[],
        Some("Day trips from Barcelona: Montserrat (an hour by train and cable car)."),
    ),
    Case(
        "day-trip-place",
        "should i take a day trip to versailles",
        &[],
        Some(
            "Day trips from Paris: Versailles (40 minutes on the RER C). Go early: the first trains are the quietest.",
        ),
    ),
    Case(
        "day-trip-place",
        "what's worth seeing in chartres",
        &[],
        Some(
            "Day trips from Paris: Chartres for the cathedral. Go early: the first trains are the quietest.",
        ),
    ),
    Case("day-trip-place", "what about edam", &[], None),
    Case("day-trip-place", "do we want windermere", &[], None),
    Case("day-trip-place", "what about lyon in july", &[], None),
    Case(
        "day-trip-place",
        "is oxford worth a day trip",
        &[],
        Some(
            "Day trips from London: Oxford and Cambridge (about an hour). Book the train a week ahead and you halve the fare.",
        ),
    ),
    Case(
        "day-trip-place",
        "what about oxford and cambridge",
        &[],
        Some(
            "Day trips from London: Cambridge (about an hour). Book the train a week ahead and you halve the fare.",
        ),
    ),
    Case(
        "day-trip-place",
        "is versailles worth a visit out of paris in december",
        &[],
        Some(
            "Day trips from Paris: Versailles (40 minutes on the RER C). Go early: the first trains are the quietest.",
        ),
    ),
    Case("day-trip-place", "a walk by the pond", &[], None),
    Case(
        "day-trip-way",
        "how do we get to sintra",
        &[],
        Some("Sintra: 40 minutes by train from Lisbon."),
    ),
    Case(
        "day-trip-way",
        "how do you get to obidos",
        &[],
        Some("Obidos: an hour by bus from Lisbon's Campo Grande."),
    ),
    Case(
        "day-trip-way",
        "how far is chartres",
        &[],
        Some(
            "Day trips from Paris: Versailles (40 minutes on the RER C), Chartres for the cathedral, and Giverny for Monet's garden from April to October. Go early: the first trains are the quietest.",
        ),
    ),
    Case(
        "day-trip-way",
        "how long does it take to get to cambridge",
        &[],
        Some("Cambridge: about an hour from London."),
    ),
    Case(
        "day-trip-way",
        "how far is bruges",
        &[],
        Some("Bruges: an hour by train from Brussels."),
    ),
    Case(
        "day-trip-way",
        "how can i get to montserrat",
        &[],
        Some("Montserrat: an hour by train and cable car from Barcelona."),
    ),
    Case(
        "day-trip-way",
        "how to get to versailles",
        &[],
        Some("Versailles: 40 minutes on the RER C from Paris."),
    ),
    Case(
        "day-trip-way",
        "is oxford worth it",
        &[],
        Some(
            "Day trips from London: Oxford and Cambridge (about an hour). Book the train a week ahead and you halve the fare.",
        ),
    ),
    Case("day-trip-way", "how far is it", &[], None),
    Case("day-trip-way", "how do you get to edam", &[], None),
    Case("day-trip-way", "how far is portsmouth", &[], None),
    Case(
        "city-best-time",
        "when is the best time to book it",
        &["we're going to lisbon in may"],
        Some(
            "For a trip in May, book by mid-March: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
        ),
    ),
    Case(
        "city-best-time",
        "when is it best to book",
        &["i'm going to rome in december", "actually january"],
        Some(
            "For a trip in January, book by mid-November: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
        ),
    ),
    Case(
        "city-best-time",
        "when should we book",
        &["I'm going to Lisbon from late January to early February"],
        Some(
            "For a trip in February, book by mid-December: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
        ),
    ),
    Case(
        "city-best-time",
        "is there a time of year you recommend",
        &["I might go to Cornwall"],
        Some(
            "For Cornwall: May, June and September: 16-20C, the coast path at its best and the lanes still quiet. July and August are the warmest weeks and the busiest by a long way, and October is mild and much emptier, with the Atlantic at its biggest.",
        ),
    ),
    Case(
        "city-best-time",
        "and what is the best month",
        &["there's a list for scotland", "we're thinking of glasgow"],
        Some(
            "For Scotland: May to September for the Highlands, the islands and the long evenings. Midges are worst in July and August, and November to February has very short days.",
        ),
    ),
    Case(
        "city-best-time",
        "what about the best time of year",
        &["I am deciding between Germany and Belgium"],
        None,
    ),
    Case(
        "city-best-time",
        "is it worth it in june",
        &["we're off to dublin"],
        Some(
            "For Dublin: May, June and September: 16-20C and the longest days. July and August are the driest and busiest, and November to February is wet, with dark by 4.30pm.",
        ),
    ),
    Case(
        "city-best-time",
        "best time to book a holiday",
        &["it was april last year"],
        Some(
            "Book six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
        ),
    ),
    Case(
        "city-transit",
        "what about getting around",
        &[
            "were going to edinburgh in august",
            "two adults and a baby",
            "for four days",
            "is the castle ok with a pram",
        ],
        Some(
            "In Edinburgh: Lothian buses are a flat £2 and you tap a card, with no change given. The old town and the new town are 15 minutes apart on foot, and the tram is only worth it for the airport.",
        ),
    ),
    Case(
        "city-transit",
        "is the travel card worth buying",
        &[
            "are we going to portland",
            "thanks yes",
            "we're three people",
        ],
        Some(
            "The rule is the same in most cities: tap a contactless bank card or your phone if the system takes one, because it charges the cheapest fare and usually caps the day, and if it does not, buy the stored-value card at a machine in the station - Oyster in London, Navigo in Paris, Suica in Tokyo. Compare a day or three-day pass with single tickets before you buy one: it only wins if you make several journeys a day, and the machines take a card, not always cash.",
        ),
    ),
    Case(
        "city-transit",
        "how to get around",
        &[
            "i plan to travel to lisbon next year",
            "actually porto",
            "two adults",
            "ok",
        ],
        Some(
            "In Lisbon: Get a Viva Viagem card from the machine and load it: a single is €1.85 and a 24-hour pass €6.80. Tram 28 is beautiful and pickpocketed, so ride it early or take the metro instead.",
        ),
    ),
    Case(
        "city-transit",
        "would the weekly pass be cheaper",
        &[
            "I am going to Tokyo in winter",
            "is there an aquarium",
            "thanks",
        ],
        None,
    ),
    Case(
        "city-transit",
        "how do we get round the city",
        &["i am going to glasgow in october", "nice", "exciting"],
        None,
    ),
    Case(
        "city-transit",
        "will a day pass help",
        &["i am going to sweden in april", "thanks"],
        Some(
            "The rule is the same in most cities: tap a contactless bank card or your phone if the system takes one, because it charges the cheapest fare and usually caps the day, and if it does not, buy the stored-value card at a machine in the station - Oyster in London, Navigo in Paris, Suica in Tokyo. Compare a day or three-day pass with single tickets before you buy one: it only wins if you make several journeys a day, and the machines take a card, not always cash.",
        ),
    ),
    Case(
        "day-trip-place",
        "what about bruges",
        &[
            "we're off to lisbon in november",
            "okay",
            "we have one more day",
        ],
        None,
    ),
    Case(
        "day-trip-place",
        "is sintra worth it",
        &["we are in lisbon in may"],
        Some(
            "Day trips from Lisbon: Sintra for the palaces (40 minutes by train). Buy the train ticket at the machine and go before 9am.",
        ),
    ),
    Case(
        "day-trip-place",
        "what about bruges",
        &[
            "we are going to brussels in november",
            "yes do that",
            "thanks",
        ],
        Some("Day trips from Brussels: Bruges (an hour by train) for the canals."),
    ),
    Case(
        "day-trip-place",
        "is obidos worth it",
        &["i was in lisbon last summer"],
        Some(
            "Day trips from Lisbon: Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there).",
        ),
    ),
    Case(
        "day-trip-place",
        "is versailles worth it",
        &["i am going to rome in december", "yes", "okay"],
        None,
    ),
    Case(
        "day-trip-way",
        "and how do we get there",
        &["is sintra worth it"],
        Some("Sintra: 40 minutes by train from Lisbon."),
    ),
    Case(
        "day-trip-way",
        "how do we get there",
        &[
            "we are off to brussels this summer",
            "is bruges worth visiting",
        ],
        Some("Bruges: an hour by train from Brussels."),
    ),
    Case(
        "day-trip-way",
        "how far is it",
        &["could we go to chartres some time"],
        Some(
            "Day trips from Paris: Versailles (40 minutes on the RER C), Chartres for the cathedral, and Giverny for Monet's garden from April to October. Go early: the first trains are the quietest.",
        ),
    ),
    Case(
        "day-trip-way",
        "how do we get there",
        &["we are going to lisbon", "is bruges worth it"],
        None,
    ),
    Case(
        "day-trip-way",
        "and how do we get there",
        &["we're going to london", "would you recommend oxford"],
        Some("Oxford: about an hour from London."),
    ),
    Case(
        "day-trip-way",
        "how long does it take",
        &["is obidos worth it", "we have all day"],
        Some("Obidos: an hour by bus from Lisbon's Campo Grande."),
    ),
];
