//! Pinned travel tables. Candidate names are in stable longest-first order.

pub(super) const DAY_TRIPS: &[(&str, &str)] = &[
    (
        "rio de janeiro",
        "Day trips from Rio De Janeiro: Petropolis for the imperial palace (90 minutes by bus), Niteroi for the museum and the beaches across the bay, and Buzios, which is three hours and wants a night.",
    ),
    (
        "san francisco",
        "Day trips from San Francisco: Muir Woods and Sausalito (ferry, then a shuttle you book ahead), Napa by car, and Santa Cruz over the hill. Yosemite is four hours, so it wants a night.",
    ),
    (
        "san sebastian",
        "Day trips from San Sebastian: Bilbao for the Guggenheim (an hour and a quarter by bus), Saint-Jean-de-Luz across the French border (40 minutes) and Getaria for the txakoli wine. Hondarribia is 25 minutes by bus.",
    ),
    (
        "buenos aires",
        "Day trips from Buenos Aires: Tigre for the delta boats (an hour by train), Colonia in Uruguay by ferry (an hour, and take your passport), and San Antonio de Areco for the gaucho town.",
    ),
    (
        "los angeles",
        "Day trips from Los Angeles: Santa Monica and Malibu down the coast, Disneyland in Anaheim (an hour by car) and Catalina island by ferry from Long Beach. Almost all of it needs a car.",
    ),
    (
        "mexico city",
        "Day trips from Mexico City: Teotihuacan for the pyramids (an hour by bus from the north terminal), Puebla and Cholula (two hours by bus), and Taxco for silver, which is a long day.",
    ),
    (
        "copenhagen",
        "Day trips from Copenhagen: Roskilde for the cathedral and the Viking ships (25 minutes), Malmo across the bridge (35 minutes) for a cheaper lunch, and the Louisiana museum up the coast, which is the best half-day of the lot.",
    ),
    (
        "manchester",
        "Day trips from Manchester: the Peak District (40 minutes by train to Edale), Liverpool (50 minutes) and Chester for the walls and the rows. York is 90 minutes.",
    ),
    (
        "birmingham",
        "Day trips from Birmingham: Stratford-upon-Avon (40 minutes by train), Warwick castle and the Cotswolds by car. The Black Country museum is a good wet-day half-day.",
    ),
    (
        "edinburgh",
        "Day trips from Edinburgh: Glasgow (50 minutes by train), Stirling for the castle, and North Berwick for the beach and the seabird centre. The Highlands need a car or a full day tour.",
    ),
    (
        "barcelona",
        "Day trips from Barcelona: Montserrat (an hour by train and cable car), Girona (40 minutes) and Sitges for the beach (40 minutes). Girona plus Figures makes a good full day.",
    ),
    (
        "amsterdam",
        "Day trips from Amsterdam: Haarlem (15 minutes), Utrecht (25 minutes) and the Zaanse Schans windmills (40 minutes). Keukenhof is worth it only in tulip season, mid-March to mid-May.",
    ),
    (
        "stockholm",
        "Day trips from Stockholm: The archipelago by ferry from Stromkajen, Uppsala for the cathedral and the Vikings (40 minutes by train), and Drottningholm by boat for the palace theatre.",
    ),
    (
        "dubrovnik",
        "Day trips from Dubrovnik: Mostar in Bosnia (two and a half hours by bus) for the bridge, Montenegro for Kotor and the bay, and the Elaphiti islands by boat. Take your passport for Mostar and Montenegro.",
    ),
    (
        "ljubljana",
        "Day trips from Ljubljana: Lake Bled (an hour by bus) for the island church, Postojna for the caves and the castle, and Skocjan for the bigger cave, which is the one to pick if you only do one.",
    ),
    (
        "marrakech",
        "Day trips from Marrakech: The Atlas mountains and Ourika valley (an hour by car), Essaouira for the seafront and the fish (two and a half hours by bus), and Ait Benhaddou, which is a long day or a night on the way to the desert.",
    ),
    (
        "liverpool",
        "Day trips from Liverpool: Chester (40 minutes by train), the Wirral coast, and Manchester in under an hour. North Wales is a longer day by train.",
    ),
    (
        "singapore",
        "Day trips from Singapore: Johor Bahru across the causeway (the bus queue is long at weekends), Batam by ferry, and Malacca, which is three and a half hours each way. The Botanic Gardens and the southern ridges are free and in the city.",
    ),
    (
        "cape town",
        "Day trips from Cape Town: The Cape of Good Hope and the Boulders Beach penguins, which is a full day by car, Stellenbosch and Franschhoek for wine (an hour), and Hermanus for whales from June to November.",
    ),
    (
        "reykjavik",
        "Day trips from Reykjavik: The Golden Circle - Thingvellir, Geysir and Gullfoss - is the full day out and needs a car or a tour, the Blue Lagoon is 45 minutes by bus and booked ahead, and Snaefellsnes is a very long day.",
    ),
    (
        "santorini",
        "Day trips from Santorini: Oia for the sunset and Akrotiri for the Minoan town under the ash are the day, and both are 20 to 30 minutes by bus from Fira; boats to Thirasia and the volcano leave from the old port.",
    ),
    (
        "florence",
        "Day trips from Florence: Pisa (an hour by train) for the tower, Siena (90 minutes) for the square, and Lucca for the walls and the bikes. Fiesole is 30 minutes by bus for the view back over Florence.",
    ),
    (
        "istanbul",
        "Day trips from Istanbul: The Princes' Islands by ferry (an hour and a half) where there are no cars, Bursa for the Ottoman tombs, and the Bosphorus village of Anadolu Kavagi at the mouth of the Black Sea.",
    ),
    (
        "brussels",
        "Day trips from Brussels: Bruges (an hour by train) for the canals, Ghent (35 minutes) for the old port, and Leuven for the university town. Book Bruges ahead in summer, it fills up.",
    ),
    (
        "new york",
        "Day trips from New York: Beacon for Dia:Beacon (80 minutes on Metro-North), Philadelphia (90 minutes on Amtrak) and the Hamptons by the LIRR. The Jersey shore is a longer day.",
    ),
    (
        "budapest",
        "Day trips from Budapest: Szentendre for the artist town and the Danube bend (40 minutes on the H5 train), Esztergom for the basilica, and Lake Balaton, which is two hours and wants a night.",
    ),
    (
        "helsinki",
        "Day trips from Helsinki: Suomenlinna by ferry (15 minutes on the same ticket as the trams), Porvoo for the wooden old town (an hour by bus) and Tallinn by ferry (two hours, and take your passport).",
    ),
    (
        "valencia",
        "Day trips from Valencia: The Albufera for the rice fields and the lagoon (30 minutes by bus), Sagunto for the Roman theatre, and Requena for the wine. The City of Arts is in the city, not a day out.",
    ),
    (
        "seville",
        "Day trips from Seville: Cordoba for the mosque-cathedral (45 minutes by train), Ronda for the gorge and the bridge, and Jerez for the sherry bodegas. Granada is three hours by bus, so it wants a night.",
    ),
    (
        "bangkok",
        "Day trips from Bangkok: Ayutthaya for the ruined temples (an hour and a half by train), Kanchanaburi for the bridge over the River Kwai, and the floating market at Damnoen Saduak, which needs a very early start.",
    ),
    (
        "bristol",
        "Day trips from Bristol: Bath (15 minutes by train), Wells for the cathedral, and the Mendips or the Gower if you have a car. Cardiff is 50 minutes by train.",
    ),
    (
        "glasgow",
        "Day trips from Glasgow: Loch Lomond (40 minutes by train), Stirling for the castle, and Edinburgh in an hour. The islands need a ferry and a full day.",
    ),
    (
        "chicago",
        "Day trips from Chicago: Milwaukee (90 minutes on Amtrak), the Indiana Dunes for the beach (an hour by train and bus) and the Baha'i temple at Wilmette, which is free and half a day on the Purple line.",
    ),
    (
        "toronto",
        "Day trips from Toronto: Niagara Falls (two hours by train or bus, and the falls are free), Niagara-on-the-Lake for the wineries on the same trip, and Stratford for the Shakespeare festival.",
    ),
    (
        "tallinn",
        "Day trips from Tallinn: Lahemaa national park for the manors and the coast (an hour by bus), Tartu (two and a half hours) and Helsinki by ferry in two hours.",
    ),
    (
        "granada",
        "Day trips from Granada: The Alhambra is the day, not a detour: book a timed ticket weeks ahead. The Alpujarras villages are an hour and a half by bus, and Cordoba is two and a half hours by train, so it wants a night.",
    ),
    (
        "london",
        "Day trips from London: Bath (80 minutes by train), Oxford and Cambridge (about an hour), and Brighton (an hour) for the sea. Book the train a week ahead and you halve the fare.",
    ),
    (
        "lisbon",
        "Day trips from Lisbon: Sintra for the palaces (40 minutes by train), Cascais for the beach (40 minutes on the same line), and Obidos for the walled town (an hour by bus from Lisbon's Campo Grande, and no train goes there). Buy the train ticket at the machine and go before 9am.",
    ),
    (
        "madrid",
        "Day trips from Madrid: Toledo (30 minutes by train), Segovia (30 minutes) and El Escorial. Toledo and Segovia both deserve a full day rather than an afternoon.",
    ),
    (
        "munich",
        "Day trips from Munich: Neuschwanstein (two hours by train and bus), Salzburg (90 minutes) and the Bavarian lakes. Buy a Bayern ticket: it covers the regional trains for a group.",
    ),
    (
        "dublin",
        "Day trips from Dublin: Howth for the cliff walk (30 minutes on the DART), Malahide castle, and Glendalough in the Wicklow mountains, which needs a car or a bus tour.",
    ),
    (
        "berlin",
        "Day trips from Berlin: Potsdam for Sanssouci (30 minutes by train) and Sachsenhausen, which is a heavy half-day and worth it. Both are covered by the Berlin ABC travel card.",
    ),
    (
        "venice",
        "Day trips from Venice: Verona (an hour by train) for the arena, Padua for the Scrovegni chapel, and the islands in the lagoon: Murano for glass and Burano for the coloured houses, on a vaporetto day pass.",
    ),
    (
        "naples",
        "Day trips from Naples: Pompeii and Herculaneum (40 minutes on the Circumvesuviana), the Amalfi coast by ferry or bus, and Ischia for a slower island day. Vesuvius is a bus and a walk from Ercolano.",
    ),
    (
        "athens",
        "Day trips from Athens: Cape Sounion for the temple at sunset, Delphi for the ruins (two and a half hours by bus), and Aegina or Hydra by ferry for an island day. The fast ferries are worth the extra.",
    ),
    (
        "prague",
        "Day trips from Prague: Kutna Hora for the bone church and the silver mines (an hour by train), Karlstejn for the castle, and Cesky Krumlov, which is three hours each way so it wants a night.",
    ),
    (
        "vienna",
        "Day trips from Vienna: Bratislava (an hour by train), the Wachau valley by boat for the vineyards and Durnstein, and Baden for the spa. The Einfach-Raus ticket covers a group on the regional trains.",
    ),
    (
        "boston",
        "Day trips from Boston: Salem for the witch museum (30 minutes by train), Concord and Lexington for the revolution, and Providence (an hour on Amtrak). Cape Cod is 90 minutes by car.",
    ),
    (
        "sydney",
        "Day trips from Sydney: The Blue Mountains (two hours by train), the Hunter Valley for wine (three hours, better on a tour) and Manly or Watsons Bay by ferry for a beach day close in.",
    ),
    (
        "havana",
        "Day trips from Havana: Vinales for the tobacco valley (three hours by bus, better with a night), the beach at Santa Maria del Mar (30 minutes by taxi), and Matanzas for the river and the poets.",
    ),
    (
        "krakow",
        "Day trips from Krakow: Auschwitz-Birkenau (90 minutes by bus, and book a timed entry), the Wieliczka salt mine (30 minutes by train) and Zakopane for the mountains (two hours). Auschwitz is a heavy half-day, not a sight.",
    ),
    (
        "zurich",
        "Day trips from Zurich: Lucerne (45 minutes by train), the Rhine Falls (an hour) and Interlaken for the mountains (two hours). A travel pass covers the trains and the boats.",
    ),
    (
        "paris",
        "Day trips from Paris: Versailles (40 minutes on the RER C), Chartres for the cathedral, and Giverny for Monet's garden from April to October. Go early: the first trains are the quietest.",
    ),
    (
        "tokyo",
        "Day trips from Tokyo: Nikko for the shrines and Kamakura for the Buddha (both about two hours), or Hakone for a view of Fuji if the sky is clear. Go on a weekday: the weekend trains are packed.",
    ),
    (
        "leeds",
        "Day trips from Leeds: York (25 minutes by train), the Yorkshire Dales (an hour by car) and Harrogate for a spa afternoon. Manchester is 50 minutes by train if you want a city.",
    ),
    (
        "porto",
        "Day trips from Porto: The Douro valley by train to Pinhao for the vineyards, Braga and Guimaraes for the churches and the old town, and Aveiro for the canals. The Douro line is the scenic one, so sit on the right.",
    ),
    (
        "split",
        "Day trips from Split: Hvar by catamaran (an hour) for the old town and the lavender, Trogir (30 minutes by bus) for the walled centre, and Krka for the waterfalls, which needs a car or a tour.",
    ),
    (
        "hanoi",
        "Day trips from Hanoi: Halong Bay for the boats (two and a half hours by bus, and a night on the water is better than a day), Ninh Binh for the river rowing boats, and Sapa by night train for the rice terraces.",
    ),
    (
        "dubai",
        "Day trips from Dubai: Abu Dhabi for the grand mosque and the Louvre (90 minutes by car or bus), the desert for a dune drive, and Hatta for the mountains and the kayaks. The Abu Dhabi bus leaves from Al Ghubaiba.",
    ),
    (
        "milan",
        "Day trips from Milan: Lake Como for Bellagio (an hour by train to Como, then a boat), Bergamo for the old town (50 minutes) and Turin (an hour on the fast train). The Last Supper needs booking weeks ahead.",
    ),
    (
        "kyoto",
        "Day trips from Kyoto: Nara for the deer and the great Buddha (45 minutes by train), Osaka (30 minutes) and Hikone for the castle on Lake Biwa. Fushimi Inari and Arashiyama are in Kyoto, so go at dawn instead.",
    ),
    (
        "rome",
        "Day trips from Rome: Ostia Antica for the Roman ruins (30 minutes on the metro and train), Tivoli for the villa gardens, and Orvieto for the cathedral. Pompeii is two hours each way, so it is a full day.",
    ),
    (
        "bath",
        "Day trips from Bath: Stonehenge (an hour by bus) with a booked slot, Lacock for the village and the abbey, and Bradford-on-Avon for the canal and the tithe barn, 15 minutes by train.",
    ),
    (
        "oslo",
        "Day trips from Oslo: The fjord by boat from the city hall pier, Drobak for the fortress and the old town (an hour by bus), and Bergen by train, which is the best rail journey in the country and wants a night.",
    ),
];
pub(super) const AIRPORTS: &[(&str, &str)] = &[
    (
        "rio de janeiro",
        "Rio has two: Galeao (GIG), 20km north, for international flights, and Santos Dumont (SDU) in the city for the Sao Paulo shuttle. From Galeao a taxi is 30 to 50 minutes.",
    ),
    (
        "san francisco",
        "San Francisco (SFO) is 21km south, and BART to the city is 30 minutes for about $10. Oakland (OAK) is across the bay and often cheaper.",
    ),
    (
        "san sebastian",
        "San Sebastian (EAS) is 20km east at Hondarribia and has very few flights, so most people fly to Bilbao (75 minutes by bus) or Biarritz (40 minutes).",
    ),
    (
        "buenos aires",
        "Buenos Aires has two: Ezeiza (EZE), 30km south, for long-haul, and Aeroparque (AEP) in the city for domestic flights. From Ezeiza a taxi is 45 minutes and about 30 dollars; bus 8 is the cheap way.",
    ),
    (
        "los angeles",
        "Los Angeles (LAX) is 30km south-west and the one people mean; the FlyAway bus to Union Station is about 45 minutes. Hollywood Burbank (BUR) is closer for domestic flights.",
    ),
    (
        "mexico city",
        "Mexico City (MEX) is 13km east, and the metro line 5 is cheap and slow; a taxi from the official stand inside the terminal is 30 to 45 minutes and about 250 pesos.",
    ),
    (
        "copenhagen",
        "Copenhagen (CPH) is 8km south-east, and the metro to Kongens Nytorv is 15 minutes for about 30 kroner.",
    ),
    (
        "dubrovnik",
        "Dubrovnik (DBV) is 20km south-east, and the Atlas shuttle to the old town is about €8 and 30-40 minutes, timed to the flights; a taxi is €40-50.",
    ),
    (
        "barcelona",
        "Barcelona El Prat (BCN) is 12km south-west, and the Aerobus to Plaza Catalunya is 35 minutes and about €7.50.",
    ),
    (
        "amsterdam",
        "Amsterdam Schiphol (AMS) is 15km south-west, and the train to Centraal is 15-20 minutes for about €6.",
    ),
    (
        "edinburgh",
        "Edinburgh (EDI) is 13km west, and the tram or the 100 bus to the centre takes about 30 minutes for £6.50.",
    ),
    (
        "stockholm",
        "Stockholm Arlanda (ARN) is 40km north, and the Arlanda Express is 20 minutes and about 320 kronor; the commuter train takes 40 minutes and costs less. Bromma (BMA) is close to the city but only for domestic flights.",
    ),
    (
        "reykjavik",
        "Keflavik (KEF) is the airport people mean, 50km south-west, and the Flybus to the city is 45 minutes for about 3,500 kroner. Reykjavik (RKV) is a small city airport for domestic flights only.",
    ),
    (
        "singapore",
        "Singapore Changi (SIN) is 20km east, and the MRT to the city is 30 to 40 minutes for about S$2. Allow an hour there before your flight: the Jewel waterfall is inside the airport.",
    ),
    (
        "cape town",
        "Cape Town (CPT) is 20km east, and a taxi or an Uber to the centre is 20 to 30 minutes and about 250 rand. There is a train, but not one to take with bags.",
    ),
    (
        "santorini",
        "Santorini (JTR) is 6km from Fira, and the bus meets the flights; a taxi is 20 to 30 euros. It is a small airport with a short runway, so summer flights are often delayed.",
    ),
    (
        "ljubljana",
        "Ljubljana (LJU) is 25km north, and the airport bus to the centre is 30 minutes for about €4; a taxi is €25 to €30.",
    ),
    (
        "budapest",
        "Budapest Ferenc Liszt (BUD) is 16km south-east, and bus 100E to Deak Ferenc ter is 30 minutes for about 900 forint; a taxi is about 10,000 forint.",
    ),
    (
        "florence",
        "Florence (FLR) is 5km north-west and small, and the T2 tram to Santa Maria Novella is 20 minutes for €1.50; a taxi is about €25.",
    ),
    (
        "new york",
        "New York has three: JFK for most international flights, Newark (EWR) for many too, and LaGuardia (LGA) for domestic. From JFK the AirTrain plus the subway is about $10 and 60-75 minutes.",
    ),
    (
        "istanbul",
        "Istanbul has two: IST on the European side, the main one, and SAW on the Asian side for budget flights. From IST the metro or the Havaist bus is 40-60 minutes.",
    ),
    (
        "brussels",
        "Brussels (BRU) is 12km north-east, and the train to the central station is 20 minutes for about €11. Charleroi (CRL) is the budget airport, 60km south, and the bus into town takes an hour.",
    ),
    (
        "helsinki",
        "Helsinki-Vantaa (HEL) is 20km north, and the I or P train to the central station is 30 minutes for about €4.10.",
    ),
    (
        "valencia",
        "Valencia (VLC) is 9km west, and metro line 3 or 5 to the centre is 20 minutes for about €5.",
    ),
    (
        "seville",
        "Seville (SVQ) is 10km north-east, and the EA bus to the centre is 35 minutes for €4; there is no train link.",
    ),
    (
        "tallinn",
        "Tallinn (TLL) is 4km from the centre, so it is a tram or a short taxi rather than a transfer: tram 4 gets you in for the price of a city ticket.",
    ),
    (
        "chicago",
        "Chicago O'Hare (ORD) is 27km north-west, and the CTA Blue Line runs all night, 45 minutes for $5. Midway (MDW) is nearer the city and on the Orange Line.",
    ),
    (
        "toronto",
        "Toronto Pearson (YYZ) is 27km north-west, and the UP Express to Union station is 25 minutes for about 12 Canadian dollars.",
    ),
    (
        "bangkok",
        "Bangkok has two: Suvarnabhumi (BKK), the main one 30km east, and Don Mueang (DMK), 25km north, for budget flights. From Suvarnabhumi the Airport Rail Link is 30 minutes for 45 baht.",
    ),
    (
        "granada",
        "Granada (GRX) is 17km west and small, and the airport bus to the centre is 40 minutes for €3. Most people fly to Malaga instead and take the bus, which is two hours.",
    ),
    (
        "krakow",
        "Krakow John Paul II (KRK) is 11km west, and the train to the main station is 20 minutes for about 12 zloty; a taxi is 70-90 zloty.",
    ),
    (
        "venice",
        "Venice has two: Marco Polo (VCE) for most flights, 13km north, and Treviso (TSF) for budget ones, 40km away. From Marco Polo the ATVO bus to Piazzale Roma is 20 minutes and €10.",
    ),
    (
        "athens",
        "Athens (ATH) is 20km east, and metro line 3 to Syntagma is 40 minutes and €9; a taxi is €40.",
    ),
    (
        "london",
        "London City (LCY) is the closest to the centre: about 25 minutes to Bank on the DLR. London has five, though, and which one suits depends where you are going: Heathrow (LHR) for most long-haul, Gatwick (LGW) for charters and short-haul, City itself if you are staying in the east. From Heathrow the Piccadilly line is an hour and about £5-6.",
    ),
    (
        "lisbon",
        "Lisbon Humberto Delgado (LIS) is 7km north of the centre, and the red metro line takes 20 minutes for about €2 with a Viva card.",
    ),
    (
        "madrid",
        "Madrid-Barajas (MAD) is 12km north-east, and the metro line 8 into the centre is about €5 and 40 minutes; a taxi is a flat €33.",
    ),
    (
        "dublin",
        "Dublin (DUB) is 10km north, and the Aircoach or the 747 bus to the centre is about €7-10 and 30-45 minutes; there is no train link yet.",
    ),
    (
        "berlin",
        "Berlin Brandenburg (BER) is 25km south-east, and the FEX or S9 train to Hauptbahnhof is 30 minutes for about €4.40.",
    ),
    (
        "prague",
        "Prague Vaclav Havel (PRG) is 15km west, and bus 119 to the metro, or the Airport Express to the main station, is about €1.50-2.50.",
    ),
    (
        "vienna",
        "Vienna (VIE) is 18km south-east, and the S-Bahn S7 to Wien Mitte is 25 minutes for about €4.60; the City Airport Train is 16 minutes and €14.90. A taxi is a flat €40.",
    ),
    (
        "munich",
        "Munich (MUC) is 38km north-east, and the S-Bahn S1 or S8 to the centre is 40 minutes for about €13; a taxi is €70 or more.",
    ),
    (
        "zurich",
        "Zurich (ZRH) is 12km north, and the train to the main station is 10 to 12 minutes for about 7 francs, which is the easiest airport transfer in Europe.",
    ),
    (
        "boston",
        "Boston Logan (BOS) is 6km east, and the Silver Line SL1 bus from the terminals takes about 20 minutes for a normal subway fare; the water taxi across the harbour is the fun way in.",
    ),
    (
        "sydney",
        "Sydney (SYD) is 8km south, and the Airport Link train to Central is 13 minutes for about 20 Australian dollars; a taxi is 25 to 35.",
    ),
    (
        "havana",
        "Havana (HAV) is 15km south of the city, and a taxi to the old town is 25 to 30 minutes and about 25 dollars or the same in euros; the airport bus is not worth the wait.",
    ),
    (
        "naples",
        "Naples (NAP) is 6km north-east, and the Alibus to the central station and the port is 20 minutes for €5; a taxi is about €25, and agree the price first.",
    ),
    (
        "porto",
        "Porto Francisco Sa Carneiro (OPO) is 11km north-west, and the metro line E to the centre takes about 30 minutes for €2; a taxi is €25-30.",
    ),
    (
        "paris",
        "Paris has two: Charles de Gaulle (CDG) for most long-haul, 25km north-east, and Orly (ORY) to the south for domestic and short-haul. The RER B from CDG is 35 minutes and about €11.80.",
    ),
    (
        "milan",
        "For Milan it is Malpensa (MXP) for long-haul, 50km north-west, or Linate (LIN) for short-haul, 7km east. The Malpensa Express to the centre is 50 minutes.",
    ),
    (
        "dubai",
        "Dubai International (DXB) is 5km from the centre, and the metro red line reaches most of the city for a few dirhams.",
    ),
    (
        "tokyo",
        "Tokyo has two: Haneda (HND), 20km south and the convenient one, and Narita (NRT), 60km east, which most long-haul flights use. From Haneda the monorail to Hamamatsucho is 20 minutes for about 500 yen.",
    ),
    (
        "hanoi",
        "Hanoi (HAN) is 35km north, and bus 86 to the old quarter is 45 minutes for 35,000 dong; a taxi or a Grab is about 350,000 dong and 40 minutes.",
    ),
    (
        "kyoto",
        "Kyoto has no airport of its own: fly into Osaka Kansai (KIX), which is 75 minutes away on the Haruka train, or into Tokyo and take the shinkansen.",
    ),
    (
        "rome",
        "Rome has two: Fiumicino (FCO), the main one, 30km south-west, and Ciampino (CIA) for budget flights, 15km east. From Fiumicino the Leonardo Express is 32 minutes to Termini for €14.",
    ),
    (
        "oslo",
        "Oslo Gardermoen (OSL) is 47km north, and the Flytoget is 20 minutes for about 240 kroner; the Vy regional train is slower and about half the price.",
    ),
    (
        "bath",
        "Bath has no airport: Bristol (BRS) is 30km west, and the Flyer bus runs to Bath in 55 minutes. By train, come into Bath Spa from Bristol Temple Meads.",
    ),
];
pub(super) const GET_TO: &[(&str, &str)] = &[
    (
        "great barrier reef",
        "Getting to the reef: fly to Cairns or Port Douglas and take a day boat from the marina, or go out from Airlie Beach in the Whitsundays. Book two days ahead, take the seasickness tablets before you board.",
    ),
    (
        "amalfi coast",
        "Getting to the Amalfi coast: fly to Naples, then the ferry from Molo Beverello to Amalfi or Positano in season, or the Circumvesuviana train to Sorrento and the SITA bus along the coast. A private transfer is the only sensible option with heavy luggage.",
    ),
    (
        "cinque terre",
        "Getting to the Cinque Terre: trains from Pisa or Genoa run to La Spezia, then the local line stops at each village. Parking is almost impossible in the villages, so leave the car at La Spezia or Levanto.",
    ),
    (
        "machu picchu",
        "Getting to Machu Picchu: fly or bus to Cusco, then the train from Poroy or Ollantaytambo to Aguas Calientes, then the bus up. The Inca Trail needs a permit booked months ahead with a licensed operator, and entry tickets are timed.",
    ),
    (
        "isle of skye",
        "Getting to Skye: drive over the bridge from Kyle of Lochalsh, or take the train to Kyle and the bus, or the ferry from Mallaig to Armadale. Book accommodation months ahead for summer and expect single-track roads.",
    ),
];
pub(super) const MONTHS: &[&str] = &[
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
    "jan",
    "feb",
    "mar",
    "apr",
    "jun",
    "jul",
    "aug",
    "sep",
    "sept",
    "oct",
    "nov",
    "dec",
];
pub(super) const REGIONS: &[(&str, Option<&str>)] = &[
    (
        "In Cornwall: St Ives for the beach and the Tate, Padstow for the food and the Camel trail, the Eden Project's two biomes near St Austell, the Minack theatre cut into the cliff above Porthcurno, and Tintagel for the Arthur legend. The coast path between St Ives and Padstow is the day out that beats all of them.",
        Some(
            "Porthmeor at St Ives for surf and a town behind it, Kynance Cove on the Lizard for the water, and Sennen or Watergate Bay for space. Car parks are full by ten in August, so go early or go in the evening.",
        ),
    ),
    (
        "In Devon: Dartmoor for the ponies and the tors, the South West Coast Path in either direction, Torquay and Paignton for the sand and the steam train to Dartmouth, and Exeter for the cathedral and the quay. Salcombe and Woolacombe are the two beaches people drive for.",
        Some(
            "Woolacombe for surf and sand that goes on, Blackpool Sands near Dartmouth for the shelter and the cafe, and Bantham for the walk to the village. The Exmouth end is sand and shallow, which is the one for small children.",
        ),
    ),
    (
        "In the Lake District: Windermere for the steamer and the low walks, Keswick for Catbells, which is the best short climb in the country, Dove Cottage in Grasmere for the Wordsworth, and the Honister pass for the slate mine. Scafell Pike is the hill, and the Ullswater steamer is the easy day.",
        None,
    ),
    (
        "In Norfolk: The Broads by boat from Wroxham, the seal trip from Blakeney Point, Holkham for the beach behind the pines, Sandringham if they like the royal houses, and Norwich for the cathedral and Elm Hill. Cromer for crab and the pier.",
        Some(
            "Holkham for the pines and the space, Wells-next-the-sea for the beach huts and the pine walk, and Cromer for the pier and the crabs.",
        ),
    ),
    (
        "In Yorkshire: Malham Cove and Gordale Scar in the Dales, Whitby for the abbey and the Dracula steps, Robin Hood's Bay for the village at the bottom of the hill, the North Yorkshire Moors railway from Pickering, and York for the Minster and the Shambles.",
        Some(
            "Whitby for sand and the abbey above it, Robin Hood's Bay for rock pools at low tide, and Filey for the long flat one that suits children.",
        ),
    ),
    (
        "In Scotland: Edinburgh for the castle and Arthur's Seat, then the Highlands: Glencoe for the glen, Skye for the Quiraing and the Fairy Pools, Glenfinnan for the viaduct, and the Cairngorms for a hill day. A car is what makes it, since the trains only reach the edges.",
        None,
    ),
    (
        "In Wales: Snowdonia for Snowdon by the Pyg track and Betws-y-Coed for the slate, Pembrokeshire for the coast path and the beaches at Barafundle and Whitesands, and the Brecon Beacons for the waterfalls walk. Cardiff for the castle and the bay if the weather turns.",
        Some(
            "Barafundle in Pembrokeshire for the walk down through the woods, Whitesands near St Davids for surf, and Tenby for a harbour and a town. The coast path links them if you want a leg of it.",
        ),
    ),
    (
        "In the Peak District: Mam Tor and the Great Ridge for the classic day, Chatsworth for the house and the garden, Dovedale for the stepping stones, and Bakewell for the pudding. Edale is the station if you come by train.",
        None,
    ),
    (
        "In the Cotswolds: Bourton-on-the-Water for the river running through the village, Castle Combe for the street everyone photographs, Chipping Campden and Broadway for the stone, and the Slaughters for a walk between them. It is a place for driving and stopping rather than one long walk.",
        None,
    ),
    (
        "On the Isle of Wight: The Needles and Alum Bay, Shanklin and Ventnor for sand and the coastal path, Carisbrooke Castle, and the cycle path that goes round the island. It is small enough for a long weekend.",
        Some(
            "Sandown and Shanklin for sand and a promenade, Compton Bay for surf and the cliffs, and Bembridge for rock pools when the tide is out.",
        ),
    ),
    (
        "In Dorset: The Jurassic Coast: Lulworth Cove and Durdle Door for the walk, Chesil Beach and Portland, Swanage for the steam train to Corfe Castle, and Studland for the sand. The fossil beaches are west of Charmouth.",
        Some(
            "Studland for sand and the nature reserve behind it, Weymouth for the long shallow bay, and Charmouth for fossils at low tide after a storm.",
        ),
    ),
    (
        "In Northumberland: Hadrian's Wall at Housesteads and Vindolanda, Alnwick Castle and the garden, Bamburgh and its beach, and Lindisfarne across the causeway at low tide. The dark skies park is the evening.",
        Some(
            "Bamburgh for the castle above the sand, Embleton for the dunes, and Alnmouth for a village at the end of the beach.",
        ),
    ),
];
pub(super) const REGION_NAMES: &[(&str, usize)] = &[
    ("bourton-on-the-water", 8),
    ("chipping campden", 8),
    ("moreton-in-marsh", 8),
    ("northumberland", 11),
    ("lake district", 2),
    ("peak district", 7),
    ("isle of wight", 9),
    ("pembrokeshire", 6),
    ("fort william", 5),
    ("betws-y-coed", 6),
    ("castle combe", 8),
    ("scarborough", 4),
    ("durdle door", 10),
    ("lindisfarne", 11),
    ("housesteads", 11),
    ("st austell", 0),
    ("porthcurno", 0),
    ("woolacombe", 1),
    ("ilfracombe", 1),
    ("windermere", 2),
    ("hunstanton", 3),
    ("cairngorms", 5),
    ("barafundle", 6),
    ("chatsworth", 7),
    ("yorkshire", 4),
    ("cotswolds", 8),
    ("dartmouth", 1),
    ("ambleside", 2),
    ("ullswater", 2),
    ("highlands", 5),
    ("inverness", 5),
    ("snowdonia", 6),
    ("cornwall", 0),
    ("scotland", 5),
    ("falmouth", 0),
    ("penzance", 0),
    ("tintagel", 0),
    ("salcombe", 1),
    ("paignton", 1),
    ("dartmoor", 1),
    ("coniston", 2),
    ("grasmere", 2),
    ("blakeney", 3),
    ("helmsley", 4),
    ("anglesey", 6),
    ("bakewell", 7),
    ("dovedale", 7),
    ("shanklin", 9),
    ("lulworth", 10),
    ("bamburgh", 11),
    ("norfolk", 3),
    ("st ives", 0),
    ("padstow", 0),
    ("newquay", 0),
    ("kynance", 0),
    ("torquay", 1),
    ("keswick", 2),
    ("wroxham", 3),
    ("holkham", 3),
    ("skipton", 4),
    ("glencoe", 5),
    ("mam tor", 7),
    ("ventnor", 9),
    ("swanage", 10),
    ("alnwick", 11),
    ("dorset", 10),
    ("kendal", 2),
    ("cromer", 3),
    ("whitby", 4),
    ("malham", 4),
    ("brecon", 6),
    ("buxton", 7),
    ("devon", 1),
    ("wales", 6),
    ("ripon", 4),
    ("hawes", 4),
    ("edale", 7),
    ("skye", 5),
    ("oban", 5),
];
