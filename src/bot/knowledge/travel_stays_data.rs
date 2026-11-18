//! Upstream day-trip, public-transport, and seasonal card data.

pub(super) const CITY_TRANSIT: &[(&str, &str)] = &[
    (
        "san sebastian",
        "In San Sebastian: Everything is walkable: the two beaches are 15 minutes apart. A bus ticket is about €2 for the ride out to Monte Igueldo, and there is a funicular for the view.",
    ),
    (
        "san francisco",
        "In San Francisco: Tap a Clipper card or a phone on Muni: $2.50 buys 90 minutes on the buses and the metro. BART is a separate fare for the airport, and the hills are real, so take the cable car once.",
    ),
    (
        "los angeles",
        "In Los Angeles: A TAP card is $1.75 a ride and a day caps at about $5, and the metro is fine for Hollywood and Santa Monica. The traffic is the reason to plan, so stay near where you are going.",
    ),
    (
        "mexico city",
        "In Mexico City: The metro is 5 pesos a ride on a card from the machine, fast and very crowded at rush hour. Uber is cheap and easier at night, and the pink line covers most of the centre.",
    ),
    (
        "copenhagen",
        "In Copenhagen: Tickets are by zone and a 24-hour city pass is about 90 kroner; a bike is what everyone actually uses and most of the centre is a five-minute ride. The harbour bus is a cheap boat trip.",
    ),
    (
        "barcelona",
        "In Barcelona: A T-casual card is ten rides for about €12.50 but it is one person only: for two or more of you the T-familiar is eight rides over 30 days and can be passed back over the gate. The metro reaches nearly everything and runs late.",
    ),
    (
        "amsterdam",
        "In Amsterdam: Tap in and out with a contactless card or an OV-chipkaart: a trip inside the city is about €3.40 and forgetting to tap out costs you. The trams go everywhere and the centre is small.",
    ),
    (
        "edinburgh",
        "In Edinburgh: Lothian buses are a flat £2 and you tap a card, with no change given. The old town and the new town are 15 minutes apart on foot, and the tram is only worth it for the airport.",
    ),
    (
        "stockholm",
        "In Stockholm: A single on the SL is about 39 kronor and a 72-hour pass about 270; the T-bana is the way round and the old town is walkable. Tickets are cashless, so buy on the app or tap a card.",
    ),
    (
        "reykjavik",
        "In Reykjavik: The centre is walkable and a bus ticket is about 630 kronur, needed only for the pools and the outer suburbs. Tours collect from stops near the hotels rather than the door, so check which stop is yours.",
    ),
    (
        "singapore",
        "In Singapore: Tap a contactless card on the MRT and the buses: a trip is about S$2 and the system is spotless. It stops around midnight, and a taxi or Grab after that.",
    ),
    (
        "hong kong",
        "In Hong Kong: Get an Octopus card: it pays for the MTR, buses, ferries and shops, and a ride is about HK$10 to 20. The MTR is the fastest way anywhere and the Star Ferry is the cheapest sightseeing.",
    ),
    (
        "melbourne",
        "In Melbourne: A myki card or a contactless card covers trams, trains and buses at about A$5 for two hours, and the tram zone in the centre is free. Trams run through the night at weekends.",
    ),
    (
        "cape town",
        "In Cape Town: There is no useful metro for visitors: use Uber or a MyCiTi bus, which is about 8 rand a trip on a card, and hire a car for the Winelands and the Cape. Agree the fare before you get in a taxi.",
    ),
    (
        "vancouver",
        "In Vancouver: A Compass card or a contactless tap covers the SkyTrain, the buses and the SeaBus, and a trip inside the city is about C$3.15. The SkyTrain from the airport costs a few dollars more, and the centre is walkable.",
    ),
    (
        "marrakech",
        "In Marrakech: Small taxis are the way round: 20 to 30 dirham inside the city, agreed before you get in. The medina is walkable and a guide the first day is money well spent.",
    ),
    (
        "florence",
        "In Florence: Walk: the historic centre is 20 minutes across and a car is a liability. A bus ticket is €1.50 for 90 minutes, from a tabacchi or the app rather than the driver.",
    ),
    (
        "valencia",
        "In Valencia: A metro ticket is €1.50 and gets you from the old town to the City of Arts; the centre itself is walkable and the old river bed is now a park you can walk through.",
    ),
    (
        "budapest",
        "In Budapest: A single is 350 forints and a 24-hour ticket about 2,500, from the machine, validated before you board. Tram 2 along the Danube is the best ride in the city and costs the same as any other.",
    ),
    (
        "istanbul",
        "In Istanbul: Get an Istanbulkart from the machine and top it up: a ride is about 20 lira on the card and nearly double as a single, and one card covers several people. The trams and the ferries go where the traffic will not.",
    ),
    (
        "helsinki",
        "In Helsinki: A ticket inside the city is about €3 and a day ticket €9, and the trams cover the centre on their own. The ferry to Suomenlinna runs on the same ticket.",
    ),
    (
        "brussels",
        "In Brussels: A single on the metro, tram or bus is €2.60 and you tap a bank card at the gate. The Grand Place is 20 minutes from most of the centre on foot.",
    ),
    (
        "new york",
        "In New York: Tap a contactless card or phone on the turnstile: a subway ride is $2.90 whatever the distance and trains run all night. Take the express where there is one, because the local stops every block.",
    ),
    (
        "seville",
        "In Seville: Walk: the centre is flat and 20 minutes across. Beyond that a bus ticket is €1.40 and the tram crosses the old town, and bikes are easy to hire on the cycle lanes.",
    ),
    (
        "granada",
        "In Granada: Walk the centre and take a bus up the hill for the Alhambra: a ticket is €1.40 and the C30 and C32 go from the Plaza Nueva. Taxis are cheap for the station.",
    ),
    (
        "chicago",
        "In Chicago: A Ventra card or a contactless tap is $2.50 a ride on the L and the buses, and most L lines run all night. The loop is walkable and the L is for everything outside it.",
    ),
    (
        "toronto",
        "In Toronto: A PRESTO tap is about $3.30 for two hours of streetcars, buses and subway. The centre is walkable and the PATH tunnels keep you out of the weather.",
    ),
    (
        "bangkok",
        "In Bangkok: Get a Rabbit card for the BTS Skytrain: a ride is 17 to 45 baht and it goes over the traffic rather than through it. The metro is a separate ticket, and the river boats are the cheapest way along the Chao Phraya.",
    ),
    (
        "london",
        "In London: Tap a contactless card or phone on the bus and the Tube and it charges the cheapest fare of the day, with a daily cap, so a paper travelcard rarely beats it. Buses are a flat £1.75 and take no cash.",
    ),
    (
        "venice",
        "In Venice: The vaporetto is the road here: a single is €9.50 and a 24-hour pass about €25, so the pass pays off after two rides. Walk between the Rialto and San Marco and take the boat for the islands.",
    ),
    (
        "naples",
        "In Naples: A ticket is €1.10 and lasts 90 minutes on the metro, the buses and the funicular, bought at a tabacchi. Metro line 1 is worth riding for the art in the stations.",
    ),
    (
        "madrid",
        "In Madrid: A ten-ride Metrobus card is the cheapest way to ride the metro and the buses, about €1.20 a trip, and the centre is walkable anyway. The metro is fast and air conditioned.",
    ),
    (
        "lisbon",
        "In Lisbon: Get a Viva Viagem card from the machine and load it: a single is €1.85 and a 24-hour pass €6.80. Tram 28 is beautiful and pickpocketed, so ride it early or take the metro instead.",
    ),
    (
        "berlin",
        "In Berlin: A single AB ticket is €3.50 and a 24-hour ticket about €10, so the day ticket wins after three rides. There are no ticket gates and the inspectors do fine, so validate before you board.",
    ),
    (
        "munich",
        "In Munich: A single is €3.90 and a day ticket for the inner zone about €8.60; the U-Bahn and S-Bahn are the way round and both run late. Validate the ticket before you get on.",
    ),
    (
        "prague",
        "In Prague: A 30-minute ticket is 30 crowns and a 90-minute one 40, from a machine or the app, and you stamp it on the first ride. The trams are the best sightseeing in the city.",
    ),
    (
        "vienna",
        "In Vienna: A single is €2.40 and a 48-hour pass about €14, which is what a weekend wants. The U-Bahn is fast, the Ringstrasse is a 40-minute walk, and you validate the ticket once.",
    ),
    (
        "krakow",
        "In Krakow: The old town and the castle are walkable; a 20-minute bus or tram ticket is 4 zloty from a machine, and the airport train is separate. The trams are old and cheap.",
    ),
    (
        "athens",
        "In Athens: A ticket is €1.20 and lasts 90 minutes on the metro, the trolleybuses and the trams; the airport line is separate. The centre under the Acropolis is walkable and the hills are not.",
    ),
    (
        "dublin",
        "In Dublin: A Leap card takes about 30% off, so a bus or tram ride is €2 on it rather than the cash fare. The centre is 20 minutes across on foot and the DART follows the coast.",
    ),
    (
        "zurich",
        "In Zurich: Buy by zone on the ZVV app or at a machine: a trip across the city is about 4.30 francs and there are no barriers. The trams go everywhere and run to the minute.",
    ),
    (
        "boston",
        "In Boston: A CharlieCard is $2.40 a ride on the T and contactless works at the gates too. The subway covers the centre and the Freedom Trail is a walk between the stops.",
    ),
    (
        "sydney",
        "In Sydney: Tap on and off with a bank card or an Opal: a city trip is about A$4, and the train from the airport is a separate A$19. The ferries are the best value sightseeing in the city.",
    ),
    (
        "paris",
        "In Paris: Get a Navigo Easy card and load ten tickets: a single is €2.15 and ten is cheaper, and it covers the Metro inside the city. Keep the ticket until you are out of the gate, because inspectors fine on the spot.",
    ),
    (
        "milan",
        "In Milan: A €2.20 ticket covers 90 minutes of metro, trams and buses, and you can tap a bank card at the metro gate. The centre is small enough to walk, the metro is for the outer bits.",
    ),
    (
        "porto",
        "In Porto: An Andante card covers the metro and the buses at about €1.60 a trip, plus 60 cents for the card itself. The historic tram is a separate €3.50 and worth doing once.",
    ),
    (
        "tokyo",
        "In Tokyo: Get a Suica or Pasmo card, or add one to your phone: a metro ride is about 180 yen and the machines take cash. Trains stop around midnight, so plan the last one.",
    ),
    (
        "kyoto",
        "In Kyoto: Buses are a flat 230 yen and the subway is only two lines, so the bus does the temples. Tap an IC card, and walk the Gion and Higashiyama side rather than ride.",
    ),
    (
        "osaka",
        "In Osaka: An ICOCA card or a phone covers the metro and the private lines, about 190 yen a ride. The loop line gets you round the centre and Dotonbori is walkable.",
    ),
    (
        "seoul",
        "In Seoul: A T-money card is about 1,400 won a ride and works on the metro, the buses and in taxis. The metro is cheap, clean, signed in English and the fastest way across the city.",
    ),
    (
        "dubai",
        "In Dubai: A Nol card covers the metro and the buses, and a ride is about 3 to 7.5 dirham. The metro is driverless and clean and goes to the malls; taxis are cheap by European standards.",
    ),
    (
        "rome",
        "In Rome: A ticket is €1.50 and lasts 100 minutes on the metro and the buses, validated in the machine on board. There are only two metro lines, so walk the centre and take a bus for the rest.",
    ),
    (
        "oslo",
        "In Oslo: A single on Ruter is about 40 kroner and a 24-hour ticket about 120; the trams and the metro are clean and the centre is walkable. Buy on the app before boarding.",
    ),
];
pub(super) const BEST_TIME: &[(&str, &str)] = &[
    (
        "australia",
        "For Australia: September to November and March to May for the south: warm, not scorching. December to February is 35C+ in Sydney and Melbourne, and the north is tropical wet season then.",
    ),
    (
        "portugal",
        "For Portugal: May, June and September: warm and dry with fewer people. July and August are hot and busy on the Algarve, and the north is wet from November to February.",
    ),
    (
        "thailand",
        "For Thailand: November to February: dry, warm and clear. March to May is hot and hazy, and June to October is the monsoon, when the islands get rough seas and some boats stop.",
    ),
    (
        "new york",
        "For New York: May to June and September to October: mild and walkable. July and August are hot and humid, January and February are cold with snow, and the Christmas weeks are the most expensive of the year.",
    ),
    (
        "scotland",
        "For Scotland: May to September for the Highlands, the islands and the long evenings. Midges are worst in July and August, and November to February has very short days.",
    ),
    (
        "iceland",
        "For Iceland: June to August gives midnight sun, open highland roads and puffins; February and March are best for the northern lights. November is dark, wet and icy, so it is the one month I would skip.",
    ),
    (
        "croatia",
        "For Croatia: June and September: the sea is warm and the coast is not yet rammed. July and August are the peak, and many islands shut down entirely from October.",
    ),
    (
        "morocco",
        "For Morocco: March to May and September to November: warm days, cool nights, no extreme heat. July and August are 40C+ inland, and the desert is cold at night all year.",
    ),
    (
        "greece",
        "For Greece: May, June, September and early October: warm sea, open tavernas and no crush. August is 35C and the islands are packed, and many small hotels close from November to March.",
    ),
    (
        "turkey",
        "For Turkey: May, June and September for the coast, and April or October for Istanbul. July and August are hot and crowded, and the coast is quiet and mild in winter.",
    ),
    (
        "norway",
        "For Norway: June to August for the fjords, the midnight sun and the coastal ferries. The northern lights need September to March and a clear, dark sky, and much of the mountain roads close in winter.",
    ),
    (
        "mexico",
        "For Mexico: November to April is the dry season, which is why it is the busy one. May to October is hot and wet, with rain usually in short afternoon storms.",
    ),
    (
        "spain",
        "For Spain: April to June and September to October, when it is 20-28C and the coast is quiet. July and August are 35C+ inland and busy on the coast; the north stays green and cooler in summer.",
    ),
    (
        "italy",
        "For Italy: April, May, late September and October: warm, not crowded, everything open. July and August are hot and packed, and August in the cities means half the restaurants shut.",
    ),
    (
        "japan",
        "For Japan: Late March to April for the blossom, or October to November for the autumn colour. June is the rainy season, August is hot and humid, and the New Year week is the busiest travel period of the year.",
    ),
    (
        "dubai",
        "For Dubai: November to March, when it is 20-30C and you can be outside. From June to September it is over 40C, so people live in air conditioning and the hotel prices fall.",
    ),
    (
        "india",
        "For India: October to March for most of the country: dry and cooler. April to June is brutally hot in the north, and the monsoon runs June to September.",
    ),
    (
        "usa",
        "For Usa: It depends where: May and September for the northeast, spring and autumn for the southwest, and June to September for the national parks and mountains. Avoid the July 4th week and Thanksgiving for flights.",
    ),
];
pub(super) const WHEN_TO_GO: &[(&str, &str)] = &[
    (
        "rio de janeiro",
        "For Rio De Janeiro: December to March is hot, 28-32C, and Carnival falls in February or early March. May to October is cooler and drier, and the sea is still warm in September.",
    ),
    (
        "san sebastian",
        "For San Sebastian: June to September: 20-24C for the beach and the pintxos, with the film festival at the end of September. July and August are the busiest, and the rest of the year is wet.",
    ),
    (
        "san francisco",
        "For San Francisco: September and October: the warmest, clearest months at 18-24C. July and August are foggy and cool, and the rain comes November to March.",
    ),
    (
        "lake district",
        "For Lake District: May, June and September for long clear days at 15-20C; September is the quietest dry-ish month and the fells are at their best. July and August bring the crowds to Windermere and the passes, and November to February is wet, dark and often icy on the tops.",
    ),
    (
        "buenos aires",
        "For Buenos Aires: March to May and September to November: 18-25C and the parks at their best. December to February is hot and humid and the city empties in January, and July and August are cool and damp.",
    ),
    (
        "mexico city",
        "For Mexico City: March to May: dry, warm and clear, before the rains. June to October is the wet season with short afternoon storms, and December to February is dry with cold nights.",
    ),
    (
        "los angeles",
        "For Los Angeles: March to May and September to November: 20-26C and clear. June to August is warm but the marine layer can sit all day at the beaches, and winter is mild with the odd storm.",
    ),
    (
        "copenhagen",
        "For Copenhagen: May to August: 18-22C, long days, and the harbour baths open from June. July is the busiest and the jazz festival, and November to February is dark by 4pm and cold.",
    ),
    (
        "edinburgh",
        "For Edinburgh: May, June and September: 15-20C, and daylight until nearly 11pm in June; August is the Festival and the Fringe, which is either why you go or when to avoid. November to February is dark by 4pm.",
    ),
    (
        "amsterdam",
        "For Amsterdam: April to June and September: 15-22C and the canals at their best, with the tulips at Keukenhof in late April. July and August are the busiest, and November to February is cold, wet and dark early.",
    ),
    (
        "ljubljana",
        "For Ljubljana: May, June and September: 20-26C, and Lake Bled is warm enough to swim from June. July and August are the busiest, and winter is cold and quiet.",
    ),
    (
        "stockholm",
        "For Stockholm: May to August: 18-23C and light until late, with midsummer at the end of June. The archipelago is best from July to mid-August, and winter is dark with short days.",
    ),
    (
        "reykjavik",
        "For Reykjavik: June to August gives midnight sun, open highland roads and puffins; February and March are best for the northern lights. November is dark, wet and icy, so it is the one month I would skip.",
    ),
    (
        "barcelona",
        "For Barcelona: May, June and September: 22-27C, the sea warm enough to swim and the terraces full. August is hot and packed, and November to February is cool with some rain.",
    ),
    (
        "santorini",
        "For Santorini: May, June, September and early October: 24-27C, warm sea and no crush. July and August are 30C+ with the cruise crowds landing mid-morning, and many hotels close from November to March.",
    ),
    (
        "dubrovnik",
        "For Dubrovnik: May, June, September and early October: 22-27C and the city walls walkable. July and August are the peak and the cruise ships fill the old town by 10am, and winter is quiet.",
    ),
    (
        "singapore",
        "For Singapore: Any month works: it is 30C and humid all year, with rain in short bursts. February to April is the driest, November to January the wettest, and the haze can arrive in September.",
    ),
    (
        "marrakech",
        "For Marrakech: March to May and October to November: 20-28C by day, cool at night, no extreme heat. July and August are 40C+ in the city, and the desert is cold at night all year.",
    ),
    (
        "cape town",
        "For Cape Town: November to March: dry, 25-28C, the beaches and the Table Mountain cableway at their best. July and August are wet and windy, which is the whale season in Hermanus.",
    ),
    (
        "melbourne",
        "For Melbourne: March to May and September to November: 18-24C, and the city is at its best in autumn. December to February is hot and changeable, and winter is cold and football.",
    ),
    (
        "yorkshire",
        "For Yorkshire: May to September: 16-20C, the Dales and the moors dry and open, and the coast best in June and September. The towns are busy at half term and in August, and the moors are bleak from November to February.",
    ),
    (
        "brussels",
        "For Brussels: May, June and September: 17-22C, and the Grand Place flower carpet in mid-August every second year. July and August are warm and quiet at weekends, and winter is grey and wet.",
    ),
    (
        "budapest",
        "For Budapest: April to June and September: 20-26C, and the thermal baths are good in cool weather too. July and August are 30C+, and December brings the Christmas markets along the Danube.",
    ),
    (
        "helsinki",
        "For Helsinki: June to August: 18-23C with almost no night, and the archipelago ferries running. January and February are the coldest, with the best chance of snow, and December is dark.",
    ),
    (
        "florence",
        "For Florence: April to June and September to October: 20-28C and the galleries without the August crush. July and August are 30C+ and busy, and winter means cool days and short queues.",
    ),
    (
        "valencia",
        "For Valencia: April to June and September to October: 20-26C, and the sea still swimmable in October. July and August are 30C+ and busy, and March is Las Fallas if you want the crowds.",
    ),
    (
        "istanbul",
        "For Istanbul: April, May, September and October: 18-25C and dry, and the Bosphorus ferries at their best. July and August are hot and humid, and December to February is cold and wet.",
    ),
    (
        "new york",
        "For New York: May to June and September to October: mild and walkable at 20-26C. July and August are hot and humid, January and February are cold with snow, and the Christmas weeks are the most expensive of the year.",
    ),
    (
        "cornwall",
        "For Cornwall: May, June and September: 16-20C, the coast path at its best and the lanes still quiet. July and August are the warmest weeks and the busiest by a long way, and October is mild and much emptier, with the Atlantic at its biggest.",
    ),
    (
        "tallinn",
        "For Tallinn: June to August: 18-22C, the old town at its best and white nights in June. December to February is cold and snowy, and the Christmas market is one of the prettiest in Europe.",
    ),
    (
        "granada",
        "For Granada: April to June and September to October: 18-28C, and the Alhambra bookable without a fight. July and August are 35C+ inland, and winter is cold up at the Alhambra.",
    ),
    (
        "seville",
        "For Seville: March to May and October: 20-28C and the orange trees in flower or in fruit, with Semana Santa and the April fair the peak. July and August are 40C+, so people live at night.",
    ),
    (
        "bangkok",
        "For Bangkok: November to February: dry, warm and clear, which is why it is the season. March to May is hot and hazy, and June to October is the monsoon with short, heavy afternoon rain.",
    ),
    (
        "chicago",
        "For Chicago: May, June and September: 20-27C and the lakefront at its best. July and August are hot and humid, and January and February are brutal with the wind off the lake.",
    ),
    (
        "toronto",
        "For Toronto: May to June and September to October: 18-25C. July and August are hot and humid, and December to March is cold with snow.",
    ),
    (
        "norfolk",
        "For Norfolk: May to September: the Broads and the north Norfolk coast at 18-22C, and the seal trips running. July and August fill the coastal villages, and October to March is quiet, cheap and cold, with huge skies and good birdwatching.",
    ),
    (
        "london",
        "For London: May, June and September: 18-22C and the long evenings. July and August are warm and crowded, and November to February is grey, wet and dark by 4.30pm.",
    ),
    (
        "dublin",
        "For Dublin: May, June and September: 16-20C and the longest days. July and August are the driest and busiest, and November to February is wet, with dark by 4.30pm.",
    ),
    (
        "munich",
        "For Munich: May to September: 20-25C, beer gardens and the Alps an hour away; late September is Oktoberfest, which doubles hotel prices. December brings the Christmas markets.",
    ),
    (
        "zurich",
        "For Zurich: May to September for the lake, the old town and the mountain railways, at 22-26C in midsummer. December to February is cold and clear, which is when the Alps look their best.",
    ),
    (
        "berlin",
        "For Berlin: May to September: 20-25C, outdoor cafes, the lakes, and the city at its most relaxed. July and August are the warmest and busiest, and November to February is grey and cold.",
    ),
    (
        "prague",
        "For Prague: April to June and September: 15-22C and the old town before the crush. July and August are busy and 25C+, and December is cold with the Christmas markets.",
    ),
    (
        "vienna",
        "For Vienna: April to June and September: 18-24C, and the ball season runs January to March. July and August are hot and the opera houses close, and December has the markets.",
    ),
    (
        "krakow",
        "For Krakow: May to September: 20-25C and the main square at its best; April and October are quiet and still fine. December to February is cold with snow and a Christmas market.",
    ),
    (
        "bergen",
        "For Bergen: May to September: 15-19C and the fjord trips running, with light until late in June and July. It rains most of the year, so pack a shell whatever the month.",
    ),
    (
        "venice",
        "For Venice: April, May, late September and October: 18-24C, warm, not crowded and everything open. July and August are hot and packed, and November to January brings acqua alta.",
    ),
    (
        "naples",
        "For Naples: April to June and September: 22-27C, with the ferries to Capri and the Amalfi coast running. July and August are hot and busy, and November to March is quiet and wet.",
    ),
    (
        "lisbon",
        "For Lisbon: May, June and September: warm and dry with fewer people. July and August are hot and busy with the tourists, and November to February is wet, though milder than the north.",
    ),
    (
        "madrid",
        "For Madrid: April, May, June and September: 20-28C and dry, with the terraces out and the Prado quiet. July and August are 35C+ and the city empties, and winter is cold and clear.",
    ),
    (
        "athens",
        "For Athens: April, May, October and early November: 20-26C and the sites walkable. July and August are 35C+ and the Acropolis is a scramble by 10am, and the islands are at their best in June and September.",
    ),
    (
        "havana",
        "For Havana: November to April: dry and 24-28C. May to October is hot and wet, and August to October is hurricane season, though the rain usually comes in short afternoon storms.",
    ),
    (
        "sydney",
        "For Sydney: September to November and March to May: 20-26C, warm without the scorch. December to February is 30C+ and the beaches are packed, and June to August is mild, with whale watching.",
    ),
    (
        "mumbai",
        "For Mumbai: November to February: dry, 25-30C and the most comfortable. March to June is hot and humid, and the monsoon runs June to September with heavy rain.",
    ),
    (
        "boston",
        "For Boston: May, June, September and October: 18-26C, and October for the foliage. July and August are hot and humid, and January and February are cold with snow.",
    ),
    (
        "paris",
        "For Paris: April to June and September to early October: 18-24C and the parks at their best. July and August are hot and many small restaurants close, and November to February is grey and cool.",
    ),
    (
        "milan",
        "For Milan: April to June and September: 20-27C and the aperitivo season. July and August are hot and humid and the city half empties, and January is cold with the sales on.",
    ),
    (
        "porto",
        "For Porto: May, June and September: 18-24C and dry, with the Douro valley at its best and the vines in leaf. July and August are warm and busy, and November to March is wet and grey.",
    ),
    (
        "crete",
        "For Crete: May, June, September and early October: 24-28C and the sea warm, and the Samaria gorge opens in May. July and August are 30C+ and busy, and winter is mild on the south coast.",
    ),
    (
        "split",
        "For Split: May, June, September and early October: 22-27C, with the ferries to Hvar and Brac running. July and August are the peak and the old town is packed by day, and many islands shut from October.",
    ),
    (
        "zadar",
        "For Zadar: May, June and September: 22-27C and the sea warm enough to swim, with the Kornati islands as a day trip. July and August are busy, and winter is quiet.",
    ),
    (
        "tokyo",
        "For Tokyo: Late March to April for the blossom, and October to November for the autumn colour and the clearest skies. June is the rainy season, August is hot and humid, and the New Year week closes many places.",
    ),
    (
        "kyoto",
        "For Kyoto: Late March to early April for the blossom, and mid-November for the maples. July and August are hot and humid, and the New Year week is the busiest of the year.",
    ),
    (
        "hanoi",
        "For Hanoi: October to December: 20-26C, dry and clear. January to March is cool and damp with drizzle, and May to September is hot and wet, with the heaviest rain in July and August.",
    ),
    (
        "dubai",
        "For Dubai: November to March, when it is 20-30C and you can be outside. From June to September it is over 40C, so people live in air conditioning and the hotel prices fall.",
    ),
    (
        "delhi",
        "For Delhi: October to March: dry and cooler, 15-25C. April to June is brutally hot at 40C+, and the monsoon runs July to September with flooding in some years.",
    ),
    (
        "devon",
        "For Devon: May, June and September: 16-20C, both coasts open and the beaches half empty. Late July and August are the school-holiday crush on the sand and the roads, and October is still mild if you do not mind a shower.",
    ),
    (
        "bath",
        "For Bath: May to September for the baths, the crescent and the canal; April and October are quieter and still fine. December is the Christmas market, which is the busiest weekend of the year.",
    ),
    (
        "oslo",
        "For Oslo: May to September: 18-23C, and the fjord islands are a ferry ride from the centre. June has the longest days, and November to February is cold and dark, with skiing an hour out of town.",
    ),
    (
        "rome",
        "For Rome: April, May, late September and October: 20-26C and the ruins walkable. July and August are 30C+ with the fountains as the only relief, and August means half the restaurants shut.",
    ),
];
pub(super) const TRANSIT_CARDS: &[(&str, &str)] = &[
    ("metrobus card", "madrid"),
    ("octopus card", "hong kong"),
    ("clipper card", "san francisco"),
    ("t-money card", "seoul"),
    ("andante card", "porto"),
    ("ov-chipkaart", "amsterdam"),
    ("compass card", "vancouver"),
    ("oyster card", "london"),
    ("navigo easy", "paris"),
    ("navigo card", "paris"),
    ("presto card", "toronto"),
    ("viva viagem", "lisbon"),
    ("charliecard", "boston"),
    ("ventra card", "chicago"),
    ("suica card", "tokyo"),
    ("pasmo card", "tokyo"),
    ("icoca card", "osaka"),
    ("opal card", "sydney"),
    ("myki card", "melbourne"),
    ("nol card", "dubai"),
];
pub(super) const CITY_TELLS: &[(&str, &str)] = &[
    ("london underground", "london"),
    ("tube station", "london"),
    ("the tube", "london"),
];
pub(super) const CARD_WORDS: &[&str] = &[
    "travel card",
    "travelcard",
    "day pass",
    "metro card",
    "transit card",
    "oyster",
    "pass for the",
    "weekly pass",
    "monthly pass",
    "tap in",
    "contactless",
];
pub(super) const TRANSIT_NOT: &[&str] = &[
    "airport",
    "flight",
    "flights",
    "fly",
    "flying",
    "booking",
    "book a",
    "hotel",
    "day trip",
    "day trips",
    "day out",
    "worth visiting",
    "best time",
    "rent a car",
    "hire a car",
    "car hire",
    "parking",
    "taxi from",
    "uber from",
];
pub(super) const TRANSIT_WORDS: &[&str] = &[
    "get around",
    "getting around",
    "get about",
    "getting about",
    "get round",
    "getting round",
    "around town",
    "metro",
    "subway",
    "underground",
    "tube",
    "u-bahn",
    "tram",
    "trams",
    "bus",
    "buses",
    "public transport",
    "transport",
    "transit",
    "travelcard",
    "travel card",
    "day pass",
    "metro card",
    "how do i travel",
    "how do people travel",
    "contactless",
    "top up",
    "chipkaart",
    "oyster",
    "ticket",
];
pub(super) const CITIES: &[&str] = &[
    "rio de janeiro",
    "san sebastian",
    "san francisco",
    "buenos aires",
    "los angeles",
    "mexico city",
    "copenhagen",
    "manchester",
    "birmingham",
    "nottingham",
    "barcelona",
    "amsterdam",
    "edinburgh",
    "stockholm",
    "reykjavik",
    "ljubljana",
    "dubrovnik",
    "marrakech",
    "santorini",
    "singapore",
    "cape town",
    "liverpool",
    "sheffield",
    "newcastle",
    "florence",
    "istanbul",
    "new york",
    "budapest",
    "brussels",
    "helsinki",
    "valencia",
    "brighton",
    "seville",
    "tallinn",
    "granada",
    "chicago",
    "toronto",
    "bangkok",
    "bristol",
    "glasgow",
    "cardiff",
    "belfast",
    "lisbon",
    "madrid",
    "berlin",
    "prague",
    "vienna",
    "london",
    "dublin",
    "athens",
    "venice",
    "boston",
    "krakow",
    "naples",
    "munich",
    "zurich",
    "sydney",
    "havana",
    "galway",
    "paris",
    "porto",
    "tokyo",
    "kyoto",
    "milan",
    "dubai",
    "hanoi",
    "leeds",
    "rome",
    "bath",
    "oslo",
    "york",
    "cork",
];
pub(super) const GENERAL_CARD: &str = "The rule is the same in most cities: tap a contactless bank card or your phone if the system takes one, because it charges the cheapest fare and usually caps the day, and if it does not, buy the stored-value card at a machine in the station - Oyster in London, Navigo in Paris, Suica in Tokyo. Compare a day or three-day pass with single tickets before you buy one: it only wins if you make several journeys a day, and the machines take a card, not always cash.";
pub(super) const WORTH_ASK: &str = r#"\bworth (?:visiting|going|a visit|it)\b"#;
pub(super) const BOOK_ASK: &str =
    r#"(?i)\b(?:book|booking|booked|reserve|reserving|reservation)\b"#;
pub(super) const BOOK_NOT_THE_TRIP: &str = r#"(?i)\b(?:restaurant|table|dinner|lunch|breakfast|meal|show|concert|gig|cinema|film|appointment|doctor|dentist|haircut|taxi|cab|car|van|hire|tour|tickets? for)\b"#;
pub(super) const BOOK_BY: &[(&str, &str)] = &[
    (
        "september",
        "For a trip in September, book by mid-July: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "february",
        "For a trip in February, book by mid-December: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "november",
        "For a trip in November, book by mid-September: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "december",
        "For a trip in December, book by mid-October: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "january",
        "For a trip in January, book by mid-November: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "october",
        "For a trip in October, book by mid-August: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "august",
        "For a trip in August, book by mid-June: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "march",
        "For a trip in March, book by mid-January: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "april",
        "For a trip in April, book by mid-February: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "june",
        "For a trip in June, book by mid-April: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "july",
        "For a trip in July, book by mid-May: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "sept",
        "For a trip in September, book by mid-July: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "may",
        "For a trip in May, book by mid-March: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "jan",
        "For a trip in January, book by mid-November: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "feb",
        "For a trip in February, book by mid-December: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "mar",
        "For a trip in March, book by mid-January: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "apr",
        "For a trip in April, book by mid-February: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "jun",
        "For a trip in June, book by mid-April: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "jul",
        "For a trip in July, book by mid-May: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "aug",
        "For a trip in August, book by mid-June: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "sep",
        "For a trip in September, book by mid-July: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "oct",
        "For a trip in October, book by mid-August: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "nov",
        "For a trip in November, book by mid-September: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
    (
        "dec",
        "For a trip in December, book by mid-October: six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.",
    ),
];
pub(super) const BOOK_GENERIC: &str = "Book six to eight weeks ahead for short-haul and two to four months for long-haul, with Tuesday and Wednesday usually the cheapest days to fly.";
pub(super) struct Trip(
    pub &'static str,
    pub &'static str,
    pub &'static str,
    pub &'static str,
);
pub(super) const TRIPS: &[Trip] = &[
    Trip(
        "niagara-on-the-lake",
        "toronto",
        "Day trips from Toronto: Niagara-on-the-Lake for the wineries on the same trip.",
        "Day trips from Toronto: Niagara Falls (two hours by train or bus, and the falls are free), Niagara-on-the-Lake for the wineries on the same trip, and Stratford for the Shakespeare festival.",
    ),
    Trip(
        "stratford-upon-avon",
        "birmingham",
        "Day trips from Birmingham: Stratford-upon-Avon (40 minutes by train). The Black Country museum is a good wet-day half-day.",
        "Stratford-Upon-Avon: 40 minutes by train from Birmingham.",
    ),
    Trip(
        "auschwitz-birkenau",
        "krakow",
        "Day trips from Krakow: Auschwitz-Birkenau (90 minutes by bus, and book a timed entry).",
        "Auschwitz-Birkenau: 90 minutes by bus from Krakow.",
    ),
    Trip(
        "the blue mountains",
        "sydney",
        "Day trips from Sydney: The Blue Mountains (two hours by train).",
        "The Blue Mountains: two hours by train from Sydney.",
    ),
    Trip(
        "bradford-on-avon",
        "bath",
        "Day trips from Bath: Bradford-on-Avon for the canal and the tithe barn.",
        "Day trips from Bath: Stonehenge (an hour by bus) with a booked slot, Lacock for the village and the abbey, and Bradford-on-Avon for the canal and the tithe barn, 15 minutes by train.",
    ),
    Trip(
        "neuschwanstein",
        "munich",
        "Day trips from Munich: Neuschwanstein (two hours by train and bus).",
        "Neuschwanstein: two hours by train and bus from Munich.",
    ),
    Trip(
        "ait benhaddou",
        "marrakech",
        "Day trips from Marrakech: Ait Benhaddou and which is a long day or a night on the way to the desert..",
        "Day trips from Marrakech: The Atlas mountains and Ourika valley (an hour by car), Essaouira for the seafront and the fish (two and a half hours by bus), and Ait Benhaddou, which is a long day or a night on the way to the desert.",
    ),
    Trip(
        "cesky krumlov",
        "prague",
        "Day trips from Prague: Cesky Krumlov and which is three hours each way so it wants a night..",
        "Day trips from Prague: Kutna Hora for the bone church and the silver mines (an hour by train), Karlstejn for the castle, and Cesky Krumlov, which is three hours each way so it wants a night.",
    ),
    Trip(
        "niagara falls",
        "toronto",
        "Day trips from Toronto: Niagara Falls (two hours by train or bus, and the falls are free).",
        "Niagara Falls: two hours by train or bus from Toronto.",
    ),
    Trip(
        "north berwick",
        "edinburgh",
        "Day trips from Edinburgh: North Berwick for the beach and the seabird centre. The Highlands need a car or a full day tour.",
        "Day trips from Edinburgh: Glasgow (50 minutes by train), Stirling for the castle, and North Berwick for the beach and the seabird centre. The Highlands need a car or a full day tour.",
    ),
    Trip(
        "sachsenhausen",
        "berlin",
        "Day trips from Berlin: Potsdam for Sanssouci (30 minutes by train) and Sachsenhausen. Both are covered by the Berlin ABC travel card.",
        "Sachsenhausen: 30 minutes by train from Berlin.",
    ),
    Trip(
        "cape sounion",
        "athens",
        "Day trips from Athens: Cape Sounion for the temple at sunset. The fast ferries are worth the extra.",
        "Day trips from Athens: Cape Sounion for the temple at sunset, Delphi for the ruins (two and a half hours by bus), and Aegina or Hydra by ferry for an island day. The fast ferries are worth the extra.",
    ),
    Trip(
        "kanchanaburi",
        "bangkok",
        "Day trips from Bangkok: Kanchanaburi for the bridge over the River Kwai.",
        "Day trips from Bangkok: Ayutthaya for the ruined temples (an hour and a half by train), Kanchanaburi for the bridge over the River Kwai, and the floating market at Damnoen Saduak, which needs a very early start.",
    ),
    Trip(
        "lake balaton",
        "budapest",
        "Day trips from Budapest: Lake Balaton and which is two hours and wants a night..",
        "Day trips from Budapest: Szentendre for the artist town and the Danube bend (40 minutes on the H5 train), Esztergom for the basilica, and Lake Balaton, which is two hours and wants a night.",
    ),
    Trip(
        "ostia antica",
        "rome",
        "Day trips from Rome: Ostia Antica for the Roman ruins (30 minutes on the metro and train). Pompeii is two hours each way, so it is a full day.",
        "Ostia Antica: 30 minutes on the metro and train from Rome.",
    ),
    Trip(
        "philadelphia",
        "new york",
        "Day trips from New York: Philadelphia (90 minutes on Amtrak) and the Hamptons by the LIRR. The Jersey shore is a longer day.",
        "Philadelphia: 90 minutes on Amtrak from New York.",
    ),
    Trip(
        "santa monica",
        "los angeles",
        "Day trips from Los Angeles: Santa Monica and Malibu down the coast and Disneyland in Anaheim (an hour by car) and Catalina island by ferry from Long Beach.",
        "Santa Monica: an hour by car from Los Angeles.",
    ),
    Trip(
        "stellenbosch",
        "cape town",
        "Day trips from Cape Town: Stellenbosch and Franschhoek for wine (an hour).",
        "Stellenbosch: an hour from Cape Town.",
    ),
    Trip(
        "the albufera",
        "valencia",
        "Day trips from Valencia: The Albufera for the rice fields and the lagoon (30 minutes by bus).",
        "The Albufera: 30 minutes by bus from Valencia.",
    ),
    Trip(
        "el escorial",
        "madrid",
        "Day trips from Madrid: El Escorial. Toledo and Segovia both deserve a full day rather than an afternoon.",
        "Day trips from Madrid: Toledo (30 minutes by train), Segovia (30 minutes) and El Escorial. Toledo and Segovia both deserve a full day rather than an afternoon.",
    ),
    Trip(
        "franschhoek",
        "cape town",
        "Day trips from Cape Town: Franschhoek for wine (an hour).",
        "Franschhoek: an hour from Cape Town.",
    ),
    Trip(
        "herculaneum",
        "naples",
        "Day trips from Naples: Herculaneum (40 minutes on the Circumvesuviana). Vesuvius is a bus and a walk from Ercolano.",
        "Herculaneum: 40 minutes on the Circumvesuviana from Naples.",
    ),
    Trip(
        "loch lomond",
        "glasgow",
        "Day trips from Glasgow: Loch Lomond (40 minutes by train). The islands need a ferry and a full day.",
        "Loch Lomond: 40 minutes by train from Glasgow.",
    ),
    Trip(
        "teotihuacan",
        "mexico city",
        "Day trips from Mexico City: Teotihuacan for the pyramids (an hour by bus from the north terminal).",
        "Teotihuacan: an hour by bus from the north terminal.",
    ),
    Trip(
        "bratislava",
        "vienna",
        "Day trips from Vienna: Bratislava (an hour by train). The Einfach-Raus ticket covers a group on the regional trains.",
        "Bratislava: an hour by train from Vienna.",
    ),
    Trip(
        "halong bay",
        "hanoi",
        "Day trips from Hanoi: Halong Bay for the boats (two and a half hours by bus, and a night on the water is better than a day).",
        "Halong Bay: two and a half hours by bus from Hanoi.",
    ),
    Trip(
        "interlaken",
        "zurich",
        "Day trips from Zurich: Interlaken for the mountains (two hours). A travel pass covers the trains and the boats.",
        "Interlaken: two hours from Zurich.",
    ),
    Trip(
        "kutna hora",
        "prague",
        "Day trips from Prague: Kutna Hora for the bone church and the silver mines (an hour by train).",
        "Kutna Hora: an hour by train from Prague.",
    ),
    Trip(
        "montenegro",
        "dubrovnik",
        "Day trips from Dubrovnik: Montenegro for Kotor and the bay. Take your passport for Mostar and Montenegro.",
        "Day trips from Dubrovnik: Mostar in Bosnia (two and a half hours by bus) for the bridge, Montenegro for Kotor and the bay, and the Elaphiti islands by boat. Take your passport for Mostar and Montenegro.",
    ),
    Trip(
        "montserrat",
        "barcelona",
        "Day trips from Barcelona: Montserrat (an hour by train and cable car).",
        "Montserrat: an hour by train and cable car from Barcelona.",
    ),
    Trip(
        "muir woods",
        "san francisco",
        "Day trips from San Francisco: Muir Woods and Sausalito (ferry, then a shuttle you book ahead).",
        "Muir Woods: ferry from San Francisco.",
    ),
    Trip(
        "petropolis",
        "rio de janeiro",
        "Day trips from Rio De Janeiro: Petropolis for the imperial palace (90 minutes by bus).",
        "Petropolis: 90 minutes by bus from Rio De Janeiro.",
    ),
    Trip(
        "providence",
        "boston",
        "Day trips from Boston: Providence (an hour on Amtrak). Cape Cod is 90 minutes by car.",
        "Providence: an hour on Amtrak from Boston.",
    ),
    Trip(
        "stonehenge",
        "bath",
        "Day trips from Bath: Stonehenge (an hour by bus) with a booked slot.",
        "Stonehenge: an hour by bus from Bath.",
    ),
    Trip(
        "szentendre",
        "budapest",
        "Day trips from Budapest: Szentendre for the artist town and the Danube bend (40 minutes on the H5 train).",
        "Szentendre: 40 minutes on the H5 train from Budapest.",
    ),
    Trip(
        "versailles",
        "paris",
        "Day trips from Paris: Versailles (40 minutes on the RER C). Go early: the first trains are the quietest.",
        "Versailles: 40 minutes on the RER C from Paris.",
    ),
    Trip(
        "abu dhabi",
        "dubai",
        "Day trips from Dubai: Abu Dhabi for the grand mosque and the Louvre (90 minutes by car or bus).",
        "Abu Dhabi: 90 minutes by car or bus from Dubai.",
    ),
    Trip(
        "ayutthaya",
        "bangkok",
        "Day trips from Bangkok: Ayutthaya for the ruined temples (an hour and a half by train).",
        "Ayutthaya: an hour and a half by train from Bangkok.",
    ),
    Trip(
        "cambridge",
        "london",
        "Day trips from London: Cambridge (about an hour). Book the train a week ahead and you halve the fare.",
        "Cambridge: about an hour from London.",
    ),
    Trip(
        "durnstein",
        "vienna",
        "Day trips from Vienna: the Wachau valley by boat for the vineyards and Durnstein. The Einfach-Raus ticket covers a group on the regional trains.",
        "Day trips from Vienna: Bratislava (an hour by train), the Wachau valley by boat for the vineyards and Durnstein, and Baden for the spa. The Einfach-Raus ticket covers a group on the regional trains.",
    ),
    Trip(
        "essaouira",
        "marrakech",
        "Day trips from Marrakech: Essaouira for the seafront and the fish (two and a half hours by bus).",
        "Essaouira: two and a half hours by bus from Marrakech.",
    ),
    Trip(
        "esztergom",
        "budapest",
        "Day trips from Budapest: Esztergom for the basilica.",
        "Day trips from Budapest: Szentendre for the artist town and the Danube bend (40 minutes on the H5 train), Esztergom for the basilica, and Lake Balaton, which is two hours and wants a night.",
    ),
    Trip(
        "guimaraes",
        "porto",
        "Day trips from Porto: Guimaraes for the churches and the old town. The Douro line is the scenic one, so sit on the right.",
        "Day trips from Porto: The Douro valley by train to Pinhao for the vineyards, Braga and Guimaraes for the churches and the old town, and Aveiro for the canals. The Douro line is the scenic one, so sit on the right.",
    ),
    Trip(
        "harrogate",
        "leeds",
        "Day trips from Leeds: Harrogate for a spa afternoon. Manchester is 50 minutes by train if you want a city.",
        "Day trips from Leeds: York (25 minutes by train), the Yorkshire Dales (an hour by car) and Harrogate for a spa afternoon. Manchester is 50 minutes by train if you want a city.",
    ),
    Trip(
        "karlstejn",
        "prague",
        "Day trips from Prague: Karlstejn for the castle.",
        "Day trips from Prague: Kutna Hora for the bone church and the silver mines (an hour by train), Karlstejn for the castle, and Cesky Krumlov, which is three hours each way so it wants a night.",
    ),
    Trip(
        "lake bled",
        "ljubljana",
        "Day trips from Ljubljana: Lake Bled (an hour by bus) for the island church.",
        "Lake Bled: an hour by bus from Ljubljana.",
    ),
    Trip(
        "lake como",
        "milan",
        "Day trips from Milan: Lake Como for Bellagio (an hour by train to Como, then a boat). The Last Supper needs booking weeks ahead.",
        "Lake Como: an hour by train to Como from Milan.",
    ),
    Trip(
        "lexington",
        "boston",
        "Day trips from Boston: Lexington for the revolution. Cape Cod is 90 minutes by car.",
        "Day trips from Boston: Salem for the witch museum (30 minutes by train), Concord and Lexington for the revolution, and Providence (an hour on Amtrak). Cape Cod is 90 minutes by car.",
    ),
    Trip(
        "liverpool",
        "manchester",
        "Day trips from Manchester: Liverpool (50 minutes). York is 90 minutes.",
        "Liverpool: 50 minutes from Manchester.",
    ),
    Trip(
        "milwaukee",
        "chicago",
        "Day trips from Chicago: Milwaukee (90 minutes on Amtrak).",
        "Milwaukee: 90 minutes on Amtrak from Chicago.",
    ),
    Trip(
        "ninh binh",
        "hanoi",
        "Day trips from Hanoi: Ninh Binh for the river rowing boats.",
        "Day trips from Hanoi: Halong Bay for the boats (two and a half hours by bus, and a night on the water is better than a day), Ninh Binh for the river rowing boats, and Sapa by night train for the rice terraces.",
    ),
    Trip(
        "sausalito",
        "san francisco",
        "Day trips from San Francisco: Sausalito (ferry, then a shuttle you book ahead).",
        "Sausalito: ferry from San Francisco.",
    ),
    Trip(
        "stratford",
        "toronto",
        "Day trips from Toronto: Stratford for the Shakespeare festival..",
        "Day trips from Toronto: Niagara Falls (two hours by train or bus, and the falls are free), Niagara-on-the-Lake for the wineries on the same trip, and Stratford for the Shakespeare festival.",
    ),
    Trip(
        "akrotiri",
        "santorini",
        "Day trips from Santorini: Akrotiri for the Minoan town under the ash are the day.",
        "Day trips from Santorini: Oia for the sunset and Akrotiri for the Minoan town under the ash are the day, and both are 20 to 30 minutes by bus from Fira; boats to Thirasia and the volcano leave from the old port.",
    ),
    Trip(
        "brighton",
        "london",
        "Day trips from London: Brighton (an hour) for the sea. Book the train a week ahead and you halve the fare.",
        "Brighton: an hour from London.",
    ),
    Trip(
        "chartres",
        "paris",
        "Day trips from Paris: Chartres for the cathedral. Go early: the first trains are the quietest.",
        "Day trips from Paris: Versailles (40 minutes on the RER C), Chartres for the cathedral, and Giverny for Monet's garden from April to October. Go early: the first trains are the quietest.",
    ),
    Trip(
        "hermanus",
        "cape town",
        "Day trips from Cape Town: Hermanus for whales from June to November..",
        "Day trips from Cape Town: The Cape of Good Hope and the Boulders Beach penguins, which is a full day by car, Stellenbosch and Franschhoek for wine (an hour), and Hermanus for whales from June to November.",
    ),
    Trip(
        "kamakura",
        "tokyo",
        "Day trips from Tokyo: Kamakura for the Buddha (both about two hours). Go on a weekday: the weekend trains are packed.",
        "Kamakura: both about two hours from Tokyo.",
    ),
    Trip(
        "matanzas",
        "havana",
        "Day trips from Havana: Matanzas for the river and the poets..",
        "Day trips from Havana: Vinales for the tobacco valley (three hours by bus, better with a night), the beach at Santa Maria del Mar (30 minutes by taxi), and Matanzas for the river and the poets.",
    ),
    Trip(
        "postojna",
        "ljubljana",
        "Day trips from Ljubljana: Postojna for the caves and the castle.",
        "Day trips from Ljubljana: Lake Bled (an hour by bus) for the island church, Postojna for the caves and the castle, and Skocjan for the bigger cave, which is the one to pick if you only do one.",
    ),
    Trip(
        "roskilde",
        "copenhagen",
        "Day trips from Copenhagen: Roskilde for the cathedral and the Viking ships (25 minutes).",
        "Roskilde: 25 minutes from Copenhagen.",
    ),
    Trip(
        "salzburg",
        "munich",
        "Day trips from Munich: Salzburg (90 minutes) and the Bavarian lakes. Buy a Bayern ticket: it covers the regional trains for a group.",
        "Salzburg: 90 minutes from Munich.",
    ),
    Trip(
        "stirling",
        "edinburgh",
        "Day trips from Edinburgh: Stirling for the castle. The Highlands need a car or a full day tour.",
        "Day trips from Edinburgh: Glasgow (50 minutes by train), Stirling for the castle, and North Berwick for the beach and the seabird centre. The Highlands need a car or a full day tour.",
    ),
    Trip(
        "zakopane",
        "krakow",
        "Day trips from Krakow: Zakopane for the mountains (two hours). Auschwitz is a heavy half-day, not a sight.",
        "Zakopane: two hours from Krakow.",
    ),
    Trip(
        "bergamo",
        "milan",
        "Day trips from Milan: Bergamo for the old town (50 minutes). The Last Supper needs booking weeks ahead.",
        "Bergamo: 50 minutes from Milan.",
    ),
    Trip(
        "cascais",
        "lisbon",
        "Day trips from Lisbon: Cascais for the beach (40 minutes on the same line). Buy the train ticket at the machine and go before 9am.",
        "Cascais: 40 minutes on the same line from Lisbon.",
    ),
    Trip(
        "chester",
        "manchester",
        "Day trips from Manchester: Chester for the walls and the rows. York is 90 minutes.",
        "Day trips from Manchester: the Peak District (40 minutes by train to Edale), Liverpool (50 minutes) and Chester for the walls and the rows. York is 90 minutes.",
    ),
    Trip(
        "cholula",
        "mexico city",
        "Day trips from Mexico City: Cholula (two hours by bus).",
        "Cholula: two hours by bus from Mexico City.",
    ),
    Trip(
        "concord",
        "boston",
        "Day trips from Boston: Concord and Lexington for the revolution. Cape Cod is 90 minutes by car.",
        "Day trips from Boston: Salem for the witch museum (30 minutes by train), Concord and Lexington for the revolution, and Providence (an hour on Amtrak). Cape Cod is 90 minutes by car.",
    ),
    Trip(
        "cordoba",
        "seville",
        "Day trips from Seville: Cordoba for the mosque-cathedral (45 minutes by train). Granada is three hours by bus, so it wants a night.",
        "Cordoba: 45 minutes by train from Seville.",
    ),
    Trip(
        "getaria",
        "san sebastian",
        "Day trips from San Sebastian: Getaria for the txakoli wine. Hondarribia is 25 minutes by bus.",
        "Day trips from San Sebastian: Bilbao for the Guggenheim (an hour and a quarter by bus), Saint-Jean-de-Luz across the French border (40 minutes) and Getaria for the txakoli wine. Hondarribia is 25 minutes by bus.",
    ),
    Trip(
        "giverny",
        "paris",
        "Day trips from Paris: Giverny for Monet's garden from April to October. Go early: the first trains are the quietest.",
        "Day trips from Paris: Versailles (40 minutes on the RER C), Chartres for the cathedral, and Giverny for Monet's garden from April to October. Go early: the first trains are the quietest.",
    ),
    Trip(
        "glasgow",
        "edinburgh",
        "Day trips from Edinburgh: Glasgow (50 minutes by train). The Highlands need a car or a full day tour.",
        "Glasgow: 50 minutes by train from Edinburgh.",
    ),
    Trip(
        "haarlem",
        "amsterdam",
        "Day trips from Amsterdam: Haarlem (15 minutes). Keukenhof is worth it only in tulip season, mid-March to mid-May.",
        "Haarlem: 15 minutes from Amsterdam.",
    ),
    Trip(
        "lucerne",
        "zurich",
        "Day trips from Zurich: Lucerne (45 minutes by train). A travel pass covers the trains and the boats.",
        "Lucerne: 45 minutes by train from Zurich.",
    ),
    Trip(
        "malacca",
        "singapore",
        "Day trips from Singapore: Malacca and which is three and a half hours each way. The Botanic Gardens and the southern ridges are free and in the city.",
        "Day trips from Singapore: Johor Bahru across the causeway (the bus queue is long at weekends), Batam by ferry, and Malacca, which is three and a half hours each way. The Botanic Gardens and the southern ridges are free and in the city.",
    ),
    Trip(
        "niteroi",
        "rio de janeiro",
        "Day trips from Rio De Janeiro: Niteroi for the museum and the beaches across the bay.",
        "Day trips from Rio De Janeiro: Petropolis for the imperial palace (90 minutes by bus), Niteroi for the museum and the beaches across the bay, and Buzios, which is three hours and wants a night.",
    ),
    Trip(
        "orvieto",
        "rome",
        "Day trips from Rome: Orvieto for the cathedral. Pompeii is two hours each way, so it is a full day.",
        "Day trips from Rome: Ostia Antica for the Roman ruins (30 minutes on the metro and train), Tivoli for the villa gardens, and Orvieto for the cathedral. Pompeii is two hours each way, so it is a full day.",
    ),
    Trip(
        "pompeii",
        "naples",
        "Day trips from Naples: Pompeii and Herculaneum (40 minutes on the Circumvesuviana). Vesuvius is a bus and a walk from Ercolano.",
        "Pompeii: 40 minutes on the Circumvesuviana from Naples.",
    ),
    Trip(
        "potsdam",
        "berlin",
        "Day trips from Berlin: Potsdam for Sanssouci (30 minutes by train) and Sachsenhausen. Both are covered by the Berlin ABC travel card.",
        "Potsdam: 30 minutes by train from Berlin.",
    ),
    Trip(
        "requena",
        "valencia",
        "Day trips from Valencia: Requena for the wine. The City of Arts is in the city, not a day out.",
        "Day trips from Valencia: The Albufera for the rice fields and the lagoon (30 minutes by bus), Sagunto for the Roman theatre, and Requena for the wine. The City of Arts is in the city, not a day out.",
    ),
    Trip(
        "sagunto",
        "valencia",
        "Day trips from Valencia: Sagunto for the Roman theatre. The City of Arts is in the city, not a day out.",
        "Day trips from Valencia: The Albufera for the rice fields and the lagoon (30 minutes by bus), Sagunto for the Roman theatre, and Requena for the wine. The City of Arts is in the city, not a day out.",
    ),
    Trip(
        "segovia",
        "madrid",
        "Day trips from Madrid: Segovia (30 minutes). Toledo and Segovia both deserve a full day rather than an afternoon.",
        "Segovia: 30 minutes from Madrid.",
    ),
    Trip(
        "skocjan",
        "ljubljana",
        "Day trips from Ljubljana: Skocjan for the bigger cave.",
        "Day trips from Ljubljana: Lake Bled (an hour by bus) for the island church, Postojna for the caves and the castle, and Skocjan for the bigger cave, which is the one to pick if you only do one.",
    ),
    Trip(
        "uppsala",
        "stockholm",
        "Day trips from Stockholm: Uppsala for the cathedral and the Vikings (40 minutes by train).",
        "Uppsala: 40 minutes by train from Stockholm.",
    ),
    Trip(
        "utrecht",
        "amsterdam",
        "Day trips from Amsterdam: Utrecht (25 minutes) and the Zaanse Schans windmills (40 minutes). Keukenhof is worth it only in tulip season, mid-March to mid-May.",
        "Utrecht: 25 minutes from Amsterdam.",
    ),
    Trip(
        "vinales",
        "havana",
        "Day trips from Havana: Vinales for the tobacco valley (three hours by bus, better with a night).",
        "Vinales: three hours by bus from Havana.",
    ),
    Trip(
        "aveiro",
        "porto",
        "Day trips from Porto: Aveiro for the canals. The Douro line is the scenic one, so sit on the right.",
        "Day trips from Porto: The Douro valley by train to Pinhao for the vineyards, Braga and Guimaraes for the churches and the old town, and Aveiro for the canals. The Douro line is the scenic one, so sit on the right.",
    ),
    Trip(
        "beacon",
        "new york",
        "Day trips from New York: Beacon for Dia:Beacon (80 minutes on Metro-North). The Jersey shore is a longer day.",
        "Beacon: 80 minutes on Metro-North from New York.",
    ),
    Trip(
        "bilbao",
        "san sebastian",
        "Day trips from San Sebastian: Bilbao for the Guggenheim (an hour and a quarter by bus).",
        "Bilbao: an hour and a quarter by bus from San Sebastian.",
    ),
    Trip(
        "bruges",
        "brussels",
        "Day trips from Brussels: Bruges (an hour by train) for the canals. Book Bruges ahead in summer, it fills up.",
        "Bruges: an hour by train from Brussels.",
    ),
    Trip(
        "burano",
        "venice",
        "Day trips from Venice: Burano for the coloured houses.",
        "Day trips from Venice: Verona (an hour by train) for the arena, Padua for the Scrovegni chapel, and the islands in the lagoon: Murano for glass and Burano for the coloured houses, on a vaporetto day pass.",
    ),
    Trip(
        "buzios",
        "rio de janeiro",
        "Day trips from Rio De Janeiro: Buzios and which is three hours and wants a night..",
        "Day trips from Rio De Janeiro: Petropolis for the imperial palace (90 minutes by bus), Niteroi for the museum and the beaches across the bay, and Buzios, which is three hours and wants a night.",
    ),
    Trip(
        "delphi",
        "athens",
        "Day trips from Athens: Delphi for the ruins (two and a half hours by bus).",
        "Delphi: two and a half hours by bus from Athens.",
    ),
    Trip(
        "drobak",
        "oslo",
        "Day trips from Oslo: Drobak for the fortress and the old town (an hour by bus).",
        "Drobak: an hour by bus from Oslo.",
    ),
    Trip(
        "geysir",
        "reykjavik",
        "Day trips from Reykjavik: Geysir and Gullfoss - is the full day out and needs a car or a tour and the Blue Lagoon is 45 minutes by bus and booked ahead and Snaefellsnes is a very long day..",
        "Day trips from Reykjavik: The Golden Circle - Thingvellir, Geysir and Gullfoss - is the full day out and needs a car or a tour, the Blue Lagoon is 45 minutes by bus and booked ahead, and Snaefellsnes is a very long day.",
    ),
    Trip(
        "girona",
        "barcelona",
        "Day trips from Barcelona: Girona (40 minutes). Girona plus Figures makes a good full day.",
        "Girona: 40 minutes from Barcelona.",
    ),
    Trip(
        "hikone",
        "kyoto",
        "Day trips from Kyoto: Hikone for the castle on Lake Biwa. Fushimi Inari and Arashiyama are in Kyoto, so go at dawn instead.",
        "Day trips from Kyoto: Nara for the deer and the great Buddha (45 minutes by train), Osaka (30 minutes) and Hikone for the castle on Lake Biwa. Fushimi Inari and Arashiyama are in Kyoto, so go at dawn instead.",
    ),
    Trip(
        "ischia",
        "naples",
        "Day trips from Naples: Ischia for a slower island day. Vesuvius is a bus and a walk from Ercolano.",
        "Day trips from Naples: Pompeii and Herculaneum (40 minutes on the Circumvesuviana), the Amalfi coast by ferry or bus, and Ischia for a slower island day. Vesuvius is a bus and a walk from Ercolano.",
    ),
    Trip(
        "lacock",
        "bath",
        "Day trips from Bath: Lacock for the village and the abbey.",
        "Day trips from Bath: Stonehenge (an hour by bus) with a booked slot, Lacock for the village and the abbey, and Bradford-on-Avon for the canal and the tithe barn, 15 minutes by train.",
    ),
    Trip(
        "leuven",
        "brussels",
        "Day trips from Brussels: Leuven for the university town. Book Bruges ahead in summer, it fills up.",
        "Day trips from Brussels: Bruges (an hour by train) for the canals, Ghent (35 minutes) for the old port, and Leuven for the university town. Book Bruges ahead in summer, it fills up.",
    ),
    Trip(
        "obidos",
        "lisbon",
        "Day trips from Lisbon: Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there).",
        "Obidos: an hour by bus from Lisbon's Campo Grande.",
    ),
    Trip(
        "oxford",
        "london",
        "Day trips from London: Oxford and Cambridge (about an hour). Book the train a week ahead and you halve the fare.",
        "Oxford: about an hour from London.",
    ),
    Trip(
        "porvoo",
        "helsinki",
        "Day trips from Helsinki: Porvoo for the wooden old town (an hour by bus) and Tallinn by ferry (two hours, and take your passport)..",
        "Porvoo: an hour by bus from Helsinki.",
    ),
    Trip(
        "puebla",
        "mexico city",
        "Day trips from Mexico City: Puebla and Cholula (two hours by bus).",
        "Puebla: two hours by bus from Mexico City.",
    ),
    Trip(
        "sintra",
        "lisbon",
        "Day trips from Lisbon: Sintra for the palaces (40 minutes by train). Buy the train ticket at the machine and go before 9am.",
        "Sintra: 40 minutes by train from Lisbon.",
    ),
    Trip(
        "sitges",
        "barcelona",
        "Day trips from Barcelona: Sitges for the beach (40 minutes). Girona plus Figures makes a good full day.",
        "Sitges: 40 minutes from Barcelona.",
    ),
    Trip(
        "tivoli",
        "rome",
        "Day trips from Rome: Tivoli for the villa gardens. Pompeii is two hours each way, so it is a full day.",
        "Day trips from Rome: Ostia Antica for the Roman ruins (30 minutes on the metro and train), Tivoli for the villa gardens, and Orvieto for the cathedral. Pompeii is two hours each way, so it is a full day.",
    ),
    Trip(
        "toledo",
        "madrid",
        "Day trips from Madrid: Toledo (30 minutes by train). Toledo and Segovia both deserve a full day rather than an afternoon.",
        "Toledo: 30 minutes by train from Madrid.",
    ),
    Trip(
        "trogir",
        "split",
        "Day trips from Split: Trogir (30 minutes by bus) for the walled centre.",
        "Trogir: 30 minutes by bus from Split.",
    ),
    Trip(
        "verona",
        "venice",
        "Day trips from Venice: Verona (an hour by train) for the arena.",
        "Verona: an hour by train from Venice.",
    ),
    Trip(
        "baden",
        "vienna",
        "Day trips from Vienna: Baden for the spa. The Einfach-Raus ticket covers a group on the regional trains.",
        "Day trips from Vienna: Bratislava (an hour by train), the Wachau valley by boat for the vineyards and Durnstein, and Baden for the spa. The Einfach-Raus ticket covers a group on the regional trains.",
    ),
    Trip(
        "braga",
        "porto",
        "Day trips from Porto: Braga and Guimaraes for the churches and the old town. The Douro line is the scenic one, so sit on the right.",
        "Day trips from Porto: The Douro valley by train to Pinhao for the vineyards, Braga and Guimaraes for the churches and the old town, and Aveiro for the canals. The Douro line is the scenic one, so sit on the right.",
    ),
    Trip(
        "bursa",
        "istanbul",
        "Day trips from Istanbul: Bursa for the Ottoman tombs.",
        "Day trips from Istanbul: The Princes' Islands by ferry (an hour and a half) where there are no cars, Bursa for the Ottoman tombs, and the Bosphorus village of Anadolu Kavagi at the mouth of the Black Sea.",
    ),
    Trip(
        "ghent",
        "brussels",
        "Day trips from Brussels: Ghent (35 minutes) for the old port. Book Bruges ahead in summer, it fills up.",
        "Ghent: 35 minutes from Brussels.",
    ),
    Trip(
        "hatta",
        "dubai",
        "Day trips from Dubai: Hatta for the mountains and the kayaks. The Abu Dhabi bus leaves from Al Ghubaiba.",
        "Day trips from Dubai: Abu Dhabi for the grand mosque and the Louvre (90 minutes by car or bus), the desert for a dune drive, and Hatta for the mountains and the kayaks. The Abu Dhabi bus leaves from Al Ghubaiba.",
    ),
    Trip(
        "howth",
        "dublin",
        "Day trips from Dublin: Howth for the cliff walk (30 minutes on the DART).",
        "Howth: 30 minutes on the DART from Dublin.",
    ),
    Trip(
        "jerez",
        "seville",
        "Day trips from Seville: Jerez for the sherry bodegas. Granada is three hours by bus, so it wants a night.",
        "Day trips from Seville: Cordoba for the mosque-cathedral (45 minutes by train), Ronda for the gorge and the bridge, and Jerez for the sherry bodegas. Granada is three hours by bus, so it wants a night.",
    ),
    Trip(
        "lucca",
        "florence",
        "Day trips from Florence: Lucca for the walls and the bikes. Fiesole is 30 minutes by bus for the view back over Florence.",
        "Day trips from Florence: Pisa (an hour by train) for the tower, Siena (90 minutes) for the square, and Lucca for the walls and the bikes. Fiesole is 30 minutes by bus for the view back over Florence.",
    ),
    Trip(
        "nikko",
        "tokyo",
        "Day trips from Tokyo: Nikko for the shrines. Go on a weekday: the weekend trains are packed.",
        "Day trips from Tokyo: Nikko for the shrines and Kamakura for the Buddha (both about two hours), or Hakone for a view of Fuji if the sky is clear. Go on a weekday: the weekend trains are packed.",
    ),
    Trip(
        "osaka",
        "kyoto",
        "Day trips from Kyoto: Osaka (30 minutes). Fushimi Inari and Arashiyama are in Kyoto, so go at dawn instead.",
        "Osaka: 30 minutes from Kyoto.",
    ),
    Trip(
        "padua",
        "venice",
        "Day trips from Venice: Padua for the Scrovegni chapel.",
        "Day trips from Venice: Verona (an hour by train) for the arena, Padua for the Scrovegni chapel, and the islands in the lagoon: Murano for glass and Burano for the coloured houses, on a vaporetto day pass.",
    ),
    Trip(
        "ronda",
        "seville",
        "Day trips from Seville: Ronda for the gorge and the bridge. Granada is three hours by bus, so it wants a night.",
        "Day trips from Seville: Cordoba for the mosque-cathedral (45 minutes by train), Ronda for the gorge and the bridge, and Jerez for the sherry bodegas. Granada is three hours by bus, so it wants a night.",
    ),
    Trip(
        "salem",
        "boston",
        "Day trips from Boston: Salem for the witch museum (30 minutes by train). Cape Cod is 90 minutes by car.",
        "Salem: 30 minutes by train from Boston.",
    ),
    Trip(
        "siena",
        "florence",
        "Day trips from Florence: Siena (90 minutes) for the square. Fiesole is 30 minutes by bus for the view back over Florence.",
        "Siena: 90 minutes from Florence.",
    ),
    Trip(
        "tartu",
        "tallinn",
        "Day trips from Tallinn: Tartu (two and a half hours) and Helsinki by ferry in two hours..",
        "Tartu: two and a half hours from Tallinn.",
    ),
    Trip(
        "taxco",
        "mexico city",
        "Day trips from Mexico City: Taxco for silver.",
        "Day trips from Mexico City: Teotihuacan for the pyramids (an hour by bus from the north terminal), Puebla and Cholula (two hours by bus), and Taxco for silver, which is a long day.",
    ),
    Trip(
        "tigre",
        "buenos aires",
        "Day trips from Buenos Aires: Tigre for the delta boats (an hour by train).",
        "Tigre: an hour by train from Buenos Aires.",
    ),
    Trip(
        "turin",
        "milan",
        "Day trips from Milan: Turin (an hour on the fast train). The Last Supper needs booking weeks ahead.",
        "Turin: an hour on the fast train from Milan.",
    ),
    Trip(
        "wells",
        "bristol",
        "Day trips from Bristol: Wells for the cathedral. Cardiff is 50 minutes by train.",
        "Day trips from Bristol: Bath (15 minutes by train), Wells for the cathedral, and the Mendips or the Gower if you have a car. Cardiff is 50 minutes by train.",
    ),
    Trip(
        "krka",
        "split",
        "Day trips from Split: Krka for the waterfalls.",
        "Day trips from Split: Hvar by catamaran (an hour) for the old town and the lavender, Trogir (30 minutes by bus) for the walled centre, and Krka for the waterfalls, which needs a car or a tour.",
    ),
    Trip(
        "nara",
        "kyoto",
        "Day trips from Kyoto: Nara for the deer and the great Buddha (45 minutes by train). Fushimi Inari and Arashiyama are in Kyoto, so go at dawn instead.",
        "Nara: 45 minutes by train from Kyoto.",
    ),
    Trip(
        "pisa",
        "florence",
        "Day trips from Florence: Pisa (an hour by train) for the tower. Fiesole is 30 minutes by bus for the view back over Florence.",
        "Pisa: an hour by train from Florence.",
    ),
    Trip(
        "york",
        "leeds",
        "Day trips from Leeds: York (25 minutes by train). Manchester is 50 minutes by train if you want a city.",
        "York: 25 minutes by train from Leeds.",
    ),
    Trip(
        "oia",
        "santorini",
        "Day trips from Santorini: Oia for the sunset.",
        "Day trips from Santorini: Oia for the sunset and Akrotiri for the Minoan town under the ash are the day, and both are 20 to 30 minutes by bus from Fira; boats to Thirasia and the volcano leave from the old port.",
    ),
];
