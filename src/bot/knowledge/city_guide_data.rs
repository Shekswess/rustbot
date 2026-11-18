//! Guide tables and word lists from PyBot 7d63c7f, bot/knowledge/travel.py.
use super::City;

pub(super) const EATING: &str = r####"\b(?:eat|eats|eating|food|foods|meal|meals|restaurant|restaurants|lunch|dinner|breakfast|snack|snacks|menu|menus|dining)\b"####;
pub(super) const VALUE_EATING: &str =
    r####"\b(?:good value|value for money|cheap eats|cheapest eats)\b"####;
pub(super) const CHEAP_EATS_ORDER: &[&str] = &[
    "rome",
    "paris",
    "barcelona",
    "madrid",
    "lisbon",
    "florence",
    "amsterdam",
];
pub(super) const GENERAL_EATS: &str = "Three habits make any city cheap to eat in: take the fixed lunch rather than dinner - menu del día, plat du jour, prato do dia, Mittagsmenü - which is the same kitchen at half the price; eat where people work and shop, at a market hall, a bakery counter or a busy bar, rather than where they sightsee; and stand up to eat, because pizza by the slice, a kebab, a bowl of noodles at a counter or a picnic from a supermarket costs a fraction of a table with a view. Coffee and breakfast at a bar, and tap water in a bottle you refill, save the rest.";

pub(super) const CITIES: &[City] = &[
    City {
        name: "lisbon",
        title: "Lisbon",
        guide: "Alfama on foot and the 28 tram, Belém for the tower and the custard tarts, and LX Factory or Time Out Market for lunch. May is warm and busy, so book the tower and the popular restaurants ahead.",
        food: Some(
            "In Lisbon a bifana or a pastel de nata at a counter costs a couple of euros, and the prato do dia at lunch in Alfama or Graça is the full cheap meal. Drink at a kiosk rather than a terrace, and the Time Out Market is the easy one-stop version.",
        ),
        areas: Some(
            "In Lisbon: by area, Alfama for the old streets and the views but steep and noisy, Baixa and Chiado to be in the middle of it, or Principe Real for quiet and smart.",
        ),
        day_trips: Some(
            "Day trips from Lisbon: Sintra for the palaces (40 minutes by train), Cascais for the beach (40 minutes on the same line), and Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there). Buy the train ticket at the machine and go before 9am.",
        ),
        other: &[
            "In Lisbon: Get a Viva Viagem card from the machine and load it: a single is €1.85 and a 24-hour pass €6.80. Tram 28 is beautiful and pickpocketed, so ride it early or take the metro instead.",
            "For Lisbon: May, June and September: warm and dry with fewer people. July and August are hot and busy with the tourists, and November to February is wet, though milder than the north.",
            "Lisbon is about £45 to £80 a day per person on the ground, room aside. Eating at the bar and using the metro rather than taxis is the difference.",
        ],
        days: 3,
    },
    City {
        name: "rome",
        title: "Rome",
        guide: "the Colosseum with a timed ticket, the Pantheon, and Trastevere in the evening for dinner. Book the Vatican museums online, and carry a refillable bottle: the fountains are free.",
        food: Some(
            "In Rome the cheap meal is pizza al taglio, sold by weight and eaten standing, or a panino from a bakery counter; supplì and a slice of pizza bianca are a couple of euros. Eat in Testaccio or Trastevere rather than beside a monument, and take your coffee standing at the bar, where it costs a third of the table price.",
        ),
        areas: Some(
            "In Rome: by area, Monti for the Colosseum on foot and quiet nights, Trastevere for restaurants and cobbles, or Prati for the Vatican and a calmer evening.",
        ),
        day_trips: Some(
            "Day trips from Rome: Ostia Antica for the Roman ruins (30 minutes on the metro and train), Tivoli for the villa gardens, and Orvieto for the cathedral. Pompeii is two hours each way, so it is a full day.",
        ),
        other: &[
            "In Rome: A ticket is €1.50 and lasts 100 minutes on the metro and the buses, validated in the machine on board. There are only two metro lines, so walk the centre and take a bus for the rest.",
            "For Rome: April, May, late September and October: 20-26C and the ruins walkable. July and August are 30C+ with the fountains as the only relief, and August means half the restaurants shut.",
            "Rome is about £55 to £95 a day per person on the ground, room aside. Coffee standing at the bar and one big meal at lunch keeps it down.",
        ],
        days: 3,
    },
    City {
        name: "florence",
        title: "Florence",
        guide: "the Duomo climb, the Uffizi (book a slot), and the Oltrarno side of the river for food. Two days is enough for those three if you don't queue for everything.",
        food: Some(
            "In Florence the cheap meal is a panino from a busy shop (a big one is about six euros), pizza al taglio, or a trattoria across the river in the Oltrarno; a table on a piazza with a view is the expensive version of the same food.",
        ),
        areas: Some(
            "In Florence: by area, Santa Croce for the centre and a lively square, Oltrarno for artisans and quiet, or around Santa Maria Novella for the station.",
        ),
        day_trips: Some(
            "Day trips from Florence: Pisa (an hour by train) for the tower, Siena (90 minutes) for the square, and Lucca for the walls and the bikes. Fiesole is 30 minutes by bus for the view back over Florence.",
        ),
        other: &[
            "In Florence: Walk: the historic centre is 20 minutes across and a car is a liability. A bus ticket is €1.50 for 90 minutes, from a tabacchi or the app rather than the driver.",
            "For Florence: April to June and September to October: 20-28C and the galleries without the August crush. July and August are 30C+ and busy, and winter means cool days and short queues.",
            "Florence is about £55 to £95 a day per person on the ground, room aside. Stay outside the centre and walk in: the hotels halve in price.",
        ],
        days: 2,
    },
    City {
        name: "paris",
        title: "Paris",
        guide: "walk the Île de la Cité to the Marais, pick one big museum rather than three, and eat in a market street rather than by the Eiffel Tower. Book the Louvre or Orsay timed entry online.",
        food: Some(
            "In Paris the cheap lunch is a bakery one: a jambon-beurre is about five euros, and a market street such as Rue Cler or Rue Mouffetard does the same for less. The plat du jour at a neighbourhood bistro is half the evening price, tap water is free and normal, and a picnic on the quays beats a table by the Eiffel Tower.",
        ),
        areas: Some(
            "In Paris: by area, Le Marais for the centre and cafes, Saint-Germain for the classic postcard, or Canal Saint-Martin for cheaper nights and a younger crowd.",
        ),
        day_trips: Some(
            "Day trips from Paris: Versailles (40 minutes on the RER C), Chartres for the cathedral, and Giverny for Monet's garden from April to October. Go early: the first trains are the quietest.",
        ),
        other: &[
            "In Paris: Get a Navigo Easy card and load ten tickets: a single is €2.15 and ten is cheaper, and it covers the Metro inside the city. Keep the ticket until you are out of the gate, because inspectors fine on the spot.",
            "For Paris: April to June and September to early October: 18-24C and the parks at their best. July and August are hot and many small restaurants close, and November to February is grey and cool.",
            "Paris is about £65 to £110 a day per person on the ground, room aside. Lunch from a bakery and the metro rather than taxis is the difference.",
        ],
        days: 4,
    },
    City {
        name: "barcelona",
        title: "Barcelona",
        guide: "the Gothic Quarter, the Sagrada Família (book weeks ahead), and the beach at Barceloneta for half a day. Eat lunch at 2pm like locals; the good places open then.",
        food: Some(
            "In Barcelona the cheat code is the menú del día at lunch: three courses and a drink for around twelve to fifteen euros, in the Eixample or Gràcia, not on Las Ramblas. Standing at a bar with tapas and a vermut costs a few euros a round, and the Boqueria's back stalls are cheaper than its front ones.",
        ),
        areas: Some(
            "In Barcelona: by area, the Eixample for a grid that is easy to walk and safe at night, Gracia for a village feel, or El Born for tapas and narrow streets.",
        ),
        day_trips: Some(
            "Day trips from Barcelona: Montserrat (an hour by train and cable car), Girona (40 minutes) and Sitges for the beach (40 minutes). Girona plus Figures makes a good full day.",
        ),
        other: &[
            "In Barcelona: A T-casual card is ten rides for about €12.50 but it is one person only: for two or more of you the T-familiar is eight rides over 30 days and can be passed back over the gate. The metro reaches nearly everything and runs late.",
            "For Barcelona: May, June and September: 22-27C, the sea warm enough to swim and the terraces full. August is hot and packed, and November to February is cool with some rain.",
            "Barcelona is about £50 to £85 a day per person on the ground, room aside. The menu del dia at lunch is half the price of the same meal at night.",
        ],
        days: 3,
    },
    City {
        name: "madrid",
        title: "Madrid",
        guide: "the Prado, the Retiro park and the tapas streets around Plaza Mayor. Dinner starts late here, so plan on eating at 9pm or later.",
        food: Some(
            "In Madrid the menú del día at lunch is the cheap meal, and tapas around La Latina or Lavapiés are a few euros each, with the drink included in most bars. Standing at the bar is cheaper than a table, and dinner starts late, so lunch is the meal to spend on.",
        ),
        areas: Some(
            "In Madrid: by area, La Latina for tapas on the doorstep, Malasana for going out, or Salamanca for somewhere smart and quiet.",
        ),
        day_trips: Some(
            "Day trips from Madrid: Toledo (30 minutes by train), Segovia (30 minutes) and El Escorial. Toledo and Segovia both deserve a full day rather than an afternoon.",
        ),
        other: &[
            "In Madrid: A ten-ride Metrobus card is the cheapest way to ride the metro and the buses, about €1.20 a trip, and the centre is walkable anyway. The metro is fast and air conditioned.",
            "For Madrid: April, May, June and September: 20-28C and dry, with the terraces out and the Prado quiet. July and August are 35C+ and the city empties, and winter is cold and clear.",
            "Madrid is about £45 to £80 a day per person on the ground, room aside. Tapas at the bar with a drink is cheaper than a sit-down dinner.",
        ],
        days: 3,
    },
    City {
        name: "amsterdam",
        title: "Amsterdam",
        guide: "the Jordaan on foot, the Rijksmuseum and a canal walk at dusk. Book Anne Frank House tickets online weeks ahead; they sell out completely.",
        food: Some(
            "In Amsterdam the cheap meal is a broodje from a bakery, a portion of chips with mayonnaise, or the market at Albert Cuypstraat; a supermarket picnic in the Vondelpark costs less than a café lunch, and tap water is free if you ask.",
        ),
        areas: Some(
            "In Amsterdam: by area, the Jordaan for canals and quiet, De Pijp for the food market and locals, or the centre if you want everything on foot and do not mind noise.",
        ),
        day_trips: Some(
            "Day trips from Amsterdam: Haarlem (15 minutes), Utrecht (25 minutes) and the Zaanse Schans windmills (40 minutes). Keukenhof is worth it only in tulip season, mid-March to mid-May.",
        ),
        other: &[
            "In Amsterdam: Tap in and out with a contactless card or an OV-chipkaart: a trip inside the city is about €3.40 and forgetting to tap out costs you. The trams go everywhere and the centre is small.",
            "For Amsterdam: April to June and September: 15-22C and the canals at their best, with the tulips at Keukenhof in late April. July and August are the busiest, and November to February is cold, wet and dark early.",
            "Amsterdam is about £60 to £100 a day per person on the ground, room aside. Hire a bike rather than trams, and eat away from the canal belt.",
        ],
        days: 2,
    },
    City {
        name: "berlin",
        title: "Berlin",
        guide: "the Wall memorial and East Side Gallery, Museum Island, and Kreuzberg for food. It's a big, spread-out city, so buy a day travel ticket.",
        food: None,
        areas: Some(
            "In Berlin: by area, Mitte for the centre and the museums, Kreuzberg for nightlife and grit, or Prenzlauer Berg for cafes and families.",
        ),
        day_trips: Some(
            "Day trips from Berlin: Potsdam for Sanssouci (30 minutes by train) and Sachsenhausen, which is a heavy half-day and worth it. Both are covered by the Berlin ABC travel card.",
        ),
        other: &[
            "In Berlin: A single AB ticket is €3.50 and a 24-hour ticket about €10, so the day ticket wins after three rides. There are no ticket gates and the inspectors do fine, so validate before you board.",
            "For Berlin: May to September: 20-25C, outdoor cafes, the lakes, and the city at its most relaxed. July and August are the warmest and busiest, and November to February is grey and cold.",
            "Berlin is about £50 to £85 a day per person on the ground, room aside. It is cheaper than the rest of western Europe, especially eating out.",
        ],
        days: 3,
    },
    City {
        name: "prague",
        title: "Prague",
        guide: "the Old Town at 7am before the crowds, Charles Bridge early, and the castle on the hill. Beer is cheaper than water, but change money at a bank, not a kiosk.",
        food: None,
        areas: Some(
            "In Prague: by area, Vinohrady for leafy streets and local restaurants, Mala Strana for quiet and pretty, or the Old Town for the middle of it and the noise.",
        ),
        day_trips: Some(
            "Day trips from Prague: Kutna Hora for the bone church and the silver mines (an hour by train), Karlstejn for the castle, and Cesky Krumlov, which is three hours each way so it wants a night.",
        ),
        other: &[
            "In Prague: A 30-minute ticket is 30 crowns and a 90-minute one 40, from a machine or the app, and you stamp it on the first ride. The trams are the best sightseeing in the city.",
            "For Prague: April to June and September: 15-22C and the old town before the crush. July and August are busy and 25C+, and December is cold with the Christmas markets.",
            "Prague is about £35 to £65 a day per person on the ground, room aside. Eat and drink away from the old town square and it halves.",
        ],
        days: 3,
    },
    City {
        name: "vienna",
        title: "Vienna",
        guide: "the Ringstrasse, one grand coffee house and the Naschmarkt. Book the Spanish Riding School or a concert only if you're sure: they are pricey.",
        food: None,
        areas: Some(
            "In Vienna: by area, the Innere Stadt to walk to the palaces and concerts, Leopoldstadt for the Prater and cheaper rooms, or Neubau for small shops and cafes.",
        ),
        day_trips: Some(
            "Day trips from Vienna: Bratislava (an hour by train), the Wachau valley by boat for the vineyards and Durnstein, and Baden for the spa. The Einfach-Raus ticket covers a group on the regional trains.",
        ),
        other: &[
            "In Vienna: A single is €2.40 and a 48-hour pass about €14, which is what a weekend wants. The U-Bahn is fast, the Ringstrasse is a 40-minute walk, and you validate the ticket once.",
            "For Vienna: April to June and September: 18-24C, and the ball season runs January to March. July and August are hot and the opera houses close, and December has the markets.",
            "Vienna is about £55 to £95 a day per person on the ground, room aside. Coffee and cake add up, and the transport pass is worth it.",
        ],
        days: 3,
    },
    City {
        name: "edinburgh",
        title: "Edinburgh",
        guide: "the Royal Mile and the castle, Arthur's Seat for the view, and Leith for food. In December it's dark by 4pm and the Christmas market fills the centre.",
        food: None,
        areas: Some(
            "In Edinburgh: by area, the New Town for Georgian calm and everything on foot, the Old Town for the Royal Mile and tourists, or Leith for the food.",
        ),
        day_trips: Some(
            "Day trips from Edinburgh: Glasgow (50 minutes by train), Stirling for the castle, and North Berwick for the beach and the seabird centre. The Highlands need a car or a full day tour.",
        ),
        other: &[
            "In Edinburgh: Lothian buses are a flat £2 and you tap a card, with no change given. The old town and the new town are 15 minutes apart on foot, and the tram is only worth it for the airport.",
            "For Edinburgh: May, June and September: 15-20C, and daylight until nearly 11pm in June; August is the Festival and the Fringe, which is either why you go or when to avoid. November to February is dark by 4pm.",
            "Edinburgh is about £55 to £90 a day per person on the ground, room aside. August doubles the hotel prices because of the festival.",
        ],
        days: 2,
    },
    City {
        name: "london",
        title: "London",
        guide: "pick one area a day rather than criss-crossing: South Bank, the museums in South Kensington, and the markets in the East End. The Tube is quickest; walking between central stops is often as fast.",
        food: None,
        areas: Some(
            "In London: by area, Bloomsbury to walk to most of it, the South Bank for the river and theatres, or Shoreditch for bars and a late night.",
        ),
        day_trips: Some(
            "Day trips from London: Bath (80 minutes by train), Oxford and Cambridge (about an hour), and Brighton (an hour) for the sea. Book the train a week ahead and you halve the fare.",
        ),
        other: &[
            "In London: Tap a contactless card or phone on the bus and the Tube and it charges the cheapest fare of the day, with a daily cap, so a paper travelcard rarely beats it. Buses are a flat £1.75 and take no cash.",
            "For London: May, June and September: 18-22C and the long evenings. July and August are warm and crowded, and November to February is grey, wet and dark by 4.30pm.",
            "London is about £65 to £110 a day per person on the ground, room aside. The tube and eating away from the centre is what keeps it down.",
        ],
        days: 4,
    },
    City {
        name: "dublin",
        title: "Dublin",
        guide: "a walk through Trinity College and the Book of Kells, the Guinness Storehouse, and pubs away from Temple Bar for music. Book the Book of Kells online.",
        food: None,
        areas: Some(
            "In Dublin: by area, Portobello for quiet and locals, St Stephen's Green to be central, or Temple Bar for the pubs and the noise that comes with them.",
        ),
        day_trips: Some(
            "Day trips from Dublin: Howth for the cliff walk (30 minutes on the DART), Malahide castle, and Glendalough in the Wicklow mountains, which needs a car or a bus tour.",
        ),
        other: &[
            "In Dublin: A Leap card takes about 30% off, so a bus or tram ride is €2 on it rather than the cash fare. The centre is 20 minutes across on foot and the DART follows the coast.",
            "For Dublin: May, June and September: 16-20C and the longest days. July and August are the driest and busiest, and November to February is wet, with dark by 4.30pm.",
            "Dublin is about £60 to £100 a day per person on the ground, room aside. The hotels are the expensive part, so book early.",
        ],
        days: 2,
    },
    City {
        name: "porto",
        title: "Porto",
        guide: "the river walk to Ribeira, a port cellar tour in Gaia, and the Lello bookshop early or late. The trams are the nicest way to see the city.",
        food: None,
        areas: Some(
            "In Porto: by area, Ribeira for the river and the crowds, Cedofeita for shops and cafes, or Bonfim for local streets and cheaper rooms.",
        ),
        day_trips: Some(
            "Day trips from Porto: The Douro valley by train to Pinhao for the vineyards, Braga and Guimaraes for the churches and the old town, and Aveiro for the canals. The Douro line is the scenic one, so sit on the right.",
        ),
        other: &[
            "In Porto: An Andante card covers the metro and the buses at about €1.60 a trip, plus 60 cents for the card itself. The historic tram is a separate €3.50 and worth doing once.",
            "For Porto: May, June and September: 18-24C and dry, with the Douro valley at its best and the vines in leaf. July and August are warm and busy, and November to March is wet and grey.",
            "Porto is about £40 to £70 a day per person on the ground, room aside. It is cheaper than Lisbon and the portions are bigger.",
        ],
        days: 2,
    },
    City {
        name: "seville",
        title: "Seville",
        guide: "the Alcázar (book ahead), the cathedral and oranges everywhere in the streets of Santa Cruz. In high summer it is 40C, so sightsee before 11am and after 7pm.",
        food: None,
        areas: Some(
            "In Seville: by area, Santa Cruz for narrow streets and the Alcazar, Triana for ceramics and locals, or the Alameda for bars.",
        ),
        day_trips: Some(
            "Day trips from Seville: Cordoba for the mosque-cathedral (45 minutes by train), Ronda for the gorge and the bridge, and Jerez for the sherry bodegas. Granada is three hours by bus, so it wants a night.",
        ),
        other: &[
            "In Seville: Walk: the centre is flat and 20 minutes across. Beyond that a bus ticket is €1.40 and the tram crosses the old town, and bikes are easy to hire on the cycle lanes.",
            "For Seville: March to May and October: 20-28C and the orange trees in flower or in fruit, with Semana Santa and the April fair the peak. July and August are 40C+, so people live at night.",
            "Seville is about £45 to £80 a day per person on the ground, room aside. Tapas at the bar is the whole trick, and it is what locals do.",
        ],
        days: 2,
    },
    City {
        name: "copenhagen",
        title: "Copenhagen",
        guide: "Nyhavn for the picture, Tivoli if it's open, and the food market at Torvehallerne. Bikes are the real way to get around; hire one.",
        food: None,
        areas: Some(
            "In Copenhagen: by area, Norrebro for food and a local feel, Vesterbro for bars near the meatpacking district, or Indre By to be in the middle.",
        ),
        day_trips: Some(
            "Day trips from Copenhagen: Roskilde for the cathedral and the Viking ships (25 minutes), Malmo across the bridge (35 minutes) for a cheaper lunch, and the Louisiana museum up the coast, which is the best half-day of the lot.",
        ),
        other: &[
            "In Copenhagen: Tickets are by zone and a 24-hour city pass is about 90 kroner; a bike is what everyone actually uses and most of the centre is a five-minute ride. The harbour bus is a cheap boat trip.",
            "For Copenhagen: May to August: 18-22C, long days, and the harbour baths open from June. July is the busiest and the jazz festival, and November to February is dark by 4pm and cold.",
            "Copenhagen is about £70 to £120 a day per person on the ground, room aside. Bikes rather than taxis, and lunch from a bakery rather than a cafe.",
        ],
        days: 2,
    },
    City {
        name: "stockholm",
        title: "Stockholm",
        guide: "Gamla Stan, the Vasa ship museum, and a ferry out to one of the islands. Everything is walkable in the centre.",
        food: None,
        areas: Some(
            "In Stockholm: by area, Sodermalm for bars and views, Gamla Stan for the postcard and the crowds, or Ostermalm for somewhere smarter.",
        ),
        day_trips: Some(
            "Day trips from Stockholm: The archipelago by ferry from Stromkajen, Uppsala for the cathedral and the Vikings (40 minutes by train), and Drottningholm by boat for the palace theatre.",
        ),
        other: &[
            "In Stockholm: A single on the SL is about 39 kronor and a 72-hour pass about 270; the T-bana is the way round and the old town is walkable. Tickets are cashless, so buy on the app or tap a card.",
            "For Stockholm: May to August: 18-23C and light until late, with midsummer at the end of June. The archipelago is best from July to mid-August, and winter is dark with short days.",
            "Stockholm is about £65 to £110 a day per person on the ground, room aside. Alcohol is the shock: eat and drink where people live, not in Gamla Stan.",
        ],
        days: 2,
    },
    City {
        name: "athens",
        title: "Athens",
        guide: "the Acropolis first thing with a timed ticket, the Acropolis Museum, and the Plaka in the evening. It is hot and shadeless up there, so take water and a hat.",
        food: None,
        areas: Some(
            "In Athens: by area, Koukaki for the museum and local tavernas, Plaka for under the Acropolis, or Exarchia if you want the edgier side.",
        ),
        day_trips: Some(
            "Day trips from Athens: Cape Sounion for the temple at sunset, Delphi for the ruins (two and a half hours by bus), and Aegina or Hydra by ferry for an island day. The fast ferries are worth the extra.",
        ),
        other: &[
            "In Athens: A ticket is €1.20 and lasts 90 minutes on the metro, the trolleybuses and the trams; the airport line is separate. The centre under the Acropolis is walkable and the hills are not.",
            "For Athens: April, May, October and early November: 20-26C and the sites walkable. July and August are 35C+ and the Acropolis is a scramble by 10am, and the islands are at their best in June and September.",
            "Athens is about £40 to £75 a day per person on the ground, room aside. The ruins are the cost here, not the food: lunch in a taverna is cheap.",
        ],
        days: 2,
    },
    City {
        name: "istanbul",
        title: "Istanbul",
        guide: "Hagia Sophia, the Blue Mosque and the Grand Bazaar on the old side, and a ferry across to Asia for lunch. Buy an Istanbulkart for the trams and ferries.",
        food: None,
        areas: Some(
            "In Istanbul: by area, Sultanahmet for the sights on foot and quiet nights, Beyoglu for going out, or Kadikoy on the Asian side for food and locals.",
        ),
        day_trips: Some(
            "Day trips from Istanbul: The Princes' Islands by ferry (an hour and a half) where there are no cars, Bursa for the Ottoman tombs, and the Bosphorus village of Anadolu Kavagi at the mouth of the Black Sea.",
        ),
        other: &[
            "In Istanbul: Get an Istanbulkart from the machine and top it up: a ride is about 20 lira on the card and nearly double as a single, and one card covers several people. The trams and the ferries go where the traffic will not.",
            "For Istanbul: April, May, September and October: 18-25C and dry, and the Bosphorus ferries at their best. July and August are hot and humid, and December to February is cold and wet.",
            "Istanbul is about £30 to £60 a day per person on the ground, room aside. The tourist quarter is the dear part, and the ferries are cheap.",
        ],
        days: 3,
    },
    City {
        name: "new york",
        title: "New York",
        guide: "pick two neighbourhoods a day: the High Line and Chelsea, the Lower East Side, and Brooklyn across the bridge. Book the big attractions online and walk a lot.",
        food: None,
        areas: Some(
            "In New York: by area, Midtown for the sights and the noise, the Lower East Side for food and bars, or Brooklyn for cheaper rooms and 20 minutes on the train.",
        ),
        day_trips: Some(
            "Day trips from New York: Beacon for Dia:Beacon (80 minutes on Metro-North), Philadelphia (90 minutes on Amtrak) and the Hamptons by the LIRR. The Jersey shore is a longer day.",
        ),
        other: &[
            "In New York: Tap a contactless card or phone on the turnstile: a subway ride is $2.90 whatever the distance and trains run all night. Take the express where there is one, because the local stops every block.",
            "For New York: May to June and September to October: mild and walkable at 20-26C. July and August are hot and humid, January and February are cold with snow, and the Christmas weeks are the most expensive of the year.",
            "New York is about £90 to £160 a day per person on the ground, room aside. The city tax and the tip on every bill are what people forget.",
        ],
        days: 4,
    },
    City {
        name: "tokyo",
        title: "Tokyo",
        guide: "one district a day: Shibuya and Shinjuku, Asakusa and Ueno, and Tsukiji or Toyosu for breakfast. Get a Suica card for the trains and carry cash for small places.",
        food: None,
        areas: Some(
            "In Tokyo: by area, Shinjuku for the transport and the nightlife, Asakusa for the old town and quiet, or Shibuya if you want to be in the middle of it.",
        ),
        day_trips: Some(
            "Day trips from Tokyo: Nikko for the shrines and Kamakura for the Buddha (both about two hours), or Hakone for a view of Fuji if the sky is clear. Go on a weekday: the weekend trains are packed.",
        ),
        other: &[
            "In Tokyo: Get a Suica or Pasmo card, or add one to your phone: a metro ride is about 180 yen and the machines take cash. Trains stop around midnight, so plan the last one.",
            "For Tokyo: Late March to April for the blossom, and October to November for the autumn colour and the clearest skies. June is the rainy season, August is hot and humid, and the New Year week closes many places.",
            "Tokyo is about £70 to £120 a day per person on the ground, room aside. Convenience store breakfast and a rail pass change the total completely.",
        ],
        days: 4,
    },
    City {
        name: "kyoto",
        title: "Kyoto",
        guide: "Fushimi Inari at 7am before the crowds, the bamboo grove early too, and Gion at dusk. Buses are slow and packed: the subway plus walking is better.",
        food: None,
        areas: Some(
            "In Kyoto: by area, Kawaramachi for the centre and dinner, Gion for the old streets, or Arashiyama for quiet and the bamboo.",
        ),
        day_trips: Some(
            "Day trips from Kyoto: Nara for the deer and the great Buddha (45 minutes by train), Osaka (30 minutes) and Hikone for the castle on Lake Biwa. Fushimi Inari and Arashiyama are in Kyoto, so go at dawn instead.",
        ),
        other: &[
            "In Kyoto: Buses are a flat 230 yen and the subway is only two lines, so the bus does the temples. Tap an IC card, and walk the Gion and Higashiyama side rather than ride.",
            "For Kyoto: Late March to early April for the blossom, and mid-November for the maples. July and August are hot and humid, and the New Year week is the busiest of the year.",
        ],
        days: 3,
    },
    City {
        name: "reykjavik",
        title: "Reykjavik",
        guide: "the harbour and Hallgrímskirkja, and use the city as a base for the Golden Circle. Book the Blue Lagoon ahead and don't rinse the cave water off in the showers.",
        food: None,
        areas: Some(
            "In Reykjavik: by area, 101 downtown to walk to everything, or Laugavegur for the shops and bars and a short walk to the harbour.",
        ),
        day_trips: Some(
            "Day trips from Reykjavik: The Golden Circle - Thingvellir, Geysir and Gullfoss - is the full day out and needs a car or a tour, the Blue Lagoon is 45 minutes by bus and booked ahead, and Snaefellsnes is a very long day.",
        ),
        other: &[
            "In Reykjavik: The centre is walkable and a bus ticket is about 630 kronur, needed only for the pools and the outer suburbs. Tours collect from stops near the hotels rather than the door, so check which stop is yours.",
            "For Reykjavik: June to August gives midnight sun, open highland roads and puffins; February and March are best for the northern lights. November is dark, wet and icy, so it is the one month I would skip.",
        ],
        days: 3,
    },
    City {
        name: "venice",
        title: "Venice",
        guide: "get lost in Cannaregio and Dorsoduro away from St Mark's, cross the Grand Canal on the traghetto, and go early or late. Day-trippers leave by 5pm and the city feels completely different.",
        food: None,
        areas: Some(
            "In Venice: by area, Cannaregio for local life and canals, Dorsoduro for art and quiet, or San Marco for the middle of it and the day trippers.",
        ),
        day_trips: Some(
            "Day trips from Venice: Verona (an hour by train) for the arena, Padua for the Scrovegni chapel, and the islands in the lagoon: Murano for glass and Burano for the coloured houses, on a vaporetto day pass.",
        ),
        other: &[
            "In Venice: The vaporetto is the road here: a single is €9.50 and a 24-hour pass about €25, so the pass pays off after two rides. Walk between the Rialto and San Marco and take the boat for the islands.",
            "For Venice: April, May, late September and October: 18-24C, warm, not crowded and everything open. July and August are hot and packed, and November to January brings acqua alta.",
            "Venice is about £70 to £120 a day per person on the ground, room aside. The water buses add up, so buy a multi-day pass once you are there.",
        ],
        days: 2,
    },
    City {
        name: "boston",
        title: "Boston",
        guide: "walk the Freedom Trail, eat in the North End, and cross to Cambridge for a half-day. The T is small and easy.",
        food: None,
        areas: Some(
            "In Boston: by area, Back Bay for the shopping and the river, Beacon Hill for the postcard streets, or the North End for Italian food.",
        ),
        day_trips: Some(
            "Day trips from Boston: Salem for the witch museum (30 minutes by train), Concord and Lexington for the revolution, and Providence (an hour on Amtrak). Cape Cod is 90 minutes by car.",
        ),
        other: &[
            "In Boston: A CharlieCard is $2.40 a ride on the T and contactless works at the gates too. The subway covers the centre and the Freedom Trail is a walk between the stops.",
            "For Boston: May, June, September and October: 18-26C, and October for the foliage. July and August are hot and humid, and January and February are cold with snow.",
        ],
        days: 2,
    },
    City {
        name: "bath",
        title: "Bath",
        guide: "the Roman baths and the Pump Room, the Royal Crescent and the Circus, and a walk along the canal. It is small enough for a day, and the baths sell timed tickets that are worth booking the night before.",
        food: None,
        areas: Some(
            "In Bath: by area, the centre by the Abbey to walk to the baths and the shops, or the Royal Crescent end for Georgian streets and quiet.",
        ),
        day_trips: Some(
            "Day trips from Bath: Stonehenge (an hour by bus) with a booked slot, Lacock for the village and the abbey, and Bradford-on-Avon for the canal and the tithe barn, 15 minutes by train.",
        ),
        other: &[
            "For Bath: May to September for the baths, the crescent and the canal; April and October are quieter and still fine. December is the Christmas market, which is the busiest weekend of the year.",
        ],
        days: 1,
    },
    City {
        name: "ljubljana",
        title: "Ljubljana",
        guide: "the riverside cafes and the Triple Bridge, the castle on the hill for the view, and a walk through Tivoli park. It is small and flat, so a day covers it before you head out to Lake Bled.",
        food: None,
        areas: Some(
            "In Ljubljana: by area, the old town by the river for cafes and everything on foot, or Tivoli park side for green and quiet.",
        ),
        day_trips: Some(
            "Day trips from Ljubljana: Lake Bled (an hour by bus) for the island church, Postojna for the caves and the castle, and Skocjan for the bigger cave, which is the one to pick if you only do one.",
        ),
        other: &[
            "For Ljubljana: May, June and September: 20-26C, and Lake Bled is warm enough to swim from June. July and August are the busiest, and winter is cold and quiet.",
        ],
        days: 1,
    },
    City {
        name: "krakow",
        title: "Krakow",
        guide: "the main square and the cloth hall, Wawel castle, and Kazimierz for food and bars. Book Auschwitz-Birkenau a few days ahead and take the early bus: it is a full half-day.",
        food: None,
        areas: Some(
            "In Krakow: by area, the Old Town for the square and the sights on foot, or Kazimierz for bars, food and a local feel.",
        ),
        day_trips: Some(
            "Day trips from Krakow: Auschwitz-Birkenau (90 minutes by bus, and book a timed entry), the Wieliczka salt mine (30 minutes by train) and Zakopane for the mountains (two hours). Auschwitz is a heavy half-day, not a sight.",
        ),
        other: &[
            "In Krakow: The old town and the castle are walkable; a 20-minute bus or tram ticket is 4 zloty from a machine, and the airport train is separate. The trams are old and cheap.",
            "For Krakow: May to September: 20-25C and the main square at its best; April and October are quiet and still fine. December to February is cold with snow and a Christmas market.",
            "Krakow is about £30 to £60 a day per person on the ground, room aside. The old town is the pricey part and the tram takes you out of it.",
        ],
        days: 2,
    },
    City {
        name: "budapest",
        title: "Budapest",
        guide: "the parliament and the Danube bank, the thermal baths (Szechenyi or Gellert), and the ruin bars in the old Jewish quarter. Buy a travel card for the trams and metro; the city is long, not wide.",
        food: None,
        areas: Some(
            "In Budapest: by area, District V for the centre, District VII for the ruin bars and the noise, or the Buda side for quiet and the views.",
        ),
        day_trips: Some(
            "Day trips from Budapest: Szentendre for the artist town and the Danube bend (40 minutes on the H5 train), Esztergom for the basilica, and Lake Balaton, which is two hours and wants a night.",
        ),
        other: &[
            "In Budapest: A single is 350 forints and a 24-hour ticket about 2,500, from the machine, validated before you board. Tram 2 along the Danube is the best ride in the city and costs the same as any other.",
            "For Budapest: April to June and September: 20-26C, and the thermal baths are good in cool weather too. July and August are 30C+, and December brings the Christmas markets along the Danube.",
            "Budapest is about £30 to £60 a day per person on the ground, room aside. The baths and the ruin bars are where the money goes.",
        ],
        days: 2,
    },
    City {
        name: "brussels",
        title: "Brussels",
        guide: "the Grand Place, the comic-strip museum and the art nouveau houses, with the waffles and fries people come for. Two days covers it, and Bruges or Ghent is an hour away by train.",
        food: None,
        areas: None,
        day_trips: Some(
            "Day trips from Brussels: Bruges (an hour by train) for the canals, Ghent (35 minutes) for the old port, and Leuven for the university town. Book Bruges ahead in summer, it fills up.",
        ),
        other: &[
            "In Brussels: A single on the metro, tram or bus is €2.60 and you tap a bank card at the gate. The Grand Place is 20 minutes from most of the centre on foot.",
            "For Brussels: May, June and September: 17-22C, and the Grand Place flower carpet in mid-August every second year. July and August are warm and quiet at weekends, and winter is grey and wet.",
            "Brussels is about £55 to £90 a day per person on the ground, room aside. The centre near the Grand Place is where the prices double.",
        ],
        days: 2,
    },
    City {
        name: "milan",
        title: "Milan",
        guide: "the Duomo and its roof terraces, the Last Supper (book weeks ahead) and the Navigli canals in the evening. Most people use it as a base: Lake Como is an hour by train.",
        food: None,
        areas: Some(
            "In Milan: by area, Brera for chic and central, the Navigli for canals and nightlife, or Porta Romana for quieter streets.",
        ),
        day_trips: Some(
            "Day trips from Milan: Lake Como for Bellagio (an hour by train to Como, then a boat), Bergamo for the old town (50 minutes) and Turin (an hour on the fast train). The Last Supper needs booking weeks ahead.",
        ),
        other: &[
            "In Milan: A €2.20 ticket covers 90 minutes of metro, trams and buses, and you can tap a bank card at the metro gate. The centre is small enough to walk, the metro is for the outer bits.",
            "For Milan: April to June and September: 20-27C and the aperitivo season. July and August are hot and humid and the city half empties, and January is cold with the sales on.",
        ],
        days: 2,
    },
    City {
        name: "naples",
        title: "Naples",
        guide: "the archaeology museum first, then pizza in the old town, and Pompeii or Herculaneum on a day trip by train. It is loud and fast, so keep bags in front of you and use the metro rather than the buses.",
        food: None,
        areas: Some(
            "In Naples: by area, Chiaia for the seafront and somewhere smart, the Centro Storico for food and chaos, or Vomero for the hill and a calm night.",
        ),
        day_trips: Some(
            "Day trips from Naples: Pompeii and Herculaneum (40 minutes on the Circumvesuviana), the Amalfi coast by ferry or bus, and Ischia for a slower island day. Vesuvius is a bus and a walk from Ercolano.",
        ),
        other: &[
            "In Naples: A ticket is €1.10 and lasts 90 minutes on the metro, the buses and the funicular, bought at a tabacchi. Metro line 1 is worth riding for the art in the stations.",
            "For Naples: April to June and September: 22-27C, with the ferries to Capri and the Amalfi coast running. July and August are hot and busy, and November to March is quiet and wet.",
        ],
        days: 2,
    },
    City {
        name: "munich",
        title: "Munich",
        guide: "Marienplatz and the English Garden, the beer halls, and a day out to Neuschwanstein or Dachau if you have the time. The trains are excellent; buy a day ticket rather than singles.",
        food: None,
        areas: Some(
            "In Munich: by area, the Altstadt for the beer halls and the sights, Schwabing for a student feel and the English Garden, or near the station for day trips.",
        ),
        day_trips: Some(
            "Day trips from Munich: Neuschwanstein (two hours by train and bus), Salzburg (90 minutes) and the Bavarian lakes. Buy a Bayern ticket: it covers the regional trains for a group.",
        ),
        other: &[
            "In Munich: A single is €3.90 and a day ticket for the inner zone about €8.60; the U-Bahn and S-Bahn are the way round and both run late. Validate the ticket before you get on.",
            "For Munich: May to September: 20-25C, beer gardens and the Alps an hour away; late September is Oktoberfest, which doubles hotel prices. December brings the Christmas markets.",
        ],
        days: 2,
    },
    City {
        name: "zurich",
        title: "Zurich",
        guide: "the old town on both sides of the river, a boat on the lake and the view from Uetliberg. Everything is expensive, so eat lunch from a bakery or a supermarket counter and save the restaurants for one evening.",
        food: None,
        areas: Some(
            "In Zurich: by area, the Niederdorf for the old town on foot, or the west side near the river for bars and cheaper rooms.",
        ),
        day_trips: Some(
            "Day trips from Zurich: Lucerne (45 minutes by train), the Rhine Falls (an hour) and Interlaken for the mountains (two hours). A travel pass covers the trains and the boats.",
        ),
        other: &[
            "In Zurich: Buy by zone on the ZVV app or at a machine: a trip across the city is about 4.30 francs and there are no barriers. The trams go everywhere and run to the minute.",
            "For Zurich: May to September for the lake, the old town and the mountain railways, at 22-26C in midsummer. December to February is cold and clear, which is when the Alps look their best.",
            "Zurich is about £95 to £160 a day per person on the ground, room aside. Everything is dear, so a supermarket lunch and the lake save the most.",
        ],
        days: 2,
    },
    City {
        name: "oslo",
        title: "Oslo",
        guide: "the opera house roof you can walk on, the Vigeland park and the fjord boats. It is compact but pricey: buy a seven-day transport pass, and take the train to Bergen if you have three days.",
        food: None,
        areas: Some(
            "In Oslo: by area, Grunerlokka for cafes and the river, the centre for the harbour and the station, or Majorstuen for quiet.",
        ),
        day_trips: Some(
            "Day trips from Oslo: The fjord by boat from the city hall pier, Drobak for the fortress and the old town (an hour by bus), and Bergen by train, which is the best rail journey in the country and wants a night.",
        ),
        other: &[
            "In Oslo: A single on Ruter is about 40 kroner and a 24-hour ticket about 120; the trams and the metro are clean and the centre is walkable. Buy on the app before boarding.",
            "For Oslo: May to September: 18-23C, and the fjord islands are a ferry ride from the centre. June has the longest days, and November to February is cold and dark, with skiing an hour out of town.",
        ],
        days: 2,
    },
    City {
        name: "helsinki",
        title: "Helsinki",
        guide: "the cathedral square, the market hall and the design district, with a ferry to Suomenlinna for half a day. In summer the evenings are light until midnight, so eat late.",
        food: None,
        areas: Some(
            "In Helsinki: by area, Kallio for bars and cheaper rooms, the Design District for the centre on foot, or Kamppi for the transport hub.",
        ),
        day_trips: Some(
            "Day trips from Helsinki: Suomenlinna by ferry (15 minutes on the same ticket as the trams), Porvoo for the wooden old town (an hour by bus) and Tallinn by ferry (two hours, and take your passport).",
        ),
        other: &[
            "In Helsinki: A ticket inside the city is about €3 and a day ticket €9, and the trams cover the centre on their own. The ferry to Suomenlinna runs on the same ticket.",
            "For Helsinki: June to August: 18-23C with almost no night, and the archipelago ferries running. January and February are the coldest, with the best chance of snow, and December is dark.",
        ],
        days: 2,
    },
    City {
        name: "tallinn",
        title: "Tallinn",
        guide: "the walled old town, the viewing platforms on Toompea, and the cafes in Kalamaja. A day covers the old town; take a ferry to Helsinki or a bus to Riga if you have longer.",
        food: None,
        areas: Some(
            "In Tallinn: by area, the old town for the walls and the square, or Kalamaja for wooden houses and cafes and a short walk in.",
        ),
        day_trips: Some(
            "Day trips from Tallinn: Lahemaa national park for the manors and the coast (an hour by bus), Tartu (two and a half hours) and Helsinki by ferry in two hours.",
        ),
        other: &[
            "For Tallinn: June to August: 18-22C, the old town at its best and white nights in June. December to February is cold and snowy, and the Christmas market is one of the prettiest in Europe.",
        ],
        days: 1,
    },
    City {
        name: "dubrovnik",
        title: "Dubrovnik",
        guide: "walk the city walls early or after 5pm once the cruise crowds have gone, then the cable car for the view. Take water and a hat up there, and swim off the rocks by the Buza gate.",
        food: None,
        areas: Some(
            "In Dubrovnik: by area, inside the walls for the postcard and the noise, or Lapad and Babin Kuk for beaches and calmer nights and a bus into town.",
        ),
        day_trips: Some(
            "Day trips from Dubrovnik: Mostar in Bosnia (two and a half hours by bus) for the bridge, Montenegro for Kotor and the bay, and the Elaphiti islands by boat. Take your passport for Mostar and Montenegro.",
        ),
        other: &[
            "For Dubrovnik: May, June, September and early October: 22-27C and the city walls walkable. July and August are the peak and the cruise ships fill the old town by 10am, and winter is quiet.",
            "Dubrovnik is about £55 to £95 a day per person on the ground, room aside. The old town is the top of that range and the walls are paid for.",
        ],
        days: 2,
    },
    City {
        name: "marrakech",
        title: "Marrakech",
        guide: "the Jemaa el-Fna at dusk, the Bahia palace and the Ben Youssef school, and a hammam. Agree taxi prices before you get in, and buy in the souks slowly: the first price is never the price.",
        food: None,
        areas: Some(
            "In Marrakech: by area, a riad inside the medina for the atmosphere, Gueliz for modern hotels and quiet, or the Kasbah for the sights.",
        ),
        day_trips: Some(
            "Day trips from Marrakech: The Atlas mountains and Ourika valley (an hour by car), Essaouira for the seafront and the fish (two and a half hours by bus), and Ait Benhaddou, which is a long day or a night on the way to the desert.",
        ),
        other: &[
            "In Marrakech: Small taxis are the way round: 20 to 30 dirham inside the city, agreed before you get in. The medina is walkable and a guide the first day is money well spent.",
            "For Marrakech: March to May and October to November: 20-28C by day, cool at night, no extreme heat. July and August are 40C+ in the city, and the desert is cold at night all year.",
        ],
        days: 2,
    },
    City {
        name: "santorini",
        title: "Santorini",
        guide: "Oia for the sunset once, but stay in Imerovigli or Pyrgos so you can sleep, and take a boat out to the volcano. Book months ahead for July and August, or come in May or September for the same light and half the people.",
        food: None,
        areas: Some(
            "In Santorini: by area, Imerovigli or Pyrgos for the view without the crowds, or Oia for the sunset and the people who come to see it.",
        ),
        day_trips: Some(
            "Day trips from Santorini: Oia for the sunset and Akrotiri for the Minoan town under the ash are the day, and both are 20 to 30 minutes by bus from Fira; boats to Thirasia and the volcano leave from the old port.",
        ),
        other: &[
            "For Santorini: May, June, September and early October: 24-27C, warm sea and no crush. July and August are 30C+ with the cruise crowds landing mid-morning, and many hotels close from November to March.",
        ],
        days: 2,
    },
    City {
        name: "valencia",
        title: "Valencia",
        guide: "the City of Arts and Sciences, the old town round the cathedral, and the beach a short tram ride away. This is the home of paella, so eat it at lunch, not in the evening.",
        food: None,
        areas: Some(
            "In Valencia: by area, the Carmen for the old town and bars, or the beach end for sand and the tram.",
        ),
        day_trips: Some(
            "Day trips from Valencia: The Albufera for the rice fields and the lagoon (30 minutes by bus), Sagunto for the Roman theatre, and Requena for the wine. The City of Arts is in the city, not a day out.",
        ),
        other: &[
            "In Valencia: A metro ticket is €1.50 and gets you from the old town to the City of Arts; the centre itself is walkable and the old river bed is now a park you can walk through.",
            "For Valencia: April to June and September to October: 20-26C, and the sea still swimmable in October. July and August are 30C+ and busy, and March is Las Fallas if you want the crowds.",
            "Valencia is about £45 to £80 a day per person on the ground, room aside. The beach is free and it is cheaper than Barcelona for the same food.",
        ],
        days: 2,
    },
    City {
        name: "granada",
        title: "Granada",
        guide: "the Alhambra first (book weeks ahead, it sells out), then the Albaicin at sunset from the San Nicolas viewpoint, with free tapas with drinks. Two nights is right: one is not enough for both.",
        food: None,
        areas: Some(
            "In Granada: by area, the Albaicin for the views and the old streets, or the centre near the cathedral for tapas on the doorstep.",
        ),
        day_trips: Some(
            "Day trips from Granada: The Alhambra is the day, not a detour: book a timed ticket weeks ahead. The Alpujarras villages are an hour and a half by bus, and Cordoba is two and a half hours by train, so it wants a night.",
        ),
        other: &[
            "In Granada: Walk the centre and take a bus up the hill for the Alhambra: a ticket is €1.40 and the C30 and C32 go from the Plaza Nueva. Taxis are cheap for the station.",
            "For Granada: April to June and September to October: 18-28C, and the Alhambra bookable without a fight. July and August are 35C+ inland, and winter is cold up at the Alhambra.",
        ],
        days: 2,
    },
    City {
        name: "san sebastian",
        title: "San Sebastian",
        guide: "the two beaches, the pintxos bars in the old town, and the walk up Monte Igueldo for the view. Eat standing at the bar: one or two pintxos a place, then move on.",
        food: None,
        areas: Some(
            "In San Sebastian: by area, the old town for pintxos on the doorstep, or Gros for the surf beach and quieter nights.",
        ),
        day_trips: Some(
            "Day trips from San Sebastian: Bilbao for the Guggenheim (an hour and a quarter by bus), Saint-Jean-de-Luz across the French border (40 minutes) and Getaria for the txakoli wine. Hondarribia is 25 minutes by bus.",
        ),
        other: &[
            "In San Sebastian: Everything is walkable: the two beaches are 15 minutes apart. A bus ticket is about €2 for the ride out to Monte Igueldo, and there is a funicular for the view.",
            "For San Sebastian: June to September: 20-24C for the beach and the pintxos, with the film festival at the end of September. July and August are the busiest, and the rest of the year is wet.",
        ],
        days: 2,
    },
    City {
        name: "san francisco",
        title: "San Francisco",
        guide: "the Golden Gate on foot or by bike, the ferry building, and one neighbourhood properly rather than five badly. Layers matter: the fog rolls in and the temperature drops ten degrees by the water.",
        food: None,
        areas: Some(
            "In San Francisco: by area, the Mission for food and the sun, North Beach for Italian and the wharf, or SoMa for the middle of it.",
        ),
        day_trips: Some(
            "Day trips from San Francisco: Muir Woods and Sausalito (ferry, then a shuttle you book ahead), Napa by car, and Santa Cruz over the hill. Yosemite is four hours, so it wants a night.",
        ),
        other: &[
            "In San Francisco: Tap a Clipper card or a phone on Muni: $2.50 buys 90 minutes on the buses and the metro. BART is a separate fare for the airport, and the hills are real, so take the cable car once.",
            "For San Francisco: September and October: the warmest, clearest months at 18-24C. July and August are foggy and cool, and the rain comes November to March.",
            "San Francisco is about £90 to £160 a day per person on the ground, room aside. The hotel tax and the tip are the parts people forget.",
        ],
        days: 2,
    },
    City {
        name: "chicago",
        title: "Chicago",
        guide: "the architecture boat tour, the Art Institute and the river walk, then deep dish once for the experience. The Loop is walkable and the trains from O'Hare run all night.",
        food: None,
        areas: Some(
            "In Chicago: by area, River North to walk to the lake and the galleries, Wicker Park for bars, or the Loop for the trains.",
        ),
        day_trips: Some(
            "Day trips from Chicago: Milwaukee (90 minutes on Amtrak), the Indiana Dunes for the beach (an hour by train and bus) and the Baha'i temple at Wilmette, which is free and half a day on the Purple line.",
        ),
        other: &[
            "In Chicago: A Ventra card or a contactless tap is $2.50 a ride on the L and the buses, and most L lines run all night. The loop is walkable and the L is for everything outside it.",
            "For Chicago: May, June and September: 20-27C and the lakefront at its best. July and August are hot and humid, and January and February are brutal with the wind off the lake.",
        ],
        days: 2,
    },
    City {
        name: "los angeles",
        title: "Los Angeles",
        guide: "pick two areas and expect to drive between them: Venice or Santa Monica, the Griffith observatory at sunset, and one studio tour if you want one. It is far bigger than it looks on a map.",
        food: None,
        areas: Some(
            "In Los Angeles: by area, Santa Monica for the beach, Hollywood for the sights and grit, or Downtown for the arts and the bars.",
        ),
        day_trips: Some(
            "Day trips from Los Angeles: Santa Monica and Malibu down the coast, Disneyland in Anaheim (an hour by car) and Catalina island by ferry from Long Beach. Almost all of it needs a car.",
        ),
        other: &[
            "In Los Angeles: A TAP card is $1.75 a ride and a day caps at about $5, and the metro is fine for Hollywood and Santa Monica. The traffic is the reason to plan, so stay near where you are going.",
            "For Los Angeles: March to May and September to November: 20-26C and clear. June to August is warm but the marine layer can sit all day at the beaches, and winter is mild with the odd storm.",
        ],
        days: 3,
    },
    City {
        name: "toronto",
        title: "Toronto",
        guide: "the islands for the skyline view, Kensington market and the Distillery district, and the St Lawrence market for lunch. The streetcars work well, and Niagara Falls is a day trip by train.",
        food: None,
        areas: Some(
            "In Toronto: by area, King West for bars and the centre, Kensington Market for food and a local feel, or the Distillery District for cobbles.",
        ),
        day_trips: Some(
            "Day trips from Toronto: Niagara Falls (two hours by train or bus, and the falls are free), Niagara-on-the-Lake for the wineries on the same trip, and Stratford for the Shakespeare festival.",
        ),
        other: &[
            "In Toronto: A PRESTO tap is about $3.30 for two hours of streetcars, buses and subway. The centre is walkable and the PATH tunnels keep you out of the weather.",
            "For Toronto: May to June and September to October: 18-25C. July and August are hot and humid, and December to March is cold with snow.",
        ],
        days: 2,
    },
    City {
        name: "sydney",
        title: "Sydney",
        guide: "the harbour walk from the Opera House to Mrs Macquarie's chair, the Bondi to Coogee coastal path, and the ferry to Manly. Use the ferries as sightseeing: the Opal card caps the day.",
        food: None,
        areas: Some(
            "In Sydney: by area, Surry Hills for cafes and bars, Circular Quay for the harbour on the doorstep, or Bondi for the beach.",
        ),
        day_trips: Some(
            "Day trips from Sydney: The Blue Mountains (two hours by train), the Hunter Valley for wine (three hours, better on a tour) and Manly or Watsons Bay by ferry for a beach day close in.",
        ),
        other: &[
            "In Sydney: Tap on and off with a bank card or an Opal: a city trip is about A$4, and the train from the airport is a separate A$19. The ferries are the best value sightseeing in the city.",
            "For Sydney: September to November and March to May: 20-26C, warm without the scorch. December to February is 30C+ and the beaches are packed, and June to August is mild, with whale watching.",
            "Sydney is about £70 to £120 a day per person on the ground, room aside. Coffee and transport are dear, and the beaches cost nothing.",
        ],
        days: 3,
    },
    City {
        name: "singapore",
        title: "Singapore",
        guide: "the hawker centres for every meal, Gardens by the Bay in the evening, and the southern ridges walk. It is hot and humid all year, so walk early and save the air-conditioned museums for midday.",
        food: None,
        areas: Some(
            "In Singapore: by area, Chinatown for food and value, Marina Bay for the sights, or Orchard for shopping.",
        ),
        day_trips: Some(
            "Day trips from Singapore: Johor Bahru across the causeway (the bus queue is long at weekends), Batam by ferry, and Malacca, which is three and a half hours each way. The Botanic Gardens and the southern ridges are free and in the city.",
        ),
        other: &[
            "In Singapore: Tap a contactless card on the MRT and the buses: a trip is about S$2 and the system is spotless. It stops around midnight, and a taxi or Grab after that.",
            "For Singapore: Any month works: it is 30C and humid all year, with rain in short bursts. February to April is the driest, November to January the wettest, and the haze can arrive in September.",
            "Singapore is about £60 to £110 a day per person on the ground, room aside. Hawker centres keep food cheap and hotels are what pushes it up.",
        ],
        days: 3,
    },
    City {
        name: "bangkok",
        title: "Bangkok",
        guide: "the Grand Palace and Wat Pho early, the river boats rather than taxis, and street food in Chinatown at night. April and May are the hottest months, and from June the rain comes in short afternoon bursts.",
        food: None,
        areas: Some(
            "In Bangkok: by area, Sukhumvit for hotels and food, the riverside for views and boats, or Khao San for the backpacker end.",
        ),
        day_trips: Some(
            "Day trips from Bangkok: Ayutthaya for the ruined temples (an hour and a half by train), Kanchanaburi for the bridge over the River Kwai, and the floating market at Damnoen Saduak, which needs a very early start.",
        ),
        other: &[
            "In Bangkok: Get a Rabbit card for the BTS Skytrain: a ride is 17 to 45 baht and it goes over the traffic rather than through it. The metro is a separate ticket, and the river boats are the cheapest way along the Chao Phraya.",
            "For Bangkok: November to February: dry, warm and clear, which is why it is the season. March to May is hot and hazy, and June to October is the monsoon with short, heavy afternoon rain.",
            "Bangkok is about £25 to £55 a day per person on the ground, room aside. Street food is a few pounds a meal, and the skytrain beats the traffic.",
        ],
        days: 3,
    },
    City {
        name: "dubai",
        title: "Dubai",
        guide: "the old creek and its abra boats, the desert at sunset, and one tall building rather than three. From June to September it is over 40C and people live indoors; November to March is the season.",
        food: None,
        areas: Some(
            "In Dubai: by area, Dubai Marina for the beach and towers, Downtown for the Burj and the malls, or Deira for the souks and lower prices.",
        ),
        day_trips: Some(
            "Day trips from Dubai: Abu Dhabi for the grand mosque and the Louvre (90 minutes by car or bus), the desert for a dune drive, and Hatta for the mountains and the kayaks. The Abu Dhabi bus leaves from Al Ghubaiba.",
        ),
        other: &[
            "In Dubai: A Nol card covers the metro and the buses, and a ride is about 3 to 7.5 dirham. The metro is driverless and clean and goes to the malls; taxis are cheap by European standards.",
            "For Dubai: November to March, when it is 20-30C and you can be outside. From June to September it is over 40C, so people live in air conditioning and the hotel prices fall.",
            "Dubai is about £70 to £130 a day per person on the ground, room aside. Alcohol is the thing that pushes it up, and it is only sold in hotels.",
        ],
        days: 3,
    },
    City {
        name: "cape town",
        title: "Cape Town",
        guide: "Table Mountain first thing if the cloud is off it, the Cape Point drive, and Bo-Kaap and the V&A for the afternoon. The wind and cloud decide your plan, so keep the mountain day flexible.",
        food: None,
        areas: Some(
            "In Cape Town: by area, the city bowl for the restaurants and the mountain, Camps Bay for the beach, or Stellenbosch if you want the wine farms.",
        ),
        day_trips: Some(
            "Day trips from Cape Town: The Cape of Good Hope and the Boulders Beach penguins, which is a full day by car, Stellenbosch and Franschhoek for wine (an hour), and Hermanus for whales from June to November.",
        ),
        other: &[
            "In Cape Town: There is no useful metro for visitors: use Uber or a MyCiTi bus, which is about 8 rand a trip on a card, and hire a car for the Winelands and the Cape. Agree the fare before you get in a taxi.",
            "For Cape Town: November to March: dry, 25-28C, the beaches and the Table Mountain cableway at their best. July and August are wet and windy, which is the whale season in Hermanus.",
            "Cape Town is about £35 to £70 a day per person on the ground, room aside. The rand helps, and the wine is cheaper than the food.",
        ],
        days: 4,
    },
    City {
        name: "mexico city",
        title: "Mexico City",
        guide: "the Zocalo and the Templo Mayor, the Frida Kahlo house in Coyoacan, and the food markets. It sits at 2,240m, so take the first day slowly, and use the metro in the day and taxis at night.",
        food: None,
        areas: Some(
            "In Mexico City: by area, Roma and Condesa for leafy streets and cafes, Centro for the sights, or Coyoacan for quiet.",
        ),
        day_trips: Some(
            "Day trips from Mexico City: Teotihuacan for the pyramids (an hour by bus from the north terminal), Puebla and Cholula (two hours by bus), and Taxco for silver, which is a long day.",
        ),
        other: &[
            "In Mexico City: The metro is 5 pesos a ride on a card from the machine, fast and very crowded at rush hour. Uber is cheap and easier at night, and the pink line covers most of the centre.",
            "For Mexico City: March to May: dry, warm and clear, before the rains. June to October is the wet season with short afternoon storms, and December to February is dry with cold nights.",
        ],
        days: 3,
    },
    City {
        name: "havana",
        title: "Havana",
        guide: "the old town in the morning, the Malecon at sunset, and a 1950s car once for the ride rather than the tour. Bring cash: foreign cards often do not work, so take enough for the whole trip.",
        food: None,
        areas: Some(
            "In Havana: by area, Habana Vieja for the old streets on foot, or Vedado for 1950s hotels and a local evening.",
        ),
        day_trips: Some(
            "Day trips from Havana: Vinales for the tobacco valley (three hours by bus, better with a night), the beach at Santa Maria del Mar (30 minutes by taxi), and Matanzas for the river and the poets.",
        ),
        other: &[
            "For Havana: November to April: dry and 24-28C. May to October is hot and wet, and August to October is hurricane season, though the rain usually comes in short afternoon storms.",
        ],
        days: 2,
    },
    City {
        name: "buenos aires",
        title: "Buenos Aires",
        guide: "Recoleta and the cemetery, San Telmo on a Sunday for the market, and a steak and a tango show in the evening. People eat at 10pm, so shift your day later or you will be dining alone.",
        food: None,
        areas: Some(
            "In Buenos Aires: by area, Palermo for restaurants and going out, Recoleta for the grand streets, or San Telmo for the old quarter.",
        ),
        day_trips: Some(
            "Day trips from Buenos Aires: Tigre for the delta boats (an hour by train), Colonia in Uruguay by ferry (an hour, and take your passport), and San Antonio de Areco for the gaucho town.",
        ),
        other: &[
            "For Buenos Aires: March to May and September to November: 18-25C and the parks at their best. December to February is hot and humid and the city empties in January, and July and August are cool and damp.",
        ],
        days: 3,
    },
    City {
        name: "rio de janeiro",
        title: "Rio De Janeiro",
        guide: "Copacabana and Ipanema for the beach, Sugarloaf and Christ the Redeemer early before the crowds, and Santa Teresa for the afternoon. Take nothing valuable to the beach and use the metro after dark.",
        food: None,
        areas: Some(
            "In Rio De Janeiro: by area, Ipanema for the beach and the bars, or Santa Teresa for the old streets on the hill.",
        ),
        day_trips: Some(
            "Day trips from Rio De Janeiro: Petropolis for the imperial palace (90 minutes by bus), Niteroi for the museum and the beaches across the bay, and Buzios, which is three hours and wants a night.",
        ),
        other: &[
            "For Rio De Janeiro: December to March is hot, 28-32C, and Carnival falls in February or early March. May to October is cooler and drier, and the sea is still warm in September.",
        ],
        days: 3,
    },
    City {
        name: "hanoi",
        title: "Hanoi",
        guide: "the old quarter on foot with a pho stop, Hoan Kiem lake in the morning, and a coffee on a balcony over the traffic. Cross the road by walking steadily: the mopeds go round you, not through you.",
        food: None,
        areas: Some(
            "In Hanoi: by area, the old quarter for the street food and the lake on foot, or the French quarter for quieter nights.",
        ),
        day_trips: Some(
            "Day trips from Hanoi: Halong Bay for the boats (two and a half hours by bus, and a night on the water is better than a day), Ninh Binh for the river rowing boats, and Sapa by night train for the rice terraces.",
        ),
        other: &[
            "For Hanoi: October to December: 20-26C, dry and clear. January to March is cool and damp with drizzle, and May to September is hot and wet, with the heaviest rain in July and August.",
        ],
        days: 3,
    },
    City {
        name: "manchester",
        title: "Manchester",
        guide: "the Northern Quarter for food and records, the Museum of Science and Industry in the old station, and a tram out to Salford Quays for the Lowry and the Imperial War Museum North. Book the Old Trafford or Etihad tour ahead.",
        food: None,
        areas: None,
        day_trips: Some(
            "Day trips from Manchester: the Peak District (40 minutes by train to Edale), Liverpool (50 minutes) and Chester for the walls and the rows. York is 90 minutes.",
        ),
        other: &[],
        days: 3,
    },
    City {
        name: "liverpool",
        title: "Liverpool",
        guide: "the Albert Dock and the two cathedrals, the Cavern and the Beatles Story if you want the music, and the ferry across the Mersey for the skyline. The Walker and the World Museum are free.",
        food: None,
        areas: None,
        day_trips: Some(
            "Day trips from Liverpool: Chester (40 minutes by train), the Wirral coast, and Manchester in under an hour. North Wales is a longer day by train.",
        ),
        other: &[],
        days: 3,
    },
    City {
        name: "birmingham",
        title: "Birmingham",
        guide: "the canals round Brindleyplace, the museum and art gallery, and a curry in the Balti Triangle rather than the middle of town. The library roof is free and has the best view in the city.",
        food: None,
        areas: None,
        day_trips: Some(
            "Day trips from Birmingham: Stratford-upon-Avon (40 minutes by train), Warwick castle and the Cotswolds by car. The Black Country museum is a good wet-day half-day.",
        ),
        other: &[],
        days: 3,
    },
    City {
        name: "leeds",
        title: "Leeds",
        guide: "the Victoria Quarter and Kirkgate Market in the centre, then Headingley or Chapel Allerton for dinner. The Royal Armouries is free, and so are the city's galleries.",
        food: None,
        areas: None,
        day_trips: Some(
            "Day trips from Leeds: York (25 minutes by train), the Yorkshire Dales (an hour by car) and Harrogate for a spa afternoon. Manchester is 50 minutes by train if you want a city.",
        ),
        other: &[],
        days: 3,
    },
    City {
        name: "sheffield",
        title: "Sheffield",
        guide: "the Winter Garden and the Peace Gardens, then Kelham Island for food and a brewery tap. The Peak District is half an hour by bus, which is the reason most people go.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
    City {
        name: "newcastle",
        title: "Newcastle",
        guide: "the Quayside and the Millennium Bridge, Grainger Town for the arcades, and the Baltic in Gateshead for the view. The metro to Tynemouth is a good half day at the coast.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
    City {
        name: "nottingham",
        title: "Nottingham",
        guide: "the castle and the Trip to Jerusalem, which is built into the rock, then Hockley for food and shops. Wollaton Hall and its deer park are free and a good hour.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
    City {
        name: "bristol",
        title: "Bristol",
        guide: "the harbourside and the SS Great Britain, the Clifton Suspension Bridge, and Wapping Wharf or Stokes Croft for food. It is a walking city; the buses are the weak point.",
        food: None,
        areas: None,
        day_trips: Some(
            "Day trips from Bristol: Bath (15 minutes by train), Wells for the cathedral, and the Mendips or the Gower if you have a car. Cardiff is 50 minutes by train.",
        ),
        other: &[],
        days: 3,
    },
    City {
        name: "brighton",
        title: "Brighton",
        guide: "the pier and the pebble beach, the Lanes for shops and the North Laine for food. The i360 is the view and the South Downs are a short bus ride.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
    City {
        name: "york",
        title: "York",
        guide: "the Minster, the Shambles and the city walls, which take an hour and cost nothing. Book the Minster and the Jorvik ahead in summer.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
    City {
        name: "glasgow",
        title: "Glasgow",
        guide: "Kelvingrove and the Burrell Collection, the Necropolis for the view, and the West End for food. The subway is a single loop and gets you round the centre.",
        food: None,
        areas: None,
        day_trips: Some(
            "Day trips from Glasgow: Loch Lomond (40 minutes by train), Stirling for the castle, and Edinburgh in an hour. The islands need a ferry and a full day.",
        ),
        other: &[],
        days: 3,
    },
    City {
        name: "cardiff",
        title: "Cardiff",
        guide: "the castle in the middle of town, the old arcades for covered shopping streets, and Cardiff Bay for the waterfront. The national museum is free.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
    City {
        name: "belfast",
        title: "Belfast",
        guide: "the Titanic museum on the old slipway, the Cathedral Quarter for pubs, and a black taxi tour if you want the history. The Giant's Causeway is a day out north.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
    City {
        name: "cork",
        title: "Cork",
        guide: "the English Market for lunch, St Anne's tower for the bells, and a pint in the old town. Kinsale and Cobh are short bus or train trips.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
    City {
        name: "galway",
        title: "Galway",
        guide: "the Latin Quarter for pubs and buskers, the Claddagh walk along the water, and a day on Inishmore or in Connemara. Book the Aran ferry ahead in summer.",
        food: None,
        areas: None,
        day_trips: None,
        other: &[],
        days: 3,
    },
];

pub(super) const BOOK_ASK: &str =
    r####"\b(?:book|booking|booked|reserve|reserving|reservation)\b"####;
pub(super) const WORTH_ASK: &str = r####"\bworth (?:visiting|going|a visit|it)\b"####;
pub(super) const EAT_ASK: &str = r####"(?<![a-z])(?:eat|eats|eating|ate|food|foods|dinner|dine|dining|lunch|breakfast|restaurant|restaurants|dish|dishes|cuisine|menus?|street food)(?![a-z])"####;
pub(super) const SIGHTSEEING_ASK: &str = r####"\b(?:what|which|where)\s+(?:should|could|can|must|do)\s+(?:i|we|you)\s+(?:see|visit|not miss)\b|\bwhat(?:'s| is| are)\s+worth\s+seeing\b|\bwhat(?:'s| is| are)\s+there\s+to\s+(?:see|visit)\b"####;
pub(super) const QUESTION_OPENS: &str = r####"^\s*(?:and\s+|so\s+|but\s+)?(?:what|how|where|when|which|who|why|can|could|should|would|do|does|did|is|are|was|were|will|any|tell|recommend|suggest|give|show)\b"####;
pub(super) const WHICH_FIRST: &str = r####"\bwhat\b[^.?!]{0,30}\b(?:see|do|visit)\b[^.?!]{0,20}\bfirst\b|\bwhere (?:should|shall|do|would) (?:i|we) (?:start|begin)\b|\bfirst thing (?:to (?:see|do)|i should|we should)\b|\bwhat(?:'s| is) the first thing\b"####;
pub(super) const GOING_TO_A_PLACE: &str = r####"\b(?:going|go|off|heading|fly(?:ing)?|flying|travell?ing|trip|holiday|weekend|break|visiting|visit|staying|renting|booked|booking|flight)\b|\b(?:\d+|a|an|one|two|three|four|five|six|seven|eight|nine|ten|a\s+few|a\s+couple\s+of)\s+(?:days?|nights?|weeks?|weekends?|fortnight)\b"####;
pub(super) const MONTH_IN_TEXT: &str = r####"(?:^|(?<=in )|(?<=for )|(?<=until )|(?<=from )|(?<=to )|(?<=by )|(?<=early )|(?<=late )|(?<=mid ))(september|february|november|december|january|october|august|march|april|june|july|sept|may|jan|feb|mar|apr|jun|jul|aug|sep|oct|nov|dec)\b"####;
pub(super) const MONTH_RANGE: &str = r####"\b(september|february|november|december|january|october|august|march|april|june|july|sept|may|jan|feb|mar|apr|jun|jul|aug|sep|oct|nov|dec)\s*(?:to|-|–|until|till|through)\s*(september|february|november|december|january|october|august|march|april|june|july|sept|may|jan|feb|mar|apr|jun|jul|aug|sep|oct|nov|dec)\b"####;
pub(super) const SEASON_IN_TEXT: &str = r####"\b(spring|summer|autumn|fall|winter)\b"####;
pub(super) const ASKING_ONLY: &[&str] = &[
    "anything",
    "go",
    "goto",
    "miss",
    "see",
    "sight",
    "sights",
    "sightseeing",
    "something",
    "somewhere",
    "visit",
    "visited",
    "worth",
];
pub(super) const MORE_PLACES: &[&str] = &[
    "afield",
    "another",
    "around",
    "countryside",
    "else",
    "excursion",
    "excursions",
    "further",
    "nearby",
    "other",
    "others",
    "outside",
    "surrounding",
    "surroundings",
    "trip",
    "trips",
];
pub(super) const SEEING: &[&str] = &[
    "miss",
    "missed",
    "saw",
    "see",
    "seeing",
    "seen",
    "sight",
    "sights",
    "sightseeing",
    "visit",
    "visited",
    "visiting",
];
pub(super) const NUMBER_WORD: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve",
];
pub(super) const FOLLOW_MARKS: &[&str] = &[
    "there",
    "that city",
    "this city",
    "also",
    "too",
    "what about",
    "and ",
    "anything else",
    "any more",
];
pub(super) const NOT_THE_ASK_WORDS: &[&str] = &[
    "a", "an", "and", "at", "but", "can", "could", "did", "do", "does", "for", "here", "i", "in",
    "it", "its", "me", "my", "of", "on", "or", "should", "that", "the", "there", "these", "this",
    "those", "to", "trip", "us", "we", "what", "whats", "would", "you", "your",
];
pub(super) const AMBIGUOUS_MONTHS: &[&str] = &["fall", "march", "may"];
pub(super) const A_PLACE_OPENS: &[&str] = &[
    "the ", "a ", "an ", "one ", "two ", "three ", "some ", "my ", "your ",
];
pub(super) const TRIP_UNITS: &[(&str, usize)] = &[
    ("long weekend", 3),
    ("three nights", 3),
    ("four nights", 4),
    ("five nights", 5),
    ("two nights", 2),
    ("two weeks", 14),
    ("fortnight", 14),
    ("one week", 7),
    ("weekend", 2),
    ("a month", 30),
    ("a week", 7),
];
pub(super) const MONTHS: &[(&str, u16)] = &[
    ("january", 1),
    ("february", 2),
    ("march", 3),
    ("april", 4),
    ("may", 5),
    ("june", 6),
    ("july", 7),
    ("august", 8),
    ("september", 9),
    ("october", 10),
    ("november", 11),
    ("december", 12),
    ("jan", 1),
    ("feb", 2),
    ("mar", 3),
    ("apr", 4),
    ("jun", 6),
    ("jul", 7),
    ("aug", 8),
    ("sep", 9),
    ("sept", 9),
    ("oct", 10),
    ("nov", 11),
    ("dec", 12),
];
pub(super) const SEASON_MONTHS: &[(&str, &[u16])] = &[
    ("spring", &[3, 4, 5]),
    ("summer", &[6, 7, 8]),
    ("high summer", &[7, 8]),
    ("autumn", &[9, 10, 11]),
    ("fall", &[9, 10, 11]),
    ("winter", &[1, 2, 12]),
];
pub(super) const DETAIL_PARTS: &[(&[&str], &[&str])] = &[
    (
        &[
            "food",
            "eat",
            "eating",
            "dinner",
            "lunch",
            "breakfast",
            "restaurant",
            "restaurants",
            "meal",
            "meals",
            "drink",
            "drinks",
            "bar",
            "bars",
            "tapas",
            "coffee",
            "market",
            "markets",
            "nightlife",
            "club",
            "clubs",
        ],
        &[
            "food",
            "eat",
            "eating",
            "dinner",
            "lunch",
            "breakfast",
            "restaurant",
            "restaurants",
            "meal",
            "meals",
            "drink",
            "drinks",
            "bar",
            "bars",
            "tapas",
            "coffee",
            "market",
            "markets",
            "nightlife",
            "club",
            "clubs",
        ],
    ),
    (
        &[
            "museum",
            "museums",
            "gallery",
            "galleries",
            "art",
            "arts",
            "sight",
            "sights",
            "sightseeing",
            "see",
            "visit",
            "castle",
            "cathedral",
            "ruins",
            "history",
            "park",
        ],
        &[
            "museum",
            "museums",
            "gallery",
            "galleries",
            "art",
            "sights",
            "castle",
            "cathedral",
            "colosseum",
            "palace",
            "abbey",
            "church",
            "park",
            "view",
            "walk",
        ],
    ),
    (
        &[
            "shop",
            "shops",
            "shopping",
            "buy",
            "souvenir",
            "souvenirs",
            "present",
            "presents",
        ],
        &[
            "shop",
            "shops",
            "shopping",
            "market",
            "markets",
            "souvenir",
            "souvenirs",
        ],
    ),
];
pub(super) const WARM_WEEK: &str = "For a warm week: light cotton tops and shorts, one long-sleeved layer and a light waterproof, plus swimwear, a hat, high-factor sun cream and shoes you can walk in. Take two pairs of shoes at most and one smarter outfit if you plan to eat out; everything else can be washed.";
pub(super) const WARM_SUMMER: &str = "For a warm summer week: light cotton tops and shorts, one long-sleeved layer for air conditioning and a light waterproof for a thunderstorm, plus swimwear, a hat, high-factor sun cream and shoes you can walk in. Take two pairs of shoes at most and one smarter outfit if you plan to eat out; everything else can be washed.";
pub(super) const WARM_SHOULDER: &str = "For a warm week out of high summer: light cotton tops and shorts, one long-sleeved layer and a light waterproof, as May and October evenings turn cool even in Spain or Italy, plus swimwear, a hat, high-factor sun cream and shoes you can walk in. Take two pairs of shoes at most and one smarter outfit if you plan to eat out; everything else can be washed.";
