//! Direct pinned callbacks for routes, sites, and country food.

pub(super) struct Case(
    pub &'static str,
    pub &'static str,
    pub &'static [&'static str],
    pub Option<&'static str>,
);
pub(super) const CASES: &[Case] = &[
    Case(
        "route-between",
        "how do we get from rome to florence",
        &[],
        None,
    ),
    Case(
        "route-between",
        "how do we get from florence to rome",
        &[],
        None,
    ),
    Case(
        "route-between",
        "how do i travel from edinburgh to london",
        &[],
        None,
    ),
    Case(
        "route-between",
        "how to get from london to edinburgh",
        &[],
        None,
    ),
    Case(
        "route-between",
        "should we get the train from london to paris",
        &[],
        Some(
            "London to Paris: the Eurostar from St Pancras is two hours twenty to the Gare du Nord, city centre to city centre, and the cheapest fares go on sale months ahead. Check in 75 to 90 minutes before, and the coach is cheaper but a whole day.",
        ),
    ),
    Case(
        "route-between",
        "go from paris to london",
        &[],
        Some(
            "London to Paris: the Eurostar from St Pancras is two hours twenty to the Gare du Nord, city centre to city centre, and the cheapest fares go on sale months ahead. Check in 75 to 90 minutes before, and the coach is cheaper but a whole day.",
        ),
    ),
    Case("route-between", "train between paris and lyon", &[], None),
    Case(
        "route-between",
        "is it possible to travel between porto and lisbon without a car",
        &[],
        None,
    ),
    Case(
        "route-between",
        "how do i get from venice to milan",
        &[],
        None,
    ),
    Case(
        "route-between",
        "how to get from japan to london",
        &[],
        None,
    ),
    Case(
        "route-between",
        "how do i move from silicon valley to new york",
        &[],
        None,
    ),
    Case(
        "route-between",
        "is amalfi coast closer to rome, naples or florence",
        &[],
        Some(
            "Naples to the Amalfi coast: the season ferry from Molo Beverello to Amalfi or Positano is the pretty way and takes 75 minutes; the SITA bus from Sorrento or Salerno is cheaper and slower, and the coast road is the part that jams in summer. A car is a liability in August.",
        ),
    ),
    Case("route-between", "london and bath and edinburgh", &[], None),
    Case(
        "route-between",
        "heres our itinerary madrid barcelona london paris and rome",
        &[],
        Some(
            "London to Paris: the Eurostar from St Pancras is two hours twenty to the Gare du Nord, city centre to city centre, and the cheapest fares go on sale months ahead. Check in 75 to 90 minutes before, and the coach is cheaper but a whole day.",
        ),
    ),
    Case(
        "route-between",
        "how do i get from lisbon to madrid and barcelona",
        &[],
        None,
    ),
    Case("route-between", "london2paris", &[], None),
    Case("route-between", "the londoner in paris", &[], None),
    Case(
        "route-between",
        "what is the name of the first route",
        &[],
        None,
    ),
    Case("route-between", "how do i get between them", &[], None),
    Case(
        "site-worth-visiting",
        "is the colosseum worth visiting",
        &[],
        Some(
            "The Colosseum together with the Forum and Palatine is one ticket and most of a day, so start early and take water. Book the timed entry and the underground or arena add-on if you want them, because they sell out first.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is the pantheon a must see",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "is ellis island worth a visit",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "would you recommend the louvre or the uffizi",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "how long for the vatican museums",
        &[],
        Some(
            "The Vatican Museums and the Sistine Chapel are three hours at a fast walk and half a day at a normal one, so book the first entry of the day. Cover your shoulders and knees, and note the chapel is silent with no photographs.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "was the tower of london built a long time ago",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "should i visit the eiffel tower",
        &[],
        Some(
            "The Eiffel Tower wants a booked time slot and two to three hours with the queues and the lifts, or 45 minutes if you walk up to the second floor. The stairs are cheaper and shorter, and the last lift up is around 11pm.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is the roman forum worth the time",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "is the trevi fountain worth it",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "is buckingham palace open",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "what do they say about alcatraz",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "whats at stonehenge",
        &[],
        Some(
            "Stonehenge is two hours at most, and the stones are seen from a path rather than walked among. Book a timed slot including the shuttle from the visitor centre, and pair it with Avebury or Salisbury if you are driving that far.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is mont saint michel good for a day out",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "walt disney world or disneyland paris",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "what would you visit in barcelona",
        &[],
        None,
    ),
    Case(
        "site-worth-visiting",
        "how long at the saint paul cathedral",
        &[],
        None,
    ),
    Case("site-worth-visiting", "what is the best site", &[], None),
    Case(
        "site-worth-visiting",
        "is la sagrada familia worth visiting",
        &[],
        Some(
            "The Sagrada Familia needs a timed ticket bought online, and an hour and a half inside. Add the tower lift if you want it - it is a separate ticket - and go in the morning when the east windows are lit.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "louvres vs alhambra",
        &[],
        Some(
            "The Alhambra is a half day and the Nasrid palaces need a timed slot booked weeks ahead, in the morning if you can. Take your passport or ID card: it is checked against the ticket, and people are turned away without it.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what is mexican food like",
        &[],
        Some(
            "In Mexico: Tacos al pastor from a stand, tamales for breakfast and mole in Oaxaca, with the salsa on the side so you choose the heat.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what do they eat in the usa",
        &[],
        Some(
            "In the USA: Go regional: barbecue in Texas or Kansas City, a po'boy in New Orleans, pizza by the slice in New York, tacos in Los Angeles.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what should i eat in america",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "what do they eat in portugal",
        &[],
        Some(
            "In Portugal: Pastéis de nata and grilled sardines in Lisbon, a francesinha in Porto and cataplana on the Algarve. Look for the prato do dia, a set lunch with wine for very little.",
        ),
    ),
    Case(
        "food-in-the-country",
        "whats portuguese food like",
        &[],
        Some(
            "In Portugal: Pastéis de nata and grilled sardines in Lisbon, a francesinha in Porto and cataplana on the Algarve. Look for the prato do dia, a set lunch with wine for very little.",
        ),
    ),
    Case("food-in-the-country", "places to eat in rome", &[], None),
    Case(
        "food-in-the-country",
        "is the food safe in mexico",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "what should i avoid eating in india",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "i am from portugal what should i eat",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "i was born in portugal what should i eat",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "i was raised in the usa what should i eat",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "what should i try in lisbon",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "whats good to eat in dubai",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "what is thai food like",
        &[],
        Some(
            "In Thailand: Street food is the point: pad thai, som tam and moo ping, cooked in front of you where it is busy. Say \"not spicy\" clearly, and eat breakfast at the market.",
        ),
    ),
    Case(
        "food-in-the-country",
        "whats vietnamese food like",
        &[],
        Some(
            "In Vietnam: Phở for breakfast in Hanoi, bánh mì from a cart and fresh spring rolls down the middle of the country. Eat where the stools are small and the queue is long.",
        ),
    ),
    Case(
        "food-in-the-country",
        "how do i keep thai food safe",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "can i bring food home from france",
        &[],
        None,
    ),
    Case(
        "food-in-the-country",
        "what should i eat on holiday in new zealand",
        &[],
        Some(
            "In New Zealand: A meat pie from a dairy, fish and chips on the beach and a hāngi if you get invited; lamb is the dinner and the flat white is the coffee.",
        ),
    ),
    Case("food-in-the-country", "what can i eat there", &[], None),
    Case(
        "food-in-the-country",
        "what does a turkey eat",
        &[],
        Some(
            "In Turkey: Balık ekmek - fish in bread - by the water in Istanbul, a breakfast of cheese, olives and menemen, and pistachio baklava in the south-east.",
        ),
    ),
    Case("food-in-the-country", "is greeks food good", &[], None),
    Case(
        "food-in-the-country",
        "what is south african food like",
        &[],
        Some(
            "In South Africa: A braai is the meal: grilled meat, a pap and relish on the side. Then bunny chow in Durban, biltong from a butcher and a bobotie.",
        ),
    ),
    Case(
        "food-in-the-country",
        "whats lebanese food like",
        &[],
        Some(
            "In Lebanon: Mezze is the meal: hummus, tabbouleh, fattoush and grilled halloumi, then kibbeh and a plate of mixed grill. Order several small plates and share.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what is good in edinburgh and portugal",
        &[],
        Some(
            "In Portugal: Pastéis de nata and grilled sardines in Lisbon, a francesinha in Porto and cataplana on the Algarve. Look for the prato do dia, a set lunch with wine for very little.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what can i eat in mexico this spring",
        &[],
        Some(
            "In Mexico: Tacos al pastor from a stand, tamales for breakfast and mole in Oaxaca, with the salsa on the side so you choose the heat.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what should i eat while visiting singapore",
        &[],
        Some(
            "In Singapore: A hawker centre is the answer: chicken rice, laksa and char kway teow from different stalls, all cheap. Order with tissue packets on the table to claim a seat.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what about food",
        &["i live in spain"],
        None,
    ),
    Case(
        "food-in-the-country",
        "what about food",
        &["my sister lives in reno", "we are going to spain in summer"],
        Some(
            "In Spain: Tapas at the bar - two or three plates between you - paella in Valencia and pintxos in San Sebastián. Jamón and pimientos de Padrón are the cheap way in.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what should i eat there",
        &["how long to stay in japan", "okay got it"],
        Some(
            "In Japan: Sushi at a counter and a bowl of ramen in Tokyo, takoyaki and okonomiyaki in Osaka, tofu and kaiseki in Kyoto. Convenience-store food is genuinely good and cheap.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what should i eat there",
        &["my parents are visiting greece"],
        Some(
            "In Greece: Souvlaki, grilled octopus and a proper village salad: tomato, cucumber, onion and a slab of feta, no lettuce. Eat where the menu is short and the bread is fresh.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what should i eat there",
        &["my sister lives in athens"],
        None,
    ),
    Case(
        "food-in-the-country",
        "what should i try when i go",
        &["i am going to granada and madrid"],
        None,
    ),
    Case(
        "food-in-the-country",
        "what should we eat there",
        &["we are going to belgium", "i have booked a hotel"],
        Some(
            "In Belgium: Mussels with chips, a waffle from a stall and frites with mayonnaise; the beer is the point, and every bar has its own glass.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what should we eat there",
        &["i went to belgium last year"],
        None,
    ),
    Case(
        "food-in-the-country",
        "what is the main dish",
        &[
            "i cannot drink the tap water in mexico",
            "we are flying to mexico next week",
        ],
        Some(
            "In Mexico: Tacos al pastor from a stand, tamales for breakfast and mole in Oaxaca, with the salsa on the side so you choose the heat.",
        ),
    ),
    Case(
        "food-in-the-country",
        "what is the food like",
        &[
            "my sister is going to croatia",
            "her husband lives in croatia",
        ],
        None,
    ),
    Case(
        "route-between",
        "how to get from naples to sorrento",
        &[],
        Some(
            "Naples to Sorrento: the Circumvesuviana train from Napoli Garibaldi takes about an hour and costs a few euros, and it is crowded with luggage at commuter times. The fast ferry from Molo Beverello is 35 to 45 minutes and pleasanter, but it runs only in season.",
        ),
    ),
    Case(
        "route-between",
        "how to get from naples to amalfi",
        &[],
        Some(
            "Naples to the Amalfi coast: the season ferry from Molo Beverello to Amalfi or Positano is the pretty way and takes 75 minutes; the SITA bus from Sorrento or Salerno is cheaper and slower, and the coast road is the part that jams in summer. A car is a liability in August.",
        ),
    ),
    Case(
        "route-between",
        "how to get from sorrento to amalfi",
        &[],
        Some(
            "Sorrento to Amalfi: the SITA bus along the coast road is about 90 minutes and the ferry is 25, but the ferry runs from spring to October only. Sit on the left of the bus going east for the view, and buy the ticket before you queue.",
        ),
    ),
    Case(
        "route-between",
        "how to get from rome to pompeii",
        &[],
        Some(
            "Rome to Pompeii: the fast train to Naples takes 70 minutes, then the Circumvesuviana to Pompeii Scavi, so about two and a half hours door to door and a long day. Herculaneum is the same trip and half the walking.",
        ),
    ),
    Case(
        "route-between",
        "how to get from naples to pompeii",
        &[],
        Some(
            "Naples to Pompeii: the Circumvesuviana from Napoli Garibaldi to Pompeii Scavi takes 35 to 40 minutes and drops you at the site gate. Get off at Pompeii Scavi, not Pompei town, and keep your bag in front of you on the train.",
        ),
    ),
    Case(
        "route-between",
        "how to get from florence to pisa",
        &[],
        Some(
            "Florence to Pisa: the regional train from Santa Maria Novella takes about an hour and the tower is a 20-minute walk from Pisa Centrale. Book the tower climb separately and go early, because the slots are timed and the station is far from the airport side.",
        ),
    ),
    Case(
        "route-between",
        "how to get from london to paris",
        &[],
        Some(
            "London to Paris: the Eurostar from St Pancras is two hours twenty to the Gare du Nord, city centre to city centre, and the cheapest fares go on sale months ahead. Check in 75 to 90 minutes before, and the coach is cheaper but a whole day.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is vatican museums worth visiting",
        &[],
        Some(
            "The Vatican Museums and the Sistine Chapel are three hours at a fast walk and half a day at a normal one, so book the first entry of the day. Cover your shoulders and knees, and note the chapel is silent with no photographs.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is sagrada familia worth visiting",
        &[],
        Some(
            "The Sagrada Familia needs a timed ticket bought online, and an hour and a half inside. Add the tower lift if you want it - it is a separate ticket - and go in the morning when the east windows are lit.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is sistine chapel worth visiting",
        &[],
        Some(
            "The Vatican Museums and the Sistine Chapel are three hours at a fast walk and half a day at a normal one, so book the first entry of the day. Cover your shoulders and knees, and note the chapel is silent with no photographs.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is eiffel tower worth visiting",
        &[],
        Some(
            "The Eiffel Tower wants a booked time slot and two to three hours with the queues and the lifts, or 45 minutes if you walk up to the second floor. The stairs are cheaper and shorter, and the last lift up is around 11pm.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is herculaneum worth visiting",
        &[],
        Some(
            "Herculaneum takes two or three hours rather than a day, and the woodwork, frescoes and upper floors survived in a way Pompeii's did not, so it is the better visit if you have one afternoon. It is the same train line as Pompeii, one stop closer to Naples, and the site is small enough to see in one go.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is the vatican worth visiting",
        &[],
        Some(
            "The Vatican Museums and the Sistine Chapel are three hours at a fast walk and half a day at a normal one, so book the first entry of the day. Cover your shoulders and knees, and note the chapel is silent with no photographs.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is stonehenge worth visiting",
        &[],
        Some(
            "Stonehenge is two hours at most, and the stones are seen from a path rather than walked among. Book a timed slot including the shuttle from the visitor centre, and pair it with Avebury or Salisbury if you are driving that far.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is auschwitz worth visiting",
        &[],
        Some(
            "Auschwitz-Birkenau is a heavy half day, five or six hours with the bus, and the timed entry must be booked ahead. Go with a guide, take tissues, and plan something gentle for afterwards.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is colosseum worth visiting",
        &[],
        Some(
            "The Colosseum together with the Forum and Palatine is one ticket and most of a day, so start early and take water. Book the timed entry and the underground or arena add-on if you want them, because they sell out first.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is vesuvius worth visiting",
        &[],
        Some(
            "Vesuvius is a bus up from Ercolano and a steep 20-minute walk from the car park, with the crater rim roped off. Go on a clear morning, take a jacket because it is cold and windy at the top, and check the bus times for the way back: the last one leaves early.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is alhambra worth visiting",
        &[],
        Some(
            "The Alhambra is a half day and the Nasrid palaces need a timed slot booked weeks ahead, in the morning if you can. Take your passport or ID card: it is checked against the ticket, and people are turned away without it.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is vatican worth visiting",
        &[],
        Some(
            "The Vatican Museums and the Sistine Chapel are three hours at a fast walk and half a day at a normal one, so book the first entry of the day. Cover your shoulders and knees, and note the chapel is silent with no photographs.",
        ),
    ),
    Case(
        "site-worth-visiting",
        "is pompeii worth visiting",
        &[],
        Some(
            "Give Pompeii most of a day: it is 66 hectares and there is no shade, so go at opening time, take water and wear shoes with grip. Book a timed ticket and the train from Naples on the Circumvesuviana; the audioguide is worth it and the walk to the amphitheatre is the quiet half.",
        ),
    ),
];
